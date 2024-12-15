use bevy::prelude::*;

use super::Player;

fn moving(mut query: Query<&mut Player>) {
    for _ in &mut query {
        Player::new();
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving);
    }
}
