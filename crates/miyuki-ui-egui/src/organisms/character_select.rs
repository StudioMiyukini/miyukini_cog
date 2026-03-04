// @id: MUIE-CharSelect @do: character-select-organism @role: organism @layer: 6 @human: miyuk

//! Character selection screen organism.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::{D2Button, D2ButtonVariant};
use crate::convert::rgba_to_color32;

/// Character slot data for the selection screen.
#[derive(Debug, Clone)]
pub struct CharSlotData {
    /// Character name.
    pub name: String,
    /// Class name.
    pub class: String,
    /// Level.
    pub level: i32,
    /// Whether this is an expansion (LoD) character.
    pub expansion: bool,
}

/// The character selection organism.
pub struct CharacterSelect;

impl CharacterSelect {
    /// Draw the character selection screen.
    ///
    /// Returns the index of the selected character, or `None`.
    pub fn show(ctx: &Context, characters: &[CharSlotData], selected: Option<usize>) -> CharSelectAction {
        let mut action = CharSelectAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let gold = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new("Selection du personnage").color(gold).size(18.0));
                ui.separator();

                for (i, character) in characters.iter().enumerate() {
                    let is_selected = selected == Some(i);
                    let text_color = if is_selected {
                        rgba_to_color32(&D2_PALETTE.text_high)
                    } else {
                        rgba_to_color32(&D2_PALETTE.text_primary)
                    };

                    let label = format!(
                        "{} - {} (Nv {})",
                        character.name, character.class, character.level
                    );

                    let resp = ui.add(
                        egui::Label::new(
                            egui::RichText::new(&label).color(text_color).size(13.0),
                        )
                        .sense(egui::Sense::click()),
                    );

                    if resp.clicked() {
                        action = CharSelectAction::Select(i);
                    }
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if D2Button::new("Jouer").show(ui).clicked() {
                        if let Some(idx) = selected {
                            action = CharSelectAction::Play(idx);
                        }
                    }
                    if D2Button::new("Nouveau")
                        .variant(D2ButtonVariant::Secondary)
                        .show(ui)
                        .clicked()
                    {
                        action = CharSelectAction::Create;
                    }
                    if D2Button::new("Supprimer")
                        .variant(D2ButtonVariant::Secondary)
                        .show(ui)
                        .clicked()
                    {
                        if let Some(idx) = selected {
                            action = CharSelectAction::Delete(idx);
                        }
                    }
                });
            });
        });

        action
    }
}

/// Action from the character selection screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSelectAction {
    /// No action.
    None,
    /// A character slot was selected.
    Select(usize),
    /// Play with the selected character.
    Play(usize),
    /// Create a new character.
    Create,
    /// Delete the selected character.
    Delete(usize),
}
