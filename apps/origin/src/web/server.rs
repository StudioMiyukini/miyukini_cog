//! Serveur Web Origin — Implémentation HTTP.

use super::{api, content::ContentManager, forum_auth, market, pages};
use crate::config::OriginConfig;
use crate::relay::rate_limiter::{RateLimiter, RateLimiterConfig, RateLimitResult};
use crate::relay::RelayServer;
use crate::tracker::{catalog::Catalog, pool::PoolManager, slug_registry::SlugRegistry, CatalogVisitTracker};
use bytes::Bytes;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

/// Base de données JayXpose (optionnelle) pour les pages vitrine publiques.
pub type JayXposeDbRef = Arc<jayxpose::JayXposeDb>;

/// Réponse HTTP : normale, binaire, redirection ou brute.
pub enum RouteResponse {
    Normal {
        status: String,
        content_type: String,
        body: String,
    },
    /// Réponse binaire pour les fichiers (ZIP, images, etc.)
    Binary {
        status: String,
        content_type: String,
        body: Vec<u8>,
        filename: Option<String>,
    },
    Redirect { location: String },
    /// Réponse HTTP brute (Phase 2 tunnel) à transmettre telle quelle.
    Raw { bytes: Vec<u8> },
}

/// Serveur web public.
pub struct WebServer {
    config: Arc<OriginConfig>,
    pool_manager: Arc<PoolManager>,
    content_manager: Arc<ContentManager>,
    /// Suivi des visites catalogue -> Home (pour considérer un COG présent ~1 min après une visite).
    visit_tracker: Arc<CatalogVisitTracker>,
    /// Base JayXpose (lecture seule) pour /vitrine/*. Absent si non configuré.
    jayxpose_db: Option<JayXposeDbRef>,
    /// Registre des slugs de sous-domaines COG (pour le routage xxx.miyukini.com).
    slug_registry: Arc<SlugRegistry>,
    /// Relay pour Phase 2 : inject HTTP via tunnel (si session active).
    relay_ref: Option<Arc<RelayServer>>,
    /// Auth forum unifiée Central (profils synchronisés). Si présent, /api/auth/forum/* activé.
    forum_auth_store: Option<Arc<forum_auth::ForumAuthStore>>,
    /// Service Market (catalogue + packages). Si présent, /api/market/* activé.
    market_store: Option<Arc<market::MarketStore>>,
    /// Rate limiter pour les connexions HTTP.
    rate_limiter: Arc<RateLimiter>,
}

impl WebServer {
    /// Crée un nouveau serveur web.
    /// Si `jayxpose_db` est fourni, les routes `/vitrine/*` sont activées.
    /// Si `relay_ref` est fourni, Phase 2 (inject HTTP via tunnel) est activée.
    /// Si `forum_auth_store` est fourni, les routes POST /api/auth/forum/validate et /sync sont activées.
    #[must_use]
    pub fn new(
        config: Arc<OriginConfig>,
        pool_manager: Arc<PoolManager>,
        jayxpose_db: Option<JayXposeDbRef>,
        slug_registry: Arc<SlugRegistry>,
        relay_ref: Option<Arc<RelayServer>>,
        forum_auth_store: Option<Arc<forum_auth::ForumAuthStore>>,
        market_store: Option<Arc<market::MarketStore>>,
    ) -> Self {
        let rate_limiter = Arc::new(RateLimiter::new(RateLimiterConfig {
            max_connections_per_ip: 60,
            max_messages_per_session: 200,
            max_failed_registrations: 10,
            window_seconds: 60,
            block_duration_seconds: 120,
            require_pow_for_temp: false,
            pow_difficulty: 0,
        }));

        Self {
            config,
            pool_manager,
            content_manager: Arc::new(ContentManager::new()),
            visit_tracker: Arc::new(CatalogVisitTracker::new()),
            jayxpose_db,
            slug_registry,
            relay_ref,
            forum_auth_store,
            market_store,
            rate_limiter,
        }
    }

    /// Démarre le serveur web.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bind_addr = format!("0.0.0.0:{}", self.config.tracker.web_port);
        let listener = TcpListener::bind(&bind_addr).await?;
        info!("🌐 Web server listening on http://{}", bind_addr);

        let cleanup_limiter = Arc::clone(&self.rate_limiter);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                cleanup_limiter.cleanup().await;
            }
        });

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    match self.rate_limiter.check_connection(addr.ip()).await {
                        RateLimitResult::Allowed => {}
                        _ => {
                            warn!("HTTP rate limit exceeded for {}", addr.ip());
                            continue;
                        }
                    }

                    let config = Arc::clone(&self.config);
                    let pool_mgr = Arc::clone(&self.pool_manager);
                    let content_mgr = Arc::clone(&self.content_manager);
                    let visit_tracker = Arc::clone(&self.visit_tracker);
                    let jayxpose_db = self.jayxpose_db.clone();
                    let slug_registry = Arc::clone(&self.slug_registry);
                    let relay_ref = self.relay_ref.clone();
                    let forum_auth_store = self.forum_auth_store.clone();
                    let market_store = self.market_store.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, config, pool_mgr, content_mgr, visit_tracker, jayxpose_db, slug_registry, relay_ref, forum_auth_store, market_store).await {
                            debug!("Connection error from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }
}

/// Extrait Content-Length des headers (optionnel).
fn content_length_from_headers(headers: &[String]) -> Option<usize> {
    const MAX_BODY: usize = 65_536;
    for line in headers {
        let lower = line.to_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            let n: usize = v.trim().parse().ok()?;
            return Some(n.min(MAX_BODY));
        }
    }
    None
}

