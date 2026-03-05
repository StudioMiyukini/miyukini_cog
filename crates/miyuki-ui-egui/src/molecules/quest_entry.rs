// @id: MUIE-QuestEntry @do: quest-entry-molecule @role: molecule @layer: 6 @human: miyuk

//! Quest entry molecule -- icon + name + completion state.

use crate::convert::rgba_to_color32;
use egui::{Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Quest completion state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestState {
    /// Quest not yet started.
    NotStarted,
    /// Quest is in progress.
    InProgress,
    /// Quest is completed.
    Completed,
}

/// A quest entry molecule.
pub struct QuestEntry<'a> {
    /// Quest name.
    pub name: &'a str,
    /// Completion state.
    pub state: QuestState,
}

impl<'a> QuestEntry<'a> {
    /// Create a new quest entry.
    pub fn new(name: &'a str, state: QuestState) -> Self {
        Self { name, state }
    }

    /// Draw the quest entry and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let (icon, text_color) = match self.state {
            QuestState::NotStarted => ("[  ]", rgba_to_color32(&D2_PALETTE.text_muted)),
            QuestState::InProgress => ("[>>]", rgba_to_color32(&D2_PALETTE.text_primary)),
            QuestState::Completed => ("[OK]", rgba_to_color32(&D2_PALETTE.text_high)),
        };

        let response = ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon).color(text_color).size(10.0));
            ui.label(egui::RichText::new(self.name).color(text_color).size(11.0))
        });

        response.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_entry_states() {
        let e1 = QuestEntry::new("Den of Evil", QuestState::Completed);
        assert_eq!(e1.state, QuestState::Completed);

        let e2 = QuestEntry::new("Search for Cain", QuestState::InProgress);
        assert_eq!(e2.state, QuestState::InProgress);
    }
}
