//! Tests unitaires DB MiyukiniAdmin (MiyukiniAdmin - Unit Tests Contract).
//!
//! COH-001 à COH-007, CONF-001 à CONF-005, STRUCT-001 à STRUCT-005, SEC-001 à SEC-006.
//! Requêtes en lecture seule via KindMother ; évaluation critères ; rapport PASS/WARN/FAIL.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// @id: miyukiniadmin_db_unit_tests_module
/// @role: infrastructure
/// @layer: operator
/// @human: Module des tests unitaires DB (cohérence, conformité, structure, sécurité).
/// @do: expose_db_unit_tests

/// @id: COH_001
/// @role: test
/// @layer: operator
/// Verdict possible pour un test unitaire (Unit Tests Contract §7.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UnitTestVerdict {
    /// Test réussi, aucune violation.
    Pass,
    /// Test réussi avec alertes mineures.
    Warn,
    /// Test échoué, violations détectées.
    Fail,
    /// Test ignoré (pré-condition non remplie).
    Skip,
    /// Erreur technique pendant le test.
    Error,
}

/// @id: COH_002
/// @role: test
/// @layer: operator
/// Sévérité d'un test (Unit Tests Contract §7.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TestSeverity {
    /// Sévérité critique : échec bloquant (ex. conformité schéma, sécurité).
    Critical,
    /// Sévérité haute : violation importante (ex. intégrité référentielle).
    High,
    /// Sévérité moyenne : anomalie à traiter (ex. cohérence inter-tables).
    Medium,
    /// Sévérité basse : point d’attention (ex. valeurs aberrantes).
    Low,
    /// Sévérité informative : observation sans impact direct (ex. tables vides).
    Info,
}

/// @id: COH_003
/// @role: test
/// @layer: operator
/// Définition d'un test unitaire DB (id, nom, catégorie, critères, sévérité).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUnitTestDefinition {
    /// Identifiant unique du test (ex. COH-001, CONF-001).
    pub id: String,
    /// Nom lisible du test.
    pub name: String,
    /// Description fonctionnelle du test.
    pub description: String,
    /// Catégorie (coherence, conformite, structure, security).
    pub category: String,
    /// Type de requête (foreign_key_check, schema_check, etc.).
    pub query_type: String,
    /// Critères attendus : clé = métrique, valeur = seuil max (0 = aucune violation).
    pub criteria: HashMap<String, u64>,
    /// Sévérité du test en cas d’échec.
    pub severity: TestSeverity,
}

/// @id: COH_004
/// @role: test
/// @layer: operator
/// Résultat d'un test unitaire individuel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUnitTestResult {
    /// Identifiant du test exécuté.
    pub id: String,
    /// Nom du test.
    pub name: String,
    /// Verdict (Pass, Warn, Fail, Skip, Error).
    pub verdict: UnitTestVerdict,
    /// Sévérité du test.
    pub severity: TestSeverity,
    /// Durée d’exécution en millisecondes.
    pub duration_ms: u64,
    /// Métriques et détails (violations_count, erreur, etc.).
    pub details: HashMap<String, serde_json::Value>,
    /// Recommandation optionnelle en cas d’échec ou avertissement.
    pub recommendation: Option<String>,
}

/// @id: COH_005
/// @role: test
/// @layer: operator
/// Résumé d'une suite de tests (Unit Tests Contract §9.1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUnitTestSummary {
    /// Nombre total de tests exécutés.
    pub total_tests: u32,
    /// Nombre de tests réussis (Pass).
    pub passed: u32,
    /// Nombre de tests avec avertissement (Warn).
    pub warnings: u32,
    /// Nombre de tests en échec (Fail).
    pub failed: u32,
    /// Nombre de tests ignorés (Skip).
    pub skipped: u32,
    /// Nombre d’erreurs techniques (Error).
    pub errors: u32,
}

/// @id: COH_006
/// @role: test
/// @layer: operator
/// Rapport complet d'exécution des tests unitaires DB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUnitTestReport {
    /// Identifiant unique du rapport.
    pub report_id: String,
    /// Nom de la suite exécutée (quick, standard, full).
    pub suite: String,
    /// Horodatage ISO 8601 de l’exécution.
    pub timestamp: String,
    /// Durée totale de la suite en secondes.
    pub duration_seconds: u64,
    /// Agrégat des compteurs (passed, failed, etc.).
    pub summary: DbUnitTestSummary,
    /// Verdict global de la suite.
    pub overall_verdict: UnitTestVerdict,
    /// Résultats individuels de chaque test.
    pub tests: Vec<DbUnitTestResult>,
    /// Recommandations issues des échecs ou avertissements.
    pub recommendations: Vec<Recommendation>,
}

