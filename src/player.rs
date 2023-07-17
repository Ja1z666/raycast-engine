use crate::{
    utilities::{deg_to_rad, get_degrees},
    HEIGHT, TILE_SIZE, WIDTH,
};
use sfml::{
    graphics::{
        Color, IntRect, RectangleShape, RenderTarget, RenderWindow, Shape, Texture, Transformable,
    },
    system::{Clock, Vector2, Vector2f, Vector2i},
};

#[derive(Clone, Copy)]
pub struct Ray {
    dist: f32,
    side: bool,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            dist: 0.,
            side: false,
        }
    }
}

pub struct Moving {
    pub forward: bool,
    pub backward: bool,
    pub right: bool,
    pub left: bool,
}

pub struct Player {
    pub speed: f32,
    pub position: Vector2<f32>,
    pub rays: [Ray; WIDTH as usize],
    pub moving: Moving,
    pub direction_horizontal: f32,
}

const FOV: f32 = 60.;

const MAX_DIST: f32 = 300.;

const PLAYER_SIZE: f32 = 10.;

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
                * f32::cos(deg_to_rad(self.direction_horizontal))
                * delta_time;
            let step_y = self.speed
                * TILE_SIZE
                * f32::sin(deg_to_rad(self.direction_horizontal))
                * delta_time;

            let mut player_size = Vector2f::new(0., 0.);

            if step_x > 0. {
                player_size.x = PLAYER_SIZE;
            } else {
                player_size.x = -PLAYER_SIZE;
            }

            if step_y > 0. {
                player_size.y = PLAYER_SIZE;
            } else {
                player_size.y = -PLAYER_SIZE;
            }

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x + player_size.x) / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(step_x, 0.));
            }
            if map[((self.position.y + step_y + player_size.y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(0., step_y));
            }
        } else if self.moving.backward {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(deg_to_rad(self.direction_horizontal))
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(deg_to_rad(self.direction_horizontal))
                * delta_time;

            let mut player_size = Vector2f::new(0., 0.);

            if step_x > 0. {
                player_size.x = PLAYER_SIZE;
            } else {
                player_size.x = -PLAYER_SIZE;
            }

            if step_y > 0. {
                player_size.y = PLAYER_SIZE;
            } else {
                player_size.y = -PLAYER_SIZE;
            }

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x + player_size.x) / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y + player_size.y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(0., step_y));
            }
        }

        if self.moving.right {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(deg_to_rad(get_degrees(self.direction_horizontal + 90.)))
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(deg_to_rad(get_degrees(self.direction_horizontal + 90.)))
                * delta_time;

            let mut player_size = Vector2f::new(0., 0.);

            if step_x > 0. {
                player_size.x = PLAYER_SIZE;
            } else {
                player_size.x = -PLAYER_SIZE;
            }

            if step_y > 0. {
                player_size.y = PLAYER_SIZE;
            } else {
                player_size.y = -PLAYER_SIZE;
            }

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x + player_size.x) / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y + player_size.y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(0., step_y));
            }
        } else if self.moving.left {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(deg_to_rad(get_degrees(self.direction_horizontal - 90.)))
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(deg_to_rad(get_degrees(self.direction_horizontal - 90.)))
                * delta_time;

            let mut player_size = Vector2f::new(0., 0.);

            if step_x > 0. {
                player_size.x = PLAYER_SIZE;
            } else {
                player_size.x = -PLAYER_SIZE;
            }

            if step_y > 0. {
                player_size.y = PLAYER_SIZE;
            } else {
                player_size.y = -PLAYER_SIZE;
            }

            if map[(self.position.y / TILE_SIZE) as usize]
                [((self.position.x + step_x + player_size.x) / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(step_x, 0.));
            }

            if map[((self.position.y + step_y + player_size.y) / TILE_SIZE) as usize]
                [(self.position.x / TILE_SIZE) as usize]
                != 1
            {
                self.move_(Vector2f::new(0., step_y));
            }
        }

        for x in 0..WIDTH {
            let pos = self.position;
            let camera = self.direction_horizontal
                + FOV * (0.5 * WIDTH as f32 - x as f32) / (WIDTH as f32 - 1.);
            let ray_dir = Vector2f::new(f32::cos(deg_to_rad(camera)), f32::sin(deg_to_rad(camera)));

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
            let mut side = false;

            while !is_hit && dist < MAX_DIST {
                if side_dist.x < side_dist.y {
                    dist = side_dist.x;
                    side_dist.x += delta_dist.x;
                    map_current.x += step.x;
                    side = false;
                } else {
                    dist = side_dist.y;
                    side_dist.y += delta_dist.y;
                    map_current.y += step.y;
                    side = true;
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
                true => Ray { dist, side },
                false => Ray { dist: 0., side },
            }
        }
    }

    pub fn draw_screen(&self, window: &mut RenderWindow) {
        let texture = Texture::from_file("assets/wall.png").unwrap();

        let ray_start = Vector2f::new(
            self.position.x + 0.5 * TILE_SIZE,
            self.position.y + 0.5 * TILE_SIZE,
        );

        let floor_height = (HEIGHT as f32 * 0.5 - 20.) as u32;

        for x in 0..floor_height {
            let mut floor = RectangleShape::new();
            floor.set_position(Vector2f::new(0., (HEIGHT - x) as f32));
            floor.set_size(Vector2f::new(WIDTH as f32, 1.));
            floor.set_fill_color(Color::rgba(
                64,
                64,
                64,
                (255. - 255. / floor_height as f32 * x as f32) as u8,
            ));

            window.draw(&floor);
        }

        for x in 0..WIDTH {
            let ray = self.rays[x as usize];

            let ray_direction =
                FOV * (f32::floor(0.5 * WIDTH as f32) - x as f32) / (WIDTH as f32 - 1.);

            let projection_distance = 0.5 * TILE_SIZE / f32::tan(deg_to_rad(0.5 * FOV));

            let column_height = HEIGHT as f32 * projection_distance
                / (ray.dist * f32::cos(deg_to_rad(ray_direction)));

            let ray_end = Vector2f::new(
                ray_start.x
                    + ray.dist
                        * f32::cos(deg_to_rad(get_degrees(
                            self.direction_horizontal + ray_direction,
                        ))),
                ray_start.y
                    + ray.dist
                        * f32::sin(deg_to_rad(get_degrees(
                            self.direction_horizontal + ray_direction,
                        ))),
            );

            let wall_texture_column_x;

            if !ray.side {
                wall_texture_column_x = ray_end.y - TILE_SIZE * f32::floor(ray_end.y / TILE_SIZE);
            } else {
                wall_texture_column_x = TILE_SIZE * f32::ceil(ray_end.x / TILE_SIZE) - ray_end.x;
            }

            let mut column = RectangleShape::new();
            column.set_position(Vector2f::new(x as f32, 0.5 * HEIGHT as f32));
            column.set_size(Vector2f::new(1., column_height));
            column.set_origin(Vector2f::new(0., column_height * 0.5));
            column.set_texture(&texture, false);
            column.set_texture_rect(IntRect::new(
                f32::round(wall_texture_column_x) as i32,
                0,
                1,
                TILE_SIZE as i32,
            ));

            let opacity = (255. / MAX_DIST * ray.dist) as u8;

            let mut fog = RectangleShape::new();
            fog.set_position(Vector2f::new(x as f32, 0.5 * HEIGHT as f32));
            fog.set_size(Vector2f::new(1., column_height));
            fog.set_origin(Vector2f::new(0., column_height * 0.5));
            fog.set_fill_color(Color::rgba(0, 0, 0, opacity));

            window.draw(&column);
            window.draw(&fog);
        }
    }

    pub fn move_(&mut self, step: Vector2<f32>) {
        self.position = self.position + step;
    }
}
