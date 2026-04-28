//! Jay Bureau — types partagés entre toutes les apps bureautiques.
//!
//! Chaque document (Doc, Sheet, Slide, Form, Meeting) possède :
//! - Une identité unique (`DocId`)
//! - Un propriétaire et une liste de collaborateurs (ACL)
//! - Un état CRDT synchronisé (via jay-collab)
//! - Des métadonnées (titre, timestamps, etc.)

pub mod doc;
pub mod form;
pub mod meeting;
pub mod permission;
pub mod sheet;
pub mod slide;

pub use doc::{DocBlock, DocContent, DocumentMeta};
pub use form::{FormField, FormFieldKind, FormResponse, FormSchema};
pub use meeting::{Meeting, MeetingParticipant, MeetingStatus};
pub use permission::{Access, AclEntry, DocumentAcl, Role};
pub use sheet::{Cell, CellValue, SheetData};
pub use slide::{Slide, SlideDeck, SlideElement};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifiant unique d'un document (tout type).
pub type DocId = String;

/// Identifiant d'un utilisateur (profile id Miyukini Connect).
pub type UserId = String;

/// Type de document dans Jay Bureau.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocKind {
    Doc,
    Sheet,
    Slide,
    Form,
    Meeting,
    /// JayClub — réseau social.
    Club,
    /// Jay Mail — client email.
    Mail,
    /// Jay Message — messagerie chiffrée E2E.
    Message,
}

impl DocKind {
    /// Libellé d'affichage.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Doc => "Jay Docs",
            Self::Sheet => "Jay Sheets",
            Self::Slide => "Jay Slides",
            Self::Form => "Jay Formulaire",
            Self::Meeting => "Jay Réunion",
            Self::Club => "JayClub",
            Self::Mail => "Jay Mail",
            Self::Message => "Jay Message",
        }
    }

    /// Icône emoji.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Doc => "📄",
            Self::Sheet => "📊",
            Self::Slide => "🎞",
            Self::Form => "📝",
            Self::Meeting => "🎥",
            Self::Club => "🌐",
            Self::Mail => "✉️",
            Self::Message => "🔒",
        }
    }

    /// Toutes les variantes pour itération.
    pub fn all() -> &'static [DocKind] {
        &[
            DocKind::Doc,
            DocKind::Sheet,
            DocKind::Slide,
            DocKind::Form,
            DocKind::Meeting,
            DocKind::Club,
            DocKind::Mail,
            DocKind::Message,
        ]
    }

    /// Nom du binaire associé.
    pub fn binary_name(&self) -> &'static str {
        match self {
            Self::Doc => "jay-docs",
            Self::Sheet => "jay-sheets",
            Self::Slide => "jay-slides",
            Self::Form => "jay-formulaire",
            Self::Meeting => "jay-reunion",
            Self::Club => "jay-club",
            Self::Mail => "jay-mail",
            Self::Message => "jay-message",
        }
    }
}

/// Métadonnées communes à tous les documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBase {
    pub id: DocId,
    pub kind: DocKind,
    pub title: String,
    pub owner_id: UserId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub acl: DocumentAcl,
    /// Emplacement dans Jay Drive (si rangé dans un dossier).
    pub folder_id: Option<String>,
    /// Taille estimée en octets (pour affichage).
    pub size_bytes: u64,
}

impl DocumentBase {
    /// Crée un nouveau document vide.
    pub fn new(kind: DocKind, title: impl Into<String>, owner_id: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        let owner = owner_id.into();
        Self {
            id: Uuid::new_v4().to_string(),
            kind,
            title: title.into(),
            owner_id: owner.clone(),
            created_at: now,
            updated_at: now,
            acl: DocumentAcl::owner_only(owner),
            folder_id: None,
            size_bytes: 0,
        }
    }
}

/// Événement de collaboration dans un document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CollabEvent {
    /// Mise à jour de l'état CRDT (binaire Yrs).
    #[serde(rename = "update")]
    Update {
        #[serde(with = "hex_bytes")]
        update: Vec<u8>,
    },
    /// Présence d'un utilisateur (nouvel arrivant ou mise à jour de son état).
    #[serde(rename = "presence")]
    Presence { user_id: UserId, presence: Presence },
    /// Un utilisateur a quitté le document.
    #[serde(rename = "leave")]
    Leave { user_id: UserId },
    /// Erreur ou message système.
    #[serde(rename = "error")]
    Error { message: String },
}

/// État de présence d'un utilisateur (curseur, sélection, couleur).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Presence {
    /// Pseudo/email affiché.
    pub display_name: String,
    /// Couleur assignée à l'utilisateur (pour curseur/highlight).
    pub color: String,
    /// Position du curseur (selon le type de doc : offset texte, cellule, slide index...).
    pub cursor: Option<CursorPos>,
    /// Sélection actuelle.
    pub selection: Option<CursorRange>,
}

/// Position de curseur (générique, interprétée selon le type de document).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CursorPos {
    /// Offset dans un texte plat (Jay Docs).
    #[serde(rename = "text")]
    Text { offset: usize },
    /// Cellule de feuille (Jay Sheets).
    #[serde(rename = "cell")]
    Cell { row: u32, col: u32 },
    /// Index de slide + élément (Jay Slides).
    #[serde(rename = "slide")]
    Slide {
        slide_index: usize,
        element_id: Option<String>,
    },
}

/// Plage de sélection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorRange {
    pub start: CursorPos,
    pub end: CursorPos,
}

/// Erreurs du module.
#[derive(Debug, thiserror::Error)]
pub enum JayBureauError {
    #[error("Document introuvable: {0}")]
    NotFound(DocId),
    #[error("Permission refusée")]
    PermissionDenied,
    #[error("Format invalide: {0}")]
    InvalidFormat(String),
    #[error("Erreur I/O: {0}")]
    Io(String),
}

/// Helper Serde pour encoder Vec<u8> en hex.
mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex_encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(d)?;
        hex_decode(&s).map_err(serde::de::Error::custom)
    }

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8] = b"0123456789abcdef";
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0xf) as usize] as char);
        }
        out
    }

    fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
        if s.len() % 2 != 0 {
            return Err("hex length must be even".into());
        }
        let mut out = Vec::with_capacity(s.len() / 2);
        for i in (0..s.len()).step_by(2) {
            let h = u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| format!("hex: {e}"))?;
            out.push(h);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_kind_all_have_unique_binary_names() {
        let mut names: Vec<&str> = DocKind::all().iter().map(|k| k.binary_name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), DocKind::all().len());
    }

    #[test]
    fn document_base_new_has_owner_in_acl() {
        let doc = DocumentBase::new(DocKind::Doc, "Test", "user-123");
        assert_eq!(doc.acl.owner, "user-123");
    }

    #[test]
    fn collab_event_update_serialize() {
        let evt = CollabEvent::Update { update: vec![1, 2, 3, 4] };
        let json = serde_json::to_string(&evt).unwrap();
        assert!(json.contains("01020304"));
    }
}
