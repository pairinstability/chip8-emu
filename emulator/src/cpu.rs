use std::fs;

use rand;
use rand::Rng;

use crate::keypad::Keypad;

use crate::consts::HEIGHT;
use crate::consts::WIDTH;
use crate::consts::RAM_SIZE;

const FONT: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, /* 0 */
                        0x20, 0x60, 0x20, 0x20, 0x70, /* 1 */
                        0xF0, 0x10, 0xF0, 0x80, 0xF0, /* 2 */
                        0xF0, 0x10, 0xF0, 0x10, 0xF0, /* 3 */
                        0x90, 0x90, 0xF0, 0x10, 0x10, /* 4 */
                        0xF0, 0x80, 0xF0, 0x10, 0xF0, /* 5 */
                        0xF0, 0x80, 0xF0, 0x90, 0xF0, /* 6 */
                        0xF0, 0x10, 0x20, 0x40, 0x40, /* 7 */
                        0xF0, 0x90, 0xF0, 0x90, 0xF0, /* 8 */
                        0xF0, 0x90, 0xF0, 0x10, 0xF0, /* 9 */
                        0xF0, 0x90, 0xF0, 0x90, 0x90, /* a */
                        0xE0, 0x90, 0xE0, 0x90, 0xE0, /* b */
                        0xF0, 0x80, 0x80, 0x80, 0xF0, /* c */
                        0xE0, 0x90, 0x90, 0x90, 0xE0, /* d */
                        0xF0, 0x80, 0xF0, 0x80, 0xF0, /* e */
                        0xF0, 0x80, 0xF0, 0x80, 0x80]; /* f */

pub struct Cpu {
    opcode: u16,
    ram: [u8; RAM_SIZE],
    vram: [[u8; WIDTH]; HEIGHT], 
    vram_changed: bool,
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    pub keypad: Keypad
}

pub struct OutputState<'a> {
    pub vram: &'a [[u8; WIDTH]; HEIGHT],
    pub vram_changed: bool,
    pub beep: bool,
}


impl Cpu {
    pub fn new() -> Self {
        let mut ram = [0u8; RAM_SIZE];

        for i in 0..FONT.len() {
            ram[i] = FONT[i];
        }

        Cpu {
            opcode: 0,
            ram: ram, 
            vram: [[0; WIDTH]; HEIGHT],
            vram_changed: false,
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: Keypad::new()
        }
    }

    /* load the ROM file into memory */
    pub fn load_rom(&mut self, filename: &str) {
        
        let rom_file = fs::read(filename).expect("unabel to read rom file");

        for (i, &byte) in rom_file.iter().enumerate() {
            let addr = 0x200 + i;

            if addr < RAM_SIZE {
                self.ram[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    /* emulate a cycle */
    pub fn emulate_cycle(&mut self) -> OutputState {
        self.vram_changed = false;

        self.fetch_opcode();
        self.opcode_execute();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        OutputState {
            vram: &self.vram,
            vram_changed: self.vram_changed,
            beep: self.sound_timer > 0,
        }
    }

    /* fetch opcode from RAM */
    fn fetch_opcode(&mut self) {
        self.opcode = (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc+1] as u16);
    }
    
    /* execute the opcode */
    fn opcode_execute(&mut self) {
        let nibbles = (
                (self.opcode & 0xf000) >> 12 as u8,
                (self.opcode & 0x0f00) >> 8 as u8,
                (self.opcode & 0x00f0) >> 4 as u8,
                (self.opcode & 0x000f) as u8,
        );

        let nnn = (self.opcode & 0x0fff) as usize;
        let nn = (self.opcode & 0x00ff) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;


        match nibbles {
            (0x00, 0x00, 0x00, 0x00) => self.op_0000(),
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xnn(x, nn),
            (0x04, _, _, _) => self.op_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xnn(x, nn),
            (0x07, _, _, _) => self.op_7xnn(x, nn),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxnn(x, nn),
            (0x0d, _, _, _)  => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _                     => { self.pc += 2; }
        };
    }

    /* NOP */
    fn op_0000(&mut self) {
        self.pc += 2;
    }

    /* CLS */
    fn op_00e0(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.vram[y][x] = 0;
            }
        }

        self.vram_changed = true;
        self.pc += 2;
    }

    /* RTS */
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    /* JMP */
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    /* CALL */
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc as u16 + 2;
        self.sp += 1;
        self.pc = nnn;
    }

