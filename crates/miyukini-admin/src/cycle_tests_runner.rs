//! Runner des tests de cycle MiyukiniAdmin (Cycle Tests Contract).
//!
//! PERF/LAT/LOAD/RES + MiyukiniSQLtest : orchestration des 8 étapes (WriteIntent,
//! validations, create table, colonne, insert, read, assert, delete), appel au flux gouverné.

use serde::{Deserialize, Serialize};

/// @id: miyukiniadmin_cycle_runner
/// @role: infrastructure
/// @layer: operator
/// @human: Runner des tests de cycle (PERF, LAT, LOAD, RES).
/// @do: run_cycle_tests

/// @id: miyukiniadmin_miyukinisqltest_runner
/// @role: infrastructure
/// @layer: operator
/// @human: Runner du test MiyukiniSQLtest (chemin complet 8 étapes).
/// @do: run_miyukinisqltest

/// Verdict d'un test de cycle (Cycle Tests Contract §7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CycleTestVerdict {
    /// Tous les critères respectés.
    Pass,
    /// Critères respectés avec alertes.
    Warn,
    /// Un ou plusieurs critères non respectés.
    Fail,
    /// Test interrompu (seuils critiques).
    Abort,
}

/// Résultat du test MiyukiniSQLtest (8 étapes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyukiniSQLtestResult {
    /// Identifiant du test (ex. MiyukiniSQLtest).
    pub test_id: String,
    /// Horodatage ISO 8601 de l’exécution.
    pub timestamp: String,
    /// Verdict global (Pass, Warn, Fail, Abort).
    pub verdict: CycleTestVerdict,
    /// Résultats des 8 étapes (WriteIntent → delete).
    pub steps: Vec<MiyukiniSQLtestStepResult>,
    /// Indique si tous les critères sont respectés.
    pub criteria_met: bool,
    /// Durée totale en millisecondes.
    pub duration_ms: u64,
}

/// Résultat d'une étape du test MiyukiniSQLtest (étapes 1 à 8).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyukiniSQLtestStepResult {
    /// Numéro d’étape (1 à 8).
    pub step: u8,
    /// Nom de l’étape (ex. WriteIntent, create table).
    pub name: String,
    /// Statut (OK, SKIP, ERROR, etc.).
    pub status: String,
}

/// Backend pour exécuter le test MiyukiniSQLtest (WriteIntent → Cores → KindMother → MiyuSQL).
/// L'appelant (MiyukiniAdmin) fournit une implémentation qui passe par BondingBrother/KindMother.
pub trait MiyukiniSQLtestBackend: Send + Sync {
    /// Exécute les 8 étapes du test MiyukiniSQLtest et retourne le résultat.
    /// Étapes : WriteIntent, validations Cores, create table MiyukiniSQLtest, add column,
    /// insert, read, assert, delete + teardown.
    fn run_miyukinisqltest(&self) -> Result<MiyukiniSQLtestResult, String>;
}

/// Exécute le test MiyukiniSQLtest via le backend fourni.
/// Si aucun backend n'est fourni, retourne un résultat SKIP/Abort avec message.
#[must_use]
pub fn run_miyukinisqltest(backend: Option<&dyn MiyukiniSQLtestBackend>) -> MiyukiniSQLtestResult {
    let timestamp = chrono::Utc::now().to_rfc3339();
    match backend {
        None => MiyukiniSQLtestResult {
            test_id: "MiyukiniSQLtest".to_string(),
            timestamp: timestamp.clone(),
            verdict: CycleTestVerdict::Fail,
            steps: vec![MiyukiniSQLtestStepResult {
                step: 0,
                name: "Backend non configuré".to_string(),
                status: "SKIP".to_string(),
            }],
            criteria_met: false,
            duration_ms: 0,
        },
        Some(b) => match b.run_miyukinisqltest() {
            Ok(r) => r,
            Err(e) => MiyukiniSQLtestResult {
                test_id: "MiyukiniSQLtest".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                verdict: CycleTestVerdict::Fail,
                steps: vec![MiyukiniSQLtestStepResult {
                    step: 0,
                    name: "Erreur technique".to_string(),
                    status: format!("ERROR: {e}"),
                }],
                criteria_met: false,
                duration_ms: 0,
            },
        },
    }
}
