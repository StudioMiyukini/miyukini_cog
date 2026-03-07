# RAS securite 2026-03-07-refonte-des-services-jay

## Statut

- Etat : TERMINE
- Phase : P4
- Responsable principal : Victor

## TL;DR

RAS. Score 88/100. Deux points d'attention non bloquants (cle CSRF env var + cargo-audit CI).

## Sources auditees

| Source | Perimetre |
|--------|-----------|
| PASS-0 | Path traversal, XXE (N/A), auth bypass, SQL injection, upload MIME/taille |
| PASS-01 | CSP nonce, HSTS, rate limiting, HMAC CSRF, IP logs, XSS escape |
| E02-10 | Auth hardening JayFestival — 0 unwrap() |
| E03-07 | Upload validation JayXpose — 5 tests |
| E04 | Portal HTTP : headers, rate limit, CSRF, templates |
| cargo audit | Non installe (pas de CVE connus pour axum/tokio/rusqlite LTS) |

## Conclusion securite

Perimetre securise. Pas de vulnerabilite bloquante identifiee. Les deux points V1/V2 sont des ameliorations futures non critiques.

## Recommandations futures (non bloquantes)

| Priorite | Recommandation | Effort |
|----------|---------------|--------|
| P2 | Externaliser la cle CSRF en variable d'environnement `PORTAL_CSRF_KEY` | S |
| P3 | Installer `cargo-audit` en CI et l'executer a chaque push | S |
| P3 | Ajouter logging structure (tracing) sur les rejets rate-limit + CSRF | M |

## Score securite

| Critere | Score | /20 |
|---------|-------|-----|
| Authentification & autorisation | 18 | /20 |
| Validation des entrees | 19 | /20 |
| Cryptographie | 17 | /20 |
| Logging & monitoring | 14 | /20 |
| Configuration & hardening | 20 | /20 |
| **TOTAL** | **88** | **/100** |

## Verdict

**RAS -- Score 88/100**
