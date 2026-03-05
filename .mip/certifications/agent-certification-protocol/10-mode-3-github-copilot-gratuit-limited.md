<!-- source: .mip/certifications/AGENT-CERTIFICATION-PROTOCOL.md lines 284-351 -->

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

