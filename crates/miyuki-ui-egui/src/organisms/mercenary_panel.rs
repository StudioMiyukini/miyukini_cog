// @id: MUIE-MercPanel @do: mercenary-panel-organism @role: organism @layer: 6 @human: miyuk

//! Mercenary panel organism -- equipment + stats display.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::molecules::equip_slot::{EquipItemData, EquipPosition, EquipSlot};
use crate::molecules::stat_row::StatRow;
use crate::convert::rgba_to_color32;

/// Mercenary data.
pub struct MercenaryData {
    /// Mercenary name.
    pub name: String,
    /// Mercenary type (e.g., "Garde du desert").
    pub merc_type: String,
    /// Level.
    pub level: i32,
    /// Life.
    pub life: i32,
    /// Defense.
    pub defense: i32,
    /// Equipment (helm, body, weapon).
    pub helm: Option<EquipItemData>,
    pub body: Option<EquipItemData>,
    pub weapon: Option<EquipItemData>,
}

/// The mercenary panel organism.
pub struct MercenaryPanel;

impl MercenaryPanel {
    /// Draw the mercenary panel.
    pub fn show(ctx: &Context, is_open: &mut bool, data: &MercenaryData) {
        if !*is_open {
            return;
        }

        egui::Window::new("Mercenaire")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([20.0, 200.0])
            .min_width(200.0)
            .show(ctx, |ui| {
                let gold = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new(&data.name).color(gold).size(14.0));

                let info = format!("{} - Nv {}", data.merc_type, data.level);
                let text_color = rgba_to_color32(&D2_PALETTE.text_primary);
                ui.label(egui::RichText::new(&info).color(text_color).size(11.0));
                ui.separator();

                // Equipment
                ui.horizontal(|ui| {
                    let mut helm_slot = EquipSlot::new(EquipPosition::Head);
                    if let Some(ref h) = data.helm {
                        helm_slot = helm_slot.item(h);
                    }
                    helm_slot.show(ui);

                    let mut body_slot = EquipSlot::new(EquipPosition::Body);
                    if let Some(ref b) = data.body {
                        body_slot = body_slot.item(b);
                    }
                    body_slot.show(ui);

                    let mut wep_slot = EquipSlot::new(EquipPosition::MainHand);
                    if let Some(ref w) = data.weapon {
                        wep_slot = wep_slot.item(w);
                    }
                    wep_slot.show(ui);
                });

                ui.separator();

                // Stats
                StatRow::derived("Vie", data.life).show(ui);
                StatRow::derived("Defense", data.defense).show(ui);
            });
    }
}
