//! Rendu UI Lord of the Castle — fenêtres, zone de jeu, layout, peinture.
//! Fonctions pures ou prenant GameState / rect / painter ; pas de dépendance à LordOfTheCastleApp.
//!
//! @id: lord_of_the_castle_app_ui
//! @do: render_game_zone_windows_and_hud
//! @role: ui
//! @layer: domain
//! @human: Rendu : bottom panel, fenêtres (player, inventaire, équipement), zone de jeu (peinture entités).

use crate::character_creation::Stat;
use crate::SkillsTab;
use crate::constants::{
    size, BOTTOM_PANEL_HEIGHT, COMBAT_SURFACE_SIZE, COMMAND_ZONE_RADIUS, ENEMY_VISION_RADIUS,
    PICKUP_RADIUS, TOWER_PROJECTILE_SIZE,
};
use crate::enemies::EnemyKind;
use crate::game_loop::{move_player, move_secondary_player, tick_battle};
use crate::game_state::{GamePhase, GameState, INVENTORY_MAX_SLOTS};
use crate::loot::{InventoryEntry, ItemSlot, LootKind};
use crate::player::{Dir8, Player};
use crate::spritesheet::{draw_sprite_frame, SpritesheetDesc};
use crate::towers::Tower;
use crate::troops::{TroopKind, TroopState};
use crate::warrior_skills::{prerequisites_met, warrior_skill_def, warrior_skill_edges, WarriorSkillId};
use eframe::egui;
use std::cell::RefCell;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Layout / allocation
// ---------------------------------------------------------------------------

/// Taille de la surface de combat (800×800 max). Conservé pour usage futur / legacy.
#[allow(dead_code)]
pub(crate) fn combat_surface_size(available: egui::Rect) -> egui::Vec2 {
    let w = COMBAT_SURFACE_SIZE.min(available.width());
    let h = COMBAT_SURFACE_SIZE.min(available.height());
    let side = w.min(h);
    egui::Vec2::new(side, side)
}

/// Alloue la surface de combat centrée (legacy). Retourne (game_rect, response).
#[allow(dead_code)]
pub(crate) fn allocate_combat_surface_centered(
    ui: &mut egui::Ui,
    available: egui::Rect,
) -> (egui::Rect, egui::Response) {
    let size = combat_surface_size(available);
    let top_space = (available.height() - size.y) / 2.0;
    let bottom_space = available.height() - size.y - top_space;
    let left_space = (available.width() - size.x) / 2.0;
    let right_space = available.width() - size.x - left_space;
    ui.add_space(top_space.max(0.0));
    let response = ui.horizontal(|ui| {
        ui.add_space(left_space.max(0.0));
        let rect = egui::Rect::from_min_size(ui.cursor().min, size);
        let response = ui.allocate_rect(rect, egui::Sense::click());
        ui.add_space(right_space.max(0.0));
        (rect, response)
    });
    let (game_rect, resp) = response.inner;
    ui.add_space(bottom_space.max(0.0));
    (game_rect, resp)
}

/// Alloue la zone de jeu pleine largeur (body moins bottom) et le rect du panneau bottom.
pub(crate) fn allocate_game_zone_and_bottom(
    ui: &mut egui::Ui,
) -> (egui::Rect, egui::Response, egui::Rect) {
    let available = ui.available_rect_before_wrap();
    let game_h = (available.height() - BOTTOM_PANEL_HEIGHT).max(1.0);
    let game_rect = egui::Rect::from_min_size(
        available.min,
        egui::Vec2::new(available.width(), game_h),
    );
    let resp = ui.allocate_rect(game_rect, egui::Sense::click());
    let bottom_min = egui::pos2(available.min.x, game_rect.max.y);
    let bottom_rect = egui::Rect::from_min_size(
        bottom_min,
        egui::Vec2::new(available.width(), BOTTOM_PANEL_HEIGHT),
    );
    ui.allocate_rect(bottom_rect, egui::Sense::hover());
    (game_rect, resp, bottom_rect)
}

/// Monde → écran : caméra centrée sur le joueur (style A-RPG type Diablo).
pub(crate) fn world_to_screen(
    rect: egui::Rect,
    wx: f32,
    wy: f32,
    camera_x: f32,
    camera_y: f32,
) -> egui::Pos2 {
    let cx = rect.center().x;
    let cy = rect.center().y;
    egui::Pos2::new(cx + wx - camera_x, cy + wy - camera_y)
}

/// Écran → monde : inverse de world_to_screen (curseur → coords monde).
pub(crate) fn screen_to_world(
    rect: egui::Rect,
    sx: f32,
    sy: f32,
    camera_x: f32,
    camera_y: f32,
) -> (f32, f32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    (sx - cx + camera_x, sy - cy + camera_y)
}

/// Centre de la caméra : joueur 1 seul, ou milieu entre joueur 1 et joueur 2 en mode 2 joueurs (pour garder les deux à l'écran).
pub(crate) fn camera_center(state: &GameState) -> (f32, f32) {
    match state.secondary_player.as_ref() {
        Some(sec) if !sec.is_dead() => (
            f32::midpoint(state.player.x, sec.x),
            f32::midpoint(state.player.y, sec.y),
        ),
        _ => (state.player.x, state.player.y),
    }
}

/// Monde → cellule de la grille de construction (origine = coin inférieur gauche du château).
pub(crate) fn world_to_cell(castle_x: f32, castle_y: f32, wx: f32, wy: f32) -> (i32, i32) {
    let cell_size = size::CONSTRUCTION_CELL_SIZE;
    let origin_x = castle_x - size::CASTLE / 2.0;
    let origin_y = castle_y - size::CASTLE / 2.0;
    let ci = ((wx - origin_x) / cell_size).floor() as i32;
    let cj = ((wy - origin_y) / cell_size).floor() as i32;
    (ci, cj)
}

