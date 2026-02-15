//! # MiyuSQL
//!
//! Kit d'outils (Toolkit) de gestion de données en base — toolkit.data.miyusql.
//! Exécution sous autorité KindMother, sans logique métier ni décision (BOUND-1, BOUND-2).
//!
//! ## Tools exposés
//!
//! - tool.query.execute, tool.query.prepare
//! - tool.transaction.begin, commit, rollback
//! - tool.cache.get, set, invalidate
//! - tool.schema.read
//!
//! ## Références
//!
//! - [MiyuSQL - Documentation Fondatrice](../../docs/tools/MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md)
//! - [MiyuSQL - Runtime Boundary Contract](../../docs/tools/MiyuSQL/contracts/boundaries/MiyuSQL%20-%20Runtime%20Boundary%20Contract.md)

// @id: toolkit.data.miyusql
// @role: infrastructure
// @layer: toolkit
// @human: Point d'entrée du toolkit MiyuSQL ; expose les modules tools, pas de capacité nouvelle.
// @do: expose_miyusql_toolkit

pub mod admin_cell;
pub mod cache;
pub mod context;
pub mod errors;
pub mod memory;
pub mod query;
pub mod schema;
pub mod transaction;

pub use admin_cell::{
    miyusql_admin_cell, MiyuSQLAdminCell, MiyuSQLIdentification, MiyuSQLIntegrity,
    MiyuSQLTestManifest, TOOLKIT_ID, MIYUKINI_SQLTEST_TABLE,
};
pub use cache::{get as cache_get, invalidate as cache_invalidate, set as cache_set, CacheExecutor};
pub use context::GovernedContext;
pub use errors::MiyuSQLError;
pub use query::{execute as query_execute, prepare as query_prepare, QueryExecutor, QueryResult};
pub use schema::{
    read as schema_read, read_table as schema_read_table, SchemaExecutor, SchemaResult,
    TableMeta, ColumnMeta,
};
pub use memory::MemoryExecutor;
pub use transaction::{
    begin as tx_begin, commit as tx_commit, rollback as tx_rollback, TransactionExecutor,
};
