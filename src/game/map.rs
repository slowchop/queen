use crate::game::pathfinding::{SideMapGraph, VisitedNodeEvent};
use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::petgraph::prelude::EdgeRef;
use bevy::utils::petgraph::visit::IntoEdgeReferences;
use bevy::utils::HashMap;
use bevy_prototype_debug_lines::DebugLines;

pub const SIDE_CELL_SIZE: u8 = 32;

#[derive(Resource, Deref, DerefMut)]
pub struct ExitPositions(Vec<SideIPos>);

impl From<Vec<SideIPos>> for ExitPositions {
    fn from(exits: Vec<SideIPos>) -> Self {
        Self(exits)
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct CellChangedEvent(Entity);

/// The side view of the world. The idea is that if we have time we can do a top down view on the
/// surface of the world.
#[derive(Resource, Deref, DerefMut)]
pub struct SideMapPosToEntities(HashMap<SideIPos, Entity>);

impl From<HashMap<SideIPos, Entity>> for SideMapPosToEntities {
    fn from(map: HashMap<SideIPos, Entity>) -> Self {
        Self(map)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellType {
    // 0 means there's still one amount of dirt left before it's empty.
    Dirt(u8),
    Empty,
    // Impassable
    Rock,
}

/// A dirt block. The u8 is the amount of dirt. 0 is empty.
#[derive(Component, Copy, Clone, Debug)]
pub struct CellContent {
    /// Underground means creatures can walk on walls.
    underground: bool,
    cell_type: CellType,
}

impl CellContent {
    pub fn empty_air() -> Self {
        Self {
            underground: false,
            cell_type: CellType::Empty,
        }
    }

    pub fn empty_underground() -> Self {
        Self {
            underground: true,
            cell_type: CellType::Empty,
        }
    }

    pub fn random_dirt() -> Self {
        Self {
            underground: true,
            cell_type: CellType::Dirt(rand::random::<u8>()),
        }
    }

    pub fn dirt(amount: u8) -> Self {
        Self {
            underground: true,
            cell_type: CellType::Dirt(amount),
        }
    }

    pub fn rock(underground: bool) -> Self {
        Self {
            underground,
            cell_type: CellType::Rock,
        }
    }

    pub fn dig(&mut self, amount: u8) {
        if let CellType::Dirt(current) = self.cell_type {
            if current > 0 {
                self.cell_type = CellType::Dirt(current.saturating_sub(amount));

                if self.cell_type == CellType::Dirt(0) {
                    // We just dug the last bit of dirt.
                    self.cell_type = CellType::Empty;
                }
            } else {
                self.cell_type = CellType::Empty;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.cell_type, CellType::Empty)
    }

    pub fn is_rock(&self) -> bool {
        matches!(self.cell_type, CellType::Rock)
    }

    pub fn amount_left(&self) -> u8 {
        if let CellType::Dirt(amount) = self.cell_type {
            amount
        } else {
            0
        }
    }

    // A weight of zero breaks the pathfinding algorithm, so we use u16 and add 1.
    pub fn weight(&self) -> Option<u16> {
        if self.is_empty() {
            Some(1)
        } else if self.is_rock() {
            None
        } else {
            Some(self.amount_left() as u16 + 1)
        }
    }

    pub fn texture_path(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else if self.is_rock() {
            Some("cell/rock.png".to_string())
        } else if self.amount_left() > 127 {
            Some("cell/full.png".to_string())
        } else {
            Some("cell/half.png".to_string())
        }
    }
}

pub fn passive_dig_when_visiting_a_cell(
    side_map_pos_to_entities: Res<SideMapPosToEntities>,
    mut query: Query<&mut CellContent>,
    mut visited_node_reader: EventReader<VisitedNodeEvent>,
    mut cell_changed_writer: EventWriter<CellChangedEvent>,
) {
    for event in visited_node_reader.iter() {
        let Some(entity) = side_map_pos_to_entities.get(&event.position) else {
            warn!(?event, "No CellContent in side_map_pos_to_entities.");
            continue;
        };

        let Ok(mut cell_content) = query.get_mut(*entity) else {
            warn!(?event, "Entity doesn't have cell content");
            continue;
        };

        cell_content.dig(50);

        // TODO: "Move" the amount removed the the previous cell (and overflow outwards if that's not possible).

        cell_changed_writer.send(CellChangedEvent(*entity));
    }
}

pub fn detect_cell_content_changes_and_update_graph(
    mut debug_lines: ResMut<DebugLines>,
    mut graph: ResMut<SideMapGraph>,
    mut side_map_pos_to_entities: ResMut<SideMapPosToEntities>,
    mut query: Query<(&CellContent, &Transform)>,
    mut cell_changed_reader: EventReader<CellChangedEvent>,
) {
    for cell_changed_event in cell_changed_reader.iter() {
        let entity = **cell_changed_event;

        // Grab the CellContent for this entity.
        let Ok((cell, transform)) = query.get(entity) else {
            warn!(?cell_changed_event, "Could not find CellContent for entity");
            continue;
        };

        let pos = SideIPos::from(transform);
        let Some(a_weight) = cell.weight() else {
            warn!(?cell_changed_event, "Could not find weight for cell content");
            continue;
        };

        // Work out the neighbours and update the edge values
        for neighbour in pos.sides() {
            let Some(other_entity) = side_map_pos_to_entities.get(&neighbour) else {
                continue;
            };

            let Ok((other_cell, _)) = query.get(*other_entity) else {
                continue;
            };

            let Some(b_weight) = other_cell.weight() else {
                continue;
            };

            // TODO: This is similar to the map generation code, so we should probably factor it out.
            let weight = a_weight as u64 + b_weight as u64;
            // graph.add_edge(*pos, neighbour, weight as u64);
            graph.edge_weight_mut(pos, neighbour).map(|w| *w = weight);
        }
    }

    // Debug draw the graph grid
    // for edge in graph.edge_references() {
    //     let weight = edge.weight();
    //     let a = edge.source().to_world_vec2()
    //         + SIDE_CELL_SIZE as f32 / 2f32
    //         + rand::random::<f32>() * 5.1;
    //     let b = edge.target().to_world_vec2()
    //         + SIDE_CELL_SIZE as f32 / 2f32
    //         + rand::random::<f32>() * 5.1;
    //
    //     debug_lines.line_colored(
    //         a.extend(10f32),
    //         b.extend(10f32),
    //         0.0,
    //         Color::rgb(*weight as f32 / 255f32, 1f32 - *weight as f32 / 255f32, 0.0),
    //     );
    // }
}

pub fn detect_cell_content_changes_and_update_rendering(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &SideIPos, &CellContent)>,
    mut cell_changed_reader: EventReader<CellChangedEvent>,
) {
    for cell_changed_event in cell_changed_reader.iter() {
        let Ok((entity, side_map_pos, cell_content)) = query.get(**cell_changed_event) else {
            warn!(?cell_changed_event, "Could not find entity in query");
            continue;
        };

        let mut entity = commands.entity(entity);

        if let Some(texture_path) = cell_content.texture_path() {
            let image_handle: Handle<Image> = asset_server.load(texture_path);
            entity.insert(image_handle);
        } else {
            entity.remove::<Handle<Image>>();
        }
    }
}
