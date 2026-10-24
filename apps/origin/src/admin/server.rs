//! @id     origin_admin_server
//! @do     definir_max_login_attempts
//! @role   orchestration
//! @layer  app
//! @human  origin — module « admin server ».

//! Serveur HTTP d'administration MiyukiniAdmin Origin.
//!
//! Expose les endpoints d'administration sur le port 8081.
//! Toutes les routes (sauf /login) nécessitent une authentification par session.
//!
//! O-04 (cookie tossing, confiance « même site ») :
//! - cookie de session `__Host-admin_session` (`Secure`, `Path=/`, sans `Domain`, `HttpOnly`,
//!   `SameSite=Strict`) : un cookie posé par un sous-domaine COG sur le domaine parent ne
//!   peut ni le remplacer ni le masquer (le navigateur refuse un `__Host-` avec Domain) ;
//! - toute requête modifiante (méthode autre que GET/HEAD) exige un `Origin` (ou à défaut
//!   un `Referer`) égal à l'origine exacte de l'admin **et** un jeton CSRF : jeton lié à la
//!   session (champ `csrf` ou en-tête `X-CSRF-Token`), ou double soumission via le cookie
//!   `__Host-admin_csrf` pour le formulaire de connexion. Tout échec = 403 (fail-closed).

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

/// Nombre maximal de tentatives de login avant blocage temporaire.
const MAX_LOGIN_ATTEMPTS: u32 = 5;
/// Durée du blocage après trop de tentatives (secondes).
const LOGIN_BLOCK_DURATION_SECS: u64 = 300;
/// Nom du cookie de session admin (préfixe `__Host-` : hôte seul, Secure, Path=/).
const COOKIE_SESSION: &str = "__Host-admin_session";
/// Nom du cookie CSRF du formulaire de connexion (double soumission).
const COOKIE_CSRF_LOGIN: &str = "__Host-admin_csrf";
/// Durée de validité du jeton CSRF de connexion (secondes).
const CSRF_LOGIN_TTL_SECS: u64 = 600;
/// Réponse de refus CSRF (corps distinct, testable).
const CORPS_REFUS_CSRF: &str = "Forbidden: CSRF check failed";

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

/// Session admin : expiration et jeton CSRF lié à la session.
struct SessionAdmin {
    expiration: Instant,
    csrf: String,
}

/// Requête admin analysée.
#[derive(Debug, Default)]
struct RequeteAdmin {
    method: String,
    path: String,
    cookie_session: Option<String>,
    cookie_csrf: Option<String>,
    origin: Option<String>,
    referer: Option<String>,
    host: Option<String>,
    csrf_entete: Option<String>,
    body: String,
}

