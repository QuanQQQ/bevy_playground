use crate::component::*;
use bevy::app::{Plugin, Startup};
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_tweening::*;
use std::f32::consts::TAU;
use std::time::Duration;

pub struct Camera3DPlugin;
impl Plugin for Camera3DPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }
        if !app.is_plugin_added::<PanOrbitCameraPlugin>() {
            app.add_plugins(PanOrbitCameraPlugin);
        }
        app.add_systems(
            Update,
            component_animator_system::<Velocity>.in_set(AnimationSystem::AnimationUpdate),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (smooth_moving, move_camera).chain());
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
    let mut camera_trans = Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    camera_trans.rotate_y(90.0_f32.to_radians());
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                // fov: 0.1,
                ..default()
            }),
            ..default()
        },
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
            yaw: Some(TAU / 8.0),
            pitch: Some(TAU / 8.0),
            radius: Some(10.0),
            // Set limits on rotation and zoom
            yaw_upper_limit: Some(TAU * 3.0 / 16.0),
            yaw_lower_limit: Some(TAU * 1.0 / 16.0),
            pitch_upper_limit: Some(TAU * 3.0 / 16.0),
            pitch_lower_limit: Some(TAU * 1.0 / 16.0),
            zoom_upper_limit: Some(200.0),
            zoom_lower_limit: Some(1.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.0,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // // Change the controls (these match Blender)
            // button_orbit: MouseButton::Middle,
            // button_pan: MouseButton::Middle,
            // modifier_pan: Some(KeyCode::ShiftLeft),

            // Reverse the zoom direction
            reversed_zoom: true,
            // touch_enabled: true,
            ..default()
        },
        Velocity(0.0, 0.0),
        Animator::new(tween),
        CameraMarker,
    ));
}

fn smooth_moving(
    mut camera: Query<(&mut Velocity, &mut Animator<Velocity>), With<CameraMarker>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventReader<KeyboardInput>,
) {
    let v = 20.0;
    for (mut velocity, mut animator) in camera.iter_mut() {
        let mut delta_x = Vec3::ZERO;
        let mut delta_z = Vec3::ZERO;
        for input in key_input.get_pressed().into_iter() {
            match input {
                KeyCode::KeyD => delta_x += Vec3::X,
                KeyCode::KeyA => delta_x += Vec3::NEG_X,
                KeyCode::KeyW => delta_z += Vec3::NEG_Z,
                KeyCode::KeyS => delta_z += Vec3::Z,
                _ => {}
            }
        }
        if input_events.read().any(|input| {
            [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD].contains(&input.key_code)
        }) {
            animator.set_tweenable(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_millis(300),
                CameraVelocityLens {
                    start: velocity.clone(),
                    end: Velocity(v * delta_x.x, v * delta_z.z),
                },
            ));
        }
    }
}
fn move_camera(
    mut camera: Query<(&mut Transform, &mut PanOrbitCamera, &mut Velocity), With<CameraMarker>>,
    time: Res<Time>,
) {
    for (mut transform, mut pan_orbit, mut velocity) in camera.iter_mut() {
        let delta_translation =
            transform.rotation * Vec3::new(1., 0., 0.) * time.delta_seconds() * velocity.0
                + (transform.rotation * Vec3::new(0., 0., 1.) * time.delta_seconds() * velocity.1)
                    .mul_add(Vec3::new(1.0, 0.0, 1.0), Vec3::ZERO);

        transform.translation += delta_translation;
        pan_orbit.target_focus += delta_translation;
    }
}
