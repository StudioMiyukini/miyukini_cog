//! Application Lord of the Castle — écrans, zone de jeu, barre haute, input.
//! Aligné sur docs/services/MiyukiniSurvivor (Ecrans et UI, Gameplay).
//! Surface de combat 800×800 centrée au milieu du body ; déplacement ZQSD.

use crate::character_creation::{
    apply_phrase_effects, pick_three_phrases, CharacterStats, PhraseDef, Stat,
};
use crate::constants::PICKUP_RADIUS;
use crate::constants::{size, COMBAT_SURFACE_SIZE, ENEMY_VISION_RADIUS};
use crate::enemies::EnemyKind;
use crate::game_loop::{move_player, tick_battle};
use crate::game_state::{
    GamePhase, GameState, INVENTORY_MAX_SLOTS, EXPERT_IDENTIFY_COST_GOLD,
};
use crate::loot::{InventoryEntry, ItemSlot, LootKind};
use crate::player::{Dir8, Player};
use crate::towers::Tower;
use eframe::egui;
use eframe::App;
use std::cell::RefCell;
use std::time::Instant;

/// Écran courant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    /// Titre : Nouvelle partie.
    Title,
    /// Lore (texte + Skip).
    Lore,
    /// Saisie nom personnage et nom de sauvegarde.
    NameInput,
    /// Création de personnage, étape 0..=3 (4 écrans × 3 phrases).
    CharacterCreation(u8),
    /// Phase Préparation : sidebar, Lancer la vague.
    Preparation,
    /// Phase Bataille : zone de jeu, ennemis.
    Battle,
    /// Game over (Château à 0 PV).
    GameOver,
}

/// Application Lord of the Castle.
pub struct LordOfTheCastleApp {
    pub screen: Screen,
    pub game: Option<GameState>,
    pub last_tick: Instant,
    /// Touches enfoncées (8 directions).
    pub keys: [bool; 8],
    /// Fenêtre « Player » (métriques joueur) ouverte.
    pub player_window_open: bool,
    /// Fenêtre « Inventaire » ouverte.
    pub inventory_window_open: bool,
    /// Slot d’inventaire sélectionné pour détail / identification (fenêtre secondaire).
    pub selected_inventory_slot: Option<usize>,
    /// Fenêtre « Équipement » ouverte.
    pub equipment_window_open: bool,
    /// Slot d'équipement sélectionné pour détail (fenêtre secondaire).
    pub selected_equipment_slot: Option<ItemSlot>,
    // ——— Parcours « Nouvelle partie » ———
    /// Texte nom du personnage (écran NameInput).
    pub creation_name: String,
    /// Texte nom de la sauvegarde (écran NameInput).
    pub creation_save_name: String,
    /// IDs de phrases non encore affichées (création).
    pub creation_available_ids: Vec<usize>,
    /// Stats courantes pendant la création.
    pub creation_stats: CharacterStats,
    /// Si « Que Nawak décide... » a été choisi (reroll en fin).
    pub creation_reroll_pending: bool,
    /// Les 3 phrases proposées à l’étape courante.
    pub creation_current_choices: Vec<PhraseDef>,
}

impl Default for LordOfTheCastleApp {
    fn default() -> Self {
        Self {
            screen: Screen::Title,
            game: None,
            last_tick: Instant::now(),
            keys: [false; 8],
            player_window_open: false,
            inventory_window_open: false,
            selected_inventory_slot: None,
            equipment_window_open: false,
            selected_equipment_slot: None,
            creation_name: String::new(),
            creation_save_name: String::new(),
            creation_available_ids: Vec::new(),
            creation_stats: CharacterStats::default(),
            creation_reroll_pending: false,
            creation_current_choices: Vec::new(),
        }
    }
}

impl LordOfTheCastleApp {
    /// Nouvelle instance (avec contexte eframe pour polices, etc.).
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Démarre le parcours « Nouvelle partie » : écran Lore.
    pub fn start_new_game(&mut self) {
        self.game = None;
        self.screen = Screen::Lore;
        self.last_tick = Instant::now();
    }

    /// Termine la création et lance la partie (écran Préparation).
    fn finish_creation_and_start(&mut self) {
        let name = std::mem::take(&mut self.creation_name);
        let save_name = if self.creation_save_name.is_empty() {
            name.clone()
        } else {
            std::mem::take(&mut self.creation_save_name)
        };
        let mut stats = std::mem::take(&mut self.creation_stats);
        if self.creation_reroll_pending {
            stats.reroll_all(&mut crate::game_state::rand_simple);
        }
        let player = Player::from_creation(name, save_name, stats, 0.0, 0.0);
        self.game = Some(GameState::new_with_player(0.0, 0.0, player));
        self.screen = Screen::Preparation;
        self.creation_available_ids.clear();
        self.creation_reroll_pending = false;
        self.creation_current_choices.clear();
    }

    /// Crée une instance pour intégration dans Miyukini Central (point d'accès utilisateur unique).
    /// Le jeu s'exécute dans le body de Central ; pas de fenêtre standalone.
    pub fn new_embedded() -> Self {
        Self::default()
    }

    /// Rendu dans un `ui` fourni (intégration Miyukini Central).
    /// Point d'accès utilisateur unique : Central ; le service s'exécute dans le body de Central.
    pub fn show_into(&mut self, ui: &mut egui::Ui) {
        // Id à focus si l'utilisateur clique sur la zone de jeu (pour recevoir ZQSD)
        let focus_request = RefCell::new(None::<egui::Id>);
        // Clic « Lancer la vague » depuis la barre haute (toujours visible)
        let start_battle_request = RefCell::new(false);

        // Input clavier (8 directions) : ZQSD + flèches (Z=haut, Q=gauche, S=bas, D=droite)
        let input = ui.ctx().input(|i| i.clone());
        use egui::Key;
        let up = input.key_down(Key::Z) || input.key_down(Key::W) || input.key_down(Key::ArrowUp);
        let down = input.key_down(Key::S) || input.key_down(Key::ArrowDown);
        let left = input.key_down(Key::Q) || input.key_down(Key::A) || input.key_down(Key::ArrowLeft);
        let right = input.key_down(Key::D) || input.key_down(Key::ArrowRight);
        self.keys[0] = up && !left && !right;
        self.keys[1] = up && right;
        self.keys[2] = right && !up && !down;
        self.keys[3] = down && right;
        self.keys[4] = down && !left && !right;
        self.keys[5] = down && left;
        self.keys[6] = left && !up && !down;
        self.keys[7] = up && left;

        if self.screen == Screen::Battle {
            ui.ctx().request_repaint();
        }

        // Barre haute : Player, Inventaire, vague, ennemis, Lancer la vague, Mode Dev, Or
        ui.horizontal(|ui| {
            if ui.button("Player").clicked() {
                self.player_window_open = true;
            }
            if ui.button("Inventaire").clicked() {
                self.inventory_window_open = true;
                self.selected_inventory_slot = None;
            }
            if ui.button("Équipement").clicked() {
                self.equipment_window_open = true;
                self.selected_equipment_slot = None;
            }
            if let Some(ref mut state) = self.game {
                ui.label(format!("Vague {}", state.wave_number));
                ui.label(format!("Ennemis: {}", state.enemies.len()));
                if self.screen == Screen::Preparation
                    && ui.button("Lancer la vague").clicked()
                {
                    start_battle_request.replace(true);
                }
                if ui
                    .button(if state.dev_mode { "Mode Dev: ON" } else { "Mode Dev" })
                    .clicked()
                {
                    state.dev_mode = !state.dev_mode;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Or: {}", state.gold));
                });
            }
        });
        ui.add_space(8.0);

