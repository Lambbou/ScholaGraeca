use bevy::prelude::*;
use crate::domain::player::{Heros, HerosStatus};
use crate::overworld::interactables::PromptText;

#[derive(Component)]
pub struct HudVitaText;

#[derive(Component)]
pub struct HudXpText;

#[derive(Component)]
pub struct HudGradusText;

#[derive(Component)]
pub struct HudRoot;

pub fn spawn_overworld_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/ancient.ttf");

    // Conteneur principal plein écran pour l'UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        HudRoot,
    )).with_children(|parent| {
        // --- BANDEAU SUPÉRIEUR (HUD STATUS) ---
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(56.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.90)),
            BorderColor(Color::srgb(0.80, 0.65, 0.20)), // Or antique
        )).with_children(|header| {
            // Informations du Héros et Rang civique
            header.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Heros Graecus | Locus: Litus Epiri"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.90, 0.75)),
                ));
                col.spawn((
                    Text::new("Gradus: Apothetes (Naufragus)"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.70, 0.85, 1.0)),
                    HudGradusText,
                ));
            });

            // Barres de Vie et d'Expérience
            header.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|stats| {
                // Barre de Vie (Vita)
                stats.spawn((
                    Text::new("Vita: 100 / 100"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.9, 0.3)),
                    HudVitaText,
                ));

                // Barre d'Expérience (Experientia)
                stats.spawn((
                    Text::new("Experientia: 0 / 150 XP"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                    HudXpText,
                ));
            });
        });

        // --- ZONE INFÉRIEURE : INVITE D'INTERACTION & COMMANDES ---
        parent.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(16.0)),
            row_gap: Val::Px(8.0),
            ..default()
        }).with_children(|footer| {
            // Invite contextuelle flottante (PNJ ou Monstre)
            footer.spawn((
                Text::new(""),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.95, 0.4)),
                Visibility::Hidden,
                PromptText,
            ));

            // Aide aux commandes en latin
            footer.spawn((
                Text::new("Cursus: [Z/Q/S/D vel Sagittae] | Conloqui: [E]"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.8, 0.8, 0.7)),
            ));
        });
    });
}

/// Met à jour les valeurs affichées sur le HUD
pub fn update_hud_system(
    hero_query: Query<&HerosStatus, (With<Heros>, Changed<HerosStatus>)>,
    mut vita_query: Query<&mut Text, (With<HudVitaText>, Without<HudXpText>, Without<HudGradusText>)>,
    mut xp_query: Query<&mut Text, (With<HudXpText>, Without<HudVitaText>, Without<HudGradusText>)>,
    mut gradus_query: Query<&mut Text, (With<HudGradusText>, Without<HudVitaText>, Without<HudXpText>)>,
) {
    let Ok(status) = hero_query.get_single() else { return };

    for mut text in vita_query.iter_mut() {
        text.0 = format!("Vita: {} / {}", status.vita, status.vita_maxima);
    }

    for mut text in xp_query.iter_mut() {
        text.0 = format!("Experientia: {} / {} XP", status.experientia, status.gradus.xp_proxima());
    }

    for mut text in gradus_query.iter_mut() {
        text.0 = format!("Gradus: {} ({})", status.gradus.titulus(), status.gradus.descriptio());
    }
}
