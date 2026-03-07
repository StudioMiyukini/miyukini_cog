# E04 — JayManga UI refonte (14 fichiers)

## Statut : A faire
## Depend de : E00
## Agents : Hugo
## Taches : 14

> Objectif : Migrer les UIs JayManga (plateforme manga/BD) vers miyuki-ui-dioxus. Le plus volumineux (~2500 lignes), attention au reader et au catalogue.

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E04-01 | CODE | Refonte components.rs JayManga | Hugo | `services/jaymanga/components.rs` | pending |
| E04-02 | CODE | Refonte sidebar.rs JayManga → SidebarNav | Hugo | `services/jaymanga/sidebar.rs` | pending |
| E04-03 | CODE | Refonte dashboard.rs (PageHeader + StatCard oeuvres/lecteurs/revenus) | Hugo | `dashboard.rs` | pending |
| E04-04 | CODE | Refonte catalogue.rs (DataTable + EmptyState + TabBar) | Hugo | `catalogue.rs` | pending |
| E04-05 | CODE | Refonte boutique.rs (DataTable + ActionBar) | Hugo | `boutique.rs` | pending |
| E04-06 | CODE | Refonte series.rs (DataTable + StatusBadge) | Hugo | `series.rs` | pending |
| E04-07 | CODE | Refonte chapters.rs (DataTable + StatusBadge) | Hugo | `chapters.rs` | pending |
| E04-08 | CODE | Refonte library.rs (DataTable + TabBar) | Hugo | `library.rs` | pending |
| E04-09 | CODE | Refonte reader.rs (garder logique, migrer style uniquement) | Hugo | `reader.rs` | pending |
| E04-10 | CODE | Refonte reader_stats.rs (PageHeader + StatCard) | Hugo | `reader_stats.rs` | pending |
| E04-11 | CODE | Refonte profile.rs (PageHeader + FormField) | Hugo | `profile.rs` | pending |
| E04-12 | CODE | Refonte onboarding.rs (steps + PageHeader) | Hugo | `onboarding.rs` | pending |
| E04-13 | CODE | Refonte sales.rs + promotions.rs (DataTable + StatusBadge) | Hugo | `sales.rs`, `promotions.rs` | pending |
| E04-14 | TEST | cargo check + clippy JayManga service | Victor | workspace | pending |

## Criteres de completion
- 0 inline `style:` hardcode dans les 13 fichiers (sauf reader.rs justifie si layout specifique)
- `cargo check -p miyukini-central` : 0 erreurs
- 0 clippy warnings
- MSCM sur tous les fichiers modifies

## Commit message template
`feat(jaymanga-ui): E04 -- refonte complete 14 fichiers UI miyuki-ui-dioxus`
