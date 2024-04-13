use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};

use bevy_tweening::*;

mod component;
mod plugin;
mod resource;

use component::Velocity;
use plugin::*;
use resource::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_plugins(CameraPlugin)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .add_systems(Startup, init)
        .add_systems(
            Update,
            component_animator_system::<Velocity>.in_set(AnimationSystem::AnimationUpdate),
        )
        .add_systems(Startup, cursor_grab)
        .run();
}

fn init(
    mut commands: Commands,
    mut query: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut window in &mut query {
        window.resolution = WindowResolution::new(1920.0, 1080.0);
        window.resizable = false;
    }
    add_shapes(&mut commands, meshes, materials);
}
const X_EXTENT: f32 = 600.;
fn add_shapes(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = vec![
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 25.0))),
    ];
    let len_of_shapes = shapes.len();
    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / len_of_shapes as f32, 0.95, 0.7);
        commands.spawn(
            (MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    -X_EXTENT / 2. + i as f32 / (len_of_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
                ..default()
            }),
        );
    }
}

use bevy::window::{CursorGrabMode, PrimaryWindow};

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    dbg!(&primary_window.cursor.grab_mode);
    // primary_window.cursor.grab_mode = CursorGrabMode::Confined;
}
