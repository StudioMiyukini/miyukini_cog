//! Sous-système runtime des **capacités services** d'Alicia (P1-P7).
//!
//! Ce module compose les quatre crates d'apprentissage et de gouvernance
//! (`miyualicia-skills`, `miyualicia-memory`, `miyualicia-dispatcher` ; le
//! [`CoreGate`] est injecté par le caller) en un sous-système unique
//! attachable à `AliciaService`.
//!
//! Il **complète** — sans remplacer — l'orchestration des dispositifs maison
//! existante : les commandes MQTT/HTTP continuent de transiter par
//! `AliciaService::execute_command`, l'`activity_log` mémoire reste dédié
//! aux interactions device. Ce sous-système gère exclusivement les
//! **capacités services COG** (JayKoa, JayKonta, Jay Bureau...).
//!
//! ## Lois d'autonomie
//!
//! Le `CapabilitiesRuntime` est entièrement **opt-in** :
//! - Si non activé, `AliciaService` se comporte comme avant.
//! - Si activé sans répertoire de manifestes, le registre reste vide.
//! - Si activé sans `CoreGate`, un `BypassCoreGate` est utilisé (avec
//!   warning) — le runtime production doit injecter un
//!   `BorderGuardCoreGate`.

// @id service.alicia.capabilities_runtime
// @role alicia_capabilities_subsystem
// @layer 7
// @do orchestrate_alicia_service_capabilities
// @human Composition des crates skills/memory/dispatcher dans le runtime Alicia.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

use miyualicia_dispatcher::{
    BypassCoreGate, CoreGate, Decision, DispatchError, Dispatcher, ExecutionResult,
    ResolvedLearnedCall, ServiceClient, ServiceClientRouter,
};
use miyualicia_memory::{
    build_persona_context, detect_repetitions, propose_from_pattern, ContextOptions,
    DetectorOptions, LearnedSkill, MemoryStore, PendingConfirmation, RepetitionPattern,
};
use miyualicia_skills::{
    build_tool_response, capabilities_to_tools, resolve_tool_call, AliciaPolicy, ManifestWatcher,
    ReloadReport, SkillRegistry, ServiceCapability, SkillsError, TickOutcome, ToolCall,
    ToolDefinition,
};

use crate::errors::AliciaError;

/// Configuration de démarrage du sous-système capacités.
#[derive(Debug, Clone, Default)]
pub struct CapabilitiesRuntimeConfig {
    /// Dossier surveillé pour les `service.manifest.json` (un sous-dossier
    /// par service, ou un manifeste à plat).
    pub services_dir: Option<PathBuf>,
    /// Chemin du fichier SQLite de la mémoire d'Alicia.
    /// `None` → store en mémoire (volatile, dev / test).
    pub memory_db_path: Option<PathBuf>,
    /// Politique d'exécution. Default = `AliciaPolicy::strict()`.
    pub policy: AliciaPolicy,
    /// Options du détecteur de patterns. Default = seuil 3.
    pub detector_options: DetectorOptions,
    /// Options de construction du bloc de contexte mémoire injecté dans le
    /// system prompt LLM.
    pub context_options: ContextOptions,
}

/// Résultat d'une exécution `dispatch_and_queue` — soit l'action est passée
/// directement, soit une `PendingConfirmation` a été enregistrée dans la
/// queue et le caller dispose de son `id` pour résolution ultérieure.
#[derive(Debug, Clone)]
pub enum QueuedDispatch {
    /// Exécution effectuée, résultat retourné.
    Executed(miyualicia_dispatcher::ExecutionResult),
    /// `PendingConfirmation` posée dans la queue. Le caller (UI / agent)
    /// peut afficher `prompt` à l'utilisateur et appeler
    /// `confirm_queued(id)` ou `cancel_queued(id)` pour résoudre.
    Queued {
        /// Identifiant de la `PendingConfirmation` créée.
        id: uuid::Uuid,
        /// `qualified_id` de la capability cible.
        qualified_id: String,
        /// Texte à afficher à l'utilisateur.
        prompt: String,
    },
}

