---
id: regional-eu
name: "Regional EU Constraint"
category: regional
description: |
  EU data residency & GDPR compliance:
  - Data never leaves EU borders
  - GDPR-compatible infrastructure
  - Limited to EU-based LLM providers
  - EU encryption standards (TLS 1.2+)
  - Right to be forgotten (deletion in 30 days)
  
applies_to: all profiles
risk_level: medium
performance_impact: "Â±0% (option: slower EU servers)"
cost_impact: "+5-10% (EU infrastructure premium)"

---

# Regional EU Constraint

## What This Constraint Does

Enforces **EU data residency** with GDPR compliance:
- All data stays within EU borders (no US, no Asia)
- Only EU-based LLM providers (Mistral, not OpenAI/Claude)
- EU datacenters for storage (Germany, Ireland, France)
- GDPR-compliant processing
- 30-day auto-deletion on request

```yaml
regional_eu:
  enabled: true
  
  # 1. Geographic Restrictions
  geography:
    allowed_regions:
      - eu_west    # Ireland, UK
      - eu_central # Germany, Austria
      - eu_south   # France, Spain
    forbidden_regions:
      - us_east
      - us_west
      - asia
      - china
      
  # 2. Provider Whitelist
  providers:
    allowed:
      - mistral    # France-based
      - ollama     # Local, not subject to US law
    forbidden:
      - openai     # US-based
      - anthropic  # US-based
      - google     # US-based
      
  # 3. Data Handling
  data:
    storage_region: EU_ONLY
    transfer_tls_min: 1.2
    encryption: aes256
    retention_days: 30  # GDPR "right to be forgotten"
    backup_region: EU_ONLY  # EU datacenters only
    
  # 4. Legal Compliance
  legal:
    framework: GDPR
    dpa_required: true  # Data Processing Agreement
    schrems_ii_compliant: true  # US data transfer restrictions
    
  # 5. User Rights
  user_rights:
    data_access: true       # Right to access
    data_portability: true  # Right to export
    deletion: true          # Right to be forgotten
    deletion_timeline: 30   # Days to process
```

## Profile Effects

When `:regional-eu` is applied:

| Aspect | Normal | EU-Only |
|--------|--------|---------|
| **Provider** | Any (OpenAI, Claude, etc) | Mistral only |
| **Storage** | Global (US/EU/Asia) | EU only |
| **Transfer** | Plain HTTP possible | TLS 1.2+ required |
| **Deletion** | Manual request | Auto 30-40 days |
| **Legal** | Depends | GDPR certified |
| **Cost** | Cheapest (US APIs) | +5-10% (EU premium) |
| **Speed** | Varies | Consistent EU speeds |

## Allowed Providers

### âœ… Approved (EU-based)

```
Mistral:
  - Mistral Large
  - Mistral Medium
  - Mistral Small
  - Mistral OpenWeights
  Location: France (Paris)
  
Ollama (local):
  - Any local Mistral
  - Any local Llama
  - Any local open-source
  Location: Your machine (EU jurisdiction)
  
LM Studio (local):
  - Same as Ollama
  Location: Your machine (EU jurisdiction)
  
Aleph Alpha (Germany):
  - Luminous models
  Location: Heidelberg, Germany
```

### âŒ Blocked (US-based)

```
OpenAI:
  âœ— ChatGPT
  âœ— GPT-4
  âœ— GPT-3.5
  Reason: Servers in US (subject to Cloud Act)
  
Anthropic:
  âœ— Claude
  âœ— Claude Opus
  Reason: US-based, data may transfer to US
  
Google:
  âœ— Gemini
  âœ— PaLM
  Reason: US-based with broad data sharing
  
AWS Bedrock:
  âœ— Any model via Bedrock
  Reason: US-based (Virginia, Oregon)
```

## GDPR Compliance Details

### Data Processing Agreement (DPA)

```
Mistral GDPR compliance:
âœ… DPA signed with EU clients
âœ… GDPR Article 28 compliant (data processor)
âœ… SCCs (Standard Contractual Clauses) in place
âœ… Schrems II compliant (no US data transfer)
âœ… Data residency in EU
```

### Right to be Forgotten (Article 17)

