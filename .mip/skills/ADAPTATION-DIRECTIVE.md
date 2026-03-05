---
id: mip.skills.adaptation-directive
title: Skills Adaptation Directive — Rendre skills LLM-agnostic
---

# Skills Adaptation Directive

> Les **Skills IA** (fichiers SKILL.md) doivent être **adaptables** à tous les LLMs. Ce guide montre comment ajouter directives conditionnelles dans chaque Skill pour supporter Modes 1-5.

---

## Problem Statement

Actuellement :
- Skill miyukini-mip-workflow.md assume clausodes entièrement en français
- Dirige directement vers `manage_todo_list` (Mode 1 only)
- Suppose access terminaux natif (Mode 1-2 only)
- Suppose MCP web search (Mode 1-2 only)

**Résultat** : Utilisateur Mode 3 (Copilot) → Frustration ("Tool not found!")

**Solution** : Ajouter sections **"Selon votre Mode MIP"** dans chaque Skill.

---

## Template de Skill Adapter

Voici le template pour adapter n'importe quel SKILL.md existant :

```markdown
# Skill Title

> Description générale du skill

---

## Capacity Requirements

Ce skill nécessite :
- [ ] Terminal access (build, run, shell)
- [ ] Multi-file edits (3+ fichiers)
- [ ] Parallel agents (optional, speedup)
- [ ] Web search / MCP (optional, context)
- [ ] TodoWrite (optional, tracking)

---

## Supported Modes

| Mode | Support | Notes |
|------|---------|-------|
| **Mode 1** (Claude Code) | ✅ Plein | Optimal, tous les outils |
| **Mode 2** (Mistral) | ✅ Plein | Fallback SearchAPI si MCP absent |
| **Mode 3** (Copilot free) | ⚠️ Partiellement | Sans tests auto, terminal manuel |
| **Mode 4** (GPT-mini) | ⚠️ Minimal | Code review only, sans exécution |
| **Mode 5** (Offline local) | ⚠️ Partial | Sans web, shells locaux seulement |

---

## Procédure par Mode

### **Mode 1-2 : Full Execution** (Claude Code, Mistral)

[Existing full procedure, no changes]

**Tools disponibles** :
- `manage_todo_list` (native)
- `run_in_terminal` (native)
- `multi_replace_string_in_file` (parallel safe)

**Workflow** :
- [Existing P0-P6]

---

### **Mode 3 : Copilot Gratuit** (Assisted Development)

#### Changements requis

1. **TodoWrite → Announcements**

```markdown
Instead of:
manage_todo_list([
  {id: 1, title: "Analyze X", status: "in-progress"},
  ...
])

Use:
📌 **Étapes à faire** :
1️⃣  Analyser X  ← En cours
2️⃣  Coder Y
3️⃣  Tests
4️⃣  Merge

Avancer ? [Y/N]
```

2. **Terminal → Manual**

```markdown
Instead of:
run_in_terminal("cargo build -p X")

Use:
▶ MANUEL : Exécutez en terminal :
  cargo build -p X
  
Collez le résultat ci-dessous.
[Utilisateur tape]
```

3. **Multi-file → File-by-file**

```markdown
Instead of:
multi_replace_string_in_file([
  {path: A, oldString: x, newString: y},
  {path: B, oldString: x, newString: y},
  ...
])

Use iteration-by-iteration:
Iteration 1:
  replace_string_in_file(A, x, y)

[User confirms: "Ok?"]

Iteration 2:
  replace_string_in_file(B, x, y)
```

4. **Context window → Pre-fetch**

```markdown
Instead of:
read_file(path, 1, 100)  # Last-minute read

Pre-fetch before conversation:
read_file(path, 1, 50)   # Aggressively shorten
# Résumé : "File A has 400 lines, 
#  but only lines 10-40 relevant"

read_file(path, 10, 40)  # Only needed range
```

#### Workflow Mode 3 (T2 example)

```
P0 Framing (Maria)
  "Décrire le bug"
  
P3 Implementation
  Iteration 1:
    Copilot: "Lisant fichier A..."
    Copilot: "Changement :
              [code block]
              Appliquer ? Y/N"
    You: "Y"
    You: [copy/paste code]
  
  Iteration 2:
    Copilot: "Fichier B..."
    [repeat]

P4 Integration (MANUAL)
  You: [Run `cargo test` in terminal]
  You: [Copy output]
  You: [Paste in chat]
  Copilot: [Analyze errors, suggest fix]

