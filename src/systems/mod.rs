pub mod components;
pub mod moving;

use moving::MovingSystem;

pub struct GameSystems {
    pub moving: MovingSystem,
}

impl GameSystems {
    pub fn new() -> Self {
        Self {
            moving: MovingSystem::new(),
        }
    }
}