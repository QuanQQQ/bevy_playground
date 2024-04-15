use bevy::prelude::*;
use bevy_tweening::Lens;
use bevy_xpbd_2d::{
    components::{ColliderDensity, LinearDamping, LinearVelocity, RigidBody},
    plugins::collision::Collider,
};

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component, Clone, Debug, Default)]
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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum Species {
    Human,
    Animal,
}
#[derive(Bundle, Debug)]
pub struct OrganismBundle {
    pub entity_name: Name,
    pub species: Species,
    pub rigid: RigidBody,
    pub collider: Collider,
    pub density: ColliderDensity,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
}
impl Default for OrganismBundle {
    fn default() -> Self {
        Self {
            entity_name: Name::new("Human"),

            species: Species::Human,
            rigid: RigidBody::Dynamic,
            collider: Collider::circle(50.0),
            density: ColliderDensity(0.5),
            linear_velocity: LinearVelocity(Vec2::ZERO),
            linear_damping: LinearDamping(1.0),
        }
    }
}
impl OrganismBundle {
    pub fn animal() -> Self {
        Self {
            entity_name: Name::new("Animal"),
            species: Species::Animal,
            rigid: RigidBody::Dynamic,
            ..default()
        }
    }
    pub fn collider_radius(mut self, radius: f32) -> Self {
        self.collider = Collider::circle(radius);
        self
    }
    pub fn collider_density(mut self, density: f32) -> Self {
        self.density = ColliderDensity(density);
        self
    }
    pub fn with_rigid(mut self, rigid: RigidBody) -> Self {
        self.rigid = rigid;
        self
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Controllable;
