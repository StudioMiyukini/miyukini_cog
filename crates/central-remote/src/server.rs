//! Serveur HTTP + WebSocket embarqué pour CentralRemote.
//!
//! @id     central_remote_server
//! @do run_embedded_remote_server
//@human ! @role   infra
//! @layer toolkit
//! @human  Serveur Axum lancé par Central quand le remote est activé.
//! Authentification des appareils par passeport + visa + preuve de possession
//! (aucun secret partagé, sortie du lot 2).
//!
//! # Protocole client (Central Mobile ou tout client natif)
//!
//! Prérequis : appareil appairé (appairage v2 ci-dessous) ; l'appareil détient
//! sa clé d'identité Ed25519 `Kd` (la même que pour l'E2E), épinglée par le COG.
//! Encodages : base64 URL **sans remplissage** (RFC 4648 §5), décodage strict.
//! Format exact des en-têtes : `miyauth::http`.
//!
//! 1. **Passeport** — `POST /api/bridge/passeport`, corps vide, requête signée
//!    par `Kd` (usage `central-remote/passeport`) :
//!    ```text
//!    X-Miyu-Signataire: b64(Kd publique)     X-Miyu-Horodatage: <secondes Unix>
//!    X-Miyu-Nonce:      b64(16 o aléatoires)  X-Miyu-Signature: b64(Ed25519(Kd, m))
//!    m = "miyukini/requete-signee/v1" || 0 || lp("central-remote/passeport") || lp("POST")
//!        || lp("/api/bridge/passeport") || horodatage (u64 BE) || lp(nonce) || SHA-256(corps)
//!    ```
//!    (`lp` = longueur u16 BE + octets ; fenêtre ± 300 s ; nonce à usage unique.)
//!    Réponse `{passeport, expire_a, cog_id}` : passeport scellé (24 h) émis par
//!    la clé d'émission du COG pour `Kd`. Refus : 401 (clé non appairée ou
//!    révoquée, signature, horodatage, rejeu).
//! 2. **Défi** — `POST /api/bridge/defi` → `{defi_id, defi, audience:
//!    "central-remote", expire_a, cog_id}` (usage unique, 60 s).
//! 3. **Visa** — `POST /api/bridge/visa`, corps vide :
//!    ```text
//!    Authorization: MiyuPasseport b64(passeport)
//!    X-Miyu-PoP:    <defi_id>.b64(Ed25519(Kd, "miyukini/pop/v1" || defi || "central-remote" || liaison))
//!    liaison = SHA-256("miyukini/http-liaison/v1" || 0 || méthode || 0 || chemin || 0 || SHA-256(corps))
//!    ```
//!    → `{visa, expire_a, role, identite, display_name}` ; visa ≤ 15 min,
//!    audience `central-remote`.
//! 4. **Chaque requête** — `Authorization: MiyuVisa b64(visa).b64(passeport)` et
//!    une **nouvelle** preuve `X-Miyu-PoP` (nouveau défi, liaison de cette
//!    requête) :
//!    - `POST /api/bridge/validate` → `{status: "ok", identite, role, expire_a,
//!      display_name}` (remplace la validation par `device_id` au porteur) ;
//!    - `GET /ws` : vérifié **avant** l'upgrade (liaison de `GET /ws`, corps
//!      vide). Toute chaîne de requête est refusée (400) : `?token=` n'existe
//!      plus. La session WebSocket est fermée à l'expiration du visa et dès la
//!      révocation de l'appareil (revérification toutes les 5 s et à chaque
//!      commande).
//!
//! Refus : 401 (titre, preuve, défi expiré ou rejoué, révocation), 503 si le
//! service est saturé (défis ou nonces), 429 en cas de débit excessif.
//!
//! # Points retirés (inventaire des secrets partagés #1 à #5)
//!
//! - `POST /api/bridge/validate` avec `device_id` au porteur → visa + PoP.
//! - `GET /api/bridge/validate-token`, `POST /api/bridge/generate-token` (jeton
//!   API de 30 jours) : supprimés (404).
//! - `GET /ws` avec jeton HMAC (`Bearer` ou `?token=`) → visa + PoP.
//! - `POST /auth` (courriel + mot de passe → session) : supprimé (404). La doc
//!   gelée (§5) fait du passeport l'identité personne + appareil ; elle n'exige
//!   aucune connexion par mot de passe dans le canal E2E. Le titulaire est le
//!   profil Central lié à l'appareil à l'approbation.
//! - Routes d'administration `/api/bridge/{devices,approve,reject,generate-token}`
//!   (secret par processus + loopback) : supprimées. Central les appelait déjà
//!   **dans le processus** ([`RemoteServerHandle::pairing`]) ; aucune surface HTTP
//!   d'administration ne subsiste.
//!
//! ## Chiffrement E2E mobile ↔ COG (R-023, fail-closed C-12)
//!
//! `POST /api/bridge/e2e-handshake` : l'appareil envoie son offre v2 signée ; le
//! COG retrouve la clé d'identité épinglée dans la liste blanche (appareil
//! inconnu = 403), vérifie l'offre (`PendingHandshake::complete`) et répond par
//! sa propre offre v2.
//!
//! ## Appairage v2 : engagement puis révélation (R-026 à R-028)
//!
//! - `POST /api/bridge/pair` : engagement de l'appareil → clé + aléa du COG.
//! - `POST /api/bridge/pair/reveal` : révélation de `Nd`, vérifiée.
//!   Engagement et révélation sont signés par la clé de l'appareil (preuve de
//!   possession) ; requête non signée = refus explicite, sans repli.
//! - `POST /api/bridge/pair/status` : suivi de la demande par l'appareil.
//! - Approbation sur Central (code SAS affiché) via [`PairingManager`] ; la clé
//!   épinglée devient la clé des passeports de l'appareil.
//! - Liste blanche persistée à côté de l'identité E2E (R-024).

use crate::bridge::RemoteBridge;
use crate::e2e::{load_or_create_cog_identity, E2eSessionStore, DEFAULT_IDENTITY_FILE};
use crate::identite::{texte_role, AccesAppareil, AutoriteAppareils, FICHIER_CLE_EMISSION_DEFAUT};
use crate::paired_store::DEFAULT_PAIRED_DEVICES_FILE;
use crate::pairing::{mask_device_id, PairingConfig, PairingError, PairingManager, DEFAULT_PENDING_TTL};
use crate::protocol::identite::{
    DefiResponse, PasseportResponse, ValidateResponse, VisaResponse, DEFI_ENDPOINT,
    PASSEPORT_ENDPOINT, VALIDATE_ENDPOINT, VISA_ENDPOINT, WS_ENDPOINT,
};
use crate::protocol::pairing::{
    check_pairing_version, PairCommitRequest, PairCommitResponse, PairRevealRequest,
    PairRevealResponse, PairStatusRequest, PairStatusResponse, PairingStatus,
    PAIRING_PROTOCOL_VERSION, PAIR_ENDPOINT, PAIR_REVEAL_ENDPOINT, PAIR_STATUS_ENDPOINT,
    STATUS_INVALID_SIGNATURE, STATUS_MISSING_SIGNATURE, STATUS_SOURCE_MISMATCH,
    STATUS_UNSUPPORTED_VERSION,
};
use crate::protocol::{RemoteCommand, RemoteEvent};

use axum::body::Bytes;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::rejection::WebSocketUpgradeRejection;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{SinkExt, StreamExt};
use miyauth::http::service::maintenant_unix;
use miyauth::http::{
    b64_encoder, EntetesSignees, RefusHttp, RequeteHttp, ENTETE_HORODATAGE, ENTETE_NONCE,
    ENTETE_POP, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE, ENTETE_DELEGATIONS, formater_delegations,
};
use miyukini_central::auth::CentralAuthDb;
use miyukini_cog_bridge::e2e::MAX_HANDSHAKE_JSON_LEN;
use miyukini_cog_bridge::{
    identity_fingerprint, parse_commitment_hex, parse_identity_key_hex,
    parse_pairing_signature_hex, E2eHandshake, E2eIdentity, E2eRole, PairingNonce,
    PendingHandshake, PAIRING_SIGNATURE_SIZE,
};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

/// Fenêtre de limitation de débit des routes d'appairage / handshake E2E /
/// passeport / visa / validation.
const BRIDGE_RATE_WINDOW: Duration = Duration::from_secs(60);
/// Nombre maximal d'appels appairage + handshake + titres par IP dans la fenêtre.
const BRIDGE_RATE_MAX: usize = 30;
/// Nombre maximal d'appels de suivi d'appairage par IP et par minute
/// (l'appareil interroge toutes les 3 s pendant l'attente d'approbation).
const PAIR_STATUS_RATE_MAX: usize = 60;
/// Nombre maximal de défis par IP et par minute (un défi par requête authentifiée).
const DEFI_RATE_MAX: usize = 60;
/// Taille maximale du corps de /api/bridge/pair (C-13).
const PAIR_MAX_BODY: usize = 4 * 1024;
/// Taille maximale du corps des routes de titres (corps vide attendu, C-13).
const TITRE_MAX_BODY: usize = 1024;
/// Intervalle de revérification de l'accès pendant une session WS.
const WS_RECHECK_INTERVAL: Duration = Duration::from_secs(5);

