extern crate sfml;


trait Movable {
    fn default_moving_pts() -> i32;
    fn current_moving_pts(&self) -> i32;
    fn cost_of_entering(map::filed::Field field) -> i32;

}

struct Mechanized {
    moving_pts: i32,
    
}