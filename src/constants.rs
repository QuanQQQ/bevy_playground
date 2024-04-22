use bevy::render::view::RenderLayers;

pub const TILE_SIZE: f32 = 32.;
pub const MAP_WIDTH: i32 = 500;
pub const MAP_HEIGHT: i32 = 500;

pub const FIRST_LAYER: RenderLayers = RenderLayers::layer(0);
pub const SECOND_LAYER: RenderLayers = RenderLayers::layer(1);
