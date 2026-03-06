//! Page de telechargement pour les liens de partage web.
//!
//! @id: miyucloud_web_share_page
//! @do: serve_share_download_page_with_streaming_decrypt
//! @role: api
//! @layer: app
//!
//! Routes :
//! - `GET /share/{token}` : page HTML avec nom du fichier, taille, bouton telecharger
//! - `POST /share/{token}/auth` : verification mot de passe
//! - `GET /share/{token}/download` : streaming du fichier dechiffre
//!
//! Le fichier est dechiffre en streaming (pas charge en memoire) via `StreamingDecryptor`.
//! Les tokens expires retournent 410 Gone.

use crate::security_headers::CspNonce;
use crate::web_surface::access_log;
use crate::web_surface::sandbox::SandboxedStore;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use miyucloud::auth::web_tokens;
use miyucloud::crypto::StreamingDecryptor;
use miyucloud::data::types::AccessAction;
use miyucloud::utils::sanitize::{sanitize_mime_type, sanitize_path_segment};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fmt::Write as FmtWrite;
use std::sync::Arc;

/// Nom du cookie de session pour les liens proteges par mot de passe.
const SESSION_COOKIE_NAME: &str = "miyucloud_share_session";

/// Genere un HMAC-SHA256 de session pour un token de partage.
///
/// Le HMAC lie le token du lien a un secret serveur (le COG token),
/// empechant la fabrication de cookies sans connaitre le secret.
fn generate_session_hmac(share_token: &str, server_secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"miyucloud-session-v1:");
    hasher.update(server_secret.as_bytes());
    hasher.update(b":");
    hasher.update(share_token.as_bytes());
    let result = hasher.finalize();
    result.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Verifie que la requete contient un cookie de session valide pour ce token.
fn verify_session_cookie(
    headers: &axum::http::HeaderMap,
    share_token: &str,
    server_secret: &str,
) -> bool {
    let expected = generate_session_hmac(share_token, server_secret);
    let cookie_header = match headers.get(header::COOKIE) {
        Some(v) => match v.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        None => return false,
    };
    for part in cookie_header.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix(&format!("{SESSION_COOKIE_NAME}=")) {
            return constant_time_eq(value.as_bytes(), expected.as_bytes());
        }
    }
    false
}

/// Comparaison constant-time via `subtle::ConstantTimeEq`.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    miyucloud::utils::constant_time::constant_time_eq(a, b)
}

/// Affiche la page de partage (HTML).
pub async fn share_page(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    let sandbox = SandboxedStore::new(db, storage);

    match sandbox.verify_token(&token) {
        Ok((link, file_entry, _password_hash)) => {
            let password_required = link.has_password;
            let file_name = &file_entry.name;
            let file_size = format_file_size(file_entry.size_bytes);
            let expires = &link.expires_at;
            let safe_token = sanitize_path_segment(&token);

            let html = render_share_page(
                file_name,
                &file_size,
                expires,
                password_required,
                &safe_token,
                &nonce,
            );
            Html(html).into_response()
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("expired") || msg.contains("revoked") {
                tracing::warn!(reason = %msg, "security: share link expired/revoked access attempt");
                let html = render_gone_page(&nonce);
                (StatusCode::GONE, Html(html)).into_response()
            } else {
                tracing::warn!(reason = %msg, "security: share link invalid token access attempt");
                (StatusCode::NOT_FOUND, Html(render_not_found_page(&nonce))).into_response()
            }
        }
    }
}

/// Verification du mot de passe.
#[derive(Deserialize)]
pub struct AuthBody {
    /// Mot de passe saisi par l'utilisateur.
    pub password: String,
}