P5 Delivery
  You: "Prêt ?"
  
P6 Archive
  Copilot: [Short report text]
```

**Key** : Vous = central, Copilot = guide.

---

### **Mode 4 : Code Review** (GPT-mini, etc.)

#### Workflow

```
Your role: Write code, use LLM as reviewer

Sequence:
1. You write code manually
2. Paste code block in chat
3. GPT-mini reviews: "Good, but change X"
4. You update
5. Repeat until approved
```

#### Adapted procedure

```markdown
## Mode 4 : Human-Centric Review

You write the implementation:

\`\`\`rust
// Your code here
fn my_function() { ... }
\`\`\`

Copilot reviews:
"Analysis:
  ✅ Clean structure
  ⚠️  Missing null check at line 42
  🔒 Security: OK
  💬 Suggestion: Add debug! log"

You update, repeat.
```

---

### **Mode 5 : Offline Local** (Llama 3.1 local)

#### Constraints

- No web search (local inference only)
- No MCP (local files only)
- Shell access local (cargo, ls, etc.)
- Slower (CPU-bound)

#### Adapted procedure

```markdown
## Mode 5 : Offline-First

**Pre-load local docs** :
  - Keep Miyukini README offline (git clone)
  - Use MIP Index JSON (zero-external)
  - Shell pipes for file search (grep, find)

Workflow:
  1. Liser local docs (grep, mscm_index/*.json)
  2. Lancer code edits (terminal via shell)
  3. No web search (offline only)
  4. Longer iterations (CPU-bound ~1 tok/sec)

Example:
  Instead: "Search Tauri docs online"
  Use:     "grep -r 'tauri::invoke' local_docs/"
```

---

## Checklist : Adapter un SKILL existant

Pour chaque `.mip/skills/*/SKILL.md` existant :

- [ ] **Section "Capacity Requirements"** ajoutée (début du doc)
- [ ] **"Supported Modes"** table ajoutée
- [ ] **Mode 1-2 section** : Garder procédure existante
- [ ] **Mode 3 section** : Copilot adaptations (TodoWrite, terminal, context)
- [ ] **Mode 4 section** : Code review workflow (humain central)
- [ ] **Mode 5 section** : Offline + local shell (si applicable)
- [ ] **Test Mode 3** : Tester avec Copilot (simulator)
- [ ] **Test Mode 5** : Tester offline (local LM Studio)

---

## Exemple : Adapter miyukini-mip-workflow.md

### Original (Mode 1 only)

```markdown
# MIP Workflow Skill

## Phases P0-P6

When task classification says T2 :
1. Use manage_todo_list
2. Call run_in_terminal
3. Run multi_replace_string_in_file
4. ...
```

### Adapted (All modes)

```markdown
# MIP Workflow Skill

## Capacity Requirements

- [ ] TodoWrite (todo_list tool)
- [ ] Terminal access (build, test)
- [ ] Multi-file edits (refactor)

## Supported Modes

| Mode | Notes |
|------|-------|
| M1-M2 | ✅ Full workflow |
| M3 | ⚠️ Text annonce + manual terminal |
| M4 | ✅ P0 framing only (no exec) |
| M5 | ✅ P0-P6 offline |

## Phases P0-P6 by Mode

### Mode 1-2 : Full Autonomy

[Original procedure unchanged]

### Mode 3 : Copilot + Manual Tests

**P0** : Framing (same)

**P3** : Implementation
  - Max 2 files/iteration
  - TodoWrite → Text announcements
  - Example: "📌 P3-1 ✅ | P3-2 en cours..."

**P4** : Tests
  - You run : cargo test -p X
  - You paste output
  - Copilot reads errors

**P5-P6** : Delivery + archive (text)

### Mode 4 : Review Only

**P0** : Framing (text)

**P3** : You code
  - You write implementation
  - Copilot reviews code block

**P4** : Approval
  - Copilot: thumbs_up/down
  
**P5-P6** : Validation (text)

### Mode 5 : Offline-First

[P0-P6 with all-locally constraints]
```

---

## Implementation Timeline

### Phase 1 : Core Skills (Week 1)
- [ ] miyukini-mip-workflow.md
- [ ] miyukini-architecture.md
- [ ] miyukini-cargo-workspace.md

### Phase 2 : Domain Skills (Week 2)
- [ ] miyukini-cores-api.md
- [ ] miyukini-dioxus-ui.md
- [ ] miyukini-rust-patterns.md

