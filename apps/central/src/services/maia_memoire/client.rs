//! @id     central_maia_memoire_client
//! @do     appeler_routes_memoire_maia
//! @role   security
//! @layer  app
//! @human  central — client des routes mémoire de MAIA : visa + preuve de possession liée à la requête
//! @human  (défi du pont), ou requête signée en transition explicite ; chemins de l'utilisateur configuré seul.

//! Client des routes `/v1/memoire/{u}/…` du pont MAIA (I4, A5).
//!
//! # Configuration
//!
//! | Variable | Rôle |
//! |---|---|
//! | `CENTRAL_MAIA_URL` | pont MAIA (défaut `http://127.0.0.1:11435`) |
//! | `CENTRAL_MAIA_UTILISATEUR` | utilisateur de la mémoire (à défaut `MAIA_MEMOIRE_UTILISATEUR`) ; `[a-z0-9_-]`, 1 à 64 |
//! | `CENTRAL_MAIA_TITRES_DIR` | titres de l'appareil (voie visa, ci-dessous) |
//! | `CENTRAL_MAIA_TRANSITION=cles-epinglees` | transition : requêtes signées par des clés épinglées |
//! | `CENTRAL_MAIA_CLE_MEMOIRE`, `CENTRAL_MAIA_CLE_APPROBATION` | graines `0600` (transition seulement) |
//!
//! # Voie visa (par défaut)
//!
//! Dossier des titres, **relu à chaque requête** (le visa, ≤ 15 min, est
//! renouvelé hors de Central) :
//!
//! | Fichier | Contenu |
//! |---|---|
//! | `appareil.key` | graine Ed25519 de l'appareil (32 o, `0600`) |
//! | `passeport` | passeport scellé (binaire) |
//! | `visa` | visa scellé, audience `maia` (binaire) |
//! | `delegations` | valeur de l'en-tête `X-Miyu-Delegations` (texte) |
//! | `approbation/…` | mêmes fichiers pour l'appareil d'approbation TAMR (portée `maia:tamr:approuver`) |
//!
//! Pour chaque requête : `POST /v1/memoire/defi` (défi à usage unique), puis
//! `Authorization: MiyuVisa …`, `X-Miyu-PoP` (signature sur le défi, l'audience
//! `maia` et la liaison méthode / chemin / corps), `X-Miyu-Delegations`.

use std::path::{Path, PathBuf};

use miyauth::http::service::{charger_cle, entetes_signees};
use miyauth::http::{
    b64_decoder, formater_autorisation_visa, formater_pop, liaison_requete, signer_pop,
    ENTETE_DELEGATIONS, ENTETE_POP, TAILLE_MAX_TITRE_ENTETE,
};
use miyusceau::CleSignature;
use serde_json::Value;

/// Pont par défaut.
pub const URL_DEFAUT: &str = "http://127.0.0.1:11435";
/// Audience des visas mémoire.
pub const AUDIENCE: &str = "maia";
/// Valeur qui active la transition.
pub const VALEUR_TRANSITION: &str = "cles-epinglees";
/// Corps de réponse lu au plus (un export peut être volumineux).
pub const MAX_REPONSE: usize = 32 * 1024 * 1024;

/// Erreur du client (message affichable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurClient(pub String);

impl std::fmt::Display for ErreurClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Méthode HTTP utilisée par les routes mémoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Methode {
    /// POST.
    Post,
    /// DELETE.
    Delete,
}

impl Methode {
    /// Nom.
    pub fn as_str(self) -> &'static str {
        match self {
            Methode::Post => "POST",
            Methode::Delete => "DELETE",
        }
    }
}

/// Mode d'authentification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeAuth {
    /// Visa (dossier des titres).
    Visa(PathBuf),
    /// Transition : clé mémoire, clé d'approbation facultative.
    Transition {
        /// Graine de la clé mémoire.
        memoire: PathBuf,
        /// Graine de la clé d'approbation.
        approbation: Option<PathBuf>,
    },
}

/// Configuration du client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigMemoire {
    /// URL du pont (sans `/` final).
    pub url: String,
    /// Utilisateur de la mémoire.
    pub utilisateur: String,
    /// Authentification.
    pub auth: ModeAuth,
}

