# 13 - Services de ville, cube, vendors et crafting

## Services de ville a couvrir

- stash
- vendeur d'armes
- vendeur de magie
- vendeur de potions
- heal / resurrect mercenaire
- identify
- repair
- gamble
- voyage / passage acte
- quete services debloques

## Stash

- stockage persistant
- tri manuel au depart
- extension future vers stash partage
- transfert d'objets entre personnages a definir selon mode solo / account

## Vendors

- catalogues dependants:
  - acte
  - progression
  - difficulte
  - seed / refresh
- rachat prix bas
- achat potions, parchemins, bases, composants
- vente des drops communs pour boucle or

## Cube equivalent

- UI propre et distincte
- recipes:
  - transmuter gemmes/composants
  - reroll objets
  - upgrade de tiers
  - ouvrir contenus caches
  - recettes de quete

## Crafting

- MVP:
  - cube recipes seulement
- post-MVP:
  - artisanat additionnel si cela sert l'identite Sodomight

## UX critique

- drag and drop fiable
- ghost preview du resultat de recette si connue
- journal des recettes decouvertes plus tard possible
- erreurs claires si recette invalide

## Exigences moteur

- table de recettes data-driven
- serialisation des inventaires et contenants
- permissions simples pour interactions vendor / cube / stash
- tests de roundtrip inventaire-cube-vendor
