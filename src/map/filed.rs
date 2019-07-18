extern crate sfml;

use sfml::graphics::CustomShapePoints;
use sfml::system::Vector2f;

use crate::hexagons::*;

#[derive(Debug, Copy, Clone)]
pub enum Field {
    Plain,
    Forest,
}

#[derive(Copy, Clone)]
pub struct HexShape<'a> {
    pub layout: &'a Layout,
}

impl<'a> CustomShapePoints for HexShape<'a> {
    fn point_count(&self) -> u32 {
        6
    }

    fn point(&self, point: u32) -> Vector2f {
        assert!(point < 6);
        self.layout.corner_offset(point)
    }
}
