use crate::game::SideCell;
use bevy::prelude::*;

#[derive(Component)]
pub enum Path {
    None,
    /// Need to calculate the path for this target destination.
    NeedsPath(SideCell),
    /// We have a path and are progressing towards the target.
    Progress(PathProgress),
}

pub struct PathProgress {
    remaining_steps: Vec<SideCell>,
}

pub fn needs_path(mut query: Query<&mut Path>) {
    for mut path in query.iter_mut() {
        let Path::NeedsPath(target) = &*path else {
            continue;
        };

        // astar
    }
}
