//! Organismes UI JayKonta.

use crate::backend::service::JayKontaBackend;
use crate::domain::model::{ContractItem, DashboardSection, DashboardViewModel};
use crate::theme::JayKontaTheme;
use crate::ui::{atoms, molecules};
use egui::RichText;

/// Sidebar complete (entrypoint + sections + meta COG).
pub fn sidebar(ui: &mut egui::Ui, backend: &mut JayKontaBackend, theme: &JayKontaTheme) {
    egui::Frame::new()
        .fill(theme.panel_bg)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.label(RichText::new("JayKonta").size(20.0).strong());
            ui.label(RichText::new("Dashboard fonctionnalites").size(11.0).color(theme.muted_text));
            ui.add_space(10.0);

            if let Some(next) = molecules::entry_point_switch(ui, backend.entry_point(), theme) {
                backend.set_entry_point(next);
            }

            ui.add_space(10.0);
            if let Some(next_section) = molecules::section_menu(ui, backend.section(), theme) {
                backend.set_section(next_section);
            }

            ui.add_space(10.0);
            egui::Frame::new()
                .fill(theme.card_bg)
                .stroke(egui::Stroke::new(1.0, theme.border))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.label(RichText::new("Etat COG").size(11.0).strong().color(theme.warning));
                    ui.label(RichText::new("Niveau securite actif: 2").size(10.0).color(theme.muted_text));
                    ui.label(RichText::new("Audit ecritures: ON").size(10.0).color(theme.muted_text));
                    ui.label(
                        RichText::new("Residence sensibles: centralisee")
                            .size(10.0)
                            .color(theme.muted_text),
                    );
                });
        });
}

/// Header hero principal.
pub fn hero(ui: &mut egui::Ui, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    egui::Frame::new()
        .fill(theme.panel_bg)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].label(RichText::new(vm.hero_title).size(21.0).strong());
                columns[0].label(RichText::new(vm.hero_subtitle).size(12.0).color(theme.muted_text));
                columns[0].horizontal_wrapped(|ui| {
                    atoms::chip(ui, "CK-SVC-01 actif", theme.accent);
                    atoms::chip(ui, "CK-INT-01 JayFestival", theme.accent_blue);
                    atoms::chip(ui, "CK-INT-02 JayRDV", theme.accent);
                    atoms::chip(ui, "Audit exports", theme.warning);
                });

                columns[1]
                    .label(RichText::new("Score sante operationnelle").size(11.0).color(theme.muted_text));
                columns[1].label(RichText::new(vm.health_score).size(28.0).strong().color(theme.positive));
                columns[1].label(RichText::new(vm.top_alert).size(11.0).color(theme.warning));
            });
        });
}

/// Grille KPI.
pub fn kpis(ui: &mut egui::Ui, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    egui::Grid::new("jaykonta_kpis")
        .num_columns(2)
        .spacing(egui::vec2(10.0, 10.0))
        .show(ui, |ui| {
            for (i, kpi) in vm.kpis.iter().enumerate() {
                atoms::kpi_card(ui, kpi, theme);
                if i % 2 == 1 {
                    ui.end_row();
                }
            }
        });
}

/// Corps du dashboard selon section.
pub fn body(ui: &mut egui::Ui, section: DashboardSection, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    match section {
        DashboardSection::Overview => overview(ui, vm, theme),
        DashboardSection::QuotesInvoices => contracts_slice(ui, "Cycle Devis / Factures", &vm.contracts, theme),
        DashboardSection::Ledger => contracts_slice(ui, "Comptabilite et Audit", &vm.contracts, theme),
        DashboardSection::Payments => integrations_slice(ui, "Paiements et passerelles", vm, theme),
        DashboardSection::Reports => toolkits_slice(ui, "Reporting et exports", vm, theme),
        DashboardSection::Integrations => integrations_slice(ui, "Flux d'integration", vm, theme),
        DashboardSection::Contracts => contracts_slice(ui, "Contrats COG JayKonta", &vm.contracts, theme),
        DashboardSection::Toolkits => toolkits_slice(ui, "Toolkits JayKonta", vm, theme),
        DashboardSection::Operators => operators_slice(ui, "Operateurs JayKonta", vm, theme),
        DashboardSection::Implementation => implementation_slice(ui, vm, theme),
    }
}

