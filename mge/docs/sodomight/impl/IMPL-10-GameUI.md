# IMPL-10 -- Game UI : HUD, Menus & Interface D2-Style

Guide d'implementation de l'interface Sodomight -- egui 0.28 + wgpu, style Diablo II.

---

## 1. Crate `sd-ui`

### Cargo.toml

```toml
[package]
name = "sd-ui"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.28"
egui-wgpu = "0.28"
egui-winit = "0.28"
wgpu = "0.20"
winit = "0.30"
image = { version = "0.25", default-features = false, features = ["png"] }
serde = { version = "1", features = ["derive"] }
log = "0.4"
```

### Structure

```
sd-ui/src/
|-- lib.rs          -- pub use, UiState
|-- theme.rs        -- Theme D2 (couleurs, polices, espacements)
|-- hud.rs          -- HUD en jeu (orbes, hotbar, belt, minimap)
|-- inventory.rs    -- Ecran inventaire grille 10x4 + slots equipement
|-- character.rs    -- Ecran personnage (stats, apparence)
|-- skill_tree.rs   -- Arbre de competences par classe
|-- menus/
|   |-- mod.rs
|   |-- main_menu.rs       -- Menu principal
|   |-- char_select.rs     -- Selection de personnage
|   |-- lobby_browser.rs   -- Navigation parties multijoueur
|   +-- pause_menu.rs      -- Menu pause en jeu
|-- dialog.rs       -- Boites de dialogue NPC
|-- tooltip.rs      -- Systeme de tooltips (items, skills)
+-- minimap.rs      -- Minimap en jeu
```

---

## 2. Theme D2

```rust
// src/theme.rs
use egui::{Color32, FontId, FontFamily, Rounding, Stroke, Style, Visuals};

/// Palette de couleurs Diablo II
pub struct D2Colors;
impl D2Colors {
    pub const BG_DARK: Color32 = Color32::from_rgb(10, 8, 5);
    pub const PANEL_BG: Color32 = Color32::from_rgb(28, 22, 14);
    pub const PANEL_BORDER: Color32 = Color32::from_rgb(80, 60, 20);
    pub const GOLD: Color32 = Color32::from_rgb(200, 165, 70);
    pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(255, 215, 0);
    pub const RED_LIFE: Color32 = Color32::from_rgb(180, 20, 20);
    pub const BLUE_MANA: Color32 = Color32::from_rgb(20, 40, 180);
    pub const TEXT_NORMAL: Color32 = Color32::from_rgb(200, 190, 160);
    pub const TEXT_MAGIC: Color32 = Color32::from_rgb(100, 100, 255);
    pub const TEXT_RARE: Color32 = Color32::from_rgb(255, 255, 100);
    pub const TEXT_UNIQUE: Color32 = Color32::from_rgb(165, 110, 0);
    pub const TEXT_SET: Color32 = Color32::from_rgb(0, 180, 0);
    pub const TEXT_RUNE_WORD: Color32 = Color32::from_rgb(255, 165, 0);
    pub const SKILL_ACTIVE: Color32 = Color32::from_rgb(255, 200, 50);
    pub const SLOT_EMPTY: Color32 = Color32::from_rgba_premultiplied(40, 30, 15, 200);
    pub const SLOT_HOVER: Color32 = Color32::from_rgba_premultiplied(60, 50, 25, 220);
}

/// Applique le theme D2 au contexte egui
pub fn apply_d2_theme(ctx: &egui::Context) {
    let mut style = Style::default();

    // Fond general
    style.visuals = Visuals {
        dark_mode: true,
        panel_fill: D2Colors::PANEL_BG,
        window_fill: D2Colors::PANEL_BG,
        window_stroke: Stroke::new(2.0, D2Colors::PANEL_BORDER),
        window_rounding: Rounding::same(2.0),
        override_text_color: Some(D2Colors::TEXT_NORMAL),
        ..Visuals::dark()
    };

    // Espacements compacts (UI dense comme D2)
    style.spacing.item_spacing = egui::vec2(4.0, 4.0);
    style.spacing.window_margin = egui::Margin::same(6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);

    ctx.set_style(style);

    // Police bitmap D2-like
    // Note : en production, charger une police TTF bitmap (ex: "ExocetBlizzard" ou similaire libre)
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}

/// Couleur du texte selon la qualite d'un item
pub fn item_quality_color(quality: &str) -> Color32 {
    match quality {
        "magic"     => D2Colors::TEXT_MAGIC,
        "rare"      => D2Colors::TEXT_RARE,
        "unique"    => D2Colors::TEXT_UNIQUE,
        "set"       => D2Colors::TEXT_SET,
        "rune_word" => D2Colors::TEXT_RUNE_WORD,
        _           => D2Colors::TEXT_NORMAL,
    }
}
```

