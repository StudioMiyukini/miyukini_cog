# P0 Temps 03 - Analyse concurrentielle

## Concurrents et references directes

- Diablo II / Lord of Destruction
  - reference de game feel, progression, itemisation, structure actes-difficultes
- Diablo II Resurrected
  - reference de modernisation UX sans casser le coeur systemique
- Path of Exile
  - reference pour profondeur de build, monetisation hors scope, endgame moderne
- Last Epoch
  - reference pour lisibilite craft, telemetrie de progression et onboarding moderne
- Grim Dawn
  - reference pour personnalisation offline, densite de contenu solo et pipelines data-driven

## Reverse-engineering et extraction technique

- OpenDiablo2
  - utile pour comprendre records, cartes, entites, mouvements, fichiers de donnees
- diablo2.io
  - utile pour cross-check runewords, breakpoints, mercenaires, progression pratique
- Diablo Wiki
  - utile pour consolider les systemes de base et les modes de jeu
- docs internes Allumina
  - utile pour accelerer l'abstraction MGE deja amorcee dans le depot

## Enseignements actionnables

- Copier D2 sans pipeline data-driven serait une erreur: trop de tables, de breakpoints et de variations.
- Le coeur de valeur n'est pas juste le combat; c'est l'emboitement campagne + loot + reroll + difficultes + social.
- Le rendu doit etre proprietaire mais modulaire: si le batch sprites et l'eclairage palette sont mauvais, tout le ressenti s'effondre.
