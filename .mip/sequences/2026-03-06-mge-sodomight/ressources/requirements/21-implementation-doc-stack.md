# 21 - Documentation de soutien a l'implementation

## Documents a produire pendant P3

- carte du workspace `mge/`
- reference des crates et responsabilites
- registre des formules gameplay
- schemas de donnees content
- bible assets et nomenclature
- architecture rendu moderne et budgets GPU
- bible visuelle D2-like originale
- bible zones / quetes Acte 1
- roster monstres Acte 1
- tables de loot et vendors
- contrat save/load et migrations
- topologie backend MMO-ready
- contrat packaging Market / Central
- matrice de tests MVP

## Repartition minimale par etape

- E01 : carte workspace, conventions Rust, contrats inter-crates
- E02 : schemas data-driven, contrat save/load, migrations
- E03 : architecture renderer, pipeline assets, budget perf
- E03 : bible visuelle originale et liste d'assets a produire
- E04 : spec UI/HUD, map inputs, feedback/accessibilite
- E05 : formules gameplay, classes, skills, combat
- E06 : taxonomie items, vendors, loot, cube/craft
- E07 : bible camp + Acte 1, quetes, zones, randomisation
- E08 : roster monstres, IA, mercenaire, hardcore, party, PvP, ladder
- E08 : topologie backend, replication, service boundaries MMO-ready
- E09 : contrat packaging/install/runtime avec `Central`
- E10 : matrice de tests, equilibrage, dossier de transfert audit

## Regle de couverture

- toute feature D2 du MVP doit avoir une doc source ou une fiche de decision
- toute feature hors expression naturelle Acte 1 doit avoir une strategie de validation explicite
- toute doc de soutien doit etre assez stable pour guider code, tests et authoring

## Regle de production

- une doc par systeme critique
- documents courts et navigables
- toute formule critique doit etre documentee avant ou en meme temps que son implementation
- toute table de contenu doit avoir sa doc source

## Finalite

Ces documents ne sont pas annexes. Ils servent a soutenir directement l'implementation, les tests, l'equilibrage et la maintenance du moteur et du jeu.
