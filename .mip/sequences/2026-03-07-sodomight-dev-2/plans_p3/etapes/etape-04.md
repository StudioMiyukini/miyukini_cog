# E04 -- AtlasHandle + MaterialHandle publics

## Statut : A faire
## Depend de : E03
## Agents : Denis
## Taches : 5
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E04-01 | CODE | Ajouter AtlasHandle::new(id: u32) -> Self pub | Denis | `mge/crates/mge-render/src/atlas.rs` | pending | -- | -- |
| E04-02 | CODE | Remplacer AtlasHandle::new_test par alias vers new() (#[cfg(test)]) | Denis | `mge/crates/mge-render/src/atlas.rs` | pending | -- | -- |
| E04-03 | CODE | Meme traitement pour MaterialHandle (new pub + new_test alias) | Denis | `mge/crates/mge-render/src/atlas.rs` | pending | -- | -- |
| E04-04 | CHECK | cargo test -p mge-render (verifier new_test fonctionne encore) | Denis | -- | pending | -- | -- |
| E04-05 | CHECK | cargo clippy -p mge-render -- -D warnings | Denis | -- | pending | -- | -- |

## Critere de sortie
`AtlasHandle::new(0)` compilable sans `#[cfg(test)]`. `MaterialHandle::new(0)` idem. Tests existants verts.

## Commit message template
`feat(mge-render): E04 -- AtlasHandle + MaterialHandle constructeurs publics`
