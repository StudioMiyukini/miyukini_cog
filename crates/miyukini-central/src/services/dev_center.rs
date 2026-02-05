//! Service Miyukini Dev Center — centralisation des tests pour le développement.
//!
//! Regroupe les tests unitaires et d'intégration de tous les crates du workspace,
//! présentés par service/crate avec une UI par page. Lance les tests et affiche
//! les résultats dans une console dev.

use crate::catalog::ServiceId;
use crate::pixel_theme::PixelChromeTheme;
use crate::services::ServiceUi;
use eframe::egui;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Type de test (unitaire ou intégration).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestKind {
    Unit,
    Integration,
}

impl TestKind {
    pub fn label(&self) -> &'static str {
        match self {
            TestKind::Unit => "Unitaire",
            TestKind::Integration => "Intégration",
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            TestKind::Unit => "◇",
            TestKind::Integration => "▣",
        }
    }
}

/// Groupe de tests (fichier ou module).
#[derive(Clone)]
pub struct TestGroup {
    pub name: &'static str,
    pub kind: TestKind,
    pub tests: Vec<&'static str>,
}

/// Suite de tests d'un crate.
#[derive(Clone)]
pub struct TestSuite {
    pub crate_name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub groups: Vec<TestGroup>,
}

impl TestSuite {
    pub fn total_tests(&self) -> usize {
        self.groups.iter().map(|g| g.tests.len()).sum()
    }
}

