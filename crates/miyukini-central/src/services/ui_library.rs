//! Service Miyukini UI Library — éléments UI natifs du COG.
//!
//! Permet de voir tous les éléments, filtrer par catégorie, et d'en créer de nouveaux
//! (ex. créateur de boutons : padding, bordure, ratio, arrondis, états, police, animation).

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use std::sync::atomic::{AtomicU64, Ordering};

/// Compteur pour générer des ID uniques.
fn next_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Catégorie d'élément UI (pour filtrage).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementCategory {
    Bouton,
    Champ,
    Carte,
    Label,
    Barre,
    Autre,
}

impl ElementCategory {
    pub fn label(&self) -> &'static str {
        match self {
            ElementCategory::Bouton => "Bouton",
            ElementCategory::Champ => "Champ",
            ElementCategory::Carte => "Carte",
            ElementCategory::Label => "Label",
            ElementCategory::Barre => "Barre",
            ElementCategory::Autre => "Autre",
        }
    }
}

/// Style de bordure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Aucune,
    Fine,
    Moyenne,
    Epaisse,
}

impl BorderStyle {
    pub fn label(&self) -> &'static str {
        match self {
            BorderStyle::Aucune => "Aucune",
            BorderStyle::Fine => "Fine",
            BorderStyle::Moyenne => "Moyenne",
            BorderStyle::Epaisse => "Épaisse",
        }
    }
    pub fn stroke_width(&self) -> f32 {
        match self {
            BorderStyle::Aucune => 0.0,
            BorderStyle::Fine => 1.0,
            BorderStyle::Moyenne => 2.0,
            BorderStyle::Epaisse => 4.0,
        }
    }
}

/// Alignement du texte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Gauche,
    Centre,
    Droite,
}

impl TextAlign {
    pub fn label(&self) -> &'static str {
        match self {
            TextAlign::Gauche => "Gauche",
            TextAlign::Centre => "Centre",
            TextAlign::Droite => "Droite",
        }
    }
    pub fn to_egui(&self) -> egui::Align {
        match self {
            TextAlign::Gauche => egui::Align::LEFT,
            TextAlign::Centre => egui::Align::Center,
            TextAlign::Droite => egui::Align::RIGHT,
        }
    }
}

/// Effet au survol de la souris (mouse hover).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverEffect {
    /// Aucun effet.
    None,
    /// Contour autour du bouton (épaisseur + couleur).
    Contour,
    /// Effet brillant (shining).
    Shining,
    /// Animation arc-en-ciel (rainbow wave).
    RainbowWave,
    /// Animation de particules.
    Particle,
}

impl HoverEffect {
    pub fn label(&self) -> &'static str {
        match self {
            HoverEffect::None => "Aucun",
            HoverEffect::Contour => "Contour",
            HoverEffect::Shining => "Shining",
            HoverEffect::RainbowWave => "Rainbow wave",
            HoverEffect::Particle => "Particules",
        }
    }
}

/// Type de bouton : press (clic relâché) ou switch (reste appuyé, recliquer pour revenir).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    /// Bouton press : clic puis relâchement, retour à l'état neutre.
    PressButton,
    /// Switch : reste appuyé après le clic ; recliquer pour revenir à l'état normal.
    Switch,
}

impl ButtonType {
    pub fn label(&self) -> &'static str {
        match self {
            ButtonType::PressButton => "Bouton press",
            ButtonType::Switch => "Switch (reste appuyé)",
        }
    }
}

/// Spécification d'un bouton (créateur).
#[derive(Debug, Clone)]
pub struct ButtonSpec {
    pub padding_ext_h: f32,
    pub padding_ext_v: f32,
    pub padding_int_h: f32,
    pub padding_int_v: f32,
    pub border_style: BorderStyle,
    pub ratio_hauteur_largeur: f32,
    pub corner_radius: f32,
    pub color_neutral_r: u8,
    pub color_neutral_g: u8,
    pub color_neutral_b: u8,
    pub color_pressed_r: u8,
    pub color_pressed_g: u8,
    pub color_pressed_b: u8,
    pub font_size: f32,
    pub text_align: TextAlign,
    pub animation: bool,
    pub state_change: bool,
    pub label_preview: String,
    /// Effet au survol (none | contour | shining | rainbow | particle).
    pub hover_effect: HoverEffect,
    /// Épaisseur du contour au survol (si hover_effect == Contour).
    pub hover_contour_thickness: f32,
    /// Couleur du contour au survol (noir par défaut).
    pub hover_contour_r: u8,
    pub hover_contour_g: u8,
    pub hover_contour_b: u8,
    /// Type : press button ou switch.
    pub button_type: ButtonType,
    /// Intensité de la désaturation en mode grisé (0 = couleur d'origine, 1 = gris complet).
    pub desaturation_intensity: f32,
    /// Rainbow : afficher en bordure à l'état normal (pas seulement au survol).
    pub rainbow_border_base: bool,
    /// Taille de la bordure rainbow (état normal et survol).
    pub rainbow_border_size: f32,
    /// Vitesse du cycle de couleurs rainbow (état normal et survol).
    pub rainbow_speed: f32,
    /// Mode ombre : ombre en partie basse du bouton, même forme que la bordure basse (même arrondi).
    pub shadow_enabled: bool,
    /// Hauteur de l’ombre en pixels (régiable).
    pub shadow_height: f32,
    /// Opacité de l’ombre (0 = transparente, 1 = opaque).
    pub shadow_alpha: f32,
}

