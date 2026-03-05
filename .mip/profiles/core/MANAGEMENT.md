---
id: mip.profiles.management
title: Profile Management Protocol (CRUD) â€” Restructure MIP Profiles
---

# Profile Management Protocol (CRUD)

> **Vision**: Users CRUD profiles at SETUP + runtime. Profiles = LLM + constraints (legal, confidential, tool-specific). Stack profiles per situation.

---

## Architecture Overview

```
.mip/profiles/
â”œâ”€â”€ MANAGEMENT.md           â† You are here
â”œâ”€â”€ INDEX.md                (profile directory)
â”œâ”€â”€ CONSTRAINTS.md          (compliance matrix)
â”œâ”€â”€ TEMPLATES.md            (create custom profiles)
â”œâ”€â”€ active                  (current active profile slug)
â”‚
â”œâ”€â”€ builtin/                (pre-defined, version-controlled)
â”‚   â”œâ”€â”€ anthropic-opus.md
â”‚   â”œâ”€â”€ mistral-nemo.md
â”‚   â”œâ”€â”€ github-copilot-free.md
â”‚   â”œâ”€â”€ ollama.md
â”‚   â””â”€â”€ lm-studio.md
â”‚
â”œâ”€â”€ constraints/            (constraint overlays, version-controlled)
â”‚   â”œâ”€â”€ legal-compliance.md (GDPR, SOC2, HIPAA, etc.)
â”‚   â”œâ”€â”€ confidential-data.md (PII, trade secrets, no logging)
â”‚   â”œâ”€â”€ tool-locked.md      (specific IDE/tool required)
â”‚   â”œâ”€â”€ regional.md         (EU, US, China regulations)
â”‚   â””â”€â”€ custom-constraints.md
â”‚
â”œâ”€â”€ custom/                 (user-created profiles, .gitignore)
â”‚   â”œâ”€â”€ .gitignore         ("*" â€” never commit custom)
â”‚   â”œâ”€â”€ my-total-setup.md
â”‚   â”œâ”€â”€ client-a-gdpr.md
â”‚   â””â”€â”€ offline-only.md
â”‚
â””â”€â”€ cache/                  (generated, not versioned)
    â”œâ”€â”€ .gitignore
    â”œâ”€â”€ capabilities.json   (merged capabilities per profile)
    â”œâ”€â”€ active-profile.json (current selection + merged)
    â””â”€â”€ profile-history.json (switching log)
```

---

## 4 Profile Types

### Type 1: **Builtin Profiles** (Version-controlled)

Pre-defined, part of MIP v2.0:
- `anthropic-opus` â€” Claude Code, Mode 1
- `mistral-nemo` â€” Mistral, Mode 2
- `github-copilot-free` â€” Copilot, Mode 3
- `ollama` â€” Local Llama, Mode 5
- `lm-studio` â€” Local GGUF, Mode 5

Cannot delete. Can be used as base for custom.

### Type 2: **Constraint Overlays** (Version-controlled)

Compliance/legal/confidentiality **layers** applied on top:
- `legal-compliance` â€” GDPR, SOC2, HIPAA, CCPA
- `confidential-data` â€” PII handling, no-logs
- `tool-locked` â€” Locked to specific IDE/tool
- `regional` â€” EU, US, Asia data residency

Stack multiple (e.g., `mistral-nemo` + `legal-compliance` + `eu-region`).

### Type 3: **Custom Profiles** (User-created, NOT versioned)

Users create at SETUP or runtime:
- `my-total-setup.md` â€” Copilot + legal constraints + no web
- `client-a-gdpr.md` â€” Mistral + GDPR + local inference
- `offline-only.md` â€” Llama local + no internet

Never committed to git. Can be shared manually.

### Type 4: **Stacked Profile** (Runtime composition)

