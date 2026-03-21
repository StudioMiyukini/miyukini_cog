# Miyukini Cloud - Information Security Management System (ISMS) Policy

**Standard**: ISO/IEC 27001:2022 | **Version**: 1.0 | **Date**: 2026-03-20

## 1. Scope

This ISMS covers the Miyukini Cloud self-hosted cloud storage platform, including:
- File storage, sharing, and management services
- CalDAV/CardDAV/WebDAV protocol services
- Authentication and authorization systems
- Database infrastructure (PostgreSQL)
- Container deployment (Docker)

## 2. Information Security Objectives

1. **Confidentiality**: Protect user data through encryption at rest (AES-256-GCM), TLS in transit, and access controls (RBAC + RLS)
2. **Integrity**: Ensure data integrity via SHA-256 content-addressable storage, BLAKE3 hash-on-write, and database constraints
3. **Availability**: Maintain service availability through dual database pools, graceful degradation, and container orchestration

## 3. Security Controls Summary

| Control Area | Implementation | ISO 27001 Reference |
|---|---|---|
| Authentication | JWT HS256, Argon2id, OIDC/SSO, App Passwords | A.8.5 |
| Authorization | RBAC (admin/user), PostgreSQL RLS | A.8.3 |
| Encryption at rest | AES-256-GCM blob encryption | A.8.24 |
| Encryption in transit | TLS via reverse proxy, HSTS | A.8.24 |
| Audit logging | Persistent audit.events table | A.8.15, A.8.16 |
| Rate limiting | IP-based moka cache per endpoint | A.8.20 |
| Input validation | MIME blocklist, path traversal prevention, parameterized SQL | A.8.24 |
| Session management | HttpOnly cookies, CSRF double-submit, token expiration | A.8.5 |
| Incident response | Security incidents table, 72h RGPD workflow | A.5.24-28 |
| Key management | Auto-generated JWT secrets (0o600), rotation support | A.8.24 |
| Bandwidth control | Per-user throttling middleware | A.8.20 |

## 4. Risk Assessment

See [risk-register.md](risk-register.md) for the current risk register.

## 5. Roles and Responsibilities

- **Platform Administrator**: Manages user accounts, reviews audit logs, handles incidents
- **Data Protection Officer (DPO)**: Oversees RGPD compliance, DPIA assessments, breach notifications
- **Security Lead**: Reviews security controls, manages key rotation, vulnerability assessment

## 6. Review Schedule

This policy shall be reviewed:
- Annually (minimum)
- After any security incident
- When significant changes are made to the platform
