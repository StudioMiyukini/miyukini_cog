//! @id     miyuregistre_editeur_origin
//! @do     pousser_registre_revocations_origin
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — client de synchro des révocations vers Origin : morceaux bornés, enveloppe A-014 signée par la clé de journal, échéance courte, reprise sur 409, arrêt sur 401/403, débit tenu.

//! Envois du registre de révocation vers Origin
//! (`POST /api/auth/revocations/sync`, A-012, A-013, A-014).
//!
//! # Déroulé d'une publication ([`PublicationOrigin::publier`])
//!
//! - Position de départ : celle qu'Origin a confirmée à la publication
//!   précédente ; inconnue (premier cycle, redémarrage, erreur) ⇒ la tête
//!   seule est envoyée à la taille du journal (sonde) : à jour, Origin
//!   l'accepte ; sinon il répond `409` avec la position à reprendre.
//! - Découpage : des **morceaux** sans tête, chacun d'au plus
//!   [`MAX_ENTREES_SYNCHRO`] entrées et d'un corps JSON d'au plus
//!   [`TAILLE_MAX_CORPS_REVOCATIONS`] (base64 compris, marge
//!   [`MARGE_CORPS`]), puis un **dernier envoi** portant la tête et les
//!   entrées restantes (ou la tête seule si elle ne tient pas avec elles).
//! - Chaque envoi porte sa propre enveloppe signée par la clé de journal
//!   ([`octets_enveloppe`]), échéance = maintenant + `validite_enveloppe`
//!   (défaut [`VALIDITE_ENVELOPPE_DEFAUT_SECS`], au plus 15 min), rendue
//!   strictement croissante d'un envoi à l'autre : un renvoi après `409` ou
//!   `429` n'est jamais pris pour un rejeu (sans effet) par Origin.
//! - Réponses :
//!   - `200` (morceau) : position suivante = `taille + en_attente` rendus par
//!     Origin (rejeu d'enveloppe compris) ; `200` (tête) : terminé, Origin
//!     doit annoncer `taille` = taille du journal.
//!   - `409` : reprise à `attendu` (sans purge côté Origin) ; au plus
//!     `max_conflits` reprises par publication.
//!   - `401` / `403` : **arrêt** ([`ErreurOrigin::Refusee`], définitive) — clé
//!     de journal non épinglée, COG non déclaré, horloge hors fenêtre : aucun
//!     nouvel essai ne peut réussir sans intervention.
//!   - `429` : pause `attente_debit` puis nouvel essai (au plus
//!     `max_attentes_debit` fois).
//!   - autre statut ou réponse illisible : échec de la publication (position
//!     oubliée, sonde au cycle suivant).
//! - Débit : au plus un envoi par `intervalle_min` (défaut 550 ms, sous la
//!   limite d'Origin de 120 envois par minute et par IP).

use std::time::{Duration, Instant};

use ed25519_dalek::{Signer, SigningKey};
use miyauth::http::b64_encoder;
use miyauth::CogId;
use miyusceau::CleSignature;

use super::Instantane;
use crate::enveloppe::{octets_enveloppe, DUREE_MAX_ENVELOPPE_SECS, MAX_ENTREES_SYNCHRO, TAILLE_MAX_CORPS_REVOCATIONS};

/// Échéance par défaut d'une enveloppe (10 min : tolère une dérive d'horloge).
pub const VALIDITE_ENVELOPPE_DEFAUT_SECS: u64 = 10 * 60;
/// Marge réservée dans le corps aux champs hors entrées et tête (JSON, signature).
pub const MARGE_CORPS: usize = 16 * 1024;
/// Taille maximale lue d'une réponse d'Origin.
pub const TAILLE_MAX_REPONSE: u64 = 64 * 1024;

/// Réponse HTTP brute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReponseHttp {
    /// Statut HTTP.
    pub statut: u16,
    /// Corps (au plus [`TAILLE_MAX_REPONSE`] octets).
    pub corps: Vec<u8>,
}

/// Transport d'un corps JSON vers la route de synchro d'Origin.
pub trait Transport {
    /// Poste `corps` (JSON) ; toute réponse HTTP, même un refus, est un `Ok`.
    fn poster(&mut self, corps: &[u8]) -> Result<ReponseHttp, String>;
}

