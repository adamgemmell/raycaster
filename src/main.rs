extern crate sdl2;
extern crate num;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use std::time::Duration;

const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 75;

// Each pixel is n x n screen pixels
const SCREEN_SCALE: u32 = 8;

const SCREEN_WIDTH_PIX: u32 = SCREEN_WIDTH * SCREEN_SCALE;
const SCREEN_HEIGHT_PIX: u32 = SCREEN_HEIGHT * SCREEN_SCALE;
const SCREEN_YMID: u32 = SCREEN_HEIGHT_PIX/2;

pub fn main() {
    let sdl_context = sdl2::init().expect("Unable to initialise SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialise video subsystem");

    let window = video_subsystem.window("Raycaster Engine", SCREEN_WIDTH_PIX, SCREEN_HEIGHT_PIX)
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

        for x in num::range_step(0, SCREEN_WIDTH_PIX, SCREEN_SCALE) {
            draw_col(&mut canvas, x as i32, x / 2);
        } 

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_col(canvas: &mut WindowCanvas, x: i32, height: u32) -> () {
        canvas.fill_rect(Rect::new(x, (SCREEN_YMID as i32) - (height as i32/2), SCREEN_SCALE, height))
        .expect("Error drawing screen column");
}
