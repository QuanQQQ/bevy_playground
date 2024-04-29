use bevy::{input::keyboard::KeyboardInput, prelude::*};
#[derive(Resource)]
pub struct GreetTimer(pub Timer);

#[derive(Resource, Default, Debug)]
pub struct LoadingProcess {
    pub total: usize,
    pub done: usize,
}
