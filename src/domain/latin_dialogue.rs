use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogusPassus {
    pub passus: u32,
    pub npc_verba: String,
    pub notio: String,
    pub lemmata: Vec<String>,
    pub exemplum: String,
    pub xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestusNpc {
    pub id: String,
    pub regio: u32,
    pub npc_nomen: String,
    pub titulus: String,
    pub dialogi: Vec<DialogusPassus>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DialogusIudicium {
    Optime(String),   // Réponse excellente avec félicitations latines
    Admissum(String), // Réponse acceptée
    Incompletum(String), // Réponse insuffisante avec indice bienveillant
}

pub fn examina_responsum(saisie: &str, passus: &DialogusPassus) -> DialogusIudicium {
    let purum = saisie.to_lowercase();
    let purum = purum.trim();

    if purum.is_empty() {
        return DialogusIudicium::Incompletum(format!("Nihil scripsisti! Exemplum: {}", passus.exemplum));
    }

    let mut lemmata_reperta = 0;
    for lemma in &passus.lemmata {
        // Recherche avec regex souple pour matcher le radical/lemme
        let pattern = format!(r"(?i)\b{}\w*", regex::escape(lemma));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(purum) {
                lemmata_reperta += 1;
            }
        }
    }

    if lemmata_reperta >= 2 {
        DialogusIudicium::Optime("Optime et latine respondisti! Sapientia tua placet.".to_string())
    } else if lemmata_reperta == 1 {
        DialogusIudicium::Admissum("Bene! Verbum rectum adhibuisti.".to_string())
    } else {
        DialogusIudicium::Incompletum(format!(
            "Verba nondum quadrant. Cura ut dicitur: '{}'",
            passus.exemplum
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_validation_exact() {
        let passus = DialogusPassus {
            passus: 1,
            npc_verba: "Quis es?".to_string(),
            notio: "Identitas".to_string(),
            lemmata: vec!["salve".to_string(), "naufragus".to_string(), "sum".to_string()],
            exemplum: "Salve! Naufragus sum.".to_string(),
            xp: 40,
        };

        let res = examina_responsum("Salve, naufragus sum in litore", &passus);
        assert!(matches!(res, DialogusIudicium::Optime(_)));
    }

    #[test]
    fn test_dialogue_validation_partial() {
        let passus = DialogusPassus {
            passus: 1,
            npc_verba: "Quis es?".to_string(),
            notio: "Identitas".to_string(),
            lemmata: vec!["salve".to_string(), "naufragus".to_string()],
            exemplum: "Salve! Naufragus sum.".to_string(),
            xp: 40,
        };

        let res = examina_responsum("Ego salve dico", &passus);
        assert!(matches!(res, DialogusIudicium::Admissum(_)));
    }

    #[test]
    fn test_dialogue_validation_failure() {
        let passus = DialogusPassus {
            passus: 1,
            npc_verba: "Quis es?".to_string(),
            notio: "Identitas".to_string(),
            lemmata: vec!["salve".to_string(), "naufragus".to_string()],
            exemplum: "Salve! Naufragus sum.".to_string(),
            xp: 40,
        };

        let res = examina_responsum("Bonjour je suis perdu", &passus);
        assert!(matches!(res, DialogusIudicium::Incompletum(_)));
    }
}
