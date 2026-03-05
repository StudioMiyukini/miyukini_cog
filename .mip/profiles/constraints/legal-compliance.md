---
id: legal-compliance
name: "Legal & Compliance Constraint"
category: legal
description: |
  Enforces legal/compliance requirements:
  - Audit logging of all operations
  - Consent checkpoints for data processing
  - Data retention policies (30 days minimum logging)
  - Immutable audit trails
  - Compliance reporting
  
applies_to: all profiles
risk_level: medium
performance_impact: "+15% overhead (logging)"
cost_impact: "$0.05-0.10 per task (storage)"

---

# Legal & Compliance Constraint

## What This Constraint Does

Adds mandatory **audit trail**, **consent mechanisms**, and **compliance logging** to any profile.

```yaml
legal_compliance:
  enabled: true
  
  # 1. Audit Trail
  audit_logging:
    level: detailed
    format: json
    destination: "audit-logs/"
    rotation: daily
    retention_days: 30
    immutable: true
    signed: true  # Cryptographically signed logs
    
  # 2. Consent Checkpoints
  consent:
    data_processing: required
    external_api: required
    ai_model_selection: required
    geographical_storage: required
    third_party_access: required
    
  # 3. Data Handling
  data:
    retention_days: 30
    auto_purge: true
    encryption: aes256
    at_rest: mandatory
    in_transit: tls1.2_minimum
    
  # 4. Reporting
  reporting:
    daily_summary: automated
    monthly_compliance: automated
    audit_export: json
    
  # 5. Access Control
  access:
    mfa: optional
    ip_whitelist: false  # Unless specified per profile
    session_timeout: 8h
    user_logging: all_actions
```

## Profile Effects

When `:legal-compliance` is applied to a profile:

| Aspect | Effect |
|--------|--------|
| **Logging** | All operations logged to audit-logs/ |
| **Startup** | Required consent for data processing before first operation |
| **API Calls** | Consent required before calling external APIs |
| **Model Selection** | Consent required before switching LLMs |
| **Data Storage** | All data encrypted AT REST + IN TRANSIT |
| **Data Deletion** | Automatic purge after 30 days (GDPR friendly) |
| **Reporting** | Daily summaries, monthly compliance reports |
| **Audit Export** | Can export full audit trail in JSON |
| **Performance** | +15% overhead (logging operations) |
| **Cost** | +$0.05-0.10 per task (storage/compliance) |

## Compatibility Matrix

```
âœ… Compatible with:
  - confidential-data (legal + confidential = full control)
  - pii-strict (legal + PII = GDPR-ready)
  - regional-* (legal + region = compliance per jurisdiction)
  - tool-locked-* (legal + tool = controlled environment)
  - All LLM profiles (mistral, claude, copilot, etc.)

âš ï¸  Conflicts with:
  - None (legal-compliance is universally applicable)

âŒ Incompatible with:
  - None
```

## Implementation Details

### Audit Log Format

```json
{
  "timestamp": "2025-01-15T10:30:00Z",
  "user": "developer@company.com",
  "action": "task_completed",
  "task_id": "T3-2025-001",
  "llm_provider": "mistral-nemo",
  "tokens_used": 12500,
  "data_accessed": ["customer_db", "product_catalog"],
  "external_calls": ["stripe_api", "analytics"],
  "consent_given": ["data_processing", "external_api"],
  "outcome": "success",
  "signature": "sha256:abc123..."
}
```

### Consent Flow

```
1. Profile activated with legal-compliance
2. Maria: "This profile requires legal compliance tracking.
           Do you consent to:
           - Audit logging of all operations? (Y/n)"
3. User confirms each checkpoint
4. Logs stored in audit-logs/
5. Cannot be deleted (immutable)
6. Auto-exported monthly
```

## Use Cases

### Case 1: Financial Services (Banking)

```yaml
base_profile: mistral-nemo
constraints:
  - legal-compliance
  - confidential-data
  - regional-us
```

**result**:
- All transactions logged â†’ audit trail
- 30-day retention â†’ SOX compliance
- Immutable logs â†’ court-ready
- Regional US â†’ no data export

### Case 2: Healthcare (Medical)

```yaml
base_profile: claude-sonnet
constraints:
  - legal-compliance
  - pii-strict
  - hipaa-compliant
```

