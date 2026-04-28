//! Section coordonnées — adresses, téléphones, emails, sites web.
//!
//! Chaque entrée porte un label (personnel, professionnel, urgence…)
//! et un flag `primary` pour indiquer le contact principal.

use serde::{Deserialize, Serialize};

/// Type de coordonnée.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContactType {
    #[default]
    Personal,
    Professional,
    Emergency,
    Billing,
    Delivery,
    Other,
}

/// Adresse postale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostalAddress {
    pub label: ContactType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    pub line1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line3: Option<String>,
    pub postal_code: String,
    pub city: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// Code ISO 3166-1 alpha-2.
    pub country: String,
    /// Adresse principale ?
    #[serde(default)]
    pub primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Type de numéro de téléphone.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhoneType {
    #[default]
    Mobile,
    Landline,
    Fax,
    Work,
    Other,
}

/// Numéro de téléphone.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneEntry {
    pub label: ContactType,
    pub phone_type: PhoneType,
    /// Format international recommandé : +33 6 12 34 56 78.
    pub number: String,
    #[serde(default)]
    pub primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Adresse email.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailEntry {
    pub label: ContactType,
    pub address: String,
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Site web ou réseau social.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebPresence {
    pub platform: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Section coordonnées complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<PostalAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<PhoneEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<EmailEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web_presences: Vec<WebPresence>,
}
