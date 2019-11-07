use super::components::*;
use crate::maps::*;

use std::collections::HashMap;

pub struct MovingSystem {
    pub components: HashMap<i32, MovingComponent>,
}

impl MovingSystem {
    pub fn new() -> Self {
        MovingSystem {
            components: HashMap::new(),
        }
    }

    pub fn get_on_hex(
        &mut self,
        hex: hexagons::HexCoordinates,
    ) -> HashMap<&i32, &mut MovingComponent> {
        self.components
            .iter_mut()
            .filter(|(_, mc)| mc.occupation == Some(hex))
            .collect()
    }
}
