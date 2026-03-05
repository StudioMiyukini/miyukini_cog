---
id: mistral-nemo
name: Mistral Nemo (API & Local) â€” Admin/Open-Source
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
  - mcp: "Non disponible â†’ recherche web alternative via SearchAPI/DuckDuckGo"
  - doc_verification: "Pas de rÃ©solution blocs â†’ Trust pattern memory ou lecture manuelle"
constraints:
  - "Context 128k mais vitesse moyenne (vs Sonnet)"
  - "Pas de native MCP â†’ Utiliser bridge SearchAPI ou offline"
  - "CoÃ»t PAYG optimisÃ© (moins cher que Anthropic)"
  - "Support de function_calling via tools JSON Schema"
industrial_scope:
  - "âœ… T2-T3 standard (petit design, implÃ©mentation, tests)"
  - "âœ… Multi-file edits (via multi_replace)"
  - "âœ… Terminal access + background jobs"
  - "âœ… Local inference (Ã©conomies 100 %)"
  - "LimitÃ© T4-T5 sans MCP web search"
---

# Mistral Nemo â€” Profil Admin/Open-Source

> **Cas rÃ©el** : Agent administratif (budget limitÃ©) ou entreprise open-source.  
> Profil fort. MIP â‰ˆ 90 % capacitÃ©s (pas MCP web).  
> Peut tourner **100 % local** avec LM Studio / Ollama.

## Profil

- **Outil** : Mistral API (mistral.ai) OU Local (LM Studio, Ollama)
- **ModÃ¨le** : Mistral Nemo (ou 7B local)
- **Context** : 128k tokens
- **CoÃ»ts** : PAYG ~$0.3/1M input, $0.9/1M output (API) | ZÃ©ro (local)

## Points forts vs Limitations

| Aspect | Status | Notes |
|--------|--------|-------|
| **Context window** | âœ… 128k | Assez pour 2-3 crates |
| **Parallelisme** | âœ… Oui | Agents simultanÃ©s possibles |
| **Terminal** | âœ… Natif | ExÃ©cution pleine (shell, cargo) |
| **Multi-file** | âœ… Oui | Ã‰dition parallÃ¨le 3-5 fichiers |
| **TodoWrite** | âœ… Oui | Orchestration interne |
| **Web search** | âŒ Pas de MCP | â†’ Fallback SearchAPI ou offline doc |
| **Doc verification** | âŒ Auto-rÃ©solution blocs | â†’ MÃ©moire pattern ou lecture |
| **CoÃ»ts** | âœ… Ã‰conomique | PAYG ou local gratuit |

## Adaptations MIP appliquÃ©es

### A : MCP non disponible â†’ Fallback SearchAPI

```yaml
If MCP unavailable:
  - Use SearchAPI.io (gratuit tier)
  - Or Qwant/DuckDuckGo locally via shell curl
  - Or read offline Markdown docs

Example:
  MCP call not found
  â†’ Query SearchAPI("how to use Tauri events in Dioxus 0.6")
  â†’ Process results, continue
```

### B : Local Inference (Ã‰conomies maximales)

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

Mistral supporte bien les tools â†’ Ã©ditions parallÃ¨les possibles.

## Workflow Mistral (T3 example)

```
ðŸ”µ P0 Framing (Maria)
   - "ImplÃ©mentez X feature dans Y service"
   - Scope, dÃ©pendances, risques â†’ 1k tokens

ðŸŸ¢ P3 Implementation (parallÃ¨le possible)
   Agent Lise (frontend) :
     - Lire ui-builder/src/components/modal.rs (~500 tokens)
     - GÃ©nÃ©rer changements
   Agent FranÃ§ois (backend) :
     - Lire service/src/lib.rs (~500 tokens)
     - GÃ©nÃ©rer changements
   [ParallÃ¨le : ~1500 tokens total vs 3k sÃ©quentiel]

ðŸŸ£ P4 Integration (Tests)
   - Terminal : `cargo build -p service && cargo test -p service`
   - Outputâ†’ Mistral â†’ Analyse errors
   - ItÃ¨re jusqu'Ã  âœ…

ðŸŸ¡ P5 Delivery (Human Review)
   - Rapport changements
   - Code review suggestions

âš« P6 Archive
   - MÃ©triques tokens: ~80k pour T3
   - CoÃ»t API : ~$0.12 (vs $3 Anthropic Pro)
   - Rapport final  + memory
```

## CoÃ»ts T3 estimÃ©s

| Profil | Context | ItÃ©rations | CoÃ»t estimÃ© | Temps |
|--------|---------|------------|------------|-------|
| **Copilot gratuit** | 8k | 4-5 man.iter | $0 | 3h |
| **Mistral API** | 128k | 2 auto.iter | $0.12 | 45min |
| **Mistral local** | 128k | 2 auto.iter | $0 | 1h (HW) |
| **Claude Code** | 200k | 1 iter | $1.50 | 15min |

Mistral = meilleur **coÃ»t/capacitÃ©s** pour PME/startups.

## Quand utiliser ce profil ?

âœ… **EXCELLENT** :
- T2-T3 (standard industrial)
- Budget constraints (PAYG ou open-source)
- On-premises / offline requis
- Multi-service light orchestration

âš ï¸ **MAUVAIS** :
- T5 (besoin recherche web avancÃ©e)
- TrÃ¨s haute sÃ©curitÃ© (< compliance audit)
- Real-time collaboration (lenteur)

âŒ **Ã€ Ã‰VITER** :
- Zero offline capability (besoin web constant)
- Model trÃ¨s spÃ©cialisÃ© nÃ©cessaire

## Setup local (Mistral 7B via Llama Studio)

```bash
# 1. Download LM Studio (lmstudio.ai)
# 2. Search "mistral-7b-instruct" GGUF
# 3. Load â†’ Auto on localhost:8000
# 4. MIP dÃ©tecte â†’ Use lm-studio profile
# 5. Bonus : ~8GB RAM, 15 tok/sec M1 Max
```

## Profil additionnel : Mistral Small (API budget)

```yaml
id: mistral-small-api
name: Mistral Small (API ultra-Ã©conomique)
tool: mistral-api
llm:
  model: mistral-small-latest
  context_window: 32000
  cost_input_per_mtok: 0.14
  cost_output_per_mtok: 0.42
# MÃªme adaptations, context x1/4 (32k vs 128k)
# Bon pour T2 et petits T3
# CoÃ»t 5x moins cher que Small Nemo
```

## Liens documentation

- [MIP Workflow](..//..//README.md) â€” P0-P6
- [Terminal Integration](..//..//README.md)
- [Tools & Function Calling](..//..//README.md)
- [SearchAPI Fallback](..//..//README.md)

