# Module MIP — Référence des outils IA compatibles

> Ce module est chargé sur demande explicite ou pendant SETUP-4.

---

## Catégorie 1 — CLIs agentiques (exécution MIP complète)

| Outil | Éditeur | Agents parallèles | Terminal | MCP | Open Source |
|-------|---------|-------------------|----------|-----|-------------|
| **Claude Code** | Anthropic | Natif | Natif | Client+Srv | Non |
| **OpenAI Codex CLI** | OpenAI | Natif | Natif | Client+Srv | Non |
| **Aider** | OSS (Paul Gauthier) | Non | Natif | Non | Oui (Apache 2) |
| **OpenCode** | OSS | Partiel | Natif | Partiel | Oui |
| **Goose** | Block | Non | Natif | Partiel | Oui |
| **Gemini CLI** | Google | Non | Natif | Oui | Non |

## Catégorie 2 — IDE IA natifs

| Outil | Éditeur | Agents parallèles | Terminal | MCP | Open Source |
|-------|---------|-------------------|----------|-----|-------------|
| **Cursor** | Anysphere | Via Agent | Natif | Client | Non |
| **Windsurf** | Cognition AI | Via Cascade | Natif | Non | Non |
| **Zed** | Zed Industries | Via Agent Panel | Natif | Client | Oui (GPL) |

## Catégorie 3 — Extensions IDE

| Outil | Éditeur | Agents parallèles | Terminal | MCP | Open Source |
|-------|---------|-------------------|----------|-----|-------------|
| **GitHub Copilot** | Microsoft | Mode Agent | Oui | Client | Non |
| **Gemini Code Assist** | Google | Mode Agent | Oui | Client+Srv | Non |
| **Amazon Q Developer** | AWS | Agent `/dev` | Agent CLI | Partiel | Non |
| **JetBrains AI** | JetBrains | Partiel | Limité | Serveur | Non |
| **Continue.dev** | OSS | Partiel | Non | Client | Oui (Apache 2) |
| **Cline** | OSS | Complet | Oui | Client | Oui |
| **Cody** | Sourcegraph | Partiel | Non | Non | Oui |

## Catégorie 4 — Runtimes d'inférence locale

| Outil | Type | API compatible | Usage MIP |
|-------|------|----------------|-----------|
| **LM Studio** | GUI + API | OpenAI-compat `localhost:1234` | Backend pour Continue.dev, JetBrains, bridges locaux |
| **Ollama** | CLI + API | OpenAI-compat | Backend headless pour scripts et agents |
| **llama.cpp** | CLI | OpenAI-compat (mode serveur) | Performance maximale, usage avancé |
| **vLLM** | Serveur Python | OpenAI-compat | Haute performance GPU, batching |
| **Jan** | GUI desktop | OpenAI-compat | Alternative légère à LM Studio |
| **LocalAI** | API conteneur | OpenAI-compat | Docker-first, multi-modèle |

## Catégorie 5 — Plateformes cloud autonomes (usage limité)

| Outil | Usage MIP | Limitation |
|-------|-----------|------------|
| **Devin 2.0** | Tâches T1-T2 isolées | Pas d'accès repo local |
| **Replit Agent 3** | Prototypage rapide | JS/TS uniquement |
| **Bolt.new** | Prototypage front | WebContainers |
| **v0** | Génération UI React | Front-end React uniquement |

---

## Profils MIP et adaptation

Chaque outil expose des capacités différentes. Les **profils MIP** (`.mip/profiles/`) permettent de basculer et d'adapter le protocole automatiquement. Voir `.mip/profiles/capabilities-matrix.md` pour la matrice complète.

## Recommandations par scénario

| Scénario | Outil recommandé | Profil |
|----------|------------------|--------|
| MIP complet (T3-T5) | Claude Code / Codex CLI | anthropic-opus, codex |
| Édition quotidienne + MIP | Cursor + Claude Code | Navigation rapide + charge lourde |
| Budget gratuit / OSS | Aider + Ollama | Git-first, local, sans coût |
| Confidentialité maximale | Continue.dev + LM Studio | Tout en local, zéro fuite |
| Enterprise multi-IDE | GitHub Copilot | Compatible VS Code, JetBrains, CLI |
| Natif AWS | Amazon Q Developer | Intégration IAM, CDK |
| Monorepo volumineux (50k+) | Augment Code ou Cody | Indexation sémantique |
