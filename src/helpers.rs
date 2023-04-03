use crate::ui::splash::SplashComponent;
use bevy::prelude::*;

pub fn delete_entities_with_component<T: Component>(
    mut commands: Commands,
    to_destroy: Query<Entity, With<T>>,
) {
    for entity in to_destroy.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
