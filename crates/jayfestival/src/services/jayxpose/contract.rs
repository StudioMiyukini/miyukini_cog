//! Contrat JayXpose — types entrée/sortie pour fiche exposant et répertoire.
//!
//! @id: jayxpose_profile_type
//! @do: type de sortie pour le profil exposant (fiche complète, répertoire).
//! @layer: domain
//!
//! @id: jayxpose_repertoire_filters
//! @do: filtres d'entrée pour la liste répertoire (secteur, pagination).
//! @layer: domain

use crate::supabase::Exposant;

/// Profil exposant (type de sortie JayXpose).
/// Fiche complète : identité, contact, vitrine, visibilité répertoire.
///
/// @id: jayxpose_profile_type
/// @do: type de sortie pour le profil exposant (fiche complète, répertoire).
/// @layer: domain
#[derive(Debug, Clone, Default)]
pub struct JayXposeProfile {
    /// Identifiant unique exposant.
    pub id: Option<String>,
    /// Raison sociale / nom entreprise.
    pub company_name: Option<String>,
    /// Nom du stand.
    pub stand_name: Option<String>,
    /// Email de contact.
    pub contact_email: Option<String>,
    /// Téléphone de contact.
    pub contact_phone: Option<String>,
    /// Adresse postale.
    pub adresse: Option<String>,
    /// URL du logo.
    pub logo_url: Option<String>,
    /// URL du site web.
    pub site_web: Option<String>,
    /// Numéro SIRET.
    pub siret: Option<String>,
    /// Secteur d'activité.
    pub secteur: Option<String>,
    /// Catégorie exposant.
    pub category: Option<String>,
    /// Description / présentation.
    pub description: Option<String>,
    /// Visible dans le répertoire public.
    pub visible_repertoire: Option<bool>,
    /// Date de création (ISO).
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    pub updated_at: Option<String>,
}

impl From<Exposant> for JayXposeProfile {
    fn from(e: Exposant) -> Self {
        JayXposeProfile {
            id: e.id,
            company_name: e.company_name,
            stand_name: e.stand_name,
            contact_email: e.contact_email,
            contact_phone: e.contact_phone,
            adresse: e.adresse,
            logo_url: e.logo_url,
            site_web: e.site_web,
            siret: e.siret,
            secteur: e.secteur,
            category: e.category,
            description: e.description,
            visible_repertoire: e.visible_repertoire,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

/// Item léger pour la liste répertoire (catalogue).
#[derive(Debug, Clone, Default)]
pub struct RepertoireItem {
    /// Identifiant exposant.
    pub id: Option<String>,
    /// Raison sociale.
    pub company_name: Option<String>,
    /// Secteur.
    pub secteur: Option<String>,
    /// Catégorie.
    pub category: Option<String>,
    /// URL logo.
    pub logo_url: Option<String>,
    /// URL site web.
    pub site_web: Option<String>,
    /// Description courte.
    pub description: Option<String>,
}

impl From<Exposant> for RepertoireItem {
    fn from(e: Exposant) -> Self {
        RepertoireItem {
            id: e.id,
            company_name: e.company_name,
            secteur: e.secteur,
            category: e.category,
            logo_url: e.logo_url,
            site_web: e.site_web,
            description: e.description,
        }
    }
}

/// Filtres d'entrée pour la liste répertoire (catalogue exposants).
///
/// @id: jayxpose_repertoire_filters
/// @do: filtres d'entrée pour la liste répertoire (secteur, pagination).
/// @layer: domain
#[derive(Debug, Clone, Default)]
pub struct RepertoireFilters {
    /// Filtre par secteur (catégorie d'activité).
    pub secteur: Option<String>,
    /// Nombre max d'éléments (pagination).
    pub limit: Option<u32>,
    /// Décalage (pagination).
    pub offset: Option<u32>,
}
