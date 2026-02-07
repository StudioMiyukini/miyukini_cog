# MiyuClicker — MVP Écrans et mécaniques de jeu

## Contexte

Ce document définit le **MVP (Minimum Viable Product)** de MiyuClicker : **écrans** à livrer et **mécaniques de jeu** de base, ainsi que le **mapping** vers les **Toolkits**, **Opérateurs** et **Service** Miyukini à utiliser ou à créer.

## Portée / Scope

- **Périmètre :** Périmètre MVP — écrans (Loading, Landing, Slots, Ma citée, Carte du monde minimal), mécaniques (clic, allocation, tick, sauvegarde), Toolkits, Opérateurs, Service COG.
- **Hors périmètre :** Formules d’équilibrage détaillées, carte stratégique complète (conquête), diplomatie, héros (beta v1.0).

---

## 1. Périmètre MVP — Écrans

| Écran | Rôle MVP | Livrable minimal |
|-------|----------|------------------|
| **Loading** | Afficher un indicateur pendant le chargement des assets et l’init. | Barre ou icône de chargement ; disparition automatique quand prêt → Landing. |
| **Landing** | Point d’entrée après chargement. | Titre / logo ; bouton **[Jouer]** ; **roue de configuration** (menu déroulant : Sauvegarder, Résolution, Langue, À propos). |
| **Sélection des slots** | Choisir une des 3 sauvegardes ou nouvelle partie. | 3 slots avec résumé (date/heure si sauvegarde existante) ou « Vide » ; **[Nouvelle partie]** / **[Charger]** ; **[Retour]** vers Landing. |
| **Ma citée** | Écran de gestion (Cookie Clicker–like). | Barre en haut (2 lignes : Or, Gens, Soldats, Recherche \| Ma citée, Carte du monde, ⚙ ; Nourriture, Bois, Pierre, Fer, Outils, Armes) ; 4 gros boutons gauche (Champs, Ateliers, Château, Village) ; liste déroulante droite (affectation des gens). |
| **Carte du monde** | Carte stratégique (version MVP). | Carte avec nœuds (cités) et arêtes (routes) ; affichage des cités (joueur / adverses) ; clic sur une cité pour afficher infos ; **envoi de troupes** (nombre + cité cible) et résolution combat simplifiée (optionnel MVP ou post-MVP immédiat). |

**Navigation :** Depuis Ma citée ou Carte du monde, la barre en haut permet de basculer entre **Ma citée** et **Carte du monde**. Menu config (roue) : Sauvegarder, Changer résolution, Langue, À propos.

---

## 2. Périmètre MVP — Mécaniques de jeu

### 2.1 Ressources et état

| Ressource (ligne 1) | Rôle MVP |
|---------------------|----------|
| **Or** | Stock ; gain par conquête de cité (post-MVP) ou événement ; dépense à définir (recrutement, construction). |
| **Gens** | Population civile ; **cap** (plafond) = f(habitations) ; consommation nourriture ; génération par clic Village + affectation. |
| **Soldats** | Troupes ; génération par clic Château (coût à définir) ; utilisés pour conquête. |
| **Recherche** | Points de recherche ; génération par affectation de gens à la recherche. |

| Ressource (ligne 2) | Rôle MVP |
|---------------------|----------|
| **Nourriture** | Clic Champs + gens affectés aux Champs ; consommée par les gens ; si manque → baisse moral / fécondité. |
| **Bois, Pierre, Fer** | Matières premières ; génération par affectation de gens (scierie, carrière, mine) ; bois+pierre → habitations (cap gens) ; fer+bois → armes ; bois/pierre/fer → outils. |
| **Outils** | Clic Ateliers + affectation ; consommation matières premières. |
| **Armes** | Fer+bois ; équipement des soldats (post-MVP) ou stock pour conquête. |

### 2.2 Mécaniques de base

