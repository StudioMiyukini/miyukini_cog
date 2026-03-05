# MiyuClicker â€” Ergonomie de lâ€™Ã©cran de gestion (Cookie Clickerâ€“like)

## Contexte

Ce document dÃ©crit lâ€™**ergonomie de lâ€™Ã©cran de gestion** de MiyuClicker : disposition en deux moitiÃ©s (gauche / droite), **quatre gros boutons** Ã  gauche (Champs, Ateliers, ChÃ¢teau, Village) pour les gains au clic, et **liste dÃ©roulante dâ€™affectation des gens** Ã  droite pour la gÃ©nÃ©ration automatique de ressources, Ã  la maniÃ¨re dâ€™un **Cookie Clicker**.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Barre en haut (ressources + navigation Ã©crans) ; layout de lâ€™Ã©cran de gestion (partie gauche : 4 boutons ; partie droite : liste dâ€™affectation) ; rÃ´le de chaque zone ; clic manuel vs gÃ©nÃ©ration automatique par affectation de gens.
- **Hors pÃ©rimÃ¨tre :** Formules de gain exactes ; implÃ©mentation technique dÃ©taillÃ©e.

---

## 1. Barre en haut de lâ€™Ã©cran de jeu

Une **barre horizontale** en **haut** de lâ€™Ã©cran de jeu rÃ©capitule les **ressources disponibles** et donne accÃ¨s aux **diffÃ©rents Ã©crans** (gestion du village, grandes stratÃ©gies).

### 1.1 PremiÃ¨re ligne â€” Ressources principales

La **premiÃ¨re ligne** affiche les ressources principales, **dans lâ€™ordre suivant** (de gauche Ã  droite) :

| Ordre | Ressource | Description |
|-------|-----------|-------------|
| 1 | **Or** | Stock dâ€™or. |
| 2 | **Gens** | Population civile disponible (ou total). |
| 3 | **Soldats** | Nombre de soldats / troupes. |
| 4 | **Recherche** | Points de recherche accumulÃ©s. |

Chaque ressource est affichÃ©e sous forme **icÃ´ne + valeur** (ou libellÃ© court + valeur), mise Ã  jour en temps rÃ©el.

### 1.2 DeuxiÃ¨me ligne â€” Ressources intermÃ©diaires et secondaires

La **deuxiÃ¨me ligne** de la barre affiche les ressources dans lâ€™**ordre suivant** (de gauche Ã  droite) :

| Ordre | Ressource | Description |
|-------|-----------|-------------|
| 1 | **Nourriture** | Stock de nourriture. |
| 2 | **Bois** | Stock de bois. |
| 3 | **Pierre** | Stock de pierre. |
| 4 | **Fer** | Stock de fer. |
| 5 | **Outils** | Stock dâ€™outils. |
| 6 | **Armes** | Stock dâ€™armes. |

Ces ressources servent Ã  **construire** ou **fabriquer** dâ€™autres biens. RÃ¨gles de conversion (logique de jeu) :

| Utilisation | Ressources consommÃ©es | Effet |
|-------------|------------------------|--------|
| **Habitations** | **Bois + Pierre** | Permet de construire plus dâ€™**habitations** â†’ **augmente le cap** (plafond) de **gens disponibles**. |
| **Armes** | **Fer + Bois** | Permet de **fabriquer des armes** (augmente le stock dâ€™armes). |
| **Outils** | **Bois** OU **Pierre** OU **Fer** | Permet de **fabriquer des outils** ; au moins une de ces trois matiÃ¨res est consommÃ©e. |

En rÃ©sumÃ© :
- **Bois et pierre** â†’ plus dâ€™habitations â†’ **cap de gens** plus Ã©levÃ©.
- **Fer et bois** â†’ **armes**.
- **Bois, ou pierre, ou fer** â†’ **outils**.

Les formules exactes (quantitÃ©s par construction, caps, etc.) relÃ¨vent des spÃ©cifications de jeu ; lâ€™ergonomie impose dâ€™afficher sur la **deuxiÃ¨me ligne** : **nourriture, bois, pierre, fer, outils, armes** â€” distincte de la premiÃ¨re ligne (or, gens, soldats, recherche).

### 1.3 Navigation vers les Ã©crans

