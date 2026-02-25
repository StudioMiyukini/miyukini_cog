//! @id allumina.prototype.gui
//! @role ui
//! @layer application
//! @domain allumina
//! @do gui_state_and_window_management
//!
//! GuiState — fenêtres déplaçables et HUD Allumina.
//!
//! Fenêtres disponibles (draggable, avec bouton X) :
//!   - Personnage (P) : fiche de stats/caracteristiques
//!   - Competences (C) : arbre de competences (stub)
//!   - Inventaire (I)  : equipement + sac a dos
//!
//! HUD bas : barres PV / Mana / XP avec compteurs numeriques.
//! Barre de navigation haut : boutons Personnage / Competences / Inventaire.

// ── Dimensions des fenêtres ───────────────────────────────────────────────────
pub const CHAR_WIN_W: f32 = 310.0;
pub const CHAR_WIN_H: f32 = 520.0;
pub const SKILLS_WIN_W: f32 = 380.0;
pub const SKILLS_WIN_H: f32 = 620.0;
pub const INV_WIN_W: f32 = 580.0;
pub const INV_WIN_H: f32 = 440.0;
/// Hauteur de la barre de titre (zone de drag + bouton X)
pub const WIN_TITLE_H: f32 = 24.0;
/// Largeur de la zone bouton X dans la barre de titre
pub const WIN_CLOSE_W: f32 = 24.0;

// ── Layout boutons navigation (top bar) ──────────────────────────────────────
pub const BTN_Y: f32 = 8.0;
pub const BTN_H: f32 = 30.0;
pub const BTN_W: f32 = 150.0;
pub const BTN_PERSONNAGE_X: f32 = 8.0;
pub const BTN_COMPETENCES_X: f32 = 166.0;
pub const BTN_INVENTAIRE_X: f32 = 324.0;

// ── Drag state ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum DragTarget {
    Character,
    Skills,
    Inventory,
}

struct DragState {
    target: DragTarget,
    offset_x: f32,
    offset_y: f32,
}

// ── GuiState ──────────────────────────────────────────────────────────────────

pub struct GuiState {
    pub show_character: bool,
    pub show_skills: bool,
    pub show_inventory: bool,
    /// Position top-left de la fenêtre Personnage (écran)
    pub char_pos: (f32, f32),
    /// Position top-left de la fenêtre Competences (écran)
    pub skills_pos: (f32, f32),
    /// Position top-left de la fenêtre Inventaire (écran)
    pub inv_pos: (f32, f32),
    drag: Option<DragState>,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            show_character: false,
            show_skills: false,
            show_inventory: false,
            char_pos: (960.0, 54.0),
            skills_pos: (480.0, 54.0),
            inv_pos: (690.0, 54.0),
            drag: None,
        }
    }
}

impl GuiState {
    pub fn toggle_character(&mut self) {
        self.show_character = !self.show_character;
    }

    pub fn toggle_skills(&mut self) {
        self.show_skills = !self.show_skills;
    }

    pub fn toggle_inventory(&mut self) {
        self.show_inventory = !self.show_inventory;
    }

    // ── Gestion souris ────────────────────────────────────────────────────────

    /// Appeler à chaque `CursorMoved`.
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if let Some(ref drag) = self.drag {
            let nx = (x - drag.offset_x).max(0.0);
            let ny = (y - drag.offset_y).max(0.0);
            match drag.target {
                DragTarget::Character => self.char_pos = (nx, ny),
                DragTarget::Skills    => self.skills_pos = (nx, ny),
                DragTarget::Inventory => self.inv_pos = (nx, ny),
            }
        }
    }

    /// Appeler sur `MouseButton::Left` Pressed.
    /// Retourne `true` si le clic a été consommé par la GUI.
    pub fn handle_mouse_press(&mut self, x: f32, y: f32) -> bool {
        // Vérifier les barres de titre (drag + bouton X) — ordre : inventaire en dernier
        // pour être moins prioritaire que les fenêtres plus petites
        for (visible, target, wx, wy, ww) in [
            (self.show_character, DragTarget::Character, self.char_pos.0, self.char_pos.1, CHAR_WIN_W),
            (self.show_skills,    DragTarget::Skills,    self.skills_pos.0, self.skills_pos.1, SKILLS_WIN_W),
            (self.show_inventory, DragTarget::Inventory, self.inv_pos.0, self.inv_pos.1, INV_WIN_W),
        ] {
            if !visible { continue; }
            if x < wx || x > wx + ww || y < wy || y > wy + WIN_TITLE_H {
                continue;
            }
            // Bouton X (fermeture) — zone droite de la barre de titre
            if x >= wx + ww - WIN_CLOSE_W {
                match target {
                    DragTarget::Character => self.show_character = false,
                    DragTarget::Skills    => self.show_skills = false,
                    DragTarget::Inventory => self.show_inventory = false,
                }
                return true;
            }
            // Zone de drag (reste de la barre de titre)
            self.drag = Some(DragState { target, offset_x: x - wx, offset_y: y - wy });
            return true;
        }

        // Boutons de navigation (barre du haut)
        if y >= BTN_Y && y <= BTN_Y + BTN_H {
            if x >= BTN_PERSONNAGE_X && x <= BTN_PERSONNAGE_X + BTN_W {
                self.toggle_character();
                return true;
            }
            if x >= BTN_COMPETENCES_X && x <= BTN_COMPETENCES_X + BTN_W {
                self.toggle_skills();
                return true;
            }
            if x >= BTN_INVENTAIRE_X && x <= BTN_INVENTAIRE_X + BTN_W {
                self.toggle_inventory();
                return true;
            }
        }

        false
    }

    /// Appeler sur `MouseButton::Left` Released.
    pub fn handle_mouse_release(&mut self) {
        self.drag = None;
    }

    /// True si la souris est actuellement en train de dragger une fenêtre.
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }
}

/// Données joueur passées au renderer pour le HUD et la fenêtre Personnage.
pub struct HudData {
    // ── HP / Mana / Endurance / XP ────────────────────────────────────────────
    pub hp: i32,
    pub hp_max: i32,
    pub mana: i32,
    pub mana_max: i32,
    pub end: i32,
    pub end_max: i32,
    pub xp: u32,
    pub xp_to_next: u32,
    pub level: u8,
    // ── 10 Caractéristiques Allumina (section 1.1 spec) ───────────────────────
    pub for_: u8,
    pub con: u8,
    pub agi: u8,
    pub dex: u8,
    pub per: u8,
    pub vol: u8,
    pub int: u8,
    pub sag: u8,
    pub cha: u8,
    pub luk: u8,
    // ── Aptitudes de combat dérivées (section 4.2 spec) ───────────────────────
    pub atk_apt: i32,
    pub atk_speed_apt: i32,
    pub jet_apt: i32,
    pub esq_apt: i32,
    pub par_apt: i32,
    pub tir_corde_apt: i32,
    pub tir_poing_apt: i32,
    pub tir_epaule_apt: i32,
    pub magie_apt: i32,
    pub cast_speed_apt: i32,
    pub base_dmg: i32,
    // ── Compétences (spec sections 10-11) — liste (catégorie, nom, valeur, cap)
    pub competences: Vec<(&'static str, &'static str, i32, i32)>,
    // ── Économie ──────────────────────────────────────────────────────────────
    pub gold: u32,
    // ── Contexte ──────────────────────────────────────────────────────────────
    pub in_safe_zone: bool,
}
