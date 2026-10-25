//! @id     miyauth_appareil
//! @do     deriver_cle_appareil_par_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — une clé d'appareil par COG de destination (non-corrélation complète), liaison prouvée au COG d'origine.

//! Une clé d'appareil par COG de destination (point ouvert d'A-003, lot 2 §5 :
//! « une clé d'appareil par COG visité — non-corrélation complète »).
//!
//! Jusqu'ici `cle_appareil` était commune à tous les passeports d'un appareil :
//! deux COG visités pouvaient relier leurs visiteurs en comparant cette clé.
//! Ici l'appareil présente à chaque COG une clé **distincte**, dérivée de façon
//! déterministe d'une graine locale, comme l'identifiant par paire
//! ([`crate::pairwise`]) :
//!
//! ```text
//! sel    = "miyukini/miyauth/cle-appareil/v1"
//! ikm    = graine d'appareil (32 o, sur l'appareil seulement, jamais exportée)
//! info   = "cog:" || cog_id de destination (26 o, longueur fixe)
//! sortie = 32 o = graine Ed25519 de la clé d'appareil pour cette destination
//! ```
//!
//! - **Stable** : même graine + même destination → même clé (aucun état à tenir).
//! - **Non reliable** : sans la graine, deux clés publiques pour deux COG sont
//!   celles de graines indistinguables de l'aléa (HKDF = PRF) ; les champs du
//!   passeport qui dépendent de l'appareil ou du titulaire (`sujet`,
//!   `cle_appareil`, `serie`) diffèrent donc tous d'un COG à l'autre.
//!
//! # Liaison au COG d'origine
//!
//! Le COG d'origine connaît la **clé principale** de l'appareil (celle de
//! l'appairage). Pour émettre un passeport portant une clé de destination, il
//! exige une [`DemandeLiaison`] : le message
//!
//! ```text
//! "miyukini/miyauth/liaison-appareil/v1" || 0x00 || cog d'origine (26) || destination (26) || clé de destination (32)
//! ```
//!
//! signé par la clé **principale** (autorisation : seul l'appareil appairé lie une
//! clé à son titulaire) **et** par la clé **de destination** (possession : on ne
//! lie pas la clé d'autrui). Les deux signatures sont vérifiées (`verify_strict`)
//! ; une clé de destination égale à la clé principale est refusée (elle
//! corrélerait le COG visité avec le COG d'origine et tout autre COG).
//!
//! Seul le COG d'origine voit la liaison ; le COG de destination ne reçoit que le
//! passeport (format inchangé, lot 2 §5) et vérifie la possession de la clé de
//! destination comme avant.
//!
//! # Révocation
//!
//! Révoquer l'appareil chez son COG d'origine = révoquer sa clé principale **et**
//! chaque clé de destination liée ([`LiaisonVerifiee::cible_revocation`]) : le COG
//! d'origine conserve les liaisons qu'il a acceptées pour le faire en un geste.
//!
//! Choix d'implémentation non fixés par la doc gelée (sel, info, encodage du
//! message de liaison, double signature) : **à consigner par amendement**.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use hkdf::Hkdf;
use miyusceau::CleSignature;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use zeroize::Zeroize;

use crate::pairwise::SecretTitulaire;
use crate::passeport::{emettre_passeport, DemandePasseport, RefusIdentite};
use crate::revocation::CibleRevocation;
use crate::titres::{CogId, Portees};

/// Sel de domaine de la dérivation des clés d'appareil par destination (versionné).
pub const SEL_CLE_APPAREIL: &[u8] = b"miyukini/miyauth/cle-appareil/v1";
/// Domaine de séparation du message de liaison (versionné).
pub const DOMAINE_LIAISON: &[u8] = b"miyukini/miyauth/liaison-appareil/v1";

/// Graine d'appareil (32 octets), détenue par l'appareil seul. Effacée de la
/// mémoire à la destruction ; jamais affichée.
#[derive(Clone)]
pub struct GraineAppareil([u8; 32]);

