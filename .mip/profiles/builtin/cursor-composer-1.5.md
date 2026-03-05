---
id: cursor-composer-1.5
name: Cursor Composer 1.5
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

# Profil Cursor Composer 1.5

> Cursor IDE avec Composer. Variante profil cursor-composer.

**Outil** : Cursor  
**Modèle** : Claude Sonnet 4 (configurable)

Adaptations MIP : TodoWrite → annonces ; arrière-plan → premier plan.
