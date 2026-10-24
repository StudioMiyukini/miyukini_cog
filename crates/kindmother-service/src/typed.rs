//! @id     kindmother_service_typed
//! @do     valider_et_traduire_intentions_typees
//! @role   securite
//! @layer  core
//! @human  kindmother-service — validation des intentions typées contre le schéma et construction du SQL côté service (K-13, EX-07).

//! Chemin typé (lot C, K-13, EX-07) : aucun texte SQL ne vient de l'opérateur.
//!
//! Le service :
//! 1. contrôle la forme de l'intention (`TypedWriteIntent::check_shape`) ;
//! 2. vérifie que la table existe, est une table (pas une vue), n'est pas réservée
//!    (`_kindmother_*`, `sqlite_*`) et que chaque colonne citée existe ;
//! 3. construit le SQL à partir des **seuls identifiants vérifiés** (entre guillemets,
//!    alphabet `[A-Za-z0-9_]`) et lie toutes les valeurs en paramètres ;
//! 4. prépare l'instruction sous l'autorisateur (défense en profondeur) : une erreur
//!    de préparation (ex. `ON CONFLICT` sans contrainte d'unicité) est un refus.

use std::collections::{BTreeMap, BTreeSet};

use kindmother::intent::{IntentOperation, SqlValue, TypedSelect, TypedWriteIntent};
use rusqlite::types::Value;
use rusqlite::{Connection, OptionalExtension};

use crate::errors::ServiceError;

/// Nature d'un objet de schéma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectKind {
    Table,
    View,
}

/// Schéma d'une table ou vue.
#[derive(Debug, Clone)]
pub(crate) struct TableSchema {
    pub kind: ObjectKind,
    pub columns: Vec<String>,
}

impl TableSchema {
    fn has_column(&self, column: &str) -> bool {
        self.columns.iter().any(|c| c.eq_ignore_ascii_case(column))
    }
}

