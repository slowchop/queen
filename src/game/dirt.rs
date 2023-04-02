use bevy::prelude::{Component, Deref, DerefMut};

/// A dirt block. The u8 is the amount of dirt. 0 is empty.
#[derive(Component, Deref, DerefMut)]
pub struct Dirt(u8);

impl Dirt {
    pub fn empty() -> Self {
        Self(0)
    }

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

    pub fn texture_path(&self) -> Option<String> {
        if self.is_empty() {
            None
        } else if self.0 > 127 {
            Some("dirt/full.png".to_string())
        } else {
            Some("dirt/half.png".to_string())
        }
    }
}
