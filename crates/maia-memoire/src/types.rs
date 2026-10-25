//! @id     maia_memoire_types
//! @do     definir_unites_memoire
//! @role   donnee
//! @layer  8
//! @human  maia-memoire — unités de mémoire, provenance, versions, candidats, pierres tombales, export (§3, §9, §10).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::erreurs::ErreurMemoire;
use crate::ident::{Branche, IdMemoire, TypeUnite};

/// Provenance obligatoire de toute unité (§3, EX-53).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Propos explicite de l'utilisateur.
    Utilisateur,
    /// Déduction de MAIA (confiance réduite, validée par l'utilisateur).
    Deduit,
    /// Contenu venu d'ailleurs (Fiches fédérées, documents relayés).
    Etranger,
}

impl Provenance {
    /// Nom stable (stockage).
    pub fn as_str(self) -> &'static str {
        match self {
            Provenance::Utilisateur => "utilisateur",
            Provenance::Deduit => "deduit",
            Provenance::Etranger => "etranger",
        }
    }

    /// Lecture du nom stable ; inconnu ⇒ refus.
    pub fn depuis_str(s: &str) -> Option<Self> {
        match s {
            "utilisateur" => Some(Provenance::Utilisateur),
            "deduit" => Some(Provenance::Deduit),
            "etranger" => Some(Provenance::Etranger),
            _ => None,
        }
    }
}

/// Étiquette de sensibilité de la mémoire (§10, EX-55) : elle ne sort jamais.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Etiquette {
    /// Ne sort jamais, ni vers la Franche ni vers la Fédération.
    Nationale,
}

impl Etiquette {
    /// Nom stable.
    pub fn as_str(self) -> &'static str {
        "nationale"
    }
}

/// Étiquette unique de toute mémoire MAIA.
pub const ETIQUETTE: Etiquette = Etiquette::Nationale;

/// Qui a produit une version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auteur {
    /// Action explicite de l'utilisateur (écriture, correction).
    Utilisateur,
    /// Candidat de MAIA validé par l'utilisateur.
    ValidationUtilisateur,
    /// Note ordinaire écrite par MAIA elle-même (A-009).
    Maia,
}

impl Auteur {
    /// Nom stable.
    pub fn as_str(self) -> &'static str {
        match self {
            Auteur::Utilisateur => "utilisateur",
            Auteur::ValidationUtilisateur => "validation_utilisateur",
            Auteur::Maia => "maia",
        }
    }

    /// Lecture du nom stable.
    pub fn depuis_str(s: &str) -> Option<Self> {
        match s {
            "utilisateur" => Some(Auteur::Utilisateur),
            "validation_utilisateur" => Some(Auteur::ValidationUtilisateur),
            "maia" => Some(Auteur::Maia),
            _ => None,
        }
    }
}

/// Empreinte d'une source N4 (SHA-256 hexadécimal) : jamais injectée, seulement référencée.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EmpreinteSource(String);

impl EmpreinteSource {
    /// Valide une empreinte (64 caractères hexadécimaux minuscules).
    pub fn parse(s: &str) -> Result<Self, ErreurMemoire> {
        if s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
            Ok(Self(s.to_string()))
        } else {
            Err(ErreurMemoire::EmpreinteInvalide)
        }
    }

    /// Texte de l'empreinte.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmpreinteSource {
    type Error = ErreurMemoire;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        EmpreinteSource::parse(&v)
    }
}

impl From<EmpreinteSource> for String {
    fn from(e: EmpreinteSource) -> Self {
        e.0
    }
}

/// Échelle 0–9 (importance, confiance).
pub fn verifier_echelle(v: u8) -> Result<u8, ErreurMemoire> {
    if v <= 9 {
        Ok(v)
    } else {
        Err(ErreurMemoire::HorsEchelle(v))
    }
}

/// Une version d'une note (§3, §9). Une note n'est jamais réécrite : chaque
/// écriture ajoute une version ; la note courante est la version la plus haute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// Identifiant hiérarchique stable.
    pub id: IdMemoire,
    /// Numéro de version (1 = création), strictement croissant.
    pub version: u32,
    /// Texte court.
    pub texte: String,
    /// Importance 0–9.
    pub importance: u8,
    /// Confiance 0–9.
    pub confiance: u8,
    /// Provenance (EX-53).
    pub provenance: Provenance,
    /// Auteur de cette version.
    pub auteur: Auteur,
    /// Identifiants liés (`↗`).
    #[serde(default)]
    pub liens: Vec<IdMemoire>,
    /// Entités citées.
    #[serde(default)]
    pub entites: Vec<IdMemoire>,
    /// Sources N4 (empreintes).
    #[serde(default)]
    pub sources: Vec<EmpreinteSource>,
    /// Création de la note (version 1).
    pub cree_le: DateTime<Utc>,
    /// Enregistrement de cette version.
    pub enregistre_le: DateTime<Utc>,
    /// Dernier usage connu (consolidation ; jamais modifié par une lecture).
    #[serde(default)]
    pub dernier_usage: Option<DateTime<Utc>>,
}

impl Note {
    /// Type d'unité (suffixe de l'identifiant).
    pub fn type_unite(&self) -> TypeUnite {
        self.id.type_unite()
    }
}

/// Proposition de MAIA en attente de validation : **hors mémoire longue** (EX-53).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidat {
    /// Identifiant du candidat.
    pub id: Uuid,
    /// Note existante à corriger (`None` : nouvelle note).
    pub cible: Option<IdMemoire>,
    /// Branche de la nouvelle note (ou de la cible).
    pub branche: Branche,
    /// Type d'unité (jamais `Regle`, EX-54).
    pub type_unite: TypeUnite,
    /// Texte proposé.
    pub texte: String,
    /// Importance proposée.
    pub importance: u8,
    /// Confiance (plafonnée selon la provenance).
    pub confiance: u8,
    /// Provenance (`deduit` ou `etranger` ; `utilisateur` pour une branche sensible).
    pub provenance: Provenance,
    /// Liens proposés.
    #[serde(default)]
    pub liens: Vec<IdMemoire>,
    /// Sources N4.
    #[serde(default)]
    pub sources: Vec<EmpreinteSource>,
    /// Branche sensible : validation TAMR requise (§9).
    pub sensible: bool,
    /// Date de proposition.
    pub propose_le: DateTime<Utc>,
}

/// Pierre tombale d'une note oubliée (§9) : aucun contenu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PierreTombale {
    /// Identifiant oublié (jamais réutilisé).
    pub id: IdMemoire,
    /// Date de l'oubli.
    pub oublie_le: DateTime<Utc>,
    /// Motif donné par l'utilisateur (sans contenu de la note).
    pub motif: String,
    /// Versions effacées.
    pub versions_effacees: u32,
}

/// Export intégral de la mémoire d'un utilisateur (EX-56).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportMemoire {
    /// Format de l'export.
    pub format: String,
    /// Étiquette (`nationale`).
    pub etiquette: Etiquette,
    /// Date de l'export.
    pub exporte_le: DateTime<Utc>,
    /// Notes courantes.
    pub notes: Vec<Note>,
    /// Historique complet (toutes versions).
    pub historique: Vec<Note>,
    /// Candidats en attente.
    pub candidats: Vec<Candidat>,
    /// Pierres tombales.
    pub pierres_tombales: Vec<PierreTombale>,
}

/// Nom du format d'export.
pub const FORMAT_EXPORT: &str = "maia-memoire/v1";