Multiple profiles merged:
```
Active profile = builtin + constraint[1] + constraint[2] + custom
Example:
  Base: mistral-nemo
  + legal-compliance (GDPR checks)
  + confidential-data (no logging)
  + regional (EU data residency)
  = Merged effective profile
```

---

## CRUD Operations (Management)

### **CREATE** â€” New Profile

#### At SETUP-5 (Interactive)

```
Maria: "Want to create a custom profile?"
User  : "Yes, corporate constraints"
Maria : "Name your profile (alphanumeric + dash)"
User  : "total-gdpr-compliant"

Maria : "Base on existing profile?"
User  : "mistral-nemo"

Maria : "Add constraints? (legal, confidential, tool, regional)"
User  : "legal-compliance, confidential-data"

Maria : "Tool locked to Cursor?"
User  : "No, VS Code"

Maria : "Creating total-gdpr-compliant...
        âœ… Done. Use `/mip_profile total-gdpr-compliant`"
```

#### Manual (YAML frontmatter)

```bash
# Create template from builtin
cp .mip/profiles/builtin/mistral-nemo.md \
   .mip/profiles/custom/my-setup.md

# Edit my-setup.md with custom values
nano .mip/profiles/custom/my-setup.md

# Validate
mip_profile validate my-setup.md
# âœ… Valid

# Activate
mip_profile my-setup
```

### **READ** â€” View Profile Details

```bash
# List all available
mip_profile list
# builtin/  anthropic-opus, mistral-nemo, github-copilot-free, ...
# constraints/  legal-compliance, confidential-data, ...
# custom/   my-setup, client-a-gdpr, ...

# View current active
mip_profile
# Active: mistral-nemo
#   Mode: 2 (Guided Autonomy)
#   LLM: Mistral Nemo
#   Context: 128k
#   Capabilities: [list]

# View specific profile
mip_profile show my-setup
# Shows fullYAML + merged capabilities

# Show constraints applied
mip_profile constraints mistral-nemo
# Legal: GDPR âŒ (not applied)
# Confidential: no logging âŒ
# Regional: EU âŒ
```

### **UPDATE** â€” Modify Profile

#### Builtin (Cannot edit, create custom base)

```bash
# âŒ Cannot edit builtin
mip_profile edit anthropic-opus
# Error: Cannot modify builtin profile.
# Solution: Create custom based on it.

# âœ… Create custom from builtin
mip_profile copy anthropic-opus my-claude-setup
# âœ… Created custom/my-claude-setup.md
# Now edit it
nano .mip/profiles/custom/my-claude-setup.md
```

#### Custom (Full edit capability)

```bash
# Edit in-place
mip_profile edit my-setup

# System opens editor with my-setup.md
# User modifies YAML or capabilities
# Save & validate

# Or direct YAML edit
nano .mip/profiles/custom/my-setup.md
mip_profile validate my-setup

# Or CLI-based updates
mip_profile update my-setup \
  --set llm.context_window=256000 \
  --set capabilities.web_search=false \
  --add constraints="regional-eu"
```

#### Constraints (Apply/unapply to profiles)

```bash
# Apply constraint to custom profile
mip_profile apply-constraint my-setup legal-compliance
# âœ… Added legal-compliance overlay
# Merged profile cached

# Unapply constraint
mip_profile remove-constraint my-setup legal-compliance
# âœ… Removed, recalculated

# Stack multiple constraints
mip_profile apply-constraint my-setup \
  legal-compliance \
  confidential-data \
  regional-eu
```

### **DELETE** â€” Remove Profile

#### Builtin (Protected)

```bash
# âŒ Cannot delete builtin
mip_profile delete anthropic-opus
# Error: Cannot delete builtin profile.
# Use `mip_profile reset` to restore defaults.
```

#### Custom (Full deletion)

```bash
# Delete with confirmation
mip_profile delete my-setup
# âš ï¸  Delete custom/my-setup.md? [y/N]
# > y
# âœ… Deleted

# Or force without confirmation
mip_profile delete my-setup --force
```

