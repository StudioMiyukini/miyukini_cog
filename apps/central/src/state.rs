//! État global de l'application Miyukini Central.

use std::sync::Arc;

use dioxus::prelude::*;
use miyukini_central::auth::CentralProfile;
use crate::data::ServiceConnections;
use crate::theme::Theme;
use crate::miou::state::{MiouState, MiouPreferences};

/// Contexte partagé (connexions + état) fourni une seule fois à la racine pour éviter hook-in-hook.
#[derive(Clone)]
pub struct AppContext {
    pub connections: Signal<Arc<ServiceConnections>>,
    pub state: Signal<AppState>,
}

/// Onglet principal de navigation (header).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainTab {
    /// Salon des Services
    #[default]
    Salon,
    /// Bibliothèque des Services installés
    Bibliotheque,
    /// Webway (réseau MWS)
    Communaute,
    /// Liste des amis (Jay1Tribu)
    MesAmis,
}

impl MainTab {
    /// Libellé affiché dans l'interface.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Salon => "SALON",
            Self::Bibliotheque => "BIBLIOTHÈQUE",
            Self::Communaute => "WEBWAY",
            Self::MesAmis => "MES AMIS",
        }
    }

    /// Toutes les valeurs pour itération.
    pub fn all() -> &'static [MainTab] {
        &[
            MainTab::Salon,
            MainTab::Bibliotheque,
            MainTab::Communaute,
            MainTab::MesAmis,
        ]
    }
}

/// Type de Service selon la nomenclature Miyukini.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    /// Type 1: Service interne COG
    InterneCog = 1,
    /// Type 2: Service à surface web externe
    SurfaceWeb = 2,
    /// Type 3: Service Inter-COG
    InterCog = 3,
}

impl ServiceType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::InterneCog => "Interne COG",
            Self::SurfaceWeb => "Surface Web",
            Self::InterCog => "Inter-COG",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::InterneCog => "#3b82f6",  // Blue
            Self::SurfaceWeb => "#10b981",  // Emerald
            Self::InterCog => "#8b5cf6",    // Violet
        }
    }
}

/// Informations sur un Service.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub service_type: ServiceType,
    pub is_installed: bool,
    pub is_favorite: bool,
    pub version: String,
    pub developer: String,
}

/// Onglet ouvert dans la zone de contenu (service actif).
#[derive(Debug, Clone, PartialEq)]
pub struct OpenTab {
    pub id: String,
    pub title: String,
    pub service_id: Option<String>,
    pub closable: bool,
}

/// État global de l'application.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Onglet principal actif
    pub main_tab: MainTab,
    /// Onglets ouverts dans la zone de contenu
    pub open_tabs: Vec<OpenTab>,
    /// Index de l'onglet actif
    pub active_tab_index: usize,
    /// Services disponibles
    pub services: Vec<ServiceInfo>,
    /// Recherche en cours
    pub search_query: String,
    /// Profil connecté (pseudo affiché dans le header).
    pub current_user: Option<CentralProfile>,
    /// True tant qu'aucun compte n'a été créé ; passe à false à la création du premier compte.
    pub is_cog_virgin: bool,
    /// Fenêtre profil ouverte (clic sur le pseudo dans le header).
    pub show_profile_window: bool,
    /// Thème visuel (Gaming = Steam, etc.).
    pub current_theme: Theme,
    /// E-mail du dernier profil connecté (pré-rempli sur l'écran de connexion).
    pub last_login_email: String,
    /// Pseudo du dernier profil connecté (pour le message d'accueil).
    pub last_login_pseudo: String,
    /// Session MiyukiniWatch en cours (pour la collecte).
    pub miyukiniwatch_session_id: Option<String>,
    /// Début de session (pour calcul durée à la déconnexion).
    pub miyukiniwatch_session_started_at: Option<std::time::Instant>,
    /// Rite d'Entrée : profil créé, en attente des infos complémentaires.
    pub rite_infos_pending: bool,
    /// État Miou de la session (bulles, file d'attente, historique).
    pub miou_state: MiouState,
    /// Préférences Miou (persistées avec le profil).
    pub miou_prefs: MiouPreferences,
    /// Première bulle Miou déjà déclenchée cette session.
    pub miou_first_trigger_done: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let services = Self::default_services();
        let mut open_tabs = vec![OpenTab {
            id: "home".to_string(),
            title: "Accueil".to_string(),
            service_id: None,
            closable: false,
        }];
        for s in &services {
            if s.is_installed {
                open_tabs.push(OpenTab {
                    id: s.id.clone(),
                    title: s.name.clone(),
                    service_id: Some(s.id.clone()),
                    closable: true,
                });
            }
        }
        Self {
            main_tab: MainTab::Salon,
            open_tabs,
            active_tab_index: 0,
            services,
            search_query: String::new(),
            current_user: None,
            is_cog_virgin: true,
            show_profile_window: false,
            current_theme: Theme::Gaming,
            last_login_email: String::new(),
            last_login_pseudo: String::new(),
            miyukiniwatch_session_id: None,
            miyukiniwatch_session_started_at: None,
            rite_infos_pending: false,
            miou_state: MiouState::default(),
            miou_prefs: MiouPreferences::default(),
            miou_first_trigger_done: false,
        }
    }
}