/// Écran → cellule (pour clic en mode construction).
pub(crate) fn screen_to_cell(
    rect: egui::Rect,
    sx: f32,
    sy: f32,
    camera_x: f32,
    camera_y: f32,
    castle_x: f32,
    castle_y: f32,
) -> (i32, i32) {
    let (wx, wy) = screen_to_world(rect, sx, sy, camera_x, camera_y);
    world_to_cell(castle_x, castle_y, wx, wy)
}

// ---------------------------------------------------------------------------
// Fenêtres (player, inventaire, équipement, bottom panel)
// ---------------------------------------------------------------------------

/// Peint le panneau bottom : barre XP, PV, Mana.
pub(crate) fn paint_bottom_panel(
    ui: &mut egui::Ui,
    bottom_rect: egui::Rect,
    state: &GameState,
) {
    let painter = ui.painter();
    let margin = 4.0;
    let line_h = 14.0;
    let gap = 6.0;
    let inner = bottom_rect.shrink(margin);
    painter.rect_filled(bottom_rect, 0.0, egui::Color32::from_rgb(28, 28, 32));

    let xp_y = inner.min.y;
    let xp_bar_h = line_h;
    let xp_rect = egui::Rect::from_min_size(
        egui::pos2(inner.min.x, xp_y),
        egui::Vec2::new(inner.width(), xp_bar_h),
    );
    painter.rect_filled(xp_rect, 2.0, egui::Color32::from_rgb(40, 44, 52));
    let xp_next = state.xp_required_for_next_level();
    let t_xp = if xp_next > 0 {
        (state.xp as f32 / xp_next as f32).min(1.0)
    } else {
        1.0
    };
    let xp_fill_w = xp_rect.width() * t_xp;
    if xp_fill_w > 0.0 {
        let xp_fill = egui::Rect::from_min_size(
            xp_rect.min,
            egui::Vec2::new(xp_fill_w, xp_bar_h),
        );
        painter.rect_filled(xp_fill, 2.0, egui::Color32::from_rgb(70, 130, 180));
    }
    let xp_text = format!("{} / {}", state.xp, xp_next);
    painter.text(
        xp_rect.center(),
        egui::Align2::CENTER_CENTER,
        xp_text,
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );

    let row2_y = xp_y + xp_bar_h + gap;
    let half_w = inner.width() / 2.0 - gap / 2.0;
    let pv_rect = egui::Rect::from_min_size(
        egui::pos2(inner.min.x, row2_y),
        egui::Vec2::new(half_w, line_h),
    );
    let mana_rect = egui::Rect::from_min_size(
        egui::pos2(inner.min.x + half_w + gap, row2_y),
        egui::Vec2::new(half_w, line_h),
    );

    let t_hp = if state.player.hp_max > 0 {
        (state.player.hp as f32 / state.player.hp_max as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    painter.rect_filled(pv_rect, 2.0, egui::Color32::from_rgb(48, 40, 40));
    if t_hp > 0.0 {
        let hp_fill = egui::Rect::from_min_size(
            pv_rect.min,
            egui::Vec2::new(pv_rect.width() * t_hp, line_h),
        );
        painter.rect_filled(hp_fill, 2.0, egui::Color32::from_rgb(180, 60, 60));
    }
    painter.text(
        pv_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{} / {}", state.player.hp, state.player.hp_max),
        egui::FontId::proportional(11.0),
        egui::Color32::WHITE,
    );

    let t_mana = if state.player.mana_max > 0 {
        (state.player.mana as f32 / state.player.mana_max as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    painter.rect_filled(mana_rect, 2.0, egui::Color32::from_rgb(40, 40, 52));
    if t_mana > 0.0 {
        let mana_fill = egui::Rect::from_min_size(
            mana_rect.min,
            egui::Vec2::new(mana_rect.width() * t_mana, line_h),
        );
        painter.rect_filled(mana_fill, 2.0, egui::Color32::from_rgb(60, 90, 180));
    }
    painter.text(
        mana_rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{} / {}", state.player.mana, state.player.mana_max),
        egui::FontId::proportional(11.0),
        egui::Color32::WHITE,
    );
}

/// Fenêtre fiche joueur : nom, niveau, PV/Mana, XP, points disponibles, caractéristiques (+ pour stats), statistiques.
pub(crate) fn paint_player_window(ui: &mut egui::Ui, state: Option<&mut GameState>) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    let name = if state.player.name.is_empty() { "Joueur" } else { state.player.name.as_str() };
    ui.horizontal(|ui| {
        ui.heading(name);
        ui.label(format!("Niveau {}", state.level));
        if state.skill_points_available > 0 || state.stat_points_available > 0 {
            ui.label("➕");
        }
    });
    ui.add_space(4.0);
    ui.label(format!("{} / {} PV  —  {} / {} Mana", state.player.hp, state.player.hp_max, state.player.mana, state.player.mana_max));
    ui.add_space(2.0);
    let xp_next = state.xp_required_for_next_level();
    ui.label(format!("{} / {} XP pour le prochain niveau", state.xp, xp_next));
    if state.skill_points_available > 0 {
        ui.label(format!("{} point(s) de compétence disponible(s)", state.skill_points_available));
    }
    if state.stat_points_available > 0 {
        ui.label(format!("{} point(s) de statistique disponible(s) — utilisez le + à côté d'une caractéristique", state.stat_points_available));
    }
    ui.add_space(8.0);
    let stats_list = [
        (Stat::For, "Force (For)"),
        (Stat::Con, "Constitution (Con)"),
        (Stat::Agi, "Agilité (Agi)"),
        (Stat::Dex, "Dextérité (Dex)"),
        (Stat::Int, "Intelligence (Int)"),
        (Stat::Sag, "Sagesse (Sag)"),
        (Stat::Cha, "Charisme (Cha)"),
        (Stat::Luk, "Chance (Luk)"),
    ];
    ui.collapsing("Caractéristiques", |ui| {
        for (stat, label) in stats_list {
            ui.horizontal(|ui| {
                if state.stat_points_available > 0
                    && ui.small_button("+").clicked() {
                        state.spend_stat_point(stat);
                    }
                let base = state.player.stats.get(stat);
                let bonus = state.stat_bonus_from_warrior_skills().get(stat);
                let total = base + bonus;
                let stat_str = if bonus != 0 {
                    format!("{total} ({base} + {bonus})")
                } else {
                    format!("{total}")
                };
                ui.label(format!("{label} : {stat_str}"));
            });
        }
    });
    let dmg = state.player_auto_attack_damage();
    let interval_s = Player::auto_attack_interval_s();
    let attack_speed = 1.0 / interval_s;
    ui.collapsing("Statistiques", |ui| {
        ui.label(format!(
            "Dégâts attaque auto : {dmg} (mains nues / CàC / distance selon équipement)"
        ));
        ui.label(format!("Vitesse de déplacement : {:.0} px/s", state.effective_move_speed()));
        ui.label(format!("Vitesse d'attaque : {attack_speed:.1}/s"));
        ui.label(format!("Rayon d'attaque : {} px", Player::auto_attack_range()));
        ui.label(format!("Rayon de collecte : {PICKUP_RADIUS} px"));
        ui.label(format!("Armure : {}", state.player_total_armor()));
        ui.label(format!("Résistance magique : {}", state.player.magic_resist));
        ui.add_space(4.0);
        ui.separator();
        ui.label(format!("Nombre d'ennemis tués : {}", state.enemies_killed));
        ui.label(format!("Vague max : {}", state.max_wave_reached));
        ui.label(format!("Nombre de Boss tués : {}", state.bosses_killed));
        ui.label(format!("Or cumulés : {}", state.gold_total));
    });
}

/// Constantes pour le rendu de l'arbre Guerrier (espacement pour éviter les chevauchements).
const TREE_NODE_W: f32 = 100.0;
const TREE_NODE_H: f32 = 32.0;
const TREE_COL_STEP: f32 = 118.0;
const TREE_ROW_STEP: f32 = 48.0;
const TREE_MARGIN: f32 = 12.0;

/// Fenêtre Compétences : 5 onglets (Guerrier, Sorcier, Saint, Régent, Commandant), chacun ouvre un arbre de compétences.
pub(crate) fn paint_skills_window(
    ui: &mut egui::Ui,
    mut state: Option<&mut GameState>,
    selected_tab: &mut SkillsTab,
    selected_warrior_skill: &mut Option<WarriorSkillId>,
) {
    ui.horizontal(|ui| {
        for tab in [
            SkillsTab::Guerrier,
            SkillsTab::Sorcier,
            SkillsTab::Saint,
            SkillsTab::Regent,
            SkillsTab::Commandant,
        ] {
            let label = match tab {
                SkillsTab::Guerrier => "Guerrier",
                SkillsTab::Sorcier => "Sorcier",
                SkillsTab::Saint => "Saint",
                SkillsTab::Regent => "Régent",
                SkillsTab::Commandant => "Commandant",
            };
            let selected = *selected_tab == tab;
            if ui.selectable_label(selected, label).clicked() {
                *selected_tab = tab;
                *selected_warrior_skill = None;
            }
        }
    });
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
    match *selected_tab {
        SkillsTab::Guerrier => {
            if let Some(ref mut state) = state {
                paint_warrior_tree(ui, state, selected_warrior_skill);
            } else {
                ui.label("Aucune partie en cours.");
            }
        }
        SkillsTab::Sorcier => {
            ui.heading("Arbre Sorcier");
            ui.label("Compétences actives et passives : sortilèges que le joueur peut lancer.");
            ui.add_space(4.0);
            ui.label("(Arbre à venir : nœuds de compétences)");
        }
        SkillsTab::Saint => {
            ui.heading("Arbre Saint");
            ui.label("Compétences de soutien pour le joueur, les tours ou les troupes.");
            ui.add_space(4.0);
            ui.label("(Arbre à venir : nœuds de compétences)");
        }
        SkillsTab::Regent => {
            ui.heading("Arbre Régent");
            ui.label("Compétences passives : plus d'or, plus de bâtiments, bonus avec le marchand.");
            ui.add_space(4.0);
            ui.label("(Arbre à venir : nœuds de compétences)");
        }
        SkillsTab::Commandant => {
            ui.heading("Arbre Commandant");
            ui.label("Compétences actives et passives pour les troupes qui accompagnent le joueur.");
            ui.add_space(4.0);
            ui.label("(Arbre à venir : nœuds de compétences)");
        }
    }
}

/// Peint l'arbre Guerrier : nœuds (rectangles arrondis), lignes de prérequis, clic = détail. Popup détail si une compétence est sélectionnée.
fn paint_warrior_tree(
    ui: &mut egui::Ui,
    state: &mut GameState,
    selected_warrior_skill: &mut Option<WarriorSkillId>,
) {
    // Grille 8 colonnes × 5 lignes pour éviter tout chevauchement
    let tree_width = 8.0 * TREE_COL_STEP + TREE_MARGIN * 2.0;
    let tree_height = 5.0 * TREE_ROW_STEP + TREE_MARGIN * 2.0;
    let tree_size = egui::Vec2::new(tree_width, tree_height);
    let (rect, _response) = ui.allocate_exact_size(tree_size, egui::Sense::click());
    let painter = ui.painter().with_clip_rect(rect);
    let min_x = rect.min.x + TREE_MARGIN;
    let min_y = rect.min.y + TREE_MARGIN;

    let node_center = |col: f32, row: f32| -> egui::Pos2 {
        egui::Pos2::new(
            min_x + col * TREE_COL_STEP + TREE_NODE_W * 0.5,
            min_y + row * TREE_ROW_STEP + TREE_NODE_H * 0.5,
        )
    };
    let node_rect = |col: f32, row: f32| -> egui::Rect {
        egui::Rect::from_center_size(
            node_center(col, row),
            egui::Vec2::new(TREE_NODE_W, TREE_NODE_H),
        )
    };

    // Lignes (prérequis → compétence)
    let stroke = egui::Stroke::new(1.5, egui::Color32::from_gray(120));
    for (from_id, to_id) in warrior_skill_edges() {
        let from_def = warrior_skill_def(from_id);
        let to_def = warrior_skill_def(to_id);
        let p0 = node_center(from_def.layout_col, from_def.layout_row);
        let p1 = node_center(to_def.layout_col, to_def.layout_row);
        painter.line_segment([p0, p1], stroke);
    }

    // Nœuds : chaque nœud est interactif (clic = ouvrir détail)
    let mut clicked_skill = None;
    for &id in WarriorSkillId::all() {
        let def = warrior_skill_def(id);
        let r = node_rect(def.layout_col, def.layout_row);
        let node_id = egui::Id::new(("warrior_node", id));
        let node_response = ui.interact(r, node_id, egui::Sense::click());
        if node_response.clicked() {
            clicked_skill = Some(id);
        }
        let rank = state.warrior_skill_ranks.get(&id).copied().unwrap_or(0);
        let learned = rank > 0;
        let can_learn = prerequisites_met(&state.warrior_skill_ranks, &def) && rank < def.max_rank;
        let fill = if learned {
            egui::Color32::from_rgb(70, 130, 180)
        } else if can_learn {
            egui::Color32::from_rgb(80, 100, 120)
        } else {
            egui::Color32::from_rgb(50, 50, 55)
        };
        let stroke_color = if *selected_warrior_skill == Some(id) {
            egui::Color32::from_rgb(255, 200, 80)
        } else if node_response.hovered() {
            egui::Color32::from_rgb(180, 180, 120)
        } else {
            egui::Color32::from_gray(90)
        };
        painter.rect_filled(r, 4.0, fill);
        painter.rect_stroke(
            r,
            4.0,
            (1.5, stroke_color),
            egui::StrokeKind::Outside,
        );
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            def.name,
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );
    }
    if let Some(id) = clicked_skill {
        *selected_warrior_skill = Some(id);
    }

    // Fenêtre de détail : s'ouvre au clic sur une compétence (nom, description, effet, Apprendre, rang / max)
    if let Some(skill_id) = *selected_warrior_skill {
        let def = warrior_skill_def(skill_id);
        let current = state.warrior_skill_ranks.get(&skill_id).copied().unwrap_or(0);
        let can_learn = state.skill_points_available > 0
            && current < def.max_rank
            && prerequisites_met(&state.warrior_skill_ranks, &def);

        egui::Area::new(egui::Id::new(("warrior_skill_detail", skill_id)))
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-16.0, 40.0))
            .order(egui::Order::Foreground)
            .constrain(true)
            .show(ui.ctx(), |ui| {
                egui::Frame::window(ui.style())
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(def.name);
                            if ui.button("✕").clicked() {
                                *selected_warrior_skill = None;
                            }
                        });
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("Description").strong());
                        ui.label(def.description);
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Effet").strong());
                        ui.label(def.effect_description);
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_enabled(can_learn, egui::Button::new("Apprendre"))
                                .clicked()
                                && can_learn
                            {
                                state.learn_warrior_skill(skill_id);
                            }
                            ui.label(format!("{} / {}", current, def.max_rank));
                        });
                    });
            });
    }
}

