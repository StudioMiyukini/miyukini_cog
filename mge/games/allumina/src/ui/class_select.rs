//! @id     games_allumina_ui_class_select
//! @do     definir_default_name
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui class select ».

//! Class-selection screen. Four side-by-side panels show a silhouette + name +
//! description + stat bars + starting weapon + role; the player picks one and
//! confirms a character name to enter the game.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::player::{ClassType, Stats};
use crate::components::Position2D;
use crate::render::animation::AnimSample;
use crate::render::camera::Camera2D;
use crate::render::figures::{self, Direction, FigureStyle, Shape, Weapon};
use crate::render::font;
use crate::render::vertex::{self, Color};
use crate::ui::{border, fill, point_in, SUB_CONTENT, SUB_PANEL, SUB_TEXT};

use super::Rect;

const DEFAULT_NAME: &str = "Héros";
const MAX_NAME_LEN: usize = 20;
/// What the screen reports back to `GameApp` each frame.
#[derive(Debug, Clone)]
pub enum ClassSelectAction {
    None,
    Start { class: ClassType, name: String },
    Back,
}

pub struct ClassSelectScreen {
    pub selected: Option<usize>,
    pub name: String,
    pub time: f32,
    /// Whether the name field is currently capturing keystrokes.
    pub editing_name: bool,
    last_panels: Vec<(Rect, usize)>,
    last_start_btn: Option<Rect>,
    last_back_btn: Option<Rect>,
    last_name_field: Option<Rect>,
}

impl Default for ClassSelectScreen {
    fn default() -> Self {
        Self {
            selected: None,
            name: DEFAULT_NAME.to_string(),
            time: 0.0,
            editing_name: false,
            last_panels: Vec::new(),
            last_start_btn: None,
            last_back_btn: None,
            last_name_field: None,
        }
    }
}

