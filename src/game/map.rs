use crate::game::pathfinding::VisitedNodeEvent;
use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub const SIDE_CELL_SIZE: u8 = 16;

#[derive(Debug, Deref, DerefMut)]
pub struct CellChangedEvent(SideIPos);

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
            Some("dirt/rock.png".to_string())
        } else if self.amount_left() > 127 {
            Some("dirt/full.png".to_string())
        } else {
            Some("dirt/half.png".to_string())
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
        info!(?event.creature_entity, "Got visiting node event");
        let Some(entity) = side_map_pos_to_entities.get(&event.position) else {
            warn!(?event, "No CellContent in side_map_pos_to_entities.");
            continue;
        };

        let Ok(mut cell_content) = query.get_mut(*entity) else {
            warn!(?event, "Entity doesn't have cell content");
            continue;
        };

        info!(?event.creature_entity, "Digging dirt");
        cell_content.dig(255);
        cell_changed_writer.send(CellChangedEvent(event.position));
    }
}

pub fn detect_cell_content_changes_and_update_rendering(
    mut commands: Commands,
    mut side_map_pos_to_entities: ResMut<SideMapPosToEntities>,
    query: Query<(Entity, &SideIPos, &CellContent)>,
    mut cell_changed_reader: EventReader<CellChangedEvent>,
) {
    for event in cell_changed_reader.iter() {
        //     if !cell_content.is_empty() {
        //         continue;
        //     }

        if let Some(entity) = side_map_pos_to_entities.get(&*event) {
            info!(?entity, "Despawning old entity");
            commands.entity(*entity).despawn();
            // side_map_pos_to_entities.remove(&*event);
        } else {
            warn!(?event, "No entity to despawn");
        }
    }
}
