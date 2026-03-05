# Rapport d'Audit -- Alicia Home Assistante

**Auditeur** : George, Audit Expert Analyste -- Miyukini AI Studio
**Date** : 2026-03-01
**Perimetre** : 8 crates Alicia + UI Dioxus dans Central
**Methode** : Build, Tests, Clippy, Securite, Renommage, Conformite Lois d'Autonomie, MSCM

---

## Resume executif

**Score global : 96/100**

Le livrable Alicia Home Assistante est de tres haute qualite. Les 8 crates compilent sans erreur, les 279 tests passent tous, Clippy pedantic produit zero warning sur le perimetre Alicia, et l'UI Dioxus compile et s'integre correctement dans Central. Le renommage MiyukiniVoice vers Alicia est complet sans residus. Les Lois d'Autonomie sont respectees. Les annotations MSCM sont presentes dans tous les fichiers source.

**Points forts** :
- 279 tests, 0 echec, couverture fonctionnelle solide
- Zero `unwrap()` en production (uniquement dans les tests)
- Zero `unsafe` (forbid dans tous les lib.rs et Cargo.toml)
- Architecture modulaire exemplaire (8 crates independants)
- Fallback NLU regex fonctionnel quand le bridge LLM est indisponible
- MQTT, LLM et API REST tous optionnels (respect des Lois d'Autonomie)

**Points d'attention** : 4 anomalies mineures/cosmetiques identifiees.

---

## 1. Build global

| Verification | Resultat |
|---|---|
| `cargo build` 8 crates Alicia | OK (1m16s) |
| `cargo build -p miyukini-central-native` | OK (43.72s, 11 warnings dont 1 seul dans le code Alicia) |

Les 8 crates Alicia compilent sans erreur ni warning :
- `miyualicia-capture` (dossier `crates/miyuvoicecapture/`)
- `miyualicia-wakeword` (dossier `crates/miyuwakeword/`)
- `miyualicia-devices`
- `miyualicia-mqtt`
- `miyualicia-http`
- `miyualicia`
- `miyualicia-api`
- `miyualicia-automations`

Le build Central produit 11 warnings, mais **un seul concerne le code Alicia** : la methode `icon()` dans `apps/central/src/services/alicia/devices.rs:36` qui n'est pas utilisee (dead code, code preparatoire pour l'UI).

---

## 2. Tests globaux

| Crate | Tests | Resultat |
|---|---|---|
| miyualicia | 47 | 47 OK |
| miyualicia-api | 26 | 26 OK |
| miyualicia-automations | 62 | 62 OK |
| miyualicia-capture | 34 | 34 OK |
| miyualicia-devices | 25 | 25 OK |
| miyualicia-http | 21 | 21 OK |
| miyualicia-mqtt | 42 | 42 OK |
| miyualicia-wakeword | 22 | 22 OK |
| **TOTAL** | **279** | **279 OK, 0 echec** |

Temps d'execution total des tests : 54.29s (compilation) + ~1.2s (execution).

### Couverture fonctionnelle par crate

- **miyualicia-capture** : admin_cell, capture config, ring buffer, context, device enumeration, VAD (detection, debounce, pre-buffer, reset, false positive rejection)
- **miyualicia-wakeword** : admin_cell, config serde, modele de detection, rustpotter integration, context governe
- **miyualicia-devices** : admin_cell, types de domaine (serde roundtrip), registre CRUD (add, get, update, remove, list_by_room, filter inactive), commandes
- **miyualicia-mqtt** : admin_cell, config, client (connect, subscribe, publish), messages, topics (Alicia + Z2M), zigbee2mqtt (12 tests Z2M : states, commandes, bridge devices)
- **miyualicia-http** : admin_cell, client config, adaptateurs Shelly Gen1/Gen2, adaptateur generique JSON, extraction JSON path
- **miyualicia** : admin_cell, config (default, from_toml, serde roundtrip), intent (serde, is_actionable), NLU bridge (parse responses, fallback auto), NLU fallback (lumiere, thermostat, volets, prises, serrures, routines, normalisation accents), command log, service (register, dispatch, snapshot, activity log), snapshot
- **miyualicia-api** : admin_cell, auth JWT (generate, verify, expired, wrong signature, scopes), config, DTOs, errors, router, server
- **miyualicia-automations** : admin_cell, engine (CRUD, trigger, sensor change, voice routine, conditions), evaluator (bool, gt, lt, between, hour, weekday, temperature, unknown), executor (success, failure, delay), parser (TOML, JSON, validation), types (serde, validation)

