---
id: mip.protocol.degradation-fallback
title: Graceful Degradation & Fallback Strategy
---

# Graceful Degradation & Fallback Strategy

> Quand une capacitÃ© n'est pas disponible, MIP ne crash pas. Au lieu de Ã§a, il **nÃ©gocie** avec l'utilisateur, propose des **fallbacks**, et donne un **mode dÃ©gradÃ©** viable.

---

## Principes

```
Principle 1: Transparency
  "Je n'ai pas X, voici ce que j'utilise Ã  la place"

Principle 2: Choice
  "PrÃ©fÃ©rez-vous A, B, ou C ?"

Principle 3: Viability
  "Mode dÃ©gradÃ© = plus lent, mais fonctionnel"

Principle 4: Automation
  "Si fallback unique, appliquer silencieusement"
```

---

## Fallback Chains (par capacitÃ©)

### CapacitÃ© 1 : Terminal execution

**Hierarchy** :
```
1. run_in_terminal â† native (Mode 1-2-5)
2. Utilisateur execute + paste â† manual (Mode 3)
3. Instructions texte (do X, Y, Z) â† fallback (Mode 4)
4. Skip (impossible) â† skip
```

**Runtime decision** :

```
Agent need: execute `cargo build`

If run_in_terminal available:
  â†’ Use it (native)
  
Else if user-central workflow:
  â†’ Ask user:
     "ExÃ©cutez : cargo build
      Collez rÃ©sultat ci-dessous"
  â†’ Wait for input
  
Else if instruction-only possible:
  â†’ "Pour tester:
      1. Open terminal
      2. Type: cargo build
      3. If errors, report back"
  
Else:
  â†’ "Tests not automated in Mode 4.
      SuggÃ©ration: Upgrade to Mode 2"
  â†’ Skip test

Fallback applied, continue
```

---

### CapacitÃ© 2 : Web search / MCP

**Hierarchy** :
```
1. MCP web search (native, Mode 1-2)
2. SearchAPI SDK (fallback, Mode 2-3)
3. DuckDuckGo shell curl (degraded, Mode 3-5)
4. Offline markdown docs (local, Mode 5)
5. Skip + use memory/pattern (none)
```

**Runtime decision** :

```
Agent need: "Find Tauri event handling patterns"

If MCP available:
  â†’ query("Tauri 1.x event patterns")
  â†’ Get sources
  â†’ Analyze
  
Else if SearchAPI configured:
  â†’ Call SearchAPI.io
  â†’ Parse results
  â†’ Analyze
  
Else if shell available:
  â†’ run_in_terminal("curl -s 'duckduckgo.com?q=...'")
  â†’ Parse HTML
  â†’ Extract links
  
Else if offline docs exist:
  â†’ read ~/.miyukini_cache/tauri_docs.md
  â†’ grep "event"
  â†’ Use local knowledge
  
Else:
  â†’ "No web search available.
      Using pattern memory:
      Tauri events typically use
      invoke + listen pattern..."
  â†’ Proceed with assumption

Fallback applied, continue
```

---

### CapacitÃ© 3 : Parallel agents

**Hierarchy** :
```
1. Parallel subagents (native, Mode 1-2)
2. Sequential agents (degraded, Mode 3-5)
3. Single agent + instructions (minimal, Mode 4)
4. Human coordination (manual, Mode 4)
```

**Runtime decision** :

```
Task: T3 (frontend + backend + tests parallel)

If parallel_agents available:
  â†’ Launch 3 agents simultaneously
  â†’ Overlap work ~60%
  â†’ Total time: 1h
  
Else if agents available:
  â†’ Agent 1: Frontend (30 min)
  â†’ Agent 2: Backend (30 min)
  â†’ Sequential, non-overlapping
  â†’ Total time: 60 min
  â†’ Tell user: "Sequential mode, slower"
  
Else if single agent only:
  â†’ One agent: Frontend (30 min)
  â†’ One agent: Backend (30 min) 
  â†’ One agent: Tests (20 min)
  â†’ Total: 80 min
  
Else:
  â†’ "No agents. Manual mode:
      Instructions file generated,
      you do frontend,
      you do backend,
      you run tests"
  â†’ Generate code blocks

User notified of fallback, adjust expectations
```

---

### CapacitÃ© 4 : Multi-file edits

**Hierarchy** :
```
1. multi_replace_string_in_file (parallel, Mode 1-2)
2. Sequential replace_string_in_file (iterative, Mode 3)
3. Code blocks + manual paste (manual, Mode 4-5)
4. Verbal instructions (degraded, Mode 4)
```

