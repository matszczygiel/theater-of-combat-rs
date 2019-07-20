extern crate sfml;

use sfml::graphics::{
    Drawable, RenderStates, RenderTarget,
};

use std::collections::{HashMap};
use std::rc::Rc;
use crate::hexagons::*;
use crate::filed::*;

#[derive(Debug, Clone, Default)]
pub struct Map<'a> {
    map: HashMap<HexCoordinates, (Field, HexShape<'a>)>,
    layout: Rc<Layout>,
}

impl<'a> Map<'a> {
    pub fn new(layout: Layout) -> Self {
        Map {map: HashMap::default(), layout: Rc::from(layout)}
    }

    pub fn insert(&mut self, coordinate: HexCoordinates, filed: Field){
        let hex_shape = HexShape::new(self.layout.clone(), coordinate);
        self.map.insert(coordinate, (filed, hex_shape));
    }

}

impl<'s> Drawable for Map<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for (_, (_, shape)) in self.map.iter(){
            target.draw(shape);
        }
    }
}