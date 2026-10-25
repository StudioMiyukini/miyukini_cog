//! @id     maia_federee_erreurs
//! @do     definir_erreurs_ia_federee
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « erreurs » (toutes signifient refus).

//! Erreurs de l'IA fédérée. Toutes signifient « refus » (fail-closed).

use miyusceau::ErreurSceau;

/// Motif de refus.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurFederee {
    /// Sceau mal formé, signature invalide, champ inconnu ou manquant.
    #[error("sceau refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// Champ hors des bornes ou de l'alphabet autorisé.
    #[error("champ invalide : {0}")]
    ChampInvalide(&'static str),
    /// Clé de l'émetteur absente, faible ou sans `cog_id` valide.
    #[error("émetteur incohérent")]
    EmetteurIncoherent,
    /// Émetteur non admis par la vue fédérale (inconnu, candidat, suspendu, révoqué, banni, vue périmée).
    #[error("émetteur refusé : {0}")]
    EmetteurRefuse(String),
    /// Fiche de son propre COG présentée comme étrangère.
    #[error("fiche locale présentée comme étrangère")]
    FicheLocale,
    /// Version antérieure, ou même version au contenu différent (modification silencieuse).
    #[error("version obsolète ou contradictoire")]
    VersionObsolete,
    /// Fiche datée dans le futur.
    #[error("fiche datée dans le futur")]
    DateFuture,
    /// Base étrangère pleine pour cet émetteur (anti-inondation).
    #[error("quota de fiches de l'émetteur atteint")]
    BasePleine,
    /// Quota de requêtes du demandeur atteint (anti-aspiration).
    #[error("quota de requêtes atteint")]
    Quota,
    /// Requête mal formée.
    #[error("requête invalide : {0}")]
    RequeteInvalide(&'static str),
    /// Publication d'une Fiche qui n'est pas signée par le COG local (F1).
    #[error("seule une fiche signée par le COG local se publie")]
    FicheNonLocale,
    /// Donnée non étiquetée `publiable`.
    #[error("donnée non publiable")]
    NonPubliable,
    /// Effet soumis à la validation explicite de l'utilisateur (EX-60).
    #[error("validation de l'utilisateur requise")]
    ValidationRequise,
    /// Effet interdit quelle que soit la validation (EX-60, EX-63).
    #[error("interdit : {0}")]
    Interdit(&'static str),
    /// Politique invalide.
    #[error("paramètre invalide : {0}")]
    ParametreInvalide(&'static str),
}