/// Fenêtre inventaire : [xx/xx slots], liste des objets. Clic = sélection pour détail / identification.
pub(crate) fn paint_inventory_window(
    ui: &mut egui::Ui,
    state: Option<&mut GameState>,
    selected_slot: &mut Option<usize>,
    pending_identify_self: &RefCell<Option<usize>>,
) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    let n = state.inventory.len();
    ui.label(format!("[{n}/{INVENTORY_MAX_SLOTS} slots]"));
    ui.add_space(6.0);
    ui.separator();
    for (i, entry) in state.inventory.iter().enumerate() {
        let (label, color) = match entry {
            InventoryEntry::Unidentified(slot) => {
                (format!("Objet non-identifié : {}", slot.label()), egui::Color32::GRAY)
            }
            InventoryEntry::Identified(item) => {
                let (r, g, b) = item.rarity.color_rgb();
                (
                    format!(
                        "{} — {} — {}",
                        item.display_name,
                        item.slot.label(),
                        item.rarity.label()
                    ),
                    egui::Color32::from_rgb(r, g, b),
                )
            }
        };
        ui.horizontal(|ui| {
            let btn = egui::Button::new(egui::RichText::new(label.clone()).color(color));
            if ui.add(btn).clicked() {
                *selected_slot = Some(i);
            }
            if let InventoryEntry::Unidentified(_) = entry {
                let can_identify =
                    !state.identified_this_phase && state.phase == GamePhase::Preparation;
                if ui
                    .add_enabled(can_identify, egui::Button::new("Identifier"))
                    .clicked()
                    && can_identify
                {
                    pending_identify_self.replace(Some(i));
                }
            }
        });
    }
}