**Runtime decision** :

```
Task: Edit 5 files simultaneously

If multi_replace available:
  â†’ 5 edits in 1 call
  â†’ All applied simultaneously
  
Else if replace_string available:
  Iteration 1:
    â†’ Edit file 1
    â†’ Ask: "Applied ? Y/N"
  Iteration 2:
    â†’ Edit file 2
    â†’ Ask: "Applied ? Y/N"
  [etc x5]
  
Else:
  â†’ Generate code block for each file:
     "FILE 1: src/lib.rs
      OLD: ...
      NEW: ...
      
      FILE 2: src/main.rs
      OLD: ...
      NEW: ..."
  â†’ "Copy-paste each into editor"
  
User applies manually, confirm

Run tests to verify
```

---

### CapacitÃ© 5 : TodoWrite (task tracking)

**Hierarchy** :
```
1. manage_todo_list (native, Mode 1-2)
2. Text announcements (degraded, Mode 3-5)
3. Checkbox markdown (minimal, offline)
4. No tracking (skip)
```

**Runtime decision** :

```
Workflow: Multi-phase task (P0â†’P3â†’P4â†’P5â†’P6)

If manage_todo_list available:
  â†’ Create 6-item list
  â†’ Update in real-time
  â†’ User sees progress in UI
  
Else if text output available:
  â†’ Announce at each phase:
     "ðŸ“Œ P0 âœ…"
     "ðŸ”„ P3-1/3..."
     "â³ P4 tests..."
     "âœ… P5 done"
  â†’ User tracks mentally
  
Else:
  â†’ Markdown checklist:
     "- [ ] P0 Framing
      - [ ] P3 Implementation
      - [x] P4 Tests
      - [ ] P5 Delivery"
  â†’ User clicks checkboxes in editor
  
Fallback applied, less automated tracking
```

---

### CapacitÃ© 6 : Context window

**Strategy** : Aggressive reduction

**Hierarchy** :
```
1. Full context (200k tokens, Mode 1-2)
2. Summarized context (50k tokens, Mode 3)
3. Indexed context (JSON Only, Mode 5)
4. Manual context (user specifies)
```

**Runtime decision** :

```
Task scope: 10 crates, ~50k LOC

If context 200k:
  â†’ Read all 10 crates
  â†’ Full semantic analysis
  
Else if context ~50k:
  â†’ Read MIP Index blocks.json
  â†’ Identify 3 critical files
  â†’ Full read those
  â†’ Summary read others
  â†’ Hybrid approach
  
Else if context ~10-20k:
  â†’ Read only MSCM tags (500 bytes each)
  â†’ Locate target file
  â†’ Read only target
  â†’ Single-file focus
  
Else:
  â†’ User: "Which file to start with?"
  â†’ User supplies file + line range
  â†’ Read only supplied range

Fallback applied, narrower scope
```

---

## Graceful Degradation Paths

### Path 1 : Terminal needed but unavailable

```
Scenario: Mode 3 (Copilot), need build + test

Plan A (best):
  User runs: `cargo test -p X`
  User pastes output
  Agent analyzes
  â†’ Continues
  
Plan B (acceptable):
  Agent suggests: "Run these steps"
  User mentally notes them
  User reports "passed" or "failed"
  â†’ Continues
  
Plan C (minimal):
  Agent assumes: "Tests will pass"
  User does manual test later
  â†’ Risk but continues

Agent tries A, falls back to B, then C if needed
```

### Path 2 : Web search needed but unavailable

```
Scenario: Mode 5 (offline), need API docs

Plan A (best):
  Local markdown docs exist
  Agent reads: ~/.miyukini_cache/tauri.md
  â†’ Continues
  
Plan B (acceptable):
  Agent uses pattern memory:
  "Tauri APIs typically use ..."
  â†’ Proceeds with educated guess
  
Plan C (okay):
  Agent asks user:
  "Do you know Tauri X behavior?"
  User: "Yeah, it's..."
  â†’ Continues
  
Plan D (risky):
  Agent tries offline inference
  (model hallucinates docs)
  â†’ Risk, but might work

Agent tries A, falls back progressively
```

### Path 3 : Parallel agents not available

