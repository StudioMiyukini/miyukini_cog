//! @id     miyauth_http
//! @do     authentifier_requete_http
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — schéma HTTP réutilisable : visa + preuve de possession, requêtes signées, défis et nonces.

//! Authentification HTTP sans secret partagé (feature `http`, sortie du lot 2).
//!
//! Module **sans dépendance de framework** : il reçoit des valeurs d'en-têtes
//! (`&str`), une méthode, un chemin et un corps, et rend un type prouvé ou un
//! refus. Chaque service (Origin, central-remote, services locaux) l'adapte à
//! son serveur HTTP. Deux mécanismes :
//!
//! # 1. Utilisateur : visa + preuve de possession (doc gelée lot 2, §5 à §7)
//!
//! ```text
//! Authorization: MiyuVisa <b64url(visa)>.<b64url(passeport)>
//! X-Miyu-PoP:    <b64url(id du défi, 16 o)>.<b64url(signature Ed25519, 64 o)>
//! ```
//!
//! Pour obtenir un visa, le porteur présente seulement son passeport :
//! `Authorization: MiyuPasseport <b64url(passeport)>` (même en-tête `X-Miyu-PoP`).
//!
//! - Le défi (32 o aléatoires) est émis par le service ([`MagasinDefis::emettre`]),
//!   lié à une audience, **à usage unique** et de courte durée ([`DUREE_DEFI_SECS`]).
//!   Il est consommé **avant** toute vérification cryptographique : un défi ne
//!   sert qu'une fois, même en cas d'échec.
//! - La signature de possession est celle de la doc §7 :
//!   `Ed25519(cle_appareil, "miyukini/pop/v1" || defi || audience || liaison)`
//!   ([`crate::message_possession`]).
//! - `liaison` : la doc prévoit l'exporteur TLS (ou la transcription Noise, lot 3).
//!   Derrière un mandataire inverse qui termine TLS, l'exporteur n'est pas
//!   disponible pour le service ; on lie alors la preuve **à la requête** :
//!   [`liaison_requete`] = `SHA-256("miyukini/http-liaison/v1" || 0 || méthode || 0 ||
//!   chemin || 0 || SHA-256(corps))`. Une preuve interceptée ne vaut que pour la
//!   même requête, une seule fois (défi consommé). Un service qui dispose de
//!   l'exporteur TLS peut fournir sa propre liaison ([`RequeteHttp::liaison`]).
//! - Base64 : alphabet URL (RFC 4648 §5), **sans remplissage**, décodage strict
//!   (représentation canonique unique).
//!
//! # 2. Machine à machine : requête signée par une clé épinglée
//!
//! ```text
//! X-Miyu-Signataire: <b64url(clé publique Ed25519, 32 o)>
//! X-Miyu-Horodatage: <secondes Unix>
//! X-Miyu-Nonce:      <b64url(16 à 64 o aléatoires)>
//! X-Miyu-Signature:  <b64url(signature Ed25519, 64 o)>
//! ```
//!
//! Octets signés ([`message_requete_signee`]) :
//! `"miyukini/requete-signee/v1" || 0 || lp(usage) || lp(méthode) || lp(chemin) ||
//! horodatage (u64 BE) || lp(nonce) || SHA-256(corps)` où `lp` = longueur u16 BE + octets.
//! `usage` sépare les domaines (ex. `origin/forum-sync`, `origin/market-publication`).
//!
//! Ordre conseillé côté serveur (en-têtes avant corps, fail-closed) :
//! 1. [`EntetesSignees::analyser`] puis contrôle de l'**appelant** (clé épinglée : c'est
//!    à l'appelant de décider quelles clés sont autorisées) et
//!    [`EntetesSignees::controler_horodatage`] — **avant** de lire un gros corps ;
//! 2. lecture bornée du corps ;
//! 3. [`verifier_requete_signee`] : fenêtre, `verify_strict`, puis enregistrement du
//!    nonce dans le [`RegistreNonces`] (rejeu refusé, registre plein = refus).

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use miyusceau::{empreinte_cle, CleSignature, Kid};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

use crate::passeport::{
    message_possession, prouver_possession, verifier_passeport, verifier_visa, AccesProuve, PolitiqueIdentite,
    PossessionProuvee, RefusIdentite,
};
use crate::titres::audience_valide;

pub mod service;