impl AppState {
    /// Services par défaut du COG.
    fn default_services() -> Vec<ServiceInfo> {
        vec![
            ServiceInfo {
                id: "jayxpose".to_string(),
                name: "JayXpose".to_string(),
                description: "Profil exposant, catalogue produits, vitrine, coffre-fort documentaire".to_string(),
                icon: "🏪".to_string(),
                service_type: ServiceType::SurfaceWeb,
                is_installed: true,
                is_favorite: true,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "jayfestival".to_string(),
                name: "JayFestival".to_string(),
                description: "Festivals, éditions, exposants, visiteurs".to_string(),
                icon: "📅".to_string(),
                service_type: ServiceType::SurfaceWeb,
                is_installed: true,
                is_favorite: false,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "jaykoa".to_string(),
                name: "JayKoa".to_string(),
                description: "Calendrier universel du COG, récepteur temporel transversal".to_string(),
                icon: "📆".to_string(),
                service_type: ServiceType::InterneCog,
                is_installed: true,
                is_favorite: false,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "jaykonta".to_string(),
                name: "JayKonta".to_string(),
                description: "Comptabilité COG unifiée Purse + Account".to_string(),
                icon: "🧮".to_string(),
                service_type: ServiceType::InterneCog,
                is_installed: true,
                is_favorite: true,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "miyukiniwatch".to_string(),
                name: "MiyukiniWatch".to_string(),
                description: "Tes habitudes et tes mesures — consulte, comprends, efface.".to_string(),
                icon: "👁".to_string(),
                service_type: ServiceType::InterneCog,
                is_installed: true,
                is_favorite: false,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "jay1tribu".to_string(),
                name: "Jay1Tribu".to_string(),
                description: "Tribus, amis et discussions — chat et tribu pleins uniquement si connecté au Webway.".to_string(),
                icon: "💬".to_string(),
                service_type: ServiceType::InterCog,
                is_installed: true,
                is_favorite: true,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "jaymanga".to_string(),
                name: "JayManga".to_string(),
                description: "Lecture et vente de manga en ligne — catalogue, lecteur, boutique, portail agrégé".to_string(),
                icon: "📚".to_string(),
                service_type: ServiceType::SurfaceWeb,
                is_installed: true,
                is_favorite: false,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "miyuclicker".to_string(),
                name: "Lord of the Click".to_string(),
                description: "Premier jeu officiel Miyukini (Idle/Clicker + Carte stratégique)".to_string(),
                icon: "🎮".to_string(),
                service_type: ServiceType::InterCog,
                is_installed: true,
                is_favorite: true,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
            ServiceInfo {
                id: "lord_of_the_castle".to_string(),
                name: "Miyukini Survivor".to_string(),
                description: "Jeu Survivor/Tower Defense officiel Miyukini".to_string(),
                icon: "🏰".to_string(),
                service_type: ServiceType::InterCog,
                is_installed: true,
                is_favorite: false,
                version: "0.1.0".to_string(),
                developer: "Miyukini".to_string(),
            },
        ]
    }

    /// Ouvre un service dans un nouvel onglet.
    pub fn open_service(&mut self, service: &ServiceInfo) {
        // Vérifier si déjà ouvert
        if let Some(idx) = self.open_tabs.iter().position(|t| {
            t.service_id.as_ref() == Some(&service.id)
        }) {
            self.active_tab_index = idx;
            return;
        }

        // Créer un nouvel onglet
        let tab = OpenTab {
            id: service.id.clone(),
            title: service.name.clone(),
            service_id: Some(service.id.clone()),
            closable: true,
        };

        self.open_tabs.push(tab);
        self.active_tab_index = self.open_tabs.len() - 1;
    }

    /// Ferme un onglet.
    pub fn close_tab(&mut self, index: usize) {
        if index < self.open_tabs.len() && self.open_tabs[index].closable {
            self.open_tabs.remove(index);
            if self.active_tab_index >= self.open_tabs.len() {
                self.active_tab_index = self.open_tabs.len().saturating_sub(1);
            }
        }
    }
}

/// Signal global pour l'état de l'application.
pub fn use_app_state() -> Signal<AppState> {
    use_context::<AppContext>().state
}
