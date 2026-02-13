//! Rate limiter pour le Relay.
//!
//! Protège contre les abus et les attaques par déni de service.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Configuration du rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Nombre maximum de connexions par IP par fenêtre.
    pub max_connections_per_ip: u32,
    /// Nombre maximum de messages par session par fenêtre.
    pub max_messages_per_session: u32,
    /// Nombre maximum d'enregistrements échoués par IP avant blocage.
    pub max_failed_registrations: u32,
    /// Durée de la fenêtre glissante (secondes).
    pub window_seconds: u64,
    /// Durée du blocage après dépassement (secondes).
    pub block_duration_seconds: u64,
    /// Activer le PoW pour les passeports temporaires.
    pub require_pow_for_temp: bool,
    /// Difficulté du PoW (nombre de zéros en préfixe).
    pub pow_difficulty: u8,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            max_messages_per_session: 100,
            max_failed_registrations: 5,
            window_seconds: 60,
            block_duration_seconds: 300,
            require_pow_for_temp: true,
            pow_difficulty: 4,
        }
    }
}

/// Entrée de compteur par IP.
#[derive(Debug, Clone)]
struct IpEntry {
    /// Nombre de connexions dans la fenêtre.
    connections: u32,
    /// Nombre d'enregistrements échoués.
    failed_registrations: u32,
    /// Heure de la première requête dans la fenêtre.
    window_start: Instant,
    /// Si bloqué, jusqu'à quand.
    blocked_until: Option<Instant>,
}

impl IpEntry {
    fn new() -> Self {
        Self {
            connections: 0,
            failed_registrations: 0,
            window_start: Instant::now(),
            blocked_until: None,
        }
    }

    fn is_window_expired(&self, window_seconds: u64) -> bool {
        self.window_start.elapsed() > Duration::from_secs(window_seconds)
    }

    fn reset_window(&mut self) {
        self.connections = 0;
        self.failed_registrations = 0;
        self.window_start = Instant::now();
    }

    fn is_blocked(&self) -> bool {
        if let Some(blocked_until) = self.blocked_until {
            Instant::now() < blocked_until
        } else {
            false
        }
    }
}

/// Entrée de compteur par session.
#[derive(Debug, Clone)]
struct SessionEntry {
    /// Nombre de messages dans la fenêtre.
    messages: u32,
    /// Heure de la première requête dans la fenêtre.
    window_start: Instant,
}

impl SessionEntry {
    fn new() -> Self {
        Self {
            messages: 0,
            window_start: Instant::now(),
        }
    }
}

/// Rate limiter.
pub struct RateLimiter {
    /// Configuration.
    config: RateLimiterConfig,
    /// Compteurs par IP.
    ip_counters: Arc<RwLock<HashMap<IpAddr, IpEntry>>>,
    /// Compteurs par session.
    session_counters: Arc<RwLock<HashMap<[u8; 16], SessionEntry>>>,
}

impl RateLimiter {
    /// Crée un nouveau rate limiter.
    #[must_use]
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            ip_counters: Arc::new(RwLock::new(HashMap::new())),
            session_counters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Vérifie si une nouvelle connexion est autorisée pour cette IP.
    pub async fn check_connection(&self, ip: IpAddr) -> RateLimitResult {
        let mut counters = self.ip_counters.write().await;
        let entry = counters.entry(ip).or_insert_with(IpEntry::new);

        // Vérifier le blocage
        if entry.is_blocked() {
            debug!("IP {} is blocked", ip);
            return RateLimitResult::Blocked {
                reason: "Too many failed attempts".to_string(),
                retry_after: entry
                    .blocked_until
                    .map(|b| b.duration_since(Instant::now()).as_secs())
                    .unwrap_or(0),
            };
        }

        // Réinitialiser la fenêtre si expirée
        if entry.is_window_expired(self.config.window_seconds) {
            entry.reset_window();
        }

        // Vérifier la limite
        if entry.connections >= self.config.max_connections_per_ip {
            warn!(
                "IP {} exceeded connection limit ({}/{})",
                ip, entry.connections, self.config.max_connections_per_ip
            );
            return RateLimitResult::Exceeded {
                current: entry.connections,
                limit: self.config.max_connections_per_ip,
            };
        }

        entry.connections += 1;
        RateLimitResult::Allowed
    }

    /// Vérifie si un message est autorisé pour cette session.
    pub async fn check_message(&self, session_id: &[u8; 16]) -> RateLimitResult {
        let mut counters = self.session_counters.write().await;
        let entry = counters.entry(*session_id).or_insert_with(SessionEntry::new);

        // Réinitialiser la fenêtre si expirée
        if entry.window_start.elapsed() > Duration::from_secs(self.config.window_seconds) {
            entry.messages = 0;
            entry.window_start = Instant::now();
        }

        // Vérifier la limite
        if entry.messages >= self.config.max_messages_per_session {
            return RateLimitResult::Exceeded {
                current: entry.messages,
                limit: self.config.max_messages_per_session,
            };
        }

        entry.messages += 1;
        RateLimitResult::Allowed
    }

    /// Signale un enregistrement échoué pour une IP.
    pub async fn record_failed_registration(&self, ip: IpAddr) {
        let mut counters = self.ip_counters.write().await;
        let entry = counters.entry(ip).or_insert_with(IpEntry::new);

        entry.failed_registrations += 1;

        // Bloquer si trop d'échecs
        if entry.failed_registrations >= self.config.max_failed_registrations {
            let block_duration = Duration::from_secs(self.config.block_duration_seconds);
            entry.blocked_until = Some(Instant::now() + block_duration);
            warn!(
                "IP {} blocked for {} seconds due to {} failed registrations",
                ip, self.config.block_duration_seconds, entry.failed_registrations
            );
        }
    }

