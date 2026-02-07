//! Catalogue mock des Services (Registre d'Opérateurs simulé pour le MVP).
//!
//! En production, la source de vérité serait Master Butler via BondingBrother.
//!
//! @id: miyukini_central_catalog_module
//! @do: provide_mock_service_catalog_and_metadata
//! @role: data
//! @layer: domain
//! @human: Registre mock des Opérateurs et métadonnées pour le HUB.

/// Identifiant unique d'un Service (factice pour le MVP).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ServiceId {
    /// Calculatrice.
    Calculator,
    /// Jeu (démonstration).
    Game,
    /// Lord of the Click — Idle/Clicker + Carte stratégique (point d'accès : Miyukini Central).
    MiyuClicker,
    /// Lord of the Castle — Miyukini Survivor, Survivor + Tower Defense (point d'accès : Miyukini Central).
    LordOfTheCastle,
    /// Traitement de texte.
    TextEditor,
    /// Notes rapides.
    Notes,
    /// Bibliothèque UI native Miyukini COG.
    UiLibrary,
    // --- Services Jay (mock) ---
    /// JayRDV — RDV, créneaux, exceptions.
    JayRDV,
    /// JayFestival — Éditions, exposants, événements.
    JayFestival,
    /// JayKoa — Agenda unifié, intégrateur des dates.
    JayKoa,
    /// JayKonta — Budget, facturation (marques JayBudget / JayKonta).
    JayKonta,
    /// JayXpose — Profil exposant, site vitrine.
    JayXpose,
    /// JayFaim — Restauration, commandes, food trucks.
    JayFaim,
    /// Miyukini Dev Center — centralisation des tests pour le développement des services.
    DevCenter,
    /// Miyukini UI Editor — personnalisation persistante de l'interface (thèmes, couleurs, formes).
    EguiEditor,
    /// Miyukini Récompenses — système de fidélisation, achievements, objets virtuels, boutique MIYU, gacha.
    MiyukiniLoyalty,
}

impl ServiceId {
    /// Retourne l'identifiant string pour affichage ou persistance.
    #[must_use] 
    pub fn as_str(&self) -> &'static str {
        match self {
            ServiceId::Calculator => "calculator",
            ServiceId::Game => "game",
            ServiceId::MiyuClicker => "Lord of the Click",
            ServiceId::LordOfTheCastle => "Lord of the Castle",
            ServiceId::TextEditor => "text_editor",
            ServiceId::Notes => "notes",
            ServiceId::UiLibrary => "Miyukini UI Builder",
            ServiceId::JayRDV => "JayRDV",
            ServiceId::JayFestival => "JayFestival",
            ServiceId::JayKoa => "JayKoa",
            ServiceId::JayKonta => "JayKonta",
            ServiceId::JayXpose => "JayXpose",
            ServiceId::JayFaim => "JayFaim",
            ServiceId::DevCenter => "Miyukini Dev Center",
            ServiceId::EguiEditor => "Miyukini UI Editor",
            ServiceId::MiyukiniLoyalty => "Miyukini Récompenses",
        }
    }
    /// Clé stable pour persistance (suivi d'utilisation, DB).
    #[must_use] 
    pub fn storage_key(&self) -> &'static str {
        match self {
            ServiceId::Calculator => "calculator",
            ServiceId::Game => "game",
            ServiceId::MiyuClicker => "miyuclicker",
            ServiceId::LordOfTheCastle => "lord_of_the_castle",
            ServiceId::TextEditor => "text_editor",
            ServiceId::Notes => "notes",
            ServiceId::UiLibrary => "ui_library",
            ServiceId::JayRDV => "jay_rdv",
            ServiceId::JayFestival => "jay_festival",
            ServiceId::JayKoa => "jay_koa",
            ServiceId::JayKonta => "jay_konta",
            ServiceId::JayXpose => "jay_xpose",
            ServiceId::JayFaim => "jay_faim",
            ServiceId::DevCenter => "dev_center",
            ServiceId::EguiEditor => "egui_editor",
            ServiceId::MiyukiniLoyalty => "miyukini_loyalty",
        }
    }

    /// Parse un ServiceId depuis une clé de stockage.
    #[must_use] 
    pub fn from_storage_key(s: &str) -> Option<ServiceId> {
        match s {
            "calculator" => Some(ServiceId::Calculator),
            "game" => Some(ServiceId::Game),
            "miyuclicker" => Some(ServiceId::MiyuClicker),
            "lord_of_the_castle" => Some(ServiceId::LordOfTheCastle),
            "text_editor" => Some(ServiceId::TextEditor),
            "notes" => Some(ServiceId::Notes),
            "ui_library" => Some(ServiceId::UiLibrary),
            "jay_rdv" => Some(ServiceId::JayRDV),
            "jay_festival" => Some(ServiceId::JayFestival),
            "jay_koa" => Some(ServiceId::JayKoa),
            "jay_konta" => Some(ServiceId::JayKonta),
            "jay_xpose" => Some(ServiceId::JayXpose),
            "jay_faim" => Some(ServiceId::JayFaim),
            "dev_center" => Some(ServiceId::DevCenter),
            "egui_editor" => Some(ServiceId::EguiEditor),
            "miyukini_loyalty" => Some(ServiceId::MiyukiniLoyalty),
            _ => None,
        }
    }

    /// Icône / emoji pour l'affichage type store.
    #[must_use] 
    pub fn icon(&self) -> &'static str {
        match self {
            ServiceId::Calculator => "🔢",
            ServiceId::Game => "🎮",
            ServiceId::MiyuClicker => "🏰",
            ServiceId::LordOfTheCastle => "⚔️",
            ServiceId::TextEditor => "📝",
            ServiceId::Notes => "📋",
            ServiceId::UiLibrary => "🎨",
            ServiceId::JayRDV => "📅",
            ServiceId::JayFestival => "🎪",
            ServiceId::JayKoa => "🗓️",
            ServiceId::JayKonta => "💰",
            ServiceId::JayXpose => "🖼️",
            ServiceId::JayFaim => "🍽️",
            ServiceId::DevCenter => "🧪",
            ServiceId::EguiEditor => "🖌️",
            ServiceId::MiyukiniLoyalty => "🎁",
        }
    }
}

