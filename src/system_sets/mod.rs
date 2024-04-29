use bevy::prelude::*;

use crate::states::MyAppState;

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MainMenu;

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InitProcess {
    SpawnEntity,
    InputManager,
}

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameProcess {
    Input,
    MainCharacterUpdate,
    SoilUpdate,
}

pub struct ConfigureSetPlugin;
impl Plugin for ConfigureSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MainMenu.run_if(in_state(MyAppState::MainMenu)))
            .configure_sets(
                OnEnter(MyAppState::InGame),
                (InitProcess::SpawnEntity, InitProcess::InputManager)
                    .chain()
                    .run_if(in_state(MyAppState::InGame)),
            )
            .configure_sets(
                Update,
                (
                    GameProcess::Input,
                    GameProcess::MainCharacterUpdate,
                    GameProcess::SoilUpdate,
                )
                    .chain()
                    .run_if(in_state(MyAppState::InGame)),
            );
    }
}