/// Configuration du serveur CentralRemote.
#[derive(Debug, Clone)]
pub struct RemoteServerConfig {
    /// Adresse d'écoute (défaut : 127.0.0.1:8091).
    /// Le bind LAN (ex. `0.0.0.0:8091`) doit être demandé explicitement.
    pub listen_addr: String,
    /// Chemin vers la base auth Central (profils : nom affiché des titulaires).
    pub auth_db_path: String,
    /// Origines CORS autorisées (vide par défaut = aucun CORS cross-origin).
    /// Le client mobile natif n'en a pas besoin ; utile seulement pour un
    /// client navigateur (ex. portail web) explicitement déclaré.
    pub cors_allowed_origins: Vec<String>,
    /// Fichier de l'identité E2E longue durée du COG (`0600`). Défaut :
    /// `central-remote-e2e-identity.key` dans le dossier de la base auth.
    pub e2e_identity_path: Option<String>,
    /// Fichier de la liste blanche des appareils appairés (`0600`). Défaut :
    /// `central-remote-paired-devices.json` dans le dossier de l'identité E2E.
    pub paired_devices_path: Option<String>,
    /// Durée de vie d'une demande d'appairage en attente (secondes ; 0 = 10 min).
    pub pairing_pending_ttl_secs: u64,
    /// Révélation et relance d'appairage acceptées seulement depuis l'adresse
    /// source de l'engagement (**vrai par défaut**). Derrière un NAT les
    /// appareils partagent une adresse (liaison sans effet, signatures
    /// suffisantes) ; ne désactiver que si l'adresse source change entre deux
    /// requêtes d'un même appareil.
    pub pairing_require_same_source: bool,
    /// Graine de la clé d'émission du COG (32 octets bruts, `0600`), qui scelle
    /// passeports et visas. Défaut : `central-remote-emission.key` dans le
    /// dossier de l'identité E2E (créée une fois si absente). Avec une racine
    /// COG : graine exportée par `miyuracine` (voir [`crate::identite`]).
    pub cle_emission_path: Option<String>,
    /// `cog_id` du COG (celui de la racine `miyuracine`). Absent : dérivé de la
    /// clé d'émission.
    pub cog_id: Option<String>,
    /// Dossier de la chaîne racine → clés d'usage (A-024 ; voir
    /// [`crate::identite`]) : racine connue, clé `visas`, délégations, **mode
    /// strict**. Défaut : variable d'environnement `CENTRAL_REMOTE_CHAINE_DIR`
    /// (absente : mode de transition).
    pub chaine_dir: Option<String>,
}

impl Default for RemoteServerConfig {
    fn default() -> Self {
        Self {
            // Loopback par défaut (fail-closed) ; le LAN nécessite une config explicite.
            listen_addr: "127.0.0.1:8091".into(),
            auth_db_path: "central.db".into(),
            cors_allowed_origins: Vec::new(),
            e2e_identity_path: None,
            paired_devices_path: None,
            pairing_pending_ttl_secs: DEFAULT_PENDING_TTL.as_secs(),
            pairing_require_same_source: true,
            cle_emission_path: None,
            cog_id: None,
            chaine_dir: std::env::var("CENTRAL_REMOTE_CHAINE_DIR").ok().filter(|v| !v.trim().is_empty()),
        }
    }
}

/// Limiteur de débit à fenêtre glissante en mémoire, par IP.
struct RateLimiter {
    window: Duration,
    max: usize,
    hits: Mutex<HashMap<IpAddr, VecDeque<Instant>>>,
}

impl RateLimiter {
    fn new(window: Duration, max: usize) -> Self {
        Self { window, max, hits: Mutex::new(HashMap::new()) }
    }

    /// Enregistre une tentative ; false si la limite est dépassée (ou verrou empoisonné).
    fn check(&self, ip: IpAddr) -> bool {
        let Ok(mut hits) = self.hits.lock() else { return false };
        let now = Instant::now();
        // Purge opportuniste pour borner la mémoire.
        if hits.len() > 4096 {
            let window = self.window;
            hits.retain(|_, q| q.back().is_some_and(|t| now.duration_since(*t) < window));
        }
        let q = hits.entry(ip).or_default();
        while q.front().is_some_and(|t| now.duration_since(*t) >= self.window) {
            q.pop_front();
        }
        if q.len() >= self.max {
            return false;
        }
        q.push_back(now);
        true
    }
}

/// État partagé du serveur remote.
#[derive(Clone)]
struct ServerState {
    bridge: RemoteBridge,
    /// Profils Central (nom affiché du titulaire).
    auth_db: Arc<CentralAuthDb>,
    pairing: Arc<PairingManager>,
    /// COG d'origine des appareils : passeports, visas, preuve de possession.
    autorite: Arc<AutoriteAppareils>,
    /// Limiteur des routes d'appairage, de handshake E2E et de titres.
    bridge_limiter: Arc<RateLimiter>,
    /// Limiteur du suivi d'appairage (`/api/bridge/pair/status`).
    status_limiter: Arc<RateLimiter>,
    /// Limiteur de l'émission de défis.
    defi_limiter: Arc<RateLimiter>,
    /// Identité E2E longue durée du COG (secret effacé à la destruction).
    cog_identity: Arc<E2eIdentity>,
    /// Sessions E2E établies avec les appareils appairés.
    e2e_sessions: Arc<E2eSessionStore>,
    /// Signal d'arrêt pour fermer les WebSocket ouverts.
    shutdown: tokio::sync::watch::Receiver<bool>,
}

/// Construit la couche CORS : aucune par défaut, liste blanche stricte sinon.
fn build_cors(origins: &[String]) -> Option<CorsLayer> {
    let parsed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| match HeaderValue::from_str(o.trim()) {
            Ok(v) if o.trim() != "*" => Some(v),
            _ => {
                warn!("CentralRemote: origine CORS ignorée: {o}");
                None
            }
        })
        .collect();
    if parsed.is_empty() {
        return None;
    }
    Some(
        CorsLayer::new()
            .allow_origin(parsed)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::HeaderName::from_static(ENTETE_POP),
                header::HeaderName::from_static(ENTETE_DELEGATIONS),
            ])
            .expose_headers([header::HeaderName::from_static(ENTETE_DELEGATIONS)]),
    )
}

fn build_router(state: ServerState, cors: Option<CorsLayer>) -> Router {
    let app = Router::new()
        .route(WS_ENDPOINT, get(handle_ws_upgrade))
        .route("/status", get(handle_status))
        // Identité signée des appareils (passeport, défi, visa, validation).
        .route(
            PASSEPORT_ENDPOINT,
            post(handle_bridge_passeport).layer(DefaultBodyLimit::max(TITRE_MAX_BODY)),
        )
        .route(DEFI_ENDPOINT, post(handle_bridge_defi))
        .route(
            VISA_ENDPOINT,
            post(handle_bridge_visa).layer(DefaultBodyLimit::max(TITRE_MAX_BODY)),
        )
        .route(
            VALIDATE_ENDPOINT,
            post(handle_bridge_validate).layer(DefaultBodyLimit::max(TITRE_MAX_BODY)),
        )
        // Appairage mobile (COG Bridge)
        .route(
            PAIR_ENDPOINT,
            post(handle_bridge_pair).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            PAIR_REVEAL_ENDPOINT,
            post(handle_bridge_pair_reveal).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            PAIR_STATUS_ENDPOINT,
            post(handle_bridge_pair_status).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            "/api/bridge/e2e-handshake",
            post(handle_bridge_e2e_handshake)
                .layer(DefaultBodyLimit::max(MAX_HANDSHAKE_JSON_LEN)),
        )
        .route("/api/bridge/discovery", get(handle_bridge_discovery))
        .with_state(state);

    match cors {
        Some(c) => app.layer(c),
        None => app,
    }
}

/// Lance le serveur CentralRemote. Retourne un handle pour l'arrêter.
/// Cette fonction est non-bloquante et spawn le serveur en background.
/// Le listener UDP de découverte LAN n'est lancé que si le serveur HTTP écoute
/// lui-même sur une interface non-loopback.
pub async fn start_remote_server(
    config: RemoteServerConfig,
    bridge: RemoteBridge,
) -> Result<RemoteServerHandle, String> {
    let auth_db = CentralAuthDb::open(Path::new(&config.auth_db_path))
        .map_err(|e| format!("CentralRemote auth DB: {e}"))?;

    // Identité E2E longue durée du COG : chargée ou créée (0600), sinon refus
    // de démarrer (fail-closed).
    let identity_path = config.e2e_identity_path.as_ref().map_or_else(
        || {
            Path::new(&config.auth_db_path)
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(DEFAULT_IDENTITY_FILE)
        },
        PathBuf::from,
    );
    let cog_identity = Arc::new(
        load_or_create_cog_identity(&identity_path).map_err(|e| format!("CentralRemote: {e}"))?,
    );
    let cog_identity_key_hex = hex::encode(cog_identity.public_key_bytes());
    let cog_identity_fingerprint = identity_fingerprint(&cog_identity.public_key_bytes());
    let identity_dir = identity_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();

    // Liste blanche persistée à côté de l'identité ; fichier corrompu = refus
    // de démarrer (fail-closed, R-024).
    let paired_path = config.paired_devices_path.as_ref().map_or_else(
        || identity_dir.join(DEFAULT_PAIRED_DEVICES_FILE),
        PathBuf::from,
    );
    let pairing_config = PairingConfig {
        pending_ttl: Duration::from_secs(config.pairing_pending_ttl_secs),
        require_same_source: config.pairing_require_same_source,
        ..PairingConfig::default()
    };
    let pairing = Arc::new(PairingManager::open(pairing_config, paired_path.clone()).map_err(
        |e| {
            format!(
                "CentralRemote: {e} ({}) — démarrage refusé ; corriger ou supprimer \
                 ce fichier (suppression = ré-appairage de tous les appareils)",
                paired_path.display()
            )
        },
    )?);

    // Clé d'émission (passeports + visas) et liste de révocation : sans elles,
    // refus de démarrer (fail-closed).
    let emission_path = config.cle_emission_path.as_ref().map_or_else(
        || identity_dir.join(FICHIER_CLE_EMISSION_DEFAUT),
        PathBuf::from,
    );
    let autorite = Arc::new(
        AutoriteAppareils::charger_avec_chaine(
            &emission_path,
            config.cog_id.as_deref(),
            config.chaine_dir.as_deref().map(Path::new),
            pairing.clone(),
        )
        .map_err(|e| format!("CentralRemote: {e}"))?,
    );
    info!("CentralRemote: COG d'origine des appareils, cog_id {}", autorite.cog());

    let listener = TcpListener::bind(&config.listen_addr)
        .await
        .map_err(|e| format!("CentralRemote bind {}: {e}", config.listen_addr))?;

    let local_addr = listener
        .local_addr()
        .map_err(|e| format!("CentralRemote local_addr: {e}"))?;

    // Découverte UDP : seulement si le HTTP est lui-même exposé hors loopback.
    let discovery_handle = if crate::discovery::should_start_discovery(&local_addr) {
        match crate::discovery::start_discovery_listener(pairing.clone()).await {
            Ok(h) => {
                info!("Découverte LAN activée (UDP port 19847)");
                Some(h)
            }
            Err(e) => {
                warn!("Découverte LAN non activée: {e}");
                None
            }
        }
    } else {
        info!("Serveur en loopback : découverte LAN UDP désactivée");
        None
    };

    let (ws_shutdown_tx, ws_shutdown_rx) = tokio::sync::watch::channel(false);

    let state = ServerState {
        bridge,
        auth_db: Arc::new(auth_db),
        pairing: pairing.clone(),
        autorite: autorite.clone(),
        bridge_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, BRIDGE_RATE_MAX)),
        status_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, PAIR_STATUS_RATE_MAX)),
        defi_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, DEFI_RATE_MAX)),
        cog_identity,
        e2e_sessions: Arc::new(E2eSessionStore::default()),
        shutdown: ws_shutdown_rx,
    };

    let app = build_router(state, build_cors(&config.cors_allowed_origins));

    info!("CentralRemote server listening on {local_addr}");

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        })
        .await
        .ok();
        info!("CentralRemote server stopped");
    });

    Ok(RemoteServerHandle {
        addr: local_addr,
        shutdown_tx: Some(shutdown_tx),
        ws_shutdown_tx,
        pairing,
        cog_id: autorite.cog().to_string(),
        cle_emission_hex: hex::encode(autorite.cle_publique_emission()),
        cog_identity_key_hex,
        cog_identity_fingerprint,
        discovery: discovery_handle,
    })
}

