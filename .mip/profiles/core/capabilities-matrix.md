# Matrice de capacités — Outils × MIP

<!-- @id mip.profiles.capabilities
     @do provide_tool_capabilities_matrix
     @role config
     @layer config
     @human Matrice capacités MIP par outil IA -->

> Chaque outil IA expose un sous-ensemble de capacités. MIP **s'adapte automatiquement** selon le profil actif.

---

## Outils × Capacités

| Outil | Parallèle | TodoWrite | MCP | Terminal | Arrière-plan | Doc vérif | Multi-fichier | AskUserQuestion |
|-------|-----------|-----------|-----|----------|--------------|-----------|---------------|-----------------|
| **Claude Code** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Cursor (Composer)** | Via Agent | ❌ | Client | ✅ | Limité | ✅ | ✅ | ✅ |
| **Codex CLI** | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Aider** | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ |
| **Ollama** | ❌ | ❌ | Via bridge | Via shell | ❌ | ❌ | Par outil | Dépend |
| **LM Studio** | ❌ | ❌ | Via bridge | Via shell | ❌ | ❌ | Par outil | Dépend |
| **Continue.dev** | Partiel | ❌ | Client | ❌ | ❌ | ✅ | Partiel | Partiel |
| **Cline** | ✅ | ❌ | Client | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Solution interne** | À définir | À définir | À définir | À définir | À définir | À définir | À définir | À définir |

---

## LLM × Contraintes

| Provider | Modèle | Context | Fallback |
|----------|--------|---------|----------|
| Anthropic | claude-sonnet-4, opus-4 | 200k | haiku |
| Anthropic (Bob) | haiku | 200k | — |
| OpenAI | gpt-4o, o1 | 128k | gpt-4o-mini |
| Google | gemini-2.0 | 1M | gemini-flash |
| Local (Ollama) | llama3.1, codellama | 128k | Dépend modèle |
| Local (LM Studio) | Configurable | Configurable | — |

---

## Adaptations automatiques

| Capacité absente | Comportement MIP |
|------------------|------------------|
| `parallel_agents` | MASS → séquentiel. Loi 9 suspendue avec avertissement. |
| `todo_write` | Liste todo tenue en texte par l'orchestrateur. Annonces à chaque étape. |
| `mcp` | Context7 indisponible → recherche web ou skip. |
| `terminal` | Instructions à l'utilisateur : « Exécutez `cargo test` et collez la sortie. » |
| `background_tasks` | Toutes les tâches en premier plan. |
| `doc_verification` | Recherche web ou confiance aux patterns mémoire. |
| `ask_user_question` | Texte formaté + attente réponse utilisateur dans le chat. |
