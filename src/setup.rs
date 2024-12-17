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
    wall.pva.position.y = -300.0;
    wall.pva.size = Vec2::new(2000.0, 400.0);

    commands.spawn((
        wall,
        Sprite {
            custom_size: Some(Vec2::new(2000.0, 400.0)),
            ..default()
        },
    ));

    let mut wall = WallBundle::default();
    wall.pva.position.x = -500.0;
    wall.pva.size = Vec2::new(400.0, 2000.0);

    commands.spawn((
        wall,
        Sprite {
            custom_size: Some(Vec2::new(400.0, 2000.0)),
            ..default()
        },
    ));

    let mut wall = WallBundle::default();
    wall.pva.position.y = 200.0;
    wall.pva.size = Vec2::new(2000.0, 400.0);

    commands.spawn((
        wall,
        Sprite {
            custom_size: Some(Vec2::new(2000.0, 400.0)),
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
