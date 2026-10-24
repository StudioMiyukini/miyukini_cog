//! @id     miyukini_cog_bridge_identite
//! @do     authentifier_appareil_par_passeport
//! @role   security
//! @layer  core
//! @human  miyukini-cog-bridge — client passeport d'appareil + visa + preuve de possession (sortie du lot 2).

//! Authentification de l'appareil auprès de CentralRemote, sans secret partagé.
//!
//! Protocole décrit dans `central_remote::server` (module) et
//! `central_remote_protocol::identite` :
//!
//! 1. passeport : `POST /api/bridge/passeport`, requête **signée** par la clé
//!    d'identité de l'appareil (celle épinglée à l'appairage v2) ;
//! 2. défi : `POST /api/bridge/defi` (usage unique, 60 s) ;
//! 3. visa : `POST /api/bridge/visa` avec `MiyuPasseport` + `X-Miyu-PoP` ;
//! 4. chaque requête : `MiyuVisa` + une **nouvelle** preuve `X-Miyu-PoP` liée à
//!    la méthode, au chemin et au corps ([`ClientAppareil::entetes`]).
//!
//! Le client renouvelle le visa (≤ 15 min) et le passeport (24 h) avant leur
//! expiration ; la clé privée ne quitte jamais l'appareil.

use central_remote_protocol::identite::{
    DefiResponse, PasseportResponse, ValidateResponse, VisaResponse, AUDIENCE, DEFI_ENDPOINT,
    PASSEPORT_ENDPOINT, USAGE_PASSEPORT, VALIDATE_ENDPOINT, VISA_ENDPOINT, WS_ENDPOINT,
};
use miyauth::http::{
    b64_decoder, formater_autorisation_passeport, formater_autorisation_visa, formater_pop,
    liaison_requete, signer_pop, signer_requete, ENTETE_POP,
};
use miyusceau::CleSignature;
use tokio::sync::Mutex;

use crate::e2e::E2eIdentity;
use crate::{BridgeError, BridgeResult};

/// Marge de renouvellement avant expiration d'un titre (secondes).
pub const MARGE_RENOUVELLEMENT_SECS: u64 = 60;
/// Taille maximale d'une réponse de titre (C-13).
const TAILLE_MAX_REPONSE: usize = 16 * 1024;

fn maintenant() -> u64 {
    miyauth::http::service::maintenant_unix()
}

/// Titres détenus par l'appareil.
#[derive(Debug, Default, Clone)]
struct Titres {
    passeport: Option<(Vec<u8>, u64)>,
    visa: Option<(Vec<u8>, u64)>,
    dernier_visa: Option<VisaResponse>,
}

/// En-têtes à poser sur une requête authentifiée : `(nom, valeur)`.
pub type EntetesAuth = [(&'static str, String); 2];

/// Client d'authentification d'un appareil appairé.
pub struct ClientAppareil {
    base_url: String,
    cle: CleSignature,
    http: reqwest::Client,
    titres: Mutex<Titres>,
}

impl std::fmt::Debug for ClientAppareil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientAppareil").field("base_url", &self.base_url).finish_non_exhaustive()
    }
}

fn refus(contexte: &str, statut: reqwest::StatusCode) -> BridgeError {
    match statut.as_u16() {
        401 | 403 => BridgeError::DeviceNotWhitelisted,
        429 | 503 => BridgeError::AuthFailed(format!("{contexte} : COG saturé, réessayer")),
        _ => BridgeError::AuthFailed(format!("{contexte} : HTTP {statut}")),
    }
}

fn decoder_titre(texte: &str) -> BridgeResult<Vec<u8>> {
    b64_decoder(texte).ok_or_else(|| BridgeError::AuthFailed("titre mal encodé".into()))
}

