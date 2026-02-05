# Miyukini Survivor — Document Fondateur

## Contexte

**Miyukini Survivor** est le **service Miyukini du domaine jeu Survivor / Tower Defense** au sein de l'écosystème COG. Il propose une expérience de jeu hybride : le joueur se déplace sur l'écran en 8 directions, attaque (base, armes de jet, sortilèges), protège un objectif central (**le Château**), construit des **tours** en phase **Préparation**, et affronte des **vagues d'ennemis** en phase **Bataille**.

Ce document est le **document fondateur** du service : il en fixe la raison d'être, le scope, les concepts de base du jeu, le positionnement (Survivor + Tower Defense) et les décisions structurantes. Il s'adresse aux équipes produit, technique et parties prenantes.

## Portée / Scope

- **Périmètre :** Définition du service Miyukini Survivor — fondation du scope, concepts du jeu, gameplay, phases, affichage ; positionnement stratégique.
- **Hors périmètre :** Spécifications techniques détaillées (API, moteur, implémentation des crates), design visuel et sonore finaux.
- **Références :** [Glossaire Miyukini](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Analyse Concurrence Survivor Tower Defense](./Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md).

---

## 1. Fondation du scope et des concepts du jeu

### 1.1 Raison d'être

**Miyukini Survivor** a pour objectif de :

- **Proposer un jeu hybride Survivor + Tower Defense** au sein de l'écosystème Miyukini : le joueur est actif (déplacement, attaques manuelles et auto), protège un objectif (le Château), et construit des tours en phase de préparation.
- **Respecter les mécaniques aimées par les joueurs** du genre (progression en run, évolution/builds, meta-progression, variété, contrôles simples, challenge) — voir [Analyse Concurrence](./Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md).
- **Exposer un service COG** : logique métier, progression, sauvegardes et intégrations (Miyukini Central, éventuels Opérateurs) selon les règles de gouvernance Miyukini.

### 1.2 Concepts de base

