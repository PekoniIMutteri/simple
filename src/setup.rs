use crate::player::Player;
use crate::pva::Pva;
use bevy::prelude::*;

fn player(mut commands: Commands) {
    commands.spawn((
        Player::default_inputs(),
        Pva {
            velocity: Vec3::new(0.0, 20.0, 0.0),
            ..Pva::default_gravity()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
    ));
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player, camera));
    }
}