---

## 3. UiState -- etat global de l'UI

```rust
// src/lib.rs
use serde::{Deserialize, Serialize};

/// Quel ecran/panel est ouvert en ce moment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UiScreen {
    MainMenu,
    CharacterSelect,
    LobbyBrowser,
    InGame,
    Paused,
}

/// Panels ouverts en jeu (plusieurs peuvent coexister)
#[derive(Debug, Clone, Default)]
pub struct OpenPanels {
    pub inventory: bool,
    pub character: bool,
    pub skill_tree: bool,
    pub quest_log: bool,
    pub minimap_big: bool,
}

/// Etat global de l'interface
pub struct UiState {
    pub screen: UiScreen,
    pub panels: OpenPanels,
    /// Item survole pour le tooltip (item_id ou None)
    pub hovered_item: Option<String>,
    /// Skill survole pour le tooltip
    pub hovered_skill: Option<String>,
    /// Message d'erreur global (ex : "Inventaire plein")
    pub error_message: Option<(String, f32)>, // (texte, secondes restantes)
    /// Dialogue NPC en cours
    pub npc_dialog: Option<String>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            screen: UiScreen::MainMenu,
            panels: OpenPanels::default(),
            hovered_item: None,
            hovered_skill: None,
            error_message: None,
            npc_dialog: None,
        }
    }

    pub fn toggle_inventory(&mut self) { self.panels.inventory = !self.panels.inventory; }
    pub fn toggle_character(&mut self) { self.panels.character = !self.panels.character; }
    pub fn toggle_skill_tree(&mut self) { self.panels.skill_tree = !self.panels.skill_tree; }

    pub fn show_error(&mut self, msg: &str, duration_secs: f32) {
        self.error_message = Some((msg.to_string(), duration_secs));
    }

    pub fn tick_error(&mut self, dt: f32) {
        if let Some((_, ref mut t)) = self.error_message {
            *t -= dt;
            if *t <= 0.0 { self.error_message = None; }
        }
    }
}

pub use theme::{apply_d2_theme, D2Colors, item_quality_color};
pub use hud::draw_hud;
pub use inventory::draw_inventory;
pub use character::draw_character_panel;
pub use skill_tree::draw_skill_tree;
pub use menus::main_menu::draw_main_menu;
pub use menus::char_select::draw_char_select;
pub use menus::lobby_browser::draw_lobby_browser;
pub use menus::pause_menu::draw_pause_menu;
pub use dialog::draw_npc_dialog;
pub use tooltip::draw_tooltip;

mod theme;
mod hud;
mod inventory;
mod character;
mod skill_tree;
pub mod menus;
mod dialog;
mod tooltip;
mod minimap;
```

---

## 4. HUD en jeu

