use bevy::prelude::*;

pub mod map;
pub mod hero;
pub mod interactables;
pub mod hud;

use crate::state::AppState;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::TabulaGeographica), (
            map::spawn_overworld_map,
            hero::spawn_hero_and_camera,
            interactables::spawn_interactables,
            hud::spawn_overworld_hud,
        ))
        .add_systems(Update, (
            hero::hero_movement_system,
            hero::camera_follow_system,
            hero::hero_sprite_evolution_system,
            interactables::interaction_system,
            hud::update_hud_system,
        ).run_if(in_state(AppState::TabulaGeographica)));
    }
}
