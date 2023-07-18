use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::{Clock, Vector2f},
    window::{Event, Key, Style},
};

mod map;
mod player;
mod utilities;

use map::Map;
use player::{Moving, Player, Ray};
use utilities::fps;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;

const TILE_SIZE: f32 = 32.;

fn main() {
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_mouse_cursor_visible(false);
    window.set_mouse_cursor_grabbed(true);

    let mut clock = Clock::start();
    let mut clock_fps = Clock::start();
    let mut frame_count = 0;

    let mut player = Player {
        speed: 5.,
        position: Vector2f::new(2. * TILE_SIZE, 2. * TILE_SIZE),
        rays: [Ray::new(); WIDTH as usize],
        moving: Moving {
            forward: false,
            backward: false,
            right: false,
            left: false,
        },
        direction_horizontal: 0.,
    };

    let map = Map::new();

    loop {
        fps(&mut window, &mut frame_count, &mut clock_fps);

        if window.has_focus() {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => return,
                    Event::KeyPressed { code: Key::W, .. } => player.moving.forward = true,
                    Event::KeyReleased { code: Key::W, .. } => player.moving.forward = false,
                    Event::KeyPressed { code: Key::S, .. } => player.moving.backward = true,
                    Event::KeyReleased { code: Key::S, .. } => player.moving.backward = false,
                    Event::KeyPressed { code: Key::D, .. } => player.moving.right = true,
                    Event::KeyReleased { code: Key::D, .. } => player.moving.right = false,
                    Event::KeyPressed { code: Key::A, .. } => player.moving.left = true,
                    Event::KeyReleased { code: Key::A, .. } => player.moving.left = false,
                    _ => {}
                }
            }

            window.clear(Color::BLACK);

            player.update(&mut clock, &map, &mut window);
            player.draw_screen(&mut window);

            window.display();
        }
    }
}