```rust
// src/hud.rs
use egui::{Context, Pos2, Vec2, Rect, Color32};
use crate::theme::D2Colors;

/// Donnees du joueur pour le HUD
pub struct HudData<'a> {
    pub life_cur: i32,
    pub life_max: i32,
    pub mana_cur: i32,
    pub mana_max: i32,
    pub level: i32,
    pub experience_pct: f32,    // 0.0..1.0
    pub gold: i64,
    pub skill_slots: &'a [SkillSlotData; 8],
    pub belt_slots: &'a [BeltSlotData; 4],
}

#[derive(Default, Clone)]
pub struct SkillSlotData {
    pub skill_id: Option<String>,
    pub icon_name: Option<String>,
    pub cooldown_pct: f32,  // 0.0 = pret, 1.0 = en CD
    pub mana_cost: i32,
}

#[derive(Default, Clone)]
pub struct BeltSlotData {
    pub item_id: Option<String>,
    pub icon_name: Option<String>,
    pub quantity: u32,
}

/// Dessine le HUD D2 complet sur la fenetre
pub fn draw_hud(ctx: &Context, data: &HudData<'_>, screen_w: f32, screen_h: f32) {
    // ---------------------------------------------------------------
    // Barre du bas (panneau principal HUD)
    // ---------------------------------------------------------------
    egui::Area::new("hud_bottom".into())
        .fixed_pos(Pos2::new(0.0, screen_h - 80.0))
        .show(ctx, |ui| {
            ui.set_width(screen_w);

            // Fond du HUD
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, Color32::from_rgba_premultiplied(10, 8, 5, 230));
            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

            ui.horizontal(|ui| {
                // Orbe Vie (gauche)
                draw_orb(ui, data.life_cur, data.life_max, D2Colors::RED_LIFE, "Vie");

                ui.add_space(8.0);

                // Hotbar competences (centre)
                draw_skill_hotbar(ui, data.skill_slots, data.mana_cur);

                ui.add_space(8.0);

                // Belt (potions rapides)
                draw_belt(ui, data.belt_slots);

                ui.add_space(8.0);

                // Orbe Mana (droite)
                draw_orb(ui, data.mana_cur, data.mana_max, D2Colors::BLUE_MANA, "Mana");
            });
        });

    // Barre d'experience (tout en bas)
    egui::Area::new("hud_xp".into())
        .fixed_pos(Pos2::new(0.0, screen_h - 6.0))
        .show(ctx, |ui| {
            let bar_rect = Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(screen_w * data.experience_pct, 5.0),
            );
            ui.painter().rect_filled(bar_rect, 0.0, D2Colors::GOLD);
        });

    // Niveau (coin bas gauche)
    egui::Area::new("hud_level".into())
        .fixed_pos(Pos2::new(6.0, screen_h - 18.0))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("Nv {}", data.level))
                    .color(D2Colors::GOLD)
                    .size(11.0)
            );
        });

    // Or (coin bas droit)
    egui::Area::new("hud_gold".into())
        .fixed_pos(Pos2::new(screen_w - 120.0, screen_h - 18.0))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("Or : {}", data.gold))
                    .color(D2Colors::GOLD)
                    .size(11.0)
            );
        });
}

fn draw_orb(ui: &mut egui::Ui, cur: i32, max: i32, color: Color32, label: &str) {
    let orb_size = Vec2::new(48.0, 64.0);
    let (resp, painter) = ui.allocate_painter(orb_size, egui::Sense::hover());
    let rect = resp.rect;

    // Fond de l'orbe
    painter.rect_filled(rect, egui::Rounding::same(24.0), Color32::from_rgb(20, 15, 10));

    // Remplissage proportionnel (bas vers haut)
    let pct = (cur as f32 / max.max(1) as f32).clamp(0.0, 1.0);
    let fill_h = orb_size.y * pct;
    let fill_rect = Rect::from_min_size(
        Pos2::new(rect.min.x, rect.max.y - fill_h),
        Vec2::new(orb_size.x, fill_h),
    );
    painter.rect_filled(fill_rect, egui::Rounding::same(24.0), color);

    // Bordure
    painter.rect_stroke(rect, egui::Rounding::same(24.0), egui::Stroke::new(1.5, D2Colors::PANEL_BORDER));

    // Valeur
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}/{}", cur, max),
        egui::FontId::proportional(10.0),
        Color32::WHITE,
    );

    // Tooltip au survol
    if resp.hovered() {
        egui::show_tooltip_text(ui.ctx(), egui::Id::new(label), format!("{} : {}/{}", label, cur, max));
    }
}

fn draw_skill_hotbar(ui: &mut egui::Ui, slots: &[SkillSlotData; 8], mana_cur: i32) {
    let slot_size = Vec2::new(40.0, 40.0);
    ui.horizontal(|ui| {
        for (i, slot) in slots.iter().enumerate() {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::click());
            let rect = resp.rect;

            // Fond du slot
            let bg = if resp.hovered() { D2Colors::SLOT_HOVER } else { D2Colors::SLOT_EMPTY };
            painter.rect_filled(rect, 3.0, bg);
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

            if let Some(ref _icon) = slot.icon_name {
                // En production : dessiner l'icone depuis l'atlas
                // Pour l'instant : placeholder couleur
                painter.rect_filled(rect.shrink(4.0), 2.0, Color32::from_rgb(80, 80, 120));
            }

            // Cooldown overlay
            if slot.cooldown_pct > 0.0 {
                let cd_rect = Rect::from_min_size(
                    rect.min,
                    Vec2::new(slot_size.x, slot_size.y * slot.cooldown_pct),
                );
                painter.rect_filled(cd_rect, 3.0, Color32::from_rgba_premultiplied(0, 0, 0, 160));
            }

            // Numero de slot (coin inferieur gauche)
            painter.text(
                Pos2::new(rect.min.x + 2.0, rect.max.y - 2.0),
                egui::Align2::LEFT_BOTTOM,
                (i + 1).to_string(),
                egui::FontId::proportional(9.0),
                D2Colors::GOLD,
            );

            // Cout mana (coin inferieur droit, rouge si insuffisant)
            if slot.mana_cost > 0 {
                let cost_color = if mana_cur >= slot.mana_cost { D2Colors::BLUE_MANA } else { D2Colors::RED_LIFE };
                painter.text(
                    Pos2::new(rect.max.x - 2.0, rect.max.y - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    slot.mana_cost.to_string(),
                    egui::FontId::proportional(9.0),
                    cost_color,
                );
            }
        }
    });
}

fn draw_belt(ui: &mut egui::Ui, slots: &[BeltSlotData; 4]) {
    let slot_size = Vec2::new(36.0, 36.0);
    ui.vertical(|ui| {
        for (i, slot) in slots.iter().enumerate() {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::click());
            let rect = resp.rect;

            let bg = if resp.hovered() { D2Colors::SLOT_HOVER } else { D2Colors::SLOT_EMPTY };
            painter.rect_filled(rect, 2.0, bg);
            painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

            // Quantite de potions
            if slot.quantity > 0 {
                // Placeholder : couleur selon type
                painter.rect_filled(rect.shrink(5.0), 2.0, Color32::from_rgb(160, 30, 30));
                painter.text(
                    rect.center_bottom() - Vec2::new(0.0, 2.0),
                    egui::Align2::CENTER_BOTTOM,
                    slot.quantity.to_string(),
                    egui::FontId::proportional(9.0),
                    Color32::WHITE,
                );
            }

            // Touche (5/6/7/8)
            painter.text(
                Pos2::new(rect.min.x + 2.0, rect.min.y + 2.0),
                egui::Align2::LEFT_TOP,
                (5 + i).to_string(),
                egui::FontId::proportional(8.0),
                D2Colors::GOLD,
            );
        }
    });
}
```

