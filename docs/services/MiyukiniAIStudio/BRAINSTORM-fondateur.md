# Miyukini AI Studio — Document Fondateur

## Contexte

Ce document est le brainstorming fondateur et l'etat des lieux complet du service **Miyukini AI Studio** (anciennement "MiouLLM Bridge"). Il definit la vision, le perimetre, l'architecture, les capacites actuelles, les manques identifies et la feuille de route pour faire de ce service le coeur intelligent de l'ecosysteme Miyukini COG.

## Portee / Scope

- **Applicable a :** Architecture complete du service IA local de l'ecosysteme COG.
- **Audience :** Equipe projet, utilisateur fondateur, agents IA.
- **Statut :** Document fondateur — AUDIT + VISION.
- **Redige par :** Maria (Chef de Projet)
- **Date :** 2026-02-27

---

## 1. Vision et philosophie

### 1.1 Principes fondateurs

| Principe | Description |
|----------|-------------|
| **Acces universel** | Tout le monde doit pouvoir utiliser l'IA, quel que soit son materiel |
| **Souverainete des donnees** | Aucune donnee ne quitte la machine sans consentement explicite |
| **Maitrise de l'entrainement** | L'utilisateur controle quel modele est charge et comment il est utilise |
| **Degradation gracieuse** | Le service fonctionne TOUJOURS, meme sans LLM (distant > local > proto-IA > refus) |
| **Anti-hallucination** | Mieux vaut refuser de repondre que d'inventer une reponse |
| **Securite par defaut** | Chaque agent a un perimetre strict, auditez, borne |
| **Integration ecosysteme** | L'IA est un service transversal qui enrichit TOUS les autres services COG |

### 1.2 Positionnement dans COG

Miyukini AI Studio est un **service interne de couche 7-8** (ServiceType::InterneCog) qui fonctionne comme un **hub IA central** :

```
Utilisateur
    |
    v
Central (Dioxus desktop) ---- Chat Miou dans le Salon
    |
    v
Miyukini AI Studio (:11435)
    |
    +--- Proxy vers LM Studio (:1234)
    +--- 17 agents specialises
    +--- 4 skills (fichiers, shell, web, services COG)
    +--- Tool calling (OpenAI function calling)
    +--- Bases de contexte
    +--- Recommandation de modeles
    +--- Proto-IA de secours
    |
    v
Autres services COG (JayKonta :11441, JayKoa :11442, ...)
    utilisent /v1/llm/chat pour avoir l'IA dans leur propre contexte
```

### 1.3 Ce qu'AI Studio n'est PAS

- Pas un clone de ChatGPT/Claude — c'est un orchestrateur d'agents specialises
- Pas un serveur d'inference — il delegue au LM Studio (ou equivalent) pour le calcul
- Pas un outil de dev exclusivement — l'equipe d'agents couvre business, marketing, compta, juridique, formation
- Pas un SaaS — tout tourne en local, sur la machine de l'utilisateur

---

## 2. Etat des lieux — Ce qui existe

### 2.1 Architecture actuelle

| Composant | Fichier | Statut |
|-----------|---------|--------|
| Serveur HTTP (Axum 0.8) | main.rs, proxy.rs | Fonctionnel |
| Config TOML | config.rs | Fonctionnel |
| Detection hardware (CPU/RAM/GPU) | hardware.rs | Fonctionnel (Windows seulement pour GPU) |
| Catalogue GGUF (12 modeles) | catalog.rs | Fonctionnel |
| Recommandation modeles | recommend.rs | Fonctionnel |
| 17 agents builtin | agents/ | Fonctionnel |
| Agents custom CRUD | agents/custom.rs | Fonctionnel |
| Skill : fichiers | skills/file_ops.rs | Fonctionnel |
| Skill : shell | skills/shell_exec.rs | Fonctionnel |
| Skill : web | skills/web_fetch.rs | Fonctionnel |
| Skill : services COG | skills/cog_services.rs | Fonctionnel |
| Tool calling (boucle 5 iter) | tools.rs | Fonctionnel |
| Bases de contexte CRUD | context.rs | Fonctionnel |
| Proto-IA de secours | fallback.rs | Fonctionnel |
| Securite (auth, rate limit, audit) | security.rs | Fonctionnel |
| API LLM partagee | llm_api.rs | Fonctionnel |
| Proxy passthrough | proxy.rs | Fonctionnel |