        // Fenêtre Player (métriques joueur)
        if self.player_window_open {
            let mut open = true;
            egui::Window::new("Player")
                .open(&mut open)
                .default_width(280.0)
                .show(ui.ctx(), |ui| paint_player_window(ui, self.game.as_ref()));
            if !open {
                self.player_window_open = false;
            }
        }

        // Fenêtre Inventaire (vue intégrée)
        if self.inventory_window_open {
            let mut open = true;
            egui::Window::new("Inventaire")
                .open(&mut open)
                .default_width(320.0)
                .default_height(400.0)
                .show(ui.ctx(), |ui| {
                    paint_inventory_window(ui, self.game.as_mut(), &mut self.selected_inventory_slot);
                });
            if !open {
                self.inventory_window_open = false;
                self.selected_inventory_slot = None;
            }
        }

        // Fenêtre Détail / Identification (vue intégrée)
        if self.selected_inventory_slot.is_some() {
            let slot_idx = self.selected_inventory_slot.unwrap();
            egui::Window::new("Détail objet")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    paint_item_detail_or_identify(ui, self, slot_idx);
                });
        }

        // Fenêtre Équipement (liste des slots + détail)
        if self.equipment_window_open {
            let mut open = true;
            egui::Window::new("Équipement")
                .open(&mut open)
                .default_width(280.0)
                .default_height(360.0)
                .show(ui.ctx(), |ui| {
                    paint_equipment_window(ui, self.game.as_mut(), &mut self.selected_equipment_slot);
                });
            if !open {
                self.equipment_window_open = false;
                self.selected_equipment_slot = None;
            }
        }
        if self.selected_equipment_slot.is_some() {
            let slot = self.selected_equipment_slot.unwrap();
            egui::Window::new("Détail équipement")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    paint_equipment_item_detail(ui, self, slot);
                });
        }

        // Contenu central selon l'écran
        match &self.screen {
            Screen::Title => {
                ui.vertical_centered(|ui| {
                    ui.heading("Lord of the Castle");
                    ui.label("Miyukini Survivor — Survivor + Tower Defense");
                    ui.add_space(20.0);
                    if ui.button("Nouvelle partie").clicked() {
                        self.start_new_game();
                    }
                    ui.add_space(4.0);
                    ui.colored_label(
                        ui.visuals().weak_text_color(),
                        "Attention : cela écrase la sauvegarde actuelle.",
                    );
                });
            }
            Screen::Lore => {
                ui.vertical_centered(|ui| {
                    ui.heading("Lore");
                    ui.add_space(12.0);
                    ui.label("Tu es le seigneur de ton domaine qui est attaqué par des mort-vivants.");
                    ui.label("Protège tes terres et ton château.");
                    ui.add_space(24.0);
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Skip").clicked() {
                            self.screen = Screen::NameInput;
                            self.creation_name.clear();
                            self.creation_save_name.clear();
                        }
                    });
                });
            }
            Screen::NameInput => {
                ui.vertical_centered(|ui| {
                    ui.heading("Création du personnage");
                    ui.label("Entre le nom de ton personnage et le nom de la sauvegarde.");
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.label("Nom du personnage :");
                        ui.add(egui::TextEdit::singleline(&mut self.creation_name).desired_width(200.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Nom de la sauvegarde :");
                        ui.add(egui::TextEdit::singleline(&mut self.creation_save_name).desired_width(200.0));
                    });
                    ui.label("(Si vide, le nom du personnage sera utilisé)");
                    ui.add_space(16.0);
                    if ui.button("Valider").clicked() {
                        let name = self.creation_name.trim();
                        if !name.is_empty() {
                            self.screen = Screen::CharacterCreation(0);
                            self.creation_available_ids = (0..=24).collect();
                            self.creation_stats = CharacterStats::default();
                            self.creation_reroll_pending = false;
                            self.creation_current_choices = pick_three_phrases(
                                &mut self.creation_available_ids,
                                &mut crate::game_state::rand_simple,
                            );
                        }
                    }
                });
            }
            Screen::CharacterCreation(step) => {
                let step = *step;
                let mut chosen: Option<usize> = None;
                ui.vertical_centered(|ui| {
                    ui.heading(format!("Création du personnage — Étape {}/4", step + 1));
                    ui.add_space(8.0);
                    ui.label("Choisis une phrase qui te décrit :");
                    ui.add_space(12.0);
                    for (i, phrase) in self.creation_current_choices.iter().enumerate() {
                        if ui.button(phrase.text).clicked() {
                            chosen = Some(i);
                        }
                    }
                    ui.add_space(16.0);
                    ui.collapsing("Caractéristiques actuelles", |ui| {
                        let s = &self.creation_stats;
                        ui.label(format!("For: {}  Con: {}  Agi: {}  Dex: {}", s.display(Stat::For), s.display(Stat::Con), s.display(Stat::Agi), s.display(Stat::Dex)));
                        ui.label(format!("Int: {}  Sag: {}  Cha: {}  Luk: {}", s.display(Stat::Int), s.display(Stat::Sag), s.display(Stat::Cha), s.display(Stat::Luk)));
                    });
                });
                if let Some(i) = chosen {
                    let phrase = self.creation_current_choices[i].clone();
                    let reroll = apply_phrase_effects(
                        &mut self.creation_stats,
                        &phrase.effects,
                        &mut crate::game_state::rand_simple,
                    );
                    if reroll {
                        self.creation_reroll_pending = true;
                    }
                    if step < 3 {
                        self.screen = Screen::CharacterCreation(step + 1);
                        self.creation_current_choices = pick_three_phrases(
                            &mut self.creation_available_ids,
                            &mut crate::game_state::rand_simple,
                        );
                    } else {
                        self.finish_creation_and_start();
                    }
                }
            }
            Screen::Preparation => {
                if let Some(ref mut state) = self.game {
                    let last_tick = self.last_tick;
                    let keys = self.keys;
                    let result = RefCell::new((last_tick, false));
                    let transition = RefCell::new(None::<Screen>);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let available = ui.available_rect_before_wrap();
                            let (game_rect, resp) = allocate_combat_surface_centered(ui, available);
                            if resp.clicked() {
                                focus_request.replace(Some(resp.id));
                            }
                            let painter = ui.painter();
                            let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                            let (tick, go) = game_zone_tick_and_paint(
                                game_rect,
                                &painter,
                                state,
                                last_tick,
                                keys,
                                ui.visuals().window_fill(),
                                cursor_screen,
                            );
                            *result.borrow_mut() = (tick, go);
                        });
                        ui.vertical(|ui| {
                            ui.set_min_width(180.0);
                            ui.heading("Préparation");
                            ui.label("Construisez des tours (50 or).");
                            ui.add_space(8.0);
                            if ui.button("Lancer la vague").clicked() {
                                state.start_battle_phase();
                                transition.replace(Some(Screen::Battle));
                            }
                            ui.add_space(16.0);
                            ui.label(format!("Or: {}", state.gold));
                        });
                    });
                    let (new_tick, game_over) = *result.borrow();
                    self.last_tick = new_tick;
                    if game_over {
                        self.screen = Screen::GameOver;
                    }
                    let screen_transition = transition.borrow_mut().take();
                    if let Some(s) = screen_transition {
                        self.screen = s;
                    }
                }
            }
            Screen::Battle => {
                if let Some(ref mut state) = self.game {
                    let available = ui.available_rect_before_wrap();
                    let (game_rect, resp) = allocate_combat_surface_centered(ui, available);
                    if resp.clicked() {
                        focus_request.replace(Some(resp.id));
                    }
                    let painter = ui.painter();
                    let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                    let (new_tick, game_over) = game_zone_tick_and_paint(
                        game_rect,
                        &painter,
                        state,
                        self.last_tick,
                        self.keys,
                        ui.visuals().window_fill(),
                        cursor_screen,
                    );
                    self.last_tick = new_tick;
                    if game_over {
                        self.screen = Screen::GameOver;
                    }
                    // Overlay victoire de vague (même logique que l'autre branche Battle)
                    if state.is_wave_won() {
                        let rect = game_rect;
                        egui::Area::new(egui::Id::new("wave_won_overlay_2"))
                            .order(egui::Order::Foreground)
                            .fixed_pos(rect.min)
                            .constrain(true)
                            .show(ui.ctx(), |ui| {
                                ui.set_min_size(rect.size());
                                let frame = egui::Frame::new()
                                    .fill(egui::Color32::from_white_alpha(240))
                                    .corner_radius(8.0);
                                frame.show(ui, |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(60.0);
                                        ui.heading("Vague terminée !");
                                        ui.label("Félicitations.");
                                        ui.add_space(16.0);
                                        ui.label("Résumé de la vague :");
                                        ui.label(format!("Ennemis tués : {}", state.enemies_killed_this_wave));
                                        ui.label(format!("Or collecté : {} or", state.gold_collected_this_wave));
                                        ui.label(format!("XP gagnée : {}", state.xp_gained_this_wave));
                                        ui.label(format!("Objets trouvés : {}", state.items_found_this_wave));
                                        ui.add_space(24.0);
                                        if ui.button("Phase suivante").clicked() {
                                            state.start_preparation_phase();
                                            self.screen = Screen::Preparation;
                                        }
                                    });
                                });
                            });
                    }
                }
            }
            Screen::GameOver => {
                ui.vertical_centered(|ui| {
                    ui.heading("Game Over");
                    ui.label("Le Château a été détruit.");
                    ui.add_space(20.0);
                    if ui.button("Retour au titre").clicked() {
                        self.screen = Screen::Title;
                        self.game = None;
                    }
                });
            }
        }

        // Log des dégâts joueur (en bas) — Bataille ou Préparation
        if let Some(ref state) = self.game {
            if self.screen == Screen::Battle || self.screen == Screen::Preparation {
                ui.add_space(8.0);
                egui::CollapsingHeader::new("Dégâts infligés par le joueur")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.set_max_height(100.0);
                        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                            for line in state.player_damage_log.iter().rev() {
                                ui.label(line);
                            }
                            if state.player_damage_log.is_empty() {
                                ui.weak("Aucun dégât infligé pour l'instant.");
                            }
                        });
                    });
            }
        }

        // Donner le focus clavier à la zone de jeu si l'utilisateur a cliqué dessus (ZQSD).
        let focus_id = focus_request.borrow_mut().take();
        if let Some(id) = focus_id {
            ui.ctx().memory_mut(|m| m.request_focus(id));
        }

        // Appliquer « Lancer la vague » cliqué depuis la barre haute.
        if start_battle_request.replace(false) {
            if let Some(ref mut state) = self.game {
                state.start_battle_phase();
            }
            self.screen = Screen::Battle;
        }
    }
}

