extern crate cgmath;

use cgmath::Rad;
use cgmath::{Vector2, vec2};
use cgmath::InnerSpace;
use cgmath::{Rotation, Rotation2, Basis2};
use std::f64::consts::PI;
use options;

// blocks/sec^2
const ACCEL: f64 = 100.0;
// blocks/sec
const MAX_VEL: f64 = 50.0;

const FRICTION: f64 = 100.0;

pub struct PlayerState {
    pub pos: Vector2<f64>,
    pub dir: Vector2<f64>,
    pub vel: Vector2<f64>,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            pos: vec2(options::START_X, options::START_Y),
            dir: vec2(-1.0, 0.0),
            vel: vec2(0.0, 0.0),
        }
    }

    // dir: 0..3 (NESW relative to player)
    // delta_time: seconds
    pub fn add_impulse(&mut self, dir: u32, delta_time: f64) {
        let impulse = self.dir.normalize_to(ACCEL * delta_time);

        self.vel = match dir {
            0 => self.vel + impulse,
            1 => {
                let rot: Basis2<f64> = Rotation2::from_angle(Rad(-0.5 * PI));
                self.vel + rot.rotate_vector(impulse)
            }
            2 => self.vel - impulse,
            3 => {
                let rot: Basis2<f64> = Rotation2::from_angle(Rad(0.5 * PI));
                self.vel + rot.rotate_vector(impulse)
            }
            _ => {
                println!("Warning: Invalid direction: {}", dir);
                self.vel
            }
        };

        if self.vel.magnitude() > MAX_VEL {
            self.vel.normalize_to(MAX_VEL);
        }
    }

    // time: secs
    pub fn walk(&mut self, time: f64) {
        let old_speed = self.vel.magnitude().min(MAX_VEL);
        let speed = (old_speed - old_speed * FRICTION * time).max(0.0);

        if speed == 0.0 {
            self.vel = vec2(0.0, 0.0);
        } else {
            self.vel.normalize_to(speed);
        }

        let future_pos = self.pos + self.vel*time;

        if options::MAP[future_pos.x as usize][future_pos.y as usize] == 0 {
            self.pos = future_pos;
        }

    }

    pub fn adjust_dir(&mut self, ang: f64) {
        //TODO
    }
}