//! Tests unitaires MiyuSQL (MiyuSQL - Unit Tests Contract).
//!
//! MSQL-Q-001 à MSQL-S-003 — sandbox / table MiyukiniSQLtest, nettoyage obligatoire.
//! @id     miyusql_unit_tests_module
//! @role   test
//! @layer  tool
//! @do documenter_module

use miyusql::{
    cache_get, cache_invalidate, cache_set, query_execute, query_prepare, schema_read,
    schema_read_table, tx_begin, tx_commit, tx_rollback, GovernedContext, MemoryExecutor,
    MiyuSQLError, SignatureMandat, MIYUKINI_SQLTEST_TABLE,
};

fn governed_ctx() -> GovernedContext {
    GovernedContext::new("mandate-test-001".to_string(), 2)
        .avec_signature(SignatureMandat::pour_tests())
}

/// Mandat non vide mais sans signature StrongFather (K-19).
fn unsigned_ctx() -> GovernedContext {
    GovernedContext::new("mandate-test-001".to_string(), 2)
}

fn empty_ctx() -> GovernedContext {
    GovernedContext::new(String::new(), 2)
}

/// @id MSQL_Q_001
/// @role test
/// @layer tool
/// @do msql_q_001_execute_select_valid
//@human / @human Execute SELECT valide — vérifie qu'une requête SELECT mandatee s'execute et retourne un résultat.
#[test]
fn msql_q_001_execute_select_valid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let sql = "CREATE TABLE IF NOT EXISTS t1 (id text)";
    let _ = query_execute(&ctx, sql, &[], &exec).unwrap();
    let sql2 = "SELECT * FROM t1";
    let res = query_execute(&ctx, sql2, &[], &exec).unwrap();
    assert_eq!(res.rows, [] as [std::collections::HashMap<std::string::String, std::string::String>; 0]);
    assert_eq!(res.count, 0);
}

/// @id MSQL_Q_002
/// @role test
/// @layer tool
/// @do msql_q_002_prepare_valid
//@human / @human Prepare requête valide — vérifie que tool.query.prepare valide une requête sans l'exécuter.
#[test]
fn msql_q_002_prepare_valid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let r = query_prepare(&ctx, "SELECT * FROM t", &[], &exec);
    assert!(r.is_ok());
}

/// @id MSQL_Q_002B
/// @role test
/// @layer tool
/// @do msql_q_002b_prepare_unhandled_clause
//@human / @human Prepare d'une clause non gérée (projection, WHERE, UPDATE) — refus explicite
//@human au lieu d'une validation de façade.
#[test]
fn msql_q_002b_prepare_unhandled_clause() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    for sql in [
        "SELECT 1 FROM t",
        "SELECT * FROM t WHERE id = 1",
        "UPDATE t SET a = 1",
    ] {
        let r = query_prepare(&ctx, sql, &[], &exec);
        assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))), "{sql}");
    }
}

/// @id MSQL_Q_003
/// @role test
/// @layer tool
/// @do msql_q_003_prepare_invalid
//@human / @human Prepare requête invalide — vérifie que tool.query.prepare rejette une requête mal formée.
#[test]
fn msql_q_003_prepare_invalid() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let r = query_prepare(&ctx, "", &[], &exec);
    assert!(matches!(r, Err(MiyuSQLError::Syntax(_))));
}

/// @id MSQL_Q_004
/// @role test
/// @layer tool
/// @do msql_q_004_no_mandate
//@human / @human No mandate — vérifie le refus sans mandat.
#[test]
fn msql_q_004_no_mandate() {
    let ctx = empty_ctx();
    let exec = MemoryExecutor::new();
    let r = query_execute(&ctx, "SELECT 1", &[], &exec);
    assert!(matches!(r, Err(MiyuSQLError::NoMandate)));
}

/// @id MSQL_Q_005
/// @role test
/// @layer tool
/// @do msql_q_005_unsigned_mandate
//@human / @human Mandat non signé par StrongFather — refus (fail-closed, K-19).
#[test]
fn msql_q_005_unsigned_mandate() {
    let ctx = unsigned_ctx();
    let exec = MemoryExecutor::new();
    let r = query_execute(&ctx, "CREATE TABLE t (id text)", &[], &exec);
    assert!(matches!(r, Err(MiyuSQLError::MandatNonSigne)));
    let r = tx_begin(&ctx, &exec);
    assert!(matches!(r, Err(MiyuSQLError::MandatNonSigne)));
    let r = cache_get(&ctx, "k", &exec);
    assert!(matches!(r, Err(MiyuSQLError::MandatNonSigne)));
    let r = schema_read(&ctx, &exec);
    assert!(matches!(r, Err(MiyuSQLError::MandatNonSigne)));
}

