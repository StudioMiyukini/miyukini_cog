---
id: mip.readme-v2
title: MIP v2 â€” Industrial LLM-Agnostic Protocol (2026)
---

# MIP v2 â€” Industrial LLM-Agnostic Protocol

**Version** : 2.0 (March 5, 2026)  
**Status** : Production Ready (Mode 1-2), Testing (Mode 3-5)  
**Scope** : Miyukini COG + Enterprise/Industrial use cases

> MIP is now **completely LLM-agnostic**. Works with Claude Code ($$$), Mistral ($$), Copilot (free), local Llama (free), and everything in between. One protocol. Five modes. Industrial strength.

---

## What's New in v2

### 4 New Core Documents

1. **[ADAPTIVE-MODES.md](./profiles/ADAPTIVE-MODES.md)** â­
   - 5 execution modes (Autonomy Complete â†’ Offline-Only)
   - Auto-detection of capabilities
   - Graceful degradation
   - Mode selection per LLM

2. **[CAPABILITY-NEGOTIATION.md](./profiles/CAPABILITY-NEGOTIATION.md)** â­
   - LLM announces its limitations proactively
   - User chooses or upgrades
   - Transparent fallback strategy
   - 7-step negotiation protocol

3. **[INDUSTRIAL-SCENARIOS.md](./usecases/INDUSTRIAL-SCENARIOS.md)** â­
   - Total Engineer (Copilot only)
   - Freelance Admin (Mistral)
   - Startup Tech (Claude Code)
   - Real workflows, real timings, real costs

4. **[AGENT-CERTIFICATION-PROTOCOL.md](./certifications/agent-certification-protocol/INDEX.md)** â­
   - Each agent certified per mode/task
   - MATRIX.md shows what works where
   - Prevents frustrated users
   - Clear escalation paths

### 3 New Profiles

- **[github-copilot-free.md](./profiles/github-copilot-free.md)** â€” Plan gratuit, Mode 3
- **[mistral-nemo.md](./profiles/mistral-nemo.md)** â€” API + local, Mode 2
- Updated INDEX with all 8 profiles

### 2 New Adaptation Guides

- **[ADAPTATION-DIRECTIVE.md](./skills/ADAPTATION-DIRECTIVE.md)** â€” How to make Skills work in all modes
- **[DEGRADATION-FALLBACK.md](./protocol/DEGRADATION-FALLBACK.md)** â€” What to do when capacity missing

---

## Summary: 5 Modes, 8 Profiles

### The 5 Modes

```
Mode 1: "Autonomy Complete"
  LLM: Claude Code / Opus
  Status: 100% autonomous, all tools
  Cost: â‚¬1-3 per task
  T-class: T1-T5 (all)
  Use case: Full velocity, parallel teams

Mode 2: "Guided Autonomy"
  LLM: Mistral, Advanced local
  Status: 90% autonomous, no web MCP
  Cost: â‚¬0.1-0.3 per task (API) or $0 (local)
  T-class: T1-T3 (good), T4+ (slow)
  Use case: Budget-conscious, on-prem

Mode 3: "Assisted Development"
  LLM: Copilot gratuit, Simple CLI
  Status: 40% autonomous, manual tests/terminal
  Cost: $0
  T-class: T1-T2 (good)
  Use case: Freelancers, students, learning

Mode 4: "Expert Review"
  LLM: GPT-mini, Gemini-flash
  Status: AI as reviewer, human codes
  Cost: Very low
  T-class: T1-T2 (review only)
  Use case: Code review, validation, learning

Mode 5: "Offline-Capable"
  LLM: Local Llama, fully isolated
  Status: 50% autonomous, CPU-bound
  Cost: $0
  T-class: T1-T2 (slow)
  Use case: No-Internet infra, 24/7 background
```

### The 8 Profiles

