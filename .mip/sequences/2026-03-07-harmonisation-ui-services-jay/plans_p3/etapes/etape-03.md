# E03 — JayKonta UI refonte (8 fichiers)

## Statut : A faire
## Depend de : E00
## Agents : Hugo
## Taches : 8

> Objectif : Migrer les UIs JayKonta (gestion budget/purse) vers miyuki-ui-dioxus.

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E03-01 | CODE | Refonte components.rs JayKonta | Hugo | `services/jaykonta/components.rs` | pending |
| E03-02 | CODE | Refonte sidebar.rs JayKonta → SidebarNav | Hugo | `services/jaykonta/sidebar.rs` | pending |
| E03-03 | CODE | Refonte purse_dashboard.rs (PageHeader + StatCard solde/mouvements) | Hugo | `purse_dashboard.rs` | pending |
| E03-04 | CODE | Refonte purse_movements.rs (DataTable + TabBar revenus/depenses) | Hugo | `purse_movements.rs` | pending |
| E03-05 | CODE | Refonte purse_movement_form.rs (FormField miyuki-ui-dioxus) | Hugo | `purse_movement_form.rs` | pending |
| E03-06 | CODE | Refonte purse_forecast.rs (PageHeader + StatCard + EmptyState) | Hugo | `purse_forecast.rs` | pending |
| E03-07 | CODE | Refonte purse_recurring.rs (DataTable + StatusBadge) | Hugo | `purse_recurring.rs` | pending |
| E03-08 | TEST | cargo check + clippy JayKonta service | Victor | workspace | pending |

## Criteres de completion
- 0 inline `style:` hardcode dans les 7 fichiers
- `cargo check -p miyukini-central` : 0 erreurs
- 0 clippy warnings
- MSCM sur tous les fichiers modifies

## Commit message template
`feat(jaykonta-ui): E03 -- refonte complete 8 fichiers UI miyuki-ui-dioxus`
