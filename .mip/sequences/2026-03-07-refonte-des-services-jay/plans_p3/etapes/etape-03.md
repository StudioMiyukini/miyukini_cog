# E03 — JayXpose prod-ready + Contrats d'exposition Portal

## Statut : Terminé
## Depend de : E01 (parallèle avec E02)
## Agents : Lise (UI) + François (backend + contrats)
## Taches : 12
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Objectif : JayXpose UI refontée (design system E02), backend hardened, + créer les contrats d'exposition Portal pour JayFestival ET JayXpose. Les contrats sont le prérequis de E04.

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E03-01 | CODE | Refonte sidebar JayXpose (utilise SidebarNav) | Lise | `apps/central/src/services/jayxpose/sidebar.rs` | pending | — | — |
| E03-02 | CODE | Refonte components.rs JayXpose (StatCard, QuickAccessCard → miyuki-ui-dioxus) | Lise | `apps/central/src/services/jayxpose/components.rs` | pending | — | — |
| E03-03 | CODE | Refonte dashboard JayXpose (design system, completude profil) | Lise | `apps/central/src/services/jayxpose/dashboard.rs` | pending | — | — |
| E03-04 | CODE | Refonte catalogue + vitrine JayXpose — UX exposant | Lise | `catalogue.rs`, `vitrine.rs`, `fiche_publique.rs` | pending | — | — |
| E03-05 | CODE | Refonte documents JayXpose (coffre-fort, upload UI) | Lise | `documents.rs`, `entreprise.rs` | pending | — | — |
| E03-06 | CODE | Hardening backend JayXpose (data/kindmother_db.rs, governance.rs) | François | `crates/jayxpose/src/data/kindmother_db.rs`, `governance.rs` | pending | — | — |
| E03-07 | CODE | Hardening upload documents JayXpose (MIME validation, path canonique) | François | `crates/jayxpose/src/data/` | pending | — | — |
| E03-08 | CODE | Créer `crates/jayfestival/src/portal_contract.rs` (impl PortalContract) | François | `crates/jayfestival/src/portal_contract.rs` | pending | — | — |
| E03-09 | CODE | Créer `crates/jayxpose/src/portal_contract.rs` (impl PortalContract) | François | `crates/jayxpose/src/portal_contract.rs` | pending | — | — |
| E03-10 | TEST | Tests contrats d'exposition (portal_contract unit tests) | François | `crates/jayfestival/tests/`, `crates/jayxpose/tests/` | pending | — | — |
| E03-11 | TEST | Tests UI JayXpose (composants critiques) | Lise | `apps/central/tests/` | pending | — | — |
| E03-12 | TEST | `cargo clippy -p jayxpose -p jayxpose-app -- -D warnings` | Denis | workspace | pending | — | — |

## Critères de complétion
- `portal_contract.rs` compilable dans jayfestival ET jayxpose
- `PortalContract::public_pages()` retourne données réelles depuis KindMother
- Aucun inline style orphelin dans jayxpose UI
- 0 `unwrap()` dans les modules modifiés
- MSCM sur tout code créé
- Tests contrats passent

## Commit message template
`feat(jayxpose): E03 -- prod-ready UI Dioxus 0.7 + contrats exposition Portal`
