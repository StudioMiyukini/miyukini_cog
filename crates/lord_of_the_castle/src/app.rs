//! Application Lord of the Castle — écrans, zone de jeu, barre haute, input.
//! Aligné sur docs/services/MiyukiniSurvivor (Ecrans et UI, Gameplay).
//! Surface de combat 800×800 centrée au milieu du body ; déplacement ZQSD.
//!
//! @id: lord_of_the_castle_app
//! @do: manage_screens_windows_and_game_flow
//! @role: ui
//! @layer: domain
//! @human: Orchestration des écrans (titre, lore, création, préparation, bataille, game over), fenêtres, header.

use crate::app_ui::{
    allocate_game_zone_and_bottom, game_zone_tick_and_paint, paint_bottom_panel,
    paint_equipment_window, paint_inventory_window, paint_player_window,
    paint_skills_window,
    screen_to_cell, screen_to_world,
};
use crate::character_creation::{
    apply_phrase_effects, pick_three_phrases, CharacterStats, PhraseDef, Stat,
};
use crate::constants::TOWER_BASE_COST_GOLD;
use crate::game_state::{
    GamePhase, GameState, INVENTORY_MAX_SLOTS, EXPERT_IDENTIFY_COST_GOLD,
};
use crate::loot::{InventoryEntry, ItemSlot};
use crate::player::{Dir8, Player};
use crate::troops::TroopKind;
use crate::warrior_skills::WarriorSkillId;
use crate::spritesheet::{load_image_from_path, SpritesheetDesc};
use std::path::Path;
use eframe::egui;
use eframe::App;
use std::cell::RefCell;
use std::time::Instant;

/// Section active dans la sidebar « Mode préparation » (legacy ; remplacée par fenêtres).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreparationSection {
    #[default]
    Marchand,
    ExpertIdentification,
    Construction,
    Recrutement,
}

/// Pool du marchand (pour achat différé).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MerchantPoolKind {
    Weapon,
    Armor,
    Accessory,
}

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
    /// Touches enfoncées (8 directions) — joueur principal (ZQSD ou OKLM si joueur 2 présent).
    pub keys: [bool; 8],
    /// Touches joueur secondaire (ZQSD) — utilisé uniquement si secondary_player présent.
    pub keys_secondary: [bool; 8],
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
    #[allow(dead_code)]
    pub preparation_sidebar_open: bool,
    #[allow(dead_code)]
    pub preparation_section: PreparationSection,
    /// Fenêtres Mode préparation (ouvertes par boutons header).
    pub competences_open: bool,
    /// Onglet actif dans la fenêtre Compétences.
    pub skills_tab: crate::SkillsTab,
    /// Compétence Guerrier sélectionnée (pour la fenêtre de détail).
    pub selected_warrior_skill: Option<WarriorSkillId>,
    pub marchand_open: bool,
    pub expert_open: bool,
    pub construction_open: bool,
    /// Case sélectionnée en mode construction (grille 20×20, origine = coin château). None = pas de sélection.
    pub construction_selected_cell: Option<(i32, i32)>,
    pub recrutement_open: bool,
    /// Achat marchand différé (pool, index) pour éviter emprunt pendant la fenêtre.
    pub pending_merchant_buy: RefCell<Option<(MerchantPoolKind, usize)>>,
    /// En dev : demande de reroll des pools du marchand (bouton).
    pub pending_merchant_reroll: RefCell<bool>,
    /// En dev : ajouter 100 or au joueur (bouton marchand).
    pub pending_dev_add_gold: RefCell<bool>,
    /// Fenêtre « Mode Dev » ouverte (bouton Mode Dev ouvre cette fenêtre).
    pub dev_mode_window_open: bool,
    /// Fenêtre « Joueur 2 » ouverte (bouton Joueur 2, phase Préparation).
    pub joueur2_window_open: bool,
    /// Identification par soi-même différée (index dans l'inventaire).
    pub pending_identify_self: RefCell<Option<usize>>,
    /// Overlay « trop encombré » quand le joueur clique Lancer la vague avec inventaire plein.
    pub show_encumbered_overlay: bool,
    /// Texture du spritesheet joueur (Knight-Idle), chargée à la première partie.
    pub player_sprite_texture: Option<egui::TextureHandle>,
    /// Description du spritesheet (6 frames horizontal).
    pub player_sprite_desc: Option<SpritesheetDesc>,
    pub player_walk_texture: Option<egui::TextureHandle>,
    pub player_walk_desc: Option<SpritesheetDesc>,
    /// Accumulateur temps pour l’animation idle (frame index dérivé).
    pub player_anim_accumulator: f32,
    /// Texture du spritesheet ennemis de base (Skeleton-Walk), chargée à la première partie.
    pub enemy_sprite_texture: Option<egui::TextureHandle>,
    /// Description du spritesheet (8 frames horizontal).
    pub enemy_sprite_desc: Option<SpritesheetDesc>,
    /// Texture du spritesheet mini-boss (Werebear-Walk), chargée à la première partie.
    pub enemy_miniboss_sprite_texture: Option<egui::TextureHandle>,
    /// Description du spritesheet mini-boss (8 frames horizontal).
    pub enemy_miniboss_sprite_desc: Option<SpritesheetDesc>,
    /// Accumulateur temps pour l'animation marche ennemis.
    pub enemy_anim_accumulator: f32,
    /// Texture du spritesheet troupes (Soldier-Walk / Miliciens), chargée à la première partie.
    pub troop_sprite_texture: Option<egui::TextureHandle>,
    /// Description du spritesheet troupes (8 frames horizontal).
    pub troop_sprite_desc: Option<SpritesheetDesc>,
    /// Texture du spritesheet attaque troupes (Soldier-Attack01 / Miliciens), 6 frames.
    pub troop_attack_sprite_texture: Option<egui::TextureHandle>,
    /// Description du spritesheet attaque troupes (6 frames horizontal).
    pub troop_attack_sprite_desc: Option<SpritesheetDesc>,
    /// Accumulateur temps pour l'animation marche troupes.
    pub troop_anim_accumulator: f32,
    /// Callback pour sauvegarder les octets (Central = DB, standalone = fichier). None = non disponible.
    pub save_callback: Option<Box<dyn Fn(&[u8]) -> Result<(), String>>>,
    /// Callback pour charger les octets. None = non disponible.
    pub load_callback: Option<Box<dyn Fn() -> Result<Vec<u8>, String>>>,
    /// Message temporaire après sauvegarde/chargement (texte, instant d'affichage). Effacé après 2 s.
    pub save_load_message: Option<(String, Instant)>,
    /// Demande de sauvegarde (bouton cliqué) ; traitée après le bloc UI pour éviter double emprunt.
    pub save_requested: bool,
    /// Demande de chargement (bouton cliqué).
    pub load_requested: bool,
    /// Notifications « niveau atteint » affichées en bas à droite (message, instant). Expirent après 3 s.
    pub level_up_toasts: Vec<(String, Instant)>,
    /// Fenêtre « Tutoriel » ouverte (bouton Tuto dans le header).
    pub tutorial_window_open: bool,
    /// Onglet actif dans la fenêtre Tutoriel (0 = Comment jouer, 1 = Contrôles, 2 = Phases, 3 = Mécaniques).
    pub tutorial_tab: usize,
}

