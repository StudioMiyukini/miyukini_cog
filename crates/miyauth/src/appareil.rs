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
//! exige une [`DemandeLiaison`] : le message (format **v2**)
//!
//! ```text
//! "miyukini/miyauth/liaison-appareil/v2" || 0x00 || cog d'origine (26) || destination (26)
//!     || clé de destination (32) || pas_apres (u64 gros-boutiste, 8)
//! ```
//!
//! `pas_apres` est l'**échéance** de la demande, fixée par l'appareil : le COG
//! d'origine la refuse si elle est passée ou si elle dépasse son horloge de
//! plus de [`DUREE_MAX_LIAISON_SECS`] (5 min) + la tolérance d'horloge — une
//! demande interceptée n'est donc rejouable que quelques minutes. Changement
//! de format par rapport à la v1 (sans échéance, rejouable sans limite) : le
//! domaine passe en `v2`, une signature v1 ne vérifie pas.
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
//! - **Fail-closed** : [`verifier_liaison`] et [`emettre_passeport_lie`]
//!   exigent une [`SourceRevocation`] qui réponde `Ok(false)` pour la clé
//!   principale (et, à l'émission, pour la clé de destination) ; révoquée ou
//!   statut indisponible / périmé ⇒ refus. L'appelant ne peut plus l'oublier.
//! - **Cascade** : révoquer l'appareil chez son COG d'origine = révoquer sa clé
//!   principale **et** chaque clé de destination liée. Le COG d'origine retient
//!   les liaisons acceptées dans un [`RegistreLiaisons`] ([`accepter_liaison`]
//!   vérifie puis retient) ; [`RegistreLiaisons::revoquer_appareil`] rend toutes
//!   les cibles à inscrire au registre, en un geste.
//!
//! Choix d'implémentation non fixés par la doc gelée (sel, info, encodage du
//! message de liaison, double signature) : **à consigner par amendement**.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use hkdf::Hkdf;
use miyusceau::CleSignature;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use zeroize::Zeroize;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::RwLock;

use miyusceau::TOLERANCE_HORLOGE_SECS;

use crate::pairwise::SecretTitulaire;
use crate::passeport::{emettre_passeport, DemandePasseport, RefusIdentite};
use crate::revocation::{CibleRevocation, ErreurRevocation, SourceRevocation};
use crate::titres::{CogId, Portees};

/// Sel de domaine de la dérivation des clés d'appareil par destination (versionné).
pub const SEL_CLE_APPAREIL: &[u8] = b"miyukini/miyauth/cle-appareil/v1";
/// Domaine de séparation du message de liaison (versionné).
pub const DOMAINE_LIAISON: &[u8] = b"miyukini/miyauth/liaison-appareil/v2";
/// Durée de validité maximale d'une demande de liaison (5 min) : échéance
/// `pas_apres` au plus `maintenant + DUREE_MAX_LIAISON_SECS` (+ tolérance d'horloge).
pub const DUREE_MAX_LIAISON_SECS: u64 = 5 * 60;
/// Nombre maximal de liaisons retenues par clé principale ([`RegistreLiaisonsMemoire`]).
pub const MAX_LIAISONS_PAR_APPAREIL: usize = 4096;

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

/// Octets signés de la liaison (v2) :
/// `DOMAINE_LIAISON || 0x00 || origine (26) || destination (26) || cle_destination (32) || pas_apres (8, BE)`.
///
/// `cog_id` de longueur fixe (26) : encodage non ambigu.
pub fn message_liaison(origine: &CogId, destination: &CogId, cle_destination: &[u8; 32], pas_apres: u64) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_LIAISON.len() + 1 + 2 * 26 + 32 + 8);
    m.extend_from_slice(DOMAINE_LIAISON);
    m.push(0x00);
    m.extend_from_slice(origine.as_str().as_bytes());
    m.extend_from_slice(destination.as_str().as_bytes());
    m.extend_from_slice(cle_destination);
    m.extend_from_slice(&pas_apres.to_be_bytes());
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
    /// Échéance de la demande (secondes Unix), signée ; ≤ 5 min après son émission.
    pub pas_apres: u64,
    /// Signature du message de liaison par la clé principale (autorisation).
    pub signature_principale: [u8; 64],
    /// Signature du même message par la clé de destination (possession).
    pub signature_destination: [u8; 64],
}

