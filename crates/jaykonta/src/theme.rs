//! @id     jaykonta_theme
//! @do     definir_feature_status
//! @role   orchestration
//! @layer  core
//! @human  jaykonta — module « theme ».

//! Types partagés anciennement liés au thème UI (conservés pour le domaine/backend).

/// Statut d'avancement d'une fonctionnalité (ex-UI, utilisé par le domaine).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeatureStatus {
    /// En production.
    Live,
    /// En construction.
    Build,
    /// Verrouillé / non disponible.
    Locked,
}