/// Taille de la surface de combat (800×800 max, ou moins si zone insuffisante). Carré.
fn combat_surface_size(available: egui::Rect) -> egui::Vec2 {
    let w = COMBAT_SURFACE_SIZE.min(available.width());
    let h = COMBAT_SURFACE_SIZE.min(available.height());
    let side = w.min(h);
    egui::Vec2::new(side, side)
}

/// Alloue la surface de combat centrée dans la zone disponible (espacements explicites pour centrage vertical et horizontal).
/// Retourne (game_rect, response) pour demander le focus au clic.
fn allocate_combat_surface_centered(
    ui: &mut egui::Ui,
    available: egui::Rect,
) -> (egui::Rect, egui::Response) {
    let size = combat_surface_size(available);
    let top_space = (available.height() - size.y) / 2.0;
    let bottom_space = available.height() - size.y - top_space;
    let left_space = (available.width() - size.x) / 2.0;
    let right_space = available.width() - size.x - left_space;
    ui.add_space(top_space.max(0.0));
    let response = ui.horizontal(|ui| {
        ui.add_space(left_space.max(0.0));
        let rect = egui::Rect::from_min_size(ui.cursor().min, size);
        let response = ui.allocate_rect(rect, egui::Sense::click());
        ui.add_space(right_space.max(0.0));
        (rect, response)
    });
    let (game_rect, resp) = response.inner;
    ui.add_space(bottom_space.max(0.0));
    (game_rect, resp)
}

