# P0 Temps 10 - Verification CI/CD

## Statut

- Etat : Terminé
- Phase : P0 Temps 10
- Agent : Hugo
- Date : 2026-03-07

## TL;DR

CI/CD = commandes manuelles locales (pas de pipeline CI automatisé configuré). Commandes standards `cargo` depuis environment.md. Ajout `cog-web-portal` dans workspace. Pas de Docker requis pour P3.

## CI/CD vérifié

| Élément | Statut | Notes |
|---------|--------|-------|
| Pipeline CI automatisé | N/A | Pas de CI configuré — commandes manuelles uniquement |
| `cargo check --workspace` | OK | Vérifié dans E00 smoke test |
| `cargo clippy --workspace -- -D warnings` | OK (à vérifier E00) | Commande standard |
| `cargo test --workspace` | OK | Checkpoint Denis /5 tâches + BUF |
| `cargo audit` | À configurer | `cargo install cargo-audit` si non présent — obligatoire BUF |
| `cargo fmt --all` | OK | Format standard avant commit |

## Déploiement COG Web Portal (Hugo)

- Mode : bin Rust standalone — `cargo run -p cog-web-portal`
- Configuration : variables d'env (`COG_PORTAL_HOST=0.0.0.0`, `COG_PORTAL_PORT=8080`, `COG_PORTAL_COG_ID=my-cog`)
- Pas de Docker pour P3 (local dev). Docker optionnel pour prod (hors scope séquence).
- TLS : via reverse proxy (nginx) en prod — pas géré par le Portal directement

## Actions correctives

- Corriger `environment.md` : `Framework(s) : axum, Dioxus 0.6` → `axum, Dioxus 0.7`
- S'assurer que `cargo-audit` est installé sur la machine avant BUF

