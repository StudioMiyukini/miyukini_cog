# MiyuClicker â€” MVP Ã‰crans et mÃ©caniques de jeu

## Contexte

Ce document dÃ©finit le **MVP (Minimum Viable Product)** de MiyuClicker : **Ã©crans** Ã  livrer et **mÃ©caniques de jeu** de base, ainsi que le **mapping** vers les **Toolkits**, **OpÃ©rateurs** et **Service** Miyukini Ã  utiliser ou Ã  crÃ©er.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** PÃ©rimÃ¨tre MVP â€” Ã©crans (Loading, Landing, Slots, Ma citÃ©e, Carte du monde minimal), mÃ©caniques (clic, allocation, tick, sauvegarde), Toolkits, OpÃ©rateurs, Service COG.
- **Hors pÃ©rimÃ¨tre :** Formules dâ€™Ã©quilibrage dÃ©taillÃ©es, carte stratÃ©gique complÃ¨te (conquÃªte), diplomatie, hÃ©ros (beta v1.0).

---

## 1. PÃ©rimÃ¨tre MVP â€” Ã‰crans

| Ã‰cran | RÃ´le MVP | Livrable minimal |
|-------|----------|------------------|
| **Loading** | Afficher un indicateur pendant le chargement des assets et lâ€™init. | Barre ou icÃ´ne de chargement ; disparition automatique quand prÃªt â†’ Landing. |
| **Landing** | Point dâ€™entrÃ©e aprÃ¨s chargement. | Titre / logo ; bouton **[Jouer]** ; **roue de configuration** (menu dÃ©roulant : Sauvegarder, RÃ©solution, Langue, Ã€ propos). |
| **SÃ©lection des slots** | Choisir une des 3 sauvegardes ou nouvelle partie. | 3 slots avec rÃ©sumÃ© (date/heure si sauvegarde existante) ou Â« Vide Â» ; **[Nouvelle partie]** / **[Charger]** ; **[Retour]** vers Landing. |
| **Ma citÃ©e** | Ã‰cran de gestion (Cookie Clickerâ€“like). | Barre en haut (2 lignes : Or, Gens, Soldats, Recherche \| Ma citÃ©e, Carte du monde, âš™ ; Nourriture, Bois, Pierre, Fer, Outils, Armes) ; 4 gros boutons gauche (Champs, Ateliers, ChÃ¢teau, Village) ; liste dÃ©roulante droite (affectation des gens). |
| **Carte du monde** | Carte stratÃ©gique (version MVP). | Carte avec nÅ“uds (citÃ©s) et arÃªtes (routes) ; affichage des citÃ©s (joueur / adverses) ; clic sur une citÃ© pour afficher infos ; **envoi de troupes** (nombre + citÃ© cible) et rÃ©solution combat simplifiÃ©e (optionnel MVP ou post-MVP immÃ©diat). |

**Navigation :** Depuis Ma citÃ©e ou Carte du monde, la barre en haut permet de basculer entre **Ma citÃ©e** et **Carte du monde**. Menu config (roue) : Sauvegarder, Changer rÃ©solution, Langue, Ã€ propos.

---

## 2. PÃ©rimÃ¨tre MVP â€” MÃ©caniques de jeu

### 2.1 Ressources et Ã©tat

| Ressource (ligne 1) | RÃ´le MVP |
|---------------------|----------|
| **Or** | Stock ; gain par conquÃªte de citÃ© (post-MVP) ou Ã©vÃ©nement ; dÃ©pense Ã  dÃ©finir (recrutement, construction). |
| **Gens** | Population civile ; **cap** (plafond) = f(habitations) ; consommation nourriture ; gÃ©nÃ©ration par clic Village + affectation. |
| **Soldats** | Troupes ; gÃ©nÃ©ration par clic ChÃ¢teau (coÃ»t Ã  dÃ©finir) ; utilisÃ©s pour conquÃªte. |
| **Recherche** | Points de recherche ; gÃ©nÃ©ration par affectation de gens Ã  la recherche. |

| Ressource (ligne 2) | RÃ´le MVP |
|---------------------|----------|
| **Nourriture** | Clic Champs + gens affectÃ©s aux Champs ; consommÃ©e par les gens ; si manque â†’ baisse moral / fÃ©conditÃ©. |
| **Bois, Pierre, Fer** | MatiÃ¨res premiÃ¨res ; gÃ©nÃ©ration par affectation de gens (scierie, carriÃ¨re, mine) ; bois+pierre â†’ habitations (cap gens) ; fer+bois â†’ armes ; bois/pierre/fer â†’ outils. |
| **Outils** | Clic Ateliers + affectation ; consommation matiÃ¨res premiÃ¨res. |
| **Armes** | Fer+bois ; Ã©quipement des soldats (post-MVP) ou stock pour conquÃªte. |

### 2.2 MÃ©caniques de base

