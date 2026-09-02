//! @id     games_allumina_ui_lobby
//! @do     definir_auto_refresh_secs
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui lobby ».

//! The multiplayer lobby screen.
//!
//! After the network client receives a [`crate::network::protocol::ServerMessage::LoginAccepted`]
//! the renderer parks the player on this screen until they `JoinedGame`.
//! Here they can browse the public game list, refresh it, create a new game
//! (with name / max-players / difficulty / optional password), or join an
//! existing one — typing a password into a popup for private games.
//!
//! Rendering follows the same immediate-mode pattern as [`title_screen`] —
//! every frame `draw` re-emits coloured quads + bitmap text and caches the
//! click rects so [`LobbyScreen::click`] can hit-test them on the next event.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::network::protocol::{Difficulty, GameInfo};
use crate::render::font;
use crate::render::vertex::{self, Color};
use crate::ui::{border, fill, point_in, SUB_CONTENT, SUB_PANEL, SUB_TEXT, SUB_TOOLTIP};

use super::Rect;

/// Auto-refresh cadence for the game list, in seconds.
pub const AUTO_REFRESH_SECS: f32 = 5.0;
/// Player-count bounds enforced by the create-game popup.
pub const MIN_PLAYERS: u32 = 2;
pub const MAX_PLAYERS: u32 = 8;

/// What the lobby screen decided this frame after user input.
#[derive(Debug, Clone, PartialEq)]
pub enum LobbyAction {
    /// Nothing actionable happened.
    None,
    /// User asked for a fresh game list.
    Refresh,
    /// User asked to join a public game.
    JoinPublic { game_id: u64 },
    /// User asked to join a private game with the supplied password.
    JoinPrivate { game_id: u64, password: String },
    /// User asked to create a new game.
    Create {
        name: String,
        max_players: u32,
        difficulty: Difficulty,
        /// `None` ⇒ public; `Some(pw)` ⇒ private.
        password: Option<String>,
    },
    /// User asked to leave the lobby (disconnect, return to title).
    Back,
}

/// What sub-popup is overlaid on top of the game list.
#[derive(Debug, Clone)]
enum Modal {
    None,
    /// User is filling out the create-game form.
    Create {
        name: String,
        max_players: u32,
        difficulty: Difficulty,
        password: String,
        /// Which form field has keyboard focus.
        focus: CreateField,
    },
    /// User clicked a private game; we're collecting its password.
    Password { game_id: u64, game_name: String, password: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CreateField {
    Name,
    Password,
}

/// Mutable client-side lobby UI state. Owned by the renderer.
pub struct LobbyScreen {
    /// The last `GameList` we received from the server.
    pub games: Vec<GameInfo>,
    /// Most recently surfaced error (`JoinFailed` etc.); shown as a toast.
    pub error: Option<String>,
    /// Seconds since the last [`LobbyAction::Refresh`] we emitted.
    refresh_clock: f32,
    /// Display-only time, drives subtle animation on the panel.
    time: f32,
    modal: Modal,

    // Cached click rects (refreshed each `draw`).
    row_rects: Vec<(Rect, u64)>,
    btn_create: Rect,
    btn_refresh: Rect,
    btn_back: Rect,
    modal_buttons: Vec<(Rect, ModalButton)>,
}

#[derive(Debug, Clone, Copy)]
enum ModalButton {
    // Create popup buttons / fields.
    CreateConfirm,
    CreateCancel,
    CreateFocusName,
    CreateFocusPassword,
    CreateMaxDec,
    CreateMaxInc,
    CreateDifficulty(Difficulty),
    // Password popup buttons.
    PasswordConfirm,
    PasswordCancel,
}

impl Default for LobbyScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl LobbyScreen {
    #[must_use]
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            error: None,
            refresh_clock: 0.0,
            time: 0.0,
            modal: Modal::None,
            row_rects: Vec::new(),
            btn_create: [0.0; 4],
            btn_refresh: [0.0; 4],
            btn_back: [0.0; 4],
            modal_buttons: Vec::new(),
        }
    }

    /// Replace the cached game list with a fresh server snapshot.
    pub fn set_games(&mut self, games: Vec<GameInfo>) {
        self.games = games;
        self.refresh_clock = 0.0;
    }

