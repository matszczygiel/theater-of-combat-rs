extern crate sfml;

use crate::maps::*;

#[derive(Debug, Copy, Clone)]
pub enum UnitType {
    Mechanized,
}

#[derive(Debug, Clone)]
pub struct Unit {
    name: String,
    kind: UnitType,
    id: i32,
}

impl Unit {
    pub fn new(name: String, kind: UnitType, id: i32) ->Self {
        Unit{name, kind ,id}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> UnitType {
        self.kind
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

/*
pub trait Unit {
    fn get_name(&self) -> &String;
    fn cost_of_entering_hex(field: types::Field) -> i32;

    fn cost_of_crossing_river(river: types::River) -> i32;

    fn get_occupation(&self) -> Option<hexagons::HexCoordinates>;

    fn update(&mut self) {}
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MovingComponent {
    def_moving_pts: i32,
    current_moving_pts: i32,
    pub occupation: Option<hexagons::HexCoordinates>,
}

impl MovingComponent {
    pub fn reduce_mv_pts(&mut self, cost: i32) -> Result<&mut Self, &'static str> {
        if self.current_moving_pts < cost {
            return Err("Cost of movement higher than available moving_pts.");
        }
        self.current_moving_pts -= cost;
        Ok(self)
    }

    pub fn reest_mv_pts(&mut self) {
        self.current_moving_pts = self.def_moving_pts;
    }
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
}

impl Unit for Mechanized {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_occupation(&self) -> Option<hexagons::HexCoordinates> {
        self.mc.occupation
    }

    fn cost_of_entering_hex(field: types::Field) -> i32 {
        match field {
            types::Field::Forest => 2,
            types::Field::Plain => 1,
            _ => unreachable!(),
        }
    }

    fn cost_of_crossing_river(river: types::River) -> i32 {
        match river {
            types::River::Small => 6,
            types::River::Stream => 4,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_component_test() {
        let mut unit = Mechanized::new("Mechanized unit");
        unit.mc.occupation = Some(hexagons::HexCoordinates::new_axial(1, -1));
        let map = map::Map::create_test_map();

        // let paths = unit.mc.get_accesible_sites(&map).unwrap();
    }
}
*/