impl Default for LordOfTheCastleApp {
    fn default() -> Self {
        Self {
            screen: Screen::Title,
            game: None,
            last_tick: Instant::now(),
            keys: [false; 8],
            keys_secondary: [false; 8],
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
            preparation_sidebar_open: true,
            preparation_section: PreparationSection::Marchand,
            competences_open: false,
            skills_tab: crate::SkillsTab::Guerrier,
            selected_warrior_skill: None,
            marchand_open: false,
            expert_open: false,
            construction_open: false,
            construction_selected_cell: None,
            recrutement_open: false,
            pending_merchant_buy: RefCell::new(None),
            pending_merchant_reroll: RefCell::new(false),
            pending_dev_add_gold: RefCell::new(false),
            dev_mode_window_open: false,
            joueur2_window_open: false,
            pending_identify_self: RefCell::new(None),
            show_encumbered_overlay: false,
            player_sprite_texture: None,
            player_sprite_desc: None,
            player_walk_texture: None,
            player_walk_desc: None,
            player_anim_accumulator: 0.0,
            enemy_sprite_texture: None,
            enemy_sprite_desc: None,
            enemy_miniboss_sprite_texture: None,
            enemy_miniboss_sprite_desc: None,
            enemy_anim_accumulator: 0.0,
            troop_sprite_texture: None,
            troop_sprite_desc: None,
            troop_attack_sprite_texture: None,
            troop_attack_sprite_desc: None,
            troop_anim_accumulator: 0.0,
            save_callback: None,
            load_callback: None,
            save_load_message: None,
            save_requested: false,
            load_requested: false,
            level_up_toasts: Vec::new(),
            tutorial_window_open: false,
            tutorial_tab: 0,
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
        self.tutorial_window_open = true;
        self.creation_available_ids.clear();
        self.creation_reroll_pending = false;
        self.creation_current_choices.clear();
    }

    /// Crée une instance pour intégration dans Miyukini Central (point d'accès utilisateur unique).
    /// Le jeu s'exécute dans le body de Central ; pas de fenêtre standalone.
    pub fn new_embedded() -> Self {
        Self::default()
    }

    /// Enregistre les callbacks de sauvegarde/chargement (Central = DB, standalone = fichier).
    pub fn set_save_load_callbacks(
        &mut self,
        save: impl Fn(&[u8]) -> Result<(), String> + 'static,
        load: impl Fn() -> Result<Vec<u8>, String> + 'static,
    ) {
        self.save_callback = Some(Box::new(save));
        self.load_callback = Some(Box::new(load));
    }

    /// Sauvegarde la partie courante (sérialisation bincode + callback). Retourne le message d’erreur éventuel.
    /// Draine les notifications « niveau atteint » du state vers les toasts (à appeler après le tick).
    fn drain_level_up_toasts(&mut self) {
        if let Some(ref mut state) = self.game {
            for msg in state.pending_level_up_notifications.drain(..) {
                self.level_up_toasts.push((msg, Instant::now()));
            }
        }
    }

    /// Affiche les toasts « niveau atteint » en bas à droite. Expire après 3 s.
    fn paint_level_up_toasts(&mut self, ctx: &egui::Context) {
        const TOAST_TTL_SECS: f32 = 3.0;
        self.level_up_toasts.retain(|(_, t)| Instant::now().duration_since(*t).as_secs_f32() <= TOAST_TTL_SECS);
        if self.level_up_toasts.is_empty() {
            return;
        }
        let messages: Vec<String> = self.level_up_toasts.iter().map(|(m, _)| m.clone()).collect();
        let rect = ctx.available_rect();
        egui::Area::new(egui::Id::new("lotc_level_toasts"))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::ZERO)
            .order(egui::Order::Foreground)
            .constrain(true)
            .show(ctx, |ui| {
                ui.set_max_width(rect.width() * 0.35);
                for msg in &messages {
                    let frame = egui::Frame::new()
                        .fill(ui.visuals().window_fill())
                        .corner_radius(6.0)
                        .inner_margin(8);
                    frame.show(ui, |ui| {
                        ui.colored_label(egui::Color32::from_rgb(100, 200, 100), msg);
                    });
                }
            });
    }

    /// Sauvegarde la partie courante (sérialisation bincode + callback). Retourne le message d'erreur éventuel.
    fn do_save(&self) -> Result<(), String> {
        let state = self.game.as_ref().ok_or("Aucune partie en cours.")?;
        let bytes = bincode::serialize(state).map_err(|e| e.to_string())?;
        let cb = self
            .save_callback
            .as_ref()
            .ok_or("Sauvegarde non disponible (connexion requise dans Central).")?;
        cb(&bytes)
    }

    /// Charge une partie (callback + désérialisation bincode). Remplace l’état courant.
    fn do_load(&mut self) -> Result<(), String> {
        let bytes = self
            .load_callback
            .as_ref()
            .ok_or("Chargement non disponible (connexion requise dans Central).")?()?;
        let state: GameState = bincode::deserialize(&bytes).map_err(|e| e.to_string())?;
        self.game = Some(state);
        self.screen = Screen::Preparation;
        self.last_tick = Instant::now();
        Ok(())
    }

