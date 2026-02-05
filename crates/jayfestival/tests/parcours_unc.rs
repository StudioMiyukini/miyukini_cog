//! Tests de parcours utilisateur non connecté (UNC).
//!
//! Vérifie que les écrans UNC sont accessibles et que les séquences de navigation
//! typiques (Landing → Liste événements → Fiche, Landing → Connexion → Inscription, etc.)
//! fonctionnent.

use jayfestival::app_state::AppState;
use jayfestival::screens::ScreenId;

#[test]
fn parcours_unc_landing_to_liste_evenements() {
    let state = AppState::new();
    assert_eq!(state.current_screen(), ScreenId::UncLanding);
    state.navigate(ScreenId::UncListeEvenements);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncListeEvenements);
    assert!(state.current_screen().is_unc());
}

#[test]
fn parcours_unc_landing_to_fiche_evenement() {
    let state = AppState::new();
    state.navigate(ScreenId::UncFicheEvenement);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncFicheEvenement);
}

#[test]
fn parcours_unc_landing_to_liste_organisateurs() {
    let state = AppState::new();
    state.navigate(ScreenId::UncListeOrganisateurs);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncListeOrganisateurs);
}

#[test]
fn parcours_unc_landing_to_fiche_organisateur() {
    let state = AppState::new();
    state.navigate(ScreenId::UncFicheOrganisateur);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncFicheOrganisateur);
}

#[test]
fn parcours_unc_landing_to_liste_exposants() {
    let state = AppState::new();
    state.navigate(ScreenId::UncListeExposants);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncListeExposants);
}

#[test]
fn parcours_unc_landing_to_fiche_exposant() {
    let state = AppState::new();
    state.navigate(ScreenId::UncFicheExposant);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncFicheExposant);
}

#[test]
fn parcours_unc_landing_to_recherche() {
    let state = AppState::new();
    state.navigate(ScreenId::UncRecherche);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncRecherche);
}

#[test]
fn parcours_unc_landing_to_cta_contextuels() {
    let state = AppState::new();
    state.navigate(ScreenId::UncCtaContextuels);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncCtaContextuels);
}

#[test]
fn parcours_unc_landing_to_connexion() {
    let state = AppState::new();
    state.navigate(ScreenId::UncConnexion);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncConnexion);
}

#[test]
fn parcours_unc_landing_to_inscription() {
    let state = AppState::new();
    state.navigate(ScreenId::UncInscription);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncInscription);
}

#[test]
fn parcours_unc_landing_to_mentions() {
    let state = AppState::new();
    state.navigate(ScreenId::UncMentions);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncMentions);
}

#[test]
fn parcours_unc_sequence_landing_liste_fiche_back() {
    let mut state = AppState::new();
    assert_eq!(state.current_screen(), ScreenId::UncLanding);
    state.navigate(ScreenId::UncListeEvenements);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncListeEvenements);
    state.navigate(ScreenId::UncFicheEvenement);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncFicheEvenement);
    state.navigate(ScreenId::UncLanding);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncLanding);
}

#[test]
fn parcours_unc_sequence_connexion_inscription() {
    let mut state = AppState::new();
    state.navigate(ScreenId::UncConnexion);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncConnexion);
    state.navigate(ScreenId::UncInscription);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncInscription);
}

#[test]
fn parcours_unc_all_screens_reachable() {
    let mut state = AppState::new();
    for &screen in ScreenId::all_unc() {
        state.navigate(screen);
        state.apply_nav_request();
        assert_eq!(state.current_screen(), screen, "screen {:?}", screen);
    }
}