/// Utilisateur acceptable dans un chemin (aucune injection de segment).
pub fn utilisateur_valide(u: &str) -> bool {
    (1..=64).contains(&u.len())
        && u.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// Segment variable acceptable (identifiant de candidat ou d'intervention).
pub fn segment_valide(s: &str) -> bool {
    (1..=128).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

impl ConfigMemoire {
    /// Lecture de l'environnement (voir l'en-tête du module).
    pub fn depuis_env() -> Result<Self, ErreurClient> {
        let v = |n: &str| {
            std::env::var(n)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        };
        Self::depuis_valeurs(
            v("CENTRAL_MAIA_URL").as_deref(),
            v("CENTRAL_MAIA_UTILISATEUR")
                .or_else(|| v("MAIA_MEMOIRE_UTILISATEUR"))
                .as_deref(),
            v("CENTRAL_MAIA_TITRES_DIR").as_deref(),
            v("CENTRAL_MAIA_TRANSITION").as_deref(),
            v("CENTRAL_MAIA_CLE_MEMOIRE").as_deref(),
            v("CENTRAL_MAIA_CLE_APPROBATION").as_deref(),
        )
    }

    /// Variante pure.
    pub fn depuis_valeurs(
        url: Option<&str>,
        utilisateur: Option<&str>,
        titres: Option<&str>,
        transition: Option<&str>,
        cle_memoire: Option<&str>,
        cle_approbation: Option<&str>,
    ) -> Result<Self, ErreurClient> {
        let utilisateur = utilisateur.ok_or_else(|| {
            ErreurClient(
                "Aucun utilisateur de mémoire configuré (CENTRAL_MAIA_UTILISATEUR).".into(),
            )
        })?;
        if !utilisateur_valide(utilisateur) {
            return Err(ErreurClient("Utilisateur de mémoire invalide.".into()));
        }
        let url = url.unwrap_or(URL_DEFAUT).trim_end_matches('/').to_string();
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err(ErreurClient("URL du pont MAIA invalide.".into()));
        }
        let auth = match (titres, transition) {
            (Some(d), _) => ModeAuth::Visa(PathBuf::from(d)),
            (None, Some(VALEUR_TRANSITION)) => ModeAuth::Transition {
                memoire: PathBuf::from(cle_memoire.ok_or_else(|| {
                    ErreurClient("Transition : CENTRAL_MAIA_CLE_MEMOIRE manquante.".into())
                })?),
                approbation: cle_approbation.map(PathBuf::from),
            },
            _ => return Err(ErreurClient(
                "Aucun titre configuré : indiquez le dossier du visa (CENTRAL_MAIA_TITRES_DIR)."
                    .into(),
            )),
        };
        Ok(Self {
            url,
            utilisateur: utilisateur.to_string(),
            auth,
        })
    }

    /// Chemin d'une route de l'utilisateur configuré.
    pub fn chemin(&self, suffixe: &str) -> String {
        format!("/v1/memoire/{}/{suffixe}", self.utilisateur)
    }
}

/// Défi reçu du pont.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefiRecu {
    /// Identifiant.
    pub id: [u8; 16],
    /// Défi.
    pub defi: [u8; 32],
}

impl DefiRecu {
    /// Décode `{"id","defi","audience"}` ; audience différente ⇒ refus.
    pub fn depuis(v: &Value) -> Option<Self> {
        if v["audience"] != AUDIENCE {
            return None;
        }
        Some(Self {
            id: b64_decoder(v["id"].as_str()?)?.try_into().ok()?,
            defi: b64_decoder(v["defi"].as_str()?)?.try_into().ok()?,
        })
    }
}

/// Titres d'un appareil (voie visa).
pub struct TitresVisa {
    appareil: CleSignature,
    passeport: Vec<u8>,
    visa: Vec<u8>,
    delegations: Option<String>,
}

impl std::fmt::Debug for TitresVisa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TitresVisa").finish_non_exhaustive()
    }
}

fn lire_titre(chemin: &Path) -> Result<Vec<u8>, ErreurClient> {
    let o =
        std::fs::read(chemin).map_err(|e| ErreurClient(format!("{} : {e}", chemin.display())))?;
    if o.is_empty() || o.len() > TAILLE_MAX_TITRE_ENTETE {
        return Err(ErreurClient(format!(
            "{} : taille invalide",
            chemin.display()
        )));
    }
    Ok(o)
}

