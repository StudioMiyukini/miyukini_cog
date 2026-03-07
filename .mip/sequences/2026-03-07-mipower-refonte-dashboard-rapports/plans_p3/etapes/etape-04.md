# E04 — Arbre badges done/pending + polish final

## Statut : Terminé
## Depend de : E01, E03
## Agents : Lise + Francois
## Taches : 3

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E04-01 | FEAT | Adapter renderArtefactTree pour utiliser champ `done` des artefacts backend | Lise | static/app.js | pending |
| E04-02 | FEAT | Styler .tree-item.done (indicateur vert) + .tree-item.pending (gris) | Lise | static/app.css | pending |
| E04-03 | POLISH | Version bump v0.3.0, tests Rust passes, clippy clean | Francois | src/api.rs, static/index.html | pending |

## Notes
- `done` = fichier contient "Etat : TERMINE" OU "Statut : Terminé"
- Version footer : v0.3.0 — E04

## Commit message template
`feat(mipower): E04 -- arbre badges done/pending + polish v0.3.0`
