extern crate sfml;

pub mod map;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

fn main() {
    let coord = map::hexagons::HexCoordinates::new_axial(2, 5);
    println!("Coord: {:?}", coord);

    let mut window = RenderWindow::new(
        (800, 600),
        "Combat theater",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    let mut object = RectangleShape::with_size(Vector2f { x: 50.0, y: 50.0 });
    object.set_position((0.0, 0.0));
    object.set_fill_color(&Color::RED);

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
                    Key::Right => object.move_((2.0, 0.0)),
                    Key::Left => object.move_((-2.0, 0.0)),
                    Key::Up => object.move_((0.0, -2.0)),
                    Key::Down => object.move_((0.0, 2.0)),
                    Key::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }

        window.clear(&Color::CYAN);
        object.draw(&mut window, RenderStates::default());
        window.display();
    }
}
