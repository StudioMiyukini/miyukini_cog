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

/// Provenance d'un Service dans le Market.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ServiceSource {
    /// Service officiel Miyukini (pré-installé ou disponible dans le Market officiel)
    Officiel,
    /// Service tiers développé par la communauté
    Tiers,
}

#[allow(dead_code)]
impl ServiceSource {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Officiel => "Officiel",
            Self::Tiers => "Communauté",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Self::Officiel => "#10b981",  // Emerald
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
// Service Registry — gestion dynamique des services installés/disponibles
// ═══════════════════════════════════════════════════════════════════════════

/// Registre des services : catalogue officiel + tiers, installation/désinstallation.
/// Prépare l'API pour le « Services Market ».
pub struct ServiceRegistry;

/// Métadonnées d'un service officiel (source unique de vérité pour le catalogue).
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
    ServiceMeta { id: "jayxpose",     name: "JayXpose",         description: "Profil exposant, catalogue produits, vitrine, coffre-fort documentaire", icon: "\u{1F3EA}", service_type: ServiceType::SurfaceWeb, is_favorite: true },
    ServiceMeta { id: "jayfestival",  name: "JayFestival",      description: "Festivals, éditions, exposants, visiteurs",                              icon: "\u{1F4C5}", service_type: ServiceType::SurfaceWeb, is_favorite: false },
    ServiceMeta { id: "jaykoa",       name: "JayKoa",           description: "Calendrier universel du COG, récepteur temporel transversal",            icon: "\u{1F4C6}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "jaykonta",     name: "JayKonta",         description: "Comptabilité COG unifiée Purse + Account",                               icon: "\u{1F9EE}", service_type: ServiceType::InterneCog, is_favorite: true },
    ServiceMeta { id: "miyukiniwatch", name: "MiyukiniWatch",   description: "Tes habitudes et tes mesures — consulte, comprends, efface.",            icon: "\u{1F441}", service_type: ServiceType::InterneCog, is_favorite: false },
    ServiceMeta { id: "jay1tribu",    name: "Jay1Tribu",        description: "Tribus, amis et discussions — chat et tribu pleins uniquement si connecté au Webway.", icon: "\u{1F4AC}", service_type: ServiceType::InterCog, is_favorite: true },
    ServiceMeta { id: "jaymanga",     name: "JayManga",         description: "Lecture et vente de manga en ligne — catalogue, lecteur, boutique, portail agrégé", icon: "\u{1F4DA}", service_type: ServiceType::SurfaceWeb, is_favorite: false },
    ServiceMeta { id: "miyuclicker",  name: "Lord of the Click", description: "Premier jeu officiel Miyukini (Idle/Clicker + Carte stratégique)",      icon: "\u{1F3AE}", service_type: ServiceType::InterCog, is_favorite: true },
    ServiceMeta { id: "lord_of_the_castle", name: "Miyukini Survivor", description: "Jeu Survivor/Tower Defense officiel Miyukini",                    icon: "\u{1F3F0}", service_type: ServiceType::InterCog, is_favorite: false },
];

impl ServiceMeta {
    /// Convertit en `ServiceInfo` avec le flag d'installation.
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
        }
    }
}

/// IDs des services compilés (features actives).
fn compiled_service_ids() -> &'static [&'static str] {
    &[
        #[cfg(feature = "service-jayxpose")]        "jayxpose",
        #[cfg(feature = "service-jayfestival")]      "jayfestival",
        #[cfg(feature = "service-jaykoa")]           "jaykoa",
        #[cfg(feature = "service-jaykonta")]         "jaykonta",
        #[cfg(feature = "service-miyukiniwatch")]    "miyukiniwatch",
        #[cfg(feature = "service-jay1tribu")]        "jay1tribu",
        #[cfg(feature = "service-jaymanga")]         "jaymanga",
        #[cfg(feature = "service-miyuclicker")]      "miyuclicker",
        #[cfg(feature = "service-lord-of-the-castle")] "lord_of_the_castle",
    ]
}

impl ServiceRegistry {
    /// Liste tous les services compilés (features actives) avec `is_installed = true`.
    /// Le Market est toujours inclus (service intégré au Central).
    pub fn compiled_services() -> Vec<ServiceInfo> {
        let ids = compiled_service_ids();
        let mut services: Vec<ServiceInfo> = OFFICIAL_CATALOG
            .iter()
            .filter(|m| ids.contains(&m.id))
            .map(|m| m.to_service_info(true))
            .collect();

        // Le Market est un service intégré, toujours disponible
        services.push(ServiceInfo {
            id: "market".into(),
            name: "Service Market".into(),
            description: "Catalogue des services \u{2014} chercher, installer, d\u{00e9}sinstaller.".into(),
            icon: "\u{1F6D2}".into(),
            service_type: ServiceType::InterneCog,
            source: ServiceSource::Officiel,
            is_installed: true,
            is_favorite: true,
            version: env!("CARGO_PKG_VERSION").into(),
            developer: "Miyukini".into(),
        });

        services
    }

