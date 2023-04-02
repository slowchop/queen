use crate::game::positions::SideIPos;
use crate::game::SIDE_CELL_SIZE;
use bevy::prelude::*;
use bevy::utils::petgraph::algo::astar;
use bevy::utils::petgraph::prelude::{EdgeRef, UnGraphMap};
use bevy_prototype_debug_lines::DebugLines;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SideMapGraph(UnGraphMap<SideIPos, u64>);

impl SideMapGraph {
    pub fn new() -> Self {
        Self(UnGraphMap::with_capacity(10_000, 40_000))
    }
}

impl From<UnGraphMap<SideIPos, u64>> for SideMapGraph {
    fn from(graph: UnGraphMap<SideIPos, u64>) -> Self {
        Self(graph)
    }
}

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

pub fn needs_path(
    graph: Res<SideMapGraph>,
    mut debug_lines: ResMut<DebugLines>,
    mut query: Query<&mut Path>,
) {
    for mut path in query.iter_mut() {
        let Path::NeedsPath(goal) = &*path else {
            continue;
        };

        let result = astar(
            &**graph,
            SideIPos::new(0, 0),
            |finish| finish == *goal,
            |e| *e.weight(),
            |z| (*z - **goal).as_vec2().length() as u64,
        );

        if let Some((_, path)) = result {
            for (a, b) in path.windows(2).map(|w| (w[0], w[1])) {
                let a = a.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
                let b = b.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
                debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 100.0, Color::LIME_GREEN);
            }
        }
    }
}
