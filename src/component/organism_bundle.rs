use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{
        AngularDamping, AngularVelocity, ColliderDensity, LinearDamping, LinearVelocity,
        LockedAxes, RigidBody,
    },
    plugins::collision::Collider,
};

use crate::{constants::TILE_SIZE, Towards};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum Species {
    Human,
    Animal,
    Plant,
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
    pub angular_damping: AngularDamping,
    pub towards: Towards,
    locked: LockedAxes,
}
impl Default for OrganismBundle {
    fn default() -> Self {
        Self {
            entity_name: Name::new("Human"),
            species: Species::Human,
            rigid: RigidBody::Dynamic,
            collider: Collider::rectangle(TILE_SIZE, TILE_SIZE),
            density: ColliderDensity(0.5),
            linear_velocity: LinearVelocity(Vec2::ZERO),
            linear_damping: LinearDamping(4.0),
            angular_damping: AngularDamping(1000000.),
            towards: Towards::default(),
            locked: LockedAxes::ROTATION_LOCKED,
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
    pub fn human() -> Self {
        default()
    }
    pub fn plant() -> Self {
        Self {
            entity_name: Name::new("Plant"),
            species: Species::Plant,
            rigid: RigidBody::Static,
            ..default()
        }
    }
    pub fn collider_radius(mut self, radius: f32) -> Self {
        self.collider = Collider::circle(radius);
        self
    }
    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = collider;
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
