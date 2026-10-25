//! @id     maia_federee_plongement
//! @do     calculer_plongement_local
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « plongement » (embeddings toujours recalculés localement, EX-62).

//! Plongements **toujours recalculés localement** (IA-FEDEREE §2, EX-62).
//!
//! [`PlongementLocal`] n'a qu'un constructeur, [`PlongementLocal::calculer`],
//! qui part du texte normalisé : aucune API n'accepte un vecteur fourni par un
//! tiers, et le format Fiche n'a pas de champ vecteur (tag inconnu = refus).
//!
//! Implémentation de référence déterministe (hachage de mots et bigrammes de
//! caractères, 256 dimensions, norme L2) ; un modèle d'embedding local pourra
//! la remplacer derrière le même type.

use sha2::{Digest, Sha256};

use crate::normalisation::cle_comparaison;

/// Dimension du plongement.
pub const DIMENSION: usize = 256;

/// Plongement calculé localement.
#[derive(Debug, Clone, PartialEq)]
pub struct PlongementLocal {
    composantes: Vec<f32>,
}

fn indice(jeton: &str) -> usize {
    let h = Sha256::digest(jeton.as_bytes());
    (usize::from(h[0]) << 8 | usize::from(h[1])) % DIMENSION
}

impl PlongementLocal {
    /// Calcule le plongement d'un texte (normalisé puis mis en minuscules).
    pub fn calculer(texte: &str) -> Self {
        let cle = cle_comparaison(texte);
        let mut composantes = vec![0f32; DIMENSION];
        for mot in cle.split(|c: char| !c.is_alphanumeric()).filter(|m| !m.is_empty()) {
            composantes[indice(mot)] += 1.0;
            let lettres: Vec<char> = mot.chars().collect();
            for paire in lettres.windows(2) {
                let bigramme: String = paire.iter().collect();
                composantes[indice(&format!("#{bigramme}"))] += 0.25;
            }
        }
        let norme = composantes.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norme > 0.0 {
            for x in &mut composantes {
                *x /= norme;
            }
        }
        Self { composantes }
    }

    /// Similarité cosinus (vecteurs normés).
    pub fn similarite(&self, autre: &Self) -> f32 {
        self.composantes.iter().zip(&autre.composantes).map(|(a, b)| a * b).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministe_et_ordonne() {
        let q = PlongementLocal::calculer("capitale de la France");
        assert_eq!(q, PlongementLocal::calculer("Capitale de la  France."));
        let proche = PlongementLocal::calculer("Paris est la capitale de la France");
        let loin = PlongementLocal::calculer("recette de la tarte aux pommes");
        assert!(q.similarite(&proche) > q.similarite(&loin));
    }
}
