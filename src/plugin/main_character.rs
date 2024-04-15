use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_tweening::*;
use bevy_xpbd_2d::prelude::*;

use crate::{move_2d, CameraTarget, Controllable, Velocity};

pub struct MainCharacterPlugin;

impl Plugin for MainCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_character);
    }
}

fn move_character(
    mut mc: Query<&mut LinearVelocity, (With<Controllable>, With<CameraTarget>)>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut velocity = mc.get_single_mut().unwrap();
    let v: f32 = 2.00;
    let mut x_delta = 0.0_f32;
    let mut y_delta = 0.0_f32;

    for input in inputs.get_pressed().into_iter() {
        match input {
            KeyCode::KeyW => y_delta += 1.0,
            KeyCode::KeyS => y_delta -= 1.0,
            KeyCode::KeyA => x_delta -= 1.0,
            KeyCode::KeyD => x_delta += 1.0,
            _ => {}
        }
    }
    velocity.x += x_delta * v;
    velocity.y += y_delta * v;

    // transform.translation += move_2d(velocity, animator, time, inputs, input_event, 100.0);
}