| Mécanique | Description MVP |
|-----------|-----------------|
| **Clic manuel** | Champs → + Nourriture ; Ateliers → + Outils ; Château → + Soldats (si conditions) ; Village → + Gens (si nourriture et cap). |
| **Affectation des gens** | Liste déroulante : lieux (Champs, Ateliers, Scierie, Carrière, Mine, Recherche, etc.) ; le joueur affecte X gens à chaque lieu → **génération automatique** (par tick) de la ressource associée. |
| **Tick (simulation)** | À chaque tick (delta temps) : consommation nourriture par les gens ; production selon affectation ; mise à jour moral / fécondité si nourriture insuffisante ; **cap gens** = f(nombre d’habitations). |
| **Habitations** | Construction avec bois + pierre → augmente le **cap de gens**. |
| **Sauvegarde / chargement** | 3 slots ; sérialisation de l’état (ressources, affectations, carte, cités, troupes) ; chargement au choix du slot ; sauvegarde auto + manuelle (menu config). |
| **Carte (MVP)** | Affichage des cités et routes ; propriété (joueur / adverses) ; troupes par cité ; **envoi de X soldats** vers une cité adverse → temps de déplacement puis **résolution combat** (hasard + stats) → troupes restantes ; cité conquise → bonus ressources (tribu). |

### 2.3 Règles métier minimales (MVP)

- **Moral / fécondité :** Si nourriture < consommation, moral baisse, fécondité baisse, population peut diminuer.
- **Cap gens :** Nombre max de gens = f(habitations). Habitations = construction (bois + pierre).
- **Combats (carte) :** Résolution simplifiée : attaquant envoie X soldats ; défenseur a Y soldats ; hasard + puissance relative → vainqueur et troupes restantes.
- **Cités adverses (v0.1) :** Pas d’IA ; troupes adverses évoluent jusqu’à un plafond (courbe calquée sur le joueur).

---

## 3. Toolkits — Utiliser ou créer

| Toolkit | Rôle | Utiliser / Créer | Référence |
|---------|------|-------------------|-----------|
| **Stack UI (egui/eframe)** | Fenêtres, panels, boutons, barres, liste déroulante, rendu 2D. | **Utiliser** | Stack UI officielle Miyukini ([Miyukini - Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)). |
| **MiyuClickerSprites** | Chargement d’images, spritesheets, cache textures, frame → Rect (animation par frame). | **Créer** | Toolkit interne jeu ; alimentation depuis `ui/game_ui_pack`. |
| **MiyuClickerIdleSim** | Tick simulation : ressources, consommation, production, moral, cap gens, affectations. | **Créer** | Outils : `tick.apply`, `state.resources_update`, `state.allocation_apply` (logique métier sans UI). |
| **MiyuClickerSave** | Sérialisation / désérialisation état partie ; lecture/écriture slots (fichier ou eframe persistence). | **Créer** | Outils : `save.slot_write`, `save.slot_read`, `save.slot_list` (métadonnées pour affichage slots). |
| **MiyuClickerCombat** | Résolution combat : attaquant, défenseur, hasard → vainqueur, troupes restantes. | **Créer** | Outil : `combat.resolve` (stats + RNG). |
| **MiyuClickerCarte** | Modèle carte (nœuds, arêtes) ; déplacements en cours ; rendu (egui painter) et hit-test. | **Créer** | Toolkit interne : modèle + rendu + interaction. |

**Règle :** Les Toolkits **n’exécutent que des capacités déclarées** (Tools) ; pas de logique métier décisionnelle (StrongFather, etc.). Les **données** (état du jeu) sont fournies dans le flux ou lues/écrites via MiyuClickerSave.

---

## 4. Opérateurs — Utiliser ou créer

