extern crate sfml;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::*;

mod graphics;
mod maps;
mod units;

use std::rc::Rc;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Combat theater",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_framerate_limit(60);
    window.set_view(&View::new(
        Vector2f { x: 0.0, y: 0.0 },
        window.view().size(),
    ));

    let mut layout = maps::hexagons::Layout {
        orientation: maps::hexagons::Orientation::FLAT,
        size: Vector2f { x: 50.0, y: 50.0 },
        origin: Vector2f { x: 0.0, y: 0.0 },
    };

    let map = maps::map::Map::create_test_map();
    let mut map_gfx = graphics::map::Map::new(&map, layout);

    let mut unit = units::unit::Mechanized::new("test unit");

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
                    Key::W => map_gfx.layout.borrow_mut().size.y *= 0.95,
                    Key::S => map_gfx.layout.borrow_mut().size.y *= 1.05,
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

        map_gfx.update(&map);

        window.clear(&Color::CYAN);

        map_gfx.draw_hexes(&mut window);
        map_gfx.draw_rivers(&mut window);
        map_gfx.draw_outlines(&mut window);

        let coordinate = maps::hexagons::world_point_to_hex(
            window.map_pixel_to_coords_current_view(&current_mouse_pos),
            map_gfx.layout.borrow().clone(),
        );

        let entry = map_gfx.hexes.get(&coordinate);
        match entry {
            Some(shape) => window.draw(shape.highlight_shape()),
            None => {}
        };

        window.display();
    }
}
