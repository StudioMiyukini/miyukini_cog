<!-- @id mem.master.cog
     @do provide_memory_overview
     @role overview
     @layer memory
     @human Mémoire maître — vue d'ensemble Miyukini COG -->

# Miyukini COG — Mémoire

## Fichiers thématiques

> **Routage détaillé** : `.mip/memory/INDEX.md` (source unique). Ne pas dupliquer la table ici.

## Patterns confirmes (top 5 — details dans patterns-and-lessons.md)

- **spawn_blocking pour SQLite dans async** : toujours wrapper `blocking_lock()` dans `tokio::task::spawn_blocking`
- **`#[serde(default)]` pour retrocompatibilite** : tout nouveau champ dans une struct serialisee
- **Extraction variables avant RSX** : styles, labels, couleurs dans des `let` avant `rsx!{}`
- **Service embarque vs externe** : MiyuVoice = embarque dans Central, tous les autres = binaires independants
- **Themes const pour UI tokens** : `const UiTheme` resolus a la compilation, zero overhead

## Erreurs critiques (top 3 — details dans patterns-and-lessons.md)

- **Jamais d'URL externe en dur** dans le code source (LOI-1)
- **Jamais d'`unwrap()` en production** (7 occurrences connues dans `apps/central/src/`)
- **Jamais de passphrase par defaut hardcodee** : refuser le demarrage ou generer un secret aleatoire

## Intégration LLM (confirmé fév. 2026)

- `apps/miou-llm-bridge/` : Service IA local. Moteur natif GGUF (llama-cpp-2) + proxy upstream (LM Studio/Ollama). 17 agents, skills, contextes.
- `apps/central/src/llm_client.rs` : Client HTTP OpenAI-compatible. Bridge `http://localhost:11435`.
- Session distante : `MiouPreferences.use_remote_session` + `remote_session_url`. Toujours lire `MiouPreferences` pour resoudre l'endpoint.

## Service Market (confirmé fév. 2026)

- Crate `miyumarket` : protocole + manifeste partages entre Central et Origin. Catalogue statique + `ServiceManager` dynamique. Fallback local si Origin injoignable (LOI-2).

## Couverture MSCM (Mar 2026)

- `apps/central/src/` : 18/156 (11.5%). `miyuki-ui-*` : 115/115 (100%). `miyucloud` : 67/67 (100%). `.mip/certifications/` : 37/37 (100%).

## Audit central-improve-secure-update (Mar 2026)

- Inventaire COG hors MGE : project-file-map, 569 fichiers monolithiques (>400 lignes, règle I-14).
- Top 10 priorisé : pages.rs (3575L), content.rs (2440L), ui_builder.rs (2204L), etc. Plan découpage dans `<sequence>/plans_p3/`.
- MSCM index : 1578 blocs, 696 fichiers. mscm-generator dans tools/.
- Rapport : `.mip/sequences/2026-03-04-central-improve-secure-update/rapports_finaux/`.

## Services enregistres dans Central (Mar 2026)

jayxpose, jayfestival, jaykoa, jaykonta, miyukiniwatch, jay1tribu (deprecie par MiyuCloud), jaymanga, miyuclicker (Lord of the Click), lord_of_the_castle (Miyukini Survivor), miou-llm-bridge (Miyukini AI Studio), miyuvoice (MiyukiniVoice), miyucloud (MiyuCloud)

## MASS — Miyukini Agent Swarm System (confirmé mars 2026)

- Pattern de parallelisation MIP v2. Chantier T5 livre. Audit George 97/100 CONFORME.
- 3 couches : Orchestrateur Maria (DAG+Loi9) -> Pool Workers (subagent burst/worktree swarm/team swarm) -> Merge Coordinator Denis.
- Livres : `.mip/skills/miyukini-mip-workflow/SKILL.md`, `.mip/protocol/conventions.md` (Loi 9 + ref MASS), index agents `.mip/agents/INDEX.md`, swarm-template.json.
- Metriques : 24 taches, 7 vagues, 79% parallelisme effectif, 0 conflits, 8 commits. Rapport : `.mip/reports/2026-03-02-mass-rapport-final.md`.
- Anti-patterns AP-08/AP-09 : voir `.mip/memory/patterns-and-lessons.md`.

## Miyukini Whisper local stack (Mar 2026)

- Sequence `2026-03-05-miyukini-whisper-local-stack` cloturee jusqu a P6.
- Livrables: `miyustt`, `miyutts`, `miyukini-whisper-app`, integration Alicia + Central (scope sequence).
- Gate P4 valide (score securite 84/100). Gate P5: ACCEPTE AVEC RESERVES.
- Backlog reserves: hotkey globale, capture/injection texte systeme, rewrite bridge, docs/scripts onboarding.

## Miyukini Connect auth general (Mar 2026)

- Sequence `2026-03-05-miyukini-connect-auth-general` cloturee jusqu a P6.
- Remplacement auth Legacy de Central par `Miyukini Connect` effectue avec migration progressive legacy -> Argon2id.
- Gate P4 valide (score securite 71/100). Gate P5: ACCEPTE AVEC RESERVES (validation utilisateur explicite).
- Reserves ouvertes: dette lint `jayrdv` hors perimetre et ajout `cargo-audit` en CI avant merge final.
