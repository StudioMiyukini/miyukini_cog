//! Types domaine JayFestival (profiles, editions, organisateurs, exposants).
//! Alignés sur la référence base de données ; persistance via KindMother (SQLite fille).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Profil utilisateur (table `profiles`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    pub id: Option<String>,
    pub username: Option<String>,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub theme: Option<String>,
    /// Hash du mot de passe (stockage local SQLite, jamais exposé).
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Type d'utilisateur (aligné `profiles.user_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    Admin,
    Manager,
    Exhibitor,
    Volunteer,
    Visitor,
    Unknown,
}

impl UserType {
    #[must_use] 
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

    #[must_use] 
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

/// Édition / événement (table `editions`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Edition {
    pub id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
    pub theme: Option<String>,
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Organisateur (table `organisateurs`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Organisateur {
    pub id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub region: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub website: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Exposant (table `exposants`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Exposant {
    pub id: Option<String>,
    pub company_name: Option<String>,
    pub stand_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub adresse: Option<String>,
    pub logo_url: Option<String>,
    pub site_web: Option<String>,
    pub siret: Option<String>,
    pub secteur: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub visible_repertoire: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Participation exposant × édition (table `editions_exposants`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EditionExposant {
    pub id: Option<String>,
    pub exposant_id: Option<String>,
    pub edition_id: Option<String>,
    pub is_accepted: Option<bool>,
    pub is_validated: Option<bool>,
    pub is_paid: Option<bool>,
    pub assigned_stand: Option<String>,
    pub size_meters: Option<f64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}
