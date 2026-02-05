//! Application Lord of the Click — écrans Loading, Landing, Slots, Ma citée, Carte du monde.
//!
//! @id: miyuclicker_app_module
//! @role: interface
//! @layer: application
//! @do: render_screens_and_handle_input

use crate::carte::{generate_carte_mvp, model_update, move_troops};
use crate::idlesim::{
    apply_allocation, apply_allocation_macons, apply_click, convert_pop_to_macon,
    start_construction_maison, tick, pts_required_maison_pub,
    GAME_SPEED_1, GAME_SPEED_2, GAME_SPEED_3, GAME_SPEED_4, GAME_SPEED_5, GAME_SPEED_PAUSE,
};
use crate::save::{slot_list, slot_read, slot_write, SlotMetadata};
use crate::state::{ClickTarget, GameState};
use eframe::egui;
use eframe::App;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Instant;

/// Écran courant (Loading, Landing, Slots, Ma cité, Carte du monde).
/// @id: miyuclicker_app_screen
/// @do: represent_current_screen
/// @role: data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Loading,
    Landing,
    Slots,
    MaCitee,
    CarteMonde,
}

/// Type de marcheur affiché dans la zone cité (sol).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CityWalkerKind {
    Gens,
    Soldat,
    Macon,
}

/// Un marcheur dans la zone sol (position normalisée 0..1, vélocité en px/s).
#[derive(Debug, Clone)]
pub struct CityWalker {
    pub kind: CityWalkerKind,
    pub x: f32,
    pub y: f32,
    /// Vélocité horizontale en pixels par seconde (3–6 px/s en valeur absolue).
    pub vx: f32,
    /// Vélocité verticale en pixels par seconde (3–6 px/s en valeur absolue).
    pub vy: f32,
}

/// Application principale.
pub struct MiyuClickerApp {
    pub screen: Screen,
    pub loading_progress: f32,
    pub loading_done: bool,
    pub config_menu_open: bool,
    pub game_state: Option<GameState>,
    pub slot_metadata: Vec<SlotMetadata>,
    pub confirm_overwrite: Option<u8>,
    pub data_dir: PathBuf,
    pub last_tick: Instant,
    pub selected_cite_id: Option<String>,
    pub send_troops_count: i64,
    /// Envoi de troupes différé (from_id, to_id, count) pour éviter double emprunt.
    pub pending_move_troops: Option<(String, String, i64)>,
    /// Vitesse de jeu : 0 = pause, 1 = 1h/s, 2 = 3h/s, 3 = 6h/s, 4 = 1j/s, 5 = 2j/s (in-game par s IRL).
    pub game_speed_index: u8,
    /// Console dev ouverte.
    pub dev_console_open: bool,
    /// Log dev : (temps_simule au moment du log, message). Cap 300 lignes.
    pub dev_log: VecDeque<(f64, String)>,
    /// Dernier delta IRL (s) pour affichage FPS.
    pub last_delta_s: f64,
    /// Compteur de frames pour FPS et log espacé.
    pub frame_count: u64,
    /// Temps IRL au dernier log espacé (pour ne pas spammer).
    pub last_log_time: f64,
    /// Filtre texte pour la liste des bâtiments (recherche).
    pub building_filter: String,
    /// Marcheurs affichés dans la zone cité (gens, soldats, maçons) — positions en temps réel.
    pub city_walkers: Vec<CityWalker>,
}

/// Valeurs par défaut (Loading, slot 1, vitesse 1, pas de jeu).
/// @id: miyuclicker_app_default
/// @do: default_app_state
/// @role: data
impl Default for MiyuClickerApp {
    fn default() -> Self {
        Self {
            screen: Screen::Loading,
            loading_progress: 0.0,
            loading_done: false,
            config_menu_open: false,
            game_state: None,
            slot_metadata: Vec::new(),
            confirm_overwrite: None,
            data_dir: PathBuf::new(),
            last_tick: Instant::now(),
            selected_cite_id: None,
            send_troops_count: 10,
            pending_move_troops: None,
            game_speed_index: 1,
            dev_console_open: false,
            dev_log: VecDeque::new(),
            last_delta_s: 0.0,
            frame_count: 0,
            last_log_time: 0.0,
            building_filter: String::new(),
            city_walkers: Vec::new(),
        }
    }
}

/// Layout Lord of Click : écarts et blocs (blocs organiques marron, molécules/atomes noir).
const LAYOUT_ORGANIC_MARGIN_I: i8 = 10;
const LAYOUT_ATOM_STROKE: f32 = 1.5;
const LAYOUT_SPACING_BLOCKS: f32 = 8.0;
const LAYOUT_NOTIFICATION_HEIGHT: f32 = 28.0;
const LAYOUT_HEADER_LORD_HEIGHT: f32 = 36.0;
/// Marge en bas de l'affichage : la liste des bâtiments s'arrête à cette distance du bord.
const LAYOUT_BOTTOM_MARGIN: f32 = 10.0;
/// Agrandissement du header ressources + horloge (en px, réparti en haut/bas).
const HEADER_BAR_EXTRA_PX: f32 = 4.0;
const HEADER_BAR_GROUP_SPACING: f32 = 8.0;
/// Couleur contour bloc organique (marron).
fn color_organic_stroke() -> egui::Color32 {
    egui::Color32::from_rgb(139, 90, 43)
}
/// Contour atome/molécule (noir).
fn color_atom_stroke() -> egui::Color32 {
    egui::Color32::from_rgb(30, 30, 30)
}

