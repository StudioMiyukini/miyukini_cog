//! Application principale JayXpose — boucle eframe.
//!
//! Utilisée par main.rs ; applique le thème et affiche l'écran courant (router global + app_state).
//! Données : base fille SQLite via KindMother (`JayXposeDb`).
//!
//! @id: jayxpose_app_struct
//! @do: run_eframe_loop_jayxpose
//! @layer: app

use crate::app_state::AppState;
use crate::data::JayXposeDb;
use crate::screens::exp::{self, ExpState};
use crate::screens::pub_::{self, PubState};
use crate::theme::JayXposeTheme;
use eframe::egui;
use eframe::App;
use std::sync::Arc;

/// Application JayXpose (état écrans EXP + PUB ; router global via `AppState`).
///
/// @id: jayxpose_app
/// @do: hold_app_state_db_theme_jayxpose
/// @layer: app
pub struct JayXposeApp {
    /// Base de données fille SQLite (KindMother Daughter).
    db: Arc<JayXposeDb>,
    /// Thème actif (tokens UI).
    theme: JayXposeTheme,
    /// État global : écran courant, navigation, utilisateur.
    app_state: AppState,
    /// État partagé des écrans EXP (espace exposant).
    exp_state: ExpState,
    /// État partagé des écrans PUB (accès public).
    pub_state: PubState,
}

impl JayXposeApp {
    /// Crée l'application avec thème par défaut (mode sombre) et écran `ExpDashboard`.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Crée l'application pour mode embarqué (sans `CreationContext`).
    pub fn new_embedded() -> Self {
        Self::default()
    }
}

impl Default for JayXposeApp {
    fn default() -> Self {
        let db = Arc::new(JayXposeDb::open("jayxpose.db").unwrap_or_else(|e| {
            panic!(
                "JayXpose: impossible d'ouvrir la base SQLite (KindMother fille): {e}"
            )
        }));
        let theme = JayXposeTheme::new(true);
        Self {
            db,
            theme,
            app_state: AppState::new(),
            exp_state: ExpState::default(),
            pub_state: PubState::default(),
        }
    }
}

impl App for JayXposeApp {
    /// @id: jayxpose_app_loop
    /// @do: update_frame_apply_theme_show_ui
    /// @layer: app
    /// Boucle de mise à jour : applique le thème, affiche l'UI selon l'écran courant (router global).
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_in_ui(ctx);
    }
}

impl JayXposeApp {
    /// Affiche l'UI JayXpose dans le contexte donné (thème + écran courant + navigation).
    /// Utilisé en mode embarqué depuis le Central : même rendu que la fenêtre standalone.
    ///
    /// @id: jayxpose_show_in_ui
    /// @do: render_jayxpose_ui_embedded
    /// @layer: app
    pub fn show_in_ui(&mut self, ctx: &egui::Context) {
        self.theme.apply(ctx);

        let current_screen = self.app_state.current_screen();
        let nav_request = self.app_state.nav_request();

        if current_screen.is_exp() {
            exp::exp_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.exp_state,
                &self.db,
            );
        } else if current_screen.is_pub() {
            pub_::pub_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.pub_state,
                &self.db,
            );
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("JayXpose");
                ui.label(format!("Écran : {current_screen:?}"));
            });
        }

        self.app_state.apply_nav_request();
    }
}
