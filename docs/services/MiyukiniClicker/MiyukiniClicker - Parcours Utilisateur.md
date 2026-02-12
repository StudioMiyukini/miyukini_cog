# MiyuClicker — Parcours utilisateur (écrans d’entrée et lancement)

## Contexte

Ce document décrit le **parcours utilisateur** des écrans d’entrée de MiyuClicker : écran de chargement, écran d’accueil (landing), sélection des slots de sauvegarde et lancement de la partie. Il sert à concevoir les écrans et à aligner l’implémentation (Dioxus) avec les besoins UX.

## Portée / Scope

- **Périmètre :** Parcours depuis le démarrage de l’application jusqu’à l’entrée en jeu (fenêtre principale gestion + carte) : loading, landing, Config, Langue, choix des 3 slots de sauvegarde, lancement.
- **Hors périmètre :** Parcours en jeu (gestion, carte stratégique) ; implémentation technique détaillée des écrans.

---

## 1. Vue d’ensemble du parcours

```
[Démarrage app]
       │
       ▼
┌──────────────────┐
│  Écran de        │  Chargement des assets, initialisation
│  chargement      │  (Loading screen)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Écran d’accueil │  [Jouer]  +  Roue config (menu déroulant à droite)
│  (Landing)       │           Sauvegarder | Résolution | Langue | À propos
└────────┬─────────┘
         │
         │  Clic [Jouer]
         ▼
┌──────────────────┐
│  Sélection      │  3 slots de sauvegarde
│  des slots      │  (nouvelle partie ou charger)
└────────┬─────────┘
         │
         │  Choix d’un slot (nouveau / charger)
         ▼
┌──────────────────┐
│  Lancement      │  Chargement de la sauvegarde (si existante)
│  du jeu         │  ou initialisation nouvelle partie
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Fenêtre        │  Gestion (ressources, gens) + Carte
│  principale     │  (gameplay)
└──────────────────┘
```

---

## 2. Écran de chargement (Loading screen)

### 2.1 Rôle

- Afficher un retour visuel pendant le **chargement des assets** (textures, sprites, polices) et l’**initialisation** du jeu (Dioxus, état initial).
- Éviter un écran blanc ou figé au démarrage.

### 2.2 Contenu attendu

| Élément | Description |
|--------|-------------|
| **Indicateur de chargement** | Barre de progression et/ou animation (ex. icône tournante). Asset possible : `ui/game_ui_pack/Cute_Fantasy_UI/.../Loading_Icon.png`. |
| **Titre / logo** | MiyuClicker ou logo du jeu. |
| **Message optionnel** | Texte type « Chargement… » ou pourcentage / étape courante. |

### 2.3 Comportement

- Affiché **dès** le premier frame après `dioxus::launch`.
- Disparition **automatique** lorsque le chargement est terminé (assets prêts, état initialisé) → transition vers l’**écran d’accueil**.
- Pas d’action utilisateur requise (écran non interactif sauf affichage).

### 2.4 États possibles

| État | Description |
|------|-------------|
| **Chargement en cours** | Barre / animation active, message éventuel. |
| **Chargement terminé** | Transition immédiate vers Landing (pas d’écran intermédiaire dédié). |
| **Erreur de chargement** | À définir : message d’erreur + bouton « Réessayer » ou « Quitter » (hors scope détaillé ici). |

---

## 3. Écran d’accueil (Landing screen)

### 3.1 Rôle

- **Point d’entrée** après le chargement : le joueur choisit de **jouer** ou d’ouvrir le **menu de configuration** (roue / engrenage) pour les options.

### 3.2 Contenu attendu

| Zone | Éléments |
|------|----------|
| **Titre / branding** | Logo ou titre MiyuClicker, éventuellement sous-titre ou version. |
| **Action principale** | Bouton **[Jouer]** → sélection des slots puis jeu. |
| **Roue de configuration** | Icône **engrenage / roue** (en haut à droite ou à droite du titre). Clic → **menu déroulant à droite** (voir § 3.5). |
| **Fond / ambiance** | Image de fond ou couleur cohérente avec le thème (Cute_Fantasy_UI, couleurs du pack). |