    /* SKIP.EQ */
    fn op_3xnn(&mut self, x: usize, nn: u8) {
        self.pc += if self.v[x] == nn { 4 } else { 2 }
    }

    /* SKIP.NE */
    fn op_4xnn(&mut self, x: usize, nn: u8) {
        self.pc += if self.v[x] != nn { 4 } else { 2 }
    }

    /* SKIP.EQ */
    fn op_5xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] == self.v[y] { 4 } else { 2 }
    }

    /* MVI */
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
        self.pc += 2;
    }

    /* ADI */
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        let vx = self.v[x] as u16;
        let val = nn as u16;
        let result = vx + val;
        self.v[x] = result as u8;
        self.pc += 2;
    }

    /* MOV. */
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += 2;
    }

    /* OR. */
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += 2;
    }

    /* AND . */
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += 2;
    }

    /* XOR. */
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += 2;
    }

    /* ADD. */
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
        self.pc += 2;
    }

    /* SUB. */
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2;
    }

    /* SHR. */
    fn op_8x06(&mut self, x: usize) {
        self.v[0x0f] = self.v[x] & 0x01;
        self.v[x] >>= 1;
        self.pc += 2;
    }

    /* SUBB. */
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2;
    }

    /* SHL */
    fn op_8x0e(&mut self, x: usize) {
        self.v[0x0f] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
        self.pc += 2;
    }

    /* SKIP.NE */
    fn op_9xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] != self.v[y] { 4 } else { 2 };
    }

    /* MVI */
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
        self.pc += 2;
    }

    /* JMP */
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc += nnn + self.v[0] as usize;
    }

    /* RNDMSK */
    fn op_cxnn(&mut self, x: usize, nn: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = nn & rng.gen::<u8>();
        self.pc += 2;
    }

    /* SPRITE */
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0x0f] = 0;

        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % WIDTH;
                let color = (self.ram[self.i + byte] >> (7-bit)) & 0x01;
                self.v[0x0f] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }

        self.vram_changed = true;
        self.pc += 2;
    }

    /* SKIPKEY.Y */
    fn op_ex9e(&mut self, x: usize) {
        self.pc += if self.keypad.pressed(self.v[x] as usize) { 4 } else { 2 };
    }

    /* SKIPKEY.N */
    fn op_exa1(&mut self, x: usize) {
        self.pc += if !self.keypad.pressed(self.v[x] as usize) { 4 } else { 2 };
    }

    /* MOV */
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += 2;
    }

    /* KEY */
    fn op_fx0a(&mut self, x: usize) {
        self.wait_keypress(x);
    }

    /* MOV */
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += 2;
    }

    /* MOV */
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += 2;
    }

    /* ADI */
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.v[0x0f] = if self.i > 0x0f00 { 1 } else { 0 };
        self.pc += 2;
    }

    /* SPRITECHAR */
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
        self.pc += 2;
    }

    /* MOVBCD */
    fn op_fx33(&mut self, x: usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        self.pc += 2;
    }

    /* MOVM */
    fn op_fx55(&mut self, x: usize) {
        for i in 0..x+1 {
            self.ram[self.i + i] = self.v[i];
        }

        self.pc += 2;
    }

    /* MOVM */
    fn op_fx65(&mut self, x: usize) {
        for i in 0..x+1 {
            self.v[i] = self.ram[self.i + i];
        }

        self.pc += 2;
    }


    fn wait_keypress(&mut self, x: usize) {
        for i in 0..16 {
            if self.keypad.pressed(i as usize) {
                self.v[x] = i;
                break;
            }
        }
        self.pc += 2;
    }
}
