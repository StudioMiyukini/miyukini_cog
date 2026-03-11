# E05 -- Scene rogue_camp dans main.rs

## Statut : A faire
## Depend de : E03, E04
## Agents : Lise
## Taches : 9
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E05-01 | CODE | Ajouter imports SpriteBatch, SpriteInstance, SortKey, RenderLayer | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-02 | CODE | Ajouter imports AtlasHandle, MaterialHandle, SpriteRect, IsoCamera | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-03 | CODE | Ajouter champ batch: SpriteBatch dans SodomightApp | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-04 | CODE | Ajouter champ camera: IsoCamera (focus [8.0, 8.0]) dans SodomightApp | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-05 | CODE | Grille 16x16 terrain — boucle (tx, ty), world_to_screen, tint brun/vert, scale [80, 40] | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-06 | CODE | Joueur — world_to_screen(8, 8), layer Entities, tint rouge, scale [48, 48] | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-07 | CODE | HUD sante + mana — positions ecran fixes, layer UiScreen, tints rouge/bleu | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-08 | CODE | Appeler batch.sort() + renderer.render(&batch) dans RedrawRequested | Lise | `mge/games/sodomight/src/main.rs` | pending | -- | -- |
| E05-09 | CHECK | cargo check -p sodomight | Lise | -- | pending | -- | -- |

## Critere de sortie
`cargo check -p sodomight` vert. 259 instances (256 terrain + 1 joueur + 2 HUD) pushees par frame.

## GATE BIG_STEPS #2
Apres E05, pause pour validation humaine avant BUF.

## Commit message template
`feat(sodomight): E05 -- scene rogue_camp (grille iso + joueur + HUD)`
