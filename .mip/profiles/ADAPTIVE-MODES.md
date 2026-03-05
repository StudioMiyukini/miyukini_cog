---
id: mip.profiles.adaptive-modes
title: MIP Adaptive Modes â€” 5 modes d'exÃ©cution selon LLM
---

# MIP Adaptive Modes (ExÃ©cution adaptÃ©e au LLM)

> Selon le profil LLM actif, MIP bascule automatiquement entre 5 **modes d'exÃ©cution** rÃ©duisant progressivement les capacitÃ©s tout en restant productif.

---

## Les 5 modes d'adaptation

### **Mode 1 : "Autonomy Complete"** (Claude Code, Opus)

```yaml
Profils : anthropic-opus, cursor-composer 1.5, codex
CapacitÃ©s actives : 100 %
  - Parallel agents (MASS)
  - TodoWrite orchestration
  - Terminal + Background jobs
  - Multi-file edits
  - MCP + web search
  - Doc verification (auto resolution)
Workflow : P0 â†’ P3 (auto) â†’ P4-5-6 (auto)
DurÃ©e T3 : ~15-30 min
CoÃ»ts : Ã‰levÃ©s (~$1-3)
```

**Quand utilisÃ©** : R&D de pointe, critical path, multi-service orchestration.

---

### **Mode 2 : "Guided Autonomy"** (Mistral Nemo, Local LLMs)

```yaml
Profils : mistral-nemo, mistral-api, ollama+bridge, lm-studio+bridge
CapacitÃ©s actives : ~85 %
  - Parallel agents âœ…
  - TodoWrite âœ…
  - Terminal âœ… (mais fallback manuel)
  - Multi-file edits âœ…
  - MCP âŒ â†’ SearchAPI fallback
  - Doc verification âš ï¸ â†’ Manual pattern
Workflow : P0 â†’ P3 (auto) â†’ P4 (manual tests) â†’ P5-6 (human)
DurÃ©e T3 : ~45 min (local) / ~1h (API)
CoÃ»ts : Ã‰conomiques (~$0.1-0.3)
DÃ©lÃ©gation : Agent peut tourner en arriÃ¨re-plan (local free)
```

**Quand utilisÃ©** : PME, startups, budget constraints, on-premises requis.

**Excellent tradeoff** : 90 % des capacitÃ©s, 10 % des coÃ»ts.

---

### **Mode 3 : "Assisted Development"** (Copilot gratuit, Llama 7B local, simple CLI)

```yaml
Profils : github-copilot-free, llama7b-cli, aider
CapacitÃ©s actives : ~40-50 %
  - Parallel agents âŒ â†’ Sequential (Loi 9)
  - TodoWrite âŒ â†’ Text announcements
  - Terminal âŒ â†’ Manual (user runs, pastes)
  - Multi-file âŒ â†’ Max 2/iteration
  - MCP âŒ â†’ Offline docs
  - Doc verification âŒ â†’ Memory patterns
Workflow : P0 â†’ P3 (semi-auto) â†’ P4 (manual) â†’ P5 (human) â†’ P6 (human)
DurÃ©e T3 : ~2-3 hours (trÃ¨s itÃ©ratif)
CoÃ»ts : Gratuit
DÃ©lÃ©gation : Non (utilisateur central)
```

**Quand utilisÃ©** : Freelancers, Ã©tudiants, free tier, learning mode.

**Key principle** : "Vous = Copilot du Copilot". L'agent guide, vous exÃ©cutez.

---

### **Mode 4 : "Expert Review + Checker"** (No-code tools, GPT-mini, etc.)

```yaml
Profils : gpt-4o-mini, gemini-flash, deepseek-fast
CapacitÃ©s actives : ~30 %
  - Parallel agents âŒ
  - TodoWrite âŒ
  - Terminal âŒ
  - Multi-file âŒ â†’ 1/iteration
  - MCP âŒ
  - Doc verification âŒ
Workflow : Code â†’ Human writes â†’ LLM reviews â†’ Suggest fix â†’ Iterate
DurÃ©e T3 : ~4-6 hours (vous coder, IA check)
CoÃ»ts : TrÃ¨s bas (~$0.01-0.05)
DÃ©lÃ©gation : Non (human-centric)
```

**Quand utilisÃ©** : Code review, junior dev pairing, validation point.

---

### **Mode 5 : "Offline-Capable" (Llama 3.1 local, NO api)** 

```yaml
Profils : ollama-llama3.1, lm-studio offline
CapacitÃ©s actives : ~50 % (dÃ©pend modÃ¨le)
  - Parallel agents âš ï¸ (CPU-bound)
  - TodoWrite âš ï¸
  - Terminal âœ… (via shell pipe)
  - Multi-file âš ï¸ (lent)
  - MCP âŒ (fully local)
  - Doc verification âŒ
Workflow : P0 â†’ P3 (slow) â†’ P4-5-6 (manual)
DurÃ©e T3 : ~3-4 hours (CPU bottleneck)
CoÃ»ts : $0 (aprÃ¨s modÃ¨le)
DÃ©lÃ©gation : âœ… (24/7 background, autonomy LOI-1)
```

**Quand utilisÃ©** : 
- Autonomy-first (offline required)
- Production infra (no internet)
- Continuous background inference

