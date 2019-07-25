extern crate sfml;

use crate::map::filed::Field;

pub trait Unit {
    fn default_moving_pts() -> i32;
    fn current_moving_pts(&self) -> i32;
    fn cost_of_entering(field: Field) -> i32;
}

struct Mechanized {
    moving_pts: i32,
}