/// Retourne le multiplicateur (s in-game / s IRL) pour l’index de vitesse (0..=5).
/// @id: miyuclicker_app_game_speed_multiplier
/// @do: resolve_game_speed_multiplier_from_index
/// @role: reader
/// @human: 0=pause, 1=1h/s, 2=3h/s, 3=6h/s, 4=1j/s, 5=2j/s.
fn game_speed_multiplier(index: u8) -> f64 {
    match index {
        0 => GAME_SPEED_PAUSE,
        1 => GAME_SPEED_1,
        2 => GAME_SPEED_2,
        3 => GAME_SPEED_3,
        4 => GAME_SPEED_4,
        5 => GAME_SPEED_5,
        _ => GAME_SPEED_1,
    }
}

/// Nombre max de lignes dans le log dev.
const DEV_LOG_MAX: usize = 300;

/// Table des boutons de clic (cible, label, taille min). Guilde gérée à part (convert_pop_to_macon).
const CLIC_BUTTONS: &[(ClickTarget, &str, (f32, f32))] = &[
    (ClickTarget::Champs, "Champs\n+1 nourriture", (100.0, 50.0)),
    (ClickTarget::Bois, "Bois\n+1", (100.0, 50.0)),
    (ClickTarget::Pierre, "Pierre\n+1", (100.0, 50.0)),
    (ClickTarget::Fer, "Fer\n+1", (100.0, 50.0)),
    (ClickTarget::Chateau, "Château\n+1 soldat\n-50 nourr. -5 armes", (100.0, 60.0)),
    (ClickTarget::Village, "Village\n+1 pop\n-30 nourriture", (100.0, 60.0)),
];

