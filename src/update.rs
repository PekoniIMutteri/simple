use crate::pva::update::PvaPlugin;
use bevy::prelude::*;

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PvaPlugin);
    }
}
