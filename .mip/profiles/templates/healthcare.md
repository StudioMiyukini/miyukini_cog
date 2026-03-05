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