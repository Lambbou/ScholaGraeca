use bevy::prelude::*;
use crate::domain::player::Heros;
use crate::state::AppState;

pub const INTERACTION_RADIUS: f32 = 48.0;

#[derive(Component)]
pub struct NpcPersona {
    pub id: String,
    pub nomen: String,
}

#[derive(Component)]
pub struct MonstrumCustos {
    pub id: String,
    pub nomen: String,
}

#[derive(Component)]
pub struct PromptText;

pub fn spawn_interactables(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. Pêcheur de l'Épire (Piscator Epiri)
    let npc_tex = asset_server.load("textures/npc_piscator.png");
    commands.spawn((
        Sprite::from_image(npc_tex),
        Transform::from_xyz(-50.0, 20.0, 10.0),
        NpcPersona {
            id: "piscator_epiri".to_string(),
            nomen: "Piscator Epiri".to_string(),
        },
    ));

    // 2. Harpie des tempêtes (Harpyia), gardienne du col vers Dodone
    let monster_tex = asset_server.load("textures/monster_harpyia.png");
    commands.spawn((
        Sprite::from_image(monster_tex),
        Transform::from_xyz(320.0, 60.0, 10.0),
        MonstrumCustos {
            id: "harpyia".to_string(),
            nomen: "Harpyia".to_string(),
        },
    ));
}

/// Détecte la proximité et gère les interactions [Clavis E]
pub fn interaction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    hero_query: Query<&Transform, With<Heros>>,
    npc_query: Query<(&Transform, &NpcPersona), Without<Heros>>,
    monster_query: Query<(&Transform, &MonstrumCustos), (Without<Heros>, Without<NpcPersona>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut prompt_query: Query<(&mut Text, &mut Visibility), With<PromptText>>,
) {
    let Ok(hero_tf) = hero_query.get_single() else { return };
    let hero_pos = hero_tf.translation.xy();

    let mut prompt_content = String::new();
    let mut prope_npc = false;
    let mut prope_monstrum = false;

    // Vérification PNJ
    for (npc_tf, npc) in npc_query.iter() {
        if hero_pos.distance(npc_tf.translation.xy()) < INTERACTION_RADIUS {
            prompt_content = format!("[Clavis E: Conloqui cum {}]", npc.nomen);
            prope_npc = true;
            break;
        }
    }

    // Vérification Monstre
    if !prope_npc {
        for (mon_tf, mon) in monster_query.iter() {
            if hero_pos.distance(mon_tf.translation.xy()) < INTERACTION_RADIUS {
                prompt_content = format!("[Clavis E: Certare contra {}]", mon.nomen);
                prope_monstrum = true;
                break;
            }
        }
    }

    // Mise à jour de l'invite textuelle
    for (mut text, mut vis) in prompt_query.iter_mut() {
        if !prompt_content.is_empty() {
            text.0 = prompt_content.clone();
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
    }

    // Déclenchement de l'interaction
    if keyboard.just_pressed(KeyCode::KeyE) {
        if prope_npc {
            next_state.set(AppState::Conloquium);
        } else if prope_monstrum {
            next_state.set(AppState::Certamen);
        }
    }
}
