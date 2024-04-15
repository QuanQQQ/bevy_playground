use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::*;
use bevy_xpbd_2d::prelude::*;
mod component;
mod plugin;
mod resource;
mod utils;

use component::*;
use plugin::*;
use resource::*;
use utils::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, init)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(TweeningPlugin)
        .add_plugins(Camera2DPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(WorldPlugin)
        .add_plugins(MainCharacterPlugin)
        .register_type::<Species>()
        .run();
}

fn init(mut query: Query<&mut Window>) {
    for mut window in &mut query {
        window.resolution = WindowResolution::new(1920.0, 1080.0);
        window.resizable = false;
    }
}
