# Alicia Home Assistante — Architecture Apprentissage et Gouvernance

<!-- @id doc.alicia.architecture.hermes -->
<!-- @do describe_alicia_learning_governance_architecture -->
<!-- @role technical-architecture -->
<!-- @layer 7 -->
<!-- @human Architecture Hermes-style d'Alicia : skills, mémoire, dispatcher, cores-gate -->

**Date :** 2026-05-08
**Version :** 1.0
**Statut :** Architecture livrée et testée (P1 → P7), runtime à câbler
**Référence :** Rapport Fondateur Alicia Home Assistante v1.0, Hermes Agent (Nous Research) https://hermes-agent.nousresearch.com/

---

## 1. Contexte

L'objectif du chantier P1-P7 a été d'étendre Alicia avec les capacités d'**apprentissage** et de **gouvernance** propres aux agents persistants modernes (modèle Hermes Agent), tout en respectant la pyramide des Cores du COG.

Concrètement, Alicia doit pouvoir :

- **Découvrir** dynamiquement les capacités exposées par les Services COG (JayKoa, JayKonta, Jay Bureau, ...) via leurs `service.manifest.json`
- **Mémoriser** les faits utilisateur, les préférences, les épisodes d'interaction
- **Apprendre** par observation : détecter une séquence répétée et proposer un raccourci nommé
- **Personnaliser** son comportement par persona (vocal, travail, famille, ...)
- **Gouverner** chaque exécution en passant par les Cores avant toute action hors de son propre périmètre

Ce document décrit l'architecture de ces capacités, livrée en sept phases (P1-P7), testée par 183 tests automatisés.

## 2. Vue d'ensemble

### 2.1 Quatre nouveaux crates

| Crate | Rôle | Strate | Tests |
|---|---|---|---|
| `miyualicia-skills` | Découverte et registre des capacités services | 7 | 65 |
| `miyualicia-memory` | Mémoire persistante locale (SQLite + FTS5) | 7 | 60 |
| `miyualicia-dispatcher` | Orchestrateur d'exécution avec gates abstraites | 7 | 22 |
| `miyualicia-cores-gate` | Bridge BorderGuard ↔ trait `CoreGate` du dispatcher | 7 | 8 |

Et un crate **client de service** témoin : `jaykoa-client` (28 tests, dont 4 fichiers E2E qui valident la chaîne de bout en bout).

### 2.2 Pipeline d'une interaction

```
                    ┌─────────────────────────────────────┐
                    │  Phrase utilisateur (chat / voix)   │
                    └─────────────────┬───────────────────┘
                                      ↓
                    ┌─────────────────────────────────────┐
                    │  Dispatcher::resolve_learned_phrase │  ← P5
                    │  ↳ match LearnedSkill ?            │
                    │  ↳ oui → bypass LLM                │
                    └─────────────────┬───────────────────┘
                                      ↓ non
                    ┌─────────────────────────────────────┐
                    │  miou-llm-bridge tool calling       │  ← P2
                    │  ↳ tools = capabilities_to_tools    │
                    │  ↳ LLM choisit qualified_id +       │
                    │      params                          │
                    └─────────────────┬───────────────────┘
                                      ↓
                    ┌─────────────────────────────────────┐
                    │  Dispatcher::execute                │  ← P2/P4
                    │  ┌─ should_confirm ?               │
                    │  │  └─ ConfirmationPolicy + prefs  │
                    │  │  └─ → PendingConfirmation       │
                    │  ├─ CoreGate::validate ★           │  ← P7 BorderGuardCoreGate
                    │  │  └─ refus → DispatchError       │
                    │  ├─ ServiceClient::call            │  ← P3 JayKoaClient (et ses pairs)
                    │  └─ MemoryStore::record_episode    │  toujours (succès comme échec)
                    └─────────────────┬───────────────────┘
                                      ↓
                    ┌─────────────────────────────────────┐
                    │  KindMother (DB fille Alicia)       │
                    │  Episodes / Facts / Prefs /         │
                    │  Personas / LearnedSkills           │
                    └─────────────────────────────────────┘

       ───────────────────────────────────────────────────────────

                    ┌─────────────────────────────────────┐
                    │  ManifestWatcher::tick              │  ← P6
                    │  └─ reload_in_dir(services/)        │
                    │     └─ delta sur SkillRegistry      │
                    └─────────────────────────────────────┘

                    ┌─────────────────────────────────────┐
                    │  detect_repetitions (périodique)    │  ← P5
                    │  └─ propose_learned_skill           │
                    │     └─ dialogue de promotion        │
                    └─────────────────────────────────────┘
```

