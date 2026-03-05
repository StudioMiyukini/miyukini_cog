<!-- source: .mip/certifications/AGENT-CERTIFICATION-PROTOCOL.md lines 68-206 -->

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

