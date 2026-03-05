// @id: MUIE-TmplCharSelect @do: char-select-template @role: template @layer: 6 @human: miyuk

//! Character selection layout template.

use crate::organisms::character_select::{CharSelectAction, CharSlotData, CharacterSelect};
use egui::Context;

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
