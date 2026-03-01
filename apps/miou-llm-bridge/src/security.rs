//! Sécurité — Auth, rate limiting, audit, sandboxing, anti-MITM, chiffrement.
//!
//! Garantit la sécurité des données, des requêtes et la disponibilité.
//! - Bearer token authentication
//! - Rate limiting par agent et par endpoint
//! - Audit log de toutes les actions (agents + sécurité)
//! - Vérification des permissions (agent → skill)
//! - Validation d'origine (anti-appels externes non autorisés)
//! - Signature HMAC-SHA256 (anti-MITM pour les échanges inter-services COG)
//! - Chiffrement AES-256-GCM des réponses sensibles

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::agents::{AgentDef, SecurityLevel};
use crate::config::SecurityConfig;

// ── Rate Limiting ────────────────────────────────────────────────────

/// Configuration du rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Nombre max de requêtes par fenêtre.
    pub max_requests: u32,
    /// Durée de la fenêtre.
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window: Duration::from_secs(60),
        }
    }
}

/// État du rate limiter pour un client.
#[derive(Debug, Clone)]
struct RateLimitState {
    count: u32,
    window_start: Instant,
}

/// Rate limiter global.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    states: Arc<RwLock<HashMap<String, RateLimitState>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Vérifie si une requête est autorisée pour un identifiant donné.
    /// Retourne Ok(remaining) ou Err(retry_after_secs).
    pub fn check(&self, identifier: &str) -> Result<u32, u64> {
        let mut states = self.states.write().unwrap();
        let now = Instant::now();

        let state = states.entry(identifier.to_string()).or_insert(RateLimitState {
            count: 0,
            window_start: now,
        });

        // Réinitialiser la fenêtre si expirée
        if now.duration_since(state.window_start) >= self.config.window {
            state.count = 0;
            state.window_start = now;
        }

        if state.count >= self.config.max_requests {
            let elapsed = now.duration_since(state.window_start);
            let retry_after = self.config.window.as_secs().saturating_sub(elapsed.as_secs());
            Err(retry_after)
        } else {
            state.count += 1;
            Ok(self.config.max_requests - state.count)
        }
    }

    /// Nettoie les entrées expirées (à appeler périodiquement).
    pub fn cleanup(&self) {
        let mut states = self.states.write().unwrap();
        let now = Instant::now();
        states.retain(|_, state| now.duration_since(state.window_start) < self.config.window * 2);
    }
}

// ── Audit Log ────────────────────────────────────────────────────────

/// Entrée d'audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Timestamp (epoch seconds).
    pub timestamp: u64,
    /// ID de l'agent qui a effectué l'action.
    pub agent_id: String,
    /// Skill utilisé.
    pub skill_id: String,
    /// Action effectuée (read, write, exec, etc.).
    pub action: String,
    /// Détails (path, command, etc.).
    pub details: String,
    /// Succès ou échec.
    pub success: bool,
    /// IP ou identifiant du client.
    pub client_id: String,
}

/// Journal d'audit (en mémoire, avec rotation).
#[derive(Debug, Clone)]
pub struct AuditLog {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    max_entries: usize,
}

impl AuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            max_entries,
        }
    }

    /// Enregistre une entrée d'audit.
    pub fn log(&self, entry: AuditEntry) {
        let mut entries = self.entries.write().unwrap();
        entries.push(entry);

        // Rotation si trop d'entrées
        if entries.len() > self.max_entries {
            let drain_count = entries.len() - self.max_entries;
            entries.drain(..drain_count);
        }
    }

    /// Récupère les N dernières entrées.
    pub fn recent(&self, count: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read().unwrap();
        let start = entries.len().saturating_sub(count);
        entries[start..].to_vec()
    }

    /// Filtre par agent.
    pub fn by_agent(&self, agent_id: &str) -> Vec<AuditEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|e| e.agent_id == agent_id)
            .cloned()
            .collect()
    }
}

// ── Vérification des permissions ─────────────────────────────────────

/// Résultat de vérification de permission.
#[derive(Debug)]
pub enum PermissionCheck {
    Allowed,
    Denied(String),
}

