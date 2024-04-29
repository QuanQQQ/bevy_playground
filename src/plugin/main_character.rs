use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{utils::transform_in_map::*, *};

use self::constants::{CHARACTER_HEIGHT, CHARACTER_WIDTH, TILE_SIZE};

pub struct MainCharacterPlugin;

impl Plugin for MainCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_character, use_tool).in_set(GameProcess::MainCharacterUpdate),
        );
    }
}
fn direction_to_towards(direction: Direction2d) -> Towards {
    match (direction.x, direction.y) {
        (x, y) if x > 0. => Towards::Right,
        (x, y) if x < 0. => Towards::Left,
        (x, y) if y > 0. => Towards::Up,
        _ => Towards::Down,
    }
}
fn move_character(
    mut mc: Query<
        (&mut LinearVelocity, &ActionState<Action>, &mut Towards),
        (With<Controllable>, With<CameraTarget>),
    >,
) {
    let (mut velocity, action_state, mut towards) = mc.get_single_mut().unwrap();
    let scale: f32 = 40.00;
    for action in Action::DIRECTIONS {
        if action_state.pressed(&action) {
            let direction = action.direction();
            *towards = direction_to_towards(direction);
            velocity.x += direction.x * scale;
            velocity.y += direction.y * scale;
        }
    }
}

fn use_tool(
    mut mc: Query<
        (&ActionState<Action>, &Transform, &Towards),
        (With<Controllable>, With<CameraTarget>),
    >,
    mut ew: EventWriter<SoilEvent>,
    unreclaimed_soil: Query<(Entity, &Transform), With<Unreclaimed>>,
    reclaimed_soil: Query<(Entity, &Transform), With<Reclaimed>>,
    planted_soil: Query<(Entity, &Transform), With<Planted>>,
) {
    let (action_state, transform, towards) = mc
        .get_single_mut()
        .expect("There should be exact one main charactor");
    let affect_trans = Transform::from_translation(
        transform.translation
            + match towards {
                Towards::Down => Vec3::new(0., -CHARACTER_HEIGHT / 4. - TILE_SIZE / 2., 0.),
                Towards::Up => Vec3::new(0., -CHARACTER_HEIGHT / 4. + TILE_SIZE / 2., 0.),
                Towards::Left => Vec3::new(
                    -CHARACTER_WIDTH / 2. - TILE_SIZE / 2.,
                    -CHARACTER_HEIGHT / 4.,
                    0.,
                ),
                Towards::Right => Vec3::new(
                    CHARACTER_WIDTH / 2. + TILE_SIZE / 2.,
                    -CHARACTER_HEIGHT / 4.,
                    0.,
                ),
            },
    );
    let affect_pos = affect_trans.to_map_pos();
    for action in Action::USE {
        if action_state.just_pressed(&action) {
            match action {
                Action::Reclaim => {
                    for (entity, transform) in unreclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();

                        if soil_pos == affect_pos {
                            ew.send(SoilEvent::Reclaim(entity));
                            return;
                        }
                    }
                    for (entity, transform) in reclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos == affect_pos {
                            ew.send(SoilEvent::Unreclaim(entity));
                            return;
                        }
                    }
                }

                Action::Plant => {
                    for (entity, transform) in reclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos == affect_pos {
                            ew.send(SoilEvent::Plant(entity));
                            return;
                        }
                    }
                    for (entity, transform) in planted_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos == affect_pos {
                            ew.send(SoilEvent::Unplant(entity));
                            return;
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
