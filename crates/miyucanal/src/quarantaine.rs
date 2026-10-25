//! @id     miyucanal_quarantaine
//! @do     persister_quarantaine_scellee
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — module « quarantaine » (liste en attente persistée, première vue, HMAC-SHA256 par enregistrement).

//! Persistance de la quarantaine (amendement A-006, décision du propriétaire 3).
//!
//! La liste des annonces en attente est persistée avec l'heure de **première
//! vue locale** (heure de confiance) ; après un redémarrage, le décompte des
//! 48 h **reprend** au lieu de repartir.
//!
//! # Format (big-endian, taille exacte)
//!
//! ```text
//! "MCQT" | version 0x01 | registre (32) | n (4, ≤ 4 096)
//! n × ( identité de l'annonce (32) | index (8) | vue_le (8) | mac (32) )
//! mac = HMAC-SHA256(K, "MCQT" | 0x01 | registre | identité | index | vue_le)
//! K   = HMAC-SHA256(secret local du COG, "miyukini/miyucanal/quarantaine/v1")
//! ```
//!
//! # Fail-closed
//!
//! - Enregistrement dont le MAC est faux : **ignoré** ⇒ l'annonce concernée
//!   repart pour une quarantaine complète.
//! - État illisible (absent, tronqué, autre version, autre registre, `n` hors
//!   bornes) : **tout** est ignoré ⇒ toutes les annonces repartent.
//! - Une quarantaine n'est **jamais raccourcie** : deux enregistrements
//!   authentiques pour la même annonce ⇒ la première vue la plus **tardive**.
//!
//! Limite (consignée par A-006) : un accès en écriture au disque peut
//! supprimer un enregistrement (relance) ou rejouer un enregistrement
//! authentique plus ancien de la même annonce, qui porte sa vraie première
//! vue : jamais moins de 48 h depuis la première vue locale.

use std::collections::HashMap;

use hmac::digest::{Key, KeyInit};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroize;

type HmacSha256 = Hmac<Sha256>;

const MAGIC: &[u8; 4] = b"MCQT";
const VERSION: u8 = 0x01;
const ETIQUETTE: &[u8] = b"miyukini/miyucanal/quarantaine/v1";
const TAILLE_ENTETE: usize = 4 + 1 + 32 + 4;
const TAILLE_ENREGISTREMENT: usize = 32 + 8 + 8 + 32;

/// Nombre maximal d'enregistrements d'un état de quarantaine.
pub const MAX_ENREGISTREMENTS_QUARANTAINE: usize = 4_096;

/// Clé de scellement local de la quarantaine, dérivée du secret local du COG.
///
/// Le secret ne quitte jamais la zone COG ; la clé est effacée à la libération.
pub struct CleQuarantaine {
    cle: [u8; 32],
}

impl CleQuarantaine {
    /// Dérive la clé : `HMAC-SHA256(secret_local, "miyukini/miyucanal/quarantaine/v1")`.
    pub fn deriver(secret_local: &[u8; 32]) -> Self {
        let mut mac = hmac_cle_32(secret_local);
        mac.update(ETIQUETTE);
        Self { cle: mac.finalize().into_bytes().into() }
    }

    fn mac(&self) -> HmacSha256 {
        hmac_cle_32(&self.cle)
    }
}

/// HMAC-SHA256 sur une clé de 32 octets, sans chemin d'échec.
///
/// RFC 2104 complète toute clé plus courte que le bloc (64 o) par des zéros :
/// la clé complétée donne exactement le même MAC que `new_from_slice(cle)`,
/// mais par le constructeur infaillible `KeyInit::new`.
fn hmac_cle_32(cle: &[u8; 32]) -> HmacSha256 {
    let mut bloc = Key::<HmacSha256>::default();
    bloc[..32].copy_from_slice(cle);
    let mac = <HmacSha256 as KeyInit>::new(&bloc);
    bloc.as_mut_slice().zeroize();
    mac
}

