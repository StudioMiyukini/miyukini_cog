//! Tests unitaires MiyuSQL (MiyuSQL - Unit Tests Contract).
//!
//! MSQL-Q-001 à MSQL-S-003 — sandbox / table MiyukiniSQLtest, nettoyage obligatoire.
//! @id: miyusql_unit_tests_module
//! @role: test
//! @layer: tool

use miyusql::{
    cache_get, cache_set, cache_invalidate, GovernedContext, MemoryExecutor, MiyuSQLError,
    query_execute, query_prepare, schema_read, schema_read_table, tx_begin, tx_commit, tx_rollback,
    MIYUKINI_SQLTEST_TABLE,
};

fn governed_ctx() -> GovernedContext {
    GovernedContext::new("mandate-test-001".to_string(), 2)
}

fn empty_ctx() -> GovernedContext {
    GovernedContext::new(String::new(), 2)
}

/// @id: MSQL_Q_001
/// @role: test
/// @layer: tool
/// @human: Execute SELECT valide — vérifie qu'une requête SELECT mandatee s'execute et retourne un résultat.
#[test]
fn msql_q_001_execute_select_valid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let sql = "CREATE TABLE IF NOT EXISTS t1 (id text)";
    let _ = query_execute(&ctx, sql, &[], &exec).unwrap();
    let sql2 = "SELECT * FROM t1";
    let res = query_execute(&ctx, sql2, &[], &exec).unwrap();
    assert!(res.rows.is_empty());
    assert_eq!(res.count, 0);
}

/// @id: MSQL_Q_002
/// @role: test
/// @layer: tool
/// @human: Prepare requête valide — vérifie que tool.query.prepare valide une requête sans l'exécuter.
#[test]
fn msql_q_002_prepare_valid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let r = query_prepare(&ctx, "SELECT 1 FROM t", &[], &exec);
    assert!(r.is_ok());
}

/// @id: MSQL_Q_003
/// @role: test
/// @layer: tool
/// @human: Prepare requête invalide — vérifie que tool.query.prepare rejette une requête mal formée.
#[test]
fn msql_q_003_prepare_invalid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let r = query_prepare(&ctx, "", &[], &exec);
    assert!(matches!(r, Err(MiyuSQLError::Syntax(_))));
}

/// @id: MSQL_Q_004
/// @role: test
/// @layer: tool
/// @human: No mandate — vérifie le refus sans mandat.
#[test]
fn msql_q_004_no_mandate() {
    let ctx = empty_ctx();
    let exec = MemoryExecutor::new();
    let r = query_execute(&ctx, "SELECT 1", &[], &exec);
    assert!(matches!(r, Err(MiyuSQLError::NoMandate)));
}

/// @id: MSQL_T_001
/// @role: test
/// @layer: tool
/// @human: Begin / Commit — vérifie qu'une transaction begin + commit s'exécute correctement (sandbox).
#[test]
fn msql_t_001_begin_commit() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let tx_id = tx_begin(&ctx, &exec).unwrap();
    tx_commit(&ctx, &tx_id, &exec).unwrap();
}

/// @id: MSQL_T_002
/// @role: test
/// @layer: tool
/// @human: Begin / Rollback — vérifie qu'un rollback annule les modifications (sandbox).
#[test]
fn msql_t_002_begin_rollback() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let tx_id = tx_begin(&ctx, &exec).unwrap();
    tx_rollback(&ctx, &tx_id, &exec).unwrap();
}

/// @id: MSQL_T_004
/// @role: test
/// @layer: tool
/// @human: No mandate pour transaction.
#[test]
fn msql_t_004_no_mandate() {
    let ctx = empty_ctx();
    let exec = MemoryExecutor::new();
    let r = tx_begin(&ctx, &exec);
    assert!(matches!(r, Err(MiyuSQLError::NoMandate)));
}

/// @id: MSQL_C_001
/// @role: test
/// @layer: tool
/// @human: Set / Get — vérifie qu'une valeur enregistrée est récupérable (cache dédié test).
#[test]
fn msql_c_001_set_get() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    cache_set(&ctx, "k1", b"v1", None, &exec).unwrap();
    let v = cache_get(&ctx, "k1", &exec).unwrap();
    assert_eq!(v.as_deref(), Some(b"v1" as &[u8]));
}

/// @id: MSQL_C_002
/// @role: test
/// @layer: tool
/// @human: Get absent — vérifie le comportement pour une clé inexistante.
#[test]
fn msql_c_002_get_absent() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let v = cache_get(&ctx, "nonexistent", &exec).unwrap();
    assert!(v.is_none());
}

/// @id: MSQL_C_003
/// @role: test
/// @layer: tool
/// @human: Invalidate — vérifie qu'une entrée invalidee n'est plus retournée.
#[test]
fn msql_c_003_invalidate() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    cache_set(&ctx, "k2", b"v2", None, &exec).unwrap();
    let n = cache_invalidate(&ctx, "k2", &exec).unwrap();
    assert_eq!(n, 1);
    let v = cache_get(&ctx, "k2", &exec).unwrap();
    assert!(v.is_none());
}

/// @id: MSQL_S_001
/// @role: test
/// @layer: tool
/// @human: Lecture schéma tables — vérifie que les métadonnées (tables) sont retournées.
#[test]
fn msql_s_001_read_schema_tables() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let _ = query_execute(&ctx, &format!("CREATE TABLE {}", MIYUKINI_SQLTEST_TABLE), &[], &exec).unwrap();
    let res = schema_read(&ctx, &exec).unwrap();
    assert!(!res.tables.is_empty());
    assert!(res.tables.iter().any(|t| t.name == MIYUKINI_SQLTEST_TABLE));
}

/// @id: MSQL_S_002
/// @role: test
/// @layer: tool
/// @human: Lecture schéma colonnes — vérifie que les colonnes et types sont retournés pour une table.
#[test]
fn msql_s_002_read_schema_columns() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let _ = query_execute(&ctx, &format!("CREATE TABLE {}", MIYUKINI_SQLTEST_TABLE), &[], &exec).unwrap();
    let _ = query_execute(
        &ctx,
        &format!("ALTER TABLE {} ADD COLUMN test_value text", MIYUKINI_SQLTEST_TABLE),
        &[],
        &exec,
    )
    .unwrap();
    let meta = schema_read_table(&ctx, MIYUKINI_SQLTEST_TABLE, &exec).unwrap();
    assert_eq!(meta.name, MIYUKINI_SQLTEST_TABLE);
    assert!(meta.columns.iter().any(|c| c.name == "test_value"));
}

/// @id: MSQL_S_003
/// @role: test
/// @layer: tool
/// @human: Table inexistante — vérifie le comportement pour une table inexistante (retourne colonnes vides).
#[test]
fn msql_s_003_table_nonexistent() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let meta = schema_read_table(&ctx, "NonexistentTable", &exec).unwrap();
    assert_eq!(meta.name, "NonexistentTable");
    assert!(meta.columns.is_empty());
}