    /// Fallback local : services officiels non compilés (sans appel réseau).
    #[allow(dead_code)]
    pub fn local_available() -> Vec<ServiceInfo> {
        let ids = compiled_service_ids();
        OFFICIAL_CATALOG
            .iter()
            .filter(|m| !ids.contains(&m.id))
            .map(|m| m.to_service_info(false))
            .collect()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API Services Market — Via MarketClient (requêtes vers Origin)
    // ═══════════════════════════════════════════════════════════════════════

    /// Récupère le catalogue complet depuis Origin.
    /// En cas d'échec réseau, retombe sur le catalogue local.
    #[allow(dead_code)]
    pub async fn fetch_catalog(client: &crate::market_client::MarketClient) -> (Vec<ServiceInfo>, Vec<ServiceInfo>) {
        match client.fetch_catalog().await {
            Ok(catalog) => {
                let official = catalog.official.into_iter().map(|e| market_entry_to_service_info(e, true)).collect();
                let community = catalog.community.into_iter().map(|e| market_entry_to_service_info(e, false)).collect();
                (official, community)
            }
            Err(e) => {
                tracing::debug!("Market catalog fetch failed (fallback local): {e}");
                (Self::local_available(), Vec::new())
            }
        }
    }

    /// Recherche dans le catalogue Origin.
    #[allow(dead_code)]
    pub async fn search(client: &crate::market_client::MarketClient, query: &str) -> Vec<ServiceInfo> {
        match client.search(query).await {
            Ok(result) => result.results.into_iter().map(|e| market_entry_to_service_info(e, false)).collect(),
            Err(e) => {
                tracing::debug!("Market search failed: {e}");
                Vec::new()
            }
        }
    }

    /// Installe un service depuis le Market.
    /// Phase 1 : les services officiels nécessitent une recompilation avec la feature activée.
    /// Phase 2 : les services tiers seront chargés via plugins WASM/dylib.
    #[allow(dead_code)]
    pub async fn install(client: &crate::market_client::MarketClient, service_id: &str, version: &str) -> Result<Vec<u8>, String> {
        // Télécharger le package depuis Origin
        client.download_package(service_id, version)
            .await
            .map_err(|e| format!("Téléchargement du package échoué : {e}"))
        // Phase 1 : retourne les bytes du package. L'installation effective
        // nécessite une recompilation de Central avec la feature activée.
        // Phase 2 : extraire le .msp, charger le WASM/dylib, enregistrer.
    }

    /// Désinstalle un service (supprime les données locales).
    #[allow(dead_code)]
    pub fn uninstall(_service_id: &str) -> Result<(), String> {
        // Phase 1 : seule la suppression des données locales est possible.
        // La feature doit être désactivée manuellement dans Cargo.toml et recompilée.
        Err("Désinstallation dynamique non encore disponible. Désactivez la feature Cargo correspondante et recompilez.".into())
    }

    /// Publie un service sur le Market (pour les développeurs).
    #[allow(dead_code)]
    pub async fn publish(client: &crate::market_client::MarketClient, manifest: miyumarket::manifest::ServiceManifest, token: &str) -> Result<String, String> {
        client.publish(manifest, token)
            .await
            .map(|resp| format!("Service {} v{} publié (checksum: {})", resp.service_id, resp.version, resp.checksum))
            .map_err(|e| format!("Publication échouée : {e}"))
    }
}

/// Convertit un `MarketEntry` (protocole réseau) en `ServiceInfo` (état local).
#[allow(dead_code)]
fn market_entry_to_service_info(entry: miyumarket::protocol::MarketEntry, is_official: bool) -> ServiceInfo {
    let m = entry.manifest;
    let compiled_ids = compiled_service_ids();
    let is_installed = compiled_ids.contains(&m.id.as_str());

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
        source: if is_official { ServiceSource::Officiel } else { ServiceSource::Tiers },
        is_installed,
        is_favorite: false,
        version: m.version,
        developer: m.developer,
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
    /// Services disponibles (compilés + installés)
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
        let services = ServiceRegistry::compiled_services();
        // Seul l'onglet Accueil est ouvert au démarrage.
        // Les services s'ouvrent à la demande via open_service().
        let open_tabs = vec![OpenTab {
            id: "home".to_string(),
            title: "Accueil".to_string(),
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
