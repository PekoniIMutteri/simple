use bevy::prelude::*;
mod pva;
mod setup;
mod update;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(setup::SetupPlugin)
        .add_plugins(update::UpdatePlugin)
        .run();
    println!("succesfully closed !")
}
