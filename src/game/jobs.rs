use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashMap;

/// Contains all the jobs available for ants (and the queen).
///
/// A job will be removed from this list when it is assigned to an ant, and returned if the job is
/// cancelled.
#[derive(Resource, Deref, DerefMut, Default)]
pub struct Jobs(HashMap<SideIPos, Job>);

pub enum Job {
    Dig,
}

#[derive(Component)]
pub enum Assignment {
    None,
    Job(Job),
}