/// Monde → écran : château au centre du rect (centre monde = centre de la surface 800×800).
fn world_to_screen(rect: egui::Rect, wx: f32, wy: f32) -> egui::Pos2 {
    let cx = rect.center().x;
    let cy = rect.center().y;
    egui::Pos2::new(cx + wx, cy + wy)
}

/// Écran → monde : inverse de world_to_screen (pour position curseur → coords monde).
fn screen_to_world(rect: egui::Rect, sx: f32, sy: f32) -> (f32, f32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    (sx - cx, sy - cy)
}

/// Fenêtre fiche joueur : nom, niveau, PV/Mana, XP, caractéristiques, statistiques.
fn paint_player_window(ui: &mut egui::Ui, state: Option<&GameState>) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    let p = &state.player;
    let name = if p.name.is_empty() { "Joueur" } else { p.name.as_str() };

    // [Nom du personnage] [Niveau]
    ui.horizontal(|ui| {
        ui.heading(name);
        ui.label(format!("Niveau {}", state.level));
    });
    ui.add_space(4.0);

    // [xx/yy PV] - [xx/yy Mana]
    ui.label(format!("{} / {} PV  —  {} / {} Mana", p.hp, p.hp_max, p.mana, p.mana_max));
    ui.add_space(2.0);

    // xxx / yyyy XP pour le prochain niveau
    let xp_next = state.xp_required_for_next_level();
    ui.label(format!("{} / {} XP pour le prochain niveau", state.xp, xp_next));
    ui.add_space(8.0);

    // Caractéristiques (collapse) : 2 colonnes
    ui.collapsing("Caractéristiques", |ui| {
        let s = &p.stats;
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("Force (For) : {}", s.display(Stat::For)));
                ui.label(format!("Constitution (Con) : {}", s.display(Stat::Con)));
                ui.label(format!("Agilité (Agi) : {}", s.display(Stat::Agi)));
                ui.label(format!("Dextérité (Dex) : {}", s.display(Stat::Dex)));
            });
            ui.vertical(|ui| {
                ui.label(format!("Intelligence (Int) : {}", s.display(Stat::Int)));
                ui.label(format!("Sagesse (Sag) : {}", s.display(Stat::Sag)));
                ui.label(format!("Charisme (Cha) : {}", s.display(Stat::Cha)));
                ui.label(format!("Chance (Luk) : {}", s.display(Stat::Luk)));
            });
        });
    });

    // Statistiques (collapse)
    let dmg = p.auto_attack_damage(); // à main nue : Force + dizaine(Constitution)
    let interval_s = Player::auto_attack_interval_s();
    let attack_speed = 1.0 / interval_s;
    ui.collapsing("Statistiques", |ui| {
        ui.label(format!("Dégât (à main nue) : {}", dmg));
        ui.label(format!("Vitesse de déplacement : {:.0} px/s", p.move_speed()));
        ui.label(format!("Vitesse d'attaque : {:.1}/s", attack_speed));
        ui.label(format!("Rayon d'attaque : {} px", Player::auto_attack_range()));
        ui.label(format!("Rayon de collecte : {} px", PICKUP_RADIUS));
        ui.label(format!("Armure : {}", p.armor));
        ui.label(format!("Résistance magique : {}", p.magic_resist));
        ui.add_space(4.0);
        ui.separator();
        ui.label(format!("Nombre d'ennemis tués : {}", state.enemies_killed));
        ui.label(format!("Vague max : {}", state.max_wave_reached));
        ui.label(format!("Nombre de Boss tués : {}", state.bosses_killed));
        ui.label(format!("Or cumulés : {}", state.gold_total));
    });
}

/// Fenêtre inventaire : [xx/xx slots], liste des objets. Clic = sélection pour détail / identification.
fn paint_inventory_window(
    ui: &mut egui::Ui,
    state: Option<&mut GameState>,
    selected_slot: &mut Option<usize>,
) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    let n = state.inventory.len();
    ui.label(format!("[{}/{} slots]", n, INVENTORY_MAX_SLOTS));
    ui.add_space(6.0);
    ui.separator();
    for (i, entry) in state.inventory.iter().enumerate() {
        let (label, color) = match entry {
            InventoryEntry::Unidentified(slot) => {
                (format!("Objet non-identifié : {}", slot.label()), egui::Color32::GRAY)
            }
            InventoryEntry::Identified(item) => {
                let (r, g, b) = item.rarity.color_rgb();
                (
                    format!("{} — {} — {}", item.display_name, item.slot.label(), item.rarity.label()),
                    egui::Color32::from_rgb(r, g, b),
                )
            }
        };
        let btn = egui::Button::new(egui::RichText::new(label).color(color));
        if ui.add(btn).clicked() {
            *selected_slot = Some(i);
        }
    }
}

/// Fenêtre équipement : liste des slots, objet équipé ou "Vide". Clic sur un objet = détail (selected_equipment_slot).
fn paint_equipment_window(
    ui: &mut egui::Ui,
    state: Option<&mut GameState>,
    selected_slot: &mut Option<ItemSlot>,
) {
    let Some(state) = state else {
        ui.label("Aucune partie en cours.");
        return;
    };
    for &slot in ItemSlot::equipment_slots() {
        let label = match state.get_equipped(slot) {
            Some(item) => format!("{} : {}", slot.label(), item.display_name),
            None => format!("{} : Vide", slot.label()),
        };
        let has_item = state.get_equipped(slot).is_some();
        let btn = egui::Button::new(label);
        if ui.add(btn).clicked() && has_item {
            *selected_slot = Some(slot);
        }
    }
}

