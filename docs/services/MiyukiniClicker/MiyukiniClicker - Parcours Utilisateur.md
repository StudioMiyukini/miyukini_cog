# MiyuClicker â€” Parcours utilisateur (Ã©crans dâ€™entrÃ©e et lancement)

## Contexte

Ce document dÃ©crit le **parcours utilisateur** des Ã©crans dâ€™entrÃ©e de MiyuClicker : Ã©cran de chargement, Ã©cran dâ€™accueil (landing), sÃ©lection des slots de sauvegarde et lancement de la partie. Il sert Ã  concevoir les Ã©crans et Ã  aligner lâ€™implÃ©mentation (Dioxus) avec les besoins UX.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Parcours depuis le dÃ©marrage de lâ€™application jusquâ€™Ã  lâ€™entrÃ©e en jeu (fenÃªtre principale gestion + carte) : loading, landing, Config, Langue, choix des 3 slots de sauvegarde, lancement.
- **Hors pÃ©rimÃ¨tre :** Parcours en jeu (gestion, carte stratÃ©gique) ; implÃ©mentation technique dÃ©taillÃ©e des Ã©crans.

---

## 1. Vue dâ€™ensemble du parcours

```
[DÃ©marrage app]
       â”‚
       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Ã‰cran de        â”‚  Chargement des assets, initialisation
â”‚  chargement      â”‚  (Loading screen)
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Ã‰cran dâ€™accueil â”‚  [Jouer]  +  Roue config (menu dÃ©roulant Ã  droite)
â”‚  (Landing)       â”‚           Sauvegarder | RÃ©solution | Langue | Ã€ propos
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â”‚  Clic [Jouer]
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  SÃ©lection      â”‚  3 slots de sauvegarde
â”‚  des slots      â”‚  (nouvelle partie ou charger)
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â”‚  Choix dâ€™un slot (nouveau / charger)
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Lancement      â”‚  Chargement de la sauvegarde (si existante)
â”‚  du jeu         â”‚  ou initialisation nouvelle partie
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  FenÃªtre        â”‚  Gestion (ressources, gens) + Carte
â”‚  principale     â”‚  (gameplay)
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 2. Ã‰cran de chargement (Loading screen)

### 2.1 RÃ´le

- Afficher un retour visuel pendant le **chargement des assets** (textures, sprites, polices) et lâ€™**initialisation** du jeu (Dioxus, Ã©tat initial).
- Ã‰viter un Ã©cran blanc ou figÃ© au dÃ©marrage.

### 2.2 Contenu attendu

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **Indicateur de chargement** | Barre de progression et/ou animation (ex. icÃ´ne tournante). Asset possible : `ui/game_ui_pack/Cute_Fantasy_UI/.../Loading_Icon.png`. |
| **Titre / logo** | MiyuClicker ou logo du jeu. |
| **Message optionnel** | Texte type Â« Chargementâ€¦ Â» ou pourcentage / Ã©tape courante. |

### 2.3 Comportement

- AffichÃ© **dÃ¨s** le premier frame aprÃ¨s `dioxus::launch`.
- Disparition **automatique** lorsque le chargement est terminÃ© (assets prÃªts, Ã©tat initialisÃ©) â†’ transition vers lâ€™**Ã©cran dâ€™accueil**.
- Pas dâ€™action utilisateur requise (Ã©cran non interactif sauf affichage).

### 2.4 Ã‰tats possibles

| Ã‰tat | Description |
|------|-------------|
| **Chargement en cours** | Barre / animation active, message Ã©ventuel. |
| **Chargement terminÃ©** | Transition immÃ©diate vers Landing (pas dâ€™Ã©cran intermÃ©diaire dÃ©diÃ©). |
| **Erreur de chargement** | Ã€ dÃ©finir : message dâ€™erreur + bouton Â« RÃ©essayer Â» ou Â« Quitter Â» (hors scope dÃ©taillÃ© ici). |

---

## 3. Ã‰cran dâ€™accueil (Landing screen)

### 3.1 RÃ´le

- **Point dâ€™entrÃ©e** aprÃ¨s le chargement : le joueur choisit de **jouer** ou dâ€™ouvrir le **menu de configuration** (roue / engrenage) pour les options.

### 3.2 Contenu attendu

| Zone | Ã‰lÃ©ments |
|------|----------|
| **Titre / branding** | Logo ou titre MiyuClicker, Ã©ventuellement sous-titre ou version. |
| **Action principale** | Bouton **[Jouer]** â†’ sÃ©lection des slots puis jeu. |
| **Roue de configuration** | IcÃ´ne **engrenage / roue** (en haut Ã  droite ou Ã  droite du titre). Clic â†’ **menu dÃ©roulant Ã  droite** (voir Â§ 3.5). |
| **Fond / ambiance** | Image de fond ou couleur cohÃ©rente avec le thÃ¨me (Cute_Fantasy_UI, couleurs du pack). |

### 3.3 Actions utilisateur (Landing)

| Action | Effet |
|--------|--------|
| **Clic [Jouer]** | Navigation vers lâ€™**Ã©cran de sÃ©lection des slots de sauvegarde** (3 slots). |
| **Clic sur la roue de configuration** | Ouverture du **menu dÃ©roulant Ã  droite** (Sauvegarder, Changer la rÃ©solution, Langue, Ã€ propos). |

### 3.4 Comportement

- Depuis le landing, **seul [Jouer]** mÃ¨ne Ã  la sÃ©lection des slots puis au jeu.
- La **roue de configuration** est Ã©galement disponible **en jeu** (fenÃªtre principale), pour accÃ©der aux mÃªmes options sans quitter la partie.

### 3.5 Menu dÃ©roulant Configuration (Ã  droite de la roue)

Le **menu dÃ©roulant** sâ€™ouvre **Ã  droite** de lâ€™icÃ´ne roue de configuration (ancrage Ã  droite du bouton, dÃ©ploiement vers la droite ou vers le bas selon lâ€™espace). Il regroupe les options suivantes :

| EntrÃ©e du menu | RÃ´le | Contexte |
|----------------|------|----------|
| **Sauvegarder** | Enregistrer la partie courante sur le slot actif. | **En jeu uniquement** (partie en cours). Sur lâ€™Ã©cran dâ€™accueil (Landing), lâ€™entrÃ©e peut Ãªtre **dÃ©sactivÃ©e** ou **masquÃ©e**. |
| **Changer la rÃ©solution** | Ouvrir un sous-menu ou une liste : choix de la rÃ©solution de la fenÃªtre (ex. 1280Ã—720, 1920Ã—1080, plein Ã©cran). | Disponible au Landing et en jeu. |
| **Langue** | Ouvrir un sous-menu ou une liste : choix de la langue de lâ€™interface (franÃ§ais, anglais, etc.). | Disponible au Landing et en jeu. |
| **Ã€ propos** | Afficher une fenÃªtre ou un panneau : version du jeu, crÃ©dits, licences (MiyuClicker, Dioxus, packs UI). | Disponible au Landing et en jeu. |

**Comportement du menu :**

- **Ouverture** : clic sur la roue de configuration â†’ le menu apparaÃ®t ancrÃ© Ã  droite de lâ€™icÃ´ne (menu dÃ©roulant vers la droite, ou vers le bas si manque de place).
- **Fermeture** : clic en dehors du menu, ou sÃ©lection dâ€™une entrÃ©e (aprÃ¨s action : ex. changement de langue ferme le menu).
- **Sous-actions** : Â« Changer la rÃ©solution Â» et Â« Langue Â» peuvent ouvrir un **sous-menu** ou une **liste dÃ©roulante** dans le mÃªme panneau, ou une petite fenÃªtre modale selon le design retenu.
- **Ã€ propos** : ouvre une fenÃªtre modale ou un panneau dÃ©diÃ© ; bouton Â« Fermer Â» pour revenir.

---

## 4. SÃ©lection des 3 slots de sauvegarde

### 4.1 RÃ´le

- Proposer **3 emplacements de sauvegarde** (slot 1, 2, 3).
- Le joueur choisit : **nouvelle partie** sur un slot vide, ou **charger** une partie existante sur un slot dÃ©jÃ  utilisÃ©.

### 4.2 Contenu attendu

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **Titre** | Ex. Â« Choisir une sauvegarde Â» ou Â« Nouvelle partie / Charger Â». |
| **3 slots** | Chaque slot affiche : numÃ©ro (1, 2, 3), **rÃ©sumÃ© de la partie** si une sauvegarde existe (date, heure, Ã©ventuellement aperÃ§u : niveau, ressources, citÃ©s), ou libellÃ© Â« Vide Â» / Â« Nouvelle partie Â». |
| **Actions par slot** | **[Jouer]** ou **[Charger]** si sauvegarde existante ; **[Nouvelle partie]** si slot vide. |
| **Retour** | Bouton **[Retour]** vers lâ€™Ã©cran dâ€™accueil (Landing). |

### 4.3 Informations affichÃ©es par slot (si sauvegarde existante)

| Information | UtilitÃ© |
|-------------|---------|
| **Date / heure** | DerniÃ¨re sauvegarde. |
| **AperÃ§u (optionnel)** | Ressources principales, nombre de citÃ©s, temps de jeu simulÃ© â€” Ã  dÃ©finir selon modÃ¨le de sauvegarde. |

### 4.4 Actions utilisateur

| Action | Effet |
|--------|--------|
| **Clic [Nouvelle partie]** sur un slot vide | CrÃ©ation dâ€™une **nouvelle partie** sur ce slot ; **lancement du jeu** avec Ã©tat initial (ressources, carte, etc.). |
| **Clic [Charger]** ou **[Jouer]** sur un slot occupÃ© | **Chargement** de la sauvegarde de ce slot ; **lancement du jeu** avec lâ€™Ã©tat chargÃ©. |
| **Clic [Retour]** | Retour Ã  lâ€™**Ã©cran dâ€™accueil** (Landing). |

### 4.5 RÃ¨gles mÃ©tier

- **Un seul slot actif par lancement** : le joueur joue une partie Ã  la fois (celle du slot choisi).
- **Ã‰crasement** : une Â« nouvelle partie Â» sur un slot dÃ©jÃ  occupÃ© peut soit **Ã©craser** lâ€™ancienne sauvegarde (avec confirmation), soit Ãªtre **interdit** â€” Ã  trancher (recommandation : confirmation avant Ã©crasement).
- Les 3 slots sont **persistants** (stockage local ou COG selon implÃ©mentation ; voir Document Fondateur, section Sauvegarde).

---

## 5. Lancement du jeu en fonction de la sauvegarde

### 5.1 RÃ´le

- AprÃ¨s choix du slot (nouvelle partie ou charger), **charger lâ€™Ã©tat** correspondant et **afficher la fenÃªtre principale** de gameplay (gestion + carte).

### 5.2 Flux

| Cas | Ã‰tapes |
|-----|--------|
| **Nouvelle partie** | 1. Initialiser lâ€™Ã©tat du jeu (ressources, gens, moral, carte, citÃ©s, troupes) selon les valeurs de dÃ©part. 2. Associer cet Ã©tat au slot choisi. 3. Afficher la **fenÃªtre principale** (Ã©cran de gestion + carte). |
| **Charger une partie** | 1. Lire la sauvegarde du slot (depuis fichier JSON / KindMother / COG). 2. DÃ©sÃ©rialiser lâ€™Ã©tat (ressources, citÃ©s, troupes, temps simulÃ©, etc.). 3. Afficher la **fenÃªtre principale** avec cet Ã©tat. |

### 5.3 Transition

- Optionnel : **court Ã©cran de chargement** ou overlay Â« Chargement de la partieâ€¦ Â» si la dÃ©sÃ©rialisation peut Ãªtre longue (grosse sauvegarde, I/O lent).
- Sinon : passage **direct** de lâ€™Ã©cran Â« SÃ©lection des slots Â» Ã  la **fenÃªtre principale**.

### 5.4 FenÃªtre principale

- Contenu : **panneau Gestion** (ressources, gens, soldats, allocation) + **carte stratÃ©gique** (citÃ©s, routes, troupes). **Ergonomie du panneau Gestion** : voir [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md) â€” partie gauche : 4 gros boutons (Champs, Ateliers, ChÃ¢teau, Village) pour gains au clic ; partie droite : liste dÃ©roulante dâ€™affectation des gens pour gÃ©nÃ©ration automatique (Cookie Clickerâ€“like).
- **Roue de configuration** (mÃªme icÃ´ne quâ€™au Landing) : en jeu, clic ouvre le **menu dÃ©roulant Ã  droite** avec **Sauvegarder** (actif), **Changer la rÃ©solution**, **Langue**, **Ã€ propos**.
- Sauvegarde : **automatique** sur le slot courant (frÃ©quence Ã  dÃ©finir) + **manuelle** via menu config â†’ Sauvegarder.
- Retour au **menu principal** (Landing) : via bouton Â« Menu Â» ou Â« Quitter la partie Â» (avec sauvegarde) â†’ retour Ã  lâ€™Ã©cran dâ€™accueil ; Â« Jouer Â» Ã  nouveau mÃ¨ne aux 3 slots.

---

## 6. RÃ©capitulatif des Ã©crans et transitions

| Ã‰cran | EntrÃ©e | Sorties possibles |
|-------|--------|--------------------|
| **Loading** | DÃ©marrage app | â†’ Landing (chargement OK) |
| **Landing** | AprÃ¨s Loading ; ou retour depuis Slots | â†’ Slots ([Jouer]) ; **menu config** (roue â†’ dÃ©roulant Ã  droite : Sauvegarder, RÃ©solution, Langue, Ã€ propos) |
| **Menu dÃ©roulant config** | Clic roue de configuration (Landing ou en jeu) | Sous-actions : Sauvegarder (en jeu), Changer la rÃ©solution, Langue, Ã€ propos ; fermeture par clic extÃ©rieur ou aprÃ¨s action |
| **SÃ©lection slots** | Clic [Jouer] depuis Landing | â†’ Jeu (nouvelle partie ou charger) ; â†’ Landing ([Retour]) |
| **FenÃªtre principale (jeu)** | Nouvelle partie ou chargement depuis Slots | â†’ Landing (Menu / Quitter partie) ; **roue config** â†’ mÃªme menu dÃ©roulant (Sauvegarder actif) |

---

## 7. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) â€” Loading_Icon, UI_Bars, UI_Buttons pour les Ã©crans.
- [Miyukini - Stack UI Dioxus](..//..//_index.md)

---

**Document crÃ©Ã© le :** 2026-02-01  
**Statut :** Parcours utilisateur â€” Ã©crans dâ€™entrÃ©e et lancement