**Metriques :**
- 20 fichiers Rust, ~4 500 lignes
- 26 endpoints API
- 17 agents builtin, 4 skills, 5 tiers hardware, 12 modeles catalogues
- 0 tests, 0 documentation externe, 0 persistence

### 2.2 Les 26 endpoints API

#### Sante
| Route | Description |
|-------|-------------|
| `GET /health` | Status "ok" + version |
| `GET /status` | Bridge + upstream + agents/skills counts |

#### Hardware & Recommandation
| Route | Description |
|-------|-------------|
| `GET /v1/hardware` | Specs hardware (CPU, RAM, GPU, tier) |
| `GET /v1/recommend` | Recommandation du meilleur modele GGUF |

#### Agents (CRUD)
| Route | Description |
|-------|-------------|
| `GET /v1/agents` | Liste tous les agents (summaries) |
| `POST /v1/agents` | Creer un agent custom |
| `GET /v1/agents/{id}` | Detail d'un agent (avec system_prompt) |
| `PUT /v1/agents/{id}` | Modifier un agent (partiel) |
| `DELETE /v1/agents/{id}` | Supprimer un agent custom |
| `POST /v1/agents/{id}/chat` | Chat avec un agent (tool calling, contexte, fallback) |

#### Skills
| Route | Description |
|-------|-------------|
| `GET /v1/skills` | Liste les skills disponibles |
| `POST /v1/skills/{id}/execute` | Executer un skill (permissions + audit) |

#### Contextes (CRUD)
| Route | Description |
|-------|-------------|
| `GET /v1/contexts` | Liste les bases de contexte |
| `POST /v1/contexts` | Creer une base |
| `GET /v1/contexts/{id}` | Detail d'une base |
| `PUT /v1/contexts/{id}` | Modifier une base |
| `DELETE /v1/contexts/{id}` | Supprimer une base |

#### Equipes
| Route | Description |
|-------|-------------|
| `GET /v1/teams` | Liste les equipes |
| `POST /v1/teams` | Creer une equipe |
| `POST /v1/teams/{id}/task` | Assigner une tache (stub) |

#### API LLM partagee
| Route | Description |
|-------|-------------|
| `POST /v1/llm/chat` | Chat simple pour services COG |
| `POST /v1/llm/complete` | Completion texte |
| `GET /v1/llm/models` | Modeles charges + catalogue |
| `GET /v1/llm/status` | Disponibilite + hardware + modeles |

#### Disponibilite
| Route | Description |
|-------|-------------|
| `GET /v1/availability` | Probe upstream + local → status degradation |

#### Proxy
| Route | Description |
|-------|-------------|
| `ANY /v1/{*path}` | Passthrough transparent vers LM Studio |

### 2.3 L'equipe d'agents