---

## 3. Clippy pedantic

| Verification | Resultat |
|---|---|
| `cargo clippy --no-deps` 8 crates | **0 warning, 0 erreur** |

Tous les crates Alicia passent Clippy pedantic avec `-D warnings` (zero tolerance). Les `[lints.clippy]` sont configures de maniere coherente dans tous les Cargo.toml avec les exemptions standard du projet (doc_markdown, cast_*, module_name_repetitions, etc.).

---

## 4. Build Central (UI Dioxus)

| Verification | Resultat |
|---|---|
| Compilation UI | OK |
| AliciaView (5 onglets) | Present et route correctement |
| Integration dans ServiceMeta | "alicia" dans le catalogue officiel |
| Navigation par onglets | Dashboard, Pieces, Dispositifs, Automatisations, Parametres |

L'UI Alicia dans Central fournit :
- `AliciaView` avec 5 onglets (TabButton pattern identique a MiyukiniWatch)
- `StatusBadge` dynamique ("Alicia ecoute" / "En veille")
- `DashboardScreen`, `RoomsScreen`, `DevicesScreen`, `AutomationsScreen`, `SettingsScreen`
- Bridge state (`AliciaService` + snapshot polling 200ms)
- Gestion d'erreur d'initialisation (affichage dans l'UI)
- `Drop` implementee sur `AliciaService` (arret propre des threads)

---

## 5. Securite

### 5.1 unwrap() en production

| Verification | Resultat |
|---|---|
| `unwrap()` hors tests | **0 occurrence** |

Tous les `unwrap()` trouves sont exclusivement dans des blocs `#[cfg(test)]` ou `mod tests`.

### 5.2 unsafe

| Verification | Resultat |
|---|---|
| `unsafe` dans le code | **0 occurrence** (hors directive `forbid`) |
| `#![forbid(unsafe_code)]` dans lib.rs | **8/8 crates** |
| `unsafe_code = "forbid"` dans Cargo.toml | **8/8 crates** |

### 5.3 JWT Secret

| Verification | Resultat |
|---|---|
| Secret hardcode | **Non** -- injecte a l'execution via `JwtSecret(Arc<Vec<u8>>)` |
| Secret en test | Utilise des constantes TEST_SECRET explicites, pas de production secret |

Le secret JWT est :
- Passe via `AppState.jwt_secret` dans le router
- Injecte comme extension axum via middleware `inject_jwt_secret`
- Jamais hardcode dans le code de production

### 5.4 Donnees sensibles

Aucun fichier `.env`, credentials ou token en clair dans les crates Alicia.

---

## 6. Conformite Lois d'Autonomie

| Loi | Verification | Resultat |
|---|---|---|
| L1 : Aucune dependance externe critique | MQTT optionnel (`Option<MqttConfig>`), LLM fallback regex, API separee | **CONFORME** |
| L2 : Isolement = etat normal | Fonctionne sans MQTT, sans LLM, sans Internet | **CONFORME** |
| L3 : Etat local souverain | Registre in-memory, snapshot local, pas de cloud | **CONFORME** |
| L5 : Cout proportionnel au hardware | Wake word Rustpotter CPU-only, VAD RMS simple | **CONFORME** |
| L7 : Strate Cores immuable | Crates a @layer 6 (toolkits) et @layer 7 (services), pas de modification des Cores | **CONFORME** |

Details :
- **MQTT optionnel** : `config.mqtt: Option<MqttConfig>`, `None` desactive le transport
- **LLM bridge optionnel** : `NluBridge::parse_intent()` bascule automatiquement sur `FallbackNluParser` si le bridge est indisponible (test `test_bridge_unavailable_parse_uses_fallback` le confirme)
- **API REST optionnelle** : crate `miyualicia-api` separe, le service `miyualicia` fonctionne sans lui
- **100% hors-ligne** : confirme par la documentation et le design (aucun appel cloud requis)

---

## 7. Verification du renommage

| Verification | Resultat |
|---|---|
| "MiyukiniVoice" dans les .rs | **0 occurrence** |
| "miyuvoice" dans les .rs | **0 occurrence** |
| "VoiceService" dans les .rs | **0 occurrence** |
| ServiceMeta id = "alicia" | **Confirme** (state.rs:162) |
| Package names Cargo.toml | Tous en `miyualicia-*` |

Le renommage est **complet et propre**. Les anciens noms de dossiers (`miyuvoicecapture`, `miyuwakeword`) persistent sur le filesystem mais les noms de packages Cargo sont corrects (`miyualicia-capture`, `miyualicia-wakeword`).

---

## 8. Coherence des dependances

| Dependance | Versions utilisees | Coherent |
|---|---|---|
| thiserror | "2" (8/8 crates) | Oui |
| serde | "1" (8/8 crates) | Oui |
| tokio | "1" (5 crates qui l'utilisent) | Oui |
| chrono | "0.4" (6 crates) | Oui |
| uuid | "1" (6 crates) | Oui |
| reqwest | "0.12" (2 crates) | Oui |
| axum | "0.8" (1 crate) | Oui |
| toml | "0.8" (2 crates) | Oui |
| tracing | "0.1" (8/8 crates) | Oui |

Toutes les versions sont coherentes entre les crates. Les 8 crates sont correctement declares dans le workspace `Cargo.toml` (lignes 89-97).

---

## 9. Annotations MSCM

| Crate | @id | @do | @role | @layer | Complet |
|---|---|---|---|---|---|
| miyualicia-capture | Oui | Oui | Oui | Oui (toolkit) | Oui |
| miyualicia-wakeword | Oui | Oui | Oui | Oui (6) | Oui |
| miyualicia-devices | Oui | Oui | Oui | Oui (6) | Oui |
| miyualicia-mqtt | Oui | Oui | Oui | Oui (6) | Oui |
| miyualicia-http | Oui | Oui | Oui | Oui (6) | Oui |
| miyualicia | Oui | Oui | Oui | Oui (7) | Oui |
| miyualicia-api | Oui | Oui | Oui | Oui (7) | Oui |
| miyualicia-automations | Oui | Oui | Oui | Oui (6) | Oui |

Tous les fichiers source `.rs` portent les 4 annotations MSCM obligatoires. La hierarchie des layers est correcte :
- Toolkits (devices, mqtt, http, automations, capture, wakeword) : layer 6
- Services (miyualicia orchestrateur, miyualicia-api) : layer 7

---

## 10. Anomalies

| # | Severite | Description | Fichier | Recommandation |
|---|----------|-------------|---------|----------------|
| A-01 | Mineure | Noms de dossiers `miyuvoicecapture` et `miyuwakeword` non renommes (les package names Cargo sont corrects) | `crates/miyuvoicecapture/`, `crates/miyuwakeword/` | Renommer les dossiers en `miyualicia-capture` et `miyualicia-wakeword` pour coherence |
| A-02 | Cosmetique | Methode `icon()` dans `DeviceType` non utilisee (dead code preparatoire) | `apps/central/src/services/alicia/devices.rs:36` | Ajouter `#[allow(dead_code)]` ou utiliser dans l'UI |
| A-03 | Mineure | Annotations MSCM dans `miyuvoicecapture` utilisent un format legerement different (ex: `@layer: toolkit` au lieu de `@layer: 6`) | `crates/miyuvoicecapture/src/*.rs` | Harmoniser vers `@layer: 6` |
| A-04 | Cosmetique | Le `AliciaConfig::default()` initialise `mqtt: Some(MqttConfig::default())` mais le test `test_config_from_toml_minimal` confirme que TOML sans `[mqtt]` donne `None` -- le `Default` et le TOML sont incoherents | `crates/miyualicia/src/config.rs:171` | Decider si `Default` doit aussi etre `None` pour MQTT, ou documenter la difference |

---

## 11. Optimisations recommandees

| # | Impact | Description | Effort |
|---|--------|-------------|--------|
| O-01 | Moyen | Ajouter des doc-tests dans les crates publics (0 doc-tests actuellement) | Moyen |
| O-02 | Faible | Deplacer les `[lints]` des Cargo.toml individuels vers `[lints] workspace = true` dans les crates qui ne l'utilisent pas encore (seul `miyualicia-automations` le fait) | Faible |
| O-03 | Moyen | Ajouter un test d'integration qui instancie `AliciaService` + `AutomationEngine` ensemble pour verifier le flux bout-en-bout | Moyen |
| O-04 | Faible | Le polling UI de 200ms dans `AliciaView` pourrait etre remplace par un signal push (channel) pour reduire la latence et la charge CPU | Moyen |

---

## 12. Metriques de performance

| Metrique | Valeur |
|---|---|
| Temps de build (8 crates, dev) | 1m16s |
| Temps de build (tests, dev) | 54.29s |
| Temps d'execution des 279 tests | ~1.2s |
| Taille du workspace Alicia | 8 crates, ~50 fichiers .rs |
| Clippy warnings | 0 |
| Couverture fonctionnelle estimee | Elevee (279 tests couvrent tous les modules) |

---

## 13. Parcours UX

### 13.1 Navigation UI

L'UI Alicia dans Central suit le pattern standard Miyukini :
- [x] ServiceMeta "alicia" dans le catalogue officiel (state.rs:162)
- [x] Navigation vers AliciaView depuis le panneau de services
- [x] 5 onglets avec TabButton coherent avec MiyukiniWatch
- [x] StatusBadge dynamique ("Alicia ecoute" / "En veille")
- [x] Gestion d'erreur d'initialisation visible dans l'UI
- [x] Theme coherent (utilise `current_theme.palette()` partout)

### 13.2 Points de friction identifies

- **Polling 200ms** : acceptable pour un usage desktop, mais cree une legere latence entre l'action (micro actif) et l'affichage du statut
- **Initialisation synchrone** : l'enumeration des devices audio est synchrone dans `use_effect`, ce qui pourrait bloquer brievement le thread UI si la machine a beaucoup de peripheriques audio

---

## Conclusion

**Avis : CONFORME**

Le livrable Alicia Home Assistante est conforme a sa documentation et aux standards du projet Miyukini COG. Les 8 crates compilent, les 279 tests passent, Clippy pedantic est propre, la securite est respectee (`forbid(unsafe_code)`, zero `unwrap()` en production, JWT non-hardcode), les Lois d'Autonomie sont satisfaites, les annotations MSCM sont completes, et l'UI Dioxus s'integre correctement dans Central.

Les 4 anomalies identifiees sont toutes mineures ou cosmetiques et n'impactent ni la fonctionnalite ni la securite.

**Score final : 96/100**

Deductions :
- -2 : noms de dossiers non renommes (A-01) -- confusion potentielle pour les nouveaux contributeurs
- -1 : incoherence `Default` vs TOML pour `mqtt` (A-04) -- ambiguite architecturale mineure
- -1 : absence de doc-tests (O-01) -- standard de documentation incomplet

---

*Rapport transmis a Alicia pour action.*
*Resultat archive avec Arianne.*
