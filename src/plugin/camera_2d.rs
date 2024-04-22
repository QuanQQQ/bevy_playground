use std::marker::PhantomData;

use crate::component::*;
use crate::constants::{FIRST_LAYER, SECOND_LAYER};
use bevy::app::{Plugin, Startup};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Component, Default)]
pub struct FocusCamera;
#[derive(Component, Default)]
pub struct StaticCamera;

#[derive(Default)]
pub struct Camera2DPlugin<T> {
    pub d: PhantomData<T>,
}

impl<T: Default + Component> Plugin for Camera2DPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera::<T>)
            .add_systems(Update, focus_target);
    }
    fn is_unique(&self) -> bool {
        false
    }
}
fn setup_camera<T: Default + Component>(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        FIRST_LAYER,
        T::default(),
    ));
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        SECOND_LAYER,
        T::default(),
    ));
}

fn focus_target(
    mut set: ParamSet<(
        Query<&Transform, With<CameraTarget>>,
        Query<&mut Transform, With<FocusCamera>>,
    )>,
) {
    let target = set.p0();
    let target_trans = target.get_single().unwrap().clone();
    let mut camera = set.p1();
    for mut camera_trans in camera.iter_mut() {
        camera_trans.translation = target_trans.translation;
    }
}