```
User requests deletion:
1. User: "Delete my data"
2. Maria: "Scheduled for deletion in 30 days
           (mailbox will be purged 2025-02-15)"
3. After 30 days: Auto-purge from all systems
   - Mistral's servers
   - MIP local cache
   - Backups
   âœ… Complete deletion
```

### Data Portability (Article 20)

```
User requests export:
1. mip_profile export-data myprofile
2. System collects:
   - All operations history
   - Generated code
   - Logs
   - Settings
3. Output: Standard JSON format
4. User: Gets download link
   âœ… Transferable to another service
```

## Compatibility Matrix

```
âœ… Compatible with:
  - legal-compliance (GDPR + audit = perfect pair)
  - confidential-data (GDPR + confidential = fortress)
  - gdpr-strict (EU + GDPR = mandatory)
  - mistral-nemo (EU provider)

âš ï¸  Partial compatibility:
  - claude profile (Claude is US, constraint forces Mistral)
  - openai profile (OpenAI is US, constraint blocks)
  
âŒ Incompatible with:
  - regional-us (EU + US = conflict)
  - regional-china (EU + China = conflict)
  - us-only profiles
```

## Region Details

### EU-West (Ireland)

```yaml
locations:
  - Ireland (Dublin) â€” AWS EU Ireland
  - UK (London) â€” Post-Brexit still EU data
  
compliance:
  - GDPR (EU/Ireland law)
  - Data Protection Act 2018
  
providers:
  - AWS Ireland
  - Mistral Irish servers
  - Local setup
```

### EU-Central (Germany)

```yaml
locations:
  - Germany (Frankfurt) â€” Stricter than most EU
  - Austria (Vienna) â€” GDPR leader
  - Switzerland â€” Strong privacy laws
  
compliance:
  - GDPR (strictest in EU)
  - German BDSG (data protection law)
  - Swiss FADP
  
providers:
  - Aleph Alpha (Heidelberg)
  - Deutsche Telekom (OnCall subsidiary)
  - Local setup
```

### EU-South (France)

```yaml
locations:
  - France (Paris) â€” Mistral HQ
  - Spain (Madrid)
  - Italy (Rome)
  
compliance:
  - GDPR (France implement)
  - French CNIL (data protection authority)
  
providers:
  - Mistral (France)
  - Local setup
```

## Data Flow Diagram

```
Blocked (âŒ):
[User] ---> [US API] âœ— Cloud Act can force access
[User] ---> [Google] âœ— Broad data sharing
[User] ---> [AWS Bedrock] âœ— US jurisdiction

Allowed (âœ…):
[User] ---> [Mistral FR] âœ“ EU data residency
[User] ---> [Local Ollama] âœ“ No network, no transfer
[User] ---> [German Provider] âœ“ GDPR leader
```

## Activation

### Via Profile Creation

```bash
mip_profile create eu-safe \
  --base mistral-nemo \
  --constraints regional-eu
  
# Maria: "Applying regional-eu constraint.
#         Configuration:
#         âœ… Provider: Mistral (EU - France)
#         âœ… Storage: EU datacenters (Ireland/Germany)
#         âœ… Legal: GDPR compliant
#         âœ… Deletion: 30 days auto-purge
#         
#         Ready for EU-compliant work."
```

### Via Existing Profile

```bash
mip_profile apply-constraint myprofile regional-eu

# If using non-EU provider:
# âš ï¸  Warning: Claude detected (US-based)
#     Switching to Mistral (EU-based)
#     Performance: Similar (both high-quality)
```

## Workflow: GDPR-Compliant Project

### Example: French Healthcare Startup

```yaml
profile: france-healthtech
base: mistral-nemo
constraints:
  - regional-eu
  - legal-compliance
  - pii-strict  # Patient data
```

**workflow**:

```
Setup:
1. Mistral API (EU servers, France)
2. Data stored: Ireland (EU-West)
3. Backups: Germany (EU-Central)
4. Legal: GDPR DPA signed
5. Maria: "âœ… Ready for EU healthcare"

Operations:
1. Code for patient management system
2. All data stays in EU
3. All APIs called to Mistral (France)
4. Audit trail for compliance
5. On request: Data export in 5 days

Deletion:
1. Patient requests "right to be forgotten"
2. Data scheduled for purge
3. 30 days: Automatic deletion
4. Proof provided to patient
```

## Restrictions & Limitations

