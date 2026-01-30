//! Suites de tests MiyukiniAdmin (Quick, Standard, Full) — Unit Tests Contract §8, Cycle Tests Contract.
//!
//! Branchement des suites unit + cycle et génération rapports (structure conforme §9.1, §6).

use crate::cycle_tests_runner::{run_miyukinisqltest, CycleTestVerdict, MiyukiniSQLtestResult};
use crate::db_unit_tests::{
    all_db_unit_test_definitions, coh_test_definitions, conf_test_definitions,
    run_suite as run_db_unit_suite, DbUnitTestBackend, DbUnitTestReport, UnitTestVerdict,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// @id: miyukiniadmin_testing_suites
/// @role: infrastructure
/// @layer: operator
/// @human: Suites de tests Quick/Standard/Full (unit + cycle).
/// @do: run_suites_and_report

/// Nom des suites (Unit Tests Contract §8.1).
/// Suite rapide : COH-001 et CONF-001 uniquement.
pub const SUITE_QUICK: &str = "quick";
/// Suite standard : tous COH + CONF-001 à CONF-003.
pub const SUITE_STANDARD: &str = "standard";
/// Suite complète : tous les tests unitaires DB + MiyukiniSQLtest.
pub const SUITE_FULL: &str = "full";

/// Rapport combiné : tests unitaires DB + test cycle MiyukiniSQLtest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingSuitesReport {
    /// Identifiant unique du rapport.
    pub report_id: String,
    /// Horodatage ISO 8601.
    pub timestamp: String,
    /// Nom de la suite (quick, standard, full).
    pub suite: String,
    /// Rapport des tests unitaires DB, si exécutés.
    pub unit_report: Option<DbUnitTestReport>,
    /// Résultat du test MiyukiniSQLtest (8 étapes), si exécuté.
    pub miyukinisqltest_result: Option<MiyukiniSQLtestResult>,
    /// Verdict global agrégé (PASS, WARN, FAIL, SKIP).
    pub overall_verdict: String,
}

/// Exécute la suite Quick (COH-001, CONF-001 — Unit Tests Contract §8.1).
pub fn run_suite_quick(
    backend: Option<&dyn DbUnitTestBackend>,
    miyukinisqltest_backend: Option<&dyn crate::cycle_tests_runner::MiyukiniSQLtestBackend>,
) -> TestingSuitesReport {
    let defs = all_db_unit_test_definitions();
    let quick_defs: Vec<_> = defs
        .into_iter()
        .filter(|d| d.id == "COH-001" || d.id == "CONF-001")
        .collect();
    run_suites_and_report(SUITE_QUICK, &quick_defs, backend, miyukinisqltest_backend)
}

/// Exécute la suite Standard (tous COH, CONF-001 à CONF-003 — Unit Tests Contract §8.1).
pub fn run_suite_standard(
    backend: Option<&dyn DbUnitTestBackend>,
    miyukinisqltest_backend: Option<&dyn crate::cycle_tests_runner::MiyukiniSQLtestBackend>,
) -> TestingSuitesReport {
    let mut defs = coh_test_definitions();
    defs.extend(conf_test_definitions().into_iter().take(3));
    run_suites_and_report(SUITE_STANDARD, &defs, backend, miyukinisqltest_backend)
}

/// Exécute la suite Full (tous les tests unitaires DB + MiyukiniSQLtest).
pub fn run_suite_full(
    backend: Option<&dyn DbUnitTestBackend>,
    miyukinisqltest_backend: Option<&dyn crate::cycle_tests_runner::MiyukiniSQLtestBackend>,
) -> TestingSuitesReport {
    let defs = all_db_unit_test_definitions();
    run_suites_and_report(SUITE_FULL, &defs, backend, miyukinisqltest_backend)
}

/// @id: miyukiniadmin_testing_suites_run_and_report
/// @role: mutator
/// @layer: operator
/// @human: Exécute les suites (unit + MiyukiniSQLtest) et produit un rapport combiné.
/// @do: run_suites_and_report
pub fn run_suites_and_report(
    suite_name: &str,
    unit_definitions: &[crate::db_unit_tests::DbUnitTestDefinition],
    backend: Option<&dyn DbUnitTestBackend>,
    miyukinisqltest_backend: Option<&dyn crate::cycle_tests_runner::MiyukiniSQLtestBackend>,
) -> TestingSuitesReport {
    let report_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let timeout = Duration::from_secs(120);

    let unit_report = run_db_unit_suite(suite_name, unit_definitions, backend, timeout);
    let miyukinisqltest_result = Some(run_miyukinisqltest(miyukinisqltest_backend));

    let overall = if unit_report.overall_verdict == UnitTestVerdict::Fail
        || miyukinisqltest_result.as_ref().map(|r| r.verdict) == Some(CycleTestVerdict::Fail)
    {
        "FAIL"
    } else if unit_report.overall_verdict == UnitTestVerdict::Warn
        || miyukinisqltest_result.as_ref().map(|r| r.verdict) == Some(CycleTestVerdict::Warn)
    {
        "WARN"
    } else if unit_report.overall_verdict == UnitTestVerdict::Pass
        && miyukinisqltest_result.as_ref().map(|r| r.verdict) == Some(CycleTestVerdict::Pass)
    {
        "PASS"
    } else {
        "SKIP"
    };

    TestingSuitesReport {
        report_id,
        timestamp,
        suite: suite_name.to_string(),
        unit_report: Some(unit_report),
        miyukinisqltest_result,
        overall_verdict: overall.to_string(),
    }
}