/// Réglages du client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReglagesOrigin {
    /// Échéance des enveloppes (secondes, ≤ 15 min).
    pub validite_enveloppe_secs: u64,
    /// Reprises sur `409` (ou sans progrès) tolérées par publication.
    pub max_conflits: u32,
    /// Intervalle minimal entre deux envois.
    pub intervalle_min: Duration,
    /// Pause après un `429`.
    pub attente_debit: Duration,
    /// Nombre de `429` tolérés par publication.
    pub max_attentes_debit: u32,
}

impl Default for ReglagesOrigin {
    fn default() -> Self {
        Self {
            validite_enveloppe_secs: VALIDITE_ENVELOPPE_DEFAUT_SECS,
            max_conflits: 8,
            intervalle_min: Duration::from_millis(550),
            attente_debit: Duration::from_secs(5),
            max_attentes_debit: 12,
        }
    }
}

/// Publication vers Origin en échec.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurOrigin {
    /// `401` / `403` : refus d'authentification ou COG non déclaré (arrêt).
    #[error("Origin refuse les envois ({statut} {motif}) : arrêt")]
    Refusee {
        /// Statut HTTP.
        statut: u16,
        /// Motif rendu par Origin.
        motif: String,
    },
    /// Échec du transport (connexion, délai).
    #[error("transport : {0}")]
    Transport(String),
    /// Statut inattendu.
    #[error("Origin : {statut} {motif}")]
    Statut {
        /// Statut HTTP.
        statut: u16,
        /// Motif rendu par Origin.
        motif: String,
    },
    /// Trop de reprises sans aboutir.
    #[error("trop de reprises ({0}) sans aboutir")]
    Conflits(u32),
    /// Trop de `429`.
    #[error("débit refusé par Origin ({0} fois)")]
    Debit(u32),
    /// Réponse illisible ou incohérente avec le journal local.
    #[error("réponse incohérente : {0}")]
    Incoherente(String),
    /// Entrée ou tête trop grande pour un envoi.
    #[error("entrée {0} trop grande pour un envoi")]
    TropGrande(u64),
    /// Réglage invalide.
    #[error("réglage invalide : {0}")]
    Reglage(&'static str),
}

impl ErreurOrigin {
    /// Vrai si l'erreur exige une intervention (aucun nouvel essai utile).
    pub fn est_definitive(&self) -> bool {
        matches!(self, Self::Refusee { .. } | Self::Reglage(_))
    }
}

/// Publication réussie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bilan {
    /// Envois effectués (refus compris).
    pub envois: u32,
    /// Reprises sur `409`.
    pub conflits: u32,
    /// Taille du miroir d'Origin.
    pub taille: u64,
    /// Révocations nouvelles prises en compte par Origin.
    pub ajoutees: u64,
    /// Tête déjà connue d'Origin (rien de neuf).
    pub rejeu: bool,
}

/// Client de publication vers Origin (un par COG).
pub struct PublicationOrigin<T: Transport> {
    cog: CogId,
    cle: SigningKey,
    transport: T,
    reglages: ReglagesOrigin,
    /// Position confirmée par Origin à la dernière publication réussie.
    position: Option<u64>,
    horloge: Box<dyn Fn() -> u64 + Send>,
    pause: Box<dyn Fn(Duration) + Send>,
    dernier_envoi: Option<Instant>,
    derniere_echeance: Option<u64>,
}

impl<T: Transport> std::fmt::Debug for PublicationOrigin<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublicationOrigin")
            .field("cog", &self.cog)
            .field("cle", &"<masquée>")
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}

/// Heure murale (secondes Unix).
pub fn maintenant_unix() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// Longueur d'un titre dans le JSON (base64 URL sans remplissage, guillemets, virgule).
fn longueur_json(octets: usize) -> usize {
    (octets * 4).div_ceil(3) + 3
}

