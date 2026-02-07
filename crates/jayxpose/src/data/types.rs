//! Types domaine JayXpose (profil exposant, catalogue, visuels, documents, vitrine, confidentialité).
//!
//! Alignés sur la référence base de données ; persistance via KindMother (SQLite fille).
//!
//! @id: jayxpose_data_types
//! @do: define_domain_types_jayxpose
//! @layer: domain

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Structures de données principales
// ---------------------------------------------------------------------------

/// Profil exposant enrichi (table `exposants`).
///
/// @id: exposant_profile_type
/// @do: represent_exposant_profile_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExposantProfile {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Raison sociale / nom entreprise.
    pub company_name: Option<String>,
    /// Forme juridique (SARL, SAS, auto-entrepreneur…).
    pub legal_form: Option<String>,
    /// Slogan / accroche courte.
    pub slogan: Option<String>,
    /// Description courte (résumé annuaire).
    pub description_short: Option<String>,
    /// Description longue (vitrine).
    pub description_long: Option<String>,
    /// Nom du stand.
    pub stand_name: Option<String>,
    /// Email de contact principal.
    pub contact_email: Option<String>,
    /// Téléphone de contact principal.
    pub contact_phone: Option<String>,
    /// Adresse du siège (JSON sérialisé).
    pub adresse_siege: Option<String>,
    /// Adresse de correspondance (JSON sérialisé).
    pub adresse_correspondance: Option<String>,
    /// Nom du contact facturation.
    pub contact_facturation_nom: Option<String>,
    /// Email du contact facturation.
    pub contact_facturation_email: Option<String>,
    /// Téléphone du contact facturation.
    pub contact_facturation_phone: Option<String>,
    /// Nom du contact logistique.
    pub contact_logistique_nom: Option<String>,
    /// Email du contact logistique.
    pub contact_logistique_email: Option<String>,
    /// Téléphone du contact logistique.
    pub contact_logistique_phone: Option<String>,
    /// URL du logo.
    pub logo_url: Option<String>,
    /// URL de la bannière.
    pub banner_url: Option<String>,
    /// Site web de l'entreprise.
    pub site_web: Option<String>,
    /// Numéro SIRET.
    pub siret: Option<String>,
    /// Numéro SIREN.
    pub siren: Option<String>,
    /// Code APE / NAF.
    pub code_ape: Option<String>,
    /// Numéro d'immatriculation (RCS, RM…).
    pub num_immatriculation: Option<String>,
    /// Secteur d'activité.
    pub secteur: Option<String>,
    /// Tags / mots-clés (JSON sérialisé).
    pub tags: Option<String>,
    /// Lien Facebook.
    pub social_facebook: Option<String>,
    /// Lien Instagram.
    pub social_instagram: Option<String>,
    /// Lien LinkedIn.
    pub social_linkedin: Option<String>,
    /// Lien TikTok.
    pub social_tiktok: Option<String>,
    /// Lien YouTube.
    pub social_youtube: Option<String>,
    /// Lien Pinterest.
    pub social_pinterest: Option<String>,
    /// Lien X (ex-Twitter).
    pub social_x: Option<String>,
    /// Visible dans l'annuaire public.
    pub visible_annuaire: Option<bool>,
    /// Slug vitrine (URL friendly).
    pub vitrine_slug: Option<String>,
    /// Statut vitrine (brouillon/publiee/suspendue).
    pub vitrine_status: Option<String>,
    /// Couleurs vitrine personnalisées (JSON).
    pub vitrine_colors: Option<String>,
    /// Titre SEO.
    pub seo_title: Option<String>,
    /// Description SEO.
    pub seo_description: Option<String>,
    /// Mots-clés SEO.
    pub seo_keywords: Option<String>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de dernière mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Produit du catalogue exposant (table `produits_catalogue`).
///
/// @id: produit_catalogue_type
/// @do: represent_product_catalogue_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProduitCatalogue {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Nom du produit.
    pub name: Option<String>,
    /// Description du produit.
    pub description: Option<String>,
    /// Prix du produit.
    pub price: Option<f64>,
    /// Devise (EUR, USD…).
    pub currency: Option<String>,
    /// Identifiant de la catégorie.
    pub category_id: Option<String>,
    /// Disponibilité (disponible/rupture/sur_commande).
    pub availability: Option<String>,
    /// Produit mis en avant.
    pub is_featured: Option<bool>,
    /// Ordre de tri.
    pub sort_order: Option<i32>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de dernière mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Catégorie de produit (table `categories_produits`).