/// Schéma `Authorization` portant un visa et son passeport.
pub const SCHEMA_VISA: &str = "MiyuVisa";
/// Schéma `Authorization` portant un passeport seul (demande de visa).
pub const SCHEMA_PASSEPORT: &str = "MiyuPasseport";
/// En-tête de preuve de possession.
pub const ENTETE_POP: &str = "x-miyu-pop";
/// En-tête : clé publique du signataire d'une requête signée.
pub const ENTETE_SIGNATAIRE: &str = "x-miyu-signataire";
/// En-tête : horodatage d'une requête signée.
pub const ENTETE_HORODATAGE: &str = "x-miyu-horodatage";
/// En-tête : nonce d'une requête signée.
pub const ENTETE_NONCE: &str = "x-miyu-nonce";
/// En-tête : signature d'une requête signée.
pub const ENTETE_SIGNATURE: &str = "x-miyu-signature";

/// Domaine de la liaison à la requête.
pub const DOMAINE_LIAISON: &[u8] = b"miyukini/http-liaison/v1";
/// Domaine des requêtes signées.
pub const DOMAINE_REQUETE_SIGNEE: &[u8] = b"miyukini/requete-signee/v1";

/// Durée de vie d'un défi de possession (choix fail-closed : 60 s).
pub const DUREE_DEFI_SECS: u64 = 60;
/// Nombre maximal de défis en attente par défaut (borne mémoire ; plein = refus).
pub const CAPACITE_DEFIS_DEFAUT: usize = 10_000;
/// Taille maximale d'un titre (visa ou passeport) dans un en-tête. Les titres réels
/// font quelques centaines d'octets ; borne fail-closed bien en deçà du maximum Sceau.
pub const TAILLE_MAX_TITRE_ENTETE: usize = 4096;
/// Fenêtre d'horodatage d'une requête signée (± 5 min).
pub const FENETRE_HORODATAGE_SECS: u64 = 300;
/// Nombre maximal de nonces mémorisés par défaut (plein = refus).
pub const CAPACITE_NONCES_DEFAUT: usize = 10_000;
/// Taille minimale d'un nonce de requête signée.
pub const TAILLE_MIN_NONCE: usize = 16;
/// Taille maximale d'un nonce de requête signée.
pub const TAILLE_MAX_NONCE: usize = 64;

/// Identifiant d'un défi (16 octets aléatoires).
pub type IdDefi = [u8; 16];

// ═══════════════════════════════════════════════════════════════════════════════
// Refus
// ═══════════════════════════════════════════════════════════════════════════════

/// Motif de refus d'une requête HTTP. Toute variante signifie « refus ».
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusHttp {
    /// En-tête obligatoire absent.
    EnteteAbsente(&'static str),
    /// En-tête présent mais mal formé (schéma, base64, taille).
    EnteteMalFormee(&'static str),
    /// Défi inconnu, expiré, déjà consommé ou d'une autre audience.
    DefiInvalide,
    /// Magasin de défis plein (fail-closed).
    MagasinPlein,
    /// Horodatage hors fenêtre.
    HorodatageHorsFenetre,
    /// Nonce déjà vu (rejeu).
    Rejeu,
    /// Registre de nonces plein (fail-closed).
    NoncesSatures,
    /// Signataire non autorisé par l'appelant.
    SignataireNonAutorise,
    /// Signature de requête invalide.
    SignatureInvalide,
    /// Audience invalide.
    AudienceInvalide,
    /// Titre d'identité refusé (passeport, visa, possession, révocation).
    Identite(RefusIdentite),
}

impl RefusHttp {
    /// Statut HTTP conseillé : 503 si le service est saturé, 401 sinon.
    pub fn statut(&self) -> u16 {
        match self {
            Self::MagasinPlein | Self::NoncesSatures => 503,
            _ => 401,
        }
    }
}

impl fmt::Display for RefusHttp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnteteAbsente(e) => write!(f, "en-tête absent : {e}"),
            Self::EnteteMalFormee(e) => write!(f, "en-tête mal formé : {e}"),
            Self::DefiInvalide => write!(f, "défi inconnu, expiré ou déjà utilisé"),
            Self::MagasinPlein => write!(f, "trop de défis en attente"),
            Self::HorodatageHorsFenetre => write!(f, "horodatage hors fenêtre"),
            Self::Rejeu => write!(f, "requête rejouée"),
            Self::NoncesSatures => write!(f, "registre de nonces saturé"),
            Self::SignataireNonAutorise => write!(f, "signataire non autorisé"),
            Self::SignatureInvalide => write!(f, "signature de requête invalide"),
            Self::AudienceInvalide => write!(f, "audience invalide"),
            Self::Identite(r) => write!(f, "identité refusée : {r}"),
        }
    }
}

impl std::error::Error for RefusHttp {}