impl TitresVisa {
    /// Titres `(appareil, passeport, visa, délégations)` en mémoire (essais).
    #[cfg(test)]
    pub fn new(
        appareil: CleSignature,
        passeport: Vec<u8>,
        visa: Vec<u8>,
        delegations: Option<String>,
    ) -> Self {
        Self {
            appareil,
            passeport,
            visa,
            delegations,
        }
    }

    /// Lit le dossier des titres (voir l'en-tête du module).
    pub fn charger(dossier: &Path) -> Result<Self, ErreurClient> {
        let appareil = charger_cle(&dossier.join("appareil.key"))
            .map_err(|e| ErreurClient(format!("clé d'appareil : {e}")))?;
        let delegations = match std::fs::read_to_string(dossier.join("delegations")) {
            Ok(d) if !d.trim().is_empty() => Some(d.trim().to_string()),
            Ok(_) => None,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(ErreurClient(format!("délégations : {e}"))),
        };
        Ok(Self {
            appareil,
            passeport: lire_titre(&dossier.join("passeport"))?,
            visa: lire_titre(&dossier.join("visa"))?,
            delegations,
        })
    }

    /// En-têtes d'une requête prouvée sur `defi` (chemin sans chaîne de requête).
    pub fn entetes(
        &self,
        defi: &DefiRecu,
        methode: Methode,
        chemin: &str,
        corps: &[u8],
    ) -> Vec<(&'static str, String)> {
        let sig = signer_pop(
            &self.appareil,
            &defi.defi,
            AUDIENCE,
            &liaison_requete(methode.as_str(), chemin, corps),
        );
        let mut v = vec![
            (
                "authorization",
                formater_autorisation_visa(&self.visa, &self.passeport),
            ),
            (ENTETE_POP, formater_pop(&defi.id, &sig)),
        ];
        if let Some(d) = &self.delegations {
            v.push((ENTETE_DELEGATIONS, d.clone()));
        }
        v
    }
}

/// Réponse du pont : statut et corps JSON (`Null` si vide ou illisible).
#[derive(Debug, Clone, PartialEq)]
pub struct Reponse {
    /// Statut HTTP.
    pub statut: u16,
    /// Corps.
    pub corps: Value,
}

/// Client des routes mémoire.
#[derive(Debug, Clone)]
pub struct ClientMemoire {
    http: reqwest::Client,
    config: ConfigMemoire,
}

