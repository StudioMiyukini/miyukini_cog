//! Application principale du Hub Miyukini Central.
//!
//! Gère l'interface utilisateur, le header, les onglets, le menu services,
//! et l'affichage du contenu selon l'onglet actif.
//!
//! @id: miyukini_central_app_module
//! @do: render_hub_ui_and_services
//! @role: interface
//! @layer: application
//! @human: Hub Central : header, onglets, grille services, overlays.

use crate::auth::{
    validate_password, validate_profiles_with_mother, CentralAuthDb, CentralProfile,
    password_rules_hint,
};
use crate::catalog::{meta_for_service, mock_catalog, CategoryId, ServiceId, ServiceMeta};
use crate::config::load_supabase_config;
use crate::loading::{LoadingState, LOADING_PHRASES};
use crate::lucide_icons;
use crate::pixel_theme::{chrome_dimensions, chrome_shapes, PixelChromeTheme};
use crate::services::{
    CalculatorService, DevCenterService, EguiEditorService, EditorThemeState, GameService,
    JayFestivalService, JayKoaService, JayXposeService, LordOfTheCastleService, MiyuClickerService,
    MockJayService,
    NotesService, ServiceUi, TextEditorService, UiLibraryService, UI_EDITOR_THEME_STORAGE_KEY,
};
use crate::usage_tracking::{UsageContext, UsageStore};
use eframe::egui;
use eframe::App;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Mode de la fenêtre auth : Connexion ou Créer un compte.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[allow(missing_docs)]
pub enum AuthMode {
    #[default]
    Connexion,
    CreerCompte,
}

/// Application principale du Hub.
pub struct MiyukiniCentralApp {
    /// État du header (onglets).
    pub header_state: HeaderState,
    /// Services ouverts (onglets actifs).
    pub open_services: Vec<Box<dyn ServiceUi>>,
    /// Catalogue des services disponibles.
    pub catalog: Vec<ServiceId>,
    /// Vue actuelle dans l'onglet HUB.
    pub hub_view: HubView,
    /// Filtre actif dans le menu services.
    pub menu_filter: Option<CategoryId>,
    /// Fenêtre overlay Profil ouverte.
    pub profile_overlay_open: bool,
    /// Fenêtre overlay Paramètres ouverte.
    pub settings_overlay_open: bool,
    /// Mode obscur (true) ou clair (false). Persistant via storage.
    pub dark_mode: bool,
    /// À true quand l'utilisateur a changé le thème dans Paramètres (pour sauvegarder en storage).
    pub theme_preference_changed: bool,
    /// Icônes Lucide du header (handle pour garder la texture vivante).
    pub header_icon_cog: Option<egui::TextureHandle>,
    /// Icône utilisateur (circle-user).
    pub header_icon_user: Option<egui::TextureHandle>,
    /// Icône paramètres (settings).
    pub header_icon_settings: Option<egui::TextureHandle>,
    /// True une fois l’écran de chargement terminé.
    pub loading_done: bool,
    /// État de l’écran de chargement (durée 10–12 s, barre à-coups, phrases).
    pub loading_state: LoadingState,
    /// État de la sidebar HUB : recherche, filtre, favoris, service sélectionné.
    pub hub_sidebar: HubSidebarState,
    /// Utilisateur connecté (true) ou non (false) — pour affichage Connexion/Déconnexion.
    pub connected: bool,
    /// Profil Miyukini COG connecté (email, id).
    pub current_profile: Option<CentralProfile>,
    /// Fenêtre Connexion / Créer un compte ouverte (ouverte par le bouton Connexion).
    pub auth_window_open: bool,
    /// Mode de la fenêtre auth : Connexion ou Créer un compte.
    pub auth_mode: AuthMode,
    /// Email saisi dans la fenêtre auth.
    pub auth_email: String,
    /// Mot de passe saisi dans la fenêtre auth.
    pub auth_password: String,
    /// Répétition du mot de passe (création de compte).
    pub auth_password_repeat: String,
    /// Message d'erreur affiché dans la fenêtre auth.
    pub auth_error: Option<String>,
    /// Base des profils Miyukini COG (SQLite).
    pub auth_db: Arc<CentralAuthDb>,
    /// IDs des Services favoris (ordre d’ajout).
    pub favorites: Vec<ServiceId>,
    /// Store de suivi d'utilisation (sessions Central / Services).
    pub usage_store: UsageStore,
    /// Configuration Supabase (Catakana), chargée depuis supabase-catakana.env.
    pub supabase_config: crate::config::SupabaseConfig,
    /// Début de la session en cours (timestamp sec).
    pub current_session_start: Option<f64>,
    /// Contexte de la session en cours (Central ou Service).
    pub current_session_context: Option<UsageContext>,
    /// Fenêtre overlay « Statistiques d'utilisation » ouverte depuis le profil.
    pub usage_stats_overlay_open: bool,
    /// Thème global édité par Miyukini UI Editor. Appliqué à toute l'app en temps réel ; None = défaut (pixel_theme).
    pub global_ui_theme: Option<EditorThemeState>,
    /// Dernière validation des ID de profils auprès de la DB mère (timestamp sec). None = jamais fait (première connexion internet).
    pub last_mother_validation_at: Option<f64>,
    /// Intervalle (sec) entre deux validations auprès de la DB mère.
    pub mother_validation_interval_sec: f64,
    /// True tant qu'un thread de validation mère est en cours.
    pub mother_validation_in_progress: bool,
    /// Réception du résultat de validation (liste (old_id, new_id) à appliquer en local).
    pub mother_validation_rx: Option<mpsc::Receiver<Result<Vec<(String, String)>, String>>>,
    /// À true quand la fenêtre Profil vient de s'ouvrir : on recopie current_profile dans les champs d'édition.
    pub profile_edit_pending_sync: bool,
    /// Champs d'édition du profil (fenêtre Profil).
    pub profile_edit_pseudonyme: String,
    /// Nom (édition profil).
    pub profile_edit_nom: String,
    /// Prénom (édition profil).
    pub profile_edit_prenom: String,
    /// Date de naissance (édition profil).
    pub profile_edit_date_naissance: String,
    /// Téléphone (édition profil).
    pub profile_edit_telephone: String,
    /// Email (édition profil).
    pub profile_edit_email: String,
    /// Numéro de voie (édition profil).
    pub profile_edit_numero_voie: String,
    /// Rue (édition profil).
    pub profile_edit_rue: String,
    /// Code postal (édition profil).
    pub profile_edit_code_postal: String,
    /// Ville (édition profil).
    pub profile_edit_ville: String,
    /// Erreur affichée après un clic sur Sauvegarder le profil.
    pub profile_save_error: Option<String>,
    /// État du système de fidélisation (points MIYU, achievements, items, missions).
    pub loyalty_state: crate::loyalty::LoyaltyState,
    /// Notifications de fidélisation à afficher.
    pub loyalty_notifications: Vec<crate::loyalty::LoyaltyNotification>,
}

/// Couleurs header/Hub/onglets résolues : UI Editor (si override) ou palette Chrome.
#[derive(Clone, Copy)]
struct ResolvedChromeColors {
    barre_exe_bg: egui::Color32,
    tab_bar_bg: egui::Color32,
    sidebar_bg: egui::Color32,
    body_bg: egui::Color32,
    tab_inactive_bg: egui::Color32,
    tab_inactive_hover_bg: egui::Color32,
    tab_active_text: egui::Color32,
    tab_inactive_text: egui::Color32,
    tab_separator: egui::Color32,
    close_btn_fg: egui::Color32,
    close_btn_hover_fg: egui::Color32,
    close_btn_hover_bg: egui::Color32,
}

/// État de la sidebar HUB (recherche, filtrage, service sélectionné).
#[derive(Default)]
pub struct HubSidebarState {
    /// Texte de recherche (nom ou description).
    pub search_query: String,
    /// Filtre par catégorie (None = toutes).
    pub category_filter: Option<CategoryId>,
    /// Service sélectionné dans la liste (pour affichage détail dans le body).
    pub selected_service_id: Option<ServiceId>,
}


/// État du header (onglets).
#[derive(Default)]
pub struct HeaderState {
    /// Onglets ouverts (HUB en premier, puis services).
    pub tabs: Vec<TabInfo>,
    /// Index de l'onglet actif.
    pub active_tab_index: usize,
}

/// Information sur un onglet.
#[derive(Clone)]
pub struct TabInfo {
    /// ID du service (None pour HUB et Profile).
    pub service_id: Option<ServiceId>,
    /// Titre affiché dans l'onglet.
    pub title: String,
    /// Type d'onglet.
    pub tab_type: TabType,
}

/// Type d'onglet.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TabType {
    /// Onglet Menu (catalogue des services, toujours en premier).
    Menu,
    /// Onglet système HUB.
    Hub,
    /// Onglet service.
    Service,
    /// Onglet profil utilisateur.
    Profile,
}

/// Vue dans l'onglet HUB.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HubView {
    /// Vue d'accueil.
    Home,
    /// Vue catalogue.
    Catalog,
    /// Vue mes services.
    MyServices,
    /// Vue paramètres.
    Settings,
}

/// Résultat d'un clic sur un onglet.
#[derive(Default)]
pub struct TabClickResult {
    /// L'onglet doit être activé.
    pub activate: bool,
    /// L'onglet doit être fermé.
    pub close: bool,
}

