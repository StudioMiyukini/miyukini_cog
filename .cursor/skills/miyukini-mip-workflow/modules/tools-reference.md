# Module MIP — Reference Outils IA Compatibles

> Ce module est charge sur demande explicite ou lors de SETUP-4.

---

## Categorie 1 — CLI agentiques (execution complete MIP)

| Outil | Editeur | Agents // | Terminal | MCP | Open Source |
|-------|---------|-----------|----------|-----|-------------|
| **Claude Code** | Anthropic | Natif | Natif | Client+Srv | Non |
| **OpenAI Codex CLI** | OpenAI | Natif | Natif | Client+Srv | Non |
| **Aider** | OSS (Paul Gauthier) | Non | Natif | Non | Oui (Apache 2) |
| **OpenCode** | OSS | Partiel | Natif | Partiel | Oui |
| **Goose** | Block | Non | Natif | Partiel | Oui |
| **Gemini CLI** | Google | Non | Natif | Oui | Non |

## Categorie 2 — IDEs natifs IA

| Outil | Editeur | Agents // | Terminal | MCP | Open Source |
|-------|---------|-----------|----------|-----|-------------|
| **Cursor** | Anysphere | Via Agent | Natif | Client | Non |
| **Windsurf** | Cognition AI | Via Cascade | Natif | Non | Non |
| **Zed** | Zed Industries | Via Agent Panel | Natif | Client | Oui (GPL) |

## Categorie 3 — Extensions IDE

| Outil | Editeur | Agents // | Terminal | MCP | Open Source |
|-------|---------|-----------|----------|-----|-------------|
| **GitHub Copilot** | Microsoft | Agent Mode | Oui | Client | Non |
| **Gemini Code Assist** | Google | Agent Mode | Oui | Client+Srv | Non |
| **Amazon Q Developer** | AWS | Agent `/dev` | CLI agent | Partiel | Non |
| **JetBrains AI** | JetBrains | Partiel | Limite | Server | Non |
| **Continue.dev** | OSS | Partiel | Non | Client | Oui (Apache 2) |
| **Cline** | OSS | Complet | Oui | Client | Oui |
| **Cody** | Sourcegraph | Partiel | Non | Non | Oui |

## Categorie 4 — Runtimes inference locale

| Outil | Type | API compatible | Usage MIP |
|-------|------|---------------|-----------|
| **LM Studio** | GUI + API | OpenAI-compat `localhost:1234` | Backend pour Continue.dev, JetBrains, miou-llm-bridge |
| **Ollama** | CLI + API | OpenAI-compat | Backend headless pour scripts et agents |
| **llama.cpp** | CLI | OpenAI-compat (server mode) | Performance maximale, usage avance |
| **vLLM** | Server Python | OpenAI-compat | Haute performance GPU, batching |
| **Jan** | GUI desktop | OpenAI-compat | Alternative LM Studio legere |
| **LocalAI** | Container API | OpenAI-compat | Docker-first, multi-modele |

## Categorie 5 — Plateformes cloud autonomes (usage limite)

| Outil | Usage MIP | Limitation |
|-------|-----------|------------|
| **Devin 2.0** | Taches isolees T1-T2 | Pas d'acces repos locaux |
| **Replit Agent 3** | Prototypage rapide | JS/TS seulement |
| **Bolt.new** | Prototypage front | WebContainers |
| **v0** | Generation UI React | Front-end React seulement |

---

## Recommandations par scenario

| Scenario | Outil recommande | Raison |
|----------|-----------------|--------|
| MIP complet (T3-T5) | Claude Code / Codex CLI | Agents // natifs, terminal, MCP, TodoWrite |
| Edition quotidienne + MIP | Cursor + Claude Code | Navigation rapide + heavy-lift |
| Budget gratuit / OSS | Aider + Ollama | Git-first, local, pas de frais |
| Confidentialite max | Continue.dev + LM Studio | Tout local, zero fuite |
| Enterprise multi-IDE | GitHub Copilot | Compatible VS Code, JetBrains, CLI |
| AWS-native | Amazon Q Developer | Integration IAM, CDK |
| Gros monorepo (50k+) | Augment Code ou Cody | Indexation semantique |
