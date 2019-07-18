extern crate sfml;

pub mod map;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::*;

use map::*;

static LAYOUT: hexagons::Layout = hexagons::Layout {
        orientation: hexagons::Orientation::POINTY,
        size: Vector2f { x: 50.0, y: 50.0 },
        origin: Vector2i { x: 0, y: 0 },
    };
    

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Combat theater",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    let mut hex = CustomShape::new(Box::new(filed::HexShape { layout: &LAYOUT }));
    let mut hex2 = CustomShape::new(Box::new(filed::HexShape { layout: &LAYOUT }));

    hex.set_fill_color(&Color::RED);
    hex.set_position((200.0, 200.0));
    hex.set_outline_thickness(3.0);
    hex.set_outline_color(&Color::BLACK);
    hex.update();

    hex2.set_fill_color(&Color::GREEN);
    hex2.set_position((200.0, 200.0));
    hex2.set_outline_thickness(3.0);
    hex2.set_outline_color(&Color::BLACK);
    hex2.update();


    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {
                    code,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => match code {
                    Key::Right => hex.move_((2.0, 0.0)),
                    Key::Left => hex.move_((-2.0, 0.0)),
                    Key::Up => hex.move_((0.0, -2.0)),
                    Key::Down => hex.move_((0.0, 2.0)),
                    Key::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }

        window.clear(&Color::CYAN);
        window.draw(&hex);
        window.draw(&hex2);
        window.display();
    }
}