    /// Réinitialise le compteur d'échecs pour une IP (après succès).
    pub async fn reset_failed_registrations(&self, ip: IpAddr) {
        let mut counters = self.ip_counters.write().await;
        if let Some(entry) = counters.get_mut(&ip) {
            entry.failed_registrations = 0;
            entry.blocked_until = None;
        }
    }

    /// Supprime une session du compteur.
    pub async fn remove_session(&self, session_id: &[u8; 16]) {
        let mut counters = self.session_counters.write().await;
        counters.remove(session_id);
    }

    /// Nettoie les entrées expirées.
    pub async fn cleanup(&self) {
        let now = Instant::now();
        let window = Duration::from_secs(self.config.window_seconds);
        let block_duration = Duration::from_secs(self.config.block_duration_seconds);

        // Nettoyer les IP
        {
            let mut counters = self.ip_counters.write().await;
            counters.retain(|_, entry| {
                let window_active = now.duration_since(entry.window_start) < window;
                let blocked = entry
                    .blocked_until
                    .map(|b| now < b + block_duration)
                    .unwrap_or(false);
                window_active || blocked
            });
        }

        // Nettoyer les sessions
        {
            let mut counters = self.session_counters.write().await;
            counters.retain(|_, entry| now.duration_since(entry.window_start) < window);
        }
    }

    /// Retourne si le PoW est requis pour les passeports temporaires.
    #[must_use]
    pub fn requires_pow_for_temp(&self) -> bool {
        self.config.require_pow_for_temp
    }

    /// Retourne la difficulté du PoW.
    #[must_use]
    pub fn pow_difficulty(&self) -> u8 {
        self.config.pow_difficulty
    }

    /// Vérifie un Proof of Work.
    ///
    /// Le PoW est un hash SHA-256 de `challenge || nonce` qui doit commencer par
    /// `difficulty` bits à zéro.
    #[must_use]
    pub fn verify_pow(&self, challenge: &[u8], nonce: &[u8], pow_hash: &[u8]) -> bool {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(challenge);
        hasher.update(nonce);
        let computed = hasher.finalize();

        // Vérifier que les premiers `difficulty` bits sont à zéro
        let zero_bytes = (self.config.pow_difficulty / 8) as usize;
        let remaining_bits = self.config.pow_difficulty % 8;

        // Vérifier les octets complets
        for i in 0..zero_bytes {
            if computed.get(i).copied().unwrap_or(1) != 0 {
                return false;
            }
        }

        // Vérifier les bits restants
        if remaining_bits > 0 {
            let byte = computed.get(zero_bytes).copied().unwrap_or(255);
            let mask = 0xFF << (8 - remaining_bits);
            if (byte & mask) != 0 {
                return false;
            }
        }

        // Vérifier que le hash fourni correspond
        computed.as_slice() == pow_hash
    }

    /// Génère un challenge pour le PoW.
    #[must_use]
    pub fn generate_challenge() -> [u8; 32] {
        let mut challenge = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut challenge);
        challenge
    }
}

/// Résultat d'une vérification de rate limit.
#[derive(Debug, Clone, PartialEq)]
pub enum RateLimitResult {
    /// Requête autorisée.
    Allowed,
    /// Limite dépassée.
    Exceeded {
        /// Valeur actuelle.
        current: u32,
        /// Limite.
        limit: u32,
    },
    /// IP bloquée.
    Blocked {
        /// Raison du blocage.
        reason: String,
        /// Secondes avant réessai.
        retry_after: u64,
    },
}

impl RateLimitResult {
    /// Vérifie si la requête est autorisée.
    #[must_use]
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_connection_rate_limit() {
        let config = RateLimiterConfig {
            max_connections_per_ip: 3,
            window_seconds: 60,
            ..Default::default()
        };
        let limiter = RateLimiter::new(config);
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

        // Les 3 premières connexions sont autorisées
        assert!(limiter.check_connection(ip).await.is_allowed());
        assert!(limiter.check_connection(ip).await.is_allowed());
        assert!(limiter.check_connection(ip).await.is_allowed());

        // La 4ème est refusée
        assert!(!limiter.check_connection(ip).await.is_allowed());
    }

    #[tokio::test]
    async fn test_failed_registration_blocking() {
        let config = RateLimiterConfig {
            max_failed_registrations: 2,
            block_duration_seconds: 5,
            ..Default::default()
        };
        let limiter = RateLimiter::new(config);
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));

        // Simuler des échecs
        limiter.record_failed_registration(ip).await;
        assert!(limiter.check_connection(ip).await.is_allowed());

        limiter.record_failed_registration(ip).await;

        // Après 2 échecs, bloqué
        let result = limiter.check_connection(ip).await;
        assert!(matches!(result, RateLimitResult::Blocked { .. }));
    }

    #[test]
    fn test_pow_verification() {
        use sha2::{Digest, Sha256};

        let config = RateLimiterConfig {
            pow_difficulty: 8, // 1 octet de zéros
            ..Default::default()
        };
        let limiter = RateLimiter::new(config);

        let challenge = b"test-challenge";

        // Trouver un nonce valide (pour le test, on triche)
        let mut nonce = 0u64;
        let pow_hash = loop {
            let mut hasher = Sha256::new();
            hasher.update(challenge);
            hasher.update(&nonce.to_le_bytes());
            let hash = hasher.finalize();

            if hash[0] == 0 {
                break hash.to_vec();
            }
            nonce += 1;
        };

        assert!(limiter.verify_pow(challenge, &nonce.to_le_bytes(), &pow_hash));
    }
}