impl Default for ButtonSpec {
    fn default() -> Self {
        Self {
            padding_ext_h: 8.0,
            padding_ext_v: 4.0,
            padding_int_h: 12.0,
            padding_int_v: 6.0,
            border_style: BorderStyle::Moyenne,
            ratio_hauteur_largeur: 0.4,
            corner_radius: 4.0,
            color_neutral_r: 60,
            color_neutral_g: 60,
            color_neutral_b: 60,
            color_pressed_r: 80,
            color_pressed_g: 80,
            color_pressed_b: 80,
            font_size: 13.0,
            text_align: TextAlign::Centre,
            animation: false,
            state_change: true,
            label_preview: "Bouton".to_string(),
            hover_effect: HoverEffect::None,
            hover_contour_thickness: 2.0,
            hover_contour_r: 0,
            hover_contour_g: 0,
            hover_contour_b: 0,
            button_type: ButtonType::PressButton,
            desaturation_intensity: 0.7,
            rainbow_border_base: false,
            rainbow_border_size: 2.0,
            rainbow_speed: 0.3,
            shadow_enabled: false,
            shadow_height: 4.0,
            shadow_alpha: 0.35,
        }
    }
}

/// Spec minimale pour un champ (saisie).
#[derive(Debug, Clone)]
pub struct ChampSpec {
    pub label: String,
    pub placeholder: String,
    pub kind: ChampKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChampKind {
    Texte,
    Nombre,
    Email,
    MotDePasse,
    ZoneTexte,
}

impl ChampKind {
    pub fn label(&self) -> &'static str {
        match self {
            ChampKind::Texte => "Texte",
            ChampKind::Nombre => "Nombre",
            ChampKind::Email => "Email",
            ChampKind::MotDePasse => "Mot de passe",
            ChampKind::ZoneTexte => "Zone de texte",
        }
    }
}

impl Default for ChampSpec {
    fn default() -> Self {
        Self {
            label: "Champ".to_string(),
            placeholder: "Saisir…".to_string(),
            kind: ChampKind::Texte,
        }
    }
}

/// Spec minimale pour une carte (conteneur).
#[derive(Debug, Clone)]
pub struct CardSpec {
    pub title: String,
    pub has_border: bool,
    pub has_shadow: bool,
}

impl Default for CardSpec {
    fn default() -> Self {
        Self {
            title: "Carte".to_string(),
            has_border: true,
            has_shadow: false,
        }
    }
}

/// Spec minimale pour un label (texte).
#[derive(Debug, Clone)]
pub struct LabelSpec {
    pub text: String,
    pub font_size: f32,
    pub bold: bool,
}

impl Default for LabelSpec {
    fn default() -> Self {
        Self {
            text: "Label".to_string(),
            font_size: 14.0,
            bold: false,
        }
    }
}

/// Spec minimale pour une barre (progression, nav, etc.).
#[derive(Debug, Clone)]
pub struct BarSpec {
    pub kind: BarKind,
    pub progress: f32,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    Progression,
    BarreNavigation,
    BarreOutils,
}

impl BarKind {
    pub fn label(&self) -> &'static str {
        match self {
            BarKind::Progression => "Progression",
            BarKind::BarreNavigation => "Navigation",
            BarKind::BarreOutils => "Outils",
        }
    }
}

impl Default for BarSpec {
    fn default() -> Self {
        Self {
            kind: BarKind::Progression,
            progress: 0.65,
            label: "Barre".to_string(),
        }
    }
}

/// Spec minimale pour autres éléments (séparateur, badge, etc.).
#[derive(Debug, Clone)]
pub struct OtherSpec {
    pub kind: OtherKind,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtherKind {
    Separateur,
    Espace,
    Badge,
    Avatar,
    Icone,
}

impl OtherKind {
    pub fn label(&self) -> &'static str {
        match self {
            OtherKind::Separateur => "Séparateur",
            OtherKind::Espace => "Espace",
            OtherKind::Badge => "Badge",
            OtherKind::Avatar => "Avatar",
            OtherKind::Icone => "Icône",
        }
    }
}

impl Default for OtherSpec {
    fn default() -> Self {
        Self {
            kind: OtherKind::Separateur,
            label: "—".to_string(),
        }
    }
}

/// Type d'élément UI (spec selon le type).
#[derive(Debug, Clone)]
pub enum ElementSpec {
    Button(ButtonSpec),
    Champ(ChampSpec),
    Card(CardSpec),
    Label(LabelSpec),
    Bar(BarSpec),
    Other(OtherSpec),
}

/// Élément UI enregistré dans la bibliothèque.
#[derive(Debug, Clone)]
pub struct UiElement {
    pub id: String,
    pub name: String,
    pub category: ElementCategory,
    pub spec: ElementSpec,
}

/// Type de créateur ouvert (pour le panneau "Créer").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatorKind {
    None,
    Button,
}

/// Service Miyukini UI Library.
pub struct UiLibraryService {
    /// Éléments enregistrés (natifs + créés).
    pub elements: Vec<UiElement>,
    /// Filtre par catégorie (None = toutes).
    pub filter_category: Option<ElementCategory>,
    /// Créateur ouvert.
    pub creator_kind: CreatorKind,
    /// Spec en cours pour le bouton en création.
    pub button_spec: ButtonSpec,
    /// Nom du nouvel élément à créer.
    pub new_element_name: String,
    /// État switch dans l'aperçu (pour type Switch).
    pub preview_switch_on: bool,
    /// Afficher l'aperçu en mode grisé (inactif).
    pub preview_show_disabled: bool,
}

