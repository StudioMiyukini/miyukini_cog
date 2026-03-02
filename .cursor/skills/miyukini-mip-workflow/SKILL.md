# Skill: MIP v2 — Miyukini Implementation Protocol

## Quand utiliser ce skill

Utiliser ce skill pour **toute demande de developpement** impliquant du code, une nouvelle fonctionnalite, un fix, un refactor, ou un nouveau crate/service. Le protocole MIP v2 orchestre l'equipe et structure le travail.

MIP v2 est **universel** : il s'adapte a n'importe quel projet, stack, et environnement. Le noyau du protocole (classification, phases, gates, agents) est invariant. Seule la **configuration projet** change via la Phase SETUP.

---

## Phase SETUP — Onboarding Universel (UNE SEULE FOIS par environnement)

> La Phase SETUP s'execute **une seule fois** lors de la premiere utilisation de MIP dans un nouvel environnement. Elle produit un fichier `.mip/environment.md` qui configure tout le reste du protocole. Si `.mip/environment.md` existe deja, cette phase est **sautee**.

### Declenchement

La Phase SETUP est declenchee automatiquement si :
- Le fichier `.mip/environment.md` n'existe pas
- L'utilisateur lance la commande `/mip_setup`
- L'utilisateur demande une reconfiguration de l'environnement

### Structure de la Phase SETUP — 6 etapes

```
SETUP-1 : Detection automatique de l'environnement (scan systeme)
SETUP-2 : Configuration de l'environnement (questionnaire interactif)
SETUP-3 : Profil utilisateur (questionnaire interactif)
SETUP-4 : Detection du support IA (outil de travail)
SETUP-5 : Detection et installation des dependances
SETUP-6 : Inventaire des capacites et installation des agents
```

---

### SETUP-1 — Detection automatique de l'environnement

> L'agent scanne le terminal et le systeme pour collecter les informations de base. **Aucune interaction utilisateur requise.**

**Scan systeme** (via Bash) :

```bash
# OS et architecture
uname -a || systeminfo        # Linux/macOS || Windows
# Shell
echo $SHELL || echo $0        # Shell courant
# Hardware
nproc                         # CPU cores
free -h || systeminfo         # RAM disponible
df -h . || wmic logicaldisk   # Espace disque
nvidia-smi || rocm-smi        # GPU (si present)
# Reseau
ping -c 1 github.com          # Acces internet
curl -s ifconfig.me            # IP publique (si souhaitee)
# Git
git --version && git config user.name && git config user.email
git remote -v                  # Remotes configures
```

**Informations collectees** :

| Categorie | Donnees |
|-----------|---------|
| **OS** | Nom, version, architecture (x86_64, arm64) |
| **Shell** | Type (bash, zsh, powershell, fish, cmd), version |
| **CPU** | Modele, nombre de coeurs, frequence |
| **RAM** | Totale, disponible |
| **GPU** | Modele, VRAM (si present — important pour inference locale) |
| **Disque** | Espace total et libre sur la partition de travail |
| **Reseau** | Acces internet (oui/non), proxy detecte, VPN |
| **Git** | Version, user.name, user.email, remotes |
| **Container runtime** | Docker/Podman (detecte via `docker version` ou `podman version`) |

---

### SETUP-2 — Configuration de l'environnement (questionnaire interactif)

> L'utilisateur repond aux questions pour definir les contraintes du projet. Maria administre ce questionnaire.

#### 2.1 — Stack technique

| # | Question | Exemples de reponse |
|---|----------|---------------------|
| S2.1 | **Quel(s) langage(s) de programmation utilisez-vous ?** | Rust, Python, TypeScript, Go, Java, C#, C++, Kotlin, Swift, Ruby, PHP, etc. |
| S2.2 | **Quel(s) framework(s) ou librairies principales ?** | Dioxus, React, Vue, Angular, Django, FastAPI, Spring, .NET, Rails, etc. |
| S2.3 | **Quelle(s) base(s) de donnees ?** | SQLite, PostgreSQL, MySQL, MongoDB, Redis, DynamoDB, Supabase, Firebase, etc. |
| S2.4 | **Quel style d'API ?** | REST, GraphQL, gRPC, WebSocket, tRPC, aucun (monolithe), etc. |
| S2.5 | **Quel gestionnaire de paquets / build tool ?** | Cargo, npm/yarn/pnpm/bun, pip/poetry/uv, Maven/Gradle, CMake, Go modules, etc. |
| S2.6 | **Monorepo ou multi-repo ?** | Monorepo (Cargo workspace, Nx, Turborepo, Lerna) / Multi-repo / Repo unique |
| S2.7 | **Y a-t-il des conventions de code existantes ?** (linter, formatteur, styleguide) | Clippy, ESLint, Prettier, Black, gofmt, .editorconfig, etc. |

#### 2.2 — Securite & conformite

| # | Question | Exemples de reponse |
|---|----------|---------------------|
| S2.8 | **Quel niveau de securite est requis ?** | Standard, renforce (crypto, audit), critique (finance, sante, defense) |
| S2.9 | **Conformite reglementaire ?** | RGPD, HIPAA, SOC2, PCI-DSS, aucune, autre |
| S2.10 | **Chiffrement des donnees ?** | At-rest, in-transit, E2E, aucun, deja en place |
| S2.11 | **Gestion des secrets ?** | Variables d'environnement, vault (HashiCorp, AWS Secrets Manager), fichier .env, aucune |

#### 2.3 — Infrastructure & deploiement

| # | Question | Exemples de reponse |
|---|----------|---------------------|
| S2.12 | **Ou le code est-il heberge ?** | GitHub, GitLab, Bitbucket, Gitea, auto-heberge, aucun |
| S2.13 | **CI/CD en place ?** | GitHub Actions, GitLab CI, Jenkins, CircleCI, aucun |
| S2.14 | **Conteneurisation ?** | Docker, Podman, Kubernetes, Docker Compose, aucun |
| S2.15 | **Hebergement / deploiement ?** | VPS (OVH, Hetzner, DigitalOcean), cloud (AWS, GCP, Azure), PaaS (Vercel, Railway, Fly.io), self-hosted, local uniquement |
| S2.16 | **Authentification ?** | OAuth2, JWT, sessions, SAML, passkeys, auth maison, aucun |

#### 2.4 — Dossier de travail & structure

| # | Question | Exemples de reponse |
|---|----------|---------------------|
| S2.17 | **Quel est le dossier racine du projet ?** | (auto-detecte si possible, sinon demander) |
| S2.18 | **Y a-t-il une structure de dossiers imposee ?** | Convention du framework, custom, aucune |
| S2.19 | **Existe-t-il deja un CLAUDE.md / AGENTS.md / rules ?** | Oui (les lire) / Non (les creer) |

---

### SETUP-3 — Profil utilisateur (questionnaire interactif)

> L'utilisateur se presente pour que les agents adaptent leur communication et leur niveau de detail.

| # | Question | Type |
|---|----------|------|
| S3.1 | **Quel est votre role ?** | Solo dev, dev en equipe, tech lead, CTO, architecte, etudiant, designer, PM, autre |
| S3.2 | **Quel est votre niveau d'experience en dev ?** | Debutant (<1 an), junior (1-3 ans), intermediaire (3-5 ans), senior (5-10 ans), expert (10+ ans) |
| S3.3 | **Quels langages maitrisez-vous ?** | Liste libre |
| S3.4 | **Quels domaines de connaissance maitrisez-vous ?** | Web, mobile, systeme, embarque, data, IA/ML, securite, devops, game dev, autre |
| S3.5 | **Quel est votre style de travail prefere ?** | Autonome (laissez l'IA faire), collaboratif (discuter avant d'agir), superviseur (valider chaque etape) |
| S3.6 | **Quel niveau de detail attendez-vous dans les communications ?** | Concis (resultat seulement), normal (resultat + explication courte), detaille (tout expliquer) |
| S3.7 | **Quel est votre objectif principal avec ce projet ?** | Livrer un produit, apprendre, prototyper, maintenir, refactorer, autre |
| S3.8 | **Y a-t-il des regles ou conventions NON NEGOCIABLES pour vous ?** | Liste libre (ex: "jamais de unwrap", "toujours des tests", "pas de dependances externes", etc.) |

**Mapping automatique** :

| Style S3.5 | Mode d'autonomie par defaut |
|------------|----------------------------|
| Autonome | FULL |
| Collaboratif | BIG_STEPS |
| Superviseur | GUIDED |

---

### SETUP-4 — Detection du support IA (outil de travail)

> Identifier l'outil IA avec lequel l'utilisateur travaille et ses capacites.

#### 4.1 — Auto-detection

L'agent detecte l'outil courant via des indices d'environnement :

| Indice | Outil detecte |
|--------|---------------|
| Variable `CLAUDE_CODE` ou contexte `claude-code` | **Claude Code CLI** |
| Variable `CURSOR_*` ou `.cursor/` present | **Cursor IDE** |
| Variable `VSCODE_*` sans `CURSOR_*` | **VS Code** (verifier extensions) |
| Variable `CODEX_*` ou `openai-codex` | **OpenAI Codex CLI** |
| Contexte MCP server actif | Outil MCP-compatible (identifier lequel) |
| Aucun indice | Demander a l'utilisateur |

#### 4.2 — Questionnaire complementaire

| # | Question | Options |
|---|----------|---------|
| S4.1 | **Quel est votre outil IA principal ?** (auto-detecte : `{outil}`) | Claude Code, Cursor, VS Code + Copilot, VS Code + Continue.dev, VS Code + Cline, OpenAI Codex, Gemini Code Assist, JetBrains AI, Zed, Windsurf, Amazon Q, Aider, OpenCode, Goose, Gemini CLI, Autre |
| S4.2 | **Utilisez-vous de l'inference locale ?** | LM Studio, Ollama, llama.cpp, vLLM, Jan, LocalAI, aucun |
| S4.3 | **Modele(s) IA prefere(s) ?** | Claude Opus, Claude Sonnet, Claude Haiku, GPT-4.5, GPT-5, Gemini 2.0, Codestral, DeepSeek, Qwen, Llama, Mixtral, autre, pas de preference |
| S4.4 | **Budget IA mensuel ?** | Gratuit, <$20/mois, $20-100/mois, $100+/mois, budget entreprise, illimite |

#### 4.3 — Matrice de compatibilite MIP

En fonction de l'outil detecte, MIP adapte ses capacites :

| Capacite MIP | Requis | Claude Code | Cursor | VS Code + ext | Codex CLI | Aider | JetBrains | Inference locale |
|--------------|--------|-------------|--------|---------------|-----------|-------|-----------|-----------------|
| **Agents paralleles** (subagent) | T3+ | Natif | Via Agent | Via extension | Natif | Non | Non | Non |
| **Terminal access** (Bash) | Toutes | Natif | Natif | Via terminal | Natif | Natif | Limite | Via shell |
| **Multi-file edit** | T2+ | Natif | Natif | Selon ext | Natif | Natif | Natif | Selon outil |
| **MCP support** | Opt | Client+Srv | Client | Selon ext | Client+Srv | Non | Server | Selon runtime |
| **Git integration** | T2+ | Natif | Natif | Via terminal | Natif | Natif | Natif | Via terminal |
| **TodoWrite tracking** | Toutes | Natif | Non | Non | Non | Non | Non | Non |
| **Background tasks** | T3+ | Natif | Limite | Non | Natif | Non | Non | Non |
| **Context7 docs** | T3+ | Via MCP | Via MCP | Via MCP | Via MCP | Non | Via MCP | Non |

**Adaptations automatiques** :

| Si l'outil ne supporte PAS... | MIP fait... |
|-------------------------------|-------------|
| Agents paralleles | Execution sequentielle (Francois puis Lise) |
| TodoWrite | Annonces texte dans le chat uniquement |
| MCP / Context7 | Skip les verifications Context7, s'appuyer sur la memoire locale |
| Background tasks | Tout en foreground, checkpoints plus frequents |
| Terminal access | Demander a l'utilisateur de lancer les commandes manuellement |