impl From<RefusIdentite> for RefusHttp {
    fn from(r: RefusIdentite) -> Self {
        Self::Identite(r)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Base64 URL sans remplissage (strict)
// ═══════════════════════════════════════════════════════════════════════════════

const ALPHABET_B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Encode en base64 URL sans remplissage (RFC 4648 §5).
pub fn b64_encoder(octets: &[u8]) -> String {
    let mut s = String::with_capacity(octets.len().div_ceil(3) * 4);
    for bloc in octets.chunks(3) {
        let n = (u32::from(bloc[0]) << 16)
            | (u32::from(*bloc.get(1).unwrap_or(&0)) << 8)
            | u32::from(*bloc.get(2).unwrap_or(&0));
        let caracteres = bloc.len() + 1;
        for i in 0..caracteres {
            let index = (n >> (18 - 6 * i)) & 0x3f;
            s.push(char::from(ALPHABET_B64[index as usize]));
        }
    }
    s
}

fn valeur_b64(c: u8) -> Option<u32> {
    let v = match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'-' => 62,
        b'_' => 63,
        _ => return None,
    };
    Some(u32::from(v))
}

/// Décode du base64 URL sans remplissage. Strict : refuse remplissage, caractères
/// hors alphabet, longueur impossible et bits de fin non nuls (forme non canonique).
pub fn b64_decoder(texte: &str) -> Option<Vec<u8>> {
    let octets = texte.as_bytes();
    if octets.len() % 4 == 1 {
        return None;
    }
    let mut sortie = Vec::with_capacity(octets.len() * 3 / 4);
    for bloc in octets.chunks(4) {
        let mut n: u32 = 0;
        for (i, &c) in bloc.iter().enumerate() {
            n |= valeur_b64(c)? << (18 - 6 * i);
        }
        let utiles = bloc.len() - 1;
        let complet = n.to_be_bytes();
        // Bits de fin non utilisés : doivent être nuls (une seule représentation).
        let masque_fin: u32 = match bloc.len() {
            2 => 0x00_ff_ff,
            3 => 0x00_00_ff,
            _ => 0,
        };
        if n & masque_fin != 0 {
            return None;
        }
        sortie.extend_from_slice(&complet[1..=utiles]);
    }
    Some(sortie)
}

fn b64_taille_fixe<const N: usize>(texte: &str, quoi: &'static str) -> Result<[u8; N], RefusHttp> {
    b64_decoder(texte)
        .and_then(|v| v.try_into().ok())
        .ok_or(RefusHttp::EnteteMalFormee(quoi))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Défis de possession (émission, usage unique, expiration)
// ═══════════════════════════════════════════════════════════════════════════════

/// Défi émis par un service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefiEmis {
    /// Identifiant (à renvoyer dans `X-Miyu-PoP`).
    pub id: IdDefi,
    /// Défi (32 o) à signer par la clé d'appareil.
    pub defi: [u8; 32],
    /// Audience à laquelle le défi est lié.
    pub audience: String,
    /// Expiration (secondes Unix).
    pub expire_a: u64,
}

impl DefiEmis {
    /// Identifiant en base64 URL.
    pub fn id_b64(&self) -> String {
        b64_encoder(&self.id)
    }

    /// Défi en base64 URL.
    pub fn defi_b64(&self) -> String {
        b64_encoder(&self.defi)
    }
}

/// Magasin de défis : émission, consommation **unique**, expiration.
///
/// Contrat : [`MagasinDefis::consommer`] retire le défi dans tous les cas (même
/// expiré ou d'une autre audience) et ne le rend que s'il est valide pour
/// `audience` à `maintenant`. Un service réparti peut l'adosser à un stockage
/// partagé ; l'implémentation mémoire suffit à un processus unique.
pub trait MagasinDefis: Send + Sync {
    /// Émet un défi lié à `audience`.
    fn emettre(&self, audience: &str, maintenant: u64) -> Result<DefiEmis, RefusHttp>;
    /// Consomme le défi `id` (usage unique) et rend ses 32 octets.
    fn consommer(&self, id: &IdDefi, audience: &str, maintenant: u64) -> Result<[u8; 32], RefusHttp>;
}

#[derive(Debug)]
struct EntreeDefi {
    defi: [u8; 32],
    audience: String,
    expire_a: u64,
}

/// Magasin de défis en mémoire, borné.
#[derive(Debug)]
pub struct MagasinDefisMemoire {
    entrees: Mutex<HashMap<IdDefi, EntreeDefi>>,
    capacite: usize,
    duree: u64,
}

impl Default for MagasinDefisMemoire {
    fn default() -> Self {
        Self::new(CAPACITE_DEFIS_DEFAUT, DUREE_DEFI_SECS)
    }
}

impl MagasinDefisMemoire {
    /// Magasin de `capacite` défis au plus, chacun valable `duree` secondes (≥ 1).
    pub fn new(capacite: usize, duree: u64) -> Self {
        Self { entrees: Mutex::new(HashMap::new()), capacite, duree: duree.max(1) }
    }

