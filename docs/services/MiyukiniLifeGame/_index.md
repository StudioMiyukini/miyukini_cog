# Miyukini Life Game — Simulateur de vie et de civilisations

## Présentation

**Miyukini Life Game** est un simulateur de vie (god simulator) dans l'écosystème Miyukini COG. Inspiré de WorldBox, il permet aux utilisateurs de créer, observer et modifier des mondes avec des civilisations autonomes.

- **Genre :** God Simulator / Sandbox / Simulation de civilisations
- **Stack :** Rust, Dioxus, pixel art
- **Cibles :** Desktop, Web (WASM)
- **Type de Service :** Service interne COG (Type 1)

## Vision

Créer un monde, y placer des créatures et des races intelligentes, observer leur évolution, intervenir avec des pouvoirs divins, déclencher des catastrophes ou aider les civilisations à prospérer. Le joueur est un dieu observateur et interventionniste.

## Ordre de lecture recommandé

1. **[Document Fondateur](MiyukiniLifeGame%20-%20Document%20Fondateur.md)** — Vision, genre, gameplay, stack, architecture COG.
2. **[Mecaniques de Jeu](MiyukiniLifeGame%20-%20Mecaniques%20de%20Jeu.md)** — Systèmes de jeu détaillés : races, civilisations, pouvoirs, diplomatie.
3. **[Pouvoirs Divins](MiyukiniLifeGame%20-%20Pouvoirs%20Divins.md)** — Catalogue complet des pouvoirs disponibles.
4. **[Systeme de Civilisations](MiyukiniLifeGame%20-%20Systeme%20de%20Civilisations.md)** — Comportements des races, royaumes, diplomatie, guerres.
5. **[Architecture Technique](MiyukiniLifeGame%20-%20Architecture%20Technique.md)** — Modules, flux, intégration dans Miyukini COG.
6. **[Guide Implementation MVP](MiyukiniLifeGame%20-%20Guide%20Implementation%20MVP.md)** — Phases d'implémentation, modèle d'état, APIs.
7. **[Reference WorldBox](MiyukiniLifeGame%20-%20Reference%20WorldBox.md)** — Analyse détaillée de WorldBox comme référence.

## Documents

| Document | Description |
|----------|-------------|
| [MiyukiniLifeGame - Document Fondateur](MiyukiniLifeGame%20-%20Document%20Fondateur.md) | Vision du jeu, genre, gameplay principal, intégration dans Miyukini COG, stack technique, inspirations. |
| [MiyukiniLifeGame - Mecaniques de Jeu](MiyukiniLifeGame%20-%20Mecaniques%20de%20Jeu.md) | Mécaniques principales : création de monde, races, créatures, évolution, simulation. |
| [MiyukiniLifeGame - Pouvoirs Divins](MiyukiniLifeGame%20-%20Pouvoirs%20Divins.md) | Catalogue des 7 catégories de pouvoirs : création, destruction, créatures, magiques, contrôle du temps, spéciaux, effets. |
| [MiyukiniLifeGame - Systeme de Civilisations](MiyukiniLifeGame%20-%20Systeme%20de%20Civilisations.md) | Systèmes de civilisations : races (Humains, Orcs, Elfes, Nains), traits raciaux, royaumes, diplomatie, guerres, rébellions. |
| [MiyukiniLifeGame - Architecture Technique](MiyukiniLifeGame%20-%20Architecture%20Technique.md) | Architecture technique : modules (simulation, entités, monde, pouvoirs), flux de données, Toolkits requis. |
| [MiyukiniLifeGame - Guide Implementation MVP](MiyukiniLifeGame%20-%20Guide%20Implementation%20MVP.md) | Guide d'implémentation du MVP : phases, modèle d'état, APIs, format de données, planning. |
| [MiyukiniLifeGame - Reference WorldBox](MiyukiniLifeGame%20-%20Reference%20WorldBox.md) | Analyse détaillée de WorldBox : mécaniques, fonctionnalités, ce qui inspire Miyukini Life Game. |

## Comparaison avec WorldBox

| Aspect | WorldBox | Miyukini Life Game |
|--------|----------|-------------------|
| Plateforme | Steam, Mobile | Desktop, Web (WASM) |
| Architecture | Standalone | Intégré dans Miyukini COG |
| Gouvernance | Aucune | Gouverné par les Cores |
| Sauvegarde | Local | Via KindMother (sync multi-device) |
| Permissions | N/A | Mandats via StrongFather |
| Races | 4 (Humains, Orcs, Elfes, Nains) | 4 + extensible via Toolkits |
| Pouvoirs | 230+ pouvoirs | MVP: 50+ pouvoirs, extensible |
| Modding | Steam Workshop | Via Toolkits Miyukini |

## Intégration Miyukini COG

### Strate 7 — Service

- **Service :** MiyukiniLifeGame
- **Type :** Service interne COG (Type 1)
- **Présence :** Miyukini Central uniquement

### Opérateurs requis

- **LifeGame.Simulation** — Gère la simulation du monde
- **LifeGame.Entities** — Gère les entités (créatures, unités, bâtiments)
- **LifeGame.World** — Gère le terrain et l'environnement
- **LifeGame.Powers** — Gère les pouvoirs divins

### Toolkits requis (Strate 6)

- **MiyuWorldGen** — Génération procédurale de mondes
- **MiyuPixelCanvas** — Rendu pixel art et carte
- **MiyuEntitySim** — Simulation d'entités autonomes
- **MiyuDiplomacy** — Système de diplomatie entre nations
- **MiyuPathfinding** — Calcul de chemins pour les unités
- **MiyuParticles** — Effets visuels des pouvoirs

### Cores utilisés (Strate 4)

- **StrongFather** — Décisions et permissions pour les pouvoirs divins
- **KindMother** — Sauvegarde et synchronisation des mondes
- **CaringNanny** — Observation de l'état du monde et des statistiques
- **MasterButler** — Orchestration des capacités et Toolkits
- **EverBuddy** — Gestion des versions de mondes et migrations
- **WorrySentinel** — Limites de ressources (taille du monde, nombre d'entités)

## Liens utiles

- [Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md) — Stack UI officielle Miyukini
- [Architecture Miyukini COG](.cursor/skills/miyukini-architecture/SKILL.md) — Strates, Cores, Lois d'Autonomie
- [Glossaire Miyukini](.cursor/skills/miyukini-glossary/SKILL.md) — Terminologie officielle
