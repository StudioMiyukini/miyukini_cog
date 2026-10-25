//! @id     miyucanal_erreurs
//! @do     definir_erreurs_canal
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — module « erreurs » (refus du canal officiel).

//! Erreurs du canal officiel. Toutes signifient « refus » : rien n'est
//! installé, l'état du relevé est inchangé.

use miyuregistre::{ErreurJournal, ErreurTemps};
use miyusceau::ErreurSceau;

/// Panne ou refus du transport (voie du Portail, dépôt en mémoire).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurTransport {
    /// Le dépôt n'a pas l'objet demandé.
    #[error("objet absent du dépôt")]
    Absent,
    /// Voie indisponible (coupée, désactivée, délai dépassé).
    #[error("voie indisponible : {0}")]
    Indisponible(String),
}

/// Refus d'un relevé du canal officiel.
#[derive(Debug, thiserror::Error)]
pub enum ErreurCanal {
    /// Configuration hors bornes (quarantaine trop courte, clés identiques…).
    #[error("configuration du canal invalide : {0}")]
    Configuration(&'static str),
    /// Transport.
    #[error("transport : {0}")]
    Transport(#[from] ErreurTransport),
    /// Réponse du dépôt mal formée ou incohérente avec la demande.
    #[error("réponse du dépôt invalide")]
    ReponseInvalide,
    /// Refus d'un sceau (tête, annonce).
    #[error("sceau refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// Refus du registre officiel (tête gelée, retour arrière, équivoque,
    /// racine différente, preuve invalide…).
    #[error("registre officiel : {0}")]
    Registre(#[from] ErreurJournal),
    /// Refus du temps de confiance (horloge antérieure au Median-Time-Past).
    #[error(transparent)]
    Temps(#[from] ErreurTemps),
    /// Annonce visée par une révocation du canal.
    #[error("annonce révoquée")]
    Revoquee,
    /// Le registre a grandi au-delà de ce qu'un relevé accepte.
    #[error("trop d'entrées nouvelles : {0}")]
    TropDEntrees(u64),
}

impl From<miyusceau::ErreurRegistre> for ErreurCanal {
    fn from(e: miyusceau::ErreurRegistre) -> Self {
        Self::Registre(ErreurJournal::Registre(e))
    }
}
