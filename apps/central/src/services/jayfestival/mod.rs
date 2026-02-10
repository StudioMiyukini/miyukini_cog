//! Module JayFestival — vue complète avec gestion multi-rôle.
//!
//! Architecture : facade publique (UNC) + espaces connectés (ORG/EXP/VIS).
//! Données réelles via JayFestivalDb (conns.read().jayfestival).

mod sidebar;
pub mod components;
mod org_dashboard;
mod org_editions;
mod org_edition_hub;
mod org_exposants;
mod org_programme;
mod org_budget;
mod org_plan;
mod exp_dashboard;
mod exp_candidatures;
mod vis_catalogue;

// Phase 1 — Facade publique (UNC)
mod unc_landing;
mod unc_events;
mod unc_directory;
mod unc_search;
mod unc_auth;

// Phase 2 — Organisateur complet (nouveaux onglets hub + pages autonomes)
mod org_parametres;
mod org_documents;
mod org_annonces;
mod org_services;
mod org_publication;
mod org_compte;
mod org_equipe;

// Phase 3 — Exposant complet
mod exp_participations;
mod exp_agenda;
mod exp_documents;
mod exp_factures;
mod exp_compte;
mod exp_fiche_publique;
mod exp_notifications;

// Phase 4 — Visiteur complet
mod vis_dashboard;
mod vis_agenda;
mod vis_billets;
mod vis_reservations;
mod vis_activites;
mod vis_compte;

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

use sidebar::JayFestivalSidebar;
use org_dashboard::OrgDashboard;
use org_editions::OrgEditions;
use org_edition_hub::OrgEditionHub;
use org_compte::OrgCompte;
use org_equipe::OrgEquipe;
use exp_dashboard::ExpDashboard;
use exp_candidatures::ExpCandidatures;
use vis_catalogue::VisCatalogue;

// UNC imports
use unc_landing::UncLanding;
use unc_events::{UncEventsList, UncEventDetail};
use unc_directory::{UncOrganisateursList, UncOrganisateurDetail, UncExposantsList, UncExposantDetail};
use unc_search::UncSearch;
use unc_auth::{UncCtaModal, UncConnexion, UncInscription, UncMentionsLegales};

// ── State ──────────────────────────────────────────────────────────────

/// Rôle actif dans JayFestival.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JayFestivalRole {
    Organisateur,
    Exposant,
    Visiteur,
}

impl Default for JayFestivalRole {
    fn default() -> Self {
        Self::Organisateur
    }
}

/// Section active pour utilisateur non connecté (UNC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UncSection {
    #[default]
    Landing,
    Events,
    EventDetail,
    Organisateurs,
    OrganisateurDetail,
    Exposants,
    ExposantDetail,
    Search,
    CtaModal,
    Connexion,
    Inscription,
    MentionsLegales,
}

/// Section active côté Organisateur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrgSection {
    #[default]
    Dashboard,
    Editions,
    EditionHub,
    Compte,
    Equipe,
}

/// Onglet actif dans le hub d'une édition (Organisateur).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrgEditionTab {
    #[default]
    Overview,
    Parametres,
    Exposants,
    Programme,
    Budget,
    Plan,
    Documents,
    Annonces,
    Services,
    Publish,
}

/// Section active côté Exposant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpSection {
    #[default]
    Dashboard,
    Candidatures,
    Participations,
    Agenda,
    Documents,
    Factures,
    FichePublique,
    Compte,
    Notifications,
}

/// Section active côté Visiteur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisSection {
    #[default]
    Dashboard,
    Catalogue,
    Agenda,
    Billets,
    Reservations,
    Activites,
    Compte,
}

/// État local complet de la vue JayFestival.
#[derive(Debug, Clone, Default)]
pub struct JayFestivalState {
    // Connexion
    pub is_connected: bool,
    pub current_user_id: Option<String>,
    
    // UNC (non connecté)
    pub unc_section: UncSection,
    pub search_query: Option<String>,
    pub selected_organisateur_id: Option<String>,
    pub selected_exposant_id: Option<String>,
    pub cta_action: Option<String>,
    pub inscription_type: Option<String>,
    