/// Gère une connexion HTTP.
async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    config: Arc<OriginConfig>,
    pool_mgr: Arc<PoolManager>,
    content_mgr: Arc<ContentManager>,
    visit_tracker: Arc<CatalogVisitTracker>,
    jayxpose_db: Option<JayXposeDbRef>,
    slug_registry: Arc<SlugRegistry>,
    relay_ref: Option<Arc<RelayServer>>,
    forum_auth_store: Option<Arc<forum_auth::ForumAuthStore>>,
    market_store: Option<Arc<market::MarketStore>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).await?;

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Lire les headers
    let mut headers = Vec::new();
    let mut host_header: Option<String> = None;
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).await?;
        if line.trim().is_empty() {
            break;
        }
        // Extraire le Host header
        if let Some(value) = line.strip_prefix("Host:").or_else(|| line.strip_prefix("host:")) {
            host_header = Some(value.trim().to_lowercase());
        }
        headers.push(line);
    }

    debug!("{} {} (Host: {:?})", method, path, host_header);

    // Auth forum : POST /api/auth/forum/* avec body JSON
    if method == "POST" && path.starts_with("/api/auth/forum/") {
        if let Some(store) = &forum_auth_store {
            let len = content_length_from_headers(&headers).unwrap_or(0);
            let mut body = vec![0u8; len];
            if !body.is_empty() {
                buf_reader.read_exact(&mut body).await?;
            }
            let route_response = forum_auth::handle_api(path, &body, store.as_ref());
            write_response(&mut writer, route_response).await?;
            return Ok(());
        }
    }

    // Service Market : /api/market/*
    if path.starts_with("/api/market/") {
        if let Some(store) = &market_store {
            let (path_only, query) = path.split_once('?').unwrap_or((path, ""));
            let path_clean = path_only.trim_end_matches('/');

            let route_response = match (method, path_clean) {
                // GET /api/market/catalog
                ("GET", "/api/market/catalog") => {
                    let body = market::api_market_catalog(store).await;
                    RouteResponse::Normal {
                        status: "200 OK".to_string(),
                        content_type: "application/json".to_string(),
                        body,
                    }
                }
                // GET /api/market/catalog/search?q=...
                ("GET", "/api/market/catalog/search") => {
                    let q = query.strip_prefix("q=").unwrap_or("");
                    let body = market::api_market_search(store, q).await;
                    RouteResponse::Normal {
                        status: "200 OK".to_string(),
                        content_type: "application/json".to_string(),
                        body,
                    }
                }
                // GET /api/market/catalog/{id}
                ("GET", _) if path_clean.starts_with("/api/market/catalog/") => {
                    let service_id = &path_clean[20..]; // après "/api/market/catalog/"
                    if let Some(body) = market::api_market_service(store, service_id).await {
                        RouteResponse::Normal {
                            status: "200 OK".to_string(),
                            content_type: "application/json".to_string(),
                            body,
                        }
                    } else {
                        RouteResponse::Normal {
                            status: "404 Not Found".to_string(),
                            content_type: "application/json".to_string(),
                            body: r#"{"success":false,"error":"Service introuvable"}"#.to_string(),
                        }
                    }
                }
                // GET /api/market/package/{id}/{version} — téléchargement binaire
                ("GET", _) if path_clean.starts_with("/api/market/package/") => {
                    let remainder = &path_clean[20..]; // après "/api/market/package/"
                    let parts: Vec<&str> = remainder.splitn(2, '/').collect();
                    if parts.len() == 2 {
                        if let Some(data) = market::api_market_download(store, parts[0], parts[1]).await {
                            let filename = miyumarket::package::package_filename(parts[0], parts[1]);
                            RouteResponse::Binary {
                                status: "200 OK".to_string(),
                                content_type: "application/octet-stream".to_string(),
                                body: data,
                                filename: Some(filename),
                            }
                        } else {
                            RouteResponse::Normal {
                                status: "404 Not Found".to_string(),
                                content_type: "application/json".to_string(),
                                body: r#"{"success":false,"error":"Package introuvable"}"#.to_string(),
                            }
                        }
                    } else {
                        RouteResponse::Normal {
                            status: "400 Bad Request".to_string(),
                            content_type: "application/json".to_string(),
                            body: r#"{"success":false,"error":"Format: /api/market/package/{id}/{version}"}"#.to_string(),
                        }
                    }
                }
                // POST /api/market/publish
                ("POST", "/api/market/publish") => {
                    let len = content_length_from_headers(&headers).unwrap_or(0);
                    let mut body_bytes = vec![0u8; len];
                    if !body_bytes.is_empty() {
                        buf_reader.read_exact(&mut body_bytes).await?;
                    }
                    let body_str = String::from_utf8_lossy(&body_bytes);
                    let body = market::api_market_publish(store, &body_str).await;
                    RouteResponse::Normal {
                        status: "200 OK".to_string(),
                        content_type: "application/json".to_string(),
                        body,
                    }
                }
                // DELETE /api/market/package/{id}/{version}
                ("DELETE", _) if path_clean.starts_with("/api/market/package/") => {
                    let remainder = &path_clean[20..];
                    let parts: Vec<&str> = remainder.splitn(2, '/').collect();
                    if parts.len() == 2 {
                        let len = content_length_from_headers(&headers).unwrap_or(0);
                        let mut body_bytes = vec![0u8; len];
                        if !body_bytes.is_empty() {
                            buf_reader.read_exact(&mut body_bytes).await?;
                        }
                        let body_str = String::from_utf8_lossy(&body_bytes);
                        let body = market::api_market_unpublish(store, parts[0], parts[1], &body_str).await;
                        RouteResponse::Normal {
                            status: "200 OK".to_string(),
                            content_type: "application/json".to_string(),
                            body,
                        }
                    } else {
                        RouteResponse::Normal {
                            status: "400 Bad Request".to_string(),
                            content_type: "application/json".to_string(),
                            body: r#"{"success":false,"error":"Format: /api/market/package/{id}/{version}"}"#.to_string(),
                        }
                    }
                }
                _ => {
                    RouteResponse::Normal {
                        status: "404 Not Found".to_string(),
                        content_type: "application/json".to_string(),
                        body: r#"{"success":false,"error":"Route Market inconnue"}"#.to_string(),
                    }
                }
            };
            write_response(&mut writer, route_response).await?;
            return Ok(());
        }
    }

    // Vérifier si la requête est pour un sous-domaine COG
    let domain = config
        .identity
        .domain
        .as_deref()
        .unwrap_or("miyukini.com");

    if let Some(slug) = extract_cog_slug(host_header.as_deref(), domain) {
        // Construire la requête HTTP brute pour Phase 2 (inject via tunnel)
        let raw_request = format!(
            "{}\r\n{}\r\n\r\n",
            request_line.trim_end(),
            headers.iter().map(|h| h.trim_end()).collect::<Vec<_>>().join("\r\n")
        );
        let route_response = handle_cog_subdomain_request(
            &slug,
            path,
            method,
            &headers,
            Bytes::from(raw_request),
            &config,
            &pool_mgr,
            &slug_registry,
            relay_ref.as_deref(),
        )
        .await;
        write_response(&mut writer, route_response).await?;
        return Ok(());
    }

    // Routage normal (miyukini.com / origin.miyukini.com / IP directe)
    let route_response = route_request(
        path,
        &pool_mgr,
        &content_mgr,
        &visit_tracker,
        jayxpose_db.as_deref(),
    )
    .await;

    write_response(&mut writer, route_response).await?;

    Ok(())
}

