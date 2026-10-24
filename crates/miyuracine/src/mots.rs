//! @id     miyuracine_mots
//! @do     convertir_mots_slip39
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « mots ».

//! Liste de mots officielle SLIP-39 (1 024 mots, anglais uniquement).
//!
//! Fichier `mots-slip39.txt` copié à l'identique de
//! `satoshilabs/slips/slip-0039/wordlist.txt` ; son SHA-256 est contrôlé par
//! un test (`liste_officielle_inchangee`).

use std::sync::OnceLock;

use crate::erreurs::ErreurRacine;

const BRUT: &str = include_str!("mots-slip39.txt");

/// Nombre de mots (2¹⁰).
pub(crate) const NOMBRE_MOTS: usize = 1024;

fn liste() -> &'static [&'static str] {
    static LISTE: OnceLock<Vec<&'static str>> = OnceLock::new();
    LISTE.get_or_init(|| BRUT.lines().collect())
}

/// Mot d'indice `i` (< 1024).
pub(crate) fn mot(i: u16) -> &'static str {
    liste()[usize::from(i)]
}

/// Indice d'un mot (minuscules ASCII ; la casse est normalisée).
pub(crate) fn indice(mot: &str) -> Result<u16, ErreurRacine> {
    // Liste altérée (fichier embarqué tronqué) : refus de tout mot.
    if liste().len() != NOMBRE_MOTS || mot.is_empty() || mot.len() > 8 || !mot.is_ascii() {
        return Err(ErreurRacine::MotInconnu);
    }
    let minuscule = zeroize::Zeroizing::new(mot.to_ascii_lowercase());
    liste()
        .binary_search(&minuscule.as_str())
        .map(|i| i as u16)
        .map_err(|_| ErreurRacine::MotInconnu)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn liste_officielle_inchangee() {
        // SHA-256 de slip-0039/wordlist.txt (dépôt satoshilabs/slips, branche master).
        let attendu = "bcc4555340332d169718aed8bf31dd9d5248cb7da6e5d355140ef4f1e601eec3";
        let obtenu = Sha256::digest(BRUT.as_bytes())
            .iter()
            .fold(String::new(), |mut s, b| {
                use std::fmt::Write as _;
                let _ = write!(s, "{b:02x}");
                s
            });
        assert_eq!(obtenu, attendu);
    }

    #[test]
    fn liste_triee_unique_et_complete() {
        let l = liste();
        assert_eq!(l.len(), NOMBRE_MOTS);
        assert!(
            l.windows(2).all(|w| w[0] < w[1]),
            "liste triée sans doublon"
        );
        assert!(l.iter().all(|m| (4..=8).contains(&m.len())));
        for (i, m) in l.iter().enumerate() {
            assert_eq!(indice(m).unwrap(), i as u16);
            assert_eq!(mot(i as u16), *m);
        }
        assert_eq!(indice("Academic").unwrap(), indice("academic").unwrap());
        assert_eq!(indice("inconnu"), Err(ErreurRacine::MotInconnu));
        assert_eq!(indice(""), Err(ErreurRacine::MotInconnu));
    }
}
