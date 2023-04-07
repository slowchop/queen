use crate::game::time::GameTime;
use bevy::prelude::{Component, Query, Res};

#[derive(Component, Debug)]
pub struct Hunger {
    pub current: f32,
    pub hungry_at: f32,
    pub starving_at: f32,
}

impl Hunger {
    pub fn new(hungry_at: f32, starving_at: f32) -> Self {
        Self {
            current: 0f32,
            hungry_at,
            starving_at,
        }
    }

    pub fn feed(&mut self, amount: f32) {
        self.current -= amount;
        if self.current < 0f32 {
            self.current = 0f32;
        }
    }

    pub fn starving_offset(&self) -> f32 {
        (self.starving_at - self.current).max(0f32)
    }

    pub fn hunger_fraction(&self) -> f32 {
        self.current / self.hungry_at
    }

    /// 0 - 1
    pub fn hunger_score(&self) -> f32 {
        self.hunger_fraction().min(1f32).max(0f32)
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
