use crate::{utilities::get_degrees, HEIGHT, PI, TILE_SIZE, WIDTH};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Clock, Vector2, Vector2f, Vector2i},
};

pub struct Moving {
    pub forward: bool,
    pub backward: bool,
    pub right: bool,
    pub left: bool,
}

pub struct Player {
    pub speed: f32,
    pub position: Vector2<f32>,
    pub rays: [f32; WIDTH as usize],
    pub moving: Moving,
    pub direction_horizontal: f32,
}

const FOV: f32 = 60.;

const MAX_DIST: f32 = 50.;

impl Player {
    pub fn update(&mut self, clock: &mut Clock, map: &[[i32; 16]; 12], window: &mut RenderWindow) {
        let delta_time = clock.restart().as_seconds();

        let window_center = Vector2i::new(
            f32::round(0.5 * WIDTH as f32) as i32,
            f32::round(0.5 * HEIGHT as f32) as i32,
        );

        let rotation_horizontal =
            FOV * ((window_center.x - window.mouse_position().x) as f32 / WIDTH as f32) * 3.;

        window.set_mouse_position(window_center);

        self.direction_horizontal = get_degrees(self.direction_horizontal + rotation_horizontal);

        if self.moving.forward {
            let step_x = self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * self.direction_horizontal)
                * delta_time;
            let step_y = self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * self.direction_horizontal)
                * delta_time;

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(0., step_y));
            }
        } else if self.moving.backward {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * self.direction_horizontal)
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * self.direction_horizontal)
                * delta_time;

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(0., step_y));
            }
        }

        if self.moving.right {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * get_degrees(self.direction_horizontal + 90.))
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * get_degrees(self.direction_horizontal + 90.))
                * delta_time;

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(0., step_y));
            }
        } else if self.moving.left {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * get_degrees(self.direction_horizontal - 90.))
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * get_degrees(self.direction_horizontal - 90.))
                * delta_time;

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.set_position(Vector2f::new(0., step_y));
            }
        }

        for x in 0..WIDTH {
            let pos = self.position;
            let camera = self.direction_horizontal
                + FOV * (0.5 * WIDTH as f32 - x as f32) / (WIDTH as f32 - 1.);
            let ray_dir = Vector2f::new(f32::cos(PI * camera / 180.), f32::sin(PI * camera / 180.));

            let delta_dist = Vector2f::new(
                TILE_SIZE * f32::sqrt(1. + (ray_dir.y * ray_dir.y) / (ray_dir.x * ray_dir.x)),
                TILE_SIZE * f32::sqrt(1. + (ray_dir.x * ray_dir.x) / (ray_dir.y * ray_dir.y)),
            );

            let mut map_current =
                Vector2f::new((pos.x / TILE_SIZE).floor(), (pos.y / TILE_SIZE).floor());

            let mut step = Vector2f::new(0., 0.);
            let mut side_dist = Vector2f::new(0., 0.);

            let mut is_hit = false;

            if ray_dir.x < 0. {
                step.x = -1.;
                side_dist.x = (pos.x / TILE_SIZE - map_current.x) * delta_dist.x;
            } else {
                step.x = 1.;
                side_dist.x = (map_current.x + 1. - pos.x / TILE_SIZE) * delta_dist.x;
            }

            if ray_dir.y < 0. {
                step.y = -1.;
                side_dist.y = (pos.y / TILE_SIZE - map_current.y) * delta_dist.y;
            } else {
                step.y = 1.;
                side_dist.y = (map_current.y + 1. - pos.y / TILE_SIZE) * delta_dist.y;
            }

            let mut dist = 0.;

            while !is_hit && dist < MAX_DIST {
                if side_dist.x < side_dist.y {
                    dist = side_dist.x;
                    side_dist.x += delta_dist.x;
                    map_current.x += step.x;
                } else {
                    dist = side_dist.y;
                    side_dist.y += delta_dist.y;
                    map_current.y += step.y;
                }

                if map_current.x >= 0.
                    && map_current.x < 16.
                    && map_current.y >= 0.
                    && map_current.y < 12.
                {
                    if map[map_current.y as usize][map_current.x as usize] == 1 {
                        is_hit = true;
                    }
                }
            }

            self.rays[x as usize] = match is_hit {
                true => dist,
                false => 0.,
            }
        }
    }

    pub fn draw_screen(&self, window: &mut RenderWindow) {
        // let ray_start = Vector2f::new(pla)

        for x in 0..WIDTH {
            let dist = self.rays[x as usize];

            if dist > 0. {
                let projection_distance = 0.5 * TILE_SIZE / f32::tan(PI * (0.5 * FOV) / 180.);

                let ray_direction =
                    FOV * (f32::floor(0.5 * WIDTH as f32) - x as f32) / (WIDTH as f32 - 1.);

                let line_height = HEIGHT as f32 * projection_distance
                    / (dist * f32::cos(PI * ray_direction / 180.));

                let line_color = Color::rgb(47, 145, 219);
                let mut line = RectangleShape::new();
                line.set_position(Vector2f::new(x as f32, 0.5 * HEIGHT as f32));
                line.set_size(Vector2f::new(1., line_height));
                line.set_fill_color(Color::rgb(
                    (line_color.r as f32 - line_color.r as f32 / MAX_DIST * dist) as u8,
                    (line_color.g as f32 - line_color.g as f32 / MAX_DIST * dist) as u8,
                    (line_color.b as f32 - line_color.b as f32 / MAX_DIST * dist) as u8,
                ));
                line.set_origin(Vector2f::new(0., line_height * 0.5));

                window.draw(&line);
            }
        }
    }

    pub fn draw_map(&self, map_obj: &Vec<RectangleShape>, window: &mut RenderWindow) {
        let mut player_entity = RectangleShape::new();
        player_entity.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
        player_entity.set_fill_color(Color::rgb(100, 100, 200));
        player_entity.set_origin(Vector2f::new(TILE_SIZE / 2., TILE_SIZE / 2.));
        player_entity.set_position(self.position);

        for obj in map_obj {
            window.draw(obj);
        }

        window.draw(&player_entity);
    }

    pub fn set_position(&mut self, step: Vector2<f32>) {
        self.position = self.position + step;
    }
}
