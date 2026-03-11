# BUF -- Tests + clippy + validation visuelle

## Statut : A faire
## Depend de : E05
## Agents : Francois (review), Denis (corrections)
## Taches : 6
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| BUF-01 | TEST | Test sprite_instance_gpu_size (assert size == 48) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| BUF-02 | TEST | Test max_instances_invariant (assert <= 65536) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| BUF-03 | TEST | Test gpu_instance_fields (conversion SpriteInstance minimal) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| BUF-04 | CHECK | cargo test -p mge-render : 0 failed (anciens + nouveaux) | Francois | -- | pending | -- | -- |
| BUF-05 | CHECK | cargo clippy -p mge-render -p sodomight -- -D warnings : 0 violations | Francois | -- | pending | -- | -- |
| BUF-06 | CHECK | cargo run -p sodomight : fenetre non-vide (grille iso visible) | Francois | -- | pending | -- | -- |

## Critere de sortie
Tests verts, clippy propre, aucun `unsafe`, rendu visible a l'ecran.

## Commit message template
`fix(mge-render): BUF -- tests pipeline + corrections post-integration`