impl Default for UiLibraryService {
    fn default() -> Self {
        Self {
            elements: default_elements(),
            filter_category: None,
            creator_kind: CreatorKind::None,
            button_spec: ButtonSpec::default(),
            new_element_name: String::new(),
            preview_switch_on: false,
            preview_show_disabled: false,
        }
    }
}

fn default_elements() -> Vec<UiElement> {
    vec![
        // ——— Boutons ———
        UiElement {
            id: "btn_primary".to_string(),
            name: "Bouton primaire".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Primaire".to_string(),
                color_neutral_r: 0,
                color_neutral_g: 120,
                color_neutral_b: 0,
                color_pressed_r: 0,
                color_pressed_g: 160,
                color_pressed_b: 0,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_secondary".to_string(),
            name: "Bouton secondaire".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec::default()),
        },
        UiElement {
            id: "btn_danger".to_string(),
            name: "Bouton danger".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Supprimer".to_string(),
                color_neutral_r: 180,
                color_neutral_g: 40,
                color_neutral_b: 40,
                color_pressed_r: 220,
                color_pressed_g: 60,
                color_pressed_b: 60,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_success".to_string(),
            name: "Bouton succès".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Valider".to_string(),
                color_neutral_r: 40,
                color_neutral_g: 140,
                color_neutral_b: 80,
                color_pressed_r: 60,
                color_pressed_g: 180,
                color_pressed_b: 100,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_warning".to_string(),
            name: "Bouton avertissement".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Attention".to_string(),
                color_neutral_r: 200,
                color_neutral_g: 140,
                color_neutral_b: 0,
                color_pressed_r: 240,
                color_pressed_g: 180,
                color_pressed_b: 40,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_info".to_string(),
            name: "Bouton info".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "En savoir plus".to_string(),
                color_neutral_r: 30,
                color_neutral_g: 100,
                color_neutral_b: 180,
                color_pressed_r: 50,
                color_pressed_g: 130,
                color_pressed_b: 220,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_outline".to_string(),
            name: "Bouton outline".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Outline".to_string(),
                border_style: BorderStyle::Moyenne,
                color_neutral_r: 240,
                color_neutral_g: 240,
                color_neutral_b: 240,
                color_pressed_r: 220,
                color_pressed_g: 220,
                color_pressed_b: 220,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_small".to_string(),
            name: "Bouton petit".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Petit".to_string(),
                padding_ext_h: 6.0,
                padding_ext_v: 2.0,
                padding_int_h: 8.0,
                padding_int_v: 2.0,
                font_size: 11.0,
                corner_radius: 3.0,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_large".to_string(),
            name: "Bouton grand".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Grand bouton".to_string(),
                padding_ext_h: 16.0,
                padding_ext_v: 10.0,
                padding_int_h: 20.0,
                padding_int_v: 10.0,
                font_size: 16.0,
                corner_radius: 8.0,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_rounded".to_string(),
            name: "Bouton arrondi".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Pill".to_string(),
                corner_radius: 24.0,
                ratio_hauteur_largeur: 0.5,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_with_shadow".to_string(),
            name: "Bouton avec ombre".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Ombre".to_string(),
                shadow_enabled: true,
                shadow_height: 6.0,
                shadow_alpha: 0.4,
                corner_radius: 6.0,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_icon".to_string(),
            name: "Bouton avec icône".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "🔒 Connexion".to_string(),
                color_neutral_r: 60,
                color_neutral_g: 80,
                color_neutral_b: 120,
                color_pressed_r: 80,
                color_pressed_g: 100,
                color_pressed_b: 160,
                ..ButtonSpec::default()
            }),
        },
        UiElement {
            id: "btn_disabled_style".to_string(),
            name: "Bouton désactivé (référence)".to_string(),
            category: ElementCategory::Bouton,
            spec: ElementSpec::Button(ButtonSpec {
                label_preview: "Désactivé".to_string(),
                desaturation_intensity: 0.9,
                color_neutral_r: 120,
                color_neutral_g: 120,
                color_neutral_b: 120,
                color_pressed_r: 140,
                color_pressed_g: 140,
                color_pressed_b: 140,
                ..ButtonSpec::default()
            }),
        },
        // ——— Champs ———
        UiElement {
            id: "champ_texte".to_string(),
            name: "Champ texte".to_string(),
            category: ElementCategory::Champ,
            spec: ElementSpec::Champ(ChampSpec {
                label: "Nom".to_string(),
                placeholder: "Votre nom…".to_string(),
                kind: ChampKind::Texte,
            }),
        },
        UiElement {
            id: "champ_email".to_string(),
            name: "Champ email".to_string(),
            category: ElementCategory::Champ,
            spec: ElementSpec::Champ(ChampSpec {
                label: "Email".to_string(),
                placeholder: "vous@exemple.fr".to_string(),
                kind: ChampKind::Email,
            }),
        },
        UiElement {
            id: "champ_mdp".to_string(),
            name: "Champ mot de passe".to_string(),
            category: ElementCategory::Champ,
            spec: ElementSpec::Champ(ChampSpec {
                label: "Mot de passe".to_string(),
                placeholder: "••••••••".to_string(),
                kind: ChampKind::MotDePasse,
            }),
        },
        UiElement {
            id: "champ_nombre".to_string(),
            name: "Champ nombre".to_string(),
            category: ElementCategory::Champ,
            spec: ElementSpec::Champ(ChampSpec {
                label: "Quantité".to_string(),
                placeholder: "0".to_string(),
                kind: ChampKind::Nombre,
            }),
        },
        UiElement {
            id: "champ_zone".to_string(),
            name: "Zone de texte".to_string(),
            category: ElementCategory::Champ,
            spec: ElementSpec::Champ(ChampSpec {
                label: "Commentaire".to_string(),
                placeholder: "Saisir un texte long…".to_string(),
                kind: ChampKind::ZoneTexte,
            }),
        },
        // ——— Cartes ———
        UiElement {
            id: "carte_simple".to_string(),
            name: "Carte simple".to_string(),
            category: ElementCategory::Carte,
            spec: ElementSpec::Card(CardSpec {
                title: "Titre".to_string(),
                has_border: true,
                has_shadow: false,
            }),
        },
        UiElement {
            id: "carte_ombre".to_string(),
            name: "Carte avec ombre".to_string(),
            category: ElementCategory::Carte,
            spec: ElementSpec::Card(CardSpec {
                title: "Contenu".to_string(),
                has_border: false,
                has_shadow: true,
            }),
        },
        UiElement {
            id: "carte_bordure".to_string(),
            name: "Carte bordure seule".to_string(),
            category: ElementCategory::Carte,
            spec: ElementSpec::Card(CardSpec {
                title: "Panel".to_string(),
                has_border: true,
                has_shadow: false,
            }),
        },
        UiElement {
            id: "carte_compacte".to_string(),
            name: "Carte compacte".to_string(),
            category: ElementCategory::Carte,
            spec: ElementSpec::Card(CardSpec {
                title: "Résumé".to_string(),
                has_border: true,
                has_shadow: false,
            }),
        },
        // ——— Labels ———
        UiElement {
            id: "label_titre".to_string(),
            name: "Titre (H1)".to_string(),
            category: ElementCategory::Label,
            spec: ElementSpec::Label(LabelSpec {
                text: "Titre principal".to_string(),
                font_size: 20.0,
                bold: true,
            }),
        },
        UiElement {
            id: "label_soustitre".to_string(),
            name: "Sous-titre (H2)".to_string(),
            category: ElementCategory::Label,
            spec: ElementSpec::Label(LabelSpec {
                text: "Sous-titre".to_string(),
                font_size: 16.0,
                bold: true,
            }),
        },
        UiElement {
            id: "label_corps".to_string(),
            name: "Texte corps".to_string(),
            category: ElementCategory::Label,
            spec: ElementSpec::Label(LabelSpec {
                text: "Texte de paragraphe.".to_string(),
                font_size: 14.0,
                bold: false,
            }),
        },
        UiElement {
            id: "label_caption".to_string(),
            name: "Légende / caption".to_string(),
            category: ElementCategory::Label,
            spec: ElementSpec::Label(LabelSpec {
                text: "Légende ou note secondaire".to_string(),
                font_size: 12.0,
                bold: false,
            }),
        },
        UiElement {
            id: "label_lien".to_string(),
            name: "Label lien".to_string(),
            category: ElementCategory::Label,
            spec: ElementSpec::Label(LabelSpec {
                text: "Lien".to_string(),
                font_size: 14.0,
                bold: false,
            }),
        },
        // ——— Barres ———
        UiElement {
            id: "barre_progression".to_string(),
            name: "Barre de progression".to_string(),
            category: ElementCategory::Barre,
            spec: ElementSpec::Bar(BarSpec {
                kind: BarKind::Progression,
                progress: 0.65,
                label: "65 %".to_string(),
            }),
        },
        UiElement {
            id: "barre_progression_indeterminee".to_string(),
            name: "Progression indéterminée".to_string(),
            category: ElementCategory::Barre,
            spec: ElementSpec::Bar(BarSpec {
                kind: BarKind::Progression,
                progress: -1.0,
                label: "Chargement…".to_string(),
            }),
        },
        UiElement {
            id: "barre_navigation".to_string(),
            name: "Barre de navigation".to_string(),
            category: ElementCategory::Barre,
            spec: ElementSpec::Bar(BarSpec {
                kind: BarKind::BarreNavigation,
                progress: 0.0,
                label: "Accueil | Profil | Paramètres".to_string(),
            }),
        },
        UiElement {
            id: "barre_outils".to_string(),
            name: "Barre d'outils".to_string(),
            category: ElementCategory::Barre,
            spec: ElementSpec::Bar(BarSpec {
                kind: BarKind::BarreOutils,
                progress: 0.0,
                label: "Éditer | Copier | Coller".to_string(),
            }),
        },
        // ——— Autre ———
        UiElement {
            id: "autre_separateur".to_string(),
            name: "Séparateur".to_string(),
            category: ElementCategory::Autre,
            spec: ElementSpec::Other(OtherSpec {
                kind: OtherKind::Separateur,
                label: "—".to_string(),
            }),
        },
        UiElement {
            id: "autre_espace".to_string(),
            name: "Espace vertical".to_string(),
            category: ElementCategory::Autre,
            spec: ElementSpec::Other(OtherSpec {
                kind: OtherKind::Espace,
                label: "8 px".to_string(),
            }),
        },
        UiElement {
            id: "autre_badge".to_string(),
            name: "Badge".to_string(),
            category: ElementCategory::Autre,
            spec: ElementSpec::Other(OtherSpec {
                kind: OtherKind::Badge,
                label: "Nouveau".to_string(),
            }),
        },
        UiElement {
            id: "autre_avatar".to_string(),
            name: "Avatar".to_string(),
            category: ElementCategory::Autre,
            spec: ElementSpec::Other(OtherSpec {
                kind: OtherKind::Avatar,
                label: "👤".to_string(),
            }),
        },
        UiElement {
            id: "autre_icone".to_string(),
            name: "Icône".to_string(),
            category: ElementCategory::Autre,
            spec: ElementSpec::Other(OtherSpec {
                kind: OtherKind::Icone,
                label: "⚙".to_string(),
            }),
        },
    ]
}

