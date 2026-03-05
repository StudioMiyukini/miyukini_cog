---
id: ollama
name: Ollama (inférence locale)
tool: ollama
llm:
  provider: ollama
  model: llama3.1
  context_window: 128000
  endpoint: http://localhost:11434
  fallback: codellama
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
  - parallel_agents: "MASS séquentiel, Loi 9 suspendue"
  - todo_write: "Annonces texte"
  - mcp: "Recherche web ou skip"
  - doc_verification: "Recherche web"
  - multi_file_edit: "Via bridge (Aider, Continue, Cline)"
---

# Profil Ollama

> Inférence locale. Requiert un bridge (Aider, Continue.dev, Cline, Miou) pour exécution MIP.

**Outil** : Ollama (backend) + bridge (Aider / Continue / Cline / solution interne)  
**Modèles** : llama3.1, codellama, mistral, etc. — configurables

MIP s'adapte : exécution séquentielle, annonces texte, pas de MCP natif. Le bridge détermine les capacités terminal/édition.
