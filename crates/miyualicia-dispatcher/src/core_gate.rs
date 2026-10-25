//! @id     service.alicia.dispatcher.core_gate
//! @do     declarer_core_gate
//! @role   cores_governance_gate
//! @layer  7
//! @human  Gate cores d'Alicia : la décision d'exécuter une capability vient de StrongFather (K-11, K-16).
//!
//! Gate **cores** — validation de gouvernance avant qu'Alicia n'exécute une
//! capability hors-limites.
//!
//! Cf. mémoire projet `Alicia — modifications hors-limites validées par les
//! cores` : toute écriture qu'Alicia fait hors de son propre périmètre doit
//! traverser cet arbitrage.
//!
//! ## Implémentations
//!
//! - [`StrongFatherCoreGate`] : **gate de production**. Construit une
//!   intention StrongFather (voir [`intention_alicia`]) et la soumet à la porte
//!   obligatoire BondingBrother (`executer_si_autorise`). Politiques chargées
//!   depuis `config/strongfather/alicia.toml` (répertoire surchargeable par
//!   `COG_POLITIQUES_DIR`) ; fichier absent ou invalide ⇒ la gate refuse tout
//!   (fail-closed, C-12), jamais « tout autoriser »
//! - BorderGuard (crate `miyualicia-cores-gate`) peut **alimenter** l'intention
//!   via un [`EnrichisseurIntention`] (niveau de confiance, verdict de
//!   franchissement) ; la décision reste celle de StrongFather (K-16)
//! - `BypassCoreGate` : n'existe que sous `cfg(test)` ou la feature
//!   `dev-bypass` (désactivée par défaut)
//!
//! ## Forme de l'intention (convention COG, partagée avec MAIA / Market / Admin)
//!
//! - `action` : `read` (capability sans écriture) ou `modification` (`writes = true`)
//! - `subject` : `alicia.skill.<service_id>.<intent>` (minuscules)
//! - `caller` : `local` (principal humain non connu à ce niveau) ; `origin` : `alicia`
//! - `data.service`, `data.intent`, `data.skill` (qualified_id), `data.ecriture`
//!   (`true`/`false`), `data.persona`, `data.confirmation` (politique UX de la capability),
//!   `data.categorie` (vide si absente), plus les clés `borderguard_*` éventuelles

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use bondingbrother::strongfather::{ActionType, CallContext, IntentData};
use bondingbrother::{executer_si_autorise, Intent, PorteStrongFather};
use serde_json::Value;

use miyualicia_skills::{ConfirmationPolicy, ServiceCapability};

/// Variable d'environnement surchargeant le répertoire des politiques StrongFather.
pub const ENV_POLITIQUES_DIR: &str = "COG_POLITIQUES_DIR";
/// Répertoire par défaut des politiques StrongFather (relatif au répertoire courant).
pub const POLITIQUES_DIR_DEFAUT: &str = "config/strongfather";
/// Nom du fichier de politiques du domaine Alicia.
pub const FICHIER_POLITIQUES_ALICIA: &str = "alicia.toml";
/// Origine des intentions Alicia.
pub const ORIGINE_ALICIA: &str = "alicia";
/// Appelant par défaut quand le principal humain n'est pas connu.
pub const APPELANT_LOCAL: &str = "local";

/// Décision rendue par les cores sur une demande d'exécution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreGateOutcome {
    /// Les cores autorisent l'exécution.
    Allowed,
    /// Les cores refusent — la `String` porte le motif loggable / affichable.
    Denied(String),
}

/// Trait async décrivant la traversée des cores avant exécution.
///
/// Une vraie implémentation (P3+) traversera :
/// - `BorderGuard` pour le contrôle d'accès (qui parle, à quel service)
/// - `StrongFather` pour la décision arbitrale (politique en vigueur)
/// - le core métier compétent selon la nature de l'action (`writes`, données
///   touchées, etc.)
///
/// L'API reste minimale en P2 : capability + params + persona, retour
/// allow/deny. Les détails plus fins (raison structurée, suggestion
/// alternative...) viendront avec les vraies impls.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait CoreGate: Send + Sync {
    /// Soumet la demande aux cores. Doit être idempotente côté lecture
    /// (ne pas modifier l'état).
    async fn validate(
        &self,
        capability: &ServiceCapability,
        params: &Value,
        persona: &str,
    ) -> CoreGateOutcome;
}