/// Registre statique des suites de tests (aligné sur les crates du workspace).
fn all_test_suites() -> Vec<TestSuite> {
    vec![
        TestSuite {
            crate_name: "miyukini-kernel",
            display_name: "Miyukini Kernel",
            description: "Kernel : config, id, time, log, lifecycle.",
            groups: vec![
                TestGroup {
                    name: "src/config.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_config_get_existing_key",
                        "test_config_get_missing_key",
                        "test_config_get_empty_key",
                        "test_config_get_case_sensitive",
                        "test_config_envconfig_from_env",
                        "test_config_no_business_logic",
                        "test_config_no_external_dependency",
                        "test_config_deterministic",
                        "test_config_multiple_keys",
                        "test_config_empty_value",
                    ],
                },
                TestGroup {
                    name: "src/id.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_id_generate_unique",
                        "test_id_display_roundtrip",
                        "test_id_parse_valid_uuid",
                        "test_id_parse_invalid_format",
                        "test_id_equality",
                        "test_id_hash",
                        "test_id_no_business_logic",
                        "test_id_no_external_dependency",
                        "test_id_deterministic_parse",
                    ],
                },
                TestGroup {
                    name: "src/time.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_time_now_returns_systemtime",
                        "test_time_now_monotonic",
                        "test_time_fake_clock_injection",
                        "test_time_no_timezone_logic",
                        "test_time_no_business_logic",
                        "test_time_no_external_dependency",
                        "test_time_deterministic_fake",
                    ],
                },
                TestGroup {
                    name: "src/log.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_log_all_levels",
                        "test_log_message_preserved",
                        "test_log_level_enum",
                        "test_log_default_logger",
                        "test_log_no_format_imposed",
                        "test_log_no_business_logic",
                        "test_log_no_external_dependency",
                        "test_log_deterministic",
                    ],
                },
                TestGroup {
                    name: "src/lifecycle.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_lifecycle_register_hook",
                        "test_lifecycle_shutdown_executes_hooks",
                        "test_lifecycle_shutdown_lifo_order",
                        "test_lifecycle_multiple_hooks",
                        "test_lifecycle_shutdown_idempotent",
                        "test_lifecycle_no_init_hooks",
                        "test_lifecycle_no_business_logic",
                        "test_lifecycle_deterministic",
                    ],
                },
                TestGroup {
                    name: "tests/integration_tests.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "test_integration_kernel_modules_work_together",
                        "test_integration_config_id_integration",
                        "test_integration_time_log_integration",
                        "test_integration_lifecycle_log_integration",
                        "test_integration_id_lifecycle_integration",
                        "test_integration_invariant_k1_no_business_logic",
                        "test_integration_invariant_k2_no_external_dependency",
                        "test_integration_invariant_k6_determinism",
                        "test_integration_invariant_k7_explicability",
                        "test_integration_invariant_k8_offline",
                        "test_integration_invariant_k10_governance",
                        "test_integration_round_trip_id_config",
                        "test_integration_lifecycle_shutdown_with_all_modules",
                    ],
                },
            ],
        },
        TestSuite {
            crate_name: "miyusql",
            display_name: "MiyuSQL",
            description: "Outil SQL gouverné : requêtes, transactions, cache, schéma.",
            groups: vec![
                TestGroup {
                    name: "tests/unit_tests.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "msql_q_001_execute_select_valid",
                        "msql_q_002_prepare_valid",
                        "msql_q_003_prepare_invalid",
                        "msql_q_004_no_mandate",
                        "msql_t_001_begin_commit",
                        "msql_t_002_begin_rollback",
                        "msql_t_004_no_mandate",
                        "msql_c_001_set_get",
                        "msql_c_002_get_absent",
                        "msql_c_003_invalidate",
                        "msql_s_001_read_schema_tables",
                        "msql_s_002_read_schema_columns",
                        "msql_s_003_table_nonexistent",
                    ],
                },
                TestGroup {
                    name: "tests/cycle_miyukinisqltest.rs",
                    kind: TestKind::Integration,
                    tests: vec!["cycle_miyukinisqltest_full_path"],
                },
            ],
        },
        TestSuite {
            crate_name: "strongfather",
            display_name: "StrongFather",
            description: "Core de décision : intent, policy, validator, decision.",
            groups: vec![
                TestGroup {
                    name: "src/intent.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_create_basic_intent",
                        "test_create_intent_with_priority",
                        "test_create_intent_with_constraints",
                        "test_all_action_types",
                    ],
                },
                TestGroup {
                    name: "src/policy.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_create_basic_policy",
                        "test_create_policy_with_priority",
                        "test_policy_set",
                        "test_all_policy_types",
                    ],
                },
                TestGroup {
                    name: "src/validator.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_validate_valid_intent",
                        "test_validate_missing_intent_id",
                        "test_validate_missing_subject",
                        "test_validate_incomplete_context",
                    ],
                },
                TestGroup {
                    name: "src/decision.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_create_accepted_decision",
                        "test_create_refused_decision",
                        "test_create_ambiguous_decision",
                        "test_create_deferred_decision",
                        "test_justification_with_reasoning",
                    ],
                },
                TestGroup {
                    name: "src/priority.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_create_priority",
                        "test_default_priority",
                        "test_calculate_with_requested_priority",
                        "test_calculate_with_priority_policies",
                        "test_compare_priorities",
                    ],
                },
                TestGroup {
                    name: "src/policy_engine.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_create_policy_engine",
                        "test_apply_permission_policy",
                        "test_policy_priority_ordering",
                    ],
                },
            ],
        },
        TestSuite {
            crate_name: "kindmother",
            display_name: "KindMother",
            description: "Core de données : state, storage, sync, api, observability.",
            groups: vec![
                TestGroup {
                    name: "src/state.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "test_mother_instance_creation",
                        "test_daughter_instance_creation",
                        "test_unique_ids",
                    ],
                },
                TestGroup {
                    name: "src/storage.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_read_write", "test_delete"],
                },
                TestGroup {
                    name: "src/sync.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_delta_creation"],
                },
                TestGroup {
                    name: "src/api.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_write_intent_creation"],
                },
                TestGroup {
                    name: "src/observability.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_default_health", "test_default_metrics"],
                },
                TestGroup {
                    name: "src/threat.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_detector_default"],
                },
            ],
        },
        TestSuite {
            crate_name: "miyukini-central",
            display_name: "Miyukini Central",
            description: "Hub Central : config, suivi d'utilisation.",
            groups: vec![
                TestGroup {
                    name: "src/config.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "supabase_config_default_is_not_available",
                        "supabase_config_with_ids_is_available",
                    ],
                },
                TestGroup {
                    name: "src/usage_tracking.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_record_and_stats"],
                },
            ],
        },
        TestSuite {
            crate_name: "miyukini-admin",
            display_name: "Miyukini Admin",
            description: "Console d'administration : API, sécurité, base, tests, backup.",
            groups: vec![
                TestGroup {
                    name: "src/database_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_instance_info"],
                },
                TestGroup {
                    name: "src/audit_logger.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_default_audit_logger"],
                },
                TestGroup {
                    name: "src/api_handlers.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_api_response_creation"],
                },
                TestGroup {
                    name: "src/bonding_brother_bridge.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_bridge_error"],
                },
                TestGroup {
                    name: "src/security_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_security_service_structure"],
                },
                TestGroup {
                    name: "src/migration_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_migration_list", "test_apply_result"],
                },
                TestGroup {
                    name: "src/monitoring_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_system_metrics"],
                },
                TestGroup {
                    name: "src/testing_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_results_creation"],
                },
                TestGroup {
                    name: "src/backup_service.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_backup_list", "test_restore"],
                },
                TestGroup {
                    name: "src/config.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_from_env"],
                },
                TestGroup {
                    name: "src/api_routes.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_router_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "jayfestival",
            display_name: "JayFestival",
            description: "Éditions, exposants, événements. Tests unitaires, globaux et parcours UNC/ORG/EXP/VIS.",
            groups: vec![
                TestGroup {
                    name: "src/app_state.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "app_state_default_landing",
                        "app_state_navigate_apply",
                        "app_state_set_current_screen",
                        "app_state_nav_request_empty_after_apply",
                        "app_state_multiple_navigate_last_wins",
                        "app_state_set_user_then_user",
                        "app_state_navigate_to_all_unc_screens",
                    ],
                },
                TestGroup {
                    name: "src/auth/permissions.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "auth_user_type_from_profile_known",
                        "auth_user_type_from_profile_unknown",
                        "auth_user_type_from_profile_volunteer_visitor",
                        "auth_can_access_edition_admin_always",
                        "auth_can_access_edition_manager_only_when_is_manager",
                        "auth_can_access_edition_exhibitor_only_when_is_manager",
                        "auth_can_access_edition_volunteer_only_when_is_manager",
                        "auth_can_access_edition_visitor_only_when_is_manager",
                        "auth_can_access_edition_unknown_only_when_is_manager",
                    ],
                },
                TestGroup {
                    name: "src/auth/mod.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "auth_session_construction",
                        "auth_session_with_profile",
                        "auth_error_display",
                        "auth_error_from_io_like",
                    ],
                },
                TestGroup {
                    name: "src/supabase/types.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "user_type_from_str_roundtrip",
                        "user_type_as_str_roundtrip",
                        "profile_default",
                        "edition_default",
                        "organisateur_default",
                        "exposant_default",
                        "edition_exposant_default",
                    ],
                },
                TestGroup {
                    name: "src/screens/mod.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "screen_id_unc_all_variants",
                        "screen_id_org_all_variants",
                        "screen_id_exp_all_variants",
                        "screen_id_vis_all_variants",
                        "screen_id_no_overlap",
                    ],
                },
                TestGroup {
                    name: "src/theme.rs",
                    kind: TestKind::Unit,
                    tests: vec![
                        "theme_new_dark",
                        "theme_new_light",
                        "theme_background_primary_dark_light_differ",
                        "theme_radius_constants",
                        "theme_spacing_constants",
                        "theme_font_sizes",
                        "theme_accent_primary_fixed",
                        "theme_accent_organisateur_exposant_visiteur_differ",
                        "theme_badge_colors",
                        "theme_section_border_dark_light_differ",
                    ],
                },
                TestGroup {
                    name: "tests/global_router.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "global_router_every_screen_classified",
                        "global_router_reserved_org_exp_vis_classified",
                        "global_router_initial_screen_is_unc_landing",
                        "global_router_navigate_then_apply_changes_screen",
                        "global_router_apply_without_request_idempotent",
                        "global_router_unc_count",
                        "global_router_org_count",
                        "global_router_exp_count",
                        "global_router_vis_count",
                    ],
                },
                TestGroup {
                    name: "tests/parcours_unc.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "parcours_unc_landing_to_liste_evenements",
                        "parcours_unc_landing_to_connexion",
                        "parcours_unc_landing_to_inscription",
                        "parcours_unc_sequence_landing_liste_fiche_back",
                        "parcours_unc_sequence_connexion_inscription",
                        "parcours_unc_all_screens_reachable",
                    ],
                },
                TestGroup {
                    name: "tests/parcours_org.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "parcours_org_dashboard_to_liste_editions",
                        "parcours_org_dashboard_to_creation_edition",
                        "parcours_org_sequence_liste_creation_dashboard",
                        "parcours_org_sequence_dashboard_edition_candidatures_fiche",
                        "parcours_org_all_screens_reachable",
                    ],
                },
                TestGroup {
                    name: "tests/parcours_exp.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "parcours_exp_dashboard_to_candidatures",
                        "parcours_exp_dashboard_to_participations",
                        "parcours_exp_sequence_full",
                        "parcours_exp_all_screens_reachable",
                    ],
                },
                TestGroup {
                    name: "tests/parcours_vis.rs",
                    kind: TestKind::Integration,
                    tests: vec![
                        "parcours_vis_dashboard_to_agenda",
                        "parcours_vis_sequence_dashboard_agenda_dashboard",
                        "parcours_vis_all_screens_reachable",
                    ],
                },
            ],
        },
        TestSuite {
            crate_name: "bondingbrother",
            display_name: "BondingBrother",
            description: "Médiation : translation, sync, connection.",
            groups: vec![
                TestGroup {
                    name: "src/translation.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_translation_creation"],
                },
                TestGroup {
                    name: "src/sync.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_sync_strategies"],
                },
                TestGroup {
                    name: "src/connection.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_connection_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "borderguard",
            display_name: "BorderGuard",
            description: "Core frontières : trust_level, crossing, boundary.",
            groups: vec![
                TestGroup {
                    name: "src/trust_level.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_trust_level_ordering"],
                },
                TestGroup {
                    name: "src/crossing.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_crossing_rule_creation"],
                },
                TestGroup {
                    name: "src/boundary.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_boundary_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "caringnanny",
            display_name: "Caring Nanny",
            description: "Observation d'état : metrics, health, observer.",
            groups: vec![
                TestGroup {
                    name: "src/metrics.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_default_metrics"],
                },
                TestGroup {
                    name: "src/health.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_health_status_equality"],
                },
                TestGroup {
                    name: "src/observer.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_event_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "everbuddy",
            display_name: "Ever Buddy",
            description: "Cycle de vie : version, compatibility, migration.",
            groups: vec![
                TestGroup {
                    name: "src/version.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_version_creation"],
                },
                TestGroup {
                    name: "src/compatibility.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_compatibility_levels"],
                },
                TestGroup {
                    name: "src/migration.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_migration_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "masterbutler",
            display_name: "Master Butler",
            description: "Capacités : step, orchestrator, workflow.",
            groups: vec![
                TestGroup {
                    name: "src/step.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_step_creation"],
                },
                TestGroup {
                    name: "src/orchestrator.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_workflow_execution_structure"],
                },
                TestGroup {
                    name: "src/workflow.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_workflow_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "worrysentinel",
            display_name: "WorrySentinel",
            description: "Gouvernance de sécurité : degradation, security_level, threat_detector.",
            groups: vec![
                TestGroup {
                    name: "src/degradation.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_degradation_state_ordering"],
                },
                TestGroup {
                    name: "src/security_level.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_security_level_ordering"],
                },
                TestGroup {
                    name: "src/threat_detector.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_threat_level_ordering"],
                },
            ],
        },
        TestSuite {
            crate_name: "tamr",
            display_name: "TAMR",
            description: "Intervention humaine : classification, metadata, taxonomy.",
            groups: vec![
                TestGroup {
                    name: "src/classification.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_classification_creation"],
                },
                TestGroup {
                    name: "src/metadata.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_metadata_creation"],
                },
                TestGroup {
                    name: "src/taxonomy.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_taxonomy_creation"],
                },
            ],
        },
        TestSuite {
            crate_name: "logisticssteward",
            display_name: "Logistics Steward",
            description: "Allocation, optimization, resource.",
            groups: vec![
                TestGroup {
                    name: "src/allocation.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_allocation_creation"],
                },
                TestGroup {
                    name: "src/optimization.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_optimization_creation"],
                },
                TestGroup {
                    name: "src/resource.rs",
                    kind: TestKind::Unit,
                    tests: vec!["test_resource_creation"],
                },
            ],
        },
    ]
}

