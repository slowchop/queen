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

#[derive(Component)]
pub struct SidePos(IVec2);

#[derive(Resource)]
pub struct SideMapDirt(HashMap<SidePos, Entity>);

#[derive(Component)]
pub struct Worker;

pub fn setup(mut commands: Commands) {
    let side_map_dirt = HashMap::with_capacity(1_000);

    // Create dirt from Y - 1 and downwards with a width of 20.
    let width = 20;
    for y in -20..0 {
        for x in -width / 2..width / 2 {
            let dirt = Dirt::random();
            let position = IVec2::new(x, y);
            let transform = Transform::from_translation(position.extend(0.0));
            let entity = commands.spawn((dirt, transform)).id();
        }
    }

    commands.insert_resource(SideMapDirt(HashMap::default()));
}