| # | ID | Nom | Role | Skills | Securite | Dispatch |
|---|-----|------|------|--------|----------|----------|
| 1 | miou | Miou | Mascotte & Guide COG | cog_services | ReadOnly | Oui |
| 2 | alicia | Alicia | Assistante Personnelle / Gouvernante | file_ops, shell_exec, web_fetch, cog_services | Extended | Oui |
| 3 | maria | Maria | Chef de Projet | file_ops | Sandboxed | Oui |
| 4 | fabrice | Fabrice | Analyste PR | web_fetch, file_ops | Sandboxed | Non |
| 5 | denis | Denis | Chef Dev Senior | file_ops, shell_exec | Full | Oui |
| 6 | francois | Francois | Dev Back-End | file_ops, shell_exec | Extended | Oui |
| 7 | lise | Lise | Dev Front-End | file_ops, web_fetch | Extended | Oui |
| 8 | arianne | Arianne | Team Manager | file_ops, shell_exec, web_fetch, cog_services | Full | Oui |
| 9 | sofia | Sofia | Chef Marketing | web_fetch, file_ops | Sandboxed | Non |
| 10 | george | George | Audit Expert | file_ops, shell_exec, web_fetch | Extended | Non |
| 11 | julie | Julie | Formatrice | file_ops, web_fetch | Sandboxed | Non |
| 12 | hugo | Hugo | Comptable | cog_services, file_ops | Sandboxed | Non |
| 13 | clara | Clara | Community Manager | web_fetch, cog_services | Sandboxed | Non |
| 14 | victor | Victor | Commercial B2B | web_fetch, cog_services, file_ops | Sandboxed | Non |
| 15 | emma | Emma | Commerciale B2C | cog_services, web_fetch | ReadOnly | Non |
| 16 | louis | Louis | Conseil Juridique | web_fetch, file_ops | ReadOnly | Non |
| 17 | camille | Camille | Coach Bien-etre | (aucun) | ReadOnly | Non |

### 2.4 Niveaux de securite

| Niveau | Valeur | Capacites | Agents |
|--------|--------|-----------|--------|
| ReadOnly | 0 | Pas d'acces fichiers/shell | Miou, Emma, Louis, Camille |
| Sandboxed | 1 | Fichiers dans sandbox, web, services COG | Maria, Fabrice, Sofia, Julie, Hugo, Clara, Victor |
| Extended | 2 | Fichiers + shell + web | Alicia, Francois, Lise, George |
| Full | 3 | Acces complet | Denis, Arianne |

### 2.5 Degradation graduee

| Niveau | Condition | Capacites |
|--------|-----------|-----------|
| **RemoteOnline** | LM Studio upstream repond | Full power — tous les agents, tous les skills, tool calling |
| **LocalOnline** | LM Studio local (:1234) ou Ollama (:11434) repond | Capacite adaptee au modele charge |
| **Fallback** | Aucun LLM disponible | Proto-IA : salutations, status, aide, liste agents. Refuse tout le reste |
| **Offline** | Erreur critique | Rien ne fonctionne |

---

## 3. Manques identifies — Ce qui ne fonctionne pas / n'existe pas

### 3.1 Critiques (bloquants pour la production)

| # | Manque | Impact | Difficulte |
|---|--------|--------|------------|
| 1 | **Aucun test** | Impossible de garantir les regressions | Moyen |
| 2 | **Aucune persistence** | Tout est perdu au redemarrage (agents custom, contextes, audit, equipes) | Moyen |
| 3 | **Team dispatch est un stub** | `POST /v1/teams/{id}/task` retourne "dispatched" sans rien executer | Eleve |
| 4 | **Pas de gestion d'historique de chat** | Chaque requete est sans memoire — pas de conversation multi-tours | Moyen |
| 5 | **Auth non appliquee sur les endpoints agents/skills** | Seul le proxy passthrough verifie le Bearer token | Faible |

### 3.2 Importants (necessaires pour une utilisation reelle)

| # | Manque | Impact | Difficulte |
|---|--------|--------|------------|
| 6 | **Detection GPU hors Windows** | Sur Linux/Mac, pas de detection GPU → tier degrade | Moyen |
| 7 | **Pas de chargement natif GGUF** | Dependance absolue a LM Studio comme backend | Tres eleve |
| 8 | **Pas de streaming SSE** | Les reponses arrivent en bloc, pas en flux temps reel | Moyen |
| 9 | **Audit log non persiste** | Les 10 000 entrees en memoire sont perdues au redemarrage | Faible |
| 10 | **Catalogue statique** | Les 12 modeles sont codes en dur — pas de mise a jour dynamique | Faible |
| 11 | **Pas de documentation API** | Aucun README, aucun OpenAPI/Swagger | Faible |