/// @id MSQL_Q_006
/// @role test
/// @layer tool
/// @do msql_q_006_delete_where_refused
//@human / @human DELETE … WHERE refusé : la table n'est pas vidée (K-19).
#[test]
fn msql_q_006_delete_where_refused() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    query_execute(&ctx, "CREATE TABLE t (id text)", &[], &exec).unwrap();
    query_execute(&ctx, "INSERT INTO t (id) VALUES ('1')", &[], &exec).unwrap();
    query_execute(&ctx, "INSERT INTO t (id) VALUES ('2')", &[], &exec).unwrap();
    let r = query_execute(&ctx, "DELETE FROM t WHERE id = ?", &["1".into()], &exec);
    assert!(matches!(r, Err(MiyuSQLError::NonImplemente(_))));
    let res = query_execute(&ctx, "SELECT * FROM t", &[], &exec).unwrap();
    assert_eq!(res.count, 2);
}

/// @id MSQL_T_001
/// @role test
/// @layer tool
/// @do msql_t_001_begin_commit
//@human / @human Begin / Commit — vérifie qu'une transaction begin + commit s'exécute correctement (sandbox).
#[test]
fn msql_t_001_begin_commit() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let tx_id = tx_begin(&ctx, &exec).unwrap();
    tx_commit(&ctx, &tx_id, &exec).unwrap();
}

/// @id MSQL_T_002
/// @role test
/// @layer tool
/// @do msql_t_002_begin_rollback
//@human / @human Begin / Rollback — vérifie qu'un rollback annule les modifications (sandbox).
#[test]
fn msql_t_002_begin_rollback() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let tx_id = tx_begin(&ctx, &exec).unwrap();
    tx_rollback(&ctx, &tx_id, &exec).unwrap();
}

/// @id MSQL_T_004
/// @role test
/// @layer tool
/// @do msql_t_004_no_mandate
//@human / @human No mandate pour transaction.
#[test]
fn msql_t_004_no_mandate() {
    let ctx = empty_ctx();
    let exec = MemoryExecutor::new();
    let r = tx_begin(&ctx, &exec);
    assert!(matches!(r, Err(MiyuSQLError::NoMandate)));
}

/// @id MSQL_C_001
/// @role test
/// @layer tool
/// @do msql_c_001_set_get
//@human / @human Set / Get — vérifie qu'une valeur enregistrée est récupérable (cache dédié test).
#[test]
fn msql_c_001_set_get() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    cache_set(&ctx, "k1", b"v1", None, &exec).unwrap();
    let v = cache_get(&ctx, "k1", &exec).unwrap();
    assert_eq!(v.as_deref(), Some(b"v1" as &[u8]));
}

/// @id MSQL_C_002
/// @role test
/// @layer tool
/// @do msql_c_002_get_absent
//@human / @human Get absent — vérifie le comportement pour une clé inexistante.
#[test]
fn msql_c_002_get_absent() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let v = cache_get(&ctx, "nonexistent", &exec).unwrap();
    assert!(v.is_none());
}

/// @id MSQL_C_003
/// @role test
/// @layer tool
/// @do msql_c_003_invalidate
//@human / @human Invalidate — vérifie qu'une entrée invalidee n'est plus retournée.
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

/// @id MSQL_S_001
/// @role test
/// @layer tool
/// @do msql_s_001_read_schema_tables
//@human / @human Lecture schéma tables — vérifie que les métadonnées (tables) sont retournées.
#[test]
fn msql_s_001_read_schema_tables() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let _ = query_execute(
        &ctx,
        &format!("CREATE TABLE {MIYUKINI_SQLTEST_TABLE}"),
        &[],
        &exec,
    )
    .unwrap();
    let res = schema_read(&ctx, &exec).unwrap();
    assert!(!res.tables.is_empty());
    assert!(res.tables.iter().any(|t| t.name == MIYUKINI_SQLTEST_TABLE));
}

/// @id MSQL_S_002
/// @role test
/// @layer tool
/// @do msql_s_002_read_schema_columns
//@human / @human Lecture schéma colonnes — vérifie que les colonnes et types sont retournés pour une table.
#[test]
fn msql_s_002_read_schema_columns() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let _ = query_execute(
        &ctx,
        &format!("CREATE TABLE {MIYUKINI_SQLTEST_TABLE}"),
        &[],
        &exec,
    )
    .unwrap();
    let _ = query_execute(
        &ctx,
        &format!(
            "ALTER TABLE {MIYUKINI_SQLTEST_TABLE} ADD COLUMN test_value text"
        ),
        &[],
        &exec,
    )
    .unwrap();
    let meta = schema_read_table(&ctx, MIYUKINI_SQLTEST_TABLE, &exec).unwrap();
    assert_eq!(meta.name, MIYUKINI_SQLTEST_TABLE);
    assert!(meta.columns.iter().any(|c| c.name == "test_value"));
}

/// @id MSQL_S_003
/// @role test
/// @layer tool
/// @do msql_s_003_table_nonexistent
//@human / @human Table inexistante — erreur explicite (et non une table vide fictive).
#[test]
fn msql_s_003_table_nonexistent() {
    let ctx = governed_ctx();
    let exec = MemoryExecutor::new();
    let r = schema_read_table(&ctx, "NonexistentTable", &exec);
    assert!(matches!(r, Err(MiyuSQLError::Execution(_))));
}
