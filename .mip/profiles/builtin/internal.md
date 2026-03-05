---
id: internal
name: Solution interne (Miou / bridge custom)
tool: internal
llm:
  provider: internal
  model: configurable
  context_window: configurable
  endpoint: http://localhost:11435
  fallback: null
capabilities:
  parallel_agents: false
  todo_write: false
  mcp: true
  terminal: true
  background_tasks: false
  doc_verification: true
  multi_file_edit: true
  ask_user_question: true
adaptations:
  - parallel_agents: "À configurer selon le bridge"
  - todo_write: "Annonces texte"
---

# Profil Solution interne

> Bridge interne (ex. Miou-LLM-Bridge, miou-llm-bridge). Adapté à l'infrastructure Miyukini.

**Outil** : Solution maison (Central + bridge)  
**Modèle** : GGUF local ou proxy upstream (LM Studio, Ollama, API cloud)

Capacités à définir selon le bridge utilisé. Exemple : `apps/miou-llm-bridge/` pour COG.
