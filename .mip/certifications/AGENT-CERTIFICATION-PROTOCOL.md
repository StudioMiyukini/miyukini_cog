---
id: mip.certifications.agent-protocol
title: Agent Certification Protocol â€” Certifier agents pour tous les modes
---

# Agent Certification Protocol

> **Context** : Un agent IA peut fonctionner excellemment en Mode 1 (Claude Code pleine autonomie) mais Ã©chouer en Mode 3 (Copilot, trÃ¨s limitÃ©). MIP doit certifier chaque agent par **mode LLM** et **task class** (T1-T5).
> **Version decoupee** : `.mip/certifications/agent-certification-protocol/INDEX.md`

---

## Vision

```
Avant:
  Maria = "Je peux faire P0-P6 orchestration"
  â†’ Ã‰choue en Mode 3 (Context 8k insuffisant)
  â†’ Utilisateur frustrÃ©

AprÃ¨s:
  Maria = "âœ… Certified Mode 1-2-5 | âš ï¸ Testing Mode 3 (T2 max) | âŒ Skip Mode 4"
  â†’ Utilisateur sait exactement ce qui fonctionne
  â†’ MIP alerte si tÃ¢che > capacity
  â†’ Fallback automatique ou human ask
```

---

## Structure Certifications

### Fichier principale

```
.mip/certifications/
â”œâ”€â”€ README.md (overview)
â”œâ”€â”€ AGENT-CERTIFICATION-PROTOCOL.md (source monolithique)
â”œâ”€â”€ MATRIX.md (rÃ©sumÃ© toutes agents Ã— modes)
â”œâ”€â”€ agents/
â”‚   â”œâ”€â”€ maria.md (Orchestration)
â”‚   â”œâ”€â”€ lise.md (Frontend)
â”‚   â”œâ”€â”€ francois.md (Backend)
â”‚   â”œâ”€â”€ victor.md (Security)
â”‚   â”œâ”€â”€ george.md (Compliance)
â”‚   â”œâ”€â”€ hugo.md (DevOps)
â”‚   â”œâ”€â”€ jean.md (Efficiency)
â”‚   â””â”€â”€ arianne.md (QA/Memory)
â””â”€â”€ skills/
    â”œâ”€â”€ miyukini-mip-workflow-modes.md
    â”œâ”€â”€ miyukini-architecture-modes.md
    â””â”€â”€ ... (1 per skill, mode-specific tests)
```

---

## Certification Levels

Pour chaque **Agent** Ã— **Mode** Ã— **Task Class** :

| Level | Status | Meaning |
|-------|--------|---------|
| âœ… **Certified** | Ready | Tested, passes all checks, production-ready |
| âš ï¸ **Testing** | In progress | Works but needs monitoring/limits |
| ðŸ”§ **Limited** | Restricted | Works but with constraints (e.g., T2 max) |
| âŒ **Not certified** | Unsupported | Doesn't work, skip or fallback |

---

## Certification Checklist

Pour certifier Agent X en Mode Y, faire :

### Step 1 : Capability mapping

```yaml
Agent: Maria (Orchestration, P0 + decisions)

Mode 1 capabilities:
  - Text analysis âœ… (native)
  - Decision making âœ… (reasoning)
  - Context assembly âœ… (200k tokens)
  - Parallel direction âœ… (subagents)
  - Terminal output read âœ… (MCP)
  
Mode 3 capabilities:
  - Text analysis âœ… (native)
  - Decision making âœ… (reasoning)
  - Context assembly âš ï¸ (8k limit!)
  - Parallel direction âŒ (sequential)
  - Terminal output read âŒ (no MCP)
```

### Step 2 : Task class testing

Test agent in each **task class** (T1-T5) :

**Template** :

```yaml
Agent: Maria
Mode: 3 (Copilot)

T1 (micro, <20 lines):
  Scenario: "Code style fix"
  Test: âœ… PASS
  Notes: "Instant, 0 context issues"
  Status: âœ… Certified

T2 (targeted, 1-3 files):
  Scenario: "Add validation function"
  Test: âœ… PASS
  Notes: "Tight context (5k/8k), needs summaries"
  Status: ðŸ”§ Certified (with summary prep)

T3 (moderate, 3-10 files):
  Scenario: "Multi-file refactor"
  Test: âŒ FAIL
  Error: "Context exceeded, can't see all files"
  Workaround: "Pre-read MIP Index, summarize 1 file at a time"
  Status: âš ï¸ Testing

T4 (major, 10+ files):
  Test: âŒ FAIL (expected)
  Status: âŒ Not supported
  Fallback: "Discover T2â†’T3 sub-tasks"

T5 (strategic):
  Test: âŒ FAIL (expected)
  Status: âŒ Not supported
  Fallback: "Upgrade to Mode 1"
```

