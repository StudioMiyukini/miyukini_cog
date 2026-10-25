//! @id     origin_web_revocations
//! @do     suivre_revocations_cogs_reconnus
//! @role   securite
//! @layer  app
//! @human  origin — révocations des COG reconnus : suivi vérifié de leur registre (`SuiviRevocationsRegistre`), synchro par morceaux bornée, fraîcheur ≤ 15 min, refus sinon.

//! Révocations des COG reconnus vues d'Origin (lot 2 §8.2, A-003, A-007).
//!
//! Chaque COG reconnu dont le registre est déclaré (`[[forum.cogs_reconnus]]` :
//! `registre`, `cle_journal`) a ici un [`SuiviRevocationsRegistre`]
//! (`miyuregistre`, fonctionnalité `miyauth`) : le **même** type que `MiyuCloud`
//! et les façades — miroir vérifié (tête signée par la clé de journal épinglée,
//! seuil 1, racine recalculée, monotonie, équivoque), liste des entrées
//! `Revocation`, **fraîcheur 15 min imposée par le type**, `RwLock` (les
//! vérifications ne prennent que la lecture ; tête, racine et sceaux sont
//! vérifiés hors du verrou d'écriture ; rejeu à l'identique sans recalcul).
//!
//! # Transport : `POST /api/auth/revocations/sync` (à consigner par amendement)
//!
//! Indépendant du store du forum (l'admin en dépend aussi) : la route existe
//! dès qu'Origin tourne. Le COG **pousse** JSON
//! `{cog_id, depuis, tete?, entrees}` (titres en base64 URL). Aucune
//! authentification de transport : tout est vérifié par la signature de la
//! tête et la racine Merkle.
//!
//! - **Morceaux** : un envoi **sans** `tete` est un morceau : ses entrées
//!   (au plus [`MAX_ENTREES_SYNCHRO`] par envoi, corps ≤ 2 Mio) s'accumulent
//!   **en attente** pour ce COG. `depuis` doit valoir `taille du miroir +
//!   entrées en attente` ; sinon (morceau incohérent) l'attente est **purgée**
//!   et `409` rend `taille` (reprendre à cet index). L'attente est bornée par
//!   COG : miroir + attente ≤ [`MAX_ENTREES_SUIVI`] entrées (1 048 576) et
//!   ≤ [`MAX_OCTETS_SUIVI`] octets (256 Mio), comme `MiyuCloud` ; au-delà, purge
//!   et `413`. Une attente sans tête depuis plus de 15 min est purgée.
//! - **Tête** : l'envoi qui porte `tete` valide le tout (attente + ses propres
//!   entrées) : racine reconstruite = racine signée. Refus ⇒ attente purgée,
//!   rien d'autre n'est modifié. Sans attente, `depuis` ≤ taille du miroir
//!   est toléré (chevauchement).
//! - Rejeu à l'identique (même tête, rien de neuf) ⇒ `200`, `rejeu: true`,
//!   sans recalcul.
//! - Débit limité par IP et par COG (limiteur d'Origin), nombre d'entrées
//!   contrôlé **avant** tout décodage base64, refus journalisés **agrégés**
//!   (un résumé par minute au plus, pas un `warn!` par requête).
//!
//! Limite connue : la route n'étant pas authentifiée, un tiers peut purger
//! l'attente d'un COG (morceau incohérent) ; il ne peut rien faire accepter.
//! Le COG reprend alors à `taille` ; la liste détenue reste valable 15 min.
//!
//! # Fail-closed
//!
//! COG sans registre déclaré, jamais synchronisé, tête de plus de 15 min, datée
//! dans le futur, verrou empoisonné ⇒ erreur de révocation, donc **refus** du
//! passeport ou du visa par miyauth. Miroir en mémoire : après redémarrage, le
//! COG renvoie son registre depuis l'index 0 (par morceaux au besoin).

use std::collections::BTreeMap;
use std::sync::Mutex;

use miyauth::http::b64_decoder;
use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{RefusSuivi, SuiviRevocationsRegistre, MAX_ENTREES_SUIVI, MAX_OCTETS_SUIVI};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tracing::warn;

use super::server::RouteResponse;
use crate::config::ForumConfig;