/// Corps JSON d'un envoi signé.
fn corps_json(cog: &CogId, depuis: u64, tete: Option<&[u8]>, entrees: &[Vec<u8>], echeance: u64, signature: &[u8; 64]) -> Vec<u8> {
    let mut corps = serde_json::json!({
        "cog_id": cog.as_str(),
        "depuis": depuis,
        "entrees": entrees.iter().map(|e| b64_encoder(e)).collect::<Vec<_>>(),
        "echeance": echeance,
        "signature": b64_encoder(signature),
    });
    if let Some(t) = tete {
        corps["tete"] = b64_encoder(t).into();
    }
    corps.to_string().into_bytes()
}

fn champ(json: &serde_json::Value, nom: &str) -> Result<u64, ErreurOrigin> {
    json.get(nom).and_then(serde_json::Value::as_u64).ok_or_else(|| ErreurOrigin::Incoherente(format!("champ `{nom}` absent")))
}

fn motif(json: Option<&serde_json::Value>) -> String {
    json.and_then(|j| j.get("error")).and_then(serde_json::Value::as_str).unwrap_or("").to_string()
}

impl<T: Transport> PublicationOrigin<T> {
    /// Client pour `cog`, envois signés par `cle` (clé de journal).
    pub fn new(cog: CogId, cle: &CleSignature, transport: T) -> Self {
        Self {
            cog,
            cle: SigningKey::from_bytes(&cle.graine()),
            transport,
            reglages: ReglagesOrigin::default(),
            position: None,
            horloge: Box::new(maintenant_unix),
            pause: Box::new(std::thread::sleep),
            dernier_envoi: None,
            derniere_echeance: None,
        }
    }

    /// Remplace les réglages.
    pub fn avec_reglages(mut self, reglages: ReglagesOrigin) -> Self {
        self.reglages = reglages;
        self
    }

    /// Remplace l'horloge (secondes Unix) et la pause (tests).
    pub fn avec_horloge(mut self, horloge: impl Fn() -> u64 + Send + 'static, pause: impl Fn(Duration) + Send + 'static) -> Self {
        self.horloge = Box::new(horloge);
        self.pause = Box::new(pause);
        self
    }

    /// Position confirmée par Origin (aucune : sonde au prochain envoi).
    pub fn position(&self) -> Option<u64> {
        self.position
    }

    /// Transport (tests, diagnostic).
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Nombre d'entrées envoyées à partir de `depuis` et présence de la tête.
    fn decouper(&self, instantane: &Instantane, depuis: u64) -> Result<(usize, bool), ErreurOrigin> {
        let budget = TAILLE_MAX_CORPS_REVOCATIONS.saturating_sub(MARGE_CORPS);
        let debut = usize::try_from(depuis).map_err(|_| ErreurOrigin::TropGrande(depuis))?;
        let mut taille = 0usize;
        let mut k = 0usize;
        for e in instantane.entrees.get(debut..).unwrap_or(&[]) {
            let l = longueur_json(e.len());
            if k == MAX_ENTREES_SYNCHRO || taille + l > budget {
                break;
            }
            taille += l;
            k += 1;
        }
        let tout = debut + k == instantane.entrees.len();
        let tete = longueur_json(instantane.tete.len());
        if tout && taille + tete <= budget {
            return Ok((k, true));
        }
        if k == 0 {
            // Ni l'entrée suivante ni la tête seule ne tiennent dans un envoi.
            return Err(ErreurOrigin::TropGrande(depuis));
        }
        Ok((k, false))
    }