impl Drop for CleQuarantaine {
    fn drop(&mut self) {
        self.cle.zeroize();
    }
}

impl std::fmt::Debug for CleQuarantaine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CleQuarantaine(..)")
    }
}

/// Enregistrement d'une annonce en attente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VuePersistee {
    /// Identité de l'annonce (SHA-256 des octets signés).
    pub identite: [u8; 32],
    /// Index de l'annonce dans le registre officiel.
    pub index: u64,
    /// Heure de confiance de la première vue locale.
    pub vue_le: u64,
}

/// Résultat de la lecture d'un état de quarantaine.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EtatQuarantaine {
    /// Enregistrements authentiques, par identité d'annonce.
    pub vues: HashMap<[u8; 32], VuePersistee>,
    /// Enregistrements refusés (MAC faux).
    pub refuses: u64,
    /// État entier refusé (absent, mal formé, autre registre, autre version).
    pub illisible: bool,
}

impl EtatQuarantaine {
    /// Vrai si une partie de l'état a été refusée.
    pub fn altere(&self) -> bool {
        self.illisible || self.refuses > 0
    }
}

fn calculer_mac(cle: &CleQuarantaine, registre: &[u8; 32], v: &VuePersistee) -> HmacSha256 {
    let mut mac = cle.mac();
    mac.update(MAGIC);
    mac.update(&[VERSION]);
    mac.update(registre);
    mac.update(&v.identite);
    mac.update(&v.index.to_be_bytes());
    mac.update(&v.vue_le.to_be_bytes());
    mac
}

/// Scelle l'état de quarantaine (au plus [`MAX_ENREGISTREMENTS_QUARANTAINE`]
/// enregistrements ; au-delà, l'excédent n'est pas persisté et repartira pour
/// une quarantaine complète).
pub fn sceller_quarantaine(registre: &[u8; 32], vues: &[VuePersistee], cle: &CleQuarantaine) -> Vec<u8> {
    let vues = &vues[..vues.len().min(MAX_ENREGISTREMENTS_QUARANTAINE)];
    let mut s = Vec::with_capacity(TAILLE_ENTETE + TAILLE_ENREGISTREMENT * vues.len());
    s.extend_from_slice(MAGIC);
    s.push(VERSION);
    s.extend_from_slice(registre);
    s.extend_from_slice(&(vues.len() as u32).to_be_bytes());
    for v in vues {
        s.extend_from_slice(&v.identite);
        s.extend_from_slice(&v.index.to_be_bytes());
        s.extend_from_slice(&v.vue_le.to_be_bytes());
        s.extend_from_slice(&calculer_mac(cle, registre, v).finalize().into_bytes());
    }
    s
}

fn u64_be(octets: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(octets);
    u64::from_be_bytes(b)
}

/// Lit un état de quarantaine (voir module : jamais d'erreur, tout refus
/// relance la quarantaine des annonces concernées).
pub fn ouvrir_quarantaine(octets: &[u8], registre: &[u8; 32], cle: &CleQuarantaine) -> EtatQuarantaine {
    let illisible = EtatQuarantaine { illisible: true, ..EtatQuarantaine::default() };
    if octets.len() < TAILLE_ENTETE || &octets[0..4] != MAGIC || octets[4] != VERSION || &octets[5..37] != registre {
        return illisible;
    }
    let n = u32::from_be_bytes([octets[37], octets[38], octets[39], octets[40]]) as usize;
    if n > MAX_ENREGISTREMENTS_QUARANTAINE || octets.len() != TAILLE_ENTETE + n * TAILLE_ENREGISTREMENT {
        return illisible;
    }
    let mut etat = EtatQuarantaine::default();
    for e in octets[TAILLE_ENTETE..].as_chunks::<TAILLE_ENREGISTREMENT>().0 {
        let mut identite = [0u8; 32];
        identite.copy_from_slice(&e[0..32]);
        let v = VuePersistee { identite, index: u64_be(&e[32..40]), vue_le: u64_be(&e[40..48]) };
        if calculer_mac(cle, registre, &v).verify_slice(&e[48..80]).is_err() {
            etat.refuses += 1;
            continue;
        }
        // Jamais raccourcir : la première vue la plus tardive l'emporte.
        let retenue = etat.vues.entry(identite).or_insert(v);
        if v.vue_le > retenue.vue_le {
            *retenue = v;
        }
    }
    etat
}