///
/// @id: categorie_produit_type
/// @do: represent_product_category_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategorieProduit {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Nom de la catégorie.
    pub name: Option<String>,
    /// Description de la catégorie.
    pub description: Option<String>,
    /// Ordre de tri.
    pub sort_order: Option<i32>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Visuel associé à un produit (table `produits_visuels`).
///
/// @id: produit_visuel_type
/// @do: represent_product_visual_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProduitVisuel {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant du produit associé.
    pub produit_id: Option<String>,
    /// URL du visuel.
    pub url: Option<String>,
    /// Texte alternatif (accessibilité).
    pub alt_text: Option<String>,
    /// Visuel principal (couverture).
    pub is_primary: Option<bool>,
    /// Ordre de tri.
    pub sort_order: Option<i32>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Document professionnel (table `documents_professionnels`).
///
/// @id: document_professionnel_type
/// @do: represent_professional_document_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DocumentProfessionnel {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Type de document (rib/assurance/kbis/…).
    pub doc_type: Option<String>,
    /// Libellé du document.
    pub label: Option<String>,
    /// URL du fichier.
    pub file_url: Option<String>,
    /// Nom du fichier original.
    pub file_name: Option<String>,
    /// Taille du fichier en octets.
    pub file_size: Option<i64>,
    /// Type MIME.
    pub mime_type: Option<String>,
    /// Statut du document (en_attente/valide/expire/rejete).
    pub status: Option<String>,
    /// Date d'expiration.
    pub expires_at: Option<String>,
    /// Numéro de version.
    pub version: Option<i32>,
    /// Notes internes.
    pub notes: Option<String>,
    /// Raison de rejet (si rejete).
    pub rejection_reason: Option<String>,
    /// Date d'upload.
    pub uploaded_at: Option<String>,
    /// Date de validation.
    pub validated_at: Option<String>,
    /// Identifiant du validateur.
    pub validated_by: Option<String>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de dernière mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Version d'un document professionnel (table `documents_versions`).
///
/// @id: document_version_type
/// @do: represent_document_version_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DocumentVersion {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant du document parent.
    pub document_id: Option<String>,
    /// Numéro de version.
    pub version: Option<i32>,
    /// URL du fichier versionné.
    pub file_url: Option<String>,
    /// Nom du fichier versionné.
    pub file_name: Option<String>,
    /// Taille du fichier en octets.
    pub file_size: Option<i64>,
    /// Date d'upload de cette version.
    pub uploaded_at: Option<String>,
}

/// Partage d'un document professionnel (table `documents_partages`).
///
/// @id: document_partage_type
/// @do: represent_document_sharing_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DocumentPartage {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant du document partagé.
    pub document_id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Identifiant de l'utilisateur cible.
    pub target_user_id: Option<String>,
    /// Type de contexte cible (candidature/edition/administratif).
    pub target_context_type: Option<String>,
    /// Identifiant du contexte cible.
    pub target_context_id: Option<String>,
    /// Statut du partage (demande/accepte/refuse/revoque/expire).
    pub status: Option<String>,
    /// Message accompagnant la demande.
    pub message: Option<String>,
    /// Date de la demande.
    pub requested_at: Option<String>,
    /// Date de réponse.
    pub responded_at: Option<String>,
    /// Date de révocation.
    pub revoked_at: Option<String>,
    /// Date d'expiration.
    pub expires_at: Option<String>,
}

/// Page de vitrine exposant (table `vitrine_pages`).
///
/// @id: vitrine_page_type
/// @do: represent_showcase_page_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VitrinePage {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Type de page (accueil/presentation/contact).
    pub page_type: Option<String>,
    /// Contenu de la page (JSON).
    pub content: Option<String>,
    /// Page visible publiquement.
    pub is_visible: Option<bool>,
    /// Ordre de tri.
    pub sort_order: Option<i32>,
    /// Date de dernière mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Règle de confidentialité pour un champ du profil exposant (table `confidentialite_profil`).
///
/// @id: confidentialite_profil_type
/// @do: represent_profile_privacy_rule_data
/// @layer: domain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfidentialiteProfil {
    /// Identifiant unique (UUID v4).
    pub id: Option<String>,
    /// Identifiant de l'exposant propriétaire.
    pub exposant_id: Option<String>,
    /// Nom du champ protégé.
    pub field_name: Option<String>,
    /// Visibilité (public/prive/service_only).
    pub visibility: Option<String>,
    /// Date de dernière mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Enums utilitaires
// ---------------------------------------------------------------------------