---

## 5. Inventaire grille 10x4

```rust
// src/inventory.rs
use egui::{Context, Vec2, Color32, Pos2};
use crate::theme::{D2Colors, item_quality_color};

/// Un item dans l'inventaire
#[derive(Debug, Clone)]
pub struct UiItem {
    pub id: String,
    pub name: String,
    pub quality: String,
    pub width: u32,  // en cellules (1..4)
    pub height: u32, // en cellules (1..4)
    pub grid_x: i32,
    pub grid_y: i32,
    pub icon_name: String,
    pub is_identified: bool,
}

/// Slots d'equipement du personnage
#[derive(Default)]
pub struct EquipSlots {
    pub head: Option<UiItem>,
    pub chest: Option<UiItem>,
    pub belt: Option<UiItem>,
    pub boots: Option<UiItem>,
    pub gloves: Option<UiItem>,
    pub amulet: Option<UiItem>,
    pub ring_left: Option<UiItem>,
    pub ring_right: Option<UiItem>,
    pub main_hand: Option<UiItem>,
    pub off_hand: Option<UiItem>,
}

pub const GRID_COLS: usize = 10;
pub const GRID_ROWS: usize = 4;
pub const CELL_SIZE: f32 = 28.0;

/// Dessine le panneau inventaire complet
pub fn draw_inventory(
    ctx: &Context,
    is_open: &mut bool,
    items: &[UiItem],
    equip: &EquipSlots,
    gold: i64,
) {
    if !*is_open { return; }

    egui::Window::new("Inventaire")
        .resizable(false)
        .collapsible(false)
        .title_bar(true)
        .open(is_open)
        .default_pos([400.0, 50.0])
        .show(ctx, |ui| {
            ui.set_min_width(340.0);

            // Slots d'equipement (silhouette personnage style D2)
            ui.group(|ui| {
                ui.label(egui::RichText::new("Equipement").color(D2Colors::GOLD).size(12.0));
                draw_equip_slots(ui, equip);
            });

            ui.separator();

            // Grille inventaire 10x4
            ui.group(|ui| {
                ui.label(egui::RichText::new("Sac").color(D2Colors::GOLD).size(12.0));
                draw_grid(ui, items);
            });

            ui.separator();
            ui.label(
                egui::RichText::new(format!("Or : {}", gold))
                    .color(D2Colors::GOLD_BRIGHT)
                    .size(11.0)
            );
        });
}

fn draw_grid(ui: &mut egui::Ui, items: &[UiItem]) {
    let grid_w = GRID_COLS as f32 * CELL_SIZE;
    let grid_h = GRID_ROWS as f32 * CELL_SIZE;

    let (resp, painter) = ui.allocate_painter(Vec2::new(grid_w, grid_h), egui::Sense::click());
    let origin = resp.rect.min;

    // Fond de la grille
    painter.rect_filled(resp.rect, 2.0, Color32::from_rgb(15, 12, 8));

    // Lignes de grille
    for col in 0..=GRID_COLS {
        let x = origin.x + col as f32 * CELL_SIZE;
        painter.line_segment(
            [Pos2::new(x, origin.y), Pos2::new(x, origin.y + grid_h)],
            egui::Stroke::new(0.5, Color32::from_rgb(40, 30, 15)),
        );
    }
    for row in 0..=GRID_ROWS {
        let y = origin.y + row as f32 * CELL_SIZE;
        painter.line_segment(
            [Pos2::new(origin.x, y), Pos2::new(origin.x + grid_w, y)],
            egui::Stroke::new(0.5, Color32::from_rgb(40, 30, 15)),
        );
    }

    // Items dans la grille
    for item in items {
        if item.grid_x < 0 || item.grid_y < 0 { continue; }
        let x = origin.x + item.grid_x as f32 * CELL_SIZE;
        let y = origin.y + item.grid_y as f32 * CELL_SIZE;
        let w = item.width as f32 * CELL_SIZE;
        let h = item.height as f32 * CELL_SIZE;
        let rect = egui::Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h));

        let border_color = item_quality_color(&item.quality);
        painter.rect_filled(rect, 2.0, Color32::from_rgba_premultiplied(30, 25, 15, 200));
        painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border_color));

        // Nom abrege
        if !item.is_identified {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "?",
                egui::FontId::proportional(10.0),
                D2Colors::TEXT_NORMAL,
            );
        } else {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                item.name.chars().take(4).collect::<String>(),
                egui::FontId::proportional(9.0),
                border_color,
            );
        }
    }
}

fn draw_equip_slots(ui: &mut egui::Ui, equip: &EquipSlots) {
    let slot_size = Vec2::new(38.0, 38.0);
    // Layout simplifie : ligne horizontale des slots principaux
    ui.horizontal(|ui| {
        for (label, item) in [
            ("Tete",  &equip.head),
            ("Corps", &equip.chest),
            ("Ceinture", &equip.belt),
            ("Bottes", &equip.boots),
            ("Gants", &equip.gloves),
        ] {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::hover());
            let rect = resp.rect;
            let bg = if resp.hovered() { D2Colors::SLOT_HOVER } else { D2Colors::SLOT_EMPTY };
            painter.rect_filled(rect, 3.0, bg);
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));
            if let Some(it) = item {
                painter.rect_filled(rect.shrink(4.0), 2.0, Color32::from_rgb(50, 40, 20));
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    it.name.chars().take(4).collect::<String>(),
                    egui::FontId::proportional(8.0),
                    item_quality_color(&it.quality),
                );
            } else {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(8.0),
                    Color32::from_rgb(80, 70, 50),
                );
            }
        }
    });
    ui.horizontal(|ui| {
        for (label, item) in [
            ("Main G", &equip.main_hand),
            ("Amulette", &equip.amulet),
            ("Anneau G", &equip.ring_left),
            ("Anneau D", &equip.ring_right),
            ("Main D",  &equip.off_hand),
        ] {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::hover());
            let rect = resp.rect;
            let bg = if resp.hovered() { D2Colors::SLOT_HOVER } else { D2Colors::SLOT_EMPTY };
            painter.rect_filled(rect, 3.0, bg);
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));
            if let Some(it) = item {
                painter.rect_filled(rect.shrink(4.0), 2.0, Color32::from_rgb(50, 40, 20));
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    it.name.chars().take(4).collect::<String>(),
                    egui::FontId::proportional(8.0),
                    item_quality_color(&it.quality),
                );
            } else {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(8.0),
                    Color32::from_rgb(80, 70, 50),
                );
            }
        }
    });
}
```