### Phase 3 : Operational Skills (Week 3)
- [ ] miyukini-testing.md
- [ ] miyukini-deployment.md
- [ ] miyukini-docs.md

### Phase 4 : Specialist Skills (Week 4)
- [ ] miyukini-error-security.md
- [ ] miyukini-kindmother-db.md
- [ ] miyukini-mys-origin.md

---

## Certification d'Agents

Quand adapter skills pour Mode X, l'agent doit être "certifié" :

```yaml
Certification Mode 3 (Copilot Gratuit):
  Skill Name: miyukini-mip-workflow
  Agent: Maria (Orchestration)
  
  Checklist:
    - Tested with Copilot (8k context)
    - TodoWrite → Annonces verified
    - Terminal → Manual workflow verified
    - Time estimate < 40 min (Mode 3 pace)
    
  Certification Date: 2026-03-05
  Certified: ✅
  Next review: 2026-06-05 (quarterly)
```

Un agent peut avoir **plusieurs certifications** :
```
Maria Certifications:
  ✅ Mode 1 (Claude Code)
  ✅ Mode 2 (Mistral)
  ✅ Mode 3 (Copilot)
  ❌ Mode 4 (GPT-mini) - not applicable for orchestration
  ✅ Mode 5 (Offline) - testing phase
```

---

## Tools Support Matrix (by Mode)

| Tool | M1 | M2 | M3 | M4 | M5 |
|------|----|----|----|----|-----|
| `manage_todo_list` | ✅ | ✅ | ❌→annonce | ❌ | ✅ |
| `run_in_terminal` | ✅ | ✅ | ❌→manual | ❌ | ✅ |
| `replace_string_in_file` | ✅ | ✅ | ✅ (1/iter) | ❌→code block | ✅ |
| `multi_replace_string_in_file` | ✅ | ✅ | ❌→seq | ❌ | ✅ |
| `read_file` | ✅ | ✅ | ✅ (short) | ✅ | ✅ |
| `grep_search` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `semantic_search` | ✅ | ✅ | ⚠️ (limit) | ⚠️ (limit) | ⚠️ |
| `vscode_askQuestions` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `fetch_webpage` | ✅ | ✅ (fallback) | ❌ | ❌ | ❌ |

---

## Agent Certification Structure

Create `.mip/certifications/` directory :

```
.mip/certifications/
├── maria-certifications.md
├── lise-certifications.md
├── francois-certifications.md
└── ...md
```

**Content** :

```yaml
---
agent: Maria
role: Orchestration (P0, decisions)
certifications:
  - mode_1_claude_code: confirmed (2026-02-15)
  - mode_2_mistral: [confirmed, test phase 50%]
  - mode_3_copilot: testing (limited T2)
  - mode_4_gpt_mini: not_applicable
  - mode_5_offline: [testing, limited parallelism]
next_review: 2026-06-05
notes: "Maria orchestrates 100% via text, works across all modes."
---

# Maria — Agent Certification

### Mode 1 (Claude Code)

Status: ✅ **Certified**

**Tested Skills**:
  - mip-workflow.md ✅
  - architecture.md ✅
  - cargo-workspace.md ✅
  - (15+ skills)

Usage:
  - Full MASS orchestration
  - P0-P6 auto execution
  - Parallel agent direction
  
Known limitations: None

### Mode 2 (Mistral Nemo)

Status: ✅ **Certified**

...

### Mode 3 (Copilot Gratuit)

Status: ⚠️ **In testing**

Tested: mip-workflow (20% features)
Issue: Context 8k too small for multi-crate framing
Workaround: Pre-context one-file-per-task
Target: Certified Q2 2026

### Mode 5 (Offline Llama)

Status: ⚠️ **In testing**

Bottleneck: CPU inference speed
Scenario: Long-running P3 (20 min vs 2 min Mode 1)
Acceptable for: Evening/night builds
Target: Certified Q2 2026
```

---

## Documentation de Référence

- [ADAPTIVE-MODES.md](./ADAPTIVE-MODES.md) — 5 modes detail
- [CAPABILITY-NEGOTIATION.md](./CAPABILITY-NEGOTIATION.md) — LLM announcement
- [Profiles](./INDEX.md) — Choisir profil
- [Industrial Scenarios](../usecases/INDUSTRIAL-SCENARIOS.md) — Cas réels
