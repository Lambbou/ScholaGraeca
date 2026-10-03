use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Littera {
    pub id: u32,
    pub majuscula: String,
    pub minuscula: String,
    pub nomen: String,
    pub trans: String,
    pub minuscula_finalis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verbum {
    pub graecum_maj: String,
    pub graecum_min: String,
    pub latinum: String,
    pub regio: u32,
}

#[derive(Debug, Clone)]
pub struct QuaestioCertaminis {
    pub enuntiatum: String,        // Ex: "Quae est haec littera Graeca?"
    pub signum_graecum: String,    // Ex: "Ω" ou "ΛΟΓΟΣ"
    pub optiones: Vec<String>,     // 4 choix
    pub index_rectus: usize,       // Index de la bonne réponse
    pub explicatio: String,        // Explication / étymologie
}

pub struct QuizGenerator {
    pub litterae: Vec<Littera>,
    pub verba: Vec<Verbum>,
}

impl QuizGenerator {
    pub fn novus(litterae_json: &str, verba_json: &str) -> Result<Self, serde_json::Error> {
        let litterae: Vec<Littera> = serde_json::from_str(litterae_json)?;
        let verba: Vec<Verbum> = serde_json::from_str(verba_json)?;
        Ok(Self { litterae, verba })
    }

    /// Génère une question sur les lettres adaptée à la région (1 = Alpha..Mu, 2 = Nu..Omega)
    pub fn genera_quaestionem_litterae(&self, regio: u32) -> QuaestioCertaminis {
        let mut rng = thread_rng();
        
        let selectae: Vec<&Littera> = if regio == 1 {
            self.litterae.iter().filter(|l| l.id <= 12).collect()
        } else {
            self.litterae.iter().filter(|l| l.id > 12).collect()
        };

        let lit_recta = selectae.choose(&mut rng).copied().unwrap_or(&self.litterae[0]);
        let is_minuscula = rand::random::<bool>();
        
        let signum = if is_minuscula {
            lit_recta.minuscula.clone()
        } else {
            lit_recta.majuscula.clone()
        };

        let recta_optio = format!("{} ({})", lit_recta.trans, lit_recta.nomen);

        let mut omnes_optiones: Vec<String> = self.litterae.iter()
            .map(|l| format!("{} ({})", l.trans, l.nomen))
            .filter(|o| o != &recta_optio)
            .collect();
        
        omnes_optiones.shuffle(&mut rng);
        let mut optiones = vec![recta_optio.clone()];
        for opt in omnes_optiones.into_iter().take(3) {
            optiones.push(opt);
        }
        optiones.shuffle(&mut rng);

        let index_rectus = optiones.iter().position(|r| r == &recta_optio).unwrap_or(0);

        QuaestioCertaminis {
            enuntiatum: "Quod est nomen et sonus huius litterae Graecae?".to_string(),
            signum_graecum: signum,
            optiones,
            index_rectus,
            explicatio: format!("Haec littera est {} ({}), translata '{}'.", lit_recta.nomen, lit_recta.majuscula, lit_recta.trans),
        }
    }

    /// Génère une question sur le vocabulaire
    pub fn genera_quaestionem_verbi(&self, regio: u32) -> QuaestioCertaminis {
        let mut rng = thread_rng();
        
        let mut selecta_verba: Vec<&Verbum> = self.verba.iter()
            .filter(|v| v.regio <= regio)
            .collect();
        if selecta_verba.is_empty() {
            selecta_verba = self.verba.iter().collect();
        }

        let verbum_rectum = selecta_verba.choose(&mut rng).copied().unwrap_or(&self.verba[0]);
        let is_minuscula = rand::random::<bool>();

        let signum = if is_minuscula {
            verbum_rectum.graecum_min.clone()
        } else {
            verbum_rectum.graecum_maj.clone()
        };

        let recta_optio = verbum_rectum.latinum.clone();

        let mut omnes_optiones: Vec<String> = self.verba.iter()
            .map(|v| v.latinum.clone())
            .filter(|l| l != &recta_optio)
            .collect();
        
        omnes_optiones.shuffle(&mut rng);
        let mut optiones = vec![recta_optio.clone()];
        for opt in omnes_optiones.into_iter().take(3) {
            optiones.push(opt);
        }
        optiones.shuffle(&mut rng);

        let index_rectus = optiones.iter().position(|r| r == &recta_optio).unwrap_or(0);

        QuaestioCertaminis {
            enuntiatum: "Quid significat hoc verbum Graecum Latine?".to_string(),
            signum_graecum: signum,
            optiones,
            index_rectus,
            explicatio: format!("Hoc verbum '{}' vertitur: {}.", verbum_rectum.graecum_maj, verbum_rectum.latinum),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_generator_creation() {
        let lit_json = r#"[{"id":1,"majuscula":"Α","minuscula":"α","nomen":"Alpha","trans":"A","minuscula_finalis":null}]"#;
        let verb_json = r#"[{"graecum_maj":"ΒΙΟΣ","graecum_min":"βιος","latinum":"BIOS (Vita)","regio":1}]"#;

        let qz = QuizGenerator::novus(lit_json, verb_json).unwrap();
        assert_eq!(qz.litterae.len(), 1);
        assert_eq!(qz.verba.len(), 1);
    }
}
