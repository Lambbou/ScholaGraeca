use bevy::prelude::*;
use crate::domain::player::{Heros, HerosStatus};
use crate::domain::ranks::GradusCivicus;
use crate::overworld::map::{SolidObstacle, TILE_SIZE};

pub const HERO_SPEED: f32 = 160.0;
pub const HERO_COLLISION_RADIUS: f32 = 14.0;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_hero_and_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 1. Caméra 2D orthographique
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 999.0),
        MainCamera,
    ));

    // 2. Position initiale du héros (sur le sable de la côte d'Épire)
    let start_x = -240.0;
    let start_y = -30.0;

    let texture = asset_server.load("textures/hero_apothetes.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(start_x, start_y, 10.0),
        Heros,
        HerosStatus::default(),
    ));
}

/// Déplacement du héros et détection des collisions
pub fn hero_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hero_query: Query<&mut Transform, With<Heros>>,
    obstacle_query: Query<&Transform, (With<SolidObstacle>, Without<Heros>)>,
) {
    let Ok(mut hero_transform) = hero_query.get_single_mut() else { return };

    let mut direction = Vec2::ZERO;

    // Support des claviers ZQSD, WASD et touches fléchées
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::KeyZ) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyQ) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let delta = direction * HERO_SPEED * time.delta_secs();

        // Test collision axe X
        let tentative_x = hero_transform.translation.x + delta.x;
        let mut collision_x = false;
        for obs in obstacle_query.iter() {
            let obs_pos = obs.translation.xy();
            let distance = Vec2::new(tentative_x, hero_transform.translation.y).distance(obs_pos);
            if distance < (HERO_COLLISION_RADIUS + TILE_SIZE * 0.4) {
                collision_x = true;
                break;
            }
        }
        if !collision_x {
            hero_transform.translation.x = tentative_x;
        }

        // Test collision axe Y
        let tentative_y = hero_transform.translation.y + delta.y;
        let mut collision_y = false;
        for obs in obstacle_query.iter() {
            let obs_pos = obs.translation.xy();
            let distance = Vec2::new(hero_transform.translation.x, tentative_y).distance(obs_pos);
            if distance < (HERO_COLLISION_RADIUS + TILE_SIZE * 0.4) {
                collision_y = true;
                break;
            }
        }
        if !collision_y {
            hero_transform.translation.y = tentative_y;
        }
    }
}

/// Suivi doux de la caméra (Lerp)
pub fn camera_follow_system(
    time: Res<Time>,
    hero_query: Query<&Transform, With<Heros>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Heros>)>,
) {
    let Ok(hero_tf) = hero_query.get_single() else { return };
    let Ok(mut cam_tf) = camera_query.get_single_mut() else { return };

    let target = hero_tf.translation.xy();
    let current = cam_tf.translation.xy();
    let new_pos = current.lerp(target, 5.0 * time.delta_secs());

    cam_tf.translation.x = new_pos.x;
    cam_tf.translation.y = new_pos.y;
}

/// Évolution visuelle du sprite du héros selon son rang civique
pub fn hero_sprite_evolution_system(
    asset_server: Res<AssetServer>,
    mut hero_query: Query<(&HerosStatus, &mut Sprite), (With<Heros>, Changed<HerosStatus>)>,
) {
    for (status, mut sprite) in hero_query.iter_mut() {
        let texture_path = match status.gradus {
            GradusCivicus::Apothetes => "textures/hero_apothetes.png",
            GradusCivicus::Ephebus => "textures/hero_ephebus.png",
            GradusCivicus::Polites => "textures/hero_polites.png",
            GradusCivicus::Philosophus => "textures/hero_philosophus.png",
            GradusCivicus::Archon => "textures/hero_archon.png",
        };
        sprite.image = asset_server.load(texture_path);
    }
}