/// Handle pour contrôler le serveur remote.
/// Le serveur s'arrête quand le handle est `shutdown()` ou droppé.
pub struct RemoteServerHandle {
    /// Adresse effective d'écoute.
    pub addr: SocketAddr,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    ws_shutdown_tx: tokio::sync::watch::Sender<bool>,
    /// Gestionnaire de l'appairage (approbation, refus, révocation : appelé
    /// dans le processus par Central Desktop, sans surface HTTP).
    pub pairing: Arc<PairingManager>,
    /// `cog_id` du COG (émetteur des passeports d'appareil).
    cog_id: String,
    /// Clé publique d'émission (hex).
    cle_emission_hex: String,
    /// Clé publique d'identité E2E du COG (hex), publique.
    cog_identity_key_hex: String,
    /// Empreinte lisible de la clé d'identité du COG.
    cog_identity_fingerprint: String,
    /// Handle du listener UDP (gardé pour qu'il reste actif).
    discovery: Option<crate::discovery::DiscoveryHandle>,
}

impl RemoteServerHandle {
    /// Arrête le serveur proprement (ferme aussi les WebSocket ouverts et l'UDP).
    pub fn shutdown(&mut self) {
        let _ = self.ws_shutdown_tx.send(true);
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(d) = self.discovery.as_mut() {
            d.shutdown();
        }
    }

    /// Adresse du serveur sous forme de string.
    pub fn addr_string(&self) -> String {
        self.addr.to_string()
    }

    /// `cog_id` du COG, émetteur et destination des passeports d'appareil.
    pub fn cog_id(&self) -> &str {
        &self.cog_id
    }

    /// Clé publique d'émission (hex 32 octets) qui scelle passeports et visas.
    pub fn cle_emission_publique_hex(&self) -> &str {
        &self.cle_emission_hex
    }

    /// Clé publique d'identité E2E du COG (hex 32 octets), épinglée par les
    /// appareils à l'appairage (affichable sur le desktop).
    pub fn e2e_identity_public_key_hex(&self) -> &str {
        &self.cog_identity_key_hex
    }

    /// Empreinte lisible de la clé d'identité du COG (8 groupes de 4 hex),
    /// à afficher sur Central pour comparaison avec le téléphone.
    pub fn e2e_identity_fingerprint(&self) -> &str {
        &self.cog_identity_fingerprint
    }
}

impl Drop for RemoteServerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Identité signée des appareils
// ═══════════════════════════════════════════════════════════════════════════

fn entete<'a>(headers: &'a HeaderMap, nom: &str) -> Option<&'a str> {
    headers.get(nom).and_then(|v| v.to_str().ok())
}

/// Vue `miyauth::http` d'une requête (liaison = méthode, chemin, SHA-256 du corps).
fn requete_http<'a>(methode: &'a str, chemin: &'a str, headers: &'a HeaderMap, corps: &'a [u8]) -> RequeteHttp<'a> {
    RequeteHttp {
        methode,
        chemin,
        corps,
        autorisation: entete(headers, header::AUTHORIZATION.as_str()),
        pop: entete(headers, ENTETE_POP),
        liaison: None,
    }
}

/// Réponse de refus : statut conseillé par `miyauth::http` (401 ou 503), sans détail.
fn refus_reply(contexte: &str, e: &RefusHttp) -> Response {
    warn!("CentralRemote {contexte} refusé: {e}");
    let code = StatusCode::from_u16(e.statut()).unwrap_or(StatusCode::UNAUTHORIZED);
    (
        code,
        [(header::WWW_AUTHENTICATE, "MiyuVisa")],
        Json(serde_json::json!({ "status": "unauthorized" })),
    )
        .into_response()
}

fn too_many() -> Response {
    (StatusCode::TOO_MANY_REQUESTS, [(header::RETRY_AFTER, "60")]).into_response()
}

/// Nom affiché du titulaire : pseudonyme du profil lié, sinon nom de l'appareil.
fn display_name(state: &ServerState, acces: &AccesAppareil) -> String {
    acces
        .appareil
        .profile_id
        .as_deref()
        .and_then(|id| {
            state
                .auth_db
                .get_profile(id)
                .ok()
                .flatten()
                .map(|p| p.pseudonyme.filter(|s| !s.is_empty()).unwrap_or(p.email))
        })
        .unwrap_or_else(|| acces.appareil.name.clone())
}

/// POST /api/bridge/passeport — passeport d'appareil (requête signée par la
/// clé d'appareil épinglée). En-têtes contrôlés avant le corps.
async fn handle_bridge_passeport(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return too_many();
    }
    let maintenant = maintenant_unix();
    let entetes = match EntetesSignees::analyser(
        entete(&headers, ENTETE_SIGNATAIRE),
        entete(&headers, ENTETE_HORODATAGE),
        entete(&headers, ENTETE_NONCE),
        entete(&headers, ENTETE_SIGNATURE),
    ) {
        Ok(e) => e,
        Err(e) => return refus_reply("passeport", &e),
    };
    match state.autorite.delivrer_passeport(&entetes, &body, maintenant) {
        Ok((passeport, expire_a)) => {
            if let Some(d) = state.pairing.device_for_identity_key(&hex::encode(entetes.signataire)) {
                state.pairing.touch_device(&d.device_id);
                info!("CentralRemote: passeport émis ({})", mask_device_id(&d.device_id));
            }
            Json(PasseportResponse {
                passeport: b64_encoder(&passeport),
                expire_a,
                cog_id: state.autorite.cog().to_string(),
            })
            .into_response()
        }
        Err(e) => refus_reply("passeport", &e),
    }
}

/// POST /api/bridge/defi — défi de possession à usage unique (60 s).
async fn handle_bridge_defi(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
) -> Response {
    if !state.defi_limiter.check(peer.ip().to_canonical()) {
        return too_many();
    }
    match state.autorite.emettre_defi(maintenant_unix()) {
        Ok(d) => Json(DefiResponse {
            defi_id: d.id_b64(),
            defi: d.defi_b64(),
            audience: d.audience,
            expire_a: d.expire_a,
            cog_id: state.autorite.cog().to_string(),
        })
        .into_response(),
        Err(e) => refus_reply("défi", &e),
    }
}

/// POST /api/bridge/visa — `MiyuPasseport` + `X-Miyu-PoP` → visa ≤ 15 min.
async fn handle_bridge_visa(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return too_many();
    }
    let requete = requete_http("POST", VISA_ENDPOINT, &headers, &body);
    let delegations = entete(&headers, ENTETE_DELEGATIONS);
    match state.autorite.delivrer_visa_avec_delegations(&requete, delegations, maintenant_unix()) {
        Ok(v) => {
            state.pairing.touch_device(&v.acces.appareil.device_id);
            let mut reponse = Json(VisaResponse {
                visa: b64_encoder(&v.visa),
                expire_a: v.acces.expire_a,
                role: texte_role(v.acces.role).into(),
                display_name: display_name(&state, &v.acces),
                identite: v.acces.identite,
            })
            .into_response();
            // Délégations 0x45 (`visas`, `emission`) à présenter avec le visa (A-024).
            let d: Vec<&[u8]> = state.autorite.delegations_visa().iter().map(Vec::as_slice).collect();
            if !d.is_empty() {
                if let Ok(valeur) = HeaderValue::from_str(&formater_delegations(&d)) {
                    reponse.headers_mut().insert(ENTETE_DELEGATIONS, valeur);
                }
            }
            reponse
        }
        Err(e) => refus_reply("visa", &e),
    }
}

/// POST /api/bridge/validate — vérifie `MiyuVisa` + `X-Miyu-PoP`.
async fn handle_bridge_validate(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return too_many();
    }
    let requete = requete_http("POST", VALIDATE_ENDPOINT, &headers, &body);
    let delegations = entete(&headers, ENTETE_DELEGATIONS);
    match state.autorite.verifier_acces_avec_delegations(&requete, delegations, maintenant_unix()) {
        Ok(acces) => {
            state.pairing.touch_device(&acces.appareil.device_id);
            Json(ValidateResponse {
                status: "ok".into(),
                role: texte_role(acces.role).into(),
                expire_a: acces.expire_a,
                display_name: display_name(&state, &acces),
                identite: acces.identite,
            })
            .into_response()
        }
        Err(e) => refus_reply("validation", &e),
    }
}

