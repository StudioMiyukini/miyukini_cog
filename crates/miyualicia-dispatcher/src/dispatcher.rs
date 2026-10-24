//! Orchestrateur central — `Dispatcher::execute` est la **seule** porte que
//! le runtime Alicia doit utiliser pour faire exécuter une capability.
//!
//! Étapes (dans l'ordre, P2) :
//! 1. **Résolution** dans le `SkillRegistry`
//! 2. **Gate cores** : `policy.requires_cores_check()` → `core_gate.validate(...)`
//! 3. **Confirmation utilisateur** : `policy.requires_user_confirmation(...)` →
//!    `Decision::Pending`. Le caller (UI) doit alors interroger l'utilisateur
//!    et rappeler `execute_confirmed(...)`.
//! 4. **Exécution** : `service_client.call(qualified_id, params)`
//! 5. **Audit** : `MemoryStore::record_episode` (toujours, succès comme échec).
//!
//! Le dispatcher ne décide pas tout seul de bypasser cores ou confirmation —
//! c'est l'`AliciaPolicy` injectée qui le pilote. La règle « pas d'écriture
//! hors-limites Alicia sans cores » est tenue par construction sauf bypass
//! explicitement activé dans la policy.

// @id service.alicia.dispatcher.dispatcher
// @role skill_dispatcher
// @layer 7
// @do dispatch_skill_execution
// @human Orchestrateur Dispatcher::execute — résolution, gates, exécution, audit.

use serde_json::Value;

use miyualicia_memory::{Episode, EpisodeOutcome, MemoryStore};
use miyualicia_skills::{AliciaPolicy, ConfirmationPolicy, ServiceCapability, SkillRegistry};
use uuid::Uuid;

use crate::core_gate::{CoreGate, CoreGateOutcome};
use crate::errors::DispatchError;
use crate::service_client::{ServiceCallResult, ServiceClient};

/// État rendu par `Dispatcher::execute` : soit l'action est partie au bout,
/// soit elle attend une confirmation utilisateur (à reprendre via
/// `execute_confirmed`).
#[derive(Debug, Clone)]
pub enum Decision {
    /// Exécution effectuée, résultat retourné.
    Executed(ExecutionResult),
    /// L'`AliciaPolicy` exige une confirmation utilisateur. Le dispatcher
    /// **n'a rien fait** côté service ni côté audit. Le caller doit obtenir
    /// le consentement utilisateur puis appeler `execute_confirmed`.
    PendingConfirmation {
        /// Identifiant qualifié de la capability.
        qualified_id: String,
        /// Description courte (description capability ou intent).
        prompt: String,
    },
}

/// Résultat d'une exécution complète.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Identifiant qualifié.
    pub qualified_id: String,
    /// Résultat brut renvoyé par le service.
    pub service_result: ServiceCallResult,
}

/// Tool call résolu depuis une **skill apprise** (P5) — équivalent
/// `ResolvedToolCall` mais issu de la mémoire d'Alicia plutôt que d'un LLM.
///
/// Le runtime Alicia consomme ça : si une phrase utilisateur matche un
/// trigger appris, on saute le LLM et on dispatche directement avec les
/// `params` du template (que le caller peut éventuellement enrichir avant
/// `Dispatcher::execute`).
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedLearnedCall {
    /// Identifiant de la `LearnedSkill` ayant matché.
    pub learned_skill_id: Uuid,
    /// Identifiant qualifié de la capability cible (`service_id.intent`).
    pub qualified_id: String,
    /// Paramètres template (JSON) à passer à la capability.
    pub params: Value,
    /// Phrase déclencheuse de la `LearnedSkill` (pour audit / affichage).
    pub trigger_phrase: String,
}

/// Le dispatcher.
///
/// Il ne stocke pas d'état runtime — ce sont les références passées à
/// `execute` qui portent le contexte.
pub struct Dispatcher;

impl Dispatcher {
    /// Construit un dispatcher (stateless).
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Exécute une capability identifiée par `qualified_id`.
    ///
    /// Si `AliciaPolicy::requires_user_confirmation(...)` retourne `true`,
    /// renvoie `Decision::PendingConfirmation` **sans appeler ni cores ni
    /// service**. Le caller doit ensuite appeler `execute_confirmed` après
    /// accord utilisateur.
    pub async fn execute(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
        registry: &SkillRegistry,
        policy: &AliciaPolicy,
        core_gate: &dyn CoreGate,
        service_client: &dyn ServiceClient,
        memory: &MemoryStore,
    ) -> Result<Decision, DispatchError> {
        let capability = self.resolve(qualified_id, registry)?;

        if self.should_confirm(&capability, persona, policy, memory)? {
            let description = capability
                .description
                .clone()
                .unwrap_or_else(|| capability.intent.clone());
            // L'utilisateur confirme ce qui sera réellement fait (R-018).
            let prompt = match service_client.confirmation_preview(&capability.qualified_id(), params) {
                None => description,
                Some(Ok(apercu)) => format!("{description}\n{apercu}"),
                Some(Err(motif)) => return Err(DispatchError::ParamsRefused(motif)),
            };
            return Ok(Decision::PendingConfirmation {
                qualified_id: capability.qualified_id(),
                prompt,
            });
        }

        let result = self
            .run_after_consent(
                &capability,
                params,
                persona,
                user_phrase,
                policy,
                core_gate,
                service_client,
                memory,
            )
            .await?;
        Ok(Decision::Executed(result))
    }

