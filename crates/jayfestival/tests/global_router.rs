//! Tests globaux : cohérence du router, classification des écrans, navigation.
//!
//! Vérifie que tous les ScreenId sont classés exactement dans un parcours (UNC, ORG, EXP, VIS)
//! et que l'état initial et la navigation sont cohérents.

use jayfestival::app_state::AppState;
use jayfestival::screens::ScreenId;

/// Tous les écrans non réservés (variants fixes).
fn all_fixed_screens() -> Vec<ScreenId> {
    let mut out = Vec::new();
    out.extend(ScreenId::all_unc().iter().copied());
    out.extend(ScreenId::all_org().iter().copied());
    out.extend(ScreenId::all_exp().iter().copied());
    out.extend(ScreenId::all_vis().iter().copied());
    out
}

#[test]
fn global_router_every_screen_classified() {
    for screen in all_fixed_screens() {
        let unc = screen.is_unc();
        let org = screen.is_org();
        let exp = screen.is_exp();
        let vis = screen.is_vis();
        let count = [unc, org, exp, vis].iter().filter(|&&b| b).count();
        assert_eq!(
            count, 1,
            "{:?} must be exactly one of UNC, ORG, EXP, VIS (unc={} org={} exp={} vis={})",
            screen, unc, org, exp, vis
        );
    }
}

#[test]
fn global_router_reserved_org_exp_vis_classified() {
    assert!(ScreenId::OrgReserved(16).is_org());
    assert!(ScreenId::OrgReserved(25).is_org());
    assert!(ScreenId::ExpReserved(7).is_exp());
    assert!(ScreenId::VisReserved(6).is_vis());
    assert!(!ScreenId::OrgReserved(16).is_unc());
    assert!(!ScreenId::ExpReserved(7).is_vis());
}

#[test]
fn global_router_initial_screen_is_unc_landing() {
    let state = AppState::new();
    assert_eq!(state.current_screen(), ScreenId::UncLanding);
    assert!(state.current_screen().is_unc());
}

#[test]
fn global_router_navigate_then_apply_changes_screen() {
    let state = AppState::new();
    state.navigate(ScreenId::UncConnexion);
    let mut state = state;
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::UncConnexion);

    state.navigate(ScreenId::OrgDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDashboard);
    assert!(state.current_screen().is_org());
}

#[test]
fn global_router_apply_without_request_idempotent() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgListeEditions);
    let before = state.current_screen();
    state.apply_nav_request();
    assert_eq!(state.current_screen(), before);
}

#[test]
fn global_router_unc_count() {
    assert_eq!(ScreenId::all_unc().len(), 12);
}

#[test]
fn global_router_org_count() {
    assert_eq!(ScreenId::all_org().len(), 12);
}

#[test]
fn global_router_exp_count() {
    assert_eq!(ScreenId::all_exp().len(), 3);
}

#[test]
fn global_router_vis_count() {
    assert_eq!(ScreenId::all_vis().len(), 2);
}
