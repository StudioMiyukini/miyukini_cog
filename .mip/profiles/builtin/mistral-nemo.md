---
id: mistral-nemo
name: Mistral Nemo (API & Local) — Admin/Open-Source
tool: mistral-api
llm:
  provider: mistral
  model: mistral-nemo-latest
  context_window: 128000
  endpoint: https://api.mistral.ai/v1 | http://localhost:8000
  fallback: mistral-7b
capabilities:
  parallel_agents: true
  todo_write: true
  mcp: false
  terminal: true
  background_tasks: true
  doc_verification: false
  multi_file_edit: true
  ask_user_question: true
adaptations:
  - mcp: "Non disponible → recherche web alternative via SearchAPI/DuckDuckGo"
  - doc_verification: "Pas de résolution blocs → Trust pattern memory ou lecture manuelle"
constraints:
  - "Context 128k mais vitesse moyenne (vs Sonnet)"
  - "Pas de native MCP → Utiliser bridge SearchAPI ou offline"
  - "Coût PAYG optimisé (moins cher que Anthropic)"
  - "Support de function_calling via tools JSON Schema"
industrial_scope:
  - "✅ T2-T3 standard (petit design, implémentation, tests)"
  - "✅ Multi-file edits (via multi_replace)"
  - "✅ Terminal access + background jobs"
  - "✅ Local inference (économies 100 %)"
  - "Limité T4-T5 sans MCP web search"
---

# Mistral Nemo — Profil Admin/Open-Source

> **Cas réel** : Agent administratif (budget limité) ou entreprise open-source.  
> Profil fort. MIP ≈ 90 % capacités (pas MCP web).  
> Peut tourner **100 % local** avec LM Studio / Ollama.

## Profil

- **Outil** : Mistral API (mistral.ai) OU Local (LM Studio, Ollama)
- **Modèle** : Mistral Nemo (ou 7B local)
- **Context** : 128k tokens
- **Coûts** : PAYG ~$0.3/1M input, $0.9/1M output (API) | Zéro (local)

## Points forts vs Limitations

| Aspect | Status | Notes |
|--------|--------|-------|
| **Context window** | ✅ 128k | Assez pour 2-3 crates |
| **Parallelisme** | ✅ Oui | Agents simultanés possibles |
| **Terminal** | ✅ Natif | Exécution pleine (shell, cargo) |
| **Multi-file** | ✅ Oui | Édition parallèle 3-5 fichiers |
| **TodoWrite** | ✅ Oui | Orchestration interne |
| **Web search** | ❌ Pas de MCP | → Fallback SearchAPI ou offline doc |
| **Doc verification** | ❌ Auto-résolution blocs | → Mémoire pattern ou lecture |
| **Coûts** | ✅ Économique | PAYG ou local gratuit |

## Adaptations MIP appliquées

### A : MCP non disponible → Fallback SearchAPI

```yaml
If MCP unavailable:
  - Use SearchAPI.io (gratuit tier)
  - Or Qwant/DuckDuckGo locally via shell curl
  - Or read offline Markdown docs

Example:
  MCP call not found
  → Query SearchAPI("how to use Tauri events in Dioxus 0.6")
  → Process results, continue
```

### B : Local Inference (Économies maximales)

```yaml
Mistral Nemo via LM Studio (local):
  1. User downloads mistral-nemo GGUF (~8GB)
  2. LM Studio server on localhost:8000
  3. MIP bridge connects to local endpoint
  4. Zero API costs, full speed
  5. Offline-capable (Autonomy Law 1)
```

### C : Function Calling via JSON Schema

```json
{
  "name": "file_edit",
  "description": "Edit file with oldString/newString",
  "parameters": {
    "type": "object",
    "properties": {
      "path": {"type": "string"},
      "oldString": {"type": "string"},
      "newString": {"type": "string"}
    },
    "required": ["path", "oldString", "newString"]
  }
}
```

Mistral supporte bien les tools → éditions parallèles possibles.

## Workflow Mistral (T3 example)

```
🔵 P0 Framing (Maria)
   - "Implémentez X feature dans Y service"
   - Scope, dépendances, risques → 1k tokens

🟢 P3 Implementation (parallèle possible)
   Agent Lise (frontend) :
     - Lire ui-builder/src/components/modal.rs (~500 tokens)
     - Générer changements
   Agent François (backend) :
     - Lire service/src/lib.rs (~500 tokens)
     - Générer changements
   [Parallèle : ~1500 tokens total vs 3k séquentiel]

🟣 P4 Integration (Tests)
   - Terminal : `cargo build -p service && cargo test -p service`
   - Output→ Mistral → Analyse errors
   - Itère jusqu'à ✅

🟡 P5 Delivery (Human Review)
   - Rapport changements
   - Code review suggestions

⚫ P6 Archive
   - Métriques tokens: ~80k pour T3
   - Coût API : ~$0.12 (vs $3 Anthropic Pro)
   - Rapport final  + memory
```

## Coûts T3 estimés

| Profil | Context | Itérations | Coût estimé | Temps |
|--------|---------|------------|------------|-------|
| **Copilot gratuit** | 8k | 4-5 man.iter | $0 | 3h |
| **Mistral API** | 128k | 2 auto.iter | $0.12 | 45min |
| **Mistral local** | 128k | 2 auto.iter | $0 | 1h (HW) |
| **Claude Code** | 200k | 1 iter | $1.50 | 15min |

Mistral = meilleur **coût/capacités** pour PME/startups.

## Quand utiliser ce profil ?

✅ **EXCELLENT** :
- T2-T3 (standard industrial)
- Budget constraints (PAYG ou open-source)
- On-premises / offline requis
- Multi-service light orchestration

⚠️ **MAUVAIS** :
- T5 (besoin recherche web avancée)
- Très haute sécurité (< compliance audit)
- Real-time collaboration (lenteur)

❌ **À ÉVITER** :
- Zero offline capability (besoin web constant)
- Model très spécialisé nécessaire

## Setup local (Mistral 7B via Llama Studio)

```bash
# 1. Download LM Studio (lmstudio.ai)
# 2. Search "mistral-7b-instruct" GGUF
# 3. Load → Auto on localhost:8000
# 4. MIP détecte → Use lm-studio profile
# 5. Bonus : ~8GB RAM, 15 tok/sec M1 Max
```

## Profil additionnel : Mistral Small (API budget)

```yaml
id: mistral-small-api
name: Mistral Small (API ultra-économique)
tool: mistral-api
llm:
  model: mistral-small-latest
  context_window: 32000
  cost_input_per_mtok: 0.14
  cost_output_per_mtok: 0.42
# Même adaptations, context x1/4 (32k vs 128k)
# Bon pour T2 et petits T3
# Coût 5x moins cher que Small Nemo
```

## Liens documentation

- [MIP Workflow](../protocol/conventions.md) — P0-P6
- [Terminal Integration](../modules/tooling-terminal.md)
- [Tools & Function Calling](../modules/tools-schema.md)
- [SearchAPI Fallback](../modules/search-fallback.md)
