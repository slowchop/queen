use bevy::prelude::*;
use bevy::utils::HashMap;
use dirt::Dirt;

mod dirt;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup);
    }
}

/// The side view of the world. The idea is that if we have time we can do a top down view on the
/// surface of the world.
#[derive(Component, Deref, DerefMut)]
pub struct SidePos(IVec2);

#[derive(Resource)]
pub struct SideMapDirt(HashMap<SidePos, Entity>);

#[derive(Component)]
pub struct Worker;

pub fn setup(mut commands: Commands) {
    let mut side_map_dirt = HashMap::with_capacity(1_000);

    // Create dirt from Y - 1 and downwards with a width of 20.
    // Y 0 or higher is the surface, so make Dirt::empty()
    let width = 20;
    for y in -20..20 {
        for x in -width / 2..width / 2 {
            let dirt = if y >= 0 {
                Dirt::random()
            } else {
                Dirt::random()
            };
            let position = SidePos::new(x, y);
            let transform = Transform::from_translation(position.as_vec2().extend(0.0));
            let entity = commands.spawn((dirt, position, transform)).id();
            side_map_dirt.insert(SidePos(position), entity);
        }
    }

    commands.insert_resource(SideMapDirt(side_map_dirt));
}
