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

pub fn spawn_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawn_ant_reader: EventReader<SpawnAntEvent>,
) {
    for SpawnAntEvent { ant_type, position } in spawn_ant_reader.iter() {
        // TODO: Different ants configured in a yaml?
        let texture_handle = asset_server.load("creatures/soldier.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(SIDE_CELL_SIZE as f32, SIDE_CELL_SIZE as f32),
            4,
            1,
            None,
            None,
        );
        let texture_atlas = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

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
