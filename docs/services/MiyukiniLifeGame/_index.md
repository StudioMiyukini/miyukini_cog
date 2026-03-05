# Miyukini Life Game â€” Simulateur de vie et de civilisations

## PrÃ©sentation

**Miyukini Life Game** est un simulateur de vie (god simulator) dans l'Ã©cosystÃ¨me Miyukini COG. InspirÃ© de WorldBox, il permet aux utilisateurs de crÃ©er, observer et modifier des mondes avec des civilisations autonomes.

- **Genre :** God Simulator / Sandbox / Simulation de civilisations
- **Stack :** Rust, Dioxus, pixel art
- **Cibles :** Desktop, Web (WASM)
- **Type de Service :** Service interne COG (Type 1)

## Vision

CrÃ©er un monde, y placer des crÃ©atures et des races intelligentes, observer leur Ã©volution, intervenir avec des pouvoirs divins, dÃ©clencher des catastrophes ou aider les civilisations Ã  prospÃ©rer. Le joueur est un dieu observateur et interventionniste.

## Ordre de lecture recommandÃ©

1. **[Document Fondateur](MiyukiniLifeGame%20-%20Document%20Fondateur.md)** â€” Vision, genre, gameplay, stack, architecture COG.
2. **[Mecaniques de Jeu](MiyukiniLifeGame%20-%20Mecaniques%20de%20Jeu.md)** â€” SystÃ¨mes de jeu dÃ©taillÃ©s : races, civilisations, pouvoirs, diplomatie.
3. **[Pouvoirs Divins](MiyukiniLifeGame%20-%20Pouvoirs%20Divins.md)** â€” Catalogue complet des pouvoirs disponibles.
4. **[Systeme de Civilisations](MiyukiniLifeGame%20-%20Systeme%20de%20Civilisations.md)** â€” Comportements des races, royaumes, diplomatie, guerres.
5. **[Architecture Technique](MiyukiniLifeGame%20-%20Architecture%20Technique.md)** â€” Modules, flux, intÃ©gration dans Miyukini COG.
6. **[Guide Implementation MVP](MiyukiniLifeGame%20-%20Guide%20Implementation%20MVP.md)** â€” Phases d'implÃ©mentation, modÃ¨le d'Ã©tat, APIs.
7. **[Reference WorldBox](MiyukiniLifeGame%20-%20Reference%20WorldBox.md)** â€” Analyse dÃ©taillÃ©e de WorldBox comme rÃ©fÃ©rence.

## Documents

| Document | Description |
|----------|-------------|
| [MiyukiniLifeGame - Document Fondateur](MiyukiniLifeGame%20-%20Document%20Fondateur.md) | Vision du jeu, genre, gameplay principal, intÃ©gration dans Miyukini COG, stack technique, inspirations. |
| [MiyukiniLifeGame - Mecaniques de Jeu](MiyukiniLifeGame%20-%20Mecaniques%20de%20Jeu.md) | MÃ©caniques principales : crÃ©ation de monde, races, crÃ©atures, Ã©volution, simulation. |
| [MiyukiniLifeGame - Pouvoirs Divins](MiyukiniLifeGame%20-%20Pouvoirs%20Divins.md) | Catalogue des 7 catÃ©gories de pouvoirs : crÃ©ation, destruction, crÃ©atures, magiques, contrÃ´le du temps, spÃ©ciaux, effets. |
| [MiyukiniLifeGame - Systeme de Civilisations](MiyukiniLifeGame%20-%20Systeme%20de%20Civilisations.md) | SystÃ¨mes de civilisations : races (Humains, Orcs, Elfes, Nains), traits raciaux, royaumes, diplomatie, guerres, rÃ©bellions. |
| [MiyukiniLifeGame - Architecture Technique](MiyukiniLifeGame%20-%20Architecture%20Technique.md) | Architecture technique : modules (simulation, entitÃ©s, monde, pouvoirs), flux de donnÃ©es, Toolkits requis. |
| [MiyukiniLifeGame - Guide Implementation MVP](MiyukiniLifeGame%20-%20Guide%20Implementation%20MVP.md) | Guide d'implÃ©mentation du MVP : phases, modÃ¨le d'Ã©tat, APIs, format de donnÃ©es, planning. |
| [MiyukiniLifeGame - Reference WorldBox](MiyukiniLifeGame%20-%20Reference%20WorldBox.md) | Analyse dÃ©taillÃ©e de WorldBox : mÃ©caniques, fonctionnalitÃ©s, ce qui inspire Miyukini Life Game. |

## Comparaison avec WorldBox

| Aspect | WorldBox | Miyukini Life Game |
|--------|----------|-------------------|
| Plateforme | Steam, Mobile | Desktop, Web (WASM) |
| Architecture | Standalone | IntÃ©grÃ© dans Miyukini COG |
| Gouvernance | Aucune | GouvernÃ© par les Cores |
| Sauvegarde | Local | Via KindMother (sync multi-device) |
| Permissions | N/A | Mandats via StrongFather |
| Races | 4 (Humains, Orcs, Elfes, Nains) | 4 + extensible via Toolkits |
| Pouvoirs | 230+ pouvoirs | MVP: 50+ pouvoirs, extensible |
| Modding | Steam Workshop | Via Toolkits Miyukini |

## IntÃ©gration Miyukini COG

### Strate 7 â€” Service

- **Service :** MiyukiniLifeGame
- **Type :** Service interne COG (Type 1)
- **PrÃ©sence :** Miyukini Central uniquement

### OpÃ©rateurs requis

- **LifeGame.Simulation** â€” GÃ¨re la simulation du monde
- **LifeGame.Entities** â€” GÃ¨re les entitÃ©s (crÃ©atures, unitÃ©s, bÃ¢timents)
- **LifeGame.World** â€” GÃ¨re le terrain et l'environnement
- **LifeGame.Powers** â€” GÃ¨re les pouvoirs divins

### Toolkits requis (Strate 6)

- **MiyuWorldGen** â€” GÃ©nÃ©ration procÃ©durale de mondes
- **MiyuPixelCanvas** â€” Rendu pixel art et carte
- **MiyuEntitySim** â€” Simulation d'entitÃ©s autonomes
- **MiyuDiplomacy** â€” SystÃ¨me de diplomatie entre nations
- **MiyuPathfinding** â€” Calcul de chemins pour les unitÃ©s
- **MiyuParticles** â€” Effets visuels des pouvoirs

### Cores utilisÃ©s (Strate 4)

- **StrongFather** â€” DÃ©cisions et permissions pour les pouvoirs divins
- **KindMother** â€” Sauvegarde et synchronisation des mondes
- **CaringNanny** â€” Observation de l'Ã©tat du monde et des statistiques
- **MasterButler** â€” Orchestration des capacitÃ©s et Toolkits
- **EverBuddy** â€” Gestion des versions de mondes et migrations
- **WorrySentinel** â€” Limites de ressources (taille du monde, nombre d'entitÃ©s)

## Liens utiles

- [Stack UI Dioxus](..//..//_index.md) â€” Stack UI officielle Miyukini
- [Architecture Miyukini COG](_index.md) â€” Strates, Cores, Lois d'Autonomie
- [Glossaire Miyukini](_index.md) â€” Terminologie officielle

