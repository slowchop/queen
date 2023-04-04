use crate::game::animation::{AnimationIndices, AnimationTimer};
use crate::game::eggs::SpawnAntEvent;
use crate::game::jobs::Assignment;
use crate::game::map::SIDE_CELL_SIZE;
use crate::game::pathfinding::Path;
use crate::game::plugin::{Crawler, Hunger, Speed, ANT_Z};
use crate::game::positions::SideIPos;
use crate::game::queen::EggLaidEvent;
use crate::game::setup::{sprite, texture_atlas_sprite};
use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
pub enum AntType {
    #[default]
    Scout,
    Cargo,
    Nurse,
    Soldier,
}

impl AntType {
    fn spawn_data(&self) -> AntSpawnData {
        match self {
            AntType::Scout => AntSpawnData {
                texture_path: "creatures/scout.png",
                columns: 4,
                rows: 1,
                animation_indices: AnimationIndices { first: 0, last: 3 },
            },
            AntType::Cargo => AntSpawnData {
                texture_path: "creatures/cargo.png",
                columns: 4,
                rows: 1,
                animation_indices: AnimationIndices { first: 0, last: 3 },
            },
            AntType::Nurse => AntSpawnData {
                texture_path: "creatures/nurse.png",
                columns: 4,
                rows: 1,
                animation_indices: AnimationIndices { first: 0, last: 3 },
            },
            AntType::Soldier => AntSpawnData {
                texture_path: "creatures/soldier.png",
                columns: 4,
                rows: 1,
                animation_indices: AnimationIndices { first: 0, last: 3 },
            },
        }
    }
}

struct AntSpawnData {
    texture_path: &'static str,
    columns: usize,
    rows: usize,
    animation_indices: AnimationIndices,
}

pub fn spawn_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawn_ant_reader: EventReader<SpawnAntEvent>,
) {
    for SpawnAntEvent { ant_type, position } in spawn_ant_reader.iter() {
        // match ant_type {
        //     AntType::Scout => ["creatures/scout.png"],
        // }
        //
        // // TODO: Different ants configured in a yaml?
        // let texture_handle = asset_server.load("creatures/soldier.png");
        // let texture_atlas = TextureAtlas::from_grid(
        //     texture_handle,
        //     Vec2::new(SIDE_CELL_SIZE as f32, SIDE_CELL_SIZE as f32),
        //     4,
        //     1,
        //     None,
        //     None,
        // );
        // let texture_atlas = texture_atlases.add(texture_atlas);
        // let animation_indices = AnimationIndices { first: 0, last: 3 };

        let ant_spawn_data = ant_type.spawn_data();
        let texture_handle = asset_server.load(ant_spawn_data.texture_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(SIDE_CELL_SIZE as f32, SIDE_CELL_SIZE as f32),
            ant_spawn_data.columns,
            ant_spawn_data.rows,
            None,
            None,
        );
        let texture_atlas = texture_atlases.add(texture_atlas);
        let animation_indices = ant_spawn_data.animation_indices;

        let transform = position.to_transform(ANT_Z);
        let sprite_sheet_bundle = SpriteSheetBundle {
            transform,
            sprite: texture_atlas_sprite(),
            texture_atlas,
            ..Default::default()
        };

        commands.spawn((
            sprite_sheet_bundle,
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Crawler,
            Speed::default(),
            Hunger::default(),
            Assignment::None,
            Path::NeedsPath(SideIPos::new(0, -20)),
        ));
    }
}
