mod state;
mod domain;
mod overworld;
mod title_screen;

use bevy::prelude::*;
use state::AppState;
use overworld::OverworldPlugin;
use title_screen::TitleScreenPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Schola Graeca - Iter Herois".to_string(),
                resolution: (1280.0_f32, 720.0_f32).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(TitleScreenPlugin)
        .add_plugins(OverworldPlugin)
        .run();
}
