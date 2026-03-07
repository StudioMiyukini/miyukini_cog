# P0 Temps 4 - Inventaire prerequis

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Denis/Hugo/Jean

## Prerequis techniques

| # | Prerequis | Disponible | Action requise |
|---|-----------|-----------|----------------|
| P1 | `miyuki-ui-dioxus` — atoms/molecules/organisms de base | OUI | Ajouter StatCard, DataTable, TabBar |
| P2 | `miyuki-ui-tokens::Palette` | OUI | Aucune |
| P3 | `provide_context(Palette)` dans `App()` | NON | E00 — ajouter dans app.rs |
| P4 | Dependency `miyuki-ui-dioxus` dans apps/central/Cargo.toml | A verifier | E00 — ajouter si absente |
| P5 | Dioxus 0.7 stable | OUI | Aucune |
| P6 | `cargo check -p miyukini-central` propre | A verifier | E00 smoke test |

## Composants miyuki-ui-dioxus a creer avant migration

| Composant | Type | Utilise par |
|-----------|------|------------|
| StatCard | molecule | JF, JX, JK, JM, JKoa |
| DataTable | molecule | JF, JX, JM (listes) |
| TabBar | molecule | JF, JX, JK, JM (navigation intra-service) |
| ActionBar | organism | JF, JX (actions org/admin) |

## Fichiers cibles (apps/central)

- `apps/central/src/app.rs` — ajouter `provide_context(Palette::default())`
- `apps/central/Cargo.toml` — verifier dep `miyuki-ui-dioxus`
- `apps/central/src/services/jayfestival/*.rs` — 15 fichiers
- `apps/central/src/services/jayxpose/*.rs` — 11 fichiers
- `apps/central/src/services/jaykonta/*.rs` — 8 fichiers
- `apps/central/src/services/jaymanga/*.rs` — 14 fichiers
- `apps/central/src/services/jaykoa/*.rs` — 7 fichiers

Total : ~55 fichiers UI a migrer

## Risques

| Risque | Probabilite | Mitigation |
|--------|------------|-----------|
| Conflit API Dioxus 0.7 avec pattern existant | Moyenne | Lire chaque fichier avant edit |
| Palette couleurs differente du legacy ThemePalette | Haute | Mapper les couleurs equivalentes |
| Composants avec logique complexe (reader, calendrier) | Haute | Garder logique, migrer style uniquement |
