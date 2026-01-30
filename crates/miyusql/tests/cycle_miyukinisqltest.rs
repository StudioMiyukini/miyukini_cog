//! Test de cycle MiyuSQL — MiyukiniSQLtest (MiyuSQL - Cycle Tests Contract).
//!
//! Scénario E2E 8 étapes : WriteIntent → validations Cores (simulées par mandat) →
//! création table MiyukiniSQLtest → colonne → insert → read → assertion → delete + teardown.
//! @id: miyusql_cycle_test_miyukinisqltest
//! @role: test
//! @layer: tool

use miyusql::{
    query_execute, GovernedContext, MemoryExecutor, MIYUKINI_SQLTEST_TABLE,
};

const TEST_VALUE_COL: &str = "test_value";

/// @id: miyusql_cycle_test_miyukinisqltest_run
/// @role: test
/// @layer: tool
/// @human: Exécute le scénario MiyukiniSQLtest complet (8 étapes).
#[test]
fn cycle_miyukinisqltest_full_path() {
    let ctx = GovernedContext::new("mandate-miyukinisqltest".to_string(), 2);
    let exec = MemoryExecutor::new();

    // Étape 1–2 : Contexte gouverné (mandat) — simulé par ctx
    assert!(ctx.has_mandate());

    // Étape 3 : Création table MiyukiniSQLtest
    let create_sql = format!("CREATE TABLE {} (id text)", MIYUKINI_SQLTEST_TABLE);
    let r = query_execute(&ctx, &create_sql, &[], &exec);
    assert!(r.is_ok(), "step 3 create table: {:?}", r);

    // Étape 4 : Création colonne test_value
    let alter_sql = format!(
        "ALTER TABLE {} ADD COLUMN {} text",
        MIYUKINI_SQLTEST_TABLE, TEST_VALUE_COL
    );
    let r = query_execute(&ctx, &alter_sql, &[], &exec);
    assert!(r.is_ok(), "step 4 add column: {:?}", r);

    // Étape 5 : Insertion donnée (valeur déterministe pour assertion)
    let insert_value = "cycle-test-value-001";
    let insert_sql = format!(
        "INSERT INTO {} (id, {}) VALUES ('1', '{}')",
        MIYUKINI_SQLTEST_TABLE, TEST_VALUE_COL, insert_value
    );
    let r = query_execute(&ctx, &insert_sql, &[], &exec);
    assert!(r.is_ok(), "step 5 insert: {:?}", r);

    // Étape 6 : Lecture
    let select_sql = format!("SELECT * FROM {}", MIYUKINI_SQLTEST_TABLE);
    let res = query_execute(&ctx, &select_sql, &[], &exec).expect("step 6 select");
    assert!(!res.rows.is_empty(), "step 6: expected rows");

    // Étape 7 : Affichage / assertion — valeur lue = valeur insérée
    let row = &res.rows[0];
    let read_value = row.get(TEST_VALUE_COL).map(String::as_str).unwrap_or("");
    assert_eq!(read_value, insert_value, "step 7: value mismatch");

    // Étape 8 : Suppression + teardown
    let delete_sql = format!("DELETE FROM {}", MIYUKINI_SQLTEST_TABLE);
    let r = query_execute(&ctx, &delete_sql, &[], &exec);
    assert!(r.is_ok(), "step 8 delete: {:?}", r);

    let drop_sql = format!("DROP TABLE {}", MIYUKINI_SQLTEST_TABLE);
    let _ = query_execute(&ctx, &drop_sql, &[], &exec);

    // Vérification nettoyage
    let res_after = query_execute(&ctx, &select_sql, &[], &exec).unwrap();
    assert!(res_after.rows.is_empty());
}