## 3. `miyualicia-skills` — Découverte et registre

### 3.1 Objectif

Charger dynamiquement les capacités exposées par les services à partir de leurs `service.manifest.json` (extension du manifeste Miyumarket existant), exposer un registre indexé, et fournir la conversion vers le format **OpenAI function calling** consommé par le LLM.

### 3.2 Modèle

- **`ServiceCapability`** : `(service_id, intent, phrases_examples, params, writes, confirmation_default, description, category)`. C'est l'unité du registre.
- **`AliciaCapabilityManifest`** : représentation parsée d'un `service.manifest.json` enrichi du bloc `alicia_capabilities`.
- **`SkillRegistry`** : `BTreeMap` indexée par `qualified_id = "service_id.intent"`. Permet `register`, `upsert`, `remove_service`, `get`, `iter`, `by_service`, `by_category`, `search_by_phrase` (matching naïf P1, à remplacer par LLM en runtime).

### 3.3 Modules clés

- **`discovery`** : `discover_in_dir(dir, registry)` (additif) et `reload_in_dir(dir, registry)` (avec suppression — voir P6).
- **`tooling`** : `capability_to_tool(cap) → ToolDefinition` (OpenAI), `resolve_tool_call(call) → ResolvedToolCall`, `build_tool_response(...)`. Mapping des `ParamType` → JSON Schema (`string`, `integer`, `number`, `boolean`, formats `date-time` / `date`, `enum`).
- **`policy`** : `AliciaPolicy { bypass_cores_validation, bypass_user_confirmations }`. Drapeaux **indépendants** pour court-circuiter les gates. Default `strict`.
- **`watcher`** : `ManifestWatcher` polling-based avec mtime tracking (voir P6).

### 3.4 Format `alicia_capabilities` dans `service.manifest.json`

```json
{
  "id": "jaykoa",
  "name": "JayKoa",
  "version": "0.1.0",
  "alicia_capabilities": [
    {
      "intent": "list_events",
      "description": "Liste les événements de l'agenda sur une période donnée.",
      "phrases_examples": [
        "qu'est-ce que j'ai aujourd'hui",
        "mon agenda du jour"
      ],
      "params": {
        "from": { "type": "date", "required": false, "description": "Date de début (ISO 8601)." },
        "to":   { "type": "date", "required": false, "description": "Date de fin (ISO 8601)." }
      },
      "writes": false,
      "confirmation_default": "never",
      "category": "calendar"
    }
  ]
}
```

Le bloc `alicia_capabilities` est **optionnel** — un service qui ne l'expose pas reste invisible pour Alicia.

## 4. `miyualicia-memory` — Mémoire persistante locale

### 4.1 Objectif

Persistance locale SQLite (rusqlite + FTS5) de toute la mémoire d'Alicia : faits, préférences, épisodes, personas, skills apprises. C'est la **DB fille** d'Alicia gérée via KindMother (sans dépendance directe pour P1-P7, l'intégration KM se fait au runtime).

### 4.2 Schéma

| Table | Clé | Usage |
|---|---|---|
| `facts` | `id` (UUID) | Faits stables sur l'utilisateur, FTS5 sur `content` |
| `preferences` | `(scope, key)` | Préférences typées JSON, scope = persona ou `"global"` |
| `episodes` | `id` (UUID) | Journal des interactions, FTS5 sur `user_phrase` |
| `personas` | `id` | Profils contextuels, flag `is_default` |
| `learned_skills` | `id` (UUID) | Skills apprises (status `proposed/accepted/rejected`) |

