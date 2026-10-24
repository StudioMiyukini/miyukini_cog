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
//! ## Garde-fous (K-19, C-12, C-14)
//!
//! - Toute exécution exige un mandat **signé par StrongFather** ([`SignatureMandat`]) ;
//!   tant que cette signature n'est pas implémentée, la vérification échoue (fail-closed).
//! - `MemoryExecutor` n'existe que sous `cfg(test)` ou la feature `test-support`
//!   (désactivée par défaut) et refuse toute instruction ou clause non gérée.
//!
//! ## Références
//!
//! - [MiyuSQL - Documentation Fondatrice](../../docs/tools/MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md)
//! - [MiyuSQL - Runtime Boundary Contract](../../docs/tools/MiyuSQL/contracts/boundaries/MiyuSQL%20-%20Runtime%20Boundary%20Contract.md)

// @id toolkit.data.miyusql
// @role infrastructure
// @layer toolkit
// @do expose_miyusql_toolkit
// @human Point d'entrée du toolkit MiyuSQL ; expose les modules tools, pas de capacité nouvelle.

pub mod admin_cell;
pub mod cache;
pub mod context;
pub mod errors;
/// Exécuteur en mémoire : tests uniquement (`cfg(test)` ou feature `test-support`).
#[cfg(any(test, feature = "test-support"))]
pub mod memory;
pub mod query;
pub mod schema;
pub mod transaction;

pub use admin_cell::{
    miyusql_admin_cell, MiyuSQLAdminCell, MiyuSQLIdentification, MiyuSQLIntegrity,
    MiyuSQLTestManifest, MIYUKINI_SQLTEST_TABLE, TOOLKIT_ID,
};
pub use cache::{
    get as cache_get, invalidate as cache_invalidate, set as cache_set, CacheExecutor,
};
pub use context::{GovernedContext, SignatureMandat};
pub use errors::MiyuSQLError;
#[cfg(any(test, feature = "test-support"))]
pub use memory::MemoryExecutor;
pub use query::{execute as query_execute, prepare as query_prepare, QueryExecutor, QueryResult};
pub use schema::{
    read as schema_read, read_table as schema_read_table, ColumnMeta, SchemaExecutor, SchemaResult,
    TableMeta,
};
pub use transaction::{
    begin as tx_begin, commit as tx_commit, rollback as tx_rollback, TransactionExecutor,
};
