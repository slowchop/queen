use crate::game::map::SIDE_CELL_SIZE;
use crate::game::positions::SideIPos;
use crate::game::time::GameTime;
use bevy::prelude::*;
use bevy::utils::petgraph::algo::astar;
use bevy::utils::petgraph::prelude::{EdgeRef, UnGraphMap};
use bevy_prototype_debug_lines::DebugLines;

#[derive(Debug)]
pub struct VisitedNodeEvent {
    pub creature_entity: Entity,
    pub position: SideIPos,
    pub is_final: bool,
}

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
    /// Made it
    Completed(SideIPos),
    /// Could not create a path to the target.
    Failed(SideIPos),
}

impl Path {
    pub fn set_target(&mut self, target: SideIPos) {
        *self = Path::NeedsPath(target);
    }

    pub fn cancel(&mut self) {
        *self = Path::None;
    }

    pub fn is_progressing(&self) -> bool {
        matches!(self, Path::Progress(_)) || matches!(self, Path::NeedsPath(_))
    }

    pub fn get_completed_position(&self) -> Option<SideIPos> {
        match self {
            Path::Completed(pos) => Some(*pos),
            _ => None,
        }
    }

    pub fn did_fail(&self) -> Option<SideIPos> {
        match self {
            Path::Failed(pos) => Some(*pos),
            _ => None,
        }
    }
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

        let start = SideIPos::from(transform);

        if start == *goal {
            *path = Path::Completed(*goal);
            continue;
        }

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
                debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 1.0, Color::LIME_GREEN);
            }
            *path = Path::Progress(PathProgress {
                remaining_steps: found_path,
            });
        } else {
            warn!("No path found!");
            *path = Path::Failed(*goal);
        }
    }
}

pub fn move_along_path(
    time: Res<GameTime>,
    mut query: Query<(Entity, &mut Path, &mut Transform)>,
    mut visited_event_writer: EventWriter<VisitedNodeEvent>,
) {
    for (entity, mut path, mut transform) in query.iter_mut() {
        let Path::Progress(progress) = &mut *path else {
            continue;
        };

        let PathProgress { remaining_steps } = progress;

        let next_step = remaining_steps.get(0);
        let Some(next_step) = next_step else {
            *path = Path::None;
            continue;
        };

        let z = transform.translation.z;
        // "Speed"
        let mut step_distance = 16f32 * time.delta_seconds();
        let mut next_step_position = next_step.to_world_vec2();
        let mut current_position = transform.translation.truncate();
        let distance = (current_position - next_step_position).length();

        if distance <= step_distance {
            // We're about to hit the next waypoint. Let's move there and remove it from the path,
            // and then subtract it from the step_distance.
            current_position = next_step_position;
            step_distance -= distance;
            transform.translation = current_position.extend(z);

            // Emit an event for the visited node.
            let event = VisitedNodeEvent {
                creature_entity: entity,
                position: *next_step,
                is_final: remaining_steps.len() == 1,
            };
            visited_event_writer.send(event);

            // Remove the current step from the path.
            let popped_step = remaining_steps.remove(0);
            let Some(next_step) = remaining_steps.get(0) else {
                *path = Path::Completed(popped_step);
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