/// Fenêtre détail (objet identifié) ou identification (objet non identifié). Ferme en mettant app.selected_inventory_slot = None.
fn paint_item_detail_or_identify(
    ui: &mut egui::Ui,
    app: &mut LordOfTheCastleApp,
    slot_idx: usize,
) {
    let Some(ref mut state) = app.game else {
        app.selected_inventory_slot = None;
        return;
    };
    let Some(entry) = state.inventory.get(slot_idx) else {
        app.selected_inventory_slot = None;
        return;
    };
    let mut close_window = false;
    let mut do_sell = false;
    let mut do_equip = false;
    let mut do_identify_self = false;
    let mut do_identify_expert = false;

    match entry {
        InventoryEntry::Unidentified(slot) => {
            ui.label("Objet non-identifié");
            ui.label(format!("Type : {}", slot.label()));
            if state.phase == GamePhase::Preparation {
                ui.add_space(8.0);
                ui.label("Voulez-vous identifier cet objet ?");
                ui.horizontal(|ui| {
                    let can_self = !state.identified_this_phase;
                    if ui.add_enabled(can_self, egui::Button::new("Moi-même")).clicked() {
                        do_identify_self = true;
                    }
                    let can_expert = state.gold >= EXPERT_IDENTIFY_COST_GOLD;
                    if ui
                        .add_enabled(can_expert, egui::Button::new("Par un expert (100 or)"))
                        .clicked()
                    {
                        do_identify_expert = true;
                    }
                    if ui.button("Annulé").clicked() {
                        close_window = true;
                    }
                });
            } else {
                ui.add_space(4.0);
                ui.label("Identifiable en phase Préparation.");
                if ui.button("Fermer").clicked() {
                    close_window = true;
                }
            }
        }
        InventoryEntry::Identified(item) => {
            let (r, g, b) = item.rarity.color_rgb();
            let name = item.display_name.clone();
            let effects = item.effects_text();
            let price = item.sell_price();
            let slot_label = item.slot.label();
            let rarity_label = item.rarity.label();
            ui.heading(egui::RichText::new(name).color(egui::Color32::from_rgb(r, g, b)));
            ui.label(format!("{} — {}", slot_label, rarity_label));
            ui.add_space(4.0);
            ui.label("Effets :");
            ui.label(effects);
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(format!("Prix de vente : {} or", price));
                if ui.button("Vendre").clicked() {
                    do_sell = true;
                }
            });
            let can_equip = state.can_equip_from_inventory(slot_idx);
            if ui.add_enabled(can_equip, egui::Button::new("Équiper")).clicked() {
                do_equip = true;
            }
            if ui.button("Fermer").clicked() {
                close_window = true;
            }
        }
    }

    if do_identify_self {
        if let Some(ref mut state) = app.game {
            state.identify_self(slot_idx);
        }
        app.selected_inventory_slot = None;
    } else if do_identify_expert {
        if let Some(ref mut state) = app.game {
            state.identify_expert(slot_idx);
        }
        app.selected_inventory_slot = None;
    } else if do_sell {
        if let Some(ref mut state) = app.game {
            state.sell_item(slot_idx);
        }
        app.selected_inventory_slot = None;
    } else if do_equip {
        if let Some(ref mut state) = app.game {
            if state.equip_item(slot_idx) {
                app.selected_inventory_slot = None;
            }
        }
    } else if close_window {
        app.selected_inventory_slot = None;
    }
}

/// Fenêtre détail d'un objet équipé : infos + bouton Déséquiper (grisé si inventaire plein).
fn paint_equipment_item_detail(
    ui: &mut egui::Ui,
    app: &mut LordOfTheCastleApp,
    slot: ItemSlot,
) {
    let Some(ref mut state) = app.game else {
        app.selected_equipment_slot = None;
        return;
    };
    let Some(item) = state.get_equipped(slot).cloned() else {
        app.selected_equipment_slot = None;
        return;
    };
    let (r, g, b) = item.rarity.color_rgb();
    ui.heading(egui::RichText::new(item.display_name.clone()).color(egui::Color32::from_rgb(r, g, b)));
    ui.label(format!("{} — {}", item.slot.label(), item.rarity.label()));
    ui.add_space(4.0);
    ui.label("Effets :");
    ui.label(item.effects_text());
    ui.add_space(8.0);
    let can_unequip = state.inventory.len() < INVENTORY_MAX_SLOTS;
    if ui.add_enabled(can_unequip, egui::Button::new("Déséquiper")).clicked() {
        if state.unequip_to_inventory(slot) {
            app.selected_equipment_slot = None;
        }
    }
    if ui.button("Fermer").clicked() {
        app.selected_equipment_slot = None;
    }
}

fn paint_castle(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let half = size::CASTLE / 2.0;
    let min = world_to_screen(rect, state.castle.x - half, state.castle.y - half);
    let max = world_to_screen(rect, state.castle.x + half, state.castle.y + half);
    painter.rect_filled(
        egui::Rect::from_min_max(min, max),
        0.0,
        egui::Color32::from_rgb(0, 180, 0),
    );
}

fn paint_player(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let half = size::MOBILE / 2.0;
    let min = world_to_screen(rect, state.player.x - half, state.player.y - half);
    let max = world_to_screen(rect, state.player.x + half, state.player.y + half);
    let color = if state.player.dead {
        egui::Color32::from_rgb(100, 50, 0)
    } else {
        egui::Color32::from_rgb(255, 165, 0)
    };
    painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
}

fn paint_enemies(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    for e in &state.enemies {
        let half = e.half_size();
        let min = world_to_screen(rect, e.x - half, e.y - half);
        let max = world_to_screen(rect, e.x + half, e.y + half);
        let color = if e.is_flashing() {
            egui::Color32::from_rgb(255, 255, 220)
        } else {
            match e.kind {
                EnemyKind::Normal => egui::Color32::from_rgb(139, 90, 43),
                EnemyKind::MiniBoss => egui::Color32::from_rgb(255, 165, 0),
                EnemyKind::Boss => egui::Color32::from_rgb(200, 0, 0),
            }
        };
        painter.rect_filled(egui::Rect::from_min_max(min, max), 0.0, color);
    }
}

fn paint_towers(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let half = size::TOWER / 2.0;
    for t in &state.towers {
        if t.hp <= 0 {
            continue;
        }
        let min = world_to_screen(rect, t.x - half, t.y - half);
        let max = world_to_screen(rect, t.x + half, t.y + half);
        let r = egui::Rect::from_min_max(min, max);
        painter.rect_filled(r, 0.0, egui::Color32::from_rgb(100, 100, 100));
        painter.rect_stroke(
            r,
            0.0,
            (1.0, egui::Color32::from_rgb(60, 60, 60)),
            egui::StrokeKind::Outside,
        );
    }
}

