# Agent François — P3 COG Web Portal (Bloc D)

## Rôle séquence

Dev back-end — Création COG Web Portal (apps/cog-web-portal). Service HTTP axum 0.7, architecture générique multi-services, sécurité niveau DURCI.

## Contexte séquence

- Pattern référence : `apps/miyucloud/` (CSP nonce, HSTS, rate limit, portal.rs)
- Context7 axum : `/tokio-rs/axum/axum_v0_7_9`
- Prérequis : `crates/jayfestival/src/portal_contract.rs` disponible (Bloc C Étape 3)
- Spec Portal : `specs/2026-03-07-refonte-des-services-jay-spec.md` section COG Web Portal

## Fichiers à charger au démarrage

1. `specs/2026-03-07-refonte-des-services-jay-spec.md` (section COG Web Portal + PortalContract)
2. `apps/miyucloud/src/security_headers.rs` (référence CSP nonce)
3. `apps/miyucloud/src/web_surface/portal.rs` (référence pattern axum)
4. `phases/p0/temps/temps-05-securite.md` (checklist sécurité Portal)

## Structure à créer

```
apps/cog-web-portal/
├── Cargo.toml          [axum, tokio, tower-http, serde, jayfestival, jayxpose, tracing]
└── src/
    ├── main.rs         [AppState + router]
    ├── routes/
    │   ├── mod.rs
    │   ├── home.rs     [GET /]
    │   └── service.rs  [GET /:service + GET /:service/:slug]
    ├── web_surface/
    │   ├── mod.rs
    │   └── templates.rs [render_portal_home, render_service_page, render_error]
    └── security_headers.rs [CspNonce middleware — copie miyucloud]
```

## Critères de complétion

- `cargo build -p cog-web-portal` : 0 erreur
- `cargo test -p cog-web-portal` : tests smoke OK
- `cargo clippy -p cog-web-portal -- -D warnings` : 0 warning
- CSP nonce per-request présent sur toutes les réponses HTML
- HSTS header présent
- Rate limiting configuré (tower-http)
- MSCM sur tout le code créé
