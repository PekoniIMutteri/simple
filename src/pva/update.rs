use bevy::prelude::*;

use super::Pva;

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

pub struct PvaPlugin;

impl Plugin for PvaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_pva, update_transform));
    }
}