    fn envoyer(&mut self, depuis: u64, tete: Option<&[u8]>, entrees: &[Vec<u8>]) -> Result<(ReponseHttp, Option<serde_json::Value>), ErreurOrigin> {
        if let Some(d) = self.dernier_envoi {
            let ecoule = d.elapsed();
            if ecoule < self.reglages.intervalle_min {
                (self.pause)(self.reglages.intervalle_min.saturating_sub(ecoule));
            }
        }
        // Échéance strictement croissante : chaque envoi a une enveloppe
        // distincte (un renvoi identique serait un rejeu sans effet chez Origin).
        let maintenant = (self.horloge)();
        let base = maintenant.saturating_add(self.reglages.validite_enveloppe_secs);
        let echeance = match self.derniere_echeance {
            Some(d) if d >= base => d.saturating_add(1),
            _ => base,
        }
        .min(maintenant.saturating_add(DUREE_MAX_ENVELOPPE_SECS));
        self.derniere_echeance = Some(echeance);
        let signature = self.cle.sign(&octets_enveloppe(&self.cog, depuis, tete, entrees, echeance)).to_bytes();
        let corps = corps_json(&self.cog, depuis, tete, entrees, echeance, &signature);
        self.dernier_envoi = Some(Instant::now());
        let reponse = self.transport.poster(&corps).map_err(ErreurOrigin::Transport)?;
        let json = serde_json::from_slice::<serde_json::Value>(&reponse.corps).ok();
        Ok((reponse, json))
    }

    /// Publie `instantane` (voir le module). Toute erreur oublie la position
    /// confirmée (sonde au prochain appel).
    pub fn publier(&mut self, instantane: &Instantane) -> Result<Bilan, ErreurOrigin> {
        let resultat = self.publier_interne(instantane);
        match &resultat {
            Ok(_) => self.position = Some(instantane.taille),
            Err(_) => self.position = None,
        }
        resultat
    }