### 3.3 Actions utilisateur (Landing)

| Action | Effet |
|--------|--------|
| **Clic [Jouer]** | Navigation vers l’**écran de sélection des slots de sauvegarde** (3 slots). |
| **Clic sur la roue de configuration** | Ouverture du **menu déroulant à droite** (Sauvegarder, Changer la résolution, Langue, À propos). |

### 3.4 Comportement

- Depuis le landing, **seul [Jouer]** mène à la sélection des slots puis au jeu.
- La **roue de configuration** est également disponible **en jeu** (fenêtre principale), pour accéder aux mêmes options sans quitter la partie.

### 3.5 Menu déroulant Configuration (à droite de la roue)

Le **menu déroulant** s’ouvre **à droite** de l’icône roue de configuration (ancrage à droite du bouton, déploiement vers la droite ou vers le bas selon l’espace). Il regroupe les options suivantes :

| Entrée du menu | Rôle | Contexte |
|----------------|------|----------|
| **Sauvegarder** | Enregistrer la partie courante sur le slot actif. | **En jeu uniquement** (partie en cours). Sur l’écran d’accueil (Landing), l’entrée peut être **désactivée** ou **masquée**. |
| **Changer la résolution** | Ouvrir un sous-menu ou une liste : choix de la résolution de la fenêtre (ex. 1280×720, 1920×1080, plein écran). | Disponible au Landing et en jeu. |
| **Langue** | Ouvrir un sous-menu ou une liste : choix de la langue de l’interface (français, anglais, etc.). | Disponible au Landing et en jeu. |
| **À propos** | Afficher une fenêtre ou un panneau : version du jeu, crédits, licences (MiyuClicker, Dioxus, packs UI). | Disponible au Landing et en jeu. |

**Comportement du menu :**

- **Ouverture** : clic sur la roue de configuration → le menu apparaît ancré à droite de l’icône (menu déroulant vers la droite, ou vers le bas si manque de place).
- **Fermeture** : clic en dehors du menu, ou sélection d’une entrée (après action : ex. changement de langue ferme le menu).
- **Sous-actions** : « Changer la résolution » et « Langue » peuvent ouvrir un **sous-menu** ou une **liste déroulante** dans le même panneau, ou une petite fenêtre modale selon le design retenu.
- **À propos** : ouvre une fenêtre modale ou un panneau dédié ; bouton « Fermer » pour revenir.

---

## 4. Sélection des 3 slots de sauvegarde

### 4.1 Rôle

- Proposer **3 emplacements de sauvegarde** (slot 1, 2, 3).
- Le joueur choisit : **nouvelle partie** sur un slot vide, ou **charger** une partie existante sur un slot déjà utilisé.

### 4.2 Contenu attendu

| Élément | Description |
|--------|-------------|
| **Titre** | Ex. « Choisir une sauvegarde » ou « Nouvelle partie / Charger ». |
| **3 slots** | Chaque slot affiche : numéro (1, 2, 3), **résumé de la partie** si une sauvegarde existe (date, heure, éventuellement aperçu : niveau, ressources, cités), ou libellé « Vide » / « Nouvelle partie ». |
| **Actions par slot** | **[Jouer]** ou **[Charger]** si sauvegarde existante ; **[Nouvelle partie]** si slot vide. |
| **Retour** | Bouton **[Retour]** vers l’écran d’accueil (Landing). |

### 4.3 Informations affichées par slot (si sauvegarde existante)

| Information | Utilité |
|-------------|---------|
| **Date / heure** | Dernière sauvegarde. |
| **Aperçu (optionnel)** | Ressources principales, nombre de cités, temps de jeu simulé — à définir selon modèle de sauvegarde. |

### 4.4 Actions utilisateur

| Action | Effet |
|--------|--------|
| **Clic [Nouvelle partie]** sur un slot vide | Création d’une **nouvelle partie** sur ce slot ; **lancement du jeu** avec état initial (ressources, carte, etc.). |
| **Clic [Charger]** ou **[Jouer]** sur un slot occupé | **Chargement** de la sauvegarde de ce slot ; **lancement du jeu** avec l’état chargé. |
| **Clic [Retour]** | Retour à l’**écran d’accueil** (Landing). |

