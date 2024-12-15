use bevy::prelude::*;
mod setup;
mod update;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        return Player {};
    }
}

//

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(setup::SetupPlugin)
            .add_plugins(update::UpdatePlugin);
    }
}