### 3.3 Souhaitables (ameliorations de confort)

| # | Manque | Impact | Difficulte |
|---|--------|--------|------------|
| 12 | **Pas de chat multi-agents** | Un agent ne peut pas deleguer a un autre pendant le tool calling | Eleve |
| 13 | **Pas de gestion de voix** | Alicia ne peut pas communiquer par voix (TTS/STT) | Eleve |
| 14 | **Pas de metriques/monitoring** | Pas de compteurs de requetes, latence, tokens utilises | Moyen |
| 15 | **Pas d'embeddings/RAG** | Pas de recherche semantique dans les contextes | Eleve |
| 16 | **Pas de gestion de fichiers GGUF** | Pas de telechargement/suppression de modeles depuis l'API | Eleve |
| 17 | **Skill data_analysis absent** | Maria et Sofia sont prevues avec ce skill mais il n'existe pas | Moyen |

---

## 4. Integration avec l'ecosysteme COG

### 4.1 Services qui doivent consommer l'IA

| Service | Port | Usage IA prevu |
|---------|------|----------------|
| **Central** | — | Chat Miou dans le Salon, recommandation modeles, bulles IA |
| **JayKonta** | 11441 | Resume comptable, analyse depenses, generation de rapports |
| **JayKoa** | 11442 | Planification intelligente, resume d'agenda, suggestions |
| **JayXpose** | 11443 | Generation de descriptions produits, SEO, traduction |
| **JayFestival** | 11444 | Generation d'evenements, textes promotionnels |
| **MiyukiniWatch** | 11445 | Analyse d'habitudes, recommandations bien-etre |
| **Jay1Tribu** | 11446 | Moderation IA, suggestions sociales, chatbots |
| **JayManga** | 11447 | Traduction de manga, OCR, suggestions de lecture |
| **JayEcole** | (TBD) | Correction d'exercices, generation de contenu pedagogique, Miyu-sensei |
| **MGE** | — | PNJ IA, dialogues proceduraux, strategie IA |

### 4.2 Comment un service COG utilise l'IA

```
Service COG (ex: JayKonta)
    |
    | POST http://localhost:11435/v1/llm/chat
    | {
    |   "model": "default",
    |   "messages": [
    |     {"role": "system", "content": "Tu es un assistant comptable..."},
    |     {"role": "user", "content": "Resume mes depenses de fevrier"}
    |   ]
    | }
    |
    v
AI Studio
    |
    | Proxy vers LM Studio
    |
    v
Reponse JSON { "content": "Vos depenses de fevrier...", "model": "...", "usage": {...} }
```

### 4.3 Enregistrement dans le catalogue COG

Dans `apps/central/src/state.rs` :
```
id: "miou-llm-bridge"
name: "Miyukini AI Studio"  ✅ RENOMME
description: "Service IA local — inference GGUF native, agents specialises, skills, tool calling"
icon: "brain"
type: InterneCog
```

---

## 5. Architecture cible — PIVOT : Inference native GGUF

### 5.0 Decision architecturale majeure (2026-02-27)

**L'utilisateur a decide de supprimer la dependance a LM Studio.** AI Studio doit etre capable d'inferer localement de maniere autonome. LM Studio devient un backup optionnel, pas le moteur principal.

**Nouvelle chaine de degradation :**
```
NativeOnline → UpstreamOnline → Fallback → Offline
     |               |              |          |
 GGUF local    LM Studio/Ollama  Proto-IA    Erreur
 (primaire)     (backup)         (secours)   critique
```

**Crate choisi : `llama-cpp-2` v0.1.136** (bindings Rust pour llama.cpp)
- 473 stars, 69 contributeurs, mise a jour du 23/02/2026
- Support CUDA, Metal, CPU (AVX2/AVX-512/NEON)
- Feature flag `cuda` pour GPU NVIDIA
- API : model loading, context, batch, sampling, chat templates

### 5.1 Schema global (revise)

