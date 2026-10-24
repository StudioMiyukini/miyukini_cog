# TEAM — index

<!-- @id tools.team.index -->
<!-- @role app-reference -->
<!-- @layer S -->
<!-- @human Hub HTTP d'orchestration des nœuds MAIA : registre, heartbeat, file de tâches, séquences MIPOWER, passerelle MWS -->
<!-- @do reference_team -->

- **Crate** : `apps/team` (binaire `team`) · **Strate** : S (application autonome ; hors catalogue Central — classement à vérifier)
- **Ce n'est pas** un framework d'agents ni un toolkit : c'est un serveur axum (`apps/team/src/main.rs`)
- **Client connu** : `apps/miou-llm-bridge/src/team_client.rs` (MAIA en mode Pro : register, heartbeat 30 s, unregister)
- **Lien MIPOWER** : lit les séquences `.mip/sequences/` (voir [MIPOWER](../MIPOWER/_index.md))

## 1. Rôle

<!-- @id tools.team.role -->
<!-- @role functional-summary -->
<!-- @do summarize_team -->

- Registre en mémoire des nœuds MAIA (`nodes/registry.rs`) : statut, tier matériel, modèle actif, persona
- Boucle heartbeat : `GET {maia_url}/maia/health` sur chaque nœud sain ; stale après seuil (`nodes/heartbeat.rs`)
- File de tâches FIFO par priorité (`orchestrator/scheduler.rs`) ; types `agent_chat`, `mi_power_step`, `direct_relay`
- Sélection de nœud par tier minimum / motif de modèle (`orchestrator/router.rs`)
- Exécuteur d'étape MIPOWER : `POST {maia_url}/v1/agents/{agent_id}/chat` (`mipower/runner.rs`)
- Politique d'isolation réseau `airgap` / `internal_only` (défaut) / `unrestricted` (`mws/policy.rs`)

## 2. Points d'entrée

<!-- @id tools.team.api -->
<!-- @role public-api -->
<!-- @do list_team_routes -->

| Méthode | Route | Rôle |
|---|---|---|
| POST | `/team/register` | enregistre un nœud (`NodeInfo`) |
| POST | `/team/heartbeat` | heartbeat poussé par le nœud |
| DELETE | `/team/nodes/{id}` | désenregistrement |
| GET | `/team/nodes` | liste interne des nœuds |
| POST | `/api/task` | soumet une tâche → `task_id`, `pending` |
| GET | `/api/task/{id}` | état d'une tâche |
| GET | `/api/nodes` | nœuds + statut |
| GET | `/api/sequences` | séquences MIPOWER chargées |
| GET | `/api/status` | état global du hub |

- **Config** : `team.toml` ou `TEAM_CONFIG` (`config.rs`) ; écoute par défaut `0.0.0.0:7800`, heartbeat 30 s, stale 90 s
- **Lancement** : `team` (aucun argument CLI lu par `main.rs`)

## 3. Statut

<!-- @id tools.team.statut -->
<!-- @role status-measured -->
<!-- @do report_team_status -->

- Membre du workspace (`Cargo.toml`, section « Apps Standalone Services ») ; lints du workspace
- 22 tests unitaires : registry 6, policy 5, router 4, scheduler 4, sequence 3 ; aucun test des routes HTTP
- ~1 800 lignes Rust (`apps/team/src`)

## 4. Sécurité

<!-- @id tools.team.securite -->
<!-- @role security-notes -->
<!-- @do assess_team_security -->

- **Aucune authentification** sur les routes ; CORS `Any` (`main.rs`)
- `hmac_secret` déclaré (« anti-MITM ») mais **jamais lu** (`config.rs`)
- `is_allowed_destination` **non appelé** hors tests : la politique d'isolation n'est pas appliquée
- Un client peut enregistrer n'importe quelle `maia_url` → le heartbeat l'interroge (risque SSRF)
- Écoute par défaut sur toutes les interfaces (`0.0.0.0`)

## 5. Écarts connus

<!-- @id tools.team.ecarts -->
<!-- @role known-gaps -->
<!-- @do list_team_gaps -->

- Aucun worker ne consomme la file : `next_pending` n'est appelé qu'en test → tâches bloquées en `pending`
- `MiPowerRunner` jamais instancié dans `main.rs`
- `db_path` et dépendance `rusqlite` inutilisés : registre et file **non persistés**
- `reap_dead_nodes` jamais appelé hors tests
- Format de séquence attendu (`index.json` = liste d'étapes, `brief.md` par dossier) absent des dossiers réels de `.mip/sequences/` → 0 séquence chargée en pratique (à vérifier à l'exécution)
- Doc de module : `GET /api/agents` annoncée (`mws/routes.rs`) mais non routée ; usage `team serve --port` non implémenté
- `/api/nodes` : `match: true` codé en dur (« Simplification S3-E02 »)
