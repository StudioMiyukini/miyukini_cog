//! MiyuCloud Server -- Point d'entree.
//!
//! @id: miyucloud_server_main
//! @do: launch_dual_server_api_and_web_surface_with_crypto_bootstrap
//! @role: entrypoint
//! @layer: app
//!
//! Au demarrage :
//! 1. Ouvrir la DB (init schema si necessaire)
//! 2. Verifier si un sel Argon2 existe dans `cloud_crypto_config`
//! 3. Si non : generer le sel, deriver la master key, creer le canary
//! 4. Si oui : deriver la master key, verifier le canary
//! 5. Lancer le serveur dual :
//!    - API sur 127.0.0.1:{api_port} (HTTP, protege par X-COG-Token)
//!    - Surface web sur 0.0.0.0:{web_port} (HTTPS, publique, sandboxee)

mod api;
mod config;
mod security_headers;
mod web_surface;

use config::MiyucloudConfig;
use miyucloud::crypto::KeyManager;
use miyucloud::data::types::CryptoConfig;
use miyucloud::data::MiyucloudDb;
use miyucloud::storage::local_fs::LocalFsStorage;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;

/// Etat partage de l'application (accessible par tous les handlers).
pub struct AppState {
    /// Base de donnees MiyuCloud.
    pub db: MiyucloudDb,
    /// Backend de stockage.
    pub storage: LocalFsStorage,
    /// Gestionnaire de cles (master key en memoire).
    pub key_manager: KeyManager,
    /// Configuration du serveur.
    pub config: MiyucloudConfig,
    /// Sel derive HKDF pour le hashing RGPD des adresses IP.
    pub ip_salt: [u8; 32],
}

#[tokio::main]
async fn main() {
    // Init tracing
    tracing_subscriber::fmt::init();

    let config = MiyucloudConfig::from_env();
    tracing::info!("MiyuCloud Server starting on port {}", config.api_port);

    // Ensure directories exist
    if let Some(parent) = config.db_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::error!("Failed to create DB directory: {e}");
            std::process::exit(1);
        }
    }

    // 1. Open DB
    let db = match MiyucloudDb::open(&config.db_path) {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to open database: {e}");
            std::process::exit(1);
        }
    };

    // 2. Open storage
    let storage = match LocalFsStorage::new(config.storage_path.clone()) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to initialize storage: {e}");
            std::process::exit(1);
        }
    };

    // 3. Crypto bootstrap
    let key_manager = match bootstrap_crypto(&db, &config) {
        Ok(km) => km,
        Err(e) => {
            tracing::error!("Crypto bootstrap failed: {e}");
            std::process::exit(1);
        }
    };

    tracing::info!("Crypto bootstrap complete. Master key derived.");

    let ip_salt = crate::web_surface::access_log::derive_ip_salt(&config.cog_token);
    let state = Arc::new(AppState {
        db,
        storage,
        key_manager,
        config: config.clone(),
        ip_salt,
    });

    // --- API server (HTTP, localhost only) ---
    let api_app = api::api_router(state.clone());
    let api_addr = format!("127.0.0.1:{}", config.api_port);
    tracing::info!("API server listening on {api_addr}");
    let api_listener = match tokio::net::TcpListener::bind(&api_addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind API to {api_addr}: {e}");
            std::process::exit(1);
        }
    };

    // --- Web surface server (HTTPS, all interfaces) ---
    if config.web_enabled {
        let web_app = web_surface::web_surface_router(state.clone());
        let web_addr = format!("0.0.0.0:{}", config.web_port);

        // Resolve TLS config
        let base_dir = config
            .storage_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let tls_config = match (&config.tls_cert_path, &config.tls_key_path) {
            (Some(cert), Some(key)) => web_surface::tls::TlsConfig {
                cert_path: cert.clone(),
                key_path: key.clone(),
                auto_generate: false,
            },
            _ => web_surface::tls::TlsConfig::default_paths(base_dir),
        };

        let rustls_config = match tls_config.load_rustls_config() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("TLS setup failed: {e}");
                std::process::exit(1);
            }
        };
        let tls_acceptor = tokio_rustls::TlsAcceptor::from(rustls_config);

        tracing::info!("Web surface listening on {web_addr} (HTTPS)");

        let web_listener = match tokio::net::TcpListener::bind(&web_addr).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to bind web surface to {web_addr}: {e}");
                std::process::exit(1);
            }
        };

        // Spawn web surface server in a separate task
        tokio::spawn(serve_tls(web_listener, tls_acceptor, web_app));
    } else {
        tracing::info!("Web surface disabled by configuration");
    }

    // Run the API server on the main task
    if let Err(e) = axum::serve(api_listener, api_app).await {
        tracing::error!("API server error: {e}");
        std::process::exit(1);
    }
}

