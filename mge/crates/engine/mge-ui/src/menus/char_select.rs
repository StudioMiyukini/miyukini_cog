//! Character selection screen.
//!
//! Displays a list of existing characters with class, level, and a "Create"
//! button. Supports up to 8 character slots.

use egui::Context;

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// Maximum number of character slots.
const MAX_CHAR_SLOTS: usize = 8;

/// Minimum character name length.
const MIN_NAME_LEN: usize = 2;
/// Maximum character name length.
const MAX_NAME_LEN: usize = 24;

/// A single character slot in the selection screen.
#[derive(Debug, Clone)]
pub struct CharacterSlot {
    /// Character display name.
    pub name: String,
    /// Character class (e.g. "Necromancer").
    pub class: String,
    /// Current character level.
    pub level: u32,
}

/// State for the character selection screen.
#[derive(Debug, Clone)]
pub struct CharSelectState {
    /// Existing character slots.
    pub slots: Vec<CharacterSlot>,
    /// Maximum allowed slots.
    pub max_slots: usize,
}

impl CharSelectState {
    /// Create a new empty character selection state with the default cap of 8.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            max_slots: MAX_CHAR_SLOTS,
        }
    }

    /// Whether the player can create another character.
    pub fn can_create(&self) -> bool {
        self.slots.len() < self.max_slots
    }
}

impl Default for CharSelectState {
    fn default() -> Self {
        Self::new()
    }
}

/// State for the character creation form.
#[derive(Debug, Clone)]
pub struct CreateCharState {
    /// Player-chosen character name.
    pub name: String,
    /// Currently selected class.
    pub selected_class: String,
    /// Classes available for creation in the current season.
    pub available_classes: Vec<String>,
}

impl CreateCharState {
    /// Create a new character-creation state.
    ///
    /// Season 3 only allows the Necromancer class.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            selected_class: String::from("Necromancer"),
            available_classes: vec![String::from("Necromancer")],
        }
    }

    /// Validate the chosen character name.
    ///
    /// # Errors
    ///
    /// Returns an error description if the name is too short, too long,
    /// or contains forbidden characters (HTML angle brackets).
    pub fn validate_name(&self) -> Result<(), String> {
        let trimmed = self.name.trim();

        if trimmed.len() < MIN_NAME_LEN {
            return Err(format!(
                "Name must be at least {MIN_NAME_LEN} characters"
            ));
        }

        if trimmed.len() > MAX_NAME_LEN {
            return Err(format!(
                "Name must be at most {MAX_NAME_LEN} characters"
            ));
        }

        if trimmed.contains('<') || trimmed.contains('>') {
            return Err("Name must not contain HTML characters".to_string());
        }

        Ok(())
    }
}

impl Default for CreateCharState {
    fn default() -> Self {
        Self::new()
    }
}

/// Draw the character selection screen (stub).
///
/// Returns `true` if the player selected a character and wants to proceed.
pub fn draw_char_select(ctx: &Context) -> bool {
    let mut selected = false;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(D2Colors::BG_DARK))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(
                    egui::RichText::new("Selection de personnage")
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(24.0)
                        .strong(),
                );
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("(A implementer -- liste des personnages)")
                        .color(D2Colors::TEXT_NORMAL)
                        .size(12.0),
                );
                ui.add_space(20.0);

                let btn = egui::Button::new(
                    egui::RichText::new("Entrer en jeu")
                        .color(D2Colors::GOLD)
                        .size(14.0),
                )
                .min_size(egui::vec2(180.0, 32.0))
                .fill(D2Colors::PANEL_BG)
                .stroke(egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

                if ui.add(btn).clicked() {
                    selected = true;
                }
            });
        });

    selected
}
