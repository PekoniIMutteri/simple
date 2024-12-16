use crate::pva::{player::PlayerBundle, wall::WallBundle};
use bevy::prelude::*;

fn player(mut commands: Commands) {
    let mut player = PlayerBundle::default();
    player.sprite.custom_size = Some(Vec2::splat(50.0));
    player.pva.size = Vec2::splat(50.0);
    commands.spawn(player);
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn test_wall(mut commands: Commands) {
    let mut wall = WallBundle::default();
    wall.pva.position.y = -200.0;
    wall.pva.size = Vec2::splat(100.0);

    commands.spawn((
        wall,
        Sprite {
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
    ));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player, camera, test_wall));
    }
}
