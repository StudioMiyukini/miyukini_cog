//! Tool schema : tool.schema.read.
//!
//! Lecture des métadonnées du schéma (tables, colonnes) sans modification.

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id: miyusql_schema_table_meta
/// @role: data
/// @layer: tool
/// @human: Métadonnées d'une table.
/// @do: represent_table_metadata
#[derive(Debug, Clone)]
pub struct TableMeta {
    /// @id: miyusql_schema_table_name
    /// @role: data
    /// @layer: tool
    /// @human: Nom de la table.
    /// @do: store_table_name
    pub name: String,
    /// @id: miyusql_schema_columns
    /// @role: data
    /// @layer: tool
    /// @human: Colonnes (nom, type).
    /// @do: store_columns
    pub columns: Vec<ColumnMeta>,
}

/// @id: miyusql_schema_column_meta
/// @role: data
/// @layer: tool
/// @human: Métadonnées d'une colonne.
/// @do: represent_column_metadata
#[derive(Debug, Clone)]
pub struct ColumnMeta {
    /// @id: miyusql_schema_column_name
    /// @role: data
    /// @layer: tool
    pub name: String,
    /// @id: miyusql_schema_column_type
    /// @role: data
    /// @layer: tool
    pub data_type: String,
}

/// @id: miyusql_schema_result
/// @role: data
/// @layer: tool
/// @human: Résultat de lecture schéma (tables ou colonnes d'une table).
/// @do: represent_schema_result
#[derive(Debug, Clone)]
pub struct SchemaResult {
    /// @id: miyusql_schema_tables
    /// @role: data
    /// @layer: tool
    pub tables: Vec<TableMeta>,
}

/// @id: miyusql_tool_schema_read
/// @role: accessor
/// @layer: tool
/// @human: Lit les métadonnées du schéma (tables, colonnes) sans modifier.
/// @do: read_schema_metadata
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

/// @id: miyusql_tool_schema_read_table
/// @role: accessor
/// @layer: tool
/// @human: Lit les colonnes d'une table.
/// @do: read_table_columns
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

/// @id: miyusql_schema_executor_trait
/// @role: infrastructure
/// @layer: tool
/// @human: Trait de lecture schéma (abstraction backend).
/// @do: define_schema_executor
pub trait SchemaExecutor: Send + Sync {
    /// @id: miyusql_schema_read_tables_impl
    /// @role: accessor
    /// @layer: tool
    /// @human: Liste les tables du schéma.
    /// @do: list_tables
    fn read_tables(&self, ctx: &GovernedContext) -> Result<SchemaResult, MiyuSQLError>;

    /// @id: miyusql_schema_read_table_impl
    /// @role: accessor
    /// @layer: tool
    /// @human: Lit les métadonnées d'une table.
    /// @do: get_table_meta
    fn read_table(
        &self,
        ctx: &GovernedContext,
        table_name: &str,
    ) -> Result<TableMeta, MiyuSQLError>;
}