    /// Nombre de défis en attente (non purgés).
    pub fn len(&self) -> usize {
        self.entrees.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len()
    }

    /// Vrai si aucun défi en attente.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl MagasinDefis for MagasinDefisMemoire {
    fn emettre(&self, audience: &str, maintenant: u64) -> Result<DefiEmis, RefusHttp> {
        if !audience_valide(audience) {
            return Err(RefusHttp::AudienceInvalide);
        }
        let mut entrees = self.entrees.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        entrees.retain(|_, e| e.expire_a > maintenant);
        if entrees.len() >= self.capacite {
            return Err(RefusHttp::MagasinPlein);
        }
        let mut id = [0u8; 16];
        let mut defi = [0u8; 32];
        OsRng.fill_bytes(&mut id);
        OsRng.fill_bytes(&mut defi);
        let expire_a = maintenant.saturating_add(self.duree);
        entrees.insert(id, EntreeDefi { defi, audience: audience.to_string(), expire_a });
        Ok(DefiEmis { id, defi, audience: audience.to_string(), expire_a })
    }

    fn consommer(&self, id: &IdDefi, audience: &str, maintenant: u64) -> Result<[u8; 32], RefusHttp> {
        let mut entrees = self.entrees.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entree = entrees.remove(id).ok_or(RefusHttp::DefiInvalide)?;
        if entree.expire_a <= maintenant || entree.audience != audience {
            return Err(RefusHttp::DefiInvalide);
        }
        Ok(entree.defi)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// En-têtes visa / passeport / PoP
// ═══════════════════════════════════════════════════════════════════════════════

/// Contenu de l'en-tête `X-Miyu-PoP`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreuvePop {
    /// Identifiant du défi consommé.
    pub id_defi: IdDefi,
    /// Signature de possession.
    pub signature: [u8; 64],
}

/// Analyse `X-Miyu-PoP: <b64(id)>.<b64(signature)>`.
pub fn analyser_pop(valeur: &str) -> Result<PreuvePop, RefusHttp> {
    let (id, sig) = valeur.trim().split_once('.').ok_or(RefusHttp::EnteteMalFormee(ENTETE_POP))?;
    Ok(PreuvePop {
        id_defi: b64_taille_fixe(id, ENTETE_POP)?,
        signature: b64_taille_fixe(sig, ENTETE_POP)?,
    })
}

/// Formate l'en-tête `X-Miyu-PoP` (côté client).
pub fn formater_pop(id_defi: &IdDefi, signature: &[u8; 64]) -> String {
    format!("{}.{}", b64_encoder(id_defi), b64_encoder(signature))
}

fn titre_entete(texte: &str) -> Result<Vec<u8>, RefusHttp> {
    if texte.is_empty() || texte.len() > TAILLE_MAX_TITRE_ENTETE.div_ceil(3) * 4 {
        return Err(RefusHttp::EnteteMalFormee("authorization"));
    }
    b64_decoder(texte).ok_or(RefusHttp::EnteteMalFormee("authorization"))
}

fn sans_schema<'a>(valeur: &'a str, schema: &str) -> Result<&'a str, RefusHttp> {
    let (s, reste) = valeur.trim().split_once(' ').ok_or(RefusHttp::EnteteMalFormee("authorization"))?;
    if s != schema {
        return Err(RefusHttp::EnteteMalFormee("authorization"));
    }
    Ok(reste.trim())
}

/// Analyse `Authorization: MiyuVisa <b64(visa)>.<b64(passeport)>` → `(visa, passeport)`.
pub fn analyser_autorisation_visa(valeur: &str) -> Result<(Vec<u8>, Vec<u8>), RefusHttp> {
    let (visa, passeport) =
        sans_schema(valeur, SCHEMA_VISA)?.split_once('.').ok_or(RefusHttp::EnteteMalFormee("authorization"))?;
    Ok((titre_entete(visa)?, titre_entete(passeport)?))
}

/// Formate `Authorization: MiyuVisa …` (côté client).
pub fn formater_autorisation_visa(visa: &[u8], passeport: &[u8]) -> String {
    format!("{SCHEMA_VISA} {}.{}", b64_encoder(visa), b64_encoder(passeport))
}

/// Analyse `Authorization: MiyuPasseport <b64(passeport)>`.
pub fn analyser_autorisation_passeport(valeur: &str) -> Result<Vec<u8>, RefusHttp> {
    titre_entete(sans_schema(valeur, SCHEMA_PASSEPORT)?)
}

