use bevy::prelude::*;

/// Status ludi (États du jeu en Latin)
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Titulus,             // Menu principal / Titre (Menu Tituli)
    TabulaGeographica,   // Carte du monde / Déplacement libre (Overworld)
    Conloquium,          // Dialogue PNJ en Latin au clavier
    Certamen,            // Combat de monstre / Épreuve en Grec
    GradusPromotio,      // Célébration de passage de rang civique
}