const STORAGE_KEY_DARK_MODE: &str = "miyukini_central_dark_mode";
const STORAGE_KEY_USAGE: &str = "miyukini_central_usage_store";

/// Charge le store d'utilisation depuis le storage (JSON).
fn load_usage_store(storage: Option<&dyn eframe::Storage>) -> UsageStore {
    storage
        .and_then(|s| s.get_string(STORAGE_KEY_USAGE))
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

impl MiyukiniCentralApp {
    /// Crée une nouvelle instance de l'application.
    /// Charge le mode clair/obscur depuis le storage (persistant).
    #[must_use] 
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut header_state = HeaderState::default();
        header_state.tabs.push(TabInfo {
            service_id: None,
            title: "HUB".to_string(),
            tab_type: TabType::Hub,
        });
        header_state.active_tab_index = 0;

        let dark_mode = cc
            .storage
            .and_then(|s| s.get_string(STORAGE_KEY_DARK_MODE))
            .is_some_and(|s| s == "true");

        if dark_mode {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
        } else {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
        }

        let (header_icon_cog, header_icon_user, header_icon_settings) =
            lucide_icons::register_header_icons(cc);

        let loading_state = LoadingState::new();

        let mut app = Self {
            header_state,
            open_services: Vec::new(),
            catalog: vec![
                ServiceId::Calculator,
                ServiceId::Game,
                ServiceId::MiyuClicker,
                ServiceId::LordOfTheCastle,
                ServiceId::TextEditor,
                ServiceId::Notes,
                ServiceId::UiLibrary,
                ServiceId::EguiEditor,
                ServiceId::DevCenter,
                ServiceId::JayRDV,
                ServiceId::JayFestival,
                ServiceId::JayKoa,
                ServiceId::JayKonta,
                ServiceId::JayXpose,
                ServiceId::JayFaim,
            ],
            hub_view: HubView::Home,
            menu_filter: None,
            profile_overlay_open: false,
            settings_overlay_open: false,
            dark_mode,
            theme_preference_changed: false,
            header_icon_cog,
            header_icon_user,
            header_icon_settings,
            loading_done: false,
            loading_state,
            hub_sidebar: HubSidebarState::default(),
            connected: false,
            current_profile: None,
            auth_window_open: false,
            auth_mode: AuthMode::default(),
            auth_email: String::new(),
            auth_password: String::new(),
            auth_password_repeat: String::new(),
            auth_error: None,
            auth_db: Arc::new(
                CentralAuthDb::open("miyukini_cog.db").unwrap_or_else(|e| panic!("auth db: {e}")),
            ),
            favorites: Vec::new(),
            usage_store: load_usage_store(cc.storage),
            supabase_config: load_supabase_config(),
            current_session_start: None,
            current_session_context: None,
            usage_stats_overlay_open: false,
            global_ui_theme: cc
                .storage
                .and_then(|s| s.get_string(UI_EDITOR_THEME_STORAGE_KEY))
                .as_ref()
                .and_then(|s| EditorThemeState::from_storage_string(s)),
            last_mother_validation_at: None,
            mother_validation_interval_sec: 3600.0,
            mother_validation_in_progress: false,
            mother_validation_rx: None,
            profile_edit_pending_sync: false,
            profile_edit_pseudonyme: String::new(),
            profile_edit_nom: String::new(),
            profile_edit_prenom: String::new(),
            profile_edit_date_naissance: String::new(),
            profile_edit_telephone: String::new(),
            profile_edit_email: String::new(),
            profile_edit_numero_voie: String::new(),
            profile_edit_rue: String::new(),
            profile_edit_code_postal: String::new(),
            profile_edit_ville: String::new(),
            profile_save_error: None,
            loyalty_state: {
                let mut state = crate::loyalty::LoyaltyState::new();
                crate::loyalty::initialize_loyalty_system(&mut state);
                state
            },
            loyalty_notifications: Vec::new(),
        };

        // Débloquer les achievements de premier lancement
        crate::loyalty::check_and_unlock_achievements(
            &mut app.loyalty_state,
            &mut app.loyalty_notifications,
        );

        app
    }
}

impl App for MiyukiniCentralApp {
    /// Point d'entrée principal de l'UI.
    /// @id: miyukini_central_app_update
    /// @do: main_loop_theme_header_overlay_content
    /// @role: interface
    /// @human: Boucle principale : thème, header, overlays, contenu selon onglet.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Appliquer le thème : si thème UI Editor (global_ui_theme) présent, lui seul s'applique (boutons et tout assortis).
        // Sinon, thème Chrome (pixel_theme) pour éviter que les boutons restent gris par défaut.
        let pixel_theme = PixelChromeTheme::new(self.dark_mode);
        if let Some(ref theme) = self.global_ui_theme {
            theme.apply(ctx);
        } else {
            ctx.set_visuals(if self.dark_mode {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            });
            pixel_theme.apply(ctx);
        }

        if !self.loading_done {
            self.ui_loading_screen(ctx);
            let time_sec = ctx.input(|i| i.time);
            let mut rng = rand::thread_rng();
            let _phrase = self.loading_state.update(time_sec, &mut rng);
            if self.loading_state.is_done(time_sec) {
                self.loading_done = true;
            }
            ctx.request_repaint();
            return;
        }

        // Suivi d'utilisation : enregistrer la session en cours si l'onglet change ou toutes les 60 s, démarrer une nouvelle.
        const FLUSH_INTERVAL_SEC: f64 = 60.0;
        let time_sec = ctx.input(|i| i.time);
        let current_ctx = self.current_usage_context();
        let mut usage_modified = false;
        if let Some(ctx_now) = current_ctx {
            let need_switch = self.current_session_context.as_ref() != Some(&ctx_now);
            let need_flush = self.current_session_start.is_some_and(|start| time_sec - start >= FLUSH_INTERVAL_SEC);
            if need_switch || need_flush {
                if let (Some(start), Some(prev_ctx)) = (self.current_session_start, self.current_session_context.as_ref()) {
                    self.usage_store.record(start, time_sec, prev_ctx);
                    usage_modified = true;
                }
                self.current_session_start = Some(time_sec);
                self.current_session_context = Some(ctx_now);
            }
        } else if self.current_session_start.is_some() {
            if let (Some(start), Some(ref prev_ctx)) = (self.current_session_start.take(), self.current_session_context.take()) {
                self.usage_store.record(start, time_sec, prev_ctx);
                usage_modified = true;
            }
        }
        if usage_modified {
            if let Some(storage) = frame.storage_mut() {
                if let Ok(json) = serde_json::to_string(&self.usage_store) {
                    storage.set_string(STORAGE_KEY_USAGE, json);
                }
            }
        }

