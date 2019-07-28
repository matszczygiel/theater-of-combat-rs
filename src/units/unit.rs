extern crate sfml;

use super::token::*;
use crate::maps::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct MovingComponent {
    def_moving_pts: i32,
    current_moving_pts: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Mechanized<'a> {
    name: String,
    pub mc: MovingComponent,
    token: Token<'a>,
}

impl<'a> Mechanized<'a> {
    pub fn new(name: &str) -> Self {
        Mechanized {
            name: name.to_owned(),
            mc: MovingComponent::default(),
            token: Token::new(50.0),
        }
    }

    fn cost_of_entering(field: field::Field) -> i32 {
        match field {
            field::Field::Forest => 2,
            field::Field::Plain => 1,
        }
    }

    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn place_on_hex(
        &mut self,
        hex: hexagons::HexCoordinates,
        map: &map::Map,
    ) -> Result<(), &'static str> {
        let pos = map.hex_to_world_point(hex)?;
        self.token.set_position(pos);
        Ok(())
    }
}