#[cfg(test)]
mod tests {
    use super::*;

    const REG: [u8; 32] = [7; 32];

    fn vues() -> Vec<VuePersistee> {
        vec![
            VuePersistee { identite: [1; 32], index: 0, vue_le: 1_000 },
            VuePersistee { identite: [2; 32], index: 3, vue_le: 2_000 },
        ]
    }

    #[test]
    fn hmac_cle_32_identique_a_new_from_slice() {
        let cle = [0xA5; 32];
        let mut a = hmac_cle_32(&cle);
        let mut b = <HmacSha256 as Mac>::new_from_slice(&cle).unwrap();
        a.update(b"essai");
        b.update(b"essai");
        assert_eq!(a.finalize().into_bytes(), b.finalize().into_bytes());
    }

    #[test]
    fn aller_retour() {
        let cle = CleQuarantaine::deriver(&[9; 32]);
        let etat = ouvrir_quarantaine(&sceller_quarantaine(&REG, &vues(), &cle), &REG, &cle);
        assert!(!etat.altere());
        assert_eq!(etat.vues.len(), 2);
        assert_eq!(etat.vues[&[2; 32]].vue_le, 2_000);
        assert_eq!(format!("{cle:?}"), "CleQuarantaine(..)");
    }

    #[test]
    fn enregistrement_altere_refuse_seul() {
        let cle = CleQuarantaine::deriver(&[9; 32]);
        let mut o = sceller_quarantaine(&REG, &vues(), &cle);
        // vue_le du premier enregistrement avancée de 24 h : MAC faux.
        o[TAILLE_ENTETE + 40 + 4] ^= 0x01;
        let etat = ouvrir_quarantaine(&o, &REG, &cle);
        assert_eq!((etat.refuses, etat.illisible), (1, false));
        assert_eq!(etat.vues.keys().copied().collect::<Vec<_>>(), vec![[2; 32]]);
    }

    #[test]
    fn etat_illisible_refuse_en_entier() {
        let cle = CleQuarantaine::deriver(&[9; 32]);
        let o = sceller_quarantaine(&REG, &vues(), &cle);
        for mauvais in [
            Vec::new(),
            o[..o.len() - 1].to_vec(),
            [o.as_slice(), &[0]].concat(),
            {
                let mut v = o.clone();
                v[4] = 2;
                v
            },
        ] {
            assert!(ouvrir_quarantaine(&mauvais, &REG, &cle).illisible);
        }
        // Autre registre, autre clé.
        assert!(ouvrir_quarantaine(&o, &[8; 32], &cle).illisible);
        let autre = ouvrir_quarantaine(&o, &REG, &CleQuarantaine::deriver(&[1; 32]));
        assert_eq!((autre.refuses, autre.vues.len()), (2, 0));
    }

    #[test]
    fn doublon_garde_la_vue_la_plus_tardive() {
        let cle = CleQuarantaine::deriver(&[9; 32]);
        let v = [
            VuePersistee { identite: [1; 32], index: 0, vue_le: 5_000 },
            VuePersistee { identite: [1; 32], index: 0, vue_le: 1_000 },
        ];
        let etat = ouvrir_quarantaine(&sceller_quarantaine(&REG, &v, &cle), &REG, &cle);
        assert_eq!(etat.vues[&[1; 32]].vue_le, 5_000);
    }
}
