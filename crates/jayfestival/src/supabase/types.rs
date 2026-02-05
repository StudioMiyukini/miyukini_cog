//! Types Supabase alignés sur la Reference Base de Donnees JayFestival.
//!
//! @id: supabase_profiles
//! @do: structs et sérialisation pour la table profiles (utilisateurs, user_type).
//! @layer: domain
//!
//! @id: supabase_editions
//! @do: structs et sérialisation pour la table editions (événements, éditions).
//! @layer: domain
//!
//! @id: supabase_exposants
//! @do: structs et sérialisation pour la table exposants (annuaire exposants).
//! @layer: domain
//!
//! @id: supabase_editions_exposants
//! @do: structs et sérialisation pour la table editions_exposants (participations).
//! @layer: domain

use serde::{Deserialize, Serialize};

/// Profil utilisateur (table `profiles`).
/// Rôles via `user_type` : admin, manager, exhibitor, volunteer, visitor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    /// Identifiant utilisateur (UUID).
    pub id: Option<String>,
    /// Nom d'affichage.
    pub username: Option<String>,
    /// Rôle : admin, manager, exhibitor, volunteer, visitor.
    pub user_type: Option<String>,
    /// Email.
    pub email: Option<String>,
    /// URL avatar.
    pub avatar_url: Option<String>,
    /// Préférence thème (light/dark).
    pub theme: Option<String>,
    /// Date de création (ISO).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Type d'utilisateur (proxy rôles alpha, aligné `profiles.user_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    /// Administrateur.
    Admin,
    /// Gestionnaire / organisateur.
    Manager,
    /// Exposant.
    Exhibitor,
    /// Bénévole.
    Volunteer,
    /// Visiteur.
    Visitor,
    /// Type inconnu ou non défini.
    Unknown,
}

impl UserType {
    /// Valeur stockée en base (snake_case).
    pub fn as_str(self) -> &'static str {
        match self {
            UserType::Admin => "admin",
            UserType::Manager => "manager",
            UserType::Exhibitor => "exhibitor",
            UserType::Volunteer => "volunteer",
            UserType::Visitor => "visitor",
            UserType::Unknown => "unknown",
        }
    }

    /// Parse depuis la chaîne en base.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => UserType::Admin,
            "manager" => UserType::Manager,
            "exhibitor" => UserType::Exhibitor,
            "volunteer" => UserType::Volunteer,
            "visitor" => UserType::Visitor,
            _ => UserType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Edition, EditionExposant, Exposant, Organisateur, Profile, UserType};

    #[test]
    fn user_type_from_str_roundtrip() {
        for (s, expected) in [
            ("admin", UserType::Admin),
            ("manager", UserType::Manager),
            ("exhibitor", UserType::Exhibitor),
            ("volunteer", UserType::Volunteer),
            ("visitor", UserType::Visitor),
            ("ADMIN", UserType::Admin),
            ("Unknown", UserType::Unknown),
            ("", UserType::Unknown),
            ("other", UserType::Unknown),
        ] {
            assert_eq!(UserType::from_str(s), expected, "from_str({:?})", s);
        }
    }

    #[test]
    fn user_type_as_str_roundtrip() {
        for ut in [
            UserType::Admin,
            UserType::Manager,
            UserType::Exhibitor,
            UserType::Volunteer,
            UserType::Visitor,
            UserType::Unknown,
        ] {
            assert_eq!(UserType::from_str(ut.as_str()), ut, "as_str roundtrip {:?}", ut);
        }
    }

    #[test]
    fn profile_default() {
        let p = Profile::default();
        assert!(p.id.is_none());
        assert!(p.username.is_none());
        assert!(p.user_type.is_none());
        assert!(p.email.is_none());
        assert!(p.avatar_url.is_none());
        assert!(p.theme.is_none());
    }

    #[test]
    fn edition_default() {
        let e = Edition::default();
        assert!(e.id.is_none());
        assert!(e.name.is_none());
        assert!(e.slug.is_none());
        assert!(e.start_date.is_none());
        assert!(e.end_date.is_none());
        assert!(e.location.is_none());
        assert!(e.theme.is_none());
        assert!(e.status.is_none());
    }

    #[test]
    fn organisateur_default() {
        let o = Organisateur::default();
        assert!(o.id.is_none());
        assert!(o.name.is_none());
        assert!(o.slug.is_none());
        assert!(o.region.is_none());
        assert!(o.description.is_none());
        assert!(o.contact_email.is_none());
        assert!(o.website.is_none());
    }

    #[test]
    fn exposant_default() {
        let e = Exposant::default();
        assert!(e.id.is_none());
        assert!(e.company_name.is_none());
        assert!(e.stand_name.is_none());
        assert!(e.contact_email.is_none());
        assert!(e.contact_phone.is_none());
        assert!(e.adresse.is_none());
        assert!(e.logo_url.is_none());
        assert!(e.siret.is_none());
        assert!(e.secteur.is_none());
        assert!(e.category.is_none());
        assert!(e.description.is_none());
        assert!(e.visible_repertoire.is_none());
    }

    #[test]
    fn edition_exposant_default() {
        let ee = EditionExposant::default();
        assert!(ee.id.is_none());
        assert!(ee.exposant_id.is_none());
        assert!(ee.edition_id.is_none());
        assert!(ee.is_accepted.is_none());
        assert!(ee.is_validated.is_none());
        assert!(ee.is_paid.is_none());
        assert!(ee.assigned_stand.is_none());
        assert!(ee.size_meters.is_none());
    }
}

/// Édition / événement (table `editions`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Edition {
    /// Identifiant édition (UUID).
    pub id: Option<String>,
    /// Nom de l'édition.
    pub name: Option<String>,
    /// Slug URL.
    pub slug: Option<String>,
    /// Date de début (ISO).
    pub start_date: Option<String>,
    /// Date de fin (ISO).
    pub end_date: Option<String>,
    /// Lieu.
    pub location: Option<String>,
    /// Thème / intitulé thématique.
    pub theme: Option<String>,
    /// Statut (draft, published, etc.).
    pub status: Option<String>,
    /// Date de création (ISO).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Organisateur / structure organisatrice (répertoire organisateurs).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Organisateur {
    /// Identifiant organisateur (UUID).
    pub id: Option<String>,
    /// Nom de la structure.
    pub name: Option<String>,
    /// Slug URL.
    pub slug: Option<String>,
    /// Région / zone géographique.
    pub region: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Email de contact.
    pub contact_email: Option<String>,
    /// URL du site web.
    pub website: Option<String>,
    /// Date de création (ISO).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Exposant (table `exposants`). Aligné Reference Base de Donnees JayXpose/JayFestival.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Exposant {
    /// Identifiant exposant (UUID).
    pub id: Option<String>,
    /// Raison sociale.
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
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Participation exposant × édition (table `editions_exposants`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EditionExposant {
    /// Identifiant de la liaison (UUID).
    pub id: Option<String>,
    /// Identifiant exposant.
    pub exposant_id: Option<String>,
    /// Identifiant édition.
    pub edition_id: Option<String>,
    /// Candidature acceptée.
    pub is_accepted: Option<bool>,
    /// Participation validée.
    pub is_validated: Option<bool>,
    /// Paiement effectué.
    pub is_paid: Option<bool>,
    /// Stand assigné (nom ou référence).
    pub assigned_stand: Option<String>,
    /// Surface en m².
    pub size_meters: Option<f64>,
    /// Date de création (ISO).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour (ISO).
    #[serde(default)]
    pub updated_at: Option<String>,
}
