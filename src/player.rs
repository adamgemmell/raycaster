extern crate cgmath;

use cgmath::vec2;
use cgmath::Vector2;
use options;
//use std::f64::consts::PI;

pub struct PlayerState {
    pub pos: Vector2<f64>,
    pub dir: Vector2<f64>,
    pub speed: f64
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState{
            pos: vec2(options::START_X, options::START_Y),
            dir: vec2(-1.0, 0.0),
            speed: 0.0,
        }
    }

    // Dir: 0..3 (NESW)
    pub fn move_player(&mut self, dir: u32) {
        //TODO: make more realistic

        match dir {
            0 => self.pos = self.pos - vec2(0.0, 0.1),
            1 => self.pos = self.pos + vec2(0.1, 0.0),
            2 => self.pos = self.pos + vec2(0.0, 0.1),
            3 => self.pos = self.pos - vec2(0.1, 0.0),
            _ => println!("Warning: Invalid direction: {}", dir)
        }
    }

    pub fn adjust_dir(&mut self, ang: f64) {
        //TODO
    }
}