### 4.5 Règles métier

- **Un seul slot actif par lancement** : le joueur joue une partie à la fois (celle du slot choisi).
- **Écrasement** : une « nouvelle partie » sur un slot déjà occupé peut soit **écraser** l’ancienne sauvegarde (avec confirmation), soit être **interdit** — à trancher (recommandation : confirmation avant écrasement).
- Les 3 slots sont **persistants** (stockage local ou COG selon implémentation ; voir Document Fondateur, section Sauvegarde).

---

## 5. Lancement du jeu en fonction de la sauvegarde

### 5.1 Rôle

- Après choix du slot (nouvelle partie ou charger), **charger l’état** correspondant et **afficher la fenêtre principale** de gameplay (gestion + carte).

### 5.2 Flux

| Cas | Étapes |
|-----|--------|
| **Nouvelle partie** | 1. Initialiser l’état du jeu (ressources, gens, moral, carte, cités, troupes) selon les valeurs de départ. 2. Associer cet état au slot choisi. 3. Afficher la **fenêtre principale** (écran de gestion + carte). |
| **Charger une partie** | 1. Lire la sauvegarde du slot (depuis fichier JSON / KindMother / COG). 2. Désérialiser l’état (ressources, cités, troupes, temps simulé, etc.). 3. Afficher la **fenêtre principale** avec cet état. |

### 5.3 Transition

- Optionnel : **court écran de chargement** ou overlay « Chargement de la partie… » si la désérialisation peut être longue (grosse sauvegarde, I/O lent).
- Sinon : passage **direct** de l’écran « Sélection des slots » à la **fenêtre principale**.

### 5.4 Fenêtre principale

- Contenu : **panneau Gestion** (ressources, gens, soldats, allocation) + **carte stratégique** (cités, routes, troupes). **Ergonomie du panneau Gestion** : voir [MiyuClicker - Ergonomie Ecran Gestion](MiyuClicker%20-%20Ergonomie%20Ecran%20Gestion.md) — partie gauche : 4 gros boutons (Champs, Ateliers, Château, Village) pour gains au clic ; partie droite : liste déroulante d’affectation des gens pour génération automatique (Cookie Clicker–like).
- **Roue de configuration** (même icône qu’au Landing) : en jeu, clic ouvre le **menu déroulant à droite** avec **Sauvegarder** (actif), **Changer la résolution**, **Langue**, **À propos**.
- Sauvegarde : **automatique** sur le slot courant (fréquence à définir) + **manuelle** via menu config → Sauvegarder.
- Retour au **menu principal** (Landing) : via bouton « Menu » ou « Quitter la partie » (avec sauvegarde) → retour à l’écran d’accueil ; « Jouer » à nouveau mène aux 3 slots.

---

## 6. Récapitulatif des écrans et transitions

| Écran | Entrée | Sorties possibles |
|-------|--------|--------------------|
| **Loading** | Démarrage app | → Landing (chargement OK) |
| **Landing** | Après Loading ; ou retour depuis Slots | → Slots ([Jouer]) ; **menu config** (roue → déroulant à droite : Sauvegarder, Résolution, Langue, À propos) |
| **Menu déroulant config** | Clic roue de configuration (Landing ou en jeu) | Sous-actions : Sauvegarder (en jeu), Changer la résolution, Langue, À propos ; fermeture par clic extérieur ou après action |
| **Sélection slots** | Clic [Jouer] depuis Landing | → Jeu (nouvelle partie ou charger) ; → Landing ([Retour]) |
| **Fenêtre principale (jeu)** | Nouvelle partie ou chargement depuis Slots | → Landing (Menu / Quitter partie) ; **roue config** → même menu déroulant (Sauvegarder actif) |

---

## 7. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)
- [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) — Loading_Icon, UI_Bars, UI_Buttons pour les écrans.
- [Miyukini - Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md)

---

**Document créé le :** 2026-02-01  
**Statut :** Parcours utilisateur — écrans d’entrée et lancement
