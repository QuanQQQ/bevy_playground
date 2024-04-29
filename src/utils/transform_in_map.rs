use bevy::transform::components::Transform;
use bevy_entitiles::math::extension::F32Integerize;

use crate::constants::TILE_SIZE;

pub trait TransformInMap {
    fn from_map_pos(x: i32, y: i32, z: f32) -> Self;
    fn to_map_pos(self: &Self) -> (i32, i32);
}
impl TransformInMap for Transform {
    fn from_map_pos(x: i32, y: i32, z: f32) -> Transform {
        Transform::from_xyz(
            x as f32 * TILE_SIZE + TILE_SIZE / 2.,
            y as f32 * TILE_SIZE + TILE_SIZE / 2.,
            z,
        )
    }
    fn to_map_pos(self: &Self) -> (i32, i32) {
        (
            (self.translation.x / TILE_SIZE).floor_to_i32(),
            (self.translation.y / TILE_SIZE).floor_to_i32(),
        )
    }
}
