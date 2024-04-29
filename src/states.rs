use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyAppState {
    #[default]
    LoadingScreen,
    MainMenu,
    InGame,
    SettingInGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Disabled,
    Main,
    Setting,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>();
        app.init_state::<MainMenuState>();
    }
}
