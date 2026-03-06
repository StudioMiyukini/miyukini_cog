# 01 - Gamedesign pillars

## Intention produit

`Sodomight` doit etre un ARPG isometrique dark fantasy focalise sur :

- satisfaction de combat immediate
- lisibilite tres forte des archetypes et des menaces
- intensite de loot et de progression
- rejouabilite via builds, difficultes et randomisation
- friction utile, pas friction punitive gratuite

## Piliers herites de Diablo II

### 1. Identite de classe tres forte

- chaque classe a une silhouette, un tempo et une relation au risque uniques
- les trees de skills forcent des choix, meme si des hybrides existent
- le build est un engagement, pas un preset interchangeable a chaque salle

### 2. Boucle "sortir -> risquer -> rentrer"

- la ville est un hub de securite
- la zone terrain est un espace de pression
- le joueur alterne combat, tension inventaire, soins, vente, craft, rerun

### 3. Itemisation plus importante que le niveau seul

- la puissance vient du niveau, du build, des breakpoints et des objets
- les bases d'items comptent autant que la rarete
- les objets systemiques generent de la conversation et du theorycraft

### 4. Monde de campagne structurant

- la progression n'est pas un menu de missions
- les actes, quetes, town hubs et boss gates donnent un cadre clair
- la randomisation intervient a l'interieur des zones, pas a la place de la structure

### 5. Rejouabilite par repetition signifiante

- reruns d'areas, boss runs, farm de runes, rerolls, ladders, hardcore
- la repetition est acceptee parce qu'elle est lisible, rentable et socialement partageable

## Traduction Sodomight

### Ce qui doit etre reproduit presque a l'identique

- vue isometrique combat-loot-ville
- archetypes de classes fortement differencies
- progression normal -> nightmare -> hell equivalent
- drop model multi-couches: bases, affixes, sets, uniques, sockets, runes
- town portal, identif, potions, mercenaires, stash, cube equivalent

### Ce qui doit etre modernise

- ergonomie inventaire et stash
- options accessibilite et remap
- pipeline de telemetrie et crash reports
- packaging et installation
- stabilite coop et reprise de session

### Ce qui doit etre reporte

- endgame moderne tres large type PoE
- housing, crafting complexe hors cube-equivalent
- systems live-service lourds

## Exigences moteur derivees

- simulation data-driven pour eviter le code special-case
- renderer centré sur lisibilite et temps de reponse
- architecture capable de gerer beaucoup de variantes d'items et de skills
- outillage de debug gameplay des le debut
