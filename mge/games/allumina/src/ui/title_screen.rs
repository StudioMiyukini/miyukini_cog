//! @id     games_allumina_ui_title_screen
//! @do     definir_title_action
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui title screen ».

//! The starting screen: big "ALLUMINA" logo, dark background with drifting
//! ember motes, and a menu (Nouvelle partie / Charger partie / Multijoueur /
//! Quitter). Driven by a tiny state machine that the renderer's `GameApp`
//! polls each frame.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::render::font;
use crate::render::particles::ParticleSystem;
use crate::render::vertex::{self, with_alpha, Color};
use crate::save::{self, SaveSummary};
use crate::ui::{border, fill, point_in, SUB_CONTENT, SUB_PANEL, SUB_TEXT};

use super::Rect;

/// Outcomes the title screen surfaces to `GameApp`.
#[derive(Debug, Clone)]
pub enum TitleAction {
    None,
    NewGame,
    LoadGame(SaveSummary),
    Multiplayer,
    Quit,
}

/// Which menu item is currently highlighted, or whether the load list is open.
#[derive(Debug, Clone)]
pub enum TitleMode {
    Menu,
    LoadList { saves: Vec<SaveSummary>, cursor: usize },
    MultiplayerStub { since: f32 },
}

pub struct TitleScreen {
    pub mode: TitleMode,
    pub selected: usize,
    pub time: f32,
    /// Cached for hit-testing when the cursor moves between frames.
    last_button_rects: Vec<(Rect, usize)>,
    last_load_rects: Vec<(Rect, usize)>,
}

const MENU_ITEMS: [&[u8]; 4] = [
    b"NOUVELLE PARTIE",
    b"CHARGER PARTIE",
    b"MULTIJOUEUR",
    b"QUITTER",
];

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            mode: TitleMode::Menu,
            selected: 0,
            time: 0.0,
            last_button_rects: Vec::new(),
            last_load_rects: Vec::new(),
        }
    }
}

