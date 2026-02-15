//! Serveur HTTP d'administration MiyukiniAdmin Origin.
//!
//! Expose les endpoints d'administration sur le port 8081.
//! Toutes les routes (sauf /login) nécessitent une authentification par session.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use crate::config::OriginConfig;
use crate::relay::session::SessionManager;
use crate::tracker::pool::PoolManager;

/// Configuration d'authentification admin (chargée depuis admin.toml).
#[derive(Debug, Clone)]
pub struct AdminAuthConfig {
    /// Email attendu (affiché sur la page de login).
    pub email: String,
    /// Hash Argon2id du mot de passe.
    pub password_hash: String,
    /// Durée de validité de la session (secondes).
    pub session_ttl_seconds: u64,
}

/// État d'authentification admin : config + sessions actives.
struct AdminAuthState {
    config: Option<AdminAuthConfig>,
    /// token -> expiration
    sessions: Mutex<HashMap<String, Instant>>,
}

/// Serveur d'administration.
pub struct AdminServer {
    /// Configuration.
    config: Arc<OriginConfig>,
    /// Gestionnaire de sessions relay.
    sessions: Option<Arc<SessionManager>>,
    /// Gestionnaire de pools tracker.
    pools: Option<Arc<PoolManager>>,
    /// État d'authentification (config + sessions).
    auth_state: Arc<AdminAuthState>,
}

impl AdminServer {
    /// Crée un nouveau serveur admin.
    #[must_use]
    pub fn new(config: Arc<OriginConfig>) -> Self {
        let auth_state = Arc::new(AdminAuthState {
            config: Self::load_admin_auth_config(&config),
            sessions: Mutex::new(HashMap::new()),
        });
        Self {
            config,
            sessions: None,
            pools: None,
            auth_state,
        }
    }

    /// Charge la config auth depuis admin.toml (config_file).
    fn load_admin_auth_config(config: &OriginConfig) -> Option<AdminAuthConfig> {
        let path = config.admin.config_file.as_deref()?;
        let content = std::fs::read_to_string(path).ok()?;
        #[derive(serde::Deserialize)]
        struct AdminToml {
            admin: Option<AdminSection>,
            session: Option<SessionSection>,
        }
        #[derive(serde::Deserialize)]
        struct AdminSection {
            email: Option<String>,
            password_hash: Option<String>,
        }
        #[derive(serde::Deserialize)]
        struct SessionSection {
            session_ttl_seconds: Option<u64>,
        }
        let t: AdminToml = toml::from_str(&content).ok()?;
        let admin = t.admin?;
        let email = admin.email.unwrap_or_else(|| "admin".to_string());
        let password_hash = admin.password_hash?;
        let session_ttl_seconds = t
            .session
            .and_then(|s| s.session_ttl_seconds)
            .unwrap_or(14400);
        Some(AdminAuthConfig {
            email,
            password_hash,
            session_ttl_seconds,
        })
    }

    /// Définit le gestionnaire de sessions.
    #[allow(dead_code)]
    pub fn with_sessions(mut self, sessions: Arc<SessionManager>) -> Self {
        self.sessions = Some(sessions);
        self
    }

    /// Définit le gestionnaire de pools.
    pub fn with_pools(mut self, pools: Arc<PoolManager>) -> Self {
        self.pools = Some(pools);
        self
    }

    /// Démarre le serveur admin.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = format!("{}:{}", self.config.admin.host, self.config.admin.port)
            .parse()?;

        let listener = TcpListener::bind(addr).await?;
        info!("Admin server listening on {}", addr);
        if self.auth_state.config.is_none() {
            warn!("Admin auth config not loaded; /admin will require login but login may fail until admin.toml is configured");
        }