/// Barre de vie verte sous une entité (monde → écran). hp/hp_max = % affiché.
fn paint_health_bar(
    painter: &egui::Painter,
    rect: egui::Rect,
    world_x: f32,
    world_y: f32,
    entity_half_size: f32,
    hp: i32,
    hp_max: i32,
) {
    if hp_max <= 0 {
        return;
    }
    let bar_w = entity_half_size * 2.4f32;
    let bar_h = 4.0;
    let offset_y = entity_half_size + 4.0;
    let min = world_to_screen(rect, world_x - bar_w / 2.0, world_y + offset_y);
    let max_bg = world_to_screen(rect, world_x + bar_w / 2.0, world_y + offset_y + bar_h);
    let bg_rect = egui::Rect::from_min_max(min, max_bg);
    painter.rect_filled(bg_rect, 0.0, egui::Color32::from_rgb(40, 40, 40));
    let t = (hp as f32 / hp_max as f32).clamp(0.0, 1.0);
    let max_fill = world_to_screen(rect, world_x - bar_w / 2.0 + bar_w * t, world_y + offset_y + bar_h);
    let fill_rect = egui::Rect::from_min_max(min, max_fill);
    painter.rect_filled(fill_rect, 0.0, egui::Color32::from_rgb(0, 200, 0));
}

/// Barres de vie sous les entités blessées (PV < PV max).
fn paint_health_bars(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    if state.castle.hp < state.castle.hp_max && state.castle.hp > 0 {
        paint_health_bar(
            painter,
            rect,
            state.castle.x,
            state.castle.y,
            size::CASTLE / 2.0,
            state.castle.hp,
            state.castle.hp_max,
        );
    }
    if state.player.hp < state.player.hp_max && state.player.hp > 0 {
        paint_health_bar(
            painter,
            rect,
            state.player.x,
            state.player.y,
            Player::half_size(),
            state.player.hp,
            state.player.hp_max,
        );
    }
    for e in &state.enemies {
        if e.hp < e.hp_max {
            paint_health_bar(painter, rect, e.x, e.y, e.half_size(), e.hp, e.hp_max);
        }
    }
    for t in &state.towers {
        if t.hp > 0 && t.hp < t.hp_max {
            paint_health_bar(
                painter,
                rect,
                t.x,
                t.y,
                Tower::half_size(),
                t.hp,
                t.hp_max,
            );
        }
    }
}

/// Pixels de loot au sol : jaune = or, bleu = xp, marron = objet.
fn paint_loot(painter: &egui::Painter, rect: egui::Rect, state: &GameState) {
    let pixel = 6.0f32; // demi-côté du pixel
    for drop_ in &state.loot_drops {
        let min = world_to_screen(rect, drop_.x - pixel, drop_.y - pixel);
        let max = world_to_screen(rect, drop_.x + pixel, drop_.y + pixel);
        let r = egui::Rect::from_min_max(min, max);
        let color = match &drop_.kind {
            LootKind::Gold(_) => egui::Color32::from_rgb(255, 215, 0),   // jaune or
            LootKind::Xp(_) => egui::Color32::from_rgb(70, 130, 255),    // bleu xp
            LootKind::Item(_) => egui::Color32::from_rgb(139, 90, 43),  // marron objet
        };
        painter.rect_filled(r, 0.0, color);
    }
}

/// Cercles de portée (mode dev) : bleu joueur (cercle + cône 40°), rouge ennemis, orange tours.
/// cursor_world : position curseur en coords monde (pour tracer le cône d’attaque vers le curseur).
fn paint_dev_ranges(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &GameState,
    cursor_world: Option<(f32, f32)>,
) {
    if !state.dev_mode {
        return;
    }
    let stroke = 1.5f32;
    let px = state.player.x;
    let py = state.player.y;
    let r_player = Player::auto_attack_range();
    let center = world_to_screen(rect, px, py);
    // Joueur : cercle bleu (rayon de portée 40 px)
    painter.circle_stroke(center, r_player, (stroke, egui::Color32::from_rgb(0, 100, 255)));
    // Joueur : deux segments délimitant le cône 40° (pointe joueur, axe vers curseur ou direction)
    let aim_angle_rad = cursor_world
        .map(|(cx, cy)| (cy - py).atan2(cx - px))
        .unwrap_or_else(|| state.player.dir.to_angle_rad());
    let half_cone_rad = 20.0_f32.to_radians();
    let left_angle = aim_angle_rad - half_cone_rad;
    let right_angle = aim_angle_rad + half_cone_rad;
    let tip_left = world_to_screen(
        rect,
        px + r_player * left_angle.cos(),
        py + r_player * left_angle.sin(),
    );
    let tip_right = world_to_screen(
        rect,
        px + r_player * right_angle.cos(),
        py + r_player * right_angle.sin(),
    );
    painter.line_segment(
        [center, tip_left],
        (stroke, egui::Color32::from_rgb(0, 150, 255)),
    );
    painter.line_segment(
        [center, tip_right],
        (stroke, egui::Color32::from_rgb(0, 150, 255)),
    );
    // Ennemis : rouge, vision 30 px
    for e in &state.enemies {
        let center = world_to_screen(rect, e.x, e.y);
        painter.circle_stroke(center, ENEMY_VISION_RADIUS, (stroke, egui::Color32::from_rgb(255, 0, 0)));
    }
    // Tours : orange, 80 px
    for t in &state.towers {
        if t.hp <= 0 {
            continue;
        }
        let center = world_to_screen(rect, t.x, t.y);
        painter.circle_stroke(center, Tower::range(), (stroke, egui::Color32::from_rgb(255, 165, 0)));
    }
}

/// Tick + peinture de la zone de jeu. Retourne (nouvel instant, game_over).
/// cursor_screen : position du curseur en coords écran (pour cône d’attaque joueur).
fn game_zone_tick_and_paint(
    rect: egui::Rect,
    painter: &egui::Painter,
    state: &mut GameState,
    last_tick: Instant,
    keys: [bool; 8],
    window_fill: egui::Color32,
    cursor_screen: Option<egui::Pos2>,
) -> (Instant, bool) {
    painter.rect_filled(rect, 0.0, window_fill.linear_multiply(0.5));

    let cursor_world = cursor_screen
        .filter(|p| rect.contains(*p))
        .map(|p| screen_to_world(rect, p.x, p.y));

    let now = Instant::now();
    if state.phase == GamePhase::Battle {
        let delta = last_tick.elapsed().as_secs_f32();
        if keys[0] {
            move_player(state, Dir8::N, delta);
        }
        if keys[1] {
            move_player(state, Dir8::NE, delta);
        }
        if keys[2] {
            move_player(state, Dir8::E, delta);
        }
        if keys[3] {
            move_player(state, Dir8::SE, delta);
        }
        if keys[4] {
            move_player(state, Dir8::S, delta);
        }
        if keys[5] {
            move_player(state, Dir8::SW, delta);
        }
        if keys[6] {
            move_player(state, Dir8::W, delta);
        }
        if keys[7] {
            move_player(state, Dir8::NW, delta);
        }
        tick_battle(state, delta, cursor_world);
    }

    paint_towers(painter, rect, state);
    paint_enemies(painter, rect, state);
    paint_castle(painter, rect, state);
    paint_player(painter, rect, state);
    paint_health_bars(painter, rect, state);
    paint_loot(painter, rect, state);
    paint_dev_ranges(painter, rect, state, cursor_world);

    // Ne pas passer en Préparation ici : l'app affiche l'overlay victoire et le joueur clique "Phase suivante".
    (now, state.is_game_over())
}