/// Fenêtre équipement : liste des slots, objet équipé ou "Vide". Clic sur un objet = détail.
pub(crate) fn paint_equipment_window(
    ui: &mut egui::Ui,
    state: Option<&mut GameState>,
    selected_slot: &mut Option<ItemSlot>,
) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    let total_armor = state.player_total_armor();
    ui.heading(format!("Armure totale : {total_armor}"));
    ui.add_space(6.0);
    for &slot in ItemSlot::equipment_slots() {
        let label = match state.get_equipped(slot) {
            Some(item) => format!("{} : {}", slot.label(), item.display_name),
            None => format!("{} : Vide", slot.label()),
        };
        let has_item = state.get_equipped(slot).is_some();
        let btn = egui::Button::new(label);
        if ui.add(btn).clicked() && has_item {
            *selected_slot = Some(slot);
        }
    }
}

// ---------------------------------------------------------------------------
// Zone de jeu : château, joueur, ennemis, tours, barres de vie, loot, dev
// ---------------------------------------------------------------------------

/// Dessine la grille de construction (20×20 px) et la case sélectionnée (bordure bleue).
pub(crate) fn paint_construction_grid(
    painter: &egui::Painter,
    rect: egui::Rect,
    castle_x: f32,
    castle_y: f32,
    camera_x: f32,
    camera_y: f32,
    selected_cell: Option<(i32, i32)>,
) {
    let cell_size = size::CONSTRUCTION_CELL_SIZE;
    let origin_x = castle_x - size::CASTLE / 2.0;
    let origin_y = castle_y - size::CASTLE / 2.0;
    // Grille de -50 à +50 en cellules = ±1000 px en monde depuis l'origine
    const GRID_EXTENT: i32 = 50;
    // Gris 70 % d'opacité pour une grille bien visible
    let line_color = egui::Color32::from_rgba_unmultiplied(100, 100, 100, 178);
    let stroke = 2.0;
    for i in -GRID_EXTENT..=GRID_EXTENT {
        let wx0 = origin_x + (i as f32) * cell_size;
        let wy0 = origin_y - (GRID_EXTENT as f32) * cell_size;
        let wy1 = origin_y + (GRID_EXTENT as f32) * cell_size;
        let p0 = world_to_screen(rect, wx0, wy0, camera_x, camera_y);
        let p1 = world_to_screen(rect, wx0, wy1, camera_x, camera_y);
        painter.line_segment([p0, p1], (stroke, line_color));
    }
    for j in -GRID_EXTENT..=GRID_EXTENT {
        let wy0 = origin_y + (j as f32) * cell_size;
        let wx0 = origin_x - (GRID_EXTENT as f32) * cell_size;
        let wx1 = origin_x + (GRID_EXTENT as f32) * cell_size;
        let p0 = world_to_screen(rect, wx0, wy0, camera_x, camera_y);
        let p1 = world_to_screen(rect, wx1, wy0, camera_x, camera_y);
        painter.line_segment([p0, p1], (stroke, line_color));
    }
    if let Some((ci, cj)) = selected_cell {
        let wx_min = origin_x + (ci as f32) * cell_size;
        let wy_min = origin_y + (cj as f32) * cell_size;
        let wx_max = wx_min + cell_size;
        let wy_max = wy_min + cell_size;
        let min = world_to_screen(rect, wx_min, wy_min, camera_x, camera_y);
        let max = world_to_screen(rect, wx_max, wy_max, camera_x, camera_y);
        let r = egui::Rect::from_min_max(min, max);
        painter.rect_stroke(
            r,
            0.0,
            (3.0, egui::Color32::from_rgb(60, 120, 255)),
            egui::StrokeKind::Outside,
        );
    }
}

