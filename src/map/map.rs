extern crate sfml;

use sfml::graphics::{Drawable, RenderStates, RenderTarget};
use sfml::system::Vector2f;

use crate::filed::*;
use crate::hexagons::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct Map<'a> {
    map: HashMap<HexCoordinates, (Field, HexShape<'a>)>,
    layout: Rc<Layout>,
}

impl<'a> Map<'a> {
    pub fn new(layout: Layout) -> Self {
        Map {
            map: HashMap::default(),
            layout: Rc::from(layout),
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
    }
}
