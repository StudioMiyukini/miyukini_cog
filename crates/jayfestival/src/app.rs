//! Application principale JayFestival — boucle eframe.
//!
//! Utilisée par main.rs ; applique le thème et affiche l'écran courant (router global + app_state).
//! Données : base fille SQLite via KindMother (JayFestivalDb).

use crate::app_state::AppState;
use crate::auth;
use crate::data::JayFestivalDb;
use crate::screens::exp::{self, ExpState};
use crate::screens::org::{self, OrgState};
use crate::screens::unc::{self, UncState};
use crate::screens::vis::{self, VisState};
use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use eframe::egui;
use eframe::App;
use std::sync::Arc;

/// Application JayFestival (état écrans UNC, ORG, EXP, VIS ; router global via AppState).
pub struct JayFestivalApp {
    /// Base de données fille SQLite (KindMother Daughter).
    db: Arc<JayFestivalDb>,
    /// Thème actif (tokens UI).
    theme: JayFestivalTheme,
    /// État global : écran courant, navigation, utilisateur.
    app_state: AppState,
    /// État partagé des écrans UNC (listes, filtres, formulaire connexion).
    unc_state: UncState,
    /// État partagé des écrans ORG (éditions, exposants, formulaires).
    org_state: OrgState,
    /// État partagé des écrans EXP (exposant : candidatures, participations).
    exp_state: ExpState,
    /// État partagé des écrans VIS (visiteur : agenda, billets, réservations, pass).
    vis_state: VisState,
}

impl JayFestivalApp {
    /// Crée l'application avec thème par défaut (mode sombre) et écran Landing.
    #[must_use] 
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Crée l'application pour mode embarqué (sans CreationContext).
    #[must_use] 
    pub fn new_embedded() -> Self {
        Self::default()
    }
}

impl Default for JayFestivalApp {
    fn default() -> Self {
        let db = Arc::new(
            JayFestivalDb::open("jayfestival.db").unwrap_or_else(|e| {
                panic!("JayFestival: impossible d'ouvrir la base SQLite (KindMother fille): {e}")
            }),
        );
        let theme = JayFestivalTheme::new(true);
        Self {
            db,
            theme,
            app_state: AppState::new(),
            unc_state: UncState::default(),
            org_state: OrgState::default(),
            exp_state: ExpState::default(),
            vis_state: VisState::default(),
        }
    }
}

impl App for JayFestivalApp {
    /// @id: jayfestival_app_loop
    /// @do: update_frame_apply_theme_show_ui
    /// @layer: app
    /// Boucle de mise à jour : applique le thème, affiche l'UI selon l'écran courant (router global).
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_in_ui(ctx);
    }
}

impl JayFestivalApp {
    /// Affiche l'UI JayFestival dans le contexte donné (thème + écran courant + navigation).
    /// Utilisé en mode embarqué depuis le Central : même rendu que la fenêtre standalone.
    ///
    /// @id: jayfestival_show_in_ui
    /// @do: render_jayfestival_ui_embedded
    /// @layer: app
    pub fn show_in_ui(&mut self, ctx: &egui::Context) {
        self.theme.apply(ctx);

        if self.unc_state.connexion_submit_requested {
            self.unc_state.connexion_submit_requested = false;
            match auth::auth_sign_in(
                self.db.as_ref(),
                self.unc_state.connexion_email.trim(),
                &self.unc_state.connexion_password,
            ) {
                Ok(session) => {
                    self.app_state.set_user(Some(session));
                    self.unc_state.connexion_error = None;
                }
                Err(e) => {
                    self.unc_state.connexion_error = Some(e.message);
                }
            }
        }

        let current_screen = self.app_state.current_screen();
        let nav_request = self.app_state.nav_request();

        if matches!(current_screen, ScreenId::UncListeEvenements) && self.unc_state.editions.is_empty() {
            if let Ok(editions) = self.db.editions_list() {
                self.unc_state.editions = editions;
            }
        }
        if matches!(current_screen, ScreenId::UncListeOrganisateurs) && self.unc_state.organisateurs.is_empty() {
            if let Ok(organisateurs) = self.db.organisateurs_list() {
                self.unc_state.organisateurs = organisateurs;
            }
        }
        if matches!(current_screen, ScreenId::UncListeExposants) && self.unc_state.exposants.is_empty() {
            if let Ok(exposants) = self.db.exposants_list(true) {
                self.unc_state.exposants = exposants;
            }
        }

        if current_screen.is_unc() {
            unc::unc_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.unc_state,
            );
        } else if current_screen.is_org() {
            org::org_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.org_state,
            );
        } else if current_screen.is_exp() {
            exp::exp_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.exp_state,
            );
        } else if current_screen.is_vis() {
            vis::vis_show(
                current_screen,
                ctx,
                &self.theme,
                nav_request,
                &mut self.vis_state,
            );
        } else {
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("JayFestival");
                ui.label(format!("Écran : {current_screen:?}"));
            });
        }

        self.app_state.apply_nav_request();
    }
}