/// Résultat d'un tool call dispatché côté Alicia, prêt à être renvoyé au LLM
/// dans la prochaine itération de la boucle de tool calling.
#[derive(Debug, Clone)]
pub struct HandledToolCall {
    /// Identifiant attribué par le LLM — à reprendre dans `tool_call_id`.
    pub id: String,
    /// `qualified_id` extrait de `function.name`.
    pub qualified_id: String,
    /// Décision rendue par le dispatcher (Executed ou PendingConfirmation).
    pub decision: Decision,
    /// Message « role: tool » prêt à être inséré dans `messages` lors du
    /// prochain appel chat — `None` si la décision est `PendingConfirmation`
    /// (le caller doit d'abord obtenir la confirmation utilisateur).
    pub tool_message: Option<serde_json::Value>,
}

/// Sous-système runtime des capacités services.
pub struct CapabilitiesRuntime {
    config: CapabilitiesRuntimeConfig,
    registry: RwLock<SkillRegistry>,
    memory: Arc<MemoryStore>,
    dispatcher: Dispatcher,
    core_gate: Arc<dyn CoreGate + Send + Sync>,
    service_clients: RwLock<ServiceClientRouter>,
    watcher: RwLock<ManifestWatcher>,
}

impl CapabilitiesRuntime {
    /// Construit et initialise le sous-système.
    ///
    /// Si `services_dir` est défini, un premier scan est lancé pour peupler
    /// le registre. Si `memory_db_path` est `None`, la mémoire est
    /// volatile (in-memory).
    pub async fn new(
        config: CapabilitiesRuntimeConfig,
        core_gate: Arc<dyn CoreGate + Send + Sync>,
    ) -> Result<Self, AliciaError> {
        let memory = match config.memory_db_path.as_ref() {
            None => MemoryStore::open_in_memory().map_err(memory_err)?,
            Some(path) => MemoryStore::open(path).map_err(memory_err)?,
        };
        let mut registry = SkillRegistry::new();
        if let Some(dir) = config.services_dir.as_ref() {
            if dir.exists() {
                let (count, errors) = miyualicia_skills::discover_in_dir(dir, &mut registry)
                    .map_err(skills_err)?;
                if !errors.is_empty() {
                    tracing::warn!(?errors, "{count} capability(ies) chargée(s) avec erreurs");
                } else {
                    tracing::info!("{count} capability(ies) chargée(s) depuis {dir:?}");
                }
            } else {
                tracing::warn!(?dir, "services_dir absent — registre vide");
            }
        }
        Ok(Self {
            config,
            registry: RwLock::new(registry),
            memory: Arc::new(memory),
            dispatcher: Dispatcher::new(),
            core_gate,
            service_clients: RwLock::new(ServiceClientRouter::new()),
            watcher: RwLock::new(ManifestWatcher::new()),
        })
    }

    /// Variante dev/test : initialise avec un `BypassCoreGate`.
    /// **Ne pas utiliser en production** — la gouvernance cores n'est pas
    /// appliquée.
    pub async fn new_with_bypass_gate(
        config: CapabilitiesRuntimeConfig,
    ) -> Result<Self, AliciaError> {
        tracing::warn!(
            "CapabilitiesRuntime initialisé avec BypassCoreGate — gouvernance désactivée"
        );
        Self::new(config, Arc::new(BypassCoreGate)).await
    }

    /// Enregistre un client de service (`Arc<dyn ServiceClient>`) sur son
    /// `service_id`. Doit être appelé pour chaque service exposant des
    /// capabilities Alicia (jaykoa-client, etc.).
    pub async fn register_service_client(
        &self,
        service_id: impl Into<String>,
        client: Arc<dyn ServiceClient + Send + Sync>,
    ) {
        self.service_clients.write().await.register(service_id, client);
    }

    /// Retourne la liste des `service_id` couverts par un client.
    pub async fn registered_services(&self) -> Vec<String> {
        self.service_clients.read().await.services()
    }

    /// Référence partagée à la mémoire (utile pour y poser des faits, des
    /// préférences, ou consulter les épisodes depuis l'UI).
    #[must_use]
    pub fn memory(&self) -> Arc<MemoryStore> {
        self.memory.clone()
    }