---

## Profile Switching (Runtime)

### Simple Switch

```bash
# Switch to another profile
mip_profile mistral-nemo
# âœ… Active profile: mistral-nemo
# Capabilities cached

# Immediate effect: New tasks use new profile
```

### Constraint-aware Switch

```bash
# User enters task with data constraint
"Processing PII data"

mip_profile suggest-constraints pii
# Suggestions:
#   - confidential-data (no logging)
#   - legal-compliance (GDPR)
#   - regional-eu (EU only)
# 
# Apply? [y/N]: y

# Profile stacked: active + constraints
# Effective: mistral-nemo + confidential + gdpr + eu
```

### Switch Back (History)

```bash
# Undo last profile switch
mip_profile undo
# Switched back from mistral-nemo to anthropic-opus

# View profile history
mip_profile history
# 1. anthropic-opus (2026-03-05 14:00)
# 2. mistral-nemo (2026-03-05 14:15) + legal
# 3. github-copilot-free (2026-03-05 14:30)
# 4. mistral-nemo (current)

# Jump to history entry
mip_profile history-goto 2
# Restored: mistral-nemo + legal
```

---

## Constraint System (Overlay)

### Applying Constraints

**Constraint** = restrictions + adapted capabilities

```yaml
# Example: legal-compliance overlay
ID: mip.constraint.legal-compliance
Applies to: All profiles
When applied:
  - Web search â†’ asks for approval per query
  - Logging â†’ audit trail enabled
  - Data residency â†’ may limit to EU
  - Consent â†’ mandatory user confirmation
  - Retention â†’ auto-purge after 30 days
```

### Conflict Detection

```bash
# Apply conflicting constraints
mip_profile apply-constraint my-setup \
  regional-eu \          # EU data residency
  regional-china         # China data residency (CONFLICT!)

# Error: Constraint conflict detected!
# regional-eu and regional-china both apply.
# 
# Choose one:
#   1. regional-eu
#   2. regional-china
#   3. Cancel
```

### Validation

```bash
# Validate profile + constraints
mip_profile validate my-setup
# âœ… Profile valid
# âœ… Capabilities merged
# âœ… No conflicting constraints
# âš ï¸  Warning: Tool locked to Cursor, but using VS Code

# Show warnings
mip_profile validate my-setup --verbose
# Profile: my-setup
#   Tool: Cursor (locked)
#   Actual: VS Code
#   Issue: Tool mismatch (will degrade)
#   Action: Update tool constraint or switch IDE
```

---

## SETUP-5 : Integration

New SETUP step (after SETUP-4 LLM detection):

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ SETUP-5: Profile Configuration          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

Maria: "LLM detected: Claude Sonnet (Mode 1)

        Profile options:
        1. Use builtin 'anthropic-opus'
           (default, full autonomy)
        
        2. Create custom profile
           (add constraints, tool lock, etc.)
        
        3. Stack constraints on builtin
           (e.g., GDPR + confidential)
        
        Choose [1/2/3]: "

User:  "2 (custom)"

Maria: "Profile name? (e.g., 'my-company-setup')"
User:  "total-gdpr"

Maria: "Base builtin profile?"
User:  "mistral-nemo"

Maria: "Constraints? (enter comma-separated slugs)
        Available:
          - legal-compliance
          - confidential-data
          - tool-locked (Cursor / VS Code / others)
          - regional (EU / US / Asia)
        
        Your constraints: "
User:  "legal-compliance, confidential-data, tool-locked-vscode"

Maria: "Tool: VS Code (locked)"
User:  "[Enter]"

Maria: "Endpoint for Mistral? (https://api.mistral.ai or local:port)"
User:  "local:8000"

