//! État global de l'application Miyukini Central.

use std::sync::Arc;

use crate::data::ServiceConnections;
use crate::miou::state::{MiouPreferences, MiouState};
use crate::remote::RemoteState;
use crate::service_manager::ServiceManager;
use crate::theme::Theme;
use dioxus::prelude::*;
use miyukini_central::auth::CentralProfile;

/// Contexte partagé (connexions + état + service manager) fourni une seule fois à la racine.
#[derive(Clone)]
pub struct AppContext {
    pub connections: Signal<Arc<ServiceConnections>>,
    pub state: Signal<AppState>,
    pub service_manager: ServiceManager,
    /// État du CentralRemote (remote activé, adresse, clients connectés).
    pub remote_state: Signal<RemoteState>,
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
            Self::Bibliotheque => "SERVICES",
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
            Self::InterneCog => "#3b82f6", // Blue
            Self::SurfaceWeb => "#10b981", // Emerald
            Self::InterCog => "#8b5cf6",   // Violet
        }
    }
}

/// Provenance d'un Service dans le Market.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceSource {
    Officiel,
    Tiers,
}

impl ServiceSource {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Officiel => "Officiel",
            Self::Tiers => "Communauté",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Self::Officiel => "#10b981", // Emerald
            Self::Tiers => "#f59e0b",    // Amber
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
    pub source: ServiceSource,
    pub is_installed: bool,
    pub is_favorite: bool,
    pub version: String,
    pub developer: String,
    pub downloadable: bool,
}

