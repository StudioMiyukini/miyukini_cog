# 05 - Itemization, loot et economie

## Hierarchie de valeur a reproduire

- base item
- quality tier
- affixes
- sockets
- gemmes / runes / joyaux equivalent
- unique / set / crafted / runeword equivalent

## Types d'objets requis

- armes melee 1h / 2h
- armes distance
- focus / catalyseurs / boucliers
- armures corps
- casques
- gants
- bottes
- ceintures
- anneaux
- amulettes
- charms equivalent
- consommables
- quete keys / organes equivalent si endgame ajoute plus tard

## Economie de base

- gold sink:
  - reparation
  - achats vendors
  - gamble
  - craft / cube recipes
- drop economy:
  - gold
  - materials
  - normal / magic / rare / unique / set
- vendor economy:
  - reroll periodic
  - buyback minimum
  - niveaux d'objets par acte / difficulte

## Sous-systemes incontournables

### Sockets et runewords equivalent

- bases compatibles
- nombre de sockets
- ordre de pose
- resultat deterministic
- difficulte d'obtention des composants

### Cube equivalent

- recipes de transmutation
- upgrade d'items
- reroll affixes
- conversion de composants
- quete recipes

### Identification et tooltip

- objets identifies / non identifies
- comparaison item equipe
- affichage:
  - quality color
  - requirements
  - defense / damage
  - affixes
  - sockets
  - tags classe / mercenaire

## Besoins Sodomight

- loot filter futur, mais hors MVP
- stash plus robuste que D2 original
- systeme de seed et tables de loot entierement data-driven
- support du trade joueur a joueur a moyen terme

## Exigences moteur

- generateur d'objets versionne
- tables de drop et treasure classes separables par zone, monstre, coffre et event
- serialisation stable des instances d'items
- tests statistiques sur le loot