| MÃ©canique | Description MVP |
|-----------|-----------------|
| **Clic manuel** | Champs â†’ + Nourriture ; Ateliers â†’ + Outils ; ChÃ¢teau â†’ + Soldats (si conditions) ; Village â†’ + Gens (si nourriture et cap). |
| **Affectation des gens** | Liste dÃ©roulante : lieux (Champs, Ateliers, Scierie, CarriÃ¨re, Mine, Recherche, etc.) ; le joueur affecte X gens Ã  chaque lieu â†’ **gÃ©nÃ©ration automatique** (par tick) de la ressource associÃ©e. |
| **Tick (simulation)** | Ã€ chaque tick (delta temps) : consommation nourriture par les gens ; production selon affectation ; mise Ã  jour moral / fÃ©conditÃ© si nourriture insuffisante ; **cap gens** = f(nombre dâ€™habitations). |
| **Habitations** | Construction avec bois + pierre â†’ augmente le **cap de gens**. |
| **Sauvegarde / chargement** | 3 slots ; sÃ©rialisation de lâ€™Ã©tat (ressources, affectations, carte, citÃ©s, troupes) ; chargement au choix du slot ; sauvegarde auto + manuelle (menu config). |
| **Carte (MVP)** | Affichage des citÃ©s et routes ; propriÃ©tÃ© (joueur / adverses) ; troupes par citÃ© ; **envoi de X soldats** vers une citÃ© adverse â†’ temps de dÃ©placement puis **rÃ©solution combat** (hasard + stats) â†’ troupes restantes ; citÃ© conquise â†’ bonus ressources (tribu). |

### 2.3 RÃ¨gles mÃ©tier minimales (MVP)

- **Moral / fÃ©conditÃ© :** Si nourriture < consommation, moral baisse, fÃ©conditÃ© baisse, population peut diminuer.
- **Cap gens :** Nombre max de gens = f(habitations). Habitations = construction (bois + pierre).
- **Combats (carte) :** RÃ©solution simplifiÃ©e : attaquant envoie X soldats ; dÃ©fenseur a Y soldats ; hasard + puissance relative â†’ vainqueur et troupes restantes.
- **CitÃ©s adverses (v0.1) :** Pas dâ€™IA ; troupes adverses Ã©voluent jusquâ€™Ã  un plafond (courbe calquÃ©e sur le joueur).

---

## 3. Toolkits â€” Utiliser ou crÃ©er