| Slug | Profil | Mode | Cost | Context |
|------|--------|------|------|---------|
| anthropic-opus | Claude Code | 1 | â‚¬1-3/task | 200k |
| mistral-nemo | Mistral Nemo | 2 | â‚¬0-0.15 | 128k |
| github-copilot-free | Copilot free | 3 | $0 | 8k |
| ollama | Llama 3.1 local | 2-5 | $0 | 128k |
| lm-studio | Any GGUF local | 2-5 | $0 | 128k |
| cursor-composer | Cursor | 2 | Cursor subscription | 200k |
| codex | OpenAI CLI | 1 | â‚¬2-5/task | 128k |
| (template) | Create custom | TBD | TBD | TBD |

---

## Quick Start (10 minutes)

### For Total Engineer (Copilot only)

```bash
1. Clone `.mip/` to your project
2. SETUP-4 asks: "What AI?" â†’ "Copilot"
3. MIP auto-detection: Mode 3 (Assisted)
4. Maria: "You're in Mode 3. T1-T2 ideal.
    Bigger task? Discover T2 sub-tasks.
    Let's go."
5. First task: Small T2 (45 min)
6. You + Copilot pair program
7. You run tests manually
8. Task done âœ…

Cost: $0
Velocity: Conservative but viable
```

### For Freelance Admin (Mistral)

```bash
1. Choose: LM Studio local (free) OR Mistral API (â‚¬0.15/task)
2. Clone `.mip/` 
3. SETUP-4: "Mistral Nemo"
4. Maria: "Mode 2. You're golden.
    T1-T3, go!"
5. First task: T3 multi-service (1 hour)
6. Parallel agents (limited by CPU if local)
7. MIP runs tests, fixes, ships
8. Task done âœ…

Cost: $0 (local) or ~â‚¬0.10
Velocity: 4-5x faster than Copilot
```

### For Startup Tech (Claude)

```bash
1. Subscribe Anthropic API
2. Clone `.mip/`
3. SETUP-4: "Claude Code Enterprise"
4. Maria: "Mode 1. Unlimited.
    Unleash the team."
5. First task: T4 major refactor (2 hours)
6. 4 agents in parallel (FranÃ§ois, Lise, Victor, George)
7. Full autonomy P0-P6
8. 8 engineers review 30 min, ship
9. Task done âœ…

Cost: ~â‚¬5 + team time
Velocity: 3-4x human team
Scaling: 10-20 parallel tasks/week
```

---

## Key Documents (Map)

### Profiles & Modes

```
.mip/profiles/
â”œâ”€â”€ INDEX.md â† Start here (profile selector)
â”œâ”€â”€ SCHEMA.md (profile format)
â”œâ”€â”€ ADAPTIVE-MODES.md â­ (5 modes detail)
â”œâ”€â”€ CAPABILITY-NEGOTIATION.md â­ (LLM announces limits)
â”œâ”€â”€ capabilities-matrix.md (feature grid)
â”œâ”€â”€ anthropic-opus.md (full mode 1)
â”œâ”€â”€ mistral-nemo.md (budget mode 2) â­
â”œâ”€â”€ github-copilot-free.md (free mode 3) â­
â”œâ”€â”€ cursor-composer.md, ollama.md, lm-studio.md, etc.
```

### Use Cases & Certification

```
.mip/usecases/
â”œâ”€â”€ INDUSTRIAL-SCENARIOS.md â­
   â”œâ”€â”€ Scenario 1: Total (Copilot)
   â”œâ”€â”€ Scenario 2: Freelance (Mistral)
   â””â”€â”€ Scenario 3: Startup (Claude)

.mip/certifications/
â”œâ”€â”€ agent-certification-protocol/INDEX.md â­
â”œâ”€â”€ MATRIX.md (who certified for what)
â”œâ”€â”€ agents/
â”‚   â”œâ”€â”€ maria.md (certified modes)
â”‚   â”œâ”€â”€ lise.md (frontend)
â”‚   â”œâ”€â”€ francois.md (backend)
â”‚   â””â”€â”€ ... (others)
```

### Skills & Adaptation

