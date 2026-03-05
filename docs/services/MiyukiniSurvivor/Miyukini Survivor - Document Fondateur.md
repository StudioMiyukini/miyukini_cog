# Miyukini Survivor â€” Document Fondateur

## Contexte

**Miyukini Survivor** est le **service Miyukini du domaine jeu Survivor / Tower Defense** au sein de l'Ã©cosystÃ¨me COG. Il propose une expÃ©rience de jeu hybride : le joueur se dÃ©place sur l'Ã©cran en 8 directions, attaque (base, armes de jet, sortilÃ¨ges), protÃ¨ge un objectif central (**le ChÃ¢teau**), construit des **tours** en phase **PrÃ©paration**, et affronte des **vagues d'ennemis** en phase **Bataille**.

Ce document est le **document fondateur** du service : il en fixe la raison d'Ãªtre, le scope, les concepts de base du jeu, le positionnement (Survivor + Tower Defense) et les dÃ©cisions structurantes. Il s'adresse aux Ã©quipes produit, technique et parties prenantes.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** DÃ©finition du service Miyukini Survivor â€” fondation du scope, concepts du jeu, gameplay, phases, affichage ; positionnement stratÃ©gique.
- **Hors pÃ©rimÃ¨tre :** SpÃ©cifications techniques dÃ©taillÃ©es (API, moteur, implÃ©mentation des crates), design visuel et sonore finaux.
- **RÃ©fÃ©rences :** [Glossaire Miyukini](..//..//miyukini-webway-system//reference//_index.md), [Analyse Concurrence Survivor Tower Defense](./Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md).

---

## 1. Fondation du scope et des concepts du jeu

### 1.1 Raison d'Ãªtre

**Miyukini Survivor** a pour objectif de :

- **Proposer un jeu hybride Survivor + Tower Defense** au sein de l'Ã©cosystÃ¨me Miyukini : le joueur est actif (dÃ©placement, attaques manuelles et auto), protÃ¨ge un objectif (le ChÃ¢teau), et construit des tours en phase de prÃ©paration.
- **Respecter les mÃ©caniques aimÃ©es par les joueurs** du genre (progression en run, Ã©volution/builds, meta-progression, variÃ©tÃ©, contrÃ´les simples, challenge) â€” voir [Analyse Concurrence](./Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md).
- **Exposer un service COG** : logique mÃ©tier, progression, sauvegardes et intÃ©grations (Miyukini Central, Ã©ventuels OpÃ©rateurs) selon les rÃ¨gles de gouvernance Miyukini.

### 1.2 Concepts de base

| Concept | Description |
|--------|-------------|
| **Zone de jeu** | Un seul Ã©cran ; le joueur, le ChÃ¢teau, les ennemis et les tours Ã©voluent dans cet espace. |
| **Le ChÃ¢teau** | Objectif central des ennemis ; il possÃ¨de des PV et de l'armure, pas d'attaques. Ã€ 0 PV = game over. |
| **Phase PrÃ©paration** | Le joueur dÃ©pense or, points de compÃ©tences, et construit des tours dans la zone autorisÃ©e (disque vert autour du ChÃ¢teau). |
| **Phase Bataille** | Vagues d'ennemis depuis les bords vers le ChÃ¢teau ; le joueur et les tours les combattent. |
| **Joueur** | Avatar dÃ©plaÃ§able en 8 directions ; attaque de base (auto), armes de jet (clic), sortilÃ¨ges (projectiles vers l'ennemi le plus proche). |
| **Tours** | BÃ¢timents construits en phase PrÃ©paration ; ils attaquent les ennemis Ã  portÃ©e pendant la Bataille. |
| **Ennemis** | Se dirigent vers le ChÃ¢teau ; prioritÃ© de cible : Joueur > BÃ¢timent > ChÃ¢teau. Donnent or et XP Ã  la mort. |

### 1.3 PÃ©rimÃ¨tre fonctionnel (scope)

- **In scope :** Un joueur, un ChÃ¢teau, vagues d'ennemis, tours construites en phase PrÃ©paration, attaque de base + armes de jet + sortilÃ¨ges, or et XP, arbre de compÃ©tences, Ã©quipement achetable, phases PrÃ©paration / Bataille, affichage single-screen avec barre haute et sidebar.
- **Hors scope (MVP) :** Multijoueur / co-op, Ã©diteur de niveaux, modes compÃ©titifs, intÃ©gration marchandise / IAP (Ã  traiter sÃ©parÃ©ment si besoin).

---

## 2. Positionnement stratÃ©gique

### 2.1 Hybride Survivor + Tower Defense

| Aspect | Survivor | Tower Defense | Miyukini Survivor |
|--------|----------|---------------|-------------------|
| **RÃ´le du joueur** | DÃ©placement + attaques (auto/manuelles) | Souvent absent ou limitÃ© | DÃ©placement 8 dir. + attaque base + jet + sorts |
| **Objectif** | Survivre X min ou battre un boss | ProtÃ©ger un objectif | ProtÃ©ger le ChÃ¢teau (PV) |
| **Structures** | Rarement | Tours / bÃ¢timents | Tours construites en phase PrÃ©paration |
| **Progression run** | XP, level up, choix d'upgrades | Vagues, or, achats | XP + or ; level up â†’ points de compÃ©tences ; or â†’ Ã©quipement + tours |

Miyukini Survivor combine : **action du joueur** (Survivor) + **protection d'un objectif** (ChÃ¢teau) + **construction de tours** (Tower Defense) + **phases distinctes** (PrÃ©paration / Bataille).

### 2.2 DiffÃ©renciation

- **Phases claires** : PrÃ©paration (compÃ©tences, Ã©quipement, construction) vs Bataille (vagues).
- **ChÃ¢teau au centre** : objectif unique, pas de Â« survie pure Â» sans cible Ã  dÃ©fendre.
- **Zone de construction** : disque vert autour du ChÃ¢teau, extensible par bonus.
- **PÃ©nalitÃ© mort joueur** : -1 PV max si le joueur meurt mais le ChÃ¢teau survit ; revivre avec 1 PV max en moins (minimum 4 PV max).

---

## 3. IntÃ©gration avec l'Ã©cosystÃ¨me COG

### 3.1 Services concernÃ©s (Ã  prÃ©ciser)

- **Miyukini Central** : hub, thÃ¨me, navigation ; Ã©ventuelle exposition du jeu comme OpÃ©rateur ou parcours.
- **KindMother** : persistance des sauvegardes, progression meta (or, dÃ©blocages) si applicable.
- **MiyuClock** : temps de run, durÃ©e des phases (trace only, pas de dÃ©pendance critique Ã  l'exÃ©cution â€” LOI-1).

### 3.2 Niveaux de sÃ©curitÃ©

- **DonnÃ©es de jeu** (sauvegardes, scores, progression) : niveau **1 (Standard)** sauf si donnÃ©es personnelles identifiantes, auquel cas **2 (Sensitive)**.
- **Aucune donnÃ©e de paiement** dans le scope MVP ; si IAP ultÃ©rieurs : niveau **3 (Critical)** pour les donnÃ©es de paiement.

---

## 4. Architecture OpÃ©rateurs (vue d'ensemble)

**OpÃ©rateurs identifiÃ©s (Ã  affiner) :**

| OpÃ©rateur | RÃ´le | Type |
|-----------|------|------|
| **SurvivorGame** | Boucle de jeu (phases, vagues, Ã©tat run) | OpÃ©rateur de Service |
| **SurvivorPlayer** | Ã‰tat joueur (PV, or, XP, compÃ©tences, Ã©quipement) | OpÃ©rateur de Service |
| **SurvivorCastle** | Ã‰tat ChÃ¢teau (PV, armure) | OpÃ©rateur de Service |
| **SurvivorTowers** | Construction, Ã©tat et attaques des tours | OpÃ©rateur de Service |
| **SurvivorEnemies** | Spawn, dÃ©placement, ciblage, dÃ©gÃ¢ts ennemis | OpÃ©rateur de Service |
| **SurvivorUI** | Interface (barre haute, sidebar, fenÃªtres Skills / construction) | OpÃ©rateur d'Interface |

**Ã‰quipe d'OpÃ©rateurs :** SurvivorService (nom Ã  valider).  
**Contrat d'Ã‰quipe :** Ã€ dÃ©finir dans les spÃ©cifications OpÃ©rateurs.

---

## 5. DÃ©cisions structurantes

| Id | DÃ©cision | Justification |
|----|----------|---------------|
| **DS-01** | Jeu hybride Survivor + Tower Defense avec ChÃ¢teau comme objectif | DiffÃ©renciation et alignement avec les attentes du genre |
| **DS-02** | Phases distinctes : PrÃ©paration (compÃ©tences, or, tours) et Bataille (vagues) | ClartÃ© du loop et place pour la stratÃ©gie |
| **DS-03** | Joueur : dÃ©placement 8 dir., attaque base auto, armes de jet au clic, sortilÃ¨ges auto-ciblÃ©s | ContrÃ´les simples + choix tactique (jet vs sort) |
| **DS-04** | Tours construites uniquement en phase PrÃ©paration, dans un disque vert autour du ChÃ¢teau | CohÃ©rence Tower Defense et lisibilitÃ© de la zone |
| **DS-05** | Game over si ChÃ¢teau Ã  0 PV ; si joueur meurt mais ChÃ¢teau survit : -1 PV max, minimum 4 PV max | Tension et pÃ©nalitÃ© lisible sans blocage dÃ©finitif |
| **DS-06** | Affichage single-screen avec barre haute (vague, or, XP) et sidebar droite en phase PrÃ©paration | UX alignÃ©e avec la documentation Ecrans et UI |
| **DS-07** | Service COG Miyukini Survivor ; donnÃ©es de jeu niveau 1â€“2 selon identification | Gouvernance et sÃ©curitÃ© cohÃ©rentes avec l'Ã©cosystÃ¨me |

---

## 6. Documents complÃ©mentaires

- **[Miyukini Survivor - Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)** : joueur, ChÃ¢teau, ennemis, tours, phases, or/XP, compÃ©tences.
- **[Miyukini Survivor - Ecrans et UI](Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md)** : layout, barre haute, zone de jeu, sidebar, overlays.
- **[Miyukini Survivor - Analyse Concurrence Survivor Tower Defense](Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md)** : fonctionnalitÃ©s aimÃ©es par les joueurs, par ordre d'importance.

---

**Document crÃ©Ã© le :** 2026-02-04  
**DerniÃ¨re mise Ã  jour :** 2026-02-04

