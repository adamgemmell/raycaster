extern crate sdl2;

use options;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Screen {
    canvas: WindowCanvas
}

impl Screen {
    pub fn init(canvas: WindowCanvas) -> Screen {
        Screen {
            canvas
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn set_draw_colour(&mut self, rgb: Color) {
        self.canvas.set_draw_color(rgb);
    }

    // x & height in logical pixels
    pub fn draw_col(&mut self, x: u32, height: u32) -> () {
        let x_real = x * options::SCREEN_SCALE;
        let h_real = height * options::SCREEN_SCALE;
        self.canvas.fill_rect(Rect::new(x_real as i32, (options::SCREEN_YMID as i32) - (h_real as
            i32 / 2), options::SCREEN_SCALE, h_real))
            .expect("Error drawing screen column");
    }
}