/// Type de document professionnel.
///
/// @id: doc_type_enum
/// @do: enumerate_professional_document_types
/// @layer: domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocType {
    /// Relevé d'identité bancaire.
    Rib,
    /// Attestation d'assurance.
    Assurance,
    /// Extrait Kbis.
    Kbis,
    /// Immatriculation (RCS, RM…).
    Immatriculation,
    /// Licence (débit de boissons, etc.).
    Licence,
    /// Attestation URSSAF.
    Urssaf,
    /// Carte professionnelle.
    CartePro,
    /// Diplôme / certification.
    Diplome,
    /// Certificat sanitaire.
    Sanitaire,
    /// Autre document.
    Autre,
}

impl DocType {
    /// Représentation texte (snake_case, alignée DB).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rib => "rib",
            Self::Assurance => "assurance",
            Self::Kbis => "kbis",
            Self::Immatriculation => "immatriculation",
            Self::Licence => "licence",
            Self::Urssaf => "urssaf",
            Self::CartePro => "carte_pro",
            Self::Diplome => "diplome",
            Self::Sanitaire => "sanitaire",
            Self::Autre => "autre",
        }
    }

    /// Parse depuis une chaîne (insensible à la casse).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rib" => Self::Rib,
            "assurance" => Self::Assurance,
            "kbis" => Self::Kbis,
            "immatriculation" => Self::Immatriculation,
            "licence" => Self::Licence,
            "urssaf" => Self::Urssaf,
            "carte_pro" => Self::CartePro,
            "diplome" => Self::Diplome,
            "sanitaire" => Self::Sanitaire,
            _ => Self::Autre,
        }
    }
}

/// Statut d'un document professionnel.
///
/// @id: doc_status_enum
/// @do: enumerate_document_statuses
/// @layer: domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocStatus {
    /// En attente de validation.
    EnAttente,
    /// Validé.
    Valide,
    /// Expiré.
    Expire,
    /// Rejeté.
    Rejete,
}

impl DocStatus {
    /// Représentation texte (snake_case, alignée DB).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnAttente => "en_attente",
            Self::Valide => "valide",
            Self::Expire => "expire",
            Self::Rejete => "rejete",
        }
    }

    /// Parse depuis une chaîne (insensible à la casse).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en_attente" => Self::EnAttente,
            "valide" => Self::Valide,
            "expire" => Self::Expire,
            "rejete" => Self::Rejete,
            _ => Self::EnAttente,
        }
    }
}

/// Statut d'une vitrine exposant.
///
/// @id: vitrine_status_enum
/// @do: enumerate_showcase_statuses
/// @layer: domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VitrineStatus {
    /// Brouillon (non publié).
    Brouillon,
    /// Publiée (visible publiquement).
    Publiee,
    /// Suspendue (désactivée temporairement).
    Suspendue,
}

impl VitrineStatus {
    /// Représentation texte (snake_case, alignée DB).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Brouillon => "brouillon",
            Self::Publiee => "publiee",
            Self::Suspendue => "suspendue",
        }
    }

    /// Parse depuis une chaîne (insensible à la casse).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "brouillon" => Self::Brouillon,
            "publiee" => Self::Publiee,
            "suspendue" => Self::Suspendue,
            _ => Self::Brouillon,
        }
    }
}

/// Disponibilité d'un produit catalogue.
///
/// @id: availability_enum
/// @do: enumerate_product_availability_statuses
/// @layer: domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    /// Disponible à la vente.
    Disponible,
    /// En rupture de stock.
    Rupture,
    /// Sur commande uniquement.
    SurCommande,
}

impl Availability {
    /// Représentation texte (snake_case, alignée DB).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disponible => "disponible",
            Self::Rupture => "rupture",
            Self::SurCommande => "sur_commande",
        }
    }

    /// Parse depuis une chaîne (insensible à la casse).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "disponible" => Self::Disponible,
            "rupture" => Self::Rupture,
            "sur_commande" => Self::SurCommande,
            _ => Self::Disponible,
        }
    }
}

/// Visibilité d'un champ de profil exposant (confidentialité).
///
/// @id: visibility_enum
/// @do: enumerate_profile_field_visibility_levels
/// @layer: domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// Visible publiquement.
    Public,
    /// Privé (visible uniquement par l'exposant).
    Prive,
    /// Visible uniquement par les services internes.
    ServiceOnly,
}

impl Visibility {
    /// Représentation texte (snake_case, alignée DB).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Prive => "prive",
            Self::ServiceOnly => "service_only",
        }
    }

    /// Parse depuis une chaîne (insensible à la casse).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "public" => Self::Public,
            "prive" => Self::Prive,
            "service_only" => Self::ServiceOnly,
            _ => Self::Public,
        }
    }
}