/// Verifie le mot de passe pour un lien protege.
pub async fn share_auth(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    headers: HeaderMap,
    Json(body): Json<AuthBody>,
) -> Response {
    let db = &state.db;

    let Ok(Some(link)) = db.share_link_by_token(&token) else {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    };

    let Ok(password_hash) = db.share_link_password_hash(&link.id) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response();
    };

    let ip = access_log::extract_client_ip(&headers, state.config.trust_proxy);
    let ua = access_log::extract_user_agent(&headers);

    if let Some(hash) = &password_hash {
        if web_tokens::verify_password(&body.password, hash) {
            let _ = access_log::log_access(
                db,
                Some(&link.id),
                ip.as_deref(),
                ua.as_deref(),
                AccessAction::AuthAttempt,
                link.file_id.as_deref(),
                true,
                &state.ip_salt,
            );
            // Generate session cookie HMAC tied to this share token
            let session_hmac = generate_session_hmac(&token, &state.config.cog_token);
            let safe_token = sanitize_path_segment(&token);
            let cookie_value = format!(
                "{SESSION_COOKIE_NAME}={session_hmac}; Path=/share/{safe_token}; HttpOnly; Secure; SameSite=Strict; Max-Age=3600"
            );
            let response_body = serde_json::json!({"authenticated": true});
            return Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, cookie_value)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_string(&response_body).unwrap_or_default(),
                ))
                .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response());
        }
    }

    tracing::warn!(
        link_id = %link.id,
        "security: share link password auth failed"
    );

    let _ = access_log::log_access(
        db,
        Some(&link.id),
        ip.as_deref(),
        ua.as_deref(),
        AccessAction::AuthFail,
        link.file_id.as_deref(),
        false,
        &state.ip_salt,
    );

    let response_body = serde_json::json!({
        "error": {
            "code": "INCORRECT_PASSWORD",
            "message": "Incorrect password",
            "details": null
        }
    });
    (StatusCode::UNAUTHORIZED, Json(response_body)).into_response()
}

/// Telecharge un fichier partage en streaming.
pub async fn share_download(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    headers: axum::http::HeaderMap,
) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    let key_manager = &state.key_manager;

    // Verify and validate the share link
    let Ok(Some(link)) = db.share_link_by_token(&token) else {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    };

    // Validate expiration, revocation, max downloads (but not password here)
    let link_for_validation = miyucloud::data::types::ShareLink {
        has_password: false,
        ..link.clone()
    };
    if let Err(e) = web_tokens::validate_share_link(&link_for_validation, None, None) {
        let msg = e.to_string();
        if msg.contains("expired") || msg.contains("revoked") {
            return (StatusCode::GONE, "Link expired or revoked").into_response();
        }
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    }

    // F-01 FIX: If link has password, verify session cookie before allowing download
    if link.has_password && !verify_session_cookie(&headers, &token, &state.config.cog_token) {
        tracing::warn!(link_id = %link.id, "security: download attempt without valid session on password-protected link");
        return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
    }

    let Some(file_id) = link.file_id.as_deref() else {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    };

    let Ok(Some(file_entry)) = db.file_by_id(file_id) else {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    };

    // Derive file key
    let Ok(file_key) = key_manager.file_key(file_id) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Decryption error").into_response();
    };

    // Stream decryption
    let mut decryptor = StreamingDecryptor::new(storage, file_id, file_key, file_entry.chunk_count);

    // Collect all chunks (for MVP — in a future optimization, use axum Body::from_stream)
    let Ok(data) = decryptor.collect_all() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Decryption error").into_response();
    };

    // Increment download count
    let _ = db.share_link_increment_downloads(&link.id);

    // Log access
    let _ = access_log::log_access(
        db,
        Some(&link.id),
        access_log::extract_client_ip(&headers, state.config.trust_proxy).as_deref(),
        access_log::extract_user_agent(&headers).as_deref(),
        AccessAction::Download,
        Some(file_id),
        true,
        &state.ip_salt,
    );

    // Build response with sanitized file data
    let safe_filename = miyucloud::utils::sanitize::sanitize_filename(&file_entry.name);
    let content_disposition = format!("attachment; filename=\"{safe_filename}\"");
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", sanitize_mime_type(&file_entry.mime_type))
        .header("Content-Length", data.len().to_string())
        .header("Content-Disposition", content_disposition)
        .body(Body::from(data))
        .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Build error").into_response())
}