| Toolkit | RÃ´le | Utiliser / CrÃ©er | RÃ©fÃ©rence |
|---------|------|-------------------|-----------|
| **Stack UI (Dioxus)** | FenÃªtres, panels, boutons, barres, liste dÃ©roulante, rendu natif WGPU. | **Utiliser** | Stack UI officielle Miyukini ([Miyukini - Stack UI Dioxus](..//..//_index.md)). |
| **MiyuClickerSprites** | Chargement dâ€™images, spritesheets, cache textures, frame â†’ Rect (animation par frame). | **CrÃ©er** | Toolkit interne jeu ; alimentation depuis `ui/game_ui_pack`. |
| **MiyuClickerIdleSim** | Tick simulation : ressources, consommation, production, moral, cap gens, affectations. | **CrÃ©er** | Outils : `tick.apply`, `state.resources_update`, `state.allocation_apply` (logique mÃ©tier sans UI). |
| **MiyuClickerSave** | SÃ©rialisation / dÃ©sÃ©rialisation Ã©tat partie ; lecture/Ã©criture slots (sauvegarde fichier JSON). | **CrÃ©er** | Outils : `save.slot_write`, `save.slot_read`, `save.slot_list` (mÃ©tadonnÃ©es pour affichage slots). |
| **MiyuClickerCombat** | RÃ©solution combat : attaquant, dÃ©fenseur, hasard â†’ vainqueur, troupes restantes. | **CrÃ©er** | Outil : `combat.resolve` (stats + RNG). |
| **MiyuClickerCarte** | ModÃ¨le carte (nÅ“uds, arÃªtes) ; dÃ©placements en cours ; rendu (Ã©lÃ©ments SVG/canvas Dioxus) et hit-test. | **CrÃ©er** | Toolkit interne : modÃ¨le + rendu + interaction. |

**RÃ¨gle :** Les Toolkits **nâ€™exÃ©cutent que des capacitÃ©s dÃ©clarÃ©es** (Tools) ; pas de logique mÃ©tier dÃ©cisionnelle (StrongFather, etc.). Les **donnÃ©es** (Ã©tat du jeu) sont fournies dans le flux ou lues/Ã©crites via MiyuClickerSave.

---

## 4. OpÃ©rateurs â€” Utiliser ou crÃ©er

| OpÃ©rateur | RÃ´le MVP | Type | Toolkits consommÃ©s |
|-----------|----------|------|---------------------|
| **MiyuClickerUI** | Rendu de tous les Ã©crans (Loading, Landing, Slots, Ma citÃ©e, Carte du monde) ; barre ressources ; menu config ; 4 boutons ; liste dÃ©roulante. | OpÃ©rateur dâ€™Interface | Dioxus, MiyuClickerSprites, MiyuClickerCarte (rendu). |
| **MiyuClickerSim** | ExÃ©cution du **tick** : mise Ã  jour des ressources, consommation, production, moral, cap. AppelÃ© Ã  chaque frame ou Ã  intervalle fixe depuis lâ€™UI. | OpÃ©rateur de Service | MiyuClickerIdleSim. |
| **MiyuClickerSave** | Sauvegarde / chargement des 3 slots ; fourniture des mÃ©tadonnÃ©es (date, rÃ©sumÃ©) pour lâ€™Ã©cran Slots. | OpÃ©rateur de Service | MiyuClickerSave (Tools). |
| **MiyuClickerCombat** | RÃ©solution des combats (carte) : appel aprÃ¨s arrivÃ©e des troupes sur une citÃ© adverse. | OpÃ©rateur de Service / Tool | MiyuClickerCombat. |
| **MiyuClickerCarte** | Gestion du **modÃ¨le** carte (citÃ©s, routes, troupes en dÃ©placement) ; mise Ã  jour des dÃ©placements et des combats. | OpÃ©rateur de Service | MiyuClickerCarte, MiyuClickerCombat. |

**Flux typique :** Lâ€™utilisateur interagit via **MiyuClickerUI** (clic, affectation, navigation). Lâ€™UI envoie des **intentions** (ex. Â« tick Â», Â« sauvegarder slot 2 Â», Â« envoyer 10 soldats vers citÃ© B Â») ; **MiyuClickerSim**, **MiyuClickerSave**, **MiyuClickerCarte** exÃ©cutent les capacitÃ©s ; lâ€™Ã©tat est mis Ã  jour et lâ€™UI affiche le nouvel Ã©tat.

---

## 5. Service COG â€” MiyuClicker

**MiyuClicker** est un **Service** (ou **Ã‰quipe dâ€™OpÃ©rateurs**) au sens Miyukini : il agrÃ¨ge les OpÃ©rateurs ci-dessus pour dÃ©livrer le **jeu** (capacitÃ© perÃ§ue par le joueur).

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **Service** | MiyuClicker |
| **OpÃ©rateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte |
| **Contrat dâ€™Ã©quipe** | Ã€ formaliser : flux autorisÃ©s entre OpÃ©rateurs (UI â†’ Sim, UI â†’ Save, UI â†’ Carte, Carte â†’ Combat). |
| **Mandat de permission** | StrongFather Ã©met un mandat pour la session de jeu ; les OpÃ©rateurs collaborent sous ce mandat. |

**Point dâ€™entrÃ©e :** Une application **Dioxus** (desktop natif via Blitz/WGPU) qui crÃ©e lâ€™environnement COG (ou un mode dÃ©mo simplifiÃ© sans COG complet pour le MVP), instancie les OpÃ©rateurs et affiche **MiyuClickerUI**. Les Toolkits sont enregistrÃ©s (Master Butler) ou utilisÃ©s en direct selon le niveau dâ€™intÃ©gration COG retenu pour le MVP.

---

## 6. SynthÃ¨se MVP â€” Livrables

| Livrable | Contenu |
|----------|---------|
| **Ã‰crans** | Loading, Landing, Slots, Ma citÃ©e (barre + 4 boutons + liste affectation), Carte du monde (citÃ©s, routes, envoi troupes, combat simplifiÃ©). |
| **MÃ©caniques** | Clic (Champs, Ateliers, ChÃ¢teau, Village), affectation des gens (liste dÃ©roulante), tick (ressources, moral, cap), habitations (bois+pierre), outils (matiÃ¨res), armes (fer+bois), sauvegarde 3 slots, combat (rÃ©solution simple). |
| **Toolkits** | Utiliser : Dioxus. CrÃ©er : MiyuClickerSprites, MiyuClickerIdleSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte. |
| **OpÃ©rateurs** | MiyuClickerUI, MiyuClickerSim, MiyuClickerSave, MiyuClickerCombat, MiyuClickerCarte. |
| **Service** | MiyuClicker = agrÃ©gat des OpÃ©rateurs ci-dessus. |

---

## 7. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Parcours Utilisateur](MiyukiniClicker%20-%20Parcours%20Utilisateur.md)
- [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md)
- [MiyuClicker - Operateurs et Toolkits](MiyukiniClicker%20-%20Operateurs%20et%20Toolkits.md)
- [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md)
- [Miyukini - Stack UI Dioxus](..//..//_index.md)

---

**Document crÃ©Ã© le :** 2026-02-01  
**Statut :** MVP â€” Ã©crans et mÃ©caniques de jeu, mapping Toolkits / OpÃ©rateurs / Service


