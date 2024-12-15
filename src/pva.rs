use bevy::prelude::*;

#[derive(Component)]
pub struct Pva {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

fn update_pva(mut query: Query<&mut Pva>, time: Res<Time>) {
    for mut pva in &mut query {
        pva.update(time.delta_secs());
    }
}

fn update_transform(mut query: Query<(&Pva, &mut Transform)>) {
    for (pva, mut transform) in &mut query {
        transform.translation = pva.position.extend(0.0);
    }
}

impl Pva {
    fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.velocity += self.acceleration * delta;
    }
}

pub struct PvaPlugin;

impl Plugin for PvaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_pva, update_transform));
    }
}
