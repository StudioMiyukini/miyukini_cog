//! @id     miyuhybride_erreurs
//! @do     typer_refus_services_hybrides
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — refus typés : manifeste, traité, guichets, façade jetable (tout refus = fail-closed).

use miyuportail::{ErreurPortail, RefusDecodage};
use miyusceau::ErreurSceau;
use thiserror::Error;

/// Refus d'un manifeste de service (EX-16, EX-40). Toute variante = refus d'installation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ErreurManifeste {
    /// Sceau refusé (format, type, signataire non épinglé, signature, TLV).
    #[error("sceau du manifeste refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// Aucun éditeur épinglé : rien n'est installable.
    #[error("aucun éditeur de confiance épinglé")]
    AucuneAncre,
    /// Contrat absent ou mal formé.
    #[error("contrat de service invalide : {0}")]
    Contrat(&'static str),
    /// Règle de classe d'exposition violée (SERVICES-HYBRIDES §1).
    #[error("classe d'exposition : {0}")]
    Classe(&'static str),
    /// Exposition demandée par défaut : interdit (EX-48).
    #[error("exposition activée par défaut : refusée (EX-48)")]
    ExpositionParDefaut,
    /// Traité incohérent avec le Portail.
    #[error("traité refusé : {0}")]
    Traite(ErreurPortail),
    /// Point non spécifié par la doc gelée : refus (amendement à prévoir).
    #[error("non spécifié par la doc gelée, refusé : {0}")]
    NonSpecifie(&'static str),
}

/// Refus de conversion message typé ⇄ déclaration (INV-H3, INV-H4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ErreurTraite {
    /// Type de déclaration inattendu pour ce message.
    #[error("type de déclaration inattendu")]
    Type,
    /// Sens inattendu.
    #[error("sens inattendu")]
    Sens,
    /// Champ obligatoire absent.
    #[error("champ obligatoire absent (tag {0})")]
    ChampAbsent(u8),
    /// Champ de nature différente du traité.
    #[error("champ de nature inattendue (tag {0})")]
    Nature(u8),
    /// Champ inconnu du traité.
    #[error("champ inconnu (tag {0})")]
    ChampInconnu(u8),
    /// Champ dupliqué ou tags non croissants.
    #[error("champs non canoniques")]
    NonCanonique,
}

/// Refus d'une opération moteur / façade / cycle de vie.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ErreurHybride {
    /// Refus du Portail (voie non déclarée, désactivée, coupée, hors traité…).
    #[error("portail : {0}")]
    Portail(#[from] ErreurPortail),
    /// Décodage de trame refusé.
    #[error("décodage : {0}")]
    Decodage(RefusDecodage),
    /// Message hors traité.
    #[error("traité : {0}")]
    Traite(#[from] ErreurTraite),
    /// Classe sans façade (H0) ou sans voie entrante (H1) : opération impossible.
    #[error("classe d'exposition : opération impossible")]
    Classe,
    /// Image de façade différente de celle du manifeste (INV-H5, EX-43).
    #[error("image de façade non conforme au manifeste")]
    ImageNonConforme,
    /// Jeton de réponse d'une session close (réponse périmée).
    #[error("réponse d'une session close")]
    SessionPerimee,
    /// Trop de messages dans une réponse.
    #[error("réponse trop longue")]
    ReponseTropLongue,
    /// Façade condamnée (voie coupée, désactivée, session close) : à reconstruire.
    #[error("façade condamnée : reconstruction requise")]
    FacadeCondamnee,
    /// Base secondaire pleine (taille du contrat).
    #[error("base secondaire pleine")]
    BaseSecondairePleine,
    /// Verrou interne empoisonné.
    #[error("état interne indisponible")]
    EtatIndisponible,
}

impl From<RefusDecodage> for ErreurHybride {
    fn from(r: RefusDecodage) -> Self {
        Self::Decodage(r)
    }
}
