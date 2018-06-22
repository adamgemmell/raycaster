extern crate sdl2;
extern crate cgmath;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::vec2;
use player::PlayerState;

use std::time::SystemTime;
use std::time;

mod options;
mod player;

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

    let mut ps = PlayerState::new();

    let mut plane = vec2(0.0, 0.66);

    let mut time = 0u64;            // Time of current frame
    let mut old_time = 0u64;        // Time of prev frame
    let mut frame_time = 0.0f64;    // Frame time (secs)

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'running,
                | Event::KeyDown {keycode: Some(Keycode::W), ..} => ps.add_impulse(0, frame_time),
                | Event::KeyDown {keycode: Some(Keycode::S), ..} => ps.add_impulse(2, frame_time),
                | Event::KeyDown {keycode: Some(Keycode::A), ..} => ps.add_impulse(3, frame_time),
                | Event::KeyDown {keycode: Some(Keycode::D), ..} => ps.add_impulse(1, frame_time),
                | Event::KeyDown {keycode: Some(Keycode::E), ..} => ps.adjust_dir(0.01),
                | Event::KeyDown {keycode: Some(Keycode::Q), ..} => ps.adjust_dir(-0.01),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 160, 0));

        ps.walk(frame_time);


        // Render
        for x in 0..options::SCREEN_WIDTH {
            let cam_x = 2.0*(x as f64)/(options::SCREEN_WIDTH as f64) -1.0;
            let ray_dir = ps.dir + plane*cam_x;

            let mut map_pos = vec2(ps.pos.x.floor() as i32, ps.pos.y.floor() as i32);

            let delta_dist: Vector2<f64> = vec2(1.0/ray_dir.x.abs(), 1.0/ray_dir.y.abs());

            let mut colour = 0usize;
            let mut is_vert_side = false;

            let mut step = vec2(1, 1);

            let side_dist_x = if ray_dir.x < 0.0 {
                step.x = -1;
                (ps.pos.x - map_pos.x as f64)*delta_dist.x
            } else {
                (map_pos.x as f64 + 1.0 - ps.pos.x)*delta_dist.x
            };

            let side_dist_y = if ray_dir.y < 0.0 {
                step.y = -1;
                (ps.pos.y-map_pos.y as f64)*delta_dist.y
            } else{
                (map_pos.y as f64 + 1.0 - ps.pos.y)*delta_dist.y
            };

            let mut side_dist = vec2(side_dist_x, side_dist_y);

            // DDA
            while colour == 0 {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    is_vert_side = true;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;
                    is_vert_side = false;
                }

                colour = options::MAP[map_pos.x as usize][map_pos.y as usize];
            }

            let perp_wall_dist = if is_vert_side {
                (map_pos.x as f64 - ps.pos.x + ((1-step.x)/2) as f64) / ray_dir.x
            } else {
                (map_pos.y as f64 - ps.pos.y + ((1-step.y)/2) as f64) / ray_dir.y
            };

            let (r, g, b)= options::COLOURS[colour-1][is_vert_side as usize];
            canvas.set_draw_color(Color::RGB(r, g, b));
            draw_col(&mut canvas, x, ((options::SCREEN_HEIGHT as f64/perp_wall_dist) as u32)
                .min(options::SCREEN_HEIGHT));
        } 

        canvas.present();

        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));

        let (ass, dicks) = calc_frametime(old_time);
        old_time = time;
        time = ass;
        frame_time = dicks;
        println!("FPS: {}", 1.0/frame_time);
    }
}

fn calc_frametime(old_time: u64) -> (u64, f64) {
    let since_epoch = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("System time reading failed");
    let time = since_epoch.as_secs()*1_000_000_000+(since_epoch.subsec_nanos() as u64);

    let frame_time: f64 = (time - old_time) as f64 / 1_000_000_000.0;

    (time, frame_time)
}

// x & height in logical pixels
fn draw_col(canvas: &mut WindowCanvas, x: u32, height: u32) -> () {
        let x_real = x * options::SCREEN_SCALE;
        let h_real = height * options::SCREEN_SCALE;
        canvas.fill_rect(Rect::new(x_real as i32, (options::SCREEN_YMID as i32) - (h_real as
            i32/2), options::SCREEN_SCALE, h_real))
        .expect("Error drawing screen column");
}
