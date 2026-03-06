# 15 - Zones, randomisation, waypoints, quetes

## Zones

- biomes distincts
- tilesets reutilisables
- connectors et transitions
- identite lisible par acte / chapitre

## Randomisation

- chunks pre-authorises
- portes / sorties compatibles
- variation de densite
- variation des packs elites
- variation coffres et events
- seed persistable pour debug

## Waypoints et fast travel

- waypoints debloquables
- persistance par personnage
- UX de selection simple
- blocages par progression de quete possibles

## Teleportation de secours

- town portal equivalent
- portail aller-retour
- fermeture si conditions de design le demandent

## Quetes

- categorie:
  - main quest
  - optional reward quest
  - service unlock quest
  - boss gate quest
- etats:
  - available
  - started
  - blocked
  - completed
  - reward claimed

## Exigences moteur

- graph de zones
- generator seeds
- state quest versionne
- triggers monde:
  - enter area
  - kill target
  - pickup item
  - talk NPC
  - activate altar