---

### SETUP-5 — Detection et installation des dependances

> Scanner l'environnement pour verifier que les outils necessaires sont installes. Proposer l'installation des manquants.

#### 5.1 — Invariants universels (toujours verifies)

| Categorie | Outil | Commande de detection | Installation si manquant |
|-----------|-------|-----------------------|--------------------------|
| **VCS** | git | `git --version` | `winget install Git.Git` / `brew install git` / `apt install git` |
| **Shell** | bash/zsh | `echo $SHELL` | Generalement pre-installe |

#### 5.2 — Detection par stack (selon SETUP-2)

**Rust** :
```bash
rustc --version          # Compilateur
cargo --version          # Build tool
rustup show              # Toolchain
cargo clippy --version   # Linter
cargo fmt --version      # Formatteur
```

**JavaScript/TypeScript** :
```bash
node --version           # Runtime
npm --version / yarn --version / pnpm --version / bun --version  # Package manager
npx tsc --version        # TypeScript compiler
npx eslint --version     # Linter
npx prettier --version   # Formatteur
```

**Python** :
```bash
python3 --version        # Runtime
pip --version / poetry --version / uv --version  # Package manager
python3 -m pytest --version  # Test framework
python3 -m mypy --version    # Type checker
ruff --version               # Linter/formatteur
```

**Go** :
```bash
go version               # Runtime + compilateur
golangci-lint --version  # Linter
```

**Java/Kotlin** :
```bash
java --version           # JDK
mvn --version / gradle --version  # Build tool
```

**C/C++** :
```bash
gcc --version / clang --version  # Compilateur
cmake --version                  # Build system
```

#### 5.3 — Outils transversaux

| Outil | Detection | Usage |
|-------|-----------|-------|
| Docker | `docker version` | Conteneurisation |
| docker-compose | `docker compose version` | Orchestration |
| kubectl | `kubectl version --client` | Kubernetes |
| gh (GitHub CLI) | `gh --version` | GitHub API |
| curl | `curl --version` | HTTP requests |
| jq | `jq --version` | JSON processing |
| openssl | `openssl version` | Crypto |

#### 5.4 — Rapport de detection

```markdown
## Rapport de detection des dependances

### Installes
- git 2.43.0
- rustc 1.77.0
- cargo 1.77.0
- clippy 0.1.77
- ...

### Manquants
- docker : Non installe → Installer ? (requis pour S2.14: Docker Compose)
- gh : Non installe → Installer ? (recommande pour GitHub integration)

### Avertissements
- rustc 1.77.0 : Version 1.80+ recommandee pour async closures
```

