# P0 Temps 2 - Ideation

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Maria/Lise

## Objectif

Definir la strategie d'harmonisation UI et les decisions d'architecture.

## Decision 1 : Infrastructure en E00 (bloquant)

Ajouter dans `apps/central/src/app.rs` — fonction `App()` :
```rust
provide_context(miyuki_ui_tokens::Palette::default());
```
Optionnel : definir une palette COG custom via `Palette { ... }` avec les couleurs Central.
→ Decision : **Palette::default() d'abord**, palette custom en BUF si necessaire.

## Decision 2 : Ordre de refonte (par impact utilisateur)

1. E00 : Infrastructure provide_context (prerequis bloquant)
2. E01 : JayFestival (plus utilise, 15 fichiers, org + exp paths)
3. E02 : JayXpose (11 fichiers, bloquer expose catalogue public)
4. E03 : JayKonta (8 fichiers, purse/budget — critique metier)
5. E04 : JayManga (14 fichiers, le plus volumineux — reader + catalogue)
6. E05 : JayKoa (7 fichiers, calendrier — composants specifiques)
7. BUF : Elimination inline styles residuels + composants manquants

## Decision 3 : Composants nouveaux a creer en E00

- `StatCard` molecule : icon + valeur + label + trend optionnel (reutilise dans tous les services)
- `DataTable` molecule : colonnes configurables + tri + empty state
- `TabBar` molecule : onglets horizontaux avec badges

## Decision 4 : Pattern de migration

Pour chaque fichier :
1. Remplacer les imports theme legacy par `use_context::<Palette>()`
2. Remplacer inline styles par variables CSS ou composants miyuki-ui-dioxus
3. Remplacer `StatCard` inline par molecule `StatCard`
4. Remplacer sidebars inline par `SidebarNav`
5. Ajouter `PageHeader` en tete de chaque vue principale
6. `EmptyState` sur tous les cas de liste vide

## Decision 5 : Parallelisation

- E01 // E02 // E03 (crates/apps/central independants → pas de conflits git)
- E04 // E05 apres E01-E03 valides
- Un agent par service (Lise = JF+JX, Hugo = JK+JM, Denis = JKoa)

## Decision 6 : Definition "done" pour chaque service

- 0 inline `style:` dans les fichiers du service (sauf cas calendrier justifie)
- 0 couleur hexadecimale hardcodee
- Tous composants via miyuki-ui-dioxus ou justifies localement avec commentaire
- MSCM sur tous les fichiers modifies
- cargo clippy -D warnings : 0 violation
