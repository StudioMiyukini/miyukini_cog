//! @id     origin_web_server
//! @do     definir_route_response
//! @role   orchestration
//! @layer  app
//! @human  origin — module « web server ».

//! Serveur Web Origin — Implémentation HTTP.

use super::{api, content::ContentManager, forum_auth, market, pages};
use crate::config::OriginConfig;
use crate::garde_reseau::{analyser_adresse, connecter_cible_publique, AdresseAnalysee};
use crate::relay::rate_limiter::{RateLimitResult, RateLimiter, RateLimiterConfig};
use crate::relay::RelayServer;
use crate::tracker::{
    catalog::Catalog, pool::PoolManager, slug_registry::SlugRegistry, CatalogVisitTracker,
};
use bytes::Bytes;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

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
    Redirect {
        location: String,
    },
    /// Réponse HTTP brute (Phase 2 tunnel) à transmettre telle quelle.
    Raw {
        bytes: Vec<u8>,
    },
    /// Contenu fourni par un COG (non fiable, O-04) : servi avec
    /// `Content-Security-Policy: sandbox`, `nosniff`, sans aucun cookie.
    NonFiable {
        status: String,
        content_type: String,
        body: Vec<u8>,
        /// Scripts autorisés dans le bac à sable (hôte `/visit` dédié uniquement).
        scripts: bool,
    },
}

/// Taille maximale d'une réponse lue depuis un COG par le proxy direct (C-13, O-10).
const TAILLE_MAX_REPONSE_COG: usize = 8 * 1024 * 1024;

/// Nettoie une valeur d'en-tête venant d'un COG (pas de CR/LF, longueur bornée).
fn entete_sur(valeur: &str, defaut: &str) -> String {
    let v: String = valeur
        .chars()
        .filter(|c| !c.is_control())
        .take(128)
        .collect();
    if v.trim().is_empty() {
        defaut.to_string()
    } else {
        v
    }
}

/// Politique de bac à sable appliquée au contenu d'un sous-domaine COG (O-04) : même
/// traitement que `/visit` sur hôte dédié — scripts permis mais origine opaque (jamais
/// `allow-same-origin`). Un document en origine opaque n'est pas « même site » que le
/// domaine principal : ses requêtes ne portent pas les cookies `SameSite` de l'admin.
const CSP_SOUS_DOMAINE: &str = "Content-Security-Policy: sandbox allow-scripts allow-forms \
allow-popups allow-popups-to-escape-sandbox; frame-ancestors 'none'";

/// Un `Set-Cookie` est-il hôte seul (aucun attribut `Domain`) ? Seul ce cas est conservé :
/// le cookie reste alors confiné au sous-domaine exact du COG.
fn cookie_hote_seul(valeur: &str) -> bool {
    valeur
        .split(';')
        .skip(1)
        .all(|attribut| {
            let nom = attribut.split('=').next().unwrap_or("").trim();
            !nom.eq_ignore_ascii_case("domain")
        })
}

/// Assainit la réponse brute d'un sous-domaine COG (Phase 2, tunnel) avant de la servir.
///
/// O-04 (cookie tossing, confiance « même site ») :
/// - tout `Set-Cookie` portant un attribut `Domain` est retiré (un COG ne peut poser de
///   cookie ni sur le domaine principal ni sur les autres COG) ; seuls les cookies hôte
///   seul sont conservés ;
/// - les lignes repliées (obs-fold), `Set-Cookie2` et les en-têtes sans `:` sont retirés ;
/// - la réponse est servie en bac à sable CSP sans `allow-same-origin`, avec `nosniff`.
///
/// Retourne `None` (réponse refusée, fail-closed) si la réponse n'est pas du HTTP/1.x bien formé.
fn assainir_reponse_sous_domaine(reponse: &[u8]) -> Option<Vec<u8>> {
    let fin = reponse.windows(4).position(|w| w == b"\r\n\r\n")?;
    let (entetes, corps) = reponse.split_at(fin);
    let entetes = std::str::from_utf8(entetes).ok()?;
    let mut lignes = entetes.split("\r\n");
    let statut = lignes.next()?;
    if !(statut.starts_with("HTTP/1.1 ") || statut.starts_with("HTTP/1.0 "))
        || statut.chars().any(char::is_control)
    {
        return None;
    }
    // Statut final uniquement : un 1xx permettrait de glisser une seconde
    // réponse non assainie dans le corps (différence d'analyse).
    let code: u16 = statut.get(9..12)?.parse().ok()?;
    if !(200..=599).contains(&code) {
        return None;
    }
    // Corps : après la ligne vide ; le cadrage annoncé par le COG n'est pas
    // repris, la réponse est reconstruite avec une longueur exacte.
    let mut corps = corps.get(4..)?;
    let mut sortie = Vec::with_capacity(reponse.len() + CSP_SOUS_DOMAINE.len() + 96);
    sortie.extend_from_slice(statut.as_bytes());
    for ligne in lignes {
        if ligne.starts_with([' ', '\t']) || ligne.chars().any(char::is_control) {
            continue; // obs-fold ou caractère de contrôle : retiré
        }
        let Some((nom, valeur)) = ligne.split_once(':') else {
            continue;
        };
        let nom = nom.trim();
        if nom.eq_ignore_ascii_case("transfer-encoding") {
            // Encodage de transfert non pris en charge : refus (fail-closed).
            if !valeur.trim().eq_ignore_ascii_case("identity") {
                return None;
            }
            continue;
        }
        if nom.eq_ignore_ascii_case("content-length") {
            // Longueur annoncée : les octets au-delà sont ignorés ; une
            // longueur supérieure au corps reçu est refusée.
            let annonce: usize = valeur.trim().parse().ok()?;
            if annonce > corps.len() {
                return None;
            }
            corps = &corps[..annonce];
            continue;
        }
        if nom.eq_ignore_ascii_case("connection") || nom.eq_ignore_ascii_case("keep-alive") {
            continue;
        }
        if nom.eq_ignore_ascii_case("set-cookie2") {
            continue;
        }
        if nom.eq_ignore_ascii_case("set-cookie") && !cookie_hote_seul(valeur) {
            continue;
        }
        sortie.extend_from_slice(b"\r\n");
        sortie.extend_from_slice(ligne.as_bytes());
    }
    sortie.extend_from_slice(b"\r\n");
    sortie.extend_from_slice(CSP_SOUS_DOMAINE.as_bytes());
    sortie.extend_from_slice(b"\r\nX-Content-Type-Options: nosniff");
    sortie.extend_from_slice(format!("\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", corps.len()).as_bytes());
    sortie.extend_from_slice(corps);
    Some(sortie)
}

