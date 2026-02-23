# Allumina — Index du Service

## Contexte

**Allumina** est le **jeu Action RPG** de l'écosystème Miyukini COG. Il utilisera le **moteur de jeu Miyukini** (Rust, maison) et le **MWS (Miyukini Webway System)** pour la partie multijoueur (découverte des parties, Lobbys, connexion entre COGs). Voir [Moteur Jeux et Central Launcher](../../games/Miyukini%20-%20Moteur%20Jeux%20et%20Central%20Launcher.md).

## Portée / Scope

Cet index référence toute la documentation du service Allumina : fondateur, concepts, architecture, intégration MWS.

---

## Documents fondateurs et de référence

| Document | Rôle |
|----------|------|
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision, positionnement, MWS multijoueur, Lois d'Autonomie |
| [Concept — Index](./Concept/_index.md) | Index des documents conceptuels |

---

## Dossier Concept

Les documents **conceptuels** décrivent la vision jeu, le genre (Action RPG), le monde, les mécaniques et l'usage du MWS pour le multijoueur.

| Document | Rôle |
|----------|------|
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Vision jeu, genre, monde, persistance, Lobbys = parties/serveurs |
| [Allumina - Vision Gameplay et Ambition](./Concept/Allumina%20-%20Vision%20Gameplay%20et%20Ambition.md) | Mélange UO/Diablo/Dynasty Warriors, progression esclave→héros, métiers, ambitions |
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Échelles, voies, ordres tactiques, cap Charisme+statut |
| [Allumina - Compétences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Usage+enseignement, pas d'XP, PNJ vs joueur |
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Caractéristiques (Force à Chance), aptitudes de combat, compétences, plafonds |

## Prototype et MVP

| Document | Rôle |
|----------|------|
| [Allumina - Prototype Premier Playable](./Allumina%20-%20Prototype%20Premier%20Playable.md) | Proto initial : 2 joueurs, sync troupes+monstres, co-op PvE |
| [Allumina - MVP Sandbox](./Allumina%20-%20MVP%20Sandbox.md) | MVP strict 3–6 mois : 13 systèmes, 50–200 joueurs, boucle combat+troupes+craft+trade, plan d'implémentation par phases |

## Analyse technique (références ARPG)

| Document | Rôle |
|----------|------|
| [Allumina - Analyse Technique Diablo II pour MGE](./Allumina%20-%20Analyse%20Technique%20Diablo%20II%20pour%20MGE.md) | Reverse-engineering des systèmes D2 (déplacement, IA, spawn, projectiles, followers) et transposition MGE |
| [Allumina - Extraction Systèmes D2 OpenDiablo2 pour MGE](./Allumina%20-%20Extraction%20Systemes%20D2%20OpenDiablo2%20pour%20MGE.md) | Extraction exhaustive des 10 systèmes D2 (code OpenDiablo2, formules exactes, structures de données, propositions ECS MGE) |
| [Allumina - Extraction Architecture UO pour MGE](./Allumina%20-%20Extraction%20Architecture%20UO%20pour%20MGE.md) | Reverse-engineering UO (ServUO, ModernUO, ClassicUO, UOX3) : housing, skills, craft, economy, persistence, réseau, adaptation MGE |

## Blueprint moteur

| Document | Rôle |
|----------|------|
| [Allumina - Blueprint Moteur Sandbox MGE](./Allumina%20-%20Blueprint%20Moteur%20Sandbox%20MGE.md) | Architecture moteur complète : 8 engines Allumina (Territorial, Progression, Écologique, Production, Régional, Scheduler, Persistence, Network), économie vivante, simulation écologique, Timer Wheel, blueprint modules, configs, cluster MMO |

---

## Références externes

| Document | Rôle |
|----------|------|
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, transport, Lobbys |
| [MWS - Lobbys, Favoris et Amis](../../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privés, surfaces, accord d'hôte |

---

**Document** : Allumina — Index du Service  
**Version** : 1.0  
**Date** : 2026-02-17
