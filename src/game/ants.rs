use crate::game::animation::{AnimationIndices, AnimationTimer};
use crate::game::brains::{
    EatAction, HungryScorer, MapTransitionAction, MoveToFoodAction,
    OfferFoodDiscoveryToQueenAction, OutsideMapDiscoveringNewFoodAction, PathfindingAction,
    SetPathToOutsideAction, SetPathToQueenAction,
};
use crate::game::eggs::SpawnAntEvent;
use crate::game::hunger::Hunger;
use crate::game::jobs::Assignment;
use crate::game::map::SIDE_CELL_SIZE;
use crate::game::pathfinding::Path;
use crate::game::plugin::{Crawler, Speed, ANT_Z};
use crate::game::positions::SideIPos;
use crate::game::queen::EggLaidEvent;
use crate::game::setup::{sprite, texture_atlas_sprite};
use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Component, Debug, Eq, PartialEq, Default, Copy, Clone)]
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

        let move_and_eat = Steps::build()
            .label("MoveAndEat")
            .step(MoveToFoodAction)
            .step(EatAction);

        let find_new_food = Steps::build()
            .label("FindNewFood")
            .step(SetPathToOutsideAction)
            .step(PathfindingAction)
            .step(MapTransitionAction::exit())
            .step(OutsideMapDiscoveringNewFoodAction::default())
            .step(MapTransitionAction::enter())
            .step(SetPathToQueenAction)
            .step(PathfindingAction)
            .step(OfferFoodDiscoveryToQueenAction);

        let thinker = match ant_type {
            AntType::Scout => Thinker::build()
                .label("ScoutThinker")
                .picker(FirstToScore { threshold: 0.5 })
                .when(HungryScorer, move_and_eat)
                .otherwise(find_new_food),

            _ => todo!(),
        };

        commands.spawn((
            sprite_sheet_bundle,
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Crawler,
            *ant_type,
            thinker,
            Speed::default(),
            Hunger::default(),
            Assignment::None,
            Path::NeedsPath(SideIPos::new(0, -20)),
        ));
    }
}
