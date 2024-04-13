use crate::component::*;
use bevy::app::{Plugin, Startup};
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_tweening::*;
use std::time::Duration;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }
        app.add_systems(
            Update,
            component_animator_system::<Velocity>.in_set(AnimationSystem::AnimationUpdate),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Update, move_camera);
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
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection::default()),
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        Velocity(0.0, 0.0),
        Animator::new(tween),
        CameraMarker,
    ));
}

fn move_camera(
    mut camera_trans: Query<
        (&mut Transform, &mut Velocity, &mut Animator<Velocity>),
        With<CameraMarker>,
    >,
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut inputs_event: EventReader<KeyboardInput>,
) {
    let v = 800.0;
    let delta = time.delta_seconds();
    let (mut mut_trans, mut_velocity, mut animator) =
        camera_trans.get_single_mut().expect("Single camera");

    let mut x_delta = 0.0_f32;
    let mut y_delta = 0.0_f32;
    if inputs_event.read().any(|input| {
        [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD].contains(&input.key_code)
    }) {
        dbg!(&inputs);
        for input in inputs.get_pressed().into_iter() {
            match input {
                KeyCode::KeyW => y_delta += 1.0,
                KeyCode::KeyS => y_delta -= 1.0,
                KeyCode::KeyA => x_delta -= 1.0,
                KeyCode::KeyD => x_delta += 1.0,
                _ => {}
            }
        }
        dbg!(Velocity(x_delta * v, y_delta * v));

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
