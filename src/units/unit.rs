extern crate sfml;


trait Movable {
    fn default_moving_pts() -> i32;
    fn current_moving_pts(&self) -> i32;
    fn cost_of_entering(map::filed::Field field) -> i32;
}


pub enum Unit {
    Mechanized {
        mv_points: i32, 
    },
    Armoured_cavalary {
        mv_points: i32,
    },
}

impl Unit {
    pub fn get_default_mv_points(&self) -> i32 {
        match self {
            Unit::Mechanized => 12,
            Unit::Armoured_cavalary => 10,
            _=> {panic!("Unknown unit type!");}
        }
    }
}