---

## 6. Panneau Personnage

```rust
// src/character.rs
use egui::Context;
use crate::theme::D2Colors;

pub struct CharacterData {
    pub name: String,
    pub class: String,
    pub level: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
    pub unspent_points: i32,
    pub life_base: i32,
    pub mana_base: i32,
    pub defense: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub attack_rating: i32,
    pub fire_res: i32,
    pub cold_res: i32,
    pub lightning_res: i32,
    pub poison_res: i32,
}

pub fn draw_character_panel(ctx: &Context, is_open: &mut bool, data: &CharacterData) {
    if !*is_open { return; }

    egui::Window::new("Personnage")
        .resizable(false)
        .collapsible(false)
        .open(is_open)
        .default_pos([20.0, 50.0])
        .min_width(220.0)
        .show(ctx, |ui| {
            // En-tete
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(&data.name).color(D2Colors::GOLD_BRIGHT).size(14.0));
                ui.label(egui::RichText::new(format!("{} - Nv {}", data.class, data.level))
                    .color(D2Colors::TEXT_NORMAL).size(11.0));
            });
            ui.separator();

            // Points non depenses
            if data.unspent_points > 0 {
                ui.label(egui::RichText::new(format!("Points a distribuer : {}", data.unspent_points))
                    .color(D2Colors::SKILL_ACTIVE).size(11.0));
                ui.separator();
            }

            // Stats de base
            ui.label(egui::RichText::new("-- Stats de base --").color(D2Colors::GOLD).size(11.0));
            egui::Grid::new("base_stats").num_columns(3).show(ui, |ui| {
                stat_row(ui, "Force", data.strength, data.unspent_points > 0);
                stat_row(ui, "Dexterite", data.dexterity, data.unspent_points > 0);
                stat_row(ui, "Vitalite", data.vitality, data.unspent_points > 0);
                stat_row(ui, "Energie", data.energy, data.unspent_points > 0);
            });
            ui.separator();

            // Stats derivees
            ui.label(egui::RichText::new("-- Stats derivees --").color(D2Colors::GOLD).size(11.0));
            egui::Grid::new("derived_stats").num_columns(2).show(ui, |ui| {
                derived_stat(ui, "Vie", data.life_base);
                derived_stat(ui, "Mana", data.mana_base);
                derived_stat(ui, "Defense", data.defense);
                derived_stat(ui, "Attaque", data.attack_rating);
                derived_stat(ui, format!("Degats {}-{}", data.damage_min, data.damage_max).as_str(), 0);
            });
            ui.separator();

            // Resistances
            ui.label(egui::RichText::new("-- Resistances --").color(D2Colors::GOLD).size(11.0));
            egui::Grid::new("res_stats").num_columns(2).show(ui, |ui| {
                res_row(ui, "Feu", data.fire_res);
                res_row(ui, "Froid", data.cold_res);
                res_row(ui, "Foudre", data.lightning_res);
                res_row(ui, "Poison", data.poison_res);
            });
        });
}

fn stat_row(ui: &mut egui::Ui, label: &str, value: i32, can_add: bool) {
    ui.label(egui::RichText::new(label).color(D2Colors::TEXT_NORMAL).size(11.0));
    ui.label(egui::RichText::new(value.to_string()).color(D2Colors::GOLD).size(11.0));
    if can_add {
        if ui.small_button("+").clicked() {
            // Signal vers le jeu : ajouter 1 point a ce stat
        }
    } else {
        ui.label("");
    }
    ui.end_row();
}

fn derived_stat(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(egui::RichText::new(label).color(D2Colors::TEXT_NORMAL).size(11.0));
    if value > 0 {
        ui.label(egui::RichText::new(value.to_string()).color(D2Colors::GOLD).size(11.0));
    }
    ui.end_row();
}

fn res_row(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(egui::RichText::new(label).color(D2Colors::TEXT_NORMAL).size(11.0));
    let color = if value >= 75 { D2Colors::GOLD_BRIGHT }
        else if value >= 0 { D2Colors::GOLD }
        else { D2Colors::RED_LIFE };
    ui.label(egui::RichText::new(format!("{}%", value)).color(color).size(11.0));
    ui.end_row();
}
```

