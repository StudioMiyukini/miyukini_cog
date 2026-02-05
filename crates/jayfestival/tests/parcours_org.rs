//! Tests de parcours organisateur (ORG).
//!
//! Vérifie que les écrans ORG sont accessibles et que les séquences de navigation
//! typiques (Dashboard → Liste éditions → Création, Dashboard édition → Candidatures, etc.)
//! fonctionnent.

use jayfestival::app_state::AppState;
use jayfestival::screens::ScreenId;

#[test]
fn parcours_org_dashboard_to_liste_editions() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgListeEditions);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgListeEditions);
    assert!(state.current_screen().is_org());
}

#[test]
fn parcours_org_dashboard_to_creation_edition() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgCreationEdition);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgCreationEdition);
}

#[test]
fn parcours_org_dashboard_to_dashboard_edition() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgDashboardEdition);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDashboardEdition);
}

#[test]
fn parcours_org_dashboard_to_liste_exposants() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgListeExposants);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgListeExposants);
}

#[test]
fn parcours_org_dashboard_to_candidatures() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgCandidatures);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgCandidatures);
}

#[test]
fn parcours_org_dashboard_to_fiche_exposant() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgFicheExposant);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgFicheExposant);
}

#[test]
fn parcours_org_dashboard_to_plan_salle() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgPlanSalle);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgPlanSalle);
}

#[test]
fn parcours_org_dashboard_to_programme() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgProgramme);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgProgramme);
}

#[test]
fn parcours_org_dashboard_to_budget() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgBudget);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgBudget);
}

#[test]
fn parcours_org_dashboard_to_devis_factures() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgDevisFactures);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDevisFactures);
}

#[test]
fn parcours_org_dashboard_to_documents() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboard);
    state.navigate(ScreenId::OrgDocuments);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDocuments);
}

#[test]
fn parcours_org_sequence_liste_creation_dashboard() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgListeEditions);
    state.navigate(ScreenId::OrgCreationEdition);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgCreationEdition);
    state.navigate(ScreenId::OrgDashboardEdition);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDashboardEdition);
    state.navigate(ScreenId::OrgDashboard);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgDashboard);
}

#[test]
fn parcours_org_sequence_dashboard_edition_candidatures_fiche() {
    let mut state = AppState::new();
    state.set_current_screen(ScreenId::OrgDashboardEdition);
    state.navigate(ScreenId::OrgCandidatures);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgCandidatures);
    state.navigate(ScreenId::OrgFicheExposant);
    state.apply_nav_request();
    assert_eq!(state.current_screen(), ScreenId::OrgFicheExposant);
}

#[test]
fn parcours_org_all_screens_reachable() {
    let mut state = AppState::new();
    for &screen in ScreenId::all_org() {
        state.navigate(screen);
        state.apply_nav_request();
        assert_eq!(state.current_screen(), screen, "screen {:?}", screen);
    }
}
