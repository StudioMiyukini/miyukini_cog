//! Tests de parcours visiteur (VIS).
//!
//! Vérifie que les écrans VIS sont accessibles et que les séquences de navigation
//! typiques (Dashboard → Agenda) fonctionnent.

use jayfestival::app_state::AppState;
use jayfestival::screens::ScreenId;

#[test]
fn parcours_vis_dashboard_to_agenda() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::VisDashboard);
    state.navigate(ScreenId::VisAgenda);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::VisAgenda);
    assert!(state.current_screen().is_vis());
}

#[test]
fn parcours_vis_agenda_back_to_dashboard() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::VisAgenda);
    state.navigate(ScreenId::VisDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::VisDashboard);
}

#[test]
fn parcours_vis_sequence_dashboard_agenda_dashboard() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::VisDashboard);
    assert_eq!(state.current_screen(), ScreenId::VisDashboard);
    state.navigate(ScreenId::VisAgenda);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::VisAgenda);
    state.navigate(ScreenId::VisDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::VisDashboard);
}

#[test]
fn parcours_vis_all_screens_reachable() {
    let mut state = AppState::new();
    for &screen in ScreenId::all_vis() {
        state.navigate(screen);
        state.apply_nav_request();
        assert_eq!(state.current_screen(), screen, "screen {:?}", screen);
    }
}

#[test]
fn parcours_vis_reserved_screen_classified() {
    let mut state = AppState::new();
    state.navigate(ScreenId::VisReserved(6));
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::VisReserved(6));
    assert!(state.current_screen().is_vis());
}
