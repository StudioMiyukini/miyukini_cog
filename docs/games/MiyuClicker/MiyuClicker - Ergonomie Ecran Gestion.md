# MiyuClicker — Ergonomie de l’écran de gestion (Cookie Clicker–like)

## Contexte

Ce document décrit l’**ergonomie de l’écran de gestion** de MiyuClicker : disposition en deux moitiés (gauche / droite), **quatre gros boutons** à gauche (Champs, Ateliers, Château, Village) pour les gains au clic, et **liste déroulante d’affectation des gens** à droite pour la génération automatique de ressources, à la manière d’un **Cookie Clicker**.

## Portée / Scope

- **Périmètre :** Barre en haut (ressources + navigation écrans) ; layout de l’écran de gestion (partie gauche : 4 boutons ; partie droite : liste d’affectation) ; rôle de chaque zone ; clic manuel vs génération automatique par affectation de gens.
- **Hors périmètre :** Formules de gain exactes ; implémentation technique détaillée.

---

## 1. Barre en haut de l’écran de jeu

Une **barre horizontale** en **haut** de l’écran de jeu récapitule les **ressources disponibles** et donne accès aux **différents écrans** (gestion du village, grandes stratégies).

### 1.1 Première ligne — Ressources principales

La **première ligne** affiche les ressources principales, **dans l’ordre suivant** (de gauche à droite) :

| Ordre | Ressource | Description |
|-------|-----------|-------------|
| 1 | **Or** | Stock d’or. |
| 2 | **Gens** | Population civile disponible (ou total). |
| 3 | **Soldats** | Nombre de soldats / troupes. |
| 4 | **Recherche** | Points de recherche accumulés. |

Chaque ressource est affichée sous forme **icône + valeur** (ou libellé court + valeur), mise à jour en temps réel.

### 1.2 Deuxième ligne — Ressources intermédiaires et secondaires

La **deuxième ligne** de la barre affiche les ressources dans l’**ordre suivant** (de gauche à droite) :

| Ordre | Ressource | Description |
|-------|-----------|-------------|
| 1 | **Nourriture** | Stock de nourriture. |
| 2 | **Bois** | Stock de bois. |
| 3 | **Pierre** | Stock de pierre. |
| 4 | **Fer** | Stock de fer. |
| 5 | **Outils** | Stock d’outils. |
| 6 | **Armes** | Stock d’armes. |

Ces ressources servent à **construire** ou **fabriquer** d’autres biens. Règles de conversion (logique de jeu) :

| Utilisation | Ressources consommées | Effet |
|-------------|------------------------|--------|
| **Habitations** | **Bois + Pierre** | Permet de construire plus d’**habitations** → **augmente le cap** (plafond) de **gens disponibles**. |
| **Armes** | **Fer + Bois** | Permet de **fabriquer des armes** (augmente le stock d’armes). |
| **Outils** | **Bois** OU **Pierre** OU **Fer** | Permet de **fabriquer des outils** ; au moins une de ces trois matières est consommée. |

En résumé :
- **Bois et pierre** → plus d’habitations → **cap de gens** plus élevé.
- **Fer et bois** → **armes**.
- **Bois, ou pierre, ou fer** → **outils**.

Les formules exactes (quantités par construction, caps, etc.) relèvent des spécifications de jeu ; l’ergonomie impose d’afficher sur la **deuxième ligne** : **nourriture, bois, pierre, fer, outils, armes** — distincte de la première ligne (or, gens, soldats, recherche).

### 1.3 Navigation vers les écrans

La barre en haut inclut des **entrées de navigation** vers les **écrans principaux** :

| Libellé affiché | Écran | Rôle |
|-----------------|-------|------|
| **Ma citée** | Gestion du village | Écran de gestion (4 boutons gauche + liste d’affectation droite) — celui décrit dans ce document. |
| **Carte du monde** | Grandes stratégies | Carte stratégique (cités-États, routes, conquête, envoi de troupes). |

L’ordre d’affichage dans la barre : **ressources** (ligne 1 puis ligne 2), puis **Ma citée**, **Carte du monde**, et la **roue de configuration** (engrenage) à droite. Le joueur clique sur « Ma citée » ou « Carte du monde » pour afficher l’écran correspondant.

### 1.4 Disposition type de la barre (deux lignes)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Or │ Gens │ Soldats │ Recherche │ Ma citée │ Carte du monde │ [⚙]         │
├─────────────────────────────────────────────────────────────────────────────┤
│  Nourriture │ Bois │ Pierre │ Fer │ Outils │ Armes │                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

