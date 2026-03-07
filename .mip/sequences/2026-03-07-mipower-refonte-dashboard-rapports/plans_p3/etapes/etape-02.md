# E02 — Dashboard : tri multi-criteres

## Statut : Terminé
## Depend de : E00
## Agents : Lise
## Taches : 3

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E02-01 | FEAT | Ajouter select#sortBy dans header dashboard (date-desc, date-asc, name-asc, class-asc, status) | Lise | static/index.html | pending |
| E02-02 | FEAT | Implémenter sortSequences() dans app.js + attacher event sortBy + mise a jour renderSequences | Lise | static/app.js | pending |
| E02-03 | FEAT | Styler .sort-group, badges statut colorés (done=vert, active=bleu, archived=gris) | Lise | static/app.css | pending |

## Notes
- Tri garde le filtre statut existant
- Badges de statut : couleur semantique claire

## Commit message template
`feat(mipower): E02 -- dashboard tri multi-criteres + badges statut colores`