### Model Limitations

Mistral models vs Claude/GPT-4:

| Aspect | Mistral | Claude/GPT-4 | Trade-off |
|--------|---------|-------------|-----------|
| **Coding** | 85% as good | 100% | Minor loss |
| **Math** | 70% as good | 100% | Noticeable |
| **Reasoning** | 75% as good | 100% | Moderate |
| **Speed** | Faster | Slower | âœ… Win for Mistral |
| **Cost** | 10x cheaper | More expensive | âœ… Win for Mistral |
| **EU Compliance** | âœ… Yes | âŒ No | âœ… Win for legal |

**Recommendation**: Mistral is good enough for 90% of tasks + saves money + guarantees EU compliance.

### Speed Trade-offs

```
Normal (US APIs):
- OpenAI: ~2 sec per request (US servers)
- Claude: ~4 sec per request (optimized)

EU-Only (Mistral):
- Mistral: ~3 sec per request (EU servers)
- Local Ollama: ~10 sec per request (your hardware)

â†’ EU APIs are 50% as fast as Claude, but fully compliant
â†’ Local is 3x slower but 100% isolated
```

## Cost Comparison

| Provider | Location | Cost | EU Compliant |
|----------|----------|------|--------------|
| OpenAI | US | $0.003/1k | âŒ No |
| Claude | US | $0.003/1k input | âŒ No |
| Mistral API | France | $0.0014/1k | âœ… Yes |
| Ollama (free) | Local | $0 | âœ… Yes |

**Insight**: Mistral is 50% cheaper than OpenAI AND EU-compliant!

## Configuration Examples

### Example 1: Maximum EU Compliance

```yaml
regional_eu:
  regions: eu_central  # Germany (strictest)
  providers: mistral   # Single provider
  encryption: aes256-gcm  # Strongest
  tls_version: 1.3    # Latest
  backup_frequency: hourly
  audit_logging: maximum
  
result:
  âœ… Strictest EU interpretation
  âŒ But: Slower, more expensive
```

### Example 2: Pragmatic EU

```yaml
regional_eu:
  regions:
    - eu_west
    - eu_central
  providers:
    - mistral
    - ollama-local
  encryption: aes256  # Standard
  tls_version: 1.2   # Good enough
  
result:
  âœ… GDPR compliant
  âœ… Good performance
  âœ… Reasonable cost
```

### Example 3: EU with Fallback

```yaml
regional_eu:
  regions: eu_only
  primary_provider: mistral
  fallback: ollama-local  # If API fails
  
result:
  âœ… EU-first approach
  âœ… Redundancy (doesn't leave EU)
  âœ… No cost for fallback
```

## Troubleshooting

### Issue: "Mistral API not working"

```
Problem: Can't reach mistral API

Solution 1: Check region config
  $ mip_profile show regional-eu
  â†’ Verify: provider: mistral
  
Solution 2: Check internet (must be EU)
  If using VPN from outside EU, disconnect
  
Solution 3: Use local fallback
  $ ollama serve
  Change config: providers: [ollama]
```

### Issue: "Configuration conflict with Claude profile"

```
Error: "Claude (US) conflicts with regional-eu"

Solution: Switch to Mistral
  $ mip_profile apply-constraint my-profile regional-eu
  
  System auto-switches:
    claude-sonnet â†’ mistral-medium
    (Similar performance, EU-compliant)
```

### Issue: "Data deletion request takes forever"

```
Problem: Waiting 30 days for GDPR deletion

Solution 1: Check status
  $ mip_profile show-deletion-status
  â†’ Shows: "Feb 15 scheduled"
  
Solution 2: Manual purge (local only)
  $ mip_profile purge-local myprofile
  (Deletes local cache immediately, API takes 30d)
  
Solution 3: Contact Mistral support
  Legal@mistral.ai (can expedite)
```

## References

- [MANAGEMENT.md](..//..//README.md) â€” How to apply regional constraints
- [CONSTRAINTS.md](..//..//README.md) â€” Overview of all constraint types
- [Legal-Compliance Constraint](./legal-compliance.md) â€” When you need GDPR + audit
- [Mistral EU Documentation](https://docs.mistral.ai/) â€” Provider details
- [GDPR Official Text](https://gdpr-info.eu/) â€” Full regulation