/// Route de synchronisation des registres de révocation.
pub const ROUTE_REVOCATIONS: &str = "/api/auth/revocations/sync";
/// Taille maximale du corps d'un envoi (tête + entrées, base64).
pub const TAILLE_MAX_CORPS_REVOCATIONS: usize = 2 * 1024 * 1024;
/// Nombre maximal d'entrées par envoi (un morceau).
pub const MAX_ENTREES_SYNCHRO: usize = 4096;
/// Durée de vie d'une attente sans tête (15 min).
pub const DUREE_ATTENTE_SECS: u64 = FRAICHEUR_MAX_DEFAUT_SECS;
/// Fenêtre d'agrégation des refus journalisés.
pub const FENETRE_JOURNAL_REFUS_SECS: u64 = 60;

/// Registre déclaré d'un COG reconnu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistreReconnu {
    /// COG reconnu.
    pub cog: CogId,
    /// Identifiant de son registre (32 o).
    pub registre: [u8; 32],
    /// Clé publique de journal : signe ses têtes et ses entrées de révocation.
    pub cle_journal: [u8; 32],
}

/// Refus d'une synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusSynchro {
    /// COG sans registre déclaré.
    CogInconnu,
    /// Position incohérente : reprendre à `taille` (attente purgée).
    Position {
        /// Taille actuelle du miroir.
        taille: u64,
    },
    /// Trop d'entrées dans un envoi.
    TropDEntrees,
    /// Registre ou attente au-delà des bornes par COG (attente purgée).
    Bornes,
    /// Tête ou entrées refusées par le registre (signature, fraîcheur, racine…).
    Registre(String),
    /// État indisponible.
    EtatIndisponible,
}

impl RefusSynchro {
    /// Motif court (journal agrégé, réponse).
    pub fn motif(&self) -> &'static str {
        match self {
            Self::CogInconnu => "cog_inconnu",
            Self::Position { .. } => "position",
            Self::TropDEntrees => "trop_d_entrees",
            Self::Bornes => "bornes",
            Self::Registre(_) => "rejected",
            Self::EtatIndisponible => "unavailable",
        }
    }
}

impl std::fmt::Display for RefusSynchro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CogInconnu => f.write_str("COG sans registre déclaré"),
            Self::Position { taille } => write!(f, "position incorrecte (taille du miroir : {taille})"),
            Self::TropDEntrees => write!(f, "plus de {MAX_ENTREES_SYNCHRO} entrées"),
            Self::Bornes => f.write_str("registre au-delà des bornes"),
            Self::Registre(e) => write!(f, "registre : {e}"),
            Self::EtatIndisponible => f.write_str("état indisponible"),
        }
    }
}

/// Synchronisation acceptée (morceau mis en attente, ou tête appliquée).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Synchronise {
    /// Taille du miroir.
    pub taille: u64,
    /// Entrées en attente (après le miroir) ; 0 après une tête.
    pub en_attente: u64,
    /// Horodatage de la tête vérifiée la plus récente (aucune : jamais synchronisé).
    pub a_jour_au: Option<u64>,
    /// Révocations nouvelles prises en compte.
    pub ajoutees: usize,
    /// Tête déjà appliquée, rien de neuf (aucun recalcul).
    pub rejeu: bool,
}

/// Entrées reçues sans tête, en attente de validation.
#[derive(Debug, Default)]
struct Attente {
    entrees: Vec<Vec<u8>>,
    octets: u64,
    /// Taille du miroir quand l'attente a commencé.
    base: u64,
    /// Heure du premier morceau.
    debut: u64,
}

impl Attente {
    fn purger(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug)]
struct SuiviCog {
    suivi: SuiviRevocationsRegistre,
    /// Sérialise les synchronisations de ce COG (jamais pris par les vérifications).
    attente: Mutex<Attente>,
}