/// Serveur TLS acceptant les connexions HTTPS.
///
/// Chaque connexion TCP est upgraee en TLS via le `TlsAcceptor`,
/// puis servie par le routeur axum via `hyper_util`.
async fn serve_tls(
    listener: tokio::net::TcpListener,
    tls_acceptor: TlsAcceptor,
    app: axum::Router,
) {
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                tracing::warn!("Web surface accept error: {e}");
                continue;
            }
        };

        let acceptor = tls_acceptor.clone();
        let service = app.clone();

        tokio::spawn(handle_tls_connection(acceptor, stream, addr, service));
    }
}

/// Gere une connexion TLS individuelle.
///
/// Separe du `serve_tls` loop pour eviter les problemes de lifetime
/// avec `hyper_util::server::conn::auto::Builder::serve_connection`.
async fn handle_tls_connection(
    acceptor: TlsAcceptor,
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    app: axum::Router,
) {
    let tls_stream = match acceptor.accept(stream).await {
        Ok(s) => s,
        Err(e) => {
            tracing::debug!("TLS handshake failed from {addr}: {e}");
            return;
        }
    };

    let io = hyper_util::rt::TokioIo::new(tls_stream);

    // Use tower::Service directly — axum::Router implements Service<Request<Body>>
    // whose Error = Infallible, and we convert it to a hyper service.
    let service = tower::ServiceBuilder::new().service(app);
    let hyper_service =
        hyper::service::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
            // Clone service for each request
            let mut svc = service.clone();
            async move {
                use tower::Service;
                let (parts, body) = req.into_parts();
                let body = axum::body::Body::new(body);
                let req = axum::http::Request::from_parts(parts, body);
                let resp: Result<axum::response::Response, std::convert::Infallible> =
                    svc.call(req).await;
                // Convert Infallible error to the expected error type
                Ok::<_, std::convert::Infallible>(resp.expect("infallible"))
            }
        });

    if let Err(e) =
        hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
            .serve_connection(io, hyper_service)
            .await
    {
        tracing::debug!("Web surface connection error from {addr}: {e}");
    }
}

/// Bootstrap cryptographique au demarrage.
///
/// - Si aucune config crypto n'existe en DB : generer sel, deriver master key,
///   creer le canary, stocker en DB.
/// - Si config crypto existe : deriver master key depuis passphrase + sel,
///   verifier le canary.
fn bootstrap_crypto(
    db: &MiyucloudDb,
    config: &MiyucloudConfig,
) -> Result<KeyManager, miyucloud::errors::MiyucloudError> {
    let owner_id = &config.owner_id;
    let passphrase = match config.passphrase.as_deref() {
        Some(p) if !p.is_empty() => p,
        _ => {
            return Err(miyucloud::errors::MiyucloudError::Crypto(
                "MIYUCLOUD_PASSPHRASE environment variable is required. \
                 The server refuses to start without an explicit passphrase."
                    .into(),
            ));
        }
    };

    let existing = db.crypto_config_get(owner_id)?;

    if let Some(crypto_cfg) = existing {
        // Derive master key from existing salt
        tracing::info!("Existing crypto config found, deriving master key...");
        let salt_bytes = base64_decode(&crypto_cfg.argon2_salt)?;
        let master_key = KeyManager::derive_master_key(passphrase, &salt_bytes)?;

        // Verify canary if present
        if let (Some(canary_ct), Some(canary_nonce)) =
            (&crypto_cfg.canary_ciphertext, &crypto_cfg.canary_nonce)
        {
            let ct_bytes = base64_decode(canary_ct)?;
            let nonce_bytes = base64_decode(canary_nonce)?;
            if nonce_bytes.len() != 12 {
                return Err(miyucloud::errors::MiyucloudError::Crypto(
                    "Invalid canary nonce length".into(),
                ));
            }
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&nonce_bytes);
            if !KeyManager::verify_passphrase(&master_key, &ct_bytes, &nonce) {
                return Err(miyucloud::errors::MiyucloudError::Crypto(
                    "Wrong passphrase: canary verification failed".into(),
                ));
            }
            tracing::info!("Passphrase verified via canary.");
        }

        Ok(KeyManager::from_master_key(master_key))
    } else {
        // First start: generate salt, derive master key, create canary
        tracing::info!("First start: generating crypto config...");
        let salt = KeyManager::generate_salt();
        let master_key = KeyManager::derive_master_key(passphrase, &salt)?;
        let canary = KeyManager::create_canary(&master_key)?;

        let now = chrono::Utc::now().to_rfc3339();
        let crypto_cfg = CryptoConfig {
            owner_id: owner_id.clone(),
            argon2_salt: base64_encode(&salt),
            canary_ciphertext: Some(base64_encode(&canary.ciphertext)),
            canary_nonce: Some(base64_encode(&canary.nonce)),
            key_version: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        db.crypto_config_upsert(&crypto_cfg)?;

        tracing::info!("Crypto config stored in DB.");
        Ok(KeyManager::from_master_key(master_key))
    }
}