    /// Rendu dans un `ui` fourni (intégration Miyukini Central).
    /// Point d'accès utilisateur unique : Central ; le service s'exécute dans le body de Central.
    pub fn show_into(&mut self, ui: &mut egui::Ui) {
        // Chargement unique des spritesheets joueur (Knight-Idle, Knight-Walk) quand une partie est en cours
        if self.game.is_some() {
            if self.player_sprite_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Idle.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("knight_idle", color_image, egui::TextureOptions::default());
                    self.player_sprite_texture = Some(tex);
                    self.player_sprite_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 6));
                }
            }
            if self.player_walk_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Walk.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("knight_walk", color_image, egui::TextureOptions::default());
                    self.player_walk_texture = Some(tex);
                    self.player_walk_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 8));
                }
            }
            if self.enemy_sprite_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Skeleton/Skeleton/Skeleton-Walk.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("skeleton_walk", color_image, egui::TextureOptions::default());
                    self.enemy_sprite_texture = Some(tex);
                    self.enemy_sprite_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 8));
                }
            }
            if self.enemy_miniboss_sprite_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Werebear/Werebear/Werebear-Walk.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("werebear_walk", color_image, egui::TextureOptions::default());
                    self.enemy_miniboss_sprite_texture = Some(tex);
                    self.enemy_miniboss_sprite_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 8));
                }
            }
            if self.troop_sprite_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Soldier/Soldier/Soldier-Walk.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("soldier_walk", color_image, egui::TextureOptions::default());
                    self.troop_sprite_texture = Some(tex);
                    self.troop_sprite_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 8));
                }
            }
            if self.troop_attack_sprite_texture.is_none() {
                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Soldier/Soldier/Soldier-Attack01.png");
                if let Some(color_image) = load_image_from_path(&path) {
                    let [w, h] = color_image.size;
                    let tex = ui.ctx().load_texture("soldier_attack01", color_image, egui::TextureOptions::default());
                    self.troop_attack_sprite_texture = Some(tex);
                    self.troop_attack_sprite_desc = Some(SpritesheetDesc::horizontal(w as u32, h as u32, 6));
                }
            }
            self.drain_level_up_toasts();
        }

        // Id à focus si l'utilisateur clique sur la zone de jeu (pour recevoir ZQSD)
        let focus_request = RefCell::new(None::<egui::Id>);
        // Clic « Lancer la vague » depuis la barre haute (toujours visible)
        let start_battle_request = RefCell::new(false);

        // Input clavier : si joueur 2 présent, J1 = OKLM (O=haut, K=gauche, L=bas, M=droite), J2 = ZQSD ; sinon J1 = ZQSD + flèches
        let input = ui.ctx().input(Clone::clone);
        use egui::Key;
        let two_players = self.game.as_ref().map(|s| s.secondary_player.is_some()).unwrap_or(false);
        let (up1, down1, left1, right1) = if two_players {
            (input.key_down(Key::O), input.key_down(Key::L), input.key_down(Key::K), input.key_down(Key::M))
        } else {
            (input.key_down(Key::Z) || input.key_down(Key::W) || input.key_down(Key::ArrowUp),
             input.key_down(Key::S) || input.key_down(Key::ArrowDown),
             input.key_down(Key::Q) || input.key_down(Key::A) || input.key_down(Key::ArrowLeft),
             input.key_down(Key::D) || input.key_down(Key::ArrowRight))
        };
        self.keys[0] = up1 && !left1 && !right1;
        self.keys[1] = up1 && right1;
        self.keys[2] = right1 && !up1 && !down1;
        self.keys[3] = down1 && right1;
        self.keys[4] = down1 && !left1 && !right1;
        self.keys[5] = down1 && left1;
        self.keys[6] = left1 && !up1 && !down1;
        self.keys[7] = up1 && left1;
        let (up2, down2, left2, right2) = (input.key_down(Key::Z), input.key_down(Key::S), input.key_down(Key::Q), input.key_down(Key::D));
        self.keys_secondary[0] = up2 && !left2 && !right2;
        self.keys_secondary[1] = up2 && right2;
        self.keys_secondary[2] = right2 && !up2 && !down2;
        self.keys_secondary[3] = down2 && right2;
        self.keys_secondary[4] = down2 && !left2 && !right2;
        self.keys_secondary[5] = down2 && left2;
        self.keys_secondary[6] = left2 && !up2 && !down2;
        self.keys_secondary[7] = up2 && left2;

        if self.screen == Screen::Battle {
            ui.ctx().request_repaint();
        }

        // Barre haute : Player, Inventaire, Équipement, Marchand, Deckard Rain, Construction, Recrutement, vague, Lancer la vague, Mode Dev, Or
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
                let prep_enabled = state.phase == GamePhase::Preparation;
                if ui.add_enabled(prep_enabled, egui::Button::new("Compétences")).clicked() && prep_enabled {
                    self.competences_open ^= true;
                }
                if ui.add_enabled(prep_enabled, egui::Button::new("Marchand")).clicked() && prep_enabled {
                    self.marchand_open ^= true;
                }
                if ui.add_enabled(prep_enabled, egui::Button::new("Deckard Rain (expert)")).clicked() && prep_enabled {
                    self.expert_open ^= true;
                }
                if ui.add_enabled(prep_enabled, egui::Button::new("Construction")).clicked() && prep_enabled {
                    self.construction_open ^= true;
                }
                if ui.add_enabled(prep_enabled, egui::Button::new("Recrutement")).clicked() && prep_enabled {
                    self.recrutement_open ^= true;
                }
                if ui.add_enabled(prep_enabled, egui::Button::new("Joueur 2")).clicked() && prep_enabled {
                    self.joueur2_window_open = true;
                }
                ui.label(format!("Vague {}", state.wave_number));
                ui.label(format!("Ennemis: {}", state.enemies.len()));
                let troops_count = state.troops.iter().filter(|t| t.is_active_in_squad()).count();
                ui.label(format!("Troupes: {} / {}", troops_count, state.max_troops()));
                if self.screen == Screen::Preparation
                    && ui.button("Lancer la vague").clicked()
                {
                    start_battle_request.replace(true);
                }
                if ui.button("Mode Dev").clicked() {
                    self.dev_mode_window_open = true;
                }
                if ui.button("Sauvegarder").clicked() {
                    self.save_requested = true;
                }
                if ui.button("Charger").clicked() {
                    self.load_requested = true;
                }
                if ui.button("Tuto").clicked() {
                    self.tutorial_window_open = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Or: {}", state.gold));
                });
            }
        });
        // Bandeau « PHASE DE REPOS » sous le header (visible uniquement en phase Préparation)
        if let Some(ref state) = self.game {
            if state.phase == GamePhase::Preparation {
                ui.add_space(4.0);
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(255, 220, 0))
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("PHASE DE REPOS").color(egui::Color32::BLACK).strong());
                        });
                    });
                ui.add_space(4.0);
            }
        }
        if self.save_requested {
            self.save_requested = false;
            match self.do_save() {
                Ok(()) => self.save_load_message = Some(("Sauvegarde OK.".into(), Instant::now())),
                Err(e) => self.save_load_message = Some((format!("Erreur: {}", e), Instant::now())),
            }
        }
        if self.load_requested {
            self.load_requested = false;
            match self.do_load() {
                Ok(()) => self.save_load_message = Some(("Partie chargée.".into(), Instant::now())),
                Err(e) => self.save_load_message = Some((format!("Erreur: {}", e), Instant::now())),
            }
        }
        if let Some((msg, t)) = &self.save_load_message {
            if t.elapsed().as_secs_f32() > 2.0 {
                self.save_load_message = None;
            } else {
                ui.colored_label(egui::Color32::from_rgb(120, 255, 120), msg.as_str());
            }
        }
        ui.add_space(8.0);

        // Fenêtre Joueur 2 (phase Préparation) : spawn Tombilol, Tal Ratchou, Sergent Garcia
        if self.joueur2_window_open {
            let mut open = true;
            let mut should_close = false;
            egui::Window::new("Joueur 2")
                .open(&mut open)
                .default_width(320.0)
                .show(ui.ctx(), |ui| {
                    ui.label("Spawn un personnage secondaire (2 joueurs) :");
                    ui.add_space(6.0);
                    if let Some(ref mut state) = self.game {
                        if state.secondary_player.is_some() {
                            ui.label("Un joueur secondaire est déjà présent.");
                            if ui.button("lvlup (joueur 2)").clicked() {
                                state.secondary_give_level_up();
                            }
                        } else {
                            if ui.button("Tombilol — 30 PV, attaque 360° 30 px, 1 att/s").clicked() {
                                state.spawn_secondary_tombilol();
                                should_close = true;
                            }
                            if ui.button("Tal Ratchou — 20 PV, projectile homing 0,6/s, 200 px").clicked() {
                                state.spawn_secondary_tal_ratchou();
                                should_close = true;
                            }
                            if ui.button("Sergent Garcia — 10 PV, 6 miliciens").clicked() {
                                state.spawn_secondary_sergent_garcia();
                                should_close = true;
                            }
                        }
                    }
                });
            if !open || should_close {
                self.joueur2_window_open = false;
            }
        }

        // Fenêtre Tutoriel (déplaçable, fermable, onglets)
        if self.tutorial_window_open {
            let mut tab = self.tutorial_tab;
            egui::Window::new("Tutoriel")
                .open(&mut self.tutorial_window_open)
                .default_size([420.0, 380.0])
                .resizable(true)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(tab == 0, "Comment jouer").clicked() { tab = 0; }
                        if ui.selectable_label(tab == 1, "Contrôles").clicked() { tab = 1; }
                        if ui.selectable_label(tab == 2, "Phases").clicked() { tab = 2; }
                        if ui.selectable_label(tab == 3, "Mécaniques").clicked() { tab = 3; }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match tab {
                            0 => {
                                ui.heading("Comment jouer");
                                ui.add_space(6.0);
                                ui.label("Lord of the Castle est un jeu de défense de tour et d’action. Votre objectif : protéger le Château des vagues d’ennemis.");
                                ui.add_space(4.0);
                                ui.label("• En phase Préparation : achetez équipement, construisez des tours, recrutez des troupes, montez en compétences.");
                                ui.label("• Cliquez sur « Lancer la vague » pour passer en Bataille.");
                                ui.label("• En Bataille : déplacez-vous (ZQSD), combattez les ennemis avec votre personnage et vos troupes ; les tours tirent automatiquement.");
                                ui.label("• Ramassez l’or et l’XP au sol, identifiez les objets trouvés, puis repassez en Préparation pour la vague suivante.");
                                ui.add_space(4.0);
                                ui.label("Si le Château tombe à 0 PV, c’est le Game Over.");
                            }
                            1 => {
                                ui.heading("Contrôles");
                                ui.add_space(6.0);
                                ui.label("• Déplacement : ZQSD ou flèches directionnelles (8 directions).");
                                ui.label("• Avec un joueur 2 : le joueur 1 utilise OKLM, le joueur 2 utilise ZQSD.");
                                ui.label("• Cliquez sur la zone de jeu pour lui donner le focus (clavier actif).");
                                ui.label("• Header : Player, Inventaire, Équipement, Compétences, Marchand, Expert (Deckard Rain), Construction, Recrutement, Lancer la vague, Sauvegarder, Charger.");
                                ui.label("• Ramassage : approchez-vous des orbes (or, XP) et des objets au sol (rayon ~30 px).");
                            }
                            2 => {
                                ui.heading("Phases");
                                ui.add_space(6.0);
                                ui.label("Préparation :");
                                ui.label("  • Compétences (arbre Guerrier), équipement au Marchand, construction de tours (grille 20×20), recrutement de troupes.");
                                ui.label("  • Inventaire max 20 slots ; ne lancez pas la vague si l’inventaire est plein.");
                                ui.add_space(4.0);
                                ui.label("Bataille :");
                                ui.label("  • Ennemis spawnent et marchent vers le Château. Vous, les tours et les troupes les attaquez.");
                                ui.label("  • Quand tous les ennemis sont morts, la vague est gagnée : or, XP et objets récupérables au sol.");
                                ui.label("  • Passez à la vague suivante en revenant en Préparation.");
                            }
                            3 => {
                                ui.heading("Mécaniques");
                                ui.add_space(6.0);
                                ui.label("Joueur : attaque auto à portée (~40 px), 1 att/s ; dégâts et PV selon équipement et stats. Mort à 0 PV : revive après la vague avec −1 PV max.");
                                ui.label("Tours : coût 50 or, champ de vision 300 px, flèches 600 px, 1 tir / 2 s.");
                                ui.label("Troupes : suivent le joueur dans un rayon ~200 px, combattent les ennemis à portée (ex. Milicien 3 dégâts, 25 px).");
                                ui.label("Ennemis : priorité Joueur > Tour > Château ; donnent or, XP et parfois objets à la mort.");
                                ui.label("Loot : or et XP en orbes ; objets à identifier (Marchand ou Expert 20 or).");
                            }
                            _ => {}
                        }
                    });
                });
            self.tutorial_tab = tab;
        }

        // Fenêtre Mode Dev (Visions des portées, +100 or, Spawn 50 ennemis, lvlup)
        if self.dev_mode_window_open {
            let mut open = true;
            egui::Window::new("Mode Dev")
                .open(&mut open)
                .default_width(280.0)
                .show(ui.ctx(), |ui| {
                    if let Some(ref mut state) = self.game {
                        ui.checkbox(&mut state.dev_mode, "Visions des portées");
                        ui.add_space(6.0);
                        if ui.button("+ 100 or").clicked() {
                            state.gold += 100;
                        }
                        if ui.button("Spawn 50 ennemis normal").clicked() {
                            state.dev_spawn_50_normal_enemies_random();
                        }
                        if ui.button("lvlup").clicked() {
                            state.dev_give_level_up();
                        }
                    }
                });
            if !open {
                self.dev_mode_window_open = false;
            }
        }

        // Fenêtre Player (métriques joueur)
        if self.player_window_open {
            let mut open = true;
            egui::Window::new("Player")
                .open(&mut open)
                .default_width(280.0)
                .show(ui.ctx(), |ui| paint_player_window(ui, self.game.as_mut()));
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
                    paint_inventory_window(ui, self.game.as_mut(), &mut self.selected_inventory_slot, &self.pending_identify_self);
                });
            if !open {
                self.inventory_window_open = false;
                self.selected_inventory_slot = None;
            }
        }

        // Fenêtre Détail / Identification (vue intégrée)
        if let Some(slot_idx) = self.selected_inventory_slot {
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
        if let Some(slot) = self.selected_equipment_slot {
            egui::Window::new("Détail équipement")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    paint_equipment_item_detail(ui, self, slot);
                });
        }

        // Fenêtre Compétences (5 onglets : Guerrier, Sorcier, Saint, Régent, Commandant)
        if self.competences_open {
            let mut open = true;
            egui::Window::new("Compétences")
                .open(&mut open)
                .default_width(420.0)
                .default_height(380.0)
                .show(ui.ctx(), |ui| paint_skills_window(ui, self.game.as_mut(), &mut self.skills_tab, &mut self.selected_warrior_skill));
            if !open {
                self.competences_open = false;
            }
        }

        // Fenêtres Mode préparation (Marchand, Expert, Construction, Recrutement) — vue intégrée Central
        let mut close_marchand = false;
        let mut close_expert = false;
        let mut close_construction = false;
        let mut close_recrutement = false;
        if let Some(ref mut state) = self.game {
            let gold = state.gold;
            if self.marchand_open {
                let mut open = true;
                egui::Window::new("Marchand")
                    .open(&mut open)
                    .default_width(380.0)
                    .default_height(480.0)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Or : {} or", gold));
                        if state.dev_mode {
                            if ui.button("+100 or (dev)").clicked() {
                                self.pending_dev_add_gold.replace(true);
                            }
                        }
                        if cfg!(debug_assertions) {
                            if ui.button("🔄 Reroll pools (dev)").clicked() {
                                self.pending_merchant_reroll.replace(true);
                            }
                        }
                        ui.add_space(6.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.heading("Armes");
                            for (i, opt) in state.merchant_weapons.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Weapon, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                            ui.add_space(8.0);
                            ui.heading("Armures");
                            for (i, opt) in state.merchant_armor.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Armor, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                            ui.add_space(8.0);
                            ui.heading("Accessoires");
                            for (i, opt) in state.merchant_accessories.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Accessory, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                        });
                    });
                if !open {
                    close_marchand = true;
                }
            }
            if self.expert_open {
                let mut open = true;
                egui::Window::new("Expert en identification")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.add_space(8.0);
                        ui.heading("Expert en identification");
                        ui.label("Identification groupée : identifier tous les objets de l'inventaire en une fois (prix cumulé).");
                        ui.weak("(À implémenter : bouton et coût total)");
                    });
                if !open {
                    close_expert = true;
                }
            }
            if self.construction_open {
                let mut open = true;
                egui::Window::new("Construction")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.add_space(8.0);
                        ui.heading("Bâtiments constructibles");
                        ui.label("Sélectionnez une case sur la zone de jeu (grille 20×20). Clic droit pour annuler.");
                        ui.add_space(8.0);
                        if let Some((ci, cj)) = self.construction_selected_cell {
                            ui.label(format!("Case sélectionnée : ({}, {})", ci, cj));
                            let has_place = state.can_build_at_cell(ci, cj);
                            let has_gold = state.gold >= TOWER_BASE_COST_GOLD;
                            if has_place {
                                if has_gold {
                                    if ui.button(format!("Tour ({} po) — Construire dedans", TOWER_BASE_COST_GOLD)).clicked() {
                                        state.build_tower_at_cell(ci, cj);
                                    }
                                } else {
                                    ui.weak(format!("Or insuffisant ({} po requis).", TOWER_BASE_COST_GOLD));
                                }
                            } else {
                                ui.weak("Pas la place ici (château ou autre bâtiment).");
                            }
                        } else {
                            ui.weak("Aucune case sélectionnée.");
                        }
                    });
                if !open {
                    close_construction = true;
                }
            }
            if self.recrutement_open {
                let mut open = true;
                let troops_count = state.troops.iter().filter(|t| t.is_active_in_squad()).count();
                let max_troops = state.max_troops();
                egui::Window::new("Recrutement")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.label(format!("Troupes : {} / {}", troops_count, max_troops));
                        ui.add_space(8.0);
                        ui.heading("Troupes disponibles");
                        ui.horizontal(|ui| {
                            ui.label(TroopKind::Milicien.label());
                            ui.label("— 100 PV, 20 % blocage, 6 dégâts, 1 att/s, portée 25 px.");
                            if ui.add_enabled(troops_count < max_troops, egui::Button::new("Recruter")).clicked()
                                && troops_count < max_troops
                            {
                                state.recruit_troop(TroopKind::Milicien);
                            }
                        });
                    });
                if !open {
                    close_recrutement = true;
                }
            }
        }
        if close_marchand {
            self.marchand_open = false;
        }
        if close_expert {
            self.expert_open = false;
        }
        if close_construction {
            self.construction_open = false;
            self.construction_selected_cell = None;
        }
        if close_recrutement {
            self.recrutement_open = false;
        }
        if let Some((pool, index)) = self.pending_merchant_buy.borrow_mut().take() {
            if let Some(ref mut state) = self.game {
                match pool {
                    MerchantPoolKind::Weapon => {
                        state.buy_merchant_weapon(index);
                    }
                    MerchantPoolKind::Armor => {
                        state.buy_merchant_armor(index);
                    }
                    MerchantPoolKind::Accessory => {
                        state.buy_merchant_accessory(index);
                    }
                }
            }
        }
        if self.pending_merchant_reroll.replace(false) {
            if let Some(ref mut state) = self.game {
                state.refresh_merchant_pools();
            }
        }
        if self.pending_dev_add_gold.replace(false) {
            if let Some(ref mut state) = self.game {
                state.gold += 100;
            }
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
                let mut state_opt = self.game.take();
                if let Some(ref mut state) = state_opt {
                    let last_tick = self.last_tick;
                    let keys = self.keys;
                    let delta = last_tick.elapsed().as_secs_f32();
                    self.player_anim_accumulator += delta;
                    self.enemy_anim_accumulator += delta;
                    self.troop_anim_accumulator += delta;
                    let is_moving = keys.iter().any(|&k| k);
                    let frame_idle = ((self.player_anim_accumulator * 6.0) as usize) % 6;
                    let frame_walk = ((self.player_anim_accumulator * 8.0) as usize) % 8;
                    let enemy_frame = ((self.enemy_anim_accumulator * 8.0) as usize) % 8;
                    let result = RefCell::new((last_tick, false));
                    let (game_rect, resp, bottom_rect) = allocate_game_zone_and_bottom(ui);
                    if resp.clicked() {
                        focus_request.replace(Some(resp.id));
                    }
                    let painter = ui.painter();
                    let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                    let (cx, cy) = (state.player.x, state.player.y);
                    let cursor_world = cursor_screen
                        .filter(|p| game_rect.contains(*p))
                        .map(|p| screen_to_world(game_rect, p.x, p.y, cx, cy));
                    let facing_left = cursor_world
                        .map(|(cw, _)| cw < state.player.x)
                        .unwrap_or_else(|| matches!(state.player.dir, Dir8::W | Dir8::NW | Dir8::SW));
                    let player_sprite = if is_moving {
                        self.player_walk_texture.as_ref()
                            .zip(self.player_walk_desc.as_ref())
                            .map(|(t, d)| (t, d, frame_walk, facing_left))
                    } else {
                        self.player_sprite_texture.as_ref()
                            .zip(self.player_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, frame_idle, facing_left))
                    };
                    let enemy_sprite = self.enemy_sprite_texture.as_ref()
                        .zip(self.enemy_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, enemy_frame));
                    let enemy_miniboss_sprite = self.enemy_miniboss_sprite_texture.as_ref()
                        .zip(self.enemy_miniboss_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, enemy_frame));
                    let troop_frame = ((self.troop_anim_accumulator * 8.0) as usize) % 8;
                    let troop_walk_sprite = self.troop_sprite_texture.as_ref()
                        .zip(self.troop_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, troop_frame));
                    let troop_attack_sprite = self.troop_attack_sprite_texture.as_ref()
                        .zip(self.troop_attack_sprite_desc.as_ref());
                    if self.construction_open {
                        if resp.clicked() {
                            let pos = resp.interact_pointer_pos()
                                .or_else(|| ui.ctx().input(|i| i.pointer.press_origin()));
                            if let Some(pos) = pos {
                                let (ci, cj) = screen_to_cell(
                                    game_rect,
                                    pos.x,
                                    pos.y,
                                    state.player.x,
                                    state.player.y,
                                    state.castle.x,
                                    state.castle.y,
                                );
                                self.construction_selected_cell = Some((ci, cj));
                            }
                        }
                        if resp.secondary_clicked() {
                            self.construction_selected_cell = None;
                        }
                    }
                    let (tick, go) = game_zone_tick_and_paint(
                        game_rect,
                        &painter,
                        state,
                        last_tick,
                        keys,
                        self.keys_secondary,
                        ui.visuals().window_fill(),
                        cursor_screen,
                        player_sprite,
                        enemy_sprite,
                        enemy_miniboss_sprite,
                        troop_walk_sprite,
                        troop_attack_sprite,
                        self.construction_open,
                        self.construction_selected_cell,
                    );
                    *result.borrow_mut() = (tick, go);
                    paint_bottom_panel(ui, bottom_rect, state);
                    let (new_tick, game_over) = *result.borrow();
                    self.last_tick = new_tick;
                    if game_over {
                    self.screen = Screen::GameOver;
                    }
                }
                self.game = state_opt;
            }
            Screen::Battle => {
                if let Some(ref mut state) = self.game {
                    let delta = self.last_tick.elapsed().as_secs_f32();
                    self.player_anim_accumulator += delta;
                    self.enemy_anim_accumulator += delta;
                    self.troop_anim_accumulator += delta;
                    let is_moving = self.keys.iter().any(|&k| k);
                    let frame_idle = ((self.player_anim_accumulator * 6.0) as usize) % 6;
                    let frame_walk = ((self.player_anim_accumulator * 8.0) as usize) % 8;
                    let enemy_frame = ((self.enemy_anim_accumulator * 8.0) as usize) % 8;
                    let (game_rect, resp, bottom_rect) = allocate_game_zone_and_bottom(ui);
                    if resp.clicked() {
                        focus_request.replace(Some(resp.id));
                    }
                    let painter = ui.painter();
                    let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                    let (cx, cy) = (state.player.x, state.player.y);
                    let cursor_world = cursor_screen
                        .filter(|p| game_rect.contains(*p))
                        .map(|p| screen_to_world(game_rect, p.x, p.y, cx, cy));
                    let facing_left = cursor_world
                        .map(|(cw, _)| cw < state.player.x)
                        .unwrap_or_else(|| matches!(state.player.dir, Dir8::W | Dir8::NW | Dir8::SW));
                    let player_sprite = if is_moving {
                        self.player_walk_texture.as_ref()
                            .zip(self.player_walk_desc.as_ref())
                            .map(|(t, d)| (t, d, frame_walk, facing_left))
                    } else {
                        self.player_sprite_texture.as_ref()
                            .zip(self.player_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, frame_idle, facing_left))
                    };
                    let enemy_sprite = self.enemy_sprite_texture.as_ref()
                        .zip(self.enemy_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, enemy_frame));
                    let enemy_miniboss_sprite = self.enemy_miniboss_sprite_texture.as_ref()
                        .zip(self.enemy_miniboss_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, enemy_frame));
                    let troop_frame = ((self.troop_anim_accumulator * 8.0) as usize) % 8;
                    let troop_walk_sprite = self.troop_sprite_texture.as_ref()
                        .zip(self.troop_sprite_desc.as_ref())
                        .map(|(t, d)| (t, d, troop_frame));
                    let troop_attack_sprite = self.troop_attack_sprite_texture.as_ref()
                        .zip(self.troop_attack_sprite_desc.as_ref());
                    let (new_tick, game_over) = game_zone_tick_and_paint(
                        game_rect,
                        &painter,
                        state,
                        self.last_tick,
                        self.keys,
                        self.keys_secondary,
                        ui.visuals().window_fill(),
                        cursor_screen,
                        player_sprite,
                        enemy_sprite,
                        enemy_miniboss_sprite,
                        troop_walk_sprite,
                        troop_attack_sprite,
                        self.construction_open,
                        self.construction_selected_cell,
                    );
                    self.last_tick = new_tick;
                    paint_bottom_panel(ui, bottom_rect, state);
                    if game_over {
                        self.screen = Screen::GameOver;
                    }
                    // Overlay victoire de vague
                    if state.is_wave_won() {
                        let rect = game_rect;
                        if rect.width() > 0.0 && rect.height() > 0.0 {
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
                if state.inventory.len() >= INVENTORY_MAX_SLOTS {
                    self.show_encumbered_overlay = true;
                } else {
                    state.start_battle_phase();
                    self.screen = Screen::Battle;
                }
            }
        }

        self.paint_level_up_toasts(ui.ctx());
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
            ui.weak("Un objet non identifié ne peut pas être équipé.");
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
                        .add_enabled(can_expert, egui::Button::new("Par un expert (20 or)"))
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