| Concept | Description |
|--------|-------------|
| **Zone de jeu** | Un seul écran ; le joueur, le Château, les ennemis et les tours évoluent dans cet espace. |
| **Le Château** | Objectif central des ennemis ; il possède des PV et de l'armure, pas d'attaques. À 0 PV = game over. |
| **Phase Préparation** | Le joueur dépense or, points de compétences, et construit des tours dans la zone autorisée (disque vert autour du Château). |
| **Phase Bataille** | Vagues d'ennemis depuis les bords vers le Château ; le joueur et les tours les combattent. |
| **Joueur** | Avatar déplaçable en 8 directions ; attaque de base (auto), armes de jet (clic), sortilèges (projectiles vers l'ennemi le plus proche). |
| **Tours** | Bâtiments construits en phase Préparation ; ils attaquent les ennemis à portée pendant la Bataille. |
| **Ennemis** | Se dirigent vers le Château ; priorité de cible : Joueur > Bâtiment > Château. Donnent or et XP à la mort. |

### 1.3 Périmètre fonctionnel (scope)

- **In scope :** Un joueur, un Château, vagues d'ennemis, tours construites en phase Préparation, attaque de base + armes de jet + sortilèges, or et XP, arbre de compétences, équipement achetable, phases Préparation / Bataille, affichage single-screen avec barre haute et sidebar.
- **Hors scope (MVP) :** Multijoueur / co-op, éditeur de niveaux, modes compétitifs, intégration marchandise / IAP (à traiter séparément si besoin).

---

## 2. Positionnement stratégique

### 2.1 Hybride Survivor + Tower Defense

| Aspect | Survivor | Tower Defense | Miyukini Survivor |
|--------|----------|---------------|-------------------|
| **Rôle du joueur** | Déplacement + attaques (auto/manuelles) | Souvent absent ou limité | Déplacement 8 dir. + attaque base + jet + sorts |
| **Objectif** | Survivre X min ou battre un boss | Protéger un objectif | Protéger le Château (PV) |
| **Structures** | Rarement | Tours / bâtiments | Tours construites en phase Préparation |
| **Progression run** | XP, level up, choix d'upgrades | Vagues, or, achats | XP + or ; level up → points de compétences ; or → équipement + tours |

Miyukini Survivor combine : **action du joueur** (Survivor) + **protection d'un objectif** (Château) + **construction de tours** (Tower Defense) + **phases distinctes** (Préparation / Bataille).

### 2.2 Différenciation

- **Phases claires** : Préparation (compétences, équipement, construction) vs Bataille (vagues).
- **Château au centre** : objectif unique, pas de « survie pure » sans cible à défendre.
- **Zone de construction** : disque vert autour du Château, extensible par bonus.
- **Pénalité mort joueur** : -1 PV max si le joueur meurt mais le Château survit ; revivre avec 1 PV max en moins (minimum 4 PV max).

---

## 3. Intégration avec l'écosystème COG

### 3.1 Services concernés (à préciser)

- **Miyukini Central** : hub, thème, navigation ; éventuelle exposition du jeu comme Opérateur ou parcours.
- **KindMother** : persistance des sauvegardes, progression meta (or, déblocages) si applicable.
- **MiyuClock** : temps de run, durée des phases (trace only, pas de dépendance critique à l'exécution — LOI-1).

### 3.2 Niveaux de sécurité

- **Données de jeu** (sauvegardes, scores, progression) : niveau **1 (Standard)** sauf si données personnelles identifiantes, auquel cas **2 (Sensitive)**.
- **Aucune donnée de paiement** dans le scope MVP ; si IAP ultérieurs : niveau **3 (Critical)** pour les données de paiement.

---

## 4. Architecture Opérateurs (vue d'ensemble)

**Opérateurs identifiés (à affiner) :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SurvivorGame** | Boucle de jeu (phases, vagues, état run) | Opérateur de Service |
| **SurvivorPlayer** | État joueur (PV, or, XP, compétences, équipement) | Opérateur de Service |
| **SurvivorCastle** | État Château (PV, armure) | Opérateur de Service |
| **SurvivorTowers** | Construction, état et attaques des tours | Opérateur de Service |
| **SurvivorEnemies** | Spawn, déplacement, ciblage, dégâts ennemis | Opérateur de Service |
| **SurvivorUI** | Interface (barre haute, sidebar, fenêtres Skills / construction) | Opérateur d'Interface |

**Équipe d'Opérateurs :** SurvivorService (nom à valider).  
**Contrat d'Équipe :** À définir dans les spécifications Opérateurs.

---

## 5. Décisions structurantes

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | Jeu hybride Survivor + Tower Defense avec Château comme objectif | Différenciation et alignement avec les attentes du genre |
| **DS-02** | Phases distinctes : Préparation (compétences, or, tours) et Bataille (vagues) | Clarté du loop et place pour la stratégie |
| **DS-03** | Joueur : déplacement 8 dir., attaque base auto, armes de jet au clic, sortilèges auto-ciblés | Contrôles simples + choix tactique (jet vs sort) |
| **DS-04** | Tours construites uniquement en phase Préparation, dans un disque vert autour du Château | Cohérence Tower Defense et lisibilité de la zone |
| **DS-05** | Game over si Château à 0 PV ; si joueur meurt mais Château survit : -1 PV max, minimum 4 PV max | Tension et pénalité lisible sans blocage définitif |
| **DS-06** | Affichage single-screen avec barre haute (vague, or, XP) et sidebar droite en phase Préparation | UX alignée avec la documentation Ecrans et UI |
| **DS-07** | Service COG Miyukini Survivor ; données de jeu niveau 1–2 selon identification | Gouvernance et sécurité cohérentes avec l'écosystème |

---

## 6. Documents complémentaires

- **[Miyukini Survivor - Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)** : joueur, Château, ennemis, tours, phases, or/XP, compétences.
- **[Miyukini Survivor - Ecrans et UI](Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md)** : layout, barre haute, zone de jeu, sidebar, overlays.
- **[Miyukini Survivor - Analyse Concurrence Survivor Tower Defense](Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md)** : fonctionnalités aimées par les joueurs, par ordre d'importance.

---

**Document créé le :** 2026-02-04  
**Dernière mise à jour :** 2026-02-04
