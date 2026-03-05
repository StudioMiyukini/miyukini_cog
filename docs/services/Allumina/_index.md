# Allumina â€” Index du Service

## Contexte

**Allumina** est le **jeu Action RPG** de l'Ã©cosystÃ¨me Miyukini COG. Il utilisera le **moteur de jeu Miyukini** (Rust, maison) et le **MWS (Miyukini Webway System)** pour la partie multijoueur (dÃ©couverte des parties, Lobbys, connexion entre COGs). Voir [Moteur Jeux et Central Launcher](..//..//_index.md).

## PortÃ©e / Scope

Cet index rÃ©fÃ©rence toute la documentation du service Allumina : fondateur, concepts, architecture, intÃ©gration MWS.

---

## Documents fondateurs et de rÃ©fÃ©rence

| Document | RÃ´le |
|----------|------|
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision, positionnement, MWS multijoueur, Lois d'Autonomie |
| [Concept â€” Index](./Concept/_index.md) | Index des documents conceptuels |

---

## Dossier Concept

Les documents **conceptuels** dÃ©crivent la vision jeu, le genre (Action RPG), le monde, les mÃ©caniques et l'usage du MWS pour le multijoueur.

| Document | RÃ´le |
|----------|------|
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Vision jeu, genre, monde, persistance, Lobbys = parties/serveurs |
| [Allumina - Vision Gameplay et Ambition](./Concept/Allumina%20-%20Vision%20Gameplay%20et%20Ambition.md) | MÃ©lange UO/Diablo/Dynasty Warriors, progression esclaveâ†’hÃ©ros, mÃ©tiers, ambitions |
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Ã‰chelles, voies, ordres tactiques, cap Charisme+statut |
| [Allumina - CompÃ©tences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Usage+enseignement, pas d'XP, PNJ vs joueur |
| [Allumina - CaractÃ©ristiques, Aptitudes et CompÃ©tences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | CaractÃ©ristiques (Force Ã  Chance), aptitudes de combat, compÃ©tences, plafonds |

## Prototype et MVP

| Document | RÃ´le |
|----------|------|
| [Allumina - Prototype Premier Playable](./Allumina%20-%20Prototype%20Premier%20Playable.md) | Proto initial : 2 joueurs, sync troupes+monstres, co-op PvE |
| [Allumina - MVP Sandbox](./Allumina%20-%20MVP%20Sandbox.md) | MVP strict 3â€“6 mois : 13 systÃ¨mes, 50â€“200 joueurs, boucle combat+troupes+craft+trade, plan d'implÃ©mentation par phases |

## Analyse technique (rÃ©fÃ©rences ARPG)

| Document | RÃ´le |
|----------|------|
| [Allumina - Analyse Technique Diablo II pour MGE](./Allumina%20-%20Analyse%20Technique%20Diablo%20II%20pour%20MGE.md) | Reverse-engineering des systÃ¨mes D2 (dÃ©placement, IA, spawn, projectiles, followers) et transposition MGE |
| [Allumina - Extraction SystÃ¨mes D2 OpenDiablo2 pour MGE](./Allumina%20-%20Extraction%20Systemes%20D2%20OpenDiablo2%20pour%20MGE.md) | Extraction exhaustive des 10 systÃ¨mes D2 (code OpenDiablo2, formules exactes, structures de donnÃ©es, propositions ECS MGE) |
| [Allumina - Extraction Architecture UO pour MGE](./Allumina%20-%20Extraction%20Architecture%20UO%20pour%20MGE.md) | Reverse-engineering UO (ServUO, ModernUO, ClassicUO, UOX3) : housing, skills, craft, economy, persistence, rÃ©seau, adaptation MGE |

## Blueprint moteur

| Document | RÃ´le |
|----------|------|
| [Allumina - Blueprint Moteur Sandbox MGE](./Allumina%20-%20Blueprint%20Moteur%20Sandbox%20MGE.md) | Architecture moteur complÃ¨te : 8 engines Allumina (Territorial, Progression, Ã‰cologique, Production, RÃ©gional, Scheduler, Persistence, Network), Ã©conomie vivante, simulation Ã©cologique, Timer Wheel, blueprint modules, configs, cluster MMO |

---

## RÃ©fÃ©rences externes

| Document | RÃ´le |
|----------|------|
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | PrÃ©sence, dÃ©couverte, transport, Lobbys |
| [MWS - Lobbys, Favoris et Amis](../../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privÃ©s, surfaces, accord d'hÃ´te |

---

**Document** : Allumina â€” Index du Service  
**Version** : 1.0  
**Date** : 2026-02-17

