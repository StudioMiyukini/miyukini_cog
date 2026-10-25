//! @id     maia_memoire_erreurs
//! @do     declarer_erreurs_memoire
//! @role   securite
//! @layer  8
//! @human  maia-memoire — erreurs typées ; tout cas non prévu est un refus (fail-closed, C-12).

use uuid::Uuid;

/// Erreurs de la mémoire de MAIA.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ErreurMemoire {
    /// Identifiant hiérarchique invalide (§4).
    #[error("identifiant mémoire invalide : {0}")]
    IdInvalide(String),

    /// Texte vide, multiligne ou contenant des caractères de contrôle.
    #[error("texte invalide : {0}")]
    TexteInvalide(&'static str),

    /// Texte au-delà du plafond de sa strate (§2), mesuré par le tokenizer actif.
    #[error("texte trop long pour la strate {strate} : {jetons} jetons > {max}")]
    TexteTropLong {
        /// Strate visée.
        strate: &'static str,
        /// Jetons mesurés.
        jetons: usize,
        /// Plafond.
        max: usize,
    },

    /// Échelle importance / confiance hors 0–9.
    #[error("valeur hors échelle 0–9 : {0}")]
    HorsEchelle(u8),

    /// Empreinte de source (N4) invalide.
    #[error("empreinte de source invalide (SHA-256 hexadécimal attendu)")]
    EmpreinteInvalide,

    /// Le budget ne permet même pas N0 (EX-50 : N0 toujours présent).
    #[error("budget de {budget} jetons insuffisant : N0 seul en demande {requis}")]
    BudgetInsuffisant {
        /// Budget demandé.
        budget: usize,
        /// Jetons requis par le bloc minimal (N0).
        requis: usize,
    },

    /// Une Règle ne peut être posée que par l'utilisateur (EX-54).
    #[error("seul l'utilisateur peut créer une règle")]
    RegleReserveeUtilisateur,

    /// Provenance refusée pour cette opération (EX-53).
    #[error("provenance refusée : {0}")]
    ProvenanceRefusee(&'static str),

    /// Validation TAMR requise (type sensible, §9).
    #[error("validation TAMR requise pour la branche sensible {0}")]
    TamrRequis(String),

    /// Plafond des candidats de MAIA atteint (en attente ou sur 24 h) : refus
    /// explicite, rien n'est proposé (revue de sécurité du lot 7, F5).
    #[error("plafond des propositions de MAIA atteint : {0}")]
    CandidatsPlafond(&'static str),

    /// Candidat inconnu (déjà validé, rejeté ou jamais proposé).
    #[error("candidat inconnu : {0}")]
    CandidatInconnu(Uuid),

    /// Note inconnue.
    #[error("note inconnue : {0}")]
    NoteInconnue(String),

    /// Identifiant oublié : la pierre tombale fait foi, il n'est jamais réutilisé.
    #[error("identifiant oublié (pierre tombale) : {0}")]
    NoteOubliee(String),

    /// Demande d'oubli invalide (motif vide ou trop long, rien à oublier).
    #[error("demande d'oubli invalide : {0}")]
    OubliInvalide(&'static str),

    /// Base sans l'étiquette `nationale` (EX-55).
    #[error("étiquette de la mémoire invalide : {0:?} (attendu « nationale »)")]
    EtiquetteInvalide(String),

    /// Base non chiffrée (EX-55).
    #[error("mémoire non chiffrée : refus (EX-55)")]
    NonChiffree,

    /// Écriture concurrente (version déjà prise) ou contrainte violée.
    #[error("conflit d'écriture : {0}")]
    Conflit(String),

    /// Erreur de l'entrepôt (KindMother injoignable, refus, ligne illisible).
    #[error("entrepôt : {0}")]
    Entrepot(String),
}
