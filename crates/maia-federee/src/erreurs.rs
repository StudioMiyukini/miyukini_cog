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
    /// Fiche signée par la clé `fiches` plus ancienne que la borne de
    /// fraîcheur d'admission ([`crate::fiche::FRAICHEUR_ADMISSION_SECS`]).
    #[error("fiche trop ancienne pour être admise (à republier)")]
    FichePerimee,
    /// Relecture d'une Fiche absente de la base étrangère (ou d'un autre
    /// contenu) : une relecture n'admet rien de neuf.
    #[error("fiche non admise : relecture refusée")]
    NonAdmise,
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
    /// Fiche non dérivée de la donnée étiquetée (règle de dérivation stricte, EX-47).
    #[error("fiche non dérivée de la donnée étiquetée : {0}")]
    NonDerivee(&'static str),
    /// Effet soumis à la validation explicite de l'utilisateur (EX-60).
    #[error("validation de l'utilisateur requise")]
    ValidationRequise,
    /// Effet interdit quelle que soit la validation (EX-60, EX-63).
    #[error("interdit : {0}")]
    Interdit(&'static str),
    /// Chaîne de signature de la clé `fiches` refusée (délégation `0x45`
    /// absente du mode strict, invalide, expirée, d'un autre usage ou d'un
    /// autre émetteur ; A-022 §4, A-024).
    #[error("délégation de la clé « fiches » refusée : {0}")]
    Delegation(&'static str),
    /// Politique invalide.
    #[error("paramètre invalide : {0}")]
    ParametreInvalide(&'static str),
}