/// Formate `Authorization: MiyuPasseport …` (côté client).
pub fn formater_autorisation_passeport(passeport: &[u8]) -> String {
    format!("{SCHEMA_PASSEPORT} {}", b64_encoder(passeport))
}

/// Liaison de la preuve de possession à la requête (voir l'en-tête du module).
pub fn liaison_requete(methode: &str, chemin: &str, corps: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOMAINE_LIAISON);
    h.update([0u8]);
    h.update(methode.as_bytes());
    h.update([0u8]);
    h.update(chemin.as_bytes());
    h.update([0u8]);
    h.update(Sha256::digest(corps));
    h.finalize().into()
}

/// Signe une preuve de possession (côté client, clé d'appareil).
pub fn signer_pop(cle_appareil: &CleSignature, defi: &[u8; 32], audience: &str, liaison: &[u8; 32]) -> [u8; 64] {
    SigningKey::from_bytes(&cle_appareil.graine()).sign(&message_possession(defi, audience, liaison)).to_bytes()
}

/// Requête HTTP à authentifier (vue indépendante du framework).
#[derive(Debug, Clone, Copy)]
pub struct RequeteHttp<'a> {
    /// Méthode (`GET`, `POST`…).
    pub methode: &'a str,
    /// Chemin sans la chaîne de requête.
    pub chemin: &'a str,
    /// Corps (vide si aucun).
    pub corps: &'a [u8],
    /// Valeur de l'en-tête `Authorization`.
    pub autorisation: Option<&'a str>,
    /// Valeur de l'en-tête `X-Miyu-PoP`.
    pub pop: Option<&'a str>,
    /// Liaison de canal fournie par le service (exporteur TLS) ; `None` =
    /// [`liaison_requete`].
    pub liaison: Option<[u8; 32]>,
}

impl RequeteHttp<'_> {
    fn liaison_effective(&self) -> [u8; 32] {
        self.liaison.unwrap_or_else(|| liaison_requete(self.methode, self.chemin, self.corps))
    }

    fn preuve(&self, audience: &str, magasin: &dyn MagasinDefis, maintenant: u64) -> Result<([u8; 32], [u8; 64]), RefusHttp> {
        let pop = analyser_pop(self.pop.ok_or(RefusHttp::EnteteAbsente(ENTETE_POP))?)?;
        let defi = magasin.consommer(&pop.id_defi, audience, maintenant)?;
        Ok((defi, pop.signature))
    }
}

/// Vérifie une requête `MiyuVisa` + `X-Miyu-PoP` pour `audience` et rend un
/// [`AccesProuve`] (seul type qui accorde un rôle). Le défi est consommé avant
/// la vérification des titres.
pub fn verifier_requete_visa(
    requete: &RequeteHttp<'_>,
    audience: &str,
    politique: &PolitiqueIdentite<'_>,
    magasin: &dyn MagasinDefis,
    maintenant: u64,
) -> Result<AccesProuve, RefusHttp> {
    let (visa, passeport) =
        analyser_autorisation_visa(requete.autorisation.ok_or(RefusHttp::EnteteAbsente("authorization"))?)?;
    let (defi, signature) = requete.preuve(audience, magasin, maintenant)?;
    let acces = verifier_visa(&visa, &passeport, audience, politique, maintenant)?;
    Ok(acces.verifier_possession(&defi, &requete.liaison_effective(), &signature)?)
}

/// Vérifie une requête `MiyuPasseport` + `X-Miyu-PoP` (demande de visa pour
/// `audience`) et rend la [`PossessionProuvee`] attendue par [`crate::emettre_visa`].
pub fn verifier_requete_passeport(
    requete: &RequeteHttp<'_>,
    audience: &str,
    politique: &PolitiqueIdentite<'_>,
    magasin: &dyn MagasinDefis,
    maintenant: u64,
) -> Result<PossessionProuvee, RefusHttp> {
    let passeport =
        analyser_autorisation_passeport(requete.autorisation.ok_or(RefusHttp::EnteteAbsente("authorization"))?)?;
    let (defi, signature) = requete.preuve(audience, magasin, maintenant)?;
    let pv = verifier_passeport(&passeport, politique, maintenant)?;
    Ok(prouver_possession(pv, &defi, audience, &requete.liaison_effective(), &signature)?)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Requêtes signées (machine à machine)
// ═══════════════════════════════════════════════════════════════════════════════

fn lp(m: &mut Vec<u8>, octets: &[u8]) {
    // Longueurs bornées par l'appelant (usage, méthode, chemin, nonce) : u16 suffit ;
    // au-delà, la valeur est tronquée à 0xFFFF puis refusée par la signature.
    let n = u16::try_from(octets.len()).unwrap_or(u16::MAX);
    m.extend_from_slice(&n.to_be_bytes());
    m.extend_from_slice(octets);
}

/// Octets signés d'une requête signée (voir l'en-tête du module).
pub fn message_requete_signee(
    usage: &str,
    methode: &str,
    chemin: &str,
    horodatage: u64,
    nonce: &[u8],
    empreinte_corps: &[u8; 32],
) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_REQUETE_SIGNEE.len() + 64 + usage.len() + chemin.len() + nonce.len());
    m.extend_from_slice(DOMAINE_REQUETE_SIGNEE);
    m.push(0);
    lp(&mut m, usage.as_bytes());
    lp(&mut m, methode.as_bytes());
    lp(&mut m, chemin.as_bytes());
    m.extend_from_slice(&horodatage.to_be_bytes());
    lp(&mut m, nonce);
    m.extend_from_slice(empreinte_corps);
    m
}

