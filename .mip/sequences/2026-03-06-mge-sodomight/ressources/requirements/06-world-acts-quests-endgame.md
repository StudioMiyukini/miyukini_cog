# 06 - Monde, actes, quetes, endgame

## Structure macro D2 a reproduire

- ville hub
- zones exterieures
- donjons connectes
- checkpoints quetes
- boss d'acte
- passage d'acte
- trois difficultes successives

## Besoins campagne

- au moins 4 a 5 actes equivalent cible finale
- MVP systemes complets + contenu borne:
  - 1 ville de depart complete
  - Acte 1 complet avec ses zones et donjons
  - 1 boss final d'acte
  - quetes critiques et secondaires de l'Acte 1
  - couverture systemique des autres features D2 via data/tests/harness si hors Acte 1

## Types de quetes a couvrir

- tuer un boss
- recuperer un objet cle
- activer un point de passage
- debloquer un service de ville
- purifier ou ouvrir une zone
- sauver un PNJ / debloquer un mercenaire

## Randomisation

- layouts de zones a partir de chunks et connectors
- densite variable des packs
- variations de coffres, elites, events
- placement stable de certains objectifs de quete critiques

## Systeme de difficulte

- `Normal`, `Nightmare`, `Hell` equivalent
- progression:
  - stats ennemies
  - resistances
  - immunites
  - tables de drop
  - XP curves

## Endgame cible

- cible MVP:
  - boss runs
  - farming areas haut niveau
  - mode hardcore
  - ladder runtime et classement validables
- cible contenu/exploitation ulterieurs:
  - uber encounters
  - maps / modifiers limites
  - economy events

## Exigences moteur

- systeme de quetes data-driven
- spawner de monde et graph de zones
- support seeds de generation
- persistance d'etat:
  - quetes terminees
  - waypoints
  - cinematics sautees
  - difficultes debloquees
