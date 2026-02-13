//! Vérification de conformité MWS.
//!
//! Implémente les trois phases de vérification :
//! - Phase A : Clé de conformité des Cores
//! - Phase B : Blocs de code MIP des Services
//! - Phase C : Santé de l'environnement

use bytes::Bytes;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::protocol::VerifyResult;

/// Erreur de vérification.
#[derive(Debug, Clone)]
pub struct VerificationError {
    /// Phase où l'erreur s'est produite.
    pub phase: String,
    /// Message d'erreur.
    pub message: String,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Verification failed at {}: {}", self.phase, self.message)
    }
}

impl std::error::Error for VerificationError {}

/// Vérificateur de conformité.
pub struct Verifier {
    /// Clés de conformité connues (version → hash).
    core_keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Hashes MIP des services connus (service_id → block_index → hash).
    service_hashes: Arc<RwLock<HashMap<String, HashMap<u32, Vec<u8>>>>>,
    /// Chemin du répertoire des clés.
    keys_dir: String,
}

impl Verifier {
    /// Crée un nouveau vérificateur.
    #[must_use]
    pub fn new(keys_dir: String) -> Self {
        Self {
            core_keys: Arc::new(RwLock::new(HashMap::new())),
            service_hashes: Arc::new(RwLock::new(HashMap::new())),
            keys_dir,
        }
    }

    /// Charge les clés de conformité depuis le disque.
    pub async fn load_keys(&self) -> Result<(), std::io::Error> {
        let keys_path = Path::new(&self.keys_dir);
        if !keys_path.exists() {
            warn!("Keys directory does not exist: {}", self.keys_dir);
            return Ok(());
        }

        let mut core_keys = self.core_keys.write().await;

        // Charger les fichiers .key
        let mut entries = tokio::fs::read_dir(keys_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("key") {
                if let Some(version) = path.file_stem().and_then(|s| s.to_str()) {
                    let content = tokio::fs::read(&path).await?;
                    let hash = Sha256::digest(&content).to_vec();
                    core_keys.insert(version.to_string(), hash);
                    info!("Loaded core key for version: {}", version);
                }
            }
        }

        Ok(())
    }

    /// Ajoute une clé de conformité.
    pub async fn add_core_key(&self, version: String, key_hash: Vec<u8>) {
        let mut keys = self.core_keys.write().await;
        keys.insert(version, key_hash);
    }

