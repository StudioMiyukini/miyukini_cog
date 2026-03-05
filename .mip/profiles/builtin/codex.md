---
id: codex
name: OpenAI Codex CLI
tool: codex
llm:
  provider: openai
  model: gpt-4o
  context_window: 128000
  fallback: gpt-4o-mini
capabilities:
  parallel_agents: true
  todo_write: false
  mcp: true
  terminal: true
  background_tasks: true
  doc_verification: true
  multi_file_edit: true
  ask_user_question: true
adaptations:
  - todo_write: "Annonces texte"
---

# Profil OpenAI Codex CLI

> OpenAI Codex (CLI agentique). Proche de Claude Code en capacités.

**Outil** : OpenAI Codex CLI  
**Modèle** : gpt-4o, o1, etc.

Seule adaptation : TodoWrite absent → annonces texte.
