---
id: mip.profiles.adaptive-modes
title: MIP Adaptive Modes — 5 modes d'exécution selon LLM
---

# MIP Adaptive Modes (Exécution adaptée au LLM)

> Selon le profil LLM actif, MIP bascule automatiquement entre 5 **modes d'exécution** réduisant progressivement les capacités tout en restant productif.

---

## Les 5 modes d'adaptation

### **Mode 1 : "Autonomy Complete"** (Claude Code, Opus)

```yaml
Profils : anthropic-opus, cursor-composer 1.5, codex
Capacités actives : 100 %
  - Parallel agents (MASS)
  - TodoWrite orchestration
  - Terminal + Background jobs
  - Multi-file edits
  - MCP + web search
  - Doc verification (auto resolution)
Workflow : P0 → P3 (auto) → P4-5-6 (auto)
Durée T3 : ~15-30 min
Coûts : Élevés (~$1-3)
```

**Quand utilisé** : R&D de pointe, critical path, multi-service orchestration.

---

### **Mode 2 : "Guided Autonomy"** (Mistral Nemo, Local LLMs)

```yaml
Profils : mistral-nemo, mistral-api, ollama+bridge, lm-studio+bridge
Capacités actives : ~85 %
  - Parallel agents ✅
  - TodoWrite ✅
  - Terminal ✅ (mais fallback manuel)
  - Multi-file edits ✅
  - MCP ❌ → SearchAPI fallback
  - Doc verification ⚠️ → Manual pattern
Workflow : P0 → P3 (auto) → P4 (manual tests) → P5-6 (human)
Durée T3 : ~45 min (local) / ~1h (API)
Coûts : Économiques (~$0.1-0.3)
Délégation : Agent peut tourner en arrière-plan (local free)
```

**Quand utilisé** : PME, startups, budget constraints, on-premises requis.

**Excellent tradeoff** : 90 % des capacités, 10 % des coûts.

---

### **Mode 3 : "Assisted Development"** (Copilot gratuit, Llama 7B local, simple CLI)

```yaml
Profils : github-copilot-free, llama7b-cli, aider
Capacités actives : ~40-50 %
  - Parallel agents ❌ → Sequential (Loi 9)
  - TodoWrite ❌ → Text announcements
  - Terminal ❌ → Manual (user runs, pastes)
  - Multi-file ❌ → Max 2/iteration
  - MCP ❌ → Offline docs
  - Doc verification ❌ → Memory patterns
Workflow : P0 → P3 (semi-auto) → P4 (manual) → P5 (human) → P6 (human)
Durée T3 : ~2-3 hours (très itératif)
Coûts : Gratuit
Délégation : Non (utilisateur central)
```

**Quand utilisé** : Freelancers, étudiants, free tier, learning mode.

**Key principle** : "Vous = Copilot du Copilot". L'agent guide, vous exécutez.

---

### **Mode 4 : "Expert Review + Checker"** (No-code tools, GPT-mini, etc.)

```yaml
Profils : gpt-4o-mini, gemini-flash, deepseek-fast
Capacités actives : ~30 %
  - Parallel agents ❌
  - TodoWrite ❌
  - Terminal ❌
  - Multi-file ❌ → 1/iteration
  - MCP ❌
  - Doc verification ❌
Workflow : Code → Human writes → LLM reviews → Suggest fix → Iterate
Durée T3 : ~4-6 hours (vous coder, IA check)
Coûts : Très bas (~$0.01-0.05)
Délégation : Non (human-centric)
```

**Quand utilisé** : Code review, junior dev pairing, validation point.

---

### **Mode 5 : "Offline-Capable" (Llama 3.1 local, NO api)** 

```yaml
Profils : ollama-llama3.1, lm-studio offline
Capacités actives : ~50 % (dépend modèle)
  - Parallel agents ⚠️ (CPU-bound)
  - TodoWrite ⚠️
  - Terminal ✅ (via shell pipe)
  - Multi-file ⚠️ (lent)
  - MCP ❌ (fully local)
  - Doc verification ❌
Workflow : P0 → P3 (slow) → P4-5-6 (manual)
Durée T3 : ~3-4 hours (CPU bottleneck)
Coûts : $0 (après modèle)
Délégation : ✅ (24/7 background, autonomy LOI-1)
```

