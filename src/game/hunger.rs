use crate::game::time::GameTime;
use bevy::prelude::{Component, Query, Res};

#[derive(Component)]
pub struct Hunger {
    current: f32,
    hungry_at: f32,
    starving_at: f32,
}

impl Hunger {
    pub fn new(hungry_at: f32, starving_at: f32) -> Self {
        Self {
            current: 0f32,
            hungry_at,
            starving_at,
        }
    }

    pub fn starving_offset(&self) -> f32 {
        (self.starving_at - self.current).max(0f32)
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Self::new(10f32, 20f32)
    }
}

pub fn hunger_system(time: Res<GameTime>, mut query: Query<&mut Hunger>) {
    for mut hunger in query.iter_mut() {
        hunger.current += time.delta_seconds();
    }
}
