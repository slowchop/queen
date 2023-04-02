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
    mut query: Query<(&mut Path, &Transform)>,
) {
    for (mut path, transform) in query.iter_mut() {
        let Path::NeedsPath(goal) = &*path else {
            continue;
        };

        let start = SideIPos::from(&transform.translation);

        let result = astar(
            &**graph,
            start,
            |finish| finish == *goal,
            |e| *e.weight(),
            |z| (*z - **goal).as_vec2().length() as u64,
        );

        if let Some((_, found_path)) = result {
            for (a, b) in found_path.windows(2).map(|w| (w[0], w[1])) {
                let a = a.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
                let b = b.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
                debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 100.0, Color::LIME_GREEN);
            }
            *path = Path::Progress(PathProgress {
                remaining_steps: found_path,
            });
        } else {
            warn!("No path found!");
            *path = Path::None;
        }
    }
}

pub fn move_along_path(time: Res<Time>, mut query: Query<(&mut Path, &mut Transform)>) {
    for (mut path, mut transform) in query.iter_mut() {
        let Path::Progress(progress) = &mut *path else {
            continue;
        };

        let PathProgress { remaining_steps } = progress;

        let next_step = remaining_steps.get(0);
        let Some(mut next_step) = next_step else {
            *path = Path::None;
            continue;
        };

        let z = transform.translation.z;
        let mut step_distance = 32f32 * time.delta_seconds();
        let mut next_step_position = next_step.to_world_vec2();
        let mut current_position = transform.translation.truncate();
        let distance = (current_position - next_step_position).length();

        if distance <= step_distance {
            // We're about to hit the next waypoint. Let's move there and remove it from the path,
            // and then subtract it from the step_distance.
            current_position = next_step_position;
            step_distance -= distance;
            transform.translation = current_position.extend(z);

            // Remove the current step from the path.
            remaining_steps.remove(0);
            let Some(next_step) = remaining_steps.get(0) else {
                *path = Path::None;
                continue;
            };

            // Recalculate the next step position.
            next_step_position = next_step.to_world_vec2();
        }

        let direction = (next_step_position - current_position).normalize_or_zero();
        current_position += direction * step_distance;

        transform.translation = current_position.extend(z);
    }
}