/// Implémentation **bypass** — autorise toujours. N'existe que sous
/// `cfg(test)` ou la feature `dev-bypass` (K-16).
///
/// **À n'utiliser que** :
/// - quand `AliciaPolicy.bypass_cores_validation == true` (dev / sandbox)
/// - dans les tests unitaires qui ne valident pas la gouvernance
///
/// Le dispatcher ne sélectionne pas automatiquement ce gate — c'est au caller
/// de fournir l'instance correcte selon `AliciaPolicy`.
#[cfg(any(test, feature = "dev-bypass"))]
#[derive(Debug, Clone, Copy, Default)]
pub struct BypassCoreGate;

#[cfg(any(test, feature = "dev-bypass"))]
#[async_trait]
impl CoreGate for BypassCoreGate {
    async fn validate(
        &self,
        _capability: &ServiceCapability,
        _params: &Value,
        _persona: &str,
    ) -> CoreGateOutcome {
        tracing::warn!("BypassCoreGate utilisé — gouvernance cores court-circuitée");
        CoreGateOutcome::Allowed
    }
}

/// @id service.alicia.dispatcher.enrichisseur_intention
/// @role data_provider
/// @layer 7
/// @do enrich_strongfather_intent
//@human / @human Fournit des données d'intention supplémentaires (ex. BorderGuard) sans décider.
pub trait EnrichisseurIntention: Send + Sync {
    /// Paires `clé → valeur` ajoutées à `data` de l'intention. Les clés
    /// canoniques (`service`, `intent`, `skill`, `ecriture`, `persona`,
    /// `confirmation`, `categorie`) ne peuvent pas être écrasées.
    fn donnees(&self, capability: &ServiceCapability, persona: &str) -> Vec<(String, String)>;
}

/// @id service.alicia.dispatcher.chemin_politiques
/// @role config
/// @layer 7
/// @do resolve_alicia_policy_path
//@human / @human Chemin du fichier de politiques Alicia : répertoire explicite, sinon
//@human `COG_POLITIQUES_DIR`, sinon `config/strongfather` ; fichier `alicia.toml`.
pub fn chemin_politiques_alicia(repertoire: Option<&Path>) -> PathBuf {
    let base = match repertoire {
        Some(r) => r.to_path_buf(),
        None => std::env::var_os(ENV_POLITIQUES_DIR)
            .filter(|v| !v.is_empty())
            .map_or_else(|| PathBuf::from(POLITIQUES_DIR_DEFAUT), PathBuf::from),
    };
    base.join(FICHIER_POLITIQUES_ALICIA)
}

fn nom_confirmation(c: ConfirmationPolicy) -> &'static str {
    match c {
        ConfirmationPolicy::Never => "never",
        ConfirmationPolicy::Always => "always",
        ConfirmationPolicy::AlwaysForWrites => "always_for_writes",
        ConfirmationPolicy::UserPreference => "user_preference",
    }
}

/// @id service.alicia.dispatcher.intention_alicia
/// @role builder
/// @layer 7
/// @do build_alicia_intent
//@human / @human Construit l'intention StrongFather d'une exécution de capability Alicia.
#[must_use]
pub fn intention_alicia(
    capability: &ServiceCapability,
    persona: &str,
    supplementaires: Vec<(String, String)>,
) -> Intent {
    let skill = capability.qualified_id();
    let mut content: HashMap<String, String> = supplementaires.into_iter().collect();
    let canoniques = [
        ("service", capability.service_id.clone()),
        ("intent", capability.intent.clone()),
        ("skill", skill.clone()),
        ("ecriture", capability.writes.to_string()),
        ("persona", persona.to_string()),
        (
            "confirmation",
            nom_confirmation(capability.confirmation_default).to_string(),
        ),
        ("categorie", capability.category.clone().unwrap_or_default()),
    ];
    for (k, v) in canoniques {
        content.insert(k.to_string(), v);
    }
    let action = if capability.writes {
        ActionType::Modification
    } else {
        ActionType::Read
    };
    Intent::new(
        format!("alicia-{}", uuid::Uuid::new_v4()),
        action,
        format!("alicia.skill.{}", skill.to_lowercase()),
        CallContext {
            caller_identity: APPELANT_LOCAL.to_string(),
            origin: ORIGINE_ALICIA.to_string(),
            instance: "local".to_string(),
        },
        IntentData { content },
    )
}

/// @id service.alicia.dispatcher.strongfather_core_gate
/// @role cores_governance_gate
/// @layer 7
/// @do gate_alicia_with_strongfather
//@human / @human Gate cores de production : StrongFather décide (porte BondingBrother).
//@human Politiques indisponibles ⇒ tout est refusé (fail-closed).
#[derive(Clone)]
pub struct StrongFatherCoreGate {
    porte: Result<PorteStrongFather, String>,
    enrichisseurs: Vec<Arc<dyn EnrichisseurIntention>>,
}

