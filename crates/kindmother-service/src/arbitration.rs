//! Moteur d'arbitrage - contrôle d'accès et validation des requêtes.
//!
//! @id     kindmother_service_arbitration
//! @do     enforce_access_control_and_validation
//@human ! @layer  core
//!
//! Le moteur d'arbitrage assure que :
//! - Seuls les opérateurs enregistrés peuvent accéder aux données
//! - Chaque opérateur ne peut accéder qu'aux bases qui lui sont attribuées
//! - Le mode d'accès (lecture / écriture) est compatible avec sa permission
//! - Les quotas sont respectés
//!
//! L'identité de l'opérateur est celle **authentifiée** sur la connexion (K-04),
//! jamais un champ de la requête. Le contenu SQL n'est plus analysé ici : la
//! politique SQL est appliquée par l'autorisateur SQLite (`sql_guard`, K-05).

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::errors::ServiceError;

/// Permission d'accès à une base de données.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    /// Lecture seule
    Read,
    /// Lecture et écriture
    ReadWrite,
    /// Accès complet (y compris DDL)
    Admin,
}

/// Mode d'accès demandé par une requête.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    /// Lecture (Query, DatabaseInfo, HealthCheck)
    Read,
    /// Écriture (Execute, Transaction)
    Write,
}

/// Configuration d'un opérateur.
#[derive(Debug, Clone)]
pub struct OperatorConfig {
    /// Identifiant unique
    pub id: String,
    /// Bases de données autorisées avec leurs permissions
    pub databases: HashMap<String, Permission>,
    /// Quota de requêtes par minute (0 = illimité)
    pub rate_limit: u32,
}

/// Moteur d'arbitrage des accès.
pub struct ArbitrationEngine {
    /// Configuration des opérateurs
    operators: Arc<RwLock<HashMap<String, OperatorConfig>>>,
    /// Compteurs de requêtes pour le rate limiting
    request_counts: Arc<RwLock<HashMap<String, RequestCounter>>>,
}

/// Compteur de requêtes pour le rate limiting.
#[derive(Debug, Clone)]
struct RequestCounter {
    count: u32,
    window_start: std::time::Instant,
}

impl Default for RequestCounter {
    fn default() -> Self {
        Self {
            count: 0,
            window_start: std::time::Instant::now(),
        }
    }
}

impl ArbitrationEngine {
    /// Crée un nouveau moteur d'arbitrage.
    pub fn new() -> Self {
        Self {
            operators: Arc::new(RwLock::new(HashMap::new())),
            request_counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Identifiants des opérateurs enregistrés.
    pub async fn operator_ids(&self) -> Vec<String> {
        let operators = self.operators.read().await;
        let mut ids: Vec<String> = operators.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Enregistre un opérateur avec ses permissions.
    pub async fn register_operator(&self, config: OperatorConfig) {
        let mut operators = self.operators.write().await;
        operators.insert(config.id.clone(), config);
    }

    /// Enregistre les opérateurs par défaut (COG services).
    pub async fn register_default_operators(&self) {
        // === Services famille Jay ===

        // JayKonta - comptabilité COG unifiée (Purse + Account)
        self.register_operator(OperatorConfig {
            id: "jaykonta".to_string(),
            databases: [("jaykonta".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0, // Illimité pour les services COG
        })
        .await;

        // JayKoa - calendrier universel COG
        self.register_operator(OperatorConfig {
            id: "jaykoa".to_string(),
            databases: [("jaykoa".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        })
        .await;

        // === Hub Central ===

        // Miyukini Central - hub de gestion, profils centraux, auth
        // Accès admin à sa propre base + lecture aux bases des services
        self.register_operator(OperatorConfig {
            id: "miyukini-central".to_string(),
            databases: [
                ("central".to_string(), Permission::Admin),
                ("jaykonta".to_string(), Permission::Read),
                ("jaykoa".to_string(), Permission::Read),
            ]
            .into_iter()
            .collect(),
            rate_limit: 0,
        })
        .await;
    }

    /// Vérifie qu'un opérateur (authentifié) peut accéder à une base dans le mode donné.
    ///
    /// Retourne la permission de l'opérateur sur cette base, d'où découle la
    /// politique SQL (`SqlPolicy::for_permission`).
    pub async fn authorize(
        &self,
        operator_id: &str,
        database: &str,
        mode: AccessMode,
    ) -> Result<Permission, ServiceError> {
        // 1. Vérifier que l'opérateur existe
        let operators = self.operators.read().await;
        let operator = operators
            .get(operator_id)
            .ok_or_else(|| ServiceError::UnknownOperator(operator_id.to_string()))?;

        // 2. Vérifier l'accès à la base
        let permission = operator.databases.get(database).cloned().ok_or_else(|| {
            ServiceError::PermissionDenied(format!(
                "Operator '{}' has no access to database '{}'",
                operator_id, database
            ))
        })?;

        // 3. Vérifier le mode d'accès
        if mode == AccessMode::Write && permission == Permission::Read {
            return Err(ServiceError::PermissionDenied(
                "Write operations not allowed with Read permission".to_string(),
            ));
        }

        // 4. Vérifier le rate limit
        let rate_limit = operator.rate_limit;
        drop(operators); // Libérer le lock avant d'acquérir un autre
        self.check_rate_limit(operator_id, rate_limit).await?;

        Ok(permission)
    }

    /// Vérifie le rate limit.
    async fn check_rate_limit(&self, operator_id: &str, limit: u32) -> Result<(), ServiceError> {
        if limit == 0 {
            return Ok(()); // Pas de limite
        }

        let mut counts = self.request_counts.write().await;
        let counter = counts.entry(operator_id.to_string()).or_default();

        let now = std::time::Instant::now();
        let window = std::time::Duration::from_secs(60);

        // Réinitialiser si la fenêtre est expirée
        if now.duration_since(counter.window_start) > window {
            counter.count = 0;
            counter.window_start = now;
        }

        counter.count += 1;

        if counter.count > limit {
            return Err(ServiceError::QuotaExceeded(format!(
                "Rate limit exceeded ({}/min)",
                limit
            )));
        }

        Ok(())
    }
}

impl Default for ArbitrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_authorization() {
        let engine = ArbitrationEngine::new();

        engine
            .register_operator(OperatorConfig {
                id: "test_op".to_string(),
                databases: [("test_db".to_string(), Permission::ReadWrite)]
                    .into_iter()
                    .collect(),
                rate_limit: 0,
            })
            .await;

        // Devrait réussir
        assert!(engine
            .authorize("test_op", "test_db", AccessMode::Read)
            .await
            .is_ok());
        assert_eq!(
            engine
                .authorize("test_op", "test_db", AccessMode::Write)
                .await
                .ok(),
            Some(Permission::ReadWrite)
        );

        // Devrait échouer - base non autorisée
        assert!(engine
            .authorize("test_op", "other_db", AccessMode::Read)
            .await
            .is_err());

        // Devrait échouer - opérateur inconnu
        assert!(engine
            .authorize("unknown", "test_db", AccessMode::Read)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_read_only_permission() {
        let engine = ArbitrationEngine::new();

        engine
            .register_operator(OperatorConfig {
                id: "reader".to_string(),
                databases: [("db".to_string(), Permission::Read)].into_iter().collect(),
                rate_limit: 0,
            })
            .await;

        // Lecture OK
        assert!(engine
            .authorize("reader", "db", AccessMode::Read)
            .await
            .is_ok());

        // Écriture interdite
        assert!(engine
            .authorize("reader", "db", AccessMode::Write)
            .await
            .is_err());
    }
}