impl MiyuClickerApp {
    /// Ajoute une ligne au log dev (tag [TEST] pour traçabilité). Cap à DEV_LOG_MAX.
    /// @id: miyuclicker_app_dev_log_message
    /// @do: append_dev_log_line
    /// @role: mutator
    fn dev_log(&mut self, msg: impl Into<String>) {
        let temps = self
            .game_state
            .as_ref()
            .map(|s| s.temps_simule_s)
            .unwrap_or(0.0);
        let line = format!("[TEST] {}", msg.into());
        self.dev_log.push_back((temps, line));
        while self.dev_log.len() > DEV_LOG_MAX {
            self.dev_log.pop_front();
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let storage = cc.storage;
        let data_dir = storage
            .and_then(|s| s.get_string("data_dir"))
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            });
        let mut app = Self {
            data_dir,
            ..Default::default()
        };
        app.slot_metadata = slot_list(&app.data_dir);
        app
    }

    /// Crée une instance pour intégration dans Miyukini Central (point d'accès utilisateur unique).
    /// Le jeu s'exécute dans le body de Central ; pas de fenêtre standalone.
    pub fn new_embedded() -> Self {
        let data_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = Self {
            data_dir,
            ..Default::default()
        };
        app.slot_metadata = slot_list(&app.data_dir);
        app
    }

    /// Fenêtre Configuration (Sauvegarder, résolution, langue, à propos, fermer).
    /// @id: miyuclicker_app_ui_config_menu
    /// @do: render_config_window
    /// @role: interface
    fn ui_config_menu(&mut self, ctx: &egui::Context, in_game: bool) {
        if !self.config_menu_open {
            return;
        }
        egui::Window::new("Configuration")
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 40.0])
            .resizable(false)
            .show(ctx, |ui| {
                if in_game {
                    if ui.button("Sauvegarder").clicked() {
                        if let Some(ref state) = self.game_state {
                            let _ = slot_write(&self.data_dir, state.slot_id, state);
                            self.slot_metadata = slot_list(&self.data_dir);
                        }
                        self.config_menu_open = false;
                    }
                }
                if ui.button("Changer la résolution").clicked() {
                    self.config_menu_open = false;
                }
                if ui.button("Langue").clicked() {
                    self.config_menu_open = false;
                }
                if ui.button("À propos").clicked() {
                    self.config_menu_open = false;
                }
                if ui.button("Fermer").clicked() {
                    self.config_menu_open = false;
                }
            });
    }

    /// Contenu de l'écran de chargement (titre, barre de progression). Utilisé par ui_loading et show_into.
    fn ui_loading_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Lord of the Click");
            ui.add_space(20.0);
            ui.label("Chargement…");
            ui.add(
                egui::ProgressBar::new(self.loading_progress)
                    .animate(true)
                    .show_percentage(),
            );
        });
    }

    /// Écran de chargement (titre, barre de progression, passage à Landing).
    /// @id: miyuclicker_app_ui_loading
    /// @do: render_loading_screen_and_advance
    /// @role: interface
    fn ui_loading(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_loading_content(ui);
        });
        self.loading_progress += 0.02;
        if self.loading_progress >= 1.0 {
            self.loading_done = true;
            self.screen = Screen::Landing;
            self.slot_metadata = slot_list(&self.data_dir);
        }
        ctx.request_repaint();
    }

    /// Contenu de l'écran d'accueil (header + bouton Jouer). Utilisé par ui_landing et show_into.
    fn ui_landing_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Lord of the Click");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙").clicked() {
                    self.config_menu_open = true;
                }
            });
        });
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            if ui.add(egui::Button::new("Jouer").min_size(egui::vec2(200.0, 50.0))).clicked() {
                self.screen = Screen::Slots;
                self.slot_metadata = slot_list(&self.data_dir);
            }
        });
    }

    /// Écran d'accueil (titre, bouton Jouer, config).
    /// @id: miyuclicker_app_ui_landing
    /// @do: render_landing_screen
    /// @role: interface
    fn ui_landing(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("landing_header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Lord of the Click");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.config_menu_open = true;
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);
                if ui.add(egui::Button::new("Jouer").min_size(egui::vec2(200.0, 50.0))).clicked() {
                    self.screen = Screen::Slots;
                    self.slot_metadata = slot_list(&self.data_dir);
                }
            });
        });
        self.ui_config_menu(ctx, false);
    }

    /// Contenu de l'écran choix de slot. Utilisé par ui_slots et show_into.
    fn ui_slots_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Choisir une sauvegarde");
        ui.add_space(20.0);
        for meta in &self.slot_metadata {
            ui.horizontal(|ui| {
                ui.label(format!("Slot {}:", meta.slot_id));
                if meta.occupied {
                    ui.label(meta.saved_at.as_deref().unwrap_or("—"));
                    if let Some(s) = &meta.summary {
                        ui.label(s);
                    }
                    if ui.button("Charger").clicked() {
                        match slot_read(&self.data_dir, meta.slot_id) {
                            Ok(state) => {
                                self.game_state = Some(state);
                                self.game_speed_index = 0; // Pause au chargement d'une partie.
                                self.screen = Screen::MaCitee;
                                if self.game_state.as_ref().map(|s| s.cites.is_empty()).unwrap_or(true) {
                                    if let Some(ref mut g) = self.game_state {
                                        generate_carte_mvp(g);
                                    }
                                }
                            }
                            Err(e) => {
                                ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                            }
                        }
                    }
                    if self.confirm_overwrite == Some(meta.slot_id) {
                        if ui.button("Confirmer écrasement").clicked() {
                            self.game_state = Some(GameState::new_game(meta.slot_id));
                            if let Some(ref mut g) = self.game_state {
                                generate_carte_mvp(g);
                            }
                            self.screen = Screen::MaCitee;
                            self.confirm_overwrite = None;
                        }
                        if ui.button("Annuler").clicked() {
                            self.confirm_overwrite = None;
                        }
                    } else if ui.button("Nouvelle partie").clicked() {
                        self.confirm_overwrite = Some(meta.slot_id);
                    }
                } else {
                    ui.label("Vide");
                    if ui.button("Nouvelle partie").clicked() {
                        self.game_state = Some(GameState::new_game(meta.slot_id));
                        self.game_speed_index = 0; // Pause au chargement.
                        if let Some(ref mut g) = self.game_state {
                            generate_carte_mvp(g);
                        }
                        self.screen = Screen::MaCitee;
                    }
                }
            });
            ui.add_space(10.0);
        }
        ui.add_space(20.0);
        if ui.button("Retour").clicked() {
            self.screen = Screen::Landing;
        }
    }

    /// Écran choix de slot (Charger, Nouvelle partie, Confirmer écrasement, Retour).
    /// @id: miyuclicker_app_ui_slots
    /// @do: render_slot_selection_screen
    /// @role: interface
    fn ui_slots(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_slots_content(ui);
        });
    }

    /// Barre en 2 lignes : ressources groupées avec leur métrique, horloge lisible, +4px hauteur.
    /// @id: miyuclicker_app_ui_bar
    /// @do: render_header_bar_resources_clock_speed_nav
    /// @role: interface
    fn ui_bar(&mut self, ui: &mut egui::Ui) {
        let state = match &self.game_state {
            Some(s) => s,
            None => return,
        };
        let clock = GameState::format_clock(state.temps_simule_s);
        let speed = self.game_speed_index;
        let half_extra = HEADER_BAR_EXTRA_PX / 2.0;
        ui.add_space(half_extra);

        // Ligne 1 : Or, Gens, Soldats, Maçons, Recherche, Bonheur | Horloge + Vitesse
        ui.horizontal(|ui| {
            Self::header_resource(ui, "Or", &format!("{}", state.or));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Gens", &format!("{}/{}", state.gens, state.cap_gens));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Soldats", &format!("{}/{}", state.soldats, state.cap_soldats()));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Maçons", &format!("{}", state.macons));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Recherche", &format!("{}", state.recherche));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Bonheur", &format!("{} %", state.bonheur_pourcent()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(clock).strong());
                ui.add_space(HEADER_BAR_GROUP_SPACING);
                if ui.selectable_label(speed == 5, "5").clicked() {
                    self.game_speed_index = 5;
                }
                if ui.selectable_label(speed == 4, "4").clicked() {
                    self.game_speed_index = 4;
                }
                if ui.selectable_label(speed == 3, "3").clicked() {
                    self.game_speed_index = 3;
                }
                if ui.selectable_label(speed == 2, "2").clicked() {
                    self.game_speed_index = 2;
                }
                if ui.selectable_label(speed == 1, "1").clicked() {
                    self.game_speed_index = 1;
                }
                ui.label("Vitesse :");
                if ui.selectable_label(speed == 0, "Pause").clicked() {
                    self.game_speed_index = 0;
                }
            });
        });
        ui.add_space(4.0);

        // Ligne 2 : Nourriture, Bois, Pierre, Fer, Outils, Armes | Ma citée, Carte du monde, ⚙, Dev
        ui.horizontal(|ui| {
            let cap_nour = state.cap_nourriture() as i64;
            Self::header_resource(ui, "Nourriture", &format!("{}/{}", state.nourriture as i64, cap_nour));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Bois", &format!("{}", state.bois as i64));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Pierre", &format!("{}", state.pierre as i64));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Fer", &format!("{}", state.fer as i64));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Outils", &format!("{}", state.outils as i64));
            ui.add_space(HEADER_BAR_GROUP_SPACING);
            Self::header_resource(ui, "Armes", &format!("{}", state.armes));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.link("Dev").clicked() {
                    self.dev_console_open = !self.dev_console_open;
                }
                if ui.button("⚙").clicked() {
                    self.config_menu_open = true;
                }
                if ui.link("Carte du monde").clicked() {
                    self.screen = Screen::CarteMonde;
                }
                if ui.link("Ma citée").clicked() {
                    self.screen = Screen::MaCitee;
                }
            });
        });
        ui.add_space(half_extra);
    }

    /// Un groupe ressource : libellé + métrique (lisibilité).
    fn header_resource(ui: &mut egui::Ui, label: &str, value: &str) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} ", label)).weak());
            ui.label(egui::RichText::new(value).strong());
        });
    }

    /// Fenêtre console dev : FPS, delta, vitesse, temps jeu, log défilant.
    /// @id: miyuclicker_app_ui_dev_console
    /// @do: render_dev_console_stats_and_log
    /// @role: interface
    /// @human: Métriques et log [TEST] pour diagnostic défilement temps.
    fn ui_dev_console(&mut self, ctx: &egui::Context) {
        if !self.dev_console_open {
            return;
        }
        let state = self.game_state.as_ref();
        let temps_s = state.map(|s| s.temps_simule_s).unwrap_or(0.0);
        let mult = game_speed_multiplier(self.game_speed_index);
        egui::Window::new("Console Dev")
            .open(&mut self.dev_console_open)
            .default_width(420.0)
            .default_height(320.0)
            .show(ctx, |ui| {
                ui.heading("Métriques");
                ui.horizontal(|ui| {
                    ui.label(format!("FPS (estimé): {:.0}", 1.0 / self.last_delta_s.max(0.001)));
                    ui.label(format!("delta IRL: {:.3} s", self.last_delta_s));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Vitesse index: {}", self.game_speed_index));
                    ui.label(format!("mult: {:.0} s jeu/s IRL", mult));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("temps_simule_s: {:.1}", temps_s));
                    ui.label(format!("frame: {}", self.frame_count));
                });
                ui.separator();
                ui.label("Log [TEST] (défilement = repaint continu)");
                egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                    for (t, line) in self.dev_log.iter().rev() {
                        ui.label(format!("[{:.0}s] {}", t, line));
                    }
                });
                if ui.button("Vider le log").clicked() {
                    self.dev_log.clear();
                }
            });
    }

    /// Cartes bâtiment/zone : Maison, Ferme, Scierie, etc. Filtre par nom. Contour atome (noir).
    /// max_height : hauteur max de la zone scrollable (pour s'arrêter à 10px du bord bas).
    /// @id: miyuclicker_app_ui_building_zone_cards
    /// @do: render_seven_cards_responsive_rounded
    /// @role: interface
    fn ui_building_cards(ui: &mut egui::Ui, state: &mut GameState, filter: &str, max_height: f32) {
        let mut am = state.allocation_macons.clone();
        let mut a = state.allocation.clone();
        let macons_total = state.macons;
        let gens_max = state.gens;

        // Molécule/atome : contour noir, marge intérieure.
        fn card_frame(ui: &egui::Ui) -> egui::Frame {
            egui::Frame::group(ui.style())
                .corner_radius(5.0)
                .inner_margin(egui::Margin::same(5i8))
                .stroke(egui::Stroke::new(LAYOUT_ATOM_STROKE, color_atom_stroke()))
        }

        fn matches_filter(name: &str, filter: &str) -> bool {
            if filter.is_empty() {
                return true;
            }
            name.to_lowercase().contains(&filter.to_lowercase())
        }

        let card_spacing = LAYOUT_SPACING_BLOCKS;
        let scroll_h = max_height.max(120.0);
        egui::ScrollArea::vertical().max_height(scroll_h).show(ui, |ui| {
            ui.set_min_width(220.0);
            ui.vertical(|ui| {
                if matches_filter("Maison", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_maison(ui, state, &mut am.maison, macons_total);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Ferme", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "F", "Ferme", "Nourriture", &mut a.champs, gens_max);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Scierie", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "S", "Scierie", "Bois", &mut a.scierie, gens_max);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Carrière", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "C", "Carrière", "Pierre", &mut a.carriere, gens_max);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Mine", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "M", "Mine", "Fer", &mut a.mine, gens_max);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Atelier", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "A", "Atelier", "Outils", &mut a.ateliers, gens_max);
                    });
                    ui.add_space(card_spacing);
                }
                if matches_filter("Forge", filter) {
                    card_frame(ui).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        Self::ui_card_zone(ui, "G", "Forge", "Armes", &mut a.forge, gens_max);
                    });
                }
            });

            if am.total() <= macons_total {
                let _ = apply_allocation_macons(state, &am);
            }
            if a.total() <= gens_max {
                let _ = apply_allocation(state, &a);
            }
        });
    }

    /// Carte Maison : logo, niveau, cap population, coût, barre construction, maçons +/-.
    /// @id: miyuclicker_app_ui_card_maison
    /// @do: render_house_card
    /// @role: interface
    fn ui_card_maison(
        ui: &mut egui::Ui,
        state: &mut GameState,
        macons_alloc: &mut i64,
        macons_total: i64,
    ) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("M").size(22.0));
            ui.vertical(|ui| {
                ui.label(format!("Maison (×{})", state.maisons));
                ui.label("+4 cap population");
                ui.label("Coût: 30b 20p 5f");
                let required = pts_required_maison_pub(state);
                let pct = if required > 0.0 { (state.construction_maison / required).min(1.0) } else { 0.0 };
                ui.add(egui::ProgressBar::new(pct as f32).show_percentage());
                let can_build = !state.construction_maison_paid
                    && state.bois >= 30.0
                    && state.pierre >= 20.0
                    && state.fer >= 5.0;
                let btn = egui::Button::new(egui::RichText::new("Construire").color(if can_build { egui::Color32::from_rgb(0, 128, 0) } else { egui::Color32::GRAY }));
                if ui.add_enabled(can_build, btn).clicked() {
                    start_construction_maison(state);
                }
            });
        });
        ui.horizontal(|ui| {
            ui.label("Maçons:");
            if ui.small_button("−").clicked() && *macons_alloc > 0 {
                *macons_alloc -= 1;
            }
            ui.label(macons_alloc.to_string());
            if ui.small_button("+").clicked() && *macons_alloc < macons_total {
                *macons_alloc += 1;
            }
        });
    }

    /// Carte zone (Ferme, Scierie, Carrière, Mine, Atelier, Forge) : logo, nom, ressource, gens +/-.
    /// @id: miyuclicker_app_ui_card_zone
    /// @do: render_zone_card
    /// @role: interface
    fn ui_card_zone(
        ui: &mut egui::Ui,
        logo: &str,
        name: &str,
        description: &str,
        gens_alloc: &mut i64,
        gens_max: i64,
    ) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(logo).size(22.0));
            ui.vertical(|ui| {
                ui.label(name);
                ui.label(description);
            });
        });
        ui.horizontal(|ui| {
            ui.label("Gens:");
            if ui.small_button("−").clicked() && *gens_alloc > 0 {
                *gens_alloc -= 1;
            }
            ui.label(gens_alloc.to_string());
            if ui.small_button("+").clicked() && *gens_alloc < gens_max {
                *gens_alloc += 1;
            }
        });
    }

    /// Overlay Game Over unique : message + bouton Retour aux slots.
    /// @id: miyuclicker_app_ui_game_over_overlay
    /// @do: render_game_over_message_and_return_to_slots
    /// @role: interface
    fn ui_game_over_overlay(ui: &mut egui::Ui, screen: &mut Screen) {
        ui.vertical_centered(|ui| {
            ui.add_space(80.0);
            ui.heading("Game Over");
            ui.label("Le seigneur est mort après plus de 7 jours sans nourriture.");
            ui.add_space(20.0);
            if ui.button("Retour aux slots").clicked() {
                *screen = Screen::Slots;
            }
        });
    }

    /// Contenu central de l'écran Ma cité : blocs organiques (zone cité, notification) + molécules (panneau ressources, panneau bâtiments).
    fn ui_ma_citee_central_content(&mut self, ui: &mut egui::Ui) {
        let game_over = self.game_state.as_ref().map(|s| s.game_over).unwrap_or(false);
        if game_over {
            Self::ui_game_over_overlay(ui, &mut self.screen);
            return;
        }
        if let Some(ref mut state) = self.game_state {
            let available = ui.available_rect_before_wrap();
            let zone_height = (available.height() * 0.25).max(180.0).min(available.height() - LAYOUT_NOTIFICATION_HEIGHT - 200.0);

            // Bloc organique : zone de vision de la cité (ciel / sol + boutons de clic).
            let zone_frame = egui::Frame::default()
                .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
                .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
            zone_frame.show(ui, |ui| {
                let zone_rect = ui.available_rect_before_wrap();
                let h = zone_rect.height().min(zone_height).max(80.0);
                let r = ui.allocate_rect(
                    egui::Rect::from_min_size(zone_rect.min, egui::vec2(zone_rect.width(), h)),
                    egui::Sense::hover(),
                ).rect;
                let sky_rect = egui::Rect::from_min_max(
                    r.min,
                    egui::pos2(r.max.x, r.min.y + r.height() * 0.6),
                );
                let ground_rect = egui::Rect::from_min_max(
                    egui::pos2(r.min.x, r.min.y + r.height() * 0.6),
                    r.max,
                );
                ui.painter().rect_filled(sky_rect, 0.0, egui::Color32::from_rgb(135, 206, 235));
                ui.painter().rect_filled(ground_rect, 0.0, egui::Color32::from_rgb(34, 139, 34));

                // Sync et mise à jour des marcheurs (gens, soldats, maçons) en temps réel (3–6 px/s).
                Self::sync_city_walkers(&mut self.city_walkers, state);
                let delta_sec = self.last_delta_s.min(0.1) as f32;
                Self::update_city_walkers(&mut self.city_walkers, delta_sec, ground_rect);
                Self::draw_city_walkers(ui, &self.city_walkers, ground_rect);

                ui.add_space(LAYOUT_SPACING_BLOCKS);
                // Molécules : boutons de clic (Champs, Bois, Pierre, etc.) + Guilde.
                ui.horizontal_wrapped(|ui| {
                    for (target, label, (w, h)) in CLIC_BUTTONS {
                        if ui.add(egui::Button::new(*label).min_size(egui::vec2(*w, *h))).clicked() {
                            apply_click(state, *target);
                        }
                    }
                    if ui.add(egui::Button::new("Guilde\n1 pop → 1 maçon").min_size(egui::vec2(100.0, 50.0))).clicked() {
                        convert_pop_to_macon(state);
                    }
                });
            });
            ui.add_space(LAYOUT_SPACING_BLOCKS);

            // Bloc organique : notification In Game en défilement.
            let notif_frame = egui::Frame::default()
                .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
                .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
            notif_frame.show(ui, |ui| {
                ui.set_min_height(LAYOUT_NOTIFICATION_HEIGHT);
                let last = self.dev_log.back().map(|(t, s)| format!("[{:.0}s] {}", t, s)).unwrap_or_else(|| "—".to_string());
                ui.label(egui::RichText::new(last).small());
            });
            ui.add_space(LAYOUT_SPACING_BLOCKS);

            // Panneau bâtiments : recherche + liste scrollable (pleine largeur, marge 10px en bas).
            let buildings_frame = egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
                .stroke(egui::Stroke::new(LAYOUT_ATOM_STROKE, color_atom_stroke()));
            buildings_frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.label("Recherche et filtrage des batiments");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.building_filter)
                            .hint_text("Filtrer…")
                            .desired_width(200.0),
                    );
                    ui.add_space(6.0);
                    let available_h = ui.available_rect_before_wrap().height();
                    Self::ui_building_cards(ui, state, self.building_filter.trim(), available_h);
                });
            });
            // Marge 10px en bas : la liste des bâtiments s'arrête à 10px du bord bas de l'affichage.
            ui.add_space(LAYOUT_BOTTOM_MARGIN);
        }
    }

    /// Synchronise la liste des marcheurs avec les effectifs (gens, soldats, maçons).
    fn sync_city_walkers(walkers: &mut Vec<CityWalker>, state: &GameState) {
        let n_gens = state.gens.clamp(0, 200) as usize;
        let n_soldats = state.soldats.clamp(0, 200) as usize;
        let n_macons = state.macons.clamp(0, 200) as usize;
        let mut out: Vec<CityWalker> = Vec::with_capacity(n_gens + n_soldats + n_macons);
        for (kind, target) in [
            (CityWalkerKind::Gens, n_gens),
            (CityWalkerKind::Soldat, n_soldats),
            (CityWalkerKind::Macon, n_macons),
        ] {
            let existing: Vec<CityWalker> = walkers.iter().filter(|w| w.kind == kind).cloned().take(target).collect();
            for w in existing {
                out.push(w);
            }
            let current = out.iter().filter(|w| w.kind == kind).count();
            for i in 0..target.saturating_sub(current) {
                let j = current + i;
                let x = ((j * 17) % 97) as f32 / 97.0;
                let y = ((j * 13) % 97) as f32 / 97.0;
                // Vitesse 3–6 px/s : (j % 4) donne 0..3, on ajoute 3 → 3..6.
                let speed_x = 3.0 + ((j * 3) % 4) as f32;
                let speed_y = 3.0 + ((j * 5) % 4) as f32;
                let sign_x = if (j % 2) == 0 { 1.0 } else { -1.0 };
                let sign_y = if ((j / 2) % 2) == 0 { 1.0 } else { -1.0 };
                let vx = sign_x * speed_x;
                let vy = sign_y * speed_y;
                out.push(CityWalker { kind, x, y, vx, vy });
            }
        }
        *walkers = out;
    }

    /// Met à jour les positions des marcheurs (vélocité en px/s, rebond dans la zone 0..1).
    fn update_city_walkers(walkers: &mut [CityWalker], delta_sec: f32, ground_rect: egui::Rect) {
        let w_px = ground_rect.width().max(1.0);
        let h_px = ground_rect.height().max(1.0);
        for w in walkers.iter_mut() {
            // Migration : ancienne vélocité normalisée (< 1) → convertir en px/s.
            let (vx_px, vy_px) = if w.vx.abs() < 1.0 && w.vy.abs() < 1.0 {
                (w.vx * w_px, w.vy * h_px)
            } else {
                (w.vx, w.vy)
            };
            let speed = (vx_px * vx_px + vy_px * vy_px).sqrt();
            let (vx_px, vy_px) = if speed > 0.0 {
                let s = speed.clamp(3.0, 6.0);
                (vx_px * s / speed, vy_px * s / speed)
            } else {
                (4.0, 4.0)
            };
            w.vx = vx_px;
            w.vy = vy_px;
            w.x += (w.vx / w_px) * delta_sec;
            w.y += (w.vy / h_px) * delta_sec;
            if w.x <= 0.0 {
                w.x = 0.0;
                w.vx = w.vx.abs();
            }
            if w.x >= 1.0 {
                w.x = 1.0;
                w.vx = -w.vx.abs();
            }
            if w.y <= 0.0 {
                w.y = 0.0;
                w.vy = w.vy.abs();
            }
            if w.y >= 1.0 {
                w.y = 1.0;
                w.vy = -w.vy.abs();
            }
        }
    }

    /// Dessine les marcheurs dans la zone sol (ground_rect).
    fn draw_city_walkers(ui: &egui::Ui, walkers: &[CityWalker], ground_rect: egui::Rect) {
        const RADIUS: f32 = 5.0;
        for w in walkers {
            let cx = ground_rect.min.x + w.x * ground_rect.width();
            let cy = ground_rect.min.y + w.y * ground_rect.height();
            let color = match w.kind {
                CityWalkerKind::Gens => egui::Color32::from_rgb(240, 220, 180),
                CityWalkerKind::Soldat => egui::Color32::from_rgb(180, 60, 50),
                CityWalkerKind::Macon => egui::Color32::from_rgb(120, 120, 120),
            };
            ui.painter().circle(
                egui::pos2(cx, cy),
                RADIUS,
                color,
                egui::Stroke::new(1.0, egui::Color32::from_gray(80)),
            );
        }
    }

    /// Écran Ma cité : layout blocs organiques (Header Central, header Lord of Click, zone cité, notification) + molécules (panneau ressources, panneau bâtiments).
    /// @id: miyuclicker_app_ui_ma_citee
    /// @do: render_ma_citee_screen
    /// @role: interface
    fn ui_ma_citee(&mut self, ctx: &egui::Context) {
        let delta = self.last_delta_s.min(0.1);
        if let Some(ref mut state) = self.game_state {
            let mult = game_speed_multiplier(self.game_speed_index);
            tick(state, delta, mult);
        }

        // Bloc organique : Header Central (barre ressources + nav + config).
        let organic_frame = egui::Frame::default()
            .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
            .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
        egui::TopBottomPanel::top("game_bar")
            .frame(organic_frame)
            .show(ctx, |ui| {
                self.ui_bar(ui);
            });

        // Bloc organique : header Lord of Click (titre jeu).
        let header_lord_frame = egui::Frame::default()
            .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
            .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
        egui::TopBottomPanel::top("header_lord_of_click")
            .min_height(LAYOUT_HEADER_LORD_HEIGHT)
            .frame(header_lord_frame)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Lord of Click");
                });
            });

        let outer_margin = egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I);
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(outer_margin))
            .show(ctx, |ui| {
                self.ui_ma_citee_central_content(ui);
            });

        self.ui_config_menu(ctx, true);
    }

    /// Contenu central de l'écran Carte du monde (carte, cités, routes, clic). Utilisé par ui_carte_monde et show_into.
    fn ui_carte_monde_central_content(&mut self, ui: &mut egui::Ui) {
        let game_over = self.game_state.as_ref().map(|s| s.game_over).unwrap_or(false);
        if game_over {
            Self::ui_game_over_overlay(ui, &mut self.screen);
            return;
        }
        let state = match &self.game_state {
            Some(s) => s,
            None => return,
        };
        let index_by_id: std::collections::HashMap<_, _> =
            state.cites.iter().enumerate().map(|(i, c)| (c.id.as_str(), i)).collect();
        let rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(rect, egui::Sense::click());
        let painter = ui.painter();
        for cite in &state.cites {
            let x = rect.min.x + cite.x * rect.width();
            let y = rect.min.y + cite.y * rect.height();
            let color = match cite.proprietaire {
                crate::state::Proprietaire::Joueur => egui::Color32::GREEN,
                crate::state::Proprietaire::Adverse(_) => egui::Color32::RED,
            };
            let r = 15.0;
            painter.circle(
                egui::pos2(x, y),
                r,
                color,
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );
            painter.text(
                egui::pos2(x + r + 5.0, y),
                egui::Align2::LEFT_CENTER,
                format!("{} ({})", cite.nom, cite.troupes),
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
        }
        for route in &state.routes {
            if let (Some(&ia), Some(&ib)) = (index_by_id.get(route.cite_a.as_str()), index_by_id.get(route.cite_b.as_str())) {
                let a = &state.cites[ia];
                let b = &state.cites[ib];
                let xa = rect.min.x + a.x * rect.width();
                let ya = rect.min.y + a.y * rect.height();
                let xb = rect.min.x + b.x * rect.width();
                let yb = rect.min.y + b.y * rect.height();
                painter.line_segment(
                    [egui::pos2(xa, ya), egui::pos2(xb, yb)],
                    egui::Stroke::new(2.0, egui::Color32::GRAY),
                );
            }
        }
        for dep in &state.deplacements {
            let (from_idx, to_idx) = (
                index_by_id.get(dep.from_cite_id.as_str()).copied(),
                index_by_id.get(dep.to_cite_id.as_str()).copied(),
            );
            if let (Some(from_idx), Some(to_idx)) = (from_idx, to_idx) {
                let from = &state.cites[from_idx];
                let to = &state.cites[to_idx];
                let xa = rect.min.x + from.x * rect.width();
                let ya = rect.min.y + from.y * rect.height();
                let xb = rect.min.x + to.x * rect.width();
                let yb = rect.min.y + to.y * rect.height();
                let x = xa + (xb - xa) * (dep.progress as f32);
                let y = ya + (yb - ya) * (dep.progress as f32);
                painter.circle(
                    egui::pos2(x, y),
                    8.0,
                    egui::Color32::YELLOW,
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                );
            }
        }
        if response.clicked() {
            let pos = response.interact_pointer_pos().unwrap_or_default();
            for cite in &state.cites {
                let cx = rect.min.x + cite.x * rect.width();
                let cy = rect.min.y + cite.y * rect.height();
                if (pos.x - cx).hypot(pos.y - cy) < 20.0 {
                    self.selected_cite_id = Some(cite.id.clone());
                    break;
                }
            }
        }
    }

    /// Panneau cité sélectionnée (contenu). Utilisé par ui_carte_monde et show_into.
    fn ui_cite_info_content(&mut self, ui: &mut egui::Ui) {
        let selected_id = match &self.selected_cite_id {
            Some(id) => id.clone(),
            None => return,
        };
        let state = match &self.game_state {
            Some(s) => s,
            None => return,
        };
        let cite = match state.cites.iter().find(|c| c.id == selected_id) {
            Some(c) => c,
            None => return,
        };
        let cite_id = cite.id.clone();
        let cite_nom = cite.nom.clone();
        let cite_troupes = cite.troupes;
        let cite_proprio = format!("{:?}", cite.proprietaire);
        let is_joueur = matches!(cite.proprietaire, crate::state::Proprietaire::Joueur);
        let others: Vec<_> = state.cites.iter()
            .filter(|c| c.id != cite_id)
            .map(|c| (c.id.clone(), c.nom.clone(), matches!(c.proprietaire, crate::state::Proprietaire::Adverse(_))))
            .collect();
        ui.heading(&cite_nom);
        ui.label(format!("Troupes: {}", cite_troupes));
        ui.label(format!("Propriétaire: {}", cite_proprio));
        if is_joueur {
            ui.add(
                egui::DragValue::new(&mut self.send_troops_count)
                    .range(1..=i64::MAX)
                    .speed(1)
                    .prefix("Envoyer: "),
            );
            for (other_id, other_nom, is_adverse) in &others {
                let label = if *is_adverse {
                    format!("Envoyer vers {} (adverse)", other_nom)
                } else {
                    format!("Envoyer vers {} (renfort)", other_nom)
                };
                if ui.button(label).clicked() {
                    self.pending_move_troops = Some((cite_id.clone(), other_id.clone(), self.send_troops_count));
                }
            }
        }
    }

    /// Écran Carte du monde : barre, carte, cités, routes, déplacements, panneau cité.
    /// @id: miyuclicker_app_ui_carte_monde
    /// @do: render_carte_monde_screen
    /// @role: interface
    fn ui_carte_monde(&mut self, ctx: &egui::Context) {
        let delta = self.last_delta_s.min(0.1);
        if let Some(ref mut state) = self.game_state {
            let mult = game_speed_multiplier(self.game_speed_index);
            tick(state, delta, mult);
            let game_delta = delta * mult;
            model_update(state, game_delta);
            if let Some((from, to, count)) = self.pending_move_troops.take() {
                let _ = move_troops(state, &from, &to, count);
            }
        }

        egui::TopBottomPanel::top("game_bar_carte").show(ctx, |ui| {
            self.ui_bar(ui);
        });

        let selected_id = self.selected_cite_id.clone();
        let outer_margin = egui::Margin::same(5i8);
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(outer_margin))
            .show(ctx, |ui| {
                self.ui_carte_monde_central_content(ui);
            });

        if selected_id.is_some() {
            egui::SidePanel::right("cite_info").show(ctx, |ui| {
                self.ui_cite_info_content(ui);
            });
        }

        self.ui_config_menu(ctx, true);
    }

    /// Rendu dans un `ui` fourni (intégration Miyukini Central).
    /// Point d'accès utilisateur unique : Central ; le service s'exécute dans le body de Central.
    pub fn show_into(&mut self, ui: &mut egui::Ui) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;
        self.last_delta_s = delta;
        self.frame_count = self.frame_count.wrapping_add(1);

        let in_game = matches!(self.screen, Screen::MaCitee | Screen::CarteMonde);
        let not_paused = self.game_speed_index != 0;
        if in_game && not_paused {
            ui.ctx().request_repaint();
        }

        if in_game && self.frame_count % 120 == 0 {
            let temps = self.game_state.as_ref().map(|s| s.temps_simule_s).unwrap_or(0.0);
            self.dev_log(format!(
                "frame {} delta={:.3}s temps_simule={:.0}s speed={}",
                self.frame_count,
                self.last_delta_s,
                temps,
                self.game_speed_index
            ));
        }

        ui.ctx().set_visuals(egui::Visuals::dark());

        match self.screen {
            Screen::Loading => {
                self.ui_loading_content(ui);
                self.loading_progress += 0.02;
                if self.loading_progress >= 1.0 {
                    self.loading_done = true;
                    self.screen = Screen::Landing;
                    self.slot_metadata = slot_list(&self.data_dir);
                }
                ui.ctx().request_repaint();
            }
            Screen::Landing => {
                self.ui_landing_content(ui);
                self.ui_config_menu(ui.ctx(), false);
            }
            Screen::Slots => {
                self.ui_slots_content(ui);
            }
            Screen::MaCitee => {
                let delta = self.last_delta_s.min(0.1);
                if let Some(ref mut state) = self.game_state {
                    let mult = game_speed_multiplier(self.game_speed_index);
                    tick(state, delta, mult);
                }
                // Bloc organique : Header Central.
                let organic_frame = egui::Frame::default()
                    .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
                    .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
                organic_frame.show(ui, |ui| self.ui_bar(ui));
                ui.add_space(LAYOUT_SPACING_BLOCKS);
                // Bloc organique : header Lord of Click.
                let header_lord_frame = egui::Frame::default()
                    .inner_margin(egui::Margin::same(LAYOUT_ORGANIC_MARGIN_I))
                    .stroke(egui::Stroke::new(2.0, color_organic_stroke()));
                header_lord_frame.show(ui, |ui| {
                    ui.set_min_height(LAYOUT_HEADER_LORD_HEIGHT);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.heading("Lord of Click");
                    });
                });
                ui.add_space(LAYOUT_SPACING_BLOCKS);
                self.ui_ma_citee_central_content(ui);
                self.ui_config_menu(ui.ctx(), true);
            }
            Screen::CarteMonde => {
                let delta = self.last_delta_s.min(0.1);
                if let Some(ref mut state) = self.game_state {
                    let mult = game_speed_multiplier(self.game_speed_index);
                    tick(state, delta, mult);
                    let game_delta = delta * mult;
                    model_update(state, game_delta);
                    if let Some((from, to, count)) = self.pending_move_troops.take() {
                        let _ = move_troops(state, &from, &to, count);
                    }
                }
                self.ui_bar(ui);
                ui.add_space(8.0);
                self.ui_carte_monde_central_content(ui);
                if self.selected_cite_id.is_some() {
                    ui.add_space(8.0);
                    egui::CollapsingHeader::new("Cité sélectionnée")
                        .default_open(true)
                        .show(ui, |ui| {
                            self.ui_cite_info_content(ui);
                        });
                }
                self.ui_config_menu(ui.ctx(), true);
            }
        }

        self.ui_dev_console(ui.ctx());
    }
}