/// État du service Dev Center.
pub struct DevCenterService {
    /// None = vue d'ensemble, Some(index) = page du crate correspondant.
    selected_page: Option<usize>,
    /// Log de la console dev (sortie des tests, messages).
    console_log: Vec<String>,
    /// True pendant l'exécution des tests (en attente du thread).
    test_running: bool,
    /// Réception des lignes de sortie une fois les tests terminés.
    test_rx: Option<mpsc::Receiver<Vec<String>>>,
}

impl Default for DevCenterService {
    fn default() -> Self {
        Self {
            selected_page: None,
            console_log: vec!["[MDC] Console dev prête. Sélectionnez un crate et cliquez sur « Lancer les tests ».".to_string()],
            test_running: false,
            test_rx: None,
        }
    }
}

impl ServiceUi for DevCenterService {
    fn id(&self) -> ServiceId {
        ServiceId::DevCenter
    }
    fn title(&self) -> &'static str {
        "Miyukini Dev Center"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        let suites = all_test_suites();
        // Couleurs depuis le thème appliqué par l'app (UI Editor ou Chrome).
        let visuals = &ui.ctx().style().visuals;
        let dark_mode = visuals.dark_mode;
        let theme = PixelChromeTheme::new(dark_mode);

        // Réception du résultat des tests (thread terminé)
        if self.test_running {
            if let Some(ref rx) = self.test_rx {
                if let Ok(lines) = rx.try_recv() {
                    self.console_log.extend(lines);
                    self.test_running = false;
                    self.test_rx = None;
                }
            }
        }