/// Journal agrégé des refus : un résumé par fenêtre au plus.
#[derive(Debug, Default)]
pub struct JournalRefus {
    etat: Mutex<(BTreeMap<&'static str, u64>, u64)>,
}

impl JournalRefus {
    /// Note un refus de motif `motif` ; rend le résumé à journaliser quand la
    /// fenêtre ([`FENETRE_JOURNAL_REFUS_SECS`]) ouverte par le premier refus est close.
    pub fn noter(&self, motif: &'static str, maintenant: u64) -> Option<String> {
        let mut etat = self.etat.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let (comptes, debut) = &mut *etat;
        if comptes.is_empty() {
            *debut = maintenant;
        }
        *comptes.entry(motif).or_default() += 1;
        if maintenant < debut.saturating_add(FENETRE_JOURNAL_REFUS_SECS) {
            return None;
        }
        let total: u64 = comptes.values().sum();
        let detail: Vec<String> = comptes.iter().map(|(m, n)| format!("{m}={n}")).collect();
        let resume = format!("{total} refus en {} s ({})", maintenant.saturating_sub(*debut), detail.join(", "));
        comptes.clear();
        Some(resume)
    }
}

/// Révocations des COG reconnus (un suivi par COG, tiré de son registre vérifié).
#[derive(Debug, Default)]
pub struct RevocationsReconnues {
    /// Fixée à la construction : aucun verrou global.
    cogs: BTreeMap<CogId, SuiviCog>,
    refus: JournalRefus,
}

impl RevocationsReconnues {
    /// Suivi des registres déclarés. Doublon de COG ou clé invalide ⇒ erreur.
    pub fn new(registres: &[RegistreReconnu]) -> Result<Self, String> {
        let mut cogs = BTreeMap::new();
        for r in registres {
            let suivi = SuiviRevocationsRegistre::new(r.cog.clone(), r.registre, &r.cle_journal)
                .map_err(|e| format!("cle_journal ({}) : {e}", r.cog))?;
            let ancien = cogs.insert(r.cog.clone(), SuiviCog { suivi, attente: Mutex::new(Attente::default()) });
            if ancien.is_some() {
                return Err(format!("registre déclaré deux fois pour {}", r.cog));
            }
        }
        Ok(Self { cogs, refus: JournalRefus::default() })
    }

    /// Registres déclarés dans `[[forum.cogs_reconnus]]` (`registre` + `cle_journal`).
    /// Un seul des deux champs, valeur invalide ⇒ erreur (aucune entrée ignorée).
    pub fn depuis_config(config: &ForumConfig) -> Result<Self, String> {
        let hex32 = |texte: &str, champ: &str| -> Result<[u8; 32], String> {
            hex::decode(texte.trim())
                .ok()
                .and_then(|v| v.try_into().ok())
                .ok_or_else(|| format!("forum.cogs_reconnus.{champ} : 32 octets hexadécimaux attendus"))
        };
        let mut registres = Vec::new();
        for c in &config.cogs_reconnus {
            let cog = CogId::new(c.cog_id.trim()).ok_or_else(|| format!("forum.cogs_reconnus : cog_id invalide « {} »", c.cog_id))?;
            match (c.registre.as_deref(), c.cle_journal.as_deref()) {
                (None, None) => {}
                (Some(registre), Some(cle)) => registres.push(RegistreReconnu {
                    cog,
                    registre: hex32(registre, "registre")?,
                    cle_journal: hex32(cle, "cle_journal")?,
                }),
                _ => return Err(format!("forum.cogs_reconnus ({cog}) : `registre` et `cle_journal` vont ensemble")),
            }
        }
        Self::new(&registres)
    }

    /// Vrai si le registre de `cog` est suivi.
    pub fn suit(&self, cog: &CogId) -> bool {
        self.cogs.contains_key(cog)
    }

    /// Taille du miroir de `cog`.
    pub fn taille(&self, cog: &CogId) -> Option<u64> {
        self.cogs.get(cog)?.suivi.taille()
    }

    /// Note un refus de la route ; journalise un résumé agrégé au plus une fois par fenêtre.
    pub fn noter_refus(&self, motif: &'static str, maintenant: u64) {
        if let Some(resume) = self.refus.noter(motif, maintenant) {
            warn!("Synchro des révocations : {resume}");
        }
    }

