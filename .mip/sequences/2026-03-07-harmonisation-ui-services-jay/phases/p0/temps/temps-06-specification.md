# P0 Temps 6 - Specification technique

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Francois

## Specification

### E00 — Infrastructure provide_context (bloquant)

**Fichier** : `apps/central/src/app.rs`
**Ajout dans `App()` avant tout use_context_provider** :
```rust
provide_context(miyuki_ui_tokens::Palette::default());
```
**Cargo.toml apps/central** : verifier/ajouter `miyuki-ui-dioxus = { path = "../../crates/miyuki-ui-dioxus" }`

**Nouveaux composants dans `crates/miyuki-ui-dioxus`** :
- `molecules/stat_card.rs` : `StatCard { icon, value, label, trend? }`
- `molecules/data_table.rs` : `DataTable { columns, rows, empty_label }`
- `molecules/tab_bar.rs` : `TabBar { tabs: Vec<TabItem>, active_tab, on_change }`
- `organisms/action_bar.rs` : `ActionBar { actions: Vec<ActionItem> }`

### E01 — JayFestival UI (15 fichiers)

Pattern par fichier :
```rust
// Avant (legacy)
let c = use_context::<Theme>(); // ou hardcode inline style

// Apres
let p = use_context::<miyuki_ui_tokens::Palette>();
// utiliser p.bg_surface, p.text_primary, p.accent, p.border_default etc.
```

Fichiers cibles prioritaires :
- `components.rs` — StatCard, ActionButton → miyuki-ui-dioxus
- `org_dashboard.rs` — PageHeader + StatCard grid + EmptyState
- `org_editions.rs`, `org_edition_hub.rs` — PageHeader + DataTable
- `exp_dashboard.rs`, `exp_candidatures.rs` — PageHeader + StatusBadge + EmptyState
- `sidebar.rs` (shared) → via `mod.rs` → SidebarNav

### E02 — JayXpose UI (11 fichiers)

Fichiers cibles prioritaires :
- `components.rs` — StatCard, composants locaux → miyuki-ui-dioxus
- `dashboard.rs` — PageHeader + StatCard + StatusBadge
- `catalogue.rs`, `vitrine.rs` — DataTable + EmptyState
- `sidebar.rs` → SidebarNav

### E03 — JayKonta UI (8 fichiers)

- `components.rs`, `purse_dashboard.rs` — PageHeader + StatCard (solde, mouvements)
- `purse_movements.rs` — DataTable + TabBar (revenus/depenses)
- `purse_forecast.rs` — PageHeader + EmptyState

### E04 — JayManga UI (14 fichiers)

- `dashboard.rs` — PageHeader + StatCard (oeuvres, lecteurs, revenus)
- `catalogue.rs`, `boutique.rs` — DataTable + EmptyState
- `reader.rs` — composants specifiques (garder logique, migrer style)
- `sidebar.rs` → SidebarNav

### E05 — JayKoa UI (7 fichiers)

- `components.rs` — evenements, styles calendrier
- `calendar_view.rs`, `month_view.rs`, `week_view.rs` — styles grille → vars CSS
- `sidebar.rs` → SidebarNav
- `event_form.rs` → FormField miyuki-ui-dioxus

### Contraintes Dioxus 0.7

- Props `#[props]` avec types Copy ou Clone
- `EventHandler<T>` pour les callbacks
- Pas de `use_state` legacy — utiliser `use_signal()`
- `rsx! {}` syntaxe 0.7
