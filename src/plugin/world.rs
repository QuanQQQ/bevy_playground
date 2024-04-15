use crate::{CameraTarget, Controllable, Ground, OrganismBundle, Species};
use bevy::{
    ecs::entity::Entities,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_entitiles::{prelude::*, tilemap::EntiTilesTilemapPlugin, EntiTilesPlugin};
use bevy_xpbd_2d::components::RigidBody;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (spawn_town, spawn_main_character).chain());
    }
}

fn spawn_town(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        21,
        16,
        None,
        None,
    ));
    let ground = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Plane3d {
                            normal: Direction3d::from_xyz(0.0, 0.0, 1.0).unwrap(),
                        }
                        .mesh()
                        .size(20000.0, 20000.0),
                    ),
                ),

                material: color_materials.add(Color::hsl(0.3, 0.5, 0.7)),
                ..default()
            },
            Ground,
        ))
        .with_children(|parent| {
            parent.spawn(
                (SpriteSheetBundle {
                    texture: asset_server.load("ground_grasss.png"),
                    atlas: TextureAtlas {
                        layout: layout,
                        index: 1,
                    },
                    transform: Transform::from_xyz(-20.0, 0.0, -20.0).with_scale(Vec3::splat(3.0)),
                    ..default()
                }),
            );
        })
        .id();

    let chick = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("animal/chick.png"),
                transform: Transform::from_xyz(-20.0, 0.0, 1.0).with_scale(Vec3::splat(0.2)),
                ..default()
            },
            OrganismBundle::animal(),
        ))
        .id();

    commands.entity(ground).add_child(chick);

    let chick = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("animal/chick.png"),
                transform: Transform::from_xyz(50.0, -50.0, 1.0).with_scale(Vec3::splat(0.2)),
                ..default()
            },
            OrganismBundle::animal().with_rigid(RigidBody::Static),
        ))
        .id();

    commands.entity(ground).add_child(chick);
}

fn spawn_main_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ground: Query<Entity, With<Ground>>,
) {
    let ground = &ground.get_single_mut().unwrap();
    let main_character = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("png/head.png"),
                transform: Transform::from_xyz(100.0, 100.0, 1.0).with_scale(Vec3::splat(0.4)),
                ..default()
            },
            OrganismBundle::default().collider_density(20.0),
            Controllable,
            CameraTarget,
        ))
        .id();
    commands.entity(*ground).add_child(main_character);
}
