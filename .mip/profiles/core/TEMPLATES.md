---
id: mip.profiles.templates
title: Profile Templates — Create Custom Profiles
---

# Profile Templates

> **No need to write YAML from scratch.** Choose a template, answer questions, get a personalized profile.

---

## Quick Start: 3 Ways to Create Profile

### Way 1: SETUP-5 (Interactive)

During `mip_profile create` with guided Q&A:

```bash
mip_profile create my-setup
# Maria walks you through step-by-step
# ✅ Fastest for beginners
```

### Way 2: Pre-made Template (Fill-in-the-blanks)

Copy template, modify YAML:

```bash
mip_profile create-from-template corporate --name total-gdpr
# Copies templates/corporate.md → custom/total-gdpr.md
# Opens editor with placeholders
# :REPLACE_ME: values to fill
```

### Way 3: CLI-based

```bash
mip_profile create my-setup \
  --base mistral-nemo \
  --constraints legal-compliance,confidential-data \
  --tool-locked vscode \
  --llm-endpoint local:8000
```

---

## Available Templates

### **Template 1: "corporate"**

For: Companies, enterprises with compliance requirements

```yaml
# File: .mip/profiles/templates/corporate.md

---
id: corporate-template
name: "Corporate Setup Template"
description: |
  For medium/large companies with:
  - Legal/compliance requirements
  - Data confidentiality
  - Specific tool requirements
  - Audit needs

placeholders:
  COMPANY_NAME: "e.g., Total, Acme"
  TOOL_IDE: "Cursor | VS Code | other"
  REGION: "EU | US | China | APAC"
  DATA_SENSITIVITY: "high | medium | low"
  BUDGET_LIMIT: "$ per task"

---

# Corporate Profile Template

## Basic Info

**Company**: :COMPANY_NAME:
**Profile Name**: :COMPANY_NAME:-standard
**Base LLM**: mistral-nemo (recommended for corporate)

## Configuration

```yaml
---
id: :COMPANY_NAME:-standard
name: ":COMPANY_NAME: Standard Profile"
tool: mistral-api
llm:
  provider: mistral
  model: mistral-nemo
  context_window: 128000
  endpoint: local:8000  # or https://api.mistral.ai

capabilities:
  parallel_agents: true
  terminal: true
  web_search: false     # Recommended for corporate
  logging: true
  audit_trail: true

constraints:
  - "legal-compliance"
  - "confidential-data"
  - "regional-:REGION:"
  - "tool-locked-:TOOL_IDE:"

custom_constraints:
  approval_process: "cto-email"
  budget_limit: ":BUDGET_LIMIT:"
  max_tokens_per_task: 100000
  allowed_models: ["mistral-nemo"]
  forbidden_models: ["openai", "google"]  # US-based

security:
  encryption_at_rest: aes256
  tls_min_version: 1.2
  audit_retention_days: 365

workflows:
  p0_approval: "required"
  p3_checkpoint: "required"
  p4_audit: "mandatory"
  p5_human_review: "required"
  p6_archive: "automatic"
```

## Constraints Applied

- ✅ legal-compliance (audit + consent)
- ✅ confidential-data (no logging, no cache)
- ✅ regional-:REGION: (:REGION: data residency)
- ✅ tool-locked-:TOOL_IDE: (IDE locked)

## Next Steps

1. Replace all `:PLACEHOLDER:` values
2. Run `mip_profile validate :COMPANY_NAME:-standard`
3. Activate: `mip_profile :COMPANY_NAME:-standard`
```

**Usage**:

```bash
# Create from corporate template
mip_profile create-from-template corporate --name total-setup

# Opens editor:
#   :COMPANY_NAME: → Total
#   :TOOL_IDE: → VS Code
#   :REGION: → EU
#   :DATA_SENSITIVITY: → high
#   :BUDGET_LIMIT: → $500

# After editing → mip_profile validate total-setup
# ✅ Valid, activate with: mip_profile total-setup
```

---

### **Template 2: "freelance"**

For: Solo developers, freelancers, agencies

```yaml
# File: .mip/profiles/templates/freelance.md

---
id: freelance-template
name: "Freelance/Solo Developer Template"
description: |
  For solo developers, freelancers with:
  - Multiple clients
  - Cost-conscious
  - No strict compliance
  - Flexibility

placeholders:
  CLIENT_NAME: "e.g., ClientA, ClientB"
  PREFERRED_LLM: "mistral | claude | copilot"
  BUDGET_CONSTRAINT: "low | medium | high"
  TOOL: "Cursor | VS Code | Vim"

---

# Freelance Profile Template

**Client**: :CLIENT_NAME:
**Profile Name**: client-:CLIENT_NAME:-dev
**Base LLM**: :PREFERRED_LLM:

```yaml
---
id: client-:CLIENT_NAME:-dev
name: "Client :CLIENT_NAME: - Dev Profile"
tool: :PREFERRED_LLM:_tool
llm:
  provider: :PREFERRED_LLM:
  model: mistral-nemo | claude-sonnet
  context_window: 128000

