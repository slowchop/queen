use crate::game::animation::{AnimationIndices, AnimationTimer};
use crate::game::camera::CameraFocus;
use crate::game::eggs::Egg;
use crate::game::jobs::Assignment;
use crate::game::map::{CellContent, MapExit, SideMapPosToEntities, SIDE_CELL_SIZE};
use crate::game::pathfinding::{Path, SideMapGraph};
use crate::game::plugin::{Crawler, Hunger, PlayerState, Speed, ANT_Z, DIRT_Z, QUEEN_Z};
use crate::game::positions::SideIPos;
use crate::game::queen::{EggLaidEvent, Queen};
use bevy::asset::AssetServer;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::petgraph::algo::astar;
use bevy::utils::petgraph::prelude::*;
use bevy::utils::petgraph::visit::IntoEdgeReferences;
use bevy::utils::HashMap;
use bevy_prototype_debug_lines::DebugLines;
use pathfinding::num_traits::Signed;

pub fn queen_start() -> SideIPos {
    SideIPos::new(0, -20)
}

/// We want the transform position specified to be on the top left of the rendered sprite.
pub fn sprite() -> Sprite {
    Sprite {
        anchor: Anchor::BottomLeft,
        ..Default::default()
    }
}
pub fn texture_atlas_sprite() -> TextureAtlasSprite {
    TextureAtlasSprite {
        anchor: Anchor::BottomLeft,
        ..Default::default()
    }
}

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug_lines: ResMut<DebugLines>,
) {
    let mut side_map_pos_to_entities = HashMap::with_capacity(1_000);

    // This is a temporary structure to help us build the graph for this system only.
    // Other systems will use a Query to look up the Cell entity via SideMapPosToEntities.
    let mut side_map_pos_to_cell = HashMap::with_capacity(1_000);

    let mut graph = UnGraphMap::<SideIPos, u64>::with_capacity(1_000, 4_000);

    let queen_start = queen_start();
    let queen_room_width = 3;
    let queen_room_height = 3;

    let width = 40;

    // Add exit points to the left and right on the surface for scout ants.
    for x in (-width / 2 - 1..width / 2 + 2).step_by(4) {
        let exit_pos = SideIPos::new(x, 0);
        let cell = CellContent::empty_air();
        let entity_id = commands.spawn((exit_pos, cell, MapExit)).id();
        side_map_pos_to_entities.insert(exit_pos, entity_id);
        side_map_pos_to_cell.insert(exit_pos, cell);
        graph.add_node(exit_pos);

        // Draw a line from the exit point to 0, 20
        debug_lines.line_colored(
            Vec3::new(
                exit_pos.x as f32 * SIDE_CELL_SIZE as f32,
                exit_pos.y as f32 * SIDE_CELL_SIZE as f32,
                0.0,
            ),
            Vec3::new(0.0, 20.0, 0.0),
            10.0,
            Color::rgb(0.0, 1.0, 0.0),
        );
    }

    for y in -30..20 {
        for x in -width / 2..(width / 2 + 1) {
            let cell_content = if y >= 0 {
                CellContent::empty_air()
            } else if x >= queen_start.x - queen_room_width / 2
                && x <= queen_start.x + queen_room_width / 2
                && y >= queen_start.y
                && y < queen_start.y + queen_room_height
            {
                CellContent::empty_underground()
            } else {
                // Create dirt from Y - 1 and downwards with a width of 20.
                // Y 0 or higher is the surface, so make Dirt::empty()
                // We want a V shape around the origin so that ants are initially biased towards
                // the middle and not dig new holes.
                //
                // e.g. the top row (at x == 0, y = -1) should have 5 very light dirt (e.g. 10).
                // the next row should have 3 light dirt (e.g. 20), and so on.
                //
                // Also, x away from the origin should be light in the middle and gradually harder
                // further away.
                //
                let forced_dirt_amount = (y.abs() * 10 + x.abs() * 30) as f32;
                // Add a random amount of dirt.
                let forced_dirt_amount = forced_dirt_amount + rand::random::<f32>() * 50.0 - 20.0;

                let forced_dirt_amount = forced_dirt_amount.max(0.0) as u64;

                if forced_dirt_amount > 0u64 && forced_dirt_amount < 255u64 {
                    CellContent::dirt(forced_dirt_amount as u8)
                } else if y >= -5 {
                    CellContent::dirt(
                        (255f32 - rand::random::<f32>() * (255.0 / 5f32) * y.abs() as f32) as u8,
                    )
                } else {
                    if rand::random::<u8>() < 5 {
                        CellContent::rock(true)
                    } else {
                        CellContent::random_dirt()
                    }
                }
            };

            let side_cell = SideIPos::new(x, y);
            let texture_path = cell_content.texture_path();
            let transform = side_cell.to_transform(DIRT_Z);

            let mut entity = commands.spawn_empty();

            if let Some(texture_path) = texture_path {
                let sprite_bundle = SpriteBundle {
                    sprite: sprite(),
                    transform,
                    texture: asset_server.load(texture_path),
                    ..Default::default()
                };
                entity.insert(sprite_bundle);
            }
            let entity_id = entity.insert((cell_content, side_cell)).id();

            side_map_pos_to_entities.insert(side_cell, entity_id);
            side_map_pos_to_cell.insert(side_cell, cell_content);

            graph.add_node(side_cell);
        }
    }

    for (pos, cell) in side_map_pos_to_cell.iter() {
        let Some(a_weight) = cell.weight() else {
            continue;
        };

        for neighbour in pos.sides() {
            if let Some(other_cell) = side_map_pos_to_cell.get(&neighbour) {
                let Some(b_weight) = other_cell.weight() else {
                    continue;
                };

                let weight = a_weight as u64 + b_weight as u64;
                graph.add_edge(*pos, neighbour, weight as u64);
            }
        }
    }

    println!(
        "Graph has {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    // // Draws all edges.
    // for edge in graph.edge_references() {
    //     let a = edge.source().to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //     let b = edge.target().to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //     // debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 100.0, Color::WHITE);
    // }

    // Finally own the dirt map and set the resource.
    commands.insert_resource(SideMapPosToEntities::from(side_map_pos_to_entities));
    commands.insert_resource(SideMapGraph::from(graph));

    // Explicitly drop the temporary side_map_pos_to_cell, just as a reminder that it's temporary.
    drop(side_map_pos_to_cell);
}

pub fn setup_queen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("creatures/queen.png");
    let queen_start = queen_start();
    let transform = queen_start.to_transform(QUEEN_Z);

    let sprite_bundle = SpriteBundle {
        sprite: sprite(),
        texture,
        transform,
        ..Default::default()
    };
    commands.spawn((sprite_bundle, Queen::default(), Hunger::default()));
}

pub fn setup_test_eggs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut egg_laid_writer: EventWriter<EggLaidEvent>,
) {
    egg_laid_writer.send(EggLaidEvent {
        egg: Egg {
            ant_type: Default::default(),
            growth: 0.0,
            hatch_at: 3.0,
        },
        position: SideIPos::new(-2, 0),
    });
}