/// Vérifie qu'un agent a le droit d'utiliser un skill.
pub fn check_agent_skill_permission(agent: &AgentDef, skill_id: &str) -> PermissionCheck {
    // Vérifier que le skill est dans la liste autorisée de l'agent
    if !agent.skill_ids.contains(&skill_id.to_string()) {
        return PermissionCheck::Denied(format!(
            "Agent '{}' n'est pas autorisé à utiliser le skill '{skill_id}'",
            agent.display_name
        ));
    }

    // Vérifier le niveau de sécurité minimum du skill
    let required_level = match skill_id {
        "shell_exec" => SecurityLevel::Extended,
        "file_ops" => SecurityLevel::Sandboxed,
        "web_fetch" => SecurityLevel::Sandboxed,
        "cog_services" => SecurityLevel::Sandboxed,
        _ => SecurityLevel::ReadOnly,
    };

    if (agent.security_level as u8) < (required_level as u8) {
        return PermissionCheck::Denied(format!(
            "Agent '{}' n'a pas le niveau de sécurité requis pour '{skill_id}' \
             (a: {:?}, requis: {:?})",
            agent.display_name, agent.security_level, required_level
        ));
    }

    PermissionCheck::Allowed
}

/// Vérifie le token d'authentification.
pub fn verify_auth_token(
    headers: &axum::http::HeaderMap,
    expected_token: &Option<String>,
) -> Result<(), &'static str> {
    let Some(expected) = expected_token else {
        return Ok(()); // Pas de token configuré = accès libre
    };

    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(value) => {
            let token = value.strip_prefix("Bearer ").unwrap_or(value);
            if token == expected {
                Ok(())
            } else {
                Err("Token invalide")
            }
        }
        None => Err("Header Authorization manquant"),
    }
}

/// Timestamp epoch actuel.
pub fn epoch_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ═══════════════════════════════════════════════════════════════════════
// Validation d'origine (anti-appels externes non autorisés)
// ═══════════════════════════════════════════════════════════════════════

/// Adresses loopback toujours autorisées.
const LOOPBACK_ADDRS: &[&str] = &["127.0.0.1", "::1", "localhost"];

/// User-Agents COG reconnus.
const COG_USER_AGENTS: &[&str] = &[
    "miyukini-central",
    "miyukini-ai-studio",
    "jaykonta",
    "jaykoa",
    "miyukini-watch",
    "cog-service",
];

/// Résultat de la vérification d'origine.
#[derive(Debug)]
pub enum OriginCheck {
    Allowed,
    Blocked(String),
}

/// Vérifie qu'une requête provient d'une origine autorisée.
///
/// Règles :
/// 1. Les adresses loopback (127.0.0.1, ::1) sont toujours autorisées
/// 2. Si `allowed_origins` est configuré, seules ces origines sont acceptées
/// 3. Si `strict_user_agent` est activé, le User-Agent doit contenir un identifiant COG
pub fn check_origin(
    remote_addr: Option<&str>,
    headers: &axum::http::HeaderMap,
    config: &SecurityConfig,
) -> OriginCheck {
    // Extraire l'IP source depuis le header ou l'adresse de connexion
    let source_ip = remote_addr
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.split(',').next().unwrap_or(v).trim())
        })
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
        })
        .unwrap_or("unknown");

    // Nettoyer l'IP (enlever le port si présent, ex: "127.0.0.1:54321")
    let clean_ip = source_ip
        .rsplit_once(':')
        .and_then(|(ip, port)| {
            // Distinguer IPv6 (::1) de IPv4:port (127.0.0.1:54321)
            if port.parse::<u16>().is_ok() && !ip.contains(':') {
                Some(ip)
            } else {
                None
            }
        })
        .unwrap_or(source_ip);

    // 1. Loopback = toujours OK
    if LOOPBACK_ADDRS.contains(&clean_ip) {
        return OriginCheck::Allowed;
    }

    // Vérifier si c'est une IP loopback parsée
    if let Ok(ip) = clean_ip.parse::<IpAddr>() {
        if ip.is_loopback() {
            return OriginCheck::Allowed;
        }
    }

    // 2. Vérifier les origines autorisées (si configurées)
    if !config.allowed_origins.is_empty() {
        let is_allowed = config.allowed_origins.iter().any(|allowed| {
            if allowed.contains('/') {
                // Support CIDR basique (ex: "192.168.1.0/24")
                match_cidr(clean_ip, allowed)
            } else {
                clean_ip == allowed
            }
        });

        if !is_allowed {
            return OriginCheck::Blocked(format!(
                "Origine non autorisée : {clean_ip} (pas dans allowed_origins)"
            ));
        }
    }

    // 3. Vérifier le User-Agent COG si strict_user_agent est activé
    if config.strict_user_agent {
        let ua = headers
            .get(axum::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let ua_lower = ua.to_lowercase();
        let is_cog = COG_USER_AGENTS.iter().any(|agent| ua_lower.contains(agent));

        if !is_cog {
            return OriginCheck::Blocked(format!(
                "User-Agent non reconnu : '{ua}' (strict_user_agent activé)"
            ));
        }
    }

    OriginCheck::Allowed
}