pub(crate) fn paint_castle(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let (cx, cy) = camera_center(state);
    let half = size::CASTLE / 2.0;
    let min = world_to_screen(rect, state.castle.x - half, state.castle.y - half, cx, cy);
    let max = world_to_screen(rect, state.castle.x + half, state.castle.y + half, cx, cy);
    let r = egui::Rect::from_min_max(min, max);
    painter.rect_filled(r, 0.0, egui::Color32::from_rgb(100, 100, 110));
    painter.rect_stroke(
        r,
        0.0,
        (2.0, egui::Color32::from_rgb(50, 50, 55)),
        egui::StrokeKind::Outside,
    );
}

/// Taille moitié du joueur secondaire (10×10 px → 5 px).
const SECONDARY_HALF: f32 = 5.0;

/// Dessine le joueur secondaire : carré 10×10 px bleu-vert fluo.
pub(crate) fn paint_secondary_player(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
) {
    let Some(ref sec) = state.secondary_player else {
        return;
    };
    if sec.is_dead() {
        return;
    }
    let (cam_x, cam_y) = camera_center(state);
    let min = world_to_screen(
        rect,
        sec.x - SECONDARY_HALF,
        sec.y - SECONDARY_HALF,
        cam_x,
        cam_y,
    );
    let max = world_to_screen(
        rect,
        sec.x + SECONDARY_HALF,
        sec.y + SECONDARY_HALF,
        cam_x,
        cam_y,
    );
    let color = egui::Color32::from_rgb(0, 255, 200); // bleu-vert fluo
    painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
}

/// Dessine les projectiles homing du joueur secondaire (Tal Ratchou).
pub(crate) fn paint_secondary_projectiles(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
) {
    let (cam_x, cam_y) = camera_center(state);
    let size = 4.0; // petit carré
    for p in &state.secondary_projectiles {
        let min = world_to_screen(rect, p.x - size / 2.0, p.y - size / 2.0, cam_x, cam_y);
        let max = world_to_screen(rect, p.x + size / 2.0, p.y + size / 2.0, cam_x, cam_y);
        painter.rect_filled(
            egui::Rect::from_min_max(min, max),
            0.0,
            egui::Color32::from_rgb(180, 255, 255),
        );
    }
}

