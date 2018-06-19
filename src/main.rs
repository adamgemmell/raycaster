extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

mod options;

pub fn main() {
    let sdl_context = sdl2::init().expect("Unable to initialise SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialise video subsystem");

    let window = video_subsystem.window("Raycaster Engine",
                    options::SCREEN_WIDTH_PIX, options::SCREEN_HEIGHT_PIX)
        .position_centered()
        .opengl()
        .build()
        .expect("Unable to initialise window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("Unable to intialise canvas");


    let mut event_pump = sdl_context.event_pump()
        .expect("Unable to initialise event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..} => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 160, 0));

        for x in 0..options::SCREEN_WIDTH {
            let (r, g, b)= options::COLOURS[x as usize %options::COLOURS.len()][0];
            canvas.set_draw_color(Color::RGB(r, g, b));
            draw_col(&mut canvas, x, x/2);
        } 

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// x & height in logical pixels
fn draw_col(canvas: &mut WindowCanvas, x: u32, height: u32) -> () {
        let x_real = x * options::SCREEN_SCALE;
        let h_real = height * options::SCREEN_SCALE;
        canvas.fill_rect(Rect::new(x_real as i32, (options::SCREEN_YMID as i32) - (h_real as
            i32/2), options::SCREEN_SCALE, h_real))
        .expect("Error drawing screen column");
}
