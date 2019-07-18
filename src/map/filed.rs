extern crate sfml;

use sfml::graphics::{Color, CustomShape, CustomShapePoints, RenderTarget, RenderWindow, Shape};
use hexagons::*


#[derive(Debug, Copy, Clone)]
pub enum Field {
    Plain,
    Forest,
}


#[derive(Copy, Clone)]
pub struct HexShape {
    layout: Layout,
}

impl CustomShapePoints for HexShape {
    const fn point_count(&self) -> u32 {
        6
    }

    fn point(&self, point: u32) -> Vector2f {
        match point {
            0 => Vector2f { x: 20., y: 580. },
            1 => Vector2f { x: 400., y: 20. },
            2 => Vector2f { x: 780., y: 580. },
            p => panic!("Non-existent point: {}", p),
        }
    }
}


Point hex_corner_offset(Layout layout, int corner) {
    Point size = layout.size;
    double angle = 2.0 * M_PI *
             (layout.orientation.start_angle + corner) / 6;
    return Point(size.x * cos(angle), size.y * sin(angle));
}