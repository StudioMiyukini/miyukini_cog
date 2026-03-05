# Module MIP â€” Phase SETUP (onboarding universel)

> Ce module est chargÃ© UNIQUEMENT quand `.mip/environment.md` n'existe pas ou quand l'utilisateur exÃ©cute `/mip_setup`.

---

## DÃ©clencheur

La phase SETUP est dÃ©clenchÃ©e automatiquement si :
- Le fichier `.mip/environment.md` n'existe pas
- L'utilisateur exÃ©cute `/mip_setup`
- L'utilisateur demande une reconfiguration de l'environnement

---

## SETUP-1 â€” DÃ©tection automatique de l'environnement

> Aucune interaction utilisateur requise. L'agent scanne le systÃ¨me.

**Scan systÃ¨me** (via terminal, commandes adaptÃ©es Ã  l'OS) :

| Cible | Linux / macOS | Windows PowerShell |
|---|---|---|
| OS et architecture | `uname -a` | `systeminfo` |
| Shell | `echo $SHELL` | `$PSVersionTable.PSVersion` + `$env:ComSpec` |
| CPU (threads logiques) | `nproc` | `(Get-CimInstance Win32_Processor).NumberOfLogicalProcessors` |
| RAM | `free -h` | `Get-CimInstance Win32_OperatingSystem | Select-Object TotalVisibleMemorySize,FreePhysicalMemory` |
| Disque | `df -h .` | `Get-PSDrive -PSProvider FileSystem` |
| GPU | `nvidia-smi \|\| rocm-smi` | `nvidia-smi` |
| AccÃ¨s Internet | `ping -c 1 github.com` | `ping -n 1 github.com` |
| Git | `git --version && git config user.name && git config user.email && git remote -v` | `git --version; git config user.name; git config user.email; git remote -v` |
| Runtime conteneur | `docker version \|\| podman version` | `docker version; podman version` |

**Informations collectÃ©es** : OS, shell, CPU, RAM, GPU (VRAM), disque, rÃ©seau, git, runtime conteneur.

---

## SETUP-2 â€” Configuration environnement (questionnaire interactif)

> Maria administre. 19 questions en 4 sections.

### 2.1 â€” Stack technique (S2.1-S2.7)

| # | Question |
|---|----------|
| S2.1 | Quel(s) langage(s) de programmation ? |
| S2.2 | Quel(s) framework(s) ou librairies principales ? |
| S2.3 | Quelle(s) base(s) de donnÃ©es ? |
| S2.4 | Style d'API ? (REST, GraphQL, gRPC, WebSocket, aucune) |
| S2.5 | Gestionnaire de paquets / outil de build ? |
| S2.6 | Monorepo ou multi-repo ? |
| S2.7 | Conventions de code existantes ? (linter, formateur, style guide) |

### 2.2 â€” SÃ©curitÃ© et conformitÃ© (S2.8-S2.11)

| # | Question |
|---|----------|
| S2.8 | Niveau de sÃ©curitÃ© requis ? (standard, durci, critique) |
| S2.9 | ConformitÃ© rÃ©glementaire ? (RGPD, HIPAA, SOC2, PCI-DSS, aucune) |
| S2.10 | Chiffrement des donnÃ©es ? (au repos, en transit, bout-en-bout, aucun) |
| S2.11 | Gestion des secrets ? (variables d'env, vault, .env, aucune) |

### 2.3 â€” Infrastructure et dÃ©ploiement (S2.12-S2.16)

| # | Question |
|---|----------|
| S2.12 | HÃ©bergement du code ? (GitHub, GitLab, Bitbucket, self-hosted) |
| S2.13 | CI/CD en place ? (GitHub Actions, GitLab CI, Jenkins, aucune) |
| S2.14 | Conteneurisation ? (Docker, Podman, Kubernetes, aucune) |
| S2.15 | HÃ©bergement / dÃ©ploiement ? (VPS, cloud, PaaS, self-hosted, local) |
| S2.16 | Authentification ? (OAuth2, JWT, sessions, passkeys, aucune) |

### 2.4 â€” RÃ©pertoire de travail (S2.17-S2.19)

| # | Question |
|---|----------|
| S2.17 | RÃ©pertoire racine du projet ? (dÃ©tectÃ© automatiquement si possible) |
| S2.18 | Structure de rÃ©pertoires imposÃ©e ? |
| S2.19 | Fichiers .mip/protocol/conventions.md, AGENTS.md ou rules existants ? |

---

## SETUP-3 â€” Profil utilisateur (questionnaire interactif)

> 8 questions pour adapter la communication et le niveau de dÃ©tail.

| # | Question |
|---|----------|
| S3.1 | RÃ´le ? (dev solo, tech lead, CTO, Ã©tudiant, PM, autre) |
| S3.2 | Niveau d'expÃ©rience ? (dÃ©butant <1 an, junior 1-3, intermÃ©diaire 3-5, senior 5-10, expert 10+) |
| S3.3 | Langages maÃ®trisÃ©s ? |
| S3.4 | Domaines de connaissance ? (web, mobile, systÃ¨mes, data, IA, sÃ©curitÃ©, devops, game dev) |
| S3.5 | Style de travail ? (autonome, collaboratif, supervision) |
| S3.6 | Niveau de dÃ©tail attendu ? (concis, normal, dÃ©taillÃ©) |
| S3.7 | Objectif principal ? (livrer, apprendre, prototyper, maintenir, refactorer) |
| S3.8 | RÃ¨gles NON NÃ‰GOCIABLES ? (liste libre) |

**Mapping style -> mode autonomie automatique** : Autonome -> FULL | Collaboratif -> BIG_STEPS | Supervision -> GUIDED

---

## SETUP-4 â€” DÃ©tection du support outil IA

### Auto-dÃ©tection

| Indicateur | Outil dÃ©tectÃ© |
|------------|---------------|
| Variable `CLAUDE_CODE` ou contexte `claude-code` | Claude Code CLI |
| Variable `CURSOR_*` ou `.cursor/` prÃ©sent | Cursor IDE |
| Variable `VSCODE_*` sans `CURSOR_*` | VS Code |
| Variable `CODEX_*` ou `openai-codex` | OpenAI Codex CLI |
| Contexte serveur MCP actif | Outil compatible MCP |
| Aucun indicateur | Demander Ã  l'utilisateur |

### Questionnaire complÃ©mentaire (S4.1-S4.4)

| # | Question |
|---|----------|
| S4.1 | Outil IA principal ? (auto-dÃ©tectÃ© : `{tool}`) |
| S4.2 | InfÃ©rence locale ? (LM Studio, Ollama, llama.cpp, vLLM, Jan, LocalAI, aucune) |
| S4.3 | ModÃ¨le(s) IA prÃ©fÃ©rÃ©(s) ? |
| S4.4 | Budget IA mensuel ? (fourchette indicatif) |
| S4.5 | Fournisseur(s) IA utilisÃ©s ? (Anthropic, OpenAI, Google/Gemini, Mistral, Moonshot/Kimi, DeepSeek, Groq, Z, autre) |
| S4.6 | Plan/abonnement par fournisseur ? (free, pro, team, enterprise, payg) |
| S4.7 | Quota tokens par pÃ©riode si connu ? (ex. 5M/mois Anthropic Pro, 1.5M/mois Gemini Free) |
| S4.8 | Suivi consommation vs quota activÃ© ? (oui â†’ crÃ©e `.mip/config/subscriptions.md`) |

### Matrice de compatibilitÃ© MIP

| CapacitÃ© MIP | Claude Code | Cursor | Codex CLI | Aider | Ollama/LM Studio | Interne |
|--------------|-------------|--------|-----------|-------|------------------|---------|
| Agents parallÃ¨les | Natif | Via Agent | Natif | Non | Non | DÃ©pend |
| TodoWrite | Natif | Non | Non | Non | Non | Non |
| MCP | Client+Srv | Client | Client+Srv | Non | Via bridge | Oui |
| Terminal | Natif | Natif | Natif | Natif | Via bridge | Natif |
| ArriÃ¨re-plan | Natif | LimitÃ© | Natif | Non | Non | Non |
| VÃ©rification docs | Via MCP | Via MCP | Via MCP | Non | Non | Oui |

**Adaptations** : Voir `.mip/profiles/capabilities-matrix.md`. MIP adapte automatiquement selon le **profil actif**.

### Profils MIP (bascule outil/LLM)

Les profils permettent de changer d'outil (Claude Code, Cursor, Codex, Ollama, LM Studio, solution interne) sans modifier le protocole. Chaque profil dÃ©finit les capacitÃ©s et les adaptations.

- **Profil actif** : `.mip/profiles/active` (1 ligne = slug)
- **Profils disponibles** : `.mip/profiles/INDEX.md`
- **Commandes** : `/mip_profile` (afficher), `/mip_profile <slug>` (basculer), `/mip_profile list` (lister)

Lors du SETUP-4, Maria propose de crÃ©er ou sÃ©lectionner un profil. Le profil par dÃ©faut est `anthropic-opus` (Claude Code + Opus/Sonnet).

---

## SETUP-5 â€” DÃ©tection et installation des dÃ©pendances

### Invariants universels

| Outil | DÃ©tection | Installation |
|-------|-----------|--------------|
| git | `git --version` | `winget install Git.Git` / `brew install git` / `apt install git` |
| bash/zsh | `echo $SHELL` | PrÃ©installÃ© |

### DÃ©tection par stack (depuis SETUP-2)

**Rust** : `rustc`, `cargo`, `rustup show`, `cargo clippy`, `cargo fmt`
**JavaScript/TypeScript** : `node`, `npm/yarn/pnpm/bun`, `tsc`, `eslint`, `prettier`
**Python** : `python3`, `pip/poetry/uv`, `pytest`, `mypy`, `ruff`
**Go** : `go version`, `golangci-lint`
**Java/Kotlin** : `java`, `mvn/gradle`
**C/C++** : `gcc/clang`, `cmake`

### Outils transverses

Docker, docker-compose, kubectl, gh (GitHub CLI), curl, jq, openssl.

L'agent propose l'installation des outils manquants (commande adaptÃ©e Ã  l'OS). L'utilisateur valide chaque installation.

---

## SETUP-6 â€” Inventaire des capacitÃ©s et installation des agents

### Agents MIP universels (10 rÃ´les coeur + Bob optionnel)

> Table complÃ¨te dans `.mip/protocol/conventions.md`, section Â« Ã‰quipe MIP Â».

Les 10 rÃ´les fonctionnels coeur sont indÃ©pendants de la stack : Chef de projet, Analyste produit, Chef dev, Dev back-end, Dev front-end, Team manager, Expert audit, Expert cybersÃ©curitÃ©, DevOps et infra, Responsable efficience IA. Bob est optionnel, surtout pour MASS et certaines tÃ¢ches simples.

### Adaptation Ã  la stack

Les agents sont configurÃ©s avec les commandes build/test/lint/format, la structure des paquets, les conventions de commit et les patterns de test adaptÃ©s au langage/framework dÃ©tectÃ©.

### Fichiers gÃ©nÃ©rÃ©s par SETUP

| Fichier | Contenu |
|---------|---------|
| `.mip/environment.md` | Config maÃ®tre : OS, matÃ©riel, stack, sÃ©curitÃ©, infra, outil IA, dÃ©pendances |
| `.mip/memory/user-profile.md` | Profil utilisateur : rÃ´le, expÃ©rience, prÃ©fÃ©rences, mode autonomie |
| `.mip/memory/project-file-map.md` | Carte des fichiers clÃ©s du projet |
| `.mip/memory/stack-patterns.md` | Patterns spÃ©cifiques Ã  la stack |
| `.mip/memory/stack-cheatsheet.md` | Cheatsheet du framework principal |
| `.mip/memory/api-contracts.md` | Contrats API (schÃ©mas, erreurs, versions) |
| `.mip/memory/test-templates.md` | Templates de tests (unit, intÃ©gration, e2e) |
| `.mip/memory/code-annotations-templates.md` | Templates d'annotations de code |
| `.mip/memory/security-patterns.md` | Patterns de sÃ©curitÃ© validÃ©s |
| `.mip/memory/mip-decisions.md` | Journal des dÃ©cisions protocole/projet |
| `.mip/memory/mip-performance-history.md` | Historique efficience (tokens, durÃ©es, actions) |
| `.mip/memory/team-skills-audit.md` | Cartographie des compÃ©tences agents |
| `.mip/memory/mip-lessons.md` | LeÃ§ons apprises par sÃ©quence |
| `.mip/memory/agent-tuning.md` | RÃ©glages prompts/modÃ¨les/outils par agent |
| `.mip/protocol/conventions.md` | Conventions MIP (classification, Ã©quipe, artefacts) |
| `.mip/agents/*.md` | Agents adaptÃ©s Ã  la stack |
| `.mip/profiles/active` | Profil MIP actif (outil + LLM) |
| `.mip/profiles/{slug}.md` | DÃ©finition des profils (capacitÃ©s, adaptations) |

### Dossier secrets (configuration initiale)

Le dossier `.mip/secrets/` contient les informations d'authentification pour les outils externes (GitHub, VPS, API, etc.). **Obligatoire lors du SETUP** :

1. CrÃ©er `.mip/secrets/` s'il n'existe pas
2. VÃ©rifier que `.mip/secrets/*` est dans le `.gitignore` du projet (le README.md du dossier peut rester versionnÃ©)
3. Les agents chargent les secrets depuis ce dossier pour s'authentifier (push, dÃ©ploiement, etc.)

#### Politique minimale secrets (normative)

- Stockage : 1 fichier par service (`github.env`, `vps.env`, `openai.env`) + `README.md` non sensible.
- Permissions minimales : Linux/macOS `chmod 600 .mip/secrets/*`; Windows `icacls .mip\\secrets\\* /inheritance:r`.
- Rotation : tout token/secret est renouvelÃ© aprÃ¨s incident, partage, ou dÃ©part collaborateur.
- TraÃ§abilitÃ© : tenir `.mip/secrets/inventory.md` (service, propriÃ©taire, date crÃ©ation, date rotation, sans valeur secrÃ¨te).
- Interdits : aucune valeur secrÃ¨te dans README, logs, captures, commits ou artefacts de sÃ©quence.

---

## ModÃ¨le `.mip/environment.md`

```markdown
# Configuration environnement MIP

## TL;DR
<Stack, OS, outil IA, mode autonomie, rÃ©sumÃ© 3 lignes>

## MÃ©tadonnÃ©es
- Date de configuration : YYYY-MM-DD
- Version MIP : v2.1
- Reconfigurable via : `/mip_setup`

## SystÃ¨me
- OS : <nom> <version> (<arch>)
- Shell : <type> <version>
- CPU : <modÃ¨le> (<N> cÅ“urs)
- RAM : <total> (<disponible> libre)
- GPU : <modÃ¨le> (<VRAM>) | Aucune
- Disque : <total> (<libre> libre)
- RÃ©seau : Internet <oui/non>, Proxy <oui/non>

## Stack technique
- Langage(s) : <liste>
- Framework(s) : <liste>
- Base(s) de donnÃ©es : <liste>
- Style API : <type>
- Gestionnaire de paquets : <nom>
- Monorepo : <oui/non> (<type>)
- Linter : <nom + config>
- Formateur : <nom + config>
- Framework de tests : <nom>

## Commandes standard
- Build : `<commande>`
- Test : `<commande>`
- Lint : `<commande>`
- Format : `<commande>`
- Test unitaire : `<commande avec placeholder>`

## SÃ©curitÃ©
- Niveau : <standard/durci/critique>
- ConformitÃ© : <rÃ©glementation>
- Chiffrement : <type>
- Secrets : <mÃ©thode>

## Infrastructure
- HÃ©bergement code : <plateforme>
- CI/CD : <outil>
- Conteneurisation : <outil>
- DÃ©ploiement : <type>
- Auth : <mÃ©thode>

## Outil IA
- Outil principal : <nom>
- InfÃ©rence locale : <outil>
- ModÃ¨le(s) : <liste>
- Budget : <fourchette>
- Abonnements : `.mip/config/subscriptions.md` (si renseignÃ© â€” Anthropic, OpenAI, Gemini, Moonshot, etc.)
- CapacitÃ©s MIP adaptÃ©es :
  - Agents parallÃ¨les : <oui/non>
  - TodoWrite : <oui/non>
  - VÃ©rification docs/MCP : <oui/non>
  - TÃ¢ches arriÃ¨re-plan : <oui/non>
  - AccÃ¨s terminal : <oui/non>

## DÃ©pendances
### InstallÃ©es
- <outil> : <version>

### Manquantes (installÃ©es pendant SETUP)
- <outil> : <version installÃ©e>

## Conventions du projet
- Convention commit : <type>
- Annotations de code : <type>
- Patterns d'erreur : <type>
- RÃ¨gles NON NÃ‰GOCIABLES utilisateur : <liste depuis S3.8>
```

---

## Invariants MIP universels

| Invariant | Universel parce que... |
|-----------|------------------------|
| Classification T1-T5 | Toute tÃ¢che a une taille |
| Git (VCS) | Standard industriel universel |
| Tests avant livraison | Fondamental, chaque langage |
| Lint/formatage | Chaque langage a ses outils |
| Revue de code / audit | Bonne pratique universelle |
| Chiffrement | Obligation lÃ©gale et technique |
| Gestion des secrets | OWASP Top 10, universel |
| CI/CD | Standard industriel |
| Documentation | Maintenance long terme |
| MÃ©triques et feedback | Kaizen / amÃ©lioration continue |

