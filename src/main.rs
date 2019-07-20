extern crate sfml;

pub mod map;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

use map::*;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Combat theater",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    let layout = hexagons::Layout {
        orientation: hexagons::Orientation::FLAT,
        size: Vector2f { x: 50.0, y: 50.0 },
        origin: Vector2f { x: 0.0, y: 0.0 },
    };

    let mut map = map::map::Map::new(layout);
    map.insert(hexagons::HexCoordinates::new_axial(0, 0), filed::Field::Plain);
    map.insert(hexagons::HexCoordinates::new_axial(0, 1), filed::Field::Plain);


    window.set_view(&View::new(Vector2f{x: 0.0, y: 0.0}, window.view().size()));
    println!("c: {:?}, s: {:?}", window.view().center(), window.view().size());

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
                    //                    Key::Right => hex.move_((2.0, 0.0)),
                    //                    Key::Left => hex.move_((-2.0, 0.0)),
                    //                    Key::Up => hex.move_((0.0, -2.0)),
                    //                    Key::Down => hex.move_((0.0, 2.0)),
                                        Key::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }

        window.clear(&Color::CYAN);
        window.draw(&map);
        window.display();
    }
}
