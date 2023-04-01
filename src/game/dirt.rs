use bevy::prelude::{Component, Deref, DerefMut};

/// A dirt block. The u8 is the amount of dirt. 0 is empty.
#[derive(Component, Deref, DerefMut)]
pub struct Dirt(u8);

impl Dirt {
    pub fn random() -> Self {
        Self(rand::random::<u8>())
    }

    pub fn dig(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