/// Conversion HSV (h 0..1, s 0..1, v 0..1) vers RGB 0..255.
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

impl UiLibraryService {
    /// Aperçu WYSIWYG du bouton : dessine dans `rect` (déjà alloué). Applique padding, bordure, couleurs, texte, hover, grisé.
    fn ui_button_preview_wysiwyg(
        ui: &mut egui::Ui,
        rect: egui::Rect,
        spec: &ButtonSpec,
        is_pressed: bool,
        is_hovered: bool,
        is_disabled: bool,
    ) {
        let inner_rect = rect.shrink2(egui::vec2(spec.padding_ext_h, spec.padding_ext_v));
        let radius = spec.corner_radius.clamp(0.0, 255.0) as u8;
        let corner_radius = egui::CornerRadius::same(radius);

        // Ombre en partie basse : même forme que la bordure basse (même arrondi), hauteur réglable.
        // En mode appuyé, l’ombre reste visible mais réduite (hauteur + opacité) pour garder l’effet de volume tout en simulant l’enfoncement.
        if spec.shadow_enabled && spec.shadow_height > 0.0 {
            let (h, alpha) = if is_pressed {
                let h = (spec.shadow_height * 0.4).max(1.0);
                let alpha = spec.shadow_alpha * 0.5;
                (h, alpha)
            } else {
                (spec.shadow_height, spec.shadow_alpha)
            };
            let shadow_rect = egui::Rect::from_min_max(
                egui::pos2(inner_rect.min.x, inner_rect.max.y),
                egui::pos2(inner_rect.max.x, inner_rect.max.y + h),
            );
            let a = (255.0 * alpha.clamp(0.0, 1.0)) as u8;
            let shadow_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, a);
            ui.painter().rect_filled(shadow_rect, corner_radius, shadow_color);
        }

