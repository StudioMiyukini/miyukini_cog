# E04 — COG Web Portal (création from scratch)

## Statut : Terminé
## Depend de : E03 (contrats d'exposition disponibles)
## Agents : François (back axum) + Victor (sécu) + Lise (frontend web HTML)
## Taches : 12
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Objectif : Portail HTTP générique multi-services opérationnel. JayFestival et JayXpose branchés via PortalContract. Sécurité niveau DURCI (CSP nonce, HSTS, rate limit, CSRF). Référence : apps/miyucloud.

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E04-01 | CODE | Créer Cargo.toml + src/main.rs Portal (AppState + router axum) | François | `apps/cog-web-portal/Cargo.toml`, `src/main.rs` | pending | — | — |
| E04-02 | CODE | Copier/adapter security_headers.rs (CspNonce middleware) depuis miyucloud | François | `apps/cog-web-portal/src/security_headers.rs` | pending | — | — |
| E04-03 | CODE | Créer routes home.rs (GET / → liste services) | François | `apps/cog-web-portal/src/routes/home.rs` | pending | — | — |
| E04-04 | CODE | Créer routes service.rs (GET /:service, GET /:service/:slug) | François | `apps/cog-web-portal/src/routes/service.rs` | pending | — | — |
| E04-05 | CODE | Créer routes contact.rs (POST /:service/contact + CSRF) | François | `apps/cog-web-portal/src/routes/contact.rs` | pending | — | — |
| E04-06 | CODE | Créer templates HTML Portal (portal_home, service_page, error_page) | Lise+François | `apps/cog-web-portal/src/web_surface/templates.rs` | pending | — | — |
| E04-07 | CODE | Brancher JayFestival (impl PortalContract dans AppState) | François | `apps/cog-web-portal/src/main.rs` | pending | — | — |
| E04-08 | CODE | Brancher JayXpose (impl PortalContract dans AppState) | François | `apps/cog-web-portal/src/main.rs` | pending | — | — |
| E04-09 | CODE | Configurer HSTS + rate limiting tower-http | François | `apps/cog-web-portal/src/main.rs` | pending | — | — |
| E04-10 | TEST-S | Audit sécurité Victor — CSP/HSTS/rate-limit/CSRF/path-traversal | Victor | `apps/cog-web-portal/src/` | pending | — | — |
| E04-11 | TEST | Tests intégration Portal (routes, CSP header, CSRF token) | François | `apps/cog-web-portal/tests/` | pending | — | — |
| E04-12 | TEST | `cargo clippy -p cog-web-portal -- -D warnings` + smoke test GREEN | Denis | workspace | pending | — | — |

## Critères de complétion
- `cargo build -p cog-web-portal` : 0 erreur
- `cargo test -p cog-web-portal` : tous passent
- `cargo clippy -p cog-web-portal -- -D warnings` : 0 warning
- CSP nonce per-request présent sur toutes réponses HTML
- HSTS header présent
- Rate limiting configuré (60 req/min par IP)
- CSRF token sur formulaires POST
- JayFestival et JayXpose exposés via routes Portal
- MSCM sur tout le code créé

## Commit message template
`feat(cog-web-portal): E04 -- portail HTTP multi-services + JayFestival+JayXpose branchés`
