# E00 -- Dependances bytemuck

## Statut : A faire
## Depend de : --
## Agents : Denis
## Taches : 3
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E00-01 | CONFIG | Ajouter bytemuck dans workspace deps | Denis | `mge/Cargo.toml` | pending | -- | -- |
| E00-02 | CONFIG | Ajouter bytemuck.workspace dans mge-render | Denis | `mge/crates/mge-render/Cargo.toml` | pending | -- | -- |
| E00-03 | CHECK | cargo check -p mge-render | Denis | -- | pending | -- | -- |

## Critere de sortie
`cargo check -p mge-render` vert. `bytemuck` dans `cargo tree -p mge-render`.

## Commit message template
`feat(mge-render): E00 -- ajout dep bytemuck workspace`
