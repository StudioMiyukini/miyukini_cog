//! État global de l'application et router (écran courant, navigation, utilisateur).
//!
//! @id: app_state_current_screen
//! @do: retourne l'écran actuellement affiché (router global)
//! @layer: app
//!
//! @id: app_state_navigate
//! @do: enregistre une demande de navigation vers un écran (appliquée en fin de frame)
//! @layer: app
//!
//! @id: app_state_user
//! @do: retourne la session utilisateur courante si connecté (auth)
//! @layer: app
//!
//! Utilisé par main/app : écrans demandent une navigation via app_state_navigate ;
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
            current_screen: ScreenId::UncLanding,
            nav_request: RefCell::new(None),
            user: None,
        }
    }
}

impl AppState {
    /// Crée un état initial (Landing, pas d'utilisateur).
    #[must_use] 
    pub fn new() -> Self {
        Self::default()
    }

    /// Retourne l'écran actuellement affiché (router global).
    ///
    /// @id: app_state_current_screen
    /// @do: retourne l'écran actuellement affiché (router global)
    /// @layer: app
    pub fn current_screen(&self) -> ScreenId {
        self.current_screen
    }

    /// Enregistre une demande de navigation vers un écran (appliquée en fin de frame).
    ///
    /// @id: app_state_navigate
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
    /// @id: app_state_user
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
    fn app_state_default_landing() {
        let state = AppState::new();
        assert_eq!(state.current_screen(), ScreenId::UncLanding);
        assert!(state.user().is_none());
    }

    #[test]
    fn app_state_navigate_apply() {
        let state = AppState::new();
        state.navigate(ScreenId::UncListeEvenements);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::UncListeEvenements);
    }

    #[test]
    fn app_state_set_current_screen() {
        let mut state = AppState::new();
        state.set_current_screen(ScreenId::OrgDashboard);
        assert_eq!(state.current_screen(), ScreenId::OrgDashboard);
    }

    #[test]
    fn app_state_nav_request_empty_after_apply() {
        let state = AppState::new();
        state.navigate(ScreenId::UncConnexion);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::UncConnexion);
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::UncConnexion);
    }

    #[test]
    fn app_state_multiple_navigate_last_wins() {
        let state = AppState::new();
        state.navigate(ScreenId::UncListeEvenements);
        state.navigate(ScreenId::UncInscription);
        let mut state = state;
        state.apply_nav_request();
        assert_eq!(state.current_screen(), ScreenId::UncInscription);
    }

    #[test]
    fn app_state_set_user_then_user() {
        use crate::auth::AuthSession;
        let mut state = AppState::new();
        assert!(state.user().is_none());
        let session = AuthSession {
            user_id: "u1".to_string(),
            email: Some("u@test.com".to_string()),
            access_token: "tok".to_string(),
            profile: None,
        };
        state.set_user(Some(session.clone()));
        assert!(state.user().is_some());
        assert_eq!(state.user().unwrap().user_id, "u1");
        assert_eq!(state.user().unwrap().email.as_deref(), Some("u@test.com"));
        state.set_user(None);
        assert!(state.user().is_none());
    }

    #[test]
    fn app_state_navigate_to_all_unc_screens() {
        let unc_screens = [
            ScreenId::UncLanding,
            ScreenId::UncListeEvenements,
            ScreenId::UncFicheEvenement,
            ScreenId::UncListeOrganisateurs,
            ScreenId::UncFicheOrganisateur,
            ScreenId::UncListeExposants,
            ScreenId::UncFicheExposant,
            ScreenId::UncRecherche,
            ScreenId::UncCtaContextuels,
            ScreenId::UncConnexion,
            ScreenId::UncInscription,
            ScreenId::UncMentions,
        ];
        for &screen in &unc_screens {
            let state = AppState::new();
            state.navigate(screen);
            let mut state = state;
            state.apply_nav_request();
            assert_eq!(state.current_screen(), screen, "navigate to {:?}", screen);
        }
    }
}
