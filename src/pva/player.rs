use super::Pva;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,

    speed: f32,
    jump: f32,
    fall: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,

            speed: 400.0,
            jump: 500.0,
            fall: -200.0,
        }
    }
}

fn inputs(
    mut query: Query<(&Player, &mut Pva)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut pva) in &mut query {
        let mut direction = 0.0;
        if keyboard.pressed(player.left) {
            direction -= 1.0;
        }
        if keyboard.pressed(player.right) {
            direction += 1.0;
        }
        if keyboard.pressed(player.up) {
            pva.velocity.y = player.jump;
        }
        if keyboard.pressed(player.down) {
            pva.velocity.y = player.fall;
        }
        direction = direction * player.speed;
        pva.velocity.x = pva.velocity.x.lerp(direction, time.delta_secs() * 10.0);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub pva: Pva,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            player: Player::default(),
            pva: Pva::default_gravity(),
            transform: Transform::default(),
            sprite: Sprite::default(),
        }
    }
}

//

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, inputs);
    }
}
