extern crate sfml;

pub mod maps;
pub mod units;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::*;

use maps::*;

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

    let mut map = maps::map::Map::new_test(layout);
    map.insert_river(
        hexagons::HexCoordinates::new_axial(1, 0),
        hexagons::HexCoordinates::new_axial(1, -1),
        field::River::Stream,
    );

    map.highlight(hexagons::HexCoordinates::new_axial(0, 0), true);

    window.set_view(&View::new(
        Vector2f { x: 0.0, y: 0.0 },
        window.view().size(),
    ));
    println!(
        "c: {:?}, s: {:?}",
        window.view().center(),
        window.view().size()
    );

    let mut unit = units::unit::Mechanized::new("test unit");
    unit.place_on_hex(hexagons::HexCoordinates::new_axial(1, -1), &map)
        .unwrap();

    let mut current_mouse_pos = Vector2i::default();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Right => {
                        let mut view = window.view().to_owned();
                        view.move_((5.0, 0.0));
                        window.set_view(&view);
                    }
                    Key::Left => {
                        let mut view = window.view().to_owned();
                        view.move_((-5.0, 0.0));
                        window.set_view(&view);
                    }
                    Key::Up => {
                        let mut view = window.view().to_owned();
                        view.move_((0.0, -5.0));
                        window.set_view(&view);
                    }
                    Key::Down => {
                        let mut view = window.view().to_owned();
                        view.move_((0.0, 5.0));
                        window.set_view(&view);
                    }
                    Key::Escape => window.close(),
                    _ => {}
                },
                Event::MouseWheelScrolled {
                    wheel: _,
                    delta,
                    x: _,
                    y: _,
                } => {
                    let mut view = window.view().to_owned();
                    if delta > 0.0 {
                        view.zoom(0.95);
                    } else if delta < 0.0 {
                        view.zoom(1.05);
                    }

                    window.set_view(&view);
                }
                Event::MouseMoved { x, y } => {
                    current_mouse_pos.x = x;
                    current_mouse_pos.y = y;
                }
                Event::Resized { width, height } => {
                    let mut view = window.view().to_owned();
                    view.set_size((width as f32, height as f32));
                    window.set_view(&view);
                }
                _ => {}
            }
        }

        map.clear_highlighting();
        let mouse_pos = window.map_pixel_to_coords_current_view(&current_mouse_pos);
        map.highlight_at_world_point(mouse_pos, true);

        window.clear(&Color::CYAN);
        window.draw(&map);
        window.draw(unit.get_token());
        window.display();
    }
}
