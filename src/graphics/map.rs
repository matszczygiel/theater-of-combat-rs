use super::shapes::*;

use crate::maps::*;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Map<'a> {
    pub layout: Rc<RefCell<hexagons::Layout>>,
    pub hexes: Vec<HexShape<'a>>,
    pub rivers: Vec<RiverShape<'a>>,
}

impl<'a> Map<'a> {
    pub fn new(map: &map::Map, layout: hexagons::Layout) -> Self {
        let layout = Rc::new(RefCell::new(layout));
        Map {
            layout: layout.clone(),
            hexes: map
                .hexes()
                .values()
                .map(|site| HexShape::new(layout.clone(), *site))
                .collect(),
            rivers: map
                .rivers()
                .values()
                .map(|site| RiverShape::new(layout.clone(), *site))
                .collect(),
        }
    }

    pub fn update(&mut self, map: &map::Map) {
        self.hexes = map
            .hexes()
            .values()
            .map(|site| HexShape::new(self.layout.clone(), *site))
            .collect();

        self.rivers = map
            .rivers()
            .values()
            .map(|site| RiverShape::new(self.layout.clone(), *site))
            .collect();
    }
}
