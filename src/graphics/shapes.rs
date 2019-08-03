extern crate sfml;

use sfml::graphics::{
    Color, ConvexShape, Drawable, RenderStates, RenderTarget, Shape, Transformable,
};

use sfml::system::Vector2f;

use std::rc::Rc;
use std::cell::RefCell;

use crate::maps::{hexagons, map, types};

pub fn hex_color(kind: types::Field) -> Color {
    match kind {
        types::Field::Plain => Color::GREEN,
        types::Field::Forest => Color::rgb(100, 140, 20),
    }
}

pub fn river_color(kind: types::River) -> Color {
    match kind {
        types::River::Small => Color::BLUE,
        types::River::Stream => Color::CYAN,
    }
}

#[derive(Debug, Clone)]
pub struct RiverShape<'a> {
    layout: Rc<RefCell<hexagons::Layout>>,
    shape: ConvexShape<'a>,
}

impl<'a> RiverShape<'a> {
    pub fn new(layout: Rc<RefCell<hexagons::Layout>>, site: map::RiverSite) -> Self {
        let shape = ConvexShape::new(4);
        let mut rs = RiverShape {
            layout,
            shape,
        };
        rs.update(site);
        rs
    }

    pub fn update(&mut self, site: map::RiverSite) {
        let coordinate1 = *site.sides().0;
        let coordinate2 = *site.sides().1;

        let layout = *self.layout.borrow();

        let vec1 = hexagons::hex_to_world_point(coordinate1, layout);
        let vec2 = hexagons::hex_to_world_point(coordinate2, layout);

        let center = (vec1 + vec2) / 2.0;
        let connecter = (vec1 - vec2) / 2.0;
        let connecter_orth = Vector2f {
            x: -connecter.y * layout.size.x / layout.size.y,
            y: connecter.x * layout.size.y / layout.size.x,
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

        self.shape.set_fill_color(&river_color(site.kind()));
    }

    pub fn shape(&self) -> &ConvexShape<'a> {
        &self.shape
    }
}

#[derive(Debug, Clone)]
pub struct HexShape<'a> {
    layout: Rc<RefCell<hexagons::Layout>>,
    shape: ConvexShape<'a>,
    highlighting_shape: ConvexShape<'a>,
    outline_shape: ConvexShape<'a>,
}

impl<'a> HexShape<'a> {
    pub fn new(layout: Rc<RefCell<hexagons::Layout>>, site: map::HexSite) -> Self {
        let mut hs = HexShape {
            layout,
            shape: ConvexShape::new(6),
            highlighting_shape: ConvexShape::new(6),
            outline_shape: ConvexShape::new(6),
        };
        hs.update(site);
        hs
    }

    pub fn update(&mut self, site: map::HexSite) {
        let layout = *self.layout.borrow();
        for i in 0..6 {
            self.shape.set_point(i, layout.corner_offset(i));
        }
        self.shape
            .set_position(hexagons::hex_to_world_point(*site.coord(), layout));
        self.highlighting_shape = self.shape.clone();
        self.outline_shape = self.shape.clone();

        let thickness = -layout.size.x.min(layout.size.y) * 0.04;
        self.outline_shape.set_outline_thickness(thickness);
        self.outline_shape.set_outline_color(&Color::BLACK);
        self.outline_shape.set_fill_color(&Color::TRANSPARENT);

        self.highlighting_shape.set_outline_thickness(0.0);
        self.highlighting_shape
            .set_fill_color(&Color::rgba(255, 0, 0, 120));

        self.shape.set_fill_color(&hex_color(site.kind()));
        self.shape.set_outline_thickness(0.0);
    }

    pub fn fill_shape(&self) -> &ConvexShape<'a> {
        &self.shape
    }

    pub fn outline_shape(&self) -> &ConvexShape<'a> {
        &self.outline_shape
    }

    pub fn highlight_shape(&self) -> &ConvexShape<'a> {
        &self.highlighting_shape
    }
}
