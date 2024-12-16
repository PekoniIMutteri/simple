use super::Pva;
use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

impl Default for Wall {
    fn default() -> Self {
        Self
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub pva: Pva,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall::default(),
            pva: Pva::default(),
        }
    }
}