### Step 3 : Skill certification

Test agent with each **skill** in that mode :

```yaml
Agent: Maria
Mode: 3
Skill: miyukini-mip-workflow

Procedure tested:
  âœ… P0 framing (text-based, short)
  âŒ P3 auto-execution (sequential, no parallel)
  âš ï¸  P4 testing (manual terminal)
  âœ… P5 validation (text)
  âœ… P6 archiving (text)

Issues:
  - P3 needs user permission per file edit
  - Tests must be manual (no terminal)

Workaround:
  - Add "PrÃªt pour P3-2?" checkpoints
  - Manual test runs between P3 iterations

Certification: âœ… Certified (with Mode 3 adaptations)
```

### Step 4 : Limitations & constraints

Document exactement **oÃ¹** l'agent casse :

```yaml
Agent: Maria
Mode: 3 (Copilot gratuit)

Hard limits:
  - Context: 8k max (absolute)
  - Files per iteration: 1 (readability)
  - Parallel agents: 0 (sequential)
  - Terminal access: 0 (manual)

Soft limits:
  - RecommandÃ© T2 max (T3 possible mais friction)
  - Estimation time 4x (sÃ©quentiel)
  - Requires user central (pair coding)

Fallback strategy:
  Task too big? â†’ "DÃ©couper en T2 multi-task"
  Context full? â†’ "Pre-load MIP Index JSON"
  Terminal needed? â†’ "Utilisateur exÃ©cute + paste"
```

### Step 5 : Validation & sign-off

```yaml
Status: âœ… CERTIFIED for Mode 3

Tested by: Development team
Test date: 2026-03-05
Expires: 2026-06-05 (quarterly review)

Sign-off:
  - Agent: Maria
  - Supervisor: Victor (security/compliance)
  - Manager: Hugo (operations)

Next steps:
  - [ ] Update agent/maria.md
  - [ ] Update MATRIX.md
  - [ ] Alert users to new available profile
  - [ ] Add to deployment checklist
```

---

## Agent Certification Template

Create `.mip/certifications/agents/{name}.md` :

```markdown
---
id: mip.cert.agent.{name}
agent_name: {Name}
role: {Role}
certifications:
  - mode_1_full: certified (2026-02-15)
  - mode_2_mistral: certified (2026-02-20)
  - mode_3_copilot: limited_t2_only (2026-03-01)
  - mode_4_gpt_mini: not_applicable
  - mode_5_offline: testing
---

# {Name} â€” Agent Certification Record

**Role** : {Role}  
**Last Updated** : 2026-03-05

---

## Certification Summary

| Mode | T1 | T2 | T3 | T4 | T5 | Notes |
|------|----|----|----|----|-----|-------|
| M1 (Claude) | âœ… | âœ… | âœ… | âœ… | âœ… | Full autonomous |
| M2 (Mistral) | âœ… | âœ… | âœ… | âš ï¸ | âŒ | Limited parallelism |
| M3 (Copilot) | âœ… | ðŸ”§ | âŒ | âŒ | âŒ | Manual tests |
| M4 (GPT-mini) | â€” | â€” | â€” | â€” | â€” | Not applicable |
| M5 (Offline) | âœ… | âœ… | âš ï¸ | âŒ | âŒ | CPU-bound |

---

## Mode 1 : Claude Code (Certified âœ…)

**Status** : Production-ready since 2026-02-15

**Capabilities** :
- All task classes T1-T5 âœ…
- All skills supported âœ…
- No constraints

**Skills tested** : [list 20+ skills]

**Known issues** : None

**Review schedule** : Quarterly (expires 2026-05-15)

---

## Mode 2 : Mistral Nemo (Certified âœ…)

**Status** : Production-ready since 2026-02-20

**Capabilities** :
- T1-T2 âœ… (full)
- T3 âœ… (with patience, CPU-bound)
- T4 âš ï¸ (possible but slow, parallelism limited)
- T5 âŒ (not recommended, no web search)

**Skills tested** : [list skills]

**Known issues** :
- #1 : MCP unavailable â†’ Use SearchAPI fallback
- #2 : CPU-bound (1 tok/sec local)

**Workaround** :
- #1 : Pre-load offline docs
- #2 : Parallel agents run slower

**Review schedule** : Quarterly (expires 2026-05-20)

---

## Mode 3 : GitHub Copilot Gratuit (Limited ðŸ”§)

**Status** : Testing, limited to T2

**Certification Date** : 2026-03-05  
**Expires** : 2026-06-05

**Capabilities** :
- T1 âœ… (excellent, ~5 min)
- T2 ðŸ”§ (good but needs prep, ~45 min)
- T3-T5 âŒ (context too small, sequential)

**Hard constraints** :
- Context : 8k max
- Files per iteration : 1
- Terminal : Manual (user runs)
- Parallel agents : None (Loi 9 suspended)

**Skills tested** :
- âœ… mip-workflow (adapted)
- âœ… architecture (summary mode)
- âœ… cargo-workspace (T2 max)
- âŒ multi-service (too big)

**Known issues** :
- #1 : Context exhaustion on T3+ â†’ Must discover T2 sub-tasks
- #2 : No auto-testing â†’ Manual cargo test required
- #3 : No web search â†’ Use offline docs

**Workarounds** :
- #1 : Read MIP Index JSON before file reads
- #2 : User executes tests, pastes output
- #3 : Keep README offline, search local grep

**Recommended task allocation** :
```
Type: micro-fix, T1 solo
Ideal: Daily pair session (Copilot + you)
Timing: 3-4 T2 per day max (rest context for next day)
Review: Human review every 3 hours (context reset)
```

**Limitations summary** :
```
"Mode 3 = constrained but viable for T1-T2. 
 Human stays central (you = main coder, Copilot guides).
 Expect 3-4x slower than Mode 1, but â‚¬0 cost."