    /// Surface an error toast (e.g. on `JoinFailed`).
    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.error = Some(msg.into());
    }

    /// Advance internal timers. Returns `Some(LobbyAction::Refresh)` when the
    /// auto-refresh interval elapses so the caller can resend `ListGames`.
    pub fn tick(&mut self, dt: f32) -> LobbyAction {
        self.time += dt;
        self.refresh_clock += dt;
        if self.refresh_clock >= AUTO_REFRESH_SECS && matches!(self.modal, Modal::None) {
            self.refresh_clock = 0.0;
            return LobbyAction::Refresh;
        }
        LobbyAction::None
    }

    /// True if a modal popup currently owns the keyboard.
    #[must_use]
    pub fn modal_active(&self) -> bool {
        !matches!(self.modal, Modal::None)
    }

    /// Discard the active popup (Esc).
    pub fn close_modal(&mut self) {
        self.modal = Modal::None;
    }

    /// Open the create-game popup with sensible defaults.
    pub fn open_create(&mut self) {
        self.modal = Modal::Create {
            name: String::new(),
            max_players: 4,
            difficulty: Difficulty::Normal,
            password: String::new(),
            focus: CreateField::Name,
        };
    }

    // ── Input ────────────────────────────────────────────────────────────

    /// Handle a left click. Returns the action the click triggered (if any).
    pub fn click(&mut self, mouse: [f32; 2]) -> LobbyAction {
        // Modal clicks first (it covers the regular UI).
        let modal_buttons = self.modal_buttons.clone();
        for (rect, btn) in &modal_buttons {
            if point_in(*rect, mouse) {
                if let Some(action) = self.activate_modal(*btn) {
                    return action;
                }
                return LobbyAction::None;
            }
        }
        if self.modal_active() {
            // Click outside an active modal button: swallow it but stay open.
            return LobbyAction::None;
        }

        if point_in(self.btn_back, mouse) {
            return LobbyAction::Back;
        }
        if point_in(self.btn_refresh, mouse) {
            self.refresh_clock = 0.0;
            return LobbyAction::Refresh;
        }
        if point_in(self.btn_create, mouse) {
            self.open_create();
            return LobbyAction::None;
        }
        // Row clicks: join.
        let rows = self.row_rects.clone();
        for (rect, id) in &rows {
            if point_in(*rect, mouse) {
                if let Some(info) = self.games.iter().find(|g| g.id == *id) {
                    if info.has_password {
                        self.modal = Modal::Password {
                            game_id: info.id,
                            game_name: info.name.clone(),
                            password: String::new(),
                        };
                        return LobbyAction::None;
                    }
                    return LobbyAction::JoinPublic { game_id: info.id };
                }
            }
        }
        LobbyAction::None
    }

    fn activate_modal(&mut self, btn: ModalButton) -> Option<LobbyAction> {
        match (&mut self.modal, btn) {
            (Modal::Create { name, max_players, difficulty, password, .. }, ModalButton::CreateConfirm) => {
                if name.trim().is_empty() {
                    self.error = Some("Nom de partie requis".into());
                    return None;
                }
                let pw = if password.is_empty() { None } else { Some(password.clone()) };
                let action = LobbyAction::Create {
                    name: name.trim().to_string(),
                    max_players: *max_players,
                    difficulty: *difficulty,
                    password: pw,
                };
                self.modal = Modal::None;
                Some(action)
            }
            (Modal::Create { .. }, ModalButton::CreateCancel) => {
                self.modal = Modal::None;
                None
            }
            (Modal::Create { focus, .. }, ModalButton::CreateFocusName) => {
                *focus = CreateField::Name;
                None
            }
            (Modal::Create { focus, .. }, ModalButton::CreateFocusPassword) => {
                *focus = CreateField::Password;
                None
            }
            (Modal::Create { max_players, .. }, ModalButton::CreateMaxDec) => {
                *max_players = max_players.saturating_sub(1).max(MIN_PLAYERS);
                None
            }
            (Modal::Create { max_players, .. }, ModalButton::CreateMaxInc) => {
                *max_players = (*max_players + 1).min(MAX_PLAYERS);
                None
            }
            (Modal::Create { difficulty, .. }, ModalButton::CreateDifficulty(d)) => {
                *difficulty = d;
                None
            }
            (Modal::Password { game_id, password, .. }, ModalButton::PasswordConfirm) => {
                if password.is_empty() {
                    return None;
                }
                let action = LobbyAction::JoinPrivate { game_id: *game_id, password: password.clone() };
                self.modal = Modal::None;
                Some(action)
            }
            (Modal::Password { .. }, ModalButton::PasswordCancel) => {
                self.modal = Modal::None;
                None
            }
            _ => None,
        }
    }

    /// Type a printable character into the focused field (Esc / Tab / Enter
    /// are handled separately by the renderer's key router).
    pub fn type_char(&mut self, ch: char) {
        match &mut self.modal {
            Modal::Create { name, password, focus, .. } => {
                let target = match focus {
                    CreateField::Name => name,
                    CreateField::Password => password,
                };
                if ch == '\u{8}' {
                    target.pop();
                    return;
                }
                if ch.is_control() {
                    return;
                }
                let cap = match focus {
                    CreateField::Name => 32,
                    CreateField::Password => 32,
                };
                if target.chars().count() < cap {
                    target.push(ch);
                }
            }
            Modal::Password { password, .. } => {
                if ch == '\u{8}' {
                    password.pop();
                    return;
                }
                if ch.is_control() {
                    return;
                }
                if password.chars().count() < 32 {
                    password.push(ch);
                }
            }
            Modal::None => {}
        }
    }

    /// "Pressed Enter" while a modal is open — commits the modal as if the
    /// user had clicked its Confirm button.
    pub fn submit_modal(&mut self) -> LobbyAction {
        match &self.modal {
            Modal::Create { .. } => self
                .activate_modal(ModalButton::CreateConfirm)
                .unwrap_or(LobbyAction::None),
            Modal::Password { .. } => self
                .activate_modal(ModalButton::PasswordConfirm)
                .unwrap_or(LobbyAction::None),
            Modal::None => LobbyAction::None,
        }
    }

    // ── Drawing ─────────────────────────────────────────────────────────

    pub fn draw(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2]) {
        // Dim backdrop.
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_PANEL, [0.02, 0.02, 0.04, 1.0]);
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_PANEL, [0.06, 0.04, 0.10, 0.5]);

        // Header.
        font::draw_text_centered(
            batch,
            b"LOBBY MULTIJOUEUR",
            viewport[0] * 0.5,
            36.0,
            3.5,
            [0.95, 0.78, 0.30, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );

        // Outer panel for the games table.
        let pad = 40.0;
        let header_h = 90.0;
        let bottom_buttons_h = 96.0;
        let panel: Rect = [
            pad,
            header_h,
            viewport[0] - pad * 2.0,
            viewport[1] - header_h - bottom_buttons_h - pad,
        ];
        fill(batch, panel, SUB_PANEL, vertex::PANEL_BG);
        border(batch, panel, 2.0, SUB_PANEL + 1, vertex::PANEL_BORDER);

        self.draw_table(batch, panel);
        self.draw_bottom_buttons(batch, viewport);

        if let Some(err) = self.error.clone() {
            self.draw_error_toast(batch, viewport, &err);
        }

        match self.modal.clone() {
            Modal::None => {
                self.modal_buttons.clear();
            }
            Modal::Create { name, max_players, difficulty, password, focus } => {
                self.draw_create_modal(batch, viewport, &name, max_players, difficulty, &password, focus);
            }
            Modal::Password { game_name, password, .. } => {
                self.draw_password_modal(batch, viewport, &game_name, &password);
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn draw_table(&mut self, batch: &mut SpriteBatch, panel: Rect) {
        self.row_rects.clear();

        // Column header.
        let col_x = [
            panel[0] + 16.0,
            panel[0] + panel[2] * 0.40,
            panel[0] + panel[2] * 0.55,
            panel[0] + panel[2] * 0.68,
            panel[0] + panel[2] * 0.82,
            panel[0] + panel[2] * 0.93,
        ];
        let header_y = panel[1] + 14.0;
        let headers: [&[u8]; 6] = [b"NOM", b"HOTE", b"JOUEURS", b"DIFF", b"NIV", b"PRIV"];
        for (i, h) in headers.iter().enumerate() {
            font::draw_text(
                batch,
                h,
                col_x[i],
                header_y,
                1.5,
                [0.78, 0.66, 0.40, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
        }
        let sep_y = header_y + 26.0;
        fill(batch, [panel[0] + 8.0, sep_y, panel[2] - 16.0, 1.5], SUB_CONTENT, vertex::PANEL_BORDER);

        if self.games.is_empty() {
            font::draw_text_centered(
                batch,
                b"AUCUNE PARTIE OUVERTE",
                panel[0] + panel[2] * 0.5,
                panel[1] + panel[3] * 0.5 - 12.0,
                2.0,
                [0.55, 0.50, 0.45, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            return;
        }

        // Rows.
        let row_h = 40.0;
        let inner_y = sep_y + 8.0;
        let max_rows = ((panel[1] + panel[3] - 20.0 - inner_y) / row_h) as usize;
        for (i, info) in self.games.iter().take(max_rows).enumerate() {
            let y = inner_y + i as f32 * row_h;
            let row: Rect = [panel[0] + 8.0, y, panel[2] - 16.0, row_h - 4.0];
            let zebra = if i % 2 == 0 { [0.08, 0.07, 0.10, 0.85] } else { [0.05, 0.04, 0.07, 0.85] };
            fill(batch, row, SUB_CONTENT, zebra);

            // Name.
            draw_clipped_text(batch, info.name.as_bytes(), col_x[0], y + 10.0, 2.0, vertex::WHITE, 0.40 * panel[2] - 8.0);
            // Host.
            draw_clipped_text(
                batch,
                info.host_name.as_bytes(),
                col_x[1],
                y + 10.0,
                1.5,
                [0.78, 0.72, 0.55, 1.0],
                0.14 * panel[2] - 8.0,
            );
            // Players.
            let players = format!("{}/{}", info.players, info.max_players);
            font::draw_text(batch, players.as_bytes(), col_x[2], y + 10.0, 2.0, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);
            // Difficulty.
            let (diff_label, diff_color) = match info.difficulty {
                Difficulty::Normal => ("NORMAL", [0.78, 0.84, 0.55, 1.0]),
                Difficulty::Cauchemar => ("CAUCH", [0.95, 0.65, 0.30, 1.0]),
                Difficulty::Enfer => ("ENFER", [0.95, 0.30, 0.25, 1.0]),
            };
            font::draw_text(batch, diff_label.as_bytes(), col_x[3], y + 10.0, 1.5, diff_color, RenderLayer::UiScreen, 0, SUB_TEXT);
            // Avg level (rounded).
            let lvl = format!("{:.0}", info.avg_level);
            font::draw_text(batch, lvl.as_bytes(), col_x[4], y + 10.0, 2.0, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT);
            // Lock icon for private games.
            if info.has_password {
                font::draw_text(batch, b"X", col_x[5], y + 10.0, 2.0, [0.95, 0.78, 0.30, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
            } else {
                font::draw_text(batch, b"-", col_x[5], y + 10.0, 2.0, [0.45, 0.42, 0.36, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
            }
            self.row_rects.push((row, info.id));
        }
    }

    fn draw_bottom_buttons(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2]) {
        let btn_h = 56.0;
        let btn_w = 240.0;
        let gap = 20.0;
        let total = btn_w * 3.0 + gap * 2.0;
        let start_x = (viewport[0] - total) * 0.5;
        let y = viewport[1] - btn_h - 24.0;

        self.btn_create = [start_x, y, btn_w, btn_h];
        self.btn_refresh = [start_x + btn_w + gap, y, btn_w, btn_h];
        self.btn_back = [start_x + (btn_w + gap) * 2.0, y, btn_w, btn_h];

        draw_button(batch, self.btn_create, b"CREER UNE PARTIE", true);
        draw_button(batch, self.btn_refresh, b"RAFRAICHIR", false);
        draw_button(batch, self.btn_back, b"RETOUR", false);
    }

    fn draw_error_toast(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2], msg: &str) {
        let w = 480.0_f32.min(viewport[0] - 60.0);
        let h = 36.0;
        let x = (viewport[0] - w) * 0.5;
        let y = viewport[1] * 0.12;
        let r: Rect = [x, y, w, h];
        fill(batch, r, SUB_TOOLTIP, [0.30, 0.00, 0.00, 0.85]);
        border(batch, r, 1.5, SUB_TOOLTIP + 1, [0.95, 0.30, 0.25, 1.0]);
        font::draw_text_centered(
            batch,
            msg.as_bytes(),
            x + w * 0.5,
            y + 10.0,
            1.5,
            [1.0, 0.85, 0.78, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 2,
        );
    }

    fn draw_create_modal(
        &mut self,
        batch: &mut SpriteBatch,
        viewport: [f32; 2],
        name: &str,
        max_players: u32,
        difficulty: Difficulty,
        password: &str,
        focus: CreateField,
    ) {
        self.modal_buttons.clear();
        // Dim backdrop.
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_TOOLTIP, [0.0, 0.0, 0.0, 0.7]);
        let w = 520.0_f32.min(viewport[0] - 60.0);
        let h = 420.0_f32;
        let x = (viewport[0] - w) * 0.5;
        let y = (viewport[1] - h) * 0.5;
        let r: Rect = [x, y, w, h];
        fill(batch, r, SUB_TOOLTIP + 1, vertex::PANEL_BG);
        border(batch, r, 2.0, SUB_TOOLTIP + 2, vertex::PANEL_BORDER);
        font::draw_text_centered(
            batch,
            b"CREER UNE PARTIE",
            x + w * 0.5,
            y + 16.0,
            3.0,
            [0.95, 0.78, 0.30, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 3,
        );

        // Name field.
        let label = |b: &mut SpriteBatch, text: &[u8], lx: f32, ly: f32| {
            font::draw_text(b, text, lx, ly, 1.5, [0.78, 0.72, 0.55, 1.0], RenderLayer::UiScreen, 0, SUB_TOOLTIP + 3);
        };
        let field_x = x + 24.0;
        let field_w = w - 48.0;
        let mut row_y = y + 70.0;
        label(batch, b"NOM DE LA PARTIE", field_x, row_y);
        row_y += 22.0;
        let name_rect: Rect = [field_x, row_y, field_w, 32.0];
        draw_text_field(batch, name_rect, name.as_bytes(), focus == CreateField::Name);
        self.modal_buttons.push((name_rect, ModalButton::CreateFocusName));
        row_y += 50.0;

        // Max players (slider replaced with -/+ buttons).
        label(batch, b"JOUEURS MAX", field_x, row_y);
        row_y += 22.0;
        let stepper_y = row_y;
        let btn_w = 40.0;
        let dec: Rect = [field_x, stepper_y, btn_w, 32.0];
        let inc: Rect = [field_x + field_w - btn_w, stepper_y, btn_w, 32.0];
        let value_rect: Rect = [field_x + btn_w + 8.0, stepper_y, field_w - 2.0 * (btn_w + 8.0), 32.0];
        draw_button(batch, dec, b"-", false);
        draw_button(batch, inc, b"+", false);
        fill(batch, value_rect, SUB_TOOLTIP + 3, [0.04, 0.04, 0.06, 0.9]);
        border(batch, value_rect, 1.0, SUB_TOOLTIP + 4, vertex::PANEL_BORDER);
        let val = format!("{max_players}");
        font::draw_text_centered(
            batch,
            val.as_bytes(),
            value_rect[0] + value_rect[2] * 0.5,
            value_rect[1] + 8.0,
            2.5,
            vertex::WHITE,
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 4,
        );
        self.modal_buttons.push((dec, ModalButton::CreateMaxDec));
        self.modal_buttons.push((inc, ModalButton::CreateMaxInc));
        row_y += 50.0;

        // Difficulty radio.
        label(batch, b"DIFFICULTE", field_x, row_y);
        row_y += 22.0;
        let dw = (field_w - 16.0) / 3.0;
        let diffs = [
            (Difficulty::Normal, b"NORMAL" as &[u8]),
            (Difficulty::Cauchemar, b"CAUCHEMAR"),
            (Difficulty::Enfer, b"ENFER"),
        ];
        for (i, (d, lbl)) in diffs.iter().enumerate() {
            let bx = field_x + i as f32 * (dw + 8.0);
            let br: Rect = [bx, row_y, dw, 32.0];
            draw_button(batch, br, lbl, difficulty == *d);
            self.modal_buttons.push((br, ModalButton::CreateDifficulty(*d)));
        }
        row_y += 50.0;

        // Password (optional).
        label(batch, b"MOT DE PASSE (OPTIONNEL)", field_x, row_y);
        row_y += 22.0;
        let pw_rect: Rect = [field_x, row_y, field_w, 32.0];
        let dots: String = "*".repeat(password.chars().count());
        draw_text_field(batch, pw_rect, dots.as_bytes(), focus == CreateField::Password);
        self.modal_buttons.push((pw_rect, ModalButton::CreateFocusPassword));

        // Confirm / Cancel.
        let cy = y + h - 56.0;
        let ok: Rect = [x + w * 0.5 + 8.0, cy, 200.0, 40.0];
        let no: Rect = [x + w * 0.5 - 208.0, cy, 200.0, 40.0];
        draw_button(batch, ok, b"CREER", true);
        draw_button(batch, no, b"ANNULER", false);
        self.modal_buttons.push((ok, ModalButton::CreateConfirm));
        self.modal_buttons.push((no, ModalButton::CreateCancel));
    }

    fn draw_password_modal(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2], game_name: &str, password: &str) {
        self.modal_buttons.clear();
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_TOOLTIP, [0.0, 0.0, 0.0, 0.7]);
        let w = 420.0_f32;
        let h = 200.0_f32;
        let x = (viewport[0] - w) * 0.5;
        let y = (viewport[1] - h) * 0.5;
        let r: Rect = [x, y, w, h];
        fill(batch, r, SUB_TOOLTIP + 1, vertex::PANEL_BG);
        border(batch, r, 2.0, SUB_TOOLTIP + 2, vertex::PANEL_BORDER);
        font::draw_text_centered(
            batch,
            b"MOT DE PASSE",
            x + w * 0.5,
            y + 14.0,
            2.5,
            [0.95, 0.78, 0.30, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 3,
        );
        font::draw_text_centered(
            batch,
            game_name.as_bytes(),
            x + w * 0.5,
            y + 44.0,
            1.5,
            [0.78, 0.72, 0.55, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TOOLTIP + 3,
        );
        let field: Rect = [x + 24.0, y + 74.0, w - 48.0, 32.0];
        let dots: String = "*".repeat(password.chars().count());
        draw_text_field(batch, field, dots.as_bytes(), true);
        let ok: Rect = [x + w * 0.5 + 8.0, y + h - 50.0, 160.0, 36.0];
        let no: Rect = [x + w * 0.5 - 168.0, y + h - 50.0, 160.0, 36.0];
        draw_button(batch, ok, b"REJOINDRE", true);
        draw_button(batch, no, b"ANNULER", false);
        self.modal_buttons.push((ok, ModalButton::PasswordConfirm));
        self.modal_buttons.push((no, ModalButton::PasswordCancel));
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn draw_button(batch: &mut SpriteBatch, rect: Rect, label: &[u8], highlight: bool) {
    let bg: Color = if highlight {
        [0.18, 0.13, 0.05, 0.95]
    } else {
        [0.08, 0.07, 0.10, 0.85]
    };
    fill(batch, rect, SUB_TOOLTIP + 3, bg);
    let bcol = if highlight { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER };
    border(batch, rect, if highlight { 2.0 } else { 1.5 }, SUB_TOOLTIP + 4, bcol);
    let px = 2.0_f32;
    let w = font::text_width(label, px);
    let tx = rect[0] + (rect[2] - w) * 0.5;
    let ty = rect[1] + (rect[3] - px * 7.0) * 0.5;
    let tc = if highlight { [1.0, 0.95, 0.65, 1.0] } else { [0.78, 0.74, 0.65, 1.0] };
    font::draw_text(batch, label, tx, ty, px, tc, RenderLayer::UiScreen, 0, SUB_TOOLTIP + 5);
}

fn draw_text_field(batch: &mut SpriteBatch, rect: Rect, text: &[u8], focused: bool) {
    fill(batch, rect, SUB_TOOLTIP + 3, [0.04, 0.04, 0.06, 0.95]);
    let bcol = if focused { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER };
    border(batch, rect, if focused { 2.0 } else { 1.0 }, SUB_TOOLTIP + 4, bcol);
    let px = 2.0_f32;
    font::draw_text(batch, text, rect[0] + 8.0, rect[1] + (rect[3] - px * 7.0) * 0.5, px, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TOOLTIP + 5);
}

fn draw_clipped_text(batch: &mut SpriteBatch, text: &[u8], x: f32, y: f32, px: f32, color: Color, max_w: f32) {
    // Cheap "clip": chop bytes off until the rendered width fits. The bitmap
    // font is fixed-width per glyph so this is monotonic.
    let mut end = text.len();
    while end > 0 && font::text_width(&text[..end], px) > max_w {
        end -= 1;
    }
    font::draw_text(batch, &text[..end], x, y, px, color, RenderLayer::UiScreen, 0, SUB_TEXT);
}
