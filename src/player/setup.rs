use crate::player::Player;
use bevy::prelude::*;

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player {});
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
