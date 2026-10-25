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

/// Préfixe des opérateurs et bases de mémoire MAIA (une par utilisateur, EX-55).
pub const MAIA_PREFIX: &str = "maia-";

/// Fichier `<data_dir>/maia-utilisateurs` : un identifiant d'utilisateur par ligne
/// (`#` : commentaire). Absent : aucune mémoire MAIA.
pub const MAIA_USERS_FILE: &str = "maia-utilisateurs";

/// Longueur maximale d'un identifiant d'utilisateur MAIA.
pub const MAX_MAIA_USER_LEN: usize = 48;

/// Identifiant d'utilisateur MAIA valide : `[a-z0-9][a-z0-9_-]{0,47}`.
pub fn is_valid_maia_user(user: &str) -> bool {
    let mut chars = user.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit())
        && user.len() <= MAX_MAIA_USER_LEN
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Lit `<data_dir>/maia-utilisateurs`. Une entrée invalide est un refus de démarrer
/// (configuration explicite fautive, fail-closed).
pub fn read_maia_users(data_dir: &std::path::Path) -> Result<Vec<String>, ServiceError> {
    let path = data_dir.join(MAIA_USERS_FILE);
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(ServiceError::Internal(format!("Cannot read {path:?}: {e}"))),
    };
    let mut users: Vec<String> = Vec::new();
    for line in content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !is_valid_maia_user(line) {
            return Err(ServiceError::Internal(format!(
                "Invalid MAIA user entry in {MAIA_USERS_FILE}: {:?}",
                line.chars().take(60).collect::<String>()
            )));
        }
        if !users.iter().any(|u| u == line) {
            users.push(line.to_string());
        }
    }
    Ok(users)
}

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
    /// Restriction par table des écritures typées : base → tables autorisées.
    /// Base absente : toutes les tables de la base (hors tables réservées).
    pub table_scope: HashMap<String, Vec<String>>,
}

impl OperatorConfig {
    /// Opérateur administrateur de sa seule base (cas des services COG).
    pub fn owner(id: &str, database: &str) -> Self {
        Self {
            id: id.to_string(),
            databases: [(database.to_string(), Permission::Admin)]
                .into_iter()
                .collect(),
            rate_limit: 0,
            table_scope: HashMap::new(),
        }
    }
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
        self.register_operator(OperatorConfig::owner("jaykonta", "jaykonta"))
            .await;
        // JayKoa - calendrier universel COG
        self.register_operator(OperatorConfig::owner("jaykoa", "jaykoa"))
            .await;
        // Jay1Tribu - social (tribus, salons, amis)
        self.register_operator(OperatorConfig::owner("jay1tribu", "jay1tribu"))
            .await;
        // JayRDV - rendez-vous
        self.register_operator(OperatorConfig::owner("jayrdv", "jayrdv"))
            .await;
        // JayManga - lecture et catalogue manga
        self.register_operator(OperatorConfig::owner("jaymanga", "jaymanga"))
            .await;
        // MiyukiniWatch - observation (métriques, audit applicatif)
        self.register_operator(OperatorConfig::owner("miyukiniwatch", "miyukiniwatch"))
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
            table_scope: HashMap::new(),
        })
        .await;
    }

    /// Enregistre la mémoire MAIA de chaque utilisateur déclaré (lot 7, EX-55) :
    /// opérateur `maia-<utilisateur>` propriétaire de la seule base `maia-<utilisateur>`
    /// (clé SQLCipher propre, aucun accès à la base d'un autre utilisateur).
    pub async fn register_maia_users(&self, users: &[String]) {
        for user in users {
            let id = format!("{MAIA_PREFIX}{user}");
            self.register_operator(OperatorConfig::owner(&id, &id)).await;
        }
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

    /// Comme [`Self::authorize`], et vérifie en plus que `table` est dans la portée
    /// de l'opérateur pour cette base (écritures typées).
    pub async fn authorize_table(
        &self,
        operator_id: &str,
        database: &str,
        table: &str,
        mode: AccessMode,
    ) -> Result<Permission, ServiceError> {
        let permission = self.authorize(operator_id, database, mode).await?;
        let operators = self.operators.read().await;
        if let Some(scope) = operators
            .get(operator_id)
            .and_then(|op| op.table_scope.get(database))
        {
            if !scope.iter().any(|t| t.eq_ignore_ascii_case(table)) {
                return Err(ServiceError::PermissionDenied(format!(
                    "Operator '{operator_id}' may not write table '{table}' in '{database}'"
                )));
            }
        }
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
                table_scope: [("test_db".to_string(), vec!["notes".to_string()])]
                    .into_iter()
                    .collect(),
            })
            .await;

        // Portée par table
        assert!(engine
            .authorize_table("test_op", "test_db", "notes", AccessMode::Write)
            .await
            .is_ok());
        assert!(engine
            .authorize_table("test_op", "test_db", "comptes", AccessMode::Write)
            .await
            .is_err());

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
                table_scope: HashMap::new(),
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

    #[tokio::test]
    async fn memoire_maia_une_base_par_utilisateur() {
        let dir = std::env::temp_dir().join(format!("km-maia-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("dir");
        assert!(read_maia_users(&dir).expect("absent").is_empty());
        std::fs::write(dir.join(MAIA_USERS_FILE), "# utilisateurs\nalice\n\nbob\nalice\n").expect("w");
        let users = read_maia_users(&dir).expect("lecture");
        assert_eq!(users, vec!["alice".to_string(), "bob".to_string()]);

        let engine = ArbitrationEngine::new();
        engine.register_maia_users(&users).await;
        assert!(engine.authorize("maia-alice", "maia-alice", AccessMode::Write).await.is_ok());
        assert!(engine.authorize("maia-alice", "maia-bob", AccessMode::Read).await.is_err());

        for invalide in ["Alice", "-a", "a b", "a/../b", &"a".repeat(49)] {
            std::fs::write(dir.join(MAIA_USERS_FILE), invalide).expect("w");
            assert!(read_maia_users(&dir).is_err(), "{invalide:?}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
