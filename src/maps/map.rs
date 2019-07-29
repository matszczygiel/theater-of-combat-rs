extern crate sfml;

use sfml::graphics::{Drawable, RenderStates, RenderTarget};
use sfml::system::Vector2f;

use crate::field::*;
use crate::hexagons::*;
use crate::graph::*;

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

    pub fn insert(&mut self, coordinate: HexCoordinates, field: Field) {
        let mut hex_shape = HexShape::new(self.layout.clone(), coordinate);
        hex_shape.set_color(&field.color());
        self.map.insert(coordinate, (field, hex_shape));
    }

    pub fn new_test(layout: Layout) -> Self {
        let mut map = Self::new(layout);
        for p in -5..5 {
            for q in -5..5 {
                if q < 0 && p < 0 {
                    map.insert(HexCoordinates::new_axial(q, p), Field::Forest);
                } else {
                    map.insert(HexCoordinates::new_axial(q, p), Field::Plain);
                }
            }
        }
        map
    }

    pub fn highlight(&mut self, coordinate: HexCoordinates, highlighted: bool) {
        let entry = self.map.get_mut(&coordinate);
        match entry {
            Some((_, shape)) => shape.highlighted = highlighted,
            None => {}
        };
    }

    pub fn highlight_at_world_point(&mut self, point: Vector2f, highlighted: bool) {
        self.highlight(world_point_to_hex(point, *self.layout), highlighted);
    }

    pub fn clear_highlighting(&mut self) {
        for (_, (_, shape)) in self.map.iter_mut() {
            shape.highlighted = false;
        }
    }

    pub fn hex_to_world_point(&self, hex: HexCoordinates) -> Result<Vector2f, &'static str> {
        if self.map.contains_key(&hex) {
            Ok(hex_to_world_point(hex, *self.layout))
        } else {
            Err("Map doesn't contain such hex")
        }
    }

    pub fn insert_river(
        &mut self,
        coordinate1: HexCoordinates,
        coordinate2: HexCoordinates,
        river: River,
    ) -> Result<(), &'static str> {
        let mut riv_shape = RiverShape::new(self.layout.clone(), coordinate1, coordinate2)?;
        riv_shape.set_color(&river.color());
        if self.map.contains_key(&coordinate1) && self.map.contains_key(&coordinate2) {
            self.rivers
                .insert((coordinate1, coordinate2), (river, riv_shape));
            Ok(())
        } else {
            Err("Map doesn't contains respective hexes")
        }
    }
}

impl<'s> Drawable for Map<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for (_, (_, shape)) in self.map.iter() {
            target.draw(shape);
        }
        for (_, (_, shape)) in self.rivers.iter() {
            target.draw(shape);
        }
    }
}