/// GET /ws — upgrade WebSocket après vérification visa + preuve de possession
/// (défi obtenu juste avant). Toute chaîne de requête est refusée : le jeton
/// `?token=` n'existe plus.
async fn handle_ws_upgrade(
    State(state): State<ServerState>,
    uri: Uri,
    headers: HeaderMap,
    ws: Result<WebSocketUpgrade, WebSocketUpgradeRejection>,
) -> Response {
    // Authentification d'abord : une requête non authentifiée est refusée
    // (400 / 401) avant tout examen de l'upgrade.
    if uri.query().is_some() {
        warn!("CentralRemote WS refusé: chaîne de requête (jeton en URL non supporté)");
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"status": "query_refused"})))
            .into_response();
    }
    let requete = requete_http("GET", WS_ENDPOINT, &headers, b"");
    let delegations = entete(&headers, ENTETE_DELEGATIONS);
    match state.autorite.verifier_acces_avec_delegations(&requete, delegations, maintenant_unix()) {
        Ok(acces) => {
            info!(
                "CentralRemote WS upgrade ({})",
                mask_device_id(&acces.appareil.device_id)
            );
            state.pairing.touch_device(&acces.appareil.device_id);
            match ws {
                Ok(ws) => ws.on_upgrade(move |socket| handle_ws_connection(socket, state, acces)),
                Err(rejet) => rejet.into_response(),
            }
        }
        // Refus avant l'upgrade.
        Err(e) => refus_reply("WS", &e),
    }
}

/// Gère une connexion WebSocket authentifiée (durée bornée par le visa).
async fn handle_ws_connection(socket: WebSocket, state: ServerState, acces: AccesAppareil) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let mut shutdown = state.shutdown.clone();
    let mut recheck = tokio::time::interval(WS_RECHECK_INTERVAL);
    let valide = |state: &ServerState| state.autorite.acces_toujours_valide(&acces, maintenant_unix());

    // Envoyer le snapshot initial.
    let snapshot = state.bridge.current_snapshot();
    let initial = RemoteEvent::Snapshot(snapshot);
    if let Ok(json) = serde_json::to_string(&initial) {
        if ws_tx.send(Message::Text(json.into())).await.is_err() {
            return;
        }
    }

    // Souscrire aux événements du bridge.
    let mut event_rx = state.bridge.subscribe_events();

    // Boucle principale : forward events ↔ commands.
    loop {
        tokio::select! {
            // Arrêt du serveur
            _ = shutdown.changed() => {
                let _ = ws_tx.send(Message::Close(None)).await;
                break;
            }
            // Revérification périodique : expiration du visa ou révocation de l'appareil
            _ = recheck.tick() => {
                if !valide(&state) {
                    warn!("CentralRemote WS fermé : visa expiré ou appareil révoqué");
                    if let Ok(json) = serde_json::to_string(&RemoteEvent::Error {
                        message: "Visa expiré ou appareil révoqué".into(),
                    }) {
                        let _ = ws_tx.send(Message::Text(json.into())).await;
                    }
                    let _ = ws_tx.send(Message::Close(None)).await;
                    break;
                }
            }
            // Événement Central → client WS
            Ok(event) = event_rx.recv() => {
                if let Ok(json) = serde_json::to_string(&event) {
                    if ws_tx.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
            // Message client WS → commande Central
            msg = ws_rx.next() => {
                // Chaque commande revalide aussi l'accès (pas d'attente du tick).
                if !valide(&state) {
                    let _ = ws_tx.send(Message::Close(None)).await;
                    break;
                }
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<RemoteCommand>(&text) {
                            match &cmd {
                                RemoteCommand::Ping => {
                                    if let Ok(json) = serde_json::to_string(&RemoteEvent::Pong) {
                                        let _ = ws_tx.send(Message::Text(json.into())).await;
                                    }
                                }
                                RemoteCommand::RequestSnapshot => {
                                    let snapshot = state.bridge.current_snapshot();
                                    let evt = RemoteEvent::Snapshot(snapshot);
                                    if let Ok(json) = serde_json::to_string(&evt) {
                                        let _ = ws_tx.send(Message::Text(json.into())).await;
                                    }
                                }
                                _ => {
                                    state.bridge.send_command(cmd);
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}

/// GET /status — santé du serveur remote.
async fn handle_status(State(state): State<ServerState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "CentralRemote",
        "connected_clients": state.bridge.connected_clients(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Handlers d'appairage mobile (COG Bridge)
// ═══════════════════════════════════════════════════════════════════════════

/// Réponse JSON `{"status": <statut>}` avec un code HTTP.
fn status_reply(code: StatusCode, status: &str) -> Response {
    (code, Json(serde_json::json!({ "status": status }))).into_response()
}

/// Traduit une erreur d'appairage en réponse HTTP (une cause = un statut).
fn pairing_error_reply(e: &PairingError) -> Response {
    match e {
        PairingError::InvalidDeviceId => status_reply(StatusCode::BAD_REQUEST, "invalid_device_id"),
        PairingError::InvalidIdentityKey => {
            status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key")
        }
        PairingError::TooManyPending => {
            status_reply(StatusCode::SERVICE_UNAVAILABLE, "too_many_pending")
        }
        PairingError::TooManyPendingForSource => {
            status_reply(StatusCode::TOO_MANY_REQUESTS, "too_many_pending_for_source")
        }
        PairingError::PendingConflict => status_reply(StatusCode::CONFLICT, "pending_conflict"),
        PairingError::KeyInUse => status_reply(StatusCode::CONFLICT, "key_in_use"),
        PairingError::AlreadyPaired => status_reply(StatusCode::CONFLICT, "already_paired"),
        PairingError::NoPending => status_reply(StatusCode::NOT_FOUND, "no_pending"),
        PairingError::AlreadyRevealed => status_reply(StatusCode::CONFLICT, "already_revealed"),
        PairingError::CommitmentMismatch => {
            status_reply(StatusCode::FORBIDDEN, "commitment_mismatch")
        }
        PairingError::NotRevealed => status_reply(StatusCode::CONFLICT, "not_revealed"),
        PairingError::CodeMismatch => status_reply(StatusCode::CONFLICT, "code_mismatch"),
        PairingError::InvalidSignature => {
            status_reply(StatusCode::UNAUTHORIZED, STATUS_INVALID_SIGNATURE)
        }
        PairingError::SourceMismatch => status_reply(StatusCode::FORBIDDEN, STATUS_SOURCE_MISMATCH),
        PairingError::KeyRevoked => status_reply(StatusCode::FORBIDDEN, "key_revoked"),
        PairingError::Persistence(_) | PairingError::Unavailable => {
            status_reply(StatusCode::SERVICE_UNAVAILABLE, "unavailable")
        }
    }
}

/// Décode la signature d'appairage d'une requête. Absente (requête v2 non
/// signée, format antérieur) : refus explicite `missing_pairing_signature`,
/// sans repli ; mal formée : `invalid_pairing_signature`.
/// L'erreur porte le statut à renvoyer (HTTP 400).
fn pairing_signature_field(s: &str) -> Result<[u8; PAIRING_SIGNATURE_SIZE], &'static str> {
    if s.is_empty() {
        warn!("CentralRemote appairage refusé: requête non signée");
        return Err(STATUS_MISSING_SIGNATURE);
    }
    parse_pairing_signature_hex(s).map_err(|_| STATUS_INVALID_SIGNATURE)
}

/// Refus explicite d'une requête d'appairage d'une autre version (v1 comprise).
fn unsupported_version_reply(version: u8) -> Response {
    warn!("CentralRemote appairage refusé: version {version} (attendue {PAIRING_PROTOCOL_VERSION})");
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "status": STATUS_UNSUPPORTED_VERSION,
            "expected_pairing_version": PAIRING_PROTOCOL_VERSION,
        })),
    )
        .into_response()
}

/// POST /api/bridge/pair — étape 1 de l'appairage v2 (engagement).
///
/// L'appareil envoie sa clé d'identité E2E et l'engagement `C` sur son aléa
/// `Nd` ; le COG enregistre la demande et répond avec sa clé d'identité et son
/// aléa `Nc`. Aucun code n'est transmis : chaque côté le calcule après la
/// révélation. L'approbation sur le desktop reste requise.
async fn handle_bridge_pair(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    let source = peer.ip().to_canonical();
    if !state.bridge_limiter.check(source) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    if let Err(v) = check_pairing_version(&body) {
        return unsupported_version_reply(v);
    }
    let Ok(req) = serde_json::from_slice::<PairCommitRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_key) = parse_identity_key_hex(&req.e2e_identity_key) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key");
    };
    let Ok(commitment) = parse_commitment_hex(&req.commitment) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_commitment");
    };
    let signature = match pairing_signature_field(&req.commit_signature) {
        Ok(sig) => sig,
        Err(status) => return status_reply(StatusCode::BAD_REQUEST, status),
    };
    let cog_key = state.cog_identity.public_key_bytes();
    // La signature est vérifiée par le gestionnaire avant toute lecture d'état.
    match state.pairing.request_pairing(
        &req.device_id,
        &req.device_name,
        &hex::encode(device_key),
        commitment,
        &signature,
        source,
    ) {
        Ok(cog_nonce) => {
            info!(
                "CentralRemote: demande d'appairage v2 ({})",
                mask_device_id(&req.device_id)
            );
            Json(PairCommitResponse {
                pairing_version: PAIRING_PROTOCOL_VERSION,
                status: PairingStatus::Committed,
                cog_identity_key: hex::encode(cog_key),
                cog_nonce: cog_nonce.to_hex().to_string(),
                message: "Révélez votre aléa, puis comparez le code avec Central Desktop".into(),
            })
            .into_response()
        }
        Err(e) => {
            warn!(
                "CentralRemote appairage refusé ({}): {e}",
                mask_device_id(&req.device_id)
            );
            pairing_error_reply(&e)
        }
    }
}

/// POST /api/bridge/pair/reveal — étape 3 (révélation de `Nd`).
///
/// Refus : version ou signature absente (400), aucune demande — donc aucun
/// `Nc` émis (404), signature invalide (401) ou autre source (403) — demande
/// intacte —, déjà révélé (409), engagement incohérent valablement signé
/// (403, demande abandonnée).
async fn handle_bridge_pair_reveal(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    let source = peer.ip().to_canonical();
    if !state.bridge_limiter.check(source) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    if let Err(v) = check_pairing_version(&body) {
        return unsupported_version_reply(v);
    }
    let Ok(req) = serde_json::from_slice::<PairRevealRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_nonce) = PairingNonce::from_hex(&req.device_nonce) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_device_nonce");
    };
    let signature = match pairing_signature_field(&req.reveal_signature) {
        Ok(sig) => sig,
        Err(status) => return status_reply(StatusCode::BAD_REQUEST, status),
    };
    let cog_key = state.cog_identity.public_key_bytes();
    match state
        .pairing
        .reveal_pairing(&req.device_id, &device_nonce, &signature, source, &cog_key)
    {
        Ok(_code) => Json(PairRevealResponse {
            status: PairingStatus::Revealed,
        })
        .into_response(),
        Err(e) => {
            warn!(
                "CentralRemote révélation refusée ({}): {e}",
                mask_device_id(&req.device_id)
            );
            pairing_error_reply(&e)
        }
    }
}