La barre en haut inclut des **entrÃ©es de navigation** vers les **Ã©crans principaux** :

| LibellÃ© affichÃ© | Ã‰cran | RÃ´le |
|-----------------|-------|------|
| **Ma citÃ©e** | Gestion du village | Ã‰cran de gestion (4 boutons gauche + liste dâ€™affectation droite) â€” celui dÃ©crit dans ce document. |
| **Carte du monde** | Grandes stratÃ©gies | Carte stratÃ©gique (citÃ©s-Ã‰tats, routes, conquÃªte, envoi de troupes). |

Lâ€™ordre dâ€™affichage dans la barre : **ressources** (ligne 1 puis ligne 2), puis **Ma citÃ©e**, **Carte du monde**, et la **roue de configuration** (engrenage) Ã  droite. Le joueur clique sur Â« Ma citÃ©e Â» ou Â« Carte du monde Â» pour afficher lâ€™Ã©cran correspondant.

### 1.4 Disposition type de la barre (deux lignes)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Or â”‚ Gens â”‚ Soldats â”‚ Recherche â”‚ Ma citÃ©e â”‚ Carte du monde â”‚ [âš™]         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Nourriture â”‚ Bois â”‚ Pierre â”‚ Fer â”‚ Outils â”‚ Armes â”‚                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

- **Ligne 1** : les 4 ressources principales (**or**, gens, soldats, recherche) ; Ã  droite : **Ma citÃ©e**, **Carte du monde**, **roue de configuration** (engrenage).
- **Ligne 2** : **nourriture**, **bois**, **pierre**, **fer**, **outils**, **armes**.

---

## 2. Vue dâ€™ensemble du layout (Ã©cran de jeu)

Sous la barre en haut, lâ€™Ã©cran affichÃ© dÃ©pend du choix de navigation : **gestion du village** (layout ci-dessous) ou **grandes stratÃ©gies** (carte).

### 2.1 Layout Â« Gestion du village Â»

Lâ€™Ã©cran **Gestion du village** est divisÃ© en **deux moitiÃ©s** :

| Zone | Position | RÃ´le |
|------|-----------|------|
| **Partie gauche** | MoitiÃ© gauche de lâ€™Ã©cran | **Quatre gros boutons** : Champs, Ateliers, ChÃ¢teau, Village. **Clic manuel** â†’ gains immÃ©diats (nourriture, outils, soldats, gens). |
| **Partie droite** | MoitiÃ© droite de lâ€™Ã©cran | **Liste dÃ©roulante** des **lieux dâ€™affectation** : le joueur y place des **gens** pour que des **ressources soient gÃ©nÃ©rÃ©es automatiquement** (idle). |

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Or â”‚ Gens â”‚ Soldats â”‚ Recherche â”‚ Ma citÃ©e â”‚ Carte du monde â”‚ [âš™]         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Nourriture â”‚ Bois â”‚ Pierre â”‚ Fer â”‚ Outils â”‚ Armes â”‚                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                              â”‚                                               â”‚
â”‚   PARTIE GAUCHE              â”‚   PARTIE DROITE                               â”‚
â”‚   (4 gros boutons)           â”‚   (liste dÃ©roulante)                          â”‚
â”‚                              â”‚                                               â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚   Lieux oÃ¹ placer des gens :                  â”‚
â”‚   â”‚   CHAMPS    â”‚  â†’ Nourriture   â–¼ [Liste dÃ©roulante]                       â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚   â€¢ Champs (nourriture auto)                   â”‚
â”‚                              â”‚   â€¢ Ateliers (outils auto)                     â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚   â€¢ â€¦                                         â”‚
â”‚   â”‚  ATELIERS   â”‚  â†’ Outils   â”‚   Affectation : X gens ici,                  â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚   Y gens lÃ  â†’ gÃ©nÃ©ration auto                  â”‚
â”‚                              â”‚                                               â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚                                               â”‚
â”‚   â”‚   CHÃ‚TEAU   â”‚  â†’ Soldats  â”‚                                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚                                               â”‚
â”‚                              â”‚                                               â”‚
â”‚   â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚                                               â”‚
â”‚   â”‚   VILLAGE   â”‚  â†’ Gens     â”‚                                               â”‚
â”‚   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚                                               â”‚
â”‚                              â”‚                                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 3. Partie gauche â€” Quatre gros boutons

