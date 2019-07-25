extern crate sfml;

use crate::map::filed::Field;
use super::token::*;

pub trait Unit {
    fn default_moving_pts() -> i32;
    fn current_moving_pts(&self) -> i32;
    fn cost_of_entering(field: Field) -> i32;
}

pub struct Mechanized<'a> {
    moving_pts: i32,
    token: Token<'a>,
}

impl<'a> Unit for Mechanized<'a> {
    fn default_moving_pts() -> i32 {
        12
    }

    fn current_moving_pts(&self) -> i32 {
        self.moving_pts
    }

    fn cost_of_entering(field: Field) -> i32 {
        match field {
            Field::Forest => 2,
            Field::Plain =>1,
        }
    }

}