impl Drop for GraineAppareil {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl GraineAppareil {
    /// Nouvelle graine aléatoire (aléa du système).
    pub fn generer() -> Self {
        let mut octets = [0u8; 32];
        OsRng.fill_bytes(&mut octets);
        Self(octets)
    }

    /// Reconstruit une graine depuis son stockage (déchiffré par l'appelant).
    pub fn depuis_octets(octets: [u8; 32]) -> Self {
        Self(octets)
    }

    /// Octets secrets (à chiffrer avant tout stockage).
    pub fn octets(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for GraineAppareil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GraineAppareil(<masqué>)")
    }
}

/// Clé d'appareil à présenter au COG `destination` (dérivation déterministe).
pub fn cle_appareil_pour(graine: &GraineAppareil, destination: &CogId) -> Result<CleSignature, RefusIdentite> {
    let hk = Hkdf::<Sha256>::new(Some(SEL_CLE_APPAREIL), &graine.0);
    let mut info = Vec::with_capacity(4 + destination.as_str().len());
    info.extend_from_slice(b"cog:");
    info.extend_from_slice(destination.as_str().as_bytes());
    let mut sortie = [0u8; 32];
    // 32 o ≤ 255 × 32 o : ne peut pas échouer ; refus par principe sinon (C-10, fail-closed).
    hk.expand(&info, &mut sortie).map_err(|_| RefusIdentite::ParametreInvalide("hkdf"))?;
    let cle = CleSignature::depuis_graine(&sortie);
    sortie.zeroize();
    Ok(cle)
}

/// Octets signés de la liaison :
/// `DOMAINE_LIAISON || 0x00 || origine (26) || destination (26) || cle_destination (32)`.
///
/// `cog_id` de longueur fixe (26) : encodage non ambigu.
pub fn message_liaison(origine: &CogId, destination: &CogId, cle_destination: &[u8; 32]) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_LIAISON.len() + 1 + 2 * 26 + 32);
    m.extend_from_slice(DOMAINE_LIAISON);
    m.push(0x00);
    m.extend_from_slice(origine.as_str().as_bytes());
    m.extend_from_slice(destination.as_str().as_bytes());
    m.extend_from_slice(cle_destination);
    m
}

/// Demande de liaison d'une clé de destination, produite par l'appareil et
/// remise à son COG d'origine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemandeLiaison {
    /// COG de destination du futur passeport.
    pub destination: CogId,
    /// Clé publique d'appareil pour cette destination.
    pub cle_destination: [u8; 32],
    /// Signature du message de liaison par la clé principale (autorisation).
    pub signature_principale: [u8; 64],
    /// Signature du même message par la clé de destination (possession).
    pub signature_destination: [u8; 64],
}

fn signer(cle: &CleSignature, message: &[u8]) -> [u8; 64] {
    SigningKey::from_bytes(&cle.graine()).sign(message).to_bytes()
}

/// Côté appareil : dérive la clé pour `destination` et produit la demande de
/// liaison signée par `principale` (clé d'appairage) et par la clé dérivée.
pub fn lier_cle_appareil(
    principale: &CleSignature,
    graine: &GraineAppareil,
    origine: &CogId,
    destination: &CogId,
) -> Result<DemandeLiaison, RefusIdentite> {
    let cle = cle_appareil_pour(graine, destination)?;
    let cle_destination = cle.cle_publique();
    if cle_destination == principale.cle_publique() {
        return Err(RefusIdentite::ParametreInvalide("cle_destination"));
    }
    let message = message_liaison(origine, destination, &cle_destination);
    Ok(DemandeLiaison {
        destination: destination.clone(),
        cle_destination,
        signature_principale: signer(principale, &message),
        signature_destination: signer(&cle, &message),
    })
}

/// Liaison vérifiée par le COG d'origine (champs privés : la détenir prouve les contrôles).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaisonVerifiee {
    origine: CogId,
    destination: CogId,
    cle_principale: [u8; 32],
    cle_destination: [u8; 32],
}

impl LiaisonVerifiee {
    /// COG d'origine (émetteur du passeport).
    pub fn origine(&self) -> &CogId {
        &self.origine
    }

    /// COG de destination.
    pub fn destination(&self) -> &CogId {
        &self.destination
    }

