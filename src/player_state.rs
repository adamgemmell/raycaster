extern crate cgmath;

use options;
use self::cgmath::{vec2, Vector2};
use self::cgmath::{Basis2, Rotation, Rotation2};
use self::cgmath::InnerSpace;
use self::cgmath::Rad;
use std::f64::consts::PI;

// blocks/sec^2
const ACCEL: f64 = 15.0;
// blocks/sec
const MAX_VEL: f64 = 15.0;

const FRICTION: f64 = 6.0;
// rads/sec
const TURN_VEL: f64 = 1.5;

pub struct PlayerState {
    pub pos: Vector2<f64>,
    pub dir: Vector2<f64>,
    pub vel: Vector2<f64>,
    pub plane: Vector2<f64>,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            pos: vec2(options::START_X, options::START_Y),
            dir: vec2(-1.0, 0.0),
            vel: vec2(0.0, 0.0),
            plane: vec2(0.0, 0.66),
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

        self.vel = if speed == 0.0 {
            vec2(0.0, 0.0)
        } else {
            self.vel.normalize_to(speed)
        };

        let future_pos = self.pos + self.vel * time;

        if options::MAP[future_pos.x as usize][future_pos.y as usize] == 0 {
            self.pos = future_pos;
        }
    }

    // time: secs
    pub fn adjust_dir(&mut self, time: f64, clockwise: bool) {
        let ang = if clockwise {
            -1.0 * TURN_VEL * time
        } else {
            TURN_VEL * time
        };
        let rotation: Basis2<f64> = Rotation2::from_angle(Rad(ang));
        self.dir = rotation.rotate_vector(self.dir);
        self.plane = rotation.rotate_vector(self.plane);
    }
}