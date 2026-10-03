use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::ranks::GradusCivicus;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HerosStatus {
    pub nomen: String,
    pub gradus: GradusCivicus,
    pub experientia: u32,
    pub vita: u32,
    pub vita_maxima: u32,
    pub regio_actualis: u32,
    pub monstra_devicta: Vec<String>,
    pub quaestiones_solutae: Vec<String>,
}

impl Default for HerosStatus {
    fn default() -> Self {
        let gradus = GradusCivicus::Apothetes;
        Self {
            nomen: "Heros Graecus".to_string(),
            gradus,
            experientia: 0,
            vita: gradus.vita_maxima(),
            vita_maxima: gradus.vita_maxima(),
            regio_actualis: 1,
            monstra_devicta: Vec::new(),
            quaestiones_solutae: Vec::new(),
        }
    }
}

impl HerosStatus {
    /// Ajoute de l'expérience et retourne le nouveau rang si un passage de niveau a eu lieu
    pub fn adde_experientiam(&mut self, quantitas: u32) -> Option<GradusCivicus> {
        self.experientia += quantitas;
        let mut novus_gradus = None;

        while let Some(proximus) = self.gradus.proximus_gradus() {
            if self.experientia >= self.gradus.xp_proxima() {
                self.gradus = proximus;
                self.vita_maxima = proximus.vita_maxima();
                self.vita = self.vita_maxima; // Soins complets lors de la promotion
                novus_gradus = Some(proximus);
            } else {
                break;
            }
        }

        novus_gradus
    }

    pub fn inflige_damnum(&mut self, damnum: u32) -> bool {
        if self.vita <= damnum {
            self.vita = 0;
            true // Mort du héros
        } else {
            self.vita -= damnum;
            false
        }
    }

    pub fn sana(&mut self, quantitas: u32) {
        self.vita = (self.vita + quantitas).min(self.vita_maxima);
    }
}

/// Marqueur pour l'entité du héros sur la carte
#[derive(Component)]
pub struct Heros;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ranks::GradusCivicus;

    #[test]
    fn test_hero_xp_progression() {
        let mut hero = HerosStatus::default();
        assert_eq!(hero.gradus, GradusCivicus::Apothetes);
        assert_eq!(hero.vita_maxima, 100);

        // Gain de 100 XP (insuffisant pour Ephebus qui demande 150)
        let promo = hero.adde_experientiam(100);
        assert_eq!(promo, None);
        assert_eq!(hero.gradus, GradusCivicus::Apothetes);

        // Gain de 60 XP supplémentaires (total 160 XP >= 150 -> Promotion en Ephebus)
        let promo = hero.adde_experientiam(60);
        assert_eq!(promo, Some(GradusCivicus::Ephebus));
        assert_eq!(hero.gradus, GradusCivicus::Ephebus);
        assert_eq!(hero.vita_maxima, 130);
        assert_eq!(hero.vita, 130);

        // Progression jusqu'au rang suprême d'Archon (>= 2500 XP)
        let promo = hero.adde_experientiam(2400);
        assert_eq!(promo, Some(GradusCivicus::Archon));
        assert_eq!(hero.gradus, GradusCivicus::Archon);
        assert_eq!(hero.vita_maxima, 350);
    }

    #[test]
    fn test_hero_damage_and_healing() {
        let mut hero = HerosStatus::default();
        let mort = hero.inflige_damnum(40);
        assert!(!mort);
        assert_eq!(hero.vita, 60);

        hero.sana(25);
        assert_eq!(hero.vita, 85);

        // Soins plafonnés aux PV Max
        hero.sana(100);
        assert_eq!(hero.vita, hero.vita_maxima);
    }
}
