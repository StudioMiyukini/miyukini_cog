# E10 -- Hardening & Audit Final

## Statut : Termine
## Depend de : E9
## Agents : Victor, Denis, Lise
## Taches : 8

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E10-01 | TEST-S | Audit path traversal complet (WebDAV, encodings, unicode) | Victor | crates/miyucloud-dav/tests/security_path_traversal.rs | done |
| E10-02 | TEST-S | Audit XXE CalDAV/CardDAV (DTD, entity expansion, XML bomb) | Victor | crates/miyucloud-dav/tests/security_xxe.rs | done |
| E10-03 | TEST-S | Audit auth bypass (tokens, sessions, CORS) | Victor | crates/miyucloud-dav/tests/security_auth_sql.rs | done |
| E10-04 | TEST-S | Audit injection SQL (rusqlite params, prepared statements) | Victor | crates/miyucloud-dav/tests/security_auth_sql.rs | done |
| E10-05 | CODE | Hardening CSP: supprimer unsafe-inline (nonce-based) | Victor | apps/miyucloud/src/security_headers.rs, portal.rs, share_page.rs | done |
| E10-06 | CODE | Hardening logging: evenements securite (tracing::warn) | Victor | apps/miyucloud/src/api/auth.rs, web_surface/rate_limiter.rs, portal.rs, share_page.rs | done |
| E10-07 | AUDIT | Score securite final (>95/100) | Victor | -- | done |
| E10-08 | AUDIT | Review architecture finale + documentation deltas fork | Denis | -- | done |

## Commit message template
`security(miyucloud): E10 -- hardening, audit final, score securite >95/100`

## Notes E10-05 (CSP Hardening)

Suppression de `'unsafe-inline'` de `script-src` ET `style-src` dans la CSP:
- Nonce UUID genere par requete dans le middleware `SecurityHeadersMiddleware`
- Nonce injecte dans les extensions de requete (`CspNonce`)
- Handlers extraient le nonce via `Extension<CspNonce>`
- Tous les `<script>` et `<style>` portent `nonce="{n}"`
- Conversion de tous les `onclick=` en `addEventListener` (elimination inline JS)
- Conversion de tous les `style=""` inline en classes CSS
- CSP finale: `script-src 'self' 'nonce-{n}'; style-src 'self' 'nonce-{n}'`

## Notes E10-06 (Security Logging)

Ajout de `tracing::warn!` structure pour:
- Rate limit depasse (IP + retry_after)
- Auth API echouee (X-COG-Token invalide/absent, path)
- Auth portail echouee (email, erreur)
- Auth share link echouee (link_id)
- Tentative download sans session sur lien protege (link_id)
- Acces token expire/revoke (raison)
- Acces token invalide (raison)

## Notes E10-07 (Score securite)

Score estime: **97/100**
- [x] CSP nonce-based (pas d'unsafe-inline)
- [x] HSTS avec includeSubDomains
- [x] X-Frame-Options DENY
- [x] X-Content-Type-Options nosniff
- [x] Referrer-Policy no-referrer
- [x] Permissions-Policy restrictive
- [x] Rate limiting par IP
- [x] Tokens HMAC-SHA256 (WOPI, sessions)
- [x] Comparaison constant-time pour tokens
- [x] IP hashees (RGPD)
- [x] XML security (DTD/XXE bloque)
- [x] Path traversal valide
- [x] SQL injection prevenue (rusqlite params)
- [x] Security event logging (tracing structured)
- [-] CORS headers non implementes (API interne seulement, -1pt)
- [-] CSP report-uri non configure (-1pt)
- [-] Subresource Integrity pas applicable (pas de CDN) (-1pt)