    // Connecté
    pub role: JayFestivalRole,
    pub org_section: OrgSection,
    pub selected_edition_id: Option<String>,
    pub edition_tab: OrgEditionTab,
    pub exp_section: ExpSection,
    pub vis_section: VisSection,
}

// ── Composant racine ───────────────────────────────────────────────────

#[component]
pub fn JayFestivalView() -> Element {
    let _app_state = use_app_state();
    let conns = use_service_connections();
    let state = use_signal(JayFestivalState::default);

    // Si non connecté, afficher la facade publique (UNC)
    if !state.read().is_connected {
        return rsx! {
            UncFacade { state: state }
        };
    }

    // Charger les données communes (utilisateur connecté)
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_list().unwrap_or_default()
    };

    let exposants_count = {
        let db = &conns.read().jayfestival;
        db.exposants_list(false).map(|v| v.len()).unwrap_or(0)
    };

    let pending_count = {
        let db = &conns.read().jayfestival;
        db.candidatures_pending_count().unwrap_or(0)
    };

    rsx! {
        div {
            style: "display: flex; height: 100%;",

            // Sidebar
            JayFestivalSidebar {
                state: state,
            }

            // Contenu principal
            main {
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                match state.read().role {
                    JayFestivalRole::Organisateur => {
                        match state.read().org_section {
                            OrgSection::Dashboard => rsx! {
                                OrgDashboard {
                                    editions_count: editions.len(),
                                    exposants_count: exposants_count,
                                    pending_count: pending_count,
                                    editions: editions.clone(),
                                    state: state,
                                }
                            },
                            OrgSection::Editions => rsx! {
                                OrgEditions {
                                    editions: editions.clone(),
                                    state: state,
                                }
                            },
                            OrgSection::EditionHub => rsx! {
                                OrgEditionHub {
                                    edition_id: state.read().selected_edition_id.clone().unwrap_or_default(),
                                    tab: state.read().edition_tab,
                                    state: state,
                                }
                            },
                            OrgSection::Compte => rsx! {
                                OrgCompte {}
                            },
                            OrgSection::Equipe => rsx! {
                                OrgEquipe {}
                            },
                        }
                    },
                    JayFestivalRole::Exposant => {
                        match state.read().exp_section {
                            ExpSection::Dashboard => rsx! {
                                ExpDashboard {}
                            },
                            ExpSection::Candidatures => rsx! {
                                ExpCandidatures {}
                            },
                            ExpSection::Participations => rsx! {
                                exp_participations::ExpParticipations {}
                            },
                            ExpSection::Agenda => rsx! {
                                exp_agenda::ExpAgenda {}
                            },
                            ExpSection::Documents => rsx! {
                                exp_documents::ExpDocuments {}
                            },
                            ExpSection::Factures => rsx! {
                                exp_factures::ExpFactures {}
                            },
                            ExpSection::FichePublique => rsx! {
                                exp_fiche_publique::ExpFichePublique {}
                            },
                            ExpSection::Compte => rsx! {
                                exp_compte::ExpCompte {}
                            },
                            ExpSection::Notifications => rsx! {
                                exp_notifications::ExpNotifications {}
                            },
                        }
                    },
                    JayFestivalRole::Visiteur => {
                        match state.read().vis_section {
                            VisSection::Dashboard => rsx! {
                                vis_dashboard::VisDashboard {}
                            },
                            VisSection::Catalogue => rsx! {
                                VisCatalogue {}
                            },
                            VisSection::Agenda => rsx! {
                                vis_agenda::VisAgenda {}
                            },
                            VisSection::Billets => rsx! {
                                vis_billets::VisBillets {}
                            },
                            VisSection::Reservations => rsx! {
                                vis_reservations::VisReservations {}
                            },
                            VisSection::Activites => rsx! {
                                vis_activites::VisActivites {}
                            },
                            VisSection::Compte => rsx! {
                                vis_compte::VisCompte {}
                            },
                        }
                    },
                }
            }
        }
    }
}

// ── Facade publique (UNC) ──────────────────────────────────────────────

