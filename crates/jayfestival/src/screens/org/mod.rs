//! Router ORG — écrans Organisateur (ORG-E04 à ORG-E25).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//!
//! @id: router_org
//! @do: dispatch_org_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran ORG correspondant à l'identifiant ; délègue à chaque module e04..e15 et e16_e25_rest.
//!
//! @id: org_navigate
//! @do: change_current_screen_from_org
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell).

mod e04_dashboard;
mod e05_liste_editions;
mod e06_creation_edition;
mod e07_dashboard_edition;
mod e08_liste_exposants;
mod e09_candidatures;
mod e10_fiche_exposant;
mod e11_plan_salle;
mod e12_programme;
mod e13_budget;
mod e14_devis_factures;
mod e15_documents;
mod e16_e25_rest;

use std::cell::RefCell;

use crate::screens::ScreenId;
use crate::supabase::{Edition, EditionExposant, Exposant};
use crate::theme::JayFestivalTheme;
use eframe::egui;

/// État mutable partagé pour les écrans ORG (éditions, exposants, formulaires).
pub struct OrgState {
    /// Liste des éditions chargées.
    pub editions: Vec<Edition>,
    /// Liaisons édition–exposant (candidatures, participations).
    pub edition_exposants: Vec<EditionExposant>,
    /// Liste des exposants chargés.
    pub exposants: Vec<Exposant>,
    /// Index de l'édition sélectionnée dans le header.
    pub selected_edition_idx: usize,
    /// Index d'édition à appliquer après navigation (ex. depuis liste éditions vers dashboard).
    pub pending_edition_idx: Option<usize>,
    /// ID de l'exposant sélectionné pour la fiche exposant.
    pub selected_exposant_id: Option<String>,
    /// Nom saisi (création édition).
    pub form_creation_name: String,
    /// Date de début saisie (création édition).
    pub form_creation_start_date: String,
    /// Date de fin saisie (création édition).
    pub form_creation_end_date: String,
    /// Lieu saisi (création édition).
    pub form_creation_location: String,
    /// Libellé revenus (écran budget).
    pub budget_revenus_label: String,
    /// Libellé dépenses (écran budget).
    pub budget_depenses_label: String,
}

impl Default for OrgState {
    fn default() -> Self {
        Self {
            editions: Vec::new(),
            edition_exposants: Vec::new(),
            exposants: Vec::new(),
            selected_edition_idx: 0,
            pending_edition_idx: None,
            selected_exposant_id: None,
            form_creation_name: String::new(),
            form_creation_start_date: String::new(),
            form_creation_end_date: String::new(),
            form_creation_location: String::new(),
            budget_revenus_label: String::new(),
            budget_depenses_label: String::new(),
        }
    }
}

impl OrgState {
    /// Options pour le sélecteur d'édition (noms).
    pub fn edition_options(&self) -> Vec<String> {
        self.editions
            .iter()
            .map(|e| e.name.as_deref().unwrap_or("Sans nom").to_string())
            .collect()
    }

    /// Nom de l'édition courante.
    pub fn current_edition_name(&self) -> String {
        self.editions
            .get(self.selected_edition_idx)
            .and_then(|e| e.name.clone())
            .unwrap_or_else(|| "Édition".to_string())
    }

    /// Exposant sélectionné pour la fiche.
    pub fn selected_exposant(&self) -> Option<&Exposant> {
        self.selected_exposant_id
            .as_ref()
            .and_then(|id| self.exposants.iter().find(|e| e.id.as_deref() == Some(id.as_str())))
    }

    /// EditionExposant pour l'exposant sélectionné et l'édition courante.
    pub fn selected_edition_exposant(&self) -> Option<&EditionExposant> {
        let edition_id = self.editions.get(self.selected_edition_idx).and_then(|e| e.id.as_ref())?;
        let exposant_id = self.selected_exposant_id.as_ref()?;
        self.edition_exposants
            .iter()
            .find(|ee| ee.edition_id.as_deref() == Some(edition_id.as_str()) && ee.exposant_id.as_deref() == Some(exposant_id.as_str()))
    }
}

/// Affiche l'écran ORG correspondant à `screen`. Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn org_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut OrgState,
) {
    if let Some(idx) = state.pending_edition_idx.take() {
        state.selected_edition_idx = idx.min(state.editions.len().saturating_sub(1));
    }

    let edition_options = state.edition_options();
    let current_edition_name = state.current_edition_name();
    let sel_exp = state.selected_exposant().cloned();
    let sel_ed_exp = state.selected_edition_exposant().cloned();
    let nav = nav_request;

    match screen {
        ScreenId::OrgDashboard => e04_dashboard::org_e04_show(ctx, theme, nav, &edition_options, &mut state.selected_edition_idx),
        ScreenId::OrgListeEditions => e05_liste_editions::org_e05_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &state.editions,
            &mut state.pending_edition_idx,
        ),
        ScreenId::OrgCreationEdition => e06_creation_edition::org_e06_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &mut state.form_creation_name,
            &mut state.form_creation_start_date,
            &mut state.form_creation_end_date,
            &mut state.form_creation_location,
        ),
        ScreenId::OrgDashboardEdition => e07_dashboard_edition::org_e07_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &current_edition_name,
        ),
        ScreenId::OrgListeExposants => e08_liste_exposants::org_e08_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &state.edition_exposants,
            &state.exposants,
            &mut state.selected_exposant_id,
        ),
        ScreenId::OrgCandidatures => e09_candidatures::org_e09_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &state.edition_exposants,
            &state.exposants,
            &mut state.selected_exposant_id,
        ),
        ScreenId::OrgFicheExposant => e10_fiche_exposant::org_e10_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            sel_exp.as_ref(),
            sel_ed_exp.as_ref(),
        ),
        ScreenId::OrgPlanSalle => e11_plan_salle::org_e11_show(ctx, theme, nav, &edition_options, &mut state.selected_edition_idx),
        ScreenId::OrgProgramme => e12_programme::org_e12_show(ctx, theme, nav, &edition_options, &mut state.selected_edition_idx),
        ScreenId::OrgBudget => e13_budget::org_e13_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            &mut state.budget_revenus_label,
            &mut state.budget_depenses_label,
        ),
        ScreenId::OrgDevisFactures => e14_devis_factures::org_e14_show(ctx, theme, nav, &edition_options, &mut state.selected_edition_idx),
        ScreenId::OrgDocuments => e15_documents::org_e15_show(ctx, theme, nav, &edition_options, &mut state.selected_edition_idx),
        ScreenId::OrgReserved(n) => e16_e25_rest::org_e16_e25_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            n,
        ),
        _ => {}
    }
}
