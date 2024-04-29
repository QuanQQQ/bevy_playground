use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
mod component;
mod constants;
mod plugin;
mod resource;
mod states;
mod system_sets;
mod ui;
mod utils;

use component::*;
use plugin::*;
use resource::*;
use seldom_state::StateMachinePlugin;
use states::StatesPlugin;
use system_sets::*;
use ui::*;
use utils::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(StatesPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(ActionPlugin)
        .add_systems(Startup, init)
        .add_plugins(Camera2DPlugin::<FocusCamera>::with_order(0))
        .add_plugins(Camera2DPlugin::<StaticCamera>::with_order(1))
        .add_plugins(WorldPlugin)
        .add_plugins(MainCharacterPlugin)
        .add_plugins(ConfigureSetPlugin)
        .add_plugins(SoilPlugin)
        .add_plugins(StateMachinePlugin)
        .add_plugins(CustomDebugPlugin)
        .register_type::<Species>()
        .register_type::<Towards>()
        .run();
}

fn init(mut query: Query<&mut Window>) {
    for mut window in &mut query {
        window.resolution = WindowResolution::new(1920.0, 1080.0);
        window.resizable = false;
    }
}