capabilities:
  parallel_agents: true
  terminal: true
  web_search: true
  caching: true
  logging: true  # You control it

constraints:
  - "confidential-data"  # Client code is confidential
  # optional:
  # - "tool-locked-:TOOL:"

budget:
  per_task_limit: :BUDGET_CONSTRAINT:
  per_month_limit: "$500"
  fallback_to_local: "if over budget"  # Switch to free Mistral local

workflows:
  p3_autocode: true
  p4_auto_test: true
  p5_human_review: required
  p6_auto_report: true

client_notes:
  name: ":CLIENT_NAME:"
  contact: "your-email@example.com"
  project_url: "https://..."
  billing: "monthly | per-task"
  rate: "$50/hr : estimate"
```

**Usage**:

```bash
mip_profile create-from-template freelance --name client-acme-dev

# Fill:
#   :CLIENT_NAME: → Acme
#   :PREFERRED_LLM: → mistral
#   :BUDGET_CONSTRAINT: → medium
#   :TOOL: → VS Code

# Activate:
#   mip_profile client-acme-dev

# Now you can:
#   1. Work on Acme project (profile auto applies confidential-data)
#   2. Cost-conscious (tracks per-task budget)
#   3. Multiple clients (create per-client profile, switch as needed)
```

---

### **Template 3: "healthcare"**

For: Healthcare providers, medical research

```yaml
# File: .mip/profiles/templates/healthcare.md

---
id: healthcare-template
name: "Healthcare/Medical Profile"
description: |
  For healthcare, medical research with:
  - HIPAA compliance
  - PHI (Protected Health Info) handling
  - Audit requirements
  - Privacy focus

---

# Healthcare Profile Template

```yaml
---
id: hospital-system-hipaacompliant
name: "Hospital System - HIPAA Compliant"
tool: claude-code  # Recommended for sensitive
llm:
  provider: anthropic
  model: claude-sonnet
  context_window: 200000

capabilities:
  parallel_agents: false  # HIPAA = single agent
  terminal: true
  web_search: false
  logging: maximum
  audit_trail: immutable

constraints:
  - "hipaa-compliant"
  - "confidential-data"
  - "pii-strict"
  - "regional-us"  # US healthcare

security:
  phi_encryption: aes256
  phi_in_logs: redacted
  access_control: strict
  mfa: required

workflows:
  p3_approval: "required per file"
  p4_hipaa_audit: "mandatory"
  p5_compliance_review: "mandatory"
  p6_retention: "6 years"

audit:
  logging_level: "maximum"
  retention_days: 2190  # 6 years
  immutable: true
  access_logs: "detailed"
```

---

### **Template 4: "startup"**

For: Fast-moving startups, prototyping

```yaml
# File: .mip/profiles/templates/startup.md

---
id: startup-template
name: "Startup/Rapid Development"
description: |
  For startups, rapid prototyping:
  - Maximum speed
  - Minimal overhead
  - Cost-optimal
  - Parallel agents

---

# Startup Profile Template

```yaml
---
id: startup-mvp
name: "Startup - MVP Mode"
tool: claude-code
llm:
  provider: anthropic
  model: claude-sonnet
  context_window: 200000

capabilities:
  parallel_agents: true     # FULL parallelism
  terminal: true
  web_search: true
  multi_file_edit: true
  todo_write: true
  background_tasks: true

constraints: []             # No constraints, speed first

budget:
  monthly: "$2000"
  alerts: "at 80%"
  fallback: "swap to mistral-nemo"

workflows:
  p0_framing: "minimal"
  p3_autocode: "aggressive parallel"
  p4_testing: "auto"
  p5_human_review: "light"
  p6_archive: "quick"

optimization:
  agent_parallelism: 4      # 4 agents simultaneous
  context_reuse: true
  model_cache: true
  cost_mode: "speed>cost"
```

---

## Template System Details

### Creating Your Own Template

**File**: `.mip/profiles/templates/{name}.md`

**Structure**:

```yaml
---
id: my-template
name: "My Template Name"
description: |
  What is this template for?
  Who should use it?
  
placeholders:
  KEY1: "description (e.g., EU | US)"
  KEY2: "description"
  KEY3: "description"

---

# Template Content (with :PLACEHOLDER: values)

[Markdown explanation]

```yaml
[YAML with :PLACEHOLDER: values]
```

[Instructions]
```

### Placeholder Rules

```
:PLACEHOLDER_NAME: 
  - Used in YAML values
  - Replaced by user input
  - Can have default: :PLACEHOLDER_NAME=default_value:
  - Can be required or optional
```

**Example**:

```yaml
tool: :PREFERRED_LLM=mistral-nemo:
# Defaults to mistral-nemo if user doesn't replace
```

### Validation

Every template must have:

```yaml
required_placeholders:
  - COMPANY_NAME
  - TOOL_IDE

optional_placeholders:
  - BUDGET_LIMIT
  - EXTRA_CONSTRAINT
```

---

## Using Templates: Step-by-Step

### Step 1: List available templates

```bash
mip_profile templates list

