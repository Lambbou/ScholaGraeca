use bevy::prelude::*;
use crate::state::AppState;

#[derive(Component)]
pub struct TitleScreenRoot;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct QuitGameButton;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Titulus), setup_title_screen)
            .add_systems(Update, (
                button_interaction_system,
            ).run_if(in_state(AppState::Titulus)))
            .add_systems(OnExit(AppState::Titulus), cleanup_title_screen);
    }
}

fn setup_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/ancient.ttf");

    commands.spawn((
        Camera2d,
        TitleScreenRoot,
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.06, 0.08, 0.14)), // Bleu nuit méditerranéen
        TitleScreenRoot,
    )).with_children(|parent| {
        // Couronne / Ornement
        parent.spawn((
            Text::new("❦  SCHOLA GRAECA  ❦"),
            TextFont {
                font: font_handle.clone(),
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.82, 0.25)), // Or antique
        ));

        parent.spawn((
            Text::new("ITER HEROIS IN TERRIS HELLADIS"),
            TextFont {
                font: font_handle.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.75, 0.85, 0.95)),
        ));

        parent.spawn((
            Text::new("Disce litteras et verba Graeca per sermonem Latinum"),
            TextFont {
                font: font_handle.clone(),
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgba(0.8, 0.8, 0.8, 0.8)),
        ));

        // Bouton : Commencer l'aventure
        parent.spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(55.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.20, 0.45, 0.70)),
            BorderColor(Color::srgb(0.95, 0.82, 0.25)),
            StartGameButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("INCIPE CURSUM"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });

        // Mention d'instructions
        parent.spawn((
            Text::new("Preme 'INCIPE CURSUM' ad litus Epiri navigandum"),
            TextFont {
                font: font_handle.clone(),
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::srgba(0.6, 0.6, 0.6, 0.7)),
        ));
    });
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&StartGameButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color, is_start) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if is_start.is_some() {
                    next_state.set(AppState::TabulaGeographica);
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.30, 0.60, 0.90));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.20, 0.45, 0.70));
            }
        }
    }
}

fn cleanup_title_screen(
    mut commands: Commands,
    query: Query<Entity, With<TitleScreenRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