/// Écrit une réponse HTTP sur le stream.
async fn write_response(
    writer: &mut (impl AsyncWriteExt + Unpin),
    route_response: RouteResponse,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const SECURITY_HEADERS: &str = "\
X-Content-Type-Options: nosniff\r\n\
X-Frame-Options: DENY\r\n\
X-XSS-Protection: 1; mode=block\r\n\
Referrer-Policy: strict-origin-when-cross-origin\r\n\
Permissions-Policy: camera=(), microphone=(), geolocation=()\r\n";

    match route_response {
        RouteResponse::Normal {
            status,
            content_type,
            body,
        } => {
            let response = format!(
                "HTTP/1.1 {}\r\n\
                 Content-Type: {}; charset=utf-8\r\n\
                 Content-Length: {}\r\n\
                 {}\
                 Connection: close\r\n\
                 \r\n",
                status,
                content_type,
                body.len(),
                SECURITY_HEADERS,
            );
            writer.write_all(response.as_bytes()).await?;
            writer.write_all(body.as_bytes()).await?;
        }
        RouteResponse::Binary {
            status,
            content_type,
            body,
            filename,
        } => {
            let disposition = filename
                .map(|f| format!("Content-Disposition: attachment; filename=\"{f}\"\r\n"))
                .unwrap_or_default();
            let response = format!(
                "HTTP/1.1 {}\r\n\
                 Content-Type: {}\r\n\
                 Content-Length: {}\r\n\
                 {}\
                 {}\
                 Connection: close\r\n\
                 \r\n",
                status,
                content_type,
                body.len(),
                disposition,
                SECURITY_HEADERS,
            );
            writer.write_all(response.as_bytes()).await?;
            writer.write_all(&body).await?;
        }
        RouteResponse::Redirect { location } => {
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {}\r\n{}Connection: close\r\n\r\n",
                location, SECURITY_HEADERS,
            );
            writer.write_all(response.as_bytes()).await?;
        }
        RouteResponse::Raw { bytes } => {
            writer.write_all(&bytes).await?;
        }
    }
    writer.flush().await?;
    Ok(())
}

/// Extrait le slug du sous-domaine COG depuis le header Host.
///
/// Retourne `Some("boutique-alice")` pour `Host: boutique-alice.miyukini.com`.
/// Retourne `None` pour le domaine principal, origin, www, ou une IP directe.
fn extract_cog_slug(host: Option<&str>, domain: &str) -> Option<String> {
    let host = host?;
    // Retirer le port si présent (ex: "boutique-alice.miyukini.com:8080")
    let host_no_port = host.split(':').next().unwrap_or(host);

    // Vérifier que c'est un sous-domaine de notre domaine
    let suffix = format!(".{}", domain);
    if !host_no_port.ends_with(&suffix) {
        return None;
    }

    // Extraire le slug (partie avant .miyukini.com)
    let slug = &host_no_port[..host_no_port.len() - suffix.len()];
    if slug.is_empty() {
        return None;
    }

    // Ignorer les sous-domaines système
    let reserved = ["www", "origin", "api", "admin", "relay", "tracker", "mail", "docs", "blog", "status", "forum"];
    if reserved.contains(&slug) {
        return None;
    }

    Some(slug.to_string())
}

