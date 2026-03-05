//! Manifeste d'un service — décrit ce qu'un service expose et requiert.
//!
//! Chaque package service contient un `service.manifest.json` à la racine.
//! Ce fichier est lu par Origin lors du `publish` et par Central lors de l'`install`.

use serde::{Deserialize, Serialize};

/// Type de service selon la nomenclature Miyukini.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType {
    /// Type 1 : Service interne COG (données locales uniquement).
    InterneCog,
    /// Type 2 : Service avec surface web externe (portail public).
    SurfaceWeb,
    /// Type 3 : Service Inter-COG (nécessite le Webway pour fonctionner pleinement).
    InterCog,
}

/// Source/provenance d'un service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceSource {
    /// Service officiel développé par Miyukini.
    Officiel,
    /// Service tiers développé par la communauté.
    Tiers,
}

/// Mode d'exécution d'un service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    /// Service avec sa propre fenêtre (processus standalone).
    #[default]
    Standalone,
    /// Service sans UI (processus d'arrière-plan).
    Background,
}

/// Binaires spécifiques par plateforme dans le package.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformBinaries {
    #[serde(default)]
    pub windows_x64: Option<String>,
    #[serde(default)]
    pub linux_x64: Option<String>,
    #[serde(default)]
    pub macos_arm64: Option<String>,
}

/// Manifeste complet d'un service.
///
/// Fichier : `service.manifest.json` dans le package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceManifest {
    /// Identifiant unique du service (ex: "jaymanga", "jayxpose").
    pub id: String,
    /// Nom d'affichage (ex: "JayManga").
    pub name: String,
    /// Description courte.
    pub description: String,
    /// Icone (emoji ou chemin vers asset).
    pub icon: String,
    /// Version semver (ex: "0.1.0").
    pub version: String,
    /// Type de service.
    pub service_type: ServiceType,
    /// Source officielle ou communautaire.
    pub source: ServiceSource,
    /// Développeur / éditeur.
    pub developer: String,
    /// Version minimale du Central requise.
    pub min_central_version: String,
    /// Dépendances vers d'autres services (par ID).
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Permissions requises (accès réseau, stockage, etc.).
    #[serde(default)]
    pub permissions: Vec<ServicePermission>,
    /// Tags pour la recherche dans le Market.
    #[serde(default)]
    pub tags: Vec<String>,
    /// URL de la page d'accueil / documentation du service.
    #[serde(default)]
    pub homepage: Option<String>,
    /// Hash SHA-256 du package (rempli par Origin après upload).
    #[serde(default)]
    pub checksum: Option<String>,
    /// Taille du package en octets (rempli par Origin après upload).
    #[serde(default)]
    pub package_size: Option<u64>,
    /// Mode d'exécution (standalone = fenêtre propre, background = sans UI).
    #[serde(default)]
    pub execution_mode: ExecutionMode,
    /// Nom du binaire dans le package (ex: "jayxpose.exe"). Déduit de l'id si absent.
    #[serde(default)]
    pub binary_name: Option<String>,
    /// Binaires par plateforme (chemins relatifs dans le package).
    #[serde(default)]
    pub platforms: PlatformBinaries,
}

/// Permission demandée par un service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServicePermission {
    /// Accès au réseau Webway (MWS).
    Webway,
    /// Accès au stockage local (base de données).
    Storage,
    /// Accès à l'identité de l'utilisateur (profil Central).
    Identity,
    /// Accès à la comptabilité (JayKonta).
    Accounting,
    /// Accès au calendrier (JayKoa).
    Calendar,
    /// Notification utilisateur.
    Notifications,
}

impl ServiceManifest {
    /// Valide que le manifeste est cohérent.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.id.is_empty() {
            return Err(ManifestError::MissingField("id".into()));
        }
        if self.id.len() > 64
            || !self
                .id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(ManifestError::InvalidId(self.id.clone()));
        }
        if self.name.is_empty() {
            return Err(ManifestError::MissingField("name".into()));
        }
        if self.version.is_empty() {
            return Err(ManifestError::MissingField("version".into()));
        }
        if self.developer.is_empty() {
            return Err(ManifestError::MissingField("developer".into()));
        }
        Ok(())
    }
}

/// Erreurs de validation du manifeste.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("champ requis manquant : {0}")]
    MissingField(String),
    #[error("identifiant invalide : '{0}' (alphanumérique, _ ou - uniquement, max 64 car.)")]
    InvalidId(String),
    #[error("erreur de parsing JSON : {0}")]
    ParseError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_validation() {
        let valid = ServiceManifest {
            id: "jaymanga".into(),
            name: "JayManga".into(),
            description: "Manga reader".into(),
            icon: "📚".into(),
            version: "0.1.0".into(),
            service_type: ServiceType::SurfaceWeb,
            source: ServiceSource::Officiel,
            developer: "Miyukini".into(),
            min_central_version: "0.1.0".into(),
            dependencies: vec![],
            permissions: vec![ServicePermission::Storage],
            tags: vec!["manga".into(), "lecture".into()],
            homepage: None,
            checksum: None,
            package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: None,
            platforms: PlatformBinaries::default(),
        };
        assert!(valid.validate().is_ok());

        let mut invalid = valid.clone();
        invalid.id = String::new();
        assert!(invalid.validate().is_err());

        let mut invalid = valid;
        invalid.id = "has spaces!".into();
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_manifest_serde_roundtrip() {
        let manifest = ServiceManifest {
            id: "jayxpose".into(),
            name: "JayXpose".into(),
            description: "Profil exposant".into(),
            icon: "🏪".into(),
            version: "0.2.0".into(),
            service_type: ServiceType::SurfaceWeb,
            source: ServiceSource::Officiel,
            developer: "Miyukini".into(),
            min_central_version: "0.1.0".into(),
            dependencies: vec![],
            permissions: vec![ServicePermission::Storage, ServicePermission::Webway],
            tags: vec!["commerce".into()],
            homepage: Some("https://miyukini.com/jayxpose".into()),
            checksum: None,
            package_size: None,
            execution_mode: ExecutionMode::Standalone,
            binary_name: Some("jayxpose.exe".into()),
            platforms: PlatformBinaries {
                windows_x64: Some("jayxpose.exe".into()),
                ..Default::default()
            },
        };
        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let parsed: ServiceManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, manifest.id);
        assert_eq!(parsed.permissions.len(), 2);
    }
}
