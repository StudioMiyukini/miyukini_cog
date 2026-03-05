---
id: mip.profiles.quickstart
title: "Quick Start — Create Your First Profile in 5 Minutes"
---

# Profile Quick Start (5 minutes)

> Don't want to read. Just want a working profile? Follow these 5 steps.

---

## 🚀 Pick a Profile

**Answer 1 question**:

### "What's my situation?"

```
A) Corporate (Total, Bank) → mistral-nemo + constraints
B) Freelancer (Client code) → mistral-nemo + confidential
C) Startup MVP (Speed first) → claude-opus
D) Healthcare (PHI) → claude-sonnet + hipaa
E) No internet (Offline) → ollama + offline-only
F) Just use GitHub Copilot → github-copilot-free
G) Something custom → Read TEMPLATES.md
```

---

## 📋 Option A: Corporate (Total Energy)

```bash
# Setup profile (2 min)
mip_profile create-from-template corporate --name total-setup

# Answer questions:
  Company: Total
  Tool: VS Code
  Region: EU
  Budget: $2000/month

# View it
mip_profile show total-setup
# → Constraints: legal-compliance, confidential-data, regional-eu

# Activate
mip_profile total-setup

# ✅ Done. Start working.
```

---

## 👥 Option B: Freelancer

```bash
# Create base profile
mip_profile create-from-template freelance --name client-acme

# Answer:
  Client: Acme
  LLM: mistral
  Budget: medium

# This profile has: confidential-data constraint (client code secret)

# Activate
mip_profile client-acme

# ✅ Done. Code is now confidential (no logs, no caching).
```

---

## 🚀 Option C: Startup

```bash
# Fastest option: just use default
mip_profile claude-opus

# Or create custom with templates
mip_profile create-from-template startup --name mvp-fast

# ✅ Done. Speed mode active.
```

---

## 🏥 Option D: Healthcare

```bash
# Healthcare template
mip_profile create-from-template healthcare --name hospital

# Auto-applies: hipaa-compliant, pii-strict, legal-compliance

# ✅ Done. HIPAA-ready.
```

---

## ⚡ Option E: No Internet (Offline)

```bash
# Prerequisites:
#   1. Ollama installed + running
#   2. ollama serve (in another terminal)

# Create profile
mip_profile create offline-vault \
  --base ollama \
  --constraints offline-only

# ✅ Done. 100% offline, no data leaves your machine ever.
```

---

## 🎯 Option F: Just Copilot

```bash
# If you only have GitHub Copilot free:
mip_profile github-copilot-free

# Maria will warn: "Limited to assisted mode (40%), not autonomous"
# But it works!

# ✅ Done. Using free Copilot.
```

---

## 🎓 Next Steps (Pick One)

### If you have 5 more minutes

Read [README.md](./README.md) to understand:
- What profiles are
- Why they matter
- When to switch profiles

### If you have 15 more minutes

Read [CONSTRAINTS.md](./CONSTRAINTS.md) to see:
- What constraints exist
- How to stack them
- When to use each one

### If you have time (Go Deep)

1. [README.md](./README.md) — 5 min
2. [TEMPLATES.md](./TEMPLATES.md) — 8 min
3. [CONSTRAINTS.md](./CONSTRAINTS.md) — 10 min
4. Specific constraint files in `constraints/` — as needed

---

## 📌 Most Common Tasks

### "Switch profiles"

```bash
mip_profile mistral-nemo
mip_profile client-secret
mip_profile claude-opus
# (each takes < 150ms)
```

### "Add constraint to my profile"

```bash
mip_profile apply-constraint myprofile confidential-data
mip_profile apply-constraint myprofile legal-compliance
mip_profile show myprofile
# → Constraints: confidential-data, legal-compliance
```

### "Remove constraint"

```bash
mip_profile remove-constraint myprofile confidential-data
```

### "See all profiles"

```bash
mip_profile list
```

### "See current profile"

```bash
mip_profile
# or
mip_profile show
```

---

## ✅ Check Your Setup

```bash
# Validate your profile works
mip_profile validate myprofile

# See what it can do
mip_profile check-capabilities myprofile

# Check constraint compatibility
mip_profile show-constraints myprofile
```

---

## ❓ Troubleshooting

### "Profile switch didn't work"

```bash
mip_profile validate myprofile
# Check error message
```

### "Constraint isn't applying"

```bash
# Remove and re-apply
mip_profile remove-constraint myprofile constraint-name
mip_profile apply-constraint myprofile constraint-name
```

### "I want to go back to previous profile"

```bash
mip_profile history
# Shows: [Claude used 2h ago] → [Mistral used 30m ago] → [active now]

mip_profile {previous-name}  # Switch back
```

### "I need help"

```
1. Read: [README.md](./README.md) — "Why profiles"
2. Read: [TEMPLATES.md](./TEMPLATES.md) — "How to create"
3. Read: [CONSTRAINTS.md](./CONSTRAINTS.md) — "What constraints"
4. Check specific constraint files in: constraints/
```

---

## 🎁 Pro Tips

### Tip 1: Different Profiles Per Task

```
Morning: mip_profile mistral-nemo (cheap, quick tasks)
10am: mip_profile client-secret (confidential code)
11am: mip_profile mistral-nemo (back to cheap)
2pm: mip_profile claude-opus (complex task, need quality)
```

→ Switch as often as you want, zero overhead

### Tip 2: Combine Constraints Wisely

```
✅ Good combinations:
  - confidential + legal (secret code with audit trails)
  - offline + confidential (maximum security)
  - legal + regional-eu (GDPR + audit)

❌ Avoid:
  - offline + regional-us (contradictory locations)
  - online-api + offline-only (impossible)
```

### Tip 3: Cost Tracking

If you have API-based profiles (Claude, Mistral):

```bash
mip_profile show-costs
# Shows: Daily cost, Monthly estimate, Budget remaining
```

---

## 🏁 You're Done!

```
✅ Profile created
✅ Understand how to switch
✅ Know how to add constraints
✅ Know how to check what works

You're ready to use the MIP profile system!
```

### Next time, to work on different task types:

```bash
# Client code (confidential)
mip_profile client-secret

# Complex architecture (need best quality)
mip_profile claude-opus

# Quick note (cheap + fast)
mip_profile mistral-nemo

# No internet (air-gap)
mip_profile ollama

# Your custom setup
mip_profile myprofile
```

---

## 📚 Full Documentation

When you're ready for more:

| Document | Time | What You'll Learn |
|----------|------|-------------------|
| [README.md](./README.md) | 5 min | What profiles are, why they matter |
| [INDEX.md](./INDEX.md) | 5 min | Navigate all the docs |
| [TEMPLATES.md](./TEMPLATES.md) | 8 min | How to create custom profiles |
| [MANAGEMENT.md](./MANAGEMENT.md) | 10 min | All CRUD commands in detail |
| [CONSTRAINTS.md](./CONSTRAINTS.md) | 10 min | All 14+ constraints explained |
| Constraint files | 5 min each | Details on each constraint |

---

## Questions?

**"Which profile should I start with?"**  
→ Mistral-Nemo (balanced, cheap, fast)

**"Do I need constraints?"**  
→ Only if you have compliance/security needs

**"Can I change profiles later?"**  
→ Yes, switch anytime with zero penalty

**"Will my code break if I switch profiles?"**  
→ No. MIP adapts automatically.

---

*Ready to go? Start with your profile choice above, then come back here when you need more info.*

*Last updated: January 2025*
