---
id: mip.profiles.readme
title: "MIP Profiles — Adaptive LLM Profile System"
---

# MIP Profiles: Adaptive LLM & Tool System

> **Switch between any LLM and any tool without changing your workflow.**  
> Default MIP adapts to your profile's capabilities automatically.

---

## What is a Profile?

A **profile** = combination of:
1. **LLM Provider** (Claude, Mistral, Copilot, Ollama)
2. **Tool/IDE** (Cursor, VS Code, terminal)
3. **Capabilities** (what this combo can do)
4. **Optional Constraints** (legal, confidential, offline, etc.)

```
Profile = LLM + Tool + Capabilities + [Constraints]

Example:
  mistral-nemo = Mistral LLM + CLI tool + fast inference + balance
  github-copilot-free = Copilot + VS Code + limited autonomy + free
  ollama = Local Llama + CLI + offline + slow
```

---

## Why Profiles?

### Problem
- Changed from Claude to Mistral → Had to rewrite all skills
- Switched to Copilot free plan → Lost all autonomy features
- Needed offline → Had to learn completely different tool
- **Result**: Fragmented, unmaintainable system

### Solution
**One MIP protocol** that adapts to your profile's capabilities.

- ✅ Switch profiles = MIP auto-adapts
- ✅ Switch LLM = No skill changes needed
- ✅ Switch tool = No workflow translation needed
- ✅ Mix profiles per task = Maximum flexibility

---

## The 4 Profile Types

### 1. **Builtin Profiles** (8 pre-made options)

Ready-to-use profiles from the MIP team.

```
anthropic-opus       ← Claude 3 Opus (best quality, most expensive)
anthropic-sonnet     ← Claude 3 Sonnet (good quality, fast)
mistral-nemo         ← Mistral Nemo (balanced, cheap)
mistral-small-api    ← Mistral Small (ultra-budget)
github-copilot-free  ← GitHub Copilot free plan
ollama              ← Local Ollama (offline, free)
lm-studio           ← LM Studio GUI (offline, free)
cursor-composer     ← Cursor IDE native integration
```

**Use case**: Most people use 1-2 builtin profiles as-is.

---

### 2. **Profiles with Constraints** (Stacking)

Builtin profile + optional constraints applied at runtime.

```
Profile: mistral-nemo
+ Constraint: confidential-data
+ Constraint: legal-compliance
+ Constraint: regional-eu
= Result: Mistral with strict confidentiality + audit trails + GDPR
```

**Use case**: Adapt builtin profile to compliance/security needs.

---

### 3. **Custom Profiles** (User-created)

Build your own from scratch or copy a template.

```
Example: "total-gdpr-safe"
  Base: mistral-nemo
  Customizations:
    - Legal compliance tracking
    - EU data residency
    - Confidential data handling
    - vs. spending $5k on custom LLM setup
```

**Use case**: Project-specific or company-specific configurations.

---

### 4. **Stacked Profiles** (Runtime composition)

Most powerful: Base profile + multiple constraints merged on-the-fly.

```
User: "I need to work on client code (confidential) + GDPR (EU) + offline"
  Base: ollama-mistral
  + Constraints: confidential-data, regional-eu, offline-only
  = Works perfectly

Next task: "Just need cheap baseline for notes"
  Base: mistral-nemo
  + No constraints
  = Switch with: mip_profile mistral-nemo
```

**Use case**: Switch profiles per task (morning = secure, afternoon = fast & cheap).

---

## Which Profile Should I Use?

### Decision Tree (3 questions)

```
Q1: "Do I need maximum autonomy?"
  YES → anthropic-opus (Claude), cost €1-3/task
  NO  → Continue...

Q2: "Budget is critical?"
  YES → mistral-nemo (Mistral), cost €0.10/task
  NO  → Continue...

Q3: "Must it work offline?"
  YES → ollama (Local), cost $0
  NO  → Use mistral-nemo (balanced default)
```

### Real-World Profiles

