# E00 — Infrastructure provide_context + nouveaux composants miyuki-ui-dioxus

## Statut : Terminé
## Depend de : —
## Agents : Francois + Lise
## Taches : 8

> Objectif : Debloquer l'utilisation de miyuki-ui-dioxus dans apps/central en ajoutant provide_context(Palette::default()) dans App(). Creer les 4 nouveaux composants necessaires pour la migration des services.

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E00-01 | CODE | Verifier/ajouter dep miyuki-ui-dioxus dans apps/central/Cargo.toml | Francois | `apps/central/Cargo.toml` | pending |
| E00-02 | CODE | Ajouter `provide_context(Palette::default())` dans App() | Francois | `apps/central/src/app.rs` | pending |
| E00-03 | TEST | cargo check -p miyukini-central (smoke test infrastructure) | Francois | workspace | pending |
| E00-04 | CODE | Creer `crates/miyuki-ui-dioxus/src/molecules/stat_card.rs` | Lise | `molecules/stat_card.rs` | pending |
| E00-05 | CODE | Creer `crates/miyuki-ui-dioxus/src/molecules/data_table.rs` | Lise | `molecules/data_table.rs` | pending |
| E00-06 | CODE | Creer `crates/miyuki-ui-dioxus/src/molecules/tab_bar.rs` | Lise | `molecules/tab_bar.rs` | pending |
| E00-07 | CODE | Creer `crates/miyuki-ui-dioxus/src/organisms/action_bar.rs` | Lise | `organisms/action_bar.rs` | pending |
| E00-08 | TEST | cargo clippy -p miyuki-ui-dioxus -p miyukini-central -- -D warnings | Victor | workspace | pending |

## Criteres de completion
- `cargo check -p miyukini-central` : 0 erreurs
- `use_context::<miyuki_ui_tokens::Palette>()` accessible dans tous les composants
- 4 nouveaux composants compilent
- 0 clippy warnings

## Commit message template
`feat(central): E00 -- provide_context Palette + StatCard + DataTable + TabBar + ActionBar`
