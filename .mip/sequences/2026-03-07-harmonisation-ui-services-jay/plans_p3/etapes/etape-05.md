# E05 — JayKoa UI refonte (7 fichiers)

## Statut : Terminé
## Depend de : E00
## Agents : Denis
## Taches : 7

> Objectif : Migrer les UIs JayKoa (calendrier/agenda) vers miyuki-ui-dioxus. Attention aux vues grille calendrier (month/week/day) qui ont des styles de positionnement specifiques.

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E05-01 | CODE | Refonte components.rs JayKoa | Denis | `services/jaykoa/components.rs` | pending |
| E05-02 | CODE | Refonte sidebar.rs JayKoa → SidebarNav | Denis | `services/jaykoa/sidebar.rs` | pending |
| E05-03 | CODE | Refonte calendar_view.rs (PageHeader + TabBar jour/semaine/mois) | Denis | `calendar_view.rs` | pending |
| E05-04 | CODE | Refonte month_view.rs (grille — migrer couleurs, garder layout CSS grid) | Denis | `month_view.rs` | pending |
| E05-05 | CODE | Refonte week_view.rs + day_view.rs (idem month_view) | Denis | `week_view.rs`, `day_view.rs` | pending |
| E05-06 | CODE | Refonte event_form.rs (FormField miyuki-ui-dioxus) | Denis | `event_form.rs` | pending |
| E05-07 | TEST | cargo check + clippy JayKoa service | Victor | workspace | pending |

## Criteres de completion
- 0 couleur hex hardcodee dans les fichiers (variables CSS ou Palette)
- Layout CSS grid calendrier justifie et documente si inline style maintenu
- `cargo check -p miyukini-central` : 0 erreurs
- 0 clippy warnings
- MSCM sur tous les fichiers modifies

## Commit message template
`feat(jaykoa-ui): E05 -- refonte complete 7 fichiers UI miyuki-ui-dioxus`
