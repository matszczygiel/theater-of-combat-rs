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

impl Field {
    pub fn color(&self) -> Color {
        match self {
            Field::Plain => Color::GREEN,
            Field::Forest => Color::rgb(100, 140, 20),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HexShape<'a> {
    layout: std::rc::Rc<Layout>,
    shape: ConvexShape<'a>,
    highlighting_shape: ConvexShape<'a>,
    pub highlighted: bool,
}

impl<'a> HexShape<'a> {
    pub fn new(layout: std::rc::Rc<Layout>, coordinate: HexCoordinates) -> Self {
        let shape = ConvexShape::new(6);
        let mut hs = HexShape {
            layout,
            shape,
            highlighting_shape: ConvexShape::new(6),
            highlighted: false,
        };
        hs.update(coordinate);
        hs
    }

    pub fn update(&mut self, coordinate: HexCoordinates) {
        for i in 0..6 {
            self.shape.set_point(i, self.layout.corner_offset(i));
        }
        self.shape
            .set_position(hex_to_world_point(coordinate, *self.layout));
        self.highlighting_shape = self.shape.clone();

        let thickness = -self.layout.size.x.min(self.layout.size.y) * 0.04;
        self.shape.set_outline_thickness(thickness);
        self.shape.set_outline_color(&Color::BLACK);

        self.highlighting_shape.set_outline_thickness(0.0);
        self.highlighting_shape
            .set_fill_color(&Color::rgba(255, 0, 0, 120));
    }

    pub fn set_color(&mut self, color: &Color) {
        self.shape.set_fill_color(color);
    }
}

impl<'s> Drawable for HexShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        _states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw(&self.shape);
        if self.highlighted {
            target.draw(&self.highlighting_shape);
        }
    }
}