L'agent propose l'installation des outils manquants (avec la commande adaptee a l'OS detecte en SETUP-1). L'utilisateur valide chaque installation.

---

### SETUP-6 — Inventaire des capacites et installation des agents

> Configurer les agents MIP en fonction de l'environnement et de la stack detectes.

#### 6.1 — Agents MIP universels (noyau invariant)

Le noyau MIP definit **7 roles fonctionnels** independants de la stack :

| Role | Agent par defaut | Responsabilite universelle |
|------|-----------------|---------------------------|
| **Chef de Projet** | Maria | Classification, brainstorming, coordination, synthese |
| **Analyste Produit** | Fabrice | Audit concurrence, cibles, fonctionnalites, points de friction |
| **Chef Dev** | Denis | Architecture, documentation technique, plan, integration, livraison |
| **Dev Back-End** | Francois | Implementation logique metier, API, DB, tests |
| **Dev Front-End** | Lise | Implementation UI/UX, composants, theme, assets |
| **Team Manager** | Arianne | Qualite, memoire, anti-hallucination, archivage, capitalisation |
| **Audit Expert** | George | Conformite, tests globaux, UX audit, securite |

#### 6.2 — Adaptation des agents a la stack

En fonction de la stack detectee (SETUP-2), les agents sont configures avec :

| Element adapte | Source | Exemple Rust | Exemple TypeScript | Exemple Python |
|----------------|--------|-------------|-------------------|----------------|
| **Commande build** | S2.5 | `cargo build --workspace` | `npm run build` | `python -m build` |
| **Commande test** | S2.5 | `cargo test --workspace` | `npm test` | `pytest` |
| **Commande lint** | S2.7 | `cargo clippy -- -D warnings` | `npx eslint .` | `ruff check .` |
| **Commande format** | S2.7 | `cargo fmt` | `npx prettier --write .` | `ruff format .` |
| **Structure crate/package** | S2.6 | `crates/{nom}/src/` | `packages/{nom}/src/` | `src/{nom}/` |
| **Convention commit** | S2.12 | `type(scope): msg` | `type(scope): msg` | `type(scope): msg` |
| **Annotations code** | Config | MSCM (`@id`, `@do`) | JSDoc / TSDoc | Docstrings |
| **Pattern test** | S2.5 | `#[test]` + `#[cfg(test)]` | `describe/it` (Jest/Vitest) | `def test_*` (pytest) |
| **Gestion erreurs** | Config | `Result<T, Error>` | `try/catch` + types | `try/except` + types |
| **Feature flags** | Config | Cargo features | Env vars / config | Env vars / config |

#### 6.3 — Generation des fichiers de configuration

La Phase SETUP produit les fichiers suivants :

| Fichier | Contenu |
|---------|---------|
| `.mip/environment.md` | **Configuration maitre** : OS, hardware, stack, securite, infra, outil IA, dependances |
| `memory/user-profile.md` | Profil utilisateur : role, experience, preferences, mode autonomie |
| `memory/project-file-map.md` | Carte des fichiers cles du projet (generee par scan du dossier) |
| `memory/stack-patterns.md` | Patterns specifiques a la stack (equivalent de rust-patterns.md, adapte) |
| `memory/stack-cheatsheet.md` | Cheatsheet du framework principal (equivalent de dioxus-cheatsheet.md, adapte) |
| `CLAUDE.md` | Conventions projet (cree ou augmente si existant) |
| `.claude/agents/*.md` | Agents adaptes a la stack (si Claude Code) |

#### 6.4 — Template `.mip/environment.md`

```markdown
# MIP Environment Configuration

## TL;DR
<Stack, OS, outil IA, mode autonomie, resume en 3 lignes>

## Metadata
- Date de configuration: YYYY-MM-DD
- Version MIP: v2.1
- Reconfigurable via: `/mip_setup`

## Systeme
- OS: <nom> <version> (<arch>)
- Shell: <type> <version>
- CPU: <modele> (<N> coeurs)
- RAM: <total> (<disponible> libre)
- GPU: <modele> (<VRAM>) | Aucun
- Disque: <total> (<libre> libre)
- Reseau: Internet <oui/non>, Proxy <oui/non>

## Stack technique
- Langage(s): <liste>
- Framework(s): <liste>
- Base(s) de donnees: <liste>
- API style: <REST/GraphQL/gRPC/etc.>
- Package manager: <nom>
- Monorepo: <oui/non> (<type>)
- Linter: <nom + config>
- Formatteur: <nom + config>
- Test framework: <nom>

## Commandes standard
- Build: `<commande>`
- Test: `<commande>`
- Lint: `<commande>`
- Format: `<commande>`
- Test single: `<commande avec placeholder>`

## Securite
- Niveau: <standard/renforce/critique>
- Conformite: <RGPD/HIPAA/SOC2/aucune>
- Chiffrement: <at-rest/in-transit/E2E/aucun>
- Secrets: <env vars/vault/fichier .env>

## Infrastructure
- Hebergement code: <GitHub/GitLab/Bitbucket/etc.>
- CI/CD: <GitHub Actions/etc./aucun>
- Conteneurisation: <Docker/Podman/aucun>
- Deploiement: <VPS/Cloud/PaaS/local>
- Auth: <OAuth2/JWT/sessions/aucun>

## Outil IA
- Outil principal: <nom>
- Inference locale: <LM Studio/Ollama/aucun>
- Modele(s): <liste>
- Budget: <fourchette>
- Capacites MIP adaptees:
  - Agents paralleles: <oui/non>
  - TodoWrite: <oui/non>
  - Context7/MCP: <oui/non>
  - Background tasks: <oui/non>
  - Terminal access: <oui/non>

## Dependances
### Installees
- <outil>: <version>

### Manquantes (installees durant SETUP)
- <outil>: <version installee>

### Avertissements
- <outil>: <avertissement>

## Conventions du projet
- Convention commit: <Conventional Commits / custom / aucune>
- Annotations code: <MSCM / JSDoc / Docstrings / aucune>
- Pattern erreurs: <Result<T,E> / try-catch / try-except / custom>
- Regles NON NEGOCIABLES utilisateur: <liste depuis S3.8>
```

---

### SETUP — Invariants universels MIP

Les elements suivants sont des **invariants** du protocole MIP, independants de la stack, du langage, ou de l'environnement :

| Invariant | Description | Universel car... |
|-----------|-------------|------------------|
| **Classification T1-T5** | Trier la complexite avant d'agir | Toute tache a une taille |
| **Git (VCS)** | Branching, commits, merge | Standard industrie universel |
| **Tests avant livraison** | Verifier que le code fonctionne | Fondamental, tout langage |
| **Linting/formatting** | Code propre et coherent | Chaque langage a ses outils |
| **Code review / audit** | Verification par un tiers | Bonne pratique universelle |
| **Chiffrement** | Proteger les donnees sensibles | Obligation legale et technique |
| **Gestion des secrets** | Ne pas hardcoder de credentials | OWASP Top 10, universel |
| **CI/CD** | Automatiser build/test/deploy | Standard industrie |
| **Documentation** | Expliquer le code et les decisions | Maintenance long terme |
| **Metriques & feedback** | Mesurer pour ameliorer | Kaizen / amelioration continue |

### SETUP — Elements projet-specifiques (configures)

| Element | Miyukini COG | Autre projet (exemple) |
|---------|-------------|----------------------|
| Langage | Rust | TypeScript |
| Framework UI | Dioxus 0.6 | React + Next.js |
| DB | KindMother (SQLite) | PostgreSQL + Prisma |
| API | axum REST | tRPC |
| Architecture | Pyramide COG, Strates 0-9 | Clean Architecture |
| Annotations | MSCM (@id, @do, @role) | TSDoc + ESLint rules |
| Lois/regles | LOI-1 a LOI-8 | Pas de dependance critique |
| Build | `cargo build --workspace` | `npm run build` |
| Test | `cargo test --workspace` | `npx vitest` |
| Lint | `cargo clippy -- -D warnings` | `npx eslint .` |

---

### Commandes universelles

| Commande | Action |
|----------|--------|
| `/mip_setup` | Relancer la Phase SETUP (reconfiguration) |
| `/mip_status` | Afficher le statut de l'environnement MIP |
| `/autonomy_mode <mode>` | Changer le mode d'autonomie (FULL/BIG_STEPS/GUIDED) |

---

## Etape 1 — Classification (OBLIGATOIRE)

Avant toute action, classer la demande :

| Classe | Critere | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 |
| **T2** | Fix cible, 1-3 fichiers, bug connu | P2 → P3 → P5 |
| **T3** | Feature moderee, 3-10 fichiers | P0 → P3 → P4 → P5 → P6 |
| **T4** | Feature majeure, 10+ fichiers, multi-crate | P0 → P3 → P4 → P5 → P6 |
| **T5** | Chantier strategique, nouveau crate/app | P0 → P3 → P4 → P5 → P6 |

**Regle** : En cas de doute, classer UN CRAN AU-DESSUS.

**Qui classifie** : Maria (Chef de Projet). En son absence, l'utilisateur ou le coordinateur Claude.

---

## Etape 2 — Routing des phases

### P0 — Cadrage complet : Brainstorming, Analyse, Specification & Planification (T3+)

**Agents** : Maria (lead) + Lise (direction visuelle) + Fabrice (analyse PR, T4-T5) + Denis (inventaire + plan) + Francois (spec technique) + Arianne (audit de faisabilite)

P0 est **LA phase humaine** : elle determine la direction de tout le travail. Aucun code ne sera ecrit avant la fin de P0. Le brainstorming est **structure en 8 temps**. Apres approbation du brief P0, **tout est automatique** (P3 → P6).

#### Suivi P0 — Annonces temps reel

Chaque Temps P0 est **trace via TodoWrite** pour que l'utilisateur puisse suivre la progression de l'equipe. A la **completion de chaque Temps**, l'agent responsable **annonce dans le chat** :

```
[YYYY-MM-DD HH:MM] ✓ P0 Temps X — <Nom du Temps> termine.
  Agent(s): <liste>
  Resultat: <resume en 1-2 lignes>
  → Prochaine etape: Temps X+1 — <Nom>
```

La TodoWrite P0 contient un item par Temps :
```
[ ] Temps 1 — Exploration & Brainstorming (Maria)
[ ] Temps 2 — Ideation (Maria + Lise)
[ ] Temps 3 — Analyse concurrentielle (Fabrice) [T4-T5]
[ ] Temps 4 — Inventaire des prerequis (Denis + equipe)
[ ] Temps 5 — Specification technique + Context7 (Francois)
[ ] Temps 6 — Plan exhaustif & Guide d'implementation (Denis)
[ ] Temps 7 — Audit de faisabilite (Arianne)
[ ] Temps 8 — Synthese & Brief (Maria)
```

#### Temps 1 — Exploration & Brainstorming structure (Maria)

Maria reformule la demande, creuse le contexte, et guide l'utilisateur a travers un **questionnaire de brainstorming structure** inspire de methodes reconnues (Design Thinking, Six Thinking Hats, SCAMPER, 5 Whys, How Might We).

**Etapes** :

1. **Reformuler** la demande utilisateur en termes precis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** : lire les fichiers concernes (Glob, Grep, Read) pour comprendre l'etat actuel
4. **Administrer le questionnaire de brainstorming** (voir ci-dessous) — poser les questions par section, adapter selon la classe T et le contexte
5. **Identifier les contraintes** : Lois d'Autonomie applicables, stack technique, compatibilite existante

**Hard gate** : NE PAS passer au temps 2 sans reponses de l'utilisateur.

---

#### Questionnaire de Brainstorming Standard — P0

> **Objectif** : Guider l'utilisateur pour extraire les informations et decisions necessaires au cadrage du projet. Chaque section s'inspire d'une methode de brainstorming reconnue. Maria **adapte les questions** au contexte : certaines sont universelles, d'autres reservees aux projets complexes (T4-T5). Les questions marquees `[OPT]` sont optionnelles pour T3.

##### Section 1 — COMPRENDRE : Le probleme et son contexte
*Inspire de : Design Thinking (Empathize/Define) + 5 Whys*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 1.1 | **Quel probleme ou besoin cette demande resout-elle ?** Decrivez la situation actuelle et ce qui ne va pas ou manque. | Design Thinking: Empathize | T3-T5 |
| 1.2 | **Pourquoi maintenant ?** Qu'est-ce qui declenche cette demande aujourd'hui ? (urgence, opportunite, prerequis pour un autre projet...) | 5 Whys (niveau 1) | T3-T5 |
| 1.3 | **Qui est l'utilisateur final ?** Qui va utiliser cette fonctionnalite au quotidien ? (vous-meme, un autre utilisateur, un service, une API...) | Design Thinking: Define | T3-T5 |
| 1.4 | **Quel est le parcours actuel ?** Si une solution partielle existe deja, comment l'utilisateur fait-il aujourd'hui ? Quels sont les points de friction ? | Design Thinking: Empathize | T3-T5 |
| 1.5 | `[OPT]` **Pourquoi cette approche plutot qu'une autre ?** Si vous avez deja une idee de solution, qu'est-ce qui vous y a mene ? (creuser les "pourquoi" sous-jacents) | 5 Whys (niveaux 2-3) | T4-T5 |

##### Section 2 — CADRER : Faits, contraintes et priorites
*Inspire de : Six Thinking Hats (White Hat: faits, Blue Hat: processus)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 2.1 | **Quelles sont les contraintes techniques connues ?** (stack, versions, dependances, performance, compatibilite, plateforme...) | White Hat: Faits | T3-T5 |
| 2.2 | **Quel est le perimetre souhaite ?** Listez ce qui doit etre INCLUS et ce qui est EXCLU explicitement. | Blue Hat: Processus | T3-T5 |
| 2.3 | **Quelle est la priorite ?** Classez par importance : (a) fonctionnalite minimale viable, (b) ameliorations souhaitees, (c) bonus / nice-to-have. | Blue Hat: Processus | T3-T5 |
| 2.4 | `[OPT]` **Y a-t-il une deadline ou un jalon externe ?** (release, demo, dependance d'un autre projet...) | White Hat: Faits | T4-T5 |
| 2.5 | `[OPT]` **Quelles donnees ou references avez-vous ?** (maquettes, specs existantes, exemples, liens, captures d'ecran...) | White Hat: Faits | T4-T5 |

##### Section 3 — IMAGINER : Idees, alternatives et inspiration
*Inspire de : Six Thinking Hats (Green Hat: creativite) + SCAMPER*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 3.1 | **Avez-vous deja des idees ou preferences d'approche technique ?** Decrivez meme partiellement — toute piste est utile. | Green Hat: Creativite | T3-T5 |
| 3.2 | **Existe-t-il dans le projet quelque chose de similaire qu'on pourrait adapter ?** (un service, composant, pattern, crate existant...) | SCAMPER: Adapter | T3-T5 |
| 3.3 | `[OPT]` **Peut-on combiner avec une fonctionnalite existante ou prevue ?** (fusionner deux besoins en un seul dev) | SCAMPER: Combiner | T4-T5 |
| 3.4 | `[OPT]` **Que peut-on eliminer pour simplifier ?** Y a-t-il des aspects non-essentiels qu'on pourrait retirer pour un MVP plus rapide ? | SCAMPER: Eliminer | T4-T5 |
| 3.5 | `[OPT]` **Connaissez-vous des produits/services qui font quelque chose de similaire ?** (inspiration concurrence, references visuelles ou fonctionnelles) | SCAMPER: Adapter | T4-T5 |
| 3.6 | `[OPT]` **"How Might We..."** — Comment pourrait-on reformuler le probleme en opportunite ? (ex: "Comment pourrait-on rendre le partage de fichiers aussi simple qu'un glisser-deposer ?") | How Might We | T5 |

##### Section 4 — EVALUER : Risques, benefices et intuition
*Inspire de : Six Thinking Hats (Yellow Hat: valeur, Black Hat: risques, Red Hat: intuition)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 4.1 | **Quel est le benefice principal attendu ?** Une fois livre, quelle est LA chose qui doit fonctionner ? | Yellow Hat: Valeur | T3-T5 |
| 4.2 | **Quels risques ou difficultes anticipez-vous ?** (techniques, UX, compatibilite, performance, securite...) | Black Hat: Risques | T3-T5 |
| 4.3 | **Quelle est votre intuition sur la complexite ?** (simple / modere / complexe / tres complexe) | Red Hat: Intuition | T3-T5 |
| 4.4 | `[OPT]` **Quelle importance strategique ?** (1 = utilitaire, 5 = critique pour l'ecosysteme Miyukini) | Red Hat: Intuition | T4-T5 |
| 4.5 | `[OPT]` **Que se passe-t-il si on ne fait PAS ce projet ?** (impact de l'inaction) | Reverse Brainstorming | T4-T5 |

##### Section 5 — DECIDER : Arbitrages et priorites
*Inspire de : Lightning Decision Jam (LDJ)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 5.1 | **Quelle est la fonctionnalite MINIMALE viable ?** Si on ne pouvait livrer qu'une seule chose, ce serait quoi ? | LDJ: Prioriser | T3-T5 |
| 5.2 | **Preference de compromis ?** En cas de tension, que privilegier : (a) rapidite de livraison, (b) exhaustivite fonctionnelle, (c) robustesse/qualite ? | LDJ: Arbitrer | T3-T5 |
| 5.3 | `[OPT]` **Qu'est-ce qui peut etre reporte a un prochain sprint ?** (fonctionnalites phase 2, optimisations, polish...) | LDJ: Reporter | T4-T5 |
| 5.4 | `[OPT]` **Y a-t-il des decisions deja verrouillees ?** (choix techniques, patterns, conventions qui ne sont pas negociables pour ce projet) | LDJ: Contraindre | T4-T5 |

---

**Utilisation par Maria** :

- **T3** : Poser les questions non-`[OPT]` (12 questions). Adapter selon le contexte — si la reponse est evidente, ne pas insister.
- **T4** : Poser toutes les questions (20 questions). Regrouper en 2-3 messages pour ne pas submerger l'utilisateur.
- **T5** : Poser toutes les questions (21 questions) + question HMW (3.6). Accepter des reponses longues et encourager la reflexion.
- **Boucle MIP** (retour P5 → P0) : Ne re-poser que les sections 1 et 4 en les orientant sur les **ecarts constates** et les **corrections souhaitees**.

**Hard gate inchangee** : NE PAS passer au temps 2 sans reponses suffisantes de l'utilisateur.

#### Temps 2 — Ideation (Maria + Lise en parallele)

Deux explorations paralleles :

**Maria** — Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. Definir le **perimetre** : IN / OUT explicite
3. Identifier les **risques** et leurs mitigations
4. Proposer **2-3 approches** techniques differentes avec pros/cons

**Lise** (T3+ si la tache a un aspect front/UI) — Vision graphique :
1. Analyser l'**UI existante** (theme, composants, patterns visuels en place)
2. Proposer la **direction artistique** : style, ton, inspirations visuelles
3. Decrire le **parcours utilisateur** (flux ecran par ecran, interactions)
4. Identifier les **composants** a creer/reutiliser (atomic design : atomes, molecules, organismes)
5. Si pertinent, referencer des **inspirations visuelles** (apps concurrentes, design systems)

#### Temps 3 — Analyse concurrentielle (Fabrice, T4-T5 seulement)

**Fabrice** (lance en parallele du temps 2 pour T4-T5) :
1. Identifier les **produits/services concurrents**
2. Analyser **forces et faiblesses** de chaque concurrent
3. Identifier la **cible utilisateur** et ses attentes
4. Lister les **fonctionnalites differenciantes** a envisager
5. Detecter les **points de friction** des concurrents

#### Temps 4 — Inventaire des prerequis (Denis + equipe)

**Denis** (lead) coordonne un inventaire complet de tout ce qui est necessaire pour realiser le projet. Chaque agent du perimetre contribue a sa section.

**Objectif** : Produire une **carte exhaustive des prerequis** AVANT la spec et le plan, pour que ceux-ci soient ultra-detailles et sans angle mort.

**1. Competences requises** (par agent) :

| Agent | Inventorier |
|-------|-------------|
| **Francois** | Competences Rust necessaires (traits, async, unsafe patterns a eviter, crates a maitriser). Technologies backend (axum, serde, SQLite, crypto...) |
| **Lise** | Competences UI necessaires (Dioxus 0.6 patterns, RSX, signals, atomic design). Technologies frontend (CSS, assets, theme system) |
| **Denis** | Competences architecture (patterns COG, integration inter-crates, tests, CI/CD) |

**2. Connaissances necessaires** :

- **Domaine metier** : Quelles connaissances metier l'equipe doit posseder ? (ex: protocoles crypto pour MiyuCloud, regles de jeu pour Sodomight)
- **Patterns existants** : Quels patterns du projet doivent etre connus ? (charger depuis `memory/mip-decisions.md`)
- **Anti-patterns** : Quelles erreurs doivent etre connues ? (charger depuis `memory/mip-antipatterns.md` et MEMORY.md)
- **Documentation** : Quelles docs sont necessaires ? (CLAUDE.md, skills, docs externes via Context7)

**3. Outils et ressources necessaires** :

| Categorie | Inventorier |
|-----------|-------------|
| **Crates externes** | Liste des dependances avec versions minimales, statut de maintenance, compatibilite |
| **Crates internes** | Crates du workspace a utiliser/modifier, types et traits a connaitre |
| **Outils dev** | Compilateur, Context7 IDs, CLI tools, formatteurs, linters |
| **Assets** | Fichiers graphiques, polices, icones, sons a creer ou reutiliser |
| **Infrastructure** | Serveurs, ports, certificats, configs reseau si applicable |
| **Docs & refs** | Liens Context7, pages de documentation, specs externes |

**4. Etapes generales du projet** :

Denis decompose le projet en **etapes macro** (avant le plan atomique du Temps 6) :

```markdown
## Etapes generales — <titre du projet>

### Etape 1 — <nom>
- **Objectif** : <ce que cette etape accomplit>
- **Agents** : <qui travaille>
- **Prerequis** : <ce qui doit etre fait avant>
- **Livrables** : <ce qui est produit>
- **Critere de completion** : <comment savoir que c'est fini>
- **Risques identifies** : <ce qui pourrait bloquer>

### Etape 2 — <nom>
[...]
```

**5. Matrice de disponibilite** :

| Prerequis | Statut | Action si manquant |
|-----------|--------|--------------------|
| Crate X v1.2 | Disponible | — |
| Pattern Y | Connu (memory) | — |
| Asset Z | A creer | Tache Lise pre-planifiee |
| Competence W | Non maitrisee | Consultation Context7 + doc |

**Output** : Section "Inventaire des prerequis" integree au brief. Alimente directement Francois (Temps 5 : spec) et Denis (Temps 6 : plan).

**Annonce** :
```
[YYYY-MM-DD HH:MM] ✓ P0 Temps 4 — Inventaire des prerequis termine.
  Agent(s): Denis (lead), Francois, Lise
  Resultat: X competences, Y outils, Z etapes generales inventories. Manquants: N
  → Prochaine etape: Temps 5 — Specification technique (Francois)
```

---

#### Temps 5 — Specification technique + Verification Context7 (Francois)

**Francois** analyse le contexte technique, **verifie les docs actuelles**, et produit la spec :

1. **Explorer le code existant** en profondeur (Glob, Grep, Read)
2. **Verification Context7 obligatoire** — Pour chaque librairie/framework implique :
   - Appeler `resolve-library-id` pour identifier la lib
   - Appeler `query-docs` pour verifier les patterns/API actuels
   - **Libs a toujours verifier** : Dioxus (`/dioxuslabs/dioxus`), axum (`/tokio-rs/axum`), serde (`/serde-rs/serde`), tokio, et toute lib ajoutee
   - Documenter les **breaking changes** ou **deprecations** detectees
   - Comparer avec les patterns existants dans le code — signaler les ecarts
3. **Charger les anti-patterns connus** : Lire `memory/mip-antipatterns.md` et `memory/MEMORY.md` (section "Erreurs a ne pas repeter") — verifier qu'aucun pattern interdit n'est planifie
4. **Identifier les fichiers** a modifier/creer avec numeros de ligne
5. **Definir les types, traits, API** (signatures completes — validees contre les docs Context7)
6. **Evaluer les dependances** entre modules et crates
7. **Verifier la conformite architecturale** :
   - [ ] Lois d'Autonomie respectees (LOI-1 a LOI-8)
   - [ ] `unsafe_code = "forbid"` dans tout nouveau Cargo.toml
   - [ ] Strate correcte dans la pyramide COG
   - [ ] Annotations MSCM planifiees (@id, @do, @role, @layer)
   - [ ] Versions des dependances a jour (pas de crates deprecated)
8. **Documenter** les risques techniques identifies

**Output supplementaire** : Section "Verification documentaire" dans la spec avec :
- Libs verifiees + versions
- Breaking changes detectes
- Anti-patterns evites
- Ecarts code existant vs docs actuelles

Artefact : `.mip/specs/YYYY-MM-DD-<slug>.md` — **Doit commencer par un TL;DR de 5 lignes max.**

#### Temps 6 — Plan exhaustif & Guide d'implementation detaille (Denis)

**Denis** compile l'inventaire (Temps 4) + la spec de Francois (Temps 5) et produit le **plan exhaustif avec guide d'implementation detaille** couvrant TOUTE la chaine de production. L'inventaire des prerequis alimente directement ce plan — chaque etape macro est decomposee en taches atomiques :

1. **Decomposer en taches atomiques** (2-5 minutes chacune)
2. **Couvrir exhaustivement** les categories suivantes :

| Categorie | Contenu |
|-----------|---------|
| **Code** | Implementation back-end (Francois) + front-end (Lise) |
| **Tests unitaires** | Un test minimum par fonction/methode ajoutee |
| **Tests d'integration** | Tests de flux complets (API, UI flows) |
| **Tests generaux** | `cargo test --workspace`, `cargo clippy --workspace -- -D warnings` |
| **Audit** | Checklist George (MSCM, securite, UX, conformite) |
| **Corrections** | Taches de correction pre-planifiees (buffer 20% des taches) |

3. **Chaque tache DOIT contenir** :
   - Numero sequentiel et categorie (`[CODE-01]`, `[TEST-U-01]`, `[TEST-I-01]`, `[AUDIT-01]`, etc.)
   - Agent assigne (Francois, Lise, Denis, George)
   - Fichier(s) exact(s) a modifier (chemin complet)
   - Code complet a ecrire (pas de "ajouter de la validation")
   - Commande de test : `cargo test -p {crate} -- {pattern}`
   - Output attendu : `"test xxx ... ok"`
   - Message de commit : `"type(scope): description"`
   - Dependances : liste des taches prerequises (ex: `depends: [CODE-01, CODE-02]`)

4. **Principe** : Presumer que l'executant n'a AUCUN contexte projet.

5. **Ordonnancement** : Les taches sont ordonnees par dependance. Les taches independantes sont marquees comme parallelisables.

6. **Guide d'implementation detaille** — Pour chaque etape macro (du Temps 4), Denis produit un **guide integre au plan** :

```markdown
## Guide d'implementation — Etape X : <nom>

### Prerequis de l'etape
- Competences : <listees dans l'inventaire Temps 4>
- Outils : <verifies disponibles>
- Crates/deps : <avec versions>
- Connaissances : <patterns a appliquer, anti-patterns a eviter>
- Docs Context7 a consulter : <IDs + queries recommandees>

### Taches atomiques de l'etape
[CODE-01] → [CODE-02] → [TEST-U-01] → ...

### Critere de completion de l'etape
- [ ] Tous les tests de l'etape passent
- [ ] Clippy propre sur les crates touches
- [ ] Code review (checkpoint Denis si ≥5 taches)
- [ ] Annonce dans le chat avec timestamp
```

Le guide sert de **feuille de route detaillee** pour Francois et Lise en P3. Chaque etape terminee est annoncee dans le chat avec date/heure.

Artefact : `.mip/plans/YYYY-MM-DD-<slug>.md` — **Doit commencer par un TL;DR de 5 lignes max.**

#### Temps 7 — Audit de faisabilite & Conformite (Arianne)

**Arianne** verifie que le projet est **realisable tel que planifie**, que les agents, dependances et outils sont conformes, et qu'il n'y a ni trou ni ambiguite.

**Verification des agents** :
1. **Agents necessaires** : Verifier que chaque tache du plan a un agent assigne et que cet agent possede les competences requises (consulter `memory/team-skills-audit.md`)
2. **Capacite du modele** : Evaluer si le modele LLM utilise est capable de la complexite des taches planifiees. Si risque de deviation → recommander un modele different ou un decoupage plus fin
3. **Coherence inter-agents** : Verifier que les outputs attendus de chaque agent correspondent aux inputs attendus par les agents suivants (pas de gap)

**Verification des dependances** :
4. **Crates externes** : Verifier que toutes les dependances listees dans la spec existent, sont maintenues, et sont compatibles entre elles (versions)
5. **Crates internes** : Verifier que les crates du workspace utilises existent et que les types/traits references sont bien definis
6. **Outils** : Verifier que tous les outils necessaires au dev sont disponibles (compilateur, Context7 IDs, outils CLI, assets)

**Verification contre la memoire** :
7. **Anti-patterns** : Relire `memory/mip-antipatterns.md` — verifier qu'aucune tache ne reproduit une erreur connue
8. **Patterns confirmes** : Relire `memory/mip-decisions.md` — verifier que les patterns confirmes sont bien utilises
9. **Historique** : Consulter `memory/mip-performance-history.md` — si un projet similaire a deja ete fait, en tirer des lecons

**Verification Context7** (complement de Francois) :
10. **Spot-check** : Verifier via Context7 que 2-3 patterns critiques du plan sont bien valides (ex: RSX signal patterns, axum middleware, serde derives)
11. **Breaking changes recents** : Verifier si les libs ont ete mises a jour depuis la derniere sequence MIP

**Diagnostic** :

| Resultat | Action |
|----------|--------|
| **Conforme** | Feu vert → Maria compile le brief (Temps 8) |
| **Trous mineurs** | Lister les manques, suggerer les complements, corriger le plan |
| **Ambiguite** | Identifier les points flous, poser des questions a l'utilisateur ou a l'agent concerne |
| **Manque critique** (outil, crate, skill agent) | Suggerer la **creation des manquants** comme projet precurseur |
| **Projet irrealisable tel quel** | Suggerer une **reorientation** : decomposer en un projet precurseur (prereqs) + projet final |

**Suggestion de projet precurseur** : Si Arianne detecte qu'il manque un crate, un outil, ou une competence pour realiser le projet, elle propose un **mini-projet precurseur** (T2-T3) a realiser d'abord, qui debloquera le projet principal. Le brief est alors modifie pour inclure cette dependance.

Artefact : Section "Audit de faisabilite" integree au brief (pas d'artefact separe en P0)

#### Temps 8 — Synthese & Brief (Maria)

Maria compile tout dans le brief final :

1. **Fusionner** les contributions de tous les agents (Maria + Lise + Fabrice + Francois + Denis + Arianne)
2. **Integrer l'audit d'Arianne** : section conformite, alertes, prerequis eventuels
3. **Rediger le brief structure** avec toutes les sections
4. **Presenter les approches** avec la recommandation de l'equipe
5. **Si projet precurseur detecte** : presenter les deux projets (precurseur + final) et demander l'ordre de priorite
6. **Inclure le plan exhaustif** de Denis en annexe du brief
7. Artefact : `.mip/briefs/YYYY-MM-DD-<slug>.md`

**Template du brief** :

```markdown
# Brief: <titre>

## Metadata
- Classe: T3/T4/T5
- Date: YYYY-MM-DD
- Demandeur: utilisateur

## Contexte
[Pourquoi cette demande, quel probleme elle resout]

## Objectifs
- Objectif principal: ...
- Objectifs secondaires: ...
- Criteres de succes mesurables: ...

## Perimetre
### Inclus
- [Fonctionnalites IN]
### Exclus
- [Fonctionnalites OUT — explicitement rejetees]

## Approches proposees
### Approche A — [nom] (RECOMMANDEE)
- Description: ...
- Pros: ...
- Cons: ...
- Effort: ...

### Approche B — [nom]
- Description: ...
- Pros: ...
- Cons: ...

## Direction visuelle (par Lise)
- Style/ton: ...
- Composants identifies: [atomes, molecules, organismes]
- Parcours utilisateur: [flux ecran par ecran]
- Inspirations: ...

## Analyse concurrentielle (par Fabrice, T4-T5)
- Concurrents: ...
- Differenciateurs: ...
- Cible utilisateur: ...

## Inventaire des prerequis (par Denis + equipe)
### Competences requises
- Back-end (Francois): [liste]
- Front-end (Lise): [liste]
- Architecture (Denis): [liste]

### Connaissances necessaires
- Domaine metier: [liste]
- Patterns a appliquer: [depuis mip-decisions.md]
- Anti-patterns a eviter: [depuis mip-antipatterns.md]

### Outils et ressources
| Prerequis | Statut | Action si manquant |
|-----------|--------|--------------------|
| ... | Disponible / A creer / Manquant | ... |

### Etapes generales du projet
1. Etape 1 — <nom> : [objectif, agents, livrables, critere completion]
2. Etape 2 — <nom> : [...]

## Specification technique (par Francois)
- Fichiers modifies/crees: [liste avec numeros de ligne]
- Types et API definis: [signatures]
- Conformite: [checklist LOI, MSCM, unsafe]
- Risques techniques: [liste]

## Plan de developpement exhaustif (par Denis)
[Voir annexe .mip/plans/YYYY-MM-DD-<slug>.md]
- Nombre total de taches: X
  - Code: X taches (Y Francois, Z Lise)
  - Tests unitaires: X taches
  - Tests integration: X taches
  - Tests generaux: X taches
  - Audit: X taches
  - Buffer corrections: X taches

## Audit de faisabilite (par Arianne)
### Conformite agents
- Agents necessaires: [liste avec competences verifiees]
- Capacite modele LLM: [OK / risque identifie]
- Coherence inter-agents: [OK / gaps identifies]

### Conformite dependances
- Crates externes: [toutes verifiees / manquants]
- Crates internes: [tous presents / manquants]
- Outils: [tous disponibles / manquants]

### Verification memoire
- Anti-patterns evites: [liste]
- Patterns confirmes appliques: [liste]
- Lecons historiques: [si applicable]

### Verdict faisabilite
- **CONFORME** / **TROUS MINEURS** (corriges) / **PREREQUIS NECESSAIRE**
- Si prerequis: [description du projet precurseur]

## Contraintes
- Lois d'Autonomie: LOI-x applicables
- Stack: ...
- Compatibilite: ...

## Risques
| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|------------|
| ... | ... | ... | ... |

## TL;DR (5 lignes max)
<Resume ultra-concis du projet, approche recommandee, effort estime, risque principal, etape critique>

## Decision
APPROUVE / REJETE / MODIFIE / PREREQUIS D'ABORD

## Mode d'autonomie
- [ ] **FULL** — Autopilot complet (P3→P6 automatique, seul P5 test humain)
- [ ] **BIG_STEPS** — Gates aux grandes etapes (validation humaine entre P3→P4, P4→P5)
- [ ] **GUIDED** — Accompagnement continu (humain valide chaque etape macro du guide)

Garder ce mode pour toutes les futures sequences MIP ? OUI / NON / JE SAIS PAS
```

**Quality Gate P0** : Utilisateur approuve le brief ET choisit l'approche ET choisit le mode d'autonomie.

**Hard gate** : AUCUN passage en execution sans brief approuve. En mode FULL, c'est la **DERNIERE intervention humaine** avant la livraison (sauf bug/delta majeur).

---

### Modes d'autonomie — Comportement detaille

Le mode d'autonomie determine **combien de gates humaines** existent entre P0 et P5.

#### Mode FULL (Autopilot complet)

Comportement actuel. Apres approbation du brief, P3→P6 s'executent automatiquement. L'utilisateur ne re-intervient qu'en P5 (test humain + verdict) ou si frein d'urgence.

```
P0 [GATE] → P3 automatique → P4 automatique → P5 [GATE test humain] → P6 automatique
```

**Ideal pour** : Taches bien cadrées (T3), utilisateur confiant dans l'equipe, projets sans ambiguïté technique.

#### Mode BIG_STEPS (Gates aux grandes etapes)

L'execution est automatique DANS chaque phase, mais l'utilisateur valide la transition entre les grandes phases. 3 gates intermediaires.

```
P0 [GATE] → P3 automatique → [GATE resume P3] → P4 automatique → [GATE resume audit] → P5 [GATE test humain] → P6 automatique
```

**Gates supplementaires** :
- **Gate P3→P4** : Denis presente un resume de l'implementation (taches completees, tests, auto-corrections). L'utilisateur peut : CONTINUER / CORRIGER / STOPPER.
- **Gate P4→P5** : George presente le rapport d'audit. L'utilisateur peut : CONTINUER / CORRIGER / STOPPER.

**Ideal pour** : Features majeures (T4), premiers projets, domaines à risque.

#### Mode GUIDED (Accompagnement continu)

L'humain est implique a chaque etape macro du guide d'implementation. Le plus interactif.

```
P0 [GATE] → Etape 1 [GATE] → Etape 2 [GATE] → ... → Etape N [GATE] → P4 [GATE] → P5 [GATE] → P6
```

**Gates supplementaires** :
- **Gate par etape macro** : A la fin de chaque etape du guide, l'agent presente le code ecrit, les tests, et demande validation avant de continuer.
- **L'utilisateur peut** : VALIDER / MODIFIER (donner des instructions) / REVENIR (refaire l'etape) / SAUTER (passer a l'etape suivante).

**Ideal pour** : Chantiers strategiques (T5), domaines inconnus, l'utilisateur veut apprendre ou superviser de pres.

#### Persistance et commande `/autonomy_mode`

Le mode choisi est enregistre dans `memory/user-profile.md` et s'applique par defaut aux sequences suivantes. L'utilisateur peut changer a tout moment avec :

```
/autonomy_mode full       # Passer en autopilot complet
/autonomy_mode big_steps  # Passer en gates aux grandes etapes
/autonomy_mode guided     # Passer en accompagnement continu
```

Si l'utilisateur a repondu "JE SAIS PAS" a la question de persistance, Maria redemande a chaque nouveau P0.

**Stockage** : `memory/user-profile.md` — section "Preferences de travail" :
```markdown
## Preferences de travail
- Mode d'autonomie par defaut: FULL | BIG_STEPS | GUIDED
- Persistance confirmee: OUI | NON | NON_DECIDE
```

---

## Metriques & Horodatage — Collecte continue

> Tout au long de la sequence MIP, les agents collectent des metriques pour mesurer la performance de l'equipe et alimenter le rapport final.

### Initialisation (debut de sequence)

A l'ouverture de chaque sequence MIP, Maria cree le fichier `.mip/metrics/YYYY-MM-DD-<slug>.json` avec la structure suivante :

```json
{
  "project": {
    "title": "<titre du brief>",
    "description": "<description courte>",
    "class": "T3|T4|T5",
    "slug": "<slug>",
    "mip_sequence_number": 1,
    "autonomy_mode": "FULL|BIG_STEPS|GUIDED"
  },
  "timestamps": {
    "p0_start": "ISO8601",
    "p0_end": null,
    "autopilot_start": null,
    "p3_start": null, "p3_end": null,
    "p4_start": null, "p4_end": null,
    "p5_start": null, "p5_end": null,
    "p5_test_start": null, "p5_test_end": null,
    "p6_start": null, "p6_end": null,
    "total_end": null
  },
  "counters": {
    "lines_written": 0,
    "lines_deleted": 0,
    "crates_touched": [],
    "crates_created": [],
    "files_created": 0,
    "files_modified": 0,
    "commits": 0,
    "agents_engaged": [],
    "mip_loops": 1,
    "unit_tests_total": 0,
    "unit_tests_failed": 0,
    "integration_tests_total": 0,
    "integration_tests_failed": 0,
    "global_tests_total": 0,
    "global_tests_failed": 0,
    "auto_corrections": 0,
    "audits": 0,
    "audit_defects": [],
    "emergency_brakes": 0
  },
  "human_interventions": [],
  "agent_questions": [],
  "satisfaction": null,
  "notes": null
}
```

### Collecte par phase

| Phase | Qui collecte | Quoi |
|-------|-------------|------|
| **P0** | Maria | `p0_start`, `p0_end`, `agents_engaged`, questions posees a l'humain |
| **Git** | Denis | `autopilot_start` |
| **P3** | Francois/Lise | `p3_start`, `p3_end`, `lines_written/deleted`, `commits`, `unit_tests_*`, `auto_corrections`, `crates_touched` |
| **P4** | Denis/George | `p4_start`, `p4_end`, `audits`, `audit_defects[]`, `global_tests_*`, `integration_tests_*` |
| **P5** | Denis | `p5_start`, `p5_end`, `p5_test_start`, `p5_test_end`, `satisfaction`, `human_interventions[]` |
| **P6** | Arianne | `p6_start`, `p6_end`, `total_end`, compilation du rapport final |

### Enregistrement des interventions humaines

Chaque intervention humaine est loggee avec :
```json
{
  "timestamp": "ISO8601",
  "type": "precision|arret|pause|changement_direction|constat_erreur|delta|autre",
  "phase": "P0|P3|P4|P5",
  "description": "<description de l'intervention>",
  "impact": "aucun|mineur|majeur|critique"
}
```

### Enregistrement des questions agents → humain

Chaque question posee a l'utilisateur est loggee avec :
```json
{
  "timestamp": "ISO8601",
  "agent": "Maria|Denis|Francois|Lise|George|Arianne|Fabrice",
  "phase": "P0|P3|P4|P5",
  "nature": "clarification|validation|choix_technique|choix_design|blocage|autre",
  "question": "<texte de la question>",
  "response_summary": "<resume de la reponse>"
}
```

---

## MODE EXECUTION — P3 a P6 (apres approbation P0)

> **Le comportement depend du mode d'autonomie** choisi par l'utilisateur en P0.
>
> - **FULL** : Execution entierement automatique. L'utilisateur n'intervient plus sauf P5 (test) ou frein d'urgence.
> - **BIG_STEPS** : Execution automatique par phase. Gates humaines entre P3→P4 et P4→P5.
> - **GUIDED** : L'utilisateur valide chaque etape macro du guide d'implementation.
>
> Le mode par defaut est lu depuis `memory/user-profile.md`. Si absent, Maria demande en P0.

### Git Branch Setup (premiere action de l'AUTOPILOT)

Avant toute implementation, creer une branche de feature et la pousser sur le remote :

```bash
# Creer la branche depuis main
git checkout -b feat/<slug>    # slug = nom court de la feature (ex: feat/miyuvoice)

# Pousser la branche pour suivi distant
git push -u origin feat/<slug>
```

**Convention de nommage des branches** :
- `feat/<slug>` — Nouvelle fonctionnalite (T3-T5)
- `fix/<slug>` — Correction de bug (T1-T2)
- `refactor/<slug>` — Refactoring (T3+)

Le `<slug>` est derive du titre du brief (ex: brief "Ajouter MiyuVoice" → `feat/miyuvoice`).

### Logging obligatoire

**Chaque tache** du plan exhaustif est tracee via **TodoWrite** pour que l'utilisateur puisse suivre l'avancement en temps reel :

- Chaque tache commence par un `TodoWrite` qui la passe en `in_progress`
- Chaque tache terminee est immediatement marquee `completed`
- Les erreurs/blocages sont signales dans le statut de la tache
- L'utilisateur voit la progression sans avoir a intervenir

### Frein d'urgence

L'autopilot s'arrete UNIQUEMENT si :
1. **Bug bloquant** : un test echoue apres 2 tentatives de correction automatique
2. **Delta majeur** : une tache revele un probleme qui remet en question le plan (ex: API incompatible, dependance cassee)
3. **Echec audit** : George identifie un defaut CRITIQUE que Denis ne peut pas corriger automatiquement

Dans ces cas, l'agent qui detecte le probleme **arrete l'autopilot** et **presente le probleme a l'utilisateur** avec une proposition de resolution.

---

### P3 — Implementation (toutes classes)

**Agents** : Francois (back-end) + Lise (front-end) en PARALLELE

**Execution par subagent frais** : Chaque tache est executee par un subagent frais pour eviter la pollution de contexte.

**Smoke test prioritaire** (avant le TDD tache par tache) :
Denis ecrit un **test d'integration end-to-end** du happy path principal AVANT de commencer les taches atomiques. Ce test DOIT echouer (il teste la fonctionnalite qui n'existe pas encore) mais doit **compiler**. Si le test ne compile pas structurellement, le plan a un defaut → corriger avant de continuer.

```rust
// Exemple smoke test pour MiyuVoice
#[test]
fn smoke_miyuvoice_capture_and_wakeword() {
    // Ce test echoue (fonctionnalite pas encore implementee)
    // mais il DOIT compiler — sinon le plan est structurellement faux
    let capture = AudioCapture::new(AudioConfig::default());
    assert!(capture.is_ok()); // RED: echoue → normal
}
```

**Pre-flight par tache** (avant d'ecrire du code) :
1. **Lire la tache** du plan exhaustif (fichier, code attendu, test)
2. **Context7 spot-check** (si la tache touche une API externe ou un pattern framework) :
   - Verifier le pattern contre la doc actuelle via `query-docs`
   - Ex: avant d'ecrire un composant Dioxus → verifier RSX syntax (`/dioxuslabs/dioxus`)
   - Ex: avant d'ecrire un handler axum → verifier extractors (`/tokio-rs/axum`)
3. **Charger le contexte anti-patterns** : relire les pieges RSX (Lise) ou patterns DB (Francois) depuis MEMORY.md

**Cycle TDD par tache** :
1. **RED** — Ecrire le test qui echoue
2. **GREEN** — Ecrire le code minimal pour que le test passe
3. **REFACTOR** — Nettoyer si necessaire
4. **VERIFY** — `cargo test -p {crate}` passe
5. **LINT** — `cargo clippy -p {crate} -- -D warnings` propre
6. **COMMIT** — Commit atomique avec message conventionnel
7. **PUSH** — `git push` sur la feature branch (sauvegarde distante)
8. **LOG** — `TodoWrite` : marquer la tache `completed`

**Checkpoint intermediaire** : Toutes les **5 taches completees**, Denis lance un mini-audit :
- `cargo build -p {crate}` des crates modifies
- `cargo clippy -p {crate} -- -D warnings`
- Verifier que les taches precedentes ne sont pas cassees par les nouvelles
- Si regression detectee → corriger avant de continuer
- `git push` — pousser l'etat courant sur la feature branch

**Annonce par etape macro** : A chaque etape du guide d'implementation completee, l'agent annonce dans le chat :
```
[YYYY-MM-DD HH:MM] ✓ Etape X/<total> — <nom de l'etape> terminee.
  Taches: X/Y completees | Tests: X passes | Commits: N
  → Prochaine etape: <nom>
```

**Comportement selon le mode d'autonomie** :
- **FULL** : L'annonce est informative, l'execution continue automatiquement.
- **BIG_STEPS** : L'annonce est informative en P3, la gate est entre P3→P4 (resume complet).
- **GUIDED** : Apres chaque annonce d'etape macro, **attendre la validation de l'utilisateur** (VALIDER / MODIFIER / REVENIR / SAUTER) avant de continuer.

**Parallelisme** : Francois et Lise travaillent simultanement quand leurs taches sont independantes. Les taches avec dependances sont sequencees par Denis.

**Auto-correction** : Si un test echoue, l'agent :
1. Lit le message d'erreur et identifie la cause (root cause analysis)
2. Verifie contre Context7 si c'est un probleme de pattern/API
3. Corrige et re-teste (tentative 1)
4. Si echec → corrige differemment (tentative 2)
5. Si echec → **frein d'urgence** avec diagnostic complet

**Quality Gate P3** : Chaque tache passe test + clippy.

**Gate BIG_STEPS (P3→P4)** : En mode `BIG_STEPS`, Denis presente un resume avant de passer a P4 :
```
[YYYY-MM-DD HH:MM] Resume P3 — Implementation terminee.
  Etapes: X/X | Taches: X/X | Tests: X passes, Y echoues | Commits: N
  Auto-corrections: N | Lignes ecrites: N
  → Continuer vers P4 (Integration & Audit) ?
  [CONTINUER] / [CORRIGER: <instructions>] / [STOPPER]
```

---

### P4 — Integration & Audit (T3+)

**Agents** : Denis + George

**Denis** — Integration :
1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo clippy --workspace -- -D warnings`
4. Verifier l'integration back + front
5. Si echec : corriger automatiquement, re-tester
6. Si echec apres 2 tentatives → **frein d'urgence**
7. **LOG** : `TodoWrite` pour chaque verification

**George** — Audit de conformite :
- [ ] Build workspace OK
- [ ] Tests workspace OK
- [ ] Clippy propre
- [ ] Pas de `unwrap()` en production (hors `#[cfg(test)]`)
- [ ] Pas d'URL hardcodees
- [ ] Pas de donnees sensibles en clair
- [ ] Annotations MSCM presentes sur les nouveaux fichiers
- [ ] Lois d'Autonomie respectees
- [ ] Parcours utilisateur coherent (si UI)

Artefact : `.mip/audits/YYYY-MM-DD-<slug>.md`

**Auto-correction** : Defauts NON-BLOQUANTS sont corriges automatiquement par Denis. Defauts CRITIQUES → **frein d'urgence**.

**Quality Gate P4** : George valide — 0 defaut BLOQUANT.

**Gate BIG_STEPS (P4→P5)** : En mode `BIG_STEPS`, George presente le resume d'audit avant de passer a P5 :
```
[YYYY-MM-DD HH:MM] Resume Audit P4 — Integration & Audit termines.
  Build: OK | Tests: X passes | Clippy: propre
  Defauts: N trouves (X corriges, Y acceptes, Z bloquants)
  Annotations MSCM: X/Y fichiers
  → Continuer vers P5 (Livraison & Test humain) ?
  [CONTINUER] / [CORRIGER: <instructions>] / [STOPPER]
```

---

### P5 — Livraison, Test humain & Validation (toutes classes)

**Agent** : Denis (livraison) + George (assistance test)

#### Etape 1 — Presentation du livrable

1. **Commit final** si necessaire (message conventionnel)
2. **Push final** — `git push` sur la feature branch
3. **Horodater** : `p5_start` dans le fichier metriques
4. **Presenter le resume a l'utilisateur** :
   - Ce qui a ete fait (fonctionnalites implementees)
   - Nombre de fichiers crees/modifies, lignes ecrites
   - Tests passes (unitaires, integration, globaux)
   - Anomalies detectees et corrigees
   - Instructions pour tester le livrable (commandes, parcours utilisateur)

#### Etape 2 — Test humain

5. **Horodater** : `p5_test_start`
6. **L'utilisateur teste le livrable** dans son environnement
7. George fournit une **checklist de test** adaptee au projet :
   - [ ] Build OK (`cargo build --workspace`)
   - [ ] Lancement de l'application OK
   - [ ] Parcours utilisateur principal fonctionne
   - [ ] Cas limites testes (si applicable)
   - [ ] Performance acceptable
   - [ ] UI conforme a la direction visuelle (si applicable)

#### Etape 3 — Questionnaire de satisfaction

8. **Horodater** : `p5_test_end`
9. Denis presente le **questionnaire de satisfaction** :

```
## Questionnaire de satisfaction — <titre du projet>

### Conformite fonctionnelle
1. Le livrable correspond-il a votre demande initiale ? (OUI / PARTIELLEMENT / NON)
2. Si non/partiellement, quels ecarts constatez-vous ?

### Qualite percue
3. Le code est-il propre et comprehensible ? (1-5)
4. L'UI est-elle satisfaisante ? (1-5, si applicable)
5. La performance est-elle acceptable ? (1-5)

### Satisfaction globale
6. Note globale de satisfaction (1-5) :
   1 = Inacceptable, 2 = Insuffisant, 3 = Acceptable, 4 = Bon, 5 = Excellent
7. Commentaires libres :

### Verdict
- [ ] ACCEPTE — Merger vers main
- [ ] ACCEPTE AVEC RESERVES — Merger, mais corrections mineures a planifier
- [ ] REFUSE — Retour en correction (boucle MIP)
```

#### Etape 4 — Decision

**Si ACCEPTE ou ACCEPTE AVEC RESERVES** :

10. **Horodater** : `p5_end`
11. **Merger les reserves** dans une liste de taches futures si applicable
12. **Merge vers main** — processus standard Git :
    ```bash
    git checkout main
    git pull origin main
    git merge feat/<slug> --no-ff
    git push origin main
    ```
13. **Tag si release** : `git tag -a vX.Y.Z -m "description"` + `git push origin vX.Y.Z`
14. **Nettoyage** : supprimer la branche de feature
    ```bash
    git branch -d feat/<slug>
    git push origin --delete feat/<slug>
    ```
15. **LOG** : `TodoWrite` marquer livraison `completed`
16. **Enregistrer** la satisfaction dans le fichier metriques

**Si REFUSE — Boucle MIP** :

10. **Logger l'intervention humaine** : type `constat_erreur` ou `delta`, impact `majeur`
11. **Incrementer** `mip_loops` dans le fichier metriques
12. **NE PAS merger** — la feature branch reste en l'etat
13. **Retour en P0** avec le contexte suivant :
    - Problemes constates par l'utilisateur (verbatim)
    - Ecarts entre l'attendu et le livre
    - Metriques de la boucle precedente
    - Maria reprend en **Temps 1** avec les problemes comme input
    - Le brief precedent sert de reference (pas de repartir de zero)
14. **Nouvelle sequence AUTOPILOT** sur la meme feature branch (pas de nouvelle branche)

**Alternative PR** : Remplacer le merge direct par `gh pr create`. L'utilisateur merge manuellement.

**Quality Gate P5** : Verdict utilisateur = ACCEPTE ou ACCEPTE AVEC RESERVES.

---

### P6 — Rapport final, Archivage & Capitalisation (T3+)

**Agent** : Arianne

#### Etape 1 — Rapport final de developpement

Arianne compile toutes les metriques collectees et produit le **rapport final independant du livrable**.

Artefact : `.mip/reports/YYYY-MM-DD-<slug>-report.md`

**Template du rapport final** :

```markdown
# Rapport MIP — <titre du projet>

## 1. Identite du projet
- **Titre** : <titre>
- **Description** : <description courte>
- **Type** : T3/T4/T5 — <description du type>
- **Complexite** : <evaluation qualitative : simple / moderee / complexe / tres complexe>
- **Branche** : feat/<slug>

## 2. Chrono & Duree
- **Debut** : <YYYY-MM-DD HH:MM> (debut P0)
- **Fin** : <YYYY-MM-DD HH:MM> (fin P6)
- **Duree totale IRL** : <Xh Ymin>
- **Decomposition** :
  | Phase | Debut | Fin | Duree |
  |-------|-------|-----|-------|
  | P0 Cadrage | ... | ... | ... |
  | P3 Implementation | ... | ... | ... |
  | P4 Integration & Audit | ... | ... | ... |
  | P5 Livraison & Test | ... | ... | ... |
  | P5 Test humain | ... | ... | ... |
  | P6 Rapport & Archivage | ... | ... | ... |

## 3. Ressources
- **Modele LLM** : <nom du modele> (ex: Claude Opus 4.6)
- **Tokens utilises** : ~<estimation> (entree: X, sortie: Y)
- **Nombre de boucles MIP** : <N> (1 = pas de retour)

## 4. Production
- **Lignes ecrites** : <N>
- **Lignes supprimees** : <N>
- **Fichiers crees** : <N>
- **Fichiers modifies** : <N>
- **Crates touches** : <N> (<liste>)
- **Crates crees** : <N> (<liste>)
- **Commits** : <N>

## 5. Equipe
- **Agents engages** : <N> (<liste avec roles>)
  | Agent | Role | Phases | Taches |
  |-------|------|--------|--------|
  | Maria | Chef de Projet | P0 | ... |
  | ... | ... | ... | ... |

## 6. Interactions humaines
- **Interventions humaines** : <N>
  | # | Timestamp | Type | Phase | Description | Impact |
  |---|-----------|------|-------|-------------|--------|
  | 1 | ... | precision | P0 | ... | mineur |
  | ... | ... | ... | ... | ... | ... |

- **Questions agents → humain** : <N>
  | # | Timestamp | Agent | Phase | Nature | Question |
  |---|-----------|-------|-------|--------|----------|
  | 1 | ... | Maria | P0 | clarification | ... |
  | ... | ... | ... | ... | ... | ... |

## 7. Tests
### Tests unitaires
- **Total** : <N>
- **Erreurs** : <N> (<N> corrigees, <N> restantes)

### Tests d'integration
- **Total** : <N>
- **Erreurs** : <N>

### Tests globaux
- **Total** : <N>
- **Erreurs** : <N>

### Auto-corrections
- **Nombre d'erreurs auto-corrigees** : <N>
- **Freins d'urgence declenches** : <N>

## 8. Audits
- **Nombre d'audits** : <N>
  | # | Type | Defauts | Gravite | Nature | Resolution |
  |---|------|---------|---------|--------|------------|
  | 1 | conformite | ... | bloquant/non-bloquant | ... | corrige/accepte |
  | ... | ... | ... | ... | ... | ... |

## 9. Satisfaction utilisateur
- **Verdict** : ACCEPTE / ACCEPTE AVEC RESERVES / REFUSE (boucle N)
- **Note satisfaction** : <1-5>
- **Commentaires** : <verbatim>

## 10. Notation globale

| Critere | Note /20 | Commentaire |
|---------|----------|-------------|
| **Note globale** | /20 | Moyenne ponderee des notes ci-dessous |
| Vitesse de dev (vs historique MIP) | /20 | Comparaison avec les sequences precedentes |
| Qualite des interventions agents | /20 | Pertinence, precision, autonomie |
| Qualite du code | /20 | Lisibilite, patterns, clippy, tests |
| Qualite de gestion des erreurs | /20 | Detection, correction, prevention |
| Qualite des interactions utilisateur | /20 | Clarte, pertinence des questions, ecoute |
| Respect du protocole MIP | /20 | Gates, artefacts, logging, TDD |
| Qualite de l'indexation MSCM | /20 | Couverture, precision des annotations |

**Bareme** :
- 18-20 : Excellent — reference pour les futures sequences
- 14-17 : Bon — quelques axes d'amelioration
- 10-13 : Acceptable — ameliorations significatives necessaires
- 6-9 : Insuffisant — problemes majeurs a resoudre
- 0-5 : Inacceptable — remise en question du processus

**Methode de notation** : Arianne evalue sur base des metriques objectives (tests, erreurs, timings) et du feedback utilisateur. La note est comparee a l'historique stocke dans `memory/mip-performance-history.md`.

## 11. Resume du developpement
<Resume narratif : ce qui a ete fait, les difficultes rencontrees, les decisions prises, les points forts et faibles de la sequence>

## 12. Profil utilisateur — Apprentissages
- **Competences techniques observees** : <ce que l'utilisateur connait/maitrise>
- **Connaissances domaine** : <expertise metier observee>
- **Preferences de travail** : <style de communication, niveau de detail souhaite, degre d'autonomie attendu>
- **Points d'attention** : <sujets sensibles, exigences recurrentes>

## 13. Capitalisation agents
- **Patterns confirmes** : <nouveaux patterns a ajouter a mip-decisions.md>
- **Anti-patterns decouverts** : <erreurs a ajouter a mip-antipatterns.md>
- **Configurations agents** : <ajustements recommandes pour les agents>
- **Ameliorations protocole** : <suggestions d'evolution du MIP>
```

#### Etape 2 — Archivage des artefacts

1. Archiver les artefacts MIP (brief, spec, plan, audit, rapport) dans `.mip/`
2. Verifier que tous les artefacts sont complets et coherents

#### Etape 3 — Capitalisation

3. Extraire les apprentissages :
   - Patterns confirmes → `memory/mip-decisions.md`
   - Erreurs a eviter → `memory/mip-antipatterns.md`
   - Lecons par chantier → `memory/mip-lessons.md`
   - Competences par agent → `memory/team-skills-audit.md`
4. **Enregistrer les notes** dans `memory/mip-performance-history.md` pour comparaison future
5. **Enregistrer le profil utilisateur** dans `memory/user-profile.md` (cumulatif)
6. **Enregistrer les configurations agents** dans `memory/agent-tuning.md`
7. Mettre a jour `memory/MEMORY.md` (index, max 200 lignes)
8. **Horodater** : `p6_end`, `total_end`
9. **LOG** : `TodoWrite` marquer archivage `completed`

---

## Regles NON NEGOCIABLES

1. **Classification avant action** — Aucun code sans classification T1-T5
2. **Spec avant code** (T3+) — Pas d'implementation sans spec Francois (Temps 5)
3. **Plan exhaustif avant execution** (T3+) — Pas d'implementation sans plan Denis (Temps 6)
4. **Verification Context7 obligatoire** (T3+) — Verifier les docs des libs impliquees avant de coder
5. **Anti-patterns charges** — Lire `memory/mip-antipatterns.md` et MEMORY.md avant chaque sprint
6. **TDD obligatoire** — RED-GREEN-REFACTOR, pas d'exception
7. **Subagent frais par tache** — Eviter la pollution de contexte
8. **Checkpoint toutes les 5 taches** — Mini-audit intermediaire en P3
9. **Gates non-bypassables** — Chaque gate doit etre explicitement validee
10. **Artefacts obligatoires** — Chaque phase produit son artefact dans `.mip/`
11. **Lint propre** — Linter du projet (ex: `cargo clippy -- -D warnings`, `eslint .`, `ruff check .`) apres chaque tache
12. **Pas de code dangereux en prod** — Pas de `unwrap()` (Rust), pas de `any` (TS), pas de `bare except` (Python) en production
13. **Archivage systematique** (T3+) — Arianne capitalise apres chaque livraison
14. **Logging obligatoire** — Chaque tache tracee via TodoWrite
15. **Autopilot apres P0** — Aucune intervention humaine sauf frein d'urgence
16. **Feature branch obligatoire** (T2+) — Tout travail sur branche, merge vers main apres validation
17. **Push regulier** — Chaque commit est pousse sur le remote pour sauvegarde
18. **Metriques obligatoires** — Horodatage et compteurs collectes tout au long de la sequence
19. **Test humain en P5** — L'utilisateur teste le livrable avant merge
20. **Questionnaire satisfaction** — Feedback structure avant decision de merge
21. **Boucle MIP si refus** — Retour en P0 avec les problemes constates, pas de merge
22. **Rapport final en P6** — Rapport complet independant du livrable, notes /20, capitalisation
23. **Audit faisabilite en P0** (T3+) — Arianne verifie agents, dependances, outils et memoire avant synthese
24. **Questionnaire brainstorming en P0** (T3+) — Maria administre le questionnaire standard (5 sections) en Temps 1 pour cadrer le projet
25. **Inventaire des prerequis en P0** (T3+) — Denis inventorie competences, connaissances, outils et etapes avant la spec et le plan
26. **Annonces temps reel** — Chaque Temps P0 et chaque etape macro P3 sont annonces dans le chat avec date/heure a la completion
27. **Mode d'autonomie** — L'utilisateur choisit FULL/BIG_STEPS/GUIDED en P0. Persistance dans `memory/user-profile.md`. Changeable via `/autonomy_mode`
28. **Smoke test prioritaire** — Un test e2e happy path est ecrit AVANT le TDD tache par tache en P3 (doit compiler, peut echouer)
29. **TL;DR obligatoire** — Chaque artefact MIP (brief, spec, plan, audit) commence par un resume de 5 lignes max
30. **Phase SETUP obligatoire** — Tout nouvel environnement MIP doit passer par la Phase SETUP avant le premier P0. Produit `.mip/environment.md`
31. **Environment.md referentiel** — Les commandes build/test/lint et conventions sont lues depuis `.mip/environment.md`, pas hardcodees dans les agents
32. **Adaptation automatique** — MIP adapte ses capacites (parallelisme, TodoWrite, Context7) a l'outil IA detecte en SETUP-4

---

## Token Efficiency — Connaissances pre-indexees

Pour maximiser l'efficacite de chaque token, les agents chargent des **fichiers memoire pre-compiles** au lieu de rechercher les informations a chaque session.

### Fichiers memoire universels (generes par Phase SETUP)

| Fichier | Contenu | Agents consommateurs |
|---------|---------|---------------------|
| `memory/stack-patterns.md` | Patterns specifiques a la stack du projet (adapte au langage/framework detecte) | Dev Back, Chef Dev |
| `memory/stack-cheatsheet.md` | Cheatsheet du framework principal (pitfalls, patterns, templates) | Dev Front |
| `memory/project-file-map.md` | Carte des fichiers cles du projet (50-80 entrees, 1 ligne chacune) | Tous |
| `memory/api-contracts.md` | Types et interfaces partages inter-modules (signatures exactes) | Dev Back, Dev Front |
| `memory/test-templates.md` | Templates de tests standard (unit, integration, e2e) adaptes au framework de test | Dev Back, Dev Front |
| `memory/code-annotations-templates.md` | Templates d'annotations de code par type de fichier (MSCM, JSDoc, Docstrings, etc.) | Tous |
| `memory/context7-cache.md` | Resultats des queries Context7 les plus frequentes (evite les re-queries) | Dev Back, Dev Front |

**Note** : Dans le contexte Miyukini COG, ces fichiers sont nommes plus specifiquement : `rust-patterns.md`, `dioxus-cheatsheet.md`, `mscm-templates.md`. Les noms universels ci-dessus sont utilises dans un nouvel environnement.

### Protocole de chargement par agent

Chaque agent charge **uniquement ses fichiers pertinents** en debut de tache :

| Agent (role) | Fichiers a charger |
|-------|-------------------|
| **Dev Back-End** (Francois) | `stack-patterns.md`, `api-contracts.md`, `test-templates.md`, `code-annotations-templates.md` |
| **Dev Front-End** (Lise) | `stack-cheatsheet.md`, `api-contracts.md`, `project-file-map.md`, `code-annotations-templates.md` |
| **Chef Dev** (Denis) | `project-file-map.md`, `stack-patterns.md`, `mip-decisions.md`, `mip-antipatterns.md` |
| **Audit Expert** (George) | `project-file-map.md`, `code-annotations-templates.md`, `mip-antipatterns.md` |
| **Team Manager** (Arianne) | `mip-decisions.md`, `mip-antipatterns.md`, `mip-performance-history.md`, `team-skills-audit.md` |

### TL;DR obligatoire sur chaque artefact

Chaque artefact MIP (brief, spec, plan, audit, rapport) **DOIT** commencer par un TL;DR de 5 lignes max. Les agents qui n'ont besoin que du contexte global lisent le TL;DR sans charger le document complet. Economie : ~300-500 tokens par artefact non-lu en detail.

---

## Registre Context7 — Libs a verifier

> Ce registre est **projet-specifique**. Les IDs sont pre-resolus en SETUP-5 ou lors du premier P0. Les libs sont ajoutees au fur et a mesure des sprints.

### Miyukini COG — IDs pre-resolus

| Lib | Context7 ID | Quand verifier |
|-----|-------------|----------------|
| **Dioxus 0.6** | `/dioxuslabs/dioxus/v0.6.3` | Tout composant UI, RSX, signals, hooks |
| **Dioxus docs** | `/llmstxt/dioxuslabs_learn_0_6_llms-full_txt` | Patterns avances, migration, pitfalls |
| **axum** | `/tokio-rs/axum/axum_v0_7_9` | Tout endpoint REST, middleware, extractors |
| **serde** | `/serde-rs/serde` | Serialization custom, derive macros, attributes |
| **Dioxus Components** | `/dioxuslabs/components` | Composants primitifs ARIA |

### Protocole universel Context7

**Quand verifier** :
- **Toujours** en P0 Temps 5 (spec) pour chaque lib impliquee
- **Spot-check** en P3 si la tache touche un pattern specifique
- **En cas d'erreur** : verifier si le pattern utilise est encore valide
- **Premier P0 d'un nouveau projet** : resoudre les IDs de toutes les libs de la stack via `resolve-library-id`

**Procedure pour une nouvelle stack** :
1. Lister les frameworks/libs du projet (depuis `.mip/environment.md`)
2. Pour chaque lib : `resolve-library-id` → noter l'ID dans ce registre
3. Stocker dans `memory/context7-cache.md` les resultats des queries les plus frequentes

---

## Integration SuperClaude

Ce protocole s'appuie sur les skills SuperClaude quand ils sont disponibles :

| Phase MIP | Skill SuperClaude | Usage |
|-----------|-------------------|-------|
| P0 (Temps 1-2) | `brainstorming` | Maria structure le brief (8 temps : exploration → ideation → analyse → inventaire → spec → plan → audit faisabilite → synthese) |
| P0 (Temps 7) | `verification-before-completion` | Arianne verifie conformite agents, deps, outils, memoire |
| P0 (Temps 6) | `writing-plans` | Denis cree les taches atomiques exhaustives + guide d'implementation |
| P3 | `subagent-driven-development` | Execution par subagent frais |
| P3 | `test-driven-development` | Cycle RED-GREEN-REFACTOR |
| P3 | `systematic-debugging` | Root cause avant tout fix + auto-correction |
| P4 | `verification-before-completion` | George verifie |
| P5 | `finishing-a-development-branch` | Denis finalise + test humain + questionnaire |
| P6 | — | Arianne : rapport final + capitalisation + profil utilisateur |

---

## Raccourcis pour taches simples

**T1 (micro-fix)** : Pas besoin de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis ecrit un mini-plan (1-3 taches), execution directe.

Le protocole est **proportionnel** : les petites taches ne sont pas alourdies.

---

## Flux concret — Exemple T4

```
Utilisateur : "Je veux ajouter MiyuVoice"
  |
  +-- Maria (P0 Temps 1) : Classifie T4, explore code, questionnaire brainstorming (20 questions)
  |   [GATE] Attendre reponses utilisateur
  |   → [2026-03-02 14:05] ✓ P0 Temps 1 termine
  |
  +-- PARALLELE (Temps 2 + 3) :
  |   +-- Maria : Cadrage fonctionnel, 2-3 approches
  |   +-- Lise : Direction visuelle, parcours UX, composants
  |   +-- Fabrice : Analyse concurrence (Alexa, Siri, etc.)
  |   → [2026-03-02 14:20] ✓ P0 Temps 2+3 termines
  |
  +-- Denis (Temps 4) : Inventaire des prerequis
  |   +-- Competences : Rust audio (Francois), Dioxus signals (Lise), archi embarquee (Denis)
  |   +-- Connaissances : VOSK API, wake word detection, streaming audio
  |   +-- Outils : crates cpal/rodio, modele VOSK, assets audio
  |   +-- Etapes macro : 1-Capture audio, 2-Wakeword, 3-API voix, 4-UI composant
  |   +-- Matrice : 2 crates externes a verifier, 1 asset a creer
  |   → [2026-03-02 14:30] ✓ P0 Temps 4 termine (4 etapes, 12 prerequis, 1 manquant)
  |
  +-- Francois (Temps 5) : Spec technique + VERIFICATION CONTEXT7
  |   +-- Context7 : Dioxus 0.6 RSX, axum handlers, serde patterns
  |   +-- Anti-patterns : charge MEMORY.md + mip-antipatterns.md
  |   +-- Output : spec + section "Verification documentaire"
  |   → [2026-03-02 14:40] ✓ P0 Temps 5 termine
  |
  +-- Denis (Temps 6) : Plan exhaustif + Guide d'implementation detaille
  |   +-- 42 taches : 18 CODE, 12 TEST-U, 4 TEST-I, 3 TEST-G, 3 AUDIT, 2 CORRECT
  |   +-- Guide : 4 etapes avec prerequis, taches atomiques, criteres completion
  |   → [2026-03-02 14:55] ✓ P0 Temps 6 termine
  |
  +-- Arianne (Temps 7) : Audit de faisabilite
  |   +-- Verification : agents, deps, outils, memoire, Context7 spot-check
  |   +-- Diagnostic : CONFORME / TROUS MINEURS / PREREQUIS
  |   +-- Si prerequis → suggere mini-projet precurseur
  |   → [2026-03-02 15:05] ✓ P0 Temps 7 termine (CONFORME)
  |
  +-- Maria (Temps 8) : Synthese → Brief complet (inclut inventaire + audit + TL;DR)
  |   [GATE] Utilisateur approuve le brief + choisit approche + choisit mode autonomie
  |   +-- "Mode d'autonomie : FULL / BIG_STEPS / GUIDED ?"
  |   +-- "Garder pour les futures sequences ? OUI / NON / JE SAIS PAS"
  |   → [2026-03-02 15:15] ✓ P0 Temps 8 termine — Brief approuve, mode BIG_STEPS
  |
  +=== EXECUTION START (mode: BIG_STEPS, metriques initialisees) ===
  |
  +-- Git : git checkout -b feat/miyuvoice + git push -u origin
  |
  +-- Denis : Smoke test e2e (compile mais echoue → structure validee)
  |
  +-- P3 PARALLELE :
  |   +-- Chargement memoire : rust-patterns.md (Francois), dioxus-cheatsheet.md (Lise)
  |   +-- Pre-flight : Context7 spot-check + anti-patterns par tache
  |   +-- Francois : Taches CODE back-end (TDD) → commit → push → metriques → TodoWrite
  |   +-- Lise : Taches CODE front-end (TDD) → commit → push → metriques → TodoWrite
  |   +-- [Checkpoint toutes les 5 taches : mini-audit Denis + push]
  |   +-- [Auto-correction intelligente : root cause + Context7 + 2 tentatives]
  |   +-- [Annonce par etape macro : "[HH:MM] ✓ Etape 1/4 — Capture audio terminee"]
  |   +-- [BIG_STEPS: Gate P3→P4 — Resume P3 a l'utilisateur → CONTINUER/CORRIGER/STOPPER]
  |
  +-- P4 :
  |   +-- Denis : Integration workspace (build/test/clippy)
  |   +-- George : Audit conformite → .mip/audits/ + metriques audit
  |   [Auto-correction defauts non-bloquants, frein d'urgence si critique]
  |   +-- [BIG_STEPS: Gate P4→P5 — Resume audit a l'utilisateur → CONTINUER/CORRIGER/STOPPER]
  |
  +-- P5 :
  |   +-- Denis : Push final + resume a l'utilisateur + instructions test
  |   +-- [Utilisateur teste le livrable]
  |   +-- Denis : Questionnaire satisfaction
  |   +-- [GATE] Verdict utilisateur :
  |       +-- ACCEPTE → merge main + push + tag + nettoyage branche
  |       +-- RESERVES → merge main + ajout taches futures
  |       +-- REFUSE → log intervention + increment boucle → retour P0
  |
  +-- P6 : Arianne
  |   +-- Rapport final (notes /20, metriques, profil utilisateur)
  |   +-- → .mip/reports/ + memory/mip-performance-history.md
  |   +-- Capitalisation : anti-patterns, decisions, agent-tuning
  |   +-- Profil utilisateur → memory/user-profile.md (+ mode autonomie prefere)
  |
  +=== EXECUTION END ==========================================
```

---

## Reference — Outils IA compatibles MIP

> Liste des outils IA de dev valides pour executer MIP v2 (mars 2026). L'outil est detecte en SETUP-4.

### Categorie 1 — CLI agentiques (execution complete MIP)

| Outil | Editeur | Agents // | Terminal | MCP | Open Source | Execution |
|-------|---------|-----------|----------|-----|-------------|-----------|
| **Claude Code** | Anthropic | Natif | Natif | Client+Srv | Non | Cloud |
| **OpenAI Codex CLI** | OpenAI | Natif | Natif | Client+Srv | Non | Cloud |
| **Aider** | OSS (Paul Gauthier) | Non | Natif | Non | Oui (Apache 2) | Cloud/Local |
| **OpenCode** | OSS | Partiel | Natif | Partiel | Oui | Cloud/Local |
| **Goose** | Block (ex-Square) | Non | Natif | Partiel | Oui | Cloud/Local |
| **Gemini CLI** | Google | Non | Natif | Oui | Non | Cloud |

### Categorie 2 — IDEs natifs IA

| Outil | Editeur | Agents // | Terminal | MCP | Open Source | Execution |
|-------|---------|-----------|----------|-----|-------------|-----------|
| **Cursor** | Anysphere | Via Agent | Natif | Client | Non | Cloud |
| **Windsurf** | Cognition AI | Via Cascade | Natif | Non | Non | Cloud |
| **Zed** | Zed Industries | Via Agent Panel | Natif | Client | Oui (GPL) | Cloud/Local |

### Categorie 3 — Extensions IDE

| Outil | Editeur | Agents // | Terminal | MCP | Open Source | Execution |
|-------|---------|-----------|----------|-----|-------------|-----------|
| **GitHub Copilot** | Microsoft | Agent Mode | Oui | Client | Non | Cloud |
| **Gemini Code Assist** | Google | Agent Mode | Oui | Client+Srv | Non | Cloud |
| **Amazon Q Developer** | AWS | Agent `/dev` | CLI agent | Partiel | Non | Cloud |
| **JetBrains AI** | JetBrains | Partiel | Limite | Server | Non | Cloud/Local |
| **Continue.dev** | OSS | Partiel | Non | Client | Oui (Apache 2) | Cloud/Local |
| **Cline** | OSS | Complet | Oui | Client | Oui | Cloud/Local |
| **Cody** | Sourcegraph | Partiel | Non | Non | Oui | Cloud/Self-hosted |
| **Tabnine** | Tabnine | Non | Non | Non | Non | Cloud/Local |
| **Augment Code** | Augment | Partiel | Non | Non | Non | Cloud |

### Categorie 4 — Runtimes inference locale

| Outil | Type | API compatible | MCP | Usage MIP |
|-------|------|---------------|-----|-----------|
| **LM Studio** | GUI + API server | OpenAI-compat `localhost:1234` | Server natif | Backend pour Continue.dev, JetBrains, miou-llm-bridge |
| **Ollama** | CLI + API server | OpenAI-compat | Via wrappers | Backend headless pour scripts et agents |
| **llama.cpp** | CLI | OpenAI-compat (server mode) | Non | Performance maximale, usage avance |
| **vLLM** | Server Python | OpenAI-compat | Non | Haute performance GPU, batching |
| **Jan** | GUI desktop | OpenAI-compat | Non | Alternative LM Studio plus legere |
| **LocalAI** | Container API | OpenAI-compat | Non | Docker-first, multi-modele |

### Categorie 5 — Plateformes cloud autonomes (usage limite MIP)

| Outil | Usage MIP | Limitation |
|-------|-----------|------------|
| **Devin 2.0** | Taches isolees T1-T2 | Pas d'acces aux repos locaux |
| **Replit Agent 3** | Prototypage rapide | JS/TS seulement, pas de monorepo complexe |
| **Bolt.new** | Prototypage front | JS/TS seulement, WebContainers |
| **v0** | Generation UI React | Front-end React seulement |

### Recommandations par scenario

| Scenario | Outil recommande | Raison |
|----------|-----------------|--------|
| **MIP complet (T3-T5)** | Claude Code / Codex CLI | Agents paralleles natifs, terminal, MCP, TodoWrite |
| **Edition quotidienne + MIP leger** | Cursor + Claude Code | Navigation rapide + heavy-lift |
| **Budget gratuit / open-source** | Aider + Ollama | Git-first, local, pas de frais |
| **Confidentialite maximale** | Continue.dev + LM Studio | Tout local, zero fuite de donnees |
| **Enterprise multi-IDE** | GitHub Copilot (extension) | Compatible VS Code, JetBrains, CLI |
| **AWS-native** | Amazon Q Developer | Integration IAM, CDK, CloudFormation |
| **Gros monorepo (50k+ fichiers)** | Augment Code ou Cody | Indexation semantique avancee |