/// @id: CONF_001
/// @role: test
/// @layer: operator
/// Recommandation associée à un échec ou avertissement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Sévérité du point soulevé.
    pub severity: TestSeverity,
    /// Description du problème détecté.
    pub issue: String,
    /// Action recommandée pour corriger.
    pub action: String,
}

/// Définitions des tests COH (cohérence).
#[must_use]
pub fn coh_test_definitions() -> Vec<DbUnitTestDefinition> {
    vec![
        DbUnitTestDefinition {
            id: "COH-001".to_string(),
            name: "Intégrité Référentielle".to_string(),
            description:
                "Vérifie que toutes les clés étrangères pointent vers des enregistrements existants"
                    .to_string(),
            category: "coherence".to_string(),
            query_type: "foreign_key_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "COH-002".to_string(),
            name: "Contraintes NOT NULL".to_string(),
            description: "Vérification des champs obligatoires".to_string(),
            category: "coherence".to_string(),
            query_type: "not_null_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "COH-003".to_string(),
            name: "Unicité".to_string(),
            description: "Vérification des contraintes UNIQUE".to_string(),
            category: "coherence".to_string(),
            query_type: "unique_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "COH-004".to_string(),
            name: "Cohérence inter-tables".to_string(),
            description: "Vérification des relations logiques".to_string(),
            category: "coherence".to_string(),
            query_type: "cross_table_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Medium,
        },
        DbUnitTestDefinition {
            id: "COH-005".to_string(),
            name: "Détection Orphelins".to_string(),
            description: "Détecte les enregistrements orphelins (sans parent valide)".to_string(),
            category: "coherence".to_string(),
            query_type: "orphans_check".to_string(),
            criteria: [("orphans_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Medium,
        },
        DbUnitTestDefinition {
            id: "COH-006".to_string(),
            name: "Doublons".to_string(),
            description: "Détection des doublons suspects".to_string(),
            category: "coherence".to_string(),
            query_type: "duplicates_check".to_string(),
            criteria: [("duplicates_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Medium,
        },
        DbUnitTestDefinition {
            id: "COH-007".to_string(),
            name: "Valeurs aberrantes".to_string(),
            description: "Détection des valeurs hors bornes".to_string(),
            category: "coherence".to_string(),
            query_type: "outliers_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Low,
        },
    ]
}

/// Définitions des tests CONF (conformité).
#[must_use]
pub fn conf_test_definitions() -> Vec<DbUnitTestDefinition> {
    vec![
        DbUnitTestDefinition {
            id: "CONF-001".to_string(),
            name: "Conformité Schéma KindMother".to_string(),
            description: "Vérifie que le schéma DB correspond aux contrats KindMother".to_string(),
            category: "conformite".to_string(),
            query_type: "schema_check".to_string(),
            criteria: [
                ("missing_tables".to_string(), 0),
                ("missing_columns".to_string(), 0),
                ("type_mismatches".to_string(), 0),
            ]
            .into_iter()
            .collect(),
            severity: TestSeverity::Critical,
        },
        DbUnitTestDefinition {
            id: "CONF-002".to_string(),
            name: "Types de données".to_string(),
            description: "Vérification des types".to_string(),
            category: "conformite".to_string(),
            query_type: "type_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "CONF-003".to_string(),
            name: "Formats".to_string(),
            description: "Vérification des formats (dates, UUID, etc.)".to_string(),
            category: "conformite".to_string(),
            query_type: "format_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Medium,
        },
        DbUnitTestDefinition {
            id: "CONF-004".to_string(),
            name: "Invariants métier".to_string(),
            description: "Vérification des règles invariantes".to_string(),
            category: "conformite".to_string(),
            query_type: "invariant_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "CONF-005".to_string(),
            name: "Conventions nommage".to_string(),
            description: "Vérification des conventions".to_string(),
            category: "conformite".to_string(),
            query_type: "naming_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Low,
        },
    ]
}

/// Définitions des tests STRUCT (structure).
#[must_use]
pub fn struct_test_definitions() -> Vec<DbUnitTestDefinition> {
    vec![
        DbUnitTestDefinition {
            id: "STRUCT-001".to_string(),
            name: "Index Requis".to_string(),
            description: "Vérifie que tous les index requis existent".to_string(),
            category: "structure".to_string(),
            query_type: "index_check".to_string(),
            criteria: [("missing_indexes".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "STRUCT-002".to_string(),
            name: "Index non utilisés".to_string(),
            description: "Détection index inutiles".to_string(),
            category: "structure".to_string(),
            query_type: "unused_index_check".to_string(),
            criteria: [("unused_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Low,
        },
        DbUnitTestDefinition {
            id: "STRUCT-003".to_string(),
            name: "Tables vides".to_string(),
            description: "Détection tables sans données".to_string(),
            category: "structure".to_string(),
            query_type: "empty_tables_check".to_string(),
            criteria: HashMap::new(),
            severity: TestSeverity::Info,
        },
        DbUnitTestDefinition {
            id: "STRUCT-004".to_string(),
            name: "Fragmentation".to_string(),
            description: "Niveau de fragmentation".to_string(),
            category: "structure".to_string(),
            query_type: "fragmentation_check".to_string(),
            criteria: HashMap::new(),
            severity: TestSeverity::Medium,
        },
        DbUnitTestDefinition {
            id: "STRUCT-005".to_string(),
            name: "Statistiques".to_string(),
            description: "Fraîcheur des statistiques".to_string(),
            category: "structure".to_string(),
            query_type: "stats_check".to_string(),
            criteria: HashMap::new(),
            severity: TestSeverity::Low,
        },
    ]
}

/// Définitions des tests SEC (sécurité).
#[must_use]
pub fn sec_test_definitions() -> Vec<DbUnitTestDefinition> {
    vec![
        DbUnitTestDefinition {
            id: "SEC-001".to_string(),
            name: "Permissions non escaladées".to_string(),
            description: "Vérifie qu'aucun utilisateur n'a des permissions supérieures à son rôle"
                .to_string(),
            category: "security".to_string(),
            query_type: "permission_check".to_string(),
            criteria: [("escalated_permissions_count".to_string(), 0)]
                .into_iter()
                .collect(),
            severity: TestSeverity::Critical,
        },
        DbUnitTestDefinition {
            id: "SEC-002".to_string(),
            name: "Tokens expirés".to_string(),
            description: "Détecte les tokens/sessions non expirés qui devraient l'être".to_string(),
            category: "security".to_string(),
            query_type: "token_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "SEC-003".to_string(),
            name: "Audit trail integrity".to_string(),
            description: "Vérifie l'intégrité des logs d'audit".to_string(),
            category: "security".to_string(),
            query_type: "audit_check".to_string(),
            criteria: [
                ("gaps_detected".to_string(), 0),
                ("sequence_breaks".to_string(), 0),
                ("hash_mismatches".to_string(), 0),
            ]
            .into_iter()
            .collect(),
            severity: TestSeverity::Critical,
        },
        DbUnitTestDefinition {
            id: "SEC-004".to_string(),
            name: "Encryption at rest".to_string(),
            description: "Vérifie que les données sensibles sont chiffrées".to_string(),
            category: "security".to_string(),
            query_type: "encryption_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::High,
        },
        DbUnitTestDefinition {
            id: "SEC-005".to_string(),
            name: "Foreign key constraints".to_string(),
            description: "Vérifie que les FK sont actives (pas de bypass)".to_string(),
            category: "security".to_string(),
            query_type: "fk_active_check".to_string(),
            criteria: [("violations_count".to_string(), 0)].into_iter().collect(),
            severity: TestSeverity::Critical,
        },
        DbUnitTestDefinition {
            id: "SEC-006".to_string(),
            name: "No orphan permissions".to_string(),
            description: "Détecte les permissions orphelines".to_string(),
            category: "security".to_string(),
            query_type: "orphan_permissions_check".to_string(),
            criteria: [("orphan_permissions_count".to_string(), 0)]
                .into_iter()
                .collect(),
            severity: TestSeverity::Medium,
        },
    ]
}

/// Toutes les définitions de tests unitaires DB (COH, CONF, STRUCT, SEC).
#[must_use]
pub fn all_db_unit_test_definitions() -> Vec<DbUnitTestDefinition> {
    let mut all = coh_test_definitions();
    all.extend(conf_test_definitions());
    all.extend(struct_test_definitions());
    all.extend(sec_test_definitions());
    all
}

/// Backend optionnel pour exécuter les requêtes de vérification (via KindMother).
/// Si absent, les tests retournent SKIP.
pub trait DbUnitTestBackend: Send + Sync {
    /// Exécute une requête de vérification en lecture seule et retourne les métriques (violations_count, etc.).
    fn run_verification(
        &self,
        test_id: &str,
        _query_type: &str,
    ) -> Result<HashMap<String, u64>, String>;
}

/// @id: miyukiniadmin_db_unit_tests_run_single
/// @role: mutator
/// @layer: operator
/// Exécute un seul test unitaire DB et retourne le résultat.
#[must_use]
pub fn run_single_test(
    def: &DbUnitTestDefinition,
    backend: Option<&dyn DbUnitTestBackend>,
    timeout: Duration,
) -> DbUnitTestResult {
    let start = std::time::Instant::now();
    let mut details = HashMap::new();

    let verdict = match backend {
        None => {
            details.insert(
                "reason".to_string(),
                serde_json::json!("no backend configured"),
            );
            UnitTestVerdict::Skip
        }
        Some(b) => match b.run_verification(&def.id, &def.query_type) {
            Ok(metrics) => {
                for (k, v) in &metrics {
                    details.insert(k.clone(), serde_json::json!(v));
                }
                let mut failed = false;
                for (criterion, expected) in &def.criteria {
                    let actual = metrics.get(criterion).copied().unwrap_or(0);
                    if actual > *expected {
                        failed = true;
                        break;
                    }
                }
                if failed {
                    UnitTestVerdict::Fail
                } else if details.values().any(|v| v.as_u64().unwrap_or(0) > 0) {
                    UnitTestVerdict::Warn
                } else {
                    UnitTestVerdict::Pass
                }
            }
            Err(e) => {
                details.insert("error".to_string(), serde_json::json!(e));
                UnitTestVerdict::Error
            }
        },
    };

    let duration_ms = start.elapsed().min(timeout).as_millis() as u64;
    DbUnitTestResult {
        id: def.id.clone(),
        name: def.name.clone(),
        verdict,
        severity: def.severity,
        duration_ms,
        details,
        recommendation: None,
    }
}

/// @id: miyukiniadmin_db_unit_tests_run_suite
/// @role: mutator
/// @layer: operator
/// Exécute une suite de tests (Quick, Standard, Full) et produit un rapport.
#[must_use]
pub fn run_suite(
    suite_name: &str,
    definitions: &[DbUnitTestDefinition],
    backend: Option<&dyn DbUnitTestBackend>,
    timeout_per_test: Duration,
) -> DbUnitTestReport {
    let report_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let start = std::time::Instant::now();

    let mut results = Vec::with_capacity(definitions.len());
    let mut passed = 0u32;
    let mut warnings = 0u32;
    let mut failed = 0u32;
    let mut skipped = 0u32;
    let mut errors = 0u32;

    for def in definitions {
        let result = run_single_test(def, backend, timeout_per_test);
        match result.verdict {
            UnitTestVerdict::Pass => passed += 1,
            UnitTestVerdict::Warn => warnings += 1,
            UnitTestVerdict::Fail => failed += 1,
            UnitTestVerdict::Skip => skipped += 1,
            UnitTestVerdict::Error => errors += 1,
        }
        results.push(result);
    }

    let overall_verdict = if failed > 0 || errors > 0 {
        UnitTestVerdict::Fail
    } else if warnings > 0 {
        UnitTestVerdict::Warn
    } else if skipped == results.len() as u32 {
        UnitTestVerdict::Skip
    } else {
        UnitTestVerdict::Pass
    };

    let recommendations: Vec<Recommendation> = results
        .iter()
        .filter(|r| r.verdict == UnitTestVerdict::Fail || r.verdict == UnitTestVerdict::Warn)
        .filter_map(|r| {
            r.recommendation.as_ref().map(|rec| Recommendation {
                severity: r.severity,
                issue: format!("{}: {}", r.id, r.name),
                action: rec.clone(),
            })
        })
        .collect();

    DbUnitTestReport {
        report_id,
        suite: suite_name.to_string(),
        timestamp,
        duration_seconds: start.elapsed().as_secs(),
        summary: DbUnitTestSummary {
            total_tests: results.len() as u32,
            passed,
            warnings,
            failed,
            skipped,
            errors,
        },
        overall_verdict,
        tests: results,
        recommendations,
    }
}
