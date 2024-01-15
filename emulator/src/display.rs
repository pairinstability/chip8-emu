use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;


use crate::consts::WIDTH;
use crate::consts::HEIGHT;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (HEIGHT as u32) * SCALE_FACTOR;


pub struct Display {
    canvas: Canvas<Window>
}


impl Display {
    pub fn new(sdl_ctx: &sdl2::Sdl) -> Self {
        let video = sdl_ctx.video().unwrap();
        let window = video
                        .window("window", SCREEN_WIDTH, SCREEN_HEIGHT)
                        .position_centered()
                        .opengl()
                        .build()
                        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Display { canvas: canvas }
    }

    pub fn draw_screen(&mut self, pixels: &[[u8; WIDTH]; HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                        .fill_rect(Rect::new(x as i32,
                                            y as i32,
                                            SCALE_FACTOR,
                                            SCALE_FACTOR));
            }
        }
        self.canvas.present();

    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0,0,0)
    } else {
        pixels::Color::RGB(0,255,0)
    }
}
