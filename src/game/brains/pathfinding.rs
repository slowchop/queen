use crate::game::food::{AssignedFoodId, FoodState};
use crate::game::map::ExitPositions;
use crate::game::pathfinding::Path;
use crate::game::plugin::PlayerState;
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use bevy::log::{error, info, warn};
use bevy::prelude::{Component, Query, Res, Transform, With};
use big_brain::actions::ActionState;
use big_brain::prelude::{ActionBuilder, Actor};
use rand::prelude::SliceRandom;

// /// Actor is on a path. This action is to follow the path and finish when the path is finished.
// #[derive(Clone, Component, Debug, ActionBuilder)]
// pub struct PathfindingAction;
//
// pub fn pathfinding_action(
//     mut ants: Query<&mut Path>,
//     mut query: Query<(&Actor, &mut ActionState), With<PathfindingAction>>,
// ) {
//     for (Actor(actor), mut state) in query.iter_mut() {
//         let Ok(mut path) = ants.get_mut(*actor) else {
//             warn!("No path for actor {:?}", actor);
//             continue;
//         };
//
//         match *state {
//             ActionState::Requested => {
//                 // We expect to already have a path set.
//                 if !path.is_progressing() {
//                     error!("Path expected to be set in pathfinding_action but it wasn't");
//                     *state = ActionState::Failure;
//                     continue;
//                 }
//
//                 *state = ActionState::Executing;
//             }
//             ActionState::Executing => {
//                 if path.did_complete() {
//                     *state = ActionState::Success;
//                 } else if path.did_fail() {
//                     error!("Path failed for actor {:?} in pathfinding_action", actor);
//                     *state = ActionState::Failure;
//                 }
//             }
//             _ => {}
//         }
//     }
// }

/// A scout or cargo ant needs to find a destination to the outside.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToRandomOutsideAction;

pub fn set_path_to_outside_action(
    exit_positions: Res<ExitPositions>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToRandomOutsideAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        if *state != ActionState::Requested {
            continue;
        }

        // Find a random exit position
        // TODO: Have an optional exit position to go to for cargo ants.
        let exit_position = exit_positions
            .choose(&mut rand::thread_rng())
            .expect("No exit positions");

        path.set_target(*exit_position);

        *state = ActionState::Success;
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToFoodStorageAction;

pub fn set_path_to_food_storage_action(
    food_state: Res<FoodState>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToFoodStorageAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        if *state != ActionState::Requested {
            continue;
        }

        let target = food_state.find_destination_to_place_food();
        path.set_target(target);

        *state = ActionState::Success;
    }
}

/// A cargo ant needs to find a destination to the outside.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToDiscoveredFoodAction;

pub fn set_path_and_assign_food_to_discovered_food_action(
    food_state: Res<FoodState>,
    mut ants: Query<(&mut Path, &mut AssignedFoodId)>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToDiscoveredFoodAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok((mut path, mut assigned_food_id)) = ants.get_mut(*actor) else {
            error!("No path for actor {:?}", actor);
            continue;
        };

        if *state != ActionState::Requested {
            continue;
        }

        let Some(food_id) = food_state.random_food_source() else {
                    warn!("No random food source found");
                    *state = ActionState::Failure;
                    continue;
                };
        let Some(exit_position) = food_state.position_of_food_source(food_id) else {
                    warn!("No position for food {:?}", assigned_food_id);
                    *state = ActionState::Failure;
                    continue;
                };

        **assigned_food_id = Some(food_id);
        path.set_target(exit_position);

        *state = ActionState::Success;
    }
}

/// Move to The Queen!
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToQueenAction;

pub fn set_path_to_queen_action(
    queen: Query<&Transform, With<Queen>>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToQueenAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!(?actor, "No path found.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                let queen_transform = queen.single();
                let queen_position = SideIPos::from(queen_transform);
                path.set_target(queen_position);

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

// #[derive(Clone, Component, Debug, ActionBuilder)]
// pub struct SetPathToStoredFoodAction;
//
// pub fn set_path_to_stored_food_action(
//     food_state: Res<FoodState>,
//     mut ants: Query<&mut Path>,
//     mut query: Query<(&Actor, &mut ActionState), With<SetPathToStoredFoodAction>>,
// ) {
//     for (Actor(actor), mut state) in query.iter_mut() {
//         let Ok(mut path) = ants.get_mut(*actor) else {
//             warn!("No path for actor {:?}", actor);
//             continue;
//         };
//
//         if *state != ActionState::Requested {
//             continue;
//         }
//
//         let Some(target) = food_state.find_destination_to_take_food() else {
//             warn!("No food to take");
//             *state = ActionState::Failure;
//             continue;
//         };
//
//         info!("------ Pathing to stored food at {:?}", target);
//         path.set_target(target);
//
//         *state = ActionState::Success;
//     }
// }
