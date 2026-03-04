# Module MIP — Phase SETUP (Onboarding Universel)

> Ce module est charge UNIQUEMENT quand `.mip/environment.md` n'existe pas ou quand l'utilisateur lance `/mip_setup`.

---

## Declenchement

La Phase SETUP est declenchee automatiquement si :
- Le fichier `.mip/environment.md` n'existe pas
- L'utilisateur lance `/mip_setup`
- L'utilisateur demande une reconfiguration de l'environnement

---

## SETUP-1 — Detection automatique de l'environnement

> Aucune interaction utilisateur requise. L'agent scanne le systeme.

**Scan systeme** (via Bash) :

```bash
uname -a || systeminfo                    # OS et architecture
echo $SHELL || echo $0                    # Shell courant
nproc                                      # CPU cores
free -h || systeminfo                     # RAM
df -h . || wmic logicaldisk               # Disque
nvidia-smi || rocm-smi                    # GPU (si present)
ping -c 1 github.com                      # Acces internet
git --version && git config user.name && git config user.email
git remote -v                              # Remotes
docker version || podman version          # Container runtime
```

**Informations collectees** : OS, shell, CPU, RAM, GPU (VRAM), disque, reseau, git, container runtime.

---

## SETUP-2 — Configuration de l'environnement (questionnaire interactif)

> Maria administre. 19 questions en 4 sections.

### 2.1 — Stack technique (S2.1-S2.7)

| # | Question |
|---|----------|
| S2.1 | Quel(s) langage(s) de programmation ? |
| S2.2 | Quel(s) framework(s) ou librairies principales ? |
| S2.3 | Quelle(s) base(s) de donnees ? |
| S2.4 | Quel style d'API ? (REST, GraphQL, gRPC, WebSocket, aucun) |
| S2.5 | Quel gestionnaire de paquets / build tool ? |
| S2.6 | Monorepo ou multi-repo ? |
| S2.7 | Conventions de code existantes ? (linter, formatteur, styleguide) |

### 2.2 — Securite & conformite (S2.8-S2.11)

| # | Question |
|---|----------|
| S2.8 | Niveau de securite requis ? (standard, renforce, critique) |
| S2.9 | Conformite reglementaire ? (RGPD, HIPAA, SOC2, PCI-DSS, aucune) |
| S2.10 | Chiffrement des donnees ? (at-rest, in-transit, E2E, aucun) |
| S2.11 | Gestion des secrets ? (env vars, vault, .env, aucune) |

### 2.3 — Infrastructure & deploiement (S2.12-S2.16)

| # | Question |
|---|----------|
| S2.12 | Hebergement code ? (GitHub, GitLab, Bitbucket, auto-heberge) |
| S2.13 | CI/CD en place ? (GitHub Actions, GitLab CI, Jenkins, aucun) |
| S2.14 | Conteneurisation ? (Docker, Podman, Kubernetes, aucun) |
| S2.15 | Hebergement / deploiement ? (VPS, cloud, PaaS, self-hosted, local) |
| S2.16 | Authentification ? (OAuth2, JWT, sessions, passkeys, aucun) |

### 2.4 — Dossier de travail (S2.17-S2.19)

| # | Question |
|---|----------|
| S2.17 | Dossier racine du projet ? (auto-detecte si possible) |
| S2.18 | Structure de dossiers imposee ? |
| S2.19 | CLAUDE.md / AGENTS.md / rules existants ? |

---

## SETUP-3 — Profil utilisateur (questionnaire interactif)

> 8 questions pour adapter communication et niveau de detail.

| # | Question |
|---|----------|
| S3.1 | Role ? (solo dev, tech lead, CTO, etudiant, PM, autre) |
| S3.2 | Niveau d'experience ? (debutant <1an, junior 1-3, intermediaire 3-5, senior 5-10, expert 10+) |
| S3.3 | Langages maitrises ? |
| S3.4 | Domaines de connaissance ? (web, mobile, systeme, data, IA, securite, devops, game dev) |
| S3.5 | Style de travail ? (autonome, collaboratif, superviseur) |
| S3.6 | Niveau de detail attendu ? (concis, normal, detaille) |
| S3.7 | Objectif principal ? (livrer, apprendre, prototyper, maintenir, refactorer) |
| S3.8 | Regles NON NEGOCIABLES ? (liste libre) |

**Mapping automatique style → mode autonomie** : Autonome → FULL | Collaboratif → BIG_STEPS | Superviseur → GUIDED

---

## SETUP-4 — Detection du support IA

### Auto-detection

| Indice | Outil detecte |
|--------|---------------|
| Variable `CLAUDE_CODE` ou contexte `claude-code` | Claude Code CLI |
| Variable `CURSOR_*` ou `.cursor/` present | Cursor IDE |
| Variable `VSCODE_*` sans `CURSOR_*` | VS Code |
| Variable `CODEX_*` ou `openai-codex` | OpenAI Codex CLI |
| Contexte MCP server actif | Outil MCP-compatible |
| Aucun indice | Demander a l'utilisateur |