        const SIDEBAR_WIDTH: f32 = 220.0;
        const GAP: f32 = 8.0;
        const CONSOLE_HEIGHT: f32 = 220.0;

        let full_rect = ui.available_rect_before_wrap();
        let full_height = full_rect.height();

        // Sidebar gauche : Vue d'ensemble + liste des crates
        let sidebar_rect = egui::Rect::from_min_size(full_rect.min, egui::vec2(SIDEBAR_WIDTH, full_height));
        #[allow(deprecated)]
        ui.allocate_ui_at_rect(sidebar_rect, |ui| {
            ui.set_min_width(SIDEBAR_WIDTH - 2.0);
            ui.set_max_width(SIDEBAR_WIDTH - 2.0);
            ui.set_min_height(full_height);
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                egui::CornerRadius::same(6),
                visuals.panel_fill,
            );
            let inner = ui.available_rect_before_wrap().shrink(8.0);
            #[allow(deprecated)]
            ui.allocate_ui_at_rect(inner, |ui| {
                ui.push_id(egui::Id::new(("mdc", "sidebar_scroll")), |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(inner.height())
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.heading(egui::RichText::new("🧪 Dev Center").size(14.0).strong());
                            ui.add_space(6.0);
                            let overview_selected = self.selected_page.is_none();
                            if ui
                                .selectable_label(overview_selected, "📋 Vue d'ensemble")
                                .clicked()
                            {
                                self.selected_page = None;
                            }
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("Par crate").size(12.0).strong());
                            ui.add_space(4.0);
                            for (idx, suite) in suites.iter().enumerate() {
                                let selected = self.selected_page == Some(idx);
                                let label = format!("{} {} ({})", suite.display_name, "▪", suite.total_tests());
                                if ui.selectable_label(selected, label).clicked() {
                                    self.selected_page = Some(idx);
                                }
                            }
                        });
                });
            });
        });

        // Body : contenu + console en bas
        let body_min_x = sidebar_rect.max.x + GAP;
        let body_max_x = full_rect.max.x;
        let content_height = (full_rect.height() - CONSOLE_HEIGHT - GAP).max(120.0);
        let content_rect = egui::Rect::from_min_max(
            egui::pos2(body_min_x, full_rect.min.y),
            egui::pos2(body_max_x, full_rect.min.y + content_height),
        );
        let console_rect = egui::Rect::from_min_max(
            egui::pos2(body_min_x, content_rect.max.y + GAP),
            egui::pos2(body_max_x, full_rect.max.y),
        );
        let body_width = content_rect.width();

        #[allow(deprecated)]
        ui.allocate_ui_at_rect(content_rect, |ui| {
            ui.set_min_height(content_rect.height());
            ui.push_id(egui::Id::new(("mdc", "content_scroll")), |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .max_height(content_rect.height())
                    .show(ui, |ui| {
                        ui.set_min_width(body_width);
                        if let Some(idx) = self.selected_page {
                            if let Some(suite) = suites.get(idx) {
                                self.ui_crate_page(ui, suite, &theme);
                            } else {
                                self.ui_overview(ui, &suites);
                            }
                        } else {
                            self.ui_overview(ui, &suites);
                        }
                    });
            });
        });

        // Console dev : log des tests + bouton Effacer
        #[allow(deprecated)]
        ui.allocate_ui_at_rect(console_rect, |ui| {
            self.ui_console(ui, console_rect.height());
        });
    }
}

