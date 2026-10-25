//! @id     miyufederation_erreurs
//! @do     definir_erreurs_federation
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « erreurs » (refus de la fédération, tous fail-closed).

//! Erreurs de la fédération. Toutes signifient « refus » : aucune ne se
//! récupère en acceptant la donnée fautive ; aucun état n'est modifié par une
//! opération refusée (sauf la note locale, qui enregistre la faute).

use miyusceau::{ErreurRegistre, ErreurSceau};

use crate::preuve::Preuve;
use crate::statut::Statut;

/// Motif de refus.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ErreurFederation {
    /// Sceau refusé (format, signature, seuil, validité).
    #[error("sceau refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// Tête de registre refusée (fraîcheur, registre attendu, monotonie).
    #[error("tête refusée : {0}")]
    Registre(ErreurRegistre),
    /// Équivoque détectée : deux têtes signées contradictoires (preuve jointe).
    #[error("équivoque : deux têtes signées contradictoires")]
    Equivoque(Box<Preuve>),
    /// Paramètre ou politique hors bornes.
    #[error("paramètre invalide : {0}")]
    ParametreInvalide(&'static str),
    /// Preuve de calcul absente, insuffisante ou d'un autre jour.
    #[error("preuve de calcul insuffisante")]
    CalculInsuffisant,
    /// COG inconnu de la fédération locale.
    #[error("COG inconnu")]
    Inconnu,
    /// COG révoqué (blacklisté) : définitif.
    #[error("COG révoqué")]
    Revoque,
    /// Même racine présentée avec une autre clé de journal ou un autre registre.
    #[error("candidature contradictoire pour une racine déjà connue")]
    Contradictoire,
    /// Le confirmateur ou le signataire n'est pas membre.
    #[error("signataire non membre")]
    NonMembre,
    /// Un COG ne se confirme pas lui-même.
    #[error("auto-confirmation refusée")]
    AutoConfirmation,
    /// Quota de confirmations du membre pour la période atteint.
    #[error("quota de confirmations atteint")]
    QuotaDepasse,
    /// Preuve de faute fausse ou ne prouvant rien.
    #[error("preuve invalide")]
    PreuveInvalide,
    /// Empreinte de la preuve différente de celle de l'accusation signée.
    #[error("preuve ne correspondant pas à l'accusation")]
    PreuveNonConcordante,
    /// Quorum de membres indépendants non atteint.
    #[error("quorum non atteint : {obtenu} membre(s) sur {requis}")]
    QuorumNonAtteint {
        /// Quorum exigé.
        requis: usize,
        /// Membres valides comptés.
        obtenu: usize,
    },
    /// Témoins (contresignatures de membres distincts) insuffisants.
    #[error("témoins insuffisants : {obtenu} sur {requis}")]
    TemoinsInsuffisants {
        /// Témoins exigés.
        requis: usize,
        /// Témoins valides comptés.
        obtenu: usize,
    },
    /// Heure de confiance en recul.
    #[error("heure en recul")]
    RetourHorloge,
    /// Vue fédérale (révocations) trop ancienne : accès fédéré refusé (EX-35).
    #[error("vue fédérale périmée")]
    VuePerimee,
    /// Statut insuffisant pour l'opération.
    #[error("statut insuffisant : {0:?}")]
    StatutInsuffisant(Statut),
    /// Pair banni localement (note locale).
    #[error("pair banni localement")]
    Banni,
    /// COG suspendu (accusation au quorum, délai de contestation en cours).
    #[error("COG suspendu")]
    Suspendu,
    /// Aucune suspension en cours pour ce COG (recours sans objet).
    #[error("aucune suspension en cours")]
    AucuneSuspension,
    /// Accusation déjà levée par un recours accepté (rejeu refusé).
    #[error("accusation levée par un recours accepté")]
    AccusationLevee,
}

impl From<ErreurRegistre> for ErreurFederation {
    fn from(e: ErreurRegistre) -> Self {
        match e {
            ErreurRegistre::Equivoque(p) => Self::Equivoque(Box::new(Preuve::depuis_equivoque(&p))),
            autre => Self::Registre(autre),
        }
    }
}