/// Lit le schéma de `table` (`None` si absente).
pub(crate) fn table_schema(
    conn: &Connection,
    table: &str,
) -> Result<Option<TableSchema>, ServiceError> {
    let kind: Option<String> = conn
        .query_row(
            "SELECT type FROM sqlite_master WHERE name = ?1 COLLATE NOCASE AND type IN ('table', 'view')",
            [table],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| ServiceError::Database(format!("Schema lookup failed: {e}")))?;
    let Some(kind) = kind else {
        return Ok(None);
    };
    let kind = if kind == "view" {
        ObjectKind::View
    } else {
        ObjectKind::Table
    };
    let mut stmt = conn
        .prepare("SELECT name FROM pragma_table_info(?1)")
        .map_err(|e| ServiceError::Database(format!("Schema lookup failed: {e}")))?;
    let columns = stmt
        .query_map([table], |r| r.get::<_, String>(0))
        .map_err(|e| ServiceError::Database(format!("Schema lookup failed: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ServiceError::Database(format!("Schema lookup failed: {e}")))?;
    Ok(Some(TableSchema { kind, columns }))
}

/// Identifiant entre guillemets (déjà validé : `[A-Za-z_][A-Za-z0-9_]*`).
fn quote(ident: &str) -> String {
    debug_assert!(kindmother::intent::is_valid_identifier(ident));
    format!("\"{ident}\"")
}

/// Convertit une valeur typée en valeur liée.
pub(crate) fn to_value(v: &SqlValue) -> Value {
    match v {
        SqlValue::Null => Value::Null,
        SqlValue::Integer(i) => Value::Integer(*i),
        SqlValue::Real(f) => Value::Real(*f),
        SqlValue::Text(s) => Value::Text(s.clone()),
        SqlValue::Blob(b) => Value::Blob(b.clone()),
    }
}

/// Clause `WHERE` d'égalités (`IS NULL` pour une valeur nulle).
fn where_clause(filter: &BTreeMap<String, SqlValue>, params: &mut Vec<Value>) -> String {
    let parts: Vec<String> = filter
        .iter()
        .map(|(column, value)| {
            if matches!(value, SqlValue::Null) {
                format!("{} IS NULL", quote(column))
            } else {
                params.push(to_value(value));
                format!("{} = ?{}", quote(column), params.len())
            }
        })
        .collect();
    parts.join(" AND ")
}

/// Valide une intention contre le schéma et construit son SQL paramétré.
///
/// `Err(motif)` : refus (l'intention n'est pas appliquée).
pub(crate) fn build_write(
    conn: &Connection,
    intent: &TypedWriteIntent,
) -> Result<(String, Vec<Value>), String> {
    intent.check_shape()?;
    let schema = table_schema(conn, &intent.table)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("unknown table: {}", intent.table))?;
    if schema.kind != ObjectKind::Table {
        return Err(format!("not a table: {}", intent.table));
    }
    for column in intent
        .values
        .keys()
        .chain(intent.increments.keys())
        .chain(intent.key.keys())
    {
        if !schema.has_column(column) {
            return Err(format!("unknown column {}.{column}", intent.table));
        }
    }

    let table = quote(&intent.table);
    let mut params = Vec::new();
    let sql = match intent.operation {
        IntentOperation::Insert => {
            let columns: Vec<String> = intent.values.keys().map(|c| quote(c)).collect();
            let placeholders: Vec<String> = intent
                .values
                .values()
                .map(|v| {
                    params.push(to_value(v));
                    format!("?{}", params.len())
                })
                .collect();
            format!(
                "INSERT INTO {table} ({}) VALUES ({})",
                columns.join(", "),
                placeholders.join(", ")
            )
        }
        IntentOperation::Update => {
            let mut sets: Vec<String> = Vec::new();
            for (column, value) in &intent.values {
                params.push(to_value(value));
                sets.push(format!("{} = ?{}", quote(column), params.len()));
            }
            for (column, value) in &intent.increments {
                params.push(to_value(value));
                sets.push(format!(
                    "{c} = {c} + ?{n}",
                    c = quote(column),
                    n = params.len()
                ));
            }
            let filter = where_clause(&intent.key, &mut params);
            format!("UPDATE {table} SET {} WHERE {filter}", sets.join(", "))
        }
        IntentOperation::Delete => {
            let filter = where_clause(&intent.key, &mut params);
            format!("DELETE FROM {table} WHERE {filter}")
        }
        IntentOperation::Upsert => {
            let mut columns = Vec::new();
            let mut placeholders = Vec::new();
            for (column, value) in intent.key.iter().chain(intent.values.iter()) {
                params.push(to_value(value));
                columns.push(quote(column));
                placeholders.push(format!("?{}", params.len()));
            }
            let conflict: Vec<String> = intent.key.keys().map(|c| quote(c)).collect();
            let action = if intent.values.is_empty() {
                "DO NOTHING".to_string()
            } else {
                let sets: Vec<String> = intent
                    .values
                    .keys()
                    .map(|c| format!("{q} = excluded.{q}", q = quote(c)))
                    .collect();
                format!("DO UPDATE SET {}", sets.join(", "))
            };
            format!(
                "INSERT INTO {table} ({}) VALUES ({}) ON CONFLICT ({}) {action}",
                columns.join(", "),
                placeholders.join(", "),
                conflict.join(", ")
            )
        }
    };
    Ok((sql, params))
}

/// Valide une lecture typée contre le schéma et construit son SQL paramétré.
pub(crate) fn build_select(
    conn: &Connection,
    query: &TypedSelect,
) -> Result<(String, Vec<Value>), String> {
    query.check_shape()?;
    let schema = table_schema(conn, &query.table)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("unknown table: {}", query.table))?;
    for column in query
        .columns
        .iter()
        .chain(query.filter.keys())
        .chain(query.order_by.iter().map(|o| &o.column))
    {
        if !schema.has_column(column) {
            return Err(format!("unknown column {}.{column}", query.table));
        }
    }
    let projection = if query.columns.is_empty() {
        "*".to_string()
    } else {
        query
            .columns
            .iter()
            .map(|c| quote(c))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut params = Vec::new();
    let mut sql = format!("SELECT {projection} FROM {}", quote(&query.table));
    if !query.filter.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_clause(&query.filter, &mut params));
    }
    if !query.order_by.is_empty() {
        let order: Vec<String> = query
            .order_by
            .iter()
            .map(|o| {
                format!(
                    "{} {}",
                    quote(&o.column),
                    if o.descending { "DESC" } else { "ASC" }
                )
            })
            .collect();
        sql.push_str(" ORDER BY ");
        sql.push_str(&order.join(", "));
    }
    if query.limit.is_some() || query.offset.is_some() {
        params.push(Value::Integer(query.limit.map_or(-1, i64::from)));
        sql.push_str(&format!(" LIMIT ?{}", params.len()));
        if let Some(offset) = query.offset {
            params.push(Value::Integer(i64::from(offset)));
            sql.push_str(&format!(" OFFSET ?{}", params.len()));
        }
    }
    Ok((sql, params))
}