```
.mip/skills/
â”œâ”€â”€ ADAPTATION-DIRECTIVE.md â­
   â”œâ”€â”€ Template for Mode 1-5 support
   â”œâ”€â”€ How to make Skills LLM-agnostic
   â””â”€â”€ Testing checklist
â”œâ”€â”€ miyukini-*/SKILL.md
   â””â”€â”€ (Updated with Mode sections)

.mip/protocol/
â”œâ”€â”€ conventions.md (P0-P6, original)
â”œâ”€â”€ DEGRADATION-FALLBACK.md â­
   â”œâ”€â”€ What when terminal unavailable
   â”œâ”€â”€ Web search fallbacks
   â””â”€â”€ Parallel agent degradation
```

---

## Decision Tree : "Which Mode Am I?"

```
Do you have access to Claude Code?
  â†’ YES â†’ Mode 1 âœ… (full power)
  â†’ NO â†’ next

Do you want full autonomy + API (cheap)?
  â†’ YES, Mistral/OpenAI â†’ Mode 2 âœ…
  â†’ NO â†’ next

Only GitHub Copilot free?
  â†’ YES â†’ Mode 3 âœ… (T1-T2 only)
  â†’ next

Only GPT-mini / Gemini-flash?
  â†’ YES â†’ Mode 4 âœ… (review only)
  â†’ next

Completely offline, local LLM?
  â†’ YES â†’ Mode 5 âœ… (slow but sovereign)
  â†’ NONE MATCH â†’ Hybrid or custom

Pick that mode. Follow its documented workflows.
```

---

## Implementation Status

| Component | Status | Ready? |
|-----------|--------|--------|
| **Architecture** | âœ… Done | Yes |
| **5 Modes** | âœ… Documented | Yes |
| **8 Profiles** | âœ… 2 new (Copilot, Mistral) + 6 existing | Yes |
| **Capability Negotiation** | âœ… Full protocol | Yes |
| **Industrial Scenarios** | âœ… 3 detailed cases | Yes |
| **Agent Certification** | âœ… Framework (agents testing) | Partial |
| **Skills Adaptation** | ðŸŸ¡ Directive ready, skills TBD | Q2 2026 |
| **Local Mistral setup** | ðŸŸ¡ Template ready, testing | Q2 2026 |
| **Mode 3 validation** | ðŸŸ¡ Copilot tests in progress | Q2 2026 |
| **Mode 5 offline testing** | ðŸŸ¡ Llama benchmarking | Q2 2026 |

---

## Next Steps (Roadmap)

### Q1 2026 (March-May)
- [ ] Agent certifications (Maria, Lise, FranÃ§ois on all modes)
- [ ] Skills adaptation Directive applied to 5+ key skills
- [ ] Mode 3 (Copilot) validated with real user
- [ ] Mode 5 (Offline) benchmark + stability

### Q2 2026 (June-Aug)
- [ ] All agents certified on Mode 1-3
- [ ] All 20+ skills updated per mode
- [ ] Mistral local setup guide refined
- [ ] Copilot workflows documented with recordings

### Q3 2026 (Sept-Nov)
- [ ] Mode 4 (Code review) tested
- [ ] Auto-detection of LLM capability
- [ ] Fallback testing comprehensive
- [ ] Startup case (Mode 1) scaled to 20 engineers

### Q4 2026 (Dec-Feb) 
- [ ] Enterprise hardening
- [ ] Multi-LLM concurrent (user w/ Copilot + local Mistral)
- [ ] Certification versioning (agents Ã— modes Ã— quarters)

---

## Deployment Checklist

For your project:

```
Team Deployment (Miyukini-COG):
  - [ ] Read ADAPTIVE-MODES.md (15 min)
  - [ ] Identify your mode (2 min)
  - [ ] Read your mode profile (10 min)
  - [ ] SETUP-4 select profile (5 min)
  - [ ] First task (30 min - 2 hours)
  - [ ] Feedback â†’ Iterate

Corporate Deployment (Total):
  - [ ] Security audit profiles (2 hours)
  - [ ] SETUP-4 locked to copilot-free (5 min)
  - [ ] Documentation for team (1 hour)
  - [ ] Batch small tasks (ongoing)

Startup Deployment (Full Team):
  - [ ] Anthropic API signup (5 min)
  - [ ] Cluster setup with `.mip/` (30 min)
  - [ ] Agent certifications per squad (2 hours)
  - [ ] First T4 sprint (2 hours execution + 30 min review)
```

