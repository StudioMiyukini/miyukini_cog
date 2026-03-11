# E01 -- Shader WGSL sprite instancie

## Statut : A faire
## Depend de : E00
## Agents : Denis
## Taches : 7
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E01-01 | CODE | Creer shader.wgsl avec struct Viewport uniform (group 0, binding 0) | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-02 | CODE | Ajouter bindings texture_2d + sampler (group 1, binding 0-1) | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-03 | CODE | Definir struct VertIn avec @location(0) quad_pos (vertex step) | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-04 | CODE | Ajouter @location(1..4) instance data (screen_pos, uv, tint, scale) | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-05 | CODE | Ecrire vs_main : pixel->NDC (px/vp*2-1, Y flip) + UV interpolation | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-06 | CODE | Ecrire fs_main : textureSample * tint | Denis | `mge/crates/mge-render/src/shader.wgsl` | pending | -- | -- |
| E01-07 | CHECK | Validation manuelle syntaxe WGSL (locations coherentes avec E02) | Denis | -- | pending | -- | -- |

## Critere de sortie
Fichier `shader.wgsl` complet, syntaxiquement valide, locations alignees avec layout `SpriteInstanceGpu` prevu en E02.

## Commit message template
`feat(mge-render): E01 -- shader WGSL sprite instancie`