    /// Snapshot des `ToolDefinition` à exposer au LLM (miou-llm-bridge).
    pub async fn list_tools(&self) -> Vec<ToolDefinition> {
        let registry = self.registry.read().await;
        capabilities_to_tools(registry.iter().map(|s| &s.capability))
    }

    /// Cherche une `LearnedSkill` active dont le `trigger_phrase` matche la
    /// phrase utilisateur — court-circuit avant LLM.
    pub fn resolve_learned_phrase(
        &self,
        phrase: &str,
        persona: &str,
    ) -> Result<Option<ResolvedLearnedCall>, AliciaError> {
        self.dispatcher
            .resolve_learned_phrase(phrase, persona, &self.memory)
            .map_err(dispatch_err)
    }

    /// Exécute une capacité.
    pub async fn dispatch_capability(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
    ) -> Result<Decision, AliciaError> {
        let registry = self.registry.read().await;
        let clients = self.service_clients.read().await;
        self.dispatcher
            .execute(
                qualified_id,
                params,
                persona,
                user_phrase,
                &registry,
                &self.config.policy,
                self.core_gate.as_ref(),
                &*clients,
                &self.memory,
            )
            .await
            .map_err(dispatch_err)
    }

    /// Reprise après confirmation utilisateur.
    pub async fn dispatch_confirmed(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
    ) -> Result<ExecutionResult, AliciaError> {
        let registry = self.registry.read().await;
        let clients = self.service_clients.read().await;
        self.dispatcher
            .execute_confirmed(
                qualified_id,
                params,
                persona,
                user_phrase,
                &registry,
                &self.config.policy,
                self.core_gate.as_ref(),
                &*clients,
                &self.memory,
            )
            .await
            .map_err(dispatch_err)
    }

    /// Reprise après refus utilisateur (pose un épisode `Cancelled`).
    pub async fn dispatch_cancelled(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
    ) -> Result<(), AliciaError> {
        self.dispatcher
            .execute_cancelled(qualified_id, params, persona, user_phrase, &self.memory)
            .map_err(dispatch_err)
    }

    /// Tick le watcher : si des manifestes ont changé, rappelle
    /// `reload_in_dir` et retourne le `ReloadReport`. Sinon retourne
    /// `None`. Aucun-op si `services_dir` n'est pas défini.
    pub async fn tick_manifest_watcher(&self) -> Result<Option<ReloadReport>, AliciaError> {
        let dir = match self.config.services_dir.as_ref() {
            Some(d) => d.clone(),
            None => return Ok(None),
        };
        if !dir.exists() {
            return Ok(None);
        }
        let mut watcher = self.watcher.write().await;
        let mut registry = self.registry.write().await;
        match watcher.tick(&dir, &mut registry).map_err(skills_err)? {
            TickOutcome::Unchanged => Ok(None),
            TickOutcome::Reloaded(report) => {
                tracing::info!(
                    added = report.added.len(),
                    kept = report.kept.len(),
                    removed = report.removed.len(),
                    errors = report.errors.len(),
                    "manifestes Alicia rechargés"
                );
                Ok(Some(report))
            }
        }
    }

    /// Détecte les patterns de répétition pour une persona donnée.
    pub async fn detect_patterns(
        &self,
        persona: &str,
    ) -> Result<Vec<RepetitionPattern>, AliciaError> {
        detect_repetitions(&self.memory, persona, &self.config.detector_options)
            .map_err(memory_err)
    }

    /// Transforme un pattern en `LearnedSkill` proposée (statut `Proposed`).
    pub async fn propose_pattern(
        &self,
        pattern: &RepetitionPattern,
    ) -> Result<LearnedSkill, AliciaError> {
        propose_from_pattern(&self.memory, pattern).map_err(memory_err)
    }

    // ------------------------------------------------------------------
    // Queue async des PendingConfirmation
    // ------------------------------------------------------------------

