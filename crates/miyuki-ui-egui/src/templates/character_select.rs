// @id: MUIE-TmplCharSelect @do: char-select-template @role: template @layer: 6 @human: miyuk

//! Character selection layout template.

use egui::Context;
use crate::organisms::character_select::{CharacterSelect, CharSelectAction, CharSlotData};

/// The character selection layout template.
pub struct CharSelectLayout;

impl CharSelectLayout {
    /// Draw the character selection layout.
    pub fn show(
        ctx: &Context,
        characters: &[CharSlotData],
        selected: Option<usize>,
    ) -> CharSelectAction {
        CharacterSelect::show(ctx, characters, selected)
    }
}
