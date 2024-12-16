use bevy::prelude::*;

#[derive(Component)]
pub struct Pva {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,

    pub movable: bool,
}

fn update_pva(mut query: Query<(&mut Pva, &mut Transform)>, time: Res<Time>) {
    for (mut pva, mut transform) in &mut query {
        pva.update(time.delta_secs());
        transform.translation = pva.position;
    }
}

impl Pva {
    fn update(&mut self, delta: f32) {
        if self.movable {
            self.position += self.velocity * delta;
            self.velocity += self.acceleration * delta;
        }
    }

    pub fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,

            movable: false,
        }
    }

    pub fn default_gravity() -> Self {
        Self {
            acceleration: Vec3::new(0.0, -1000.0, 0.0),
            movable: true,
            ..Self::default()
        }
    }
}

pub struct PvaPlugin;

impl Plugin for PvaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_pva);
    }
}