---

## 7. Tooltip

```rust
// src/tooltip.rs
use egui::{Context, Pos2};
use crate::theme::{D2Colors, item_quality_color};

pub struct ItemTooltipData {
    pub name: String,
    pub quality: String,
    pub base_type: String,
    pub item_level: u32,
    pub required_level: u32,
    pub required_strength: Option<u32>,
    pub required_dexterity: Option<u32>,
    pub properties: Vec<String>, // ex: "+20% Vitesse d'attaque"
    pub affixes: Vec<(String, f32)>, // (affix_label, value)
    pub durability: Option<(u32, u32)>, // (cur, max)
}

pub fn draw_tooltip(ctx: &Context, pos: Pos2, data: &ItemTooltipData) {
    egui::Area::new("item_tooltip".into())
        .fixed_pos(pos + egui::vec2(16.0, 0.0))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(220.0);

                // Nom de l'item
                ui.label(
                    egui::RichText::new(&data.name)
                        .color(item_quality_color(&data.quality))
                        .size(13.0)
                        .strong()
                );

                // Type de base
                ui.label(
                    egui::RichText::new(&data.base_type)
                        .color(D2Colors::TEXT_NORMAL)
                        .size(11.0)
                );

                ui.separator();

                // Proprietes magiques
                for prop in &data.properties {
                    ui.label(
                        egui::RichText::new(prop)
                            .color(D2Colors::TEXT_MAGIC)
                            .size(11.0)
                    );
                }

                // Affixes
                for (label, _val) in &data.affixes {
                    ui.label(
                        egui::RichText::new(label)
                            .color(D2Colors::TEXT_MAGIC)
                            .size(10.0)
                    );
                }

                ui.separator();

                // Durabilite
                if let Some((cur, max)) = data.durability {
                    ui.label(
                        egui::RichText::new(format!("Durabilite : {}/{}", cur, max))
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0)
                    );
                }

                // Niveau requis
                if data.required_level > 0 {
                    ui.label(
                        egui::RichText::new(format!("Niveau requis : {}", data.required_level))
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0)
                    );
                }

                // Stats requises
                if let Some(req_str) = data.required_strength {
                    ui.label(
                        egui::RichText::new(format!("Force requise : {}", req_str))
                            .color(D2Colors::TEXT_NORMAL)
                            .size(10.0)
                    );
                }
            });
        });
}
```

