use bevy::prelude::*;

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
        app.configure_sets(
            Startup,
            (InitProcess::SpawnEntity, InitProcess::InputManager).chain(),
        )
        .configure_sets(
            Update,
            (
                GameProcess::Input,
                GameProcess::MainCharacterUpdate,
                GameProcess::SoilUpdate,
            )
                .chain(),
        );
    }
}
