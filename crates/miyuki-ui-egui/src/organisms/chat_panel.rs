// @id: MUIE-ChatPanel @do: chat-panel-organism @role: organism @layer: 6 @human: miyuk

//! Chat panel organism -- message list + input field.

use crate::convert::rgba_to_color32;
use crate::molecules::chat_message::{ChatMessage, ChatType};
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Chat message data.
#[derive(Debug, Clone)]
pub struct ChatMsgData {
    /// Sender name.
    pub sender: String,
    /// Message text.
    pub text: String,
    /// Message type.
    pub chat_type: ChatType,
}

/// The chat panel organism.
pub struct ChatPanel;

impl ChatPanel {
    /// Draw the chat panel at the bottom-left.
    pub fn show(
        ctx: &Context,
        messages: &[ChatMsgData],
        input_buffer: &mut String,
        screen_h: f32,
    ) -> bool {
        let mut sent = false;

        egui::Area::new("chat_panel".into())
            .fixed_pos(egui::Pos2::new(5.0, screen_h - 180.0))
            .show(ctx, |ui| {
                ui.set_max_width(300.0);
                ui.set_max_height(100.0);

                // Message history
                egui::ScrollArea::vertical()
                    .max_height(80.0)
                    .show(ui, |ui| {
                        for msg in messages {
                            ChatMessage::new(&msg.sender, &msg.text, msg.chat_type).show(ui);
                        }
                    });

                // Input
                ui.horizontal(|ui| {
                    let resp = ui.add(
                        egui::TextEdit::singleline(input_buffer)
                            .desired_width(250.0)
                            .hint_text("Ecrire un message...")
                            .text_color(rgba_to_color32(&D2_PALETTE.text_primary)),
                    );

                    if resp.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                        && !input_buffer.trim().is_empty()
                    {
                        sent = true;
                    }
                });
            });

        sent
    }
}
