use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::{Event, Key, Style},
};
use std::f32::consts::PI;

mod map;
mod player;

use map::Map;
use player::{Moving, Player, Rotation};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const TILE_SIZE: f32 = 5.;

fn fps(window: &mut RenderWindow, frame_count: &mut i32, clock: &mut Clock) {
    *frame_count += 1;
    if clock.elapsed_time().as_seconds() >= 1.0 {
        let fps = *frame_count as f32 / clock.elapsed_time().as_seconds();
        let title = format!("fps: {:.2}", fps);
        window.set_title(&title);
        *frame_count = 0;
        clock.restart();
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut clock = Clock::start();
    let mut clock_fps = Clock::start();
    let mut frame_count = 0;

    let mut player = Player {
        speed: 5.,
        rays: [0.; WIDTH as usize],
        moving: Moving {
            forward: false,
            backward: false,
        },
        rotation: Rotation {
            right: false,
            left: false,
        },
    };
    let mut player_entity = Player::spawn();

    let map = Map::new();
    let map_obj = Map::fill_map(&map, &mut player_entity);

    loop {
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
                Event::KeyPressed { code: Key::D, .. } => player.rotation.right = true,
                Event::KeyReleased { code: Key::D, .. } => player.rotation.right = false,
                Event::KeyPressed { code: Key::A, .. } => player.rotation.left = true,
                Event::KeyReleased { code: Key::A, .. } => player.rotation.left = false,
                _ => {}
            }
        }

        window.clear(Color::BLACK);

        fps(&mut window, &mut frame_count, &mut clock_fps);

        player.update(&mut player_entity, &mut clock, &map);
        player.draw_screen(&mut window);
        player.draw_map(&map_obj, &player_entity, &mut window);

        window.display();
    }
}