| Scenario | Profile | Constraints | Cost |
|----------|---------|-------------|------|
| **Total Energy (Corporate)** | mistral-nemo | legal, confidential, regional-eu | €0.15/task |
| **Freelancer (Client work)** | mistral-nemo | confidential-data | €0.10/task |
| **Healthcare** | claude-sonnet | hipaa, pii-strict, legal | €0.50/task |
| **Startup (MVP)** | claude-opus | none (speed first) | €1-3/task |
| **Startup (Budget)** | mistral-nemo | none | €0.10/task |
| **Top Secret** | ollama | offline-only, confidential | $0 |
| **Just Copilot** | github-copilot | none | $0 |

---

## How Profiles Work

### Step 1: Activate Profile

```bash
mip_profile mistral-nemo
# Output: ✅ Active: mistral-nemo
#         Capabilities: [list]
#         Constraints: None
```

### Step 2: MIP Auto-adapts

When you give a task:

```
User: "Refactor payment logic (T3 task)"

MIP checks:
  ✓ Active profile: mistral-nemo
  ✓ Capabilities: Parallel agents, terminal, web search
  ✓ No constraints
  
MIP switches mode automatically:
  → Use Mode 2 (Guided Autonomy)
  → Parallel agents enabled
  → Aggressive parallelism
  → Streaming responses
  → Full logging
  
✅ Start coding with optimizations for this profile
```

### Step 3: Profile Awareness Throughout

Every MIP component checks the active profile:

```
Maria (Orchestrator)
  → "What mode should I use?" → Checks profile → Chooses optimal mode

Lise (Frontend Agent)
  → "Can I use Dioxus 0.6?" → Checks profile.capabilities → "Yes"

Victor (Security Agent)
  → "Should I enable logging?" → Checks profile.constraints → "No logging"

Hugo (DevOps)
  → "Can I auto-test?" → Checks profile → "Yes if Mistral, No if Copilot"
```

---

## Profile Switching: How Often?

### You Can Switch Anytime

```
Task 1 (9-10am): "Review client code" → mip_profile client-confidential
Task 2 (10-11am): "Code translation" → mip_profile mistral-nemo  ← Fast/cheap
Task 3 (11-12pm): "Debug complex bug" → mip_profile claude-opus  ← Quality
Task 4 (12-1pm): "Offline environment" → mip_profile ollama      ← No internet
```

### Or Stick With One

```
Use the same profile all day:
  mip_profile mistral-nemo
  (works for 90% of tasks)
```

### No Performance Penalty

Switching profiles takes:
- **Validation**: <100ms
- **Loading new config**: <50ms
- **Zero code changes needed**

→ Total overhead: **<150ms** (imperceptible)

---

## Builtin Profiles Overview

### anthropic-opus (Mode 1 — 100% Autonomy)

```yaml
llm: Claude 3 Opus
speed: Medium
quality: Best (state-of-the-art)
cost: €1-3 per task
autonomy: 100%
use_case: Complex tasks, high stakes, quality essential
```

**Pros**:
- ✅ Best reasoning ability
- ✅ 200k context window
- ✅ Handles complex code + architecture

**Cons**:
- ❌ Expensive
- ❌ Slower than Mistral

**When to use**: Complex T4-T5 tasks, strategic decisions, code reviews

---

### mistral-nemo (Mode 2 — 90% Autonomy, Balanced)

```yaml
llm: Mistral Nemo 7B
speed: Fast
quality: Good (85% of Claude quality)
cost: €0.10 per task
autonomy: 90%
use_case: Most tasks (default recommendation)
```

**Pros**:
- ✅ Cheap (10x vs Claude)
- ✅ Fast inference
- ✅ Good coding ability
- ✅ EU-based provider (GDPR friendly)

**Cons**:
- ⚠️ Slightly lower reasoning than Claude

**When to use**: Default choice for 90% of tasks

---

### github-copilot-free (Mode 3 — 40% Autonomy, Assisted)

```yaml
llm: GitHub Copilot (Claude-based)
speed: Fast
quality: OK
cost: Free ($0)
autonomy: 40%
use_case: Assisted only, user drives, LLM suggests
```

**Pros**:
- ✅ Free
- ✅ Great for pair programming
- ✅ Built into VS Code