impl TitleScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, dt: f32, particles: &mut ParticleSystem) {
        self.time += dt;
        // Drift a steady stream of warm ember motes upward as ambient mood.
        if (self.time * 4.0).fract() < dt * 4.0 {
            let cx = ((self.time * 0.31).sin() + 1.0) * 6.0 - 6.0;
            let pos = crate::components::Position2D::new(cx, 4.0);
            particles.emit_burst(pos, [0.95, 0.55, 0.18, 1.0], 3, 1.2);
        }
    }

    /// Move the menu cursor up/down.
    pub fn move_selection(&mut self, delta: i32) {
        match &mut self.mode {
            TitleMode::Menu => {
                let n = MENU_ITEMS.len() as i32;
                let s = (self.selected as i32 + delta).rem_euclid(n);
                self.selected = s as usize;
            }
            TitleMode::LoadList { saves, cursor } => {
                if saves.is_empty() {
                    return;
                }
                let n = saves.len() as i32;
                let s = (*cursor as i32 + delta).rem_euclid(n);
                *cursor = s as usize;
            }
            TitleMode::MultiplayerStub { .. } => {}
        }
    }

    /// Commit whatever the current selection is.
    pub fn confirm(&mut self) -> TitleAction {
        match &self.mode {
            TitleMode::Menu => match self.selected {
                0 => TitleAction::NewGame,
                1 => {
                    let saves = save::list_saves();
                    self.mode = TitleMode::LoadList { saves, cursor: 0 };
                    TitleAction::None
                }
                2 => {
                    self.mode = TitleMode::MultiplayerStub { since: 0.0 };
                    TitleAction::None
                }
                _ => TitleAction::Quit,
            },
            TitleMode::LoadList { saves, cursor } => {
                if let Some(s) = saves.get(*cursor) {
                    TitleAction::LoadGame(s.clone())
                } else {
                    TitleAction::None
                }
            }
            TitleMode::MultiplayerStub { .. } => {
                self.mode = TitleMode::Menu;
                TitleAction::None
            }
        }
    }

    /// Esc / back: drop one level (load list → menu, stub → menu, menu → quit).
    pub fn back(&mut self) -> TitleAction {
        match &self.mode {
            TitleMode::Menu => TitleAction::Quit,
            _ => {
                self.mode = TitleMode::Menu;
                TitleAction::None
            }
        }
    }

    /// Track the mouse hover: highlight whichever item is under the cursor.
    pub fn hover(&mut self, mouse: [f32; 2]) {
        match &mut self.mode {
            TitleMode::Menu => {
                for &(r, i) in &self.last_button_rects {
                    if point_in(r, mouse) {
                        self.selected = i;
                        return;
                    }
                }
            }
            TitleMode::LoadList { cursor, .. } => {
                for &(r, i) in &self.last_load_rects {
                    if point_in(r, mouse) {
                        *cursor = i;
                        return;
                    }
                }
            }
            TitleMode::MultiplayerStub { .. } => {}
        }
    }

    /// Click → confirm the item under the cursor.
    pub fn click(&mut self, mouse: [f32; 2]) -> TitleAction {
        self.hover(mouse);
        match &self.mode {
            TitleMode::Menu => {
                for &(r, _) in &self.last_button_rects {
                    if point_in(r, mouse) {
                        return self.confirm();
                    }
                }
                TitleAction::None
            }
            TitleMode::LoadList { .. } => {
                for &(r, _) in &self.last_load_rects {
                    if point_in(r, mouse) {
                        return self.confirm();
                    }
                }
                TitleAction::None
            }
            TitleMode::MultiplayerStub { .. } => {
                self.mode = TitleMode::Menu;
                TitleAction::None
            }
        }
    }

    pub fn draw(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2], particles: &ParticleSystem) {
        // Full-screen dark backdrop with a soft vignette.
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_PANEL, [0.02, 0.02, 0.04, 1.0]);
        let vignette = (self.time * 0.3).sin() * 0.05 + 0.85;
        fill(
            batch,
            [0.0, 0.0, viewport[0], viewport[1]],
            SUB_PANEL,
            [0.06, 0.04, 0.10, 0.45 * vignette],
        );

        // Ambient particles, projected as if seen from the bottom of the world.
        let cam = crate::render::camera::Camera2D::new(
            crate::components::Position2D::new(0.0, 0.0),
            1.0,
            viewport,
        )
        .with_time(self.time);
        particles.draw(batch, &cam);

        // Logo: huge bitmap text. Two passes: dark drop-shadow + main glow tint.
        let logo_px = (viewport[0] / 220.0).clamp(6.0, 14.0);
        let logo_y = viewport[1] * 0.20;
        let pulse = 0.55 + 0.20 * (self.time * 1.6).sin().abs();
        let logo_color = [1.0, 0.85 * pulse + 0.15, 0.30 * pulse + 0.10, 1.0];
        font::draw_text_centered(
            batch,
            b"ALLUMINA",
            viewport[0] * 0.5 + 3.0,
            logo_y + 3.0,
            logo_px,
            [0.0, 0.0, 0.0, 0.7],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        font::draw_text_centered(
            batch,
            b"ALLUMINA",
            viewport[0] * 0.5,
            logo_y,
            logo_px,
            logo_color,
            RenderLayer::UiScreen,
            0,
            SUB_TEXT + 1,
        );

        // Subtitle.
        font::draw_text_centered(
            batch,
            b"UN MONDE DE TENEBRES VOUS ATTEND",
            viewport[0] * 0.5,
            logo_y + logo_px * 9.0 + 16.0,
            2.0,
            [0.78, 0.66, 0.40, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );

        match self.mode.clone() {
            TitleMode::Menu => self.draw_menu(batch, viewport),
            TitleMode::LoadList { saves, cursor } => self.draw_load_list(batch, viewport, &saves, cursor),
            TitleMode::MultiplayerStub { .. } => self.draw_multiplayer_stub(batch, viewport),
        }

        // Footer hint.
        font::draw_text_centered(
            batch,
            b"FLECHES + ENTREE OU SOURIS",
            viewport[0] * 0.5,
            viewport[1] - 24.0,
            1.0,
            [0.5, 0.45, 0.35, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
    }

    fn draw_menu(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2]) {
        self.last_button_rects.clear();
        let btn_w = 360.0_f32.min(viewport[0] - 80.0);
        let btn_h = 44.0;
        let gap = 14.0;
        let start_y = viewport[1] * 0.50;
        for (i, label) in MENU_ITEMS.iter().enumerate() {
            let x = (viewport[0] - btn_w) * 0.5;
            let y = start_y + i as f32 * (btn_h + gap);
            let rect: Rect = [x, y, btn_w, btn_h];
            let highlight = i == self.selected;
            let bg: Color = if highlight {
                [0.18, 0.13, 0.05, 0.95]
            } else {
                [0.08, 0.07, 0.10, 0.85]
            };
            fill(batch, rect, SUB_PANEL, bg);
            border(
                batch,
                rect,
                if highlight { 2.5 } else { 1.5 },
                SUB_PANEL + 1,
                if highlight { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER },
            );
            let px = 3.0;
            let w = font::text_width(label, px);
            let tx = x + (btn_w - w) * 0.5;
            let ty = y + (btn_h - px * 7.0) * 0.5;
            let color = if highlight { [1.0, 0.95, 0.65, 1.0] } else { [0.78, 0.74, 0.68, 1.0] };
            font::draw_text(batch, label, tx, ty, px, color, RenderLayer::UiScreen, 0, SUB_TEXT);
            self.last_button_rects.push((rect, i));
        }
    }

    fn draw_load_list(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2], saves: &[SaveSummary], cursor: usize) {
        self.last_load_rects.clear();
        let w = 720.0_f32.min(viewport[0] - 80.0);
        let h = 360.0_f32.min(viewport[1] - 220.0);
        let x = (viewport[0] - w) * 0.5;
        let y = viewport[1] * 0.45;
        let panel: Rect = [x, y, w, h];
        fill(batch, panel, SUB_PANEL, vertex::PANEL_BG);
        border(batch, panel, 2.0, SUB_PANEL + 1, vertex::PANEL_BORDER);
        font::draw_text(
            batch,
            b"CHARGER PARTIE",
            x + 16.0,
            y + 12.0,
            2.5,
            [0.95, 0.85, 0.45, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );

        if saves.is_empty() {
            font::draw_text_centered(
                batch,
                b"AUCUNE SAUVEGARDE",
                viewport[0] * 0.5,
                y + h * 0.5,
                2.0,
                [0.55, 0.50, 0.45, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            font::draw_text_centered(
                batch,
                b"ESC POUR REVENIR",
                viewport[0] * 0.5,
                y + h - 30.0,
                1.5,
                [0.55, 0.50, 0.45, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            return;
        }

        let row_h = 56.0;
        let inner_x = x + 12.0;
        let inner_y = y + 50.0;
        for (i, s) in saves.iter().enumerate() {
            let ry = inner_y + i as f32 * row_h;
            if ry + row_h > y + h - 10.0 {
                break;
            }
            let row: Rect = [inner_x, ry, w - 24.0, row_h - 6.0];
            let highlight = i == cursor;
            let bg = if highlight { [0.18, 0.13, 0.05, 0.95] } else { [0.06, 0.06, 0.08, 0.8] };
            fill(batch, row, SUB_CONTENT, bg);
            if highlight {
                border(batch, row, 2.0, SUB_CONTENT + 1, [0.95, 0.78, 0.30, 1.0]);
            }
            // Top line: name + class + level.
            let head = format!(
                "{} - {} LVL {}",
                s.character_name.to_uppercase(),
                s.class.display_name().to_uppercase(),
                s.level
            );
            font::draw_text(
                batch,
                head.as_bytes(),
                row[0] + 10.0,
                row[1] + 6.0,
                2.0,
                if highlight { [1.0, 0.95, 0.65, 1.0] } else { [0.85, 0.80, 0.70, 1.0] },
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            // Bottom line: zone, play time, timestamp.
            let foot = format!(
                "{}  -  {}  -  {}",
                s.current_zone.to_uppercase(),
                save::fmt_play_time(s.play_time_secs),
                s.timestamp,
            );
            font::draw_text(
                batch,
                foot.as_bytes(),
                row[0] + 10.0,
                row[1] + row_h - 20.0,
                1.0,
                [0.62, 0.58, 0.50, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            self.last_load_rects.push((row, i));
        }
    }

    fn draw_multiplayer_stub(&self, batch: &mut SpriteBatch, viewport: [f32; 2]) {
        let w = 460.0_f32.min(viewport[0] - 80.0);
        let h = 160.0;
        let x = (viewport[0] - w) * 0.5;
        let y = (viewport[1] - h) * 0.5;
        let panel: Rect = [x, y, w, h];
        fill(batch, panel, SUB_PANEL, [0.0, 0.0, 0.0, 0.85]);
        border(batch, panel, 2.0, SUB_PANEL + 1, vertex::PANEL_BORDER);
        font::draw_text_centered(
            batch,
            b"MULTIJOUEUR",
            viewport[0] * 0.5,
            y + 22.0,
            3.0,
            [0.95, 0.78, 0.30, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        font::draw_text_centered(
            batch,
            b"BIENTOT DISPONIBLE",
            viewport[0] * 0.5,
            y + h * 0.55,
            2.0,
            [0.78, 0.74, 0.65, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        font::draw_text_centered(
            batch,
            b"ENTREE OU CLIC POUR REVENIR",
            viewport[0] * 0.5,
            y + h - 26.0,
            1.5,
            [0.55, 0.50, 0.45, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
    }
}

/// Floating "Sauvegarde..." disc icon, drawn while `remaining > 0`.
pub fn draw_save_indicator(batch: &mut SpriteBatch, viewport: [f32; 2], remaining: f32, total: f32) {
    if remaining <= 0.0 {
        return;
    }
    let frac = (remaining / total).clamp(0.0, 1.0);
    let alpha = frac.min(1.0);
    let w = 180.0;
    let h = 36.0;
    let x = viewport[0] - w - 20.0;
    let y = viewport[1] - h - 20.0;
    let r: Rect = [x, y, w, h];
    fill(batch, r, SUB_TEXT, [0.0, 0.0, 0.0, 0.70 * alpha]);
    border(batch, r, 1.5, SUB_TEXT + 1, with_alpha(vertex::PANEL_BORDER, alpha));
    // Disc icon.
    let cx = x + 18.0;
    let cy = y + h * 0.5;
    crate::render::shapes::circle(batch, cx, cy, 10.0, RenderLayer::UiScreen, 0, SUB_TEXT + 2, with_alpha([0.30, 0.55, 1.0, 1.0], alpha));
    crate::render::shapes::circle(batch, cx, cy, 3.0, RenderLayer::UiScreen, 0, SUB_TEXT + 3, with_alpha([0.95, 0.95, 0.95, 1.0], alpha));
    font::draw_text(
        batch,
        b"SAUVEGARDE...",
        x + 36.0,
        y + (h - 14.0) * 0.5,
        2.0,
        with_alpha([0.95, 0.92, 0.85, 1.0], alpha),
        RenderLayer::UiScreen,
        0,
        SUB_TEXT + 4,
    );
}
