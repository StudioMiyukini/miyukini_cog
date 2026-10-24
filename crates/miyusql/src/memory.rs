//! @id     miyusql_memory
//! @do     fournir_executeur_memoire_de_test
//! @role   infrastructure
//! @layer  tool
//! @human  miyusql — exécuteur en mémoire réservé aux tests (sandbox / MiyukiniSQLtest).
//!
//! Exécuteur en mémoire pour tests (sandbox / MiyukiniSQLtest).
//!
//! Implémentation des traits QueryExecutor, TransactionExecutor, CacheExecutor, SchemaExecutor
//! sans driver SQL réel. Compilé uniquement sous `cfg(test)` ou avec la feature
//! `test-support` (désactivée par défaut) : il n'est pas destiné à la production.
//!
//! ## Sous-ensemble SQL géré (tout le reste est refusé, K-19 / C-14)
//!
//! - `CREATE TABLE [IF NOT EXISTS] t [(col type, …)]`
//! - `ALTER TABLE t ADD [COLUMN] col type`
//! - `INSERT INTO t (cols…) VALUES (littéraux ou ?…)`
//! - `SELECT * FROM t` (sans clause)
//! - `DELETE FROM t` (table entière, sans clause)
//! - `DROP TABLE t`
//!
//! Toute clause non gérée (`WHERE`, `ORDER BY`, `LIMIT`, projection autre que `*`,
//! contraintes de colonnes, TTL de cache…) renvoie une erreur au lieu d'être ignorée.

use crate::cache::CacheExecutor;
use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;
use crate::query::{QueryExecutor, QueryResult};
use crate::schema::{ColumnMeta, SchemaExecutor, SchemaResult, TableMeta};
use crate::transaction::TransactionExecutor;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

// @id miyusql_memory_executor
// @role infrastructure
// @layer tool
// @do implement_memory_executor
// @human Exécuteur en mémoire pour tests (tables, cache, transactions).
// État partagé : tables (nom -> lignes), cache (clé -> valeur), transactions.

/// Alias pour une ligne de table (colonne -> valeur).
type TableRow = HashMap<String, String>;
/// Alias pour les données de transaction (snapshot des tables et du schéma).
type TransactionSnapshot = (
    HashMap<String, Vec<TableRow>>,
    HashMap<String, Vec<(String, String)>>,
);

/// Instruction SQL reconnue par l'exécuteur en mémoire.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    CreateTable {
        table: String,
        if_not_exists: bool,
        columns: Vec<(String, String)>,
    },
    AddColumn {
        table: String,
        column: String,
        data_type: String,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<String>,
    },
    SelectAll {
        table: String,
    },
    DeleteAll {
        table: String,
    },
    DropTable {
        table: String,
    },
}

/// Exécuteur en mémoire pour tests (sandbox / MiyukiniSQLtest).
pub struct MemoryExecutor {
    /// Tables : nom -> vec de rows (HashMap colonne -> valeur)
    tables: Mutex<HashMap<String, Vec<TableRow>>>,
    /// Schéma : nom table -> colonnes (nom, type)
    schema: Mutex<HashMap<String, Vec<(String, String)>>>,
    /// Cache : clé -> données (sans TTL : un TTL est refusé)
    cache: Mutex<HashMap<String, Vec<u8>>>,
    /// tx_id -> snapshot des tables au begin
    transactions: Mutex<HashMap<String, TransactionSnapshot>>,
}

impl Default for MemoryExecutor {
    fn default() -> Self {
        Self::new()
    }
}

fn lock<T>(m: &Mutex<T>) -> Result<MutexGuard<'_, T>, MiyuSQLError> {
    m.lock().map_err(|_| MiyuSQLError::Execution("lock".into()))
}

fn non_gere(msg: impl Into<String>) -> MiyuSQLError {
    MiyuSQLError::NonImplemente(msg.into())
}