    /// Variante de `dispatch_capability` qui **enregistre automatiquement**
    /// dans la queue toute `PendingConfirmation` au lieu de la retourner
    /// telle quelle.
    ///
    /// Utile pour les chemins d'appel où le caller ne peut pas attendre
    /// (tool call LLM, agent autonome, API REST sans long-polling) :
    /// l'utilisateur résoudra la pending plus tard via une UI qui consomme
    /// `list_pending_confirmations()` puis `confirm_queued`/`cancel_queued`.
    pub async fn dispatch_and_queue(
        &self,
        qualified_id: &str,
        params: &Value,
        persona: &str,
        user_phrase: &str,
    ) -> Result<QueuedDispatch, AliciaError> {
        match self
            .dispatch_capability(qualified_id, params, persona, user_phrase)
            .await?
        {
            Decision::Executed(r) => Ok(QueuedDispatch::Executed(r)),
            Decision::PendingConfirmation { qualified_id: qid, prompt } => {
                let pc = self
                    .memory
                    .enqueue_pending_confirmation(
                        &qid,
                        params.clone(),
                        persona,
                        user_phrase,
                        &prompt,
                    )
                    .map_err(memory_err)?;
                Ok(QueuedDispatch::Queued {
                    id: pc.id,
                    qualified_id: qid,
                    prompt,
                })
            }
        }
    }

    /// Liste les `PendingConfirmation` `Pending` (toutes personas).
    pub async fn list_pending_confirmations(
        &self,
    ) -> Result<Vec<PendingConfirmation>, AliciaError> {
        self.memory.list_pending_confirmations().map_err(memory_err)
    }

    /// Liste les `PendingConfirmation` `Pending` filtrées par persona.
    pub async fn pending_confirmations_for(
        &self,
        persona: &str,
    ) -> Result<Vec<PendingConfirmation>, AliciaError> {
        self.memory
            .pending_confirmations_for_persona(persona)
            .map_err(memory_err)
    }

