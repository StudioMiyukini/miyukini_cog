//! @id     miou_llm_bridge_security
//! @do     definir_rate_limit_config
//! @role   securite
//! @layer  app
//! @human  miou-llm-bridge — module « security ».

//! Sécurité — Auth, rate limiting, audit, sandboxing, anti-MITM, chiffrement.
//!
//! Garantit la sécurité des données, des requêtes et la disponibilité.
//! - Authentification : requêtes signées par des appelants épinglés
//!   (module `signature` ; plus de jeton Bearer ni de secret HMAC partagés, lot 2)
//! - Rate limiting par agent et par endpoint
//! - Audit log de toutes les actions (agents + sécurité)
//! - Vérification des permissions (agent → skill)
//! - Validation d'origine (anti-appels externes non autorisés)
//! - Chiffrement AES-256-GCM des réponses sensibles

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
// C-10 : un verrou empoisonné (panique d'un autre fil pendant l'écriture)
// est levé sans panique ; les sections critiques ne laissent pas d'état
// partiel et l'état réel est conservé (pas de remise à zéro permissive).
use std::sync::{Arc, PoisonError, RwLock};
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
        let mut states = self.states.write().unwrap_or_else(PoisonError::into_inner);
        let now = Instant::now();

        let state = states
            .entry(identifier.to_string())
            .or_insert(RateLimitState {
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
            let retry_after = self
                .config
                .window
                .as_secs()
                .saturating_sub(elapsed.as_secs());
            Err(retry_after)
        } else {
            state.count += 1;
            Ok(self.config.max_requests - state.count)
        }
    }

    /// Nettoie les entrées expirées (à appeler périodiquement).
    pub fn cleanup(&self) {
        let mut states = self.states.write().unwrap_or_else(PoisonError::into_inner);
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
        let mut entries = self.entries.write().unwrap_or_else(PoisonError::into_inner);
        entries.push(entry);

        // Rotation si trop d'entrées
        if entries.len() > self.max_entries {
            let drain_count = entries.len() - self.max_entries;
            entries.drain(..drain_count);
        }
    }

    /// Récupère les N dernières entrées.
    pub fn recent(&self, count: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read().unwrap_or_else(PoisonError::into_inner);
        let start = entries.len().saturating_sub(count);
        entries[start..].to_vec()
    }

    /// Filtre par agent.
    pub fn by_agent(&self, agent_id: &str) -> Vec<AuditEntry> {
        let entries = self.entries.read().unwrap_or_else(PoisonError::into_inner);
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
        "file_ops" | "web_fetch" | "cog_services" => SecurityLevel::Sandboxed,
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

/// Autorise l'exécution d'un skill pour un agent donné (fail-closed).
///
/// - `agent_id` absent ou vide => refus (401-like : l'appelant doit s'identifier)
/// - agent inconnu => refus
/// - permission (skill autorisé + niveau de sécurité) vérifiée par
///   `check_agent_skill_permission`.
pub fn authorize_skill_request(
    registry: &crate::agents::AgentRegistry,
    agent_id: Option<&str>,
    skill_id: &str,
) -> Result<(), String> {
    let Some(agent_id) = agent_id.map(str::trim).filter(|a| !a.is_empty()) else {
        return Err("agent_id requis pour exécuter un skill".into());
    };
    let Some(agent) = registry.get_agent(agent_id) else {
        return Err(format!("Agent '{agent_id}' inconnu — exécution refusée"));
    };
    match check_agent_skill_permission(&agent, skill_id) {
        PermissionCheck::Allowed => Ok(()),
        PermissionCheck::Denied(reason) => Err(reason),
    }
}

/// Détermine l'IP source réelle d'une requête.
///
/// `peer` est l'adresse TCP de la connexion (ConnectInfo). `X-Forwarded-For`
/// n'est pris en compte que si `peer` appartient à `trusted_proxies` ; on
/// parcourt alors la chaîne de droite à gauche en sautant les proxies de
/// confiance, et on retient la première adresse non fiable (la plus à droite
/// est celle ajoutée par notre proxy, donc non falsifiable par le client).
/// `X-Real-IP` est ignoré (trivialement falsifiable).
pub fn resolve_client_ip(
    peer: IpAddr,
    headers: &axum::http::HeaderMap,
    trusted_proxies: &[String],
) -> IpAddr {
    let peer = peer.to_canonical();
    if !ip_in_list(peer, trusted_proxies) {
        return peer;
    }
    let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) else {
        return peer;
    };
    for part in xff.rsplit(',') {
        match part.trim().parse::<IpAddr>() {
            Ok(ip) => {
                let ip = ip.to_canonical();
                if !ip_in_list(ip, trusted_proxies) {
                    return ip;
                }
            }
            // Entrée illisible dans la chaîne : on n'invente rien, on garde le proxy.
            Err(_) => return peer,
        }
    }
    peer
}

