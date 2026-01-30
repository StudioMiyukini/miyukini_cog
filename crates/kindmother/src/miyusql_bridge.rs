//! Pont KindMother ↔ MiyuSQL (MiyuSQL - KindMother Integration Contract).
//!
//! Trait permettant à KindMother de déléguer l'exécution SQL à MiyuSQL (mandat + appel
//! execute/prepare/tx/cache/schema). Implémentation minimale (stub ou réelle selon stockage).

use miyusql::{
    GovernedContext, MiyuSQLError, QueryExecutor, QueryResult, SchemaExecutor, TableMeta,
    TransactionExecutor, query_execute, query_prepare, schema_read, schema_read_table, tx_begin,
    tx_commit, tx_rollback,
};

/// @id: kindmother_miyusql_bridge
/// @role: infrastructure
/// @layer: core
/// @human: Trait de délégation d'exécution SQL à MiyuSQL sous mandat KindMother.
/// @do: delegate_execution_to_miyusql
/// KindMother invoque ce bridge après validation WriteIntent pour exécution effective.

/// @id: kindmother_miyusql_bridge_trait
/// @role: infrastructure
/// @layer: core
/// @human: Interface de délégation SQL vers MiyuSQL (mandat, execute, prepare, tx, cache, schema).
/// @do: define_miyusql_bridge_trait
pub trait MiyuSQLExecutionBridge: Send + Sync {
    /// Exécute une requête sous mandat gouverné (après validation WriteIntent si écriture).
    fn execute_query(
        &self,
        mandate_id: &str,
        security_level: u8,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError>;

    /// Prépare ou valide une requête sans l'exécuter.
    fn prepare_query(
        &self,
        mandate_id: &str,
        security_level: u8,
        sql: &str,
        params: &[String],
    ) -> Result<(), MiyuSQLError>;

    /// Démarre une transaction sous mandat.
    fn begin_transaction(&self, mandate_id: &str, security_level: u8) -> Result<String, MiyuSQLError>;

    /// Valide la transaction.
    fn commit_transaction(
        &self,
        mandate_id: &str,
        security_level: u8,
        tx_id: &str,
    ) -> Result<(), MiyuSQLError>;

    /// Annule la transaction.
    fn rollback_transaction(
        &self,
        mandate_id: &str,
        security_level: u8,
        tx_id: &str,
    ) -> Result<(), MiyuSQLError>;

    /// Lit le schéma (tables).
    fn read_schema(
        &self,
        mandate_id: &str,
        security_level: u8,
    ) -> Result<miyusql::SchemaResult, MiyuSQLError>;

    /// Lit les métadonnées d'une table.
    fn read_table_schema(
        &self,
        mandate_id: &str,
        security_level: u8,
        table_name: &str,
    ) -> Result<TableMeta, MiyuSQLError>;
}

/// @id: kindmother_miyusql_bridge_impl
/// @role: infrastructure
/// @layer: core
/// @human: Implémentation du bridge qui délègue à un exécuteur unique (Query + Transaction + Schema).
/// @do: implement_miyusql_bridge
/// Utilise les types MiyuSQL (GovernedContext, execute/prepare/tx/schema).
pub struct MiyuSQLBridgeImpl<E>
where
    E: QueryExecutor + TransactionExecutor + SchemaExecutor + Send + Sync,
{
    executor: std::sync::Arc<E>,
}

impl<E> MiyuSQLBridgeImpl<E>
where
    E: QueryExecutor + TransactionExecutor + SchemaExecutor + Send + Sync,
{
    /// Crée un bridge avec l'exécuteur fourni (utilisé pour query, transaction, schema).
    pub fn new(executor: std::sync::Arc<E>) -> Self {
        Self { executor }
    }

    fn ctx(&self, mandate_id: &str, security_level: u8) -> GovernedContext {
        GovernedContext::new(mandate_id.to_string(), security_level)
    }
}

impl<E> MiyuSQLExecutionBridge for MiyuSQLBridgeImpl<E>
where
    E: QueryExecutor + TransactionExecutor + SchemaExecutor + Send + Sync,
{
    fn execute_query(
        &self,
        mandate_id: &str,
        security_level: u8,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        query_execute(&ctx, sql, params, self.executor.as_ref())
    }

    fn prepare_query(
        &self,
        mandate_id: &str,
        security_level: u8,
        sql: &str,
        params: &[String],
    ) -> Result<(), MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        query_prepare(&ctx, sql, params, self.executor.as_ref())
    }

    fn begin_transaction(&self, mandate_id: &str, security_level: u8) -> Result<String, MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        tx_begin(&ctx, self.executor.as_ref())
    }

    fn commit_transaction(
        &self,
        mandate_id: &str,
        security_level: u8,
        tx_id: &str,
    ) -> Result<(), MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        tx_commit(&ctx, tx_id, self.executor.as_ref())
    }

    fn rollback_transaction(
        &self,
        mandate_id: &str,
        security_level: u8,
        tx_id: &str,
    ) -> Result<(), MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        tx_rollback(&ctx, tx_id, self.executor.as_ref())
    }

    fn read_schema(
        &self,
        mandate_id: &str,
        security_level: u8,
    ) -> Result<miyusql::SchemaResult, MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        schema_read(&ctx, self.executor.as_ref())
    }

    fn read_table_schema(
        &self,
        mandate_id: &str,
        security_level: u8,
        table_name: &str,
    ) -> Result<TableMeta, MiyuSQLError> {
        let ctx = self.ctx(mandate_id, security_level);
        schema_read_table(&ctx, table_name, self.executor.as_ref())
    }
}
