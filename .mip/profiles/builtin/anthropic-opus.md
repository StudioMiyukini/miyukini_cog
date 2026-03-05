---
id: anthropic-opus
name: Anthropic Opus 4.6 (Claude Code)
tool: claude-code
llm:
  provider: anthropic
  model: claude-sonnet-4-20250514
  context_window: 200000
  fallback: claude-3-5-haiku-20241022
capabilities:
  parallel_agents: true
  todo_write: true
  mcp: true
  terminal: true
  background_tasks: true
  doc_verification: true
  multi_file_edit: true
  ask_user_question: true
adaptations: []
---

# Profil Anthropic Opus (Claude Code)

> Profil par défaut. Capacités MIP complètes. Recommandé pour T3-T5.

**Outil** : Claude Code CLI (Anthropic)  
**Modèle principal** : claude-sonnet-4 ou Opus 4  
**Contexte** : 200k tokens

Aucune adaptation nécessaire. MIP tourne avec toutes les capacités (parallélisme, TodoWrite, MCP, terminal, vérification docs).
