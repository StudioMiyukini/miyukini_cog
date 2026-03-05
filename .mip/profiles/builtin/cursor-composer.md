---
id: cursor-composer
name: Cursor Composer (Claude/autre)
tool: cursor
llm:
  provider: anthropic
  model: claude-sonnet-4
  context_window: 200000
  fallback: claude-3-5-haiku
capabilities:
  parallel_agents: true
  todo_write: false
  mcp: true
  terminal: true
  background_tasks: false
  doc_verification: true
  multi_file_edit: true
  ask_user_question: true
adaptations:
  - todo_write_absent: "Annonces texte à chaque étape, liste tenue par orchestrateur"
  - background_tasks: "Toutes tâches en premier plan"
---

# Profil Cursor Composer

> Cursor IDE avec Composer. Agents via Agent Panel. Pas de TodoWrite natif.

**Outil** : Cursor (Anthropic / OpenAI selon config)  
**Modèle** : Configurable dans Cursor (Claude, GPT-4o, etc.)

**Adaptations** : TodoWrite → annonces texte. Arrière-plan → premier plan.
