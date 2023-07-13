use crate::{HEIGHT, PI, TILE_SIZE, WIDTH};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Clock, Vector2f},
};

pub struct Moving {
    pub forward: bool,
    pub backward: bool,
}

pub struct Rotation {
    pub right: bool,
    pub left: bool,
}

pub struct Player {
    pub speed: f32,
    pub rays: [f32; WIDTH as usize],
    pub moving: Moving,
    pub rotation: Rotation,
}

const MAX_DIST: f32 = 50.;

impl Player {
    pub fn update(
        &mut self,
        player_entity: &mut RectangleShape,
        clock: &mut Clock,
        map: &[[i32; 16]; 12],
    ) {
        let delta_time = clock.restart().as_seconds();

        if self.moving.forward {
            let step_x = self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * player_entity.rotation())
                * delta_time;
            let step_y = self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * player_entity.rotation())
                * delta_time;

            if map[(player_entity.position().y / TILE_SIZE) as usize]
                [((player_entity.position().x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                player_entity.move_((step_x, 0.));
            }

            if map[((player_entity.position().y + step_y) / TILE_SIZE) as usize]
                [(player_entity.position().x / TILE_SIZE) as usize]
                != 1
            {
                player_entity.move_((0., step_y));
            }
        }

        if self.moving.backward {
            let step_x = -self.speed
                * TILE_SIZE
                * f32::cos(PI / 180. * player_entity.rotation())
                * delta_time;
            let step_y = -self.speed
                * TILE_SIZE
                * f32::sin(PI / 180. * player_entity.rotation())
                * delta_time;

            if map[(player_entity.position().y / TILE_SIZE) as usize]
                [((player_entity.position().x + step_x) / TILE_SIZE) as usize]
                != 1
            {
                player_entity.move_((step_x, 0.));
            }

            if map[((player_entity.position().y + step_y) / TILE_SIZE) as usize]
                [(player_entity.position().x / TILE_SIZE) as usize]
                != 1
            {
                player_entity.move_((0., step_y));
            }
        }

        if self.rotation.right {
            player_entity.rotate(-self.speed * 25. * delta_time);
        }

        if self.rotation.left {
            player_entity.rotate(self.speed * 25. * delta_time);
        }

        for x in 0..WIDTH {
            let pos = player_entity.position();
            let camera = player_entity.rotation()
                + 90. * (0.5 * WIDTH as f32 - x as f32) / (WIDTH as f32 - 1.);
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
        for x in 0..WIDTH {
            let dist = self.rays[x as usize];
            if dist > 0. {
                let line_height = HEIGHT as f32 / dist * 2.;

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

    pub fn draw_map(
        &self,
        map_obj: &Vec<RectangleShape>,
        player_entity: &RectangleShape,
        window: &mut RenderWindow,
    ) {
        for obj in map_obj {
            window.draw(obj);
        }

        window.draw(player_entity);
    }

    pub fn spawn() -> RectangleShape<'static> {
        let mut player_entity = RectangleShape::new();
        player_entity.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
        player_entity.set_fill_color(Color::rgb(100, 100, 200));
        player_entity.set_origin(Vector2f::new(TILE_SIZE / 2., TILE_SIZE / 2.));

        player_entity
    }
}