/// Tables d'opérateur et leurs colonnes (hors catalogue et tables système).
pub(crate) fn schema_snapshot(
    conn: &Connection,
) -> Result<BTreeMap<String, BTreeSet<String>>, ServiceError> {
    let mut stmt = conn
        .prepare(
            "SELECT name FROM sqlite_master WHERE type = 'table'
               AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\'
               AND name NOT LIKE '\\_kindmother\\_%' ESCAPE '\\'",
        )
        .map_err(|e| ServiceError::Database(format!("Schema snapshot failed: {e}")))?;
    let tables = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| ServiceError::Database(format!("Schema snapshot failed: {e}")))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ServiceError::Database(format!("Schema snapshot failed: {e}")))?;
    let mut out = BTreeMap::new();
    for table in tables {
        let columns = table_schema(conn, &table)?
            .map(|s| s.columns.into_iter().map(|c| c.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        out.insert(table.to_ascii_lowercase(), columns);
    }
    Ok(out)
}

/// Vérifie qu'aucune table ni colonne de `before` n'a disparu dans `after`.
pub(crate) fn check_non_destructive(
    before: &BTreeMap<String, BTreeSet<String>>,
    after: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    for (table, columns) in before {
        let Some(now) = after.get(table) else {
            return Err(format!("table {table} would disappear"));
        };
        if let Some(missing) = columns.iter().find(|c| !now.contains(*c)) {
            return Err(format!("column {table}.{missing} would disappear"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().expect("mem");
        c.execute_batch(
            "CREATE TABLE t (id TEXT PRIMARY KEY, n INTEGER, v TEXT);
             CREATE VIEW vt AS SELECT id FROM t;
             CREATE TABLE _kindmother_meta (key TEXT);",
        )
        .expect("schema");
        c
    }

    fn apply(c: &Connection, i: &TypedWriteIntent) -> usize {
        let (sql, params) = build_write(c, i).expect("build");
        c.execute(&sql, rusqlite::params_from_iter(params)).expect("exec")
    }

    #[test]
    fn insert_update_upsert_delete() {
        let c = conn();
        assert_eq!(apply(&c, &TypedWriteIntent::insert("t").value("id", "a").value("n", 1_i64)), 1);
        assert_eq!(
            apply(&c, &TypedWriteIntent::update("t").key("id", "a").increment("n", 2_i64).value("v", "x")),
            1
        );
        let n: i64 = c.query_row("SELECT n FROM t WHERE id='a'", [], |r| r.get(0)).expect("n");
        assert_eq!(n, 3);
        assert_eq!(apply(&c, &TypedWriteIntent::upsert("t").key("id", "a").value("v", "y")), 1);
        assert_eq!(apply(&c, &TypedWriteIntent::upsert("t").key("id", "b").value("v", "z")), 1);
        let count: i64 = c.query_row("SELECT COUNT(*) FROM t", [], |r| r.get(0)).expect("c");
        assert_eq!(count, 2);
        assert_eq!(apply(&c, &TypedWriteIntent::delete("t").key("id", "b")), 1);
        assert_eq!(apply(&c, &TypedWriteIntent::delete("t").key("v", SqlValue::Null)), 0);
    }

    #[test]
    fn refus_schema() {
        let c = conn();
        assert!(build_write(&c, &TypedWriteIntent::insert("absente").value("id", "a")).is_err());
        assert!(build_write(&c, &TypedWriteIntent::insert("t").value("inconnue", "a")).is_err());
        assert!(build_write(&c, &TypedWriteIntent::insert("vt").value("id", "a")).is_err());
        assert!(build_write(&c, &TypedWriteIntent::insert("_kindmother_meta").value("key", "a")).is_err());
        assert!(build_write(&c, &TypedWriteIntent::delete("t").key("id; DROP", "a")).is_err());
        // Les valeurs ne sont jamais interprétées.
        let hostile = "x'); DELETE FROM t; --";
        assert_eq!(apply(&c, &TypedWriteIntent::insert("t").value("id", hostile)), 1);
        let v: String = c.query_row("SELECT id FROM t", [], |r| r.get(0)).expect("id");
        assert_eq!(v, hostile);
    }

    #[test]
    fn lecture_typee() {
        let c = conn();
        apply(&c, &TypedWriteIntent::insert("t").value("id", "a").value("n", 1_i64));
        apply(&c, &TypedWriteIntent::insert("t").value("id", "b").value("n", 2_i64));
        let q = TypedSelect::from("t").columns(["id"]).order_by("n", true).limit(1);
        let (sql, params) = build_select(&c, &q).expect("select");
        let id: String = c
            .query_row(&sql, rusqlite::params_from_iter(params), |r| r.get(0))
            .expect("row");
        assert_eq!(id, "b");
        assert!(build_select(&c, &TypedSelect::from("t").filter("zz", 1_i64)).is_err());
        assert!(build_select(&c, &TypedSelect::from("vt")).is_ok());
    }

    #[test]
    fn instantane_non_destructif() {
        let c = conn();
        let before = schema_snapshot(&c).expect("before");
        assert!(before.contains_key("t") && !before.contains_key("_kindmother_meta"));
        c.execute_batch("ALTER TABLE t ADD COLUMN w TEXT").expect("add");
        let after = schema_snapshot(&c).expect("after");
        assert!(check_non_destructive(&before, &after).is_ok());
        c.execute_batch("ALTER TABLE t DROP COLUMN v").expect("drop col");
        let after = schema_snapshot(&c).expect("after");
        assert!(check_non_destructive(&before, &after).is_err());
    }
}
