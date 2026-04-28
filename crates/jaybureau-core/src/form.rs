//! Types pour Jay Formulaire (questionnaires collaboratifs).

use crate::UserId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Schéma d'un formulaire.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSchema {
    pub title: String,
    pub description: String,
    pub fields: Vec<FormField>,
    /// Confirmation affichée après soumission.
    pub thank_you_message: String,
    /// Formulaire accepte-t-il les réponses anonymes ?
    pub allow_anonymous: bool,
    /// Collecter les emails des répondants.
    pub collect_email: bool,
}

/// Un champ du formulaire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub description: String,
    pub required: bool,
    pub kind: FormFieldKind,
}

impl FormField {
    pub fn new(label: impl Into<String>, kind: FormFieldKind) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            label: label.into(),
            description: String::new(),
            required: false,
            kind,
        }
    }
}

/// Type de champ.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FormFieldKind {
    /// Texte court (1 ligne).
    ShortText,
    /// Texte long (multi-lignes).
    LongText,
    /// Nombre.
    Number { min: Option<f64>, max: Option<f64> },
    /// Email.
    Email,
    /// Date.
    Date,
    /// Heure.
    Time,
    /// Choix unique parmi N options.
    Radio { options: Vec<String> },
    /// Choix multiples.
    Checkbox { options: Vec<String> },
    /// Menu déroulant.
    Dropdown { options: Vec<String> },
    /// Échelle de 1 à N.
    Scale {
        min: u8,
        max: u8,
        min_label: Option<String>,
        max_label: Option<String>,
    },
    /// Upload de fichier (stocké dans Jay Drive).
    File { max_size_mb: u32 },
}

/// Réponse à un formulaire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormResponse {
    pub id: String,
    pub form_id: String,
    pub respondent_id: Option<UserId>,
    pub respondent_email: Option<String>,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub answers: Vec<FormAnswer>,
}

/// Réponse à un champ.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormAnswer {
    pub field_id: String,
    #[serde(flatten)]
    pub value: FormAnswerValue,
}

/// Valeur d'une réponse.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FormAnswerValue {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "number")]
    Number(f64),
    #[serde(rename = "date")]
    Date(chrono::NaiveDate),
    #[serde(rename = "time")]
    Time(chrono::NaiveTime),
    #[serde(rename = "choice")]
    Choice(String),
    #[serde(rename = "choices")]
    Choices(Vec<String>),
    #[serde(rename = "scale")]
    Scale(u8),
    #[serde(rename = "file")]
    File { drive_file_id: String },
}

impl FormSchema {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            fields: Vec::new(),
            thank_you_message: "Merci pour votre réponse !".into(),
            allow_anonymous: true,
            collect_email: false,
        }
    }

    pub fn add_field(&mut self, field: FormField) -> &mut Self {
        self.fields.push(field);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_form() {
        let mut form = FormSchema::new("Sondage");
        form.add_field(FormField::new("Votre nom", FormFieldKind::ShortText));
        form.add_field(FormField::new(
            "Votre âge",
            FormFieldKind::Number { min: Some(0.0), max: Some(120.0) },
        ));
        assert_eq!(form.fields.len(), 2);
    }
}
