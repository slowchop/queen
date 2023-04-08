use crate::game::food::{FeedEvent, FoodState};
use crate::game::map::UpdateFoodRenderingEvent;
use crate::game::pathfinding::Path;
use crate::game::positions::SideIPos;
use crate::game::simple_brain::{Idea, Sequence};
use crate::game::time::GameTime;
use bevy::ecs::system::EntityCommands;
use bevy::log::{error, info, warn};
use bevy::prelude::*;
use std::time::Duration;

pub fn new_eat_food_steps() -> Sequence {
    let mut steps = Sequence::new();
    steps.push(Action::SetPathToStoredFoodAction);
    steps.push(Action::PathfindingAction);
    steps.push(Action::EatAction);
    steps
}

#[derive(Debug)]
pub enum Action {
    SetPathToStoredFoodAction,
    PathfindingAction,
    EatAction,
    // SetPAthToRandomOutsideAction,
    // MapTransitionAction,
}

impl Action {
    pub fn insert(&self, ec: &mut EntityCommands) {
        match self {
            Action::SetPathToStoredFoodAction => ec.insert(SetPathToStoredFoodAction2),
            Action::EatAction => ec.insert(EatAction2::default()),
            Action::PathfindingAction => ec.insert(PathfindingAction2),
        };
        ()
    }

    pub fn remove(&self, ec: &mut EntityCommands) {
        match self {
            Action::SetPathToStoredFoodAction => ec.remove::<SetPathToStoredFoodAction2>(),
            Action::EatAction => ec.remove::<EatAction2>(),
            Action::PathfindingAction => ec.remove::<PathfindingAction2>(),
        };
        ()
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct EatAction2(Option<EatActionInner>);

pub struct EatActionInner {
    finish_eating_at: Duration,
}

#[derive(Component)]
pub struct SetPathToStoredFoodAction2;

pub fn set_path_to_stored_food_action_2(
    food_state: Res<FoodState>,
    mut query: Query<(&mut Idea, &mut Path), With<SetPathToStoredFoodAction2>>,
) {
    for (mut idea, mut path) in &mut query {
        let Some(target) = food_state.find_destination_to_take_food() else {
            warn!("No food to take");
            idea.abort();
            continue;
        };

        info!("------ Pathing to stored food at {:?}", target);
        path.set_target(target);

        idea.next_step();
    }
}

pub fn eat_action_2(
    time: Res<GameTime>,
    mut food_state: ResMut<FoodState>,
    mut query: Query<(Entity, &mut Idea, &mut EatAction2, &Transform)>,
    mut feed_writer: EventWriter<FeedEvent>,
    mut update_food_rendering_writer: EventWriter<UpdateFoodRenderingEvent>,
) {
    for (entity, mut idea, mut action, transform) in &mut query {
        if action.is_none() {
            let pos = SideIPos::from(transform);

            let Some(carrying_food) = food_state.take_food_from_position(pos, 1f32) else {
                warn!("Tried to eat food but there was none.");
                idea.abort();
                continue;
            };

            info!(?carrying_food, "Eating food");

            feed_writer.send(FeedEvent {
                target: entity,
                carrying_food,
            });

            update_food_rendering_writer.send(UpdateFoodRenderingEvent(pos));

            **action = Some(EatActionInner {
                finish_eating_at: time.since_startup() + Duration::from_secs(5),
            });
        };

        let inner = action.0.as_ref().unwrap();
        if time.since_startup() >= inner.finish_eating_at {
            info!("Finished eating food");
            idea.next_step();
        }
    }
}

#[derive(Component, Default)]
pub struct PathfindingAction2;

pub fn pathfinding_action_2(mut query: Query<(&mut Idea, &mut Path), With<PathfindingAction2>>) {
    for (mut idea, mut path) in &mut query {
        info!(?path, "---------------------...");

        if path.is_progressing() {
            info!("Progressing path");
            continue;
        }

        info!("------- end");

        if path.did_complete() {
            idea.next_step();
        } else {
            error!("Path failed");
            idea.abort();
        }
    }
}
