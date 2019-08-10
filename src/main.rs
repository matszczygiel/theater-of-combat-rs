#[macro_use]
extern crate log;
extern crate chrono;
extern crate simplelog;

use chrono::*;
use simplelog::*;

mod game;
mod maps;
mod messaging;
mod units;
mod graphics;
mod systems;

fn main() {
    let mut log_config = Config::default();
    log_config.offset = *Local::now().offset();
    TermLogger::init(LevelFilter::Trace, log_config, TerminalMode::Stdout).unwrap();
    game::Game::new().run().unwrap();
}
