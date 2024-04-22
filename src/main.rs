use bevy::{prelude::*, window::WindowResolution};
use bevy_editor_pls::EditorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
mod component;
mod constants;
mod plugin;
mod resource;
mod system_sets;
mod utils;

use component::*;
use plugin::*;
use resource::*;
use seldom_state::StateMachinePlugin;
use system_sets::*;
use utils::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(ActionPlugin)
        .add_systems(Startup, init)
        .add_plugins(Camera2DPlugin::<FocusCamera>::default())
        .add_plugins(WorldInspectorPlugin::default())
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