fn vide() -> QueryResult {
    QueryResult {
        rows: vec![],
        count: 0,
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
        tables: &mut HashMap<String, Vec<TableRow>>,
        schema: &mut HashMap<String, Vec<(String, String)>>,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        match parse(sql, params)? {
            Instruction::CreateTable {
                table,
                if_not_exists,
                columns,
            } => {
                if tables.contains_key(&table) {
                    if if_not_exists {
                        return Ok(vide());
                    }
                    return Err(MiyuSQLError::Execution(format!(
                        "table « {table} » déjà existante"
                    )));
                }
                tables.insert(table.clone(), Vec::new());
                schema.insert(table, columns);
                Ok(vide())
            }
            Instruction::AddColumn {
                table,
                column,
                data_type,
            } => {
                let cols = schema
                    .get_mut(&table)
                    .ok_or_else(|| table_inconnue(&table))?;
                if cols.iter().any(|(n, _)| *n == column) {
                    return Err(MiyuSQLError::Execution(format!(
                        "colonne « {column} » déjà existante"
                    )));
                }
                cols.push((column, data_type));
                Ok(vide())
            }
            Instruction::Insert {
                table,
                columns,
                values,
            } => {
                let cols = schema.get(&table).ok_or_else(|| table_inconnue(&table))?;
                if let Some(c) = columns.iter().find(|c| !cols.iter().any(|(n, _)| n == *c)) {
                    return Err(MiyuSQLError::Execution(format!(
                        "colonne « {c} » inconnue dans « {table} »"
                    )));
                }
                let row: TableRow = columns.into_iter().zip(values).collect();
                let rows = tables
                    .get_mut(&table)
                    .ok_or_else(|| table_inconnue(&table))?;
                rows.push(row);
                Ok(QueryResult {
                    rows: vec![],
                    count: 1,
                })
            }
            Instruction::SelectAll { table } => {
                let rows = tables
                    .get(&table)
                    .cloned()
                    .ok_or_else(|| table_inconnue(&table))?;
                let count = rows.len() as u64;
                Ok(QueryResult { rows, count })
            }
            Instruction::DeleteAll { table } => {
                let rows = tables
                    .get_mut(&table)
                    .ok_or_else(|| table_inconnue(&table))?;
                let removed = rows.len() as u64;
                rows.clear();
                Ok(QueryResult {
                    rows: vec![],
                    count: removed,
                })
            }
            Instruction::DropTable { table } => {
                if tables.remove(&table).is_none() {
                    return Err(table_inconnue(&table));
                }
                schema.remove(&table);
                Ok(vide())
            }
        }
    }
}

fn table_inconnue(table: &str) -> MiyuSQLError {
    MiyuSQLError::Execution(format!("table « {table} » inexistante"))
}

/// Retire le mot-clé `kw` (insensible à la casse, ASCII) en tête de `s`, s'il est
/// suivi d'une fin, d'un blanc, de `(` ou de `*`. Renvoie le reste sans blancs initiaux.
fn strip_kw<'a>(s: &'a str, kw: &str) -> Option<&'a str> {
    let s = s.trim_start();
    let head = s.get(..kw.len())?;
    if !head.eq_ignore_ascii_case(kw) {
        return None;
    }
    let rest = &s[kw.len()..];
    match rest.chars().next() {
        None => Some(rest),
        Some(c) if c.is_whitespace() || c == '(' || c == '*' => Some(rest.trim_start()),
        Some(_) => None,
    }
}

/// Lit un identifiant (`nom` ou `"nom"`), renvoie (identifiant, reste).
fn take_ident(s: &str) -> Result<(String, &str), MiyuSQLError> {
    let s = s.trim_start();
    if let Some(inner) = s.strip_prefix('"') {
        let end = inner
            .find('"')
            .ok_or_else(|| MiyuSQLError::Syntax("identifiant non fermé".into()))?;
        let id = &inner[..end];
        if id.is_empty() {
            return Err(MiyuSQLError::Syntax("identifiant vide".into()));
        }
        return Ok((id.to_string(), &inner[end + 1..]));
    }
    let end = s
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(s.len());
    if end == 0 {
        return Err(MiyuSQLError::Syntax("identifiant attendu".into()));
    }
    Ok((s[..end].to_string(), &s[end..]))
}