    /// Synchronise `cog` : morceau (`tete` absente, entrées mises en attente)
    /// ou tête (attente + `entrees` validées ensemble).
    ///
    /// Atomicité : le miroir et la liste de `cog` sont modifiés ensemble ou pas
    /// du tout ([`SuiviRevocationsRegistre::appliquer`]). Seule l'**attente**
    /// change sur refus : un morceau incohérent ou une tête refusée la purge.
    pub fn synchroniser(
        &self,
        cog: &CogId,
        depuis: u64,
        tete: Option<&[u8]>,
        entrees: &[Vec<u8>],
        maintenant: u64,
    ) -> Result<Synchronise, RefusSynchro> {
        if entrees.len() > MAX_ENTREES_SYNCHRO {
            return Err(RefusSynchro::TropDEntrees);
        }
        let suivi_cog = self.cogs.get(cog).ok_or(RefusSynchro::CogInconnu)?;
        let mut attente = suivi_cog.attente.lock().map_err(|_| RefusSynchro::EtatIndisponible)?;
        let suivi = &suivi_cog.suivi;
        let taille = suivi.taille().ok_or(RefusSynchro::EtatIndisponible)?;
        let perimee = maintenant > attente.debut.saturating_add(DUREE_ATTENTE_SECS);
        if !attente.entrees.is_empty() && (attente.base != taille || perimee) {
            attente.purger();
        }
        let en_attente = attente.entrees.len() as u64;
        let attendu = taille.saturating_add(en_attente);

        let Some(tete) = tete else {
            // Morceau : position exacte, bornes cumulées par COG.
            if depuis != attendu {
                attente.purger();
                return Err(RefusSynchro::Position { taille });
            }
            let octets = entrees.iter().fold(attente.octets, |t, e| t.saturating_add(e.len() as u64));
            if attendu.saturating_add(entrees.len() as u64) > MAX_ENTREES_SUIVI || octets > MAX_OCTETS_SUIVI {
                attente.purger();
                return Err(RefusSynchro::Bornes);
            }
            if attente.entrees.is_empty() {
                attente.base = taille;
                attente.debut = maintenant;
            }
            attente.entrees.extend_from_slice(entrees);
            attente.octets = octets;
            return Ok(Synchronise {
                taille,
                en_attente: attente.entrees.len() as u64,
                a_jour_au: suivi.a_jour_au(),
                ajoutees: 0,
                rejeu: false,
            });
        };

        // Tête : valide l'attente et les entrées de cet envoi, ensemble.
        let resultat = if attente.entrees.is_empty() {
            suivi.appliquer(tete, depuis, entrees, maintenant)
        } else if depuis != attendu {
            attente.purger();
            return Err(RefusSynchro::Position { taille });
        } else {
            let mut toutes = std::mem::take(&mut attente.entrees);
            toutes.extend_from_slice(entrees);
            suivi.appliquer(tete, taille, &toutes, maintenant)
        };
        attente.purger();
        match resultat {
            Ok(a) => Ok(Synchronise {
                taille: a.taille,
                en_attente: 0,
                a_jour_au: Some(a.a_jour_au),
                ajoutees: a.ajoutees,
                rejeu: a.rejeu,
            }),
            Err(RefusSuivi::Couverture { taille, .. }) => Err(RefusSynchro::Position { taille }),
            Err(RefusSuivi::Bornes) => Err(RefusSynchro::Bornes),
            Err(RefusSuivi::EtatIndisponible) => Err(RefusSynchro::EtatIndisponible),
            Err(e @ RefusSuivi::Registre(_)) => Err(RefusSynchro::Registre(e.to_string())),
        }
    }
}

impl SourceRevocation for RevocationsReconnues {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        // Aucun verrou global : seul le RwLock (lecture) du suivi du COG est pris.
        self.cogs.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?.suivi.est_revoque(emetteur, cible, maintenant_secs)
    }
}

// ---------------------------------------------------------------------------
// Route HTTP
// ---------------------------------------------------------------------------

/// Corps JSON de `POST /api/auth/revocations/sync` (poussé par un COG reconnu).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevocationsSyncRequest {
    /// COG dont le registre est poussé.
    pub cog_id: String,
    /// Index de la première entrée envoyée.
    pub depuis: u64,
    /// Tête de registre scellée (base64 URL) ; absente : morceau mis en attente.
    #[serde(default)]
    pub tete: Option<String>,
    /// Entrées `depuis..` (base64 URL), au plus [`MAX_ENTREES_SYNCHRO`].
    pub entrees: Vec<String>,
}

/// Requête analysée (COG valide, nombre d'entrées borné), pas encore décodée.
#[derive(Debug)]
pub struct RequeteSynchro {
    /// COG visé.
    pub cog: CogId,
    requete: RevocationsSyncRequest,
}

