use bevy::prelude::*;
use bevy_tweening::Lens;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component, Clone, Debug)]
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
