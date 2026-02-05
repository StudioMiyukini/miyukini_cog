//! Tests de parcours exposant (EXP).
//!
//! Vérifie que les écrans EXP sont accessibles et que les séquences de navigation
//! typiques (Dashboard → Candidatures → Participations) fonctionnent.

use jayfestival::app_state::AppState;
use jayfestival::screens::ScreenId;

#[test]
fn parcours_exp_dashboard_to_candidatures() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::ExpDashboard);
    state.navigate(ScreenId::ExpCandidatures);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpCandidatures);
    assert!(state.current_screen().is_exp());
}

#[test]
fn parcours_exp_dashboard_to_participations() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::ExpDashboard);
    state.navigate(ScreenId::ExpParticipations);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpParticipations);
}

#[test]
fn parcours_exp_candidatures_to_participations() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::ExpCandidatures);
    state.navigate(ScreenId::ExpParticipations);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpParticipations);
}

#[test]
fn parcours_exp_participations_back_to_dashboard() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::ExpParticipations);
    state.navigate(ScreenId::ExpDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpDashboard);
}

#[test]
fn parcours_exp_sequence_full() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::ExpDashboard);
    assert_eq!(state.current_screen(), ScreenId::ExpDashboard);
    state.navigate(ScreenId::ExpCandidatures);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpCandidatures);
    state.navigate(ScreenId::ExpParticipations);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpParticipations);
    state.navigate(ScreenId::ExpDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpDashboard);
}

#[test]
fn parcours_exp_all_screens_reachable() {
    let mut state = AppState::new();
    for &screen in ScreenId::all_exp() {
        state.navigate(screen);
        state.apply_nav_request();
        assert_eq!(state.current_screen(), screen, "screen {:?}", screen);
    }
}

#[test]
fn parcours_exp_reserved_screen_classified() {
    let mut state = AppState::new();
    state.navigate(ScreenId::ExpReserved(7));
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::ExpReserved(7));
    assert!(state.current_screen().is_exp());
}