        let fill_color = if is_disabled {
            let (r, g, b) = (
                f32::from(spec.color_neutral_r),
                f32::from(spec.color_neutral_g),
                f32::from(spec.color_neutral_b),
            );
            let grey = 120.0;
            let i = spec.desaturation_intensity.clamp(0.0, 1.0);
            let rr = (1.0 - i) * r + i * grey;
            let gg = (1.0 - i) * g + i * grey;
            let bb = (1.0 - i) * b + i * grey;
            egui::Color32::from_rgb(rr as u8, gg as u8, bb as u8)
        } else if is_pressed {
            egui::Color32::from_rgb(
                spec.color_pressed_r,
                spec.color_pressed_g,
                spec.color_pressed_b,
            )
        } else {
            egui::Color32::from_rgb(
                spec.color_neutral_r,
                spec.color_neutral_g,
                spec.color_neutral_b,
            )
        };

        ui.painter()
            .rect_filled(inner_rect, corner_radius, fill_color);

        if spec.border_style != BorderStyle::Aucune {
            let stroke_w = spec.border_style.stroke_width();
            let border_color = egui::Color32::from_rgb(
                spec.color_neutral_r.saturating_sub(30),
                spec.color_neutral_g.saturating_sub(30),
                spec.color_neutral_b.saturating_sub(30),
            );
            ui.painter().rect_stroke(
                inner_rect,
                corner_radius,
                egui::Stroke::new(stroke_w, border_color),
                egui::StrokeKind::Inside,
            );
        }

