//! Atomes UI JayKonta.

use crate::domain::model::KpiMetric;
use crate::theme::{FeatureStatus, JayKontaTheme};
use egui::{Color32, RichText, Ui};

/// Badge de statut pour fonctionnalite.
pub fn status_badge(ui: &mut Ui, status: FeatureStatus, theme: &JayKontaTheme) {
    let color = theme.status_color(status);
    let label = match status {
        FeatureStatus::Live => "LIVE",
        FeatureStatus::Build => "BUILD",
        FeatureStatus::Locked => "LOCKED",
    };
    ui.label(
        RichText::new(label)
            .size(10.0)
            .strong()
            .color(color)
            .background_color(color.linear_multiply(0.18)),
    );
}

/// Badge de severite textuelle.
pub fn severity_badge(ui: &mut Ui, severity: &str, theme: &JayKontaTheme) {
    let color = match severity {
        "ok" => theme.positive,
        "warn" => theme.warning,
        "risk" => theme.danger,
        _ => theme.accent_blue,
    };
    ui.label(
        RichText::new(severity.to_uppercase())
            .size(9.0)
            .strong()
            .color(color)
            .background_color(color.linear_multiply(0.2)),
    );
}

/// Petite puce contractuelle.
pub fn chip(ui: &mut Ui, text: &str, color: Color32) {
    ui.label(
        RichText::new(text)
            .size(10.0)
            .strong()
            .color(color)
            .background_color(color.linear_multiply(0.16)),
    );
}

/// Carte KPI atomique.
pub fn kpi_card(ui: &mut Ui, kpi: &KpiMetric, theme: &JayKontaTheme) {
    egui::Frame::new()
        .fill(theme.card_bg)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(egui::CornerRadius::same(10))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.label(RichText::new(kpi.label).size(11.0).color(theme.muted_text));
            ui.label(RichText::new(&kpi.value).size(18.0).strong().color(theme.text));
            ui.label(RichText::new(&kpi.delta).size(10.0).color(theme.positive));
        });
}