fn reponse(statut: u16, corps: &serde_json::Value) -> RouteResponse {
    let ligne = match statut {
        200 => "200 OK",
        400 => "400 Bad Request",
        403 => "403 Forbidden",
        409 => "409 Conflict",
        413 => "413 Payload Too Large",
        429 => "429 Too Many Requests",
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    RouteResponse::Normal {
        status: ligne.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: corps.to_string(),
    }
}

/// Réponse de refus `statut` de motif `motif` (noté au journal agrégé).
pub fn refus(revocations: &RevocationsReconnues, statut: u16, motif: &'static str, maintenant: u64) -> RouteResponse {
    revocations.noter_refus(motif, maintenant);
    reponse(statut, &serde_json::json!({ "ok": false, "error": motif }))
}

/// Clé du limiteur de débit pour `cog` (16 premiers octets de SHA-256).
pub fn cle_debit(cog: &CogId) -> [u8; 16] {
    let h = Sha256::digest(format!("origin/revocations/sync/{cog}").as_bytes());
    let mut cle = [0u8; 16];
    cle.copy_from_slice(&h[..16]);
    cle
}

/// Analyse le corps : JSON, `cog_id`, nombre d'entrées (**avant** tout décodage base64).
pub fn analyser(revocations: &RevocationsReconnues, body: &[u8], maintenant: u64) -> Result<RequeteSynchro, RouteResponse> {
    if body.len() > TAILLE_MAX_CORPS_REVOCATIONS {
        return Err(refus(revocations, 413, "payload_too_large", maintenant));
    }
    let Ok(requete) = serde_json::from_slice::<RevocationsSyncRequest>(body) else {
        return Err(refus(revocations, 400, "invalid_json", maintenant));
    };
    if requete.entrees.len() > MAX_ENTREES_SYNCHRO {
        return Err(refus(revocations, 413, "trop_d_entrees", maintenant));
    }
    let Some(cog) = CogId::new(requete.cog_id.trim()) else {
        return Err(refus(revocations, 400, "cog_id", maintenant));
    };
    if !revocations.suit(&cog) {
        return Err(refus(revocations, 403, "cog_inconnu", maintenant));
    }
    Ok(RequeteSynchro { cog, requete })
}

/// Décode et applique une requête analysée (calcul lourd : hors de la boucle async).
pub fn traiter(revocations: &RevocationsReconnues, r: &RequeteSynchro, maintenant: u64) -> RouteResponse {
    let tete = match r.requete.tete.as_deref().map(b64_decoder) {
        None => None,
        Some(Some(t)) => Some(t),
        Some(None) => return refus(revocations, 400, "base64", maintenant),
    };
    let Some(entrees) = r.requete.entrees.iter().map(|e| b64_decoder(e)).collect::<Option<Vec<Vec<u8>>>>() else {
        return refus(revocations, 400, "base64", maintenant);
    };
    match revocations.synchroniser(&r.cog, r.requete.depuis, tete.as_deref(), &entrees, maintenant) {
        Ok(s) => reponse(
            200,
            &serde_json::json!({
                "ok": true,
                "taille": s.taille,
                "en_attente": s.en_attente,
                "a_jour_au": s.a_jour_au,
                "ajoutees": s.ajoutees,
                "rejeu": s.rejeu,
            }),
        ),
        Err(e) => {
            revocations.noter_refus(e.motif(), maintenant);
            let statut = match &e {
                RefusSynchro::CogInconnu => 403,
                RefusSynchro::Position { .. } => 409,
                RefusSynchro::TropDEntrees | RefusSynchro::Bornes => 413,
                RefusSynchro::Registre(_) => 400,
                RefusSynchro::EtatIndisponible => 503,
            };
            let mut corps = serde_json::json!({ "ok": false, "error": e.motif() });
            if let RefusSynchro::Position { taille } = e {
                corps["taille"] = taille.into();
            }
            reponse(statut, &corps)
        }
    }
}

/// [`analyser`] puis [`traiter`] (sans limitation de débit : voir `web::server`).
pub fn route_synchro(revocations: &RevocationsReconnues, body: &[u8], maintenant: u64) -> RouteResponse {
    match analyser(revocations, body, maintenant) {
        Ok(r) => traiter(revocations, &r, maintenant),
        Err(refus) => refus,
    }
}

/// Banc d'essai : COG reconnu tenant un journal (tests seulement).
#[cfg(test)]
pub(crate) mod banc {
    use super::*;
    use miyuregistre::{cible_serie, Cible, Corps, EntreeRegistre, Journal, StockageMemoire};
    use miyusceau::{sceller, CleSignature};

    /// Registre d'un COG reconnu (côté COG).
    pub(crate) struct RegistreCog {
        pub(crate) cog: CogId,
        pub(crate) registre: [u8; 32],
        pub(crate) cle_journal: CleSignature,
        pub(crate) journal: Journal<StockageMemoire>,
    }

    impl RegistreCog {
        pub(crate) fn new(cog: CogId) -> Self {
            let registre = [0x52; 32];
            Self {
                journal: Journal::ouvrir(registre, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
                cog,
                registre,
                cle_journal: CleSignature::generer(),
            }
        }

        pub(crate) fn declaration(&self) -> RegistreReconnu {
            RegistreReconnu { cog: self.cog.clone(), registre: self.registre, cle_journal: self.cle_journal.cle_publique() }
        }

        /// Inscrit une révocation signée par `cle` (la clé de journal par défaut).
        pub(crate) fn revoquer(&mut self, cible: &CibleRevocation, cle: Option<&CleSignature>, t: u64) {
            let cible = match cible {
                CibleRevocation::Cle(kid) => Cible::Cle(*kid),
                CibleRevocation::Serie(s) => cible_serie(s),
            };
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible, motif: "test".into() } };
            let octets = sceller(&e, cle.unwrap_or(&self.cle_journal)).unwrap();
            self.journal.ajouter(&octets).unwrap();
        }

        /// Tête émise à `t` et entrées depuis `depuis`.
        pub(crate) fn publier(&mut self, depuis: u64, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            let tete = self.journal.emettre_tete(&[&self.cle_journal], t, 86_400).unwrap();
            (tete, self.journal.entrees(depuis, u64::MAX).unwrap())
        }

        /// Synchronise `origin` (tête à `t`).
        pub(crate) fn pousser(&mut self, origin: &RevocationsReconnues, t: u64) -> Result<Synchronise, RefusSynchro> {
            let depuis = origin.taille(&self.cog).unwrap_or(0);
            let (tete, entrees) = self.publier(depuis, t);
            origin.synchroniser(&self.cog, depuis, Some(&tete), &entrees, t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::banc::RegistreCog;
    use super::*;
    use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
    use miyusceau::CleSignature;

    const T0: u64 = 1_800_000_000;

    fn cog(c: char) -> CogId {
        CogId::new(&c.to_string().repeat(26)).unwrap()
    }

    #[test]
    fn revocations_du_registre_appliquees() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        let cible = CibleRevocation::Cle([7; 32]);
        // Jamais synchronisé : liste absente (refus).
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Err(ErreurRevocation::ListeAbsente));

        r.revoquer(&cible, None, T0);
        r.revoquer(&CibleRevocation::Serie([4; 16]), None, T0);
        // Signée par une clé non épinglée : ignorée.
        let intrus = CleSignature::generer();
        r.revoquer(&CibleRevocation::Cle([8; 32]), Some(&intrus), T0);
        let s = r.pousser(&origin, T0).unwrap();
        assert_eq!((s.taille, s.a_jour_au, s.ajoutees), (3, Some(T0), 2));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Serie([4; 16]), T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([8; 32]), T0), Ok(false));
        // COG non déclaré : absente.
        assert_eq!(origin.est_revoque(&cog('z'), &cible, T0), Err(ErreurRevocation::ListeAbsente));

        // Synchronisation incrémentale.
        r.revoquer(&CibleRevocation::Cle([9; 32]), None, T0 + 60);
        assert_eq!(r.pousser(&origin, T0 + 60).unwrap().ajoutees, 1);
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([9; 32]), T0 + 60), Ok(true));
    }

    #[test]
    fn declaration_depuis_config() {
        use crate::config::{CogReconnuConfig, ForumConfig};
        let entree = |registre: Option<&str>, cle: Option<&str>| CogReconnuConfig {
            cog_id: "h".repeat(26),
            cle_emission: "11".repeat(32),
            registre: registre.map(str::to_string),
            cle_journal: cle.map(str::to_string),
        };
        let config = |e: CogReconnuConfig| ForumConfig { cogs_reconnus: vec![e], ..Default::default() };
        let cle = hex::encode(CleSignature::generer().cle_publique());
        let r = RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&cle)))).unwrap();
        assert!(r.suit(&cog('h')));
        // Sans registre déclaré : non suivi (ses passeports seront refusés).
        assert!(!RevocationsReconnues::depuis_config(&config(entree(None, None))).unwrap().suit(&cog('h')));
        // Un seul des deux champs, ou valeur invalide : erreur.
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), None))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(None, Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some("52"), Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&"00".repeat(32))))).is_err());
    }

    #[test]
    fn perimee_ou_invalide_refusee() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        r.pousser(&origin, T0).unwrap();
        let cible = CibleRevocation::Cle([1; 32]);
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS), Ok(false));
        // Au-delà de 15 min sans nouvelle tête : périmée, refus.
        assert!(matches!(
            origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));

        // Mauvaise position : 409 avec la taille.
        let (tete, entrees) = r.publier(0, T0 + 10);
        assert_eq!(origin.synchroniser(&r.cog, 5, Some(&tete), &entrees, T0 + 10), Err(RefusSynchro::Position { taille: 0 }));
        // Tête signée par une autre clé que la clé de journal : refus, état inchangé.
        let mut faux = RegistreCog::new(r.cog.clone());
        let (tete, entrees) = faux.publier(0, T0 + 20);
        assert!(matches!(origin.synchroniser(&r.cog, 0, Some(&tete), &entrees, T0 + 20), Err(RefusSynchro::Registre(_))));
        // Entrée omise : position à reprendre ; entrée altérée (racine différente) : refus.
        r.revoquer(&cible, None, T0 + 30);
        let (tete, entrees) = r.publier(0, T0 + 30);
        assert_eq!(origin.synchroniser(&r.cog, 0, Some(&tete), &[], T0 + 30), Err(RefusSynchro::Position { taille: 0 }));
        let mut alterees = entrees.clone();
        alterees[0] = b"faux".to_vec();
        assert!(matches!(origin.synchroniser(&r.cog, 0, Some(&tete), &alterees, T0 + 30), Err(RefusSynchro::Registre(_))));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + 30), Ok(false));
        // COG inconnu, trop d'entrées.
        assert_eq!(origin.synchroniser(&cog('z'), 0, Some(&tete), &[], T0), Err(RefusSynchro::CogInconnu));
        let trop = vec![Vec::new(); MAX_ENTREES_SYNCHRO + 1];
        assert_eq!(origin.synchroniser(&r.cog, 0, Some(&tete), &trop, T0), Err(RefusSynchro::TropDEntrees));
        // Déclaration en double refusée.
        assert!(RevocationsReconnues::new(&[r.declaration(), r.declaration()]).is_err());
    }

    /// Pousse `entrees` par morceaux de `pas` puis la tête seule.
    fn pousser_par_morceaux(
        origin: &RevocationsReconnues,
        c: &CogId,
        depuis: u64,
        tete: &[u8],
        entrees: &[Vec<u8>],
        pas: usize,
        t: u64,
    ) -> Result<Synchronise, RefusSynchro> {
        let mut position = depuis;
        for morceau in entrees.chunks(pas) {
            let s = origin.synchroniser(c, position, None, morceau, t)?;
            position += morceau.len() as u64;
            assert_eq!(s.taille + s.en_attente, position);
        }
        origin.synchroniser(c, position, Some(tete), &[], t)
    }

    #[test]
    fn registre_de_5000_entrees_synchronise_par_morceaux() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..5000u32 {
            let mut kid = [0u8; 32];
            kid[..4].copy_from_slice(&i.to_be_bytes());
            r.revoquer(&CibleRevocation::Cle(kid), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        assert_eq!(entrees.len(), 5000);
        // D'un seul envoi : impossible (borne par envoi).
        assert_eq!(origin.synchroniser(&r.cog, 0, Some(&tete), &entrees, T0), Err(RefusSynchro::TropDEntrees));
        let s = pousser_par_morceaux(&origin, &r.cog, 0, &tete, &entrees, MAX_ENTREES_SYNCHRO, T0).unwrap();
        assert_eq!((s.taille, s.en_attente, s.ajoutees, s.a_jour_au), (5000, 0, 5000, Some(T0)));
        let mut derniere = [0u8; 32];
        derniere[..4].copy_from_slice(&4999u32.to_be_bytes());
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle(derniere), T0), Ok(true));
        // Rejeu à l'identique : 200, rien de recalculé.
        let s = origin.synchroniser(&r.cog, 5000, Some(&tete), &[], T0 + 1).unwrap();
        assert!(s.rejeu);
        assert_eq!((s.taille, s.ajoutees), (5000, 0));
    }

    #[test]
    fn morceau_incoherent_purge_l_attente() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..10u8 {
            r.revoquer(&CibleRevocation::Cle([i; 32]), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        assert_eq!(origin.synchroniser(&r.cog, 0, None, &entrees[..4], T0).unwrap().en_attente, 4);
        // Morceau à une mauvaise position : attente purgée, reprendre à `taille`.
        assert_eq!(origin.synchroniser(&r.cog, 6, None, &entrees[6..], T0), Err(RefusSynchro::Position { taille: 0 }));
        assert_eq!(origin.synchroniser(&r.cog, 4, None, &entrees[4..], T0), Err(RefusSynchro::Position { taille: 0 }));
        // Morceau falsifié : la tête refuse le tout, l'attente est purgée, rien n'est appliqué.
        origin.synchroniser(&r.cog, 0, None, &entrees[..5], T0).unwrap();
        origin.synchroniser(&r.cog, 5, None, &[b"faux".to_vec()], T0).unwrap();
        assert!(matches!(origin.synchroniser(&r.cog, 6, Some(&tete), &entrees[6..], T0), Err(RefusSynchro::Registre(_))));
        assert_eq!(origin.taille(&r.cog), Some(0));
        assert_eq!(origin.synchroniser(&r.cog, 6, None, &[], T0), Err(RefusSynchro::Position { taille: 0 }));
        // Tête avant la fin des morceaux : position à reprendre, attente purgée.
        origin.synchroniser(&r.cog, 0, None, &entrees[..5], T0).unwrap();
        assert_eq!(origin.synchroniser(&r.cog, 5, Some(&tete), &[], T0), Err(RefusSynchro::Position { taille: 0 }));
        // Attente abandonnée plus de 15 min : purgée au morceau suivant.
        origin.synchroniser(&r.cog, 0, None, &entrees[..5], T0).unwrap();
        let tard = T0 + DUREE_ATTENTE_SECS + 1;
        assert_eq!(origin.synchroniser(&r.cog, 5, None, &entrees[5..], tard), Err(RefusSynchro::Position { taille: 0 }));
        // Puis tout en règle : accepté (dernière partie portée par l'envoi de la tête).
        origin.synchroniser(&r.cog, 0, None, &entrees[..5], T0).unwrap();
        let s = origin.synchroniser(&r.cog, 5, Some(&tete), &entrees[5..], T0).unwrap();
        assert_eq!((s.taille, s.ajoutees, s.en_attente), (10, 10, 0));
    }

    #[test]
    fn attente_bornee_par_cog() {
        let r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        // Octets : 256 Mio au plus en attente. Pages nulles non touchées : rien n'est
        // réellement occupé, le refus a lieu avant toute copie.
        origin.synchroniser(&r.cog, 0, None, &[vec![1u8]], T0).unwrap();
        let gros: Vec<Vec<u8>> = (0..MAX_ENTREES_SYNCHRO).map(|_| vec![0u8; 64 * 1024]).collect();
        assert_eq!(origin.synchroniser(&r.cog, 1, None, &gros, T0), Err(RefusSynchro::Bornes));
        drop(gros);
        // Purgée : on repart de 0.
        assert_eq!(origin.synchroniser(&r.cog, 1, None, &[], T0), Err(RefusSynchro::Position { taille: 0 }));
        // Entrées : miroir + attente ≤ 1 048 576.
        let petit = vec![vec![1u8]; MAX_ENTREES_SYNCHRO];
        let mut position = 0;
        while position + (MAX_ENTREES_SYNCHRO as u64) <= MAX_ENTREES_SUIVI {
            origin.synchroniser(&r.cog, position, None, &petit, T0).unwrap();
            position += MAX_ENTREES_SYNCHRO as u64;
        }
        assert_eq!(origin.synchroniser(&r.cog, position, None, &petit[..1], T0), Err(RefusSynchro::Bornes));
    }

    #[test]
    fn refus_journalises_par_agregat() {
        let j = JournalRefus::default();
        assert_eq!(j.noter("position", T0), None);
        for _ in 0..100 {
            assert_eq!(j.noter("rejected", T0 + 10), None);
        }
        let resume = j.noter("position", T0 + FENETRE_JOURNAL_REFUS_SECS).unwrap();
        assert!(resume.starts_with("102 refus"), "{resume}");
        assert!(resume.contains("position=2") && resume.contains("rejected=100"), "{resume}");
        assert_eq!(j.noter("position", T0 + 61), None, "nouvelle fenêtre");
    }

    #[test]
    fn route_controle_avant_decodage() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        let statut = |rep: &RouteResponse| match rep {
            RouteResponse::Normal { status, .. } => status.clone(),
            _ => String::new(),
        };
        // Trop d'entrées : refus avant de décoder (chaînes invalides en base64).
        let corps = serde_json::json!({
            "cog_id": r.cog.as_str(), "depuis": 0, "entrees": vec!["!!"; MAX_ENTREES_SYNCHRO + 1],
        })
        .to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("413"));
        // COG inconnu : 403 avant décodage.
        let corps = serde_json::json!({"cog_id": cog('z').as_str(), "depuis": 0, "entrees": ["!!"]}).to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("403"));
        assert!(statut(&route_synchro(&origin, b"{}", T0)).starts_with("400"));
        // Morceau puis tête, par la route.
        r.revoquer(&CibleRevocation::Cle([3; 32]), None, T0);
        let (tete, entrees) = r.publier(0, T0);
        let b64 = |o: &[u8]| miyauth::http::b64_encoder(o);
        let corps = serde_json::json!({"cog_id": r.cog.as_str(), "depuis": 0, "entrees": [b64(&entrees[0])]}).to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("200"));
        let corps = serde_json::json!({"cog_id": r.cog.as_str(), "depuis": 1, "tete": b64(&tete), "entrees": []}).to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("200"));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([3; 32]), T0), Ok(true));
        assert_ne!(cle_debit(&r.cog), cle_debit(&cog('z')));
    }
}
