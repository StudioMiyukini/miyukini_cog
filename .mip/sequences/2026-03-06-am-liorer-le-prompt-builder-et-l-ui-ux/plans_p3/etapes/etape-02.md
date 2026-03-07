# E02 -- UI Base : layout bi-panneaux + champs enrichis

## Statut : A faire
## Depend de : E00 (parallele E01)
## Agents : Lise
## Taches : 3
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E02-01 | CODE | Layout bi-panneaux CSS : .builder-layout flex-row, .prompt-form flex:1, .prompt-preview flex:1, responsive 900px | Lise | static/app.css | pending | | |
| E02-02 | CODE | Champs enrichis HTML : ajouter T1/T2, stack select+preset, domaine etendu (ai-ml/securite/data), complexite en haut, mode autonomie | Lise | static/index.html | pending | | |
| E02-03 | CODE | Zone preview droite : ajouter textarea#pb-preview readonly dans la colonne droite, boutons Copier+InitSequence conserves | Lise | static/index.html | pending | | |

## Notes
E02 peut demarrer en parallele d'E01 (fichiers disjoints : CSS+HTML vs Rust). La zone preview est presente mais vide jusqu'en E03.

## Criteres de completion
- Visuel : layout bi-panneaux visible sur >= 900px, colonne sur < 900px
- T1/T2 presents dans le select pb-class
- Stack preset : 9 stacks + Autre, champ pb-stack auto-rempli
- Complexite pb-complexity visible avant le bouton Generer

## Commit message template
`feat(mipower): E02 -- UI layout bi-panneaux + champs enrichis (T1-T5, stack, domaine, autonomie)`