fn signer(cle: &CleSignature, message: &[u8]) -> [u8; 64] {
    SigningKey::from_bytes(&cle.graine()).sign(message).to_bytes()
}

/// Côté appareil : dérive la clé pour `destination` et produit la demande de
/// liaison signée par `principale` (clé d'appairage) et par la clé dérivée,
/// valable jusqu'à `pas_apres` (au plus 5 min après l'envoi, sinon refusée).
pub fn lier_cle_appareil(
    principale: &CleSignature,
    graine: &GraineAppareil,
    origine: &CogId,
    destination: &CogId,
    pas_apres: u64,
) -> Result<DemandeLiaison, RefusIdentite> {
    let cle = cle_appareil_pour(graine, destination)?;
    let cle_destination = cle.cle_publique();
    if cle_destination == principale.cle_publique() {
        return Err(RefusIdentite::ParametreInvalide("cle_destination"));
    }
    let message = message_liaison(origine, destination, &cle_destination, pas_apres);
    Ok(DemandeLiaison {
        destination: destination.clone(),
        cle_destination,
        pas_apres,
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

/// Exige `Ok(false)` de `revocations` pour la clé `cle` émise sous `emetteur`
/// (révoquée ⇒ [`RefusIdentite::Revoque`], doute ⇒ [`RefusIdentite::Revocation`]).
fn exiger_non_revoquee(
    revocations: &dyn SourceRevocation,
    emetteur: &CogId,
    cle: &[u8; 32],
    maintenant: u64,
) -> Result<(), RefusIdentite> {
    let cible = CibleRevocation::appareil(cle);
    match revocations.est_revoque(emetteur, &cible, maintenant) {
        Ok(false) => Ok(()),
        Ok(true) => Err(RefusIdentite::Revoque(cible)),
        Err(e) => Err(RefusIdentite::Revocation(e)),
    }
}

/// Côté COG d'origine : vérifie une demande de liaison émise par l'appareil dont
/// la clé principale (appairée) est `cle_principale`, à l'instant `maintenant`
/// (horloge de confiance du COG).
///
/// Refus : échéance passée ou à plus de 5 min (+ tolérance), signature
/// invalide, clé corrélante, clé principale révoquée **ou statut de
/// révocation indisponible / périmé** (fail-closed).
pub fn verifier_liaison(
    origine: &CogId,
    cle_principale: &[u8; 32],
    demande: &DemandeLiaison,
    revocations: &dyn SourceRevocation,
    maintenant: u64,
) -> Result<LiaisonVerifiee, RefusIdentite> {
    let limite = maintenant.saturating_add(DUREE_MAX_LIAISON_SECS).saturating_add(TOLERANCE_HORLOGE_SECS);
    if demande.pas_apres < maintenant || demande.pas_apres > limite {
        return Err(RefusIdentite::ParametreInvalide("pas_apres"));
    }
    let principale = cle_valide(cle_principale).ok_or(RefusIdentite::PossessionInvalide)?;
    let destination = cle_valide(&demande.cle_destination).ok_or(RefusIdentite::ParametreInvalide("cle_destination"))?;
    if &demande.cle_destination == cle_principale {
        // Même clé partout : corrélation avec le COG d'origine et les autres COG visités.
        return Err(RefusIdentite::ParametreInvalide("cle_destination"));
    }
    let message = message_liaison(origine, &demande.destination, &demande.cle_destination, demande.pas_apres);
    principale
        .verify_strict(&message, &Signature::from_bytes(&demande.signature_principale))
        .map_err(|_| RefusIdentite::PossessionInvalide)?;
    destination
        .verify_strict(&message, &Signature::from_bytes(&demande.signature_destination))
        .map_err(|_| RefusIdentite::PossessionInvalide)?;
    exiger_non_revoquee(revocations, origine, cle_principale, maintenant)?;
    Ok(LiaisonVerifiee {
        origine: origine.clone(),
        destination: demande.destination.clone(),
        cle_principale: *cle_principale,
        cle_destination: demande.cle_destination,
    })
}

/// Émet un passeport pour la destination d'une liaison vérifiée : `cle_appareil`
/// = clé de destination (jamais la clé principale).
///
/// La révocation est **recontrôlée à l'émission** (une liaison peut avoir été
/// vérifiée plus tôt) : clé principale et clé de destination doivent être
/// `Ok(false)` dans `revocations` à `maintenant`, sinon refus.
#[allow(clippy::too_many_arguments)]
pub fn emettre_passeport_lie(
    emetteur: &CogId,
    cle_emission: &CleSignature,
    secret: &SecretTitulaire,
    liaison: &LiaisonVerifiee,
    portees: Portees,
    pas_avant: u64,
    pas_apres: u64,
    revocations: &dyn SourceRevocation,
    maintenant: u64,
) -> Result<Vec<u8>, RefusIdentite> {
    if &liaison.origine != emetteur {
        return Err(RefusIdentite::EmetteurIncoherent);
    }
    exiger_non_revoquee(revocations, emetteur, &liaison.cle_principale, maintenant)?;
    exiger_non_revoquee(revocations, emetteur, &liaison.cle_destination, maintenant)?;
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

/// Persistance des liaisons acceptées par le COG d'origine (cascade de
/// révocation). Une implémentation durable (base du COG) doit rendre
/// [`RegistreLiaisons::retenir`] **durable avant** l'émission du passeport.
pub trait RegistreLiaisons {
    /// Retient une liaison acceptée (idempotent). Erreur ⇒ l'émission doit être refusée.
    fn retenir(&self, liaison: &LiaisonVerifiee) -> Result<(), ErreurRevocation>;

    /// Clés de destination liées à `principale` (toutes celles jamais retenues).
    fn cles_liees(&self, principale: &[u8; 32]) -> Result<Vec<[u8; 32]>, ErreurRevocation>;

    /// Révocation de l'appareil **en un geste** : cibles de la clé principale
    /// et de chaque clé de destination liée, à inscrire au registre du COG.
    /// Erreur (persistance illisible) : aucune liste partielle n'est rendue.
    fn revoquer_appareil(&self, principale: &[u8; 32]) -> Result<Vec<CibleRevocation>, ErreurRevocation> {
        let mut cibles = vec![CibleRevocation::appareil(principale)];
        cibles.extend(self.cles_liees(principale)?.iter().map(CibleRevocation::appareil));
        Ok(cibles)
    }
}

/// [`RegistreLiaisons`] en mémoire (tests, COG sans persistance) ; au plus
/// [`MAX_LIAISONS_PAR_APPAREIL`] clés de destination par clé principale.
#[derive(Debug, Default)]
pub struct RegistreLiaisonsMemoire {
    liaisons: RwLock<BTreeMap<[u8; 32], BTreeSet<[u8; 32]>>>,
}

impl RegistreLiaisonsMemoire {
    /// Registre vide.
    pub fn new() -> Self {
        Self::default()
    }
}

impl RegistreLiaisons for RegistreLiaisonsMemoire {
    fn retenir(&self, liaison: &LiaisonVerifiee) -> Result<(), ErreurRevocation> {
        let mut l = self.liaisons.write().map_err(|_| ErreurRevocation::Indisponible("verrou".into()))?;
        let cles = l.entry(liaison.cle_principale).or_default();
        if !cles.contains(&liaison.cle_destination) && cles.len() >= MAX_LIAISONS_PAR_APPAREIL {
            return Err(ErreurRevocation::Indisponible("trop de liaisons pour cet appareil".into()));
        }
        cles.insert(liaison.cle_destination);
        Ok(())
    }

    fn cles_liees(&self, principale: &[u8; 32]) -> Result<Vec<[u8; 32]>, ErreurRevocation> {
        let l = self.liaisons.read().map_err(|_| ErreurRevocation::Indisponible("verrou".into()))?;
        Ok(l.get(principale).map(|c| c.iter().copied().collect()).unwrap_or_default())
    }
}

/// Côté COG d'origine : [`verifier_liaison`] puis [`RegistreLiaisons::retenir`]
/// (la liaison n'est rendue que si elle est retenue : cascade garantie).
pub fn accepter_liaison(
    registre: &dyn RegistreLiaisons,
    origine: &CogId,
    cle_principale: &[u8; 32],
    demande: &DemandeLiaison,
    revocations: &dyn SourceRevocation,
    maintenant: u64,
) -> Result<LiaisonVerifiee, RefusIdentite> {
    let liaison = verifier_liaison(origine, cle_principale, demande, revocations, maintenant)?;
    registre.retenir(&liaison).map_err(RefusIdentite::Revocation)?;
    Ok(liaison)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::revocation::ListeRevocation;

    const T0: u64 = 1_800_000_000;

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
        let m = message_liaison(&cog('a'), &cog('b'), &[9; 32], 0x0102_0304_0506_0708);
        assert_eq!(m.len(), DOMAINE_LIAISON.len() + 1 + 52 + 32 + 8);
        assert_eq!(m[DOMAINE_LIAISON.len()], 0);
        assert_eq!(&m[m.len() - 8..], &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(DOMAINE_LIAISON.ends_with(b"/v2"));
    }

    /// Liste de `h` à jour à `T0`.
    fn liste(h: &CogId) -> ListeRevocation {
        ListeRevocation::new(h.clone(), T0, 900)
    }

    #[test]
    fn echeance_bornee_et_rejeu_limite() {
        let (h, a) = (cog('h'), cog('a'));
        let principale = CleSignature::generer();
        let graine = GraineAppareil::generer();
        let revocations = liste(&h);
        let ok = lier_cle_appareil(&principale, &graine, &h, &a, T0 + 60).unwrap();
        assert!(verifier_liaison(&h, &principale.cle_publique(), &ok, &revocations, T0).is_ok());
        // Rejouée après l'échéance : refus.
        assert_eq!(
            verifier_liaison(&h, &principale.cle_publique(), &ok, &revocations, T0 + 61).err(),
            Some(RefusIdentite::ParametreInvalide("pas_apres"))
        );
        // Échéance trop lointaine (> 5 min + tolérance) : refus.
        let loin = lier_cle_appareil(&principale, &graine, &h, &a, T0 + DUREE_MAX_LIAISON_SECS + TOLERANCE_HORLOGE_SECS + 1)
            .unwrap();
        assert_eq!(
            verifier_liaison(&h, &principale.cle_publique(), &loin, &revocations, T0).err(),
            Some(RefusIdentite::ParametreInvalide("pas_apres"))
        );
        // Échéance modifiée après signature : signature invalide.
        let mut altere = ok.clone();
        altere.pas_apres = T0 + 120;
        assert_eq!(
            verifier_liaison(&h, &principale.cle_publique(), &altere, &revocations, T0).err(),
            Some(RefusIdentite::PossessionInvalide)
        );
    }

    #[test]
    fn cle_principale_revoquee_ou_statut_inconnu_refuse() {
        let (h, a) = (cog('h'), cog('a'));
        let principale = CleSignature::generer();
        let graine = GraineAppareil::generer();
        let demande = lier_cle_appareil(&principale, &graine, &h, &a, T0 + 60).unwrap();
        let mut revocations = liste(&h);
        let liaison = verifier_liaison(&h, &principale.cle_publique(), &demande, &revocations, T0).unwrap();
        // Statut indisponible (liste d'un autre COG) ou périmé : refus.
        let autre = liste(&cog('z'));
        assert!(matches!(
            verifier_liaison(&h, &principale.cle_publique(), &demande, &autre, T0),
            Err(RefusIdentite::Revocation(ErreurRevocation::ListeAbsente))
        ));
        assert!(matches!(
            verifier_liaison(&h, &principale.cle_publique(), &demande, &revocations, T0 + 1_000),
            Err(RefusIdentite::ParametreInvalide("pas_apres"))
        ));
        let secret = SecretTitulaire::generer();
        let cle_h = CleSignature::generer();
        let emettre = |r: &dyn SourceRevocation, t: u64| {
            emettre_passeport_lie(&h, &cle_h, &secret, &liaison, Portees::new(["x:y"]).unwrap(), t, t + 60, r, t)
        };
        assert!(emettre(&revocations, T0).is_ok());
        assert!(matches!(emettre(&revocations, T0 + 1_000), Err(RefusIdentite::Revocation(ErreurRevocation::Perimee { .. }))));
        // Clé principale révoquée : liaison et émission refusées.
        let cible = CibleRevocation::appareil(&principale.cle_publique());
        revocations.revoquer(cible);
        assert_eq!(
            verifier_liaison(&h, &principale.cle_publique(), &demande, &revocations, T0).err(),
            Some(RefusIdentite::Revoque(cible))
        );
        assert_eq!(emettre(&revocations, T0).err(), Some(RefusIdentite::Revoque(cible)));
        // Clé de destination révoquée seule : émission refusée.
        let mut r2 = liste(&h);
        r2.revoquer(liaison.cible_revocation());
        assert_eq!(emettre(&r2, T0).err(), Some(RefusIdentite::Revoque(liaison.cible_revocation())));
    }

    #[test]
    fn cascade_de_revocation_en_un_geste() {
        let h = cog('h');
        let principale = CleSignature::generer();
        let graine = GraineAppareil::generer();
        let revocations = liste(&h);
        let registre = RegistreLiaisonsMemoire::new();
        let mut attendues = vec![CibleRevocation::appareil(&principale.cle_publique())];
        for d in ['a', 'b', 'c'] {
            let demande = lier_cle_appareil(&principale, &graine, &h, &cog(d), T0 + 60).unwrap();
            let l = accepter_liaison(&registre, &h, &principale.cle_publique(), &demande, &revocations, T0).unwrap();
            // Rejeu de la même demande dans sa fenêtre : idempotent.
            accepter_liaison(&registre, &h, &principale.cle_publique(), &demande, &revocations, T0).unwrap();
            attendues.push(l.cible_revocation());
        }
        let mut cibles = registre.revoquer_appareil(&principale.cle_publique()).unwrap();
        let tri = |v: &mut Vec<CibleRevocation>| v.sort_by_key(|c| format!("{c:?}"));
        tri(&mut cibles);
        tri(&mut attendues);
        assert_eq!(cibles, attendues);
        // Appareil inconnu : seule sa clé principale.
        assert_eq!(registre.revoquer_appareil(&[5; 32]).unwrap(), vec![CibleRevocation::appareil(&[5; 32])]);
        // Une liaison refusée (révoquée) n'est pas retenue.
        let autre = CleSignature::generer();
        let mut r = liste(&h);
        r.revoquer(CibleRevocation::appareil(&autre.cle_publique()));
        let d = lier_cle_appareil(&autre, &graine, &h, &cog('a'), T0 + 60).unwrap();
        assert!(accepter_liaison(&registre, &h, &autre.cle_publique(), &d, &r, T0).is_err());
        assert_eq!(registre.cles_liees(&autre.cle_publique()).unwrap(), Vec::<[u8; 32]>::new());
    }
}
