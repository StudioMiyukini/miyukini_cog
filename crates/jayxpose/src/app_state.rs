//! État global de l'application JayXpose et router (écran courant, navigation, utilisateur).
//!
//! @id: jayxpose_app_state
//! @do: manage_global_state_and_navigation_jayxpose
//! @layer: app
//!
//! @id: jayxpose_app_state_current_screen
//! @do: retourne l'écran actuellement affiché (router global)
//! @layer: app
//!
//! @id: jayxpose_app_state_navigate
//! @do: enregistre une demande de navigation vers un écran (appliquée en fin de frame)
//! @layer: app
//!
//! @id: jayxpose_app_state_user
//! @do: retourne la session utilisateur courante si connecté (auth)
//! @layer: app
//!
//! Utilisé par app.rs : les écrans demandent une navigation via `navigate()` ;
//! la boucle update applique la navigation et affiche l'écran correspondant.

use crate::auth::AuthSession;
use crate::screens::ScreenId;
use std::cell::RefCell;

/// État global de l'application : écran courant, demande de navigation, utilisateur.
pub struct AppState {
    /// Écran actuellement affiché.
    current_screen: ScreenId,
    /// Demande de navigation en attente (écrite par les écrans, lue en fin de frame).
    nav_request: RefCell<Option<ScreenId>>,
    /// Session utilisateur si connecté.
    user: Option<AuthSession>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: ScreenId::ExpDashboard,
            nav_request: RefCell::new(None),
            user: None,
        }
    }
}

impl AppState {
    /// Crée un état initial (ExpDashboard, pas d'utilisateur).
    pub fn new() -> Self {
        Self::default()
    }

    /// Retourne l'écran actuellement affiché (router global).
    ///
    /// @id: jayxpose_app_state_current_screen
    /// @do: retourne l'écran actuellement affiché (router global)
    /// @layer: app
    pub fn current_screen(&self) -> ScreenId {
        self.current_screen
    }

    /// Enregistre une demande de navigation vers un écran (appliquée en fin de frame).
    ///
    /// @id: jayxpose_app_state_navigate
    /// @do: enregistre une demande de navigation vers un écran (appliquée en fin de frame)
    /// @layer: app
    pub fn navigate(&self, screen: ScreenId) {
        *self.nav_request.borrow_mut() = Some(screen);
    }

    /// Consomme la demande de navigation en attente et met à jour l'écran courant si présente.
    /// À appeler une fois par frame (dans update).
    pub fn apply_nav_request(&mut self) {
        if let Some(s) = self.nav_request.replace(None) {
            self.current_screen = s;
        }
    }

    /// Retourne la session utilisateur courante si connecté.
    ///
    /// @id: jayxpose_app_state_user
    /// @do: retourne la session utilisateur courante si connecté (auth)
    /// @layer: app
    pub fn user(&self) -> Option<&AuthSession> {
        self.user.as_ref()
    }

    /// Définit la session utilisateur (après sign_in ou sign_up ; None après sign_out).
    pub fn set_user(&mut self, session: Option<AuthSession>) {
        self.user = session;
    }

    /// Accès mutable à l'écran courant (pour appliquer la navigation).
    pub fn set_current_screen(&mut self, screen: ScreenId) {
        self.current_screen = screen;
    }

    /// Référence vers la cellule de demande de navigation (pour les écrans qui appellent navigate).
    pub fn nav_request(&self) -> &RefCell<Option<ScreenId>> {
        &self.nav_request
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;
    use crate::screens::ScreenId;

    #[test]
    fn app_state_default_dashboard() {
        let state = AppState::new();
        assert_eq!(state.current_screen(), ScreenId::ExpDashboard);
        assert!(state.user().is_none());
    }

    #[test]
    fn app_state_navigate_apply() {
        let state = AppState::new();
        state.navigate(ScreenId::ExpCatalogueListe);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::ExpCatalogueListe);
    }

    #[test]
    fn app_state_set_current_screen() {
        let mut state = AppState::new();
        state.set_current_screen(ScreenId::PubAnnuaire);
        assert_eq!(state.current_screen(), ScreenId::PubAnnuaire);
    }

    #[test]
    fn app_state_nav_request_empty_after_apply() {
        let state = AppState::new();
        state.navigate(ScreenId::ExpDocuments);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::ExpDocuments);
        // Second apply should not change anything (request consumed).
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::ExpDocuments);
    }

    #[test]
    fn app_state_multiple_navigate_last_wins() {
        let state = AppState::new();
        state.navigate(ScreenId::ExpCatalogueListe);
        state.navigate(ScreenId::PubRecherche);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::PubRecherche);
    }

    #[test]
    fn app_state_set_user_then_user() {
        use crate::auth::AuthSession;
        let mut state = AppState::new();
        assert!(state.user().is_none());
        let session = AuthSession {
            user_id: "exp-1".to_string(),
            email: Some("contact@entreprise.fr".to_string()),
            access_token: "tok-local".to_string(),
            profile: None,
        };
        state.set_user(Some(session.clone()));
        assert!(state.user().is_some());
        assert_eq!(state.user().unwrap().user_id, "exp-1");
        assert_eq!(
            state.user().unwrap().email.as_deref(),
            Some("contact@entreprise.fr")
        );
        state.set_user(None);
        assert!(state.user().is_none());
    }

    #[test]
    fn app_state_navigate_to_all_exp_screens() {
        for &screen in ScreenId::all_exp() {
            let state = AppState::new();
            state.navigate(screen);
            let mut state = state;
            state.apply_nav_request();
            assert_eq!(state.current_screen(), screen, "navigate to {screen:?}");
        }
    }

    #[test]
    fn app_state_navigate_to_all_pub_screens() {
        for &screen in ScreenId::all_pub() {
            let state = AppState::new();
            state.navigate(screen);
            let mut state = state;
            state.apply_nav_request();
            assert_eq!(state.current_screen(), screen, "navigate to {screen:?}");
        }
    }

    #[test]
    fn app_state_nav_request_ref() {
        let state = AppState::new();
        let nr = state.nav_request();
        assert!(nr.borrow().is_none());
        state.navigate(ScreenId::ExpVitrinePreview);
        assert!(nr.borrow().is_some());
    }
}