### Questionnaire complementaire (S4.1-S4.4)

| # | Question |
|---|----------|
| S4.1 | Outil IA principal ? (auto-detecte : `{outil}`) |
| S4.2 | Inference locale ? (LM Studio, Ollama, llama.cpp, vLLM, Jan, LocalAI, aucun) |
| S4.3 | Modele(s) IA prefere(s) ? |
| S4.4 | Budget IA mensuel ? |

### Matrice de compatibilite MIP

| Capacite MIP | Claude Code | Cursor | VS Code + ext | Codex CLI | Aider | Inference locale |
|--------------|-------------|--------|---------------|-----------|-------|-----------------|
| Agents paralleles | Natif | Via Agent | Via extension | Natif | Non | Non |
| Terminal access | Natif | Natif | Via terminal | Natif | Natif | Via shell |
| Multi-file edit | Natif | Natif | Selon ext | Natif | Natif | Selon outil |
| MCP support | Client+Srv | Client | Selon ext | Client+Srv | Non | Selon runtime |
| TodoWrite | Natif | Non | Non | Non | Non | Non |
| Background tasks | Natif | Limite | Non | Natif | Non | Non |
| Context7 docs | Via MCP | Via MCP | Via MCP | Via MCP | Non | Non |

**Adaptations si pas supporte** : Agents // → sequentiel | TodoWrite → annonces texte | MCP → memoire locale | Background → foreground | Terminal → utilisateur lance manuellement.

---

## SETUP-5 — Detection et installation des dependances

### Invariants universels

| Outil | Detection | Installation |
|-------|-----------|-------------|
| git | `git --version` | `winget install Git.Git` / `brew install git` / `apt install git` |
| bash/zsh | `echo $SHELL` | Pre-installe |

### Detection par stack (selon SETUP-2)

**Rust** : `rustc`, `cargo`, `rustup show`, `cargo clippy`, `cargo fmt`
**JavaScript/TypeScript** : `node`, `npm/yarn/pnpm/bun`, `tsc`, `eslint`, `prettier`
**Python** : `python3`, `pip/poetry/uv`, `pytest`, `mypy`, `ruff`
**Go** : `go version`, `golangci-lint`
**Java/Kotlin** : `java`, `mvn/gradle`
**C/C++** : `gcc/clang`, `cmake`

### Outils transversaux

Docker, docker-compose, kubectl, gh (GitHub CLI), curl, jq, openssl.

L'agent propose l'installation des manquants (commande adaptee a l'OS). L'utilisateur valide chaque installation.

---

## SETUP-6 — Inventaire des capacites et installation des agents

### Agents MIP universels (9 roles)

> Table complete dans CLAUDE.md, section "Equipe dev".

Les 9 roles fonctionnels sont independants de la stack : Chef de Projet, Analyste Produit, Chef Dev, Dev Back-End, Dev Front-End, Team Manager, Audit Expert, Expert Cybersecurite, DevOps & Infra.

### Adaptation a la stack

Les agents sont configures avec les commandes build/test/lint/format, la structure des packages, les conventions de commit, et les patterns de test adaptes au langage/framework detecte.

### Fichiers generes par SETUP

| Fichier | Contenu |
|---------|---------|
| `.mip/environment.md` | Configuration maitre : OS, hardware, stack, securite, infra, outil IA, dependances |
| `memory/user-profile.md` | Profil utilisateur : role, experience, preferences, mode autonomie |
| `memory/project-file-map.md` | Carte des fichiers cles du projet |
| `memory/stack-patterns.md` | Patterns specifiques a la stack |
| `memory/stack-cheatsheet.md` | Cheatsheet du framework principal |
| `CLAUDE.md` | Conventions projet (cree ou augmente si existant) |
| `.claude/agents/*.md` | Agents adaptes a la stack (si Claude Code) |

---

## Template `.mip/environment.md`

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
- API style: <type>
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
- Conformite: <reglementation>
- Chiffrement: <type>
- Secrets: <methode>

## Infrastructure
- Hebergement code: <plateforme>
- CI/CD: <outil>
- Conteneurisation: <outil>
- Deploiement: <type>
- Auth: <methode>

## Outil IA
- Outil principal: <nom>
- Inference locale: <outil>
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

## Conventions du projet
- Convention commit: <type>
- Annotations code: <type>
- Pattern erreurs: <type>
- Regles NON NEGOCIABLES utilisateur: <liste depuis S3.8>
```

---

## Invariants universels MIP

| Invariant | Universel car... |
|-----------|------------------|
| Classification T1-T5 | Toute tache a une taille |
| Git (VCS) | Standard industrie universel |
| Tests avant livraison | Fondamental, tout langage |
| Linting/formatting | Chaque langage a ses outils |
| Code review / audit | Bonne pratique universelle |
| Chiffrement | Obligation legale et technique |
| Gestion des secrets | OWASP Top 10, universel |
| CI/CD | Standard industrie |
| Documentation | Maintenance long terme |
| Metriques & feedback | Kaizen / amelioration continue |
