use crate::game::ants::AntType;
use crate::game::plugin::EGG_Z;
use crate::game::positions::SideIPos;
use crate::game::queen::EggLaidEvent;
use crate::game::setup::sprite;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub struct SpawnAntEvent {
    pub ant_type: AntType,
    pub position: SideIPos,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Egg {
    pub ant_type: AntType,
    pub growth: f32,
    pub hatch_at: f32,
}

impl Egg {
    pub fn new(ant_type: AntType, hatch_at: f32) -> Self {
        Self {
            ant_type,
            growth: 0f32,
            hatch_at,
        }
    }
}

pub fn spawn_eggs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egg_laid_reader: EventReader<EggLaidEvent>,
) {
    for EggLaidEvent { egg, position } in egg_laid_reader.iter() {
        let texture = asset_server.load("creatures/egg.png");
        let transform = position.to_transform(EGG_Z);

        let sprite_bundle = SpriteBundle {
            transform,
            sprite: sprite(),
            texture,
            ..Default::default()
        };
        commands.spawn((sprite_bundle, *egg));
    }
}

pub fn grow_eggs(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Egg, &Transform)>,
    mut spawn_ant_writer: EventWriter<SpawnAntEvent>,
) {
    for (entity, mut egg, transform) in query.iter_mut() {
        egg.growth += time.delta_seconds();
        if egg.growth < egg.hatch_at {
            continue;
        }

        let position = SideIPos::from(transform);
        spawn_ant_writer.send(SpawnAntEvent {
            ant_type: egg.ant_type,
            position,
        });

        commands.entity(entity).despawn();
    }
}