        let draw_rainbow_border = |ui: &egui::Ui, rect: egui::Rect| {
            let t = ui.ctx().input(|i| i.time) as f32;
            let h = (t * spec.rainbow_speed) % 1.0;
            let (r, g, b) = hsv_to_rgb(h, 1.0, 1.0);
            let stroke_w = spec.rainbow_border_size.max(0.5);
            ui.painter().rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(stroke_w, egui::Color32::from_rgb(r, g, b)),
                egui::StrokeKind::Outside,
            );
            ui.ctx().request_repaint();
        };

        if !is_disabled && spec.hover_effect == HoverEffect::RainbowWave && spec.rainbow_border_base {
            draw_rainbow_border(ui, inner_rect);
        }

        if is_hovered && !is_disabled && spec.hover_effect != HoverEffect::None {
            match spec.hover_effect {
                HoverEffect::Contour => {
                    let c = egui::Color32::from_rgb(
                        spec.hover_contour_r,
                        spec.hover_contour_g,
                        spec.hover_contour_b,
                    );
                    ui.painter().rect_stroke(
                        inner_rect,
                        corner_radius,
                        egui::Stroke::new(spec.hover_contour_thickness.max(0.5), c),
                        egui::StrokeKind::Outside,
                    );
                }
                HoverEffect::Shining => {
                    let glow = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 60);
                    ui.painter().rect_filled(
                        inner_rect.shrink2(egui::vec2(2.0, 2.0)),
                        corner_radius,
                        glow,
                    );
                }
                HoverEffect::RainbowWave => {
                    draw_rainbow_border(ui, inner_rect);
                }
                HoverEffect::Particle => {
                    let t = ui.ctx().input(|i| i.time) as f32;
                    for i in 0..6 {
                        let angle = t * 2.0 + (i as f32 * 1.05);
                        let dx = angle.cos() * (inner_rect.width() * 0.3);
                        let dy = angle.sin() * (inner_rect.height() * 0.3);
                        let c = inner_rect.center() + egui::vec2(dx, dy);
                        ui.painter().circle_filled(c, 2.0, egui::Color32::WHITE);
                    }
                    ui.ctx().request_repaint();
                }
                HoverEffect::None => {}
            }
        }

        let text_color = if is_disabled {
            let i = spec.desaturation_intensity.clamp(0.0, 1.0);
            let grey = 140u8;
            let w = (255.0 * (1.0 - i) + f32::from(grey) * i) as u8;
            egui::Color32::from_rgb(w, w, w)
        } else {
            egui::Color32::WHITE
        };
        let text_rect = inner_rect.shrink2(egui::vec2(spec.padding_int_h, spec.padding_int_v));
        let layout = match spec.text_align {
            TextAlign::Gauche => egui::Layout::left_to_right(egui::Align::Center)
                .with_main_align(egui::Align::LEFT),
            TextAlign::Centre => egui::Layout::left_to_right(egui::Align::Center)
                .with_main_align(egui::Align::Center),
            TextAlign::Droite => egui::Layout::right_to_left(egui::Align::Center)
                .with_main_align(egui::Align::RIGHT),
        };
        #[allow(deprecated)]
        ui.allocate_ui_at_rect(text_rect, |ui| {
            ui.with_layout(layout, |ui| {
                ui.label(
                    egui::RichText::new(&spec.label_preview)
                        .size(spec.font_size)
                        .color(text_color),
                );
            });
        });
    }

    fn filtered_elements(&self) -> Vec<&UiElement> {
        self.elements
            .iter()
            .filter(|el| {
                self.filter_category
                    .is_none_or(|cat| el.category == cat)
            })
            .collect()
    }

    fn ui_filter_pills(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Catégorie :");
            if ui.button("Toutes").clicked() {
                self.filter_category = None;
            }
            for &cat in &[
                ElementCategory::Bouton,
                ElementCategory::Champ,
                ElementCategory::Carte,
                ElementCategory::Label,
                ElementCategory::Barre,
                ElementCategory::Autre,
            ] {
                let selected = self.filter_category == Some(cat);
                if ui.selectable_label(selected, cat.label()).clicked() {
                    self.filter_category = Some(cat);
                }
            }
        });
    }

    fn ui_elements_table(&mut self, ui: &mut egui::Ui) {
        let filtered: Vec<UiElement> = self
            .filtered_elements()
            .into_iter()
            .cloned()
            .collect();

        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            if filtered.is_empty() {
                ui.label("Aucun élément dans cette catégorie.");
                return;
            }
            let row_height = 28.0;
            for el in &filtered {
                ui.horizontal(|ui| {
                    ui.set_min_height(row_height);
                    ui.label(&el.name);
                    ui.label(el.category.label());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match &el.spec {
                            ElementSpec::Button(spec) => {
                                let color = egui::Color32::from_rgb(
                                    spec.color_neutral_r,
                                    spec.color_neutral_g,
                                    spec.color_neutral_b,
                                );
                                let radius = spec.corner_radius.clamp(0.0, 255.0) as u8;
                                let btn = egui::Button::new(
                                    egui::RichText::new(&spec.label_preview)
                                        .size(spec.font_size)
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(color)
                                .corner_radius(egui::CornerRadius::same(radius));
                                let _ = ui.add_sized([80.0, 24.0], btn);
                            }
                            ElementSpec::Champ(spec) => {
                                ui.label(
                                    egui::RichText::new(format!("{} — {}", spec.kind.label(), spec.placeholder))
                                        .size(11.0)
                                        .weak(),
                                );
                            }
                            ElementSpec::Card(spec) => {
                                ui.label(egui::RichText::new(&spec.title).size(12.0));
                            }
                            ElementSpec::Label(spec) => {
                                let mut rt = egui::RichText::new(&spec.text).size(spec.font_size.min(12.0));
                                if spec.bold {
                                    rt = rt.strong();
                                }
                                ui.label(rt);
                            }
                            ElementSpec::Bar(spec) => {
                                if spec.kind == BarKind::Progression && spec.progress >= 0.0 {
                                    let _ = ui.add_sized(
                                        [80.0, 16.0],
                                        egui::ProgressBar::new(spec.progress)
                                            .show_percentage(),
                                    );
                                } else {
                                    ui.label(spec.kind.label());
                                }
                            }
                            ElementSpec::Other(spec) => {
                                ui.label(&spec.label);
                            }
                        }
                    });
                });
                ui.separator();
            }
        });
    }

    fn ui_creator_button_panel(&mut self, ui: &mut egui::Ui) {
        let spec = &mut self.button_spec;
        ui.heading("Créateur de bouton");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Nom :");
            ui.text_edit_singleline(&mut self.new_element_name);
        });
        ui.add_space(4.0);

        ui.collapsing("Padding", |ui| {
            ui.horizontal(|ui| {
                ui.label("Ext. H:");
                ui.add(egui::DragValue::new(&mut spec.padding_ext_h).range(0.0..=50.0).speed(1.0));
                ui.label("Ext. V:");
                ui.add(egui::DragValue::new(&mut spec.padding_ext_v).range(0.0..=50.0).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Int. H:");
                ui.add(egui::DragValue::new(&mut spec.padding_int_h).range(0.0..=50.0).speed(1.0));
                ui.label("Int. V:");
                ui.add(egui::DragValue::new(&mut spec.padding_int_v).range(0.0..=50.0).speed(1.0));
            });
        });
        ui.collapsing("Bordure et forme", |ui| {
            ui.horizontal(|ui| {
                ui.label("Bordure :");
                for &b in &[
                    BorderStyle::Aucune,
                    BorderStyle::Fine,
                    BorderStyle::Moyenne,
                    BorderStyle::Epaisse,
                ] {
                    if ui.selectable_label(spec.border_style == b, b.label()).clicked() {
                        spec.border_style = b;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Ratio H/L :");
                ui.add(egui::DragValue::new(&mut spec.ratio_hauteur_largeur).range(0.1..=2.0).speed(0.05));
            });
            ui.horizontal(|ui| {
                ui.label("Rayon coins :");
                ui.add(egui::DragValue::new(&mut spec.corner_radius).range(0.0..=24.0).speed(1.0));
            });
        });
        ui.collapsing("Ombre", |ui| {
            ui.checkbox(&mut spec.shadow_enabled, "Activer l’ombre (partie basse, même forme que la bordure)");
            ui.horizontal(|ui| {
                ui.label("Hauteur (px) :");
                ui.add(
                    egui::DragValue::new(&mut spec.shadow_height)
                        .range(1.0..=24.0)
                        .speed(0.5),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Opacité :");
                ui.add(
                    egui::Slider::new(&mut spec.shadow_alpha, 0.0..=1.0)
                        .step_by(0.05)
                        .custom_formatter(|v, _| format!("{:.0} %", v * 100.0)),
                );
            });
        });
        ui.collapsing("Couleurs (phase neutre / appuyé)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Neutre :");
                let mut neutral = egui::Color32::from_rgb(
                    spec.color_neutral_r,
                    spec.color_neutral_g,
                    spec.color_neutral_b,
                );
                if egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut neutral,
                    egui::color_picker::Alpha::Opaque,
                )
                .changed()
                {
                    spec.color_neutral_r = neutral.r();
                    spec.color_neutral_g = neutral.g();
                    spec.color_neutral_b = neutral.b();
                }
                ui.label("(palette)");
            });
            ui.horizontal(|ui| {
                ui.label("Appuyé :");
                let mut pressed = egui::Color32::from_rgb(
                    spec.color_pressed_r,
                    spec.color_pressed_g,
                    spec.color_pressed_b,
                );
                if egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut pressed,
                    egui::color_picker::Alpha::Opaque,
                )
                .changed()
                {
                    spec.color_pressed_r = pressed.r();
                    spec.color_pressed_g = pressed.g();
                    spec.color_pressed_b = pressed.b();
                }
                ui.label("(palette)");
            });
            ui.horizontal(|ui| {
                ui.label("Neutre R/G/B :");
                ui.add(egui::DragValue::new(&mut spec.color_neutral_r).range(0..=255).speed(5));
                ui.add(egui::DragValue::new(&mut spec.color_neutral_g).range(0..=255).speed(5));
                ui.add(egui::DragValue::new(&mut spec.color_neutral_b).range(0..=255).speed(5));
            });
            ui.horizontal(|ui| {
                ui.label("Appuyé R/G/B :");
                ui.add(egui::DragValue::new(&mut spec.color_pressed_r).range(0..=255).speed(5));
                ui.add(egui::DragValue::new(&mut spec.color_pressed_g).range(0..=255).speed(5));
                ui.add(egui::DragValue::new(&mut spec.color_pressed_b).range(0..=255).speed(5));
            });
        });
        ui.collapsing("Texte", |ui| {
            ui.horizontal(|ui| {
                ui.label("Libellé aperçu :");
                ui.text_edit_singleline(&mut spec.label_preview);
            });
            ui.horizontal(|ui| {
                ui.label("Taille police :");
                ui.add(egui::DragValue::new(&mut spec.font_size).range(8.0..=24.0).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Alignement :");
                for &a in &[TextAlign::Gauche, TextAlign::Centre, TextAlign::Droite] {
                    if ui.selectable_label(spec.text_align == a, a.label()).clicked() {
                        spec.text_align = a;
                    }
                }
            });
        });
        ui.collapsing("Comportement", |ui| {
            ui.checkbox(&mut spec.animation, "Animation");
            ui.checkbox(&mut spec.state_change, "Changement d'état / couleur");
        });

        ui.collapsing("Effet au survol (mouse hover)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Effet :");
                for &e in &[
                    HoverEffect::None,
                    HoverEffect::Contour,
                    HoverEffect::Shining,
                    HoverEffect::RainbowWave,
                    HoverEffect::Particle,
                ] {
                    if ui.selectable_label(spec.hover_effect == e, e.label()).clicked() {
                        spec.hover_effect = e;
                    }
                }
            });
            if spec.hover_effect == HoverEffect::Contour {
                ui.horizontal(|ui| {
                    ui.label("Épaisseur :");
                    ui.add(
                        egui::DragValue::new(&mut spec.hover_contour_thickness)
                            .range(0.5..=20.0)
                            .speed(0.5),
                    );
                    ui.label("Couleur :");
                    let mut contour = egui::Color32::from_rgb(
                        spec.hover_contour_r,
                        spec.hover_contour_g,
                        spec.hover_contour_b,
                    );
                    if egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut contour,
                        egui::color_picker::Alpha::Opaque,
                    )
                    .changed()
                    {
                        spec.hover_contour_r = contour.r();
                        spec.hover_contour_g = contour.g();
                        spec.hover_contour_b = contour.b();
                    }
                });
            }
            if spec.hover_effect == HoverEffect::RainbowWave {
                ui.checkbox(&mut spec.rainbow_border_base, "Rainbow en bordure de base (état normal)");
                ui.horizontal(|ui| {
                    ui.label("Taille bordure :");
                    ui.add(
                        egui::DragValue::new(&mut spec.rainbow_border_size)
                            .range(0.5..=20.0)
                            .speed(0.5),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Vitesse changement couleurs :");
                    ui.add(
                        egui::DragValue::new(&mut spec.rainbow_speed)
                            .range(0.05..=2.0)
                            .speed(0.05),
                    );
                });
            }
        });
        ui.collapsing("Type de bouton", |ui| {
            ui.horizontal(|ui| {
                for &bt in &[ButtonType::PressButton, ButtonType::Switch] {
                    if ui.selectable_label(spec.button_type == bt, bt.label()).clicked() {
                        spec.button_type = bt;
                        if bt == ButtonType::PressButton {
                            self.preview_switch_on = false;
                        }
                    }
                }
            });
            ui.label("Switch : reste appuyé après le clic ; recliquer pour revenir à l'état normal.");
        });
        ui.collapsing("Mode inactif / grisé", |ui| {
            ui.horizontal(|ui| {
                ui.label("Intensité désaturation :");
                ui.add(
                    egui::Slider::new(&mut spec.desaturation_intensity, 0.0..=1.0)
                        .step_by(0.05)
                        .custom_formatter(|v, _| format!("{:.0} %", v * 100.0)),
                );
            });
            ui.checkbox(&mut self.preview_show_disabled, "Aperçu grisé (mode inactif)");
            ui.label("Tous les boutons ont un mode grisé indiquant qu'ils sont inactifs.");
        });
        ui.add_space(12.0);
        ui.separator();
        ui.label("Aperçu (reflète tous les réglages) :");
        ui.add_space(4.0);
        let mut height = (220.0 * spec.ratio_hauteur_largeur) + 2.0 * spec.padding_ext_v;
        if spec.shadow_enabled && spec.shadow_height > 0.0 {
            height += spec.shadow_height;
        }
        let size = egui::vec2(220.0 + 2.0 * spec.padding_ext_h, height);
        let (rect, response) = ui.allocate_exact_size(
            size,
            egui::Sense::click().union(egui::Sense::hover()),
        );
        let is_hovered = response.hovered();
        if response.clicked() && spec.button_type == ButtonType::Switch {
            self.preview_switch_on = !self.preview_switch_on;
        }
        // Tant que l’utilisateur garde le clic enfoncé sur l’aperçu, le bouton reste en phase « appuyé ».
        let is_pressed = response.is_pointer_button_down_on()
            || (spec.button_type == ButtonType::Switch && self.preview_switch_on);
        Self::ui_button_preview_wysiwyg(
            ui,
            rect,
            spec,
            is_pressed,
            is_hovered,
            self.preview_show_disabled,
        );
        ui.add_space(8.0);
        if ui.button("Enregistrer l'élément").clicked() {
            let name = if self.new_element_name.is_empty() {
                format!("Bouton {}", next_id())
            } else {
                self.new_element_name.clone()
            };
            self.elements.push(UiElement {
                id: format!("btn_{}", next_id()),
                name,
                category: ElementCategory::Bouton,
                spec: ElementSpec::Button(self.button_spec.clone()),
            });
            self.creator_kind = CreatorKind::None;
            self.new_element_name.clear();
            self.button_spec = ButtonSpec::default();
        }
        if ui.button("Annuler").clicked() {
            self.creator_kind = CreatorKind::None;
            self.new_element_name.clear();
            self.button_spec = ButtonSpec::default();
        }
    }
}

