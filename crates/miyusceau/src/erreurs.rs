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
    /// Code d'algorithme inconnu (refusé au décodage).
    AlgorithmeInconnu(u8),
    /// Algorithme réservé mais non implémenté (ML-DSA-65, hybride) : refusé.
    AlgorithmeNonPrisEnCharge(u8),
    /// Algorithme d'une signature différent de celui de l'entête.
    AlgorithmeIncoherent,
    /// Nombre de signatures hors de `1..=8`.
    NombreSignaturesInvalide(u8),
    /// Signatures non triées par `kid` croissant.
    SignaturesNonTriees,
    /// Même signataire présent deux fois.
    SignataireDuplique,
    /// Seuil demandé hors de `1..=8`.
    SeuilInvalide,
    /// Pas assez de signataires épinglés valides.
    SeuilNonAtteint {
        /// Seuil demandé.
        requis: usize,
        /// Signataires épinglés valides trouvés.
        obtenu: usize,
    },
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
            Self::AlgorithmeInconnu(a) => write!(f, "algorithme de signature inconnu : {a:#04x}"),
            Self::AlgorithmeNonPrisEnCharge(a) => write!(f, "algorithme de signature non pris en charge : {a:#04x}"),
            Self::AlgorithmeIncoherent => write!(f, "algorithme de signature incohérent avec l'entête"),
            Self::NombreSignaturesInvalide(n) => write!(f, "nombre de signatures invalide : {n}"),
            Self::SignaturesNonTriees => write!(f, "signatures non canoniques (kid non croissants)"),
            Self::SignataireDuplique => write!(f, "signataire présent deux fois"),
            Self::SeuilInvalide => write!(f, "seuil de signatures invalide"),
            Self::SeuilNonAtteint { requis, obtenu } => {
                write!(f, "seuil non atteint : {obtenu} signataire(s) valide(s) sur {requis} requis")
            }
            Self::PasEncoreValide => write!(f, "titre pas encore valide"),
            Self::Expire => write!(f, "titre expiré"),
        }
    }
}

impl std::error::Error for ErreurSceau {}

/// Preuve d'équivoque : deux têtes signées du même registre, incompatibles
/// entre elles (même numéro de séquence et contenu différent, ou histoire
/// réécrite). Les deux sceaux sont vérifiables par un tiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreuveEquivoque {
    /// Sceau de la tête acceptée précédemment.
    pub acceptee: Vec<u8>,
    /// Sceau de la tête contradictoire présentée.
    pub contradictoire: Vec<u8>,
}

/// Motif de refus d'une tête de registre, d'une preuve Merkle ou d'un état
/// persistant. Toutes les variantes signifient « refus ».
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRegistre {
    /// Refus au niveau du sceau (format, signature, seuil, validité).
    Sceau(ErreurSceau),
    /// Politique hors bornes (seuil hors `1..=8`, âge maximal nul ou au-delà
    /// du plafond).
    PolitiqueInvalide,
    /// Tête trop ancienne : le registre est considéré comme gelé (anti-gel).
    TeteGelee {
        /// Horodatage d'émission de la tête.
        emise_a: u64,
        /// Âge maximal effectif appliqué (secondes).
        age_max_secs: u64,
    },
    /// La tête appartient à un autre registre que celui attendu.
    RegistreInattendu,
    /// Tête plus ancienne que la dernière acceptée (séquence ou horodatage
    /// en recul) : retour en arrière refusé.
    RetourArriere,
    /// Deux têtes signées contradictoires : preuve d'équivoque.
    Equivoque(Box<PreuveEquivoque>),
    /// Le registre a grandi mais aucune preuve de cohérence n'a été fournie.
    CoherenceManquante,
    /// Preuve Merkle (inclusion ou cohérence) mal formée ou fausse.
    PreuveInvalide,
    /// État persistant mal formé.
    EtatInvalide,
}

impl From<ErreurSceau> for ErreurRegistre {
    fn from(e: ErreurSceau) -> Self {
        Self::Sceau(e)
    }
}

impl fmt::Display for ErreurRegistre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sceau(e) => write!(f, "tête de registre refusée : {e}"),
            Self::PolitiqueInvalide => write!(f, "politique de tête de registre invalide"),
            Self::TeteGelee { emise_a, age_max_secs } => {
                write!(f, "registre gelé : tête émise à {emise_a}, âge maximal {age_max_secs} s dépassé")
            }
            Self::RegistreInattendu => write!(f, "tête d'un autre registre"),
            Self::RetourArriere => write!(f, "tête plus ancienne que la dernière acceptée (retour en arrière)"),
            Self::Equivoque(_) => write!(f, "équivoque : deux têtes signées contradictoires"),
            Self::CoherenceManquante => write!(f, "preuve de cohérence manquante"),
            Self::PreuveInvalide => write!(f, "preuve Merkle invalide"),
            Self::EtatInvalide => write!(f, "état de registre persistant invalide"),
        }
    }
}

impl std::error::Error for ErreurRegistre {}