impl ClientAppareil {
    /// Client pour le COG joignable à `base_url` (`http://hôte:port`), avec
    /// l'identité Ed25519 de l'appareil (clé épinglée à l'appairage).
    pub fn new(base_url: impl Into<String>, appareil: &E2eIdentity) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            cle: CleSignature::depuis_graine(&appareil.secret_bytes()),
            http: reqwest::Client::new(),
            titres: Mutex::new(Titres::default()),
        }
    }

    async fn lire<T: for<'de> serde::Deserialize<'de>>(contexte: &str, resp: reqwest::Response) -> BridgeResult<T> {
        if !resp.status().is_success() {
            return Err(refus(contexte, resp.status()));
        }
        let octets = crate::bridge::read_bounded(resp, TAILLE_MAX_REPONSE).await?;
        serde_json::from_slice(&octets).map_err(|e| BridgeError::AuthFailed(format!("{contexte} : {e}")))
    }

    async fn post(&self, chemin: &str, entetes: &[(&'static str, String)]) -> BridgeResult<reqwest::Response> {
        let mut req = self
            .http
            .post(format!("{}{chemin}", self.base_url))
            .timeout(std::time::Duration::from_secs(10));
        for (nom, valeur) in entetes {
            req = req.header(*nom, valeur);
        }
        req.send().await.map_err(|e| BridgeError::NetworkError(e.to_string()))
    }

    /// Demande un passeport (requête signée par la clé d'appareil, corps vide).
    async fn demander_passeport(&self) -> BridgeResult<(Vec<u8>, u64)> {
        let entetes = signer_requete(&self.cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant()).en_entetes();
        let resp = self.post(PASSEPORT_ENDPOINT, &entetes).await?;
        let r: PasseportResponse = Self::lire("passeport", resp).await?;
        Ok((decoder_titre(&r.passeport)?, r.expire_a))
    }

    /// Obtient un défi neuf et signe la preuve de possession liée à la requête.
    async fn preuve(&self, methode: &str, chemin: &str, corps: &[u8]) -> BridgeResult<String> {
        let resp = self.post(DEFI_ENDPOINT, &[]).await?;
        let d: DefiResponse = Self::lire("défi", resp).await?;
        if d.audience != AUDIENCE {
            return Err(BridgeError::AuthFailed("défi d'une autre audience".into()));
        }
        let id: [u8; 16] = decoder_titre(&d.defi_id)?
            .try_into()
            .map_err(|_| BridgeError::AuthFailed("défi mal formé".into()))?;
        let defi: [u8; 32] = decoder_titre(&d.defi)?
            .try_into()
            .map_err(|_| BridgeError::AuthFailed("défi mal formé".into()))?;
        let signature = signer_pop(&self.cle, &defi, AUDIENCE, &liaison_requete(methode, chemin, corps));
        Ok(formater_pop(&id, &signature))
    }

    /// Passeport valide (renouvelé si absent ou proche de l'expiration).
    async fn passeport_valide(&self, titres: &mut Titres) -> BridgeResult<Vec<u8>> {
        match &titres.passeport {
            Some((p, exp)) if *exp > maintenant() + MARGE_RENOUVELLEMENT_SECS => Ok(p.clone()),
            _ => {
                let (p, exp) = self.demander_passeport().await?;
                titres.passeport = Some((p.clone(), exp));
                titres.visa = None;
                Ok(p)
            }
        }
    }

    /// Obtient (ou renouvelle) le visa ; rend la réponse du COG.
    pub async fn ouvrir_session(&self) -> BridgeResult<VisaResponse> {
        let mut titres = self.titres.lock().await;
        let passeport = self.passeport_valide(&mut titres).await?;
        let pop = self.preuve("POST", VISA_ENDPOINT, b"").await?;
        let entetes = [
            ("authorization", formater_autorisation_passeport(&passeport)),
            (ENTETE_POP, pop),
        ];
        let resp = self.post(VISA_ENDPOINT, &entetes).await?;
        let v: VisaResponse = Self::lire("visa", resp).await?;
        titres.visa = Some((decoder_titre(&v.visa)?, v.expire_a));
        titres.dernier_visa = Some(v.clone());
        Ok(v)
    }

    /// En-têtes `Authorization: MiyuVisa …` + `X-Miyu-PoP` (preuve neuve) pour
    /// la requête `methode chemin` de corps `corps`. Renouvelle les titres au besoin.
    pub async fn entetes(&self, methode: &str, chemin: &str, corps: &[u8]) -> BridgeResult<EntetesAuth> {
        let a_renouveler = {
            let titres = self.titres.lock().await;
            !matches!(&titres.visa, Some((_, exp)) if *exp > maintenant() + MARGE_RENOUVELLEMENT_SECS)
        };
        if a_renouveler {
            self.ouvrir_session().await?;
        }
        let (visa, passeport) = {
            let titres = self.titres.lock().await;
            match (&titres.visa, &titres.passeport) {
                (Some((v, _)), Some((p, _))) => (v.clone(), p.clone()),
                _ => return Err(BridgeError::Disconnected),
            }
        };
        let pop = self.preuve(methode, chemin, corps).await?;
        Ok([("authorization", formater_autorisation_visa(&visa, &passeport)), (ENTETE_POP, pop)])
    }

    /// En-têtes d'ouverture du WebSocket (`GET /ws`, corps vide) : à poser sur
    /// la requête d'upgrade, juste après leur obtention (défi de 60 s).
    pub async fn entetes_ws(&self) -> BridgeResult<EntetesAuth> {
        self.entetes("GET", WS_ENDPOINT, b"").await
    }

    /// Vérifie l'accès auprès du COG (`POST /api/bridge/validate`).
    pub async fn valider(&self) -> BridgeResult<ValidateResponse> {
        let entetes = self.entetes("POST", VALIDATE_ENDPOINT, b"").await?;
        let resp = self.post(VALIDATE_ENDPOINT, &entetes).await?;
        Self::lire("validation", resp).await
    }

    /// Dernière réponse de visa (nom affiché, identité opaque), s'il y en a une.
    pub async fn session(&self) -> Option<VisaResponse> {
        self.titres.lock().await.dernier_visa.clone()
    }

    /// Oublie les titres détenus (déconnexion).
    pub async fn oublier(&self) {
        *self.titres.lock().await = Titres::default();
    }
}