/// Gère une requête HTTP destinée à un sous-domaine COG.
///
/// Phase 2 : si le COG a une session Relay active, inject via tunnel (masquage IP).
/// Phase 1 (fallback) : proxy HTTP direct vers l'IP:port du COG.
async fn handle_cog_subdomain_request(
    slug: &str,
    path: &str,
    _method: &str,
    _headers: &[String],
    raw_request: Bytes,
    config: &OriginConfig,
    pool_mgr: &PoolManager,
    slug_registry: &SlugRegistry,
    relay_ref: Option<&RelayServer>,
) -> RouteResponse {
    // Chercher le cog_id associé au slug
    let cog_id = match slug_registry.lookup(slug).await {
        Some(id) => id,
        None => {
            return cog_not_found_page(slug, config);
        }
    };

    // Phase 2 : inject via tunnel si le COG a une session Relay active
    if let Some(relay) = relay_ref {
        match relay.inject_http_request(&cog_id, raw_request.clone()).await {
            Ok(response_bytes) => {
                info!(
                    "COG subdomain Phase 2 (tunnel): {}.{} (cog_id: {})",
                    slug,
                    config.identity.domain.as_deref().unwrap_or("miyukini.com"),
                    cog_id
                );
                return raw_http_response_to_route(&response_bytes);
            }
            Err(e) => {
                debug!(
                    "Phase 2 tunnel not available for '{}' ({}), falling back to Phase 1: {}",
                    slug, cog_id, e
                );
            }
        }
    }

    // Phase 1 (ou fallback) : proxy HTTP direct vers le COG
    let entry = pool_mgr.find_cog(&cog_id).await;

    match entry {
        Some(entry) => {
            let target_url = if entry.address.starts_with("http") {
                format!("{}{}", entry.address.trim_end_matches('/'), path)
            } else {
                format!("http://{}{}", entry.address, path)
            };

            info!(
                "COG subdomain proxy: {}.{} -> {} (cog_id: {})",
                slug,
                config.identity.domain.as_deref().unwrap_or("miyukini.com"),
                target_url,
                cog_id
            );

            match proxy_http_request(&target_url).await {
                Ok(response) => response,
                Err(e) => {
                    warn!(
                        "COG proxy failed for slug '{}' (cog_id: {}): {}",
                        slug, cog_id, e
                    );
                    cog_offline_page(slug, &cog_id, config)
                }
            }
        }
        None => {
            cog_offline_page(slug, &cog_id, config)
        }
    }
}

/// Convertit une réponse HTTP brute (Phase 2 tunnel) en RouteResponse.
fn raw_http_response_to_route(response: &[u8]) -> RouteResponse {
    RouteResponse::Raw {
        bytes: response.to_vec(),
    }
}