```

**Testing results** :

| Test | Result | Time | Notes |
|------|--------|------|-------|
| T1-style-fix | âœ… PASS | 5 min | Fast, trivial |
| T2-validator | âœ… PASS (adapted) | 45 min | Needs file summaries |
| T3-refactor | âŒ FAIL | â€” | Context overflow |

**Transition plan** (from Mode 3 to Mode 1) :

When should users upgrade Copilot to Claude Code?
- Task hits T3+ ?
- Budget allows (+â‚¬2-5) ?
- Timeline urgent (<1 week) ?

If yes â†’ Switch to Mode 1, re-run from P0.

---

## Mode 5 : Offline Llama (Testing âš ï¸)

**Status** : Testing, limited parallelism

**Certification Date** : In progress (target 2026-06-05)

**Capabilities** :
- T1-T2 âœ… (works, slow)
- T3 âš ï¸ (possible but CPU-bottleneck)
- T4-T5 âŒ (not recommended)

**Hard constraints** :
- Context : 128k (depends LLM)
- Speed : 1 token/sec (CPU-bound)
- Web search : âŒ (fully offline)
- Parallelism : âš ï¸ (CPU contention)

**Testing in progress** :
- [ ] Verify T2 performance on M1 (target < 15 min)
- [ ] Verify offline docs sufficiency (no web)
- [ ] Benchmark CPU + memory usage
- [ ] Test disk caching (inference cache)

**Issues encountered** :
- #1 : CPU spikes cause context thrashing
- #2 : Semantic search too slow (local embedding)
- #3 : Context window variable per GGUF

**Workarounds being tested** :
- #1 : Limit parallel agents to 1-2 max
- #2 : Use grep instead of semantic search
- #3 : Standardize on llama3.1 contexts.bin

**Target use cases for Mode 5** :
- Offline-first production (no internet access)
- 24/7 background inference (long-running P3)
- Educational/open-source (zero API cost)

**Review schedule** : Bi-weekly (active testing)

---

## Summary

Agent: {Name}

Recommended deployments by Mode:
- **Mode 1** : âœ… Use fully
- **Mode 2** : âœ… Use fully
- **Mode 3** : ðŸ”§ T1-T2 only, user-centric
- **Mode 4** : âŒ Not applicable
- **Mode 5** : âš ï¸ Testing, T2 max

Questions?
- Is your task T1-T2? â†’ Try Mode 3 Copilot
- Is your task T3+? â†’ Use Mode 1-2
- No internet access? â†’ Mode 5 offline (test)

---

## Change log

| Date | Change | Version |
|------|--------|---------|
| 2026-02-15 | Initial Mode 1 certification | v1.0 |
| 2026-02-20 | Mode 2 (Mistral) certified | v1.1 |
| 2026-03-01 | Mode 3 (Copilot) limited cert | v1.2 |
| 2026-03-05 | Mode 5 (Offline) testing | v1.3 |
```