/// Dessine le joueur : sprite (spritesheet) par-dessus la hitbox si fourni, sinon carré de couleur.
pub(crate) fn paint_player(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
    player_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize, bool)>,
) {
    let (cam_x, cam_y) = camera_center(state);
    let half = size::MOBILE / 2.0;
    let center = world_to_screen(rect, state.player.x, state.player.y, cam_x, cam_y);

    if let Some((texture, desc, frame_index, flip_h)) = player_sprite {
        let uv = desc.frame_uv_rect(frame_index);
        let (fw, fh) = desc.frame_size();
        let tint = if state.player.dead {
            egui::Color32::from_rgb(140, 100, 80)
        } else {
            egui::Color32::WHITE
        };
        draw_sprite_frame(painter, texture, uv, center, (fw, fh), tint, flip_h);
    } else {
        let min = world_to_screen(
            rect,
            state.player.x - half,
            state.player.y - half,
            cam_x,
            cam_y,
        );
        let max = world_to_screen(
            rect,
            state.player.x + half,
            state.player.y + half,
            cam_x,
            cam_y,
        );
        let color = if state.player.dead {
            egui::Color32::from_rgb(100, 50, 0)
        } else {
            egui::Color32::from_rgb(255, 165, 0)
        };
        painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
    }
}

/// Dessine les ennemis : Skeleton-Walk pour Normal, Werebear-Walk pour MiniBoss si fournis, sinon carrés de couleur.
pub(crate) fn paint_enemies(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
    enemy_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
    enemy_miniboss_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
) {
    let (cam_x, cam_y) = camera_center(state);
    for e in &state.enemies {
        let half = e.half_size();
        let center = world_to_screen(rect, e.x, e.y, cam_x, cam_y);

        let draw_normal_sprite = e.kind == EnemyKind::Normal && enemy_sprite.is_some();
        let draw_miniboss_sprite = e.kind == EnemyKind::MiniBoss && enemy_miniboss_sprite.is_some();

        if draw_normal_sprite {
            if let Some((texture, desc, frame_index)) = enemy_sprite {
                let uv = desc.frame_uv_rect(frame_index);
                let (fw, fh) = desc.frame_size();
                let tint = if e.is_flashing() {
                    egui::Color32::from_rgb(255, 255, 220)
                } else {
                    egui::Color32::WHITE
                };
                let flip_h = e.x > state.player.x;
                draw_sprite_frame(painter, texture, uv, center, (fw, fh), tint, flip_h);
            }
        } else if draw_miniboss_sprite {
            if let Some((texture, desc, frame_index)) = enemy_miniboss_sprite {
                let uv = desc.frame_uv_rect(frame_index);
                let (fw, fh) = desc.frame_size();
                let tint = if e.is_flashing() {
                    egui::Color32::from_rgb(255, 255, 220)
                } else {
                    egui::Color32::WHITE
                };
                let flip_h = e.x > state.player.x;
                draw_sprite_frame(painter, texture, uv, center, (fw, fh), tint, flip_h);
            }
        } else {
            let min = world_to_screen(rect, e.x - half, e.y - half, cam_x, cam_y);
            let max = world_to_screen(rect, e.x + half, e.y + half, cam_x, cam_y);
            let color = if e.is_flashing() {
                egui::Color32::from_rgb(255, 255, 220)
            } else {
                match e.kind {
                    EnemyKind::Normal => egui::Color32::from_rgb(139, 90, 43),
                    EnemyKind::MiniBoss => egui::Color32::from_rgb(255, 165, 0),
                    EnemyKind::Boss => egui::Color32::from_rgb(200, 0, 0),
                }
            };
            painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
        }
    }
}

/// Durée d'affichage de l'animation d'attaque des miliciens (6 frames), en secondes.
const TROOP_ATTACK_ANIM_DURATION_S: f32 = 0.35;

/// Dessine les troupes alliées : Soldier-Walk ou Soldier-Attack01 pour Miliciens si fourni, sinon carrés de couleur.
pub(crate) fn paint_troops(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
    troop_walk_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
    troop_attack_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc)>,
) {
    let (cam_x, cam_y) = camera_center(state);
    for t in &state.troops {
        let half = t.half_size();
        let center = world_to_screen(rect, t.x, t.y, cam_x, cam_y);
        let draw_as_sprite = t.kind == TroopKind::Milicien
            && (troop_walk_sprite.is_some() || troop_attack_sprite.is_some());
        if draw_as_sprite {
            let elapsed_attack = t.last_attack.as_ref().map(|i| i.elapsed().as_secs_f32());
            let in_attack_anim = elapsed_attack.is_some_and(|e| e < TROOP_ATTACK_ANIM_DURATION_S);
            let (texture, desc, frame_index) = if in_attack_anim && troop_attack_sprite.is_some() {
                let (tex, attack_desc) = troop_attack_sprite.unwrap();
                let elapsed = elapsed_attack.unwrap_or(0.0);
                let frame = ((elapsed / TROOP_ATTACK_ANIM_DURATION_S) * 6.0).min(5.0) as usize;
                (tex, attack_desc, frame)
            } else if let Some((tex, walk_desc, frame)) = troop_walk_sprite {
                (tex, walk_desc, frame)
            } else {
                continue;
            };
            let uv = desc.frame_uv_rect(frame_index);
            let (fw, fh) = desc.frame_size();
            let flip_h = t.x > state.player.x;
            draw_sprite_frame(painter, texture, uv, center, (fw, fh), egui::Color32::WHITE, flip_h);
        } else {
            let color = match &t.state {
                TroopState::Dead { .. } => egui::Color32::from_rgb(80, 80, 80),
                TroopState::AtCastle => egui::Color32::from_rgb(120, 120, 100),
                TroopState::OutOfZone => egui::Color32::from_rgb(100, 180, 100),
                TroopState::InZone => egui::Color32::from_rgb(50, 200, 80),
            };
            let min = world_to_screen(rect, t.x - half, t.y - half, cam_x, cam_y);
            let max = world_to_screen(rect, t.x + half, t.y + half, cam_x, cam_y);
            painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
        }
    }
}