impl App for LordOfTheCastleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Input clavier (8 directions) : ZQSD + flèches (Z=haut, Q=gauche, S=bas, D=droite).
        let input = ctx.input(|i| i.clone());
        use egui::Key;
        let up = input.key_down(Key::Z) || input.key_down(Key::W) || input.key_down(Key::ArrowUp);
        let down = input.key_down(Key::S) || input.key_down(Key::ArrowDown);
        let left = input.key_down(Key::Q) || input.key_down(Key::A) || input.key_down(Key::ArrowLeft);
        let right = input.key_down(Key::D) || input.key_down(Key::ArrowRight);
        self.keys[0] = up && !left && !right;
        self.keys[1] = up && right;
        self.keys[2] = right && !up && !down;
        self.keys[3] = down && right;
        self.keys[4] = down && !left && !right;
        self.keys[5] = down && left;
        self.keys[6] = left && !up && !down;
        self.keys[7] = up && left;

        egui::TopBottomPanel::top("lotc_top")
            .min_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Player").clicked() {
                        self.player_window_open = true;
                    }
                    if ui.button("Inventaire").clicked() {
                        self.inventory_window_open = true;
                        self.selected_inventory_slot = None;
                    }
                    if ui.button("Équipement").clicked() {
                        self.equipment_window_open = true;
                        self.selected_equipment_slot = None;
                    }
                    if let Some(ref mut state) = self.game {
                        ui.label(format!("Vague {}", state.wave_number));
                        ui.label(format!("Ennemis: {}", state.enemies.len()));
                        if ui
                            .button(if state.dev_mode { "Mode Dev: ON" } else { "Mode Dev" })
                            .clicked()
                        {
                            state.dev_mode = !state.dev_mode;
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Or: {}", state.gold));
                        });
                    }
                });
            });

        // Fenêtre Player (métriques joueur) — standalone
        if self.player_window_open {
            let mut open = true;
            egui::Window::new("Player")
                .open(&mut open)
                .default_width(280.0)
                .show(ctx, |ui| paint_player_window(ui, self.game.as_ref()));
            if !open {
                self.player_window_open = false;
            }
        }

        // Fenêtre Inventaire
        if self.inventory_window_open {
            let mut open = true;
            egui::Window::new("Inventaire")
                .open(&mut open)
                .default_width(320.0)
                .default_height(400.0)
                .show(ctx, |ui| {
                    paint_inventory_window(ui, self.game.as_mut(), &mut self.selected_inventory_slot);
                });
            if !open {
                self.inventory_window_open = false;
                self.selected_inventory_slot = None;
            }
        }

        // Fenêtre Détail / Identification (objet sélectionné)
        if self.selected_inventory_slot.is_some() {
            let slot_idx = self.selected_inventory_slot.unwrap();
            egui::Window::new("Détail objet")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    paint_item_detail_or_identify(ui, self, slot_idx);
                });
        }

        // Fenêtre Équipement
        if self.equipment_window_open {
            let mut open = true;
            egui::Window::new("Équipement")
                .open(&mut open)
                .default_width(280.0)
                .default_height(360.0)
                .show(ctx, |ui| {
                    paint_equipment_window(ui, self.game.as_mut(), &mut self.selected_equipment_slot);
                });
            if !open {
                self.equipment_window_open = false;
                self.selected_equipment_slot = None;
            }
        }
        if self.selected_equipment_slot.is_some() {
            let slot = self.selected_equipment_slot.unwrap();
            egui::Window::new("Détail équipement")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    paint_equipment_item_detail(ui, self, slot);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screen::Title => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Lord of the Castle");
                        ui.label("Miyukini Survivor — Survivor + Tower Defense");
                        ui.add_space(20.0);
                        if ui.button("Nouvelle partie").clicked() {
                            self.start_new_game();
                        }
                        ui.add_space(4.0);
                        ui.colored_label(
                            ui.visuals().weak_text_color(),
                            "Attention : cela écrase la sauvegarde actuelle.",
                        );
                    });
                }
                Screen::Lore => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Lore");
                        ui.add_space(12.0);
                        ui.label("Tu es le seigneur de ton domaine qui est attaqué par des mort-vivants.");
                        ui.label("Protège tes terres et ton château.");
                        ui.add_space(24.0);
                    });
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Skip").clicked() {
                                self.screen = Screen::NameInput;
                                self.creation_name.clear();
                                self.creation_save_name.clear();
                            }
                        });
                    });
                }
                Screen::NameInput => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Création du personnage");
                        ui.label("Entre le nom de ton personnage et le nom de la sauvegarde.");
                        ui.add_space(16.0);
                        ui.horizontal(|ui| {
                            ui.label("Nom du personnage :");
                            ui.add(egui::TextEdit::singleline(&mut self.creation_name).desired_width(200.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Nom de la sauvegarde :");
                            ui.add(egui::TextEdit::singleline(&mut self.creation_save_name).desired_width(200.0));
                        });
                        ui.label("(Si vide, le nom du personnage sera utilisé)");
                        ui.add_space(16.0);
                        if ui.button("Valider").clicked() {
                            let name = self.creation_name.trim();
                            if !name.is_empty() {
                                self.screen = Screen::CharacterCreation(0);
                                self.creation_available_ids = (0..=24).collect();
                                self.creation_stats = CharacterStats::default();
                                self.creation_reroll_pending = false;
                                self.creation_current_choices = pick_three_phrases(
                                    &mut self.creation_available_ids,
                                    &mut crate::game_state::rand_simple,
                                );
                            }
                        }
                    });
                }
                Screen::CharacterCreation(step) => {
                    let mut chosen: Option<usize> = None;
                    ui.vertical_centered(|ui| {
                        ui.heading(format!("Création du personnage — Étape {}/4", step + 1));
                        ui.add_space(8.0);
                        ui.label("Choisis une phrase qui te décrit :");
                        ui.add_space(12.0);
                        for (i, phrase) in self.creation_current_choices.iter().enumerate() {
                            if ui.button(phrase.text).clicked() {
                                chosen = Some(i);
                            }
                        }
                        ui.add_space(16.0);
                        ui.collapsing("Caractéristiques actuelles", |ui| {
                            let s = &self.creation_stats;
                            ui.label(format!("For: {}  Con: {}  Agi: {}  Dex: {}", s.display(Stat::For), s.display(Stat::Con), s.display(Stat::Agi), s.display(Stat::Dex)));
                            ui.label(format!("Int: {}  Sag: {}  Cha: {}  Luk: {}", s.display(Stat::Int), s.display(Stat::Sag), s.display(Stat::Cha), s.display(Stat::Luk)));
                        });
                    });
                    if let Some(i) = chosen {
                        let phrase = self.creation_current_choices[i].clone();
                        let reroll = apply_phrase_effects(
                            &mut self.creation_stats,
                            &phrase.effects,
                            &mut crate::game_state::rand_simple,
                        );
                        if reroll {
                            self.creation_reroll_pending = true;
                        }
                        if step < 3 {
                            self.screen = Screen::CharacterCreation(step + 1);
                            self.creation_current_choices = pick_three_phrases(
                                &mut self.creation_available_ids,
                                &mut crate::game_state::rand_simple,
                            );
                        } else {
                            self.finish_creation_and_start();
                        }
                    }
                }
                Screen::Preparation => {
                    if let Some(ref mut state) = self.game {
                        let last_tick = self.last_tick;
                        let keys = self.keys;
                        let result = RefCell::new((last_tick, false));
                        let transition = RefCell::new(None::<Screen>);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                let available = ui.available_rect_before_wrap();
                                let (game_rect, resp) = allocate_combat_surface_centered(ui, available);
                                if resp.clicked() {
                                    ctx.memory_mut(|m| m.request_focus(resp.id));
                                }
                                let painter = ui.painter();
                                let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                                let (tick, go) = game_zone_tick_and_paint(
                                    game_rect,
                                    &painter,
                                    state,
                                    last_tick,
                                    keys,
                                    ui.visuals().window_fill(),
                                    cursor_screen,
                                );
                                *result.borrow_mut() = (tick, go);
                            });
                            ui.vertical(|ui| {
                                ui.set_min_width(180.0);
                                ui.heading("Préparation");
                                ui.label("Construisez des tours (50 or).");
                                ui.add_space(8.0);
                                if ui.button("Lancer la vague").clicked() {
                                    state.start_battle_phase();
                                    transition.replace(Some(Screen::Battle));
                                }
                                ui.add_space(16.0);
                                ui.label(format!("Or: {}", state.gold));
                            });
                        });
                        let (new_tick, game_over) = *result.borrow();
                        self.last_tick = new_tick;
                        if game_over {
                            self.screen = Screen::GameOver;
                        }
                        let screen_transition = transition.borrow_mut().take();
                        if let Some(s) = screen_transition {
                            self.screen = s;
                        }
                        // Log dégâts joueur (standalone Préparation)
                        ui.add_space(8.0);
                        egui::CollapsingHeader::new("Dégâts infligés par le joueur")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.set_max_height(100.0);
                                egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                                    for line in state.player_damage_log.iter().rev() {
                                        ui.label(line);
                                    }
                                    if state.player_damage_log.is_empty() {
                                        ui.weak("Aucun dégât infligé pour l'instant.");
                                    }
                                });
                            });
                    }
                }
                Screen::Battle => {
                    if let Some(ref mut state) = self.game {
                        let available = ui.available_rect_before_wrap();
                        let (game_rect, resp) = allocate_combat_surface_centered(ui, available);
                        if resp.clicked() {
                            ctx.memory_mut(|m| m.request_focus(resp.id));
                        }
                        let painter = ui.painter();
                        let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                        let (new_tick, game_over) = game_zone_tick_and_paint(
                            game_rect,
                            &painter,
                            state,
                            self.last_tick,
                            self.keys,
                            ui.visuals().window_fill(),
                            cursor_screen,
                        );
                        self.last_tick = new_tick;
                        if game_over {
                            self.screen = Screen::GameOver;
                        }
                        // Overlay victoire de vague : félicitations + résumé + "Phase suivante"
                        if state.is_wave_won() {
                            let rect = game_rect;
                            egui::Area::new(egui::Id::new("wave_won_overlay"))
                                .order(egui::Order::Foreground)
                                .fixed_pos(rect.min)
                                .constrain(true)
                                .show(ctx, |ui| {
                                    ui.set_min_size(rect.size());
                                    let frame = egui::Frame::new()
                                        .fill(egui::Color32::from_white_alpha(240))
                                        .corner_radius(8.0);
                                    frame.show(ui, |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(60.0);
                                            ui.heading("Vague terminée !");
                                            ui.label("Félicitations.");
                                            ui.add_space(16.0);
                                            ui.label("Résumé de la vague :");
                                            ui.label(format!("Ennemis tués : {}", state.enemies_killed_this_wave));
                                            ui.label(format!("Or collecté : {} or", state.gold_collected_this_wave));
                                            ui.label(format!("XP gagnée : {}", state.xp_gained_this_wave));
                                            ui.label(format!("Objets trouvés : {}", state.items_found_this_wave));
                                            ui.add_space(24.0);
                                            if ui.button("Phase suivante").clicked() {
                                                state.start_preparation_phase();
                                                self.screen = Screen::Preparation;
                                            }
                                        });
                                    });
                                });
                        }
                        // Log dégâts joueur (standalone)
                        ui.add_space(8.0);
                        egui::CollapsingHeader::new("Dégâts infligés par le joueur")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.set_max_height(100.0);
                                egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                                    for line in state.player_damage_log.iter().rev() {
                                        ui.label(line);
                                    }
                                    if state.player_damage_log.is_empty() {
                                        ui.weak("Aucun dégât infligé pour l'instant.");
                                    }
                                });
                            });
                    }
                }
                Screen::GameOver => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Game Over");
                        ui.label("Le Château a été détruit.");
                        ui.add_space(20.0);
                        if ui.button("Retour au titre").clicked() {
                            self.screen = Screen::Title;
                            self.game = None;
                        }
                    });
                }
            }
        });

        ctx.request_repaint();
    }
}
