use sdl2::keyboard::Keycode;


pub struct Keypad {
    keys: [bool; 16]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn press(&mut self, key: Keycode) {

        match key {
            Keycode::Num1 => self.keys[0x1] = true,
            Keycode::Num2 => self.keys[0x2] = true,
            Keycode::Num3 => self.keys[0x3] = true,
            Keycode::Num4 => self.keys[0xc] = true,
            Keycode::Q => self.keys[0x4] = true,
            Keycode::W => self.keys[0x5] = true,
            Keycode::E => self.keys[0x6] = true,
            Keycode::R => self.keys[0xd] = true,
            Keycode::A => self.keys[0x7] = true,
            Keycode::S => self.keys[0x8] = true,
            Keycode::D => self.keys[0x9] = true,
            Keycode::F => self.keys[0xe] = true,
            Keycode::Z => self.keys[0xa] = true,
            Keycode::X => self.keys[0x0] = true,
            Keycode::C => self.keys[0xb] = true,
            Keycode::V => self.keys[0xf] = true,
            _ => {}
        }
    }
}