**Quand utilisé** : 
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
- Agents en parallèle possible

### Mode 3 (Copilot Free)
- Pas de TodoWrite
- Terminal via utilisateur
- Fichier par fichier (max 2)

### Mode 4 (GPT-mini)
- Générez changements en code block
- Utilisateur exécute manuellement
- Focus sur review

### Mode 5 (Offline Llama)
- Changements seulement sur contexte local
- MIP Index consulté hors-ligne
- Shell commands via pipe
```

---

## Workflow de Négociation (Capability Negotiation)

À l'initialisation, MIP demande au LLM :

```
Maria (orchestrator) :
  "Quel est votre profil IA ?
   1. Claude Code / Opus (∞ capacités)?
   2. Mistral / Open-Source (90 % capacités)?
   3. Copilot Free / Mini (40 % capacités)?
   4. Code Reviewer (30 % capacités)?
   5. Offline Local (50 %, autonome)?
   
   [Utilisateur ou auto-detect]
   
  ✅ Profil détecté : Mistral Nemo
  Adaptation active : Mode 2 (Guided Autonomy)
  
  ⚠️ Limitations :
   - Web search → SearchAPI fallback
   - MCP ❌ → Offline docs
   - Suggestion : Gardez docs Miyukini locale
   
  Prêt? Commençons P0."
```

---

## Sélection automatique du Mode

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
| **T1** (< 20 lines) | ✅ 2min | ✅ 5min | ✅ 15min | ✅ 30min | ✅ 10min |
| **T2** (1-3 files) | ✅ 5min | ✅ 15min | ⚠️ 1h | ⚠️ 2h | ⚠️ 1h30 |
| **T3** (3-10 files) | ✅ 15min | ✅ 45min | ❌ 3-4h | ❌ 6h | ⚠️ 4h |
| **T4** (10-30 files) | ✅ 30min | ⚠️ 2h | ❌ 10h+ | ❌ 20h+ | ❌ 12h+ |
| **T5** (Strategic) | ✅ 1-2h | ⚠️ 4-8h | ❌ Days | ❌ N/A | ❌ N/A |

**Legend** :
- ✅ Excellent fit (productif)
- ⚠️ Possible mais friction (très itératif)
- ❌ Not recommended (trop lent ou pas de sens)

---

## Cas réalistes

### Case 1 : Total Engineer (Copilot gratuit)

```
Profil : github-copilot-free → Mode 3
Task : T2 (fix bug dans service)
Stratégie :
  1. Décrire bug (chat Copilot)
  2. Copilot suggère changements (1-2 fichiers)
  3. Vous exécutez : `cargo test -p service`
  4. Copilot lit errors, itère
Durée : ~1h (pas grave, vous pair-program)
Coûts : $0
```

### Case 2 : Admin freelance (Mistral Local)

```
Profil : mistral-nemo + LM Studio local → Mode 2
Task : T3 (nouvelle API endpoint)
Stratégie :
  1. Décrire feature (MIP P0)
  2. Mistral lance agents parallèles
        - Backend : API lib
        - Frontend : client wrapper
  3. Mistral run tests (terminal)
  4. Report & merge
Durée : ~45min (8GB RAM, local GPU)
Coûts : $0 (amortissement HW)
```

### Case 3 : Startup lead (Claude Code)

```
Profil : anthropic-opus → Mode 1
Task : T4 (multi-service refactor)
Stratégie :
  1. Framing (MIP P0)
  2. Agents parallèles :
        - Lise : UI refactor
        - François : Backend schema change
        - Victor : Security audit
  3. Auto-integration P4-5-6
Durée : ~1h autorisée, human review 30min
Coûts : ~$2
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

## Docs de référence

- [Capacities Matrix](./capabilities-matrix.md) — Par outil
- [Profiles INDEX](./INDEX.md) — Basculer profils
- [Subscriptions](../config/subscriptions.md) — Budget token
- [MIP Workflow](../protocol/conventions.md) — P0-P6 phases
