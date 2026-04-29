//! Types pour Jay Docs (document texte riche).
//!
//! Le contenu temps-réel est géré par Yrs (dans jay-collab).
//! Ce module décrit la structure sérialisée (snapshot, export).

use serde::{Deserialize, Serialize};

/// Métadonnées spécifiques à un document Jay Docs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMeta {
    /// Nombre de mots (calculé).
    pub word_count: u32,
    /// Nombre de caractères.
    pub char_count: u32,
    /// Langue détectée (ISO 639-1).
    pub language: Option<String>,
}

/// Snapshot du contenu (pour export/restauration).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocContent {
    /// Blocs du document dans l'ordre.
    pub blocks: Vec<DocBlock>,
    pub meta: DocumentMeta,
}

/// Un bloc de contenu (paragraphe, titre, liste...).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum DocBlock {
    /// Paragraphe de texte.
    Paragraph { text: String, style: TextStyle },
    /// Titre (h1-h6).
    Heading {
        level: u8,
        text: String,
    },
    /// Élément de liste à puces.
    BulletItem {
        text: String,
        indent: u8,
    },
    /// Élément de liste numérotée.
    NumberedItem {
        text: String,
        number: u32,
        indent: u8,
    },
    /// Bloc de code.
    Code {
        text: String,
        language: Option<String>,
    },
    /// Image (référence à un fichier Jay Drive).
    Image {
        drive_file_id: String,
        alt: Option<String>,
        width: Option<u32>,
    },
    /// Séparateur horizontal.
    Divider,
    /// Citation.
    Quote { text: String },
    /// Tableau simple (M lignes × N colonnes).
    Table {
        rows: Vec<Vec<String>>,
        has_header: bool,
    },
}

/// Style d'un paragraphe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    /// Alignement (left, center, right, justify).
    pub align: Option<String>,
    /// Couleur de texte (CSS color).
    pub color: Option<String>,
    /// Couleur de fond (CSS color).
    pub background: Option<String>,
}

impl DocContent {
    /// Compte les mots et caractères dans tout le document.
    pub fn recompute_stats(&mut self) {
        let mut words = 0u32;
        let mut chars = 0u32;
        for block in &self.blocks {
            let text = block_text(block);
            chars += text.chars().count() as u32;
            words += text.split_whitespace().count() as u32;
        }
        self.meta.word_count = words;
        self.meta.char_count = chars;
    }

    /// Retourne le texte brut du document.
    pub fn to_plain_text(&self) -> String {
        self.blocks
            .iter()
            .map(block_text)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn block_text(block: &DocBlock) -> String {
    match block {
        DocBlock::Paragraph { text, .. }
        | DocBlock::Heading { text, .. }
        | DocBlock::BulletItem { text, .. }
        | DocBlock::NumberedItem { text, .. }
        | DocBlock::Code { text, .. }
        | DocBlock::Quote { text } => text.clone(),
        DocBlock::Image { alt, .. } => alt.clone().unwrap_or_default(),
        DocBlock::Divider => String::new(),
        DocBlock::Table { rows, .. } => rows
            .iter()
            .map(|r| r.join("\t"))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recompute_stats() {
        let mut doc = DocContent::default();
        doc.blocks.push(DocBlock::Heading {
            level: 1,
            text: "Hello world".into(),
        });
        doc.blocks.push(DocBlock::Paragraph {
            text: "This is a test.".into(),
            style: TextStyle::default(),
        });
        doc.recompute_stats();
        assert_eq!(doc.meta.word_count, 2 + 4);
        assert!(doc.meta.char_count > 0);
    }

    #[test]
    fn to_plain_text() {
        let mut doc = DocContent::default();
        doc.blocks.push(DocBlock::Heading {
            level: 1,
            text: "Titre".into(),
        });
        doc.blocks.push(DocBlock::Paragraph {
            text: "Corps.".into(),
            style: TextStyle::default(),
        });
        assert_eq!(doc.to_plain_text(), "Titre\nCorps.");
    }
}
