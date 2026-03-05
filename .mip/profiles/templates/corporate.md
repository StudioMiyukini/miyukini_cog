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