    /// Récupère une `PendingConfirmation` par son id (tous statuts).
    pub async fn pending_confirmation(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<PendingConfirmation>, AliciaError> {
        self.memory.pending_confirmation(id).map_err(memory_err)
    }

    /// Confirme une `PendingConfirmation` : exécute la capability via
    /// `dispatch_confirmed` (gates cores actives, audit posé) puis marque la
    /// pending `Confirmed` en mémoire.
    pub async fn confirm_queued(
        &self,
        id: uuid::Uuid,
    ) -> Result<miyualicia_dispatcher::ExecutionResult, AliciaError> {
        let pc = self
            .memory
            .pending_confirmation(id)
            .map_err(memory_err)?
            .ok_or_else(|| {
                AliciaError::InternalError(format!("PendingConfirmation {id} introuvable"))
            })?;
        if pc.status != miyualicia_memory::PendingStatus::Pending {
            return Err(AliciaError::InternalError(format!(
                "PendingConfirmation {id} déjà résolue (statut {:?})",
                pc.status
            )));
        }
        let result = self
            .dispatch_confirmed(&pc.qualified_id, &pc.params, &pc.persona, &pc.user_phrase)
            .await?;
        self.memory
            .mark_pending_confirmed(id)
            .map_err(memory_err)?;
        Ok(result)
    }

    /// Annule une `PendingConfirmation` : marque l'épisode `Cancelled` dans
    /// l'audit puis marque la pending `Cancelled` en mémoire.
    pub async fn cancel_queued(&self, id: uuid::Uuid) -> Result<(), AliciaError> {
        let pc = self
            .memory
            .pending_confirmation(id)
            .map_err(memory_err)?
            .ok_or_else(|| {
                AliciaError::InternalError(format!("PendingConfirmation {id} introuvable"))
            })?;
        if pc.status != miyualicia_memory::PendingStatus::Pending {
            return Err(AliciaError::InternalError(format!(
                "PendingConfirmation {id} déjà résolue (statut {:?})",
                pc.status
            )));
        }
        self.dispatch_cancelled(&pc.qualified_id, &pc.params, &pc.persona, &pc.user_phrase)
            .await?;
        self.memory
            .mark_pending_cancelled(id)
            .map_err(memory_err)?;
        Ok(())
    }

    // ------------------------------------------------------------------
    // Bridge LLM (miou-llm-bridge / maia)
    // ------------------------------------------------------------------

    /// Construit le bloc Markdown de contexte mémoire à injecter en tant que
    /// message `role: "system"` dans la requête LLM. Reflète la persona,
    /// les faits, les préférences, et les épisodes récents.
    pub fn build_system_context(&self, persona: &str) -> Result<String, AliciaError> {
        build_persona_context(persona, &self.memory, &self.config.context_options)
            .map_err(memory_err)
    }

    /// Snapshot des `(qualified_id, ServiceCapability)` connus du registre.
    /// Permet à un caller externe de filtrer/personnaliser la liste de tools.
    pub async fn snapshot_capabilities(&self) -> Vec<ServiceCapability> {
        let registry = self.registry.read().await;
        registry.iter().map(|s| s.capability.clone()).collect()
    }

    /// Dispatche les `tool_calls` extraits d'une réponse LLM (format OpenAI).
    ///
    /// Pour chaque tool call :
    /// - parse `function.name` (`service_id.intent`) + arguments JSON
    /// - délègue à `dispatch_capability(...)`
    /// - sérialise le résultat en message `role: "tool"` réinjectable dans la
    ///   prochaine itération chat (sauf si `PendingConfirmation`)
    ///
    /// Le caller (typiquement `AliciaService`) appelle ensuite
    /// `dispatch_confirmed` / `dispatch_cancelled` après accord utilisateur
    /// pour les tool calls restés `PendingConfirmation`.
    pub async fn handle_tool_calls(
        &self,
        tool_calls: &[ToolCall],
        persona: &str,
        user_phrase: &str,
    ) -> Result<Vec<HandledToolCall>, AliciaError> {
        let mut out = Vec::with_capacity(tool_calls.len());
        for call in tool_calls {
            let resolved = resolve_tool_call(call).map_err(skills_err)?;
            let decision = self
                .dispatch_capability(&resolved.qualified_id, &resolved.params, persona, user_phrase)
                .await?;
            let tool_message = match &decision {
                Decision::Executed(result) => {
                    let content = serde_json::json!({
                        "success": result.service_result.success,
                        "data":    result.service_result.data,
                        "message": result.service_result.message,
                    });
                    Some(build_tool_response(&call.id, &content))
                }
                Decision::PendingConfirmation { .. } => None,
            };
            out.push(HandledToolCall {
                id: call.id.clone(),
                qualified_id: resolved.qualified_id,
                decision,
                tool_message,
            });
        }
        Ok(out)
    }

    /// Extrait les `tool_calls` depuis une réponse LLM brute (compatible
    /// OpenAI : `choices[0].message.tool_calls[]`).
    ///
    /// Renvoie `None` si la réponse est une réponse texte normale (pas de
    /// tool calling). Renvoie `Some(vec)` même vide si le champ existe mais
    /// est vide (cas de bord).
    #[must_use]
    pub fn extract_tool_calls(response: &serde_json::Value) -> Option<Vec<ToolCall>> {
        let arr = response
            .get("choices")?
            .get(0)?
            .get("message")?
            .get("tool_calls")?
            .as_array()?;
        let parsed: Vec<ToolCall> = arr
            .iter()
            .filter_map(|tc| serde_json::from_value(tc.clone()).ok())
            .collect();
        Some(parsed)
    }
}

fn memory_err(e: miyualicia_memory::MemoryError) -> AliciaError {
    AliciaError::InternalError(format!("memory: {e}"))
}

fn skills_err(e: SkillsError) -> AliciaError {
    AliciaError::InternalError(format!("skills: {e}"))
}

fn dispatch_err(e: DispatchError) -> AliciaError {
    AliciaError::InternalError(format!("dispatch: {e}"))
}

/// Erreur d'I/O sur le chemin de la mémoire.
pub fn validate_memory_path(path: &Path) -> Result<(), AliciaError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(AliciaError::InternalError(format!(
                "dossier parent absent : {}",
                parent.display()
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn runtime_with_bypass_starts_empty() {
        let runtime =
            CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
                .await
                .unwrap();
        assert!(runtime.list_tools().await.is_empty());
        assert!(runtime.registered_services().await.is_empty());
    }

    #[tokio::test]
    async fn runtime_loads_manifests_from_services_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let sub = tmp.path().join("jaykoa");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(
            sub.join("service.manifest.json"),
            r#"{
                "id": "jaykoa",
                "alicia_capabilities": [
                    {"intent": "list_today", "phrases_examples": ["x"], "params": {}, "writes": false}
                ]
            }"#,
        )
        .unwrap();
        let cfg = CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        };
        let runtime = CapabilitiesRuntime::new_with_bypass_gate(cfg).await.unwrap();
        let tools = runtime.list_tools().await;
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].function.name, "jaykoa.list_today");
    }

    #[tokio::test]
    async fn tick_watcher_returns_none_when_no_dir_configured() {
        let runtime =
            CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
                .await
                .unwrap();
        let r = runtime.tick_manifest_watcher().await.unwrap();
        assert!(r.is_none());
    }

    #[tokio::test]
    async fn tick_watcher_reloads_after_manifest_drop() {
        let tmp = tempfile::tempdir().unwrap();
        let cfg = CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        };
        let runtime = CapabilitiesRuntime::new_with_bypass_gate(cfg).await.unwrap();
        // 1er tick sur dossier vide → reload (vide) ou unchanged (selon état)
        let _ = runtime.tick_manifest_watcher().await.unwrap();

        let sub = tmp.path().join("jaykoa");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(
            sub.join("service.manifest.json"),
            r#"{"id":"jaykoa","alicia_capabilities":[{"intent":"x","phrases_examples":["a"],"params":{},"writes":false}]}"#,
        )
        .unwrap();
        let report = runtime
            .tick_manifest_watcher()
            .await
            .unwrap()
            .expect("reload after drop");
        assert_eq!(report.added, vec!["jaykoa".to_string()]);
    }

    #[tokio::test]
    async fn unknown_capability_dispatch_returns_error() {
        let runtime =
            CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
                .await
                .unwrap();
        let err = runtime
            .dispatch_capability("ghost.x", &Value::Null, "voice", "?")
            .await
            .unwrap_err();
        match err {
            AliciaError::InternalError(msg) => assert!(msg.contains("UnknownCapability") || msg.contains("inconnue")),
            other => panic!("expected InternalError, got {other:?}"),
        }
    }

    // -------------------------------------------------------------------
    // Bridge LLM (P-bridge.1)
    // -------------------------------------------------------------------

    #[tokio::test]
    async fn build_system_context_works_for_unknown_persona() {
        let runtime =
            CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
                .await
                .unwrap();
        let ctx = runtime.build_system_context("voice").unwrap();
        assert!(ctx.contains("Persona « voice »"));
    }

    #[tokio::test]
    async fn snapshot_capabilities_returns_loaded_caps() {
        let tmp = tempfile::tempdir().unwrap();
        let demo = tmp.path().join("demo");
        std::fs::create_dir_all(&demo).unwrap();
        std::fs::write(
            demo.join("service.manifest.json"),
            r#"{"id":"demo","alicia_capabilities":[{"intent":"x","phrases_examples":["a"],"params":{},"writes":false}]}"#,
        )
        .unwrap();
        let runtime = CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap();
        let snap = runtime.snapshot_capabilities().await;
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].qualified_id(), "demo.x");
    }

    #[test]
    fn extract_tool_calls_returns_none_for_text_response() {
        let body = serde_json::json!({
            "choices": [{
                "message": {"role": "assistant", "content": "Bonjour"}
            }]
        });
        assert!(CapabilitiesRuntime::extract_tool_calls(&body).is_none());
    }

    #[test]
    fn extract_tool_calls_parses_openai_format() {
        let body = serde_json::json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "demo.x",
                            "arguments": "{}"
                        }
                    }]
                }
            }]
        });
        let calls = CapabilitiesRuntime::extract_tool_calls(&body).expect("some");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, "call_1");
        assert_eq!(calls[0].function.name, "demo.x");
    }
}