/// En-têtes d'une requête signée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntetesSignees {
    /// Clé publique du signataire.
    pub signataire: [u8; 32],
    /// Horodatage (secondes Unix).
    pub horodatage: u64,
    /// Nonce (16 à 64 octets).
    pub nonce: Vec<u8>,
    /// Signature Ed25519.
    pub signature: [u8; 64],
}

impl EntetesSignees {
    /// Analyse les quatre en-têtes (valeurs brutes, `None` si absent).
    pub fn analyser(
        signataire: Option<&str>,
        horodatage: Option<&str>,
        nonce: Option<&str>,
        signature: Option<&str>,
    ) -> Result<Self, RefusHttp> {
        let signataire = b64_taille_fixe(signataire.ok_or(RefusHttp::EnteteAbsente(ENTETE_SIGNATAIRE))?.trim(), ENTETE_SIGNATAIRE)?;
        let horodatage = horodatage.ok_or(RefusHttp::EnteteAbsente(ENTETE_HORODATAGE))?.trim();
        if horodatage.is_empty() || horodatage.len() > 20 || !horodatage.bytes().all(|b| b.is_ascii_digit()) {
            return Err(RefusHttp::EnteteMalFormee(ENTETE_HORODATAGE));
        }
        let horodatage = horodatage.parse().map_err(|_| RefusHttp::EnteteMalFormee(ENTETE_HORODATAGE))?;
        let nonce = nonce.ok_or(RefusHttp::EnteteAbsente(ENTETE_NONCE))?.trim();
        if nonce.len() > TAILLE_MAX_NONCE.div_ceil(3) * 4 {
            return Err(RefusHttp::EnteteMalFormee(ENTETE_NONCE));
        }
        let nonce = b64_decoder(nonce)
            .filter(|n| (TAILLE_MIN_NONCE..=TAILLE_MAX_NONCE).contains(&n.len()))
            .ok_or(RefusHttp::EnteteMalFormee(ENTETE_NONCE))?;
        let signature = b64_taille_fixe(signature.ok_or(RefusHttp::EnteteAbsente(ENTETE_SIGNATURE))?.trim(), ENTETE_SIGNATURE)?;
        Ok(Self { signataire, horodatage, nonce, signature })
    }

    /// Paires `(nom, valeur)` à poser sur la requête (côté client).
    pub fn en_entetes(&self) -> [(&'static str, String); 4] {
        [
            (ENTETE_SIGNATAIRE, b64_encoder(&self.signataire)),
            (ENTETE_HORODATAGE, self.horodatage.to_string()),
            (ENTETE_NONCE, b64_encoder(&self.nonce)),
            (ENTETE_SIGNATURE, b64_encoder(&self.signature)),
        ]
    }

    /// `kid` du signataire.
    pub fn kid(&self) -> Kid {
        empreinte_cle(&self.signataire)
    }

    /// Contrôle de fenêtre (à faire avant de lire le corps).
    pub fn controler_horodatage(&self, maintenant: u64, fenetre: u64) -> Result<(), RefusHttp> {
        if maintenant.abs_diff(self.horodatage) > fenetre {
            return Err(RefusHttp::HorodatageHorsFenetre);
        }
        Ok(())
    }
}

/// Signe une requête (côté client) avec un nonce aléatoire de 16 octets.
pub fn signer_requete(
    cle: &CleSignature,
    usage: &str,
    methode: &str,
    chemin: &str,
    corps: &[u8],
    horodatage: u64,
) -> EntetesSignees {
    let mut nonce = vec![0u8; TAILLE_MIN_NONCE];
    OsRng.fill_bytes(&mut nonce);
    let empreinte: [u8; 32] = Sha256::digest(corps).into();
    let message = message_requete_signee(usage, methode, chemin, horodatage, &nonce, &empreinte);
    let signature = SigningKey::from_bytes(&cle.graine()).sign(&message).to_bytes();
    EntetesSignees { signataire: cle.cle_publique(), horodatage, nonce, signature }
}

/// Nonces déjà vus (anti-rejeu), bornés en nombre et en durée.
#[derive(Debug)]
pub struct RegistreNonces {
    vus: Mutex<HashMap<(Kid, Vec<u8>), u64>>,
    capacite: usize,
    fenetre: u64,
}

impl Default for RegistreNonces {
    fn default() -> Self {
        Self::new(CAPACITE_NONCES_DEFAUT, FENETRE_HORODATAGE_SECS)
    }
}

impl RegistreNonces {
    /// Registre de `capacite` nonces, fenêtre d'horodatage `fenetre` secondes.
    pub fn new(capacite: usize, fenetre: u64) -> Self {
        Self { vus: Mutex::new(HashMap::new()), capacite, fenetre }
    }