/// Catégorie de Services (pour le catalogue).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryId {
    /// Utilitaires (calculatrice, etc.).
    Utilitaires,
    /// Loisirs (jeux).
    Loisirs,
    /// Productivité (texte, notes).
    Productivite,
    /// Design / UI (bibliothèque de composants).
    Design,
    /// Services Jay (interpolarité : RDV, festival, agenda, budget, vitrine, restauration).
    Jay,
    /// Système (fidélisation, paramètres).
    Systeme,
}

impl CategoryId {
    /// Libellé affiché.
    #[must_use] 
    pub fn label(&self) -> &'static str {
        match self {
            CategoryId::Utilitaires => "Utilitaires",
            CategoryId::Loisirs => "Loisirs",
            CategoryId::Productivite => "Productivité",
            CategoryId::Design => "Design",
            CategoryId::Jay => "Services Jay",
            CategoryId::Systeme => "Système",
        }
    }
    /// Icône pour les filtres / pills.
    #[must_use] 
    pub fn icon(&self) -> &'static str {
        match self {
            CategoryId::Utilitaires => "🛠️",
            CategoryId::Loisirs => "🎯",
            CategoryId::Productivite => "⚡",
            CategoryId::Design => "🎨",
            CategoryId::Jay => "🌐",
            CategoryId::Systeme => "⚙️",
        }
    }
}

/// Métadonnées d'un Service (ce que l'utilisateur voit dans le catalogue).
/// @id: miyukini_central_catalog_service_meta
/// @do: represent_service_display_metadata
/// @role: data
/// @human: Nom, description, catégorie, version pour affichage catalogue.
#[derive(Debug, Clone)]
pub struct ServiceMeta {
    /// Identifiant du Service.
    pub id: ServiceId,
    /// Nom affiché.
    pub name: String,
    /// Description courte.
    pub description: String,
    /// Catégorie.
    pub category: CategoryId,
    /// Version factice (état de vie simulé).
    pub version: String,
}