/// Serveur web public.
pub struct WebServer {
    config: Arc<OriginConfig>,
    pool_manager: Arc<PoolManager>,
    content_manager: Arc<ContentManager>,
    /// Suivi des visites catalogue -> Home (pour considérer un COG présent ~1 min après une visite).
    visit_tracker: Arc<CatalogVisitTracker>,
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
    /// Si `relay_ref` est fourni, Phase 2 (inject HTTP via tunnel) est activée.
    /// Si `forum_auth_store` est fourni, les routes POST /api/auth/forum/validate et /sync sont activées.
    #[must_use]
    pub fn new(
        config: Arc<OriginConfig>,
        pool_manager: Arc<PoolManager>,
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
                    // Derrière nginx (peer loopback), l'IP réelle n'est connue qu'après
                    // lecture des en-têtes : le contrôle est alors fait dans handle_connection.
                    if !is_trusted_proxy(addr.ip()) {
                        match self.rate_limiter.check_connection(addr.ip()).await {
                            RateLimitResult::Allowed => {}
                            _ => {
                                warn!("HTTP rate limit exceeded for {}", addr.ip());
                                continue;
                            }
                        }
                    }
                    let rate_limiter = Arc::clone(&self.rate_limiter);

                    let config = Arc::clone(&self.config);
                    let pool_mgr = Arc::clone(&self.pool_manager);
                    let content_mgr = Arc::clone(&self.content_manager);
                    let visit_tracker = Arc::clone(&self.visit_tracker);
                    let slug_registry = Arc::clone(&self.slug_registry);
                    let relay_ref = self.relay_ref.clone();
                    let forum_auth_store = self.forum_auth_store.clone();
                    let market_store = self.market_store.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(
                            stream,
                            config,
                            pool_mgr,
                            content_mgr,
                            visit_tracker,
                            slug_registry,
                            relay_ref,
                            forum_auth_store,
                            market_store,
                            addr.ip(),
                            rate_limiter,
                        )
                        .await
                        {
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

/// Plafond du corps pour les uploads de packages Market (64 Mio).
const MAX_PACKAGE_BODY: usize = 64 * 1024 * 1024;
/// Plafond du corps pour toutes les autres routes (1 Mio).
const MAX_DEFAULT_BODY: usize = 1024 * 1024;

/// Erreur de lecture du corps d'une requête.
#[derive(Debug, PartialEq, Eq)]
enum BodyError {
    /// Content-Length mal formé.
    Malformed,
    /// Content-Length supérieur au plafond (413).
    TooLarge,
}

/// Extrait Content-Length des headers et le compare au plafond, AVANT toute allocation.
/// Absent → 0.
fn content_length_checked(headers: &[String], max: usize) -> Result<usize, BodyError> {
    for line in headers {
        let lower = line.to_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            let n: usize = v.trim().parse().map_err(|_| BodyError::Malformed)?;
            if n > max {
                return Err(BodyError::TooLarge);
            }
            return Ok(n);
        }
    }
    Ok(0)
}

/// Horloge système (secondes Unix).
fn epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Réponse JSON du Market avec son statut HTTP.
fn market_json(code: u16, body: String) -> RouteResponse {
    let status = match code {
        200 => "200 OK",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        404 => "404 Not Found",
        409 => "409 Conflict",
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    RouteResponse::Normal { status: status.to_string(), content_type: "application/json".to_string(), body }
}

/// Refus d'une écriture Market (avant lecture du corps).
fn market_refus(e: &market::RefusMarket) -> RouteResponse {
    market_json(
        e.statut,
        serde_json::json!({ "success": false, "error": e.message }).to_string(),
    )
}

/// Réponse d'erreur pour un corps rejeté.
fn body_error_response(e: &BodyError) -> RouteResponse {
    let (status, msg) = match e {
        BodyError::TooLarge => ("413 Payload Too Large", "payload too large"),
        BodyError::Malformed => ("400 Bad Request", "invalid content-length"),
    };
    RouteResponse::Normal {
        status: status.to_string(),
        content_type: "application/json".to_string(),
        body: format!(r#"{{"ok":false,"success":false,"error":"{msg}"}}"#),
    }
}

/// Lit exactement `len` octets sans pré-allocation (la mémoire croît avec les données reçues).
async fn read_body(
    reader: &mut (impl AsyncReadExt + Unpin),
    len: usize,
) -> std::io::Result<Vec<u8>> {
    let mut body = Vec::new();
    let read = reader.take(len as u64).read_to_end(&mut body).await?;
    if read != len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "corps incomplet",
        ));
    }
    Ok(body)
}

/// Le pair est-il un proxy de confiance (nginx local) ? Seul le loopback l'est.
fn is_trusted_proxy(ip: std::net::IpAddr) -> bool {
    ip.to_canonical().is_loopback()
}

/// IP cliente réelle : `X-Real-IP` UNIQUEMENT si le pair est un proxy de confiance.
fn effective_client_ip(peer: std::net::IpAddr, headers: &[String]) -> Option<std::net::IpAddr> {
    if !is_trusted_proxy(peer) {
        return Some(peer);
    }
    for line in headers {
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("x-real-ip") {
                return v.trim().parse::<std::net::IpAddr>().ok().map(|ip| ip.to_canonical());
            }
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
    slug_registry: Arc<SlugRegistry>,
    relay_ref: Option<Arc<RelayServer>>,
    forum_auth_store: Option<Arc<forum_auth::ForumAuthStore>>,
    market_store: Option<Arc<market::MarketStore>>,
    peer_ip: std::net::IpAddr,
    rate_limiter: Arc<RateLimiter>,
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
        if let Some(value) = line
            .strip_prefix("Host:")
            .or_else(|| line.strip_prefix("host:"))
        {
            host_header = Some(value.trim().to_lowercase());
        }
        headers.push(line);
    }

    debug!("{} {} (Host: {:?})", method, path, host_header);

    // Rate limit par IP réelle quand le pair est le proxy local de confiance.
    if is_trusted_proxy(peer_ip) {
        if let Some(real_ip) = effective_client_ip(peer_ip, &headers) {
            if !matches!(
                rate_limiter.check_connection(real_ip).await,
                RateLimitResult::Allowed
            ) {
                warn!("HTTP rate limit exceeded for {} (via proxy)", real_ip);
                let resp = RouteResponse::Normal {
                    status: "429 Too Many Requests".to_string(),
                    content_type: "application/json".to_string(),
                    body: r#"{"ok":false,"success":false,"error":"rate_limited"}"#.to_string(),
                };
                write_response(&mut writer, resp).await?;
                return Ok(());
            }
        }
    }

    // Auth forum : POST /api/auth/* (passeport + visa + PoP, sync signée par COG).
    // En-têtes contrôlés avant toute lecture du corps (borné par `taille_max_corps`).
    if method == "POST" && path.starts_with("/api/auth/") {
        if let Some(store) = &forum_auth_store {
            if let Some(refus) = forum_auth::precontrole(path, &headers, store.as_ref()) {
                write_response(&mut writer, refus).await?;
                return Ok(());
            }
            let len = match content_length_checked(&headers, forum_auth::taille_max_corps(path)) {
                Ok(n) => n,
                Err(e) => {
                    write_response(&mut writer, body_error_response(&e)).await?;
                    return Ok(());
                }
            };
            let body = read_body(&mut buf_reader, len).await?;
            let route_response = forum_auth::handle_api(path, &body, &headers, store.as_ref());
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
                        if let Some(data) =
                            market::api_market_download(store, parts[0], parts[1]).await
                        {
                            let filename =
                                miyumarket::package::package_filename(parts[0], parts[1]);
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
                                body: r#"{"success":false,"error":"Package introuvable"}"#
                                    .to_string(),
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
                // POST /api/market/publish — requête signée par un éditeur officiel.
                // En-têtes (clé épinglée, horodatage, nonce) contrôlés AVANT de lire le corps.
                ("POST", market::CHEMIN_PUBLICATION) => {
                    let maintenant = epoch_secs();
                    match store.controler_entetes(&headers, maintenant) {
                        Err(e) => market_refus(&e),
                        Ok(entetes) => match content_length_checked(&headers, MAX_PACKAGE_BODY) {
                            Err(e) => body_error_response(&e),
                            Ok(len) => {
                                let body_bytes = read_body(&mut buf_reader, len).await?;
                                let (code, body) =
                                    market::api_market_publish(store, &entetes, &body_bytes, maintenant).await;
                                market_json(code, body)
                            }
                        },
                    }
                }
                // DELETE /api/market/package/{id}/{version}
                ("DELETE", _) if path_clean.starts_with("/api/market/package/") => {
                    let remainder = &path_clean[20..];
                    let parts: Vec<&str> = remainder.splitn(2, '/').collect();
                    if parts.len() == 2 {
                        // Requête signée, corps vide obligatoire (rien n'est lu).
                        let maintenant = epoch_secs();
                        match (
                            store.controler_entetes(&headers, maintenant),
                            content_length_checked(&headers, 0),
                        ) {
                            (Err(e), _) => market_refus(&e),
                            (_, Err(e)) => body_error_response(&e),
                            (Ok(entetes), Ok(_)) => {
                                let (code, body) = market::api_market_unpublish(
                                    store, &entetes, parts[0], parts[1], maintenant,
                                )
                                .await;
                                market_json(code, body)
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
                _ => RouteResponse::Normal {
                    status: "404 Not Found".to_string(),
                    content_type: "application/json".to_string(),
                    body: r#"{"success":false,"error":"Route Market inconnue"}"#.to_string(),
                },
            };
            write_response(&mut writer, route_response).await?;
            return Ok(());
        }
    }

    // O-04 : hôte bac à sable dédié à /visit (site distinct). Rien d'autre n'y est servi.
    if let Some(visit_host) = config.web.visit_host.as_deref() {
        let hote = host_header
            .as_deref()
            .map(|h| h.split(':').next().unwrap_or(h));
        if hote == Some(visit_host.trim().to_ascii_lowercase().as_str()) {
            let (chemin, query) = path.split_once('?').unwrap_or((path, ""));
            let route_response = if method == "GET" && chemin == "/visit" {
                visit_route(
                    query,
                    &visit_tracker,
                    &config,
                    &pool_mgr,
                    &slug_registry,
                    relay_ref.as_deref(),
                    true,
                )
                .await
            } else {
                not_found_page()
            };
            write_response(&mut writer, route_response).await?;
            return Ok(());
        }
    }

    // Vérifier si la requête est pour un sous-domaine COG
    let domain = config.identity.domain.as_deref().unwrap_or("miyukini.com");

    if let Some(slug) = extract_cog_slug(host_header.as_deref(), domain) {
        // Construire la requête HTTP brute pour Phase 2 (inject via tunnel)
        let raw_request = format!(
            "{}\r\n{}\r\n\r\n",
            request_line.trim_end(),
            headers
                .iter()
                .map(|h| h.trim_end())
                .collect::<Vec<_>>()
                .join("\r\n")
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
        &config,
        &slug_registry,
        relay_ref.as_deref(),
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
        RouteResponse::NonFiable {
            status,
            content_type,
            body,
            scripts,
        } => {
            let response = entetes_non_fiables(&status, &content_type, body.len(), scripts);
            writer.write_all(response.as_bytes()).await?;
            writer.write_all(&body).await?;
        }
    }
    writer.flush().await?;
    Ok(())
}

/// En-têtes d'une réponse au contenu non fiable (O-04) : bac à sable CSP (origine opaque,
/// sans `allow-same-origin`), `nosniff`, pas de cache, aucun cookie.
fn entetes_non_fiables(status: &str, content_type: &str, longueur: usize, scripts: bool) -> String {
    let sandbox = if scripts {
        "sandbox allow-scripts allow-forms allow-popups allow-popups-to-escape-sandbox"
    } else {
        "sandbox allow-forms allow-popups allow-popups-to-escape-sandbox"
    };
    format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Content-Security-Policy: {}; frame-ancestors 'none'\r\n\
         X-Content-Type-Options: nosniff\r\n\
         X-Frame-Options: DENY\r\n\
         Referrer-Policy: no-referrer\r\n\
         Cache-Control: no-store\r\n\
         Cross-Origin-Resource-Policy: same-origin\r\n\
         Connection: close\r\n\
         \r\n",
        entete_sur(status, "502 Bad Gateway"),
        entete_sur(content_type, "application/octet-stream"),
        longueur,
        sandbox,
    )
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
    let reserved = [
        "www", "origin", "api", "admin", "relay", "tracker", "mail", "docs", "blog", "status",
        "forum",
    ];
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
        match relay
            .inject_http_request(&cog_id, raw_request.clone())
            .await
        {
            Ok(response_bytes) => {
                info!(
                    "COG subdomain Phase 2 (tunnel): {}.{} (cog_id: {})",
                    slug,
                    config.identity.domain.as_deref().unwrap_or("miyukini.com"),
                    cog_id
                );
                return match assainir_reponse_sous_domaine(&response_bytes) {
                    Some(assainie) => raw_http_response_to_route(&assainie),
                    None => bad_gateway(),
                };
            }
            Err(e) => {
                debug!(
                    "Phase 2 tunnel not available for '{}' ({}), falling back to Phase 1: {}",
                    slug, cog_id, e
                );
            }
        }
    }

    // Phase 1 (ou fallback) : proxy HTTP direct vers le COG (cibles publiques seules, O-03)
    if config.web.disable_direct_proxy {
        return cog_offline_page(slug, &cog_id, config);
    }
    let entry = pool_mgr.find_cog(&cog_id).await;

    match entry {
        Some(entry) => {
            let Ok(adresse) = analyser_adresse(&entry.address) else {
                warn!("COG {} has an invalid stored address, proxy refused", cog_id);
                return cog_offline_page(slug, &cog_id, config);
            };
            info!(
                "COG subdomain proxy: {}.{} -> {} (cog_id: {})",
                slug,
                config.identity.domain.as_deref().unwrap_or("miyukini.com"),
                adresse.canonique(),
                cog_id
            );

            match proxy_http_request(&adresse, path).await {
                // Contenu du COG : non fiable, en bac à sable sans `allow-same-origin` (O-04).
                Ok(reponse) => RouteResponse::NonFiable {
                    status: reponse.status,
                    content_type: reponse.content_type,
                    body: reponse.body,
                    scripts: true,
                },
                Err(e) => {
                    warn!(
                        "COG proxy failed for slug '{}' (cog_id: {}): {}",
                        slug, cog_id, e
                    );
                    cog_offline_page(slug, &cog_id, config)
                }
            }
        }
        None => cog_offline_page(slug, &cog_id, config),
    }
}

/// Convertit une réponse HTTP brute (Phase 2 tunnel) en RouteResponse.
fn raw_http_response_to_route(response: &[u8]) -> RouteResponse {
    RouteResponse::Raw {
        bytes: response.to_vec(),
    }
}

/// Réponse HTTP minimale lue depuis un COG.
struct ReponseCog {
    status: String,
    content_type: String,
    location: Option<String>,
    body: Vec<u8>,
}

/// Découpe une réponse HTTP brute (statut, Content-Type, Location, corps).
fn decouper_reponse_http(brute: &[u8]) -> Option<ReponseCog> {
    let fin = brute.windows(4).position(|w| w == b"\r\n\r\n")?;
    let entetes = String::from_utf8_lossy(&brute[..fin]);
    let mut lignes = entetes.split("\r\n");
    let status = lignes
        .next()
        .and_then(|l| l.strip_prefix("HTTP/1.1 ").or_else(|| l.strip_prefix("HTTP/1.0 ")))
        .map(|s| entete_sur(s, "502 Bad Gateway"))?;
    let mut content_type = "text/html".to_string();
    let mut location = None;
    for ligne in lignes {
        if let Some((nom, valeur)) = ligne.split_once(':') {
            let nom = nom.trim().to_ascii_lowercase();
            if nom == "content-type" {
                content_type = entete_sur(valeur.trim(), "text/html");
            } else if nom == "location" {
                location = Some(valeur.trim().chars().filter(|c| !c.is_control()).collect());
            }
        }
    }
    Some(ReponseCog {
        status,
        content_type,
        location,
        body: brute[fin + 4..].to_vec(),
    })
}

/// Proxy HTTP direct vers le COG (Phase 1).
///
/// O-03 : la cible est résolue et **chaque** IP obtenue doit être publique ; la connexion
/// est ouverte vers l'IP vérifiée (pas de seconde résolution), à chaque requête ; aucune
/// redirection n'est suivie ; la réponse est bornée (O-10).
async fn proxy_http_request(
    adresse: &AdresseAnalysee,
    path: &str,
) -> Result<ReponseCog, Box<dyn std::error::Error + Send + Sync>> {
    if !path.starts_with('/') || path.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return Err("invalid path".into());
    }
    let mut stream =
        connecter_cible_publique(adresse, std::time::Duration::from_secs(5)).await?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: MiyukiniOriginProxy/1.0\r\n\r\n",
        path,
        adresse.entete_host()
    );
    stream.write_all(request.as_bytes()).await?;

    let mut response_buf = Vec::new();
    let limite = u64::try_from(TAILLE_MAX_REPONSE_COG).unwrap_or(u64::MAX);
    tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        (&mut stream).take(limite).read_to_end(&mut response_buf),
    )
    .await
    .map_err(|_| "response timeout (10s)")??;

    decouper_reponse_http(&response_buf).ok_or_else(|| "Invalid response from COG".into())
}

/// Page affichée quand le slug ne correspond à aucun COG enregistré.
fn cog_not_found_page(slug: &str, config: &OriginConfig) -> RouteResponse {
    let domain = config.identity.domain.as_deref().unwrap_or("miyukini.com");
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
    let domain = config.identity.domain.as_deref().unwrap_or("miyukini.com");
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
    config: &OriginConfig,
    slug_registry: &SlugRegistry,
    relay_ref: Option<&RelayServer>,
) -> RouteResponse {
    // Séparer path et query string
    let (path_only, query) = path.split_once('?').unwrap_or((path, ""));

    // Nettoyer le path
    let path_clean = path_only.trim_end_matches('/');
    let path_clean = if path_clean.is_empty() {
        "/"
    } else {
        path_clean
    };

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
            if filename.is_empty()
                || filename.len() > 255
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
                    } else if filename.ends_with(".apk") {
                        "application/vnd.android.package-archive"
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

        "/catalog" => RouteResponse::Redirect {
            location: "/mws".to_string(),
        },

        "/visit" => {
            // O-04 : si un hôte bac à sable dédié est configuré, /visit n'est jamais servi
            // sur l'origine principale.
            if let Some(visit_host) = config.web.visit_host.as_deref() {
                return RouteResponse::Redirect {
                    location: format!("https://{visit_host}/visit?{query}"),
                };
            }
            visit_route(query, visit_tracker, config, pool_mgr, slug_registry, relay_ref, false)
                .await
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

        "/mip" => {
            let body = pages::mip_page();
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
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

        "/pricing" | "/tarifs" => {
            let body = pages::pricing_page();
            RouteResponse::Normal {
                status: "200 OK".to_string(),
                content_type: "text/html".to_string(),
                body,
            }
        }

        "/studio" => {
            let body = pages::studio_page();
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
                    if let Some(article) = content_mgr.get_doc_article(section_id, article_id).await
                    {
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
                    if let Some(body) =
                        api::api_doc_article(content_mgr, section_id, article_id).await
                    {
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

/// Traite `/visit?cog_id=…&path=…` (identifiant validé, visite enregistrée pour un COG connu).
async fn visit_route(
    query: &str,
    visit_tracker: &CatalogVisitTracker,
    config: &OriginConfig,
    pool_mgr: &PoolManager,
    slug_registry: &SlugRegistry,
    relay_ref: Option<&RelayServer>,
    bac_a_sable_dedie: bool,
) -> RouteResponse {
    match parse_cog_id_from_query(query) {
        Some(id) if miyuwebway_participant::identite::cog_id_bien_forme(&id) => {
            let Some(target_path) = parse_visit_path_from_query(query).or_else(|| {
                (!query.contains("path=")).then(|| "/".to_string())
            }) else {
                return not_found_page();
            };
            // O-15 : n'enregistrer de visite que pour un COG connu.
            if pool_mgr.find_cog(&id).await.is_some() {
                visit_tracker.record_visit(&id).await;
            }
            handle_cog_visit_request(
                &id,
                &target_path,
                config,
                pool_mgr,
                slug_registry,
                relay_ref,
                bac_a_sable_dedie,
            )
            .await
        }
        _ => not_found_page(),
    }
}

fn parse_cog_id_from_query(query: &str) -> Option<String> {
    query.split('&').find_map(|part| {
        let value = part.strip_prefix("cog_id=")?;
        let value = value.split('&').next().unwrap_or(value);
        urlencoding::decode(value)
            .ok()
            .map(|decoded| decoded.into_owned())
    })
}

fn parse_visit_path_from_query(query: &str) -> Option<String> {
    let raw_path = query.split('&').find_map(|part| {
        let value = part.strip_prefix("path=")?;
        let value = value.split('&').next().unwrap_or(value);
        urlencoding::decode(value)
            .ok()
            .map(|decoded| decoded.into_owned())
    })?;

    normalize_visit_path(&raw_path)
}

fn normalize_visit_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return Some("/".to_string());
    }
    if path.chars().any(|c| c.is_control() || c.is_whitespace())
        || path.contains("://")
        || path.starts_with("//")
        || path.contains('\\')
    {
        return None;
    }

    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };

    Some(normalized)
}

/// Sert `/visit` : contenu d'un COG, toujours en bac à sable (O-04).
///
/// `bac_a_sable_dedie` : vrai quand la requête arrive sur l'hôte `web.visit_host` (site
/// distinct du domaine principal) ; les scripts y sont alors permis dans une origine opaque.
/// Sinon (repli sur l'origine principale), le bac à sable interdit tout script.
async fn handle_cog_visit_request(
    cog_id: &str,
    target_path: &str,
    config: &OriginConfig,
    pool_mgr: &PoolManager,
    slug_registry: &SlugRegistry,
    relay_ref: Option<&RelayServer>,
    bac_a_sable_dedie: bool,
) -> RouteResponse {
    let slug = slug_registry
        .get_slug(cog_id)
        .await
        .unwrap_or_else(|| SlugRegistry::sanitize_cog_id(cog_id));

    // Requête construite par Origin : aucun cookie ni en-tête du visiteur n'est transmis.
    let raw_request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: MiyukiniOriginVisit/1.0\r\n\r\n",
        target_path, cog_id
    );

    if let Some(relay) = relay_ref {
        match relay
            .inject_http_request(cog_id, Bytes::from(raw_request.clone()))
            .await
        {
            Ok(response_bytes) => {
                return match decouper_reponse_http(&response_bytes) {
                    Some(reponse) => reponse_visite(reponse, cog_id, bac_a_sable_dedie),
                    None => bad_gateway(),
                };
            }
            Err(e) => {
                debug!(
                    "Visit tunnel unavailable for '{}' (slug '{}'), falling back to direct proxy: {}",
                    cog_id, slug, e
                );
            }
        }
    }

    if config.web.disable_direct_proxy {
        return cog_offline_page(&slug, cog_id, config);
    }

    match pool_mgr.find_cog(cog_id).await {
        Some(entry) => {
            let Ok(adresse) = analyser_adresse(&entry.address) else {
                return cog_offline_page(&slug, cog_id, config);
            };
            match proxy_http_request(&adresse, target_path).await {
                Ok(reponse) => reponse_visite(reponse, cog_id, bac_a_sable_dedie),
                Err(e) => {
                    warn!(
                        "Visit proxy failed for cog '{}' (slug '{}', path '{}'): {}",
                        cog_id, slug, target_path, e
                    );
                    cog_offline_page(&slug, cog_id, config)
                }
            }
        }
        None => cog_not_found_page(&slug, config),
    }
}

/// Réponse 502 générique.
fn bad_gateway() -> RouteResponse {
    RouteResponse::Normal {
        status: "502 Bad Gateway".to_string(),
        content_type: "text/plain".to_string(),
        body: "Invalid response from COG".to_string(),
    }
}

/// Convertit la réponse d'un COG pour `/visit` : jamais d'en-tête du COG recopié
/// (donc aucun `Set-Cookie`), HTML réécrit, contenu servi en bac à sable.
fn reponse_visite(reponse: ReponseCog, cog_id: &str, scripts: bool) -> RouteResponse {
    if reponse.status.starts_with("30") {
        if let Some(location) = reponse.location {
            return redirection_visite(&location, cog_id);
        }
    }
    let body = if reponse
        .content_type
        .to_ascii_lowercase()
        .starts_with("text/html")
    {
        rewrite_visit_html(&String::from_utf8_lossy(&reponse.body), cog_id).into_bytes()
    } else {
        reponse.body
    };
    RouteResponse::NonFiable {
        status: reponse.status,
        content_type: reponse.content_type,
        body,
        scripts,
    }
}

/// Redirection émise par un COG pendant une visite (O-16) : seuls les chemins relatifs
/// sont suivis dans `/visit` ; une URL absolue donne une page intermédiaire.
fn redirection_visite(location: &str, cog_id: &str) -> RouteResponse {
    if location.starts_with('/') && !location.starts_with("//") && !location.contains('\\') {
        return RouteResponse::Redirect {
            location: rewrite_visit_location(location, cog_id),
        };
    }
    let lien = pages::html_escape(location);
    RouteResponse::Normal {
        status: "200 OK".to_string(),
        content_type: "text/html".to_string(),
        body: format!(
            "<!DOCTYPE html><html lang=\"fr\"><head><meta charset=\"UTF-8\"><title>Vous quittez Miyukini</title></head>\
             <body><h1>Vous quittez Miyukini</h1><p>Ce COG vous redirige vers un site externe :</p>\
             <p><code>{lien}</code></p><p><a href=\"{lien}\" rel=\"noopener noreferrer nofollow\">Continuer</a></p></body></html>"
        ),
    }
}

fn rewrite_visit_html(html: &str, cog_id: &str) -> String {
    let mut output = html.to_string();
    for attr in ["href", "src", "action"] {
        output = rewrite_visit_attr_paths(&output, attr, cog_id);
    }
    output
}

fn rewrite_visit_attr_paths(html: &str, attr: &str, cog_id: &str) -> String {
    let needle = format!("{attr}=\"/");
    let mut rewritten = String::with_capacity(html.len());
    let mut cursor = 0;

    while let Some(found) = html[cursor..].find(&needle) {
        let start = cursor + found;
        let path_start = start + attr.len() + 2;
        let Some(path_end_rel) = html[path_start..].find('"') else {
            break;
        };
        let path_end = path_start + path_end_rel;
        let path = &html[path_start..path_end];

        rewritten.push_str(&html[cursor..start]);
        rewritten.push_str(attr);
        rewritten.push_str("=\"");
        rewritten.push_str(&rewrite_visit_location(path, cog_id));
        rewritten.push('"');
        cursor = path_end + 1;
    }

    rewritten.push_str(&html[cursor..]);
    rewritten
}

fn rewrite_visit_location(location: &str, cog_id: &str) -> String {
    if location.starts_with('/') && !location.starts_with("//") {
        format!(
            "/visit?cog_id={}&path={}",
            urlencoding::encode(cog_id),
            urlencoding::encode(location)
        )
    } else {
        location.to_string()
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
</html>"#
        .to_string();

    RouteResponse::Normal {
        status: "404 Not Found".to_string(),
        content_type: "text/html".to_string(),
        body,
    }
}

/// Page d'une section de documentation.
fn doc_section_page(section: &super::content::DocSection) -> String {
    let articles_html: String = section
        .articles
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
            let cells: Vec<&str> = line
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            if !in_table {
                html.push_str(r#"<table class="doc-table"><thead><tr>"#);
                for cell in &cells {
                    html.push_str("<th>");
                    html.push_str(&md_links_to_html(cell));
                    html.push_str("</th>");
                }
                html.push_str("</tr></thead><tbody>");
                in_table = true;
            } else if cells
                .iter()
                .all(|c| c.chars().all(|ch| ch == '-' || ch == ':'))
            {
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
            html.push_str(&format!(
                "<blockquote>{}</blockquote>\n",
                md_links_to_html(&line[2..])
            ));
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

#[cfg(test)]
mod tests {
    use super::{
        extract_cog_slug, normalize_visit_path, parse_cog_id_from_query, rewrite_visit_location,
    };

    #[test]
    fn parse_cog_id_from_query_decodes_value() {
        let cog_id = parse_cog_id_from_query("foo=bar&cog_id=Mon%20COG&x=1");
        assert_eq!(cog_id.as_deref(), Some("Mon COG"));
    }

    #[test]
    fn extract_cog_slug_ignores_main_domain_and_reserved_hosts() {
        assert_eq!(extract_cog_slug(Some("miyukini.com"), "miyukini.com"), None);
        assert_eq!(
            extract_cog_slug(Some("origin.miyukini.com"), "miyukini.com"),
            None
        );
        assert_eq!(
            extract_cog_slug(Some("miyukini-cog.miyukini.com"), "miyukini.com"),
            Some("miyukini-cog".to_string())
        );
    }

    #[test]
    fn normalize_visit_path_rejects_absolute_urls() {
        assert_eq!(
            normalize_visit_path("service"),
            Some("/service".to_string())
        );
        assert_eq!(
            normalize_visit_path("/service"),
            Some("/service".to_string())
        );
        assert_eq!(normalize_visit_path("https://evil.invalid"), None);
    }

    #[test]
    fn rewrite_visit_location_wraps_root_relative_paths() {
        assert_eq!(
            rewrite_visit_location("/service/catalogue", "Miyukini"),
            "/visit?cog_id=Miyukini&path=%2Fservice%2Fcatalogue"
        );
        assert_eq!(
            rewrite_visit_location("https://miyukini.com", "Miyukini"),
            "https://miyukini.com"
        );
    }

    #[test]
    fn content_length_plafonne_avant_allocation() {
        let h = vec!["Content-Length: 2000000\r\n".to_string()];
        assert_eq!(super::content_length_checked(&h, 1024 * 1024), Err(super::BodyError::TooLarge));
        assert_eq!(super::content_length_checked(&h, 64 * 1024 * 1024), Ok(2_000_000));
        let bad = vec!["Content-Length: abc\r\n".to_string()];
        assert_eq!(super::content_length_checked(&bad, 10), Err(super::BodyError::Malformed));
        assert_eq!(super::content_length_checked(&[], 10), Ok(0));
    }

    #[test]
    fn x_real_ip_lu_uniquement_depuis_loopback() {
        let h = vec!["X-Real-IP: 203.0.113.7\r\n".to_string()];
        let lo: std::net::IpAddr = "127.0.0.1".parse().unwrap();
        let remote: std::net::IpAddr = "198.51.100.9".parse().unwrap();
        assert_eq!(
            super::effective_client_ip(lo, &h),
            Some("203.0.113.7".parse().unwrap())
        );
        // Pair non fiable : l'en-tête usurpé est ignoré.
        assert_eq!(super::effective_client_ip(remote, &h), Some(remote));
        // Loopback sans en-tête valide : pas d'IP exploitable.
        assert_eq!(super::effective_client_ip(lo, &[]), None);
    }

    #[test]
    fn visite_servie_en_bac_a_sable_sans_cookie() {
        let brute = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nSet-Cookie: s=1; Domain=miyukini.com\r\n\r\n<script>fetch('/admin/api/config')</script>";
        let reponse = super::decouper_reponse_http(brute).expect("réponse");
        let route = super::reponse_visite(reponse, "aaaaaaaaaaaaaaaaaaaaaaaaaa", false);
        let super::RouteResponse::NonFiable {
            status,
            content_type,
            body,
            scripts,
        } = route
        else {
            panic!("contenu COG non marqué non fiable");
        };
        assert!(!scripts);
        let entetes = super::entetes_non_fiables(&status, &content_type, body.len(), scripts);
        assert!(entetes.contains("Content-Security-Policy: sandbox allow-forms"));
        assert!(!entetes.contains("allow-scripts"));
        assert!(!entetes.contains("allow-same-origin"));
        assert!(entetes.contains("X-Content-Type-Options: nosniff"));
        assert!(!entetes.to_ascii_lowercase().contains("set-cookie"));
        // Hôte dédié : scripts permis mais toujours en origine opaque.
        let dedie = super::entetes_non_fiables("200 OK", "text/html", 0, true);
        assert!(dedie.contains("allow-scripts"));
        assert!(!dedie.contains("allow-same-origin"));
    }

    #[test]
    fn entetes_cog_sans_injection_crlf() {
        let e = super::entetes_non_fiables("200 OK\r\nSet-Cookie: x=1", "text/html\r\nX: y", 0, false);
        assert!(!e.contains("\r\nSet-Cookie"));
        assert!(!e.contains("\r\nX: y"));
    }

    #[test]
    fn redirection_absolue_du_cog_page_intermediaire() {
        match super::redirection_visite("https://evil.example/<script>", "aaaaaaaaaaaaaaaaaaaaaaaaaa") {
            super::RouteResponse::Normal { body, .. } => {
                assert!(!body.contains("<script>"));
                assert!(body.contains("&lt;script&gt;"));
            }
            _ => panic!("une URL absolue ne doit pas produire de 302"),
        }
        assert!(matches!(
            super::redirection_visite("//evil.example", "aaaaaaaaaaaaaaaaaaaaaaaaaa"),
            super::RouteResponse::Normal { .. }
        ));
        assert!(matches!(
            super::redirection_visite("/page", "aaaaaaaaaaaaaaaaaaaaaaaaaa"),
            super::RouteResponse::Redirect { .. }
        ));
    }

    #[test]
    fn sous_domaine_ne_peut_pas_poser_de_cookie_parent() {
        let brute = b"HTTP/1.1 200 OK\r\nSet-Cookie: a=1; Path=/; Secure\r\nset-cookie: b=2; Domain=.miyukini.com\r\nSet-Cookie: c=3; path=/; DOMAIN = miyukini.com\r\nSet-Cookie: __Host-admin_session=x; Path=/; domain=miyukini.com\r\nSet-Cookie2: d=4\r\nX-Pli: ok\r\n Set-Cookie: e=5; Domain=miyukini.com\r\n\r\ncorps";
        let r = String::from_utf8(super::assainir_reponse_sous_domaine(brute).expect("réponse"))
            .expect("utf8");
        // Cookie hôte seul : conservé (confiné au sous-domaine du COG).
        assert!(r.contains("Set-Cookie: a=1"));
        // Tout cookie avec Domain (quelle que soit la casse ou les espaces) : retiré.
        assert!(!r.contains("b=2"));
        assert!(!r.contains("c=3"));
        assert!(!r.contains("__Host-admin_session"));
        assert!(!r.contains("d=4"));
        // Ligne repliée (obs-fold) : retirée.
        assert!(!r.contains("e=5"));
        // Bac à sable CSP sans allow-same-origin.
        assert!(r.contains("Content-Security-Policy: sandbox allow-scripts"));
        assert!(!r.contains("allow-same-origin"));
        assert!(r.contains("X-Content-Type-Options: nosniff"));
        assert!(r.ends_with("Content-Length: 5\r\nConnection: close\r\n\r\ncorps"));
    }

    #[test]
    fn sous_domaine_reponse_mal_formee_refusee() {
        assert!(super::assainir_reponse_sous_domaine(b"pas du http").is_none());
        assert!(super::assainir_reponse_sous_domaine(b"FOO 200\r\n\r\n").is_none());
        assert!(super::assainir_reponse_sous_domaine(b"HTTP/1.1 200 OK\r\nX: \xff\r\n\r\n").is_none());
    }

    #[test]
    fn sous_domaine_pas_de_seconde_reponse_glissee() {
        // 1xx suivi d'une seconde réponse dans le « corps » : refus.
        let continu = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nSet-Cookie: x=1; Domain=miyukini.com\r\n\r\npiege";
        assert!(super::assainir_reponse_sous_domaine(continu).is_none());
        // Encodage de transfert non identité : refus.
        let chunk = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\n\r\n";
        assert!(super::assainir_reponse_sous_domaine(chunk).is_none());
        // Longueur annoncée plus grande que le corps : refus.
        let long = b"HTTP/1.1 200 OK\r\nContent-Length: 99\r\n\r\ncourt";
        assert!(super::assainir_reponse_sous_domaine(long).is_none());
        // Octets au-delà de Content-Length (seconde réponse) : ignorés, cadrage exact.
        let colle = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\nokHTTP/1.1 200 OK\r\nSet-Cookie: y=1; Domain=miyukini.com\r\n\r\nx";
        let r = String::from_utf8(super::assainir_reponse_sous_domaine(colle).expect("réponse"))
            .expect("utf8");
        assert!(r.ends_with("Content-Length: 2\r\nConnection: close\r\n\r\nok"));
        assert!(!r.contains("y=1"));
        assert!(!r.contains("keep-alive"));
    }

    #[test]
    fn chemins_de_visite_dangereux_refuses() {
        assert_eq!(normalize_visit_path("//evil.example"), None);
        assert_eq!(normalize_visit_path("/a b"), None);
        assert_eq!(normalize_visit_path("/a\\b"), None);
    }
}