        // Validation des ID de profils auprès de la DB mère (première connexion internet puis à intervalle régulier).
        if let Some(ref rx) = self.mother_validation_rx {
            if let Ok(result) = rx.try_recv() {
                self.mother_validation_rx = None;
                self.mother_validation_in_progress = false;
                self.last_mother_validation_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs_f64(),
                );
                if let Ok(updates) = result {
                    for (old_id, new_id) in &updates {
                        let _ = self.auth_db.update_profile_id(old_id, new_id);
                    }
                    if let Some(ref mut profile) = self.current_profile {
                        for (old_id, new_id) in &updates {
                            if profile.id == *old_id {
                                profile.id = new_id.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        if let Some(ref url) = self.supabase_config.mother_db_url {
            if !self.mother_validation_in_progress && self.mother_validation_rx.is_none() {
                let now_sec = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64();
                let should_run = self.last_mother_validation_at.is_none_or(|last| {
                    now_sec - last >= self.mother_validation_interval_sec
                });
                if should_run {
                    let profiles = self.auth_db.list_profiles().unwrap_or_default();
                    if profiles.is_empty() {
                        self.last_mother_validation_at = Some(now_sec);
                    } else {
                        let url_clone = url.clone();
                        let (tx, rx) = mpsc::channel();
                        std::thread::spawn(move || {
                            let result = validate_profiles_with_mother(&url_clone, &profiles)
                                .map_err(|e| e.to_string());
                            let _ = tx.send(result);
                        });
                        self.mother_validation_rx = Some(rx);
                        self.mother_validation_in_progress = true;
                    }
                }
            }
        }

        // Header en 2 lignes.
        self.ui_header(ctx, frame);

        // Fenêtres overlay Profil et Paramètres.
        self.ui_profile_overlay(ctx);
        self.ui_settings_overlay(ctx);
        self.ui_auth_overlay(ctx);
        self.ui_usage_stats_overlay(ctx);
        self.ui_loyalty_notifications(ctx);

        // Contenu principal selon l'onglet actif (fond body : blanc en clair, R45 G45 B45 en nuit).
        // Pas de marge intérieure pour que le body + sidebar occupent toute la hauteur restante.
        let pixel_theme_body = PixelChromeTheme::new(self.dark_mode);
        let r_body = self.resolve_chrome(&pixel_theme_body);
        let body_bg = r_body.body_bg;
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(body_bg)
                    .inner_margin(egui::Margin::ZERO)
                    .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)),
            )
            .show(ctx, |ui| {
                self.ui_content(ui, frame);
            });

        // Persister l'état des services (ex. thème Miyukini UI Editor) si modifié.
        if let Some(storage) = frame.storage_mut() {
            for service in &mut self.open_services {
                service.persist_if_needed(Some(storage));
            }
        }
        // Synchroniser le thème global depuis l'UI Editor si l'onglet actif est l'éditeur (temps réel).
        if let Some(tab) = self.header_state.tabs.get(self.header_state.active_tab_index) {
            if tab.service_id == Some(ServiceId::EguiEditor) {
                if let Some(service) = self
                    .open_services
                    .iter_mut()
                    .find(|s| s.id() == ServiceId::EguiEditor)
                {
                    if let Some(theme) = service.ui_editor_theme_mut() {
                        self.global_ui_theme = Some(theme.clone());
                    }
                }
            }
        }

        // Sauvegarder la préférence thème en storage (persistant) après un changement dans Paramètres.
        if self.theme_preference_changed {
            if let Some(storage) = frame.storage_mut() {
                storage.set_string(
                    STORAGE_KEY_DARK_MODE,
                    if self.dark_mode { "true".to_string() } else { "false".to_string() },
                );
            }
            self.theme_preference_changed = false;
        }
    }
}

impl MiyukiniCentralApp {
    /// Affiche l’écran de chargement : LoadingBlock (titre, barre 75 % avec bordure, phrase) centré en hauteur et largeur.
    fn resolve_chrome(&self, pixel_theme: &PixelChromeTheme) -> ResolvedChromeColors {
        let t = self.global_ui_theme.as_ref();
        ResolvedChromeColors {
            barre_exe_bg: t.and_then(|x| x.barre_exe_bg).unwrap_or_else(|| pixel_theme.barre_exe_bg()),
            tab_bar_bg: t.and_then(|x| x.tab_bar_bg).unwrap_or_else(|| pixel_theme.tab_bar_bg()),
            sidebar_bg: t.and_then(|x| x.sidebar_bg).unwrap_or_else(|| pixel_theme.sidebar_bg()),
            body_bg: t.and_then(|x| x.body_bg).unwrap_or_else(|| pixel_theme.body_bg()),
            tab_inactive_bg: t.and_then(|x| x.tab_inactive_bg).unwrap_or_else(|| pixel_theme.tab_inactive_bg()),
            tab_inactive_hover_bg: t.and_then(|x| x.tab_inactive_hover_bg).unwrap_or_else(|| pixel_theme.tab_inactive_hover_bg()),
            tab_active_text: t.and_then(|x| x.tab_active_text).unwrap_or_else(|| pixel_theme.tab_active_text()),
            tab_inactive_text: t.and_then(|x| x.tab_inactive_text).unwrap_or_else(|| pixel_theme.tab_inactive_text()),
            tab_separator: t.and_then(|x| x.tab_separator).unwrap_or_else(|| pixel_theme.tab_separator()),
            close_btn_fg: t.and_then(|x| x.close_btn_fg).unwrap_or({
                if pixel_theme.dark_mode {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_DARK
                } else {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_LIGHT
                }
            }),
            close_btn_hover_fg: t.and_then(|x| x.close_btn_hover_fg).unwrap_or({
                if pixel_theme.dark_mode {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_HOVER_DARK
                } else {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_HOVER_LIGHT
                }
            }),
            close_btn_hover_bg: t.and_then(|x| x.close_btn_hover_bg).unwrap_or({
                if pixel_theme.dark_mode {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_BG_HOVER_DARK
                } else {
                    crate::pixel_theme::chrome_colors::CLOSE_BUTTON_BG_HOVER_LIGHT
                }
            }),
        }
    }