    fn publier_interne(&mut self, instantane: &Instantane) -> Result<Bilan, ErreurOrigin> {
        if self.reglages.validite_enveloppe_secs == 0 || self.reglages.validite_enveloppe_secs > DUREE_MAX_ENVELOPPE_SECS {
            return Err(ErreurOrigin::Reglage("validité de l'enveloppe hors 1..=900 s"));
        }
        let n = instantane.taille;
        if instantane.entrees.len() as u64 != n {
            return Err(ErreurOrigin::Incoherente("instantané : entrées ≠ taille".into()));
        }
        let mut position = self.position.filter(|&p| p <= n).unwrap_or(n);
        let (mut envois, mut conflits, mut debit) = (0u32, 0u32, 0u32);
        loop {
            let (k, avec_tete) = self.decouper(instantane, position)?;
            let debut = usize::try_from(position).map_err(|_| ErreurOrigin::TropGrande(position))?;
            let entrees = &instantane.entrees[debut..debut + k];
            let tete = avec_tete.then_some(instantane.tete.as_slice());
            let (reponse, json) = self.envoyer(position, tete, entrees)?;
            envois += 1;
            match reponse.statut {
                200 => {
                    let json = json.ok_or_else(|| ErreurOrigin::Incoherente("corps illisible".into()))?;
                    let taille = champ(&json, "taille")?;
                    if avec_tete {
                        if taille != n {
                            return Err(ErreurOrigin::Incoherente(format!("Origin annonce {taille} entrées, journal {n}")));
                        }
                        return Ok(Bilan {
                            envois,
                            conflits,
                            taille,
                            ajoutees: champ(&json, "ajoutees").unwrap_or(0),
                            rejeu: json.get("rejeu").and_then(serde_json::Value::as_bool).unwrap_or(false),
                        });
                    }
                    let suivante = taille.saturating_add(champ(&json, "en_attente")?);
                    if suivante > n {
                        return Err(ErreurOrigin::Incoherente(format!("Origin en avance ({suivante} > {n})")));
                    }
                    if suivante <= position {
                        // Morceau accepté sans progrès (attente purgée, rejeu) : compté comme reprise.
                        conflits += 1;
                    }
                    position = suivante;
                }
                409 => {
                    conflits += 1;
                    let attendu = json.as_ref().map_or(Err(ErreurOrigin::Incoherente("409 illisible".into())), |j| champ(j, "attendu"))?;
                    if attendu > n {
                        return Err(ErreurOrigin::Incoherente(format!("Origin attend {attendu} > {n} entrées locales")));
                    }
                    position = attendu;
                }
                401 | 403 => return Err(ErreurOrigin::Refusee { statut: reponse.statut, motif: motif(json.as_ref()) }),
                429 => {
                    debit += 1;
                    if debit > self.reglages.max_attentes_debit {
                        return Err(ErreurOrigin::Debit(debit));
                    }
                    (self.pause)(self.reglages.attente_debit);
                }
                statut => return Err(ErreurOrigin::Statut { statut, motif: motif(json.as_ref()) }),
            }
            if conflits > self.reglages.max_conflits {
                return Err(ErreurOrigin::Conflits(conflits));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Transport qui rejoue des réponses écrites d'avance.
    struct Script(Vec<ReponseHttp>, Vec<serde_json::Value>);

    impl Transport for Script {
        fn poster(&mut self, corps: &[u8]) -> Result<ReponseHttp, String> {
            self.1.push(serde_json::from_slice(corps).unwrap());
            if self.0.is_empty() {
                return Err("fin du script".into());
            }
            Ok(self.0.remove(0))
        }
    }

    fn rep(statut: u16, json: serde_json::Value) -> ReponseHttp {
        ReponseHttp { statut, corps: json.to_string().into_bytes() }
    }

    fn instantane(n: usize, l: usize) -> Instantane {
        Instantane { tete: vec![7; 300], taille: n as u64, racine: [0; 32], emise_a: 0, entrees: vec![vec![1; l]; n] }
    }

    fn client(script: Vec<ReponseHttp>) -> PublicationOrigin<Script> {
        let cog = CogId::new(&"k".repeat(26)).unwrap();
        PublicationOrigin::new(cog, &CleSignature::generer(), Script(script, Vec::new())).avec_horloge(|| 1_000, |_| {})
    }

    #[test]
    fn decoupage_borne_en_nombre_et_en_octets() {
        let c = client(Vec::new());
        assert_eq!(c.decouper(&instantane(5000, 10), 0).unwrap(), (MAX_ENTREES_SYNCHRO, false));
        assert_eq!(c.decouper(&instantane(5000, 10), 4096).unwrap(), (904, true));
        // Entrées de 64 Kio : ~ 85 Kio en base64, au plus 23 par corps de 2 Mio.
        let (k, tete) = c.decouper(&instantane(100, 64 * 1024), 0).unwrap();
        assert!(!tete && k < 25 && k > 20, "{k}");
        // Une entrée qui ne tient pas seule : refus.
        assert_eq!(c.decouper(&instantane(1, 2 << 20), 0), Err(ErreurOrigin::TropGrande(0)));
    }

    #[test]
    fn statuts_et_arret() {
        let mut c = client(vec![rep(401, serde_json::json!({"ok": false, "error": "non_authentifie"}))]);
        let e = c.publier(&instantane(3, 10)).unwrap_err();
        assert!(e.est_definitive());
        assert_eq!(c.transport().1.len(), 1, "aucun nouvel essai après 401");
        // 429 puis 409 puis succès.
        let mut c = client(vec![
            rep(429, serde_json::json!({"ok": false})),
            rep(409, serde_json::json!({"ok": false, "taille": 0, "attendu": 0})),
            rep(200, serde_json::json!({"ok": true, "taille": 3, "en_attente": 0, "ajoutees": 3, "rejeu": false})),
        ]);
        let b = c.publier(&instantane(3, 10)).unwrap();
        assert_eq!((b.envois, b.conflits, b.ajoutees), (3, 1, 3));
        let envoyes = &c.transport().1;
        assert_eq!((envoyes[0]["depuis"].as_u64(), envoyes[0]["entrees"].as_array().unwrap().len()), (Some(3), 0));
        assert_eq!((envoyes[2]["depuis"].as_u64(), envoyes[2]["entrees"].as_array().unwrap().len()), (Some(0), 3));
        assert_eq!(c.position(), Some(3));
        // Origin en avance sur le journal local : incohérent, position oubliée.
        let mut c = client(vec![rep(409, serde_json::json!({"taille": 9, "attendu": 9}))]);
        assert!(matches!(c.publier(&instantane(3, 10)), Err(ErreurOrigin::Incoherente(_))));
        assert_eq!(c.position(), None);
        // Conflits sans fin : borne.
        let mut c = client(vec![rep(409, serde_json::json!({"taille": 0, "attendu": 0})); 20]);
        assert!(matches!(c.publier(&instantane(3, 10)), Err(ErreurOrigin::Conflits(_))));
    }
}