Les **quatre gros boutons** occupent la **moitiÃ© gauche** de lâ€™Ã©cran. Chaque bouton est **grand, visible, cliquable** ; un clic dÃ©clenche un **gain immÃ©diat** (ressource ou unitÃ©), Ã  la maniÃ¨re du cookie dans Cookie Clicker.

### 3.1 Les Champs

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Champs** â€” reprÃ©sente les terres agricoles. |
| **Action au clic** | Le joueur **gagne des points en nourriture** (gain immÃ©diat Ã  chaque clic). |
| **RÃ´le** | Boucle **clicker** : cliquer pour accumuler de la nourriture manuellement. |
| **Feedback** | Retour visuel au clic (animation, +N nourriture, son optionnel). |

### 3.2 Les Ateliers

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Ateliers** â€” reprÃ©sente les ateliers de production. |
| **Action au clic** | Le joueur **gagne des points en outils** (gain immÃ©diat Ã  chaque clic). |
| **RÃ´le** | Boucle **clicker** : cliquer pour accumuler des outils manuellement. |
| **Feedback** | Retour visuel au clic (animation, +N outils, son optionnel). |

### 3.3 Le ChÃ¢teau

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **ChÃ¢teau** â€” reprÃ©sente le recrutement militaire. |
| **Action au clic** | Le joueur **crÃ©e des soldats** (un ou plusieurs par clic, selon rÃ¨gles de coÃ»t Ã  dÃ©finir). |
| **RÃ´le** | Boucle **clicker** : cliquer pour recruter des soldats (sous rÃ©serve de ressources / conditions). |
| **Feedback** | Retour visuel au clic ; si coÃ»t insuffisant, message ou feedback nÃ©gatif. |

### 2.4 Le Village

| Attribut | Description |
|----------|-------------|
| **Label / sens** | **Village** â€” reprÃ©sente la population civile. |
| **Action au clic** | Le joueur **crÃ©e des gens** (population civile). |
| **RÃ´le** | Boucle **clicker** : cliquer pour faire croÃ®tre la population (sous rÃ©serve de nourriture / conditions). |
| **Feedback** | Retour visuel au clic ; si conditions non remplies (ex. nourriture), message ou feedback nÃ©gatif. |

### 3.5 SynthÃ¨se partie gauche

| Bouton | Effet au clic (immÃ©diat) |
|--------|---------------------------|
| **Champs** | + Nourriture |
| **Ateliers** | + Outils |
| **ChÃ¢teau** | + Soldats (si conditions remplies) |
| **Village** | + Gens (si conditions remplies) |

Les **coÃ»ts** (nourriture pour crÃ©er des gens, ressources pour soldats, etc.) et les **montants par clic** relÃ¨vent des spÃ©cifications de jeu (formules, Ã©quilibrage) ; lâ€™ergonomie impose seulement que chaque gros bouton ait un **effet clair et immÃ©diat** au clic.

---

## 4. Partie droite â€” Liste dÃ©roulante dâ€™affectation des gens

La **moitiÃ© droite** de lâ€™Ã©cran affiche une **liste dÃ©roulante** (ou liste extensible) des **endroits oÃ¹ le joueur peut placer des gens** pour que des **ressources soient gÃ©nÃ©rÃ©es automatiquement**.

### 4.1 Principe (style Cookie Clicker)

- Le joueur dispose dâ€™un **stock de gens** (population disponible).
- Il **affecte** une partie de ces gens Ã  diffÃ©rents **lieux** (Champs, Ateliers, etc.).
- Les gens affectÃ©s **produisent des ressources en continu** (gÃ©nÃ©ration passive / idle), sans avoir Ã  cliquer.
- La **liste dÃ©roulante** recense ces lieux et permet de **choisir combien de gens** envoyer dans chaque lieu.

### 3.2 Contenu de la liste dÃ©roulante

La liste propose les **lieux dâ€™affectation** possibles, par exemple :

| Lieu dâ€™affectation | Ressource gÃ©nÃ©rÃ©e automatiquement |
|--------------------|-------------------------------------|
| **Champs** | Nourriture (par tick / par seconde) |
| **Ateliers** | Outils |
| **Recherche** (optionnel) | Points de recherche |
| **MatiÃ¨res premiÃ¨res** (optionnel) | MatiÃ¨res premiÃ¨res |
| **Armes** (optionnel) | Armes |
| â€¦ | Selon rÃ¨gles de jeu (Document Fondateur) |