/// Vérification CIDR simplifiée pour IPv4 (ex: "192.168.1.0/24").
fn match_cidr(ip_str: &str, cidr: &str) -> bool {
    let Some((network_str, prefix_str)) = cidr.split_once('/') else {
        return ip_str == cidr;
    };

    let Ok(prefix_len) = prefix_str.parse::<u32>() else {
        return false;
    };

    let (Ok(ip), Ok(network)) = (
        ip_str.parse::<std::net::Ipv4Addr>(),
        network_str.parse::<std::net::Ipv4Addr>(),
    ) else {
        return false;
    };

    if prefix_len > 32 {
        return false;
    }

    let mask = if prefix_len == 0 {
        0u32
    } else {
        !0u32 << (32 - prefix_len)
    };

    let ip_bits = u32::from(ip);
    let net_bits = u32::from(network);

    (ip_bits & mask) == (net_bits & mask)
}

// ═══════════════════════════════════════════════════════════════════════
// Signature HMAC-SHA256 (anti-MITM)
// ═══════════════════════════════════════════════════════════════════════

/// Headers utilisés pour la signature inter-services COG.
pub const HEADER_COG_SIGNATURE: &str = "x-cog-signature";
pub const HEADER_COG_TIMESTAMP: &str = "x-cog-timestamp";
pub const HEADER_COG_SERVICE: &str = "x-cog-service";
pub const HEADER_COG_ENCRYPT: &str = "x-cog-encrypt";

/// Résultat de la vérification HMAC.
#[derive(Debug)]
pub enum HmacCheck {
    /// Pas de HMAC configuré — vérification ignorée.
    NotConfigured,
    /// Signature valide.
    Valid { service_id: String },
    /// Signature invalide ou expirée.
    Invalid(String),
}

/// Vérifie la signature HMAC-SHA256 d'une requête inter-service.
///
/// Le message signé est : `{timestamp}:{method}:{path}:{body_hash}`
/// où body_hash = hex(SHA256(body)).
///
/// Anti-replay : le timestamp doit être dans la fenêtre `hmac_max_age_secs`.
pub fn verify_hmac(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    body: &[u8],
    config: &SecurityConfig,
) -> HmacCheck {
    let Some(ref secret) = config.hmac_secret else {
        return HmacCheck::NotConfigured;
    };

    // Extraire les headers COG
    let signature = match headers.get(HEADER_COG_SIGNATURE).and_then(|v| v.to_str().ok()) {
        Some(s) => s,
        None => return HmacCheck::NotConfigured, // Pas de signature = pas un appel inter-service
    };

    let timestamp_str = match headers.get(HEADER_COG_TIMESTAMP).and_then(|v| v.to_str().ok()) {
        Some(t) => t,
        None => return HmacCheck::Invalid("Header X-COG-Timestamp manquant".into()),
    };

    let service_id = headers
        .get(HEADER_COG_SERVICE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Vérifier le timestamp (anti-replay)
    let timestamp: u64 = match timestamp_str.parse() {
        Ok(t) => t,
        Err(_) => return HmacCheck::Invalid("Timestamp invalide".into()),
    };

    let now = epoch_now();
    let age = if now > timestamp {
        now - timestamp
    } else {
        timestamp - now
    };

    if age > config.hmac_max_age_secs {
        return HmacCheck::Invalid(format!(
            "Requête expirée (age: {age}s, max: {}s) — possible attaque par rejeu",
            config.hmac_max_age_secs
        ));
    }

    // Calculer le hash du body
    use sha2::Digest;
    let body_hash = hex::encode(sha2::Sha256::digest(body));

    // Construire le message à vérifier
    let message = format!("{timestamp}:{method}:{path}:{body_hash}");

    // Vérifier la signature HMAC-SHA256
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<sha2::Sha256>;

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return HmacCheck::Invalid("Clé HMAC invalide".into());
    };

    mac.update(message.as_bytes());

    let Ok(expected_signature) = hex::decode(signature) else {
        return HmacCheck::Invalid("Signature non hex-valide".into());
    };

    match mac.verify_slice(&expected_signature) {
        Ok(()) => HmacCheck::Valid { service_id },
        Err(_) => HmacCheck::Invalid(format!(
            "Signature HMAC invalide (service: {service_id}) — possible MITM"
        )),
    }
}

