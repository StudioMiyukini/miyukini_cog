//! Module JayXpose — profil exposant, catalogue, vitrine, coffre-fort documentaire.
//!
//! Architecture : sidebar + contenu dynamique par section (XP-E01..XP-E12).
//! Données réelles via JayXposeDb (conns.read().jayxpose).

mod sidebar;
pub mod components;
mod dashboard;
mod entreprise;
mod catalogue;
mod produit_form;
mod vitrine;
mod documents;
mod fiche_publique;

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

use sidebar::JayXposeSidebar;
use dashboard::Dashboard;
use entreprise::Entreprise;
use catalogue::Catalogue;
use produit_form::ProduitForm;
use vitrine::Vitrine;
use documents::Documents;
use fiche_publique::FichePublique;

// ── State ──────────────────────────────────────────────────────────────

/// Section active dans JayXpose (ecrans XP-E01 a XP-E12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JayXposeSection {
    #[default]
    Dashboard,
    Entreprise,
    Catalogue,
    NouveauProduit,
    ModifierProduit,
    Vitrine,
    Documents,
    FichePublique,
}

/// Etat local complet de la vue JayXpose.
#[derive(Debug, Clone, Default)]
pub struct JayXposeState {
    /// Section active.
    pub section: JayXposeSection,
    /// ID de l'exposant courant (premier trouve en DB ou cree).
    pub exposant_id: Option<String>,
    /// ID du produit en cours de modification (None = creation).
    pub editing_product_id: Option<String>,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn JayXposeView() -> Element {
    let _app_state = use_app_state();
    let conns = use_service_connections();
    let mut state = use_signal(JayXposeState::default);

    // Charger l'exposant courant au montage
    let exposant = {
        let db = &conns.read().jayxpose;
        db.exposants_list_annuaire().unwrap_or_default().into_iter().next()
    };

    // Mettre a jour l'ID exposant si disponible et pas encore set
    if state.read().exposant_id.is_none() {
        if let Some(ref e) = exposant {
            if let Some(ref id) = e.id {
                state.write().exposant_id = Some(id.clone());
            }
        }
    }

    let has_exposant = state.read().exposant_id.is_some();

    rsx! {
        div {
            style: "display: flex; height: 100%;",

            // Sidebar
            JayXposeSidebar { state: state }

            // Contenu principal
            main {
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                if !has_exposant {
                    components::EmptyState {
                        title: "Aucun profil exposant".to_string(),
                        message: "Creez votre profil exposant pour commencer a utiliser JayXpose.".to_string(),
                        icon: "🏪".to_string(),
                        action_label: "Creer mon profil".to_string(),
                        on_action: move |_| {
                            // Creer un exposant vide
                            let db = &conns.read().jayxpose;
                            let new_id = uuid::Uuid::new_v4().to_string();
                            let profile = jayxpose::data::ExposantProfile {
                                id: Some(new_id.clone()),
                                ..Default::default()
                            };
                            if db.exposant_upsert(&profile).is_ok() {
                                let mut s = state.write();
                                s.exposant_id = Some(new_id);
                                s.section = JayXposeSection::Entreprise;
                            }
                        },
                    }
                } else {
                    match state.read().section {
                        JayXposeSection::Dashboard => rsx! {
                            Dashboard { state: state }
                        },
                        JayXposeSection::Entreprise => rsx! {
                            Entreprise { state: state }
                        },
                        JayXposeSection::Catalogue => rsx! {
                            Catalogue { state: state }
                        },
                        JayXposeSection::NouveauProduit | JayXposeSection::ModifierProduit => rsx! {
                            ProduitForm { state: state }
                        },
                        JayXposeSection::Vitrine => rsx! {
                            Vitrine { state: state }
                        },
                        JayXposeSection::Documents => rsx! {
                            Documents { state: state }
                        },
                        JayXposeSection::FichePublique => rsx! {
                            FichePublique { state: state }
                        },
                    }
                }
            }
        }
    }
}