Maria: "Creating total-gdpr profile...
        âœ… Done.
        
        Profile stack:
          Base: mistral-nemo
          + legal-compliance
          + confidential-data
          + tool-locked (VS Code)
        
        Activated: total-gdpr
        
        Next steps:
          â†’ /mip_profile list (see all)
          â†’ /mip_profile total-gdpr (details)
          â†’ Start P0 with active profile"
```

---

## File Structure Changes

### Before (Messy)

```
.mip/profiles/
â”œâ”€â”€ INDEX.md
â”œâ”€â”€ SCHEMA.md
â”œâ”€â”€ ADAPTIVE-MODES.md
â”œâ”€â”€ CAPABILITY-NEGOTIATION.md
â”œâ”€â”€ anthropic-opus.md
â”œâ”€â”€ mistral-nemo.md
â”œâ”€â”€ mistral-small-api.md
â”œâ”€â”€ github-copilot-free.md
â”œâ”€â”€ ollama.md
â”œâ”€â”€ lm-studio.md
â”œâ”€â”€ cursor-composer.md
â”œâ”€â”€ codex.md
â”œâ”€â”€ active
â”œâ”€â”€ capabilities-matrix.md
â””â”€â”€ [8+ files mixed]
```

### After (Organized)

```
.mip/profiles/
â”œâ”€â”€ MANAGEMENT.md           â† Protocol (this file)
â”œâ”€â”€ CONSTRAINTS.md          â† Constraint matrix
â”œâ”€â”€ TEMPLATES.md            â† Create custom template
â”œâ”€â”€ INDEX.md                â† Profile directory (nav)
â”œâ”€â”€ active                  â† Current active profile
â”‚
â”œâ”€â”€ builtin/
â”‚   â”œâ”€â”€ anthropic-opus.md
â”‚   â”œâ”€â”€ mistral-nemo.md
â”‚   â”œâ”€â”€ github-copilot-free.md
â”‚   â”œâ”€â”€ mistral-small-api.md
â”‚   â”œâ”€â”€ ollama.md
â”‚   â”œâ”€â”€ lm-studio.md
â”‚   â”œâ”€â”€ cursor-composer.md
â”‚   â””â”€â”€ codex.md
â”‚
â”œâ”€â”€ constraints/
â”‚   â”œâ”€â”€ legal-compliance.md  (GDPR, SOC2, HIPAA, CCPA)
â”‚   â”œâ”€â”€ confidential-data.md (PII, no logging, secrets)
â”‚   â”œâ”€â”€ tool-locked.md       (IDE / tool requirement)
â”‚   â”œâ”€â”€ regional.md          (EU, US, China, APAC)
â”‚   â””â”€â”€ custom.md            (user-defined constraints)
â”‚
â”œâ”€â”€ custom/
â”‚   â”œâ”€â”€ .gitignore          ("*")
â”‚   â”œâ”€â”€ my-setup.md
â”‚   â”œâ”€â”€ client-gdpr.md
â”‚   â””â”€â”€ ...
â”‚
â””â”€â”€ cache/
    â”œâ”€â”€ .gitignore
    â”œâ”€â”€ capabilities.json
    â”œâ”€â”€ active-profile.json
    â”œâ”€â”€ profile-history.json
    â””â”€â”€ constraints-cache.json
```

---

## Commands (MIP CLI)

```bash
# Profile management
mip_profile list                           # List all profiles
mip_profile                                # Show active
mip_profile show {profile}                 # Show details
mip_profile {profile}                      # Activate

mip_profile create {name} --base {builtin} # Create custom
mip_profile edit {name}                    # Edit custom
mip_profile update {name} --set KEY=VAL    # Update field
mip_profile copy {source} {dest}           # Clone profile
mip_profile delete {name} [--force]        # Delete custom

mip_profile apply-constraint {profile} {constraint...}  # Stack
mip_profile remove-constraint {profile} {constraint}    # Unstack
mip_profile constraints {profile}          # Show applied

