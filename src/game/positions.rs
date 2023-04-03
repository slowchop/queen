use crate::game;
use crate::game::map::SIDE_CELL_SIZE;
use bevy::math::{IVec2, Vec2, Vec3};
use bevy::prelude::*;

/// The side position of a fixed position, e.g. a dirt cell.
#[derive(Component, Deref, DerefMut, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SideIPos(IVec2);

impl SideIPos {
    pub fn sides(&self) -> [SideIPos; 4] {
        [
            SideIPos::new(self.0.x + 1, self.0.y),
            SideIPos::new(self.0.x - 1, self.0.y),
            SideIPos::new(self.0.x, self.0.y + 1),
            SideIPos::new(self.0.x, self.0.y - 1),
        ]
    }
}

impl SideIPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn to_world_vec2(&self) -> Vec2 {
        Vec2::new(
            self.0.x as f32 * SIDE_CELL_SIZE as f32,
            self.0.y as f32 * SIDE_CELL_SIZE as f32,
        )
    }

    pub fn to_world_vec3(&self, z: f32) -> Vec3 {
        self.to_world_vec2().extend(z)
    }

    pub fn to_transform(&self, z: f32) -> Transform {
        Transform::from_translation(self.to_world_vec3(z))
    }
}

impl From<&Vec3> for SideIPos {
    fn from(vec: &Vec3) -> Self {
        Self::new(
            (vec.x / SIDE_CELL_SIZE as f32).floor() as i32,
            (vec.y / SIDE_CELL_SIZE as f32).floor() as i32,
        )
    }
}

impl From<&Transform> for SideIPos {
    fn from(transform: &Transform) -> Self {
        Self::from(&transform.translation)
    }
}

impl PartialOrd<Self> for SideIPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SideIPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        morton_encode(self.0.x, self.0.y).cmp(&morton_encode(other.0.x, other.0.y))
    }
}

/// The side position of a floating point position. Used for crawler positions.
#[derive(Deref, DerefMut, Copy, Clone, Debug)]
pub struct SidePosition(Vec2);

impl SidePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn to_cell(&self) -> SideIPos {
        SideIPos::new(
            (self.0.x / SIDE_CELL_SIZE as f32).floor() as i32,
            (self.0.y / SIDE_CELL_SIZE as f32).floor() as i32,
        )
    }
}

fn morton_encode(x: i32, y: i32) -> i64 {
    let mut z = 0i64;
    let x = x as i64;
    let y = y as i64;

    for i in 0..32 {
        z |= ((x >> i) & 1) << (2 * i);
        z |= ((y >> i) & 1) << (2 * i + 1);
    }

    z
}
