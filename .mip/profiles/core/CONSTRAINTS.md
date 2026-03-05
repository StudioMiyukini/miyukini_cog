---
id: mip.profiles.constraints
title: Constraint Library — Legal, Compliance, Confidentiality
---

# Constraint Library

> Profiles can be **layered with constraints** (legal, confidential, tool-specific, regional). This doc defines all available constraints and their effects.

---

## Overview : Constraint Types

```
┌─────────────────────────────────────────────────────────┐
│ Constraint Categories                                   │
├─────────────────────────────────────────────────────────┤
│ 1. Legal & Compliance     (GDPR, SOC2, HIPAA, CCPA)    │
│ 2. Confidentiality & Data (PII, secrets, no-logs)      │
│ 3. Tool & Environment     (IDE, OS, network)           │
│ 4. Regional & Data Residency (EU, US, China, APAC)     │
│ 5. Custom Constraints     (user-defined)               │
└─────────────────────────────────────────────────────────┘
```

---

## 1. Legal & Compliance Constraints

### `legal-compliance`

**Applies to: All profiles**

**Effects:**

```yaml
id: legal-compliance
applies_to: [all]

settings:
  audit_logging: enabled      # All decisions logged
  consent_required: true      # User approval per sensitive action
  data_retention: 30 days     # Auto-purge logs after 30d
  encryption: required        # All data in transit TLS
  data_subject_rights: true   # User can request data export/delete

  compliance_frameworks:
    - gdpr: enabled
    - hipaa: optional
    - ccpa: optional
    - sox: optional
    - pci_dss: optional
    - iso27001: optional

overrides:
  capabilities:
    web_search: ask_per_query    # Not automatic
    terminal_output_logging: enabled
    model_eval: disabled         # Don't train on results
    persistent_cache: 30day_ttl  # Cache expires
  
  workflows:
    p3_checkpoint: mandatory     # Approval before P3 auto-execute
    p4_audit: mandatory          # Audit report required
    human_review: required       # Human approval in P5

warnings:
  - "Audit logging enabled. Performance -5%"
  - "Consent checkpoints may slow workflow"
  - "Data retention = 30 days (check compliance reqs)"
```

**User impact:**

```
Task starts:
  Maria: "LEGAL-COMPLIANCE constraint active.
          
          This requires:
          - Audit log (all decisions)
          - Consent checkpoints
          - Data expires in 30 days
          
          Continue? [y/N]"

During P3:
  "⚠️  P3 checkpoint: Approve implementation? [y/N]"

During P4:
  "Generating audit report...
   ✅ Done (saved to .mip/audits/)"

End:
  "Task archived. Auto-delete in 30 days."
```

---

### `gdpr-strict`

**Applies to: EU-based users / EU data**

**Extends: legal-compliance**

```yaml
id: gdpr-strict
extends: legal-compliance

settings:
  gdpr:
    right_to_be_forgotten: enabled   # User can delete
    data_portability: enabled        # User can export
    consent_granular: required       # Per-feature opt-in
    breach_notification: mandatory   # Report within 72h
    dpa: required                    # Data processor agreement
    international_transfer: forbidden # No data outside EU
  
  data_residency: eu_only
  subprocessors: forbidden            # No 3rd party APIs without consent

overrides:
  capabilities:
    api_calls: blocked               # No external APIs without explicit approval
    web_search: blocked              # No external web
    data_export: logged              # Every export logged
    
  tools:
    allowed_llm_providers:
      - anthropic (datacenters in EU region)
      - mistral (France-based)
    forbidden:
      - openai (US-based)
      - google (US-based)

constraints:
  - "No PII in prompts"
  - "No model training on data"
  - "No caching across users"
```

**When applied:**

```
Total employee (EU project):
  mip_profile apply-constraint my-setup gdpr-strict
  
Result:
  - Claude Code allowed (EU data centers)
  - Mistral allowed (France)
  - OpenAI blocked (US)
  - Web search blocked
  - All data purged 30d
  - Audit trail mandatory
```

---

### `hipaa-compliant`

**For: Healthcare, medical data**

```yaml
id: hipaa-compliant
extends: legal-compliance

settings:
  hipaa:
    phi_handling: required           # Protected health info rules
    access_logs: mandatory           # Who accessed what, when
    encryption_at_rest: aes256       # Minimum encryption
    minimum_tls: 1.2
    business_associate: required     # BAA signed
    breach_notification: immediate   # Report < 60 min
    audit_retention: 6 years

overrides:
  capabilities:
    logging: maximum                 # Every action logged
    web_search: blocked              # No external APIs
    multi_file_edit: disabled        # Edit one at a time (trackable)
    parallel_agents: max_1           # Single agent only
    
  workflows:
    p3_approval: required_per_file
    p4_security_audit: mandatory
    p5_compliance_review: mandatory

restrictions:
  - "No de-identification"
  - "No external processing"
  - "Encrypted channels only"
  - "Access control strict"
```

