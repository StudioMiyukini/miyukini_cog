# 23 - Bible visuelle et liste d'assets Sodomight

## Regle directrice

`Sodomight` doit evoquer Diablo II par sa lisibilite, son angle isometrique, sa densite de details et son rythme d'animation, sans jamais en etre une copie exacte.

## Ce qui est autorise

- reprendre les grandes familles d'assets a couvrir
- reprendre les principes de silhouette et de contraste
- reprendre l'idee de spritesheets directionnelles et de tilesets par biome

## Ce qui est interdit

- reutiliser des assets D2
- tracer, repeindre ou recoloriser un sprite D2
- cloner les bordures UI, runes, icones ou silhouettes de boss

## Signatures visuelles Sodomight

- silhouettes un peu plus hautes et anguleuses que D2
- materiaux plus corroded iron, bone ivory, wax red, verdigris
- motifs religieux et occultes originaux
- UI gothique plus sobre, moins baroque que D2
- VFX plus nets dans leur lecture, mais retenus dans leur saturation

## Liste d'assets a generer pour le MVP

### Heros jouables

- 7 archetypes visuels jouables
- corps de base par sexe si retenu au design final
- 3 familles de silhouette d'armure:
  - legere
  - moyenne
  - lourde
- overlays armes:
  - epee
  - hache
  - masse
  - dague
  - baton
  - baguette
  - lance/polearm
  - arc
  - arbalete
- overlays off-hand:
  - petits boucliers
  - grands boucliers
  - focus occultes
- clips minimum:
  - idle
  - town idle
  - walk
  - run
  - basic attack 1
  - basic attack 2
  - cast start
  - cast release
  - hit react
  - block
  - interact
  - death
  - corpse

### PNJ du camp

- forgeron
- soigneuse
- marchande generaliste
- occultiste / identificateur
- gambler
- chef mercenaire
- ancien / donneur de quetes
- gardien stash
- gardien waypoint / portail

### Monstres Acte 1

- demonettes de melee type swarm
- invocateurs / shamans
- zombies ou husks
- archers corrompus
- brutes bestiales
- squelettes melee
- squelettes ranged
- wraiths / fantomes
- arachnides / vermines
- volants charognards
- elites palette-swaps + accessoires uniques
- boss final d'acte

### Environnements

- camp de depart
- lande sanglante / moor
- cimetiere
- grotte
- crypte
- village en ruine
- monastere exterieur
- casernes
- prison
- cathedrale
- catacombes
- salle de boss

### Props et interactifs

- coffres
- tonneaux
- sarcophages
- torches
- braseros
- puits
- arbres morts
- rochers
- barricades
- portes
- grilles
- wagons
- tentes
- autels
- waypoints
- portails
- stashes
- stands vendors

### VFX

- traines melee
- impacts physiques
- sang
- poison
- feu
- glace
- eclairs
- maledictions
- auras
- portails
- resurrection
- sparkles de loot
- meteo locale

### UI et icones

- cadre HUD
- orbes vie/ressource
- inventaire
- stash
- vendor
- journal de quetes
- feuille personnage
- barres de skill
- icones potions
- icones skills
- icones affixes / raretes
- curseurs et ping carte

## Methode de generation

1. blockout 3D ou concept paint pour poser volume et silhouette
2. rendu source fixe sous camera/light de reference
3. paintover et salissure manuelle
4. reduction/polish palette
5. decoupe sheet, metadata, tests de lisibilite

## Reference visuelle

Des bases comme Spriters Resource servent a verifier la couverture des familles d'assets D2: classes, NPCs, pieces Rogue, animaux, ennemis et boss, tiles d'acte, objets, projectiles et effets. Elles ne servent pas de bibliotheque de production.