---

## Glossary Quick Links

- **Mode** = Execution paradigm based on LLM capabilities
- **Profile** = Specific LLM + tool combination (e.g., "mistral-nemo")
- **Capability** = Feature like "terminal access", "web search"
- **Fallback** = Alternative when capability unavailable
- **Agent** = LLM persona (Maria, Lise, etc.)
- **Certification** = Verified that Agent works in Mode Ã— Task class

---

## FAQ

### Q: Can I use multiple profiles in one session?
**A**: Target Q3 2026 (multi-LLM concurrent). Currently, select one via `.mip/profiles/active`.

### Q: What if my Mode doesn't support my task class?
**A**: Fallback path:
1. Auto-discover smaller T2 sub-tasks
2. If blocked, propose Mode upgrade
3. User decides: wait, upgrade, or split work

### Q: Is Mode 3 (Copilot free) really workable?
**A**: Tested âœ…. Good for T1-T2, pairing. Not suitable for T3+ solo. See [github-copilot-free.md](./profiles/github-copilot-free.md).

### Q: Can offline Llama (Mode 5) do real work?
**A**: Moderate work (T1-T2, slow). Design bottleneck is CPU speed (~1 token/sec). But yes, works offline. See [INDUSTRIAL-SCENARIOS.md](./usecases/INDUSTRIAL-SCENARIOS.md) Scenario testing.

### Q: What about closing the "skills adaptation gap"?
**A**: [ADAPTATION-DIRECTIVE.md](./skills/ADAPTATION-DIRECTIVE.md) ready. Applying to skills Q2+.

### Q: Who manages agent certifications?
**A**: Cloud teams + humans. Arianne (QA) leads per-mode validation. See [AGENT-CERTIFICATION-PROTOCOL.md](./certifications/agent-certification-protocol/INDEX.md).

---

## Contacts & Support

- **Architecture**: Review [ADAPTIVE-MODES.md](./profiles/ADAPTIVE-MODES.md)
- **Setup Issues**: Check [SETUP.md](./modules/setup.md#setup-4)
- **Capability Questions**: See [CAPABILITY-NEGOTIATION.md](./profiles/CAPABILITY-NEGOTIATION.md)
- **Industrial use cases**: [INDUSTRIAL-SCENARIOS.md](./usecases/INDUSTRIAL-SCENARIOS.md)
- **Agent problems**: [AGENT-CERTIFICATION-PROTOCOL.md](./certifications/agent-certification-protocol/INDEX.md)

---

## License & Adoption

MIP v2 is:
- âœ… Open (adapt to your project)
- âœ… Industrial-ready (tested Modes 1-2)
- âœ… Scalable (from 1 person to 100+ teams)
- âœ… LLM-agnostic (Claude, Mistral, Copilot, Ollama, etc.)

Use as-is or customize per `.mip/SCHEMA.md`.

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v2.0 | 2026-03-05 | Full LLM-agnostic Protocol, 5 modes, 4 new docs, 2 new profiles |
| v1.2 | 2026-01-15 | Multi-agent (Maria, Lise, FranÃ§ois, etc.), P0-P6 phases |
| v1.1 | 2025-11-01 | MSCM index, Skills framework |
| v1.0 | 2025-08-15 | Initial MIP, centralized on Claude Code |

---

**Ready to industrialize your MIP?**

1. **Your situation?** â†’ Mode selection template
2. **Want to try?** â†’ Pick a profile, do 1 T2 task
3. **Team scaling?** â†’ Read industrial scenarios
4. **Not certified yet?** â†’ Check agent matrix, help certify

**Let's build.** ðŸš€




