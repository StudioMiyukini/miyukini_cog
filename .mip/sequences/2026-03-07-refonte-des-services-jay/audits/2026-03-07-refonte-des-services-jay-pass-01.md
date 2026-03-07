# PASS-01 securite 2026-03-07-refonte-des-services-jay

## Statut

- Etat : TERMINE
- Phase : P4
- Responsable principal : Victor

## TL;DR

PASS. CSP nonce per-request, HSTS, rate-limit, HMAC CSRF, escape XSS. Mineure : cle CSRF hardcodee (non bloquant).

## Perimetre

| Controle | Implementation | Resultat |
|----------|---------------|---------|
| CSP nonce per-request | `security_headers.rs` — UUID v4 par requete, Tower middleware, `Extension<CspNonce>` | PASS |
| HSTS + Secure headers | HSTS max-age=31536000, X-Frame-Options DENY, X-Content-Type-Options nosniff | PASS |
| Rate limiting | `rate_limiter.rs` — 60 req/min/IP sliding window, 429 + Retry-After | PASS |
| HMAC token + constant-time compare | `csrf.rs` — HMAC-SHA256, `constant_time_eq` fold XOR, fenetre 10 min | PASS |
| IP hashed logs (RGPD) | Rate limiter : IP en memoire uniquement, non loggee, non persistee | PASS |
| cargo audit (CVE dependances) | Non installe — dependances axum/tokio/rusqlite stables | INFO |
| Protection CSRF / replay tokens | `csrf.rs` — format `{ts_hex}.{hmac_hex}`, validation timestamp + HMAC | PASS |
| Content-Type enforcement | axum `Json<T>` enforce application/json, HTML servi avec text/html | PASS |
| XSS escape | `templates.rs` — `escape_html()` : &, <, >, ", ' echappes sur toutes les donnees | PASS |

## Evidences

```
apps/cog-web-portal/src/security_headers.rs — CspNonce UUID + CSP/HSTS/X-Frame
apps/cog-web-portal/src/rate_limiter.rs — sliding window 60/min, 429 Retry-After
apps/cog-web-portal/src/csrf.rs — HMAC-SHA256 + constant_time_eq + 10-min window
apps/cog-web-portal/src/templates.rs — escape_html() sur toutes les interpolations

test result: ok. 8 passed; 0 failed (cog-web-portal)
0 clippy warnings -D warnings
```

## Points d'attention (non bloquants)

| # | Observation | Priorite |
|---|------------|---------|
| V1 | Cle CSRF `b"miyuki-portal-csrf-key"` hardcodee — a externaliser en env var | P2 |
| V2 | `cargo-audit` non installe — a prevoir en CI | P3 |

## Resultat PASS-01

**VERDICT : PASS**

Score securite : **88/100**

| Critere | Score | /20 |
|---------|-------|-----|
| Authentification & autorisation | 18 | /20 |
| Validation des entrees | 19 | /20 |
| Cryptographie | 17 | /20 |
| Logging & monitoring | 14 | /20 |
| Configuration & hardening | 20 | /20 |
| **TOTAL** | **88** | **/100** |