pub(crate) fn paint_towers(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let (cam_x, cam_y) = camera_center(state);
    let half = size::TOWER / 2.0;
    for t in &state.towers {
        if t.hp <= 0 {
            continue;
        }
        let min = world_to_screen(rect, t.x - half, t.y - half, cam_x, cam_y);
        let max = world_to_screen(rect, t.x + half, t.y + half, cam_x, cam_y);
        let r = egui::Rect::from_min_max(min, max);
        painter.rect_filled(r, 0.0, egui::Color32::from_rgb(100, 100, 100));
        painter.rect_stroke(
            r,
            0.0,
            (1.0, egui::Color32::from_rgb(60, 60, 60)),
            egui::StrokeKind::Outside,
        );
    }
}

/// Dessine les projectiles des tours : sprite 2×2 px noir à 600 px/s.
pub(crate) fn paint_tower_projectiles(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
) {
    let (cam_x, cam_y) = camera_center(state);
    let half = TOWER_PROJECTILE_SIZE / 2.0;
    for p in &state.tower_projectiles {
        let min = world_to_screen(rect, p.x - half, p.y - half, cam_x, cam_y);
        let max = world_to_screen(rect, p.x + half, p.y + half, cam_x, cam_y);
        let r = egui::Rect::from_min_max(min, max);
        painter.rect_filled(r, 0.0, egui::Color32::BLACK);
    }
}

/// Barre de vie verte sous une entité (monde → écran).
pub(crate) fn paint_health_bar(
    painter: &egui::Painter,
    rect: egui::Rect,
    world_x: f32,
    world_y: f32,
    entity_half_size: f32,
    hp: i32,
    hp_max: i32,
    camera_x: f32,
    camera_y: f32,
) {
    if hp_max <= 0 {
        return;
    }
    let bar_w = entity_half_size * 2.4f32;
    let bar_h = 4.0;
    let offset_y = entity_half_size + 4.0;
    let min = world_to_screen(
        rect,
        world_x - bar_w / 2.0,
        world_y + offset_y,
        camera_x,
        camera_y,
    );
    let max_bg = world_to_screen(
        rect,
        world_x + bar_w / 2.0,
        world_y + offset_y + bar_h,
        camera_x,
        camera_y,
    );
    let bg_rect = egui::Rect::from_min_max(min, max_bg);
    painter.rect_filled(bg_rect, 0.0, egui::Color32::from_rgb(40, 40, 40));
    let t = (hp as f32 / hp_max as f32).clamp(0.0, 1.0);
    let max_fill = world_to_screen(
        rect,
        world_x - bar_w / 2.0 + bar_w * t,
        world_y + offset_y + bar_h,
        camera_x,
        camera_y,
    );
    let fill_rect = egui::Rect::from_min_max(min, max_fill);
    painter.rect_filled(fill_rect, 0.0, egui::Color32::from_rgb(0, 200, 0));
}

pub(crate) fn paint_health_bars(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let (cam_x, cam_y) = camera_center(state);
    if state.castle.hp < state.castle.hp_max && state.castle.hp > 0 {
        paint_health_bar(
            painter,
            rect,
            state.castle.x,
            state.castle.y,
            size::CASTLE / 2.0,
            state.castle.hp,
            state.castle.hp_max,
            cam_x,
            cam_y,
        );
    }
    if state.player.hp < state.player.hp_max && state.player.hp > 0 {
        paint_health_bar(
            painter,
            rect,
            state.player.x,
            state.player.y,
            Player::half_size(),
            state.player.hp,
            state.player.hp_max,
            cam_x,
            cam_y,
        );
    }
    for e in &state.enemies {
        if e.hp < e.hp_max {
            paint_health_bar(
                painter, rect, e.x, e.y, e.half_size(), e.hp, e.hp_max, cam_x, cam_y,
            );
        }
    }
    for t in &state.towers {
        if t.hp > 0 && t.hp < t.hp_max {
            paint_health_bar(
                painter,
                rect,
                t.x,
                t.y,
                Tower::half_size(),
                t.hp,
                t.hp_max,
                cam_x,
                cam_y,
            );
        }
    }
}

pub(crate) fn paint_loot(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let (cam_x, cam_y) = camera_center(state);
    let pixel = 6.0f32;
    for drop_ in &state.loot_drops {
        let min = world_to_screen(
            rect,
            drop_.x - pixel,
            drop_.y - pixel,
            cam_x,
            cam_y,
        );
        let max = world_to_screen(
            rect,
            drop_.x + pixel,
            drop_.y + pixel,
            cam_x,
            cam_y,
        );
        let r = egui::Rect::from_min_max(min, max);
        let color = match &drop_.kind {
            LootKind::Gold(_) => egui::Color32::from_rgb(255, 215, 0),
            LootKind::Xp(_) => egui::Color32::from_rgb(70, 130, 255),
            LootKind::Item(_) => egui::Color32::from_rgb(139, 90, 43),
        };
        painter.rect_filled(r, 0.0, color);
    }
}

