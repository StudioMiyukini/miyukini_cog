# E02 — JayXpose UI refonte (11 fichiers)

## Statut : A faire
## Depend de : E00
## Agents : Lise
## Taches : 11

> Objectif : Migrer tous les fichiers UI JayXpose vers miyuki-ui-dioxus. Parallélisable avec E01 (crates/apps disjoints).

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E02-01 | CODE | Refonte components.rs JayXpose (StatCard, composants locaux) | Lise | `services/jayxpose/components.rs` | pending |
| E02-02 | CODE | Refonte sidebar.rs JayXpose → SidebarNav | Lise | `services/jayxpose/sidebar.rs` | pending |
| E02-03 | CODE | Refonte dashboard.rs (PageHeader + StatCard + StatusBadge) | Lise | `dashboard.rs` | pending |
| E02-04 | CODE | Refonte catalogue.rs (DataTable + EmptyState + TabBar) | Lise | `catalogue.rs` | pending |
| E02-05 | CODE | Refonte vitrine.rs (PageHeader + ActionBar) | Lise | `vitrine.rs` | pending |
| E02-06 | CODE | Refonte fiche_publique.rs (PageHeader) | Lise | `fiche_publique.rs` | pending |
| E02-07 | CODE | Refonte documents.rs (DataTable + StatusBadge + EmptyState) | Lise | `documents.rs` | pending |
| E02-08 | CODE | Refonte entreprise.rs (PageHeader + FormField) | Lise | `entreprise.rs` | pending |
| E02-09 | CODE | Refonte onboarding.rs (PageHeader + steps) | Lise | `onboarding.rs` | pending |
| E02-10 | CODE | Refonte produit_form.rs (FormField miyuki-ui-dioxus) | Lise | `produit_form.rs` | pending |
| E02-11 | TEST | cargo check + clippy JayXpose service | Victor | workspace | pending |

## Criteres de completion
- 0 inline `style:` hardcode dans les 10 fichiers
- `cargo check -p miyukini-central` : 0 erreurs
- 0 clippy warnings
- MSCM sur tous les fichiers modifies

## Commit message template
`feat(jayxpose-ui): E02 -- refonte complete 11 fichiers UI miyuki-ui-dioxus`
