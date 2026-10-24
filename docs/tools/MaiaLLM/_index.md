# maia-llm — index

<!-- @id tools.maiallm.index -->
<!-- @role engine-reference -->
<!-- @layer T -->
<!-- @human Moteur d'inférence GGUF local (llama.cpp, CUDA par défaut) avec API compatible OpenAI ; moteur S1 de MAIA pour les petites configurations -->
<!-- @do reference_maia_llm -->

- **Binaire** : `maia-llm` · **Chemin** : `apps/maia-llm` (pas de crate `crates/maia-llm`)
- **Strate** : T (moteur d'inférence) — livré comme service HTTP local
- **Statut** : fonctionnel (inférence réelle llama.cpp) ; 3 tests, tous sur la config bind/auth (§6)

## 1. Rôle dans MAIA

<!-- @id tools.maiallm.maia -->
<!-- @role architecture-link -->
<!-- @do relate_maia_llm_to_maia -->

- Profil **S1 · petit modèle local** (≈ 1–4 B) de la [mémoire MAIA](../../architecture/MEMOIRE-MAIA.md) : reclasse, compresse, peut répondre seul
- Moteur retenu pour les profils P1 à P3 ; Strata au-delà ([petites configurations](../../architecture/MODELES-IA-PETITES-CONFIGS.md) §8)
- Consommateur : `apps/miou-llm-bridge` (`MaiaLlmClient`, `inference.rs`), URL `MAIA_LLM_URL`, défaut `http://localhost:11435`
- Prévu par l'architecture, **non implémenté** dans le code : modèles signés via canal officiel (EX-26), sandbox, choix de profil par CaringNanny

## 2. Ce que fait le moteur

<!-- @id tools.maiallm.fonctions -->
<!-- @role feature-summary -->
<!-- @do summarize_maia_llm -->

- Scanne `models_dir` (`*.gguf`), cache du scan 30 s (`store.rs`)
- Auto-chargement du plus gros modèle tenant dans 75 % de la RAM disponible (`/proc/meminfo` sous Linux, sinon 8 Go supposés)
- Inférence llama.cpp (`llama-cpp-2 =0.1.134`, workspace) ; GPU : features `cuda` (défaut), `rocm`, `metal`
- Chat complétion bloquante ou en flux SSE ; prompt au format **ChatML** fixe (`engine.rs` `build_chat_prompt`)
- Empreinte SHA-256 des modèles avec fichier compagnon `.gguf.sha256`

## 3. API

<!-- @id tools.maiallm.api -->
<!-- @role public-api -->
<!-- @do list_maia_llm_api -->

| Méthode | Route | Auth |
|---|---|---|
| GET | `/maia/llm/health` | publique |
| POST | `/v1/chat/completions` (OpenAI, `stream`, `temperature` 0.7, `max_tokens` 2048 par défaut) | protégée |
| GET | `/v1/models` | protégée |
| GET | `/maia/llm/models` | protégée |
| POST | `/maia/llm/models/load` (`{model}` : id, nom de fichier ou sous-chaîne) | protégée |
| DELETE | `/maia/llm/models/unload` | protégée |
| POST | `/maia/llm/models/{id}/hash` | protégée |

- CLI : `--bind`, `--models-dir`, `--gpu-layers`, `--auth-key`, `--no-auto-load` (`main.rs`)
- Health renvoie : statut, modèle chargé, taille, couches GPU, contexte, nombre de modèles, uptime

## 4. Données et stockage

<!-- @id tools.maiallm.donnees -->
<!-- @role data-storage -->
<!-- @do describe_maia_llm_storage -->

| Élément | Emplacement | Défaut |
|---|---|---|
| Config TOML | `$MAIA_DATA_DIR/maia-llm.toml` sinon `./maia-llm.toml` (créée si absente) | — |
| Modèles | `models_dir` | `models` |
| Empreintes | `<modèle>.gguf.sha256` | — |
| Contexte | `context_size` | 4096 |
| Couches GPU | `gpu_layers` | -1 (auto) |

- Aucun historique de conversation conservé ; état = modèle chargé (`Arc<Mutex<LlmState>>`)

## 5. Posture sécurité

<!-- @id tools.maiallm.securite -->
<!-- @role security-posture -->
<!-- @do assess_maia_llm_security -->

- **Bind** : `127.0.0.1:11435` par défaut ; hors loopback **sans** appelant épinglé → refus de démarrer (**fail-closed**, `config.rs` `validate_bind`, testé) ; nom d'hôte non parsable = non loopback
- **Auth** (lot 2, sortie) : requêtes signées par des clés épinglées (`appelants_inference` pour `/v1/*`, usage `maia-llm/inference` ; `appelants_modeles` pour `/maia/llm/models*`, usage `maia-llm/modeles`) ; famille sans clé → 503, même en loopback ; `auth_key` / `X-MAIA-Key` / `--auth-key` retirés (plus de pass-through)
- **CORS** : liste explicite, `*` ignoré, vide = aucun en-tête CORS
- **Chargement** : uniquement des fichiers trouvés dans `models_dir` (pas de chemin libre) ; correspondance partielle par sous-chaîne → modèle chargé ambigu possible
- **Gaps** :
  - `max_tokens` sans plafond côté serveur ; un seul `Mutex` tenu pendant toute la génération → une requête longue bloque les autres
  - ChatML construit par concaténation : `role` / `content` non échappés → injection de balises `<|im_start|>system` possible
  - Gabarit ChatML imposé quel que soit le modèle
  - Modèles non vérifiés (pas de signature ; SHA-256 informatif seulement)
  - Aucune gouvernance COG (MasterButler, LogisticsSteward pour les quotas)

## 6. Statut

<!-- @id tools.maiallm.statut -->
<!-- @role status -->
<!-- @do report_maia_llm_status -->

| Module | Tests |
|---|---|
| `config.rs` | 3 (loopback, refus sans clé, hôte non parsable) |
| `engine.rs`, `store.rs`, `auth.rs`, `api/*` | 0 |

- Inférence réelle (pas de stub) ; non couverte par des tests automatisés

## 7. Lacunes connues

<!-- @id tools.maiallm.lacunes -->
<!-- @role known-gaps -->
<!-- @do list_maia_llm_gaps -->

- **Conflit de port** : `miou-llm-bridge` écoute aussi sur `127.0.0.1:11435` par défaut (`apps/miou-llm-bridge/src/config.rs` `default_bind_addr`) et vise maia-llm sur ce même port (`main.rs` l. 114-116) ; Central vise aussi `localhost:11435` (`apps/central/src/llm_client.rs`) — à arbitrer
- Pas de tests sur auth, routes, store, moteur
- Pas de plafond de requêtes ni de file d'attente
- Canal officiel signé, sandbox et choix de profil automatique : prévus par l'architecture, absents du code
