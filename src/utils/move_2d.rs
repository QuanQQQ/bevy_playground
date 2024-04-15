use std::time::Duration;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_tweening::*;

use crate::{CameraVelocityLens, Velocity};

pub fn move_2d<'w>(
    // mut camera_trans: Query<
    //     (&mut Transform, &mut Velocity, &mut Animator<Velocity>),
    //     With<CameraMarker>,
    // >,
    velocity: Mut<'w, Velocity>,
    mut animator: Mut<'w, Animator<Velocity>>,
    time: f32,
    inputs: Res<ButtonInput<KeyCode>>,
    mut inputs_event: EventReader<KeyboardInput>,
    v: f32,
) -> Vec3 {
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
                start: velocity.clone(),
                end: Velocity(x_delta * v, y_delta * v),
            },
        ));
    }

    Vec3::new(velocity.0 * time, velocity.1 * time, 0.0)
}
