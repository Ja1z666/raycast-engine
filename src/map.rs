use sfml::{
    graphics::{Color, RectangleShape, Shape, Transformable},
    system::Vector2f,
};

use crate::{player::Player, TILE_SIZE};

pub struct Map;

impl Map {
    pub fn new() -> [[i32; 16]; 12] {
        [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]
    }

    pub fn fill_map(map: &[[i32; 16]; 12], player: &mut Player) -> Vec<RectangleShape<'static>> {
        let mut map_obj: Vec<RectangleShape> = Vec::new();

        for i in 0..12 {
            for j in 0..16 {
                if map[i][j] == 1 {
                    let mut box_obj = RectangleShape::new();
                    box_obj.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
                    box_obj.set_fill_color(Color::rgb(255, 51, 255));
                    box_obj.set_position(Vector2f::new(j as f32 * TILE_SIZE, i as f32 * TILE_SIZE));
                    map_obj.push(box_obj);
                } else if map[i][j] == 2 {
                    player.set_position(Vector2f::new(
                        j as f32 * TILE_SIZE + (TILE_SIZE / 2.),
                        i as f32 * TILE_SIZE + (TILE_SIZE / 2.),
                    ));
                }
            }
        }

        map_obj
    }
}
