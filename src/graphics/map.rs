use super::shapes::*;

use crate::maps::*;

use std::rc::Rc;

#[derive(Debug, Default)]
struct Map<'a> {
    layout: Rc<hexagons::Layout>,
    pub hexes: Vec<HexShape<'a>>,
    pub rivers: Vec<RiverShape<'a>>,
}

impl<'a> Map<'a> {
    pub fn from(map: &map::Map) -> Self {
        let mut m = Map {
            hexes: Vec::new(),
            rivers: Vec::new(),
        };

        m.hexes = map.hexes().values().map(|site|{ HexShape::new(layout: Rc<hexagons::Layout>, site: map::HexSite)});


        m
    }
}
