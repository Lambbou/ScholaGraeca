use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GradusCivicus {
    Apothetes = 1,   // Naufragé / Sans cité
    Ephebus = 2,     // Jeune apprenti
    Polites = 3,     // Citoyen libre
    Philosophus = 4, // Sage philosophe
    Archon = 5,      // Magistrat suprême
}

impl GradusCivicus {
    pub fn ex_numero(n: u32) -> Self {
        match n {
            1 => GradusCivicus::Apothetes,
            2 => GradusCivicus::Ephebus,
            3 => GradusCivicus::Polites,
            4 => GradusCivicus::Philosophus,
            _ => GradusCivicus::Archon,
        }
    }

    pub fn titulus(&self) -> &'static str {
        match self {
            GradusCivicus::Apothetes => "Apothetes",
            GradusCivicus::Ephebus => "Ephebus",
            GradusCivicus::Polites => "Polites",
            GradusCivicus::Philosophus => "Philosophus",
            GradusCivicus::Archon => "Archon",
        }
    }

    pub fn descriptio(&self) -> &'static str {
        match self {
            GradusCivicus::Apothetes => "Naufragus in litore Epiri eiectus, sine civitate.",
            GradusCivicus::Ephebus => "Tiro iuvenis chlamyde indutus, primis litteris initiatus.",
            GradusCivicus::Polites => "Civis liber togatus, agoram et leges Graecas callens.",
            GradusCivicus::Philosophus => "Vir sapiens papyro munitus, arcana linguae perspicax.",
            GradusCivicus::Archon => "Magistratus supremus laureatus, totius Helladis decus.",
        }
    }

    pub fn xp_proxima(&self) -> u32 {
        match self {
            GradusCivicus::Apothetes => 150,
            GradusCivicus::Ephebus => 450,
            GradusCivicus::Polites => 1100,
            GradusCivicus::Philosophus => 2500,
            GradusCivicus::Archon => 5000,
        }
    }

    pub fn vita_maxima(&self) -> u32 {
        match self {
            GradusCivicus::Apothetes => 100,
            GradusCivicus::Ephebus => 130,
            GradusCivicus::Polites => 180,
            GradusCivicus::Philosophus => 250,
            GradusCivicus::Archon => 350,
        }
    }

    pub fn damnum_basis(&self) -> u32 {
        match self {
            GradusCivicus::Apothetes => 10,
            GradusCivicus::Ephebus => 18,
            GradusCivicus::Polites => 30,
            GradusCivicus::Philosophus => 48,
            GradusCivicus::Archon => 75,
        }
    }

    pub fn multiplicator_combo(&self) -> f32 {
        match self {
            GradusCivicus::Apothetes => 1.0,
            GradusCivicus::Ephebus => 1.2,
            GradusCivicus::Polites => 1.5,
            GradusCivicus::Philosophus => 1.8,
            GradusCivicus::Archon => 2.2,
        }
    }

    pub fn proximus_gradus(&self) -> Option<GradusCivicus> {
        match self {
            GradusCivicus::Apothetes => Some(GradusCivicus::Ephebus),
            GradusCivicus::Ephebus => Some(GradusCivicus::Polites),
            GradusCivicus::Polites => Some(GradusCivicus::Philosophus),
            GradusCivicus::Philosophus => Some(GradusCivicus::Archon),
            GradusCivicus::Archon => None,
        }
    }
}