**Cons**:
- ❌ Limited autonomy
- ❌ Terminal access limited
- ❌ Context window small (8k)

**When to use**: Only Copilot available (corporate), learning mode, pair programming

---

### ollama (Mode 5 — 50% Autonomy, Offline)

```yaml
llm: Llama 2 / Mistral 7B (local)
speed: Slow (10-100 tok/sec depending on GPU)
quality: OK (70-80% of Claude)
cost: Free ($0, hardware only)
autonomy: 50%
use_case: Offline/air-gapped, maximum privacy
```

**Pros**:
- ✅ Free (after hardware)
- ✅ Complete privacy (no data leaves)
- ✅ Works offline

**Cons**:
- ❌ Slow (need GPU for decent speed)
- ❌ Lower quality models

**When to use**: Offline requirement, maximum privacy, air-gapped networks

---

## Constraints: Add Security/Compliance

Constraints are **optional overlays** that modify profile behavior.

### Common Constraints

| Constraint | Purpose | Overhead | When To Use |
|-----------|---------|----------|------------|
| **legal-compliance** | Audit trails + immutable logs | +15% | Financial, regulated |
| **confidential-data** | No logging, no cache, local-only | +30% | Client code (NDA) |
| **offline-only** | Complete isolation (no internet) | -50% speed | Air-gapped, top secret |
| **regional-eu** | EU data residency + GDPR | +5% cost | Europe, GDPR required |
| **pii-strict** | PII scanning + redaction | +10% | Healthcare, customer data |
| **hipaa-compliant** | Healthcare compliance | +20% | Medical/healthcare |
| **tool-locked-cursor** | Requires Cursor IDE | Varies | Cursor-only environments |

### Combining Constraints

```
mistral-nemo + confidential-data + regional-eu
= Mistral with strict confidentiality + EU compliance
= Perfect for Total Energy in France

claude-opus + legal-compliance + offline-only
= (Impossible: Claude needs internet)
= System auto-switches to: ollama + legal-compliance + offline-only
```

---

## How to Create a Profile

### Option 1: Interactive Setup (Easiest)

```bash
mip_profile create my-setup
# Maria asks questions → auto-generates profile
```

### Option 2: Template

```bash
mip_profile create-from-template corporate
# Copies template, opens editor, replace :PLACEHOLDER: values
```

### Option 3: Builtin + Constraints

```bash
# Take builtin
mip_profile create myprofile --base mistral-nemo

# Add constraints
mip_profile apply-constraint myprofile legal-compliance
mip_profile apply-constraint myprofile regional-eu
```

### Option 4: Manual YAML

```bash
cp builtin/mistral-nemo.md custom/my-profile.md
# Edit YAML
mip_profile validate my-profile
```

---

## File Structure

```
.mip/profiles/
├── Core Documentation
│   ├── README.md ..................... THIS FILE
│   ├── INDEX.md ...................... Navigation map
│   ├── MANAGEMENT.md ................. CRUD protocol
│   ├── TEMPLATES.md .................. 4 templates
│   └── CONSTRAINTS.md ................ All constraints
│
├── builtin/ (Pre-made, don't edit)
│   ├── anthropic-opus.md
│   ├── mistral-nemo.md
│   ├── github-copilot-free.md
│   └── ... (8 total)
│
├── constraints/ (Constraint definitions)
│   ├── legal-compliance.md
│   ├── confidential-data.md
│   ├── offline-only.md
│   ├── regional-eu.md
│   └── ... (14+ total)
│
├── custom/ (Your profiles)
│   ├── my-work-setup.md
│   └── client-acme.md
│
├── cache/ (Auto-managed)
│   ├── merged-profiles/
│   ├── history/
│   └── validation-logs/
│
└── Config
    ├── active (current profile slug)
    └── subscriptions.md (token quotas)
```

---

## Quick Reference: Commands

