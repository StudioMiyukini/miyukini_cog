# E01 — JayFestival UI refonte (15 fichiers)

## Statut : Terminé
## Depend de : E00
## Agents : Lise
## Taches : 15

> Objectif : Migrer tous les fichiers UI JayFestival vers miyuki-ui-dioxus. Eliminer inline styles, utiliser Palette, SidebarNav, PageHeader, StatCard, DataTable, StatusBadge, EmptyState.

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E01-01 | CODE | Refonte components.rs JayFestival (StatCard, ActionButton) | Lise | `services/jayfestival/components.rs` | pending |
| E01-02 | CODE | Refonte sidebar JayFestival → SidebarNav | Lise | `services/jayfestival/mod.rs` (sidebar inline) | pending |
| E01-03 | CODE | Refonte org_dashboard (PageHeader + StatCard + EmptyState) | Lise | `org_dashboard.rs` | pending |
| E01-04 | CODE | Refonte org_editions (PageHeader + DataTable) | Lise | `org_editions.rs` | pending |
| E01-05 | CODE | Refonte org_edition_hub (PageHeader + TabBar + ActionBar) | Lise | `org_edition_hub.rs` | pending |
| E01-06 | CODE | Refonte org_exposants (DataTable + StatusBadge) | Lise | `org_exposants.rs` | pending |
| E01-07 | CODE | Refonte org_annonces, org_equipe, org_budget (PageHeader + DataTable) | Lise | `org_annonces.rs`, `org_equipe.rs`, `org_budget.rs` | pending |
| E01-08 | CODE | Refonte org_documents, org_compte (PageHeader + DataTable) | Lise | `org_documents.rs`, `org_compte.rs` | pending |
| E01-09 | CODE | Refonte exp_dashboard (PageHeader + StatCard + StatusBadge) | Lise | `exp_dashboard.rs` | pending |
| E01-10 | CODE | Refonte exp_candidatures (DataTable + StatusBadge + EmptyState) | Lise | `exp_candidatures.rs` | pending |
| E01-11 | CODE | Refonte exp_participations, exp_agenda (DataTable + TabBar) | Lise | `exp_participations.rs`, `exp_agenda.rs` | pending |
| E01-12 | CODE | Refonte exp_documents, exp_factures (DataTable + EmptyState) | Lise | `exp_documents.rs`, `exp_factures.rs` | pending |
| E01-13 | CODE | Refonte exp_notifications (liste + StatusBadge) | Lise | `exp_notifications.rs` | pending |
| E01-14 | CODE | Refonte exp_fiche_publique, exp_compte (PageHeader) | Lise | `exp_fiche_publique.rs`, `exp_compte.rs` | pending |
| E01-15 | TEST | cargo check + clippy JayFestival service | Victor | workspace | pending |

## Criteres de completion
- 0 inline `style:` hardcode dans les 14 fichiers (sauf justifie)
- `cargo check -p miyukini-central` : 0 erreurs
- 0 clippy warnings
- MSCM sur tous les fichiers modifies

## Commit message template
`feat(jayfestival-ui): E01 -- refonte complete 15 fichiers UI miyuki-ui-dioxus`
