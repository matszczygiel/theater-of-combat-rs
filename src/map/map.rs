extern crate sfml;

use sfml::system::Vector2i;
use std::collections::{HashMap, Set};
use hexagons::*;
use filed::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Map {
    origin_point: Vector2i,
    size: f32,
    map: HashMap<HexCoordinates, Field>,
    
}