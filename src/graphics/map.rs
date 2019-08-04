extern crate sfml;

use sfml::graphics::RenderTarget;

use super::shapes::*;

use crate::maps::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Map<'a> {
    pub layout: Rc<RefCell<hexagons::Layout>>,
    pub hexes: HashMap<hexagons::HexCoordinates, HexShape<'a>>,
    pub rivers: Vec<RiverShape<'a>>,
}

impl<'a> Map<'a> {
    pub fn new(map: &map::Map, layout: hexagons::Layout) -> Self {
        let layout = Rc::new(RefCell::new(layout));
        let mut res = Map {
            layout,
            hexes: HashMap::new(),
            rivers: Vec::new(),
        };
        res.update(map);
        res
    }

    pub fn update(&mut self, map: &map::Map) -> &mut Self {
        self.hexes = map
            .hexes()
            .values()
            .map(|site| (*site.coord(), HexShape::new(self.layout.clone(), *site)))
            .collect();

        self.rivers = map
            .rivers()
            .values()
            .map(|site| RiverShape::new(self.layout.clone(), *site))
            .collect();
        self
    }

    pub fn draw_hexes(&self, target: &mut RenderTarget) {
        for hex in self.hexes.values() {
            target.draw(hex.fill_shape());
        }
    }

    pub fn draw_rivers(&self, target: &mut RenderTarget) {
        for riv in self.rivers.iter() {
            target.draw(riv.shape());
        }
    }

    pub fn draw_outlines(&self, target: &mut RenderTarget) {
        for hex in self.hexes.values() {
            target.draw(hex.outline_shape());
        }
    }
}