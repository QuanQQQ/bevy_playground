use bevy::prelude::*;
use bevy_entitiles::math::extension::F32Integerize;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::*;

pub struct MainCharacterPlugin;

impl Plugin for MainCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_character, use_tool));
    }
}
fn direction_to_towards(direction: Direction2d) -> Towards {
    if (direction.x > 0. && direction.y == 0.) {
        Towards::Right
    } else if direction.x < 0. && direction.y == 0. {
        Towards::Left
    } else if direction.x == 0. && direction.y > 0. {
        Towards::Up
    } else {
        Towards::Down
    }
}
fn move_character(
    mut mc: Query<
        (&mut LinearVelocity, &ActionState<Action>, &mut Towards),
        (With<Controllable>, With<CameraTarget>),
    >,
) {
    let (mut velocity, action_state, mut towards) = mc.get_single_mut().unwrap();
    let scale: f32 = 10.00;
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
    let mc_pos = transform.to_map_pos();
    let forward_dir = towards.to_direction2d();
    for action in Action::USE {
        if action_state.just_pressed(&action) {
            match action {
                Action::Reclaim => {
                    for (entity, transform) in unreclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();

                        if soil_pos
                            == (
                                mc_pos.0 + forward_dir.x.floor_to_i32(),
                                mc_pos.1 + forward_dir.y.floor_to_i32(),
                            )
                        {
                            dbg!(
                                soil_pos,
                                forward_dir.x.floor_to_i32(),
                                forward_dir.y.floor_to_i32(),
                                mc_pos.0 + forward_dir.x.floor_to_i32(),
                                mc_pos.1 + forward_dir.y.floor_to_i32(),
                            );
                            ew.send(SoilEvent::Reclaim(entity));
                            return;
                        }
                    }
                    for (entity, transform) in reclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos
                            == (
                                mc_pos.0 + forward_dir.x.floor_to_i32(),
                                mc_pos.1 + forward_dir.y.floor_to_i32(),
                            )
                        {
                            ew.send(SoilEvent::Unreclaim(entity));
                            return;
                        }
                    }
                }

                Action::Plant => {
                    for (entity, transform) in reclaimed_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos
                            == (
                                mc_pos.0 + forward_dir.x.floor_to_i32(),
                                mc_pos.1 + forward_dir.y.floor_to_i32(),
                            )
                        {
                            ew.send(SoilEvent::Plant(entity));
                            return;
                        }
                    }
                    for (entity, transform) in planted_soil.iter() {
                        let soil_pos = transform.to_map_pos();
                        if soil_pos
                            == (
                                mc_pos.0 + forward_dir.x.floor_to_i32(),
                                mc_pos.1 + forward_dir.y.floor_to_i32(),
                            )
                        {
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