**Bonus** : Fonctionne **seul** (pas d'API, pas de quota).

---

## Transposition dans Skills IA

Chaque SKILL.md contient directives adaptativas :

```markdown
## Comment utiliser ce skill selon votre LLM

### Mode 1 (Claude Code / Opus)
- Utilisez Function Calling natif
- Appelez `multi_replace_string_in_file` directement
- `Parallel agents` via subagents

### Mode 2 (Mistral / Local LLM)
- Utilisez Function Calling (JSON Schema)
- Groupez edits via `multi_replace_string_in_file`
- Agents en parallÃ¨le possible

### Mode 3 (Copilot Free)
- Pas de TodoWrite
- Terminal via utilisateur
- Fichier par fichier (max 2)

### Mode 4 (GPT-mini)
- GÃ©nÃ©rez changements en code block
- Utilisateur exÃ©cute manuellement
- Focus sur review

### Mode 5 (Offline Llama)
- Changements seulement sur contexte local
- MIP Index consultÃ© hors-ligne
- Shell commands via pipe
```

---

## Workflow de NÃ©gociation (Capability Negotiation)

Ã€ l'initialisation, MIP demande au LLM :

```
Maria (orchestrator) :
  "Quel est votre profil IA ?
   1. Claude Code / Opus (âˆž capacitÃ©s)?
   2. Mistral / Open-Source (90 % capacitÃ©s)?
   3. Copilot Free / Mini (40 % capacitÃ©s)?
   4. Code Reviewer (30 % capacitÃ©s)?
   5. Offline Local (50 %, autonome)?
   
   [Utilisateur ou auto-detect]
   
  âœ… Profil dÃ©tectÃ© : Mistral Nemo
  Adaptation active : Mode 2 (Guided Autonomy)
  
  âš ï¸ Limitations :
   - Web search â†’ SearchAPI fallback
   - MCP âŒ â†’ Offline docs
   - Suggestion : Gardez docs Miyukini locale
   
  PrÃªt? CommenÃ§ons P0."
```

---

## SÃ©lection automatique du Mode

```python
def detect_llm_mode(profile):
    """Auto-select adaptation mode based on capabilities"""
    
    if profile.capabilities.all_true():
        return "Mode1_Autonomy_Complete"
    
    if profile.missing in ["mcp"]:
        return "Mode2_Guided_Autonomy"
    
    if profile.missing in ["parallel", "todo_write", "terminal"]:
        return "Mode3_Assisted_Development"
    
    if profile.missing in ["multi_file", "doc_verification"]:
        return "Mode4_Expert_Review"
    
    if profile.endpoint == "localhost":
        return "Mode5_Offline_Capable"
    
    return "Mode3_Assisted_Development"  # Fallback
```

---

## Matrice : Quand utiliser quel mode

| Task Class | Mode 1 | Mode 2 | Mode 3 | Mode 4 | Mode 5 |
|-----------|--------|--------|--------|--------|--------|
| **T1** (< 20 lines) | âœ… 2min | âœ… 5min | âœ… 15min | âœ… 30min | âœ… 10min |
| **T2** (1-3 files) | âœ… 5min | âœ… 15min | âš ï¸ 1h | âš ï¸ 2h | âš ï¸ 1h30 |
| **T3** (3-10 files) | âœ… 15min | âœ… 45min | âŒ 3-4h | âŒ 6h | âš ï¸ 4h |
| **T4** (10-30 files) | âœ… 30min | âš ï¸ 2h | âŒ 10h+ | âŒ 20h+ | âŒ 12h+ |
| **T5** (Strategic) | âœ… 1-2h | âš ï¸ 4-8h | âŒ Days | âŒ N/A | âŒ N/A |

**Legend** :
- âœ… Excellent fit (productif)
- âš ï¸ Possible mais friction (trÃ¨s itÃ©ratif)
- âŒ Not recommended (trop lent ou pas de sens)

---

## Cas rÃ©alistes

### Case 1 : Total Engineer (Copilot gratuit)

```
Profil : github-copilot-free â†’ Mode 3
Task : T2 (fix bug dans service)
StratÃ©gie :
  1. DÃ©crire bug (chat Copilot)
  2. Copilot suggÃ¨re changements (1-2 fichiers)
  3. Vous exÃ©cutez : `cargo test -p service`
  4. Copilot lit errors, itÃ¨re
DurÃ©e : ~1h (pas grave, vous pair-program)
CoÃ»ts : $0
```

### Case 2 : Admin freelance (Mistral Local)

```
Profil : mistral-nemo + LM Studio local â†’ Mode 2
Task : T3 (nouvelle API endpoint)
StratÃ©gie :
  1. DÃ©crire feature (MIP P0)
  2. Mistral lance agents parallÃ¨les
        - Backend : API lib
        - Frontend : client wrapper
  3. Mistral run tests (terminal)
  4. Report & merge
DurÃ©e : ~45min (8GB RAM, local GPU)
CoÃ»ts : $0 (amortissement HW)
```

### Case 3 : Startup lead (Claude Code)

```
Profil : anthropic-opus â†’ Mode 1
Task : T4 (multi-service refactor)
StratÃ©gie :
  1. Framing (MIP P0)
  2. Agents parallÃ¨les :
        - Lise : UI refactor
        - FranÃ§ois : Backend schema change
        - Victor : Security audit
  3. Auto-integration P4-5-6
DurÃ©e : ~1h autorisÃ©e, human review 30min
CoÃ»ts : ~$2
```

---

## Roadmap MIP Adaptivity v2

- [ ] Auto-detect LLM from IDE (Cursor, VS Code, etc.)
- [ ] Dynamic fallback (SearchAPI, offline cache)
- [ ] Degradation graceful (retry with fewer parallel agents)
- [ ] Cost projection before task start
- [ ] Mode suggestion by task class
- [ ] Skills regeneration per mode (optimize prompts)

---

## Docs de rÃ©fÃ©rence

- [Capacities Matrix](..//README.md) â€” Par outil
- [Profiles INDEX](..//README.md) â€” Basculer profils
- [Subscriptions](../config/subscriptions.md) â€” Budget token
- [MIP Workflow](../protocol/conventions.md) â€” P0-P6 phases

