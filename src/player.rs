use crate::pva::Pva;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,

    speed: f32,
}

impl Player {
    pub fn default_inputs() -> Player {
        Player {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,

            speed: 300.0,
        }
    }
}

fn inputs(
    mut query: Query<(&Player, &mut Pva)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut pva) in &mut query {
        let mut direction = Vec2::ZERO;
        if keyboard.pressed(player.left) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(player.right) {
            direction.x += 1.0;
        }
        if keyboard.pressed(player.up) {
            direction.y += 1.0;
        }
        if keyboard.pressed(player.down) {
            direction.y -= 1.0;
        }
        direction = direction.normalize_or_zero() * player.speed;
        pva.velocity = pva.velocity.lerp(direction, time.delta_secs() * 10.0);
    }
}

//

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, inputs);
    }
}
