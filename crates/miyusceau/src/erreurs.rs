//! @id     miyusceau_erreurs
//! @do     definir_erreurs_sceau
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « erreurs ».

//! Erreurs du Sceau. Toutes signifient « refus » : aucune n'est récupérable
//! en acceptant le titre.

use std::fmt;

/// Motif de refus d'un sceau ou d'erreur d'encodage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSceau {
    /// Octets trop courts, trop longs ou longueur incohérente.
    TailleInvalide,
    /// Entête « MSCE » absente.
    MagicInvalide,
    /// Version de format non supportée.
    VersionNonSupportee(u8),
    /// Code de type inconnu.
    TypeInconnu(u8),
    /// Le sceau est d'un autre type que celui attendu.
    TypeInattendu {
        /// Type attendu par l'appelant.
        attendu: u8,
        /// Type trouvé dans le sceau.
        trouve: u8,
    },
    /// Charge utile au-delà de [`crate::TAILLE_MAX_CHARGE`].
    ChargeTropGrande,
    /// Aucune ancre de confiance pour ce signataire.
    SignataireInconnu,
    /// Clé publique faible (ordre petit) ou invalide.
    CleInvalide,
    /// Signature invalide.
    SignatureInvalide,
    /// Tags TLV non strictement croissants (doublon ou désordre).
    TagsNonCroissants,
    /// Champ TLV tronqué.
    ChampTronque,
    /// Champ non prévu pour ce type de titre.
    ChampInconnu(u8),
    /// Champ obligatoire absent.
    ChampManquant(u8),
    /// Valeur de champ mal formée (taille, UTF-8, contenu).
    ValeurInvalide(u8),
    /// Titre pas encore valide.
    PasEncoreValide,
    /// Titre expiré.
    Expire,
}

impl fmt::Display for ErreurSceau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TailleInvalide => write!(f, "taille de sceau invalide"),
            Self::MagicInvalide => write!(f, "entête de sceau invalide"),
            Self::VersionNonSupportee(v) => write!(f, "version de sceau non supportée : {v}"),
            Self::TypeInconnu(t) => write!(f, "type de sceau inconnu : {t:#04x}"),
            Self::TypeInattendu { attendu, trouve } => {
                write!(f, "type de sceau inattendu : {trouve:#04x} au lieu de {attendu:#04x}")
            }
            Self::ChargeTropGrande => write!(f, "charge utile trop grande"),
            Self::SignataireInconnu => write!(f, "signataire inconnu (aucune ancre de confiance)"),
            Self::CleInvalide => write!(f, "clé publique invalide ou faible"),
            Self::SignatureInvalide => write!(f, "signature invalide"),
            Self::TagsNonCroissants => write!(f, "champs non canoniques (tags non croissants)"),
            Self::ChampTronque => write!(f, "champ tronqué"),
            Self::ChampInconnu(t) => write!(f, "champ inconnu : {t}"),
            Self::ChampManquant(t) => write!(f, "champ obligatoire manquant : {t}"),
            Self::ValeurInvalide(t) => write!(f, "valeur invalide pour le champ {t}"),
            Self::PasEncoreValide => write!(f, "titre pas encore valide"),
            Self::Expire => write!(f, "titre expiré"),
        }
    }
}

impl std::error::Error for ErreurSceau {}
