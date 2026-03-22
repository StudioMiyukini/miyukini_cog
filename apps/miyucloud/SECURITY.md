# Security Policy — Miyukini Cloud

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.5.x   | Yes       |
| < 0.5   | No        |

## Reporting a Vulnerability

If you discover a security vulnerability in Miyukini Cloud, please report it responsibly:

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email: **security@miyukini-home.org**
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (if any)

We will acknowledge receipt within **48 hours** and provide a timeline for resolution within **5 business days**.

## Security Architecture

### Authentication
- JWT HS256 with BLAKE3 validation cache
- Argon2id password hashing (64 MiB, 3 iterations, 2 lanes)
- Account lockout: 5 consecutive failures → 15-minute lockout
- Rate limiting: login (10/60s), register (5/3600s), refresh (20/60s)
- OIDC/SSO support with PKCE and nonce validation
- JWT key rotation via `MIYUCLOUD_JWT_SECRET_PREVIOUS`

### Authorization
- Role-Based Access Control (admin / user)
- PostgreSQL Row-Level Security on all data tables
- IDOR protection on file uploads (folder ownership verification)

### Encryption
- **In transit**: TLS 1.2+ via rustls, HSTS with preload
- **At rest**: AES-256-GCM blob encryption (optional, per-deployment)
- **Passwords**: Argon2id (no reversible encryption)

### Data Protection (GDPR/RGPD)
- Data export (Art. 15 + Art. 20 portability)
- Data erasure with audit anonymization (Art. 17)
- Data rectification (Art. 16)
- Consent tracking with versioning (Art. 6-7)
- 72-hour breach notification workflow (Art. 33-34)

### Audit & Monitoring
- Persistent audit log (`audit.events` table)
- Indexed by timestamp, user, action, outcome
- Batch async writes (non-blocking request path)
- Audit actions: login, upload, download, delete, admin operations, GDPR requests

### Input Validation
- MIME type blocklist with magic byte detection
- SQL injection prevention (sqlx compile-time checked parameterized queries)
- Path traversal prevention (filename sanitization)
- CSRF double-submit cookie pattern
- Content Security Policy with restrictive defaults

### Session Management
- HttpOnly, Secure, SameSite=Lax cookies
- Absolute session timeout (24h default)
- Session cleanup background task
- Session revocation on password change / account lockout

## Compliance Alignment

| Standard | Status |
|----------|--------|
| ISO/IEC 27001 | Partial — ISMS documented, controls implemented |
| RGPD (GDPR) | Implemented — Art. 15-20 rights, consent, breach notification |
| HDS (Healthcare) | In preparation — requires ISO 27001 base + encryption at rest |

## Dependency Security

- All dependencies are pinned versions in `Cargo.lock`
- TLS uses `rustls` (memory-safe, no OpenSSL)
- No `unsafe` code in application layer
- `cargo audit` should be run regularly

## Disclosure Timeline

- **Day 0**: Vulnerability reported
- **Day 2**: Acknowledgment sent
- **Day 5**: Assessment and timeline communicated
- **Day 30**: Fix released (or interim mitigation)
- **Day 90**: Public disclosure (coordinated)