### 4.3 API principale

- **Faits** : `remember`, `forget`, `facts_for_persona`, `recall(query)` (FTS5)
- **Préférences** : `prefer`, `preference`, `preferences_for_scope`, `unprefer`
- **Épisodes** : `record_episode`, `recent_episodes`, `episodes_for_service`, `search_episodes`
- **Personas** : `upsert_persona`, `persona`, `personas`, `default_persona`, `active_persona_id`, `set_active_persona`
- **Confirmations conditionnelles** : `confirmation_preference(persona, qualified_id)`, `set_confirmation_preference(scope, qualified_id, ask)`
- **Apprentissage** : `propose_learned_skill`, `accept_learned_skill`, `reject_learned_skill`, `active_learned_skills`, `pending_learned_skills`, `learned_skill_exists`, `match_learned_skill(persona, phrase)`

### 4.4 Modules clés

- **`prompt`** : `build_persona_context(persona, store, opts) → String` — assemble un bloc Markdown à injecter dans le system prompt du LLM (faits, préférences, épisodes récents, persona active). C'est ce qui donne à Alicia sa **continuité** d'une session à l'autre.
- **`learning`** : `detect_repetitions(store, persona, opts) → Vec<RepetitionPattern>` qui scanne les épisodes `Ok` et regroupe par `qualified_id`. `propose_from_pattern` transforme un pattern en `LearnedSkill` Proposed.

## 5. `miyualicia-dispatcher` — Orchestrateur d'exécution

### 5.1 Objectif

Centraliser **toute** exécution de capacité : de la phrase utilisateur (ou tool call LLM) jusqu'à l'audit en mémoire, en passant par les gates de gouvernance et de confirmation.

### 5.2 Traits abstraits

- **`CoreGate`** (async) : `validate(capability, params, persona) → Allowed | Denied(reason)`. Implémentations fournies : `BypassCoreGate` (no-op, dev), `BorderGuardCoreGate` (P7).
- **`ServiceClient`** (async) : `call(qualified_id, params) → ServiceCallResult`. Implémentations fournies : `MockServiceClient` (tests), `JayKoaClient` (premier vrai client). À écrire pour chaque service exposant des capabilities Alicia.

### 5.3 Cycle d'exécution

`Dispatcher::execute(qualified_id, params, persona, user_phrase, registry, policy, core_gate, service_client, memory)` :

1. **Résolution** : lookup dans le `SkillRegistry`. Erreur `UnknownCapability` si absent.
2. **Gate UX** : `should_confirm()` consulte `policy` + `confirmation_default` + `MemoryStore::confirmation_preference` → si oui, retourne `Decision::PendingConfirmation` (sans exécuter).
3. **Gate cores** : si `policy.requires_cores_check()`, appelle `core_gate.validate(...)`. Refus → `Episode::Failed` audité + `DispatchError::CoresDenied`.
4. **Exécution** : `service_client.call(...)`.
5. **Audit** : `MemoryStore::record_episode` avec outcome `Ok` / `Failed`.

Reprise post-confirmation : `execute_confirmed(...)` saute la gate UX, `execute_cancelled(...)` audite un épisode `Cancelled`.

### 5.4 Résolution des skills apprises (P5)

`resolve_learned_phrase(phrase, persona, memory) → Option<ResolvedLearnedCall>` consulte `MemoryStore::match_learned_skill` (matching normalisé sur les triggers acceptés). Le runtime appelle ça **avant** d'envoyer la phrase au LLM ; en cas de match, dispatch direct avec `params_template`.

## 6. `miyualicia-cores-gate` — Bridge BorderGuard

### 6.1 Objectif