# Output:
# Templates available:
#   - corporate (for enterprises)
#   - freelance (for solo/agencies)
#   - healthcare (HIPAA)
#   - startup (fast-moving)
#   - [custom templates you saved]
```

### Step 2: Select template

```bash
mip_profile create-from-template corporate

# Interactive mode:
#   "Choose template: [1/2/3/4] or type name"
#   User: "corporate"
```

### Step 3: Answer questions (SETUP-5 style)

```bash
Maria: "Creating profile from corporate template.

        Required info:
        1. Company name? (e.g., Acme, Google)"
        User: "Total"
        
        2. Tool/IDE? (Cursor / VS Code / other)"
        User: "VS Code"
        
        3. Region? (EU / US / China / APAC)"
        User: "EU"
        
        4. Data sensitivity? (high / medium / low)"
        User: "high"
        
        5. Budget per task? (e.g., $500)"
        User: "$1000"

Creating profile total-standard...
✅ Done
```

### Step 4: Review & edit (optional)

```bash
mip_profile edit total-standard
# Opens editor, user can fine-tune YAML

# Or validate before activation
mip_profile validate total-standard
# ✅ Valid, no conflicts
```

### Step 5: Activate

```bash
mip_profile total-standard
# ✅ Active: total-standard
# Constraints: legal-compliance, confidential-data, regional-eu, tool-locked-vscode
```

---

## Saving Your Own Template

Once you perfect a profile, save it as reusable template:

```bash
mip_profile save-as-template total-gdpr --name corporate-gdpr-template
# ✅ Saved to templates/corporate-gdpr-template.md
#    Can now be used by team members
```

---

## Template Best Practices

### Do ✅

- [ ] Use clear placeholder names (`:COMPANY_NAME:`, not `:X:`)
- [ ] Provide good descriptions
- [ ] Pre-fill common constraints
- [ ] Include example values
- [ ] Validate before sharing

### Don't ❌

- [ ] Hard-code sensitive values in template
- [ ] Use too many placeholders (keep < 10)
- [ ] Nest complex conditions in template
- [ ] Forget to document constraints applied

---

## Examples: Filled Templates

### Example 1: Total Energy (Corporate)

```
Input:
  COMPANY_NAME = Total
  TOOL_IDE = VS Code
  REGION = EU
  DATA_SENSITIVITY = high
  BUDGET_LIMIT = $2000/month

Output profile:
  ID: total-standard
  Base: mistral-nemo (cost-optimized)
  Constraints: legal-compliance, confidential-data, regional-eu, tool-locked-vscode
  Audit: Enabled (legal requirement)
  Budget: $2000/month, alert @ 80%
```

### Example 2: Freelancer (Flexible)

```
Input:
  CLIENT_NAME = Acme Corp
  PREFERRED_LLM = mistral
  BUDGET_CONSTRAINT = medium
  TOOL = VS Code

Output profile:
  ID: client-acme-dev
  Base: mistral-nemo (cost-conscious)
  Constraints: confidential-data (client's code)
  Budget: Pay-as-you-go or $500/month
  Fallback: Switch to free Llama local if over budget
```

### Example 3: Startup (Speed)

```
Input:
  STARTUP_NAME = MyAI
  TEAM_SIZE = 8
  BUDGET = "$3000/month"
  STRATEGY = "speed first"

Output profile:
  ID: myai-mvp
  Base: claude-sonnet (fastest)
  Constraints: None (speed first)
  Parallelism: 4 agents simultaneous
  Budget: $3000/month, auto-fallback to mistral
```

---

## Advanced: Conditional Templates

For complex scenarios, templates can have **conditional sections**:

```yaml
# Template with conditions

placeholders:
  REGION: "EU | US | China"

---

# Regional Configuration

IF :REGION: = EU:
  constraints:
    - gdpr-strict
    - regional-eu
  allowed_models:
    - mistral (France)
    - anthropic-eu

ELIF :REGION: = US:
  constraints:
    - ccpa
    - regional-us
  allowed_models:
    - openai
    - anthropic

ELIF :REGION: = China:
  constraints:
    - pipl
    - regional-china
  allowed_models:
    - baichuan (local)

ENDIF
```

---

## Sharing Templates

### With team (Git)

```bash
# Save template to version control
git add .mip/profiles/templates/my-template.md
git commit -m "Add corporate-gdpr template"
git push origin main

# Team members can use immediately
# mip_profile create-from-template corporate-gdpr
```

### With external users (Export)

```bash
# Export specific template
mip_profile export-template corporate-template \
  --format yaml > /tmp/corp-template.yaml

# Share file with external team
# They import:
# mip_profile import-template /tmp/corp-template.yaml
```

---

## References

- [MANAGEMENT.md](./MANAGEMENT.md) — How to CRUD profiles
- [CONSTRAINTS.md](./CONSTRAINTS.md) — What constraints available
- [INDEX.md](./INDEX.md) — All profiles directory
- [SETUP.md](../modules/setup.md#setup-5) — Integration with SETUP
