use crate::game::camera::CameraFocus;
use crate::game::jobs::Assignment;
use crate::game::map::CellContent;
use crate::game::pathfinding::{Path, SideMapGraph};
use crate::game::positions::SideIPos;
use crate::game::{
    Crawler, Hunger, PlayerState, QueenMode, SideMapPosToEntities, Speed, SIDE_CELL_SIZE,
};
use bevy::asset::AssetServer;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::petgraph::algo::astar;
use bevy::utils::petgraph::prelude::*;
use bevy::utils::petgraph::visit::IntoEdgeReferences;
use bevy::utils::HashMap;
use bevy_prototype_debug_lines::DebugLines;

/// We want the transform position specified to be on the top left of the rendered sprite.
pub fn sprite() -> Sprite {
    Sprite {
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

    // Create dirt from Y - 1 and downwards with a width of 20.
    // Y 0 or higher is the surface, so make Dirt::empty()
    let width = 20;
    for y in -20..20 {
        for x in -width / 2..width / 2 {
            let cell_content = if y >= 0 {
                CellContent::empty_air()
            } else {
                if rand::random::<u8>() < 5 {
                    CellContent::rock(true)
                } else {
                    CellContent::random_dirt()
                }
            };

            let side_cell = SideIPos::new(x, y);
            let texture_path = cell_content.texture_path();
            let transform = Transform::from_translation(side_cell.to_world_vec3());

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
            let entity_id = commands.spawn(cell_content).id();

            side_map_pos_to_entities.insert(side_cell, entity_id);
            side_map_pos_to_cell.insert(side_cell, cell_content);

            graph.add_node(side_cell);
        }
    }

    // Work through each graph node and add edges to the nodes above, below, left and right.
    // for (side_cell, node_id) in graph_positions.iter() {
    //     for neighbour in side_cell.sides() {
    //         if let Some(other_node_id) = graph_positions.get(&neighbour) {
    //             graph.add_edge(*node_id, *other_node_id, ());
    //         }
    //     }
    // }

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

    // let goal = SideIPos::new(0, -20);
    // let result = astar(
    //     &graph,
    //     SideIPos::new(0, 0),
    //     |finish| finish == goal,
    //     |e| *e.weight(),
    //     |z| (*z - *goal).as_vec2().length() as u64,
    // );
    // dbg!(&result);
    //
    // // Draws all edges.
    // for edge in graph.edge_references() {
    //     let a = edge.source().to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //     let b = edge.target().to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //     // debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 100.0, Color::WHITE);
    // }
    //
    // // Draw debug lines for the found path.
    // if let Some((_, path)) = result {
    //     for (a, b) in path.windows(2).map(|w| (w[0], w[1])) {
    //         let a = a.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //         let b = b.to_world_vec2() + SIDE_CELL_SIZE as f32 / 2f32;
    //         debug_lines.line_colored(a.extend(0f32), b.extend(0f32), 100.0, Color::LIME_GREEN);
    //     }
    // }

    // Finally own the dirt map and set the resource.
    commands.insert_resource(SideMapPosToEntities(side_map_pos_to_entities));
    commands.insert_resource(SideMapGraph::from(graph));
}

pub fn setup_queen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("creatures/queen.png");

    let sprite_bundle = SpriteBundle {
        sprite: sprite(),
        texture,
        ..Default::default()
    };
    commands.spawn((
        sprite_bundle,
        Crawler,
        Speed::default(),
        Hunger::default(),
        Assignment::None,
    ));
}

pub fn setup_ants(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..5 {
        let texture = asset_server.load("creatures/ant.png");

        let sprite_bundle = SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                (2.0 + x as f32) * SIDE_CELL_SIZE as f32,
                0.0,
                0.0,
            )),
            sprite: sprite(),
            texture,
            ..Default::default()
        };
        commands.spawn((
            sprite_bundle,
            Crawler,
            Speed::default(),
            Hunger::default(),
            Assignment::None,
            Path::NeedsPath(SideIPos::new(0, -20)),
        ));
    }
}