/// Exige qu'il ne reste rien après l'instruction (sinon clause non gérée).
fn fin(rest: &str) -> Result<(), MiyuSQLError> {
    let rest = rest.trim();
    if rest.is_empty() {
        Ok(())
    } else {
        Err(non_gere(format!("clause non gérée : « {rest} »")))
    }
}

/// Extrait le contenu d'un groupe parenthésé en tête, renvoie (intérieur, reste).
fn take_parens(s: &str) -> Result<(&str, &str), MiyuSQLError> {
    let s = s.trim_start();
    let inner = s
        .strip_prefix('(')
        .ok_or_else(|| MiyuSQLError::Syntax("« ( » attendu".into()))?;
    let mut in_quote = false;
    for (i, c) in inner.char_indices() {
        match c {
            '\'' => in_quote = !in_quote,
            ')' if !in_quote => return Ok((&inner[..i], &inner[i + 1..])),
            _ => {}
        }
    }
    Err(MiyuSQLError::Syntax("« ) » attendu".into()))
}

/// Découpe une liste de littéraux (`'texte'`, `nombre`, `?`) séparés par des virgules.
fn parse_values(inner: &str, params: &[String]) -> Result<Vec<String>, MiyuSQLError> {
    let mut values = Vec::new();
    let mut next_param = params.iter();
    let mut rest = inner.trim();
    loop {
        let (value, after) = if let Some(q) = rest.strip_prefix('\'') {
            // Chaîne entre apostrophes, '' = apostrophe échappée.
            let mut out = String::new();
            let mut chars = q.char_indices().peekable();
            let mut end = None;
            while let Some((i, c)) = chars.next() {
                if c == '\'' {
                    if matches!(chars.peek(), Some((_, '\''))) {
                        out.push('\'');
                        chars.next();
                    } else {
                        end = Some(i + 1);
                        break;
                    }
                } else {
                    out.push(c);
                }
            }
            let end = end.ok_or_else(|| MiyuSQLError::Syntax("chaîne non fermée".into()))?;
            (out, &q[end..])
        } else if let Some(after) = rest.strip_prefix('?') {
            let p = next_param
                .next()
                .ok_or_else(|| MiyuSQLError::Syntax("paramètre manquant".into()))?;
            (p.clone(), after)
        } else {
            let end = rest.find(',').unwrap_or(rest.len());
            let tok = rest[..end].trim();
            if tok.is_empty()
                || !tok
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
            {
                return Err(non_gere(format!("valeur non gérée : « {tok} »")));
            }
            (tok.to_string(), &rest[end..])
        };
        values.push(value);
        let after = after.trim_start();
        if after.is_empty() {
            break;
        }
        rest = after
            .strip_prefix(',')
            .ok_or_else(|| MiyuSQLError::Syntax("« , » attendue".into()))?
            .trim_start();
    }
    if next_param.next().is_some() {
        return Err(MiyuSQLError::Syntax("paramètres en trop".into()));
    }
    Ok(values)
}

