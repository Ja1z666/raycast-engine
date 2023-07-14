use sfml::{graphics::RenderWindow, system::Clock};

pub fn fps(window: &mut RenderWindow, frame_count: &mut i32, clock: &mut Clock) {
    *frame_count += 1;
    if clock.elapsed_time().as_seconds() >= 1.0 {
        let fps = *frame_count as f32 / clock.elapsed_time().as_seconds();
        let title = format!("fps: {:.2}", fps);
        window.set_title(&title);
        *frame_count = 0;
        clock.restart();
    }
}

fn fmod(x: f32, y: f32) -> f32 {
    x % y
}

pub fn get_degrees(degrees: f32) -> f32 {
    fmod(360. + fmod(degrees, 360.), 360.)
}