| Opérateur | Rôle MVP | Type | Toolkits consommés |
|-----------|----------|------|---------------------|
| **MiyuClickerUI** | Rendu de tous les écrans (Loading, Landing, Slots, Ma citée, Carte du monde) ; barre ressources ; menu config ; 4 boutons ; liste déroulante. | Opérateur d’Interface | egui/eframe, MiyuClickerSprites, MiyuClickerCarte (rendu). |
| **MiyuClickerSim** | Exécution du **tick** : mise à jour des ressources, consommation, production, moral, cap. Appelé à chaque frame ou à intervalle fixe depuis l’UI. | Opérateur de Service | MiyuClickerIdleSim. |
| **MiyuClickerSave** | Sauvegarde / chargement des 3 slots ; fourniture des métadonnées (date, résumé) pour l’écran Slots. | Opérateur de Service | MiyuClickerSave (Tools). |
| **MiyuClickerCombat** | Résolution des combats (carte) : appel après arrivée des troupes sur une cité adverse. | Opérateur de Service / Tool | MiyuClickerCombat. |
| **MiyuClickerCarte** | Gestion du **modèle** carte (cités, routes, troupes en déplacement) ; mise à jour des déplacements et des combats. | Opérateur de Service | MiyuClickerCarte, MiyuClickerCombat. |

**Flux typique :** L’utilisateur interagit via **MiyuClickerUI** (clic, affectation, navigation). L’UI envoie des **intentions** (ex. « tick », « sauvegarder slot 2 », « envoyer 10 soldats vers cité B ») ; **MiyuClickerSim**, **MiyuClickerSave**, **MiyuClickerCarte** exécutent les capacités ; l’état est mis à jour et l’UI affiche le nouvel état.

---

## 5. Service COG — MiyuClicker

**MiyuClicker** est un **Service** (ou **Équipe d’Opérateurs**) au sens Miyukini : il agrège les Opérateurs ci-dessus pour délivrer le **jeu** (capacité perçue par le joueur).

| Élément | Description |
|--------|-------------|
| **Service** | MiyuClicker |
| **Opérateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte |
| **Contrat d’équipe** | À formaliser : flux autorisés entre Opérateurs (UI → Sim, UI → Save, UI → Carte, Carte → Combat). |
| **Mandat de permission** | StrongFather émet un mandat pour la session de jeu ; les Opérateurs collaborent sous ce mandat. |

**Point d’entrée :** Une application **eframe** (desktop ou WASM) qui crée l’environnement COG (ou un mode démo simplifié sans COG complet pour le MVP), instancie les Opérateurs et affiche **MiyuClickerUI**. Les Toolkits sont enregistrés (Master Butler) ou utilisés en direct selon le niveau d’intégration COG retenu pour le MVP.

---

## 6. Synthèse MVP — Livrables

| Livrable | Contenu |
|----------|---------|
| **Écrans** | Loading, Landing, Slots, Ma citée (barre + 4 boutons + liste affectation), Carte du monde (cités, routes, envoi troupes, combat simplifié). |
| **Mécaniques** | Clic (Champs, Ateliers, Château, Village), affectation des gens (liste déroulante), tick (ressources, moral, cap), habitations (bois+pierre), outils (matières), armes (fer+bois), sauvegarde 3 slots, combat (résolution simple). |
| **Toolkits** | Utiliser : egui/eframe. Créer : MiyuClickerSprites, MiyuClickerIdleSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte. |
| **Opérateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte. |
| **Service** | MiyuClicker = agrégat des Opérateurs ci-dessus. |

---

## 7. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Parcours Utilisateur](MiyuClicker%20-%20Parcours%20Utilisateur.md)
- [MiyuClicker - Ergonomie Ecran Gestion](MiyuClicker%20-%20Ergonomie%20Ecran%20Gestion.md)
- [MiyuClicker - Operateurs et Toolkits](MiyuClicker%20-%20Operateurs%20et%20Toolkits.md)
- [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md)
- [Miyukini - Stack UI egui eframe](../../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)

---

**Document créé le :** 2026-02-01  
**Statut :** MVP — écrans et mécaniques de jeu, mapping Toolkits / Opérateurs / Service