    /// Fenêtre d'horodatage acceptée.
    pub fn fenetre(&self) -> u64 {
        self.fenetre
    }

    /// Vrai si le nonce a déjà été accepté (contrôle précoce, sans enregistrement).
    pub fn deja_vu(&self, kid: &Kid, nonce: &[u8]) -> bool {
        self.vus
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .contains_key(&(*kid, nonce.to_vec()))
    }

    fn enregistrer(&self, kid: Kid, nonce: &[u8], maintenant: u64) -> Result<(), RefusHttp> {
        let mut vus = self.vus.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        vus.retain(|_, expire| *expire > maintenant);
        let cle = (kid, nonce.to_vec());
        if vus.contains_key(&cle) {
            return Err(RefusHttp::Rejeu);
        }
        if vus.len() >= self.capacite {
            return Err(RefusHttp::NoncesSatures);
        }
        // Un nonce reste mémorisé tant que son horodatage peut encore être accepté.
        vus.insert(cle, maintenant.saturating_add(2 * self.fenetre + 1));
        Ok(())
    }
}

/// Vérifie une requête signée : fenêtre, signature (`verify_strict`) sur
/// `usage`/méthode/chemin/horodatage/nonce/SHA-256(corps), puis nonce non rejoué
/// (enregistré seulement si tout le reste est valide). L'autorisation du
/// signataire (clé épinglée) reste à la charge de l'appelant.
pub fn verifier_requete_signee(
    entetes: &EntetesSignees,
    usage: &str,
    methode: &str,
    chemin: &str,
    corps: &[u8],
    nonces: &RegistreNonces,
    maintenant: u64,
) -> Result<(), RefusHttp> {
    entetes.controler_horodatage(maintenant, nonces.fenetre)?;
    let cle = VerifyingKey::from_bytes(&entetes.signataire)
        .ok()
        .filter(|k| !k.is_weak())
        .ok_or(RefusHttp::SignatureInvalide)?;
    let empreinte: [u8; 32] = Sha256::digest(corps).into();
    let message = message_requete_signee(usage, methode, chemin, entetes.horodatage, &entetes.nonce, &empreinte);
    cle.verify_strict(&message, &Signature::from_bytes(&entetes.signature))
        .map_err(|_| RefusHttp::SignatureInvalide)?;
    nonces.enregistrer(entetes.kid(), &entetes.nonce, maintenant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b64_aller_retour_et_strict() {
        for n in 0..70usize {
            let octets: Vec<u8> = (0..n).map(|i| (i * 37 + 11) as u8).collect();
            let t = b64_encoder(&octets);
            assert!(!t.contains('='));
            assert_eq!(b64_decoder(&t).unwrap(), octets, "{n}");
        }
        assert!(b64_decoder("A").is_none());
        assert!(b64_decoder("AB==").is_none());
        assert!(b64_decoder("AB+/").is_none());
        // « AB » encode 0x00 avec des bits de fin non nuls : forme non canonique.
        assert!(b64_decoder("AB").is_none());
        assert_eq!(b64_decoder("AA").unwrap(), vec![0]);
    }

    #[test]
    fn defi_usage_unique_audience_expiration() {
        let m = MagasinDefisMemoire::new(2, 60);
        let d = m.emettre("forum", 100).unwrap();
        assert_eq!(m.consommer(&d.id, "forum", 120).unwrap(), d.defi);
        assert_eq!(m.consommer(&d.id, "forum", 120), Err(RefusHttp::DefiInvalide));
        let d = m.emettre("forum", 100).unwrap();
        assert_eq!(m.consommer(&d.id, "autre", 101), Err(RefusHttp::DefiInvalide));
        // Consommé même en cas d'échec.
        assert_eq!(m.consommer(&d.id, "forum", 101), Err(RefusHttp::DefiInvalide));
        let d = m.emettre("forum", 100).unwrap();
        assert_eq!(m.consommer(&d.id, "forum", 160), Err(RefusHttp::DefiInvalide));
        m.emettre("forum", 200).unwrap();
        m.emettre("forum", 200).unwrap();
        assert_eq!(m.emettre("forum", 200), Err(RefusHttp::MagasinPlein));
        // Purge des expirés.
        assert!(m.emettre("forum", 261).is_ok());
        assert_eq!(m.emettre("Forum!", 300), Err(RefusHttp::AudienceInvalide));
    }

    #[test]
    fn requete_signee_aller_retour_rejeu_alteration() {
        let cle = CleSignature::generer();
        let nonces = RegistreNonces::new(10, 300);
        let e = signer_requete(&cle, "test/usage", "POST", "/x", b"corps", 1000);
        let brut = e.en_entetes();
        let e2 = EntetesSignees::analyser(Some(&brut[0].1), Some(&brut[1].1), Some(&brut[2].1), Some(&brut[3].1)).unwrap();
        assert_eq!(e, e2);
        assert!(verifier_requete_signee(&e2, "test/usage", "POST", "/x", b"corps", &nonces, 1010).is_ok());
        assert!(nonces.deja_vu(&e2.kid(), &e2.nonce));
        assert_eq!(verifier_requete_signee(&e2, "test/usage", "POST", "/x", b"corps", &nonces, 1010), Err(RefusHttp::Rejeu));
        let e = signer_requete(&cle, "test/usage", "POST", "/x", b"corps", 1000);
        for (u, m, c, b) in [("autre", "POST", "/x", &b"corps"[..]), ("test/usage", "GET", "/x", b"corps"), ("test/usage", "POST", "/y", b"corps"), ("test/usage", "POST", "/x", b"corpz")] {
            assert_eq!(verifier_requete_signee(&e, u, m, c, b, &nonces, 1000), Err(RefusHttp::SignatureInvalide));
        }
        assert_eq!(verifier_requete_signee(&e, "test/usage", "POST", "/x", b"corps", &nonces, 1301), Err(RefusHttp::HorodatageHorsFenetre));
        // Un refus n'a pas consommé le nonce : la requête intacte passe encore.
        assert!(verifier_requete_signee(&e, "test/usage", "POST", "/x", b"corps", &nonces, 1000).is_ok());
    }

    #[test]
    fn entetes_signees_mal_formees() {
        let cle = CleSignature::generer();
        let e = signer_requete(&cle, "u", "POST", "/x", b"", 5).en_entetes();
        assert_eq!(EntetesSignees::analyser(None, Some("5"), Some(&e[2].1), Some(&e[3].1)), Err(RefusHttp::EnteteAbsente(ENTETE_SIGNATAIRE)));
        assert!(EntetesSignees::analyser(Some(&e[0].1), Some("-5"), Some(&e[2].1), Some(&e[3].1)).is_err());
        assert!(EntetesSignees::analyser(Some(&e[0].1), Some("5"), Some("AAAA"), Some(&e[3].1)).is_err());
        assert!(EntetesSignees::analyser(Some(&e[0].1), Some("5"), Some(&e[2].1), Some("AAAA")).is_err());
    }

    #[test]
    fn entetes_autorisation() {
        let h = formater_autorisation_visa(b"visa", b"passeport");
        assert_eq!(analyser_autorisation_visa(&h).unwrap(), (b"visa".to_vec(), b"passeport".to_vec()));
        assert!(analyser_autorisation_visa("Bearer abc").is_err());
        assert!(analyser_autorisation_visa(&formater_autorisation_passeport(b"p")).is_err());
        assert_eq!(analyser_autorisation_passeport(&formater_autorisation_passeport(b"p")).unwrap(), b"p".to_vec());
        let p = formater_pop(&[1; 16], &[2; 64]);
        assert_eq!(analyser_pop(&p).unwrap(), PreuvePop { id_defi: [1; 16], signature: [2; 64] });
        assert!(analyser_pop("abc").is_err());
        let grand = "A".repeat(TAILLE_MAX_TITRE_ENTETE * 2);
        assert!(analyser_autorisation_passeport(&format!("MiyuPasseport {grand}")).is_err());
    }
}
