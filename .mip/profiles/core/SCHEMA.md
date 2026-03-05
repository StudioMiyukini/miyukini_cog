# Profils MIP — Schéma et normes

<!-- @id mip.profiles.schema
     @do define_mip_profile_structure
     @role config
     @layer config
     @human Schéma profils MIP — outil, LLM, adaptations -->

> Les profils MIP permettent de **basculer** entre outils IA et LLM tout en adaptant le protocole aux capacités disponibles.

---

## Structure

```
.mip/profiles/
├── SCHEMA.md              # Ce fichier — normes, structure
├── INDEX.md               # Liste des profils, profil actif
├── active                 # Fichier : nom du profil actif (1 ligne)
├── capabilities-matrix.md # Matrice outil × capacités MIP
├── anthropic-opus.md      # Profil par défaut (exemple)
├── cursor-composer.md
├── ollama.md
├── lm-studio.md
├── codex.md
└── {custom}.md            # Profils personnalisés
```

---

## Définition d'un profil

Chaque fichier `{slug}.md` définit :

| Champ | Type | Description |
|-------|------|-------------|
| `id` | string | Slug unique (ex. `anthropic-opus`) |
| `name` | string | Nom lisible (ex. « Anthropic Opus 4.6 ») |
| `tool` | string | Outil IA : `claude-code`, `cursor`, `codex`, `ollama`, `lm-studio`, `aider`, `continue`, `internal` |
| `llm` | object | Provider, modèle, context_window, endpoint (si local) |
| `capabilities` | object | Surcharges par rapport à la matrice outil |
| `adaptations` | array | Liste des adaptations MIP appliquées |
| `subscription_ref` | string | (optionnel) Clé du fournisseur pour quotas : `anthropic`, `openai`, `google`, etc. |

Le champ `llm.provider` est utilisé pour associer le profil aux quotas tokens configurés dans `.mip/config/subscriptions.md`. L'utilisateur renseigne ses abonnements (tokens disponibles par période) pour une estimation consommation vs quota.

### Format YAML frontmatter (recommandé)

```yaml
---
id: anthropic-opus
name: Anthropic Opus 4.6 (Claude Code)
tool: claude-code
llm:
  provider: anthropic
  model: claude-sonnet-4-20250514
  context_window: 200000
  fallback: claude-3-5-haiku
capabilities:
  parallel_agents: true
  todo_write: true
  mcp: true
  terminal: true
  background_tasks: true
  doc_verification: true
adaptations: []
---
```

---

## Matrice de capacités MIP

| Capacité | Description | Si `false` → adaptation |
|----------|-------------|--------------------------|
| `parallel_agents` | Agents parallèles (MASS, subagents) | Exécution séquentielle |
| `todo_write` | Outil TodoWrite disponible | Annonces texte dans le chat |
| `mcp` | Serveur MCP (Context7, etc.) | Mémoire locale, recherche web |
| `terminal` | Accès terminal natif | Utilisateur exécute manuellement |
| `background_tasks` | Tâches en arrière-plan | Exécution premier plan |
| `doc_verification` | Vérification docs (resolve-library-id) | Recherche web ou skip |
| `multi_file_edit` | Édition multi-fichiers | Fichiers un par un |
| `ask_user_question` | AskUserQuestion / équivalent | Texte libre + attente réponse |

---

## Commandes

| Commande | Action |
|----------|--------|
| `/mip_profile` | Afficher le profil actif |
| `/mip_profile <slug>` | Basculer vers le profil |
| `/mip_profile list` | Lister les profils disponibles |
| `/mip_profile create <slug>` | Créer un profil (SETUP assisté) |

---

## Intégration SETUP

Lors du SETUP-4 (détection outil IA), Maria propose de créer ou sélectionner un profil. Le profil actif est enregistré dans `.mip/profiles/active`. L'agent charge ce fichier au démarrage pour connaître les adaptations à appliquer.

## Abonnements et quotas tokens

L'utilisateur peut renseigner ses abonnements (Anthropic, OpenAI, Gemini, Moonshot/Kimi, etc.) et ses tokens disponibles dans `.mip/config/subscriptions.md`. MIP utilise ces données pour :
- Estimer la consommation avant une séquence (P0)
- Comparer consommation mesurée vs quota (P6, Jean)
- Alerter si >80 % du quota consommé