/// Encode bytes en base64.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = if chunk.len() > 1 {
            u32::from(chunk[1])
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            u32::from(chunk[2])
        } else {
            0
        };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Decode base64 en bytes.
fn base64_decode(input: &str) -> Result<Vec<u8>, miyucloud::errors::MiyucloudError> {
    fn char_val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some(u32::from(c - b'A')),
            b'a'..=b'z' => Some(u32::from(c - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(c - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => Some(0),
            _ => None,
        }
    }

    let bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r')
        .collect();
    if !bytes.len().is_multiple_of(4) {
        return Err(miyucloud::errors::MiyucloudError::Crypto(
            "Invalid base64 length".into(),
        ));
    }

    let mut result = Vec::new();
    for chunk in bytes.chunks(4) {
        let a = char_val(chunk[0]).ok_or_else(|| {
            miyucloud::errors::MiyucloudError::Crypto("Invalid base64 char".into())
        })?;
        let b = char_val(chunk[1]).ok_or_else(|| {
            miyucloud::errors::MiyucloudError::Crypto("Invalid base64 char".into())
        })?;
        let c_val = char_val(chunk[2]).ok_or_else(|| {
            miyucloud::errors::MiyucloudError::Crypto("Invalid base64 char".into())
        })?;
        let d = char_val(chunk[3]).ok_or_else(|| {
            miyucloud::errors::MiyucloudError::Crypto("Invalid base64 char".into())
        })?;
        let triple = (a << 18) | (b << 12) | (c_val << 6) | d;
        result.push((triple >> 16) as u8);
        if chunk[2] != b'=' {
            result.push((triple >> 8) as u8);
        }
        if chunk[3] != b'=' {
            result.push(triple as u8);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> MiyucloudConfig {
        MiyucloudConfig {
            api_port: 11440,
            storage_path: std::path::PathBuf::from("/tmp/test"),
            db_path: std::path::PathBuf::from(":memory:"),
            cog_token: "test-token".to_string(),
            passphrase: Some("test-passphrase".to_string()),
            owner_id: "test-owner".to_string(),
            web_port: 11442,
            web_enabled: false,
            trust_proxy: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }

    #[test]
    fn test_first_start_generates_salt_and_stores_in_db() {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let config = test_config();
        let km = bootstrap_crypto(&db, &config).unwrap();
        assert!(!km.master_key().iter().all(|&b| b == 0));

        // Verify config was stored
        let cfg = db.crypto_config_get("test-owner").unwrap();
        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert!(!cfg.argon2_salt.is_empty());
        assert!(cfg.canary_ciphertext.is_some());
        assert!(cfg.canary_nonce.is_some());
    }

    #[test]
    fn test_subsequent_start_reads_salt_from_db() {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let config = MiyucloudConfig {
            passphrase: Some("my-secure-pass".to_string()),
            owner_id: "test-owner-2".to_string(),
            ..test_config()
        };

        // First start
        let km1 = bootstrap_crypto(&db, &config).unwrap();

        // Second start (same passphrase)
        let km2 = bootstrap_crypto(&db, &config).unwrap();

        // Should derive the same master key
        assert_eq!(km1.master_key(), km2.master_key());
    }

    #[test]
    fn test_wrong_passphrase_rejects() {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let config1 = MiyucloudConfig {
            passphrase: Some("correct-passphrase".to_string()),
            owner_id: "test-owner-3".to_string(),
            ..test_config()
        };

        // First start with correct passphrase
        bootstrap_crypto(&db, &config1).unwrap();

        // Second start with wrong passphrase
        let config2 = MiyucloudConfig {
            passphrase: Some("wrong-passphrase".to_string()),
            ..config1
        };
        let result = bootstrap_crypto(&db, &config2);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Wrong passphrase"));
    }

    #[test]
    fn test_base64_roundtrip() {
        let data = b"Hello, World!";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_nonce() {
        let nonce = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let encoded = base64_encode(&nonce);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, nonce);
    }
}
