extern crate log;
extern crate sfml;

use sfml::graphics::*;
use sfml::system::{Vector2f, Vector2i};
use sfml::window::*;

use super::graphics;
use super::maps::*;
use super::systems;
use super::units;

pub struct Game {
    running: bool,
    map: map::Map,
    systems: systems::GameSystems,
    units: units::unit_set::UnitSet,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            running: false,
            map: map::Map::create_test_map(),
            systems: systems::GameSystems::new(),
            units: units::unit_set::UnitSet::new(),
        };

        game.units = units::unit_set::UnitSet::create_test_unit_set(&mut game.systems);
        game
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        trace!("Initializing window.");
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

        let mut map_gfx = graphics::map::Map::new(
            &self.map,
            hexagons::Layout {
                orientation: hexagons::Orientation::POINTY,
                size: Vector2f { x: 50.0, y: 50.0 },
                origin: Vector2f { x: 0.0, y: 0.0 },
            },
        );

        let font = Font::from_file("resources/fonts/OpenSans-Regular.ttf")
            .ok_or("Failed to load font.")?;

        //    let mut unit = units::unit::Mechanized::new("test unit");
        // unit.mc.occupation = Some(hexagons::HexCoordinates::new_axial(1, -1));

        //  let mut token = graphics::tokens::Token::new(map_gfx.layout.clone(), &unit);

        let mut current_mouse_pos = Vector2i::default();

        self.running = true;
        while self.running {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => self.running = false,
                    Event::KeyPressed { code, .. } => match code {
                        Key::Right => {
                            let mut view = window.view().to_owned();
                            view.move_((8.0, 0.0));
                            window.set_view(&view);
                        }
                        Key::Left => {
                            let mut view = window.view().to_owned();
                            view.move_((-8.0, 0.0));
                            window.set_view(&view);
                        }
                        Key::Up => {
                            let mut view = window.view().to_owned();
                            view.move_((0.0, -8.0));
                            window.set_view(&view);
                        }
                        Key::Down => {
                            let mut view = window.view().to_owned();
                            view.move_((0.0, 8.0));
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

            map_gfx.update(&self.map);
            //     token.update(&unit);

            window.clear(&Color::CYAN);

            map_gfx.draw_hexes(&mut window);
            map_gfx.draw_rivers(&mut window);
            map_gfx.draw_outlines(&mut window);
            map_gfx.draw_coords(&mut window, &font);

            //       window.draw(token.fill_shape());

            let coordinate = hexagons::world_point_to_hex(
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
        window.close();
        return Ok(());
    }
}
