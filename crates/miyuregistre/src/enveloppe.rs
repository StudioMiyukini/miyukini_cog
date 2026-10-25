//! @id     miyuregistre_enveloppe
//! @do     definir_enveloppe_synchro_revocations
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — format partagé de l'enveloppe signée d'un envoi de synchro des révocations vers Origin (A-014) : octets signés, empreinte des entrées, bornes du protocole.

//! Enveloppe signée de la synchro des révocations vers Origin (A-013, A-014).
//!
//! Format **unique**, utilisé par l'émetteur (COG : [`crate::editeur`]) et par
//! le vérificateur (Origin, `POST /api/auth/revocations/sync`). Chaque envoi
//! (morceau ou tête) est signé (Ed25519, vérifié en `verify_strict`) par la
//! **clé de journal** du COG sur [`octets_enveloppe`] :
//!
//! ```text
//! "miyukini/origin/revocations/morceau/v1" ‖ 0x00     (DOMAINE_ENVELOPPE)
//! ‖ u16 BE longueur(cog_id) ‖ cog_id (UTF-8)
//! ‖ u64 BE depuis
//! ‖ u64 BE nombre d'entrées
//! ‖ SHA-256( ‖ pour chaque entrée : u32 BE longueur ‖ octets )   (32 o)
//! ‖ u8 tête présente (0 | 1) ‖ SHA-256(tête) ou 32 × 0x00         (33 o)
//! ‖ u64 BE echeance (secondes Unix)
//! ```
//!
//! Corps JSON : `{cog_id, depuis, tete?, entrees, echeance, signature}`
//! (titres et signature en base64 URL sans remplissage).

use miyauth::CogId;
use sha2::{Digest, Sha256};

/// Route de synchronisation des registres de révocation (Origin).
pub const ROUTE_REVOCATIONS: &str = "/api/auth/revocations/sync";
/// Domaine de séparation de l'enveloppe signée d'un envoi.
pub const DOMAINE_ENVELOPPE: &[u8] = b"miyukini/origin/revocations/morceau/v1";
/// Échéance maximale d'une enveloppe, comptée depuis l'heure d'Origin (15 min).
pub const DUREE_MAX_ENVELOPPE_SECS: u64 = 15 * 60;
/// Nombre maximal d'entrées par envoi (un morceau).
pub const MAX_ENTREES_SYNCHRO: usize = 4096;
/// Taille maximale du corps d'un envoi (tête + entrées, base64, JSON).
pub const TAILLE_MAX_CORPS_REVOCATIONS: usize = 2 * 1024 * 1024;

/// Empreinte des entrées d'un envoi : SHA-256 des entrées concaténées, chacune
/// préfixée de sa longueur (u32 BE) pour fixer les frontières.
pub fn empreinte_entrees(entrees: &[Vec<u8>]) -> [u8; 32] {
    let mut h = Sha256::new();
    for e in entrees {
        // Entrées bornées par le corps (≤ 2 Mio) : la longueur tient sur u32.
        h.update(u32::try_from(e.len()).unwrap_or(u32::MAX).to_be_bytes());
        h.update(e);
    }
    h.finalize().into()
}

/// Octets signés par la clé de journal du COG pour un envoi (voir le module).
pub fn octets_enveloppe(cog: &CogId, depuis: u64, tete: Option<&[u8]>, entrees: &[Vec<u8>], echeance: u64) -> Vec<u8> {
    let id = cog.as_str().as_bytes();
    let mut m = Vec::with_capacity(DOMAINE_ENVELOPPE.len() + id.len() + 100);
    m.extend_from_slice(DOMAINE_ENVELOPPE);
    m.push(0);
    // Un CogId est court (validé par `CogId::new`) : la longueur tient sur u16.
    m.extend_from_slice(&u16::try_from(id.len()).unwrap_or(u16::MAX).to_be_bytes());
    m.extend_from_slice(id);
    m.extend_from_slice(&depuis.to_be_bytes());
    m.extend_from_slice(&(entrees.len() as u64).to_be_bytes());
    m.extend_from_slice(&empreinte_entrees(entrees));
    if let Some(t) = tete {
        m.push(1);
        m.extend_from_slice(&Sha256::digest(t));
    } else {
        m.push(0);
        m.extend_from_slice(&[0u8; 32]);
    }
    m.extend_from_slice(&echeance.to_be_bytes());
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disposition_figee() {
        let cog = CogId::new(&"h".repeat(26)).unwrap();
        let entrees = vec![b"ab".to_vec(), b"c".to_vec()];
        let m = octets_enveloppe(&cog, 7, Some(b"tete"), &entrees, 99);
        let mut attendu = DOMAINE_ENVELOPPE.to_vec();
        attendu.push(0);
        attendu.extend_from_slice(&26u16.to_be_bytes());
        attendu.extend_from_slice("h".repeat(26).as_bytes());
        attendu.extend_from_slice(&7u64.to_be_bytes());
        attendu.extend_from_slice(&2u64.to_be_bytes());
        let mut h = Sha256::new();
        h.update(2u32.to_be_bytes());
        h.update(b"ab");
        h.update(1u32.to_be_bytes());
        h.update(b"c");
        attendu.extend_from_slice(&h.finalize());
        attendu.push(1);
        attendu.extend_from_slice(&Sha256::digest(b"tete"));
        attendu.extend_from_slice(&99u64.to_be_bytes());
        assert_eq!(m, attendu);
        // Frontières fixées : ["ab","c"] ≠ ["a","bc"] ; tête absente ≠ tête vide.
        assert_ne!(empreinte_entrees(&entrees), empreinte_entrees(&[b"a".to_vec(), b"bc".to_vec()]));
        assert_ne!(octets_enveloppe(&cog, 7, None, &[], 99), octets_enveloppe(&cog, 7, Some(b""), &[], 99));
    }
}
