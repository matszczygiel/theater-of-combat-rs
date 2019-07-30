extern crate sfml;

use sfml::graphics::{Drawable, RenderStates, RenderTarget};
use sfml::system::Vector2f;

use crate::field::*;
use crate::hexagons::*;
use crate::graph::*;
use crate::types::*;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct HexSite {
    coord: HexCoordinates,
    kind: Field,
}

#[derive(Debug, Clone)]
struct RiverSite {
    side1: HexCoordinates,
    side2: HexCoordinates,
    kind: River,
}


#[derive(Debug, Clone, Default)]
pub struct Map {
    graph: BidirectionalGraph<i32>,
    hexes: HashMap<i32, HexSite>,
    rivers: HashMap<i32, RiverSite>,

    current_free_id: i32,

}

impl Map {
    pub fn new() -> Self {
        Map {
            graph: BidirectionalGraph::default(),
            hexes: HashMap::default(),
            rivers: HashMap::default(),
            current_free_id : 0,
        }
    }

    pub fn insert_river(
        &mut self,
        river: RiverSite,
    ) -> Result<(), &'static str> {
        let found_hexes = self.hexes.iter().filter(|(id, hex)| { if hex == river.coord1 || hex == river.coord2 {return true;} else {return false;}}).collect();
            
        Ok(())
    }
}
