extern crate sfml;

use crate::maps::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MovingComponent {
    def_moving_pts: i32,
    current_moving_pts: i32,
    pub occupation: Option<hexagons::HexCoordinates>,
}

impl MovingComponent {
}

#[derive(Debug, Clone, Default)]
pub struct Mechanized {
    name: String,
    pub mc: MovingComponent,
}

impl Mechanized {
    pub fn new(name: &str) -> Self {
        Mechanized {
            name: name.to_owned(),
            mc: MovingComponent {
                def_moving_pts: 15,
                current_moving_pts: 15,
                occupation: None,
            },
        }
    }

    fn cost_of_entering(field: types::Field) -> i32 {
        match field {
            types::Field::Forest => 2,
            types::Field::Plain => 1,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn place_on_hex(
        &mut self,
        hex: hexagons::HexCoordinates,
        map: &map::Map,
    ) -> Result<(), &'static str> {
        //  let pos = map.hex_to_world_point(hex)?;
        //  self.token.set_position(pos);
        Ok(())
    }
}