/// Retourne le catalogue mock de tous les Services disponibles (MVP).
/// @id: miyukini_central_catalog_mock_catalog
/// @do: return_mock_service_metadata_list
/// @role: reader
/// @human: Liste des ServiceMeta pour HUB et menu.
#[must_use] 
pub fn mock_catalog() -> Vec<ServiceMeta> {
    vec![
        ServiceMeta {
            id: ServiceId::Calculator,
            name: "Calculatrice".to_string(),
            description: "Calculs basiques : addition, soustraction, multiplication, division.".to_string(),
            category: CategoryId::Utilitaires,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::Game,
            name: "Jeu".to_string(),
            description: "Jeu de démonstration : cliquez le plus vite possible.".to_string(),
            category: CategoryId::Loisirs,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::MiyuClicker,
            name: "Lord of the Click".to_string(),
            description: "Lord of the Click — Premier jeu officiel Miyukini, Idle/Clicker + Carte stratégique. Point d'accès : Miyukini Central.".to_string(),
            category: CategoryId::Loisirs,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::LordOfTheCastle,
            name: "Lord of the Castle".to_string(),
            description: "Lord of the Castle — Miyukini Survivor, Survivor + Tower Defense. Protégez le Château, construisez des tours, affrontez les vagues. Point d'accès : Miyukini Central.".to_string(),
            category: CategoryId::Loisirs,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::TextEditor,
            name: "Traitement de texte".to_string(),
            description: "Éditeur de texte simple pour rédiger des documents.".to_string(),
            category: CategoryId::Productivite,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::Notes,
            name: "Notes".to_string(),
            description: "Notes rapides : listez vos idées et tâches.".to_string(),
            category: CategoryId::Productivite,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::UiLibrary,
            name: "Miyukini UI Library".to_string(),
            description: "Bibliothèque des éléments UI natifs du COG : voir, filtrer par catégorie, créer (ex. boutons avec padding, bordure, ratio, états).".to_string(),
            category: CategoryId::Design,
            version: "1.0.0".to_string(),
        },
        // --- Services Jay (mock) ---
        ServiceMeta {
            id: ServiceId::JayRDV,
            name: "JayRDV".to_string(),
            description: "RDV, créneaux, exceptions. Service agenda des rendez-vous.".to_string(),
            category: CategoryId::Jay,
            version: "0.1.0 (mock)".to_string(),
        },
        ServiceMeta {
            id: ServiceId::JayFestival,
            name: "JayFestival".to_string(),
            description: "Éditions, exposants, événements. Gestion des festivals et éditions.".to_string(),
            category: CategoryId::Jay,
            version: "0.1.0 (mock)".to_string(),
        },
        ServiceMeta {
            id: ServiceId::JayKoa,
            name: "JayKoa".to_string(),
            description: "Agenda unifié : intègre tout ce qui manipule des dates (JayRDV, JayFestival, conflits, calendrier).".to_string(),
            category: CategoryId::Jay,
            version: "0.2.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::JayKonta,
            name: "JayKonta".to_string(),
            description: "Budget, facturation. Service COG (marques JayBudget, JayKonta).".to_string(),
            category: CategoryId::Jay,
            version: "0.1.0 (mock)".to_string(),
        },
        ServiceMeta {
            id: ServiceId::JayXpose,
            name: "JayXpose".to_string(),
            description: "Profil exposant, site vitrine. S'intègre dans JayFestival (fiche exposant).".to_string(),
            category: CategoryId::Jay,
            version: "0.1.0 (mock)".to_string(),
        },
        ServiceMeta {
            id: ServiceId::JayFaim,
            name: "JayFaim".to_string(),
            description: "Restauration, commandes en ligne, food trucks. Couplage avec JayFestival pour les stands.".to_string(),
            category: CategoryId::Jay,
            version: "0.1.0 (mock)".to_string(),
        },
        ServiceMeta {
            id: ServiceId::DevCenter,
            name: "Miyukini Dev Center".to_string(),
            description: "Centralise les tests unitaires et d'intégration de tous les services du Central. Vue par crate, commandes pour lancer les tests.".to_string(),
            category: CategoryId::Utilitaires,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::EguiEditor,
            name: "Miyukini UI Editor".to_string(),
            description: "Démo egui en local : changez couleurs, formes et angles, visibles en direct.".to_string(),
            category: CategoryId::Design,
            version: "1.0.0".to_string(),
        },
        ServiceMeta {
            id: ServiceId::MiyukiniLoyalty,
            name: "Miyukini Récompenses".to_string(),
            description: "Système de fidélisation : gagnez des points MIYU, débloquez des achievements, collectionnez des objets, et tentez votre chance au gacha. Tous les services participent !".to_string(),
            category: CategoryId::Systeme,
            version: "1.0.0".to_string(),
        },
    ]
}

/// Retourne les métadonnées d'un Service par son identifiant.
/// @id: miyukini_central_catalog_meta_for_service
/// @do: lookup_service_metadata_by_id
/// @role: reader
/// @human: Recherche dans le catalogue pour affichage carte HUB.
#[must_use] 
pub fn meta_for_service(catalog: &[ServiceMeta], id: ServiceId) -> Option<&ServiceMeta> {
    catalog.iter().find(|m| m.id == id)
}