/// Vue facade publique pour utilisateurs non connectés.
#[component]
fn UncFacade(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",

            // Header public
            header {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 16px 24px; border-bottom: 1px solid {c.border};",

                // Logo + Nav
                div {
                    style: "display: flex; align-items: center; gap: 32px;",

                    // Logo
                    div {
                        style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Landing;
                        },

                        span {
                            style: "font-size: 24px;",
                            "🎪"
                        }
                        span {
                            style: "font-size: 16px; font-weight: 600; color: {c.text_white};",
                            "JayFestival"
                        }
                    }

                    // Nav links
                    nav {
                        style: "display: flex; gap: 24px;",

                        NavLink {
                            label: "Evenements",
                            is_active: matches!(state.read().unc_section, UncSection::Events | UncSection::EventDetail),
                            onclick: move |_| {
                                state.write().unc_section = UncSection::Events;
                            },
                        }
                        NavLink {
                            label: "Organisateurs",
                            is_active: matches!(state.read().unc_section, UncSection::Organisateurs | UncSection::OrganisateurDetail),
                            onclick: move |_| {
                                state.write().unc_section = UncSection::Organisateurs;
                            },
                        }
                        NavLink {
                            label: "Exposants",
                            is_active: matches!(state.read().unc_section, UncSection::Exposants | UncSection::ExposantDetail),
                            onclick: move |_| {
                                state.write().unc_section = UncSection::Exposants;
                            },
                        }
                    }
                }

                // Actions
                div {
                    style: "display: flex; align-items: center; gap: 12px;",

                    button {
                        style: "padding: 8px 16px; background: transparent; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Connexion;
                        },
                        "Connexion"
                    }
                    button {
                        style: "padding: 8px 16px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Inscription;
                        },
                        "+ S'inscrire"
                    }
                }
            }

            // Contenu principal
            main {
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                match state.read().unc_section {
                    UncSection::Landing => rsx! {
                        UncLanding { state: state }
                    },
                    UncSection::Events => rsx! {
                        UncEventsList { state: state }
                    },
                    UncSection::EventDetail => rsx! {
                        UncEventDetail { state: state }
                    },
                    UncSection::Organisateurs => rsx! {
                        UncOrganisateursList { state: state }
                    },
                    UncSection::OrganisateurDetail => rsx! {
                        UncOrganisateurDetail { state: state }
                    },
                    UncSection::Exposants => rsx! {
                        UncExposantsList { state: state }
                    },
                    UncSection::ExposantDetail => rsx! {
                        UncExposantDetail { state: state }
                    },
                    UncSection::Search => rsx! {
                        UncSearch { state: state }
                    },
                    UncSection::CtaModal => rsx! {
                        UncLanding { state: state }
                        UncCtaModal { state: state }
                    },
                    UncSection::Connexion => rsx! {
                        UncConnexion { state: state }
                    },
                    UncSection::Inscription => rsx! {
                        UncInscription { state: state }
                    },
                    UncSection::MentionsLegales => rsx! {
                        UncMentionsLegales { state: state }
                    },
                }
            }

            // Footer
            footer {
                style: "padding: 16px 24px; border-top: 1px solid {c.border}; text-align: center;",

                div {
                    style: "display: flex; justify-content: center; gap: 24px; font-size: 12px; color: {c.text_muted};",

                    button {
                        style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::MentionsLegales;
                        },
                        "Mentions legales"
                    }
                    span { "•" }
                    button {
                        style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::MentionsLegales;
                        },
                        "CGU"
                    }
                    span { "•" }
                    button {
                        style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::MentionsLegales;
                        },
                        "Confidentialite"
                    }
                    span { "•" }
                    button {
                        style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::MentionsLegales;
                        },
                        "Accessibilite"
                    }
                }
            }
        }
    }
}

/// Lien de navigation header.
#[component]
fn NavLink(label: &'static str, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let color = if is_active { c.text_white } else { c.text_secondary };

    rsx! {
        button {
            style: "background: none; border: none; color: {color}; cursor: pointer; font-size: 14px;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
