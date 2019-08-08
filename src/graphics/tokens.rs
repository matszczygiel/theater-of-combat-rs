extern crate sfml;

use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable,
};

use sfml::system::Vector2f;

use std::cell::RefCell;
use std::rc::Rc;

use crate::maps::{hexagons, map};

#[derive(Debug, Clone, Default)]
pub struct Token<'a> {
    layout: Rc<RefCell<hexagons::Layout>>,
    shape: RectangleShape<'a>,
    highlighting_shape: RectangleShape<'a>,
}

impl<'a> Token<'a> {
    pub fn new(layout: Rc<RefCell<hexagons::Layout>>, site: map::HexSite) -> Self {
        let mut token = Self {
            layout,
            shape: RectangleShape::default(),
            highlighting_shape: RectangleShape::default(),
        };
        token.update(site);
        token
    }

    pub fn update(&mut self, site: map::HexSite) {
        let layout = *self.layout.borrow();
        let size = layout.size;
        self.shape.set_size(size);
        self.shape.set_origin(size / 2.0);
        self.shape
            .set_position(hexagons::hex_to_world_point(*site.coord(), layout));

        self.highlighting_shape = self.shape.clone();

        let thickness = -layout.size.x.min(layout.size.y) * 0.04;
        self.shape.set_outline_thickness(thickness);
        self.shape.set_outline_color(&Color::BLACK);
        self.shape.set_fill_color(&Color::YELLOW);
        self.shape.set_outline_thickness(0.0);

        self.highlighting_shape.set_outline_thickness(0.0);
        self.highlighting_shape
            .set_fill_color(&Color::rgba(255, 0, 0, 120));
    }

    pub fn fill_shape(&self) -> &RectangleShape<'a> {
        &self.shape
    }

    pub fn highlight_shape(&self) -> &RectangleShape<'a> {
        &self.highlighting_shape
    }

    pub fn contains(&self, position: Vector2f) -> bool {
        self.shape.global_bounds().contains(position)
    }
}
