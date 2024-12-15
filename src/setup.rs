use crate::player::Player;
use crate::pva::Pva;
use bevy::prelude::*;

fn player(mut commands: Commands) {
    commands.spawn((
        Player::default_inputs(),
        Pva {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(20.0, 0.0),
            acceleration: Vec2::ZERO,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            custom_size: Some(Vec2::splat(200.0)),
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