impl ClientMemoire {
    /// Client pour `config`.
    pub fn new(config: ConfigMemoire) -> Result<Self, ErreurClient> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| ErreurClient(format!("client HTTP : {e}")))?;
        Ok(Self { http, config })
    }

    /// Configuration.
    pub fn config(&self) -> &ConfigMemoire {
        &self.config
    }

    /// Vrai si une identité d'approbation est configurée.
    pub fn approbation_configuree(&self) -> bool {
        match &self.config.auth {
            ModeAuth::Visa(d) => d.join("approbation").join("visa").is_file(),
            ModeAuth::Transition { approbation, .. } => approbation.is_some(),
        }
    }

    async fn lire(r: reqwest::Response) -> Result<(Reponse, Option<String>), ErreurClient> {
        let statut = r.status().as_u16();
        let instance = r
            .headers()
            .get("x-maia-instance")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        if r.content_length().is_some_and(|n| n > MAX_REPONSE as u64) {
            return Err(ErreurClient("réponse trop volumineuse".into()));
        }
        let octets = r
            .bytes()
            .await
            .map_err(|e| ErreurClient(format!("réponse : {e}")))?;
        if octets.len() > MAX_REPONSE {
            return Err(ErreurClient("réponse trop volumineuse".into()));
        }
        Ok((
            Reponse {
                statut,
                corps: serde_json::from_slice(&octets).unwrap_or(Value::Null),
            },
            instance,
        ))
    }

    async fn post_brut(&self, chemin: &str) -> Result<(Reponse, Option<String>), ErreurClient> {
        let r = self
            .http
            .post(format!("{}{chemin}", self.config.url))
            .send()
            .await
            .map_err(|e| ErreurClient(format!("Pont MAIA injoignable : {e}")))?;
        Self::lire(r).await
    }

    async fn defi(&self) -> Result<DefiRecu, ErreurClient> {
        let (r, _) = self.post_brut("/v1/memoire/defi").await?;
        if r.statut != 200 {
            return Err(ErreurClient(super::modele::message_refus(
                r.statut, &r.corps,
            )));
        }
        DefiRecu::depuis(&r.corps).ok_or_else(|| ErreurClient("défi illisible".into()))
    }

    /// Envoie une requête authentifiée sur la route `suffixe` de l'utilisateur
    /// configuré ; `approbation` : identité d'approbation TAMR.
    pub async fn envoyer(
        &self,
        methode: Methode,
        suffixe: &str,
        corps: Option<&Value>,
        approbation: bool,
    ) -> Result<Reponse, ErreurClient> {
        let chemin = self.config.chemin(suffixe);
        let octets = match corps {
            Some(c) => serde_json::to_vec(c).map_err(|e| ErreurClient(e.to_string()))?,
            None => Vec::new(),
        };
        let entetes: Vec<(&'static str, String)> = match &self.config.auth {
            ModeAuth::Visa(dossier) => {
                let dossier = if approbation {
                    dossier.join("approbation")
                } else {
                    dossier.clone()
                };
                let titres = TitresVisa::charger(&dossier)?;
                let defi = self.defi().await?;
                titres.entetes(&defi, methode, &chemin, &octets)
            }
            ModeAuth::Transition {
                memoire,
                approbation: cle_approbation,
            } => {
                let (cle, usage) = if approbation {
                    let c = cle_approbation.as_ref().ok_or_else(|| {
                        ErreurClient("Aucune clé d'approbation configurée.".into())
                    })?;
                    (c, "maia/tamr-approbation")
                } else {
                    (memoire, "maia/memoire")
                };
                let cle = charger_cle(cle).map_err(|e| ErreurClient(format!("clé : {e}")))?;
                let (_, instance) = self.post_brut(&self.config.chemin("consulter")).await?;
                let instance =
                    instance.ok_or_else(|| ErreurClient("identifiant d'instance absent".into()))?;
                let usage = format!("{usage}/{instance}/{}", self.config.utilisateur);
                entetes_signees(&cle, &usage, methode.as_str(), &chemin, &octets)
                    .into_iter()
                    .collect()
            }
        };
        let url = format!("{}{chemin}", self.config.url);
        let mut rb = match methode {
            Methode::Post => self.http.post(url),
            Methode::Delete => self.http.delete(url),
        };
        for (n, v) in entetes {
            rb = rb.header(n, v);
        }
        if corps.is_some() {
            rb = rb.header(reqwest::header::CONTENT_TYPE, "application/json");
        }
        let r = rb
            .body(octets)
            .send()
            .await
            .map_err(|e| ErreurClient(format!("Pont MAIA injoignable : {e}")))?;
        Ok(Self::lire(r).await?.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyauth::http::{analyser_autorisation_visa, analyser_delegations, analyser_pop};

    #[test]
    fn configuration() {
        let c =
            ConfigMemoire::depuis_valeurs(None, Some("alice"), Some("/titres"), None, None, None)
                .expect("config");
        assert_eq!(c.url, URL_DEFAUT);
        assert_eq!(c.auth, ModeAuth::Visa("/titres".into()));
        assert_eq!(c.chemin("consulter"), "/v1/memoire/alice/consulter");
        // Transition seulement si demandée explicitement, avec une clé.
        assert!(
            ConfigMemoire::depuis_valeurs(None, Some("alice"), None, None, Some("/k"), None)
                .is_err()
        );
        assert!(ConfigMemoire::depuis_valeurs(
            None,
            Some("alice"),
            None,
            Some("oui"),
            Some("/k"),
            None
        )
        .is_err());
        assert!(ConfigMemoire::depuis_valeurs(
            None,
            Some("alice"),
            None,
            Some(VALEUR_TRANSITION),
            None,
            None
        )
        .is_err());
        let t = ConfigMemoire::depuis_valeurs(
            Some("http://h:1/"),
            Some("alice"),
            None,
            Some(VALEUR_TRANSITION),
            Some("/k"),
            None,
        )
        .expect("transition");
        assert_eq!(t.url, "http://h:1");
        assert!(matches!(
            t.auth,
            ModeAuth::Transition {
                approbation: None,
                ..
            }
        ));
        // Aucune autre base que celle de l'utilisateur configuré : nom strict.
        for u in ["", "../bob", "alice/x", "Alice", "a b", &"a".repeat(65)] {
            assert!(
                ConfigMemoire::depuis_valeurs(None, Some(u), Some("/t"), None, None, None).is_err(),
                "{u}"
            );
        }
        assert!(ConfigMemoire::depuis_valeurs(None, None, Some("/t"), None, None, None).is_err());
        assert!(ConfigMemoire::depuis_valeurs(
            Some("file:///x"),
            Some("alice"),
            Some("/t"),
            None,
            None,
            None
        )
        .is_err());
        assert!(segment_valide("8d4f7c1e-0000-4000-8000-000000000001"));
        assert!(!segment_valide("../x") && !segment_valide("a/b") && !segment_valide(""));
    }

    #[test]
    fn defi_decode_et_audience() {
        let v = serde_json::json!({"id": miyauth::http::b64_encoder(&[1; 16]), "defi": miyauth::http::b64_encoder(&[2; 32]), "audience": "maia", "expire_a": 1});
        assert_eq!(
            DefiRecu::depuis(&v),
            Some(DefiRecu {
                id: [1; 16],
                defi: [2; 32]
            })
        );
        let mut autre = v.clone();
        autre["audience"] = "autre".into();
        assert_eq!(DefiRecu::depuis(&autre), None);
        let mut court = v;
        court["id"] = miyauth::http::b64_encoder(&[1; 8]).into();
        assert_eq!(DefiRecu::depuis(&court), None);
    }

    #[test]
    fn entetes_visa_lies_a_la_requete() {
        let t = TitresVisa::new(
            CleSignature::depuis_graine(&[9; 32]),
            b"passeport".to_vec(),
            b"visa".to_vec(),
            Some("AAAA".into()),
        );
        let d = DefiRecu {
            id: [1; 16],
            defi: [2; 32],
        };
        let e = t.entetes(&d, Methode::Post, "/v1/memoire/alice/consulter", b"");
        let get = |n: &str| {
            e.iter()
                .find(|(k, _)| *k == n)
                .map(|(_, v)| v.clone())
                .expect(n)
        };
        assert_eq!(
            analyser_autorisation_visa(&get("authorization")),
            Ok((b"visa".to_vec(), b"passeport".to_vec()))
        );
        let pop = analyser_pop(&get(ENTETE_POP)).expect("pop");
        assert_eq!(pop.id_defi, [1; 16]);
        assert_eq!(
            analyser_delegations(&get(ENTETE_DELEGATIONS))
                .expect("délégations")
                .len(),
            1
        );
        // Liaison : autre chemin, autre méthode ou autre corps ⇒ autre preuve.
        let sig = |m: Methode, c: &str, b: &[u8]| {
            let e = t.entetes(&d, m, c, b);
            e.iter()
                .find(|(k, _)| *k == ENTETE_POP)
                .map(|(_, v)| v.clone())
        };
        let base = sig(Methode::Post, "/v1/memoire/alice/consulter", b"");
        assert_ne!(base, sig(Methode::Post, "/v1/memoire/alice/exporter", b""));
        assert_ne!(
            base,
            sig(Methode::Delete, "/v1/memoire/alice/consulter", b"")
        );
        assert_ne!(
            base,
            sig(Methode::Post, "/v1/memoire/alice/consulter", b"{}")
        );
        // Preuve = signature de l'appareil sur (défi, « maia », liaison de cette requête).
        let attendue = signer_pop(
            &CleSignature::depuis_graine(&[9; 32]),
            &d.defi,
            AUDIENCE,
            &liaison_requete("POST", "/v1/memoire/alice/consulter", b""),
        );
        assert_eq!(pop.signature, attendue);
    }

    #[test]
    fn titres_lus_depuis_le_dossier() {
        use std::os::unix::fs::PermissionsExt;
        let d = std::env::temp_dir().join(format!("central-maia-titres-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).expect("dossier");
        // Absents : refus.
        assert!(TitresVisa::charger(&d).is_err());
        std::fs::write(d.join("appareil.key"), [5u8; 32]).expect("clé");
        std::fs::set_permissions(
            d.join("appareil.key"),
            std::fs::Permissions::from_mode(0o600),
        )
        .expect("droits");
        std::fs::write(d.join("passeport"), b"p").expect("p");
        std::fs::write(d.join("visa"), b"v").expect("v");
        let t = TitresVisa::charger(&d).expect("titres");
        assert!(t.delegations.is_none());
        // Clé aux droits ouverts : refus.
        std::fs::set_permissions(
            d.join("appareil.key"),
            std::fs::Permissions::from_mode(0o644),
        )
        .expect("droits");
        assert!(TitresVisa::charger(&d).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}