    /// Clé principale de l'appareil (connue du seul COG d'origine).
    pub fn cle_principale(&self) -> &[u8; 32] {
        &self.cle_principale
    }

    /// Clé d'appareil présentée à la destination.
    pub fn cle_destination(&self) -> &[u8; 32] {
        &self.cle_destination
    }

    /// Cible de révocation de la clé de destination (à révoquer avec l'appareil).
    pub fn cible_revocation(&self) -> CibleRevocation {
        CibleRevocation::appareil(&self.cle_destination)
    }
}

fn cle_valide(cle: &[u8; 32]) -> Option<VerifyingKey> {
    VerifyingKey::from_bytes(cle).ok().filter(|k| !k.is_weak())
}

/// Côté COG d'origine : vérifie une demande de liaison émise par l'appareil dont
/// la clé principale (appairée, non révoquée — à contrôler par l'appelant) est
/// `cle_principale`.
pub fn verifier_liaison(
    origine: &CogId,
    cle_principale: &[u8; 32],
    demande: &DemandeLiaison,
) -> Result<LiaisonVerifiee, RefusIdentite> {
    let principale = cle_valide(cle_principale).ok_or(RefusIdentite::PossessionInvalide)?;
    let destination = cle_valide(&demande.cle_destination).ok_or(RefusIdentite::ParametreInvalide("cle_destination"))?;
    if &demande.cle_destination == cle_principale {
        // Même clé partout : corrélation avec le COG d'origine et les autres COG visités.
        return Err(RefusIdentite::ParametreInvalide("cle_destination"));
    }
    let message = message_liaison(origine, &demande.destination, &demande.cle_destination);
    principale
        .verify_strict(&message, &Signature::from_bytes(&demande.signature_principale))
        .map_err(|_| RefusIdentite::PossessionInvalide)?;
    destination
        .verify_strict(&message, &Signature::from_bytes(&demande.signature_destination))
        .map_err(|_| RefusIdentite::PossessionInvalide)?;
    Ok(LiaisonVerifiee {
        origine: origine.clone(),
        destination: demande.destination.clone(),
        cle_principale: *cle_principale,
        cle_destination: demande.cle_destination,
    })
}

/// Émet un passeport pour la destination d'une liaison vérifiée : `cle_appareil`
/// = clé de destination (jamais la clé principale).
pub fn emettre_passeport_lie(
    emetteur: &CogId,
    cle_emission: &CleSignature,
    secret: &SecretTitulaire,
    liaison: &LiaisonVerifiee,
    portees: Portees,
    pas_avant: u64,
    pas_apres: u64,
) -> Result<Vec<u8>, RefusIdentite> {
    if &liaison.origine != emetteur {
        return Err(RefusIdentite::EmetteurIncoherent);
    }
    emettre_passeport(
        emetteur,
        cle_emission,
        &DemandePasseport {
            secret,
            destination: liaison.destination.clone(),
            cle_appareil: liaison.cle_destination,
            portees,
            pas_avant,
            pas_apres,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cog(c: char) -> CogId {
        CogId::new(&c.to_string().repeat(26)).unwrap()
    }

    #[test]
    fn derivation_stable_et_distincte() {
        let g = GraineAppareil::depuis_octets([7; 32]);
        let a1 = cle_appareil_pour(&g, &cog('a')).unwrap().cle_publique();
        let a2 = cle_appareil_pour(&g, &cog('a')).unwrap().cle_publique();
        let b = cle_appareil_pour(&g, &cog('b')).unwrap().cle_publique();
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        let autre = cle_appareil_pour(&GraineAppareil::depuis_octets([8; 32]), &cog('a')).unwrap().cle_publique();
        assert_ne!(a1, autre);
        assert_eq!(format!("{g:?}"), "GraineAppareil(<masqué>)");
    }

    #[test]
    fn message_liaison_encodage_fixe() {
        let m = message_liaison(&cog('a'), &cog('b'), &[9; 32]);
        assert_eq!(m.len(), DOMAINE_LIAISON.len() + 1 + 52 + 32);
        assert_eq!(m[DOMAINE_LIAISON.len()], 0);
    }
}