/// POST /api/bridge/pair/status — suivi d'une demande par l'appareil.
async fn handle_bridge_pair_status(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    if !state.status_limiter.check(peer.ip().to_canonical()) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    let Ok(req) = serde_json::from_slice::<PairStatusRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_key) = parse_identity_key_hex(&req.e2e_identity_key) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key");
    };
    let status = state
        .pairing
        .pairing_status(&req.device_id, &hex::encode(device_key));
    Json(PairStatusResponse { status }).into_response()
}

/// POST /api/bridge/e2e-handshake — handshake E2E v2 avec un appareil appairé.
///
/// Refus : offre mal formée ou version ≠ 2 (400), appareil inconnu (403),
/// offre non vérifiable — signature, rôle, clé éphémère (401).
async fn handle_bridge_e2e_handshake(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    let offer = match E2eHandshake::from_json(&body) {
        Ok(o) => o,
        Err(e) => {
            warn!("CentralRemote handshake E2E refusé: {e}");
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "offre E2E invalide ou version non supportée"})),
            )
                .into_response();
        }
    };
    let Ok(device_key) = parse_identity_key_hex(&offer.identity_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let device_key_hex = hex::encode(device_key);
    let Some(device) = state.pairing.device_for_identity_key(&device_key_hex) else {
        warn!("CentralRemote handshake E2E refusé: appareil non appairé");
        return StatusCode::FORBIDDEN.into_response();
    };
    let (pending, cog_offer) =
        match PendingHandshake::initiate(E2eRole::Cog, &state.cog_identity, &device_key) {
            Ok(p) => p,
            Err(e) => {
                warn!("CentralRemote handshake E2E refusé: {e}");
                return StatusCode::FORBIDDEN.into_response();
            }
        };
    // Vérification complète de l'offre (signature avant dérivation, C-23).
    let session = match pending.complete(&offer) {
        Ok(s) => s,
        Err(e) => {
            warn!(
                "CentralRemote handshake E2E refusé ({}): {e}",
                mask_device_id(&device.device_id)
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    if !state
        .e2e_sessions
        .insert(&device.device_id, &device_key_hex, session)
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    state.pairing.touch_device(&device.device_id);
    info!(
        "CentralRemote: session E2E v2 établie ({})",
        mask_device_id(&device.device_id)
    );
    Json(cog_offer).into_response()
}


/// GET /api/bridge/discovery — info de découverte LAN.
async fn handle_bridge_discovery(
    State(state): State<ServerState>,
) -> Json<serde_json::Value> {
    let info = state.pairing.discovery_info();
    Json(serde_json::to_value(info).unwrap_or_default())
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::connect_info::MockConnectInfo;
    use central_remote_protocol::identite::{AUDIENCE, USAGE_PASSEPORT};
    use miyauth::http::{
        b64_decoder, formater_autorisation_passeport, formater_autorisation_visa, formater_pop,
        liaison_requete, signer_pop, signer_requete,
    };
    use miyusceau::CleSignature;
    use tower::ServiceExt;

    const PASSWORD: &str = "Str0ng!Passw0rd-xyz";

    fn test_state() -> (ServerState, tokio::sync::watch::Sender<bool>, String) {
        let path = std::env::temp_dir().join(format!("central-remote-test-{}.db", uuid::Uuid::new_v4()));
        let db = CentralAuthDb::open(&path).expect("open db");
        let profile = db.sign_up("alice@example.org", PASSWORD, Some("Alice")).expect("sign_up");

        let pairing = Arc::new(PairingManager::new());
        let autorite = Arc::new(
            AutoriteAppareils::new(CleSignature::generer(), None, [9; 32], pairing.clone()).unwrap(),
        );
        let (tx, rx) = tokio::sync::watch::channel(false);
        let state = ServerState {
            bridge: RemoteBridge::new(),
            auth_db: Arc::new(db),
            pairing,
            autorite,
            bridge_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, 1000)),
            status_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, PAIR_STATUS_RATE_MAX)),
            defi_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, 1000)),
            cog_identity: Arc::new(E2eIdentity::generate()),
            e2e_sessions: Arc::new(E2eSessionStore::default()),
            shutdown: rx,
        };
        (state, tx, profile.id)
    }

    /// Demande d'appairage honnête jusqu'à la révélation, directement sur le
    /// gestionnaire ; retourne l'identité de l'appareil et le code SAS.
    fn commit_reveal(state: &ServerState, device_id: &str) -> (E2eIdentity, String) {
        let device = E2eIdentity::generate();
        let cog_key = state.cog_identity.public_key_bytes();
        let code = direct_pair(&state.pairing, cog_key, &device, device_id, "192.168.1.30");
        (device, code)
    }

    /// Engagement + révélation signés, directement sur le gestionnaire ;
    /// retourne le code SAS (identique des deux côtés).
    fn direct_pair(
        mgr: &PairingManager,
        cog_key: [u8; 32],
        device: &E2eIdentity,
        device_id: &str,
        src: &str,
    ) -> String {
        let key = device.public_key_bytes();
        let pairing = DevicePairing::start(key);
        let c = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        let sig = parse_pairing_signature_hex(&pairing.commit_signature_hex(device, device_id).unwrap()).unwrap();
        let src: IpAddr = src.parse().unwrap();
        let nc = mgr
            .request_pairing(device_id, "Pixel", &hex::encode(key), c, &sig, src)
            .unwrap();
        let revealed = pairing.receive_cog_nonce(cog_key, nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let rsig =
            parse_pairing_signature_hex(&revealed.reveal_signature_hex(device, device_id).unwrap()).unwrap();
        let code = mgr.reveal_pairing(device_id, &nd, &rsig, src, &cog_key).unwrap();
        assert_eq!(code, revealed.code());
        code
    }

    fn app_for(state: ServerState, peer: &str) -> Router {
        let addr: SocketAddr = peer.parse().unwrap();
        build_router(state, None).layer(MockConnectInfo(addr))
    }

    async fn call(app: Router, req: axum::http::Request<Body>) -> (StatusCode, String) {
        let resp = app.oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20).await.unwrap();
        (status, String::from_utf8_lossy(&bytes).to_string())
    }

    #[test]
    fn default_config_is_loopback() {
        let config = RemoteServerConfig::default();
        assert_eq!(config.listen_addr, "127.0.0.1:8091");
        assert_eq!(config.cors_allowed_origins, Vec::<String>::new());
        assert!(config.cle_emission_path.is_none() && config.cog_id.is_none());
    }

    #[test]
    fn cors_none_by_default_and_wildcard_ignored() {
        assert!(build_cors(&[]).is_none());
        assert!(build_cors(&["*".into()]).is_none());
        assert!(build_cors(&["http://localhost:3000".into()]).is_some());
    }

    #[test]
    fn rate_limiter_blocks_after_max() {
        let rl = RateLimiter::new(Duration::from_secs(60), 3);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        assert!(rl.check(ip) && rl.check(ip) && rl.check(ip));
        assert!(!rl.check(ip));
        // Autre IP indépendante.
        assert!(rl.check("10.0.0.2".parse().unwrap()));
        // Fenêtre expirée.
        let rl = RateLimiter::new(Duration::from_millis(1), 1);
        assert!(rl.check(ip));
        std::thread::sleep(Duration::from_millis(5));
        assert!(rl.check(ip));
    }

    // ── E2E mobile ↔ COG (R-023) ─────────────────────────────────────────

    use miyukini_cog_bridge::{DevicePairing, E2eSession};

    const PEER_LAN: &str = "192.168.1.20:7100";

    fn post_json(uri: &str, body: String) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }

    /// Appaire un appareil (demande + approbation desktop) et retourne son identité.
    fn paired_device(state: &ServerState, device_id: &str) -> E2eIdentity {
        let (device, code) = commit_reveal(state, device_id);
        state.pairing.approve_device(device_id, &code).unwrap();
        device
    }

    async fn handshake(state: &ServerState, device: &E2eIdentity) -> E2eSession {
        let cog_key = state.cog_identity.public_key_bytes();
        let (pending, offer) =
            PendingHandshake::initiate(E2eRole::Device, device, &cog_key).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let cog_offer = E2eHandshake::from_json(body.as_bytes()).unwrap();
        pending.complete(&cog_offer).unwrap()
    }


    // ── Identité signée : passeport → visa → requête (sortie du lot 2) ──────

    /// Appareil appairé, lié au profil `profile_id`, et sa clé de signature.
    fn appareil_appaire(state: &ServerState, device_id: &str, profile_id: Option<&str>) -> (E2eIdentity, CleSignature) {
        let (device, code) = commit_reveal(state, device_id);
        state.pairing.approve_device_for(device_id, &code, profile_id).unwrap();
        let cle = CleSignature::depuis_graine(&device.secret_bytes());
        (device, cle)
    }

    fn req_passeport(cle: &CleSignature) -> axum::http::Request<Body> {
        let e = signer_requete(cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant_unix());
        let mut b = axum::http::Request::builder().method("POST").uri(PASSEPORT_ENDPOINT);
        for (k, v) in e.en_entetes() {
            b = b.header(k, v);
        }
        b.body(Body::empty()).unwrap()
    }

    async fn passeport(state: &ServerState, cle: &CleSignature) -> Vec<u8> {
        let (st, body) = call(app_for(state.clone(), PEER_LAN), req_passeport(cle)).await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let r: PasseportResponse = serde_json::from_str(&body).unwrap();
        assert_eq!(r.cog_id, state.autorite.cog().as_str());
        b64_decoder(&r.passeport).unwrap()
    }

    /// Défi neuf (HTTP) et preuve de possession liée à `methode chemin` (corps vide).
    async fn pop(state: &ServerState, cle: &CleSignature, methode: &str, chemin: &str) -> String {
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            axum::http::Request::builder().method("POST").uri(DEFI_ENDPOINT).body(Body::empty()).unwrap(),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let d: DefiResponse = serde_json::from_str(&body).unwrap();
        assert_eq!(d.audience, AUDIENCE);
        let id: [u8; 16] = b64_decoder(&d.defi_id).unwrap().try_into().unwrap();
        let defi: [u8; 32] = b64_decoder(&d.defi).unwrap().try_into().unwrap();
        let sig = signer_pop(cle, &defi, AUDIENCE, &liaison_requete(methode, chemin, b""));
        formater_pop(&id, &sig)
    }

    fn req_titre(methode: &str, uri: &str, autorisation: &str, pop: &str) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method(methode)
            .uri(uri)
            .header(header::AUTHORIZATION, autorisation)
            .header(ENTETE_POP, pop)
            .body(Body::empty())
            .unwrap()
    }

    /// Requête d'ouverture WebSocket authentifiée.
    fn req_ws(uri: &str, autorisation: &str, pop: &str) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("GET")
            .uri(uri)
            .header(header::CONNECTION, "upgrade")
            .header(header::UPGRADE, "websocket")
            .header(header::SEC_WEBSOCKET_VERSION, "13")
            .header(header::SEC_WEBSOCKET_KEY, "dGhlIHNhbXBsZSBub25jZQ==")
            .header(header::AUTHORIZATION, autorisation)
            .header(ENTETE_POP, pop)
            .body(Body::empty())
            .unwrap()
    }

    async fn visa(state: &ServerState, cle: &CleSignature, passeport: &[u8]) -> (StatusCode, String) {
        let pop = pop(state, cle, "POST", VISA_ENDPOINT).await;
        call(
            app_for(state.clone(), PEER_LAN),
            req_titre("POST", VISA_ENDPOINT, &formater_autorisation_passeport(passeport), &pop),
        )
        .await
    }

    /// Passeport + visa d'un appareil appairé ; rend `Authorization: MiyuVisa …`.
    async fn autorisation_visa(state: &ServerState, cle: &CleSignature) -> String {
        let p = passeport(state, cle).await;
        let (st, body) = visa(state, cle, &p).await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let v: VisaResponse = serde_json::from_str(&body).unwrap();
        formater_autorisation_visa(&b64_decoder(&v.visa).unwrap(), &p)
    }

    #[tokio::test]
    async fn full_flow_pair_passport_visa_request() {
        let (state, _tx, profile_id) = test_state();
        let (_device, cle) = appareil_appaire(&state, "device-flow-0001", Some(&profile_id));
        let p = passeport(&state, &cle).await;
        let (st, body) = visa(&state, &cle, &p).await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let v: VisaResponse = serde_json::from_str(&body).unwrap();
        assert_eq!(v.role, "citizen");
        assert_eq!(v.display_name, "Alice");
        assert!(v.identite.starts_with(&format!("pw:{}:", state.autorite.cog())));
        assert!(v.expire_a <= maintenant_unix() + miyauth::titres::DUREE_MAX_VISA_SECS);

        let auth = formater_autorisation_visa(&b64_decoder(&v.visa).unwrap(), &p);
        let pop1 = pop(&state, &cle, "POST", VALIDATE_ENDPOINT).await;
        let (st, body) = call(app_for(state.clone(), PEER_LAN), req_titre("POST", VALIDATE_ENDPOINT, &auth, &pop1)).await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let r: ValidateResponse = serde_json::from_str(&body).unwrap();
        assert_eq!((r.status.as_str(), r.identite.as_str()), ("ok", v.identite.as_str()));

        // Visa sans preuve de possession : refus (pas de jeton au porteur).
        let req = axum::http::Request::builder()
            .method("POST")
            .uri(VALIDATE_ENDPOINT)
            .header(header::AUTHORIZATION, &auth)
            .body(Body::empty())
            .unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Passeport présenté comme visa : refus.
        let pop2 = pop(&state, &cle, "POST", VALIDATE_ENDPOINT).await;
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            req_titre("POST", VALIDATE_ENDPOINT, &formater_autorisation_passeport(&p), &pop2),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn replayed_pop_refused() {
        let (state, _tx, _) = test_state();
        let (_d, cle) = appareil_appaire(&state, "device-replay-01", None);
        let auth = autorisation_visa(&state, &cle).await;
        let pop1 = pop(&state, &cle, "POST", VALIDATE_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_titre("POST", VALIDATE_ENDPOINT, &auth, &pop1)).await;
        assert_eq!(st, StatusCode::OK);
        // Même preuve rejouée : défi déjà consommé.
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_titre("POST", VALIDATE_ENDPOINT, &auth, &pop1)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Preuve produite pour une autre requête (liaison) : refus.
        let pop_ws = pop(&state, &cle, "GET", WS_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_titre("POST", VALIDATE_ENDPOINT, &auth, &pop_ws)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Requête de passeport rejouée (même nonce) : refus.
        let req = req_passeport(&cle);
        let (parts, _) = req.into_parts();
        let rebuild = || {
            let mut b = axum::http::Request::builder().method("POST").uri(PASSEPORT_ENDPOINT);
            for (k, v) in &parts.headers {
                b = b.header(k, v);
            }
            b.body(Body::empty()).unwrap()
        };
        let (st, _) = call(app_for(state.clone(), PEER_LAN), rebuild()).await;
        assert_eq!(st, StatusCode::OK);
        let (st, _) = call(app_for(state.clone(), PEER_LAN), rebuild()).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn revoked_device_refused() {
        let (state, _tx, _) = test_state();
        let (_d, cle) = appareil_appaire(&state, "device-revoke-01", None);
        let p = passeport(&state, &cle).await;
        let auth = autorisation_visa(&state, &cle).await;
        // Révocation depuis Central (bouton « Révoquer » → PairingManager::revoke_device).
        assert_eq!(state.pairing.revoke_device("device-revoke-01"), Ok(true));
        let pop1 = pop(&state, &cle, "POST", VALIDATE_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_titre("POST", VALIDATE_ENDPOINT, &auth, &pop1)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED, "visa d'un appareil révoqué accepté");
        let (st, _) = visa(&state, &cle, &p).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED, "passeport d'un appareil révoqué accepté");
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_passeport(&cle)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        let pop2 = pop(&state, &cle, "GET", WS_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_ws(WS_ENDPOINT, &auth, &pop2)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn unpaired_key_gets_no_passport() {
        let (state, _tx, _) = test_state();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_passeport(&CleSignature::generer())).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Appareil seulement en attente (révélé, non approuvé) : refus aussi.
        let (device, _code) = commit_reveal(&state, "device-pending-02");
        let cle = CleSignature::depuis_graine(&device.secret_bytes());
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_passeport(&cle)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Sans en-têtes signés.
        let req = axum::http::Request::builder().method("POST").uri(PASSEPORT_ENDPOINT).body(Body::empty()).unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn ws_requires_visa_and_pop_and_refuses_query_token() {
        let (state, _tx, _) = test_state();
        let (_d, cle) = appareil_appaire(&state, "device-ws-0001", None);
        let auth = autorisation_visa(&state, &cle).await;
        // `?token=` : refusé, même accompagné d'un visa valide.
        let pop1 = pop(&state, &cle, "GET", WS_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_ws("/ws?token=abc", &auth, &pop1)).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        let req = axum::http::Request::builder().uri("/ws?token=abc").body(Body::empty()).unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        // Ancien jeton HMAC en Bearer : refus.
        let req = axum::http::Request::builder()
            .uri(WS_ENDPOINT)
            .header(header::AUTHORIZATION, "Bearer prof-1.123.abc.def")
            .body(Body::empty())
            .unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Visa + preuve neuve : authentification acceptée ; la requête de test
        // n'étant pas une vraie connexion HTTP/1.1, seul l'upgrade échoue (426).
        let pop2 = pop(&state, &cle, "GET", WS_ENDPOINT).await;
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_ws(WS_ENDPOINT, &auth, &pop2)).await;
        assert_eq!(st, StatusCode::UPGRADE_REQUIRED);
        // La même preuve ne rouvre pas de session (défi consommé).
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req_ws(WS_ENDPOINT, &auth, &pop2)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn old_api_token_and_device_id_refused() {
        let (state, _tx, _) = test_state();
        let _ = appareil_appaire(&state, "device-legacy-01", None);
        // Mode clé API retiré.
        let req = axum::http::Request::builder()
            .uri("/api/bridge/validate-token")
            .header(header::AUTHORIZATION, "Bearer myk_0123456789abcdef0123456789abcdef")
            .body(Body::empty())
            .unwrap();
        let (st, _) = call(app_for(state.clone(), "127.0.0.1:5000"), req).await;
        assert_eq!(st, StatusCode::NOT_FOUND);
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            post_json("/api/bridge/generate-token", "{\"device_name\":\"Tel\"}".into()),
        )
        .await;
        assert_eq!(st, StatusCode::NOT_FOUND);
        // `device_id` au porteur : plus aucune valeur.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(VALIDATE_ENDPOINT, "{\"device_id\":\"device-legacy-01\"}".into()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Routes d'administration HTTP supprimées (appel dans le processus).
        for (m, uri) in [("GET", "/api/bridge/devices"), ("POST", "/api/bridge/approve"), ("POST", "/api/bridge/reject")] {
            let req = axum::http::Request::builder()
                .method(m)
                .uri(uri)
                .header("x-central-admin-secret", "00")
                .header("content-type", "application/json")
                .body(Body::from("{\"device_id\":\"x\"}"))
                .unwrap();
            let (st, _) = call(app_for(state.clone(), "127.0.0.1:5000"), req).await;
            assert!(st == StatusCode::NOT_FOUND || st == StatusCode::METHOD_NOT_ALLOWED, "{uri}: {st}");
        }
    }

    #[tokio::test]
    async fn password_alone_refused() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-pwd-0001");
        let body = serde_json::json!({"email": "alice@example.org", "password": PASSWORD}).to_string();
        for peer in ["127.0.0.1:6000", PEER_LAN] {
            let (st, resp) = call(app_for(state.clone(), peer), post_json("/auth", body.clone())).await;
            assert_eq!(st, StatusCode::NOT_FOUND, "{resp}");
            assert!(!resp.contains("token"));
        }
        // Même dans une session E2E établie : aucun point d'accès par mot de passe.
        let session = handshake(&state, &device).await;
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/auth")
            .header("content-type", "application/json")
            .header(miyukini_cog_bridge::E2E_SESSION_HEADER, hex::encode(session.session_id()))
            .body(Body::from(body))
            .unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn defi_rate_limited() {
        let (mut state, _tx, _) = test_state();
        state.defi_limiter = Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, 2));
        let req = || axum::http::Request::builder().method("POST").uri(DEFI_ENDPOINT).body(Body::empty()).unwrap();
        for _ in 0..2 {
            let (st, _) = call(app_for(state.clone(), "10.9.9.9:1"), req()).await;
            assert_eq!(st, StatusCode::OK);
        }
        let (st, _) = call(app_for(state.clone(), "10.9.9.9:1"), req()).await;
        assert_eq!(st, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn e2e_handshake_paired_device_succeeds() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0001-abcdef");
        let session = handshake(&state, &device).await;
        assert_eq!(state.e2e_sessions.len(), 1);
        assert!(state
            .e2e_sessions
            .get_hex(&hex::encode(session.session_id()))
            .is_some());
    }

    #[tokio::test]
    async fn e2e_handshake_unknown_device_refused() {
        let (state, _tx, _) = test_state();
        let stranger = E2eIdentity::generate();
        let (_p, offer) = PendingHandshake::initiate(
            E2eRole::Device,
            &stranger,
            &state.cog_identity.public_key_bytes(),
        )
        .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        // Appareil seulement en attente (révélé, non approuvé) : refus également.
        let (pending_dev, _code) = commit_reveal(&state, "device-pending-01");
        let (_p, offer) = PendingHandshake::initiate(
            E2eRole::Device,
            &pending_dev,
            &state.cog_identity.public_key_bytes(),
        )
        .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(state.e2e_sessions.is_empty());
    }

    #[tokio::test]
    async fn e2e_handshake_v1_offer_refused() {
        let (state, _tx, _) = test_state();
        let _device = paired_device(&state, "device-0002-abcdef");
        let v1 = serde_json::json!({
            "public_key": "00".repeat(32),
            "protocol_version": 1,
        });
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", v1.to_string()),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        // Sans champ de version (v1 historique) : refus aussi.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                "/api/bridge/e2e-handshake",
                format!("{{\"public_key\":\"{}\"}}", "00".repeat(32)),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        // Corps surdimensionné : refusé avant tout décodage.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", " ".repeat(MAX_HANDSHAKE_JSON_LEN + 1)),
        )
        .await;
        assert!(st.is_client_error());
        assert!(state.e2e_sessions.is_empty());
    }

    #[tokio::test]
    async fn e2e_handshake_tampered_offer_refused() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0003-abcdef");
        let cog_key = state.cog_identity.public_key_bytes();
        // Clé éphémère substituée (signature invalide).
        let (_p, mut offer) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        let (_q, other) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        offer.ephemeral_key = other.ephemeral_key;
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Rôle usurpé (offre « COG »).
        let (_p, mut offer) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        offer.role = E2eRole::Cog;
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Offre signée pour un autre COG.
        let other_cog = E2eIdentity::generate();
        let (_p, offer) =
            PendingHandshake::initiate(E2eRole::Device, &device, &other_cog.public_key_bytes())
                .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(state.e2e_sessions.is_empty());
    }

    fn commit_body(device_id: &str, key_hex: &str, commitment_hex: &str, sig_hex: &str) -> String {
        serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION,
            "device_id": device_id,
            "device_name": "Pixel",
            "mode": "whitelist",
            "e2e_identity_key": key_hex,
            "commitment": commitment_hex,
            "commit_signature": sig_hex,
        })
        .to_string()
    }

    fn reveal_body(device_id: &str, nonce_hex: &str, sig_hex: &str) -> String {
        serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION,
            "device_id": device_id,
            "device_nonce": nonce_hex,
            "reveal_signature": sig_hex,
        })
        .to_string()
    }

    /// Corps d'engagement signé honnête pour `device`.
    fn signed_commit_body(device_id: &str, device: &E2eIdentity, pairing: &DevicePairing) -> String {
        commit_body(
            device_id,
            &hex::encode(device.public_key_bytes()),
            &pairing.commitment_hex(),
            &pairing.commit_signature_hex(device, device_id).unwrap(),
        )
    }

    /// Engagement HTTP signé ; retourne l'aléa du COG.
    async fn http_commit(
        state: &ServerState,
        peer: &str,
        device_id: &str,
        device: &E2eIdentity,
        pairing: &DevicePairing,
    ) -> PairingNonce {
        let (st, body) = call(
            app_for(state.clone(), peer),
            post_json(PAIR_ENDPOINT, signed_commit_body(device_id, device, pairing)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let resp: PairCommitResponse = serde_json::from_str(&body).unwrap();
        PairingNonce::from_hex(&resp.cog_nonce).unwrap()
    }

    async fn pair_status(state: &ServerState, device_id: &str, key_hex: &str) -> String {
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_STATUS_ENDPOINT,
                serde_json::json!({"device_id": device_id, "e2e_identity_key": key_hex}).to_string(),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        v["status"].as_str().unwrap().to_string()
    }


    #[tokio::test]
    async fn http_pairing_v2_end_to_end() {
        let (state, _tx, _) = test_state();
        let id = "device-0004-abcdef";
        let device = E2eIdentity::generate();
        let key = device.public_key_bytes();
        let key_hex = hex::encode(key);
        let pairing = DevicePairing::start(key);

        // 1. Engagement : la réponse porte la clé et l'aléa du COG, aucun code.
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &pairing)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let resp: PairCommitResponse = serde_json::from_str(&body).unwrap();
        assert_eq!(resp.status, PairingStatus::Committed);
        assert_eq!(resp.pairing_version, PAIRING_PROTOCOL_VERSION);
        assert!(!body.contains("confirmation_code"));
        let cog_key = parse_identity_key_hex(&resp.cog_identity_key).unwrap();
        assert_eq!(cog_key, state.cog_identity.public_key_bytes());
        assert_eq!(pair_status(&state, id, &key_hex).await, "committed");

        // 3. Révélation.
        let nc = PairingNonce::from_hex(&resp.cog_nonce).unwrap();
        let revealed = pairing.receive_cog_nonce(cog_key, nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        assert_eq!(pair_status(&state, id, &key_hex).await, "revealed");
        // Seconde révélation refusée.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);

        // 4. Central affiche le même code que le téléphone.
        let pending = state.pairing.pending_devices();
        assert_eq!(pending[0].confirmation_code.as_deref(), Some(revealed.code()));
        assert!(!state.pairing.is_whitelisted(id), "jamais d'approbation implicite");

        // Approbation (Central, dans le processus) : mauvais code refusé, bon code accepté.
        let wrong = if revealed.code() == "00000000" { "11111111" } else { "00000000" };
        assert_eq!(state.pairing.approve_device(id, wrong), Err(PairingError::CodeMismatch));
        assert!(!state.pairing.is_whitelisted(id));
        state.pairing.approve_device(id, revealed.code()).unwrap();
        assert_eq!(pair_status(&state, id, &key_hex).await, "approved");
        let paired = state.pairing.device_for_identity_key(&key_hex).unwrap();
        assert_eq!(paired.device_id, id);
        // La clé épinglée permet le handshake E2E.
        let _session = handshake(&state, &device).await;
    }


    #[tokio::test]
    async fn http_old_pairing_protocol_refused() {
        let (state, _tx, _) = test_state();
        let key_hex = hex::encode(E2eIdentity::generate().public_key_bytes());
        // Requête v1 (sans version ni engagement).
        let v1 = serde_json::json!({
            "device_id": "d-000000001", "device_name": "x",
            "mode": "whitelist", "e2e_identity_key": key_hex,
        });
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, v1.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_UNSUPPORTED_VERSION), "{body}");
        // Version future.
        let mut v3: serde_json::Value =
            serde_json::from_str(&commit_body("d-000000001", &key_hex, &"00".repeat(32), &"00".repeat(64))).unwrap();
        v3["pairing_version"] = 3.into();
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, v3.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_UNSUPPORTED_VERSION));
        // Révélation v1 refusée aussi.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_REVEAL_ENDPOINT,
                serde_json::json!({"device_id": "d-000000001", "device_nonce": "00".repeat(32)}).to_string(),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }

    #[tokio::test]
    async fn http_reveal_wrong_nonce_or_without_commit_refused() {
        let (state, _tx, _) = test_state();
        // Révélation sans engagement préalable (aucun Nc émis).
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_REVEAL_ENDPOINT,
                reveal_body("device-0010-abcdef", &"11".repeat(32), &"00".repeat(64)),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::NOT_FOUND, "{body}");
        // Mauvais Nd valablement signé par Kd : appairage abandonné.
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0011-abcdef";
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        let wrong_nd = PairingNonce::from_bytes([0x22; 32]);
        let sig = miyukini_cog_bridge::sign_pairing_reveal(&device, id, &wrong_nd, &nc).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &"22".repeat(32), &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN, "{body}");
        assert!(body.contains("commitment_mismatch"));
        assert_eq!(state.pairing.pending_devices().len(), 0);
        assert_eq!(pair_status(&state, "device-0011-abcdef", &key_hex).await, "rejected");
    }

    #[tokio::test]
    async fn http_pending_overwrite_and_per_ip_cap_refused() {
        let (state, _tx, _) = test_state();
        let commit = |id: &str| {
            let device = E2eIdentity::generate();
            signed_commit_body(id, &device, &DevicePairing::start(device.public_key_bytes()))
        };
        let (st, _) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, commit("device-0020-abcdef"))).await;
        assert_eq!(st, StatusCode::OK);
        // Autre clé pour le même appareil, depuis une autre IP : conflit.
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_ENDPOINT, commit("device-0020-abcdef")),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);
        assert!(body.contains("pending_conflict"));
        // Plafond par IP (3) : la 4e demande depuis la même IP est refusée.
        for i in 1..3 {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit(&format!("device-002{i}-abcdef"))),
            )
            .await;
            assert_eq!(st, StatusCode::OK);
        }
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, commit("device-0029-abcdef"))).await;
        assert_eq!(st, StatusCode::TOO_MANY_REQUESTS);
        assert!(body.contains("too_many_pending_for_source"));
        assert_eq!(state.pairing.pending_devices().len(), 3);
    }

    #[tokio::test]
    async fn http_unsigned_or_wrongly_signed_commit_refused() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0030-abcdef";
        // Requête v2 non signée (format antérieur) : refus explicite, sans repli.
        let unsigned = serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION, "device_id": id, "device_name": "Pixel",
            "mode": "whitelist", "e2e_identity_key": key_hex, "commitment": pairing.commitment_hex(),
        });
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, unsigned.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_MISSING_SIGNATURE), "{body}");
        // Signature mal formée.
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, commit_body(id, &key_hex, &pairing.commitment_hex(), "zz")),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        // Signature d'une autre clé.
        let other = E2eIdentity::generate();
        let c = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        let sig = miyukini_cog_bridge::sign_pairing_commit(&other, id, &c).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, commit_body(id, &key_hex, &pairing.commitment_hex(), &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }

    #[tokio::test]
    async fn http_bad_reveal_leaves_pending_entry_intact() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0031-abcdef";
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        let wrong = "33".repeat(32);
        // 1. Révélation non signée (v2 antérieure) : refus explicite.
        let unsigned = serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION, "device_id": id, "device_nonce": wrong,
        });
        let (st, body) =
            call(app_for(state.clone(), PEER_LAN), post_json(PAIR_REVEAL_ENDPOINT, unsigned.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_MISSING_SIGNATURE), "{body}");
        // 2. Signée par un tiers qui connaît device_id.
        let attacker = E2eIdentity::generate();
        let wrong_nd = PairingNonce::from_hex(&wrong).unwrap();
        let sig = miyukini_cog_bridge::sign_pairing_reveal(&attacker, id, &wrong_nd, &nc).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &wrong, &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        // 3. Révélation honnête rejouée depuis une autre adresse.
        let revealed = pairing.receive_cog_nonce(state.cog_identity.public_key_bytes(), nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(body.contains(STATUS_SOURCE_MISMATCH), "{body}");
        // L'entrée est intacte : la révélation légitime aboutit.
        assert_eq!(pair_status(&state, id, &key_hex).await, "committed");
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        assert_eq!(pair_status(&state, id, &key_hex).await, "revealed");
    }

    #[tokio::test]
    async fn http_same_key_recommit_after_reveal_or_other_source_refused() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let id = "device-0032-abcdef";
        let pairing = DevicePairing::start(device.public_key_bytes());
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        // Relance (nouvel Nd) depuis une autre source : refusée.
        let retry = DevicePairing::start(device.public_key_bytes());
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &retry)),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(body.contains(STATUS_SOURCE_MISMATCH), "{body}");
        // Révélation, puis relance depuis la même source : refusée (PendingConflict).
        let revealed = pairing.receive_cog_nonce(state.cog_identity.public_key_bytes(), nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &retry)),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);
        assert!(body.contains("pending_conflict"), "{body}");
        assert_eq!(
            state.pairing.pending_devices()[0].confirmation_code.as_deref(),
            Some(revealed.code())
        );
    }

    #[tokio::test]
    async fn pair_without_valid_key_or_commitment_refused() {
        let (state, _tx, _) = test_state();
        // Sans clé : rejet par le désérialiseur.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_ENDPOINT,
                serde_json::json!({
                    "pairing_version": PAIRING_PROTOCOL_VERSION,
                    "device_id": "d-000000001", "device_name": "x", "mode": "whitelist",
                    "commitment": "00".repeat(32),
                })
                .to_string(),
            ),
        )
        .await;
        assert!(st.is_client_error());
        // Clé faible / mal formée.
        let mut weak = [0u8; 32];
        weak[0] = 1;
        for key in [hex::encode(weak), "zz".into(), "00".repeat(31)] {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit_body("d-000000001", &key, &"00".repeat(32), &"00".repeat(64))),
            )
            .await;
            assert_eq!(st, StatusCode::BAD_REQUEST);
        }
        // Engagement mal formé.
        let key_hex = hex::encode(E2eIdentity::generate().public_key_bytes());
        for c in ["", "zz", &"00".repeat(31)] {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit_body("d-000000001", &key_hex, c, &"00".repeat(64))),
            )
            .await;
            assert_eq!(st, StatusCode::BAD_REQUEST);
        }
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }


    /// Bout en bout sur un vrai serveur : appairage → client de l'appareil
    /// (`miyukini_cog_bridge::ClientAppareil`) → passeport → visa → validation →
    /// WebSocket authentifié ; révocation ⇒ fermeture du WebSocket et refus.
    #[tokio::test]
    async fn real_server_client_flow_and_ws_closed_on_revocation() {
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        let dir = std::env::temp_dir().join(format!("central-remote-e2e-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = RemoteServerConfig {
            listen_addr: "127.0.0.1:0".into(),
            auth_db_path: dir.join("central.db").to_string_lossy().to_string(),
            ..RemoteServerConfig::default()
        };
        let h = start_remote_server(config, RemoteBridge::new()).await.unwrap();
        assert!(dir.join(FICHIER_CLE_EMISSION_DEFAUT).exists());
        let device = E2eIdentity::generate();
        let cog_key = parse_identity_key_hex(h.e2e_identity_public_key_hex()).unwrap();
        let code = direct_pair(&h.pairing, cog_key, &device, "device-real-0001", "127.0.0.1");
        h.pairing.approve_device("device-real-0001", &code).unwrap();

        let base = format!("http://{}", h.addr);
        let client = miyukini_cog_bridge::ClientAppareil::new(base, &device);
        let v = client.ouvrir_session().await.unwrap();
        assert_eq!(v.display_name, "Pixel");
        let r = client.valider().await.unwrap();
        assert_eq!(r.identite, v.identite);

        let mut req = format!("ws://{}{WS_ENDPOINT}", h.addr).into_client_request().unwrap();
        for (k, val) in client.entetes_ws().await.unwrap() {
            req.headers_mut().insert(k, val.parse().unwrap());
        }
        let (mut ws, _) = tokio_tungstenite::connect_async(req).await.unwrap();
        let first = ws.next().await.unwrap().unwrap();
        assert!(first.to_text().unwrap().contains("snapshot"), "{first:?}");

        // Sans en-têtes : upgrade refusé.
        let nu = format!("ws://{}{WS_ENDPOINT}", h.addr).into_client_request().unwrap();
        assert!(tokio_tungstenite::connect_async(nu).await.is_err());

        // Révocation : la session ouverte est fermée à la revérification suivante.
        assert_eq!(h.pairing.revoke_device("device-real-0001"), Ok(true));
        let ferme = tokio::time::timeout(Duration::from_secs(8), async {
            while let Some(m) = ws.next().await {
                match m {
                    Ok(tokio_tungstenite::tungstenite::Message::Close(_)) | Err(_) => return true,
                    Ok(_) => {}
                }
            }
            true
        })
        .await
        .unwrap_or(false);
        assert!(ferme, "WebSocket non fermé après révocation");
        assert!(client.valider().await.is_err());
    }

    #[tokio::test]
    async fn server_start_creates_persistent_identity() {
        let dir = std::env::temp_dir().join(format!("central-remote-srv-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = RemoteServerConfig {
            listen_addr: "127.0.0.1:0".into(),
            auth_db_path: dir.join("central.db").to_string_lossy().to_string(),
            ..RemoteServerConfig::default()
        };
        let h1 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        let k1 = h1.e2e_identity_public_key_hex().to_string();
        drop(h1);
        assert!(dir.join(DEFAULT_IDENTITY_FILE).exists());
        let h2 = start_remote_server(config, RemoteBridge::new()).await.unwrap();
        assert_eq!(h2.e2e_identity_public_key_hex(), k1);
        assert_eq!(h2.e2e_identity_fingerprint().split(' ').count(), 8);
    }

    #[tokio::test]
    async fn server_whitelist_persists_and_corruption_blocks_start() {
        let dir = std::env::temp_dir().join(format!("central-remote-wl-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = RemoteServerConfig {
            listen_addr: "127.0.0.1:0".into(),
            auth_db_path: dir.join("central.db").to_string_lossy().to_string(),
            ..RemoteServerConfig::default()
        };
        let h1 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        let device = E2eIdentity::generate();
        let key = device.public_key_bytes();
        let cog_key = parse_identity_key_hex(h1.e2e_identity_public_key_hex()).unwrap();
        let code = direct_pair(&h1.pairing, cog_key, &device, "device-persist-01", "10.0.0.5");
        h1.pairing.approve_device("device-persist-01", &code).unwrap();
        drop(h1);

        let file = dir.join(DEFAULT_PAIRED_DEVICES_FILE);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&file).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        let h2 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        assert!(h2.pairing.is_paired_with_key("device-persist-01", &hex::encode(key)));
        drop(h2);

        // Fichier corrompu : refus de démarrer (fail-closed), message explicite.
        std::fs::write(&file, b"{corrompu").unwrap();
        let Err(err) = start_remote_server(config, RemoteBridge::new()).await else {
            panic!("démarrage accepté avec une liste blanche corrompue");
        };
        assert!(err.contains("liste blanche"), "{err}");
    }
}