```
                    +-----------------------------------------+
                    |          Miyukini AI Studio              |
                    |         (axum :11435)                    |
                    +-----------------------------------------+
                    |                                         |
                    |  [Routeur HTTP — 30+ endpoints]          |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | InferenceRouter              [NEW] |  |
                    |  | - NativeBackend (llama-cpp-2)      |  |
                    |  | - UpstreamBackend (HTTP proxy)     |  |
                    |  | - Route auto : native > upstream   |  |
                    |  | - Chat completions OpenAI-compat   |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | ModelManager                 [NEW] |  |
                    |  | - Scan GGUF dans models_dir        |  |
                    |  | - Load / unload / swap              |  |
                    |  | - RAM/VRAM budget awareness         |  |
                    |  | - Metadata extraction (params, Q)   |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Agent Chat Engine                  |  |
                    |  | - System prompt injection          |  |
                    |  | - Contexte compilation             |  |
                    |  | - Tool calling loop (5 iter)       |  |
                    |  | - Historique de conversation  [NEW]|  |
                    |  | - Streaming SSE              [NEW] |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Agent Registry                     |  |
                    |  | - 17 builtin + custom CRUD         |  |
                    |  | - Teams & dispatch           [STUB]|  |
                    |  | - Inter-agent delegation     [NEW] |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Skill Layer                        |  |
                    |  | - file_ops (sandboxed)             |  |
                    |  | - shell_exec (whitelist)           |  |
                    |  | - web_fetch (restricted)           |  |
                    |  | - cog_services (COG inter-service) |  |
                    |  | - data_analysis             [NEW]  |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Hardware & Recommendation          |  |
                    |  | - sysinfo (CPU/RAM)                |  |
                    |  | - GPU detection (Windows+Linux)    |  |
                    |  | - Catalogue GGUF (dynamique) [NEW] |  |
                    |  | - Scoring 70% qualite + 30% FR     |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Security & Monitoring              |  |
                    |  | - Bearer auth (tous endpoints)     |  |
                    |  | - Rate limiting par identifiant    |  |
                    |  | - Audit log persiste         [NEW] |  |
                    |  | - Metriques (latence, tokens)[NEW] |  |
                    |  +-----------------------------------+  |
                    |                                         |
                    |  +-----------------------------------+  |
                    |  | Fallback Proto-IA                  |  |
                    |  | - Classification heuristique       |  |
                    |  | - Anti-hallucination (refus)       |  |
                    |  | - Degradation graduee (4 niveaux)  |  |
                    |  +-----------------------------------+  |
                    +-----------------------------------------+
                        |          |                |
                        v          v                v
                  GGUF local  LM Studio       Services COG
                  (models/)   (backup)        (:11441-11448)
```

### 5.2 Dependances (revisees)

```toml
# Actuelles
axum = "0.8"           # HTTP server
tokio = "1"            # Async runtime (fs, process, signal)
reqwest = "0.12"       # HTTP client (upstream backup)
serde = "1.0"          # Serialisation
serde_json = "1.0"     # JSON
toml = "0.8"           # Config
sysinfo = "0.33"       # Hardware detection
async-recursion = "1"  # File ops recursives
tracing = "0.1"        # Logging
tower-http = "0.6"     # CORS, tracing middleware

# NOUVEAU — Inference GGUF native
llama-cpp-2 = "0.1"    # Bindings llama.cpp — moteur principal

# A ajouter selon besoins futurs
# rusqlite / diesel       → Persistence SQLite (KindMother)
# axum-streams / sse      → Streaming SSE
# tokio-stream            → Stream utilities
```

### 5.3 Nouveaux modules

| Module | Fichier | Description |
|--------|---------|-------------|
| **InferenceRouter** | inference.rs | Abstraction qui route les completions vers NativeBackend ou UpstreamBackend |
| **ModelManager** | model_manager.rs | Scan, load, unload, swap de modeles GGUF locaux |
| **NativeBackend** | inference.rs | Inference via llama-cpp-2 (CPU/GPU) |
| **UpstreamBackend** | inference.rs | Proxy HTTP vers LM Studio/Ollama (backup) |

