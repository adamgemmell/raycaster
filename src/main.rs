extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_YMID: u32 = SCREEN_HEIGHT/2;

pub fn main() {
    let sdl_context = sdl2::init().expect("Unable to initialise SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialise video subsystem");

    let window = video_subsystem.window("Raycaster Engine", SCREEN_WIDTH, SCREEN_HEIGHT)
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

        for x in 0..(SCREEN_WIDTH as i32) {
            draw_col(&mut canvas, x, x as u32/2);
        } 

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_col(canvas: &mut WindowCanvas, x: i32, height: u32) -> () {
        canvas.draw_rect(Rect::new(x, (SCREEN_YMID as i32) - (height as i32/2), 1, height))
        .expect("Error drawing screen column");
}
