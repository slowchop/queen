use crate::game::ants::AntType;
use crate::game::hunger::Hunger;
use crate::game::map::ExitPositions;
use crate::game::pathfinding::Path;
use crate::game::positions::SideIPos;
use bevy::prelude::*;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;

/// A scout or cargo ant to leave the map for food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct LeaveMapAction;

pub fn leave_map_action(
    exit_positions: Res<ExitPositions>,
    mut ants: Query<(&mut Path, &mut Visibility), With<AntType>>,
    mut query: Query<(&Actor, &mut ActionState), With<LeaveMapAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok((mut path, mut visibility)) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                info!("Requested");

                // Find a random exit position
                // TODO: Have an optional exit position to go to for cargo ants.
                let exit_position = exit_positions
                    .choose(&mut rand::thread_rng())
                    .expect("No exit positions");

                path.set_target(*exit_position);

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Check if path is complete and is at the exit position.
                if path.get_completed_position().is_some() {
                    *visibility = Visibility::Hidden;
                    *state = ActionState::Success;
                }

                if path.did_fail() {
                    *state = ActionState::Failure;
                }
            }
            _ => {}
        }
    }
}

/// The ant is off the map going to get new food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OutsideMapGettingNewFoodAction;

pub fn outside_map_getting_new_food_action()

/// The ant is off the map going to get approved food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OutsideMapGettingApprovedFoodAction;

/// Move to The Queen!
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToQueenAction;

/// The scout ant is back on the map
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OfferNewFoodToQueenAction;

/// When hungry or needs food for the queen, move to some food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToFoodAction;

/// At should be at a food cell and will eat it.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct EatAction;

/// Should be at a food cell and will pick it up (to carry to the queen).
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct CarryFoodAction;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct HungryScorer;