### 5.4 Nouveaux endpoints

| Route | Methode | Description |
|-------|---------|-------------|
| `GET /v1/models/local` | GET | Liste les modeles GGUF disponibles localement |
| `POST /v1/models/load` | POST | Charger un modele GGUF en memoire |
| `POST /v1/models/unload` | POST | Decharger le modele courant |
| `GET /v1/inference/status` | GET | Backend actif (native/upstream), modele charge, RAM/VRAM |

---

## 6. Feuille de route priorisee (REVISEE — pivot native)

### Phase 1 — Inference native GGUF (priorite immediate)

| # | Tache | Description | Fichiers |
|---|-------|-------------|----------|
| 1.1 | InferenceRouter + traits | Trait `InferenceBackend` avec `chat_completions()`, router qui choisit native > upstream | inference.rs |
| 1.2 | NativeBackend (llama-cpp-2) | Chargement GGUF, context, tokenization, sampling, chat completions | inference.rs |
| 1.3 | UpstreamBackend | Deplacer la logique HTTP proxy existante dans le trait | inference.rs |
| 1.4 | ModelManager | Scan models_dir, metadata GGUF, load/unload, RAM budget | model_manager.rs |
| 1.5 | Config native | Ajouter `models_dir`, `prefer_native`, `gpu_layers` dans bridge.toml | config.rs |
| 1.6 | Degradation revisee | NativeOnline > UpstreamOnline > Fallback > Offline | fallback.rs |
| 1.7 | Refactor proxy + tools + llm_api | Utiliser InferenceRouter au lieu de reqwest direct | proxy.rs, tools.rs, llm_api.rs |
| 1.8 | Endpoints modeles locaux | GET /v1/models/local, POST /v1/models/load, POST /v1/models/unload | proxy.rs |

### Phase 2 — Stabilisation

| # | Tache | Description | Fichiers |
|---|-------|-------------|----------|
| 2.1 | Tests unitaires | Tester inference, tools, security, catalog | tests/ |
| 2.2 | Persistence KindMother | Agents custom, contextes, audit, historique | persistence.rs |
| 2.3 | Auth sur tous les endpoints | Bearer token sur agents, skills, contextes | proxy.rs |
| 2.4 | Historique de conversation | Multi-tours avec persistence | proxy.rs |
| 2.5 | Delegation inter-agents | Skill `agent_dispatch` | skills/, tools.rs |

### Phase 3 — Innovation

| # | Tache | Description |
|---|-------|-------------|
| 3.1 | Streaming SSE | Reponses en flux temps reel |
| 3.2 | Embeddings / RAG | Recherche semantique dans les contextes |
| 3.3 | Voix TTS/STT | Alicia communique par voix |
| 3.4 | Telechargement GGUF | Telecharger des modeles depuis HuggingFace via API |
| 3.5 | MCP | Model Context Protocol |
| 3.6 | Metriques Prometheus | Monitoring |

---

## 7. Decisions de l'utilisateur (2026-02-27)

| # | Question | Decision | Impact |
|---|----------|----------|--------|
| 1 | **Persistence** | **KindMother (SQLite)** — utiliser le systeme de persistence existant de l'ecosysteme COG | Architecture |
| 2 | **Streaming** | **A decider** — question en suspens | UX |
| 3 | **GGUF natif** | **OUI — PRIORITE IMMEDIATE** — supprimer la dependance a LM Studio, inference native en primaire | Architecture majeure |
| 4 | **Renommage officiel** | **"Miyukini AI Studio"** confirme comme nom definitif | Branding |
| 5 | **Inter-agent delegation** | **Oui, pendant le chat** — un agent peut appeler un autre agent en cours de conversation | Complexite |
| 6 | **Chat UI** | **Reste dans le Salon de Central** — pas de fenetre separee | UX |
| 7 | **Dispatching principal** | **Alicia** est la gouvernante/dispatching principal | UX/Flux |
| 8 | **LM Studio** | **Backup uniquement** — plus de dependance, LM Studio = fallback optionnel | Architecture majeure |