impl std::fmt::Debug for StrongFatherCoreGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrongFatherCoreGate")
            .field("operationnelle", &self.porte.is_ok())
            .field("enrichisseurs", &self.enrichisseurs.len())
            .finish()
    }
}

impl StrongFatherCoreGate {
    /// Gate adossée à une porte déjà construite.
    #[must_use]
    pub fn new(porte: PorteStrongFather) -> Self {
        Self {
            porte: Ok(porte),
            enrichisseurs: Vec::new(),
        }
    }

    /// Gate chargée depuis un fichier de politiques (TOML / JSON). Fichier
    /// absent ou invalide ⇒ gate **indisponible** qui refuse tout (journalisé).
    #[must_use]
    pub fn depuis_fichier(chemin: impl AsRef<Path>) -> Self {
        let chemin = chemin.as_ref();
        match PorteStrongFather::depuis_fichier(chemin) {
            Ok(porte) => {
                tracing::info!(chemin = %chemin.display(), "politiques StrongFather Alicia chargées");
                Self::new(porte)
            }
            Err(e) => {
                tracing::error!(
                    chemin = %chemin.display(),
                    "politiques StrongFather Alicia indisponibles ({e}) — toute capability sera refusée"
                );
                Self::indisponible(format!("{} : {e}", chemin.display()))
            }
        }
    }

    /// Gate chargée depuis le chemin par défaut ([`chemin_politiques_alicia`]).
    #[must_use]
    pub fn depuis_repertoire(repertoire: Option<&Path>) -> Self {
        Self::depuis_fichier(chemin_politiques_alicia(repertoire))
    }

    /// Gate sans politiques : refuse tout avec le motif donné.
    #[must_use]
    pub fn indisponible(motif: impl Into<String>) -> Self {
        Self {
            porte: Err(motif.into()),
            enrichisseurs: Vec::new(),
        }
    }

    /// Ajoute une source de données d'intention (ex. BorderGuard).
    #[must_use]
    pub fn avec_enrichisseur(mut self, enrichisseur: Arc<dyn EnrichisseurIntention>) -> Self {
        self.enrichisseurs.push(enrichisseur);
        self
    }

    /// `true` si des politiques valides sont chargées.
    #[must_use]
    pub fn est_operationnelle(&self) -> bool {
        self.porte.is_ok()
    }

    /// Intention soumise à StrongFather pour cette demande.
    #[must_use]
    pub fn intention(&self, capability: &ServiceCapability, persona: &str) -> Intent {
        let supplementaires = self
            .enrichisseurs
            .iter()
            .flat_map(|e| e.donnees(capability, persona))
            .collect();
        intention_alicia(capability, persona, supplementaires)
    }
}

#[async_trait]
impl CoreGate for StrongFatherCoreGate {
    async fn validate(
        &self,
        capability: &ServiceCapability,
        _params: &Value,
        persona: &str,
    ) -> CoreGateOutcome {
        let porte = match &self.porte {
            Ok(p) => p,
            Err(motif) => {
                return CoreGateOutcome::Denied(format!(
                    "StrongFather indisponible (politiques non chargées : {motif}) — refus"
                ));
            }
        };
        let intent = self.intention(capability, persona);
        let qualified_id = capability.qualified_id();
        match executer_si_autorise(porte, &intent, |jeton| {
            tracing::info!(
                %qualified_id,
                persona = %persona,
                regle = %jeton.regle(),
                empreinte = %jeton.empreinte(),
                "StrongFather autorise la capability"
            );
        }) {
            Ok(()) => CoreGateOutcome::Allowed,
            Err(refus) => {
                tracing::info!(
                    %qualified_id,
                    persona = %persona,
                    regle = %refus.regle(),
                    "StrongFather refuse la capability"
                );
                CoreGateOutcome::Denied(refus.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia_skills::{ConfirmationPolicy, ServiceCapability};
    use std::collections::BTreeMap;

    fn cap() -> ServiceCapability {
        ServiceCapability {
            service_id: "jaykoa".into(),
            intent: "schedule_event".into(),
            phrases_examples: vec![],
            params: BTreeMap::new(),
            writes: true,
            confirmation_default: ConfirmationPolicy::AlwaysForWrites,
            description: None,
            category: None,
        }
    }

    #[tokio::test]
    async fn bypass_always_allows() {
        let gate = BypassCoreGate;
        let outcome = gate
            .validate(&cap(), &serde_json::Value::Null, "voice")
            .await;
        assert_eq!(outcome, CoreGateOutcome::Allowed);
    }
}
