// @id: MUIE-ChatMsg @do: chat-message-molecule @role: molecule @layer: 6 @human: miyuk

//! Chat message molecule -- colored pseudo + text.

use egui::{Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::convert::rgba_to_color32;

/// Chat message type (determines name color).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatType {
    /// Normal player message (white).
    Normal,
    /// Whisper (yellow).
    Whisper,
    /// Party message (green).
    Party,
    /// System message (gold).
    System,
}

/// A chat message molecule.
pub struct ChatMessage<'a> {
    /// Sender name.
    pub sender: &'a str,
    /// Message text.
    pub text: &'a str,
    /// Message type.
    pub chat_type: ChatType,
}

impl<'a> ChatMessage<'a> {
    /// Create a new chat message.
    pub fn new(sender: &'a str, text: &'a str, chat_type: ChatType) -> Self {
        Self {
            sender,
            text,
            chat_type,
        }
    }

    /// Draw the chat message and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let name_color = match self.chat_type {
            ChatType::Normal => rgba_to_color32(&D2_PALETTE.text_primary),
            ChatType::Whisper => egui::Color32::from_rgb(255, 255, 100),
            ChatType::Party => egui::Color32::from_rgb(0, 200, 0),
            ChatType::System => rgba_to_color32(&D2_PALETTE.accent_primary),
        };

        let text_color = rgba_to_color32(&D2_PALETTE.text_primary);

        let response = ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(self.sender)
                    .color(name_color)
                    .size(11.0),
            );
            ui.label(
                egui::RichText::new(": ")
                    .color(text_color)
                    .size(11.0),
            );
            ui.label(
                egui::RichText::new(self.text)
                    .color(text_color)
                    .size(11.0),
            );
        });

        response.response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_types() {
        let msg = ChatMessage::new("Player1", "Hello", ChatType::Normal);
        assert_eq!(msg.sender, "Player1");
        assert_eq!(msg.chat_type, ChatType::Normal);
    }
}