---

### `hipaa-minimal-risk`

**For: Non-critical healthcare data**

```yaml
id: hipaa-minimal-risk
extends: legal-compliance (not hipaa-compliant)

settings:
  hipaa:
    phi_de_identified: enabled       # Can work with de-id data
    encryption_tls: required
    access_logs: required
    audit_retention: 3 years

overrides:
  capabilities:
    web_search: ask_per_use          # Can search, logged
    multi_file_edit: enabled
    parallel_agents: enabled
    
  restrictions:
    - "No raw names"
    - "No raw SSNs"
    - "No contact info"
```

---

### `sox-financials`

**For: Finance / Securities / Audited companies**

```yaml
id: sox-financials
extends: legal-compliance

settings:
  sox:
    transaction_logging: mandatory   # Every calc logged
    segregation_of_duties: required  # Approval chain
    access_controls: strict
    audit_trail: immutable           # Cannot be deleted
    retention: 7 years

overrides:
  workflows:
    p3_approval: required
    p4_independent_verification: mandatory
    p5_manager_sign_off: mandatory
    
  restrictions:
    - "No parallel agents" (SoD)
    - "Change log immutable"
    - "All calculations auditable"
```

---

## 2. Confidentiality & Data Constraints

### `confidential-data`

**Applies to: Sensitive/proprietary data**

```yaml
id: confidential-data
applies_to: [all]

settings:
  logging:
    level: off                       # Minimal logging
    input_logging: false             # No prompt caching
    output_logging: false
    
  caching:
    prompt_cache: disabled           # No API-side cache
    local_cache: 24h_ttl             # Auto-delete
    
  data_handling:
    no_model_training: required      # Opt-out required on APIs
    no_telemetry: required
    no_profiling: required
    encryption_end_to_end: required
    
  ai_model_limits:
    max_context_tokens: 50000        # Smaller context, less surface
    no_multimodal: true              # Text + code only, no images
    
  export_restrictions:
    - "Can only save to local disk"
    - "Cannot copy to external services"
    - "Manual approval required"

overrides:
  capabilities:
    web_search: disabled             # No external
    ai_agent_tools: blocked          # No external API calls
    clipboard_access: ask            # Confirm each paste
    
  workflows:
    manual_file_edits: required      # User applies, not auto
    human_verification: required     # All outputs reviewed
```

**When applied:**

```
Proprietary code:
  mip_profile apply-constraint my-setup confidential-data
  
Effect:
  - No caching (each query fresh)
  - No logging
  - No external APIs
  - User manually applies changes
  - Output never leaves local disk
```

---

### `pii-strict`

**For: Processing PII (names, SSN, contacts, etc.)**

```yaml
id: pii-strict
extends: confidential-data

settings:
  pii_handling:
    consent_required: true           # User must consent
    retention: minimal               # Purge ASAP
    export_forbidden: true
    sharing_forbidden: true
    
  detection:
    pii_scanner: enabled             # Detect PII in prompts, block
    alert_on_pii: true               # Warn user
    
  restrictions:
    - "No sending PII to API endpoints"
    - "No logging PII"
    - "No caching PII"
    - "Auto-redact in outputs"
```

---

### `secrets-vault`

**For: API keys, passwords, credentials**

```yaml
id: secrets-vault
extends: confidential-data

settings:
  secrets:
    no_plaintext: required           # Must be encrypted
    no_logging: absolute
    no_api_calls: required           # Keep local
    vault_integration: required      # Use HashiCorp, AWS Secrets, etc.
    
  detection:
    secret_scanner: enabled          # Detect keys, block
    entropy_check: enabled           # Block high-entropy strings
    
  restrictions:
    - "No prompting with secrets"
    - "No model output with secrets"
    - "Secrets in files only, encrypted"
    - "Manual key rotation required"
```

---

## 3. Tool & Environment Constraints

### `tool-locked-cursor`

**Constraint: Must use Cursor IDE**

```yaml
id: tool-locked-cursor
applies_to: [all]

settings:
  ide:
    required: Cursor
    min_version: 1.5
    
  fallback: fail                     # Don't degrade, fail loudly

overrides:
  capabilities:
    native_editor_integration: enabled
    composer_mode: required
    
  warnings:
    - "IDE not Cursor. Profile may not work."
```