        loop {
            let (mut stream, peer_addr) = listener.accept().await?;
            debug!("Admin connection from {}", peer_addr);

            let config = Arc::clone(&self.config);
            let sessions = self.sessions.clone();
            let pools = self.pools.clone();
            let auth_state = Arc::clone(&self.auth_state);

            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let request = String::from_utf8_lossy(&buf[..n]).to_string();
                        let response =
                            Self::handle_request(&request, &config, sessions.as_ref(), pools.as_ref(), &auth_state).await;
                        let _ = stream.write_all(response.as_bytes()).await;
                    }
                    _ => {}
                }
            });
        }
    }

    /// Parse requête HTTP : méthode, chemin, cookie admin_session, body.
    fn parse_request(request: &str) -> (String, String, Option<String>, String) {
        let mut method = "GET".to_string();
        let mut path = "/".to_string();
        let mut cookie_session: Option<String> = None;
        let mut body = String::new();

        if let Some(first) = request.lines().next() {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() >= 2 {
                method = parts[0].to_string();
                path = parts[1].to_string();
            }
        }

        let in_headers = true;
        for line in request.lines().skip(1) {
            if line.is_empty() {
                if let Some(pos) = request.find("\r\n\r\n") {
                    body = request[pos + 4..].to_string();
                }
                break;
            }
            if in_headers && line.to_lowercase().starts_with("cookie:") {
                let rest = line[7..].trim();
                for part in rest.split(';') {
                    let part = part.trim();
                    if part.starts_with("admin_session=") {
                        cookie_session = Some(part[13..].trim().to_string());
                        break;
                    }
                }
            }
        }

        (method, path, cookie_session, body)
    }

    /// Vérifie que le token de session est valide (présent et non expiré).
    async fn is_session_valid(auth_state: &AdminAuthState, token: Option<&str>) -> bool {
        let token = match token {
            Some(t) if !t.is_empty() => t,
            _ => return false,
        };
        let mut sessions = auth_state.sessions.lock().await;
        if let Some(expiry) = sessions.get(token) {
            if *expiry > Instant::now() {
                return true;
            }
            sessions.remove(token);
        }
        false
    }

    /// Réponse redirect vers /admin/login.
    fn redirect_to_login() -> String {
        "HTTP/1.1 302 Found\r\nLocation: /admin/login\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
    }

    /// Réponse redirect vers /admin avec Set-Cookie session.
    fn redirect_with_session(token: &str, ttl_secs: u64) -> String {
        format!(
            "HTTP/1.1 302 Found\r\nLocation: /admin/\r\nSet-Cookie: admin_session={}; HttpOnly; Path=/admin; Max-Age={}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            token, ttl_secs
        )
    }

    /// Réponse redirect vers /admin/login avec cookie supprimé (logout).
    fn redirect_logout() -> String {
        "HTTP/1.1 302 Found\r\nLocation: /admin/login\r\nSet-Cookie: admin_session=; HttpOnly; Path=/admin; Max-Age=0\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
    }

    /// Gère une requête.
    async fn handle_request(
        request: &str,
        config: &OriginConfig,
        sessions: Option<&Arc<SessionManager>>,
        pools: Option<&Arc<PoolManager>>,
        auth_state: &Arc<AdminAuthState>,
    ) -> String {
        let (method, path, cookie_session, body) = Self::parse_request(request);

        let path_normalized = if path.starts_with("/admin") {
            path.strip_prefix("/admin").unwrap_or("/").to_string()
        } else {
            path.clone()
        };
        let path_norm = path_normalized.as_str();

        // Routes publiques : login (GET = formulaire, POST = traitement)
        if path_norm == "/login" || path_norm == "login" {
            if method == "GET" {
                return Self::login_form_html(auth_state).await;
            }
            if method == "POST" {
                return Self::login_post(config, body, auth_state).await;
            }
        }

        // Logout
        if (path_norm == "/logout" || path_norm == "logout") && method == "POST" {
            if let Some(ref t) = cookie_session {
                let mut sessions_map = auth_state.sessions.lock().await;
                sessions_map.remove(t.as_str());
            }
            return Self::redirect_logout();
        }

        // Toutes les autres routes : authentification requise
        if !Self::is_session_valid(auth_state, cookie_session.as_deref()).await {
            return Self::redirect_to_login();
        }

        match (method.as_str(), path_norm) {
            ("GET", "/" | "") => Self::dashboard_html(config, sessions, pools).await,
            ("GET", "/api/status") => Self::api_status(config, sessions, pools).await,
            ("GET", "/api/sessions") => Self::api_sessions(sessions).await,
            ("GET", "/api/pools") => Self::api_pools(pools).await,
            ("GET", "/api/config") => Self::api_config(config),
            _ => Self::http_response(404, "Not Found", "Endpoint not found"),
        }
    }

    /// Page de login (formulaire).
    async fn login_form_html(auth_state: &AdminAuthState) -> String {
        let email_placeholder = auth_state
            .config
            .as_ref()
            .map(|c| c.email.as_str())
            .unwrap_or("admin@example.com");
        let email_placeholder_escaped = email_placeholder
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('<', "&lt;");
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MiyukiniAdmin Origin — Connexion</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Segoe UI', system-ui, sans-serif;
            background: #0c0a1d;
            color: #e8e4f0;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .login-box {{
            background: #1a1432;
            padding: 2rem;
            border-radius: 12px;
            border: 1px solid rgba(139, 92, 246, 0.3);
            width: 100%;
            max-width: 380px;
        }}
        h1 {{ font-size: 1.5rem; margin-bottom: 1.5rem; color: #8b5cf6; }}
        label {{ display: block; margin-bottom: 0.5rem; color: #9d95b0; font-size: 0.9rem; }}
        input {{
            width: 100%;
            padding: 0.75rem;
            margin-bottom: 1rem;
            border: 1px solid rgba(139, 92, 246, 0.3);
            border-radius: 8px;
            background: #0c0a1d;
            color: #e8e4f0;
        }}
        button {{
            width: 100%;
            padding: 0.75rem;
            background: #8b5cf6;
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 1rem;
            cursor: pointer;
        }}
        button:hover {{ background: #7c3aed; }}
        .error {{ color: #ef4444; font-size: 0.9rem; margin-top: 0.5rem; }}
    </style>
</head>
<body>
    <div class="login-box">
        <h1>MiyukiniAdmin Origin</h1>
        <form method="post" action="/admin/login">
            <label for="email">Email</label>
            <input type="email" id="email" name="email" placeholder="{}" required autocomplete="email">
            <label for="password">Mot de passe</label>
            <input type="password" id="password" name="password" required autocomplete="current-password">
            <button type="submit">Connexion</button>
        </form>
    </div>
</body>
</html>"#,
            email_placeholder_escaped
        );
        Self::http_response_html(200, "OK", &html)
    }

    /// Traitement POST /login : vérification mot de passe, création session.
    async fn login_post(
        _config: &OriginConfig,
        body: String,
        auth_state: &Arc<AdminAuthState>,
    ) -> String {
        let auth_config = match &auth_state.config {
            Some(c) => c,
            None => {
                return Self::http_response_html(403, "Forbidden", "<p>Admin non configuré (admin.toml manquant).</p>");
            }
        };

        let email = Self::form_value(&body, "email");
        let password = Self::form_value(&body, "password");

        if email.is_empty() || password.is_empty() {
            return Self::redirect_to_login();
        }

        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed_hash = match PasswordHash::new(&auth_config.password_hash) {
            Ok(h) => h,
            Err(_) => return Self::redirect_to_login(),
        };

        if argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Self::redirect_to_login();
        }

        let token: String = (0..32).map(|_| rand::random::<u8>()).map(|b| format!("{:02x}", b)).collect();
        let ttl = Duration::from_secs(auth_config.session_ttl_seconds);
        {
            let mut sessions = auth_state.sessions.lock().await;
            sessions.insert(token.clone(), Instant::now() + ttl);
        }
        Self::redirect_with_session(&token, auth_config.session_ttl_seconds)
    }

    /// Extrait une valeur d'un body application/x-www-form-urlencoded.
    fn form_value(body: &str, key: &str) -> String {
        let key_eq = format!("{}=", key);
        for part in body.split('&') {
            let part = part.trim();
            if part.starts_with(&key_eq) {
                let value = part[key_eq.len()..].trim();
                return urlencoding::decode(value).unwrap_or(std::borrow::Cow::Borrowed(value)).into_owned();
            }
        }
        String::new()
    }

    /// Dashboard HTML.
    async fn dashboard_html(
        config: &OriginConfig,
        sessions: Option<&Arc<SessionManager>>,
        pools: Option<&Arc<PoolManager>>,
    ) -> String {
        let session_count = if let Some(s) = sessions {
            s.count().await
        } else {
            0
        };

        let pool_count = if let Some(p) = pools {
            p.total_cog_count().await
        } else {
            0
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MiyukiniAdmin Origin</title>
    <style>
        :root {{
            --primary: #8b5cf6;
            --bg: #0c0a1d;
            --surface: #1a1432;
            --text: #e8e4f0;
            --muted: #9d95b0;
            --success: #10b981;
            --warning: #f59e0b;
            --error: #ef4444;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Segoe UI', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            min-height: 100vh;
        }}
        .header {{
            background: var(--surface);
            padding: 1rem 2rem;
            border-bottom: 1px solid rgba(139, 92, 246, 0.2);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .logout-btn {{
            padding: 0.5rem 1rem;
            background: transparent;
            border: 1px solid var(--muted);
            color: var(--muted);
            border-radius: 6px;
            cursor: pointer;
            font-size: 0.9rem;
            text-decoration: none;
        }}
        .logout-btn:hover {{ color: var(--text); border-color: var(--primary); }}
        .header h1 {{
            font-size: 1.5rem;
            background: linear-gradient(135deg, var(--primary), #c084fc);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .status-badge {{
            background: var(--success);
            color: white;
            padding: 0.25rem 0.75rem;
            border-radius: 1rem;
            font-size: 0.875rem;
        }}
        .container {{ max-width: 1400px; margin: 0 auto; padding: 2rem; }}
        .grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }}
        .card {{
            background: var(--surface);
            border-radius: 1rem;
            padding: 1.5rem;
            border: 1px solid rgba(139, 92, 246, 0.1);
        }}
        .card h2 {{
            font-size: 0.875rem;
            color: var(--muted);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 0.5rem;
        }}
        .card .value {{
            font-size: 2.5rem;
            font-weight: 600;
            color: var(--primary);
        }}
        .card .label {{ color: var(--muted); font-size: 0.875rem; }}
        .section {{ margin-bottom: 2rem; }}
        .section h3 {{
            font-size: 1.25rem;
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ padding: 0.75rem; text-align: left; }}
        th {{ color: var(--muted); font-weight: 500; border-bottom: 1px solid rgba(139, 92, 246, 0.2); }}
        td {{ border-bottom: 1px solid rgba(139, 92, 246, 0.1); }}
        .api-btn {{
            display: inline-block;
            background: var(--primary);
            color: white;
            padding: 0.5rem 1rem;
            border-radius: 0.5rem;
            text-decoration: none;
            font-size: 0.875rem;
            margin-right: 0.5rem;
            margin-bottom: 0.5rem;
        }}
        .api-btn:hover {{ background: #7c3aed; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🛡️ MiyukiniAdmin Origin</h1>
        <span>
            <span class="status-badge">● En ligne</span>
            <form method="post" action="/admin/logout" style="display:inline;margin-left:1rem">
                <button type="submit" class="logout-btn">Déconnexion</button>
            </form>
        </span>
    </div>
    <div class="container">
        <div class="grid">
            <div class="card">
                <h2>Sessions Relay</h2>
                <div class="value">{}</div>
                <div class="label">connexions actives</div>
            </div>
            <div class="card">
                <h2>COGs dans les Pools</h2>
                <div class="value">{}</div>
                <div class="label">participants</div>
            </div>
            <div class="card">
                <h2>Port Relay</h2>
                <div class="value">{}</div>
                <div class="label">TCP + TLS</div>
            </div>
            <div class="card">
                <h2>Port Tracker</h2>
                <div class="value">{}</div>
                <div class="label">TCP + TLS</div>
            </div>
        </div>

        <div class="section">
            <h3>📡 Configuration</h3>
            <div class="card">
                <table>
                    <tr><th>Paramètre</th><th>Valeur</th></tr>
                    <tr><td>Rôle</td><td>{}</td></tr>
                    <tr><td>IP publique</td><td>{}</td></tr>
                    <tr><td>Max connexions</td><td>{}</td></tr>
                    <tr><td>Timeout tunnel</td><td>{} s</td></tr>
                    <tr><td>Rate limit (register/min/IP)</td><td>{}</td></tr>
                    <tr><td>PoW activé</td><td>{}</td></tr>
                </table>
            </div>
        </div>

        <div class="section">
            <h3>🔗 API Endpoints</h3>
            <a href="/admin/api/status" class="api-btn">Status</a>
            <a href="/admin/api/sessions" class="api-btn">Sessions</a>
            <a href="/admin/api/pools" class="api-btn">Pools</a>
            <a href="/admin/api/config" class="api-btn">Config</a>
        </div>
    </div>
</body>
</html>"#,
            session_count,
            pool_count,
            config.relay.port,
            config.tracker.port,
            config.identity.role,
            config.identity.ip,
            config.limits.max_connections,
            config.limits.tunnel_timeout_seconds,
            config.rate_limits.register_per_minute_per_ip,
            if config.pow.enabled { "Oui" } else { "Non" },
        );

        Self::http_response_html(200, "OK", &html)
    }

    /// API status.
    async fn api_status(
        config: &OriginConfig,
        sessions: Option<&Arc<SessionManager>>,
        pools: Option<&Arc<PoolManager>>,
    ) -> String {
        let session_count = if let Some(s) = sessions {
            s.count().await
        } else {
            0
        };

        let pool_count = if let Some(p) = pools {
            p.total_cog_count().await
        } else {
            0
        };

        let pool_versions = if let Some(p) = pools {
            p.list_versions().await
        } else {
            Vec::new()
        };

        let json = serde_json::json!({
            "status": "online",
            "role": config.identity.role,
            "ip": config.identity.ip,
            "relay_port": config.relay.port,
            "tracker_port": config.tracker.port,
            "sessions": session_count,
            "cogs_in_pools": pool_count,
            "pool_versions": pool_versions,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        Self::http_response_json(200, "OK", &json)
    }

    /// API sessions.
    async fn api_sessions(sessions: Option<&Arc<SessionManager>>) -> String {
        let count = if let Some(s) = sessions {
            s.count().await
        } else {
            0
        };

        let json = serde_json::json!({
            "total": count,
            "note": "Detailed session list requires authentication",
        });

        Self::http_response_json(200, "OK", &json)
    }

    /// API pools.
    async fn api_pools(pools: Option<&Arc<PoolManager>>) -> String {
        let (total, versions) = if let Some(p) = pools {
            (p.total_cog_count().await, p.list_versions().await)
        } else {
            (0, Vec::new())
        };

        let json = serde_json::json!({
            "total_cogs": total,
            "versions": versions,
        });

        Self::http_response_json(200, "OK", &json)
    }

    /// API config (sanitized).
    fn api_config(config: &OriginConfig) -> String {
        let json = serde_json::json!({
            "identity": {
                "role": config.identity.role,
                "ip": config.identity.ip,
            },
            "relay": {
                "port": config.relay.port,
            },
            "tracker": {
                "port": config.tracker.port,
                "web_port": config.tracker.web_port,
            },
            "limits": {
                "max_connections": config.limits.max_connections,
                "heartbeat_interval": config.limits.heartbeat_interval_seconds,
                "tunnel_timeout": config.limits.tunnel_timeout_seconds,
            },
            "rate_limits": {
                "register_per_minute_per_ip": config.rate_limits.register_per_minute_per_ip,
                "connections_per_token": config.rate_limits.connections_per_token,
            },
            "pow": {
                "enabled": config.pow.enabled,
                "difficulty_normal": config.pow.difficulty_normal,
            },
        });

        Self::http_response_json(200, "OK", &json)
    }

    /// Réponse HTTP texte.
    fn http_response(status: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }

    /// Réponse HTTP HTML.
    fn http_response_html(status: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }

    /// Réponse HTTP JSON.
    fn http_response_json(status: u16, status_text: &str, json: &serde_json::Value) -> String {
        let body = serde_json::to_string_pretty(json).unwrap_or_default();
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }
}
