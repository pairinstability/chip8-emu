use std::env;
use std::fs;

fn disassemble(instr: &[u8], pc: usize)
{
    /* pc, instruction */
    print!("{:04X?} {:02X?} {:02X?} ", pc, instr[0], instr[1]);

    /* the unknowns may just be data which is fine bc the instruction pointer
     * should never point there
     * https://stackoverflow.com/questions/37368412/how-do-deal-with-impossible-chip-8-instructions
     */

    /* shift so we work with the nibble in instr[0] */
    match instr[0] >> 4 {
        0 => {
            match instr[1] {
                /* 0x00 */
                0 => print!("NOP"),
                /* 0xe0 */
                224 => print!("CLS"),
                /* 0xee */
                238 => print!("RTS"),
                /* 0NNN op for RCA 1802. not needed here */
                _ => print!(""),
            }
        },
        1 => print!("JMP ${:01X?}{:02X?}", instr[0] & 0x0f, instr[1]),
        2 => print!("CALL ${:01X?}{:02X?}", instr[0] & 0x0f, instr[1]),
        3 => print!("SKIP.EQ V{:01X?},#${:02X}", instr[0] & 0x0f, instr[1]),
        4 => print!("SKIP.NE V{:01X?},#${:02X}", instr[0] & 0x0f, instr[1]),
        5 => print!("SKIP.EQ V{:01X?},V{:01X?}", instr[0] & 0x0f, instr[1] >> 4),
        6 => print!("MVI V{:01X?},#${:02X}", instr[0] & 0x0f, instr[1]),
        7 => print!("ADI V{:01X?},#${:02X}", instr[0] & 0x0f, instr[1]),
        8 => {
            match instr[1] & 0x0f {
                0 => print!("MOV. V{:01X?},V{:01X?}", instr[0] & 0x0f, 
                                                    instr[1] >> 4),
                1 => print!("OR. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                2 => print!("AND. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                3 => print!("XOR. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                4 => print!("ADD. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                5 => print!("ADD. V{:01X?},V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[0] & 0x0f,
                                                    instr[1] >> 4),
                6 => print!("SHR. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                7 => print!("SUBB. V{:01X?},V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4,
                                                    instr[1] >> 4),
                10 => print!("SHL. V{:01X?},V{:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4),
                _ => print!(""),
            }
        },
        9 => print!("SKIP.NE V{:01X?},V{:01X?}", instr[0] & 0x0f, instr[1] >> 4),
        /* a */
        10 => print!("MVI I,#${:01X?}{:02X?}", instr[0] & 0x0f, instr[1]),
        /* b */
        11 => print!("JMP ${:01X?}{:02X?}(V0)", instr[0] & 0x0f, instr[1]),
        /* c */
        12 => print!("RNDMSK V{:01X?},$%{:02X?}", instr[0] & 0x0f, instr[1]),
        /* d */
        13 => print!("SPRITE V{:01X?},V{:01X?},#${:01X?}", instr[0] & 0x0f,
                                                    instr[1] >> 4,
                                                    instr[1] & 0x0f),
        /* e */
        14 => {
            match instr[1] {
                /* 0x9e */
                158 => print!("SKIPKEY.Y V{:01X?}", instr[0] & 0x0f),
                /* 0xa1 */
                161 => print!("SKIPKEY.N V{:01X?}", instr[0] & 0x0f),
                _ => print!(""),
            }
        },
        /* f */
        15 => {
            match instr[1] {
                /* 0x07 */
                7 => print!("MOV V{:01X?},DELAY", instr[0] & 0x0f),
                /* 0x0a */
                10 => print!("KEY V{:01X?}", instr[0] & 0x0f),
                /* 0x15 */
                21 => print!("MOV DELAY,V{:01X?}", instr[0] & 0x0f),
                /* 0x18 */
                24 => print!("MOV SOUND,V{:01X?}", instr[0] & 0x0f),
                /* 0x1e */
                30 => print!("ADI I,V{:01X?}", instr[0] & 0x0f),
                /* 0x29 */
                41 => print!("SPRITECHAR I,V{:01X?}", instr[0] & 0x0f),
                /* 0x33 */
                51 => print!("MOVBCD (I),V{:01X?}", instr[0] & 0x0f),
                /* 0x55 */
                85 => print!("MOVM (I),V0-V{:01X?}", instr[0] & 0x0f),
                /* 0x65 */
                101 => print!("MOVM V0-V{:01X?}, (I)", instr[0] & 0x0f),
                _ => print!(""),
            }
        },
        _ => panic!("unknown instruction"),
    }

    print!("\n");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: Vec<u8> = fs::read(&args[1])
                                    .expect("error reading file");

    let size = fs::metadata(&args[1]).unwrap().len();

    let mut pc: usize = 0; 

    while pc < size.try_into().unwrap() {

        if pc == (size - 1).try_into().unwrap() {
            /* this condition is hit if there is a trailing byte and
             * instructions are always 2 bytes */
            break;
        }

        /* the instructions are 2 bytes */
        let instr: [u8; 2] = [contents[pc], contents[pc+1]];

        /* add 0x200 here for the offset */
        disassemble(&instr, pc + 0x200);

        pc += 2;
    }
}
