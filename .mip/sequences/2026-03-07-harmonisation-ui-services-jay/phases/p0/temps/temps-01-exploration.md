# P0 Temps 1 - Exploration

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Maria

## Objectif

Inventaire complet de l'etat actuel des UIs de tous les services Jay dans apps/central.

## Inventaire services Jay

| Service | Dossier | Fichiers UI | Lignes (total) | Blocage actuel |
|---------|---------|-------------|----------------|----------------|
| JayFestival | apps/central/src/services/jayfestival/ | 15 | ~4000 | provide_theme absent |
| JayXpose | apps/central/src/services/jayxpose/ | 11 | ~3000 | provide_theme absent |
| JayKonta | apps/central/src/services/jaykonta/ | 8 | ~1500 | provide_theme absent |
| JayManga | apps/central/src/services/jaymanga/ | 14 | ~2500 | provide_theme absent |
| JayKoa | apps/central/src/services/jaykoa/ | 7 | ~1500 | provide_theme absent |

## Diagnostic infrastructure

- `apps/central/src/app.rs` — `App()` : `use_context_provider` pour `AppState` uniquement
- `miyuki-ui-dioxus` components utilisent `use_context::<miyuki_ui_tokens::Palette>()`
- **Bloquant** : `provide_context(Palette::default())` ABSENT de `App()`
- Sans ce call, tout composant miyuki-ui-dioxus panique au runtime (context missing)

## Etat des inline styles

- JayFestival : inline styles omniprésents dans components.rs, org_dashboard.rs, exp_dashboard.rs, etc.
- JayXpose : même pattern — sidebar.rs, dashboard.rs, catalogue.rs contiennent des styles hardcodes
- JayKonta : purse_dashboard.rs, purse_movements.rs — styles inline + couleurs hardcodees
- JayManga : le plus volumeux — dashboard.rs, reader.rs, boutique.rs, chapters.rs — styles inline massivement
- JayKoa : calendar_view.rs, month_view.rs, week_view.rs — grille calendrier en inline style

## Composants miyuki-ui-dioxus disponibles

- Atoms : Button, Badge, Input, StatusBadge, Spinner
- Molecules : Card, StatCard (via Badge), EmptyState, FormField
- Organisms : PageHeader, SidebarNav

## Composants manquants a creer

- StatCard molecule propre (actuellement clone inline dans chaque service)
- DataTable molecule (tableaux listes)
- TabBar molecule (onglets navigation intra-service)
- ActionBar organism (barre actions contextuelle)
