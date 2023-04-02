use crate::game::camera::CameraFocus;
use crate::game::dirt::CellContent;
use crate::game::jobs::Assignment;
use crate::game::pathfinding::Path;
use crate::game::{
    Crawler, Hunger, PlayerState, QueenMode, SideCell, SideDirtCells, Speed, SIDE_CELL_SIZE,
};
use bevy::asset::AssetServer;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::petgraph::matrix_graph::MatrixGraph;
use bevy::utils::petgraph::Graph;
use bevy::utils::HashMap;

/// We want the transform position specified to be on the top left of the rendered sprite.
pub fn sprite() -> Sprite {
    Sprite {
        anchor: Anchor::TopLeft,
        ..Default::default()
    }
}

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut side_map_dirt = HashMap::with_capacity(1_000);
    let mut graph = Graph::<CellContent, ()>::with_capacity(1_000, 4_000);

    let mut graph_positions = HashMap::with_capacity(1_000);

    // Create dirt from Y - 1 and downwards with a width of 20.
    // Y 0 or higher is the surface, so make Dirt::empty()
    let width = 20;
    for y in -20..20 {
        for x in -width / 2..width / 2 {
            let dirt = if y >= 0 {
                CellContent::empty_air()
            } else {
                CellContent::random_dirt()
            };

            let side_cell = SideCell::new(x, y);
            let texture_path = dirt.texture_path();
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
            let entity_id = commands.spawn((dirt)).id();

            side_map_dirt.insert(side_cell, entity_id);

            let node_id = graph.add_node(dirt);
            graph_positions.insert(side_cell, node_id);
        }
    }

    // Finally own the dirt map and set the resource.
    commands.insert_resource(SideDirtCells(side_map_dirt));
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
            Path::NeedsPath(SideCell::new(0, -20)),
        ));
    }
}