impl ClassSelectScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn move_selection(&mut self, delta: i32) {
        let cur = self.selected.unwrap_or(0) as i32;
        let next = (cur + delta).rem_euclid(4);
        self.selected = Some(next as usize);
    }

    pub fn confirm(&mut self) -> ClassSelectAction {
        let Some(idx) = self.selected else { return ClassSelectAction::None };
        let class = ClassType::all()[idx];
        let name = if self.name.trim().is_empty() {
            DEFAULT_NAME.to_string()
        } else {
            self.name.trim().to_string()
        };
        ClassSelectAction::Start { class, name }
    }

    pub fn back(&mut self) -> ClassSelectAction {
        ClassSelectAction::Back
    }

    pub fn click(&mut self, mouse: [f32; 2]) -> ClassSelectAction {
        if let Some(r) = self.last_name_field {
            if point_in(r, mouse) {
                self.editing_name = true;
                return ClassSelectAction::None;
            } else {
                self.editing_name = false;
            }
        }
        if let Some(r) = self.last_back_btn {
            if point_in(r, mouse) {
                return ClassSelectAction::Back;
            }
        }
        if let Some(r) = self.last_start_btn {
            if point_in(r, mouse) && self.selected.is_some() {
                return self.confirm();
            }
        }
        for &(r, i) in &self.last_panels {
            if point_in(r, mouse) {
                self.selected = Some(i);
                return ClassSelectAction::None;
            }
        }
        ClassSelectAction::None
    }

    /// Feed a printable character into the name field. Honours backspace.
    pub fn type_char(&mut self, ch: char) {
        if !self.editing_name {
            return;
        }
        if ch == '\u{8}' {
            self.name.pop();
        } else if (ch.is_ascii_alphanumeric() || ch == ' ' || ch == '-' || ch == '_')
            && self.name.len() < MAX_NAME_LEN
        {
            self.name.push(ch);
        }
    }

    pub fn draw(&mut self, batch: &mut SpriteBatch, viewport: [f32; 2]) {
        // Backdrop.
        fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_PANEL, [0.04, 0.04, 0.06, 1.0]);

        // Title.
        font::draw_text_centered(
            batch,
            b"CHOISIS TA CLASSE",
            viewport[0] * 0.5,
            36.0,
            3.5,
            [0.95, 0.85, 0.45, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );

        // Four panels.
        self.last_panels.clear();
        let classes = ClassType::all();
        let n = classes.len() as f32;
        let pad = 18.0;
        let panel_w = (viewport[0] - pad * (n + 1.0)) / n;
        let panel_w = panel_w.min(280.0);
        let total_w = panel_w * n + pad * (n - 1.0);
        let start_x = (viewport[0] - total_w) * 0.5;
        let panel_h = (viewport[1] - 240.0).max(360.0);
        let panel_y = 110.0;

        for (i, &class) in classes.iter().enumerate() {
            let x = start_x + i as f32 * (panel_w + pad);
            let r: Rect = [x, panel_y, panel_w, panel_h];
            let selected = self.selected == Some(i);
            self.draw_class_panel(batch, r, class, selected);
            self.last_panels.push((r, i));
        }

        // Name field.
        let field_w = 360.0;
        let field_h = 36.0;
        let field_x = (viewport[0] - field_w) * 0.5;
        let field_y = panel_y + panel_h + 16.0;
        let field: Rect = [field_x, field_y, field_w, field_h];
        let editing = self.editing_name;
        fill(
            batch,
            field,
            SUB_PANEL,
            if editing { [0.18, 0.13, 0.05, 0.95] } else { [0.10, 0.08, 0.12, 0.9] },
        );
        border(
            batch,
            field,
            if editing { 2.5 } else { 1.5 },
            SUB_PANEL + 1,
            if editing { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER },
        );
        font::draw_text(
            batch,
            b"NOM:",
            field_x - 60.0,
            field_y + 10.0,
            2.0,
            [0.78, 0.72, 0.58, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        let cursor = if editing && ((self.time * 2.0).fract() < 0.5) { "_" } else { "" };
        let shown = format!("{}{}", self.name.to_uppercase(), cursor);
        font::draw_text(
            batch,
            shown.as_bytes(),
            field_x + 12.0,
            field_y + 10.0,
            2.5,
            [0.95, 0.92, 0.85, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        self.last_name_field = Some(field);

        // Start button.
        let active = self.selected.is_some();
        let btn_w = 320.0;
        let btn_h = 46.0;
        let btn_x = (viewport[0] - btn_w) * 0.5;
        let btn_y = field_y + field_h + 22.0;
        let btn: Rect = [btn_x, btn_y, btn_w, btn_h];
        fill(
            batch,
            btn,
            SUB_PANEL,
            if active { [0.20, 0.14, 0.06, 0.95] } else { [0.10, 0.10, 0.12, 0.85] },
        );
        border(
            batch,
            btn,
            if active { 2.5 } else { 1.5 },
            SUB_PANEL + 1,
            if active { [0.95, 0.78, 0.30, 1.0] } else { [0.35, 0.32, 0.28, 1.0] },
        );
        font::draw_text_centered(
            batch,
            b"COMMENCER L AVENTURE",
            btn_x + btn_w * 0.5,
            btn_y + (btn_h - 21.0) * 0.5,
            3.0,
            if active { [1.0, 0.95, 0.65, 1.0] } else { [0.55, 0.52, 0.48, 1.0] },
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        self.last_start_btn = Some(btn);

        // Back button (bottom-left).
        let back_w = 160.0;
        let back_h = 36.0;
        let back: Rect = [24.0, viewport[1] - back_h - 24.0, back_w, back_h];
        fill(batch, back, SUB_PANEL, [0.06, 0.06, 0.08, 0.9]);
        border(batch, back, 1.5, SUB_PANEL + 1, vertex::PANEL_BORDER);
        font::draw_text_centered(
            batch,
            b"RETOUR",
            back[0] + back[2] * 0.5,
            back[1] + (back_h - 14.0) * 0.5,
            2.0,
            [0.85, 0.80, 0.70, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        self.last_back_btn = Some(back);
    }

    fn draw_class_panel(&self, batch: &mut SpriteBatch, r: Rect, class: ClassType, selected: bool) {
        let bg: Color = if selected { [0.12, 0.10, 0.06, 0.96] } else { [0.06, 0.06, 0.08, 0.88] };
        fill(batch, r, SUB_PANEL, bg);
        let bw = if selected { 3.0 } else { 1.5 };
        let bc = if selected { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER };
        border(batch, r, bw, SUB_PANEL + 1, bc);

        // Silhouette.
        let style = class_style(class);
        let cam = Camera2D::new(Position2D::new(0.0, 0.0), 1.0, [r[2], r[3]]);
        let figure_y = r[1] + r[3] * 0.40;
        let figure_x = r[0] + r[2] * 0.5;
        // Cheat: bypass world projection by manually building screen anchor.
        // Draw the figure as if its feet were at (figure_x, figure_y).
        draw_standalone_figure(batch, figure_x, figure_y, [40.0, 70.0], &style, self.time, class);
        let _ = cam;

        // Class name.
        let name = class.display_name().to_uppercase();
        font::draw_text_centered(
            batch,
            name.as_bytes(),
            r[0] + r[2] * 0.5,
            r[1] + r[3] * 0.50,
            2.5,
            if selected { [1.0, 0.95, 0.65, 1.0] } else { [0.90, 0.86, 0.78, 1.0] },
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );

        // Description (2-3 short lines).
        let desc = class_description(class);
        let mut y = r[1] + r[3] * 0.56;
        for line in desc {
            font::draw_text(
                batch,
                line.as_bytes(),
                r[0] + 12.0,
                y,
                1.4,
                [0.78, 0.74, 0.68, 1.0],
                RenderLayer::UiScreen,
                0,
                SUB_TEXT,
            );
            y += 14.0;
        }

        // Stat bars (FOR/DEX/INT/VIT/ENE).
        let stats = Stats::new(class, 1);
        let attrs = &stats.inner.attributes;
        let pairs: [(&[u8], i32); 5] = [
            (b"FOR", attrs.strength),
            (b"DEX", attrs.dexterity),
            (b"INT", attrs.energy),
            (b"VIT", attrs.vitality),
            (b"ENE", attrs.energy),
        ];
        let bar_y0 = r[1] + r[3] * 0.74;
        let bar_w = r[2] - 24.0;
        for (i, (lbl, val)) in pairs.iter().enumerate() {
            let by = bar_y0 + i as f32 * 14.0;
            font::draw_text(batch, lbl, r[0] + 12.0, by, 1.4, [0.70, 0.66, 0.58, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
            let track: Rect = [r[0] + 44.0, by + 1.0, bar_w - 32.0, 8.0];
            fill(batch, track, SUB_CONTENT, [0.04, 0.04, 0.06, 0.95]);
            let frac = (*val as f32 / 50.0).clamp(0.05, 1.0);
            let fill_r: Rect = [track[0], track[1], track[2] * frac, track[3]];
            fill(batch, fill_r, SUB_CONTENT + 1, stat_color(i));
        }

        // Starting weapon + role.
        let (weapon_label, role) = class_weapon_role(class);
        let foot_y = r[1] + r[3] - 36.0;
        font::draw_text(
            batch,
            weapon_label.as_bytes(),
            r[0] + 12.0,
            foot_y,
            1.4,
            [0.68, 0.62, 0.52, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
        font::draw_text(
            batch,
            role.as_bytes(),
            r[0] + 12.0,
            foot_y + 14.0,
            1.7,
            [0.95, 0.78, 0.30, 1.0],
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
    }
}

fn class_description(class: ClassType) -> Vec<String> {
    match class {
        ClassType::Soldat => vec![
            "Maitre du champ de bataille.".into(),
            "Commande une armee de paysans,".into(),
            "miliciens et guerriers.".into(),
        ],
        ClassType::Clerc => vec![
            "Gardien sacre. Auras de soin.".into(),
            "Invoque des anges et controle".into(),
            "les ennemis par la priere.".into(),
        ],
        ClassType::MageDeGuerre => vec![
            "Maitrise trois elements:".into(),
            "feu, glace et foudre.".into(),
            "Devaste les hordes.".into(),
        ],
        ClassType::Archer => vec![
            "Frappe a distance avec".into(),
            "precision. Pose des pieges".into(),
            "mortels et survit par agilite.".into(),
        ],
    }
}

fn class_weapon_role(class: ClassType) -> (String, String) {
    match class {
        ClassType::Soldat => ("ARME: EPEE COURTE".into(), "ROLE: TANK".into()),
        ClassType::Clerc => ("ARME: SCEPTRE".into(), "ROLE: SUPPORT".into()),
        ClassType::MageDeGuerre => ("ARME: BATON".into(), "ROLE: CASTER".into()),
        ClassType::Archer => ("ARME: ARC".into(), "ROLE: DPS".into()),
    }
}

fn stat_color(i: usize) -> Color {
    match i {
        0 => [0.85, 0.25, 0.20, 1.0],
        1 => [0.30, 0.85, 0.40, 1.0],
        2 => [0.45, 0.55, 0.95, 1.0],
        3 => [0.95, 0.65, 0.20, 1.0],
        _ => [0.65, 0.45, 0.95, 1.0],
    }
}

fn class_style(class: ClassType) -> FigureStyle {
    match class {
        ClassType::Soldat => FigureStyle {
            primary: vertex::CLASS_SOLDAT,
            skin: vertex::SKIN,
            build: 1.1,
            weapon: Weapon::Sword,
            offhand: None,
            shape: Shape::Soldier,
            alpha: 1.0,
            outline: None,
            aura: None,
            eyes: None,
            shield: Some([0.5, 0.4, 0.25, 1.0]),
        },
        ClassType::Clerc => FigureStyle {
            primary: vertex::CLASS_CLERC,
            skin: vertex::SKIN,
            build: 1.0,
            weapon: Weapon::Scepter,
            offhand: None,
            shape: Shape::Cleric,
            alpha: 1.0,
            outline: None,
            aura: Some([1.0, 0.95, 0.55, 1.0]),
            eyes: None,
            shield: None,
        },
        ClassType::MageDeGuerre => FigureStyle {
            primary: vertex::CLASS_MAGE,
            skin: vertex::SKIN,
            build: 0.95,
            weapon: Weapon::Staff,
            offhand: None,
            shape: Shape::Warmage,
            alpha: 1.0,
            outline: None,
            aura: None,
            eyes: None,
            shield: None,
        },
        ClassType::Archer => FigureStyle {
            primary: vertex::CLASS_ARCHER,
            skin: vertex::SKIN,
            build: 0.9,
            weapon: Weapon::Bow,
            offhand: None,
            shape: Shape::Archer,
            alpha: 1.0,
            outline: None,
            aura: None,
            eyes: None,
            shield: None,
        },
    }
}

/// Draw a figure at a specific screen anchor (`feet_x`, `feet_y`). This calls
/// `figures::draw` with a unity camera centred on (0,0) and a position that
/// projects to the requested anchor.
fn draw_standalone_figure(
    batch: &mut SpriteBatch,
    feet_x: f32,
    feet_y: f32,
    base: [f32; 2],
    style: &FigureStyle,
    time: f32,
    class: ClassType,
) {
    // Build a camera whose centre is (0,0) and whose half-viewport equals the
    // figure anchor; then placing the figure at world (0,0) lands its feet on
    // (feet_x, feet_y).
    let cam = Camera2D::new(Position2D::new(0.0, 0.0), 1.0, [feet_x * 2.0, feet_y * 2.0])
        .with_time(time);
    let id = match class {
        ClassType::Soldat => 1001,
        ClassType::Clerc => 1002,
        ClassType::MageDeGuerre => 1003,
        ClassType::Archer => 1004,
    };
    figures::draw(
        batch,
        &cam,
        Position2D::new(0.0, 0.0),
        base,
        style,
        &AnimSample::still(),
        Direction::S,
        id,
        time,
    );
}