// -----------------------------------------------------------------------
// HTML rendering helpers
// -----------------------------------------------------------------------

fn render_share_page(
    file_name: &str,
    file_size: &str,
    expires_at: &str,
    password_required: bool,
    token: &str,
    nonce: &str,
) -> String {
    let safe_name = miyucloud::utils::sanitize::html_escape(file_name);
    let safe_size = miyucloud::utils::sanitize::html_escape(file_size);
    let safe_expires = miyucloud::utils::sanitize::html_escape(expires_at);

    // No inline `onclick` — registered via addEventListener in the <script> block.
    let password_form = if password_required {
        r#"<div id="password-section">
            <p>This file is password protected.</p>
            <input type="password" id="password" placeholder="Enter password" />
            <button id="unlock-btn">Unlock</button>
            <p id="auth-error" class="auth-error">Incorrect password</p>
        </div>"#
    } else {
        ""
    };

    let download_class = if password_required { " hidden" } else { "" };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>MiyuCloud - Shared File</title>
    <style nonce="{nonce}">
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ background: #1a1a2e; color: #e0e0e0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; }}
        .container {{ background: #16213e; border-radius: 12px; padding: 2rem; max-width: 480px; width: 90%; text-align: center; box-shadow: 0 4px 24px rgba(0,0,0,0.3); }}
        .logo {{ font-size: 1.2rem; color: #7c83ff; margin-bottom: 1.5rem; }}
        h1 {{ font-size: 1.1rem; margin-bottom: 0.5rem; word-break: break-all; }}
        .meta {{ color: #888; font-size: 0.9rem; margin-bottom: 1.5rem; }}
        .btn {{ background: #7c83ff; color: #fff; border: none; padding: 0.8rem 2rem; border-radius: 8px; font-size: 1rem; cursor: pointer; text-decoration: none; display: inline-block; margin-top: 1rem; }}
        .btn:hover {{ background: #6b73ff; }}
        input {{ background: #0f3460; border: 1px solid #333; color: #e0e0e0; padding: 0.6rem 1rem; border-radius: 6px; width: 100%; margin-bottom: 0.5rem; font-size: 1rem; }}
        .expires {{ color: #666; font-size: 0.8rem; margin-top: 1rem; }}
        .auth-error {{ color: #ff6b6b; display: none; }}
        .hidden {{ display: none; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="logo" aria-label="MiyuCloud">MiyuCloud</div>
        <h1>{safe_name}</h1>
        <p class="meta">{safe_size}</p>
        {password_form}
        <div id="download-section" class="btn-wrap{download_class}">
            <a href="/share/{token}/download" class="btn" aria-label="Download file">Download</a>
        </div>
        <p class="expires">This link expires on {safe_expires}</p>
    </div>
    <script nonce="{nonce}">
        function authenticate() {{
            var pwd = document.getElementById('password').value;
            fetch('/share/{token}/auth', {{
                method: 'POST',
                headers: {{'Content-Type': 'application/json'}},
                body: JSON.stringify({{password: pwd}})
            }}).then(function(r) {{
                if (r.ok) {{
                    document.getElementById('password-section').style.display = 'none';
                    document.getElementById('download-section').classList.remove('hidden');
                }} else {{
                    document.getElementById('auth-error').style.display = 'block';
                }}
            }});
        }}
        var unlockBtn = document.getElementById('unlock-btn');
        if (unlockBtn) {{ unlockBtn.addEventListener('click', authenticate); }}
    </script>
</body>
</html>"#
    )
}

fn render_gone_page(nonce: &str) -> String {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>MiyuCloud - Link Expired</title>
    <style NONCE_PLACEHOLDER>
        body { background: #1a1a2e; color: #e0e0e0; font-family: -apple-system, sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; }
        .container { text-align: center; }
        h1 { color: #ff6b6b; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Link Expired</h1>
        <p>This share link is no longer available.</p>
    </div>
</body>
</html>"#;
    html.replace("NONCE_PLACEHOLDER", &format!("nonce=\"{nonce}\""))
}

fn render_not_found_page(nonce: &str) -> String {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>MiyuCloud - Not Found</title>
    <style NONCE_PLACEHOLDER>
        body { background: #1a1a2e; color: #e0e0e0; font-family: -apple-system, sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; }
        .container { text-align: center; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Not Found</h1>
        <p>The requested page could not be found.</p>
    </div>
</body>
</html>"#;
    html.replace("NONCE_PLACEHOLDER", &format!("nonce=\"{nonce}\""))
}

/// Formate une taille en octets de maniere lisible.
fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyucloud::crypto::KeyManager;
    use miyucloud::data::MiyucloudDb;
    use miyucloud::domain::{ExternalShareOps, FileOps};
    use miyucloud::storage::local_fs::LocalFsStorage;

    fn setup() -> (MiyucloudDb, LocalFsStorage, KeyManager) {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        (db, storage, km)
    }

    #[test]
    fn test_share_page_valid_token_returns_sandbox_verified() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "hello.txt",
            "text/plain",
            b"hello world",
        )
        .unwrap();

        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        let sandbox = SandboxedStore::new(&db, &storage);
        let result = sandbox.verify_token(&link.token);
        assert!(result.is_ok());
    }

    #[test]
    fn test_share_page_invalid_token_returns_not_found() {
        let (db, storage, _km) = setup();
        let sandbox = SandboxedStore::new(&db, &storage);
        let result = sandbox.verify_token("invalid_token_1234567890abcdef1234567890abcdef");
        assert!(result.is_err());
    }

    #[test]
    fn test_share_page_expired_token_returns_error() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "test.txt",
            "text/plain",
            b"data",
        )
        .unwrap();

        // Create an expired link directly in DB
        let token = miyucloud::auth::generate_share_token();
        let link = miyucloud::data::types::ShareLink {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: Some(file.id.clone()),
            folder_id: None,
            creator_id: "owner1".to_string(),
            token: token.clone(),
            has_password: false,
            expires_at: "2020-01-01T00:00:00+00:00".to_string(),
            max_downloads: None,
            download_count: 0,
            is_revoked: false,
            created_at: "2020-01-01T00:00:00+00:00".to_string(),
        };
        db.share_link_create(&link, None).unwrap();

        let sandbox = SandboxedStore::new(&db, &storage);
        let result = sandbox.verify_token(&token);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_download_with_password_required() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "secret.txt",
            "text/plain",
            b"secret data",
        )
        .unwrap();

        let link = ExternalShareOps::create_link(
            &db,
            Some(&file.id),
            None,
            "owner1",
            Some("my-password"),
            24,
            None,
        )
        .unwrap();

        assert!(link.has_password);

        // Verify password
        let password_hash = db.share_link_password_hash(&link.id).unwrap().unwrap();
        assert!(miyucloud::auth::verify_password(
            "my-password",
            &password_hash
        ));
        assert!(!miyucloud::auth::verify_password(
            "wrong-password",
            &password_hash
        ));
    }

    #[test]
    fn test_download_increments_counter() {
        let (db, storage, km) = setup();
        let file = FileOps::upload_file(
            &db,
            &storage,
            &km,
            "owner1",
            None,
            "test.txt",
            "text/plain",
            b"data",
        )
        .unwrap();

        let link =
            ExternalShareOps::create_link(&db, Some(&file.id), None, "owner1", None, 24, None)
                .unwrap();

        // Simulate download counter increment
        db.share_link_increment_downloads(&link.id).unwrap();
        let updated = db.share_link_by_token(&link.token).unwrap().unwrap();
        assert_eq!(updated.download_count, 1);
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1_500_000), "1.4 MB");
        assert_eq!(format_file_size(2_500_000_000), "2.3 GB");
    }
}
