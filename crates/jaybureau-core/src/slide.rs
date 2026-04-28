//! Types pour Jay Slides (présentation collaborative).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Présentation = ensemble de slides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlideDeck {
    pub slides: Vec<Slide>,
    /// Thème actif (nom).
    pub theme: String,
    /// Ratio d'aspect (16:9, 4:3).
    pub aspect_ratio: String,
}

/// Une slide = fond + éléments positionnés.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub id: String,
    /// Fond (couleur, gradient, image).
    pub background: SlideBackground,
    /// Éléments de la slide (texte, image, forme).
    pub elements: Vec<SlideElement>,
    /// Notes du présentateur.
    pub notes: String,
}

impl Slide {
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            background: SlideBackground::Color("#ffffff".into()),
            elements: Vec::new(),
            notes: String::new(),
        }
    }
}

/// Fond d'une slide.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlideBackground {
    #[serde(rename = "color")]
    Color(String),
    #[serde(rename = "gradient")]
    Gradient { from: String, to: String, angle: u16 },
    #[serde(rename = "image")]
    Image { drive_file_id: String },
}

/// Élément positionné sur une slide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideElement {
    pub id: String,
    /// Position x (en %).
    pub x: f32,
    /// Position y (en %).
    pub y: f32,
    /// Largeur (en %).
    pub width: f32,
    /// Hauteur (en %).
    pub height: f32,
    /// Rotation en degrés.
    pub rotation: f32,
    /// Z-index.
    pub z: i32,
    pub kind: SlideElementKind,
}

/// Type d'élément.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SlideElementKind {
    #[serde(rename = "text")]
    Text {
        content: String,
        font_family: String,
        font_size: f32,
        color: String,
        bold: bool,
        italic: bool,
        align: String,
    },
    #[serde(rename = "image")]
    Image { drive_file_id: String },
    #[serde(rename = "shape")]
    Shape {
        shape: String,
        fill: String,
        stroke: String,
        stroke_width: f32,
    },
}

impl SlideDeck {
    /// Crée une présentation vierge avec une première slide.
    pub fn new() -> Self {
        Self {
            slides: vec![Slide::empty()],
            theme: "default".into(),
            aspect_ratio: "16:9".into(),
        }
    }

    /// Ajoute une slide à la fin.
    pub fn add_slide(&mut self) -> &mut Slide {
        self.slides.push(Slide::empty());
        self.slides.last_mut().unwrap()
    }

    /// Supprime une slide par index.
    pub fn remove_slide(&mut self, index: usize) -> Option<Slide> {
        if index < self.slides.len() {
            Some(self.slides.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_one_slide() {
        let deck = SlideDeck::new();
        assert_eq!(deck.slides.len(), 1);
    }

    #[test]
    fn add_and_remove_slide() {
        let mut deck = SlideDeck::new();
        deck.add_slide();
        deck.add_slide();
        assert_eq!(deck.slides.len(), 3);
        deck.remove_slide(1);
        assert_eq!(deck.slides.len(), 2);
    }
}