impl App for MiyuClickerApp {
    /// Boucle principale : delta unique, repaint continu en jeu, log [TEST] espacé.
    /// @id: miyuclicker_app_update
    /// @do: main_loop_delta_repaint_log
    /// @role: interface
    /// @human: Un seul delta par frame ; request_repaint si jeu actif pour défilement fluide.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;
        self.last_delta_s = delta;
        self.frame_count = self.frame_count.wrapping_add(1);

        let in_game = matches!(self.screen, Screen::MaCitee | Screen::CarteMonde);
        let not_paused = self.game_speed_index != 0;
        if in_game && not_paused {
            ctx.request_repaint();
        }

        if in_game && self.frame_count % 120 == 0 {
            let temps = self.game_state.as_ref().map(|s| s.temps_simule_s).unwrap_or(0.0);
            self.dev_log(format!(
                "frame {} delta={:.3}s temps_simule={:.0}s speed={}",
                self.frame_count,
                self.last_delta_s,
                temps,
                self.game_speed_index
            ));
        }

        ctx.set_visuals(egui::Visuals::dark());
        match self.screen {
            Screen::Loading => self.ui_loading(ctx),
            Screen::Landing => self.ui_landing(ctx),
            Screen::Slots => self.ui_slots(ctx),
            Screen::MaCitee => self.ui_ma_citee(ctx),
            Screen::CarteMonde => self.ui_carte_monde(ctx),
        }

        self.ui_dev_console(ctx);
    }
}
