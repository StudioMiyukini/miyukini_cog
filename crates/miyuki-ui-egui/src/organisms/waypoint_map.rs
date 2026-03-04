// @id: MUIE-WPMap @do: waypoint-map-organism @role: organism @layer: 6 @human: miyuk

//! Waypoint map organism -- 5 act tabs with teleport destinations.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::molecules::waypoint_entry::WaypointEntry;
use crate::convert::rgba_to_color32;

/// Waypoint data for one act.
#[derive(Debug, Clone)]
pub struct ActWaypointData {
    /// Act name.
    pub name: String,
    /// Waypoints in this act.
    pub waypoints: Vec<WaypointInfo>,
}

/// Individual waypoint info.
#[derive(Debug, Clone)]
pub struct WaypointInfo {
    /// Zone name.
    pub name: String,
    /// Whether this waypoint has been discovered.
    pub discovered: bool,
}

/// The waypoint map organism.
pub struct WaypointMap;

impl WaypointMap {
    /// Draw the waypoint map as a window.
    pub fn show(
        ctx: &Context,
        is_open: &mut bool,
        acts: &[ActWaypointData],
        active_act: usize,
        current_wp: Option<&str>,
    ) {
        if !*is_open {
            return;
        }

        egui::Window::new("Waypoints")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([200.0, 100.0])
            .min_width(280.0)
            .show(ctx, |ui| {
                // Act tabs
                ui.horizontal(|ui| {
                    for (i, act) in acts.iter().enumerate() {
                        let color = if i == active_act {
                            rgba_to_color32(&D2_PALETTE.text_high)
                        } else {
                            rgba_to_color32(&D2_PALETTE.text_secondary)
                        };
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&act.name).color(color).size(11.0),
                            )
                            .sense(egui::Sense::click()),
                        );
                    }
                });

                ui.separator();

                // Waypoint entries
                let act_idx = active_act.min(acts.len().saturating_sub(1));
                if let Some(act) = acts.get(act_idx) {
                    for wp in &act.waypoints {
                        let is_current = current_wp.is_some_and(|c| c == wp.name);
                        WaypointEntry::new(&wp.name, wp.discovered)
                            .current(is_current)
                            .show(ui);
                    }
                }
            });
    }
}