mip_profile validate {name}                # Check validity
mip_profile undo                           # Previous profile
mip_profile history                        # View switches
mip_profile history-goto {index}           # Jump to history

mip_profile merge {profile1} {profile2}    # Merge two profiles
mip_profile diff {p1} {p2}                 # Compare

mip_profile export {profile} --format json # Export
mip_profile import {file}                  # Import
```

---

## Profile Merging (Stacking)

When constraints applied, profiles **merge**:

```yaml
# Base profile: mistral-nemo
llm:
  model: mistral-nemo
  context_window: 128000
capabilities:
  parallel_agents: true
  web_search: true
  logging: true

# + Constraint: legal-compliance
constraints:
  - require_audit_log: true
  - require_consent: true

# + Constraint: confidential-data
logging:
  level: off
  disabled: true

# = Merged effective profile
llm:
  model: mistral-nemo
  context_window: 128000
capabilities:
  parallel_agents: true
  web_search: true       # Has override?
  logging: false         # OVERRIDDEN by confidential-data
constraints:
  - require_audit_log: true
  - require_consent: true
warnings:
  - "Logging disabled but audit_log required: Conflict!"
```

---

## Cache & Performance

### Auto-cache merged profiles

```
When profile activated:
  1. Read base profile YAML
  2. Read all applied constraints
  3. Merge capabilities
  4. Detect conflicts
  5. Write to .mip/profiles/cache/active-profile.json
  
Next task startup:
  1. Check cache validity
  2. If base profile not changed, use cache
  3. If constraint changed, invalidate & rebuild
```

### Cache invalidation

```
Invalidate if:
  - Profile file modified
  - Constraint file modified
  - Explicit `mip_profile invalidate-cache`
  
Command:
  mip_profile rebuild-cache
  # Rebuilds all profile merges from scratch
```

---

## Serialization (Save/Load)

### Save profile to file

```bash
mip_profile export total-gdpr --format yaml
# Outputs to stdout or `{name}.exported.yaml`

mip_profile export total-gdpr --format json
mip_profile export total-gdpr --format toml
```

### Share profile (manual)

```bash
# Export for colleague
mip_profile export total-gdpr > /tmp/total-gdpr.yaml

# Colleague imports
mip_profile import /tmp/total-gdpr.yaml
# âœ… Imported as custom profile
```

---

## Recommendations

### For Teams

1. **Builtin + constraints** = Share standard (GDPR, legal)
2. **Custom per user** = Personal overrides
3. **Git-ignore custom** = No accidental commits

### For Enterprises

1. **Define corporate profiles** in `constraints/`
2. **Lock to specific constraint set** via `mip_profile lock --constraint legal, confidential`
3. **Audit profile usage** via `cache/profile-history.json`
4. **Disable custom profiles** if needed (config option)

---

## Status & Timeline

### NOW (Phase 2 START)
- [ ] Create MANAGEMENT.md (this file)
- [ ] Create CONSTRAINTS.md
- [ ] Create TEMPLATES.md
- [ ] Reorganize .mip/profiles/ structure
- [ ] Create 4 builtin constraint profiles

### Q2 Week 2
- [ ] Implement CLI commands (`mip_profile`)
- [ ] Validation & merging logic
- [ ] Cache system
- [ ] SETUP-5 integration

### Q2 Week 3
- [ ] Testing (profile switching, constraints)
- [ ] Documentation for users
- [ ] Agent certification for custom profiles

### Q2 Week 4
- [ ] Enterprise features (profile locking, audit)
- [ ] Multi-user coordination

---

## References

- [TEMPLATES.md](./TEMPLATES.md) â€” Create custom profiles
- [CONSTRAINTS.md](./CONSTRAINTS.md) â€” Constraint details + matrix
- [INDEX.md](./INDEX.md) â€” Profile directory
- [SETUP.md](..//..//README.md#setup-5) â€” Integration
- [SCHEMA.md](./SCHEMA.md) â€” Profile YAML format

