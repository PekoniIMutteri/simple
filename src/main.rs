use bevy::prelude::*;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: bevy::window::WindowResolution::new(700.0, 500.0),
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(player::PlayerPlugin)
        .run();
}
