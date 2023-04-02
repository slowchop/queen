use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::petgraph::prelude::UnGraphMap;

pub struct SideMapGraph(UnGraphMap::<SideIPos, u64>::with_capacity(1_000, 4_000));

#[derive(Component)]
pub enum Path {
    None,
    /// Need to calculate the path for this target destination.
    NeedsPath(SideIPos),
    /// We have a path and are progressing towards the target.
    Progress(PathProgress),
}

pub struct PathProgress {
    remaining_steps: Vec<SideIPos>,
}

pub fn needs_path(mut query: Query<&mut Path>) {
    for mut path in query.iter_mut() {
        let Path::NeedsPath(target) = &*path else {
            continue;
        };

        // astar
    }
}