impl App for LordOfTheCastleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.game.is_some() {
            self.drain_level_up_toasts();
        }
        // Input clavier : si joueur 2 présent, J1 = OKLM (O=haut, K=gauche, L=bas, M=droite), J2 = ZQSD ; sinon J1 = ZQSD + flèches
        let input = ctx.input(Clone::clone);
        use egui::Key;
        let two_players = self.game.as_ref().map(|s| s.secondary_player.is_some()).unwrap_or(false);
        let (up1, down1, left1, right1) = if two_players {
            (input.key_down(Key::O), input.key_down(Key::L), input.key_down(Key::K), input.key_down(Key::M))
        } else {
            (input.key_down(Key::Z) || input.key_down(Key::W) || input.key_down(Key::ArrowUp),
             input.key_down(Key::S) || input.key_down(Key::ArrowDown),
             input.key_down(Key::Q) || input.key_down(Key::A) || input.key_down(Key::ArrowLeft),
             input.key_down(Key::D) || input.key_down(Key::ArrowRight))
        };
        self.keys[0] = up1 && !left1 && !right1;
        self.keys[1] = up1 && right1;
        self.keys[2] = right1 && !up1 && !down1;
        self.keys[3] = down1 && right1;
        self.keys[4] = down1 && !left1 && !right1;
        self.keys[5] = down1 && left1;
        self.keys[6] = left1 && !up1 && !down1;
        self.keys[7] = up1 && left1;
        let (up2, down2, left2, right2) = (input.key_down(Key::Z), input.key_down(Key::S), input.key_down(Key::Q), input.key_down(Key::D));
        self.keys_secondary[0] = up2 && !left2 && !right2;
        self.keys_secondary[1] = up2 && right2;
        self.keys_secondary[2] = right2 && !up2 && !down2;
        self.keys_secondary[3] = down2 && right2;
        self.keys_secondary[4] = down2 && !left2 && !right2;
        self.keys_secondary[5] = down2 && left2;
        self.keys_secondary[6] = left2 && !up2 && !down2;
        self.keys_secondary[7] = up2 && left2;

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
                        let prep_enabled = state.phase == GamePhase::Preparation;
                        if ui.add_enabled(prep_enabled, egui::Button::new("Compétences")).clicked() && prep_enabled {
                            self.competences_open ^= true;
                        }
                        if ui.add_enabled(prep_enabled, egui::Button::new("Marchand")).clicked() && prep_enabled {
                            self.marchand_open ^= true;
                        }
                        if ui.add_enabled(prep_enabled, egui::Button::new("Deckard Rain")).clicked() && prep_enabled {
                            self.expert_open ^= true;
                        }
                        if ui.add_enabled(prep_enabled, egui::Button::new("Construction")).clicked() && prep_enabled {
                            self.construction_open ^= true;
                        }
                        if ui.add_enabled(prep_enabled, egui::Button::new("Recrutement")).clicked() && prep_enabled {
                            self.recrutement_open ^= true;
                        }
                        if ui.add_enabled(prep_enabled, egui::Button::new("Joueur 2")).clicked() && prep_enabled {
                            self.joueur2_window_open = true;
                        }
                        if state.phase == GamePhase::Preparation {
                            if ui.button("Lancer la vague").clicked() {
                                if state.inventory.len() >= INVENTORY_MAX_SLOTS {
                                    self.show_encumbered_overlay = true;
                                } else {
                                    state.start_battle_phase();
                                    self.screen = Screen::Battle;
                                }
                            }
                        }
                        ui.label(format!("Vague {}", state.wave_number));
                        ui.label(format!("Ennemis: {}", state.enemies.len()));
                        let troops_count_standalone = state.troops.iter().filter(|t| t.is_active_in_squad()).count();
                        ui.label(format!("Troupes: {} / {}", troops_count_standalone, state.max_troops()));
                        if ui.button("Mode Dev").clicked() {
                            self.dev_mode_window_open = true;
                        }
                        if ui.button("Sauvegarder").clicked() {
                            self.save_requested = true;
                        }
                        if ui.button("Charger").clicked() {
                            self.load_requested = true;
                        }
                        if ui.button("Tuto").clicked() {
                            self.tutorial_window_open = true;
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Or: {}", state.gold));
                        });
                    }
                });
                if self.save_requested {
                    self.save_requested = false;
                    match self.do_save() {
                        Ok(()) => self.save_load_message = Some(("Sauvegarde OK.".into(), Instant::now())),
                        Err(e) => self.save_load_message = Some((format!("Erreur: {}", e), Instant::now())),
                    }
                }
                if self.load_requested {
                    self.load_requested = false;
                    match self.do_load() {
                        Ok(()) => self.save_load_message = Some(("Partie chargée.".into(), Instant::now())),
                        Err(e) => self.save_load_message = Some((format!("Erreur: {}", e), Instant::now())),
                    }
                }
                if let Some((msg, t)) = &self.save_load_message {
                    if t.elapsed().as_secs_f32() > 2.0 {
                        self.save_load_message = None;
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(120, 255, 120), msg.as_str());
                    }
                }
            });

        // Bandeau « PHASE DE REPOS » sous le header (visible uniquement en phase Préparation)
        if let Some(ref state) = self.game {
            if state.phase == GamePhase::Preparation {
                egui::TopBottomPanel::top("lotc_phase_bandeau")
                    .min_height(28.0)
                    .show(ctx, |ui| {
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(255, 220, 0))
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(egui::RichText::new("PHASE DE REPOS").color(egui::Color32::BLACK).strong());
                                });
                            });
                    });
            }
        }

        // Fenêtre Joueur 2 (standalone)
        if self.joueur2_window_open {
            let mut open = true;
            let mut should_close = false;
            egui::Window::new("Joueur 2")
                .open(&mut open)
                .default_width(320.0)
                .show(ctx, |ui| {
                    ui.label("Spawn un personnage secondaire (2 joueurs) :");
                    ui.add_space(6.0);
                    if let Some(ref mut state) = self.game {
                        if state.secondary_player.is_some() {
                            ui.label("Un joueur secondaire est déjà présent.");
                            if ui.button("lvlup (joueur 2)").clicked() {
                                state.secondary_give_level_up();
                            }
                        } else {
                            if ui.button("Tombilol — 30 PV, attaque 360° 30 px, 1 att/s").clicked() {
                                state.spawn_secondary_tombilol();
                                should_close = true;
                            }
                            if ui.button("Tal Ratchou — 20 PV, projectile homing 0,6/s, 200 px").clicked() {
                                state.spawn_secondary_tal_ratchou();
                                should_close = true;
                            }
                            if ui.button("Sergent Garcia — 10 PV, 6 miliciens").clicked() {
                                state.spawn_secondary_sergent_garcia();
                                should_close = true;
                            }
                        }
                    }
                });
            if !open || should_close {
                self.joueur2_window_open = false;
            }
        }

        // Fenêtre Tutoriel (standalone)
        if self.tutorial_window_open {
            let mut tab = self.tutorial_tab;
            egui::Window::new("Tutoriel")
                .open(&mut self.tutorial_window_open)
                .default_size([420.0, 380.0])
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(tab == 0, "Comment jouer").clicked() { tab = 0; }
                        if ui.selectable_label(tab == 1, "Contrôles").clicked() { tab = 1; }
                        if ui.selectable_label(tab == 2, "Phases").clicked() { tab = 2; }
                        if ui.selectable_label(tab == 3, "Mécaniques").clicked() { tab = 3; }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match tab {
                            0 => {
                                ui.heading("Comment jouer");
                                ui.add_space(6.0);
                                ui.label("Lord of the Castle est un jeu de défense de tour et d'action. Votre objectif : protéger le Château des vagues d'ennemis.");
                                ui.add_space(4.0);
                                ui.label("• En phase Préparation : achetez équipement, construisez des tours, recrutez des troupes, montez en compétences.");
                                ui.label("• Cliquez sur « Lancer la vague » pour passer en Bataille.");
                                ui.label("• En Bataille : déplacez-vous (ZQSD), combattez les ennemis avec votre personnage et vos troupes ; les tours tirent automatiquement.");
                                ui.label("• Ramassez l'or et l'XP au sol, identifiez les objets trouvés, puis repassez en Préparation pour la vague suivante.");
                                ui.add_space(4.0);
                                ui.label("Si le Château tombe à 0 PV, c'est le Game Over.");
                            }
                            1 => {
                                ui.heading("Contrôles");
                                ui.add_space(6.0);
                                ui.label("• Déplacement : ZQSD ou flèches directionnelles (8 directions).");
                                ui.label("• Avec un joueur 2 : le joueur 1 utilise OKLM, le joueur 2 utilise ZQSD.");
                                ui.label("• Cliquez sur la zone de jeu pour lui donner le focus (clavier actif).");
                                ui.label("• Header : Player, Inventaire, Équipement, Compétences, Marchand, Expert (Deckard Rain), Construction, Recrutement, Lancer la vague, Sauvegarder, Charger.");
                                ui.label("• Ramassage : approchez-vous des orbes (or, XP) et des objets au sol (rayon ~30 px).");
                            }
                            2 => {
                                ui.heading("Phases");
                                ui.add_space(6.0);
                                ui.label("Préparation :");
                                ui.label("  • Compétences (arbre Guerrier), équipement au Marchand, construction de tours (grille 20×20), recrutement de troupes.");
                                ui.label("  • Inventaire max 20 slots ; ne lancez pas la vague si l'inventaire est plein.");
                                ui.add_space(4.0);
                                ui.label("Bataille :");
                                ui.label("  • Ennemis spawnent et marchent vers le Château. Vous, les tours et les troupes les attaquez.");
                                ui.label("  • Quand tous les ennemis sont morts, la vague est gagnée : or, XP et objets récupérables au sol.");
                                ui.label("  • Passez à la vague suivante en revenant en Préparation.");
                            }
                            3 => {
                                ui.heading("Mécaniques");
                                ui.add_space(6.0);
                                ui.label("Joueur : attaque auto à portée (~40 px), 1 att/s ; dégâts et PV selon équipement et stats. Mort à 0 PV : revive après la vague avec −1 PV max.");
                                ui.label("Tours : coût 50 or, champ de vision 300 px, flèches 600 px, 1 tir / 2 s.");
                                ui.label("Troupes : suivent le joueur dans un rayon ~200 px, combattent les ennemis à portée (ex. Milicien 3 dégâts, 25 px).");
                                ui.label("Ennemis : priorité Joueur > Tour > Château ; donnent or, XP et parfois objets à la mort.");
                                ui.label("Loot : or et XP en orbes ; objets à identifier (Marchand ou Expert 20 or).");
                            }
                            _ => {}
                        }
                    });
                });
            self.tutorial_tab = tab;
        }

        // Fenêtre Mode Dev (standalone)
        if self.dev_mode_window_open {
            let mut open = true;
            egui::Window::new("Mode Dev")
                .open(&mut open)
                .default_width(280.0)
                .show(ctx, |ui| {
                    if let Some(ref mut state) = self.game {
                        ui.checkbox(&mut state.dev_mode, "Visions des portées");
                        ui.add_space(6.0);
                        if ui.button("+ 100 or").clicked() {
                            state.gold += 100;
                        }
                        if ui.button("Spawn 50 ennemis normal").clicked() {
                            state.dev_spawn_50_normal_enemies_random();
                        }
                        if ui.button("lvlup").clicked() {
                            state.dev_give_level_up();
                        }
                    }
                });
            if !open {
                self.dev_mode_window_open = false;
            }
        }

        // Overlay « trop encombré » : inventaire plein au clic sur Lancer la vague.
        if self.show_encumbered_overlay {
            let mut open = true;
            egui::Window::new("Attention")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Vous êtes trop encombré pour vous battre. Vendez quelques objets avant de partir au combat.");
                    ui.add_space(8.0);
                    if ui.button("OK").clicked() {
                        open = false;
                    }
                });
            if !open {
                self.show_encumbered_overlay = false;
            }
        }

        // Fenêtre Player (métriques joueur) — standalone
        if self.player_window_open {
            let mut open = true;
            egui::Window::new("Player")
                .open(&mut open)
                .default_width(280.0)
                .show(ctx, |ui| paint_player_window(ui, self.game.as_mut()));
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
                    paint_inventory_window(ui, self.game.as_mut(), &mut self.selected_inventory_slot, &self.pending_identify_self);
                });
            if !open {
                self.inventory_window_open = false;
                self.selected_inventory_slot = None;
            }
        }

        // Fenêtre Détail / Identification (objet sélectionné)
        if let Some(slot_idx) = self.selected_inventory_slot {
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
        if let Some(slot) = self.selected_equipment_slot {
            egui::Window::new("Détail équipement")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    paint_equipment_item_detail(ui, self, slot);
                });
        }

        // Fenêtre Compétences (5 onglets)
        if self.competences_open {
            let mut open = true;
            egui::Window::new("Compétences")
                .open(&mut open)
                .default_width(420.0)
                .default_height(380.0)
                .show(ctx, |ui| paint_skills_window(ui, self.game.as_mut(), &mut self.skills_tab, &mut self.selected_warrior_skill));
            if !open {
                self.competences_open = false;
            }
        }

        // Fenêtres Mode préparation (Marchand, Expert, Construction, Recrutement)
        let mut close_marchand = false;
        let mut close_expert = false;
        let mut close_construction = false;
        let mut close_recrutement = false;
        if let Some(ref mut state) = self.game {
            let gold = state.gold;
            if self.marchand_open {
                let mut open = true;
                egui::Window::new("Marchand")
                    .open(&mut open)
                    .default_width(380.0)
                    .default_height(480.0)
                    .show(ctx, |ui| {
                        ui.label(format!("Or : {} or", gold));
                        if state.dev_mode {
                            if ui.button("+100 or (dev)").clicked() {
                                self.pending_dev_add_gold.replace(true);
                            }
                        }
                        if cfg!(debug_assertions) {
                            if ui.button("🔄 Reroll pools (dev)").clicked() {
                                self.pending_merchant_reroll.replace(true);
                            }
                        }
                        ui.add_space(6.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.heading("Armes");
                            for (i, opt) in state.merchant_weapons.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Weapon, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                            ui.add_space(8.0);
                            ui.heading("Armures");
                            for (i, opt) in state.merchant_armor.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Armor, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                            ui.add_space(8.0);
                            ui.heading("Accessoires");
                            for (i, opt) in state.merchant_accessories.iter().enumerate() {
                                match opt {
                                    Some(me) => {
                                        match &me.entry {
                                            InventoryEntry::Identified(item) => {
                                                let (r, g, b) = item.rarity.color_rgb();
                                                ui.colored_label(egui::Color32::from_rgb(r, g, b), &item.display_name);
                                            }
                                            InventoryEntry::Unidentified(slot) => {
                                                ui.label(format!("Objet non identifié — {} — {} or", slot.label(), me.price));
                                            }
                                        }
                                        if ui.button(format!("Acheter ({} or)", me.price)).clicked() && gold >= me.price {
                                            self.pending_merchant_buy.borrow_mut().replace((MerchantPoolKind::Accessory, i));
                                        }
                                    }
                                    None => {
                                        ui.weak("Slot vide");
                                    }
                                }
                            }
                        });
                    });
                if !open {
                    close_marchand = true;
                }
            }
            if self.expert_open {
                let mut open = true;
                egui::Window::new("Expert en identification")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ctx, |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.add_space(8.0);
                        ui.heading("Expert en identification");
                        ui.label("Identification groupée : identifier tous les objets de l'inventaire en une fois (prix cumulé).");
                        ui.weak("(À implémenter : bouton et coût total)");
                    });
                if !open {
                    close_expert = true;
                }
            }
            if self.construction_open {
                let mut open = true;
                egui::Window::new("Construction")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ctx, |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.add_space(8.0);
                        ui.heading("Bâtiments constructibles");
                        ui.label("Sélectionnez une case sur la zone de jeu (grille 20×20). Clic droit pour annuler.");
                        ui.add_space(8.0);
                        if let Some((ci, cj)) = self.construction_selected_cell {
                            ui.label(format!("Case sélectionnée : ({}, {})", ci, cj));
                            let has_place = state.can_build_at_cell(ci, cj);
                            let has_gold = state.gold >= TOWER_BASE_COST_GOLD;
                            if has_place {
                                if has_gold {
                                    if ui.button(format!("Tour ({} po) — Construire dedans", TOWER_BASE_COST_GOLD)).clicked() {
                                        state.build_tower_at_cell(ci, cj);
                                    }
                                } else {
                                    ui.weak(format!("Or insuffisant ({} po requis).", TOWER_BASE_COST_GOLD));
                                }
                            } else {
                                ui.weak("Pas la place ici (château ou autre bâtiment).");
                            }
                        } else {
                            ui.weak("Aucune case sélectionnée.");
                        }
                    });
                if !open {
                    close_construction = true;
                }
            }
            if self.recrutement_open {
                let mut open = true;
                let troops_count = state.troops.iter().filter(|t| t.is_active_in_squad()).count();
                let max_troops = state.max_troops();
                egui::Window::new("Recrutement")
                    .open(&mut open)
                    .default_width(320.0)
                    .show(ctx, |ui| {
                        ui.label(format!("Or : {} or", gold));
                        ui.label(format!("Troupes : {} / {}", troops_count, max_troops));
                        ui.add_space(8.0);
                        ui.heading("Troupes disponibles");
                        ui.horizontal(|ui| {
                            ui.label(TroopKind::Milicien.label());
                            ui.label("— 100 PV, 20 % blocage, 6 dégâts, 1 att/s, portée 25 px.");
                            if ui.add_enabled(troops_count < max_troops, egui::Button::new("Recruter")).clicked()
                                && troops_count < max_troops
                            {
                                state.recruit_troop(TroopKind::Milicien);
                            }
                        });
                    });
                if !open {
                    close_recrutement = true;
                }
            }
        }
        if close_marchand {
            self.marchand_open = false;
        }
        if close_expert {
            self.expert_open = false;
        }
        if close_construction {
            self.construction_open = false;
            self.construction_selected_cell = None;
        }
        if close_recrutement {
            self.recrutement_open = false;
        }
        if let Some((pool, index)) = self.pending_merchant_buy.borrow_mut().take() {
            if let Some(ref mut state) = self.game {
                match pool {
                    MerchantPoolKind::Weapon => {
                        state.buy_merchant_weapon(index);
                    }
                    MerchantPoolKind::Armor => {
                        state.buy_merchant_armor(index);
                    }
                    MerchantPoolKind::Accessory => {
                        state.buy_merchant_accessory(index);
                    }
                }
            }
        }
        if self.pending_merchant_reroll.replace(false) {
            if let Some(ref mut state) = self.game {
                state.refresh_merchant_pools();
            }
        }
        if self.pending_dev_add_gold.replace(false) {
            if let Some(ref mut state) = self.game {
                state.gold += 100;
            }
        }
        if let Some(idx) = self.pending_identify_self.borrow_mut().take() {
            if let Some(ref mut state) = self.game {
                state.identify_self(idx);
            }
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
                    let mut state_opt = self.game.take();
                    if let Some(ref mut state) = state_opt {
                        let last_tick = self.last_tick;
                        let keys = self.keys;
                        let delta = last_tick.elapsed().as_secs_f32();
                        self.player_anim_accumulator += delta;
                        self.enemy_anim_accumulator += delta;
                        self.troop_anim_accumulator += delta;
                        let is_moving = keys.iter().any(|&k| k);
                        let frame_idle = ((self.player_anim_accumulator * 6.0) as usize) % 6;
                        let frame_walk = ((self.player_anim_accumulator * 8.0) as usize) % 8;
                        let enemy_frame = ((self.enemy_anim_accumulator * 8.0) as usize) % 8;
                        let result = RefCell::new((last_tick, false));
                        let (game_rect, resp, bottom_rect) = allocate_game_zone_and_bottom(ui);
                        if resp.clicked() {
                            ctx.memory_mut(|m| m.request_focus(resp.id));
                        }
                        if self.construction_open {
                            if resp.clicked() {
                                let pos = resp.interact_pointer_pos()
                                    .or_else(|| ui.ctx().input(|i| i.pointer.press_origin()));
                                if let Some(pos) = pos {
                                    let (ci, cj) = screen_to_cell(
                                        game_rect,
                                        pos.x,
                                        pos.y,
                                        state.player.x,
                                        state.player.y,
                                        state.castle.x,
                                        state.castle.y,
                                    );
                                    self.construction_selected_cell = Some((ci, cj));
                                }
                            }
                            if resp.secondary_clicked() {
                                self.construction_selected_cell = None;
                            }
                        }
                        let painter = ui.painter();
                        let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                        let (cx, cy) = (state.player.x, state.player.y);
                        let cursor_world = cursor_screen
                            .filter(|p| game_rect.contains(*p))
                            .map(|p| screen_to_world(game_rect, p.x, p.y, cx, cy));
                        let facing_left = cursor_world
                            .map(|(cw, _)| cw < state.player.x)
                            .unwrap_or_else(|| matches!(state.player.dir, Dir8::W | Dir8::NW | Dir8::SW));
                        let player_sprite = if is_moving {
                            self.player_walk_texture.as_ref()
                                .zip(self.player_walk_desc.as_ref())
                                .map(|(t, d)| (t, d, frame_walk, facing_left))
                        } else {
                            self.player_sprite_texture.as_ref()
                                .zip(self.player_sprite_desc.as_ref())
                                .map(|(t, d)| (t, d, frame_idle, facing_left))
                        };
                        let enemy_sprite = self.enemy_sprite_texture.as_ref()
                            .zip(self.enemy_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, enemy_frame));
                        let enemy_miniboss_sprite = self.enemy_miniboss_sprite_texture.as_ref()
                            .zip(self.enemy_miniboss_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, enemy_frame));
                        let troop_frame = ((self.troop_anim_accumulator * 8.0) as usize) % 8;
                        let troop_walk_sprite = self.troop_sprite_texture.as_ref()
                            .zip(self.troop_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, troop_frame));
                        let troop_attack_sprite = self.troop_attack_sprite_texture.as_ref()
                            .zip(self.troop_attack_sprite_desc.as_ref());
                        let (tick, go) = game_zone_tick_and_paint(
                            game_rect,
                            &painter,
                            state,
                            last_tick,
                            keys,
                            self.keys_secondary,
                            ui.visuals().window_fill(),
                            cursor_screen,
                            player_sprite,
                            enemy_sprite,
                            enemy_miniboss_sprite,
                            troop_walk_sprite,
                            troop_attack_sprite,
                            self.construction_open,
                            self.construction_selected_cell,
                        );
                        *result.borrow_mut() = (tick, go);
                        paint_bottom_panel(ui, bottom_rect, state);
                        let (new_tick, game_over) = *result.borrow();
                        self.last_tick = new_tick;
                        if game_over {
                            self.screen = Screen::GameOver;
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
                    self.game = state_opt;
                }
                Screen::Battle => {
                    if let Some(ref mut state) = self.game {
                        let delta = self.last_tick.elapsed().as_secs_f32();
                        self.player_anim_accumulator += delta;
                        self.enemy_anim_accumulator += delta;
                        self.troop_anim_accumulator += delta;
                        let is_moving = self.keys.iter().any(|&k| k);
                        let frame_idle = ((self.player_anim_accumulator * 6.0) as usize) % 6;
                        let frame_walk = ((self.player_anim_accumulator * 8.0) as usize) % 8;
                        let enemy_frame = ((self.enemy_anim_accumulator * 8.0) as usize) % 8;
                        let (game_rect, resp, bottom_rect) = allocate_game_zone_and_bottom(ui);
                        if resp.clicked() {
                            ctx.memory_mut(|m| m.request_focus(resp.id));
                        }
                        let painter = ui.painter();
                        let cursor_screen = ui.ctx().input(|i| i.pointer.hover_pos());
                        let (cx, cy) = (state.player.x, state.player.y);
                        let cursor_world = cursor_screen
                            .filter(|p| game_rect.contains(*p))
                            .map(|p| screen_to_world(game_rect, p.x, p.y, cx, cy));
                        let facing_left = cursor_world
                            .map(|(cw, _)| cw < state.player.x)
                            .unwrap_or_else(|| matches!(state.player.dir, Dir8::W | Dir8::NW | Dir8::SW));
                        let player_sprite = if is_moving {
                            self.player_walk_texture.as_ref()
                                .zip(self.player_walk_desc.as_ref())
                                .map(|(t, d)| (t, d, frame_walk, facing_left))
                        } else {
                            self.player_sprite_texture.as_ref()
                                .zip(self.player_sprite_desc.as_ref())
                                .map(|(t, d)| (t, d, frame_idle, facing_left))
                        };
                        let enemy_sprite = self.enemy_sprite_texture.as_ref()
                            .zip(self.enemy_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, enemy_frame));
                        let enemy_miniboss_sprite = self.enemy_miniboss_sprite_texture.as_ref()
                            .zip(self.enemy_miniboss_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, enemy_frame));
                        let troop_frame = ((self.troop_anim_accumulator * 8.0) as usize) % 8;
                        let troop_walk_sprite = self.troop_sprite_texture.as_ref()
                            .zip(self.troop_sprite_desc.as_ref())
                            .map(|(t, d)| (t, d, troop_frame));
                        let troop_attack_sprite = self.troop_attack_sprite_texture.as_ref()
                            .zip(self.troop_attack_sprite_desc.as_ref());
                        let (new_tick, game_over) = game_zone_tick_and_paint(
                            game_rect,
                            &painter,
                            state,
                            self.last_tick,
                            self.keys,
                            self.keys_secondary,
                            ui.visuals().window_fill(),
                            cursor_screen,
                            player_sprite,
                            enemy_sprite,
                            enemy_miniboss_sprite,
                            troop_walk_sprite,
                            troop_attack_sprite,
                            self.construction_open,
                            self.construction_selected_cell,
                        );
                        self.last_tick = new_tick;
                        paint_bottom_panel(ui, bottom_rect, state);
                        if game_over {
                            self.screen = Screen::GameOver;
                        }
                        // Overlay victoire de vague : félicitations + résumé + "Phase suivante"
                        if state.is_wave_won() {
                            let rect = game_rect;
                            if rect.width() > 0.0 && rect.height() > 0.0 {
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

        self.paint_level_up_toasts(ctx);
        ctx.request_repaint();
    }
}
