//! Tool schema : tool.schema.read.
//!
//! Lecture des métadonnées du schéma (tables, colonnes) sans modification.

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id miyusql_schema_table_meta
/// @role data
/// @layer tool
/// @do represent_table_metadata
//@human / @human Métadonnées d'une table.
#[derive(Debug, Clone)]
pub struct TableMeta {
    /// @id miyusql_schema_table_name
    /// @role data
    /// @layer tool
    /// @do store_table_name
    //@human / @human Nom de la table.
    pub name: String,
    /// @id miyusql_schema_columns
    /// @role data
    /// @layer tool
    /// @do store_columns
    //@human / @human Colonnes (nom, type).
    pub columns: Vec<ColumnMeta>,
}

/// @id miyusql_schema_column_meta
/// @role data
/// @layer tool
/// @do represent_column_metadata
//@human / @human Métadonnées d'une colonne.
#[derive(Debug, Clone)]
pub struct ColumnMeta {
    /// @id miyusql_schema_column_name
    /// @role data
    /// @layer tool
    /// @do documenter_name
    pub name: String,
    /// @id miyusql_schema_column_type
    /// @role data
    /// @layer tool
    /// @do documenter_type
    pub data_type: String,
}

/// @id miyusql_schema_result
/// @role data
/// @layer tool
/// @do represent_schema_result
//@human / @human Résultat de lecture schéma (tables ou colonnes d'une table).
#[derive(Debug, Clone)]
pub struct SchemaResult {
    /// @id miyusql_schema_tables
    /// @role data
    /// @layer tool
    /// @do documenter_tables
    pub tables: Vec<TableMeta>,
}

/// @id miyusql_tool_schema_read
/// @role accessor
/// @layer tool
/// @do read_schema_metadata
//@human / @human Lit les métadonnées du schéma (tables, colonnes) sans modifier.
/// tool.schema.read — lecture seule.
pub fn read(
    ctx: &GovernedContext,
    executor: &dyn SchemaExecutor,
) -> Result<SchemaResult, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.read_tables(ctx)
}

/// @id miyusql_tool_schema_read_table
/// @role accessor
/// @layer tool
/// @do read_table_columns
//@human / @human Lit les colonnes d'une table.
pub fn read_table(
    ctx: &GovernedContext,
    table_name: &str,
    executor: &dyn SchemaExecutor,
) -> Result<TableMeta, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.read_table(ctx, table_name)
}

/// @id miyusql_schema_executor_trait
/// @role infrastructure
/// @layer tool
/// @do define_schema_executor
//@human / @human Trait de lecture schéma (abstraction backend).
pub trait SchemaExecutor: Send + Sync {
    /// @id miyusql_schema_read_tables_impl
    /// @role accessor
    /// @layer tool
    /// @do list_tables
    //@human / @human Liste les tables du schéma.
    fn read_tables(&self, ctx: &GovernedContext) -> Result<SchemaResult, MiyuSQLError>;

    /// @id miyusql_schema_read_table_impl
    /// @role accessor
    /// @layer tool
    /// @do get_table_meta
    //@human / @human Lit les métadonnées d'une table.
    fn read_table(
        &self,
        ctx: &GovernedContext,
        table_name: &str,
    ) -> Result<TableMeta, MiyuSQLError>;
}
