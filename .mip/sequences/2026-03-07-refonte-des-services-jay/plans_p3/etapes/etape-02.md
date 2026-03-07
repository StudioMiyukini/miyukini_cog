# E02 — JayFestival prod-ready (Dioxus 0.7 + Sécurité)

## Statut : A faire
## Depend de : E01
## Agents : Lise (UI Dioxus) + François (backend hardening) + Victor (sécu spot-check)
## Taches : 14
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Objectif : JayFestival UI refontée avec design system miyuki-ui-dioxus, backend hardened, parcours org/exp/vis fluides et qualité production. Parallélisable avec E03 (crates disjoints).

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E02-01 | CODE | Créer/enrichir organisms SidebarNav dans miyuki-ui-dioxus | Lise | `crates/miyuki-ui-dioxus/src/organisms/sidebar_nav.rs` | pending | — | — |
| E02-02 | CODE | Créer PageHeader + EmptyState + StatusBadge dans miyuki-ui-dioxus | Lise | `organisms/page_header.rs`, `molecules/empty_state.rs`, `atoms/status_badge.rs` | pending | — | — |
| E02-03 | CODE | Refonte sidebar JayFestival (utilise SidebarNav) | Lise | `apps/central/src/services/jayfestival/sidebar.rs` | pending | — | — |
| E02-04 | CODE | Refonte components.rs (StatCard → miyuki-ui-dioxus, ActionButton unifié) | Lise | `apps/central/src/services/jayfestival/components.rs` | pending | — | — |
| E02-05 | CODE | Refonte org_dashboard — design system, PageHeader, stats | Lise | `org_dashboard.rs` | pending | — | — |
| E02-06 | CODE | Refonte org_editions + org_edition_hub — UX org | Lise | `org_editions.rs`, `org_edition_hub.rs` | pending | — | — |
| E02-07 | CODE | Refonte exp_dashboard + exp_candidatures — UX exposant | Lise | `exp_dashboard.rs`, `exp_candidatures.rs` | pending | — | — |
| E02-08 | CODE | Refonte vis_catalogue + unc_landing — UX visiteur + accueil | Lise | `vis_catalogue.rs`, `unc_landing.rs`, `unc_events.rs` | pending | — | — |
| E02-09 | CODE | Hardening `crates/jayfestival/src/data/kindmother_db.rs` | François | `kindmother_db.rs` | pending | — | — |
| E02-10 | CODE | Hardening auth JayFestival (HMAC, constant-time compare, rate limit signal) | François | `crates/jayfestival/src/auth/mod.rs`, `permissions.rs` | pending | — | — |
| E02-11 | TEST | Tests unitaires UI JayFestival (composants critiques) | Lise | `apps/central/tests/` | pending | — | — |
| E02-12 | TEST | Tests intégration JayFestival DB (hardening) | François | `crates/jayfestival/src/data/` | pending | — | — |
| E02-13 | TEST-S | Spot-check sécurité Victor — auth bypass + SQL | Victor | `crates/jayfestival/src/auth/` + `data/` | pending | — | — |
| E02-14 | TEST | `cargo clippy -p jayfestival-app -- -D warnings` + smoke complet | Denis | workspace | pending | — | — |

## Critères de complétion
- Aucun inline style orphelin dans `apps/central/src/services/jayfestival/`
- Tous composants via miyuki-ui-dioxus ou justifiés localement
- 0 `unwrap()` dans les modules modifiés
- MSCM sur tout code nouveau
- `cargo clippy -p jayfestival-app -- -D warnings` : 0 warning
- Parcours org/exp/vis testés manuellement

## Commit message template
`feat(jayfestival): E02 -- prod-ready UI Dioxus 0.7 + hardening sécu`
