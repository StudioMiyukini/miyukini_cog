//! Exécuteur en mémoire pour tests (sandbox / MiyukiniSQLtest).
//!
//! Implémentation des traits QueryExecutor, TransactionExecutor, CacheExecutor, SchemaExecutor
//! sans driver SQL réel — utilisé pour tests unitaires et cycle.

use crate::cache::CacheExecutor;
use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;
use crate::query::{QueryExecutor, QueryResult};
use crate::schema::{ColumnMeta, SchemaExecutor, SchemaResult, TableMeta};
use crate::transaction::TransactionExecutor;
use std::collections::HashMap;
use std::sync::Mutex;

// @id: miyusql_memory_executor
// @role: infrastructure
// @layer: tool
// @human: Exécuteur en mémoire pour tests (tables, cache, transactions).
// @do: implement_memory_executor
// État partagé : tables (nom -> lignes), cache (clé -> valeur), transactions.

/// Alias pour une ligne de table (colonne -> valeur).
type TableRow = HashMap<String, String>;
/// Alias pour une entrée de cache (données, TTL optionnel).
type CacheEntry = (Vec<u8>, Option<u64>);
/// Alias pour les données de transaction (snapshot des tables).
type TransactionSnapshot = HashMap<String, Vec<TableRow>>;

/// Exécuteur en mémoire pour tests (sandbox / MiyukiniSQLtest).
pub struct MemoryExecutor {
    /// Tables : nom -> vec de rows (HashMap colonne -> valeur)
    tables: Mutex<HashMap<String, Vec<TableRow>>>,
    /// Schéma : nom table -> colonnes (nom, type)
    schema: Mutex<HashMap<String, Vec<(String, String)>>>,
    /// Cache : clé -> (données, TTL optionnel)
    cache: Mutex<HashMap<String, CacheEntry>>,
    /// tx_id -> snapshot des tables au begin
    transactions: Mutex<HashMap<String, TransactionSnapshot>>,
}

impl Default for MemoryExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryExecutor {
    /// Crée un exécuteur en mémoire pour tests (sandbox / MiyukiniSQLtest).
    #[must_use] 
    pub fn new() -> Self {
        Self {
            tables: Mutex::new(HashMap::new()),
            schema: Mutex::new(HashMap::new()),
            cache: Mutex::new(HashMap::new()),
            transactions: Mutex::new(HashMap::new()),
        }
    }

    fn exec_sql(
        tables: &mut HashMap<String, Vec<HashMap<String, String>>>,
        schema: &mut HashMap<String, Vec<(String, String)>>,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        let sql_upper = sql.trim().to_uppercase();
        if sql_upper.is_empty() {
            return Err(MiyuSQLError::Syntax("empty query".into()));
        }
        if sql_upper.starts_with("CREATE TABLE") {
            let name = extract_table_name_create(sql).unwrap_or_else(|| "unknown".to_string());
            if !tables.contains_key(&name) {
                tables.insert(name.clone(), Vec::new());
                schema.insert(name, vec![]);
            }
            return Ok(QueryResult { rows: vec![], count: 0 });
        }
        if sql_upper.contains("ADD COLUMN") || sql_upper.starts_with("ALTER TABLE") {
            if let Some((table, col, ty)) = extract_alter_column(sql) {
                schema.entry(table).or_default().push((col, ty));
            }
            return Ok(QueryResult { rows: vec![], count: 0 });
        }
        if sql_upper.starts_with("INSERT INTO") {
            let (table, cols, vals) = extract_insert(sql, params).unwrap_or((String::new(), vec![], vec![]));
            if table.is_empty() {
                return Err(MiyuSQLError::Syntax("invalid INSERT".into()));
            }
            let row: HashMap<String, String> = cols.into_iter().zip(vals).collect();
            let ent = tables.entry(table).or_default();
            ent.push(row);
            return Ok(QueryResult { rows: vec![], count: ent.len() as u64 });
        }
        if sql_upper.starts_with("SELECT") {
            let (table, _cols) = extract_select(sql).unwrap_or((String::new(), vec![]));
            if table.is_empty() {
                return Ok(QueryResult { rows: vec![], count: 0 });
            }
            let rows = tables.get(&table).cloned().unwrap_or_default();
            let count = rows.len() as u64;
            return Ok(QueryResult { rows, count });
        }
        if sql_upper.starts_with("DELETE FROM") {
            let table = extract_table_name_delete(sql).unwrap_or_default();
            let removed = tables.get_mut(&table).map_or(0, |r| r.len());
            tables.entry(table).or_default().clear();
            return Ok(QueryResult { rows: vec![], count: removed as u64 });
        }
        if sql_upper.starts_with("DROP TABLE") {
            let name = extract_table_name_drop(sql).unwrap_or_default();
            tables.remove(&name);
            schema.remove(&name);
            return Ok(QueryResult { rows: vec![], count: 0 });
        }
        Ok(QueryResult { rows: vec![], count: 0 })
    }
}

fn extract_table_name_create(sql: &str) -> Option<String> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("CREATE TABLE")?;
    let rest = s.get(idx + 12..)?.trim();
    let tokens: Vec<&str> = rest.split_whitespace().collect();
    let name = if tokens.len() >= 4 && tokens[1].eq_ignore_ascii_case("IF") && tokens[2].eq_ignore_ascii_case("NOT") && tokens[3].eq_ignore_ascii_case("EXISTS") {
        tokens.get(4)
    } else {
        tokens.first()
    };
    name.map(|n| n.trim_matches('"').trim_matches('(').to_string())
}