/// Analyse une instruction SQL du sous-ensemble géré ; tout le reste est refusé.
fn parse(sql: &str, params: &[String]) -> Result<Instruction, MiyuSQLError> {
    let s = sql.trim();
    let s = s.strip_suffix(';').unwrap_or(s).trim();
    if s.is_empty() {
        return Err(MiyuSQLError::Syntax("empty query".into()));
    }
    if s.contains(';') {
        return Err(non_gere("instructions multiples"));
    }
    let sans_params = |i: Instruction| {
        if params.is_empty() {
            Ok(i)
        } else {
            Err(MiyuSQLError::Syntax("paramètres non utilisés".into()))
        }
    };

    if let Some(r) = strip_kw(s, "CREATE").and_then(|r| strip_kw(r, "TABLE")) {
        let (if_not_exists, r) = match strip_kw(r, "IF")
            .and_then(|r| strip_kw(r, "NOT"))
            .and_then(|r| strip_kw(r, "EXISTS"))
        {
            Some(r) => (true, r),
            None => (false, r),
        };
        let (table, r) = take_ident(r)?;
        let mut columns = Vec::new();
        let r = r.trim();
        if !r.is_empty() {
            let (inner, after) = take_parens(r)?;
            fin(after)?;
            for def in inner.split(',') {
                let (col, rest) = take_ident(def)?;
                let (ty, rest) = take_ident(rest)?;
                fin(rest)?;
                columns.push((col, ty));
            }
        }
        return sans_params(Instruction::CreateTable {
            table,
            if_not_exists,
            columns,
        });
    }
    if let Some(r) = strip_kw(s, "ALTER").and_then(|r| strip_kw(r, "TABLE")) {
        let (table, r) = take_ident(r)?;
        let r = strip_kw(r, "ADD").ok_or_else(|| non_gere("seul ALTER TABLE … ADD est géré"))?;
        let r = strip_kw(r, "COLUMN").unwrap_or(r);
        let (column, r) = take_ident(r)?;
        let (data_type, r) = take_ident(r)?;
        fin(r)?;
        return sans_params(Instruction::AddColumn {
            table,
            column,
            data_type,
        });
    }
    if let Some(r) = strip_kw(s, "INSERT").and_then(|r| strip_kw(r, "INTO")) {
        let (table, r) = take_ident(r)?;
        let (cols, r) = take_parens(r)?;
        let columns = cols
            .split(',')
            .map(|c| {
                let (id, rest) = take_ident(c)?;
                fin(rest)?;
                Ok(id)
            })
            .collect::<Result<Vec<_>, MiyuSQLError>>()?;
        let r = strip_kw(r, "VALUES").ok_or_else(|| non_gere("seul INSERT … VALUES est géré"))?;
        let (vals, r) = take_parens(r)?;
        fin(r)?;
        let values = parse_values(vals, params)?;
        if values.len() != columns.len() {
            return Err(MiyuSQLError::Syntax(format!(
                "{} colonne(s) pour {} valeur(s)",
                columns.len(),
                values.len()
            )));
        }
        return Ok(Instruction::Insert {
            table,
            columns,
            values,
        });
    }
    if let Some(r) = strip_kw(s, "SELECT") {
        let r = r
            .strip_prefix('*')
            .ok_or_else(|| non_gere("seule la projection « * » est gérée"))?;
        let r = strip_kw(r, "FROM").ok_or_else(|| MiyuSQLError::Syntax("FROM attendu".into()))?;
        let (table, r) = take_ident(r)?;
        fin(r)?;
        return sans_params(Instruction::SelectAll { table });
    }
    if let Some(r) = strip_kw(s, "DELETE").and_then(|r| strip_kw(r, "FROM")) {
        let (table, r) = take_ident(r)?;
        fin(r)?;
        return sans_params(Instruction::DeleteAll { table });
    }
    if let Some(r) = strip_kw(s, "DROP").and_then(|r| strip_kw(r, "TABLE")) {
        let (table, r) = take_ident(r)?;
        fin(r)?;
        return sans_params(Instruction::DropTable { table });
    }
    Err(non_gere("instruction non gérée"))
}

