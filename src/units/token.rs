extern crate sfml;

use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable,
};

use sfml::system::Vector2f;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    shape: RectangleShape<'a>,
    highlighting_shape: RectangleShape<'a>,
    pub highlighted: bool,
}

impl<'a> Token<'a> {
    pub fn new(size: f32) -> Self {
        let mut token = Self {
            shape: RectangleShape::default(),
            highlighting_shape: RectangleShape::default(),
            highlighted: false,
        };

        token.resize(size);

        token.shape.set_outline_color(&Color::BLACK);
        token.shape.set_fill_color(&Color::BLUE);

        token.highlighting_shape.set_outline_thickness(0.0);
        token
            .highlighting_shape
            .set_fill_color(&Color::rgba(255, 0, 0, 120));

        token
    }

    pub fn resize(&mut self, size: f32) {
        self.shape.set_size((size, size));
        self.shape.set_outline_thickness(-size * 0.05);
        self.shape.set_origin((size / 2.0, size / 2.0));

        self.highlighting_shape.set_size((size, size));
        self.highlighting_shape.set_origin((size / 2.0, size / 2.0));
    }

    pub fn set_color(&mut self, color: &Color) {
        self.shape.set_fill_color(color);
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.shape.set_position(position);
        self.highlighting_shape.set_position(position);
    }

    pub fn contains(&self, position: Vector2f) -> bool {
        self.shape.global_bounds().contains(position)
    }
}

impl<'s> Drawable for Token<'s> {
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