```bash
# Manage profiles
mip_profile list                          # See all
mip_profile show                          # Active profile
mip_profile {name}                        # Activate
mip_profile create {name} --base {base}   # Create
mip_profile delete {name}                 # Remove

# Add constraints
mip_profile apply-constraint {profile} {constraint}   # Add
mip_profile remove-constraint {profile} {constraint}  # Remove

# Info
mip_profile validate {profile}            # Check validity
mip_profile check-capabilities {profile}  # What can it do
mip_profile history                       # Recent switches

# Advanced
mip_profile export {profile} > file.yaml  # Backup
mip_profile import file.yaml              # Restore
```

---

## Examples: Real Workflows

### Workflow 1: Freelancer with 3 Clients

```
Base: mistral-nemo-budget
  → Fast + cheap, no constraints

Client 1 (Acme) - NDA:
  → mistral-nemo + confidential-data
  
Client 2 (BankCorp) - Regulated:
  → mistral-nemo + legal-compliance + regional-eu
  
Client 3 (HealthTech) - HIPAA:
  → claude-sonnet + hipaa-compliant + pii-strict

Daily routine:
  Morning: mip_profile mistral-nemo (administration)
  
  10am: mip_profile client-acme (secret code review)
        → confidential-data constraint active
  
  11am: mip_profile mistral-nemo-budget (own project)
  
  2pm: mip_profile client-bankcorp (refactor)
       → legal + regional-eu constraints active
  
  4pm: mip_profile client-healthtech (backend fix)
       → HIPAA + PII strict constraints active
```

### Workflow 2: Startup (Early Stage)

```
Profile: claude-opus (best quality, MVP is critical)
  Constraints: None (speed over cost)
  
  Parallelism: Aggressive (4 agents)
  Auto-testing: On
  Logging: Minimal
  Cost: ~€50/day
  
  Result: 2-3x faster than solo developer
  
After funding:
  (No change needed, profile stays the same)
  Cost tracking just increases (budget rises with company)
```

### Workflow 3: Corporate (Compliance)

```
Profile: mistral-nemo
  Constraints:
    - legal-compliance (audit trails)
    - confidential-data (no external caching)
    - regional-eu (GDPR)
  
  Result:
    ✅ GDPR ready
    ✅ SOX ready (audit logs)
    ✅ No data leaves France
    ✅ Cost: €0.15/task (cheap)
    
  Override for emergency?
    mip_profile override-compliance
    (asks for manager approval)
```

---

## Next Steps

1. **Read [INDEX.md](./INDEX.md)** (5 min) — Full navigation
2. **Pick a template** → [TEMPLATES.md](./TEMPLATES.md)
3. **Create first profile** → `mip_profile create my-setup`
4. **Learn constraints** → [CONSTRAINTS.md](./CONSTRAINTS.md)
5. **Deep dive** → Read specific constraint docs as needed

---

## FAQ

**Q: Do profiles have breaking changes between versions?**  
A: No. Builtin profiles are backward compatible. Custom profiles you create are yours.

**Q: Can I use multiple profiles simultaneously?**  
A: Not simultaneously, but switching takes <150ms, so you can switch per-task.

**Q: What if my profile doesn't have a capability I need?**  
A: Maria (orchestrator) will ask you to switch or offer fallback options.

**Q: Can I combine all constraints?**  
A: Most yes, but some conflict. System warns you (E.g., offline + regional-eu = works; offline + online = impossible)

**Q: Is there a "best" profile?**  
A: No universal best. `mistral-nemo` is a good default (balanced speed/cost/quality).

**Q: How do I get constraints to work with my custom profile?**  
A: Add `constraints: [constraint-name]` to YAML and run `mip_profile validate`

---

## References

- **All Profiles** → [INDEX.md](./INDEX.md) (navigation)
- **Create Profiles** → [TEMPLATES.md](./TEMPLATES.md)
- **CRUD Operations** → [MANAGEMENT.md](./MANAGEMENT.md)
- **Constraints** → [CONSTRAINTS.md](./CONSTRAINTS.md)
- **Individual builtin profiles** → `builtin/*.md`
- **Individual constraints** → `constraints/*.md`

---

*Part of MIP v2 — Adaptive Profile System*  
*Consolidating AI workflows across all LLMs and tools*