---

## 8. Menus principaux

```rust
// src/menus/main_menu.rs
use egui::Context;
use crate::theme::D2Colors;

pub enum MainMenuAction {
    NewGame,
    LoadGame,
    Multiplayer,
    Options,
    Quit,
}

pub fn draw_main_menu(ctx: &Context) -> Option<MainMenuAction> {
    let mut action = None;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(D2Colors::BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);

                ui.label(
                    egui::RichText::new("SODOMIGHT")
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(48.0)
                        .strong()
                );
                ui.label(
                    egui::RichText::new("Dark is the Night")
                        .color(D2Colors::TEXT_NORMAL)
                        .size(16.0)
                );

                ui.add_space(60.0);

                let btn_size = egui::vec2(200.0, 36.0);
                let buttons: &[(&str, MainMenuAction)] = &[
                    ("Nouvelle partie",  MainMenuAction::NewGame),
                    ("Charger partie",   MainMenuAction::LoadGame),
                    ("Multijoueur",      MainMenuAction::Multiplayer),
                    ("Options",          MainMenuAction::Options),
                    ("Quitter",          MainMenuAction::Quit),
                ];
                for (label, act) in buttons.iter() {
                    let btn = egui::Button::new(
                        egui::RichText::new(*label).color(D2Colors::GOLD).size(14.0)
                    )
                    .min_size(btn_size)
                    .fill(D2Colors::PANEL_BG)
                    .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                    if ui.add(btn).clicked() {
                        action = Some(match act {
                            MainMenuAction::NewGame    => MainMenuAction::NewGame,
                            MainMenuAction::LoadGame   => MainMenuAction::LoadGame,
                            MainMenuAction::Multiplayer => MainMenuAction::Multiplayer,
                            MainMenuAction::Options    => MainMenuAction::Options,
                            MainMenuAction::Quit       => MainMenuAction::Quit,
                        });
                    }
                    ui.add_space(8.0);
                }
            });
        });

    action
}
```

```rust
// src/menus/pause_menu.rs
use egui::Context;
use crate::theme::D2Colors;

pub enum PauseAction { Resume, Options, SaveAndQuit }

pub fn draw_pause_menu(ctx: &Context) -> Option<PauseAction> {
    let mut action = None;

    egui::Window::new("Pause")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(180.0);
            ui.vertical_centered(|ui| {
                let buttons: &[(&str, PauseAction)] = &[
                    ("Reprendre",       PauseAction::Resume),
                    ("Options",         PauseAction::Options),
                    ("Sauvegarder & Quitter", PauseAction::SaveAndQuit),
                ];
                for (label, act) in buttons.iter() {
                    let btn = egui::Button::new(
                        egui::RichText::new(*label).color(D2Colors::GOLD).size(13.0)
                    )
                    .min_size(egui::vec2(160.0, 30.0))
                    .fill(D2Colors::PANEL_BG)
                    .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                    if ui.add(btn).clicked() {
                        action = Some(match act {
                            PauseAction::Resume       => PauseAction::Resume,
                            PauseAction::Options      => PauseAction::Options,
                            PauseAction::SaveAndQuit  => PauseAction::SaveAndQuit,
                        });
                    }
                    ui.add_space(4.0);
                }
            });
        });

    action
}
```

---

## 9. Dialogue NPC

```rust
// src/dialog.rs
use egui::Context;
use crate::theme::D2Colors;

pub fn draw_npc_dialog(ctx: &Context, npc_name: &str, text: &str, is_open: &mut bool) {
    if !*is_open { return; }

    egui::Window::new(npc_name)
        .resizable(false)
        .collapsible(false)
        .open(is_open)
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -90.0])
        .min_width(400.0)
        .max_width(600.0)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .color(D2Colors::TEXT_NORMAL)
                    .size(12.0)
            );
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(egui::RichText::new("Fermer").color(D2Colors::GOLD)).clicked() {
                    *is_open = false;
                }
            });
        });
}
```

---

## 10. Integration egui + wgpu dans sd-client

