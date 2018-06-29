extern crate cgmath;
extern crate sdl2;

use game::Game;
use screen::Screen;

mod screen;
mod player_state;
mod options;
mod game;

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

    let canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("Unable to intialise canvas");

    let event_pump = sdl_context.event_pump()
        .expect("Unable to initialise event pump");

    let screen = Screen::init(canvas);

    let mut game = Game::init(screen, event_pump);

    game.game_loop();
}