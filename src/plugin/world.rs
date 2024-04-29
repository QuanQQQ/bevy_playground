use crate::{
    constants::{
        CHARACTOR_HEIGHT, CHARACTOR_WIDTH, FIRST_LAYER, MAP_HEIGHT, MAP_WIDTH, SECOND_LAYER,
        TILE_SIZE,
    },
    states::MyAppState,
    CameraTarget, Controllable, Ground, InitProcess, OrganismBundle, Soil, Species, TransformInMap,
    Unreclaimed,
};
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
};
use bevy_entitiles::{prelude::*, tilemap::EntiTilesTilemapPlugin, EntiTilesPlugin};
use bevy_xpbd_2d::{components::RigidBody, plugins::collision::Collider};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(MyAppState::InGame),
            (spawn_town, spawn_main_character)
                .chain()
                .in_set(InitProcess::SpawnEntity),
        );
    }
}

fn spawn_town(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Plane3d {
                        normal: Direction3d::from_xyz(0.0, 0.0, 1.0).unwrap(),
                    }
                    .mesh()
                    .size(MAP_WIDTH as f32 * TILE_SIZE, MAP_HEIGHT as f32 * TILE_SIZE),
                ),
            ),
            transform: Transform::from_translation(Vec3::new(
                MAP_WIDTH as f32 * TILE_SIZE / 2.,
                MAP_HEIGHT as f32 * TILE_SIZE / 2.,
                -1.,
            )),
            material: color_materials.add(Color::hsl(0.3, 0.5, 0.7)),
            ..default()
        },
        Ground,
        FIRST_LAYER,
    ));

    for i in 1..20 {
        for j in 1..20 {
            commands
                .spawn((
                    Soil::default().with_transform(Transform::from_map_pos(i, j, 0.)),
                    Unreclaimed,
                ))
                .with_children(|parent| {
                    parent.spawn((MaterialMesh2dBundle {
                        material: color_materials.add(Color::rgb(0.23, 0.52, 0.95)),
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::from_size(Vec2::splat(TILE_SIZE))),
                        ),
                        ..default()
                    },));
                    parent.spawn((Text2dBundle {
                        text: Text::from_section("Soil", TextStyle::default())
                            .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..default()
                    },));
                });
        }
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("animal/chick.png"),
            transform: Transform::from_map_pos(25, 25, 0.).with_scale(Vec3::splat(0.2)),
            ..default()
        },
        OrganismBundle::animal().collider_radius(TILE_SIZE),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("animal/chick.png"),
            transform: Transform::from_map_pos(26, 26, 0.).with_scale(Vec3::splat(0.2)),
            ..default()
        },
        OrganismBundle::animal()
            .collider_radius(TILE_SIZE)
            .with_rigid(RigidBody::Static),
    ));
}

fn spawn_main_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_mat: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            OrganismBundle::human()
                .with_collider(Collider::rectangle(CHARACTOR_WIDTH, CHARACTOR_HEIGHT)),
            SpatialBundle::from_transform(Transform::from_map_pos(1, 1, 1.)),
            Controllable,
            CameraTarget,
        ))
        .with_children(|parent| {
            parent.spawn((MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::new(
                    CHARACTOR_WIDTH,
                    CHARACTOR_HEIGHT,
                )))),
                material: color_mat.add(Color::GRAY),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },));
            parent.spawn((Text2dBundle {
                text: Text::from_section("down", TextStyle::default())
                    .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },));
        });
}
