//! Catalogue mock des Services (Registre d'Opérateurs simulé pour le MVP).
//!
//! En production, la source de vérité serait Master Butler via BondingBrother.

/// Identifiant unique d'un Service (factice pour le MVP).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceId {
    /// Calculatrice.
    Calculator,
    /// Jeu (démonstration).
    Game,
    /// Traitement de texte.
    TextEditor,
    /// Notes rapides.
    Notes,
}

impl ServiceId {
    /// Retourne l'identifiant string pour affichage ou persistance.
    pub fn as_str(&self) -> &'static str {
        match self {
            ServiceId::Calculator => "calculator",
            ServiceId::Game => "game",
            ServiceId::TextEditor => "text_editor",
            ServiceId::Notes => "notes",
        }
    }
    /// Icône / emoji pour l'affichage type store.
    pub fn icon(&self) -> &'static str {
        match self {
            ServiceId::Calculator => "🔢",
            ServiceId::Game => "🎮",
            ServiceId::TextEditor => "📝",
            ServiceId::Notes => "📋",
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
}

impl CategoryId {
    /// Libellé affiché.
    pub fn label(&self) -> &'static str {
        match self {
            CategoryId::Utilitaires => "Utilitaires",
            CategoryId::Loisirs => "Loisirs",
            CategoryId::Productivite => "Productivité",
        }
    }
    /// Icône pour les filtres / pills.
    pub fn icon(&self) -> &'static str {
        match self {
            CategoryId::Utilitaires => "🛠️",
            CategoryId::Loisirs => "🎯",
            CategoryId::Productivite => "⚡",
        }
    }
}

/// Métadonnées d'un Service (ce que l'utilisateur voit dans le catalogue).
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
    ]
}