fn extract_alter_column(sql: &str) -> Option<(String, String, String)> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("ALTER TABLE")?;
    let rest = s.get(idx + 11..)?.trim();
    let table = rest.split_whitespace().next()?.trim_matches('"').to_string();
    let add_pos = rest.to_uppercase().find("ADD")?;
    let rest2 = rest[add_pos + 3..].trim();
    let idx2 = rest2.to_uppercase().find("COLUMN")?;
    let rest3 = rest2.get(idx2 + 6..)?.trim();
    let parts: Vec<&str> = rest3.split_whitespace().collect();
    let col = parts.first()?.trim_matches('"').to_string();
    let ty = parts.get(1).copied().unwrap_or("text").to_string();
    Some((table, col, ty))
}

fn extract_insert(sql: &str, params: &[String]) -> Option<(String, Vec<String>, Vec<String>)> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("INSERT INTO")?;
    let rest = s.get(idx + 11..)?.trim();
    let table = rest.split_whitespace().next()?.trim_matches('"').to_string();
    let idx2 = rest.find('(')?;
    let idx3 = rest.find(')')?;
    let cols_str = rest.get(idx2 + 1..idx3)?;
    let cols: Vec<String> = cols_str.split(',').map(|c| c.trim().trim_matches('"').to_string()).collect();
    let vals = if params.is_empty() {
        let vidx = rest.to_uppercase().rfind("VALUES")?;
        let vpart = rest.get(vidx + 6..)?.trim().trim_matches('(').trim_matches(')');
        vpart.split(',').map(|v| v.trim().trim_matches('\'').to_string()).collect()
    } else {
        params.to_vec()
    };
    Some((table, cols, vals))
}

fn extract_select(sql: &str) -> Option<(String, Vec<String>)> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("FROM")?;
    let rest = s.get(idx + 4..)?.trim();
    let table = rest.split_whitespace().next()?.trim_matches('"').to_string();
    Some((table, vec![]))
}

fn extract_table_name_delete(sql: &str) -> Option<String> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("DELETE FROM")?;
    let rest = s.get(idx + 11..)?.trim();
    Some(rest.split_whitespace().next()?.trim_matches('"').to_string())
}

fn extract_table_name_drop(sql: &str) -> Option<String> {
    let s = sql.trim();
    let idx = s.to_uppercase().find("DROP TABLE")?;
    let rest = s.get(idx + 10..)?.trim();
    Some(rest.split_whitespace().next()?.trim_matches('"').to_string())
}

impl QueryExecutor for MemoryExecutor {
    fn execute(
        &self,
        ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let mut tables = self.tables.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let mut schema = self.schema.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        Self::exec_sql(&mut tables, &mut schema, sql, params)
    }

    fn prepare(&self, ctx: &GovernedContext, sql: &str, params: &[String]) -> Result<(), MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        if sql.trim().is_empty() {
            return Err(MiyuSQLError::Syntax("empty query".into()));
        }
        let _ = params;
        Ok(())
    }
}

impl TransactionExecutor for MemoryExecutor {
    fn begin(&self, ctx: &GovernedContext) -> Result<String, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let tx_id = format!("tx-{}", ctx.mandate_id);
        let tables = self.tables.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let snapshot: TransactionSnapshot = tables.clone();
        drop(tables);
        let mut tx = self.transactions.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        tx.insert(tx_id.clone(), snapshot);
        Ok(tx_id)
    }

    fn commit(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let mut tx = self.transactions.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        tx.remove(tx_id);
        Ok(())
    }

    fn rollback(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let mut tx = self.transactions.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        if let Some(snapshot) = tx.remove(tx_id) {
            let mut tables = self.tables.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
            *tables = snapshot;
        }
        Ok(())
    }
}

impl CacheExecutor for MemoryExecutor {
    fn get(&self, ctx: &GovernedContext, key: &str) -> Result<Option<Vec<u8>>, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let cache = self.cache.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        Ok(cache.get(key).map(|(v, _)| v.clone()))
    }

    fn set(
        &self,
        ctx: &GovernedContext,
        key: &str,
        value: &[u8],
        ttl_secs: Option<u64>,
    ) -> Result<(), MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let mut cache = self.cache.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        cache.insert(key.to_string(), (value.to_vec(), ttl_secs));
        Ok(())
    }

    fn invalidate(&self, ctx: &GovernedContext, key_or_pattern: &str) -> Result<u64, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let mut cache = self.cache.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let before = cache.len();
        if key_or_pattern.contains('*') {
            let prefix = key_or_pattern.trim_end_matches('*');
            cache.retain(|k, _| !k.starts_with(prefix));
        } else {
            cache.remove(key_or_pattern);
        }
        Ok((before - cache.len()) as u64)
    }
}

impl SchemaExecutor for MemoryExecutor {
    fn read_tables(&self, ctx: &GovernedContext) -> Result<SchemaResult, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let tables_guard = self.tables.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let schema = self.schema.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let tables: Vec<TableMeta> = tables_guard
            .keys()
            .map(|name| {
                let columns = schema
                    .get(name)
                    .map(|cols| cols.iter().map(|(n, t)| ColumnMeta { name: n.clone(), data_type: t.clone() }).collect())
                    .unwrap_or_default();
                TableMeta { name: name.clone(), columns }
            })
            .collect();
        Ok(SchemaResult { tables })
    }

    fn read_table(&self, ctx: &GovernedContext, table_name: &str) -> Result<TableMeta, MiyuSQLError> {
        if !ctx.has_mandate() {
            return Err(MiyuSQLError::NoMandate);
        }
        let schema = self.schema.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))?;
        let columns = schema
            .get(table_name)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|(name, data_type)| ColumnMeta { name, data_type })
            .collect();
        Ok(TableMeta {
            name: table_name.to_string(),
            columns,
        })
    }
}