- **Ligne 1** : les 4 ressources principales (**or**, gens, soldats, recherche) ; à droite : **Ma citée**, **Carte du monde**, **roue de configuration** (engrenage).
- **Ligne 2** : **nourriture**, **bois**, **pierre**, **fer**, **outils**, **armes**.

---

## 2. Vue d’ensemble du layout (écran de jeu)

Sous la barre en haut, l’écran affiché dépend du choix de navigation : **gestion du village** (layout ci-dessous) ou **grandes stratégies** (carte).

### 2.1 Layout « Gestion du village »

L’écran **Gestion du village** est divisé en **deux moitiés** :

| Zone | Position | Rôle |
|------|-----------|------|
| **Partie gauche** | Moitié gauche de l’écran | **Quatre gros boutons** : Champs, Ateliers, Château, Village. **Clic manuel** → gains immédiats (nourriture, outils, soldats, gens). |
| **Partie droite** | Moitié droite de l’écran | **Liste déroulante** des **lieux d’affectation** : le joueur y place des **gens** pour que des **ressources soient générées automatiquement** (idle). |

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Or │ Gens │ Soldats │ Recherche │ Ma citée │ Carte du monde │ [⚙]         │
├─────────────────────────────────────────────────────────────────────────────┤
│  Nourriture │ Bois │ Pierre │ Fer │ Outils │ Armes │                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                              │                                               │
│   PARTIE GAUCHE              │   PARTIE DROITE                               │
│   (4 gros boutons)           │   (liste déroulante)                          │
│                              │                                               │
│   ┌─────────────┐            │   Lieux où placer des gens :                  │
│   │   CHAMPS    │  → Nourriture   ▼ [Liste déroulante]                       │
│   └─────────────┘            │   • Champs (nourriture auto)                   │
│                              │   • Ateliers (outils auto)                     │
│   ┌─────────────┐            │   • …                                         │
│   │  ATELIERS   │  → Outils   │   Affectation : X gens ici,                  │
│   └─────────────┘            │   Y gens là → génération auto                  │
│                              │                                               │
│   ┌─────────────┐            │                                               │
│   │   CHÂTEAU   │  → Soldats  │                                               │
│   └─────────────┘            │                                               │
│                              │                                               │
│   ┌─────────────┐            │                                               │
│   │   VILLAGE   │  → Gens     │                                               │
│   └─────────────┘            │                                               │
│                              │                                               │
└──────────────────────────────┴───────────────────────────────────────────────┘
```

---

## 3. Partie gauche — Quatre gros boutons

Les **quatre gros boutons** occupent la **moitié gauche** de l’écran. Chaque bouton est **grand, visible, cliquable** ; un clic déclenche un **gain immédiat** (ressource ou unité), à la manière du cookie dans Cookie Clicker.

### 3.1 Les Champs

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Champs** — représente les terres agricoles. |
| **Action au clic** | Le joueur **gagne des points en nourriture** (gain immédiat à chaque clic). |
| **Rôle** | Boucle **clicker** : cliquer pour accumuler de la nourriture manuellement. |
| **Feedback** | Retour visuel au clic (animation, +N nourriture, son optionnel). |

### 3.2 Les Ateliers

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Ateliers** — représente les ateliers de production. |
| **Action au clic** | Le joueur **gagne des points en outils** (gain immédiat à chaque clic). |
| **Rôle** | Boucle **clicker** : cliquer pour accumuler des outils manuellement. |
| **Feedback** | Retour visuel au clic (animation, +N outils, son optionnel). |

### 3.3 Le Château

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Château** — représente le recrutement militaire. |
| **Action au clic** | Le joueur **crée des soldats** (un ou plusieurs par clic, selon règles de coût à définir). |
| **Rôle** | Boucle **clicker** : cliquer pour recruter des soldats (sous réserve de ressources / conditions). |
| **Feedback** | Retour visuel au clic ; si coût insuffisant, message ou feedback négatif. |

### 2.4 Le Village

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Village** — représente la population civile. |
| **Action au clic** | Le joueur **crée des gens** (population civile). |
| **Rôle** | Boucle **clicker** : cliquer pour faire croître la population (sous réserve de nourriture / conditions). |
| **Feedback** | Retour visuel au clic ; si conditions non remplies (ex. nourriture), message ou feedback négatif. |

### 3.5 Synthèse partie gauche

| Bouton | Effet au clic (immédiat) |
|--------|---------------------------|
| **Champs** | + Nourriture |
| **Ateliers** | + Outils |
| **Château** | + Soldats (si conditions remplies) |
| **Village** | + Gens (si conditions remplies) |

Les **coûts** (nourriture pour créer des gens, ressources pour soldats, etc.) et les **montants par clic** relèvent des spécifications de jeu (formules, équilibrage) ; l’ergonomie impose seulement que chaque gros bouton ait un **effet clair et immédiat** au clic.

---

## 4. Partie droite — Liste déroulante d’affectation des gens

La **moitié droite** de l’écran affiche une **liste déroulante** (ou liste extensible) des **endroits où le joueur peut placer des gens** pour que des **ressources soient générées automatiquement**.

### 4.1 Principe (style Cookie Clicker)

- Le joueur dispose d’un **stock de gens** (population disponible).
- Il **affecte** une partie de ces gens à différents **lieux** (Champs, Ateliers, etc.).
- Les gens affectés **produisent des ressources en continu** (génération passive / idle), sans avoir à cliquer.
- La **liste déroulante** recense ces lieux et permet de **choisir combien de gens** envoyer dans chaque lieu.

### 3.2 Contenu de la liste déroulante

La liste propose les **lieux d’affectation** possibles, par exemple :

| Lieu d’affectation | Ressource générée automatiquement |
|--------------------|-------------------------------------|
| **Champs** | Nourriture (par tick / par seconde) |
| **Ateliers** | Outils |
| **Recherche** (optionnel) | Points de recherche |
| **Matières premières** (optionnel) | Matières premières |
| **Armes** (optionnel) | Armes |
| … | Selon règles de jeu (Document Fondateur) |

Chaque ligne (ou entrée déroulante) permet typiquement :
- d’**afficher le lieu** et la **ressource générée** ;
- de **régler le nombre de gens affectés** (slider, champs +/- ou liste de choix).

### 4.3 Comportement attendu

| Action | Effet |
|--------|--------|
| **Ouvrir la liste** | Afficher tous les lieux où des gens peuvent être affectés. |
| **Choisir un lieu** | Afficher le détail : nom, ressource générée, nombre de gens actuellement affectés, capacité ou limite éventuelle. |
| **Affecter X gens à un lieu** | Réduire le nombre de **gens disponibles** de X et augmenter la **génération automatique** de la ressource correspondante. |
| **Retirer des gens d’un lieu** | Les gens redeviennent **disponibles** ; la génération automatique de ce lieu diminue. |

### 4.4 Cohérence avec la partie gauche

- **Partie gauche (clic)** : gains **immédiats** au clic (nourriture, outils, soldats, gens).
- **Partie droite (liste)** : **affectation de gens** → génération **automatique** dans le temps (idle).
- Les **Champs** et **Ateliers** apparaissent à la fois :
  - comme **boutons à cliquer** (gauche) pour un gain manuel ;
  - comme **lieux d’affectation** (droite) pour une production automatique par les gens.
- Le **Château** (soldats) et le **Village** (gens) sont surtout des **boutons de création** à gauche ; l’affectation à droite concerne les **lieux de production** (Champs, Ateliers, Recherche, etc.).

---

## 5. Résumé ergonomique

| Zone | Élément | Rôle |
|------|---------|------|
| **Barre haut (ligne 1)** | **Ressources + navigation** | **Or**, Gens, Soldats, Recherche ; puis **Ma citée**, **Carte du monde**, roue de configuration. |
| **Barre haut (ligne 2)** | **Ressources secondaires** | **Nourriture**, Bois, Pierre, Fer, Outils, Armes — matières premières (bois+pierre → habitations ; fer+bois → armes ; bois/pierre/fer → outils) + stocks nourriture, outils, armes. |
| **Gauche** | **Champs** (gros bouton) | Clic → + Nourriture |
| **Gauche** | **Ateliers** (gros bouton) | Clic → + Outils |
| **Gauche** | **Château** (gros bouton) | Clic → + Soldats |
| **Gauche** | **Village** (gros bouton) | Clic → + Gens |
| **Droite** | **Liste déroulante** | Lieux où placer des gens → **génération automatique** de ressources (nourriture, outils, etc.) à la manière d’un Cookie Clicker. |

L’écran de jeu comporte donc **une barre en haut** (ressources + navigation) et, sur l’écran **Gestion du village**, **deux piliers** : **clic manuel** (quatre gros boutons) et **idle par affectation** (liste droite).

---

## 6. Références

- [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md) — Gameplay gestion (ressources, gens, moral).
- [MiyuClicker - Parcours Utilisateur](MiyuClicker%20-%20Parcours%20Utilisateur.md) — Fenêtre principale.
- [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) — Assets UI (boutons, listes).

---

**Document créé le :** 2026-02-01  
**Statut :** Ergonomie écran de gestion — 4 boutons gauche, liste affectation droite (Cookie Clicker–like)