Chaque ligne (ou entrÃ©e dÃ©roulante) permet typiquement :
- dâ€™**afficher le lieu** et la **ressource gÃ©nÃ©rÃ©e** ;
- de **rÃ©gler le nombre de gens affectÃ©s** (slider, champs +/- ou liste de choix).

### 4.3 Comportement attendu

| Action | Effet |
|--------|--------|
| **Ouvrir la liste** | Afficher tous les lieux oÃ¹ des gens peuvent Ãªtre affectÃ©s. |
| **Choisir un lieu** | Afficher le dÃ©tail : nom, ressource gÃ©nÃ©rÃ©e, nombre de gens actuellement affectÃ©s, capacitÃ© ou limite Ã©ventuelle. |
| **Affecter X gens Ã  un lieu** | RÃ©duire le nombre de **gens disponibles** de X et augmenter la **gÃ©nÃ©ration automatique** de la ressource correspondante. |
| **Retirer des gens dâ€™un lieu** | Les gens redeviennent **disponibles** ; la gÃ©nÃ©ration automatique de ce lieu diminue. |

### 4.4 CohÃ©rence avec la partie gauche

- **Partie gauche (clic)** : gains **immÃ©diats** au clic (nourriture, outils, soldats, gens).
- **Partie droite (liste)** : **affectation de gens** â†’ gÃ©nÃ©ration **automatique** dans le temps (idle).
- Les **Champs** et **Ateliers** apparaissent Ã  la fois :
  - comme **boutons Ã  cliquer** (gauche) pour un gain manuel ;
  - comme **lieux dâ€™affectation** (droite) pour une production automatique par les gens.
- Le **ChÃ¢teau** (soldats) et le **Village** (gens) sont surtout des **boutons de crÃ©ation** Ã  gauche ; lâ€™affectation Ã  droite concerne les **lieux de production** (Champs, Ateliers, Recherche, etc.).

---

## 5. RÃ©sumÃ© ergonomique

| Zone | Ã‰lÃ©ment | RÃ´le |
|------|---------|------|
| **Barre haut (ligne 1)** | **Ressources + navigation** | **Or**, Gens, Soldats, Recherche ; puis **Ma citÃ©e**, **Carte du monde**, roue de configuration. |
| **Barre haut (ligne 2)** | **Ressources secondaires** | **Nourriture**, Bois, Pierre, Fer, Outils, Armes â€” matiÃ¨res premiÃ¨res (bois+pierre â†’ habitations ; fer+bois â†’ armes ; bois/pierre/fer â†’ outils) + stocks nourriture, outils, armes. |
| **Gauche** | **Champs** (gros bouton) | Clic â†’ + Nourriture |
| **Gauche** | **Ateliers** (gros bouton) | Clic â†’ + Outils |
| **Gauche** | **ChÃ¢teau** (gros bouton) | Clic â†’ + Soldats |
| **Gauche** | **Village** (gros bouton) | Clic â†’ + Gens |
| **Droite** | **Liste dÃ©roulante** | Lieux oÃ¹ placer des gens â†’ **gÃ©nÃ©ration automatique** de ressources (nourriture, outils, etc.) Ã  la maniÃ¨re dâ€™un Cookie Clicker. |

Lâ€™Ã©cran de jeu comporte donc **une barre en haut** (ressources + navigation) et, sur lâ€™Ã©cran **Gestion du village**, **deux piliers** : **clic manuel** (quatre gros boutons) et **idle par affectation** (liste droite).

---

## 6. RÃ©fÃ©rences

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md) â€” Gameplay gestion (ressources, gens, moral).
- [MiyuClicker - Parcours Utilisateur](MiyukiniClicker%20-%20Parcours%20Utilisateur.md) â€” FenÃªtre principale.
- [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) â€” Assets UI (boutons, listes).

---

**Document crÃ©Ã© le :** 2026-02-01  
**Statut :** Ergonomie Ã©cran de gestion â€” 4 boutons gauche, liste affectation droite (Cookie Clickerâ€“like)

