//! @id     miyuracine_rs1024
//! @do     calculer_somme_rs1024
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « rs1024 ».

//! Somme de contrôle RS1024 de SLIP-39 (Reed-Solomon sur GF(1024), 3 mots),
//! transcription exacte du code Python de la spécification (section « Checksum »).
//!
//! Aucune correction d'erreur n'est tentée (recommandation de la spécification) :
//! une part invalide est refusée.

/// Générateurs du code (spécification SLIP-39).
const GEN: [u32; 10] = [
    0x00e0_e040,
    0x01c1_c080,
    0x0383_8100,
    0x0707_0200,
    0x0e0e_0009,
    0x1c0c_2412,
    0x3808_6c24,
    0x3090_fc48,
    0x21b1_f890,
    0x03f3_f120,
];

/// Chaîne de personnalisation quand `ext` = 0.
pub(crate) const PERSONNALISATION: &[u8] = b"shamir";
/// Chaîne de personnalisation quand `ext` = 1 (sauvegarde extensible).
pub(crate) const PERSONNALISATION_EXTENSIBLE: &[u8] = b"shamir_extendable";

fn polymod(valeurs: impl Iterator<Item = u32>) -> u32 {
    let mut chk: u32 = 1;
    for v in valeurs {
        let b = chk >> 20;
        chk = ((chk & 0x000f_ffff) << 10) ^ v;
        for (i, g) in GEN.iter().enumerate() {
            if (b >> i) & 1 == 1 {
                chk ^= g;
            }
        }
    }
    chk
}

fn personnalisation(extensible: bool) -> &'static [u8] {
    if extensible {
        PERSONNALISATION_EXTENSIBLE
    } else {
        PERSONNALISATION
    }
}

/// Calcule les 3 mots de contrôle de `donnees` (mots de 10 bits).
pub(crate) fn creer(extensible: bool, donnees: &[u16]) -> [u16; 3] {
    let valeurs = personnalisation(extensible)
        .iter()
        .map(|&c| u32::from(c))
        .chain(donnees.iter().map(|&m| u32::from(m)))
        .chain([0, 0, 0]);
    let p = polymod(valeurs) ^ 1;
    [
        ((p >> 20) & 1023) as u16,
        ((p >> 10) & 1023) as u16,
        (p & 1023) as u16,
    ]
}

/// Vérifie une part complète (données + 3 mots de contrôle).
pub(crate) fn verifier(extensible: bool, mots: &[u16]) -> bool {
    let valeurs = personnalisation(extensible)
        .iter()
        .map(|&c| u32::from(c))
        .chain(mots.iter().map(|&m| u32::from(m)));
    polymod(valeurs) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creer_puis_verifier() {
        for ext in [false, true] {
            let mut mots: Vec<u16> = (0..17u16).map(|i| (i * 61) % 1024).collect();
            let c = creer(ext, &mots);
            mots.extend_from_slice(&c);
            assert!(verifier(ext, &mots));
            // La personnalisation fait partie du contrôle.
            assert!(!verifier(!ext, &mots));
            // Toute altération d'un mot est détectée.
            for i in 0..mots.len() {
                let mut altere = mots.clone();
                altere[i] ^= 1;
                assert!(!verifier(ext, &altere));
            }
        }
    }
}