/// Proxy HTTP direct vers le COG (Phase 1).
async fn proxy_http_request(
    target_url: &str,
) -> Result<RouteResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Extraire host:port et path depuis l'URL
    let url = target_url
        .strip_prefix("http://")
        .unwrap_or(target_url.strip_prefix("https://").unwrap_or(target_url));

    let (host_port, path) = match url.find('/') {
        Some(idx) => (&url[..idx], &url[idx..]),
        None => (url, "/"),
    };

    // Connexion TCP vers le COG
    let mut stream = match tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        tokio::net::TcpStream::connect(host_port),
    )
    .await
    {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => return Err(format!("connection failed: {}", e).into()),
        Err(_) => return Err("connection timeout (5s)".into()),
    };

    // Envoyer la requête HTTP
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: MiyukiniOriginProxy/1.0\r\n\r\n",
        path, host_port
    );
    stream.write_all(request.as_bytes()).await?;

    // Lire la réponse complète
    let mut response_buf = Vec::new();
    tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        stream.read_to_end(&mut response_buf),
    )
    .await
    .map_err(|_| "response timeout (10s)")??;

    // Parser la réponse HTTP minimale
    let response_str = String::from_utf8_lossy(&response_buf);

    // Séparer headers et body
    if let Some(header_end) = response_str.find("\r\n\r\n") {
        let body = &response_str[header_end + 4..];
        let headers_part = &response_str[..header_end];

        // Extraire le status
        let status = headers_part
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("HTTP/1.1 ").or_else(|| line.strip_prefix("HTTP/1.0 ")))
            .unwrap_or("200 OK")
            .to_string();

        // Extraire Content-Type
        let content_type = headers_part
            .lines()
            .find_map(|line| {
                let lower = line.to_lowercase();
                if lower.starts_with("content-type:") {
                    Some(line.split_once(':')?.1.trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "text/html".to_string());

        Ok(RouteResponse::Normal {
            status,
            content_type,
            body: body.to_string(),
        })
    } else {
        Ok(RouteResponse::Normal {
            status: "502 Bad Gateway".to_string(),
            content_type: "text/plain".to_string(),
            body: "Invalid response from COG".to_string(),
        })
    }
}

/// Page affichée quand le slug ne correspond à aucun COG enregistré.
fn cog_not_found_page(slug: &str, config: &OriginConfig) -> RouteResponse {
    let domain = config
        .identity
        .domain
        .as_deref()
        .unwrap_or("miyukini.com");
    let catalog_url = format!("https://{}/mws", domain);
    let slug_escaped = pages::html_escape(slug);
    let body = format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{slug_escaped}.{domain} — COG introuvable</title>
    <style>
        :root {{ --primary: #8b5cf6; --bg: #0a0a0f; --text: #f0f0f5; --text-muted: #9ca3af; }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: system-ui, sans-serif; background: var(--bg); color: var(--text);
            min-height: 100vh; display: flex; align-items: center; justify-content: center; text-align: center; }}
        .container {{ padding: 2rem; max-width: 480px; }}
        h1 {{ font-size: 1.5rem; color: var(--primary); margin-bottom: 1rem; }}
        p {{ color: var(--text-muted); margin-bottom: 1.5rem; line-height: 1.6; }}
        .slug {{ color: var(--primary); font-weight: 600; }}
        a {{ display: inline-block; background: var(--primary); color: white; padding: 0.75rem 1.5rem;
            border-radius: 0.5rem; text-decoration: none; font-weight: 600; }}
        a:hover {{ opacity: 0.9; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>COG introuvable</h1>
        <p>Le sous-domaine <span class="slug">{slug_escaped}.{domain}</span> ne correspond a aucun COG enregistre sur le Webway.</p>
        <p>Le COG n'est peut-etre pas encore connecte, ou le slug est incorrect.</p>
        <a href="{catalog_url}">Voir le MWS</a>
    </div>
</body>
</html>"#,
        slug_escaped = slug_escaped,
        domain = domain,
        catalog_url = catalog_url,
    );
    RouteResponse::Normal {
        status: "404 Not Found".to_string(),
        content_type: "text/html; charset=utf-8".to_string(),
        body,
    }
}

/// Page affichée quand le COG est enregistré mais offline / injoignable.
fn cog_offline_page(slug: &str, cog_id: &str, config: &OriginConfig) -> RouteResponse {
    let domain = config
        .identity
        .domain
        .as_deref()
        .unwrap_or("miyukini.com");
    let catalog_url = format!("https://{}/mws", domain);
    let slug_escaped = pages::html_escape(slug);
    let cog_escaped = pages::html_escape(cog_id);
    let body = format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{slug_escaped}.{domain} — COG hors ligne</title>
    <style>
        :root {{ --primary: #8b5cf6; --bg: #0a0a0f; --text: #f0f0f5; --text-muted: #9ca3af; --warning: #f59e0b; }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: system-ui, sans-serif; background: var(--bg); color: var(--text);
            min-height: 100vh; display: flex; align-items: center; justify-content: center; text-align: center; }}
        .container {{ padding: 2rem; max-width: 480px; }}
        h1 {{ font-size: 1.5rem; color: var(--warning); margin-bottom: 1rem; }}
        p {{ color: var(--text-muted); margin-bottom: 1.5rem; line-height: 1.6; }}
        .slug {{ color: var(--primary); font-weight: 600; }}
        .cog-id {{ font-family: monospace; font-size: 0.875rem; color: var(--text-muted); }}
        a {{ display: inline-block; background: var(--primary); color: white; padding: 0.75rem 1.5rem;
            border-radius: 0.5rem; text-decoration: none; font-weight: 600; }}
        a:hover {{ opacity: 0.9; }}
        .retry {{ margin-top: 1rem; font-size: 0.875rem; color: var(--text-muted); }}
    </style>
</head>
<body>
    <div class="container">
        <h1>COG hors ligne</h1>
        <p>Le COG <span class="slug">{slug_escaped}</span> est enregistre sur le Webway mais n'est pas joignable actuellement.</p>
        <p class="cog-id">{cog_escaped}</p>
        <a href="{catalog_url}">Voir le MWS</a>
        <p class="retry">Rechargez la page dans quelques instants.</p>
    </div>
</body>
</html>"#,
        slug_escaped = slug_escaped,
        domain = domain,
        cog_escaped = cog_escaped,
        catalog_url = catalog_url,
    );
    RouteResponse::Normal {
        status: "503 Service Unavailable".to_string(),
        content_type: "text/html; charset=utf-8".to_string(),
        body,
    }
}

/// Route les requêtes HTTP.
async fn route_request(
    path: &str,
    pool_mgr: &PoolManager,
    content_mgr: &ContentManager,
    visit_tracker: &CatalogVisitTracker,
    jayxpose_db: Option<&jayxpose::JayXposeDb>,
) -> RouteResponse {
    // Séparer path et query string
    let (path_only, query) = path.split_once('?').unwrap_or((path, ""));

    // Nettoyer le path
    let path_clean = path_only.trim_end_matches('/');
    let path_clean = if path_clean.is_empty() { "/" } else { path_clean };

    match path_clean {
        // ═══════════════════════════════════════════════════════════════════
        // Pages HTML
        // ═══════════════════════════════════════════════════════════════════
        "/" => {
            let body = pages::home_page(content_mgr, pool_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/docs" => {
            let body = pages::docs_page(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/downloads" => {
            let body = pages::downloads_page(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // Fichiers statiques (téléchargements)
        // ═══════════════════════════════════════════════════════════════════
        _ if path_clean.starts_with("/files/") => {
            let filename = &path_clean[7..]; // Après "/files/"
            if filename.is_empty() || filename.len() > 255
                || filename.contains("..")
                || filename.contains('\\')
                || filename.contains('\0')
                || filename.starts_with('/')
            {
                return RouteResponse::Normal {
                    status: "400 Bad Request".to_string(),
                    content_type: "text/plain".to_string(),
                    body: "Invalid path".to_string(),
                };
            }
            let base_dir = std::path::Path::new("/opt/miyukini-origin/files");
            let requested = base_dir.join(filename);
            let file_path = match requested.canonicalize() {
                Ok(p) => p,
                Err(_) => return not_found_page(),
            };
            if !file_path.starts_with(base_dir) {
                return RouteResponse::Normal {
                    status: "400 Bad Request".to_string(),
                    content_type: "text/plain".to_string(),
                    body: "Invalid path".to_string(),
                };
            }
            if file_path.is_symlink() {
                return RouteResponse::Normal {
                    status: "403 Forbidden".to_string(),
                    content_type: "text/plain".to_string(),
                    body: "Forbidden".to_string(),
                };
            }
            match tokio::fs::read(&file_path).await {
                Ok(data) => {
                    let content_type = if filename.ends_with(".zip") {
                        "application/zip"
                    } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
                        "application/gzip"
                    } else if filename.ends_with(".exe") {
                        "application/vnd.microsoft.portable-executable"
                    } else {
                        "application/octet-stream"
                    };
                    RouteResponse::Binary {
                        status: "200 OK".to_string(),
                        content_type: content_type.to_string(),
                        body: data,
                        filename: Some(filename.to_string()),
                    }
                }
                Err(_) => not_found_page(),
            }
        }

        "/blog" => {
            let body = pages::blog_page(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/announcements" => {
            let body = pages::announcements_page(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/mws" => {
            let catalog = Catalog::from_pools(pool_mgr).await;
            let body = catalog.to_html();
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/catalog" => {
            RouteResponse::Redirect { location: "/mws".to_string() }
        }

        "/visit" => {
            let cog_id = query
                .split('&')
                .find_map(|p| {
                    let v = p.strip_prefix("cog_id=")?;
                    let v = v.split('&').next().unwrap_or(v);
                    urlencoding::decode(v).ok().map(|c| c.into_owned())
                });
            match cog_id {
                Some(id) if !id.is_empty() && id.len() <= 128 => {
                    if let Some(entry) = pool_mgr.find_cog(&id).await {
                        visit_tracker.record_visit(&id).await;
                        let location = if entry.address.starts_with("http") {
                            entry.address
                        } else {
                            format!("http://{}/", entry.address)
                        };
                        RouteResponse::Redirect { location }
                    } else {
                        not_found_page()
                    }
                }
                _ => not_found_page(),
            }
        }

        "/services" => {
            let body = pages::services_page(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        // Route dynamique /services/{id}
        _ if path_clean.starts_with("/services/") => {
            let service_id = &path_clean[10..]; // Après "/services/"
            if let Some(body) = pages::service_detail_page(content_mgr, service_id).await {
                RouteResponse::Normal {
                    status: "200 OK".to_string(),
                    content_type: "text/html".to_string(),
                    body,
                }
            } else {
                not_found_page()
            }
        }

        "/about" => {
            let body = pages::about_page();
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // API JSON
        // ═══════════════════════════════════════════════════════════════════
        "/api/health" => {
            let body = api::api_health();
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/status" => {
            let body = api::api_status(pool_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/blog" => {
            let body = api::api_blog(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/announcements" => {
            let body = api::api_announcements(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/downloads" => {
            let body = api::api_downloads(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/docs" => {
            let body = api::api_docs(content_mgr).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        "/api/catalog" => {
            let body = api::api_catalog(pool_mgr, Some(visit_tracker)).await;
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // Routes dynamiques
        // ═══════════════════════════════════════════════════════════════════
        _ if path_clean.starts_with("/blog/") => {
            let post_id = &path_clean[6..]; // Après "/blog/"
            if let Some(body) = pages::blog_post_page(content_mgr, post_id).await {
                RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
            } else {
                not_found_page()
            }
        }

        _ if path_clean.starts_with("/api/blog/") => {
            let post_id = &path_clean[10..]; // Après "/api/blog/"
            if let Some(body) = api::api_blog_post(content_mgr, post_id).await {
                RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
            } else {
                RouteResponse::Normal {
                status: "404 Not Found".to_string(),
                content_type: "application/json".to_string(),
                body: r#"{"error": "Post not found"}"#.to_string(),
            }
            }
        }

        _ if path_clean.starts_with("/api/downloads/") => {
            let category = &path_clean[15..]; // Après "/api/downloads/"
            if let Some(body) = api::api_downloads_by_category(content_mgr, category).await {
                RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
            } else {
                RouteResponse::Normal {
                status: "404 Not Found".to_string(),
                content_type: "application/json".to_string(),
                body: r#"{"error": "Category not found"}"#.to_string(),
            }
            }
        }

        _ if path_clean.starts_with("/docs/") => {
            // /docs/{section} ou /docs/{section}/{article}
            let remainder = &path_clean[6..];
            let parts: Vec<&str> = remainder.split('/').collect();

            match parts.as_slice() {
                [section_id] => {
                    if let Some(section) = content_mgr.get_doc_section(section_id).await {
                        let body = doc_section_page(&section);
                        RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                    } else {
                        not_found_page()
                    }
                }
                [section_id, article_id] => {
                    if let Some(article) = content_mgr.get_doc_article(section_id, article_id).await {
                        let body = doc_article_page(section_id, &article);
                        RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                    } else {
                        not_found_page()
                    }
                }
                _ => not_found_page(),
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // Pages vitrine JayXpose (publiques, données par KM)
        // ═══════════════════════════════════════════════════════════════════
        _ if path_clean.starts_with("/vitrine/") => {
            if let Some(db) = jayxpose_db {
                let remainder = path_clean.strip_prefix("/vitrine/").unwrap_or("");
                let segments: Vec<&str> = remainder.split('/').filter(|s| !s.is_empty()).collect();
                match segments.as_slice() {
                    [slug] => {
                        if let Some(body) = pages::vitrine_home_page(db, slug) {
                            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                        } else {
                            not_found_page()
                        }
                    }
                    [slug, "catalogue"] => {
                        if let Some(body) = pages::vitrine_catalogue_page(db, slug, None) {
                            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                        } else {
                            not_found_page()
                        }
                    }
                    [slug, "catalogue", produit_id] => {
                        if let Some(body) = pages::vitrine_produit_page(db, slug, produit_id) {
                            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                        } else {
                            not_found_page()
                        }
                    }
                    [slug, "presentation"] => {
                        if let Some(body) = pages::vitrine_presentation_page(db, slug) {
                            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                        } else {
                            not_found_page()
                        }
                    }
                    [slug, "contact"] => {
                        if let Some(body) = pages::vitrine_contact_page(db, slug) {
                            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                        } else {
                            not_found_page()
                        }
                    }
                    _ => not_found_page(),
                }
            } else {
                not_found_page()
            }
        }

        _ if path_clean == "/vitrine" => {
            if let Some(db) = jayxpose_db {
                if let Some(body) = pages::vitrine_index_page(db) {
                    RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
                } else {
                    not_found_page()
                }
            } else {
                not_found_page()
            }
        }

        _ if path_clean.starts_with("/api/docs/") => {
            let remainder = &path_clean[10..];
            let parts: Vec<&str> = remainder.split('/').collect();

            match parts.as_slice() {
                [section_id] => {
                    if let Some(body) = api::api_doc_section(content_mgr, section_id).await {
                        RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
                    } else {
                        RouteResponse::Normal {
                status: "404 Not Found".to_string(),
                content_type: "application/json".to_string(),
                body: r#"{"error": "Section not found"}"#.to_string(),
            }
                    }
                }
                [section_id, article_id] => {
                    if let Some(body) = api::api_doc_article(content_mgr, section_id, article_id).await {
                        RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "application/json".to_string(),
                body,
            }
                    } else {
                        RouteResponse::Normal {
                status: "404 Not Found".to_string(),
                content_type: "application/json".to_string(),
                body: r#"{"error": "Article not found"}"#.to_string(),
            }
                    }
                }
                _ => RouteResponse::Normal {
                status: "404 Not Found".to_string(),
                content_type: "application/json".to_string(),
                body: r#"{"error": "Invalid path"}"#.to_string(),
            },
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // 404
        // ═══════════════════════════════════════════════════════════════════
        _ => not_found_page(),
    }
}

/// Page 404.
fn not_found_page() -> RouteResponse {
    let body = r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 — Miyukini</title>
    <style>
        :root {
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
        }
        .container { padding: 2rem; }
        h1 { font-size: 8rem; color: var(--primary); opacity: 0.5; }
        h2 { font-size: 2rem; margin: 1rem 0; }
        p { color: var(--text-muted); margin-bottom: 2rem; }
        a {
            display: inline-block;
            background: var(--primary);
            color: white;
            padding: 0.75rem 1.5rem;
            border-radius: 0.5rem;
            text-decoration: none;
            font-weight: 600;
        }
        a:hover { transform: translateY(-2px); }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>Page non trouvée</h2>
        <p>Le chemin que vous cherchez n'existe pas ou a été déplacé.</p>
        <a href="/">Retour à l'accueil</a>
    </div>
</body>
</html>"#.to_string();

    RouteResponse::Normal {
        status: "404 Not Found".to_string(),
        content_type: "text/html".to_string(),
        body,
    }
}

/// Page d'une section de documentation.
fn doc_section_page(section: &super::content::DocSection) -> String {
    let articles_html: String = section.articles
        .iter()
        .map(|a| {
            format!(
                r#"<a href="/docs/{}/{}" class="card" style="display: block;">
                    <h3>{}</h3>
                    <p style="color: var(--text-muted); font-size: 0.875rem;">
                        Mis à jour le {}
                    </p>
                </a>"#,
                pages::html_escape(&section.id),
                pages::html_escape(&a.id),
                pages::html_escape(&a.title),
                a.updated_at.format("%d %B %Y")
            )
        })
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Documentation — Miyukini</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --bg-elevated: #1a1a25;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
        }}
        a {{ color: var(--primary); text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .breadcrumb {{ margin-bottom: 2rem; color: var(--text-muted); }}
        h1 {{ font-size: 2rem; margin-bottom: 0.5rem; }}
        .description {{ color: var(--text-muted); margin-bottom: 2rem; }}
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            margin-bottom: 0.75rem;
            transition: border-color 0.2s;
        }}
        .card:hover {{ border-color: var(--primary); }}
        h3 {{ margin-bottom: 0.25rem; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="breadcrumb">
            <a href="/">Accueil</a> / <a href="/docs">Documentation</a> / {}
        </div>
        <h1>{} {}</h1>
        <p class="description">{}</p>
        <div class="articles">
            {}
        </div>
    </div>
</body>
</html>"#,
        pages::html_escape(&section.title),
        pages::html_escape(&section.title),
        pages::html_escape(&section.icon),
        pages::html_escape(&section.title),
        pages::html_escape(&section.description),
        articles_html
    )
}

/// Page d'un article de documentation.
fn doc_article_page(section_id: &str, article: &super::content::DocArticle) -> String {
    // Simple markdown to HTML
    let content_html = simple_md_to_html(&article.content);

    format!(
        r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Documentation — Miyukini</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #8b5cf6;
            --bg: #0a0a0f;
            --bg-surface: #12121a;
            --bg-elevated: #1a1a25;
            --text: #f0f0f5;
            --text-muted: #9ca3af;
            --border: rgba(139, 92, 246, 0.2);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
        }}
        a {{ color: var(--primary); text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .breadcrumb {{ margin-bottom: 2rem; color: var(--text-muted); }}
        h1 {{ font-size: 2rem; margin-bottom: 0.5rem; }}
        h2 {{ font-size: 1.5rem; margin: 2rem 0 1rem; }}
        h3 {{ font-size: 1.25rem; margin: 1.5rem 0 0.75rem; }}
        p {{ margin-bottom: 1rem; }}
        code {{ font-family: 'JetBrains Mono', monospace; background: var(--bg-elevated); padding: 0.2rem 0.4rem; border-radius: 0.25rem; }}
        pre {{
            background: var(--bg-elevated);
            border: 1px solid var(--border);
            border-radius: 0.5rem;
            padding: 1rem;
            overflow-x: auto;
            margin: 1rem 0;
        }}
        pre code {{ background: none; padding: 0; }}
        ul, ol {{ margin: 1rem 0; padding-left: 1.5rem; }}
        li {{ margin-bottom: 0.5rem; }}
        table.doc-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }}
        table.doc-table th, table.doc-table td {{
            border: 1px solid var(--border);
            padding: 0.5rem 0.75rem;
            text-align: left;
        }}
        table.doc-table th {{ background: var(--bg-elevated); font-weight: 600; }}
        table.doc-table tr:hover td {{ background: rgba(139, 92, 246, 0.05); }}
        blockquote {{ border-left: 4px solid var(--primary); padding-left: 1rem; margin: 1rem 0; color: var(--text-muted); font-style: italic; }}
        .meta {{ color: var(--text-muted); font-size: 0.875rem; margin-bottom: 2rem; }}
        .content {{ background: var(--bg-surface); border-radius: 0.5rem; padding: 2rem; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="breadcrumb">
            <a href="/">Accueil</a> / <a href="/docs">Documentation</a> / <a href="/docs/{}">{}</a>
        </div>
        <h1>{}</h1>
        <p class="meta">Mis à jour le {}</p>
        <div class="content">
            {}
        </div>
    </div>
</body>
</html>"#,
        pages::html_escape(&article.title),
        pages::html_escape(section_id),
        pages::html_escape(section_id),
        pages::html_escape(&article.title),
        article.updated_at.format("%d %B %Y"),
        content_html
    )
}

/// Convertit les liens Markdown [text](url) en HTML.
fn md_links_to_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find('[') {
        if let Some(mid) = rest[start..].find(']') {
            let mid = start + mid;
            if rest.get(mid + 1..mid + 2) == Some("(") {
                if let Some(end) = rest[mid + 2..].find(')') {
                    let end = mid + 2 + end;
                    let text = &rest[start + 1..mid];
                    let url = &rest[mid + 2..end];
                    result.push_str(&pages::html_escape(&rest[..start]));
                    result.push_str("<a href=\"");
                    result.push_str(&pages::html_escape(url));
                    result.push_str("\">");
                    result.push_str(&pages::html_escape(text));
                    result.push_str("</a>");
                    rest = &rest[end + 1..];
                    continue;
                }
            }
        }
        result.push_str(&pages::html_escape(&rest[..start + 1]));
        rest = &rest[start + 1..];
    }
    result.push_str(&pages::html_escape(rest));
    result
}

/// Conversion Markdown basique en HTML.
pub(crate) fn simple_md_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_table = false;

    for line in markdown.lines() {
        if line.starts_with("```") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&pages::html_escape(line));
            html.push('\n');
            continue;
        }

        if line.is_empty() {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            continue;
        } else if line.starts_with("# ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h1>{}</h1>\n", md_links_to_html(&line[2..])));
        } else if line.starts_with("## ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h2>{}</h2>\n", md_links_to_html(&line[3..])));
        } else if line.starts_with("### ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<h3>{}</h3>\n", md_links_to_html(&line[4..])));
        } else if line.starts_with("|") && line.contains('|') {
            let cells: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            if !in_table {
                html.push_str(r#"<table class="doc-table"><thead><tr>"#);
                for cell in &cells {
                    html.push_str("<th>");
                    html.push_str(&md_links_to_html(cell));
                    html.push_str("</th>");
                }
                html.push_str("</tr></thead><tbody>");
                in_table = true;
            } else if cells.iter().all(|c| c.chars().all(|ch| ch == '-' || ch == ':')) {
                continue;
            } else {
                html.push_str("<tr>");
                for cell in &cells {
                    html.push_str("<td>");
                    html.push_str(&md_links_to_html(cell));
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }
        } else if line.starts_with("- ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<li>{}</li>\n", md_links_to_html(&line[2..])));
        } else if line.starts_with("> ") {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<blockquote>{}</blockquote>\n", md_links_to_html(&line[2..])));
        } else {
            if in_table {
                html.push_str("</tbody></table>\n");
                in_table = false;
            }
            html.push_str(&format!("<p>{}</p>\n", md_links_to_html(line)));
        }
    }

    if in_table {
        html.push_str("</tbody></table>\n");
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }

    html
}