    fn ui_loading_screen(&mut self, ctx: &egui::Context) {
        let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
        let r = self.resolve_chrome(&pixel_theme);
        let body_bg = r.body_bg;
        let time_sec = ctx.input(|i| i.time);
        let elapsed = self.loading_state.effective_elapsed_sec(time_sec);
        let progress = self.loading_state.progress_at(elapsed);
        let phrase = LOADING_PHRASES[self.loading_state.current_phrase_index];

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(body_bg))
            .show(ctx, |ui| {
                let full_rect = ui.available_rect_before_wrap();

                // LoadingBlock : centré en hauteur et largeur (titre + barre 75 % + phrase)
                const BLOCK_PADDING: f32 = 32.0;
                const TITLE_HEIGHT: f32 = 60.0;
                const BAR_HEIGHT: f32 = 24.0;
                const BAR_BORDER: f32 = 2.0;
                const PHRASE_HEIGHT: f32 = 40.0;
                const GAP: f32 = 20.0;
                let bar_width = (full_rect.width() * 0.75).min(480.0).max(280.0);
                let block_width = bar_width + BLOCK_PADDING * 2.0;
                let block_height = TITLE_HEIGHT + GAP + (BAR_HEIGHT + BAR_BORDER * 2.0) + GAP + PHRASE_HEIGHT + BLOCK_PADDING * 2.0;
                let block_size = egui::vec2(block_width, block_height);
                let block_min = full_rect.center() - block_size / 2.0;
                let loading_block_rect = egui::Rect::from_min_size(block_min, block_size);

                #[allow(deprecated)]
                ui.allocate_ui_at_rect(loading_block_rect, |ui| {
                    ui.set_min_size(block_size);
                    ui.add_space(BLOCK_PADDING);

                    // Block titre (centré dans le LoadingBlock)
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.heading(
                            egui::RichText::new("MIYUKINI COG System")
                                .size(24.0)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Chargement").size(16.0));
                    });
                    ui.add_space(GAP);

                    // Barre de chargement avec bordure (75 % largeur), centrée
                    let bar_outer_width = bar_width;
                    let bar_outer_height = BAR_HEIGHT + BAR_BORDER * 2.0;
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let (bar_rect, _) = ui.allocate_exact_size(
                            egui::vec2(bar_outer_width, bar_outer_height),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_stroke(
                            bar_rect,
                            4.0,
                            egui::Stroke::new(BAR_BORDER, r.tab_active_text.gamma_multiply(0.6)),
                            egui::StrokeKind::Outside,
                        );
                        let bar_inner = bar_rect.shrink(BAR_BORDER);
                        ui.painter().rect_filled(bar_inner, 2.0, r.tab_bar_bg);
                        let fill_width = bar_inner.width() * progress;
                        if fill_width > 0.01 {
                            let fill_rect = egui::Rect::from_min_size(bar_inner.min, egui::vec2(fill_width, bar_inner.height()));
                            ui.painter().rect_filled(fill_rect, 2.0, r.tab_active_text);
                        }
                    });

                    ui.add_space(GAP);
                    // Phrase de chargement en dessous
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("\"{phrase}\"")).size(12.0).weak(),
                        );
                    });
                });

                const SKIP_MARGIN: f32 = 16.0;
                const SKIP_WIDTH: f32 = 72.0;
                const SKIP_HEIGHT: f32 = 28.0;
                #[allow(deprecated)]
                let response = ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(
                        egui::pos2(
                            full_rect.max.x - SKIP_WIDTH - SKIP_MARGIN,
                            full_rect.max.y - SKIP_HEIGHT - SKIP_MARGIN,
                        ),
                        egui::vec2(SKIP_WIDTH, SKIP_HEIGHT),
                    ),
                    |ui| (ui.button("Skip").clicked(), ()),
                );
                if response.inner.0 {
                    self.loading_done = true;
                }
            });
    }

    /// Affiche le header : deux « molécule » distinctes (deux rects fixes) pour éviter que
    /// le padding ou le layout ne mélange titre et onglets. Ligne 1 = barre exe (titre + boutons), ligne 2 = onglets.
    /// Palette mode clair : barre exe #d9b99b, header/onglets #fff0db.
    fn ui_header(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        const LINE1_HEIGHT: f32 = 36.0;
        const LINE2_HEIGHT: f32 = 41.0; // hauteur onglets (46 - 5 px)
        const GAP_LINE1_LINE2: f32 = 5.0;

        let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
        let r = self.resolve_chrome(&pixel_theme);
        let margin = egui::Margin { left: 4i8, right: 4i8, top: 4i8, bottom: 0i8 };

        egui::TopBottomPanel::top("header")
            .resizable(false)
            .show_separator_line(false)
            .frame(
                egui::Frame::NONE
                    .inner_margin(margin)
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT)),
            )
            .show(ctx, |ui| {
                let available = ui.available_rect_before_wrap();
                let w = available.width();
                // Fond en deux zones : barre exe (ligne 1 + gap), puis barre onglets (ligne 2).
                let rect_barre_exe = egui::Rect::from_min_size(
                    available.min,
                    egui::vec2(w, LINE1_HEIGHT + GAP_LINE1_LINE2),
                );
                let rect_tabs = egui::Rect::from_min_size(
                    egui::pos2(available.min.x, available.min.y + LINE1_HEIGHT + GAP_LINE1_LINE2),
                    egui::vec2(w, LINE2_HEIGHT),
                );
                ui.painter().rect_filled(rect_barre_exe, 0.0, r.barre_exe_bg);
                ui.painter().rect_filled(rect_tabs, 0.0, r.tab_bar_bg);

                // Molécule 1 : rect fixe pour la ligne titre + connexion/profil/config (pas d'onglets)
                let rect_line1 = egui::Rect::from_min_size(available.min, egui::vec2(w, LINE1_HEIGHT));
                // Molécule 2 : rect fixe pour la ligne onglets uniquement (HUB + services)
                let rect_line2 = egui::Rect::from_min_size(
                    egui::pos2(available.min.x, available.min.y + LINE1_HEIGHT + GAP_LINE1_LINE2),
                    egui::vec2(w, LINE2_HEIGHT),
                );

                #[allow(deprecated)]
                let _ = ui.allocate_ui_at_rect(rect_line1, |ui| {
                    ui.set_clip_rect(rect_line1);
                    ui.horizontal(|ui| {
                        self.ui_header_logo(ui);
                        ui.add_space(16.0);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            self.ui_header_profile(ui, frame);
                        });
                    });
                });

                #[allow(deprecated)]
                let _ = ui.allocate_ui_at_rect(rect_line2, |ui| {
                    ui.set_clip_rect(rect_line2);
                    self.ui_header_tabs(ui, ctx);
                });
            });
    }

    /// Affiche le bloc titre + version à gauche (MIYUKINI COG V0.0.1).
    fn ui_header_logo(&mut self, ui: &mut egui::Ui) {
        const TITLE_SIZE: f32 = 14.0;
        const META_SIZE: f32 = 11.0;
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("MIYUKINI COG").size(TITLE_SIZE).strong(),
            );
            ui.add_space(4.0);
            ui.label(egui::RichText::new("V0.0.1").size(META_SIZE).weak());
        });
    }

    /// Affiche à droite : bloc connexion (Connexion/Déconnexion, Profil) + bloc configuration (bouton roue).
    fn ui_header_profile(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        const ACTION_SIZE: f32 = 12.0;
        ui.horizontal(|ui| {
            // Points MIYU en haut à droite (layout right_to_left donc en premier = plus à droite)
            let miyu_points = self.loyalty_state.miyu_points.get();
            ui.label(
                egui::RichText::new(format!("💎 {} MIYU", miyu_points))
                    .size(13.0)
                    .strong()
                    .color(egui::Color32::from_rgb(255, 215, 0)), // Couleur or
            );
            ui.add_space(16.0);

            // Bloc connexion : Connexion / Déconnexion
            if self.connected {
                if ui
                    .button(egui::RichText::new("Déconnexion").size(ACTION_SIZE))
                    .clicked()
                {
                    self.connected = false;
                    self.current_profile = None;
                }
            } else if ui
                .button(egui::RichText::new("Connexion").size(ACTION_SIZE))
                .clicked()
            {
                self.auth_window_open = true;
                self.auth_error = None;
            }
            ui.add_space(8.0);
            // Profil : icône Lucide (handle garde la texture) ou fallback texte
            let icon_tint = ui.ctx().style().visuals.text_color();
            let profile_clicked = if let Some(ref handle) = self.header_icon_user {
                let img = egui::Image::new(egui::load::SizedTexture::new(handle.id(), egui::vec2(20.0, 20.0))).tint(icon_tint);
                ui.add(egui::Button::image(img)).on_hover_text("Profil").clicked()
            } else {
                ui.button(egui::RichText::new("Profil").size(12.0)).on_hover_text("Profil").clicked()
            };
            if profile_clicked {
                self.profile_overlay_open = true;
                self.profile_edit_pending_sync = true;
                self.profile_save_error = None;
            }
            ui.add_space(12.0);
            // Configuration : icône Lucide ou fallback texte
            let settings_clicked = if let Some(ref handle) = self.header_icon_settings {
                let img = egui::Image::new(egui::load::SizedTexture::new(handle.id(), egui::vec2(20.0, 20.0))).tint(icon_tint);
                ui.add(egui::Button::image(img)).on_hover_text("Configuration").clicked()
            } else {
                ui.button(egui::RichText::new("Config").size(12.0)).on_hover_text("Configuration").clicked()
            };
            if settings_clicked {
                self.settings_overlay_open = true;
            }
            ui.add_space(12.0);
            // MDC — Miyukini Dev Center : accès direct aux tests (depuis n'importe quel onglet)
            if ui
                .button(egui::RichText::new("🧪 MDC").size(ACTION_SIZE))
                .on_hover_text("Miyukini Dev Center — Tests des services")
                .clicked()
            {
                self.open_service(ServiceId::DevCenter, frame.storage());
            }
        });
    }

    /// Affiche les onglets dans la ligne 2 du header.
    /// Largeur adaptée à l'espace (style Chrome) : HUB fixe (1/3 de l'ancienne largeur), autres onglets partagent l'espace restant.
    fn ui_header_tabs(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        use chrome_dimensions::TAB_GAP;

        let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
        let r = self.resolve_chrome(&pixel_theme);

        let tabs_data: Vec<(usize, TabInfo)> = self
            .header_state
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| (i, tab.clone()))
            .collect();

        let tab_bar_rect = ui.available_rect_before_wrap();
        let total_width = tab_bar_rect.width();
        let tab_bar_top = tab_bar_rect.min.y;
        let tab_bar_bottom = tab_bar_rect.max.y;

        // HUB : largeur fixe (ancienne 232 px / 3), seul onglet qui ne change pas de taille.
        const HUB_TAB_WIDTH: f32 = 232.0 / 3.0;
        const TAB_MIN_WIDTH: f32 = 72.0;
        const TAB_MAX_WIDTH: f32 = 200.0;

        let n_tabs = tabs_data.len();
        let n_service_tabs = tabs_data.iter().filter(|(_, t)| t.tab_type != TabType::Hub).count();
        let gaps_width = if n_tabs > 1 { (n_tabs - 1) as f32 * TAB_GAP } else { 0.0 };
        let has_hub = tabs_data.first().is_some_and(|(_, t)| t.tab_type == TabType::Hub);
        let hub_width = if has_hub { HUB_TAB_WIDTH } else { 0.0 };
        let remaining = total_width - hub_width - gaps_width;
        let service_tab_width = if n_service_tabs > 0 {
            (remaining / n_service_tabs as f32).clamp(TAB_MIN_WIDTH, TAB_MAX_WIDTH)
        } else {
            TAB_MIN_WIDTH
        };

        ui.horizontal(|ui| {
            let mut current_x = tab_bar_rect.min.x;

            for (index, tab_info) in tabs_data {
                let is_active = index == self.header_state.active_tab_index;
                let tab_width = if tab_info.tab_type == TabType::Hub {
                    HUB_TAB_WIDTH
                } else {
                    service_tab_width
                };

                let tab_rect = egui::Rect::from_min_max(
                    egui::pos2(current_x, tab_bar_top),
                    egui::pos2(current_x + tab_width, tab_bar_bottom),
                );

                let result = ui.push_id(egui::Id::new(("header_tab", index, tab_info.title.as_str())), |ui| {
                    let _response = ui.allocate_space(tab_rect.size());
                    Self::ui_tab_static(ui, &tab_info, is_active, ctx, tab_rect, &r)
                }).inner;

                if result.activate {
                    self.header_state.active_tab_index = index;
                }
                if result.close {
                    self.close_tab(index);
                    break;
                }

                current_x += tab_width + TAB_GAP;
            }
        });
    }

    /// Affiche un onglet statique (sans mutation d'état). Couleurs résolues depuis UI Editor ou Chrome.
    fn ui_tab_static(
        ui: &mut egui::Ui,
        tab_info: &TabInfo,
        is_active: bool,
        _ctx: &egui::Context,
        tab_rect: egui::Rect,
        r: &ResolvedChromeColors,
    ) -> TabClickResult {
        use chrome_dimensions::{CLOSE_BUTTON_SIZE, TAB_PADDING_X, TAB_PADDING_Y, CLOSE_BUTTON_PADDING, CLOSE_CROSS_SIZE, CLOSE_CROSS_STROKE, TAB_TEXT_SIZE};
        use chrome_shapes::{create_active_tab_shape, create_inactive_tab_shape};

        let painter = ui.painter();
        let closable = matches!(tab_info.tab_type, TabType::Service | TabType::Profile);
        let close_button_size = CLOSE_BUTTON_SIZE;
        let content_rect = egui::Rect::from_min_max(
            egui::pos2(tab_rect.min.x + TAB_PADDING_X, tab_rect.min.y + TAB_PADDING_Y),
            egui::pos2(tab_rect.max.x - TAB_PADDING_X, tab_rect.max.y - TAB_PADDING_Y),
        );
        let close_button_rect = egui::Rect::from_min_max(
            egui::pos2(
                content_rect.max.x - close_button_size - CLOSE_BUTTON_PADDING,
                content_rect.center().y - close_button_size / 2.0,
            ),
            egui::pos2(
                content_rect.max.x - CLOSE_BUTTON_PADDING,
                content_rect.center().y + close_button_size / 2.0,
            ),
        );
        let clickable_rect = egui::Rect::from_min_max(
            tab_rect.min,
            egui::pos2(
                if closable { close_button_rect.min.x - 4.0 } else { tab_rect.max.x },
                tab_rect.max.y,
            ),
        );

        // Interaction d’abord pour connaître le survol (2e niveau de gris).
        let response = ui.interact(clickable_rect, ui.id().with("tab"), egui::Sense::click());
        let hovered = response.hovered();

        // Couleurs : actif = body ; inactif = gris ; inactif + survol = 2e niveau gris.
        // Exception : onglet HUB actif = couleur de la sidebar (sidebar_bg).
        let bg_color = if is_active {
            if tab_info.tab_type == TabType::Hub {
                r.sidebar_bg
            } else {
                r.body_bg
            }
        } else if hovered {
            r.tab_inactive_hover_bg
        } else if tab_info.tab_type == TabType::Hub {
            r.sidebar_bg
        } else {
            r.tab_inactive_bg
        };
        let text_color = if is_active {
            r.tab_active_text
        } else {
            r.tab_inactive_text
        };

        // Dessiner la forme de l'onglet. Actif et inactif : pleine hauteur, collés au body.
        if is_active {
            let shapes = create_active_tab_shape(tab_rect, bg_color);
            for shape in shapes {
                painter.add(shape);
            }
        } else {
            let shape = create_inactive_tab_shape(tab_rect, bg_color);
            painter.add(shape);
        }

        let mut result = TabClickResult::default();
        if response.clicked() {
            result.activate = true;
        }

        let close_hovered = if closable {
            let close_id = ui.id().with("close").with(tab_info.title.clone());
            let close_response = ui.interact(
                close_button_rect.expand(4.0),
                close_id,
                egui::Sense::click(),
            );
            if close_response.clicked() {
                result.close = true;
            }
            close_response.hovered()
        } else {
            false
        };

        if closable {
            let close_color = if close_hovered {
                r.close_btn_hover_fg
            } else {
                r.close_btn_fg
            };

            if close_hovered {
                painter.circle_filled(close_button_rect.center(), close_button_size / 2.0, r.close_btn_hover_bg);
            }

            let cross_size = CLOSE_CROSS_SIZE;
            let cross_stroke = CLOSE_CROSS_STROKE;
            let center = close_button_rect.center();
            let half_size = cross_size / 2.0;
            painter.line_segment(
                [
                    egui::pos2(center.x - half_size, center.y - half_size),
                    egui::pos2(center.x + half_size, center.y + half_size),
                ],
                egui::Stroke::new(cross_stroke, close_color),
            );
            painter.line_segment(
                [
                    egui::pos2(center.x - half_size, center.y + half_size),
                    egui::pos2(center.x + half_size, center.y - half_size),
                ],
                egui::Stroke::new(cross_stroke, close_color),
            );
        }

        // Texte de l'onglet (zone jusqu'au bouton fermer ou bord droite).
        let text_rect = egui::Rect::from_min_max(
            content_rect.min,
            egui::pos2(
                if closable { close_button_rect.min.x - 8.0 } else { content_rect.max.x },
                content_rect.max.y,
            ),
        );
        
        // Utiliser painter.text pour dessiner le texte avec troncature automatique
        let font_id = egui::FontId::proportional(TAB_TEXT_SIZE);
        let available_width = text_rect.width();
        
        // Créer le texte avec troncature si nécessaire
        let text_to_display = {
            let full_text = &tab_info.title;
            // Estimer la largeur approximative du texte
            let estimated_width = full_text.len() as f32 * (TAB_TEXT_SIZE * 0.6); // Approximation
            if estimated_width > available_width {
                // Tronquer avec ellipses
                let max_chars = ((available_width / (TAB_TEXT_SIZE * 0.6)) as usize).saturating_sub(3);
                if max_chars > 0 && full_text.len() > max_chars {
                    format!("{}...", &full_text[..max_chars])
                } else {
                    full_text.clone()
                }
            } else {
                full_text.clone()
            }
        };
        
        // Dessiner le texte avec painter.text qui gère automatiquement le layout
        painter.text(
            egui::pos2(
                text_rect.min.x,
                text_rect.center().y,
            ),
            egui::Align2::LEFT_CENTER,
            text_to_display,
            font_id,
            text_color,
        );

        result
    }

    /// Ferme un onglet.
    fn close_tab(&mut self, index: usize) {
        if index >= self.header_state.tabs.len() {
            return;
        }

        // Ne pas fermer l'onglet HUB.
        if let Some(tab) = self.header_state.tabs.get(index) {
            if tab.tab_type == TabType::Hub {
                return;
            }
        }
        if self.header_state.tabs.len() <= 1 {
            return;
        }

        // Si l'onglet fermé était actif, activer un autre onglet.
        let was_active = index == self.header_state.active_tab_index;
        
        // Supprimer le service associé si c'est un onglet service.
        if let Some(TabInfo { service_id: Some(service_id), .. }) = self.header_state.tabs.get(index) {
            self.open_services.retain(|s| s.id() != *service_id);
        }

        // Supprimer l'onglet.
        self.header_state.tabs.remove(index);

        // Réactiver un onglet si nécessaire.
        if was_active {
            let new_index = if index > 0 { index - 1 } else { 0 };
            self.header_state.active_tab_index = new_index.min(self.header_state.tabs.len() - 1);
        } else if index < self.header_state.active_tab_index {
            self.header_state.active_tab_index -= 1;
        }
    }

    /// Ouvre un service (crée un nouvel onglet).
    fn open_service(&mut self, service_id: ServiceId, storage: Option<&dyn eframe::Storage>) {
        // Vérifier si le service est déjà ouvert.
        if let Some(index) = self.header_state.tabs.iter().position(|t| {
            if let Some(id) = t.service_id {
                id == service_id
            } else {
                false
            }
        }) {
            self.header_state.active_tab_index = index;
            return;
        }

        // Créer le service. JayKoa, JayFestival, JayXpose = prod ; JayRDV, JayKonta, JayFaim = mock. UI Editor : thème depuis storage.
        let profile_id = self.current_profile.as_ref().map(|p| p.id.clone());
        let service: Box<dyn ServiceUi> = match service_id {
            ServiceId::Calculator => Box::new(CalculatorService::default()),
            ServiceId::Game => Box::new(GameService::default()),
            ServiceId::MiyuClicker => Box::new(MiyuClickerService::default()),
            ServiceId::LordOfTheCastle => Box::new(LordOfTheCastleService::default()),
            ServiceId::TextEditor => Box::new(TextEditorService::default()),
            ServiceId::Notes => Box::new(NotesService::default()),
            ServiceId::UiLibrary => Box::new(UiLibraryService::default()),
            ServiceId::EguiEditor => Box::new(EguiEditorService::from_storage(storage)),
            ServiceId::JayRDV => Box::new(MockJayService::new(service_id, "JayRDV")),
            ServiceId::JayFestival => Box::new(JayFestivalService::default()),
            ServiceId::JayKoa => Box::new(JayKoaService::with_profile(profile_id)),
            ServiceId::JayKonta => Box::new(MockJayService::new(service_id, "JayKonta")),
            ServiceId::JayXpose => Box::new(JayXposeService::default()),
            ServiceId::JayFaim => Box::new(MockJayService::new(service_id, "JayFaim")),
            ServiceId::DevCenter => Box::new(DevCenterService::default()),
            ServiceId::MiyukiniLoyalty => Box::new(crate::services::LoyaltyService::default()),
        };

        // Ajouter l'onglet.
        self.header_state.tabs.push(TabInfo {
            service_id: Some(service_id),
            title: service.title().to_string(),
            tab_type: TabType::Service,
        });
        self.open_services.push(service);
        self.header_state.active_tab_index = self.header_state.tabs.len() - 1;
    }

    /// Ouvre l'onglet profil (réservé pour navigation par onglet).
    #[allow(dead_code)]
    fn open_profile_tab(&mut self) {
        // Vérifier si l'onglet profil existe déjà.
        if let Some(index) = self.header_state.tabs.iter().position(|t| t.tab_type == TabType::Profile) {
            self.header_state.active_tab_index = index;
            return;
        }

        // Créer l'onglet profil.
        self.header_state.tabs.push(TabInfo {
            service_id: None,
            title: "Profile".to_string(),
            tab_type: TabType::Profile,
        });
        self.header_state.active_tab_index = self.header_state.tabs.len() - 1;
    }

    /// Fenêtre overlay Profil (ouverte depuis le header).
    fn ui_profile_overlay(&mut self, ctx: &egui::Context) {
        let mut open = self.profile_overlay_open;
        egui::Window::new("Profil utilisateur")
            .open(&mut open)
            .default_size(egui::vec2(420.0, 560.0))
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui_profile_content(ui);
            });
        self.profile_overlay_open = open;
        if !open {
            self.profile_edit_pending_sync = true;
        }
    }

    /// Fenêtre overlay Paramètres (ouverte depuis le header).
    fn ui_settings_overlay(&mut self, ctx: &egui::Context) {
        let mut open = self.settings_overlay_open;
        egui::Window::new("Paramètres")
            .open(&mut open)
            .default_size(egui::vec2(400.0, 300.0))
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui_settings_content(ui);
            });
        self.settings_overlay_open = open;
    }

    /// Fenêtre login (Connexion / Créer un compte) ouverte par le bouton Connexion du header.
    fn ui_auth_overlay(&mut self, ctx: &egui::Context) {
        if !self.auth_window_open {
            return;
        }
        let mut open = self.auth_window_open;
        let title = match self.auth_mode {
            AuthMode::Connexion => "Connexion — Miyukini COG",
            AuthMode::CreerCompte => "Créer un compte — Miyukini COG",
        };
        let (width, height) = match self.auth_mode {
            AuthMode::Connexion => (380.0, 320.0),
            AuthMode::CreerCompte => (400.0, 420.0),
        };
        egui::Window::new(title)
            .open(&mut open)
            .default_size(egui::vec2(width, height))
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                match self.auth_mode {
                    AuthMode::Connexion => self.ui_auth_screen_connexion(ui),
                    AuthMode::CreerCompte => self.ui_auth_screen_enregistrement(ui),
                }
            });
        // Ne pas rouvrir si on a fermé depuis l'intérieur (connexion ou création de compte réussie).
        if self.auth_window_open {
            self.auth_window_open = open;
        }
    }

    /// Écran Connexion : email, mot de passe, bouton Se connecter, lien vers Créer un compte.
    fn ui_auth_screen_connexion(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.heading("Connexion");
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Connectez-vous avec votre compte Miyukini COG.").small().weak());
        ui.add_space(16.0);

        ui.label("Email (login)");
        ui.add(
            egui::TextEdit::singleline(&mut self.auth_email)
                .hint_text("vous@exemple.fr")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(10.0);

        ui.label("Mot de passe");
        ui.add(
            egui::TextEdit::singleline(&mut self.auth_password)
                .password(true)
                .hint_text("••••••••")
                .desired_width(f32::INFINITY),
        );

        if let Some(ref err) = self.auth_error {
            ui.add_space(8.0);
            ui.colored_label(egui::Color32::RED, err);
        }

        ui.add_space(16.0);
        if ui.add_sized(egui::vec2(120.0, 28.0), egui::Button::new("Se connecter")).clicked() {
            self.auth_error = None;
            match self.auth_db.sign_in(&self.auth_email, &self.auth_password) {
                Ok(Some(profile)) => {
                    self.connected = true;
                    self.current_profile = Some(profile);
                    self.auth_window_open = false;
                    self.auth_email.clear();
                    self.auth_password.clear();
                    self.auth_error = None;
                }
                Ok(None) => {
                    self.auth_error = Some("Email ou mot de passe incorrect.".to_string());
                }
                Err(e) => {
                    self.auth_error = Some(e.to_string());
                }
            }
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Pas encore de compte ?").small().weak());
            if ui.link("Créer un compte").clicked() {
                self.auth_mode = AuthMode::CreerCompte;
                self.auth_error = None;
            }
        });
    }

    /// Écran Enregistrement : email, mot de passe, répétition, règles, bouton Créer le compte, lien vers Connexion.
    fn ui_auth_screen_enregistrement(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.heading("Créer un compte");
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Enregistrez-vous pour utiliser Miyukini COG.").small().weak());
        ui.add_space(16.0);

        ui.label("Email (login)");
        ui.add(
            egui::TextEdit::singleline(&mut self.auth_email)
                .hint_text("vous@exemple.fr")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(10.0);

        ui.label("Mot de passe");
        ui.add(
            egui::TextEdit::singleline(&mut self.auth_password)
                .password(true)
                .hint_text("••••••••")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(4.0);
        ui.label(egui::RichText::new(password_rules_hint()).small().weak());
        ui.add_space(6.0);

        ui.label("Répéter le mot de passe");
        ui.add(
            egui::TextEdit::singleline(&mut self.auth_password_repeat)
                .password(true)
                .hint_text("••••••••")
                .desired_width(f32::INFINITY),
        );

        if let Some(ref err) = self.auth_error {
            ui.add_space(8.0);
            ui.colored_label(egui::Color32::RED, err);
        }

        ui.add_space(16.0);
        if ui.add_sized(egui::vec2(140.0, 28.0), egui::Button::new("Créer le compte")).clicked() {
            self.auth_error = None;
            if self.auth_password == self.auth_password_repeat {
                match validate_password(&self.auth_password) {
                    Ok(()) => {
                        match self.auth_db.sign_up(&self.auth_email, &self.auth_password) {
                            Ok(profile) => {
                                self.connected = true;
                                self.current_profile = Some(profile);
                                self.auth_window_open = false;
                                self.auth_email.clear();
                                self.auth_password.clear();
                                self.auth_password_repeat.clear();
                                self.auth_error = None;
                            }
                            Err(e) => {
                                self.auth_error = Some(e.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        self.auth_error = Some(e.to_string());
                    }
                }
            } else {
                self.auth_error = Some("Les deux mots de passe ne correspondent pas.".to_string());
            }
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Déjà un compte ?").small().weak());
            if ui.link("Se connecter").clicked() {
                self.auth_mode = AuthMode::Connexion;
                self.auth_error = None;
            }
        });
    }

    /// Affiche le contenu principal selon l'onglet actif.
    /// Chaque onglet est dessiné dans un scope d'ID unique pour éviter les conflits (ex. calculatrice).
    /// Affiche le contenu selon l'onglet actif (HUB, service, etc.).
    /// @id: miyukini_central_app_ui_content
    /// @do: dispatch_content_by_active_tab
    /// @role: interface
    /// @human: Route vers ui_hub_content, ui_service_content ou autre selon onglet.
    fn ui_content(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let active_index = self.header_state.active_tab_index;
        let (tab_type, service_id, title) = if let Some(tab) = self.header_state.tabs.get(active_index) {
            (tab.tab_type, tab.service_id, tab.title.clone())
        } else {
            return;
        };
        let tab_scope_id = egui::Id::new(("tab_content", active_index, title));
        ui.push_id(tab_scope_id, |ui| {
            match tab_type {
                TabType::Menu => self.ui_hub_content(ui, frame),
                TabType::Hub => self.ui_hub_content(ui, frame),
                TabType::Service => {
                    if let Some(sid) = service_id {
                        self.ui_service_content(ui, sid);
                    }
                }
                TabType::Profile => self.ui_profile_content(ui),
            }
        });
    }

    /// Affiche le contenu de l'onglet HUB (sidebar + body détail du service sélectionné).
    /// Utilise le rect complet du CentralPanel pour que sidebar et body remplissent toute la hauteur.
    #[allow(deprecated)] // allocate_ui_at_rect requis pour placer sidebar/body à des rects précis
    fn ui_hub_content(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let pixel_theme = PixelChromeTheme::new(ui.ctx().style().visuals.dark_mode);
        let r = self.resolve_chrome(&pixel_theme);
        let catalog = mock_catalog();

        const SIDEBAR_WIDTH: f32 = 240.0;
        const GAP: f32 = 8.0;

        // Rect complet attribué au CentralPanel (toute la hauteur sous le header).
        let full_rect = ui.available_rect_before_wrap();
        let full_height = full_rect.height();

        // Sidebar : gauche, toute la hauteur.
        let sidebar_rect = egui::Rect::from_min_size(
            full_rect.min,
            egui::vec2(SIDEBAR_WIDTH, full_height),
        );
        ui.allocate_ui_at_rect(sidebar_rect, |ui| {
            ui.set_min_width(SIDEBAR_WIDTH - 2.0);
            ui.set_max_width(SIDEBAR_WIDTH - 2.0);
            ui.set_min_height(full_height);
            self.ui_hub_sidebar(ui, &catalog, &r, frame);
        });

        // Séparateur + marge.
        let body_min_x = sidebar_rect.max.x + GAP;
        let body_rect = egui::Rect::from_min_max(
            egui::pos2(body_min_x, full_rect.min.y),
            egui::pos2(full_rect.max.x, full_rect.max.y),
        );
        let body_width = body_rect.width();
        let body_height = body_rect.height();

        // Body : reste de la largeur, toute la hauteur.
        ui.allocate_ui_at_rect(body_rect, |ui| {
            ui.set_min_height(body_height);
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(body_height)
                .show(ui, |ui| {
                    ui.set_min_width(body_width);
                    self.ui_hub_body_detail(ui, &catalog, &r, frame);
                });
        });
    }

    /// Sidebar HUB : barre de recherche, filtrage, Services favoris, liste des Services (ordre alphabétique).
    /// Le fond occupe toute la hauteur allouée (jusqu'en bas de la fenêtre).
    #[allow(deprecated)] // allocate_ui_at_rect requis pour placer le contenu avec marge
    fn ui_hub_sidebar(
        &mut self,
        ui: &mut egui::Ui,
        catalog: &[ServiceMeta],
        r: &ResolvedChromeColors,
        frame: &mut eframe::Frame,
    ) {
        // Dessiner le fond sur toute la hauteur allouée (sidebar jusqu'en bas de la fenêtre).
        let sidebar_fill_rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(
            sidebar_fill_rect,
            egui::CornerRadius::same(6),
            r.sidebar_bg,
        );
        // Contenu dans une zone avec marge, défilable si nécessaire (fond déjà peint sur toute la hauteur).
        let inner_margin = 8.0;
        let content_rect = sidebar_fill_rect.shrink(inner_margin);
        ui.allocate_ui_at_rect(content_rect, |ui| {
            ui.set_max_width(ui.available_width());
            egui::ScrollArea::vertical()
                .max_height(content_rect.height())
                .auto_shrink([false, false])
                .show(ui, |ui| {
            ui.label(egui::RichText::new("Recherche").size(12.0).strong());
            ui.add_space(4.0);
            ui.add(
                egui::TextEdit::singleline(&mut self.hub_sidebar.search_query)
                    .hint_text("Nom ou description…")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(12.0);
            ui.label(egui::RichText::new("Filtrage").size(12.0).strong());
            ui.add_space(4.0);
            let current_label = match self.hub_sidebar.category_filter {
                None => "Toutes".to_string(),
                Some(cat) => format!("{} {}", cat.icon(), cat.label()),
            };
            egui::ComboBox::from_id_salt("hub_category_filter")
                .selected_text(current_label)
                .width(ui.available_width().min(200.0))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.hub_sidebar.category_filter, None, "Toutes");
                    for &cat in &[
                        CategoryId::Utilitaires,
                        CategoryId::Loisirs,
                        CategoryId::Productivite,
                        CategoryId::Design,
                    ] {
                        ui.selectable_value(
                            &mut self.hub_sidebar.category_filter,
                            Some(cat),
                            format!("{} {}", cat.icon(), cat.label()),
                        );
                    }
                });
            ui.add_space(12.0);
            ui.label(egui::RichText::new("Services favoris").size(12.0).strong());
            ui.add_space(4.0);
            let mut fav_to_remove = None;
            for (i, &sid) in self.favorites.iter().enumerate() {
                let meta = meta_for_service(catalog, sid);
                let name = meta.map_or_else(|| sid.as_str(), |m| m.name.as_str());
                ui.horizontal(|ui| {
                    if ui.selectable_label(
                        self.hub_sidebar.selected_service_id == Some(sid),
                        format!("{} {}", sid.icon(), name),
                    ).clicked() {
                        self.hub_sidebar.selected_service_id = Some(sid);
                    }
                    if ui.small_button("−").on_hover_text("Retirer des favoris").clicked() {
                        fav_to_remove = Some(i);
                    }
                });
            }
            if let Some(i) = fav_to_remove {
                self.favorites.remove(i);
            }
            if self.favorites.is_empty() {
                ui.weak("Aucun favori. Cliquez ★ dans la liste pour ajouter.");
            }
            ui.add_space(12.0);
            ui.label(egui::RichText::new("Services (A–Z)").size(12.0).strong());
            ui.add_space(4.0);
            let services_sorted = self.hub_filtered_services_sorted_by_name(catalog);
            for &sid in &services_sorted {
                let meta = meta_for_service(catalog, sid);
                let name = meta.map_or_else(|| sid.as_str(), |m| m.name.as_str());
                let is_fav = self.favorites.contains(&sid);
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            self.hub_sidebar.selected_service_id == Some(sid),
                            format!("{} {}", sid.icon(), name),
                        )
                        .clicked()
                    {
                        self.hub_sidebar.selected_service_id = Some(sid);
                    }
                    let (label, add_fav) = if is_fav { ("★", false) } else { ("☆", true) };
                    if ui.small_button(label).on_hover_text(if is_fav { "Retirer des favoris" } else { "Ajouter aux favoris" }).clicked() {
                        if add_fav {
                            if !self.favorites.contains(&sid) {
                                self.favorites.push(sid);
                            }
                        } else {
                            self.favorites.retain(|&id| id != sid);
                        }
                    }
                    if self.hub_sidebar.selected_service_id == Some(sid)
                        && ui.small_button("Lancer").on_hover_text("Lancer le service").clicked() {
                            self.open_service(sid, frame.storage());
                        }
                });
            }
                });
        });
    }

    /// Body HUB : logo + titre + description complète + screenshots (mock) + bouton Lancer.
    fn ui_hub_body_detail(
        &mut self,
        ui: &mut egui::Ui,
        catalog: &[ServiceMeta],
        r: &ResolvedChromeColors,
        frame: &mut eframe::Frame,
    ) {
        let Some(service_id) = self.hub_sidebar.selected_service_id else {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(
                    egui::RichText::new("Sélectionnez un service dans la liste.")
                        .size(16.0)
                        .color(ui.ctx().style().visuals.weak_text_color()),
                );
            });
            return;
        };
        let Some(meta) = meta_for_service(catalog, service_id) else {
            ui.label("Service inconnu.");
            return;
        };
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(service_id.icon()).size(48.0));
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.heading(egui::RichText::new(&meta.name).size(20.0).strong());
                ui.add_space(8.0);
                if ui.button(egui::RichText::new("Lancer").size(14.0)).clicked() {
                    self.open_service(service_id, frame.storage());
                }
            });
        });
        ui.add_space(16.0);
        ui.label(egui::RichText::new("Description").size(12.0).strong());
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(&meta.description)
                .size(13.0)
                .color(ui.ctx().style().visuals.text_color()),
        );
        ui.add_space(16.0);
        ui.label(egui::RichText::new("Aperçu (mock)").size(12.0).strong());
        ui.add_space(8.0);
        let mock_w = 320.0_f32;
        let mock_h = 180.0_f32;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(mock_w, mock_h), egui::Sense::hover());
        let radius = egui::CornerRadius::same(4);
        ui.painter().rect_filled(rect, radius, r.tab_inactive_bg);
        ui.painter().rect_stroke(
            rect,
            radius,
            egui::Stroke::new(1.0, r.tab_separator),
            egui::StrokeKind::Outside,
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Screenshot 1 (mock)",
            egui::FontId::proportional(12.0),
            ui.ctx().style().visuals.weak_text_color(),
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            for i in 0..3 {
                let (rect_mock, _) = ui.allocate_exact_size(egui::vec2(100.0, 56.0), egui::Sense::hover());
                let rad = egui::CornerRadius::same(4);
                ui.painter().rect_filled(rect_mock, rad, r.tab_inactive_bg);
                ui.painter().rect_stroke(
                    rect_mock,
                    rad,
                    egui::Stroke::new(1.0, r.tab_separator),
                    egui::StrokeKind::Outside,
                );
                ui.painter().text(
                    rect_mock.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("Mock {}", i + 1),
                    egui::FontId::proportional(10.0),
                    ui.ctx().style().visuals.weak_text_color(),
                );
                ui.add_space(8.0);
            }
        });
    }

    /// Retourne la liste des services filtrés par recherche et catégorie, triés par nom (A–Z).
    fn hub_filtered_services_sorted_by_name(&self, catalog: &[ServiceMeta]) -> Vec<ServiceId> {
        let q = self.hub_sidebar.search_query.trim().to_lowercase();
        let cat_filter = self.hub_sidebar.category_filter;
        let mut ids: Vec<ServiceId> = self
            .catalog
            .iter()
            .copied()
            .filter(|&id| {
                let meta = match meta_for_service(catalog, id) {
                    Some(m) => m,
                    None => return true,
                };
                if let Some(cat) = cat_filter {
                    if meta.category != cat {
                        return false;
                    }
                }
                if q.is_empty() {
                    return true;
                }
                meta.name.to_lowercase().contains(&q)
                    || meta.description.to_lowercase().contains(&q)
            })
            .collect();
        ids.sort_by(|a, b| {
            let na = meta_for_service(catalog, *a).map_or("", |m| m.name.as_str());
            let nb = meta_for_service(catalog, *b).map_or("", |m| m.name.as_str());
            na.cmp(nb)
        });
        ids
    }

    /// Affiche le contenu d'un service.
    fn ui_service_content(&mut self, ui: &mut egui::Ui, service_id: ServiceId) {
        if let Some(service) = self.open_services.iter_mut().find(|s| s.id() == service_id) {
            service.set_lotc_save_load(
                Some(self.auth_db.clone()),
                self.current_profile.as_ref().map(|p| p.id.as_str()),
            );
            service.show(ui);
        }
    }

    /// Contexte d'utilisation courant selon l'onglet actif (Central ou Service).
    fn current_usage_context(&self) -> Option<UsageContext> {
        let tab = self.header_state.tabs.get(self.header_state.active_tab_index)?;
        Some(match tab.tab_type {
            TabType::Menu | TabType::Hub | TabType::Profile => UsageContext::Central,
            TabType::Service => {
                tab.service_id
                    .map_or(UsageContext::Central, |id| UsageContext::Service(id.storage_key().to_string()))
            }
        })
    }

    /// Affiche le contenu de l'onglet profil (utilisé dans l'overlay et éventuellement en onglet).
    fn ui_profile_content(&mut self, ui: &mut egui::Ui) {
        if self.profile_edit_pending_sync
            || (self.current_profile.is_some()
                && self.profile_edit_email.is_empty()
                && !self.current_profile.as_ref().unwrap().email.is_empty())
        {
            if let Some(ref p) = self.current_profile {
                self.profile_edit_pseudonyme = p.pseudonyme.clone().unwrap_or_default();
                self.profile_edit_nom = p.nom.clone().unwrap_or_default();
                self.profile_edit_prenom = p.prenom.clone().unwrap_or_default();
                self.profile_edit_date_naissance = p.date_naissance.clone().unwrap_or_default();
                self.profile_edit_telephone = p.telephone.clone().unwrap_or_default();
                self.profile_edit_email = p.email.clone();
                self.profile_edit_numero_voie = p.numero_voie.clone().unwrap_or_default();
                self.profile_edit_rue = p.rue.clone().unwrap_or_default();
                self.profile_edit_code_postal = p.code_postal.clone().unwrap_or_default();
                self.profile_edit_ville = p.ville.clone().unwrap_or_default();
            }
            self.profile_edit_pending_sync = false;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Profil utilisateur");
            ui.add_space(8.0);

            if self.current_profile.is_none() {
                ui.label(egui::RichText::new("Connectez-vous pour modifier votre profil.").weak());
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label("Statistiques d'utilisation");
                ui.label("Temps passé sur Central et chaque service, première utilisation, pics dans la journée.");
                if ui.button("Voir le détail — Statistiques d'utilisation").clicked() {
                    self.usage_stats_overlay_open = true;
                }
                return;
            }

            ui.label(egui::RichText::new("Informations modifiables (sauvegardées en base).").small().weak());
            ui.add_space(10.0);

            ui.label("Pseudonyme");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_pseudonyme).hint_text("Pseudo").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Nom");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_nom).hint_text("Nom de famille").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Prénom");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_prenom).hint_text("Prénom").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Date de naissance");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_date_naissance).hint_text("JJ/MM/AAAA").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Téléphone");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_telephone).hint_text("Numéro").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Adresse mail");
            ui.add(egui::TextEdit::singleline(&mut self.profile_edit_email).hint_text("email@exemple.fr").desired_width(f32::INFINITY));
            ui.add_space(6.0);

            ui.label("Adresse");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.profile_edit_numero_voie).hint_text("N°").desired_width(50.0));
                ui.add(egui::TextEdit::singleline(&mut self.profile_edit_rue).hint_text("Rue").desired_width(f32::INFINITY));
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.profile_edit_code_postal).hint_text("Code postal").desired_width(100.0));
                ui.add(egui::TextEdit::singleline(&mut self.profile_edit_ville).hint_text("Ville").desired_width(f32::INFINITY));
            });

            if let Some(ref err) = self.profile_save_error {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::RED, err);
            }

            ui.add_space(12.0);
            if ui.button("Sauvegarder le profil").clicked() {
                self.profile_save_error = None;
                if let Some(ref mut p) = self.current_profile {
                    if self.profile_edit_email.trim().is_empty() {
                        self.profile_save_error = Some("L'adresse mail est obligatoire.".to_string());
                    } else {
                        p.email = self.profile_edit_email.trim().to_string();
                        p.pseudonyme = Some(self.profile_edit_pseudonyme.trim().to_string()).filter(|s| !s.is_empty());
                        p.nom = Some(self.profile_edit_nom.trim().to_string()).filter(|s| !s.is_empty());
                        p.prenom = Some(self.profile_edit_prenom.trim().to_string()).filter(|s| !s.is_empty());
                        p.date_naissance = Some(self.profile_edit_date_naissance.trim().to_string()).filter(|s| !s.is_empty());
                        p.telephone = Some(self.profile_edit_telephone.trim().to_string()).filter(|s| !s.is_empty());
                        p.numero_voie = Some(self.profile_edit_numero_voie.trim().to_string()).filter(|s| !s.is_empty());
                        p.rue = Some(self.profile_edit_rue.trim().to_string()).filter(|s| !s.is_empty());
                        p.code_postal = Some(self.profile_edit_code_postal.trim().to_string()).filter(|s| !s.is_empty());
                        p.ville = Some(self.profile_edit_ville.trim().to_string()).filter(|s| !s.is_empty());
                        match self.auth_db.update_profile(p) {
                            Ok(()) => {}
                            Err(e) => self.profile_save_error = Some(e.to_string()),
                        }
                    }
                }
            }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label("Statistiques d'utilisation");
            ui.label("Temps passé sur Central et chaque service, première utilisation, pics dans la journée.");
            if ui.button("Voir le détail — Statistiques d'utilisation").clicked() {
                self.usage_stats_overlay_open = true;
            }
        });
    }

    /// Fenêtre overlay « Statistiques d'utilisation » (accessible depuis le profil).
    fn ui_usage_stats_overlay(&mut self, ctx: &egui::Context) {
        let mut open = self.usage_stats_overlay_open;
        let time_sec = ctx.input(|i| i.time);
        let stats = self.usage_store.compute_stats(time_sec);
        egui::Window::new("Statistiques d'utilisation")
            .open(&mut open)
            .default_size(egui::vec2(440.0, 520.0))
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui_usage_stats_content(ui, &stats, time_sec);
            });
        self.usage_stats_overlay_open = open;
    }

    /// Contenu de l'écran Statistiques d'utilisation (résumé + détail).
    fn ui_usage_stats_content(&mut self, ui: &mut egui::Ui, stats: &crate::usage_tracking::UsageStats, now_sec: f64) {
        use crate::usage_tracking::format_duration_sec;
        use egui::ScrollArea;

        ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Résumé");
            ui.add_space(4.0);
            if let Some(ref s) = stats.first_use_relative_human(now_sec) {
                ui.label(format!("Première utilisation : {s}"));
            } else {
                ui.label("Aucune session enregistrée.");
            }
            ui.label(format!("Temps total (Central + services) : {}", stats.total_duration_human()));
            ui.label(format!("Temps sur le HUB Central : {}", stats.central_duration_human()));
            if let Some(sid) = stats.most_used_service {
                ui.label(format!("Service le plus utilisé : {} {}", sid.icon(), sid.as_str()));
            }
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.heading("Répartition");
            if stats.total_sec_all > 0.0 {
                ui.label(format!(
                    "Central : {:.0} %  |  Services : {:.0} %",
                    stats.central_pct,
                    stats.services_pct()
                ));
                if !stats.top3_services.is_empty() {
                    ui.label("Top 3 services :");
                    for (i, (key, sec)) in stats.top3_services.iter().enumerate() {
                        let pct = 100.0 * sec / stats.total_sec_all;
                        let name = crate::catalog::ServiceId::from_storage_key(key).map_or_else(|| key.clone(), |id| format!("{} {}", id.icon(), id.as_str()));
                        ui.label(format!("  {}. {} — {:.0} % ({})", i + 1, name, pct, format_duration_sec(*sec)));
                    }
                }
            }
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.heading("Sessions");
            ui.label(format!("Nombre de sessions : {}", stats.session_count));
            if stats.session_count > 0 {
                ui.label(format!("Durée moyenne par session : {}", format_duration_sec(stats.avg_session_sec)));
                let longest_ctx_label = stats
                    .longest_session_context_key
                    .as_ref()
                    .map(|k| {
                        if k == "central" {
                            "HUB Central".to_string()
                        } else {
                            crate::catalog::ServiceId::from_storage_key(k).map_or_else(|| k.clone(), |id| format!("{} {}", id.icon(), id.as_str()))
                        }
                    })
                    .unwrap_or_else(|| "—".to_string());
                ui.label(format!(
                    "Plus longue session : {} ({})",
                    format_duration_sec(stats.longest_session_sec),
                    longest_ctx_label
                ));
            }
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.heading("Tendances");
            ui.label(format!("Cette semaine (7 j) : {}", format_duration_sec(stats.this_week_sec)));
            if let Some(ref s) = stats.week_comparison_human() {
                ui.label(s);
            }
            ui.label(format!("Ce mois (30 j) : {}", format_duration_sec(stats.this_month_sec)));
            if let Some(ref s) = stats.month_comparison_human() {
                ui.label(s);
            }
            ui.label(format!("Jours actifs (7 j) : {}", stats.active_days_this_week));
            ui.label(format!("Jours actifs (30 j) : {}", stats.active_days_this_month));
            ui.label(format!("Jours actifs (total) : {}", stats.active_days_total));
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.heading("Temps par service");
            for (key, sec) in &stats.total_sec_by_service {
                let name = crate::catalog::ServiceId::from_storage_key(key).map_or_else(|| key.clone(), |id| format!("{} {}", id.icon(), id.as_str()));
                ui.label(format!("{} : {}", name, format_duration_sec(*sec)));
            }
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.heading("Pics d'utilisation dans la journée (UTC)");
            let peaks = stats.peak_hours();
            let top = peaks.iter().take(5);
            for (hour, sec) in top {
                let label = if *sec > 0.0 {
                    format!("{}h–{}h : {}", hour, hour + 1, format_duration_sec(*sec))
                } else {
                    format!("{}h–{}h : —", hour, hour + 1)
                };
                ui.label(label);
            }
        });
    }

    /// Affiche les notifications de fidélisation (achievements, MIYU, items).
    fn ui_loyalty_notifications(&mut self, ctx: &egui::Context) {
        let time_sec = ctx.input(|i| i.time);

        // Filtrer les notifications expirées
        self.loyalty_notifications.retain(|notif| {
            time_sec - notif.created_at < notif.duration as f64
        });

        // Afficher les notifications en haut à droite, sous le header
        let screen_rect = ctx.content_rect();
        let notification_width = 300.0;
        let notification_spacing = 10.0;
        let mut y_offset = 90.0; // Décalage depuis le haut (sous le header)

        for notif in &self.loyalty_notifications {
            let remaining = notif.duration - (time_sec - notif.created_at) as f32;
            let alpha = (remaining / notif.duration).clamp(0.0, 1.0);

            let window_pos = egui::pos2(
                screen_rect.max.x - notification_width - 20.0,
                screen_rect.min.y + y_offset,
            );

            egui::Area::new(egui::Id::new(format!("notification_{}", notif.created_at)))
                .fixed_pos(window_pos)
                .show(ctx, |ui| {
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 50, (255.0 * alpha) as u8))
                        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(255, 215, 0, (255.0 * alpha) as u8)))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_max_width(notification_width - 24.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&notif.icon).size(24.0));
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&notif.title).strong().color(egui::Color32::WHITE));
                                    ui.label(egui::RichText::new(&notif.message).color(egui::Color32::LIGHT_GRAY));
                                });
                            });
                        });
                });

            y_offset += 80.0 + notification_spacing;
        }

        // Redemander un repaint si des notifications sont actives
        if !self.loyalty_notifications.is_empty() {
            ctx.request_repaint();
        }
    }

    /// Affiche le contenu Paramètres (utilisé dans l'overlay).
    fn ui_settings_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Paramètres");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Thème :");
                let label = if self.dark_mode { "Mode obscur" } else { "Mode clair" };
                if ui.checkbox(&mut self.dark_mode, "Mode obscur").on_hover_text(label).changed() {
                    self.theme_preference_changed = true;
                }
                ui.label(if self.dark_mode { "Obscur" } else { "Clair" });
            });
            ui.label("Le choix est enregistré et conservé au prochain lancement.");
        });
    }
}