```rust
// sd-client/src/ui_integration.rs
use egui_winit::State as EguiWinitState;
use egui_wgpu::Renderer as EguiRenderer;
use egui::Context as EguiContext;
use winit::window::Window;
use wgpu::{Device, Queue, SurfaceConfiguration, TextureFormat};

pub struct UiIntegration {
    pub ctx: EguiContext,
    pub state: EguiWinitState,
    pub renderer: EguiRenderer,
}

impl UiIntegration {
    pub fn new(
        device: &Device,
        surface_format: TextureFormat,
        window: &Window,
    ) -> Self {
        let ctx = EguiContext::default();
        sd_ui::apply_d2_theme(&ctx);

        let state = EguiWinitState::new(
            ctx.clone(),
            ctx.viewport_id(),
            window,
            None,
            None,
        );

        let renderer = EguiRenderer::new(device, surface_format, None, 1, false);

        Self { ctx, state, renderer }
    }

    /// Appeler pour chaque WindowEvent winit
    pub fn on_window_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        self.state.on_window_event(window, event).consumed
    }

    /// Appeler au debut de chaque frame
    pub fn begin_frame(&mut self, window: &Window) {
        let raw_input = self.state.take_egui_input(window);
        self.ctx.begin_frame(raw_input);
    }

    /// Appeler a la fin de chaque frame (apres le code UI)
    pub fn end_frame_and_render(
        &mut self,
        window: &Window,
        device: &Device,
        queue: &Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        config: &SurfaceConfiguration,
    ) {
        let full_output = self.ctx.end_frame();
        let paint_jobs = self.ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        let screen_desc = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer.update_texture(device, queue, *id, image_delta);
        }
        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        self.renderer.update_buffers(device, queue, encoder, &paint_jobs, &screen_desc);

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // ne pas effacer, superposer sur le jeu
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            self.renderer.render(&mut rpass, &paint_jobs, &screen_desc);
        }

        self.state.handle_platform_output(window, full_output.platform_output);
    }
}
```

---

## 11. Boucle frame complete (sd-client)

```rust
// Pseudo-code frame complete dans sd-client/src/main.rs

// -- BEGIN FRAME --
ui.begin_frame(&window);

// 1. Jeu (wgpu renderer)
render_game_world(&ctx, &mut batch, &atlas, &tiles, &entities, camera, &dual_res);

// 2. UI en jeu (egui)
match ui_state.screen {
    UiScreen::InGame => {
        draw_hud(&ui.ctx, &hud_data, screen_w, screen_h);
        draw_inventory(&ui.ctx, &mut ui_state.panels.inventory, &items, &equip, gold);
        draw_character_panel(&ui.ctx, &mut ui_state.panels.character, &char_data);
        if let Some(ref dialog_text) = ui_state.npc_dialog.clone() {
            let mut open = true;
            draw_npc_dialog(&ui.ctx, "Akara", &dialog_text, &mut open);
            if !open { ui_state.npc_dialog = None; }
        }
    }
    UiScreen::MainMenu => {
        if let Some(action) = draw_main_menu(&ui.ctx) {
            match action {
                MainMenuAction::Quit => elwt.exit(),
                _ => {}
            }
        }
    }
    UiScreen::Paused => {
        if let Some(action) = draw_pause_menu(&ui.ctx) {
            match action {
                PauseAction::Resume => ui_state.screen = UiScreen::InGame,
                _ => {}
            }
        }
    }
    _ => {}
}

// 3. Rendu egui sur la surface wgpu
ui.end_frame_and_render(&window, &device, &queue, &mut encoder, &view, &config);
// -- END FRAME --
```

---

## 12. Checklist integration

- [ ] `sd-ui` ajout au workspace `Cargo.toml`
- [ ] `egui-wgpu`, `egui-winit`, `egui` coherents en version 0.28
- [ ] `apply_d2_theme()` appele au demarrage -- fond sombre, couleurs D2
- [ ] `UiIntegration::new()` cree sans panique
- [ ] `begin_frame()` / `end_frame_and_render()` appeles chaque frame
- [ ] `on_window_event()` appele avant `InputProcessor::process()` -- UI consomme en priorite
- [ ] HUD visible : orbes vie/mana, hotbar 8 slots, belt 4 slots, barre XP
- [ ] Touche `I` ouvre/ferme le panneau Inventaire
- [ ] Touche `C` ouvre/ferme le panneau Personnage
- [ ] Tooltip apparait au survol d'un item dans la grille
- [ ] Menu principal affiche les 5 boutons style D2
- [ ] `cargo build -p sd-ui` : aucune erreur de compilation
- [ ] `cargo clippy -p sd-ui -- -D warnings` : aucun warning

---

*Fin IMPL-10 -- Game UI D2-style. Documentation d'implementation Sodomight complete : IMPL-01 a IMPL-10.*