Implémentation concrète de `CoreGate` qui consulte la gouvernance BorderGuard avant chaque exécution hors-limites Alicia. Conforme à INV-BG-2 (« BorderGuard définit, n'applique pas »).

### 6.2 Convention

- Chaque service `<svc>` dispose d'une **frontière BorderGuard** identifiée `service.<svc>` (ex : `service.jaykoa`).
- La **persona** active est l'**entité** soumise au `TrustLevelClassifier`.
- Validation : `validate(capability, params, persona)` consulte `BoundaryRegistry::get_crossing_rules(boundary_id)`. Autorise si au moins une règle a `trust(persona) >= min_trust_level && allowed = true`. Refus conservateur sinon (incl. absence de règles).

### 6.3 Construction

```rust
let registry: Arc<BoundaryRegistry> = ...;          // chargé depuis KindMother
let classifier: Arc<dyn TrustLevelClassifier + Send + Sync> = ...;
let gate = BorderGuardCoreGate::new(registry, classifier);
```

Le gate est ensuite passé en argument à `Dispatcher::execute`. Substituer `BypassCoreGate` par `BorderGuardCoreGate` côté runtime suffit pour activer la gouvernance.

## 7. Cycle de vie d'une interaction (exemple JayKoa)

Voici la trace complète d'une demande « quel est mon prochain rendez-vous » sous le persona `voice`, avant et après apprentissage.

### 7.1 Premier passage (LLM dans la boucle)

1. L'utilisateur dit la phrase.
2. `Dispatcher::resolve_learned_phrase("quel est mon prochain rendez-vous", "voice", &mem)` → `None` (rien d'appris).
3. `miou-llm-bridge` reçoit la phrase + le system prompt enrichi par `build_persona_context("voice", &mem, opts)`. Les tools exposés sont `capabilities_to_tools(reg.iter().map(|s| &s.capability))`, soit `jaykoa.list_events` et `jaykoa.next_event`.
4. Le LLM répond avec un tool call `jaykoa.next_event` sans paramètres.
5. `Dispatcher::execute(...)` :
   - Resolve OK
   - `should_confirm` : `confirmation_default = Never` → false
   - `core_gate.validate` : `voice = Trusted` ≥ `service.jaykoa[Verified]` → Allowed
   - `JayKoaClient::call("jaykoa.next_event", null)` → `{ event: { ... Stand-up ... } }`
   - `record_episode` avec `outcome = Ok, user_phrase = "quel est mon prochain rendez-vous"`
6. Le runtime renvoie le résultat au LLM, qui formule la réponse vocale finale.

### 7.2 Après 3 passages similaires : apprentissage

7. Un timer (ou event de fin de session) appelle `detect_repetitions(&mem, "voice", &DetectorOptions::default())`.
8. Le détecteur trouve `RepetitionPattern { qualified_id: "jaykoa.next_event", occurrence_count: 3, last_phrase: "...", last_params: null }`.
9. Le runtime appelle `propose_from_pattern(&mem, &pattern)` → `LearnedSkill { status: Proposed }` persistée.
10. Au prochain démarrage / événement « dialogue de promotion », l'utilisateur voit la proposition. Il choisit un trigger court (« prochain rdv »), édite l'objet, accepte → `accept_learned_skill(id)`.

### 7.3 Passage suivant (court-circuit LLM)

11. L'utilisateur dit « prochain rdv ».
12. `Dispatcher::resolve_learned_phrase(...)` → `Some(ResolvedLearnedCall { qualified_id: "jaykoa.next_event", params: null })`.
13. `Dispatcher::execute(...)` traverse les gates et appelle `JayKoaClient` directement. **Sans LLM, sans latence d'inférence.**
14. L'épisode est enregistré normalement.

## 8. Hot reload des manifestes (P6)

`ManifestWatcher` polling-based, sans dépendance `notify` :

```rust
let mut watcher = ManifestWatcher::new();
loop {
    tokio::time::sleep(Duration::from_secs(5)).await;
    match watcher.tick(&services_dir, &mut registry)? {
        TickOutcome::Unchanged => continue,
        TickOutcome::Reloaded(report) => {
            tracing::info!(?report, "registre Alicia rechargé");
        }
    }
}
```

`reload_in_dir` détecte aussi les **suppressions** : un service dont le `service.manifest.json` disparaît voit ses capabilities retirées du registre (`SkillRegistry::remove_service`). Validé E2E : après suppression, `Dispatcher::execute` renvoie `DispatchError::UnknownCapability`.

## 9. Gouvernance cores : invariant

> **Toute modification de fichier que Alicia (le service assistante) effectue hors de ses propres limites doit être validée par les cores avant exécution.**
> *(mémoire projet 2026-05-07)*

Cet invariant est tenu **par construction** du dispatcher :

- Le dispatcher ne donne aucune voie à `service_client.call(...)` sans avoir au préalable consulté `core_gate.validate(...)`, sauf si `policy.bypass_cores_validation == true`.
- Le bypass est **explicite** (champ booléen sur `AliciaPolicy`), jamais silencieux. Chaque utilisation log un `warn!`.
- Les écritures qu'Alicia fait **dans** sa propre mémoire (`facts`, `preferences`, `episodes`, `learned_skills`) ne traversent pas le gate — c'est sa DB fille, dans son périmètre.
- L'apprentissage et le hot reload ne déclenchent jamais de gate cores : ce sont des opérations internes (lecture FS, lecture/écriture mémoire d'Alicia).

`AliciaPolicy::permissive()` (les deux gates bypassées) est réservé au dev / test / sandbox isolé. **Jamais en production utilisateur.**

## 10. Intégration runtime (à venir — ce que le runtime devra faire)

Le crate `crates/miyualicia/` (le runtime principal) doit être adapté pour utiliser ces 4 nouveaux crates. Étapes (résumé) :

1. **Au démarrage** : ouvrir le `MemoryStore` (chemin DB fille via KindMother). Créer un `SkillRegistry`. Lancer `discover_in_dir(services_dir, &mut registry)` une première fois. Construire le `BorderGuardCoreGate` à partir du `BoundaryRegistry` et du `TrustLevelClassifier` chargés depuis KindMother. Construire les `ServiceClient` (un par service exposant des capabilities — JayKoaClient pour JayKoa, etc.).
2. **Pour chaque interaction** :
   - Récupérer la persona active via `MemoryStore::active_persona_id`.
   - Tenter `Dispatcher::resolve_learned_phrase` ; si match, dispatcher direct.
   - Sinon, déléguer au `miou-llm-bridge` avec les tools `capabilities_to_tools` et le system prompt enrichi par `build_persona_context`.
   - Recevoir le tool call → `Dispatcher::execute(...)`.
   - Si `Decision::PendingConfirmation`, présenter la confirmation à l'utilisateur. Sur OK → `execute_confirmed`. Sur refus → `execute_cancelled`.
3. **Tâches de fond** :
   - Toutes les N secondes : `ManifestWatcher::tick` pour détecter les hot swaps.
   - Périodiquement (fin de session, timer 1h, ...) : `detect_repetitions` puis dialogue de promotion.

## 11. Référence test

| Crate | Tests | Nature |
|---|---|---|
| `miyualicia-skills` | 65 | Unitaires (capability, manifest, registry, tooling, policy, watcher, discovery) |
| `miyualicia-memory` | 60 | Unitaires (prompt, learning) + intégration (store_tests, learning_e2e) |
| `miyualicia-dispatcher` | 22 | Unitaires (core_gate, service_client) + intégration (dispatcher_tests) |
| `miyualicia-cores-gate` | 8 | Unitaires (BorderGuardCoreGate matrix) |
| `jaykoa-client` | 28 | Unitaires (client) + 4 fichiers E2E (dispatch standard, Hermes loop, hot swap, BorderGuard) |
| **Total** | **183** | |

Run global :

```bash
cargo test -p miyualicia-skills -p miyualicia-memory \
           -p miyualicia-dispatcher -p miyualicia-cores-gate \
           -p jaykoa-client
```

## 12. Voir aussi

- [Spec NLU bridge](./specs/nlu-bridge-interface.md) — interface historique avec `miou-llm-bridge`
- [Plan Dev General](./Alicia%20Home%20Assistante%20-%20Plan%20Dev%20General.md)
- [Rapport Fondateur](./Alicia%20Home%20Assistante%20-%20Rapport%20Fondateur.md)
- Hermes Agent (Nous Research) — https://hermes-agent.nousresearch.com/