/// Onglet ouvert dans la zone de contenu (service actif).
#[derive(Debug, Clone, PartialEq)]
pub struct OpenTab {
    pub id: String,
    pub title: String,
    pub service_id: Option<String>,
    pub closable: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Service Registry — registre dynamique des services installés/disponibles
// ═══════════════════════════════════════════════════════════════════════════

/// Registre des services. Lit les services installés depuis le ServiceManager
/// et les combine avec le catalogue officiel (services non encore installés).
pub struct ServiceRegistry;

/// Métadonnées d'un service officiel (catalogue de référence).
struct ServiceMeta {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    icon: &'static str,
    service_type: ServiceType,
    is_favorite: bool,
}

/// Catalogue officiel complet — chaque service est décrit une seule fois.
const OFFICIAL_CATALOG: &[ServiceMeta] = &[
    ServiceMeta { id: "jaykoa",       name: "JayKoa",           description: "Calendrier universel du COG, récepteur temporel transversal",            icon: "\u{1F4C6}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "jaykonta",     name: "JayKonta",         description: "Comptabilité COG unifiée Purse + Account",                               icon: "\u{1F9EE}", service_type: ServiceType::InterneCog, is_favorite: true },
    ServiceMeta { id: "miyukiniwatch", name: "MiyukiniWatch",   description: "Tes habitudes et tes mesures — consulte, comprends, efface.",            icon: "\u{1F441}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "jay1tribu",    name: "Jay1Tribu",        description: "Tribus, amis et discussions — chat et tribu pleins uniquement si connecté au Webway.", icon: "\u{1F4AC}", service_type: ServiceType::InterCog, is_favorite: true },
    ServiceMeta { id: "jaymanga",     name: "JayManga",         description: "Lecture et vente de manga en ligne — catalogue, lecteur, boutique, portail agrégé", icon: "\u{1F4DA}", service_type: ServiceType::SurfaceWeb, is_favorite: false },
    ServiceMeta { id: "miou-llm-bridge", name: "Miyukini AI Studio",   description: "Service IA local — inférence GGUF native, agents spécialisés, skills, tool calling", icon: "\u{1F9E0}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "miyukini-whisper", name: "Miyukini Whisper", description: "Dictee locale STT/TTS avec presets hardware FR/EN et fallback opt-in", icon: "\u{1F3A4}", service_type: ServiceType::InterneCog, is_favorite: true },
    ServiceMeta { id: "alicia",          name: "Alicia Home Assistante", description: "Assistant vocal local Alicia — capture audio, détection de mot-clé, domotique 100% hors-ligne", icon: "\u{1F399}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "miyucloud",       name: "MiyuCloud",             description: "Cloud priv\u{00e9} \u{2014} fichiers, sync, partage s\u{00e9}curis\u{00e9}",                                      icon: "\u{2601}",  service_type: ServiceType::InterCog,   is_favorite: true },
];

impl ServiceMeta {
    fn to_service_info(&self, installed: bool) -> ServiceInfo {
        ServiceInfo {
            id: self.id.into(),
            name: self.name.into(),
            description: self.description.into(),
            icon: self.icon.into(),
            service_type: self.service_type,
            source: ServiceSource::Officiel,
            is_installed: installed,
            is_favorite: self.is_favorite,
            version: "0.1.0".into(),
            developer: "Miyukini".into(),
            downloadable: false,
        }
    }
}

impl ServiceRegistry {
    /// Liste uniquement les services réellement installés (sans injecter le catalogue officiel).
    pub fn installed_only(manager: &ServiceManager) -> Vec<ServiceInfo> {
        let installed = manager.installed_services();
        let mut services: Vec<ServiceInfo> = Vec::new();

        for svc in &installed {
            let catalog_meta = OFFICIAL_CATALOG.iter().find(|m| m.id == svc.manifest.id);
            services.push(ServiceInfo {
                id: svc.manifest.id.clone(),
                name: svc.manifest.name.clone(),
                description: svc.manifest.description.clone(),
                icon: svc.manifest.icon.clone(),
                service_type: match svc.manifest.service_type {
                    miyumarket::manifest::ServiceType::InterneCog => ServiceType::InterneCog,
                    miyumarket::manifest::ServiceType::SurfaceWeb => ServiceType::SurfaceWeb,
                    miyumarket::manifest::ServiceType::InterCog => ServiceType::InterCog,
                },
                source: match svc.manifest.source {
                    miyumarket::manifest::ServiceSource::Officiel => ServiceSource::Officiel,
                    miyumarket::manifest::ServiceSource::Tiers => ServiceSource::Tiers,
                },
                is_installed: true,
                is_favorite: catalog_meta.map_or(false, |m| m.is_favorite),
                version: svc.manifest.version.clone(),
                developer: svc.manifest.developer.clone(),
                downloadable: false,
            });
        }

        services
    }

    /// Liste les services installés (depuis le registre dynamique sur disque).
    /// Le Market est toujours inclus comme service intégré.
    pub fn installed_services(manager: &ServiceManager) -> Vec<ServiceInfo> {
        let mut services: Vec<ServiceInfo> = Self::installed_only(manager);
        let installed_ids: Vec<String> = services.iter().map(|s| s.id.clone()).collect();

        // Ajouter les services du catalogue officiel non installés (pour la liste complète)
        for meta in OFFICIAL_CATALOG {
            if !installed_ids.iter().any(|id| id == meta.id) {
                services.push(meta.to_service_info(false));
            }
        }

        // Le Market est un service intégré, toujours disponible
        services.push(ServiceInfo {
            id: "market".into(),
            name: "Services".into(),
            description:
                "Catalogue des services \u{2014} chercher, installer, d\u{00e9}sinstaller.".into(),
            icon: "\u{1F6D2}".into(),
            service_type: ServiceType::InterneCog,
            source: ServiceSource::Officiel,
            is_installed: true,
            is_favorite: true,
            version: env!("CARGO_PKG_VERSION").into(),
            developer: "Miyukini".into(),
            downloadable: false,
        });

        services
    }

    /// Services officiels non installés (fallback local sans réseau).
    pub fn available_services(manager: &ServiceManager) -> Vec<ServiceInfo> {
        let installed = manager.installed_services();
        let installed_ids: Vec<String> = installed.iter().map(|s| s.manifest.id.clone()).collect();

        OFFICIAL_CATALOG
            .iter()
            .filter(|m| !installed_ids.iter().any(|id| id == m.id))
            .map(|m| m.to_service_info(false))
            .collect()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API Services Market — Via MarketClient (requêtes vers Origin)
    // ═══════════════════════════════════════════════════════════════════════

    /// Récupère le catalogue complet depuis Origin.
    #[allow(dead_code)]
    pub async fn fetch_catalog(
        client: &crate::market_client::MarketClient,
        manager: &ServiceManager,
    ) -> (Vec<ServiceInfo>, Vec<ServiceInfo>) {
        match client.fetch_catalog().await {
            Ok(catalog) => {
                let official = catalog
                    .official
                    .into_iter()
                    .map(|e| market_entry_to_service_info(e, manager))
                    .collect();
                let community = catalog
                    .community
                    .into_iter()
                    .map(|e| market_entry_to_service_info(e, manager))
                    .collect();
                (official, community)
            }
            Err(e) => {
                tracing::debug!("Market catalog fetch failed (fallback local): {e}");
                (Self::available_services(manager), Vec::new())
            }
        }
    }

    /// Recherche dans le catalogue Origin.
    #[allow(dead_code)]
    pub async fn search(
        client: &crate::market_client::MarketClient,
        query: &str,
        manager: &ServiceManager,
    ) -> Vec<ServiceInfo> {
        match client.search(query).await {
            Ok(result) => result
                .results
                .into_iter()
                .map(|e| market_entry_to_service_info(e, manager))
                .collect(),
            Err(e) => {
                tracing::debug!("Market search failed: {e}");
                Vec::new()
            }
        }
    }
}

/// Convertit un `MarketEntry` (protocole réseau) en `ServiceInfo` (état local).
pub fn market_entry_to_service_info(
    entry: miyumarket::protocol::MarketEntry,
    manager: &ServiceManager,
) -> ServiceInfo {
    let m = entry.manifest;
    let is_installed = manager.is_installed(&m.id);

    ServiceInfo {
        id: m.id,
        name: m.name,
        description: m.description,
        icon: m.icon,
        service_type: match m.service_type {
            miyumarket::manifest::ServiceType::InterneCog => ServiceType::InterneCog,
            miyumarket::manifest::ServiceType::SurfaceWeb => ServiceType::SurfaceWeb,
            miyumarket::manifest::ServiceType::InterCog => ServiceType::InterCog,
        },
        source: match m.source {
            miyumarket::manifest::ServiceSource::Officiel => ServiceSource::Officiel,
            miyumarket::manifest::ServiceSource::Tiers => ServiceSource::Tiers,
        },
        is_installed,
        is_favorite: false,
        version: m.version,
        developer: m.developer,
        downloadable: entry.downloadable,
    }
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
    /// Services disponibles (installés + catalogue)
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
    /// Rite d'Entrée : profil créé, en attente des infos complémentaires.
    pub rite_infos_pending: bool,
    /// État Miou de la session (bulles, file d'attente, historique).
    pub miou_state: MiouState,
    /// Préférences Miou (persistées avec le profil).
    pub miou_prefs: MiouPreferences,
    /// Première bulle Miou déjà déclenchée cette session.
    pub miou_first_trigger_done: bool,
    /// Historique du chat Miou (persiste entre changements d'onglets).
    pub miou_chat_messages: Vec<MiouChatMsg>,
    /// Status du chat LLM (nom du modèle chargé, ou None si pas encore connecté).
    pub miou_chat_model: Option<String>,
}

/// Message du chat Miou (persisté dans AppState).
#[derive(Debug, Clone)]
pub struct MiouChatMsg {
    pub role: String,
    pub content: String,
}

impl AppState {
    /// Crée l'état initial avec le ServiceManager pour charger les services.
    pub fn new(manager: &ServiceManager) -> Self {
        let services = ServiceRegistry::installed_services(manager);
        let open_tabs = vec![OpenTab {
            id: "home".to_string(),
            title: "Salon".to_string(),
            service_id: None,
            closable: false,
        }];
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
            rite_infos_pending: false,
            miou_state: MiouState::default(),
            miou_prefs: MiouPreferences::default(),
            miou_first_trigger_done: false,
            miou_chat_messages: Vec::new(),
            miou_chat_model: None,
        }
    }

    /// Rafraîchit la liste des services depuis le ServiceManager.
    pub fn refresh_services(&mut self, manager: &ServiceManager) {
        self.services = ServiceRegistry::installed_services(manager);
    }

    /// Ouvre un service dans un nouvel onglet.
    pub fn open_service(&mut self, service: &ServiceInfo) {
        if let Some(idx) = self
            .open_tabs
            .iter()
            .position(|t| t.service_id.as_ref() == Some(&service.id))
        {
            self.active_tab_index = idx;
            return;
        }

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

/// Default utilisé pour les cas où le ServiceManager n'est pas dispo
/// (ne devrait pas arriver en pratique).
impl Default for AppState {
    fn default() -> Self {
        Self {
            main_tab: MainTab::Salon,
            open_tabs: vec![OpenTab {
                id: "home".to_string(),
                title: "Salon".to_string(),
                service_id: None,
                closable: false,
            }],
            active_tab_index: 0,
            services: Vec::new(),
            search_query: String::new(),
            current_user: None,
            is_cog_virgin: true,
            show_profile_window: false,
            current_theme: Theme::Gaming,
            last_login_email: String::new(),
            last_login_pseudo: String::new(),
            rite_infos_pending: false,
            miou_state: MiouState::default(),
            miou_prefs: MiouPreferences::default(),
            miou_first_trigger_done: false,
            miou_chat_messages: Vec::new(),
            miou_chat_model: None,
        }
    }
}

/// Signal global pour l'état de l'application.
pub fn use_app_state() -> Signal<AppState> {
    use_context::<AppContext>().state
}

/// Accès au ServiceManager depuis un composant Dioxus.
pub fn use_service_manager() -> ServiceManager {
    use_context::<AppContext>().service_manager.clone()
}