fn overview(ui: &mut egui::Ui, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    ui.columns(2, |columns| {
        card(
            &mut columns[0],
            "Actions rapides",
            "Operations principales",
            theme,
            |ui| {
                molecules::quick_actions(ui, &vm.quick_actions, theme);
            },
        );

        card(
            &mut columns[1],
            "Conformite et risques",
            "Signalement proactif",
            theme,
            |ui| {
                for risk in &vm.risks {
                    egui::Frame::new()
                        .fill(theme.card_bg)
                        .stroke(egui::Stroke::new(1.0, theme.border))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(risk.label).size(11.0).strong());
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        atoms::severity_badge(ui, risk.severity, theme);
                                        ui.label(RichText::new(risk.value).size(11.0).color(theme.text));
                                    },
                                );
                            });
                        });
                }
            },
        );
    });

    card(
        ui,
        "Fonctionnalites strategiques",
        "Maturite modules",
        theme,
        |ui| {
            for capability in &vm.capabilities {
                egui::Frame::new()
                    .fill(theme.card_bg)
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(capability.name).size(11.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                atoms::status_badge(ui, capability.status, theme);
                            });
                        });
                    });
            }
        },
    );
}

fn integrations_slice(ui: &mut egui::Ui, title: &str, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    card(ui, title, "Couplage services metier -> JayKonta", theme, |ui| {
        for flow in &vm.integrations {
            egui::Frame::new()
                .fill(theme.card_bg)
                .stroke(egui::Stroke::new(1.0, theme.border))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} : {}", flow.source, flow.operation))
                                .size(11.0)
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            atoms::chip(ui, flow.contract, theme.accent);
                            ui.label(RichText::new(flow.state).size(10.0).color(theme.muted_text));
                        });
                    });
                });
        }
    });
}

fn contracts_slice(ui: &mut egui::Ui, title: &str, contracts: &[ContractItem], theme: &JayKontaTheme) {
    card(ui, title, "Contrats normatifs services/ops/tk/sec/audit", theme, |ui| {
        for contract in contracts {
            egui::Frame::new()
                .fill(theme.card_bg)
                .stroke(egui::Stroke::new(1.0, theme.border))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(contract.id)
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.label(RichText::new(contract.summary).size(11.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(contract.status).size(10.0).color(theme.positive));
                            ui.label(RichText::new(contract.scope).size(10.0).color(theme.muted_text));
                        });
                    });
                });
        }
    });
}

fn toolkits_slice(ui: &mut egui::Ui, title: &str, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    card(ui, title, "Toolkits relies aux parcours metier", theme, |ui| {
        for toolkit in &vm.toolkits {
            egui::Frame::new()
                .fill(theme.card_bg)
                .stroke(egui::Stroke::new(1.0, theme.border))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(toolkit.name)
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.label(RichText::new(toolkit.capability).size(11.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(toolkit.status).size(10.0).color(theme.positive));
                            ui.label(RichText::new(toolkit.audience).size(10.0).color(theme.muted_text));
                        });
                    });
                });
        }
    });
}

fn operators_slice(ui: &mut egui::Ui, title: &str, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    card(ui, title, "Responsabilites operateurs par public", theme, |ui| {
        for operator in &vm.operators {
            egui::Frame::new()
                .fill(theme.card_bg)
                .stroke(egui::Stroke::new(1.0, theme.border))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(operator.name)
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.label(RichText::new(operator.responsibility).size(11.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(operator.status).size(10.0).color(theme.positive));
                            ui.label(RichText::new(operator.audience).size(10.0).color(theme.muted_text));
                        });
                    });
                });
        }
    });
}

fn implementation_slice(ui: &mut egui::Ui, vm: &DashboardViewModel, theme: &JayKontaTheme) {
    card(
        ui,
        "Bornage implementation",
        "Phases in scope / hors scope / criteres",
        theme,
        |ui| {
            for item in &vm.implementation {
                egui::Frame::new()
                    .fill(theme.card_bg)
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.label(RichText::new(item.phase).size(12.0).strong().color(theme.warning));
                        ui.label(RichText::new(format!("In scope: {}", item.in_scope)).size(11.0));
                        ui.label(
                            RichText::new(format!("Hors scope: {}", item.out_scope))
                                .size(11.0)
                                .color(theme.muted_text),
                        );
                        ui.label(
                            RichText::new(format!("Sortie: {}", item.exit_criteria))
                                .size(10.0)
                                .color(theme.accent),
                        );
                    });
            }
        },
    );
}

fn card(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    theme: &JayKontaTheme,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::new()
        .fill(theme.panel_bg)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.label(RichText::new(title).size(14.0).strong());
            ui.label(RichText::new(subtitle).size(11.0).color(theme.muted_text));
            ui.add_space(8.0);
            add_contents(ui);
        });
}
