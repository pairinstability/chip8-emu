use std::env;
use std::thread;
use std::time::Duration;
use std::error::Error;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod cpu;
mod display;
mod keypad;
mod consts;

use cpu::Cpu;
use display::Display;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    /* emulation cycle duration */
    let cycle_duration = Duration::from_millis(2);

    /* SDL2 context */
    let sdl_ctx = sdl2::init().unwrap();

    /* display */
    let mut dp = Display::new(&sdl_ctx);

    /* CPU */
    let mut cpu = Cpu::new();
    cpu.load_rom(&args[1]);

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { break 'main },
                Event::KeyDown {keycode: Some(keycode), .. } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else {
                        cpu.keypad.press(keycode);
                    }
                }
                _ => {}
            }
        }

        let output = cpu.emulate_cycle();
        
        /* only update screen if the vram has actually been changed */
        if output.vram_changed {
            dp.draw_screen(output.vram);
        }

        if output.beep {
            println!("BEEP");
        }

        thread::sleep(cycle_duration);
    }

    Ok(())
}
