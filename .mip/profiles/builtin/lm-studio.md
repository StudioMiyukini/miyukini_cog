---
id: lm-studio
name: LM Studio (inférence locale)
tool: lm-studio
llm:
  provider: lm-studio
  model: configurable
  context_window: configurable
  endpoint: http://localhost:1234/v1
  fallback: null
capabilities:
  parallel_agents: false
  todo_write: false
  mcp: false
  terminal: true
  background_tasks: false
  doc_verification: false
  multi_file_edit: false
  ask_user_question: true
adaptations:
  - parallel_agents: "MASS séquentiel"
  - todo_write: "Annonces texte"
  - mcp: "Recherche web"
  - doc_verification: "Recherche web"
---

# Profil LM Studio

> Inférence locale via LM Studio. API OpenAI-compatible sur localhost:1234.

**Outil** : LM Studio + bridge (Continue.dev, Miou-LLM-Bridge, etc.)  
**Modèle** : Dépend des modèles chargés dans LM Studio

Même adaptations qu'Ollama. Souvent utilisé avec Continue.dev ou bridge interne (Miou).
