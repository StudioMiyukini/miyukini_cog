//! Molecules UI JayKonta.

use crate::domain::model::{DashboardSection, EntryPoint, QuickAction};
use crate::theme::JayKontaTheme;
use egui::{RichText, Ui};

/// Selecteur `Account` / `Purse`.
pub fn entry_point_switch(
    ui: &mut Ui,
    selected: EntryPoint,
    theme: &JayKontaTheme,
) -> Option<EntryPoint> {
    let mut changed = None;
    ui.horizontal(|ui| {
        for entry in [EntryPoint::Account, EntryPoint::Purse] {
            let is_selected = entry == selected;
            let label = if is_selected {
                RichText::new(entry.label()).strong().color(theme.text)
            } else {
                RichText::new(entry.label()).color(theme.muted_text)
            };
            if ui
                .add(
                    egui::Button::new(label)
                        .fill(if is_selected {
                            theme.accent_blue.linear_multiply(0.24)
                        } else {
                            theme.card_bg
                        })
                        .stroke(egui::Stroke::new(1.0, theme.border))
                        .corner_radius(egui::CornerRadius::same(8)),
                )
                .clicked()
            {
                changed = Some(entry);
            }
        }
    });
    changed
}

/// Menu de sections lateral.
pub fn section_menu(
    ui: &mut Ui,
    selected: DashboardSection,
    theme: &JayKontaTheme,
) -> Option<DashboardSection> {
    let mut changed = None;
    for section in DashboardSection::ALL {
        let active = section == selected;
        let text = if active {
            RichText::new(section.label()).strong().color(theme.text)
        } else {
            RichText::new(section.label()).color(theme.muted_text)
        };
        if ui
            .add(
                egui::Button::new(text)
                    .fill(if active {
                        theme.accent.linear_multiply(0.18)
                    } else {
                        theme.card_bg
                    })
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .corner_radius(egui::CornerRadius::same(8))
                    .min_size(egui::vec2(ui.available_width(), 28.0)),
            )
            .clicked()
        {
            changed = Some(section);
        }
    }
    changed
}

/// Liste compacte d'actions rapides.
pub fn quick_actions(ui: &mut Ui, actions: &[QuickAction], theme: &JayKontaTheme) {
    for action in actions {
        egui::Frame::new()
            .fill(theme.card_bg)
            .stroke(egui::Stroke::new(1.0, theme.border))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(action.label).size(12.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(action.state).size(10.0).color(theme.accent));
                    });
                });
            });
    }
}