impl QueryExecutor for MemoryExecutor {
    fn execute(
        &self,
        ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError> {
        ctx.verifier_mandat()?;
        let mut tables = lock(&self.tables)?;
        let mut schema = lock(&self.schema)?;
        Self::exec_sql(&mut tables, &mut schema, sql, params)
    }

    fn prepare(
        &self,
        ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<(), MiyuSQLError> {
        ctx.verifier_mandat()?;
        parse(sql, params).map(|_| ())
    }
}

impl TransactionExecutor for MemoryExecutor {
    fn begin(&self, ctx: &GovernedContext) -> Result<String, MiyuSQLError> {
        ctx.verifier_mandat()?;
        let tx_id = format!("tx-{}", ctx.mandate_id);
        let snapshot = (lock(&self.tables)?.clone(), lock(&self.schema)?.clone());
        let mut tx = lock(&self.transactions)?;
        if tx.contains_key(&tx_id) {
            return Err(MiyuSQLError::Execution(format!(
                "transaction « {tx_id} » déjà ouverte"
            )));
        }
        tx.insert(tx_id.clone(), snapshot);
        Ok(tx_id)
    }

    fn commit(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError> {
        ctx.verifier_mandat()?;
        lock(&self.transactions)?
            .remove(tx_id)
            .map(|_| ())
            .ok_or_else(|| MiyuSQLError::Execution(format!("transaction « {tx_id} » inconnue")))
    }

    fn rollback(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError> {
        ctx.verifier_mandat()?;
        let (tables_snap, schema_snap) = lock(&self.transactions)?
            .remove(tx_id)
            .ok_or_else(|| MiyuSQLError::Execution(format!("transaction « {tx_id} » inconnue")))?;
        *lock(&self.tables)? = tables_snap;
        *lock(&self.schema)? = schema_snap;
        Ok(())
    }
}

impl CacheExecutor for MemoryExecutor {
    fn get(&self, ctx: &GovernedContext, key: &str) -> Result<Option<Vec<u8>>, MiyuSQLError> {
        ctx.verifier_mandat()?;
        Ok(lock(&self.cache)?.get(key).cloned())
    }

    fn set(
        &self,
        ctx: &GovernedContext,
        key: &str,
        value: &[u8],
        ttl_secs: Option<u64>,
    ) -> Result<(), MiyuSQLError> {
        ctx.verifier_mandat()?;
        if ttl_secs.is_some() {
            return Err(non_gere("TTL de cache non géré par l'exécuteur en mémoire"));
        }
        lock(&self.cache)?.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn invalidate(&self, ctx: &GovernedContext, key_or_pattern: &str) -> Result<u64, MiyuSQLError> {
        ctx.verifier_mandat()?;
        let mut cache = lock(&self.cache)?;
        let before = cache.len();
        if let Some(prefix) = key_or_pattern.strip_suffix('*') {
            if prefix.contains('*') {
                return Err(non_gere("seul un « * » final est géré dans un motif"));
            }
            cache.retain(|k, _| !k.starts_with(prefix));
        } else if key_or_pattern.contains('*') {
            return Err(non_gere("seul un « * » final est géré dans un motif"));
        } else {
            cache.remove(key_or_pattern);
        }
        Ok((before - cache.len()) as u64)
    }
}

impl SchemaExecutor for MemoryExecutor {
    fn read_tables(&self, ctx: &GovernedContext) -> Result<SchemaResult, MiyuSQLError> {
        ctx.verifier_mandat()?;
        let tables_guard = lock(&self.tables)?;
        let schema = lock(&self.schema)?;
        let tables: Vec<TableMeta> = tables_guard
            .keys()
            .map(|name| TableMeta {
                name: name.clone(),
                columns: schema
                    .get(name)
                    .map(|cols| {
                        cols.iter()
                            .map(|(n, t)| ColumnMeta {
                                name: n.clone(),
                                data_type: t.clone(),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            })
            .collect();
        Ok(SchemaResult { tables })
    }

    fn read_table(
        &self,
        ctx: &GovernedContext,
        table_name: &str,
    ) -> Result<TableMeta, MiyuSQLError> {
        ctx.verifier_mandat()?;
        let schema = lock(&self.schema)?;
        let columns = schema
            .get(table_name)
            .ok_or_else(|| table_inconnue(table_name))?
            .iter()
            .map(|(name, data_type)| ColumnMeta {
                name: name.clone(),
                data_type: data_type.clone(),
            })
            .collect();
        Ok(TableMeta {
            name: table_name.to_string(),
            columns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::SignatureMandat;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m-test".into(), 2).avec_signature(SignatureMandat::pour_tests())
    }

    fn exec_avec_lignes() -> MemoryExecutor {
        let e = MemoryExecutor::new();
        let c = ctx();
        e.execute(&c, "CREATE TABLE t (id text, v text)", &[])
            .unwrap();
        e.execute(&c, "INSERT INTO t (id, v) VALUES ('1', 'a')", &[])
            .unwrap();
        e.execute(
            &c,
            "INSERT INTO t (id, v) VALUES (?, ?)",
            &["2".into(), "b".into()],
        )
        .unwrap();
        e
    }

    /// @id miyusql_memory_test_delete_where_refuse
    /// @role test
    /// @layer tool
    /// @do verify_delete_where_refused
    //@human / @human DELETE … WHERE est refusé et ne vide pas la table (K-19).
    #[test]
    fn delete_where_refuse_sans_perte() {
        let e = exec_avec_lignes();
        let r = e.execute(&ctx(), "DELETE FROM t WHERE id = ?", &["1".into()]);
        assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))));
        let rows = e.execute(&ctx(), "SELECT * FROM t", &[]).unwrap();
        assert_eq!(rows.count, 2);
    }

    /// @id miyusql_memory_test_select_where_refuse
    /// @role test
    /// @layer tool
    /// @do verify_select_where_refused
    //@human / @human SELECT … WHERE est refusé au lieu de renvoyer toutes les lignes.
    #[test]
    fn select_where_refuse() {
        let e = exec_avec_lignes();
        let r = e.execute(&ctx(), "SELECT * FROM t WHERE id = '1'", &[]);
        assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))));
        let r = e.execute(&ctx(), "SELECT id FROM t", &[]);
        assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))));
        let r = e.execute(&ctx(), "SELECT * FROM t ORDER BY id", &[]);
        assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))));
    }

    /// @id miyusql_memory_test_instruction_inconnue
    /// @role test
    /// @layer tool
    /// @do verify_unknown_statement_refused
    //@human / @human Une instruction inconnue (UPDATE…) renvoie une erreur, pas Ok.
    #[test]
    fn instruction_inconnue_refusee() {
        let e = exec_avec_lignes();
        for sql in [
            "UPDATE t SET v = 'x'",
            "TRUNCATE t",
            "SELECT * FROM t; DROP TABLE t",
            "ALTER TABLE t DROP COLUMN v",
            "CREATE TABLE u (id text PRIMARY KEY)",
        ] {
            let r = e.execute(&ctx(), sql, &[]);
            assert!(
                matches!(r, Err(MiyuSQLError::NonImplemente(_))),
                "{sql} : {r:?}"
            );
        }
        assert!(e.prepare(&ctx(), "UPDATE t SET v = 'x'", &[]).is_err());
    }

    /// @id miyusql_memory_test_mandat_non_signe
    /// @role test
    /// @layer tool
    /// @do verify_unsigned_mandate_refused
    //@human / @human Un mandat non signé est refusé par toutes les opérations.
    #[test]
    fn mandat_non_signe_refuse() {
        let e = MemoryExecutor::new();
        let c = GovernedContext::new("n-importe-quoi".into(), 4);
        assert!(matches!(
            e.execute(&c, "CREATE TABLE t", &[]),
            Err(MiyuSQLError::MandatNonSigne)
        ));
        assert!(matches!(e.begin(&c), Err(MiyuSQLError::MandatNonSigne)));
        assert!(matches!(e.get(&c, "k"), Err(MiyuSQLError::MandatNonSigne)));
        assert!(matches!(
            e.read_tables(&c),
            Err(MiyuSQLError::MandatNonSigne)
        ));
    }

    /// @id miyusql_memory_test_tables_inexistantes
    /// @role test
    /// @layer tool
    /// @do verify_missing_table_errors
    //@human / @human Insertion, lecture ou suppression sur table inexistante : erreur.
    #[test]
    fn table_inexistante_refusee() {
        let e = MemoryExecutor::new();
        let c = ctx();
        assert!(e.execute(&c, "SELECT * FROM absent", &[]).is_err());
        assert!(e
            .execute(&c, "INSERT INTO absent (id) VALUES ('1')", &[])
            .is_err());
        assert!(e.execute(&c, "DELETE FROM absent", &[]).is_err());
        assert!(e.execute(&c, "DROP TABLE absent", &[]).is_err());
        assert!(e.read_table(&c, "absent").is_err());
    }

    /// @id miyusql_memory_test_insert_incoherent
    /// @role test
    /// @layer tool
    /// @do verify_inconsistent_insert_refused
    //@human / @human INSERT incohérent (colonnes/valeurs/paramètres, colonne inconnue) : erreur.
    #[test]
    fn insert_incoherent_refuse() {
        let e = exec_avec_lignes();
        let c = ctx();
        assert!(e
            .execute(&c, "INSERT INTO t (id) VALUES ('1', 'x')", &[])
            .is_err());
        assert!(e.execute(&c, "INSERT INTO t (id) VALUES (?)", &[]).is_err());
        assert!(e
            .execute(
                &c,
                "INSERT INTO t (id) VALUES (?)",
                &["1".into(), "2".into()]
            )
            .is_err());
        assert!(e
            .execute(&c, "INSERT INTO t (zz) VALUES ('1')", &[])
            .is_err());
        assert!(e.execute(&c, "SELECT * FROM t", &["1".into()]).is_err());
    }

    /// @id miyusql_memory_test_insert_valeurs
    /// @role test
    /// @layer tool
    /// @do verify_insert_literals_parsed
    //@human / @human Les littéraux (apostrophe échappée, virgule) sont lus correctement.
    #[test]
    fn insert_valeurs_litterales() {
        let e = exec_avec_lignes();
        let c = ctx();
        e.execute(&c, "INSERT INTO t (id, v) VALUES (3, 'l''a, b')", &[])
            .unwrap();
        let res = e.execute(&c, "SELECT * FROM t", &[]).unwrap();
        assert!(res
            .rows
            .iter()
            .any(|r| r.get("id").map(String::as_str) == Some("3")
                && r.get("v").map(String::as_str) == Some("l'a, b")));
    }

    /// @id miyusql_memory_test_transactions_inconnues
    /// @role test
    /// @layer tool
    /// @do verify_unknown_transaction_refused
    //@human / @human Commit/rollback d'une transaction inconnue : erreur, pas Ok.
    #[test]
    fn transaction_inconnue_refusee() {
        let e = MemoryExecutor::new();
        let c = ctx();
        assert!(e.commit(&c, "tx-absente").is_err());
        assert!(e.rollback(&c, "tx-absente").is_err());
        let tx = e.begin(&c).unwrap();
        assert!(e.begin(&c).is_err());
        e.commit(&c, &tx).unwrap();
    }

    /// @id miyusql_memory_test_rollback
    /// @role test
    /// @layer tool
    /// @do verify_rollback_restores
    //@human / @human Le rollback restaure tables et schéma.
    #[test]
    fn rollback_restaure() {
        let e = exec_avec_lignes();
        let c = ctx();
        let tx = e.begin(&c).unwrap();
        e.execute(&c, "DELETE FROM t", &[]).unwrap();
        e.execute(&c, "ALTER TABLE t ADD COLUMN w text", &[])
            .unwrap();
        e.rollback(&c, &tx).unwrap();
        assert_eq!(e.execute(&c, "SELECT * FROM t", &[]).unwrap().count, 2);
        assert_eq!(e.read_table(&c, "t").unwrap().columns.len(), 2);
    }

    /// @id miyusql_memory_test_cache_ttl
    /// @role test
    /// @layer tool
    /// @do verify_cache_ttl_refused
    //@human / @human Un TTL de cache (non géré) est refusé au lieu d'être ignoré.
    #[test]
    fn cache_ttl_refuse() {
        let e = MemoryExecutor::new();
        let c = ctx();
        assert!(matches!(
            e.set(&c, "k", b"v", Some(10)),
            Err(MiyuSQLError::NonImplemente(_))
        ));
        assert!(e.get(&c, "k").unwrap().is_none());
        assert!(e.invalidate(&c, "a*b").is_err());
    }
}