impl DevCenterService {
    /// Vue d'ensemble : résumé global + liste des crates avec nombre de tests.
    fn ui_overview(&self, ui: &mut egui::Ui, suites: &[TestSuite]) {
        ui.add_space(12.0);
        ui.heading(egui::RichText::new("Vue d'ensemble").size(18.0).strong());
        ui.add_space(8.0);
        let total: usize = suites.iter().map(TestSuite::total_tests).sum();
        ui.label(format!("{} crates avec tests  •  {} tests au total", suites.len(), total));
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        for (_idx, suite) in suites.iter().enumerate() {
            let n = suite.total_tests();
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(suite.display_name).size(13.0).strong());
                ui.label(format!("{} tests", n));
            });
            ui.label(egui::RichText::new(suite.description).size(12.0).weak());
            ui.add_space(6.0);
        }
    }

    /// Page détail d'un crate : groupes de tests + bouton Lancer les tests + commande.
    fn ui_crate_page(&mut self, ui: &mut egui::Ui, suite: &TestSuite, theme: &PixelChromeTheme) {
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(suite.display_name).size(18.0).strong());
            ui.label(format!("({} tests)", suite.total_tests()));
            ui.add_space(16.0);
            let run_label = if self.test_running {
                "Exécution…"
            } else {
                "Lancer les tests"
            };
            if ui
                .button(egui::RichText::new(run_label).size(12.0))
                .on_hover_text(format!("cargo test -p {}", suite.crate_name))
                .clicked()
                && !self.test_running
            {
                self.run_tests_for_crate(suite.crate_name);
            }
        });
        ui.add_space(4.0);
        ui.label(egui::RichText::new(suite.description).size(12.0).weak());
        ui.add_space(12.0);

        // Commande pour lancer les tests
        let cmd = format!("cargo test -p {}", suite.crate_name);
        ui.label(egui::RichText::new("Commande").size(12.0).strong());
        ui.add_space(4.0);
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width().min(520.0), 36.0),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(rect, 4.0, theme.tab_inactive_bg());
        ui.painter().rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(1.0, theme.tab_separator()),
            egui::StrokeKind::Outside,
        );
        ui.painter().text(
            egui::pos2(rect.min.x + 10.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            cmd.as_str(),
            egui::FontId::monospace(12.0),
            ui.ctx().style().visuals.text_color(),
        );
        ui.allocate_rect(rect, egui::Sense::hover());
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Groupes de tests
        ui.label(egui::RichText::new("Tests regroupés par module/fichier").size(12.0).strong());
        ui.add_space(8.0);

        for group in &suite.groups {
            ui.push_id(egui::Id::new((suite.crate_name, group.name)), |ui| {
                ui.collapsing(
                    format!("{} {} — {} ({})", group.kind.icon(), group.name, group.kind.label(), group.tests.len()),
                    |ui| {
                        for name in &group.tests {
                            ui.horizontal(|ui| {
                                ui.label("  ");
                                ui.label(egui::RichText::new(*name).size(11.0).family(egui::FontFamily::Monospace));
                            });
                        }
                    },
                );
            });
        }
    }

    /// Lance les tests du crate dans un thread et envoie la sortie sur le channel.
    fn run_tests_for_crate(&mut self, crate_name: &'static str) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let h = (ts / 3600) % 24;
        let m = (ts / 60) % 60;
        let s = ts % 60;
        self.console_log
            .push(format!("[MDC {:02}:{:02}:{:02}] Lancement des tests pour {}…", h, m, s, crate_name));

        let (tx, rx) = mpsc::channel();
        let crate_name = crate_name.to_string();
        thread::spawn(move || {
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let output = std::process::Command::new("cargo")
                .args(["test", "-p", &crate_name, "--", "--nocapture"])
                .current_dir(&cwd)
                .output();
            let lines = match output {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    let mut v: Vec<String> = stdout
                        .lines()
                        .chain(stderr.lines())
                        .map(String::from)
                        .collect();
                    if v.is_empty() && !o.status.success() {
                        v.push(format!("Process exited with: {}", o.status));
                    }
                    v
                }
                Err(e) => vec![format!("Erreur: {}", e)],
            };
            let _ = tx.send(lines);
        });
        self.test_rx = Some(rx);
        self.test_running = true;
    }

    /// Affiche la console dev : titre, bouton Effacer, log scrollable (couleurs depuis le thème actif).
    fn ui_console(&mut self, ui: &mut egui::Ui, height: f32) {
        let v = &ui.ctx().style().visuals;
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            egui::CornerRadius::same(6),
            v.panel_fill,
        );
        let inner = ui.available_rect_before_wrap().shrink(8.0);
        #[allow(deprecated)]
        ui.allocate_ui_at_rect(inner, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Console dev").size(12.0).strong());
                ui.add_space(8.0);
                if ui.small_button("Effacer").clicked() {
                    self.console_log.clear();
                    self.console_log.push("[MDC] Console effacée.".to_string());
                }
                if self.test_running {
                    ui.label(egui::RichText::new("⏳ Exécution des tests…").size(11.0).weak());
                }
            });
            ui.add_space(4.0);
            ui.push_id(egui::Id::new(("mdc", "console_scroll")), |ui| {
                let scroll = egui::ScrollArea::vertical()
                    .max_height((height - 36.0).max(80.0))
                    .auto_shrink([false, false])
                    .stick_to_bottom(true);
                scroll.show(ui, |ui| {
                    let w = ui.available_width();
                    ui.set_min_width(w);
                    for line in &self.console_log {
                        ui.label(
                            egui::RichText::new(line.as_str())
                                .size(11.0)
                                .family(egui::FontFamily::Monospace),
                        );
                    }
                });
            });
        });
    }
}
