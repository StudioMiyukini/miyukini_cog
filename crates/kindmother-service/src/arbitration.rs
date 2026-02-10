//! Moteur d'arbitrage - contrôle d'accès et validation des requêtes.
//!
//! @id: kindmother_service_arbitration
//! @do: enforce_access_control_and_validation
//! @layer: core
//!
//! Le moteur d'arbitrage assure que :
//! - Seuls les opérateurs autorisés peuvent accéder aux données
//! - Chaque opérateur ne peut accéder qu'à ses propres bases
//! - Les requêtes SQL sont validées avant exécution
//! - Les quotas sont respectés

use std::collections::{HashMap, HashSet};
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
    /// Mots-clés SQL interdits en lecture seule
    write_keywords: HashSet<&'static str>,
    /// Mots-clés DDL (réservés aux admins)
    ddl_keywords: HashSet<&'static str>,
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
        let mut write_keywords = HashSet::new();
        write_keywords.insert("INSERT");
        write_keywords.insert("UPDATE");
        write_keywords.insert("DELETE");
        write_keywords.insert("REPLACE");
        
        let mut ddl_keywords = HashSet::new();
        ddl_keywords.insert("CREATE");
        ddl_keywords.insert("DROP");
        ddl_keywords.insert("ALTER");
        ddl_keywords.insert("TRUNCATE");
        ddl_keywords.insert("VACUUM");
        ddl_keywords.insert("REINDEX");
        
        Self {
            operators: Arc::new(RwLock::new(HashMap::new())),
            request_counts: Arc::new(RwLock::new(HashMap::new())),
            write_keywords,
            ddl_keywords,
        }
    }

    /// Enregistre un opérateur avec ses permissions.
    pub async fn register_operator(&self, config: OperatorConfig) {
        let mut operators = self.operators.write().await;
        operators.insert(config.id.clone(), config);
    }

    /// Enregistre les opérateurs par défaut (COG services).
    pub async fn register_default_operators(&self) {
        // === Services famille Jay ===
        
        // JayXpose - profil exposant, catalogue, vitrine
        self.register_operator(OperatorConfig {
            id: "jayxpose".to_string(),
            databases: [("jayxpose".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0, // Illimité pour les services COG
        })
        .await;

        // JayKonta - comptabilité COG unifiée (Purse + Account)
        self.register_operator(OperatorConfig {
            id: "jaykonta".to_string(),
            databases: [("jaykonta".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        })
        .await;

        // JayFestival - festivals, éditions, exposants, visiteurs
        self.register_operator(OperatorConfig {
            id: "jayfestival".to_string(),
            databases: [("jayfestival".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
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
                ("jayxpose".to_string(), Permission::Read),
                ("jaykonta".to_string(), Permission::Read),
                ("jayfestival".to_string(), Permission::Read),
                ("jaykoa".to_string(), Permission::Read),
            ]
            .into_iter()
            .collect(),
            rate_limit: 0,
        })
        .await;

        // === Services de jeux ===
        
        // Lord of the Castle - jeu survivor
        self.register_operator(OperatorConfig {
            id: "lord-of-the-castle".to_string(),
            databases: [("lord_of_the_castle".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        })
        .await;

        // MiyuClicker - jeu idle clicker
        self.register_operator(OperatorConfig {
            id: "miyuclicker".to_string(),
            databases: [("miyuclicker".to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        })
        .await;
    }

    /// Vérifie qu'un opérateur peut effectuer une requête.
    pub async fn authorize(
        &self,
        operator_id: &str,
        database: &str,
        sql: &str,
        is_write: bool,
    ) -> Result<(), ServiceError> {
        // 1. Vérifier que l'opérateur existe
        let operators = self.operators.read().await;
        let operator = operators.get(operator_id).ok_or_else(|| {
            ServiceError::UnknownOperator(operator_id.to_string())
        })?;

        // 2. Vérifier l'accès à la base
        let permission = operator.databases.get(database).ok_or_else(|| {
            ServiceError::PermissionDenied(format!(
                "Operator '{}' has no access to database '{}'",
                operator_id, database
            ))
        })?;

        // 3. Vérifier les permissions selon le type d'opération
        self.check_sql_permission(sql, permission, is_write)?;

        // 4. Vérifier le rate limit
        let rate_limit = operator.rate_limit;
        drop(operators); // Libérer le lock avant d'acquérir un autre
        self.check_rate_limit(operator_id, rate_limit).await?;

        Ok(())
    }

    /// Vérifie les permissions SQL.
    fn check_sql_permission(
        &self,
        sql: &str,
        permission: &Permission,
        is_write: bool,
    ) -> Result<(), ServiceError> {
        let sql_upper = sql.to_uppercase();
        let first_keyword = sql_upper
            .split_whitespace()
            .next()
            .unwrap_or("");

        // Vérifier les mots-clés DDL
        if self.ddl_keywords.contains(first_keyword) {
            if *permission != Permission::Admin {
                return Err(ServiceError::PermissionDenied(
                    "DDL operations require Admin permission".to_string(),
                ));
            }
        }

        // Vérifier les mots-clés d'écriture
        if is_write || self.write_keywords.contains(first_keyword) {
            match permission {
                Permission::Read => {
                    return Err(ServiceError::PermissionDenied(
                        "Write operations not allowed with Read permission".to_string(),
                    ));
                }
                Permission::ReadWrite | Permission::Admin => {}
            }
        }

        // Validation SQL basique (empêcher les injections évidentes)
        self.validate_sql(sql)?;

        Ok(())
    }

    /// Validation SQL basique.
    fn validate_sql(&self, sql: &str) -> Result<(), ServiceError> {
        let sql_upper = sql.to_uppercase();

        // Interdire les commandes système dangereuses
        if sql_upper.contains("ATTACH DATABASE")
            || sql_upper.contains("DETACH DATABASE")
            || sql_upper.contains("PRAGMA KEY")
            || sql_upper.contains("PRAGMA REKEY")
        {
            return Err(ServiceError::Validation(
                "Forbidden SQL command".to_string(),
            ));
        }

        // Interdire les commentaires qui pourraient masquer des injections
        if sql.contains("--") || sql.contains("/*") {
            return Err(ServiceError::Validation(
                "SQL comments are not allowed".to_string(),
            ));
        }

        Ok(())
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
        
        engine.register_operator(OperatorConfig {
            id: "test_op".to_string(),
            databases: [("test_db".to_string(), Permission::ReadWrite)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        }).await;

        // Devrait réussir
        assert!(engine.authorize("test_op", "test_db", "SELECT * FROM users", false).await.is_ok());
        assert!(engine.authorize("test_op", "test_db", "INSERT INTO users VALUES (?)", true).await.is_ok());

        // Devrait échouer - base non autorisée
        assert!(engine.authorize("test_op", "other_db", "SELECT 1", false).await.is_err());

        // Devrait échouer - opérateur inconnu
        assert!(engine.authorize("unknown", "test_db", "SELECT 1", false).await.is_err());
    }

    #[tokio::test]
    async fn test_read_only_permission() {
        let engine = ArbitrationEngine::new();
        
        engine.register_operator(OperatorConfig {
            id: "reader".to_string(),
            databases: [("db".to_string(), Permission::Read)]
                .into_iter()
                .collect(),
            rate_limit: 0,
        }).await;

        // Lecture OK
        assert!(engine.authorize("reader", "db", "SELECT * FROM t", false).await.is_ok());

        // Écriture interdite
        assert!(engine.authorize("reader", "db", "INSERT INTO t VALUES (1)", true).await.is_err());
    }
}