---

## Certification Matrix (All Agents)

Create `.mip/certifications/MATRIX.md`:

```markdown
# Agent Ã— Mode Certification Matrix

Last updated: 2026-03-05

## Overview

| Agent | Role | M1 | M2 | M3 | M4 | M5 |
|-------|------|----|----|----|----|-----|
| **Maria** | Orchestration | âœ… | âœ… | ðŸ”§T2 | âŒ | âš ï¸ |
| **Lise** | Frontend (Dioxus UI) | âœ… | âœ… | ðŸ”§T2 | âš ï¸ | âœ… |
| **FranÃ§ois** | Backend (spec + impl) | âœ… | âœ… | âœ… | âŒ | âœ… |
| **Victor** | Security audit | âœ… | âœ… | âš ï¸ | âŒ | âœ… |
| **George** | Compliance | âœ… | âœ… | âœ… | âœ… | âš ï¸ |
| **Hugo** | DevOps/infra | âœ… | âš ï¸ | âŒ | âŒ | âŒ |
| **Jean** | Token efficiency | âœ… | âœ… | âœ… | âš ï¸ | âœ… |
| **Arianne** | QA + Memory | âœ… | âœ… | âœ… | âœ… | âœ… |

## Legend

- âœ… Certified (production-ready)
- âš ï¸ Testing (use with monitoring)
- ðŸ”§ Limited (with constraints)
- âŒ Not certified (skip or fallback)

## Details (click agent)
- [Maria](README.md)
- [Lise](README.md)
- ... (etc)

## By Mode

### Mode 1 (Claude Code)
- Fully certified: All agents
- Status: Production, 2+ months stable

### Mode 2 (Mistral)
- Fully certified: 7/8 agents (Hugo limited on DevOps)
- Status: Production, 2 weeks stable

### Mode 3 (Copilot)
- Fully certified: FranÃ§ois, George, Jean, Arianne (text-centric)
- Limited: Maria (T2 max), Lise (T2 max, UI-specific)
- Not certified: Hugo (needs full terminal)
- Status: Testing phase 1, 1 week

### Mode 4 (GPT-mini)
- Applicable: George (compliance review), Arianne (QA)
- Not applicable: Others (code execution needed)
- Status: Not tested

### Mode 5 (Offline)
- Certified: FranÃ§ois, Jean, Arianne
- Testing: Maria (P0 text-only), Lise (simple components)
- Not certified: Hugo (needs internet infra), Victor (needs web recon)
- Status: Active testing

---

## Upgrade path (when to change modes)

User in Mode 3 (Copilot gratuit), hits T3:

```
Maria: "T3 trop big pour Mode 3.
       Options?
       1. Upgrade Ã  Mode 2 (Mistral) : â‚¬0-0.15 par task
       2. Upgrade Ã  Mode 1 (Claude) : â‚¬5-10 par task
       3. DÃ©couper en 3 T2 : â‚¬0 mais 3x temps
       
       Votre choix?"
```

---

## Review schedule

All certifications reviewed quarterly:
- Q1 (Mar 5) : Mode 1-2 stable, Mode 3 testing
- Q2 (Jun 5) : Mode 3 certified or deprecated, Mode 5 target
- Q3 (Sep 5) : Full matrix stabil
- Q4 (Dec 5) : New agents or mode retire
```

---

## Deployment Integration

When user selects profile (SETUP-4):

```
Maria: "Profile mistral-nemo (Mode 2) sÃ©lectionnÃ©

Agent availability in Mode 2:
  âœ… Toutes les agents certifiÃ©es
  âš ï¸  Hugo (DevOps) = limitÃ© (peut causer delays)
  
Recommendations:
  - T1-T2-T3: Full team, go
  - T4 (needs DevOps): Manual Hugo + Claude P4
  
Ready?"
```

---

## Documentation

- [ADAPTIVE-MODES.md](../profiles/ADAPTIVE-MODES.md) â€” 5 modes overview
- [Industrial Scenarios](../usecases/INDUSTRIAL-SCENARIOS.md) â€” Real cases
- [Skills Adaptation](../skills/ADAPTATION-DIRECTIVE.md) â€” Skill-specific tests
- Each agent: `.mip/certifications/agents/{name}.md`


