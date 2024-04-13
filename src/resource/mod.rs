use bevy::{input::keyboard::KeyboardInput, prelude::*};
#[derive(Resource)]
pub struct GreetTimer(pub Timer);
