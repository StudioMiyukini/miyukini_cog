# Design system UI — index

<!-- @id ux.designsystem.index -->
<!-- @role ui-library-reference -->
<!-- @layer T -->
<!-- @human Design system Miyukini : tokens agnostiques (thème COG) et composants Dioxus en Atomic Design -->
<!-- @do reference_design_system -->

| Crate | Rôle | Couche annotée | Workspace |
|---|---|---|---|
| `crates/miyuki-ui-tokens` | tokens sans framework (couleurs, espacements, typo…) | 5 | oui |
| `crates/miyuki-ui-dioxus` | composants Dioxus 0.7 sur les tokens | 6 | oui |
| `crates/miyuki-ui-egui` | composants egui de l'ancien moteur de jeu (retiré) | 6 | **non** — hors périmètre de cette fiche |

- **Strate** : T (bibliothèques partagées ; non cataloguées par MasterButler — à vérifier)
- **Référence composants HTML** : [HyperUI](../ui/hyperui/HyperUI%20-%20Index%20Composants.md) (catalogue externe, sans lien code avec ces crates)

## 1. Tokens — `miyuki-ui-tokens`

<!-- @id ux.designsystem.tokens -->
<!-- @role design-tokens -->
<!-- @do list_tokens -->

- Aucune dépendance graphique ; `serde` optionnel (feature `serde`) ; `unsafe_code = "forbid"`
- `UiTheme` = `palette` + `spacing` + `typography` + `radius` + `shadow` + `animation` + `z_index` (`theme.rs`)
- Thème exporté : `COG_THEME` « Miyukini Gaming (COG) » (`themes/cog.rs`) — sombre, accent bleu, accent sakura, police Inter

| Module | Types / constantes |
|---|---|
| `color` | `Rgba` (`new`, `with_alpha`, `alpha`, `to_css`, `to_hex`, `to_array`) |
| `palette` | `Palette` : 6 fonds, 4 textes, accents primaire/secondaire, sémantiques ; `COG_PALETTE` |
| `spacing` | `SpacingScale::standard()` |
| `typography` | `FontFamily` (`COG_UI`, `COG_MONO`), `FontSize`, `FontWeight`, `TypographyScale`, `TextStyle` |
| `radius` · `shadow` | `RadiusScale::cog()` · `Shadow`, `ShadowScale::cog()` |
| `animation` | `TransitionDuration`, `Easing` (`STANDARD`, `ENTRANCE`, `EXIT`), `TransitionScale::standard()` |
| `z_index` | `ZIndexScale::standard()` |

| Couleur COG | Valeur |
|---|---|
| `bg_base` | `#0e1015` |
| `text_primary` | `#c6d4df` |
| `accent_primary` | `#1a9fff` |
| `accent_secondary` | `#e8a0bf` |

## 2. Composants — `miyuki-ui-dioxus`

<!-- @id ux.designsystem.dioxus -->
<!-- @role component-library -->
<!-- @do list_components -->

- Thème : `provide_theme(COG_THEME.clone())` à la racine, puis `use_theme()`, `use_palette()`, `set_theme()` (`context.rs`)
- Helpers CSS inline : `bg`, `fg`, `border`, `padding`, `gap`, `font_*`, `box_shadow`, `transition`, `css` (`styles.rs`)
- Ré-exporte `miyuki_ui_tokens`

| Niveau | Composants |
|---|---|
| atoms (14) | Avatar, Badge, Button, Checkbox, Divider, Icon, TextInput, ProgressBar, Slider, Spinner, StatusBadge, Text, Toggle, Tooltip |
| molecules (11) | Breadcrumb, Card, Dropdown, EmptyState, FormField, MenuItem, Pagination, SearchBar, StatRow, TabItem, Toast |
| organisms (11) | CardGrid, CommandPalette, DataTable, Form, AppHeader, Modal, NavBar, NotificationCenter, PageHeader, AppSidebar, TabBar |
| templates (7) | AuthLayout, DashboardLayout, DetailLayout, FullscreenLayout, ServiceLayout, SettingsLayout, SplitLayout |

## 3. Consommateurs

<!-- @id ux.designsystem.consommateurs -->
<!-- @role dependency-map -->
<!-- @do list_consumers -->

| App / crate | tokens | dioxus |
|---|---|---|
| `apps/central` | oui | oui |
| `crates/miyukini-service-ui` | oui | oui |
| `apps/central-mobile` | oui | — |
| `apps/jaybureau-suite/hub` | oui | — |
| `apps/jaybureau-suite/jay-docs` | oui | — |

## 4. Statut

<!-- @id ux.designsystem.statut -->
<!-- @role status-measured -->
<!-- @do report_design_system_status -->

- `miyuki-ui-tokens` : ~41 `#[test]` + doctests (`lib.rs`, `theme.rs`) ; lints propres (clippy pedantic)
- `miyuki-ui-dioxus` : 7 tests, tous dans `styles.rs` ; aucun test de composant ; lints du workspace ; ~4 800 lignes
- Comptes mesurés le 2026-10-24, pendant un retrait de thème en cours : à recompter

## 5. Écarts connus

<!-- @id ux.designsystem.ecarts -->
<!-- @role known-gaps -->
<!-- @do list_design_system_gaps -->

- `miyuki-ui-dioxus/src/lib.rs` annonce « Dioxus 0.6 » ; `Cargo.toml` utilise Dioxus 0.7
- Feature `desktop` forcée dans `miyuki-ui-dioxus` : usage web / mobile à vérifier
- Seuls 5 consommateurs ; MIPOWER et d'autres apps Dioxus gardent leur CSS propre
- `miyuki-ui-egui` hors workspace et sans consommateur : sort du design system (décision de suppression à prendre)
- Pas de doc d'usage par composant (props, variantes) : se référer aux commentaires `//!` de chaque fichier