/// Génère une signature HMAC-SHA256 pour un appel sortant vers un service COG.
pub fn sign_request(
    method: &str,
    path: &str,
    body: &[u8],
    secret: &str,
    service_id: &str,
) -> (String, String, String) {
    use sha2::Digest;
    let timestamp = epoch_now().to_string();
    let body_hash = hex::encode(sha2::Sha256::digest(body));
    let message = format!("{timestamp}:{method}:{path}:{body_hash}");

    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<sha2::Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC key");
    mac.update(message.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    (signature, timestamp, service_id.to_string())
}

// ═══════════════════════════════════════════════════════════════════════
// Chiffrement AES-256-GCM des réponses
// ═══════════════════════════════════════════════════════════════════════

/// Chiffre un payload avec AES-256-GCM.
///
/// Le client envoie `X-COG-Encrypt: true` pour demander un chiffrement.
/// La clé doit être exactement 32 bytes (hex-encoded = 64 caractères).
/// Retourne un JSON `{ "encrypted": base64(nonce + ciphertext), "algorithm": "AES-256-GCM" }`.
pub fn encrypt_response(plaintext: &[u8], key_hex: &str) -> Result<serde_json::Value, String> {
    use aes_gcm::{
        aead::{Aead, KeyInit, OsRng},
        Aes256Gcm, AeadCore,
    };

    let key_bytes = hex::decode(key_hex).map_err(|e| format!("Clé hex invalide : {e}"))?;
    if key_bytes.len() != 32 {
        return Err(format!(
            "Clé AES-256 doit faire 32 bytes, reçu : {}",
            key_bytes.len()
        ));
    }

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| format!("Erreur de chiffrement : {e}"))?;

    // Concaténer nonce (12 bytes) + ciphertext
    let mut output = nonce.to_vec();
    output.extend_from_slice(&ciphertext);

    Ok(serde_json::json!({
        "encrypted": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &output),
        "algorithm": "AES-256-GCM",
        "nonce_size": 12,
    }))
}

/// Déchiffre un payload AES-256-GCM (pour les tests et les services COG).
pub fn decrypt_response(encrypted_b64: &str, key_hex: &str) -> Result<Vec<u8>, String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };

    let key_bytes = hex::decode(key_hex).map_err(|e| format!("Clé hex invalide : {e}"))?;
    if key_bytes.len() != 32 {
        return Err(format!(
            "Clé AES-256 doit faire 32 bytes, reçu : {}",
            key_bytes.len()
        ));
    }

    let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted_b64)
        .map_err(|e| format!("Base64 invalide : {e}"))?;

    if data.len() < 12 {
        return Err("Données trop courtes (nonce manquant)".into());
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Déchiffrement échoué : {e}"))
}

// ═══════════════════════════════════════════════════════════════════════
// Journal d'audit sécurité (événements de sécurité)
// ═══════════════════════════════════════════════════════════════════════

/// Type d'événement de sécurité.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventKind {
    OriginBlocked,
    HmacInvalid,
    HmacReplay,
    AuthFailed,
    RateLimited,
    EncryptionError,
}

/// Entrée du journal de sécurité.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: u64,
    pub kind: SecurityEventKind,
    pub source_ip: String,
    pub path: String,
    pub details: String,
}

/// Journal d'audit spécialisé sécurité.
#[derive(Debug, Clone)]
pub struct SecurityAuditLog {
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    max_events: usize,
}

impl SecurityAuditLog {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            max_events,
        }
    }

    pub fn log_event(&self, event: SecurityEvent) {
        tracing::warn!(
            "SECURITY [{:?}] {} — {} — {}",
            event.kind,
            event.source_ip,
            event.path,
            event.details,
        );

        let mut events = self.events.write().unwrap();
        events.push(event);

        if events.len() > self.max_events {
            let drain_count = events.len() - self.max_events;
            events.drain(..drain_count);
        }
    }

    pub fn recent(&self, count: usize) -> Vec<SecurityEvent> {
        let events = self.events.read().unwrap();
        let start = events.len().saturating_sub(count);
        events[start..].to_vec()
    }

    pub fn count_by_kind(&self, kind: &SecurityEventKind) -> usize {
        let events = self.events.read().unwrap();
        let kind_str = serde_json::to_string(kind).unwrap_or_default();
        events
            .iter()
            .filter(|e| serde_json::to_string(&e.kind).unwrap_or_default() == kind_str)
            .count()
    }

    pub fn blocked_ips(&self) -> Vec<String> {
        let events = self.events.read().unwrap();
        let mut ips: Vec<String> = events
            .iter()
            .filter(|e| matches!(e.kind, SecurityEventKind::OriginBlocked))
            .map(|e| e.source_ip.clone())
            .collect();
        ips.sort();
        ips.dedup();
        ips
    }
}