**result**:
- Full audit trail â†’ HIPAA ready (6 years)
- PII redaction â†’ privacy-safe
- Consent before PHI access â†’ compliance
- Immutable records â†’ legal protection

### Case 3: EU Enterprise (GDPR)

```yaml
base_profile: mistral-nemo
constraints:
  - legal-compliance
  - gdpr-strict
  - confidential-data
  - regional-eu
```

**result**:
- Audit trail â†’ Data Processing Agreement (DPA) ready
- 30-day auto-purge â†’ Right to be forgotten
- Consent tracking â†’ Consent management
- EU region â†’ No US data transfer

## Activation

### Via Template

```bash
# Use corporate template (includes legal-compliance)
mip_profile create-from-template corporate --name acme-gdpr

# Template already applies legal-compliance + regional-eu
```

### Via CRUD

```bash
# Start with base profile
mip_profile create myprofile --base mistral-nemo

# Add constraint
mip_profile apply-constraint myprofile legal-compliance

# Maria asks for consent
# âœ… Active constraints: legal-compliance
```

### Via CLI

```bash
mip_profile create myprofile \
  --base mistral-nemo \
  --constraints legal-compliance,confidential-data
```

### Via YAML

```yaml
# Direct YAML edit
constraints:
  - legal-compliance
  
legal_compliance:
  retention_days: 30
  immutable: true
  audit_signing: true
```

## Audit Export

### Monthly Report

```bash
mip_profile export-audit myprofile --period "2025-01"

# Output: audit_export_2025-01.json
# Contains:
#   - All logged operations
#   - Consent records
#   - Data access history
#   - Cost breakdown
#   - Compliance status
```

### Compliance Checklist

```bash
mip_profile compliance-check myprofile

# Output:
# âœ… Legal compliance enabled
# âœ… Audit logging active
# âœ… Immutable logs confirmed
# âœ… 30-day retention active
# âœ… Encryption verified
# âš ï¸  Export pending (monthly)
```

## Configuration Options

```yaml
legal_compliance:
  # Customize retention period (30 days min)
  retention_days: 30  # 90, 365, 2190 (6 years), etc.
  
  # Customize audit level
  audit_level: detailed  # minimal, standard, detailed, maximum
  
  # Require consent for all operations
  strict_consent: false  # true = consent every operation
  
  # Digital signatures on logs
  log_signing: true
  signature_algorithm: sha256
  
  # Auto-export frequency
  export_frequency: monthly  # daily, weekly, monthly, yearly
  
  # Compliance frameworks supported
  frameworks:
    - GDPR
    - HIPAA
    - SOX
    - CCPA
    - PIPL
```

## Performance Considerations

### Overhead

| Operation | Overhead | Impact |
|-----------|----------|--------|
| Task start | +50ms | Consent checkpoint |
| Per operation | +2ms | Audit log write |
| Month end | +500ms | Log rotation/export |
| Disk usage | +5GB/month | ~10k operations |

### Optimization

```yaml
# Batch logging (reduce write overhead)
audit:
  batch_size: 100
  flush_interval: 60s
  
# Compression (reduce disk usage)
audit:
  compression: gzip
  
# Archival (move old logs to cold storage)
audit:
  archive_after_days: 30
  archive_destination: s3://compliance-vault/
```

## Troubleshooting

### Issue: "Consent checkpoint appears every time"

â†’ Solution: Set `strict_consent: false` (default)

```yaml
legal_compliance:
  strict_consent: false
  # Consent remembered for 24h
```

### Issue: "Audit logs taking too much disk"

â†’ Solution: Reduce retention or enable compression

```yaml
legal_compliance:
  retention_days: 7  # Down from 30
  compression: gzip  # ~80% space savings
```

### Issue: "Can't delete a log I need removed"

â†’ Solution: Immutable by design (privacy protection)

Use audit redaction instead:

```bash
mip_profile redact-audit myprofile \
  --operation "2025-01-15T10:30:00Z" \
  --pii-fields customer_id,email
```

## References

- [MANAGEMENT.md](..//..//README.md) â€” How to apply constraints
- [CONSTRAINTS.md](..//..//README.md) â€” All constraint types
- [Compliance Frameworks](https://en.wikipedia.org/wiki/Comparison_of_information_governance_regulations)

