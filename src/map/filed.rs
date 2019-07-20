extern crate sfml;

use sfml::graphics::{
    Color, ConvexShape, Drawable, RenderStates, RenderTarget, Shape, Transformable,
};

use crate::hexagons::*;

#[derive(Debug, Copy, Clone)]
pub enum Field {
    Plain,
    Forest,
}

#[derive(Debug, Clone)]
pub struct HexShape<'a> {
    pub layout: std::rc::Rc<Layout>,
    shape: ConvexShape<'a>,
}

impl<'a> HexShape<'a> {
    pub fn new(layout: std::rc::Rc<Layout>, coordinate: HexCoordinates) -> Self {
        let shape = ConvexShape::new(6);
        let mut hs = HexShape {
            layout: layout,
            shape,
        };
        hs.update(coordinate);
        hs
    }

    pub fn update(&mut self, coordinate: HexCoordinates) {
        for i in 0..6 {
            self.shape.set_point(i, self.layout.corner_offset(i));
        }
        self.shape
            .set_position(hex_to_pixel(coordinate, *self.layout));
            let thickness = -self.layout.size.x.min(self.layout.size.y) * 0.075;
        self.shape.set_outline_thickness(thickness);
        self.shape.set_fill_color(&Color::RED);
        self.shape.set_outline_color(&Color::BLACK);
    }

    pub fn set_color(&mut self, color: &Color){
        self.shape.set_fill_color(color);
    }
}

impl<'s> Drawable for HexShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.shape.draw(target, states);
    }
}