### Implications des decisions

**Inference native GGUF (PRIORITE) :**
- `llama-cpp-2` v0.1.136 comme moteur d'inference principal
- Feature flag `cuda` pour support GPU NVIDIA
- Dossier `models/` configure dans bridge.toml pour stocker les GGUF
- AI Studio fonctionne de maniere 100% autonome sans logiciel tiers
- Gestion memoire : calcul automatique des couches GPU en fonction de la VRAM
- LM Studio/Ollama restent disponibles en backup via la meme API

**Persistence KindMother :**
- Reutiliser le crate `kindmother` (SQLite chiffre, deja utilise par les autres services COG)
- Stocker : agents custom, contextes, historique de conversations, audit log
- La config reste en TOML (bridge.toml)

**Inter-agent delegation :**
- Pendant le tool calling, un agent (ex: Alicia) peut invoquer un autre agent (ex: Hugo pour la compta)
- Necessite un nouveau skill `agent_dispatch` ou une extension du tool calling
- Limite de profondeur pour eviter les boucles infinies (ex: max 3 niveaux)

**Alicia comme dispatching principal :**
- Alicia recoit les requetes generales et les redirige vers l'agent specialise
- Miou reste la mascotte/guide mais c'est Alicia qui orchestre
- Le flux devient : Utilisateur → Alicia → Agent specialise → Reponse

---

## 8. Feuille de route revisee (post-pivot)

### Phase 1 — Inference native (EN COURS)

1. **Denis** : Implementer InferenceRouter + NativeBackend + UpstreamBackend (inference.rs)
2. **Denis** : Implementer ModelManager (model_manager.rs)
3. **Denis** : Mettre a jour config.rs avec les parametres natifs
4. **Denis** : Mettre a jour fallback.rs avec la nouvelle degradation
5. **Francois** : Refactorer proxy.rs, tools.rs, llm_api.rs pour utiliser InferenceRouter
6. ~~**Lise** : Renommer dans Central~~ ✅ FAIT

### Phase 2 — Stabilisation

7. **Francois** : Tests unitaires et d'integration
8. **Denis** : Persistence KindMother
9. **Denis** : Auth Bearer sur tous les endpoints
10. **Denis** : Historique de conversation (multi-tours)
11. **Francois** : Delegation inter-agents
12. **George** : Audit de securite

### Phase 3 — Innovation

13. Streaming SSE
14. Embeddings / RAG
15. Voix TTS/STT pour Alicia
16. Telechargement GGUF depuis HuggingFace
17. MCP / Metriques

---

## 9. Prochaines etapes immediates

1. ~~**Lise** : Renommer "MiouLLM Bridge" → "Miyukini AI Studio" dans Central~~ ✅ FAIT
2. **Denis** : Ajouter `llama-cpp-2` dans Cargo.toml avec feature flags
3. **Denis** : Creer `src/inference.rs` — trait InferenceBackend + NativeBackend + UpstreamBackend + InferenceRouter
4. **Denis** : Creer `src/model_manager.rs` — scan, load, unload, metadata
5. **Denis** : Mettre a jour `src/config.rs` — models_dir, prefer_native, gpu_layers
6. **Denis** : Mettre a jour `src/fallback.rs` — NativeOnline dans la chaine
7. **Francois** : Refactorer proxy.rs, tools.rs, llm_api.rs pour utiliser InferenceRouter
8. **Francois** : Ajouter les endpoints /v1/models/local, /v1/models/load, /v1/models/unload

---

*Document redige par Maria, Chef de Projet Miyukini AI Studio*
*Brainstorming fondateur — Etat des lieux + Vision + Feuille de route*
*Date : 2026-02-27 — MISE A JOUR : pivot inference native*