```
Scenario: Mode 3 (Copilot), T3 needs 3 agents

Plan A (best):
  Discover T2 sub-tasks:
  Task1: T2 frontend
  Task2: T2 backend
  Task3: T2 tests
  
  Do sequential with clear handoffs
  â†’ Slower but viable
  
Plan B (acceptable):
  Single agent does all (Lise):
  Frontend code
  Then backend code
  Then tests
  
  â†’ Takes 3x longer
  â†’ Risk of inconsistency
  
Plan C (fallback):
  Ask user:
  "Can you code frontend?
   I'll do backend + tests"
  
  â†’ Hybrid approach
  â†’ Better in some cases

Agent picks best path for mode + task
```

---

## Fallback Negotiation (Interactive)

When fallback needed, ask user :

```
MIP: "I need web search, but no MCP available.

Options:
  A) Use SearchAPI (slight cost, fast)
  B) Use offline Tauri docs (free, limited)
  C) Use pattern memory (free, estimated)
  D) Skip (risky, might miss edge cases)

Recommend: C (we have good Tauri patterns)

Your choice? [A/B/C/D/ask_me_later]"

User: "C"

MIP: "Using pattern memory for Tauri.
    Will ask if pattern fails."
    
[Continues with fallback]
```

---

## Automatic Fallback (Silent)

If only one fallback sensible, apply silently :

```
Example 1 : No TodoWrite â†’ Auto use text announcements
Example 2 : No terminal â†’ Auto ask user to execute
Example 3 : No MCP â†’ Auto try SearchAPI if available
Example 4 : Context too small â†’ Auto summarize files

User sees: No interruption, seamless experience
```

---

## Fallback Matrix (Quick Reference)

| Capacity | M1 | M2 | M3 | Fallback chain |
|----------|----|----|----|----|
| Terminal | âœ… | âœ… | âŒ | manual â†’ instructions |
| Web search | âœ… | âœ… | âš ï¸ | SearchAPI â†’ DuckDuckGo â†’ offline |
| Parallel | âœ… | âœ… | âŒ | sequential â†’ single agent |
| Multi-file | âœ… | âœ… | âŒâ†’iterative | code blocks â†’ manual |
| TodoWrite | âœ… | âœ… | âŒ | text annonce â†’ checkbox |
| Context | âœ… | âœ… | âš ï¸ | summary â†’ index JSON |

---

## Implementation Checklist

For each agent Ã— mode combination, specify:

- [ ] **What's missing** (capabilities absent)
- [ ] **Fallback order** (A â†’ B â†’ C)
- [ ] **User notification** (silent vs ask)
- [ ] **Viability** (still productive?)
- [ ] **Testing** (does fallback work?)

### Example: Maria (Orchestration) in Mode 3

```yaml
Missing capabilities:
  - Parallel agents (sequential fallback)
  - Large context (summarize fallback)
  - Terminal access (user manual fallback)

Fallback strategy:
  Agent runs sequentially
  Reads files in summary mode
  Asks user for terminal outputs
  
Notification: Silent (user knows Mode 3)
Viability: Good (T1-T2)
Testing: âœ… Done (T2 test passed)
```

---

## Error Handling

If fallback fails :

```
Scenario: DuckDuckGo curl fails (no internet)

Fallback order:
  1. curl to DuckDuckGo â†’ FAIL (no internet)
  2. Local markdown cache â†’ FAIL (file missing)
  3. Pattern memory â†’ OFFER (educated guess)
  4. User input â†’ ASK ("Tell me...")
  5. Skip â†’ OFFER ("Ignore, might have issues")

Display to user:
  "âŒ Web search failed (no internet)
   âŒ Offline docs not found
   
   Options:
     A) Use estimated pattern (risky)
     B) You tell me the answer
     C) Skip (might fail later)
   
   Recommend: B
   
   Your choice?"
```

---

## Success metrics

| Metric | Target | How to measure |
|--------|--------|--------|
| **Zero crashes on missing capability** | 100% | Test each mode with missing feature |
| **User satisfied with fallback** | 90%+ | Survey post-task |
| **Fallback actually works** | 95%+ | Test coverage per fallback |
| **Time overhead acceptable** | <30% | Measure time Mode 3 vs Mode 1 |
| **Doc clarity** | 90% understand | Check comprehension test |

---

## Documentation

- [ADAPTIVE-MODES.md](../profiles/ADAPTIVE-MODES.md) â€” Mode overview
- [CAPABILITY-NEGOTIATION.md](../profiles/CAPABILITY-NEGOTIATION.md) â€” User asks
- [AGENT-CERTIFICATION-PROTOCOL.md](../certifications/agent-certification-protocol/INDEX.md) â€” Agent capacity per mode
- [Each agent file](../certifications/agents/) â€” Specific fallbacks