impl ServiceUi for UiLibraryService {
    fn id(&self) -> ServiceId {
        ServiceId::UiLibrary
    }
    fn title(&self) -> &'static str {
        "Miyukini UI Builder"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Éléments UI natifs Miyukini COG");
            ui.label("Consultez les composants disponibles, filtrez par catégorie, créez de nouveaux éléments.");
            ui.add_space(12.0);

            self.ui_filter_pills(ui);
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Tableau d'affichage :");
                if ui.button("Créer un élément…").clicked() {
                    self.creator_kind = CreatorKind::Button;
                    self.button_spec = ButtonSpec::default();
                }
            });
            ui.add_space(4.0);
            self.ui_elements_table(ui);

        });

        if self.creator_kind == CreatorKind::Button {
            let mut creator_open = true;
            egui::Window::new("Créateur de bouton")
                .id(egui::Id::new("ui_library_button_creator"))
                .open(&mut creator_open)
                .collapsible(false)
                .resizable(true)
                .default_width(440.0)
                .default_height(560.0)
                .show(ui.ctx(), |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(f32::INFINITY)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            self.ui_creator_button_panel(ui);
                        });
                });
            if !creator_open {
                self.creator_kind = CreatorKind::None;
                self.new_element_name.clear();
                self.button_spec = ButtonSpec::default();
                self.preview_switch_on = false;
                self.preview_show_disabled = false;
            }
        }
    }
}