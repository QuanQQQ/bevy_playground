use bevy::prelude::*;
use bevy_tweening::Lens;

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct Velocity(pub f32, pub f32);

pub struct CameraVelocityLens {
    pub start: Velocity,
    pub end: Velocity,
}

impl Lens<Velocity> for CameraVelocityLens {
    fn lerp(&mut self, target: &mut Velocity, ratio: f32) -> () {
        target.0 = self.start.0 + (self.end.0 - self.start.0) * ratio;
        target.1 = self.start.1 + (self.end.1 - self.start.1) * ratio;
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component, Default, Debug, Clone)]
pub struct Controllable;

#[derive(Component, Clone, Copy, Default, Debug, Reflect)]
pub struct Tool;

#[derive(Component, Clone, Default, Copy, Debug, Reflect)]
pub enum Towards {
    Up,
    #[default]
    Down,
    Left,
    Right,
}
impl Towards {
    pub fn to_direction2d(self: Self) -> Direction2d {
        match self {
            Towards::Down => Direction2d::NEG_Y,
            Towards::Left => Direction2d::NEG_X,
            Towards::Right => Direction2d::X,
            Towards::Up => Direction2d::Y,
        }
    }
}

mod organism_bundle;
mod soil;

pub use organism_bundle::*;
pub use soil::*;