**Runtime check:**

```
User activates profile:
  mip_profile total-gdpr

Check: Current IDE = Cursor?
  ✗ NO (User has VS Code)
  
Warning:
  "⚠️  Profile total-gdpr locked to Cursor.
   You're using VS Code.
   Some features may not work.
   
   Options:
     1. Switch to Cursor
     2. Use different profile
     3. Continue anyway [risky]
   
   Choice [1/2/3]:"
```

---

### `tool-locked-vscode`

**Constraint: Must use VS Code**

```yaml
id: tool-locked-vscode
applies_to: [all, copilot, cline, continue]

settings:
  ide:
    required: VS Code
    extensions: [GitHub Copilot, Cline]
    min_version: 1.80
```

---

### `offline-only`

**Constraint: No external network, local inference only**

```yaml
id: offline-only
applies_to: [local_llms]

settings:
  network:
    external_api: forbidden
    web_search: forbidden
    model_download: forbidden        # Models must be pre-loaded
    
  dns:
    enabled: false                   # No DNS queries
    
  storage:
    cloud_sync: disabled
    
  model_requirements:
    local_inference: required        # Llama, GGUF only
    min_context: 32000               # Enough for complex tasks
    quantization: q4_k_m             # Recommend efficient

overrides:
  capabilities:
    web_search: offline              # Use local docs only
    api_calls: forbidden
    mcp: disabled                    # No external servers
    
  workflows:
    p3_autocode: limited             # GGUF slower, expect delays
    p4_testing: terminal_only        # Local shell

workflow_adaptations:
  turbo_mode: disabled               # No cloud inference
  estimated_slowdown: 10-50x         # CPU bound
  
  recommendations:
    - "Pre-download Miyukini README offline"
    - "Use grep/sed instead of semantic search"
    - "Schedule long tasks for CPU idle"
    - "Disk space: ~20GB for model + cache"
```

**When active:**

```
User applies offline-only:
  No external calls
  No web search
  No model downloads
  Terminal only for execution
  
Expected delays:
  - Slower inference (1 tok/sec local vs 100 token/sec API)
  - No real-time docs
  - All answers from local memory
  
Benefit:
  - Completely autonomous (no internet)
  - Privacy perfect (0 data leaks)
  - Compliance strong (no external parties)
```

---

## 4. Regional & Data Residency Constraints

### `regional-eu`

**EU data residency + GDPR**

```yaml
id: regional-eu
extends: gdpr-strict

settings:
  region: european_union
  data_residency: eu_only            # Data never leaves EU
  
  allowed_providers:
    anthropic:
      datacenters: [eu-france, eu-ireland]
    mistral:
      datacenters: [france]
    others:
      - forbidden
      
  forbidden_providers:
    - openai (US)
    - google (US)
    - deepseek (China)
    - bbai (Israel)

restrictions:
  - "No transatlantic data transfers"
  - "No China model APIs"
  - "GDPR compliance mandatory"
  - "French/Irish datacenters only"
```

---

### `regional-us`

**US data residency + CCPA**

```yaml
id: regional-us
extends: legal-compliance (ccpa module)

settings:
  region: united_states
  data_residency: us_only
  
  allowed_providers:
    openai:
      datacenters: [us-west, us-east]
    anthropic:
      datacenters: [us]
    google:
      datacenters: [us]
```

---

### `regional-china`

**China data residency (PIPL compliance)**

```yaml
id: regional-china
extends: legal-compliance

settings:
  region: china
  data_residency: china_only
  
  restrictions:
    - "No international data transfers"
    - "PIPL compliance"
    - "Content filter for sensitive topics"
    - "Local model required or authorized partner"
    
  allowed_models:
    - baichuan (local)
    - alibaba_tonyi (local)
    - bytedance_doubao (local)
```

---

### `regional-apac`

**Asia-pacific multi-country**

```yaml
id: regional-apac
applies_to: [multi-asia-operations]

settings:
  regions_allowed:
    - singapore (data hub)
    - japan
    - south_korea
    - australia
    
  restrictions:
    - "Singapore residency preferred"
    - "No mainland China data center"
    - "PDPA (Singapore) + local laws"
```

---

## 5. Custom Constraints

Users can define custom:

```yaml
# File: .mip/profiles/constraints/custom-constraints.md

---
id: custom.my-constraint
name: My Custom Constraint
applies_to: [custom_profiles_only]

description: |
  My company-specific constraint for project XYZ.
  Enforce special approval requirements.

settings:
  approval_process: email_cto       # CTO email approval required
  budget_limit: 100                 # Max $100 per task
  tool_requirement: "Slack notif"   # Notify Slack on completion
  
  max_tokens:
    per_task: 50000
    per_day: 500000
    
  allowed_models:
    - mistral-nemo
  forbidden_models:
    - openai (cost)
    - google (latency)

custom_workflows:
  p0_approval: slack_channel        # Must be approved in #ai-tasks
  p3_checkpoint: email              # Email when P3 starts
  p4_security: None                 # Skip (low risk)
  p5_delivery: manual               # Always human review

overrides:
  capabilities:
    web_search: disabled
    parallel_agents: max_2
```

---

## Constraint Matrix (Quick Reference)

| Constraint | Legal | Audit | Confidential | Offline | API |
|-----------|-------|-------|-------------|---------|-----|
| **legal-compliance** | ✅ | ✅ | ⚠️ | ❌ | ✅ |
| **gdpr-strict** | ✅ | ✅ | ✅ | ⚠️ | ✅ (EU only) |
| **hipaa-compliant** | ✅ | ✅ | ✅ | ❌ | ✅ (audit) |
| **sox-financials** | ✅ | ✅ | ⚠️ | ❌ | ✅ (locked) |
| **confidential-data** | ❌ | ✅ | ✅ | ✅ | ❌ |
| **pii-strict** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **secrets-vault** | ❌ | ✅ | ✅ | ✅ | ❌ |
| **tool-locked** | ❌ | ❌ | ❌ | ❌ | IDE-dep |
| **offline-only** | ❌ | ❌ | ✅ | ✅ | ❌ |
| **regional-eu** | ✅ | ✅ | ✅ | ⚠️ | ✅ (EU) |
| **regional-us** | ✅ | ✅ | ⚠️ | ⚠️ | ✅ (US) |
| **regional-china** | ✅ | ✅ | ✅ | ⚠️ | ⚠️ |
| **custom** | ? | ? | ? | ? | ? |

---

## Conflict Resolution

If constraints conflict:

```bash
# Example: regional-eu + regional-us (CONFLIET!)
mip_profile apply-constraint temp-profile \
  regional-eu \
  regional-us

# Error: CONFLICT
# regional-eu: Data must stay in EU
# regional-us: Data must stay in US
# 
# Resolution:
#   1. Remove one constraint
#   2. Choose different regions
#   3. Use multi-region profile (if defined)
```

---

## Performance Impact

```
Base profile (no constraints)
  Task T3: 1 hour
  Cost: €5
  
+ legal-compliance
  Task T3: 1h 15min (+15%)
  Cost: €5.50
  (Audit logging + consent checks)
  
+ confidential-data
  Task T3: 1h 20min (+25%)
  Cost: €5
  (No caching, manual edits)
  
+ legal + confidential + gdpr-strict
  Task T3: 2 hours (+100% !)
  Cost: €6
  (Audit + consent + EU-only + manual)
```

---

## Recommendations

### For Startups
```
Default: No constraints
Optional per task: confidential-data (if proprietary)
```

### For SMEs (European)
```
Standard stack:
  Base: mistral-nemo
  + legal-compliance
  + confidential-data
  + regional-eu
```

### For Enterprises (Healthcare)
```
Standard stack:
  Base: anthropic-opus
  + hipaa-compliant
  + gdpr-strict
  + sox-financials
  + tool-locked-cursor
```

### For Freelancers
```
Flexible:
  Default: No constraints
  Per-client: Apply custom stack
  Example client-a: + confidential-data + tool-locked-vscode
```

---

## Check & Verify

```bash
# View all constraints
mip_profile constraints list

# Show specific constraint details
mip_profile constraints show legal-compliance

# Validate profile + constraints
mip_profile validate my-setup

# Show conflicts
mip_profile check-conflicts my-setup
# No conflicts ✅

# Simulate applying constraint
mip_profile apply-constraint --dry-run my-setup gdpr-strict
# Would apply gdpr-strict
# Conflicts: None
# Impact: +20% time, €1 cost
# OK to apply? [y/N]
```

---

## References

- [MANAGEMENT.md](./MANAGEMENT.md) — How to use constraints
- [TEMPLATES.md](./TEMPLATES.md) — Create custom profiles
- [INDEX.md](./INDEX.md) — Profile directory
- [builtin/](./builtin/) — Base profiles
- [custom/](./custom/) — User profiles (not versioned)
