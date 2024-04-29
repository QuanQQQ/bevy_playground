use crate::{states::MyAppState, Controllable, InitProcess, Tool, Velocity};
use bevy::{prelude::*, transform::commands};
use leafwing_input_manager::{
    input_map::InputMap, plugin::InputManagerPlugin, Actionlike, InputManagerBundle,
};

#[derive(Actionlike, Clone, Copy, Debug, Reflect, Hash, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Reclaim,
    Plant,
}
impl Action {
    pub const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
    pub const USE: [Self; 2] = [Self::Reclaim, Self::Plant];
    pub fn direction(self: &Self) -> Direction2d {
        match self {
            Self::Up => Direction2d::Y,
            Self::Down => Direction2d::NEG_Y,
            Self::Left => Direction2d::NEG_X,
            Self::Right => Direction2d::X,
            _ => Direction2d::from_xy(0., 0.).unwrap(),
        }
    }
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                OnEnter(MyAppState::InGame),
                init_action_for_main.in_set(InitProcess::InputManager),
            );
    }
}

fn init_action_for_main(mut commands: Commands, mut controled: Query<Entity, With<Controllable>>) {
    let controled = controled.get_single_mut().unwrap();
    let mut inputs_map = InputMap::default();
    inputs_map.insert(Action::Up, KeyCode::KeyW);
    inputs_map.insert(Action::Down, KeyCode::KeyS);
    inputs_map.insert(Action::Left, KeyCode::KeyA);
    inputs_map.insert(Action::Right, KeyCode::KeyD);
    inputs_map.insert(Action::Reclaim, MouseButton::Left);
    inputs_map.insert(Action::Plant, MouseButton::Right);
    commands
        .entity(controled)
        .insert(InputManagerBundle::with_map(inputs_map));
}
