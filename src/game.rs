extern crate cgmath;

use options;
use player_state::PlayerState;
use screen::Screen;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use self::cgmath::{vec2, Vector2};
use std::time;
use std::time::{SystemTime, Duration};

pub struct Game {
    ps: PlayerState,
    screen: Screen,
    event_pump: EventPump,
}

impl Game {
    pub fn init(screen: Screen, event_pump: EventPump) -> Game {
        Game {
            ps: PlayerState::new(),
            screen,
            event_pump,
        }
    }

    pub fn game_loop(&mut self) {
        let mut time = 0u64;            // Time of current frame (nanos)
        let mut old_time = 0u64;        // Time of prev frame (nanos)
        let mut frame_time = 0.0f64;    // Frame time (secs)

        let target_frame_time = 1_000_000_000u32/options::TARGET_FPS;    // nanos

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }


            {
                let ep = &self.event_pump;

                if ep.keyboard_state().is_scancode_pressed(Scancode::W) {self.ps.add_impulse(0, frame_time)};
                if ep.keyboard_state().is_scancode_pressed(Scancode::S) {self.ps.add_impulse(2, frame_time)};
                if ep.keyboard_state().is_scancode_pressed(Scancode::A) {self.ps.add_impulse(3, frame_time)};
                if ep.keyboard_state().is_scancode_pressed(Scancode::D) {self.ps.add_impulse(1, frame_time)};
                if ep.keyboard_state().is_scancode_pressed(Scancode::Q) {self.ps.adjust_dir(-0.01)};
                if ep.keyboard_state().is_scancode_pressed(Scancode::E) {self.ps.adjust_dir(0.01)};
            }

            self.screen.set_draw_colour(Color::RGB(0, 0, 0));
            self.screen.clear();

            self.ps.walk(frame_time);

            self.render();
            self.screen.present();

            //let frame_time_nanos = (frame_time*1_000_000_000f64) as u32;
            //if frame_time_nanos < target_frame_time {
                //::std::thread::sleep(Duration::new(0, target_frame_time));
            //}

            let (new_time, new_frame_time) = calc_frametime(old_time);
            old_time = time;
            time = new_time;
            frame_time = new_frame_time;
            println!("FPS: {}", 1.0 / frame_time);
        }
    }

    fn render(&mut self) {

        // For each vertical column
        for x in 0..options::SCREEN_WIDTH {
            let cam_x = 2.0 * (x as f64) / (options::SCREEN_WIDTH as f64) - 1.0;
            let ray_dir = self.ps.dir + self.ps.plane * cam_x;

            let mut map_pos = vec2(self.ps.pos.x.floor() as i32, self.ps.pos.y.floor() as i32);

            let delta_dist: Vector2<f64> = vec2(1.0 / ray_dir.x.abs(), 1.0 / ray_dir.y.abs());

            let mut colour = 0usize;
            let mut is_vert_side = false;

            let mut step = vec2(1, 1);

            let side_dist_x = if ray_dir.x < 0.0 {
                step.x = -1;
                (self.ps.pos.x - map_pos.x as f64) * delta_dist.x
            } else {
                (map_pos.x as f64 + 1.0 - self.ps.pos.x) * delta_dist.x
            };

            let side_dist_y = if ray_dir.y < 0.0 {
                step.y = -1;
                (self.ps.pos.y - map_pos.y as f64) * delta_dist.y
            } else {
                (map_pos.y as f64 + 1.0 - self.ps.pos.y) * delta_dist.y
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
                (map_pos.x as f64 - self.ps.pos.x + ((1 - step.x) / 2) as f64) / ray_dir.x
            } else {
                (map_pos.y as f64 - self.ps.pos.y + ((1 - step.y) / 2) as f64) / ray_dir.y
            };

            let (r, g, b) = options::COLOURS[colour - 1][is_vert_side as usize];
            self.screen.set_draw_colour(Color::RGB(r, g, b));
            self.screen.draw_col(x, ((options::SCREEN_HEIGHT as f64 / perp_wall_dist) as u32)
                .min(options::SCREEN_HEIGHT));
        }
    }
}

fn calc_frametime(old_time: u64) -> (u64, f64) {
    let since_epoch = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("System time reading failed");
    let time = since_epoch.as_secs() * 1_000_000_000 + (since_epoch.subsec_nanos() as u64);

    let frame_time: f64 = (time - old_time) as f64 / 1_000_000_000.0;

    (time, frame_time)
}