extern crate sfml;

use sfml::graphics::{
    Color, ConvexShape, Drawable, RenderStates, RenderTarget, Shape, Transformable,
};

use sfml::system::Vector2f;

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

#[derive(Debug, Copy, Clone)]
pub enum River {
    Small,
    Stream,
}

impl River {
    pub fn color(&self) -> Color {
        match self {
            River::Small => Color::BLUE,
            River::Stream => Color::CYAN,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiverShape<'a> {
    layout: std::rc::Rc<Layout>,
    shape: ConvexShape<'a>,
}

impl<'a> RiverShape<'a> {
    pub fn new(
        layout: std::rc::Rc<Layout>,
        coordinate1: HexCoordinates,
        coordinate2: HexCoordinates,
    ) -> Result<Self, &'static str> {
        let shape = ConvexShape::new(4);
        let mut rs = RiverShape { layout, shape };
        rs.update(coordinate1, coordinate2)?;
        Ok(rs)
    }

    pub fn update(
        &mut self,
        coordinate1: HexCoordinates,
        coordinate2: HexCoordinates,
    ) -> Result<(), &'static str> {
        if coordinate1.distance_to(&coordinate2) != 1 {
            Err("River updated with non neighboring hex coordinates")
        } else {
            let vec1 = hex_to_world_point(coordinate1, *self.layout);
            let vec2 = hex_to_world_point(coordinate2, *self.layout);

            let center = (vec1 + vec2) / 2.0;
            let connecter = (vec1 - vec2) / 2.0;
            let connecter_orth = Vector2f {
                x: -connecter.y * self.layout.size.x / self.layout.size.y,
                y: connecter.x * self.layout.size.y / self.layout.size.x,
            };

            self.shape
                .set_point(0, 1.05 * connecter_orth / 2.0 + 0.1 * connecter);
            self.shape
                .set_point(1, 1.05 * connecter_orth / 2.0 - 0.1 * connecter);
            self.shape
                .set_point(2, -1.05 * connecter_orth / 2.0 - 0.1 * connecter);
            self.shape
                .set_point(3, -1.05 * connecter_orth / 2.0 + 0.1 * connecter);

            self.shape.set_position(center);

            self.shape.set_outline_thickness(0.0);

            Ok(())
        }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.shape.set_fill_color(color);
    }
}

impl<'s> Drawable for RiverShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        _states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw(&self.shape);
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