/// Cône d'attaque joueur + cercles de portée ennemis/tours (mode dev).
pub(crate) fn paint_dev_ranges(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
    cursor_world: Option<(f32, f32)>,
) {
    let stroke = 1.5f32;
    let color_circle = egui::Color32::from_rgb(0, 100, 255);
    let color_cone = egui::Color32::from_rgb(0, 150, 255);
    let (cam_x, cam_y) = camera_center(state);
    let px = state.player.x;
    let py = state.player.y;
    let r_player = Player::auto_attack_range();
    let center = world_to_screen(rect, px, py, cam_x, cam_y);
    painter.circle_stroke(center, r_player, (stroke, color_circle));
    let aim_angle_rad = cursor_world.map_or_else(|| state.player.dir.to_angle_rad(), |(cx, cy)| (cy - py).atan2(cx - px));
    let half_cone_rad = 20.0_f32.to_radians();
    let left_angle = aim_angle_rad - half_cone_rad;
    let right_angle = aim_angle_rad + half_cone_rad;
    let tip_left = world_to_screen(
        rect,
        px + r_player * left_angle.cos(),
        py + r_player * left_angle.sin(),
        cam_x,
        cam_y,
    );
    let tip_right = world_to_screen(
        rect,
        px + r_player * right_angle.cos(),
        py + r_player * right_angle.sin(),
        cam_x,
        cam_y,
    );
    painter.line_segment([center, tip_left], (stroke, color_cone));
    painter.line_segment([center, tip_right], (stroke, color_cone));
    if !state.dev_mode {
        return;
    }
    // Zone de commandement (orange, semi-transparent pour visibilité)
    let center_cmd = world_to_screen(rect, px, py, cam_x, cam_y);
    let orange = egui::Color32::from_rgb(255, 165, 0);
    let orange_alpha = egui::Color32::from_rgba_unmultiplied(255, 165, 0, 60);
    painter.circle_filled(center_cmd, COMMAND_ZONE_RADIUS, orange_alpha);
    painter.circle_stroke(center_cmd, COMMAND_ZONE_RADIUS, (stroke + 0.5, orange));
    for e in &state.enemies {
        let center = world_to_screen(rect, e.x, e.y, cam_x, cam_y);
        painter.circle_stroke(
            center,
            ENEMY_VISION_RADIUS,
            (stroke, egui::Color32::from_rgb(255, 0, 0)),
        );
    }
    for t in &state.towers {
        if t.hp <= 0 {
            continue;
        }
        let center = world_to_screen(rect, t.x, t.y, cam_x, cam_y);
        painter.circle_stroke(
            center,
            Tower::range(),
            (stroke, egui::Color32::from_rgb(255, 165, 0)),
        );
    }
}

/// Tick + peinture de la zone de jeu. Retourne (nouvel instant, game_over).
pub(crate) fn game_zone_tick_and_paint(
    rect: egui::Rect,
    painter: &egui::Painter,
    state: &mut GameState,
    last_tick: Instant,
    keys: [bool; 8],
    keys_secondary: [bool; 8],
    _window_fill: egui::Color32,
    cursor_screen: Option<egui::Pos2>,
    player_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize, bool)>,
    enemy_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
    enemy_miniboss_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
    troop_walk_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc, usize)>,
    troop_attack_sprite: Option<(&egui::TextureHandle, &SpritesheetDesc)>,
    construction_mode: bool,
    construction_selected_cell: Option<(i32, i32)>,
) -> (Instant, bool) {
    let grass_fill = egui::Color32::from_rgb(78, 98, 78);
    painter.rect_filled(rect, 0.0, grass_fill);

    let (cam_x, cam_y) = camera_center(state);
    let cursor_world = cursor_screen
        .filter(|p| rect.contains(*p))
        .map(|p| screen_to_world(rect, p.x, p.y, cam_x, cam_y));

    let now = Instant::now();
    if state.phase == GamePhase::Battle {
        let delta = last_tick.elapsed().as_secs_f32();
        if keys[0] {
            move_player(state, Dir8::N, delta);
        }
        if keys[1] {
            move_player(state, Dir8::NE, delta);
        }
        if keys[2] {
            move_player(state, Dir8::E, delta);
        }
        if keys[3] {
            move_player(state, Dir8::SE, delta);
        }
        if keys[4] {
            move_player(state, Dir8::S, delta);
        }
        if keys[5] {
            move_player(state, Dir8::SW, delta);
        }
        if keys[6] {
            move_player(state, Dir8::W, delta);
        }
        if keys[7] {
            move_player(state, Dir8::NW, delta);
        }
        if keys_secondary[0] {
            move_secondary_player(state, Dir8::N, delta);
        }
        if keys_secondary[1] {
            move_secondary_player(state, Dir8::NE, delta);
        }
        if keys_secondary[2] {
            move_secondary_player(state, Dir8::E, delta);
        }
        if keys_secondary[3] {
            move_secondary_player(state, Dir8::SE, delta);
        }
        if keys_secondary[4] {
            move_secondary_player(state, Dir8::S, delta);
        }
        if keys_secondary[5] {
            move_secondary_player(state, Dir8::SW, delta);
        }
        if keys_secondary[6] {
            move_secondary_player(state, Dir8::W, delta);
        }
        if keys_secondary[7] {
            move_secondary_player(state, Dir8::NW, delta);
        }
        tick_battle(state, delta, cursor_world);
    }

    paint_towers(painter, rect, state);
    paint_tower_projectiles(painter, rect, state);
    paint_troops(painter, rect, state, troop_walk_sprite, troop_attack_sprite);
    paint_enemies(painter, rect, state, enemy_sprite, enemy_miniboss_sprite);
    paint_castle(painter, rect, state);
    paint_secondary_player(painter, rect, state);
    paint_secondary_projectiles(painter, rect, state);
    paint_dev_ranges(painter, rect, state, cursor_world);
    paint_player(painter, rect, state, player_sprite);
    paint_health_bars(painter, rect, state);
    paint_loot(painter, rect, state);

    if construction_mode {
        paint_construction_grid(
            painter,
            rect,
            state.castle.x,
            state.castle.y,
            cam_x,
            cam_y,
            construction_selected_cell,
        );
    }

    (now, state.is_game_over())
}