/// État d'authentification admin : config + sessions actives + rate limiting login.
struct AdminAuthState {
    config: Option<AdminAuthConfig>,
    /// token -> session (expiration, jeton CSRF)
    sessions: Mutex<HashMap<String, SessionAdmin>>,
    /// IP -> (tentatives, dernière tentative, bloqué_jusqu'à)
    login_attempts: Mutex<HashMap<std::net::IpAddr, (u32, Instant, Option<Instant>)>>,
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
            login_attempts: Mutex::new(HashMap::new()),
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
        let addr: SocketAddr =
            format!("{}:{}", self.config.admin.host, self.config.admin.port).parse()?;

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
                        let response = Self::handle_request(
                            &request,
                            &config,
                            sessions.as_ref(),
                            pools.as_ref(),
                            &auth_state,
                            peer_addr,
                        )
                        .await;
                        let _ = stream.write_all(response.as_bytes()).await;
                    }
                    _ => {}
                }
            });
        }
    }

    /// Parse requête HTTP : méthode, chemin, cookies admin, en-têtes CSRF, body.
    fn parse_request(request: &str) -> RequeteAdmin {
        let mut r = RequeteAdmin {
            method: "GET".to_string(),
            path: "/".to_string(),
            ..RequeteAdmin::default()
        };

        if let Some(first) = request.lines().next() {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() >= 2 {
                r.method = parts[0].to_string();
                r.path = parts[1].to_string();
            }
        }

        for line in request.lines().skip(1) {
            if line.is_empty() {
                if let Some(pos) = request.find("\r\n\r\n") {
                    r.body = request[pos + 4..].to_string();
                }
                break;
            }
            let Some((nom, valeur)) = line.split_once(':') else {
                continue;
            };
            let nom = nom.trim().to_ascii_lowercase();
            let valeur = valeur.trim();
            match nom.as_str() {
                "cookie" => {
                    if let Some(v) = Self::cookie_value(valeur, COOKIE_SESSION) {
                        r.cookie_session = Some(v);
                    }
                    if let Some(v) = Self::cookie_value(valeur, COOKIE_CSRF_LOGIN) {
                        r.cookie_csrf = Some(v);
                    }
                }
                "origin" => r.origin = Some(valeur.to_string()),
                "referer" => r.referer = Some(valeur.to_string()),
                "host" => r.host = Some(valeur.to_string()),
                "x-csrf-token" => r.csrf_entete = Some(valeur.to_string()),
                _ => {}
            }
        }

        r
    }

    /// Jeton aléatoire de 256 bits (hexadécimal), tiré de `OsRng` (C-24).
    fn jeton_aleatoire() -> String {
        let mut octets = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut octets);
        hex::encode(octets)
    }

    /// Comparaison de secrets en temps constant (C-22).
    fn egal_temps_constant(a: &str, b: &str) -> bool {
        if a.is_empty() || a.len() != b.len() {
            return false;
        }
        a.bytes().zip(b.bytes()).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
    }

    /// Origine exacte attendue pour l'admin : `admin.public_origin`, sinon `https://<Host>`.
    fn origine_attendue(config: &OriginConfig, host: Option<&str>) -> Option<String> {
        if let Some(o) = config.admin.public_origin.as_deref() {
            let o = o.trim().trim_end_matches('/');
            return (!o.is_empty()).then(|| o.to_ascii_lowercase());
        }
        let host = host?.trim();
        if host.is_empty() || host.contains(['/', ' ', '@']) {
            return None;
        }
        Some(format!("https://{}", host.to_ascii_lowercase()))
    }

    /// Origine (`schéma://autorité`) d'une URL de `Referer`.
    fn origine_de_referer(referer: &str) -> Option<String> {
        let (schema, reste) = referer.split_once("://")?;
        let autorite = reste.split(['/', '?', '#']).next()?;
        if schema.is_empty() || autorite.is_empty() || autorite.contains('@') {
            return None;
        }
        Some(format!("{schema}://{autorite}").to_ascii_lowercase())
    }

    /// `Origin` (ou à défaut `Referer`) égal à l'origine admin exacte ? Absent = refus.
    fn origine_valide(req: &RequeteAdmin, attendue: &str) -> bool {
        if let Some(origin) = req.origin.as_deref() {
            return origin.trim().trim_end_matches('/').eq_ignore_ascii_case(attendue);
        }
        req.referer
            .as_deref()
            .and_then(Self::origine_de_referer)
            .is_some_and(|o| o == attendue)
    }

    /// Jeton CSRF présenté : champ de formulaire `csrf`, sinon en-tête `X-CSRF-Token`.
    fn csrf_presente(req: &RequeteAdmin) -> String {
        let champ = Self::form_value(&req.body, "csrf");
        if champ.is_empty() {
            req.csrf_entete.clone().unwrap_or_default()
        } else {
            champ
        }
    }

    /// Réponse 403 pour un échec de contrôle CSRF.
    fn refus_csrf() -> String {
        Self::http_response(403, "Forbidden", CORPS_REFUS_CSRF)
    }

    /// Extrait la valeur d'un cookie nommé dans le contenu d'un en-tête `Cookie:`.
    fn cookie_value(header_rest: &str, name: &str) -> Option<String> {
        let prefix = format!("{name}=");
        for part in header_rest.trim().split(';') {
            if let Some(v) = part.trim().strip_prefix(prefix.as_str()) {
                return Some(v.trim().to_string());
            }
        }
        None
    }

    /// L'adresse du pair est-elle un proxy de confiance (nginx local) ?
    fn is_trusted_proxy(ip: std::net::IpAddr) -> bool {
        ip.to_canonical().is_loopback()
    }

    /// IP cliente pour le rate-limit : `X-Real-IP` lu UNIQUEMENT si le pair est
    /// loopback (proxy de confiance) ; sinon l'IP du pair (en-tête ignoré).
    fn client_ip(request: &str, peer: std::net::IpAddr) -> std::net::IpAddr {
        if !Self::is_trusted_proxy(peer) {
            return peer;
        }
        for line in request.lines().skip(1) {
            if line.is_empty() {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                if k.trim().eq_ignore_ascii_case("x-real-ip") {
                    if let Ok(ip) = v.trim().parse::<std::net::IpAddr>() {
                        return ip.to_canonical();
                    }
                    break;
                }
            }
        }
        peer
    }

    /// Une entrée de rate-limit bloque-t-elle encore à l'instant `now` ?
    /// Après expiration du blocage (ou de la fenêtre d'inactivité), l'IP est libérée.
    fn entry_blocks(entry: &(u32, Instant, Option<Instant>), now: Instant) -> bool {
        let (count, last, blocked_until) = entry;
        if let Some(until) = blocked_until {
            return now < *until;
        }
        *count >= MAX_LOGIN_ATTEMPTS
            && now.saturating_duration_since(*last) < Duration::from_secs(LOGIN_BLOCK_DURATION_SECS)
    }

    /// Vérifie si une IP est bloquée pour tentatives de login excessives.
    async fn check_login_rate_limit(auth_state: &AdminAuthState, ip: std::net::IpAddr) -> bool {
        let attempts = auth_state.login_attempts.lock().await;
        match attempts.get(&ip) {
            Some(entry) => !Self::entry_blocks(entry, Instant::now()),
            None => true,
        }
    }

    /// Enregistre une tentative de login échouée.
    async fn record_failed_login(auth_state: &AdminAuthState, ip: std::net::IpAddr) {
        let mut attempts = auth_state.login_attempts.lock().await;
        let now = Instant::now();
        let entry = attempts.entry(ip).or_insert((0, now, None));
        // Entrée dont le blocage/la fenêtre a expiré : on repart de zéro.
        let expired = match entry.2 {
            Some(until) => now >= until,
            None => {
                now.saturating_duration_since(entry.1)
                    >= Duration::from_secs(LOGIN_BLOCK_DURATION_SECS)
            }
        };
        if expired {
            *entry = (0, now, None);
        }
        entry.0 += 1;
        entry.1 = now;
        if entry.0 >= MAX_LOGIN_ATTEMPTS && entry.2.is_none() {
            entry.2 = Some(now + Duration::from_secs(LOGIN_BLOCK_DURATION_SECS));
            warn!(
                "Admin login blocked for IP {} ({} failed attempts)",
                ip, entry.0
            );
        }
    }

    /// Réinitialise les tentatives après un login réussi.
    async fn reset_login_attempts(auth_state: &AdminAuthState, ip: std::net::IpAddr) {
        let mut attempts = auth_state.login_attempts.lock().await;
        attempts.remove(&ip);
    }

    /// Jeton CSRF de la session si le token de session est valide (présent, non expiré).
    async fn session_csrf(auth_state: &AdminAuthState, token: Option<&str>) -> Option<String> {
        let token = match token {
            Some(t) if !t.is_empty() => t,
            _ => return None,
        };
        let mut sessions = auth_state.sessions.lock().await;
        if let Some(session) = sessions.get(token) {
            if session.expiration > Instant::now() {
                return Some(session.csrf.clone());
            }
            sessions.remove(token);
        }
        None
    }

    /// Réponse redirect vers /admin/login.
    fn redirect_to_login() -> String {
        "HTTP/1.1 302 Found\r\nLocation: /admin/login\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
    }

    /// En-tête `Set-Cookie` d'un cookie `__Host-` (Secure, Path=/, sans Domain).
    fn cookie_host(nom: &str, valeur: &str, max_age: u64) -> String {
        format!("Set-Cookie: {nom}={valeur}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age={max_age}\r\n")
    }

    /// Réponse redirect vers /admin avec Set-Cookie session ; le cookie CSRF de connexion
    /// est effacé.
    fn redirect_with_session(token: &str, ttl_secs: u64) -> String {
        format!(
            "HTTP/1.1 302 Found\r\nLocation: /admin/\r\n{}{}Content-Length: 0\r\nConnection: close\r\n\r\n",
            Self::cookie_host(COOKIE_SESSION, token, ttl_secs),
            Self::cookie_host(COOKIE_CSRF_LOGIN, "", 0),
        )
    }

    /// Réponse redirect vers /admin/login avec cookie supprimé (logout).
    fn redirect_logout() -> String {
        format!(
            "HTTP/1.1 302 Found\r\nLocation: /admin/login\r\n{}Content-Length: 0\r\nConnection: close\r\n\r\n",
            Self::cookie_host(COOKIE_SESSION, "", 0),
        )
    }

    /// Gère une requête.
    async fn handle_request(
        request: &str,
        config: &OriginConfig,
        sessions: Option<&Arc<SessionManager>>,
        pools: Option<&Arc<PoolManager>>,
        auth_state: &Arc<AdminAuthState>,
        peer_addr: SocketAddr,
    ) -> String {
        let req = Self::parse_request(request);
        let client_ip = Self::client_ip(request, peer_addr.ip());
        let method = req.method.as_str();

        let path_normalized = if req.path.starts_with("/admin") {
            req.path.strip_prefix("/admin").unwrap_or("/").to_string()
        } else {
            req.path.clone()
        };
        let path_norm = path_normalized.as_str();

        // Anti-CSRF (O-04) : toute requête modifiante exige l'origine admin exacte.
        let modifiante = !matches!(method, "GET" | "HEAD");
        if modifiante {
            let attendue = Self::origine_attendue(config, req.host.as_deref());
            if !attendue.is_some_and(|a| Self::origine_valide(&req, &a)) {
                warn!("Admin {} {} refused: missing or foreign Origin/Referer", method, path_norm);
                return Self::refus_csrf();
            }
        }

        // Routes publiques : login (GET = formulaire, POST = traitement)
        if path_norm == "/login" || path_norm == "login" {
            if method == "GET" {
                return Self::login_form_html(auth_state).await;
            }
            if method == "POST" {
                // Double soumission : champ `csrf` == cookie `__Host-admin_csrf`.
                let cookie = req.cookie_csrf.as_deref().unwrap_or("");
                if !Self::egal_temps_constant(&Self::csrf_presente(&req), cookie) {
                    warn!("Admin login refused: CSRF token mismatch");
                    return Self::refus_csrf();
                }
                if !Self::check_login_rate_limit(auth_state, client_ip).await {
                    return Self::http_response(
                        429,
                        "Too Many Requests",
                        "Too many login attempts. Try again later.",
                    );
                }
                return Self::login_post(config, req.body, auth_state, client_ip).await;
            }
        }

        // Toutes les autres routes : authentification requise
        let Some(csrf_session) =
            Self::session_csrf(auth_state, req.cookie_session.as_deref()).await
        else {
            return Self::redirect_to_login();
        };

        // Jeton CSRF lié à la session pour toute requête modifiante.
        if modifiante && !Self::egal_temps_constant(&Self::csrf_presente(&req), &csrf_session) {
            warn!("Admin {} {} refused: CSRF token mismatch", method, path_norm);
            return Self::refus_csrf();
        }

        match (method, path_norm) {
            ("POST", "/logout" | "logout") => {
                if let Some(t) = req.cookie_session.as_deref() {
                    auth_state.sessions.lock().await.remove(t);
                }
                Self::redirect_logout()
            }
            ("GET", "/" | "") => Self::dashboard_html(config, sessions, pools, &csrf_session).await,
            ("GET", "/api/status") => Self::api_status(config, sessions, pools).await,
            ("GET", "/api/sessions") => Self::api_sessions(sessions).await,
            ("GET", "/api/pools") => Self::api_pools(pools).await,
            ("GET", "/api/config") => Self::api_config(config),
            _ => Self::http_response(404, "Not Found", "Endpoint not found"),
        }
    }

    /// Page de login (formulaire).
    ///
    /// Pose le cookie `__Host-admin_csrf` et place le même jeton dans un champ caché
    /// (double soumission, vérifiée au POST).
    async fn login_form_html(auth_state: &AdminAuthState) -> String {
        let csrf = Self::jeton_aleatoire();
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
            <input type="hidden" name="csrf" value="{}">
            <label for="email">Email</label>
            <input type="email" id="email" name="email" placeholder="{}" required autocomplete="email">
            <label for="password">Mot de passe</label>
            <input type="password" id="password" name="password" required autocomplete="current-password">
            <button type="submit">Connexion</button>
        </form>
    </div>
</body>
</html>"#,
            csrf, email_placeholder_escaped
        );
        let reponse = Self::http_response_html(200, "OK", &html);
        // Insère le cookie CSRF après la ligne de statut.
        match reponse.split_once("\r\n") {
            Some((statut, reste)) => format!(
                "{statut}\r\n{}Cache-Control: no-store\r\n{reste}",
                Self::cookie_host(COOKIE_CSRF_LOGIN, &csrf, CSRF_LOGIN_TTL_SECS)
            ),
            None => reponse,
        }
    }

    /// Traitement POST /login : vérification mot de passe, création session, rate limiting.
    async fn login_post(
        _config: &OriginConfig,
        body: String,
        auth_state: &Arc<AdminAuthState>,
        client_ip: std::net::IpAddr,
    ) -> String {
        let auth_config = match &auth_state.config {
            Some(c) => c,
            None => {
                return Self::http_response_html(
                    403,
                    "Forbidden",
                    "<p>Admin auth not configured.</p>",
                );
            }
        };

        let email = Self::form_value(&body, "email");
        let password = Self::form_value(&body, "password");

        if email.is_empty() || password.is_empty() || email.len() > 256 || password.len() > 256 {
            Self::record_failed_login(auth_state, client_ip).await;
            return Self::redirect_to_login();
        }

        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed_hash = match PasswordHash::new(&auth_config.password_hash) {
            Ok(h) => h,
            Err(_) => {
                Self::record_failed_login(auth_state, client_ip).await;
                return Self::redirect_to_login();
            }
        };

        // Le mot de passe est toujours vérifié (pas d'oracle sur l'email), puis l'email.
        let password_ok = argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        let email_ok = Self::emails_match(&email, &auth_config.email);
        if !(password_ok && email_ok) {
            Self::record_failed_login(auth_state, client_ip).await;
            return Self::redirect_to_login();
        }

        Self::reset_login_attempts(auth_state, client_ip).await;

        let token = Self::jeton_aleatoire();
        let ttl = Duration::from_secs(auth_config.session_ttl_seconds);
        {
            let mut sessions = auth_state.sessions.lock().await;
            sessions.insert(
                token.clone(),
                SessionAdmin {
                    expiration: Instant::now() + ttl,
                    csrf: Self::jeton_aleatoire(),
                },
            );
        }
        Self::redirect_with_session(&token, auth_config.session_ttl_seconds)
    }

    /// Compare deux emails (insensible à la casse/espaces) en temps constant.
    fn emails_match(provided: &str, expected: &str) -> bool {
        let a = provided.trim().to_lowercase();
        let b = expected.trim().to_lowercase();
        if a.len() != b.len() {
            return false;
        }
        a.bytes().zip(b.bytes()).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
    }

    /// Extrait une valeur d'un body application/x-www-form-urlencoded.
    fn form_value(body: &str, key: &str) -> String {
        let key_eq = format!("{}=", key);
        for part in body.split('&') {
            let part = part.trim();
            if part.starts_with(&key_eq) {
                let value = part[key_eq.len()..].trim();
                return urlencoding::decode(value)
                    .unwrap_or(std::borrow::Cow::Borrowed(value))
                    .into_owned();
            }
        }
        String::new()
    }

    /// Dashboard HTML.
    async fn dashboard_html(
        config: &OriginConfig,
        sessions: Option<&Arc<SessionManager>>,
        pools: Option<&Arc<PoolManager>>,
        csrf: &str,
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
    <meta name="csrf-token" content="{}">
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
                <input type="hidden" name="csrf" value="{}">
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
            csrf,
            csrf,
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

    /// Réponse HTTP texte (avec security headers).
    fn http_response(status: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }

    /// Réponse HTTP HTML (avec security headers).
    fn http_response_html(status: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }

    /// Réponse HTTP JSON (admin : pas de CORS, headers de sécurité).
    fn http_response_json(status: u16, status_text: &str, json: &serde_json::Value) -> String {
        let body = serde_json::to_string_pretty(json).unwrap_or_default();
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{}",
            status, status_text, body.len(), body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> AdminAuthState {
        AdminAuthState {
            config: None,
            sessions: Mutex::new(HashMap::new()),
            login_attempts: Mutex::new(HashMap::new()),
        }
    }

    #[test]
    fn cookie_admin_prefixe_host_strict() {
        // O-04 : `__Host-` (Secure, Path=/, sans Domain) : un cookie posé par un sous-domaine
        // COG sur le domaine parent ne peut pas le masquer.
        for r in [
            AdminServer::redirect_with_session("jeton", 60),
            AdminServer::redirect_logout(),
        ] {
            let ligne = r
                .lines()
                .find(|l| l.starts_with("Set-Cookie: __Host-admin_session="))
                .expect("Set-Cookie");
            assert!(ligne.contains("; HttpOnly"));
            assert!(ligne.contains("; Secure"));
            assert!(ligne.contains("; SameSite=Strict"));
            assert!(ligne.contains("; Path=/;"));
            assert!(!ligne.to_ascii_lowercase().contains("domain="));
            // Plus aucun cookie sans préfixe.
            assert!(!r.contains("Set-Cookie: admin_session"));
        }
        assert!(AdminServer::redirect_with_session("jeton", 60)
            .contains("Set-Cookie: __Host-admin_session=jeton;"));
    }

    #[test]
    fn cookie_parse_utilise_le_bon_nom() {
        let req = "GET / HTTP/1.1\r\nCookie: a=1; __Host-admin_session=abc123; b=2\r\n\r\n";
        assert_eq!(AdminServer::parse_request(req).cookie_session.as_deref(), Some("abc123"));
        let req = "GET / HTTP/1.1\r\nCookie: __Host-admin_session=x\r\n\r\n";
        assert_eq!(AdminServer::parse_request(req).cookie_session.as_deref(), Some("x"));
        // Ancien nom sans préfixe (posable par un sous-domaine via Domain=) : ignoré.
        let req = "GET / HTTP/1.1\r\nCookie: admin_session=zz\r\n\r\n";
        assert_eq!(AdminServer::parse_request(req).cookie_session, None);
        let req = "GET / HTTP/1.1\r\nCookie: other__Host-admin_session=zz\r\n\r\n";
        assert_eq!(AdminServer::parse_request(req).cookie_session, None);
    }

    #[test]
    fn cookie_session_est_secure() {
        assert!(AdminServer::redirect_with_session("t", 60).contains("; Secure;"));
        assert!(AdminServer::redirect_logout().contains("; Secure;"));
    }

    fn config_admin() -> OriginConfig {
        let mut c = OriginConfig::default();
        c.admin.public_origin = Some("https://admin.miyukini.com".to_string());
        c
    }

    fn pair() -> SocketAddr {
        "198.51.100.9:5555".parse().unwrap()
    }

    async fn etat_avec_session() -> (Arc<AdminAuthState>, String, String) {
        let st = Arc::new(state());
        let jeton = "s".repeat(64);
        let csrf = "c".repeat(64);
        st.sessions.lock().await.insert(
            jeton.clone(),
            SessionAdmin {
                expiration: Instant::now() + Duration::from_secs(60),
                csrf: csrf.clone(),
            },
        );
        (st, jeton, csrf)
    }

    async fn requete(st: &Arc<AdminAuthState>, brute: &str) -> String {
        AdminServer::handle_request(brute, &config_admin(), None, None, st, pair()).await
    }

    #[tokio::test]
    async fn post_admin_sans_origin_ou_sans_jeton_refuse_avec_les_deux_accepte() {
        let (st, jeton, csrf) = etat_avec_session().await;
        let cookie = format!("Cookie: __Host-admin_session={jeton}");

        // Sans Origin ni Referer : refusé, session intacte.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\n{cookie}\r\n\r\ncsrf={csrf}"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 403"), "{r}");
        assert!(r.ends_with(CORPS_REFUS_CSRF));
        assert!(st.sessions.lock().await.contains_key(&jeton));

        // Origin d'un sous-domaine COG (même site, autre origine) : refusé.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nOrigin: https://boutique.miyukini.com\r\n{cookie}\r\n\r\ncsrf={csrf}"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 403"));
        // Origin « null » (document en bac à sable) : refusé.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nOrigin: null\r\n{cookie}\r\n\r\ncsrf={csrf}"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 403"));

        // Bonne Origin mais sans jeton : refusé.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nOrigin: https://admin.miyukini.com\r\n{cookie}\r\n\r\n"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 403"));
        // Bonne Origin, mauvais jeton : refusé.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nOrigin: https://admin.miyukini.com\r\n{cookie}\r\n\r\ncsrf={}", "d".repeat(64)),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 403"));
        assert!(st.sessions.lock().await.contains_key(&jeton));

        // Referer (sans Origin) + jeton en en-tête : accepté.
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nReferer: https://admin.miyukini.com/admin/\r\nX-CSRF-Token: {csrf}\r\n{cookie}\r\n\r\n"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 302"), "{r}");
        assert!(!st.sessions.lock().await.contains_key(&jeton));

        // Origin + jeton de formulaire : accepté.
        let (st, jeton, csrf) = etat_avec_session().await;
        let r = requete(
            &st,
            &format!("POST /admin/logout HTTP/1.1\r\nOrigin: https://admin.miyukini.com\r\nCookie: __Host-admin_session={jeton}\r\n\r\ncsrf={csrf}"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 302"));
        assert!(r.contains("Set-Cookie: __Host-admin_session=;"));
        assert!(!st.sessions.lock().await.contains_key(&jeton));
    }

    #[tokio::test]
    async fn login_post_exige_origin_et_double_soumission() {
        let st = Arc::new(state());
        // Le formulaire pose le cookie CSRF et le même jeton en champ caché.
        let page = requete(&st, "GET /admin/login HTTP/1.1\r\n\r\n").await;
        let ligne = page
            .lines()
            .find(|l| l.starts_with("Set-Cookie: __Host-admin_csrf="))
            .expect("cookie csrf");
        assert!(ligne.contains("; Secure") && ligne.contains("; Path=/;"));
        let jeton = ligne
            .trim_start_matches("Set-Cookie: __Host-admin_csrf=")
            .split(';')
            .next()
            .unwrap()
            .to_string();
        assert_eq!(jeton.len(), 64);
        assert!(page.contains(&format!("name=\"csrf\" value=\"{jeton}\"")));

        let corps = format!("csrf={jeton}&email=a%40b.c&password=x");
        let origin = "Origin: https://admin.miyukini.com";
        let cookie = format!("Cookie: __Host-admin_csrf={jeton}");
        // Sans Origin : refusé.
        let r = requete(&st, &format!("POST /admin/login HTTP/1.1\r\n{cookie}\r\n\r\n{corps}")).await;
        assert!(r.ends_with(CORPS_REFUS_CSRF));
        // Sans cookie CSRF : refusé.
        let r = requete(&st, &format!("POST /admin/login HTTP/1.1\r\n{origin}\r\n\r\n{corps}")).await;
        assert!(r.ends_with(CORPS_REFUS_CSRF));
        // Sans champ CSRF : refusé.
        let r = requete(
            &st,
            &format!("POST /admin/login HTTP/1.1\r\n{origin}\r\n{cookie}\r\n\r\nemail=a%40b.c&password=x"),
        )
        .await;
        assert!(r.ends_with(CORPS_REFUS_CSRF));
        // Les deux : le contrôle CSRF passe (puis refus métier : auth non configurée).
        let r = requete(&st, &format!("POST /admin/login HTTP/1.1\r\n{origin}\r\n{cookie}\r\n\r\n{corps}")).await;
        assert!(!r.ends_with(CORPS_REFUS_CSRF), "{r}");
        assert!(r.contains("Admin auth not configured"));
    }

    #[tokio::test]
    async fn get_admin_sans_origin_reste_accessible() {
        let (st, jeton, csrf) = etat_avec_session().await;
        let r = requete(
            &st,
            &format!("GET /admin/ HTTP/1.1\r\nCookie: __Host-admin_session={jeton}\r\n\r\n"),
        )
        .await;
        assert!(r.starts_with("HTTP/1.1 200"));
        // Le jeton CSRF de session est fourni au formulaire et au JS (meta).
        assert!(r.contains(&format!("name=\"csrf\" value=\"{csrf}\"")));
        assert!(r.contains(&format!("<meta name=\"csrf-token\" content=\"{csrf}\">")));
    }

    #[test]
    fn origine_attendue_et_referer() {
        let mut c = OriginConfig::default();
        assert_eq!(
            AdminServer::origine_attendue(&c, Some("Admin.Miyukini.com")),
            Some("https://admin.miyukini.com".to_string())
        );
        assert_eq!(AdminServer::origine_attendue(&c, None), None);
        c.admin.public_origin = Some("https://admin.miyukini.com/".to_string());
        assert_eq!(
            AdminServer::origine_attendue(&c, Some("evil.example")),
            Some("https://admin.miyukini.com".to_string())
        );
        assert_eq!(
            AdminServer::origine_de_referer("https://admin.miyukini.com/admin/?x=1"),
            Some("https://admin.miyukini.com".to_string())
        );
        assert_eq!(AdminServer::origine_de_referer("https://u@evil.example/"), None);
        assert_eq!(AdminServer::origine_de_referer("pas-une-url"), None);
    }

    #[test]
    fn email_compare_insensible_a_la_casse() {
        assert!(AdminServer::emails_match(" Admin@X.org ", "admin@x.org"));
        assert!(!AdminServer::emails_match("autre@x.org", "admin@x.org"));
    }

    #[test]
    fn x_real_ip_ignore_hors_loopback() {
        let req = "POST /login HTTP/1.1\r\nX-Real-IP: 203.0.113.5\r\n\r\n";
        let lo: std::net::IpAddr = "127.0.0.1".parse().unwrap();
        let ext: std::net::IpAddr = "198.51.100.1".parse().unwrap();
        assert_eq!(AdminServer::client_ip(req, lo), "203.0.113.5".parse::<std::net::IpAddr>().unwrap());
        assert_eq!(AdminServer::client_ip(req, ext), ext);
        assert_eq!(AdminServer::client_ip("GET / HTTP/1.1\r\n\r\n", lo), lo);
    }

    #[tokio::test]
    async fn rate_limit_bloque_puis_se_libere() {
        let st = state();
        let ip: std::net::IpAddr = "203.0.113.5".parse().unwrap();
        let other: std::net::IpAddr = "203.0.113.6".parse().unwrap();
        for _ in 0..MAX_LOGIN_ATTEMPTS {
            AdminServer::record_failed_login(&st, ip).await;
        }
        assert!(!AdminServer::check_login_rate_limit(&st, ip).await);
        // Une autre IP (autre client derrière le même proxy) n'est pas affectée.
        assert!(AdminServer::check_login_rate_limit(&st, other).await);
        // Expiration du blocage : l'IP est libérée malgré count >= MAX.
        {
            let mut m = st.login_attempts.lock().await;
            let e = m.get_mut(&ip).unwrap();
            e.2 = Some(Instant::now() - Duration::from_secs(1));
        }
        assert!(AdminServer::check_login_rate_limit(&st, ip).await);
        // Un nouvel échec repart de zéro (pas de re-blocage immédiat).
        AdminServer::record_failed_login(&st, ip).await;
        assert!(AdminServer::check_login_rate_limit(&st, ip).await);
    }
}