    /// Reprise après que l'utilisateur a explicitement accepté la confirmation.
    /// Saute la gate UX, garde la gate cores et l'audit.
    pub async fn execute_confirmed(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
        registry: &SkillRegistry,
        policy: &AliciaPolicy,
        core_gate: &dyn CoreGate,
        service_client: &dyn ServiceClient,
        memory: &MemoryStore,
    ) -> Result<ExecutionResult, DispatchError> {
        let capability = self.resolve(qualified_id, registry)?;
        self.run_after_consent(
            &capability,
            params,
            persona,
            user_phrase,
            policy,
            core_gate,
            service_client,
            memory,
        )
        .await
    }

    /// Reprise quand l'utilisateur a refusé. Enregistre un épisode `Cancelled`.
    pub fn execute_cancelled(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
        memory: &MemoryStore,
    ) -> Result<(), DispatchError> {
        let (service_id, intent) =
            miyualicia_skills::split_qualified_id(qualified_id).map_err(|_| {
                DispatchError::UnknownCapability(qualified_id.to_string())
            })?;
        let ep = Episode::new(
            persona.to_string(),
            service_id,
            intent,
            user_phrase.to_string(),
            params.clone(),
            EpisodeOutcome::Cancelled,
            None,
        );
        memory.record_episode(&ep)?;
        Ok(())
    }

    /// Cherche dans la mémoire une skill apprise dont le `trigger_phrase`
    /// matche la phrase utilisateur. Sert de raccourci avant d'envoyer la
    /// phrase au LLM pour tool calling.
    ///
    /// Retourne :
    /// - `Some(ResolvedLearnedCall)` si une skill apprise active matche →
    ///   le caller peut directement appeler `execute(...)` avec ces valeurs
    /// - `None` si aucune correspondance → fallback LLM nécessaire
    pub fn resolve_learned_phrase(
        &self,
        phrase: &str,
        persona: &str,
        memory: &MemoryStore,
    ) -> Result<Option<ResolvedLearnedCall>, DispatchError> {
        match memory.match_learned_skill(persona, phrase)? {
            None => Ok(None),
            Some(skill) => Ok(Some(ResolvedLearnedCall {
                learned_skill_id: skill.id,
                qualified_id: skill.qualified_id,
                params: skill.params_template,
                trigger_phrase: skill.trigger_phrase,
            })),
        }
    }

    /// Décide si la capability doit déclencher une confirmation utilisateur.
    ///
    /// Pipeline :
    /// 1. `policy.bypass_user_confirmations` → `false` immédiat
    /// 2. `confirmation_default` :
    ///    - `Never` → `false`
    ///    - `Always` → `true`
    ///    - `AlwaysForWrites` → `capability.writes`
    ///    - `UserPreference` → consulte `MemoryStore::confirmation_preference`
    ///      (scope persona puis global) ; défaut conservateur `true` si rien.
    pub fn should_confirm(
        &self,
        capability: &ServiceCapability,
        persona: &str,
        policy: &AliciaPolicy,
        memory: &MemoryStore,
    ) -> Result<bool, DispatchError> {
        if policy.bypass_user_confirmations {
            return Ok(false);
        }
        let needs = match capability.confirmation_default {
            ConfirmationPolicy::Never => false,
            ConfirmationPolicy::Always => true,
            ConfirmationPolicy::AlwaysForWrites => capability.writes,
            ConfirmationPolicy::UserPreference => memory
                .confirmation_preference(persona, &capability.qualified_id())?
                .unwrap_or(true),
        };
        Ok(needs)
    }

    fn resolve(
        &self,
        qualified_id: &str,
        registry: &SkillRegistry,
    ) -> Result<ServiceCapability, DispatchError> {
        let skill = registry
            .get(qualified_id)
            .ok_or_else(|| DispatchError::UnknownCapability(qualified_id.to_string()))?;
        Ok(skill.capability.clone())
    }

    #[allow(clippy::too_many_arguments)]
    async fn run_after_consent(
        &self,
        capability: &ServiceCapability,
        params: &Value,
        persona: &str,
        user_phrase: &str,
        policy: &AliciaPolicy,
        core_gate: &dyn CoreGate,
        service_client: &dyn ServiceClient,
        memory: &MemoryStore,
    ) -> Result<ExecutionResult, DispatchError> {
        // Gate cores
        if policy.requires_cores_check() {
            match core_gate.validate(capability, params, persona).await {
                CoreGateOutcome::Allowed => {}
                CoreGateOutcome::Denied(reason) => {
                    let ep = Episode::new(
                        persona.to_string(),
                        capability.service_id.clone(),
                        capability.intent.clone(),
                        user_phrase.to_string(),
                        params.clone(),
                        EpisodeOutcome::Failed,
                        Some(format!("cores denied: {reason}")),
                    );
                    memory.record_episode(&ep)?;
                    return Err(DispatchError::CoresDenied(reason));
                }
            }
        } else {
            tracing::warn!(
                qualified_id = %capability.qualified_id(),
                persona = %persona,
                "execution sans validation cores (bypass actif)"
            );
        }

        // Exécution
        let qualified_id = capability.qualified_id();
        let result = service_client.call(&qualified_id, params).await;

        // Audit
        let outcome = if result.success {
            EpisodeOutcome::Ok
        } else {
            EpisodeOutcome::Failed
        };
        let error = if result.success {
            None
        } else {
            result.message.clone()
        };
        let ep = Episode::new(
            persona.to_string(),
            capability.service_id.clone(),
            capability.intent.clone(),
            user_phrase.to_string(),
            params.clone(),
            outcome,
            error,
        );
        memory.record_episode(&ep)?;

        if !result.success {
            return Err(DispatchError::ServiceFailed {
                service: capability.service_id.clone(),
                intent: capability.intent.clone(),
                message: result.message.clone().unwrap_or_default(),
            });
        }

        Ok(ExecutionResult {
            qualified_id,
            service_result: result,
        })
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}
