# Miyukini Survivor — Gameplay et mécaniques

## Contexte

Ce document précise les **aspects gameplay** du service Miyukini Survivor : **personnalisation du personnage** (nouvelle partie, caractéristiques, +5 pts), joueur (déplacement, attaques, **slots d'équipement**), Château, ennemis, phases (Préparation / Bataille), tours, compétences, or et XP, drops, loot (préfixes/suffixes), vagues spéciales, dimensions (px), troupes de soldats, inventaire, modes (Normal/Hardcore), achievements et quêtes journalières. Il complète le [Document Fondateur](./Miyukini%20Survivor%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Périmètre :** Règles de jeu, stats, priorités de cible, conditions de victoire/défaite, personnalisation du personnage (nouvelle partie), construction de tours, arbre de compétences, drops, loot, vagues spéciales, modes, inventaire, troupes, équipement du joueur (slots), achievements et quêtes.
- **Hors périmètre :** Valeurs numériques exactes (équilibrage), implémentation technique.
- **Référence loot :** [Miyukini Survivor - Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md) (table exhaustive d'équipement et d'objets).

---

## 1. Personnalisation du personnage (nouvelle partie)

- **À chaque nouvelle partie**, le joueur arrive sur un **écran de personnalisation du personnage**.
- Les **caractéristiques** sont **attribuées aléatoirement** dans une **fourchette de référence 1–5** par stat (équilibrage possible).
- Le joueur a droit d'**ajouter 5 points** à répartir librement parmi les caractéristiques.

### 1.1 Caractéristiques (stats de base)

| Caractéristique | Effets |
|-----------------|--------|
| **Force** | Dégâts de l'attaque auto ; prérequis d'équipement. |
| **Constitution** | PV max ; tick de récupération de PV (**0,01 PV/s** par point de Constitution). |
| **Agilité** | Vitesse d'attaque / tir. |
| **Dextérité** | Portée des attaques ; **% de chance de crocheter les coffres**. |
| **Intelligence** | Puissance des sorts ; mana. |
| **Sagesse** | Quantité et qualité des sorts ; identification des objets. |
| **Charisme** | **Nombre max de troupes** (recrutement) ; qualité des interactions PNJ ; bonus achat/vente. |
| **Chance** | Chance de critique ; meilleur drop rate ; cadeaux ; chance d'ouvrir un coffre sans crochetage. |

- **Résumé :** 8 caractéristiques (Force, Constitution, Agilité, Dextérité, Intelligence, Sagesse, Charisme, Chance) ; tirage aléatoire dans une fourchette standard + **5 points libres** à répartir par le joueur.

---

## 2. Le joueur dans l'espace de jeu

### 2.1 Déplacement

- L'avatar du joueur peut se **déplacer sur l'écran**.
- Déplacement en **8 directions** (haut, bas, gauche, droite + diagonales).
- **Vitesse de déplacement** : **10 px/s** + **Agilité %** (bonus en % sur la base).

### 2.2 Attaque de base (auto)

- **Tous les X temps** (intervalle défini par l'arme/équipement). **Arme de base : X = 1 s.**
- **À Y px autour du joueur** : zone de dégâts (portée minimale **6 px**). **Arme de base : Y = 6 px.**
- Inflige **Z dégâts** aux ennemis à portée. **Arme de base : Z = 1–2.**
- Pas d'action manuelle requise ; l'attaque se déclenche automatiquement.

### 2.3 Armes de jet (clic)

- **À chaque clic** dans la zone de jeu : un **projectile** part **depuis le joueur** vers les **coordonnées du curseur**.
- Le projectile parcourt **X px** (portée de l'arme). **Arme de jet de base : X = 80 px.**
- Si le projectile **touche un ennemi** : celui-ci subit **Y dégâts** de l'arme. **Arme de jet de base : Y = 2–4.**
- **Cooldown de Z secondes** après le tir avant de pouvoir tirer à nouveau. **Arme de jet de base : Z = 2 s.**
- **Pierce** : certains objets ont le passif **« Transperce x fois »** (le projectile traverse jusqu'à x ennemis).

### 2.4 Sortilèges

- **Mana** : **pool max = Intelligence × 2** ; **régénération = 1 mana/s**. L'Intelligence augmente aussi les **dégâts magiques** (bonus en %).
- **Barre de sorts (hot bar)** : les sorts sont équipés dans une **barre dédiée**. Le **nombre de slots de sort** dépend de la **Sagesse** : **+1 slot toutes les 10 points** de Sagesse. Certains sorts sont **auto-attaque** (ciblage auto), d'autres s'**activent par le raccourci clavier** du slot associé. Un **temps d'incantation** s'écoule avant que les effets du sort ne s'appliquent ; ce temps **diminue avec la Sagesse** (ex. temps_base × (1 − Sagesse/100), à affiner à l'équilibrage).
- Les **projectiles de sort** vont vers **l'ennemi le plus proche** (ciblage automatique) et disparaissent après application des dégâts (pas de traverse par défaut).
- Les sorts sont **contrés par la résistance magique** des ennemis : **réduction flat des dégâts magiques**, **minimum 0** (pas par l'armure physique).

### 2.5 Effets spéciaux (armes et sortilèges)

- Certaines attaques (armes ou sortilèges) ont des **effets de zone**, **DoT** (dégâts dans le temps), ou autres effets (ralentissement, etc.).

---

## 3. Le Château

### 3.1 Rôle

- Au **centre de l'écran** se trouve l'**objectif des ennemis** : **le Château**.
- Le Château a des **stats** : **PV**, **armure**. L'**armure** est une **absorption flat** des dégâts reçus, **minimum 0** (chaque coup subi est réduit du montant armure).
- Il **n'a pas de mécaniques offensives** (pas d'attaque).

### 3.2 Game over

- Si le Château arrive à **0 PV** → **game over**.

### 3.3 Dimensions et valeurs de référence

- **Château : cube de 40×40 px**.
- **Château : 50 PV max** (valeur de référence ; équilibrage à affiner).

---

## 4. Dimensions des entités (px)

| Entité | Dimensions | Remarque |
|--------|------------|----------|
| **Château** | 40×40 px | Cube |
| **Tours** | 20×20 px | Bâtiments |
| **Entités mobiles** (joueur, ennemis normaux, followers, troupes, etc.) | 10×10 px | Taille standard |
| **Mini-boss** | 20×20 px | |
| **Boss** | 30×30 px | |

### 4.1 Couleurs des entités (identification visuelle)

| Entité | Couleur |
|--------|---------|
| **Château** | Vert |
| **Joueur** | Orange |
| **Troupes** | Bleu |
| **Ennemis** (normaux) | Marron |
| **Mini-boss** | Orange |
| **Boss** | Rouge |
| **Tours** | Couleur **selon le type** de tour ; **bordures** pour les différencier des autres entités (bâtiments identifiables visuellement). |

*(Le joueur et le mini-boss sont tous deux orange ; la distinction se fait par la taille : joueur 10×10 px, mini-boss 20×20 px.)*

**Projectiles :**

| Type | Couleur |
|------|---------|
| **Projectiles physiques** (armes de jet, attaque auto) | Blanc |
| **Projectiles magiques** (sortilèges) | Jaune |

---

## 5. Ennemis

### 5.1 Comportement et priorité de cible

- Les ennemis se **dirigent vers le Château** par défaut.
- **Champ de vision** : **rayon de 30 px** autour de l'ennemi. **Si le joueur** (priorité 1) **ou un bâtiment** (priorité 2) est dans ce rayon, ils se dirigent vers leur **cible par ordre de priorité** : **Joueur > Bâtiment > Château**.
- **Vitesses de déplacement** (référence) : **Normal = 8 px/s**, **Mini-boss = 6 px/s**, **Boss = 4 px/s**.

### 5.2 PV des ennemis (basés sur la Constitution du joueur)

Les **PV max** des ennemis sont calculés à partir de la **Constitution du joueur** (voir [1.1 Caractéristiques](#11-caractéristiques-stats-de-base)) :

| Type d'ennemi | PV max |
|---------------|--------|
| **Normal** | **½ × Constitution** du joueur (moitié de la Constitution du joueur) |
| **Mini-boss** | **5 ×** cette valeur (soit 2,5 × Constitution du joueur) |
| **Boss** | **20 ×** cette valeur (soit 10 × Constitution du joueur) |

### 5.3 Dégâts au contact

- **Tous** les ennemis infligent **au minimum 1 pt de dégâts** au contact de la **hitbox** du joueur, des troupes ou des bâtiments.
- Valeurs de référence par type (à ajuster à l'équilibrage) :

| Type d'ennemi | Dégâts au contact (référence) |
|---------------|-------------------------------|
| **Normal** | 1 PV |
| **Mini-boss** | 3 PV |
| **Boss** | 10 PV |

### 5.4 Comportements additionnels

- **Ennemis à projectiles** : certains types tirent des projectiles ; **liste des types à inclure en bêta**.
- **Ennemis qui traquent le joueur** : certains types ont priorité joueur fixe ; **à inclure plus tard** (hors bêta).

### 5.5 Récompenses (drops)

- **Tuer un ennemi** fait apparaître au sol :
  - de l'**or** ;
  - des **objets** (équipement, consommables ; soumis à un **drop rate**) ;
  - des **orbes d'XP**.
- **Tous les drops restent au sol un temps limité** : **valeur par défaut 4 secondes** (évolutive via équipement, compétences ou événements).
- **Quand il reste moins de 2 secondes** avant disparition : les **entités au sol (or, objets, orbes) clignotent** pour alerter le joueur.
- Le **loot** (objets avec préfixes et suffixes, type Diablo / Path of Exile) est **primordial pour maintenir l'engagement**. Une **table exhaustive d'équipement et d'objets avec préfixes et suffixes** permet une liste d'objets dépassant largement la concurrence — voir [Miyukini Survivor - Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md).

### 5.6 Synthèse : progression de la difficulté des mobs

Formules issues de la doc (référence unique pour l’équilibrage) :

**1. PV des ennemis** (dépend de la **Constitution** du joueur, *C*) :

| Type       | PV max |
|------------|--------|
| Normal     | **⌊ *C* / 2 ⌋** (ou ½ × *C*) |
| Mini-boss  | **5 ×** (PV normal) = 2,5 × *C* |
| Boss       | **20 ×** (PV normal) = 10 × *C* |

**2. Dégâts au contact** (hitbox joueur / troupes / bâtiments) :

- **Minimum** : **1 pt** pour tous les types.
- Référence : Normal 1 PV, Mini-boss 3 PV, Boss 10 PV.

**3. Spawn par vague** (nombre et fréquence d’apparition) :

- **Vague 1** : spawn quantity = **5**, spawn rate = **3 s**.
- **Vague *n* + 1** :
  - **Spawn quantity** = **⌈ spawn_quantity(*n*) × 1,1 ⌉ + 1**
  - **Spawn rate** = **spawn_rate(*n*) × 0,99**

**4. Vagues Boss** : vague **10, 20, 30, …** (tous les 10 vagues). **1 Boss** par vague Boss.

**5. Mini-boss** : entre **1 et 3** mini-boss par vague (hors Boss), avec **10 % + nombre de vagues depuis le dernier spawn de mini-boss** (formule de probabilité / quantité à affiner à l'équilibrage).

**6. Lune rouge** : voir [§ 6.1 Lune rouge](#61-lune-rouge).

---

## 6. Vagues spéciales

### 6.1 Lune rouge

- **Fréquence** : **10 % de base** + **nombre de vagues sans Lune rouge** (cumul), **maximum 20 vagues** sans Lune rouge avant apparition forcée (ou équivalent à l'équilibrage).
- **Vague suivante plus difficile** : **quantité d'ennemis × (1,5 + 0,1 × n)**, où **n = nombre de vagues depuis la dernière Lune rouge**.
- **Récompenses plus grandes** : **or, XP, drop rate × (2 + 0,1 × n)** (même n).

### 6.2 Vagues Boss

- Les **vagues Boss** sont **signalées** à l'écran (et dans la barre de vague).
- Une **vague Boss** a lieu **tous les 10 vagues** (vague 10, 20, 30, etc.) ; **1 Boss** par vague Boss.

---

## 7. Les phases

### 7.0 Fin de phase et passage

- **Fin de phase Bataille** : **tous les ennemis morts** **ou** **timer max 15 min** (IRL). La vague est alors remportée (ou perdue si Château à 0 PV).
- **Fin de phase Préparation** : le joueur **déclenche** le passage via le bouton **« Lancer la vague »**, **ou** **timer max 30 min** (IRL).
- **Séquence** : **fin Bataille** → (optionnel) écran de résumé → **phase Préparation** ; **fin Préparation** (bouton ou 30 min max) → **phase Bataille**.

### 7.1 Phase « Préparation »

Pendant cette phase, le joueur peut :

1. **Utiliser ses points de compétences** (gagnés en level up) à répartir dans un **arbre de compétences**.
2. **Acheter de l'équipement** (avec l'or disponible).
3. **Construire des tours** dans un certain **rayon autour du Château**, symbolisé par un **disque vert avec opacité** centré sur le Château. Cette zone peut **grandir** en fonction des divers bonus.

#### 7.1.1 Types de tours

- Il existe **différents types de tours** aux **effets et attaques différentes**.
- Les tours ont des **stats comme le joueur** (PV, dégâts, portée, etc.).

**Tour de base** (référence) :

| Attribut | Valeur |
|----------|--------|
| **Coût** | **50 or** |
| **Portée / champ de vision** | **80 px** (référence tour de base) |
| **Attaque** | Tire un **projectile** vers l'**ennemi le plus proche** dans son **champ de vision** |
| **Ciblage à égalité** | Si plusieurs ennemis à égale distance → **plus proche du Château** |
| **Cadence** | **1 projectile par seconde** |
| **Dégâts** | **1 pt** par projectile |
| **PV / armure** | Les tours **prennent des dégâts** : **PV base 100**, **armure base 0** ; à **0 PV** la tour est **détruite**. |

**Autres types de tours** (à détailler) : ralentissement, zone (AoE), multi-cible, etc. — une ligne de description par type à inclure en bêta.

#### 7.1.2 Construction de bâtiments (tours)

- **Sidebar à droite** : liste de bâtiments disponibles.
- La barre liste des **blocs « bâtiment »** avec :
  - **Bouton « Construire »** et **bouton « Info »**
  - **Nom du bâtiment**, **coût en or**, **courte description**
- **Bouton « Info »** : affiche **toutes les métriques** du bâtiment.
- **Bouton « Construire »** :
  - Crée un **fantôme du bâtiment** collé au curseur.
  - Une **grille de construction** apparaît dans la zone de jeu ; les **cases font 20×20 px** (une tour 20×20 px par case).
  - Le joueur place le bâtiment dans la grille **et** dans la **portée de construction** (disque vert autour du Château).
  - **Fantôme rouge** : conditions non réunies → construction impossible à cet emplacement.
  - **Fantôme vert** : construction possible → si le joueur **clic**, le bâtiment est **ajouté à la zone de jeu**.

#### 7.1.3 Fenêtre « Skills »

- Un bouton **« Skills »** ouvre la **fenêtre des compétences** avec les **arbres de compétences**.
- Il existe **plusieurs arbres** rangés par **onglet**.
- **Level up** : pendant la vague, le joueur gagne un niveau ; le **choix d'upgrade** (où placer le point de skill) est **différé en phase Préparation**. Chaque **level up** donne **1 point de skill** à placer dans **n'importe quel arbre** (prérequis respectés), **en phase Préparation uniquement**.

---

### 7.2 Phase « Bataille »

- Les **ennemis apparaissent** depuis le **bord de l'écran** vers le Château.
- Ils se **déplacent à des vitesses variées** selon le type d'ennemi.

#### 7.2.0 Spawn des vagues (spawn rate et spawn quantity)

Chaque vague a un **spawn rate** (intervalle entre deux spawns) et une **spawn quantity** (nombre d'ennemis par spawn) qui évoluent d'une vague à l'autre.

- **Valeur initiale (vague 1) :** **5 ennemis** toutes les **3 secondes** (spawn quantity = 5, spawn rate = 3 s).
- **À chaque nouvelle vague :**
  - **Spawn quantity** = **⌈ spawn quantity précédent × 1,1 ⌉ + 1** (× 1,1 à l'entier supérieur, puis + 1).
  - **Spawn rate** = **spawn rate précédent × 0,99** (l'intervalle diminue : les spawns deviennent plus fréquents).

*Exemple : vague 1 → 5 ennemis / 3 s ; vague 2 → 7 ennemis / 2,97 s ; vague 3 → 9 ennemis / 2,94 s ; etc.*

#### 7.2.1 Mort du joueur

- Si le joueur est à **0 PV** : il est **bloqué sur place**, ne peut **plus attaquer ni bouger** jusqu'à la **fin de la phase**.
- Si le **Château survit** à la phase : la phase est **remportée** ; le joueur **revit** avec toutes ses capacités et **PV pleins**, mais avec **1 PV max en moins** (pénalité persistante pour la run).
- **Le joueur ne peut pas avoir moins de 4 PV max.**
- **Joueur : 10 PV max** au départ (valeur de référence).
- **Tours : 100 PV base**, armure base 0 ; à 0 PV la tour est détruite (voir § 7.1.1).

#### 7.2.2 Tours en phase Bataille

- Les **tours installées** **attaquent les ennemis les plus proches** dans leur **champ de vision**.
- **Type d'attaque**, **vitesse**, **dégâts** et **effets** dépendent du **type de tour**.

---

## 8. Troupes de soldats

- Les **troupes de soldats** **suivent le joueur** et **attaquent les ennemis à portée**. **Recrutement** : fenêtre **« Recruit »** (phase **Préparation** uniquement), **coût en or** ; **nombre max de troupes = Charisme** du joueur.
- **PV des troupes** : **PV max figés au moment du recrutement** (basés sur les PV max du joueur à ce moment). Entités mobiles 10×10 px.
- **Si un soldat meurt**, il **revient à la vague suivante** (réapparition en phase Préparation ou début de vague suivante).
- **Clic droit** : **toute la troupe revient sur le joueur** pendant **2 secondes** (regroupement). Pendant ces 2 s, les troupes **peuvent encore attaquer**.

---

## 9. Inventaire

### 9.1 Pendant une vague

- **L'inventaire n'a pas de limite pendant une vague** : le joueur peut **ramasser autant d'objets qu'il y arrive** (pick up illimité).

### 9.2 À la fin d'une vague réussie

- **L'ensemble de l'inventaire non occupé** ne peut contenir qu'**un certain nombre d'objets différents** (hors stack), **par défaut 10**.
- **Surplus** : **vente automatique du surplus** au-delà de la limite ; **ordre de priorité** pour déterminer ce qui est vendu : **rareté**, puis **valeur** (à affiner à l'équilibrage). Le joueur peut équiper ou conserver avant application.
- **Des équipements ou des compétences** peuvent **augmenter** la limite (ex. slot Dos « sac », monture).

---

## 10. Équipement du joueur (slots)

Le joueur peut équiper les **slots** suivants. Chaque slot accepte un type d'objet défini (voir [Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md) pour les bases et catégories).

### 10.1 Armure et corps

| Slot | Description |
|------|--------------|
| **Tête** | Casque, chapeau, etc. |
| **Collier** | Amulette, collier. |
| **Épaules** | Épaulières. |
| **Brassard** | Brassard. |
| **Gants** | Gants. |
| **Bague 1** / **Bague 2** | 2 slots bagues. |
| **Torse** | Plastron, armure de torse. |
| **Ceinture** | Ceinture. |
| **Jambes** | Jambières. |
| **Pieds** | Bottes. |

### 10.2 Mains

| Slot | Description |
|------|--------------|
| **Main gauche** | Arme, bouclier, etc. |
| **Main droite** | Arme, focus, etc. |

### 10.3 Dos, monture et divers

| Slot | Description / effet gameplay |
|------|-----------------------------|
| **Dos** | Un seul slot « dos » : **sac**, **carquois** ou **cape** (un type à la fois). **Sac** = capacité inventaire augmentée. **Carquois** = bonus **dégâts flat** et/ou **vitesse de tir** (armes de jet). **Cape** = **défense**, **vitesse**, **mana** et/ou **vitesse d'incantation** (cast speed). |
| **Monture** | **Vitesse de déplacement** augmentée ; **PV en plus** pour le joueur ; **bonus passif** = **plus de slots d'inventaire**. |

### 10.4 Objets divers et potions

| Slot | Description |
|------|--------------|
| **Objets divers** | **5 slots** pour talismans, artefacts, etc. |
| **Potion de vie** | 1 slot dédié ; **effet over time** ; **cooldown** selon la potion ; **x utilisations par run** (stack par slot). **Recharge en phase Préparation** en **payant** (or). |
| **Potion mana** | 1 slot dédié ; **effet over time** ; **cooldown** selon la potion ; **x utilisations par run** (stack par slot). **Recharge en phase Préparation** en **payant** (or). |

---

## 11. Modes de jeu

### 11.0 Slots de sauvegarde

- Il n'existe que **1 slot de sauvegarde Normal** et **1 slot de sauvegarde Hardcore**.
- Chaque mode utilise son slot dédié ; les sauvegardes Normal et Hardcore sont indépendantes.

### 11.1 Mode Normal

- En cas de **Game Over** (Château à 0 PV) : le joueur peut **rejouer la vague** (recommencer la même vague) **ou** **charger 3 vagues en arrière** (reprendre à la vague N-3).

### 11.2 Mode Hardcore

- En cas de **Game Over** : **tout est supprimé** (run terminée, pas de reprise).
- En **mode Hardcore**, le **drop rate** est **×2** (récompenses doublées pour compenser le risque).

---

## 12. Achievements et quêtes journalières

### 12.1 Achievements

- Des **achievements** (succès) **allongent la progression du joueur** : objectifs à atteindre (vagues, kills, objets, builds, etc.), récompenses **types** : **titres**, **cosmétiques**, **déblocages** (aligné sur les usages courants du genre).

### 12.2 Quêtes journalières

- **Objectifs types** : tuer X ennemis, X clics dans la journée, jouer X min, tuer un boss, etc.
- **Récompenses** : **bonus 1 h** (or, XP, drop, etc.) — format court pour maintenir l'engagement.

---

## 13. Or, coffres et PNJ

### 13.1 Or

- **Sources de gain** : **drops ennemis**, **vente d'objets**, **quêtes**, **coffres**, **Lune rouge** (récompenses augmentées).
- **Dépenses** : **achat d'équipement** (marchand en phase Préparation), **construction de tours**, **recrutement de troupes**, **achat de sortilèges**, **events** (ex. recharge potions en Préparation).

### 13.2 Coffres

- **Apparition** : à la **fin d'un boss** ou d'un **event**.
- **Contenu** : **or**, **objets**, **XP** (tous possibles).
- **Ouverture** : **crochetage** = **Dextérité × (1 + Chance/100)** = % de chance d'ouvrir le coffre (formule à affiner). **Chance** = **% de chance d'ouverture directe** (sans crochetage).

### 13.3 PNJ et Charisme

- **PNJ** : **marchands**, **quêteurs** ; accessibles via une **fenêtre** ouverte **en phase Préparation**.
- **Effets du Charisme** : **prix d'achat** = prix de base **/ (1 + Charisme/100)** (réduction) ; **quêtes supplémentaires** et **meilleures récompenses** (qualité / quantité).

---

## 14. Synthèse des valeurs de référence

| Entité | PV max (départ) | Remarque |
|--------|------------------|----------|
| **Joueur** | 10 | Minimum 4 PV max après pénalités |
| **Château** | 50 | 0 PV = game over |
| **Tours** | 100 (base) | PV base 100, armure base 0 ; 0 PV = détruite |

| Type ennemi | Dégâts au contact |
|-------------|-------------------|
| Normal | 1 PV |
| Mini-boss | 3 PV |
| Boss | 10 PV |

| Entité | Dimensions (px) |
|--------|------------------|
| Château | 40×40 (cube) |
| Tours | 20×20 |
| Mobiles (joueur, ennemis normaux, troupes, followers) | 10×10 |
| Mini-boss | 20×20 |
| Boss | 30×30 |

| Contexte | Règle |
|-----------|--------|
| Drops au sol | Disparition par défaut après 4 s ; clignotement si < 2 s restantes |
| Inventaire fin de vague réussie | Max 10 objets différents (hors stack) par défaut ; surplus **vente auto** (priorité rareté, valeur) ; équipement/compétences peuvent augmenter la limite |
| Vitesse déplacement | Joueur : 10 px/s + Agilité % ; Normal 8 px/s, Mini-boss 6 px/s, Boss 4 px/s |
| Mode Normal (Game Over) | Rejouer la vague ou charger 3 vagues en arrière |
| Mode Hardcore (Game Over) | Tout supprimé ; drop rate ×2 |
| Vague Boss | Tous les 10 vagues (10, 20, 30…) |
| Clic droit (troupes) | Toute la troupe revient sur le joueur pendant 2 s |
| **Troupes max** | **= Charisme** du joueur |
| **Sauvegardes** | 1 slot Normal, 1 slot Hardcore (indépendants) |

**Slots d'équipement du joueur (résumé) :** Tête, Collier, Épaules, Brassard, Gants, 2× Bagues, Torse, Ceinture, Jambes, Pieds — Main gauche, Main droite — Dos (sac / carquois / cape), Monture — 5 objets divers (talismans, artefacts), 1 slot potion vie, 1 slot potion mana.

---

## 15. Références

- [Miyukini Survivor - Document Fondateur](./Miyukini%20Survivor%20-%20Document%20Fondateur.md)
- [Miyukini Survivor - Ecrans et UI](./Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md)
- [Miyukini Survivor - Analyse Concurrence Survivor Tower Defense](./Miyukini%20Survivor%20-%20Analyse%20Concurrence%20Survivor%20Tower%20Defense.md)
- [Miyukini Survivor - Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md)

---

**Document créé le :** 2026-02-04  
**Dernière mise à jour :** 2026-02-04  
**Révision :** Intégration des décisions d'audit : fourchette perso 1–5 ; valeurs de référence attaque base (1 s, 6 px, 1–2) et armes de jet (80 px, 2–4, 2 s, pierce passif) ; mana (Int×2, 1 mana/s), barre de sorts (Sagesse/10 = slots), incantation, rés. mag. flat ; armure Château absorption flat ; champ de vision ennemis 30 px, vitesses 8/6/4 px/s ; Lune rouge (freq, modificateurs) ; phases (fin Bataille 15 min, fin Préparation bouton ou 30 min) ; tours (80 px, ciblage, PV 100, armure 0) ; troupes (PV figés, regroupement = peuvent attaquer) ; inventaire (vente auto surplus, priorité rareté/valeur) ; dos/monture/potions ; level up (choix en Préparation) ; or, coffres, PNJ, Charisme ; achievements et quêtes ; numérotation § 3.3, § 7, § 7.1.2, § 11.1, § 13–15.