    /// Ajoute un hash de service.
    pub async fn add_service_hash(&self, service_id: String, block_index: u32, hash: Vec<u8>) {
        let mut hashes = self.service_hashes.write().await;
        let service = hashes.entry(service_id).or_default();
        service.insert(block_index, hash);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Phase A : Vérification de la clé Cores
    // ─────────────────────────────────────────────────────────────────────────

    /// Vérifie la clé de conformité des Cores (Phase A).
    pub async fn verify_phase_a(&self, core_version: &str, provided_key: &[u8]) -> VerifyResult {
        debug!("Phase A: Verifying core key for version {}", core_version);

        let keys = self.core_keys.read().await;

        // Chercher la clé pour cette version
        if let Some(expected_hash) = keys.get(core_version) {
            // Calculer le hash de la clé fournie
            let provided_hash = Sha256::digest(provided_key);

            if provided_hash.as_slice() == expected_hash {
                info!("Phase A: Core key verified for version {}", core_version);
                VerifyResult::Ok
            } else {
                warn!("Phase A: Core key mismatch for version {}", core_version);
                VerifyResult::Fail
            }
        } else {
            // Version inconnue - demander une vérification étendue
            warn!("Phase A: Unknown core version {}", core_version);
            VerifyResult::ExtendedRequired
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Phase B : Vérification des blocs de service
    // ─────────────────────────────────────────────────────────────────────────

    /// Vérifie un bloc de code MIP d'un service (Phase B).
    pub async fn verify_phase_b(
        &self,
        service_id: &str,
        block_index: u32,
        encrypted_block: &[u8],
    ) -> VerifyResult {
        debug!(
            "Phase B: Verifying service {} block {}",
            service_id, block_index
        );

        let hashes = self.service_hashes.read().await;

        if let Some(service_hashes) = hashes.get(service_id) {
            if let Some(expected_hash) = service_hashes.get(&block_index) {
                // Calculer le hash du bloc fourni
                let provided_hash = Sha256::digest(encrypted_block);

                if provided_hash.as_slice() == expected_hash {
                    info!(
                        "Phase B: Service {} block {} verified",
                        service_id, block_index
                    );
                    VerifyResult::Ok
                } else {
                    warn!(
                        "Phase B: Service {} block {} mismatch",
                        service_id, block_index
                    );
                    VerifyResult::Fail
                }
            } else {
                warn!(
                    "Phase B: Unknown block {} for service {}",
                    block_index, service_id
                );
                VerifyResult::ExtendedRequired
            }
        } else {
            // Service inconnu
            warn!("Phase B: Unknown service {}", service_id);
            VerifyResult::Fail
        }
    }

    /// Retourne les indices de blocs à vérifier pour un service.
    pub async fn get_required_blocks(&self, service_id: &str) -> Vec<u32> {
        let hashes = self.service_hashes.read().await;

        if let Some(service_hashes) = hashes.get(service_id) {
            service_hashes.keys().copied().collect()
        } else {
            // Service inconnu : demander le bloc 0 par défaut
            vec![0]
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Phase C : Vérification de la santé de l'environnement
    // ─────────────────────────────────────────────────────────────────────────

    /// Vérifie la santé de l'environnement (Phase C).
    pub async fn verify_phase_c(&self, environment_health: &[u8]) -> VerifyResult {
        debug!("Phase C: Verifying environment health");

        // Parser le rapport de santé (JSON)
        let health: serde_json::Result<EnvironmentHealth> =
            serde_json::from_slice(environment_health);

        match health {
            Ok(health) => {
                // Vérifier les critères de santé
                if !health.cores_healthy {
                    warn!("Phase C: Cores not healthy");
                    return VerifyResult::Fail;
                }

                if health.security_level < 1 {
                    warn!("Phase C: Security level too low: {}", health.security_level);
                    return VerifyResult::Fail;
                }

                if health.trust_state > 2 {
                    warn!("Phase C: Trust state degraded: {}", health.trust_state);
                    return VerifyResult::Fail;
                }

                info!("Phase C: Environment health verified");
                VerifyResult::Ok
            }
            Err(e) => {
                warn!("Phase C: Failed to parse health report: {}", e);
                VerifyResult::Fail
            }
        }
    }
}

/// Rapport de santé de l'environnement.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnvironmentHealth {
    /// Les Cores sont-ils en bon état ?
    #[serde(default = "default_true")]
    pub cores_healthy: bool,
    /// Niveau de sécurité (0-4).
    #[serde(default = "default_security_level")]
    pub security_level: u8,
    /// État de confiance (T0-T4).
    #[serde(default)]
    pub trust_state: u8,
    /// Timestamp du rapport.
    #[serde(default)]
    pub timestamp: u64,
    /// Version des Cores.
    #[serde(default)]
    pub core_version: String,
    /// Services actifs.
    #[serde(default)]
    pub active_services: Vec<String>,
}

fn default_true() -> bool {
    true
}

fn default_security_level() -> u8 {
    1
}

/// Builder pour construire un rapport de santé.
#[derive(Debug, Default)]
pub struct EnvironmentHealthBuilder {
    cores_healthy: bool,
    security_level: u8,
    trust_state: u8,
    core_version: String,
    active_services: Vec<String>,
}

impl EnvironmentHealthBuilder {
    /// Crée un nouveau builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cores_healthy: true,
            security_level: 1,
            trust_state: 0,
            core_version: String::new(),
            active_services: Vec::new(),
        }
    }

    /// Définit l'état des Cores.
    #[must_use]
    pub const fn cores_healthy(mut self, healthy: bool) -> Self {
        self.cores_healthy = healthy;
        self
    }

    /// Définit le niveau de sécurité.
    #[must_use]
    pub const fn security_level(mut self, level: u8) -> Self {
        self.security_level = level;
        self
    }

    /// Définit l'état de confiance.
    #[must_use]
    pub const fn trust_state(mut self, state: u8) -> Self {
        self.trust_state = state;
        self
    }

    /// Définit la version des Cores.
    #[must_use]
    pub fn core_version(mut self, version: impl Into<String>) -> Self {
        self.core_version = version.into();
        self
    }

    /// Ajoute un service actif.
    #[must_use]
    pub fn add_service(mut self, service_id: impl Into<String>) -> Self {
        self.active_services.push(service_id.into());
        self
    }

    /// Construit le rapport en JSON.
    #[must_use]
    pub fn build_json(&self) -> Bytes {
        let health = serde_json::json!({
            "cores_healthy": self.cores_healthy,
            "security_level": self.security_level,
            "trust_state": self.trust_state,
            "timestamp": chrono::Utc::now().timestamp(),
            "core_version": self.core_version,
            "active_services": self.active_services,
        });

        Bytes::from(serde_json::to_vec(&health).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_phase_a_verification() {
        let verifier = Verifier::new("test_keys".to_string());

        // Ajouter une clé de test
        let test_key = b"test-core-key-1.0.0";
        let key_hash = Sha256::digest(test_key).to_vec();
        verifier.add_core_key("1.0.0".to_string(), key_hash).await;

        // Test avec la bonne clé
        let result = verifier.verify_phase_a("1.0.0", test_key).await;
        assert_eq!(result, VerifyResult::Ok);

        // Test avec une mauvaise clé
        let result = verifier.verify_phase_a("1.0.0", b"wrong-key").await;
        assert_eq!(result, VerifyResult::Fail);

        // Test avec une version inconnue
        let result = verifier.verify_phase_a("2.0.0", test_key).await;
        assert_eq!(result, VerifyResult::ExtendedRequired);
    }

    #[tokio::test]
    async fn test_phase_b_verification() {
        let verifier = Verifier::new("test_keys".to_string());

        // Ajouter un hash de service
        let test_block = b"test-service-block-data";
        let block_hash = Sha256::digest(test_block).to_vec();
        verifier
            .add_service_hash("jayfestival".to_string(), 0, block_hash)
            .await;

        // Test avec le bon bloc
        let result = verifier
            .verify_phase_b("jayfestival", 0, test_block)
            .await;
        assert_eq!(result, VerifyResult::Ok);

        // Test avec un mauvais bloc
        let result = verifier
            .verify_phase_b("jayfestival", 0, b"wrong-block")
            .await;
        assert_eq!(result, VerifyResult::Fail);

        // Test avec un service inconnu
        let result = verifier
            .verify_phase_b("unknown-service", 0, test_block)
            .await;
        assert_eq!(result, VerifyResult::Fail);
    }

    #[tokio::test]
    async fn test_phase_c_verification() {
        let verifier = Verifier::new("test_keys".to_string());

        // Rapport de santé valide
        let health = EnvironmentHealthBuilder::new()
            .cores_healthy(true)
            .security_level(1)
            .trust_state(0)
            .core_version("1.0.0")
            .add_service("jayfestival")
            .build_json();

        let result = verifier.verify_phase_c(&health).await;
        assert_eq!(result, VerifyResult::Ok);

        // Rapport avec Cores non sains
        let bad_health = serde_json::json!({
            "cores_healthy": false,
            "security_level": 1,
            "trust_state": 0,
        });
        let result = verifier
            .verify_phase_c(&serde_json::to_vec(&bad_health).unwrap())
            .await;
        assert_eq!(result, VerifyResult::Fail);

        // Rapport avec trust_state dégradé
        let bad_health = serde_json::json!({
            "cores_healthy": true,
            "security_level": 1,
            "trust_state": 3,
        });
        let result = verifier
            .verify_phase_c(&serde_json::to_vec(&bad_health).unwrap())
            .await;
        assert_eq!(result, VerifyResult::Fail);
    }

    #[test]
    fn test_environment_health_builder() {
        let health = EnvironmentHealthBuilder::new()
            .cores_healthy(true)
            .security_level(2)
            .trust_state(0)
            .core_version("1.0.0")
            .add_service("jayfestival")
            .add_service("jayxpose")
            .build_json();

        assert!(!health.is_empty());

        // Vérifier que c'est du JSON valide
        let parsed: serde_json::Value = serde_json::from_slice(&health).unwrap();
        assert_eq!(parsed["cores_healthy"], true);
        assert_eq!(parsed["security_level"], 2);
    }
}
