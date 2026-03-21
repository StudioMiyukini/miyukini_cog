# Miyukini Cloud - Risk Register

**Standard**: ISO/IEC 27001:2022 Clause 6.1 | **Version**: 1.0 | **Date**: 2026-03-20

## Risk Assessment Methodology

- **Likelihood**: Low (1) / Medium (2) / High (3)
- **Impact**: Low (1) / Medium (2) / High (3) / Critical (4)
- **Risk Score**: Likelihood x Impact
- **Treatment**: Accept / Mitigate / Transfer / Avoid

## Active Risks

| ID | Risk | Likelihood | Impact | Score | Treatment | Controls |
|---|---|---|---|---|---|---|
| R-001 | Unauthorized data access via application bypass | 1 | 4 | 4 | Mitigate | PostgreSQL RLS, RBAC, audit logging |
| R-002 | Data breach via stolen JWT token | 2 | 3 | 6 | Mitigate | Short-lived tokens (1h), token caching with 30s TTL, key rotation |
| R-003 | Malware upload via file sharing | 2 | 3 | 6 | Mitigate | MIME blocklist, magic byte detection, file type validation |
| R-004 | Data loss from storage corruption | 1 | 4 | 4 | Mitigate | SHA-256 dedup integrity, PostgreSQL WAL, encrypted backups |
| R-005 | Brute force attack on authentication | 2 | 2 | 4 | Mitigate | Rate limiting (10/60s), account lockout (5 failures/15min) |
| R-006 | SQL injection | 1 | 4 | 4 | Mitigate | sqlx parameterized queries (compile-time checked) |
| R-007 | Unencrypted data at rest (pre-migration) | 2 | 3 | 6 | Mitigate | AES-256-GCM encryption service, migration tool |
| R-008 | RGPD non-compliance (data retention) | 2 | 3 | 6 | Mitigate | GDPR rights API, consent management, retention policies |
| R-009 | Denial of service via large uploads | 2 | 2 | 4 | Mitigate | Bandwidth throttling, body size limits, streaming uploads |
| R-010 | Insider threat (admin abuse) | 1 | 3 | 3 | Mitigate | Admin action audit trail, principle of least privilege |

## Risk Acceptance Threshold

Risks with score <= 3 are accepted. Risks with score >= 4 require active mitigation controls.

## Review

This register is reviewed quarterly and after any security incident.
