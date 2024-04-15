use crate::component::*;
use bevy::app::{Plugin, Startup};
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_tweening::*;
use std::time::Duration;
pub struct Camera2DPlugin;
impl Plugin for Camera2DPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }
        app.add_systems(
            Update,
            component_animator_system::<Velocity>.in_set(AnimationSystem::AnimationUpdate),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Update, focus_target);
    }
}

fn setup_camera(mut commands: Commands) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(300),
        CameraVelocityLens {
            start: Velocity(0.0, 0.0),
            end: Velocity(0.0, 0.0),
        },
    );
    let c = &mut commands;
    c.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Velocity(0.0, 0.0),
        Animator::new(tween),
        CameraMarker,
    ));
}

fn focus_target(
    mut set: ParamSet<(
        Query<&Transform, With<CameraTarget>>,
        Query<&mut Transform, With<CameraMarker>>,
    )>,
) {
    let target = set.p0();
    let target_trans = target.get_single().unwrap().clone();
    let mut camera = set.p1();
    let mut camera_trans = camera.get_single_mut().unwrap();
    camera_trans.translation = target_trans.translation;
}

fn move_camera_by_input(
    mut camera_trans: Query<
        (&mut Transform, &mut Velocity, &mut Animator<Velocity>),
        With<CameraMarker>,
    >,
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut inputs_event: EventReader<KeyboardInput>,
) {
    let v = 600.0;
    let delta = time.delta_seconds();
    let (mut mut_trans, mut_velocity, mut animator) =
        camera_trans.get_single_mut().expect("Single camera");

    let mut x_delta = 0.0_f32;
    let mut y_delta = 0.0_f32;
    if inputs_event.read().any(|input| {
        [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD].contains(&input.key_code)
    }) {
        for input in inputs.get_pressed().into_iter() {
            match input {
                KeyCode::KeyW => y_delta += 1.0,
                KeyCode::KeyS => y_delta -= 1.0,
                KeyCode::KeyA => x_delta -= 1.0,
                KeyCode::KeyD => x_delta += 1.0,
                _ => {}
            }
        }

        animator.set_tweenable(Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_millis(500),
            CameraVelocityLens {
                start: mut_velocity.clone(),
                end: Velocity(x_delta * v, y_delta * v),
            },
        ));
    }

    // dbg!(&mut_velocity);
    mut_trans.translation += Vec3::new(mut_velocity.0 * delta, mut_velocity.1 * delta, 0.0);
}
