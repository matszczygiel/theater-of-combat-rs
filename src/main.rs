extern crate sfml;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate simplelog;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::*;

use chrono::*;
use simplelog::*;

use std::rc::Rc;

mod game;
mod graphics;
mod maps;
mod messaging;
mod units;

fn main() {
    let mut log_config = Config::default();
    log_config.offset = *Local::now().offset();
    TermLogger::init(LevelFilter::Trace, log_config, TerminalMode::Stdout).unwrap();
    game::Game::new().run().unwrap();
}
