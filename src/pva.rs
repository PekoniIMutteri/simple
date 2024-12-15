use bevy::prelude::*;
pub mod update;

#[derive(Component)]
pub struct Pva {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Pva {
    fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.velocity += self.acceleration * delta;
    }
}
