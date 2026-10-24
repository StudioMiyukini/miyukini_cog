//! Test de cycle MiyuSQL — MiyukiniSQLtest (MiyuSQL - Cycle Tests Contract).
//!
//! Scénario E2E 8 étapes : WriteIntent → validations Cores (simulées par mandat) →
//! création table MiyukiniSQLtest → colonne → insert → read → assertion → delete + teardown.
//! @id     miyusql_cycle_test_miyukinisqltest
//! @role   test
//! @layer  tool
//! @do documenter_miyukinisqltest

use miyusql::{
    query_execute, GovernedContext, MemoryExecutor, SignatureMandat, MIYUKINI_SQLTEST_TABLE,
};

const TEST_VALUE_COL: &str = "test_value";

/// @id miyusql_cycle_test_miyukinisqltest_run
/// @role test
/// @layer tool
/// @do cycle_miyukinisqltest_full_path
//@human / @human Exécute le scénario MiyukiniSQLtest complet (8 étapes).
#[test]
fn cycle_miyukinisqltest_full_path() {
    // Signature de test : en production, le mandat doit être signé par StrongFather.
    let ctx = GovernedContext::new("mandate-miyukinisqltest".to_string(), 2)
        .avec_signature(SignatureMandat::pour_tests());
    let exec = MemoryExecutor::new();

    // Étape 1–2 : Contexte gouverné (mandat) — simulé par ctx
    assert!(ctx.has_mandate());

    // Étape 3 : Création table MiyukiniSQLtest
    let create_sql = format!("CREATE TABLE {MIYUKINI_SQLTEST_TABLE} (id text)");
    let r = query_execute(&ctx, &create_sql, &[], &exec);
    assert!(r.is_ok(), "step 3 create table: {r:?}");

    // Étape 4 : Création colonne test_value
    let alter_sql = format!(
        "ALTER TABLE {MIYUKINI_SQLTEST_TABLE} ADD COLUMN {TEST_VALUE_COL} text"
    );
    let r = query_execute(&ctx, &alter_sql, &[], &exec);
    assert!(r.is_ok(), "step 4 add column: {r:?}");

    // Étape 5 : Insertion donnée (valeur déterministe pour assertion)
    let insert_value = "cycle-test-value-001";
    let insert_sql = format!(
        "INSERT INTO {MIYUKINI_SQLTEST_TABLE} (id, {TEST_VALUE_COL}) VALUES ('1', '{insert_value}')"
    );
    let r = query_execute(&ctx, &insert_sql, &[], &exec);
    assert!(r.is_ok(), "step 5 insert: {r:?}");

    // Étape 6 : Lecture
    let select_sql = format!("SELECT * FROM {MIYUKINI_SQLTEST_TABLE}");
    let res = query_execute(&ctx, &select_sql, &[], &exec).expect("step 6 select");
    assert!(!res.rows.is_empty(), "step 6: expected rows");

    // Étape 7 : Affichage / assertion — valeur lue = valeur insérée
    let row = &res.rows[0];
    let read_value = row.get(TEST_VALUE_COL).map_or("", String::as_str);
    assert_eq!(read_value, insert_value, "step 7: value mismatch");

    // Étape 8 : Suppression + teardown
    let delete_sql = format!("DELETE FROM {MIYUKINI_SQLTEST_TABLE}");
    let r = query_execute(&ctx, &delete_sql, &[], &exec);
    assert!(r.is_ok(), "step 8 delete: {r:?}");

    let drop_sql = format!("DROP TABLE {MIYUKINI_SQLTEST_TABLE}");
    let r = query_execute(&ctx, &drop_sql, &[], &exec);
    assert!(r.is_ok(), "teardown drop: {r:?}");

    // Vérification nettoyage : la table n'existe plus, la lecture est refusée.
    let res_after = query_execute(&ctx, &select_sql, &[], &exec);
    assert!(res_after.is_err());
}
