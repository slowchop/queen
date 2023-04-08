use crate::game::new_brain::Action;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SimpleBrainSet {
    Actions,
    AssignComponents,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IdeaState {
    Prepare(usize),
    Executing(usize),
    Done,
    Aborting(usize),
    Aborted,
}

pub fn assign_step_components(mut commands: Commands, mut executing: Query<(Entity, &mut Idea)>) {
    for (entity, mut executing) in &mut executing {
        match &executing.state {
            IdeaState::Prepare(step) => {
                if *step > 0 {
                    let last_step = step - 1;
                    let action = &executing.steps[last_step];
                    info!(?action, "Removing component");
                    action.remove(&mut commands.entity(entity));
                }

                if *step >= executing.steps.len() {
                    info!("No more steps.");
                    executing.state = IdeaState::Done;
                    continue;
                }

                let action = &executing.steps[*step];
                info!(?action, "Inserting component");
                action.insert(&mut commands.entity(entity));
                executing.state = IdeaState::Executing(*step);
                info!(?executing.state, "State");
            }
            IdeaState::Aborting(step) => {
                let action = &executing.steps[*step];
                info!(?action, "Removing component because we're aborting.");
                action.remove(&mut commands.entity(entity));
                executing.state = IdeaState::Aborted;
            }
            _ => (),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Sequence(Vec<Action>);

impl Sequence {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Component)]
pub struct Idea {
    state: IdeaState,
    steps: Sequence,
}

impl Idea {
    pub fn abort(&mut self) {
        let current_step = match self.state {
            IdeaState::Executing(step) => step,
            _ => {
                warn!("Invalid state: {:?}", self.state);
                return;
            }
        };
        self.state = IdeaState::Aborting(current_step);
    }

    pub fn next_step(&mut self) {
        match self.state {
            IdeaState::Executing(step) => {
                // OK to overflow. assign_step_components will handle it.
                self.state = IdeaState::Prepare(step + 1);
                debug!(?self.state, "Next step");
            }
            _ => {
                panic!("Invalid state: {:?}", self.state);
            }
        }
    }
}

impl From<Sequence> for Idea {
    fn from(steps: Sequence) -> Self {
        Self {
            state: IdeaState::Prepare(0),
            steps,
        }
    }
}