fn ip_in_list(ip: IpAddr, list: &[String]) -> bool {
    let ip_s = ip.to_string();
    list.iter().any(|entry| {
        let entry = entry.trim();
        if entry.contains('/') {
            match_cidr(&ip_s, entry)
        } else {
            entry
                .parse::<IpAddr>()
                .is_ok_and(|e| e.to_canonical() == ip)
        }
    })
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
        .or_else(|| headers.get("x-real-ip").and_then(|v| v.to_str().ok()))
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
// En-têtes COG
// ═══════════════════════════════════════════════════════════════════════

/// En-tête par lequel un client demande une réponse chiffrée.
pub const HEADER_COG_ENCRYPT: &str = "x-cog-encrypt";

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
        AeadCore, Aes256Gcm,
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

        let mut events = self.events.write().unwrap_or_else(PoisonError::into_inner);
        events.push(event);

        if events.len() > self.max_events {
            let drain_count = events.len() - self.max_events;
            events.drain(..drain_count);
        }
    }

    pub fn recent(&self, count: usize) -> Vec<SecurityEvent> {
        let events = self.events.read().unwrap_or_else(PoisonError::into_inner);
        let start = events.len().saturating_sub(count);
        events[start..].to_vec()
    }

    pub fn count_by_kind(&self, kind: &SecurityEventKind) -> usize {
        let events = self.events.read().unwrap_or_else(PoisonError::into_inner);
        let kind_str = serde_json::to_string(kind).unwrap_or_default();
        events
            .iter()
            .filter(|e| serde_json::to_string(&e.kind).unwrap_or_default() == kind_str)
            .count()
    }

    pub fn blocked_ips(&self) -> Vec<String> {
        let events = self.events.read().unwrap_or_else(PoisonError::into_inner);
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{header, HeaderMap, HeaderValue};

    #[test]
    fn agent_id_absent_refuse() {
        let reg = crate::agents::AgentRegistry::new();
        assert!(authorize_skill_request(&reg, None, "file_ops").is_err());
        assert!(authorize_skill_request(&reg, Some("  "), "file_ops").is_err());
        assert!(authorize_skill_request(&reg, Some("agent-inexistant"), "file_ops").is_err());
    }

    #[test]
    fn agent_connu_avec_permission_autorise() {
        let reg = crate::agents::AgentRegistry::new();
        // Au moins un agent builtin doit pouvoir utiliser au moins un de ses skills.
        let ok = reg.list_agents().iter().any(|a| {
            a.skill_ids
                .iter()
                .any(|s| authorize_skill_request(&reg, Some(&a.id), s).is_ok())
        });
        assert!(ok);
        // Skill hors liste de l'agent => refus.
        let a = &reg.list_agents()[0];
        assert!(authorize_skill_request(&reg, Some(&a.id), "skill_inconnu").is_err());
    }

    #[test]
    fn ip_source_ignore_xff_sans_proxy_de_confiance() {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", HeaderValue::from_static("127.0.0.1"));
        let peer: IpAddr = "203.0.113.9".parse().unwrap();
        // Liste vide : XFF falsifié ignoré, on garde l'IP réelle.
        assert_eq!(resolve_client_ip(peer, &h, &[]), peer);
        // Peer non listé : idem.
        assert_eq!(resolve_client_ip(peer, &h, &["10.0.0.1".into()]), peer);
    }

    #[test]
    fn ip_source_xff_lu_si_proxy_de_confiance() {
        let mut h = HeaderMap::new();
        // Le client a injecté 127.0.0.1 ; notre proxy a ajouté 198.51.100.7 à droite.
        h.insert(
            "x-forwarded-for",
            HeaderValue::from_static("127.0.0.1, 198.51.100.7"),
        );
        let proxy: IpAddr = "10.0.0.1".parse().unwrap();
        let got = resolve_client_ip(proxy, &h, &["10.0.0.0/24".into()]);
        assert_eq!(got, "198.51.100.7".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn origine_distante_bloquee_par_defaut_loopback_ok() {
        let cfg = SecurityConfig {
            allowed_origins: vec!["192.168.1.0/24".into()],
            ..SecurityConfig::default()
        };
        let h = HeaderMap::new();
        assert!(matches!(
            check_origin(Some("127.0.0.1"), &h, &cfg),
            OriginCheck::Allowed
        ));
        assert!(matches!(
            check_origin(Some("203.0.113.9"), &h, &cfg),
            OriginCheck::Blocked(_)
        ));
    }
}
