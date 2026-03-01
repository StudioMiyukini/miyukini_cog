# SD-Systems — Systèmes Transversaux Sodomight

## Contexte

Ce document décrit exhaustivement tous les systèmes transversaux du jeu Sodomight — clone fidèle de Diablo 2 : Lord of Destruction développé sur le moteur MGE (Miyukini Game Engine, ECS archetype, data-driven TOML). Il couvre l'interface utilisateur, l'économie, le multijoueur, la progression, l'audio et le loot avancé. Les données sont issues de l'Arreat Summit officiel, du wiki Diablo 2, de Maxroll.gg et de l'analyse des fichiers de données D2 (1.13c/D2R).

Les noms propriétaires Blizzard sont renommés pour Sodomight selon la colonne "Sodomight".

## Portée / Scope

- Résolution de référence : 800 × 600 px (D2 original)
- Version de référence : Diablo 2 LoD 1.13c / D2R 2.x
- Crates concernées : `mge-ui`, `mge-net`, `mge-save`, `mge-audio`

---

## SECTION 1 : Interface Utilisateur (UI/UX)

### 1.1 HUD Principal

Le HUD de Sodomight est identique à D2 original en 800×600. Toutes les positions sont en px depuis le coin supérieur gauche de l'écran.

#### 1.1.1 Orbe de Vie (Life Globe)

| Propriété | Valeur D2 original | Sodomight |
|-----------|-------------------|-----------|
| Position X (centre) | ~60 px depuis bord gauche | identique |
| Position Y (centre) | ~560 px depuis haut | identique |
| Forme | Globe elliptique (sprite DC6) | identique |
| Couleur liquide | Rouge (#CC0000 base) | identique |
| Affichage numérique | HP actuel / HP max au survol | identique |
| Overflow | Impossible (clamped) | identique |
| Remplissage | Bas vers haut proportionnel à HP% | identique |

La valeur numérique vie/mana n'est visible que lors du survol de l'orbe ou via un toggle (option D2R adoptée dans Sodomight).

#### 1.1.2 Orbe de Mana (Mana Globe)

| Propriété | Valeur D2 original |
|-----------|-------------------|
| Position X (centre) | ~740 px depuis bord gauche |
| Position Y (centre) | ~560 px depuis haut |
| Couleur liquide | Bleu (#0000CC base) |
| Comportement | Identique à l'orbe de vie |

#### 1.1.3 Ceinture (Belt) — Gestion des Potions

La ceinture occupe le bas central du HUD, entre les deux orbes.

| Type de ceinture | Rangées | Slots total | Sodomight |
|-----------------|---------|-------------|-----------|
| Sash / Light Sash | 2 | 8 (4×2) | identique |
| Light Belt | 2 | 8 | identique |
| Belt | 3 | 12 (4×3) | identique |
| Heavy Belt | 3 | 12 | identique |
| Plated Belt | 4 | 16 (4×4) | identique |
| Tous Exceptional | 4 | 16 | identique |
| Tous Elite | 4 | 16 | identique |

**Règles de fonctionnement de la ceinture :**

- 4 colonnes fixes, nombre de rangées variable selon le type de ceinture équipé
- Colonne 1–4 = slots visibles dans le HUD. La ceinture est l'UNIQUE inventaire visible en permanence
- **Assignation automatique** : quand on ramasse une potion et que la barre de ceinture a de la place dans la colonne correspondante (potions de vie → colonnes configurées), elle est insérée automatiquement
- **Shift+clic** sur une potion en inventaire → place dans la ceinture (première colonne disponible de même type)
- **Clic gauche** sur la ceinture en combat → consomme la potion dans le slot du bas
- Les potions "montent" automatiquement quand la rangée basse est consommée
- **Assignation de type par colonne** : le joueur peut configurer quelle colonne accueille quel type de potion (vie, mana, rejuvenation)

Position HUD (800×600) :
- Slot [0,0] centre approximatif : X=354, Y=593 px
- Espacement inter-slot : 29 px horizontal
- Hauteur visible d'une rangée : ~29 px

#### 1.1.4 Barre de Skills Inférieure

| Élément | Position approximative (800×600) |
|---------|----------------------------------|
| Skill gauche (LMB) | X=117, Y=593 (icône 50×50) |
| Skill droit (RMB) | X=683, Y=593 (icône 50×50) |
| Séparateur central | Centre 400 px |

- **Clic gauche** sur l'icône LMB = ouvre le menu déroulant de sélection du skill gauche
- **Clic droit** sur l'icône RMB = ouvre le menu déroulant de sélection du skill droit
- Le menu déroulant liste les skills disponibles avec niveau actuel, groupés par arbre
- L'icône du skill actif s'affiche avec son nom au survol
- Affichage "quantité restante" pour les skills à charges (nombre en bas-droit de l'icône)

#### 1.1.5 Boutons d'Interface

Rangée basse gauche (stack vertical ou horizontal selon layout) :

| Bouton | Raccourci | Fenêtre ouverte |
|--------|-----------|-----------------|
| Character (C) | C | Paperdoll + stats |
| Skills (S) | S | Arbre de compétences |
| Inventory (I) | I | Inventaire grille |
| Party (O) | O | Menu de groupe |
| Message Log (Enter) | Enter | Journal de messages |
| Quest (Q) | Q | Journal de quêtes |
| Map (Tab) | Tab | Auto-map |

#### 1.1.6 Stamina Bar

- Position : juste sous l'orbe de vie gauche, barre horizontale dorée/jaune
- Dimensions approximatives : 80×8 px
- Se vide en courant, se régénère à l'arrêt ou en marchant
- Clignote en rouge quand presque vide

#### 1.1.7 Experience Bar

- Position : tout en bas de l'écran, ligne horizontale continue sur toute la largeur (800 px × 4 px)
- Couleur : violet/violet foncé rempli de gauche à droite
- Au survol : affiche le % d'XP vers le prochain niveau

#### 1.1.8 Mini-map (Auto-map)

- Toggle : touche **Tab** (toggle on/off)
- Style : overlay semi-transparent sur l'écran principal (pas de fenêtre séparée)
- Zoom : molette de défilement (ou touches +/-)
- Marqueurs : waypoints (étoile bleue), town portals (cercle violet), PNJ (points dorés), boss (crâne rouge)
- Mode D2R (adopté dans Sodomight) : mini-map dans un coin, superposable en jeu sans bloquer le gameplay

---

### 1.2 Inventaire et Paperdoll

#### 1.2.1 Fenêtre Inventaire

La fenêtre inventaire (raccourci I) ouvre une fenêtre à droite de l'écran contenant :

**Grille d'inventaire :**

| Propriété | D2 Original | D2R | Sodomight |
|-----------|-------------|-----|-----------|
| Colonnes | 10 | 10 | 10 |
| Rangées | 4 | 4 | 4 |
| Total slots | 40 | 40 | 40 |
| Taille d'une case | 29×29 px (800×600) | adapté | 29×29 px base |
| Couleur fond grille | Noir/brun foncé | identique | identique |

Chaque item occupe un nombre de cases correspondant à sa taille (ex : épée longue = 1×3, armure = 2×3, potion = 1×1).

#### 1.2.2 Paperdoll — Slots d'équipement

Positions extraites du fichier `Inventory.txt` (D2 1.13c, résolution 800×600, classe standard) :

| Slot | Nom interne | Left | Right | Top | Bottom | W | H |
|------|-------------|------|-------|-----|--------|---|---|
| Casque | Helm | 133 | 174 | 15 | 57 | 41 | 42 |
| Amulette | Neck | 200 | 221 | 15 | 38 | 21 | 23 |
| Armure | Torso | 133 | 174 | 57 | 139 | 41 | 82 |
| Arme gauche (Main) | LArm | 47 | 117 | 15 | 159 | 70 | 144 |
| Arme droite (Off) | RArm | 203 | 253 | 15 | 159 | 50 | 144 |
| Gants | Gloves | 47 | 117 | 159 | 204 | 70 | 45 |
| Ceinture (item) | Belt | 133 | 174 | 162 | 196 | 41 | 34 |
| Bottes | Feet | 203 | 253 | 162 | 234 | 50 | 72 |
| Anneau gauche | LHand | 110 | 131 | 170 | 191 | 21 | 21 |
| Anneau droit | RHand | 175 | 196 | 170 | 191 | 21 | 21 |

**Note** : Les personnages Expansion (Amazon2, etc.) utilisent des coordonnées décalées d'environ +80 px en X pour tenir compte du re-layout avec la fenêtre agrandie.

**Système de double slot d'arme (Weapon Swap) :**

- Touche **W** = swap entre Set1 (arme/bouclier principaux) et Set2 (arme/bouclier secondaires)
- Chaque set = 1 slot Main Hand + 1 slot Off Hand
- Slots Set2 visibles sous les slots Set1 dans le paperdoll
- Le swap est instantané, sans animation (uniquement son)
- Restriction : certains skills sont liés à un type d'arme (ne fonctionnent pas après le swap si incompatible)

#### 1.2.3 Fenêtre de Stats du Personnage

Onglet "Character" (touche C) — affiche à gauche de l'écran :

| Stat affichée | Description |
|---------------|-------------|
| Strength | Force — affecte dégâts mêlée, prérequis items |
| Dexterity | Dextérité — chance de toucher, defense, prérequis items |
| Vitality | Vitalité — vie, stamina |
| Energy | Énergie — mana |
| Life / Stamina / Mana | Valeurs courantes et maximales |
| Defense | Valeur calculée (base + armure + bonus) |
| Attack Rating | Chance de toucher calculée |
| Gold (inventory + stash) | Affichage séparé |
| Experience / Level | Niveau actuel et XP totale |

Points disponibles à distribuer affichés en rouge/orange si > 0.

#### 1.2.4 Drag-and-Drop — Règles

| Situation | Comportement |
|-----------|-------------|
| Slot vide | Item déposé dans le slot |
| Slot occupé par item compatible | Swap automatique : l'item équipé passe en main, le nouvel item s'équipe |
| Slot occupé par item incompatible | Impossible, item retourne à son emplacement d'origine |
| Inventaire plein lors du swap | L'item source reste à sa position, le swap échoue (message d'erreur) |
| Clic droit sur item équipé | Retire l'item vers le premier espace libre de l'inventaire |
| Shift+clic sur item | Place directement dans le premier slot libre compatible |

#### 1.2.5 Tooltip d'Item

Format exact du tooltip (affiché au survol de la souris) :

```
[Nom de l'item — couleur selon qualité]
[Base type — ex: "Long Sword"]
[Qualité — ex: "Unique Item"]
[Niveau requis — ex: "Required Level: 29"]
[Prérequis stat — ex: "Required Strength: 60"]
[Durabilité — ex: "Durability: 44/44"]
[Sockets — ex: "Socketed (2)"]
---
[Stat 1 : valeur — couleur bleue si bonus]
[Stat 2 : valeur]
...
[Affixe magique 1 — bleu]
[Affixe magique 2 — bleu]
...
[Description de set/unique — doré/vert]
```

**Couleurs de qualité :**

| Qualité | Couleur texte | Hex |
|---------|---------------|-----|
| Normal | Blanc | #FFFFFF |
| Superior | Blanc | #FFFFFF |
| Magic | Bleu | #6969FF |
| Rare | Jaune | #FFFF64 |
| Set | Vert | #00FF00 |
| Unique | Doré | #C8B464 |
| Crafted | Orange | #FF7F00 |
| Rune | Orange-jaune | #C8B464 |

**Comparaison avec item équipé** : au survol d'un item dans l'inventaire, le tooltip de l'item équipé correspondant apparaît en parallèle à gauche (feature D2R, adopter dans Sodomight).

---

### 1.3 Fenêtre de Compétences

#### 1.3.1 Structure des Arbres

Chaque classe possède **3 onglets** correspondant à 3 arbres de compétences. Noms D2 originaux → Sodomight :

| Classe D2 | Arbre 1 | Arbre 2 | Arbre 3 |
|-----------|---------|---------|---------|
| Amazon | Javelin & Spear | Bow & Crossbow | Passive & Magic |
| Necromancer | Summoning Spells | Poison & Bone Spells | Curses |
| Barbarian | Combat Skills | Combat Masteries | Warcries |
| Sorceress | Fire Spells | Lightning Spells | Cold Spells |
| Paladin | Defensive Auras | Offensive Auras | Combat Skills |
| Druid (LoD) | Elemental | Shape Shifting | Summoning |
| Assassin (LoD) | Martial Arts | Shadow Disciplines | Traps |

*(Classes Sodomight = renommées, même structure)*

#### 1.3.2 Représentation Visuelle

- Chaque arbre = liste verticale de skills, 3 colonnes × N lignes
- **Nœuds** : icônes 50×50 px avec cadre doré
- **Connexions** : lignes entre prérequis et skill (trait doré si prérequis rempli, gris sinon)
- Prérequis non rempli → skill grisé et non-allouable
- **Points disponibles** : affichés en haut de la fenêtre (nombre de points non distribués)
- **Niveau de skill** : petit nombre dans le coin inférieur gauche de chaque icône

#### 1.3.3 Tooltip au Survol

| Information | Description |
|-------------|-------------|
| Nom du skill | Titre |
| Description | Effet textuel |
| Niveau actuel / Niveau suivant | Deux colonnes comparatives |
| Coût Mana | Par utilisation |
| Prérequis | Liste des skills requis |
| Synergies | Liste des skills qui boostent ce skill |
| Bonus par synergie | "+X% de dégâts par niveau de [Skill]" |

**Hard cap** : 20 points maximum dans un skill via points alloués. Les bonus "+skills" d'items peuvent pousser au-delà (soft cap illimité).

---

### 1.4 Cube Alchimique (Horadric Cube → Sodomight : "Cube Alchimique")

| Propriété | Valeur |
|-----------|--------|
| Taille dans l'inventaire | 2×2 cases |
| Grille interne | 3 colonnes × 4 rangées (12 slots) |
| Ouverture | Clic droit sur le cube dans l'inventaire |
| Positionnement fenêtre | Superposée sur l'écran, déplaçable |
| Bouton "Transmute" | Visible uniquement quand le cube est ouvert, en bas de la fenêtre |

**Règles de transmutation :**

- Seuls les items requis par la recette doivent être présents (aucun item superflu)
- Si la recette est invalide ou si des items étrangers sont présents : échec silencieux ou message d'erreur
- Résultat instantané : les composants disparaissent, l'item résultant apparaît dans le cube
- Le cube peut stocker des items en surplus d'inventaire (usage secondaire comme rangement temporaire 3×4)

---

### 1.5 Stash (Coffre Personnel)

| Version | Taille | Onglets | Partage |
|---------|--------|---------|---------|
| D2 Original (pré-LoD) | 6×8 = 48 slots | 1 | Non |
| D2 LoD | 6×8 = 48 slots | 1 | Non |
| D2R | 10×10 = 100 slots | 3 (1 perso + 2 partagés) | Partagé entre personnages du compte |
| **Sodomight** | **10×10 = 100 slots** | **5 minimum** | **Oui, partagé par compte** |

**Règles Sodomight :**

- 5 onglets : 1 onglet personnel (lié au personnage) + 4 onglets partagés (compte)
- Le stash partagé est accessible par tous les personnages du même compte
- Le gold du stash est SEPARE du gold en inventaire (cap différent, voir Section 2.1)
- Clic sur un onglet = animation de transition, contenu chargé depuis la sauvegarde locale

---

### 1.6 Fenêtre de Trade Joueur-à-Joueur

#### 1.6.1 Layout

La fenêtre de trade s'ouvre après une demande mutuelle de trade entre deux joueurs.

```
┌────────────────────────────────────────────┐
│  TRADE                                      │
├──────────────────┬─────────────────────────┤
│  [Joueur A]      │  [Joueur B]             │
│  Zone de dépôt   │  Zone de dépôt          │
│  (items A)       │  (items B)              │
│  Gold: [input]   │  Gold: [affiché]        │
│                  │                         │
│  [ACCEPTER]      │  [ACCEPTER]             │
├──────────────────┴─────────────────────────┤
│  Statut : "En attente de confirmation..."  │
└────────────────────────────────────────────┘
```

#### 1.6.2 Mécanisme Anti-Scam (Double Confirmation)

1. **Phase 1** : Chaque joueur dépose ses items et la quantité de gold dans sa zone
2. **Phase 2** : Les deux joueurs cliquent "Accepter" → un écran de récapitulatif final apparaît (items listés, gold affiché en clair)
3. **Phase 3** : Confirmation finale des deux joueurs → trade exécuté
4. Si l'un des joueurs modifie les items après que l'autre ait accepté → les confirmations se réinitialisent

**Gold tradeable** : oui, via un champ numérique avec validation (ne peut pas dépasser le gold possédé).

**Sécurité Sodomight** :
- Affichage des noms d'item en clair dans la phase de récapitulatif
- Timer de 5 secondes après la dernière modification avant de pouvoir confirmer
- Log de trade enregistré localement (horodatage ISO 8601)

---

### 1.7 Shop Vendeurs NPC

#### 1.7.1 Layout de la Boutique

```
┌─────────────────────────────────────────────────┐
│  [Nom du vendeur] — [Act / Zone]                │
├─────────────────────────────────────────────────┤
│  [Item 1]  [Item 2]  [Item 3]  [Item 4]         │
│  [Item 5]  [Item 6]  [Item 7]  [Item 8]         │
│  ...                                             │
├─────────────────────────────────────────────────┤
│  Gold : [valeur]        [ACHETER]  [QUITTER]    │
└─────────────────────────────────────────────────┘
```

#### 1.7.2 Rafraîchissement de l'Inventaire Vendeur

| Condition | Comportement |
|-----------|-------------|
| Quitter la ville et y revenir (même acte) | Inventaire rechargé (nouveaux items générés) |
| Changer d'acte via waypoint | Inventaire PAS rechargé |
| Nouveau personnage dans la partie | Inventaire rechargé |
| Monter de niveau | L'ilvl des items disponibles augmente (ilvl = clvl + 5, cap 99) |

**ilvl des items en boutique** : clvl + 5, cappé à 99. Les vendors normaux et exceptionnels vendent des items normaux (avec sockets) jusqu'à ilvl 25, puis exclusivement des items magiques.

#### 1.7.3 Calcul des Prix

**Formule de prix d'achat** (simplifiée) :
```
prix_achat = prix_base × (1 + somme_modificateurs_affixes / 1024) + bonus_additifs
```

**Prix de vente** (joueur → NPC) : environ 20–25% du prix d'achat (facteur NPC défini dans `Misc.txt`).

#### 1.7.4 Service de Gambling

| Propriété | Valeur |
|-----------|--------|
| Nom Sodomight | "Échange Alchimique" ou "Pari de l'Oracle" |
| Mécanique | Acheter un item "non-identifié" dont la qualité est révélée à l'achat |
| ilvl de l'item gamblé | clvl - 5 à clvl + 4 (random, minimum 5) |
| Qualité Magic | 89.85% (1797/2000) |
| Qualité Rare | 10% (200/2000) |
| Qualité Set | 0.10% (2/2000) |
| Qualité Unique | 0.05% (1/2000) |
| Coût | Très élevé, basé sur ilvl et type d'item |
| Items disponibles | Anneaux, amulettes, circlets, casques, gants, bottes, ceintures |
| Influence du MF | Aucune (le gambling est indépendant du Magic Find) |

**Formule d'upgrade Exceptional/Elite** au gambling :
- Chance Exceptional = `max(0%, (ilvl − qlvl_exc) × 0.9% + 1%)`
- Chance Elite = `max(0%, (ilvl − qlvl_elite) × 0.33% + 1%)`

**Pourquoi gambler les circlets/coronets** : ces bases peuvent générer des stats qui ne se trouvent sur aucun autre slot (bonus de skills, +2 skills toutes classes), et leurs qlvl permettent d'obtenir des variantes Elite dès ~clvl 80+.

#### 1.7.5 Service de Réparation

- Clic droit sur un item en paperdoll dans la fenêtre NPC → affiche le coût de réparation
- "Réparer tout" : répare tous les items équipés en une seule transaction
- Coût = proportion de durabilité perdue × prix de base de réparation de l'item

#### 1.7.6 Identification par Deckard Cain (Sodomight : "Sage Aldric")

| Condition | Comportement |
|-----------|-------------|
| Quête de sauvetage complétée | Identification gratuite pour tous les items |
| Quête non complétée (A1) | 100 gold par item |
| Quête non complétée (A2+) | 200 gold par item |
| "Identifier tout" (D2R feature adoptée) | Identifie tous les items non-identifiés en une action |

---

### 1.8 Menus et Navigation

#### 1.8.1 Menu Principal

```
SODOMIGHT
─────────
New Game
Load Character
Online / LAN
Options
Credits
Exit
```

**Options disponibles :**

| Catégorie | Paramètres |
|-----------|-----------|
| Audio | Volume musique, volume effets, volume ambiance, volume voix |
| Vidéo | Résolution (800×600, 1280×720, 1920×1080+), Gamma, Contrast, Qualité effets |
| Gameplay | Difficulté par défaut, affichage HP/Mana en permanence, toujours afficher les noms d'items |
| Réseau | Port, latence maximale, annonce de partie |
| Automap | Opacité, taille, markers |
| Filtre de Loot | Activation, fichier de filtre sélectionné |

#### 1.8.2 Création de Personnage

1. Sélection de la classe (7 classes, icône animée au survol)
2. Saisie du nom (2–15 caractères, A-Z a-z 0-9 _-)
3. Toggle Hardcore (case à cocher avec avertissement)
4. Bouton "Create Character"

#### 1.8.3 Lobby Multijoueur

| Section | Fonctionnalité |
|---------|---------------|
| Liste des parties | Nom, mode, difficulté, nombre de joueurs, description |
| Filtre | Par mode (Normal/HC/Ladder), par acte, par difficulté |
| Créer une partie | Nom, mot de passe optionnel, description, mode de jeu |
| Rejoindre | Clic sur la partie + saisie du mot de passe si requis |
| Chat de lobby | Canal public, canaux privés |

#### 1.8.4 Menu Pause In-Game

- Accessible via Escape
- Options : Reprendre, Options, Save and Exit, Exit to Desktop
- En multijoueur : pause locale uniquement (le jeu continue pour les autres)

---

### 1.9 Système de Chat et Communications

| Type | Commande | Portée |
|------|----------|--------|
| Chat de partie | Saisie directe (Enter) | Tous les joueurs de la partie |
| Whisper | `/w [nom] message` | Un seul joueur (cross-partie) |
| Canal public | `/join [canal]` | Salon Battle.net / serveur |
| Réponse whisper | `/r message` | Dernier joueur qui a envoyé un whisper |

**Messages système automatiques :**

| Événement | Message affiché |
|-----------|----------------|
| Level up | "[Nom] est passé au niveau X !" |
| Boss mort | "[Boss] a été vaincu !" |
| Drop de rune rare | (optionnel, configurable dans Sodomight) |
| Joueur rejoint | "[Nom] a rejoint la partie." |
| Joueur quitte | "[Nom] a quitté la partie." |
| Hostilité déclarée | "[Nom] vous est désormais hostile." |

---

## SECTION 2 : Économie et Commerce

### 2.1 Gold

#### 2.1.1 Capacité de Portage

| Emplacement | Formule | Maximum absolu |
|-------------|---------|----------------|
| Inventaire (sur soi) | `clvl × 10 000` | 990 000 (clvl 99) |
| Stash | Palier par niveau (voir tableau) | 2 500 000 |
| **Total maximum** | — | **3 490 000** |

**Capacité du Stash par niveau :**

| Niveau | Cap Stash |
|--------|-----------|
| 1–30 | 200 000 |
| 31 | 800 000 |
| 33 | 900 000 |
| 35 | 1 000 000 |
| 37+ | +50 000 tous les 2 niveaux |
| 98 | 2 500 000 |
| 99 | 2 500 000 (cap atteint à 98) |

#### 2.1.2 Perte de Gold à la Mort (Softcore)

| Niveau | % de gold perdu |
|--------|-----------------|
| 1–20 | clvl% (ex. niveau 15 = 15% de l'inventaire) |
| 21+ | 20% fixe (inventaire + stash combinés) |

Si le gold restant après la pénalité sur le stash dépasse 20%, le surplus de l'inventaire est droppé sur le corpse.

**Note Sodomight** : le gold droppé par la mort d'un joueur est lootable par d'autres joueurs dans la même partie.

#### 2.1.3 Formule de Drop de Gold par les Monstres

Le montant de gold droppé est déterminé par la **Treasure Class** du monstre et son niveau (mlvl). Les piles de gold ont un montant aléatoire dans une fourchette dépendant du mlvl et de la difficulté :

- Normal : faibles montants (5–500 gold pour monstres low-level)
- Nightmare : ×4 environ
- Hell : ×8 environ

**Auto-pickup de gold** : option configurable dans Sodomight (activé par défaut).

---

### 2.2 Vendeurs NPC — Services Détaillés

#### 2.2.1 Service Imbue (Sodomight : "Trempe Alchimique")

- **NPC** : Charsi (Act 1 → Sodomight : forgeron de l'acte 1)
- **Récompense de quête** : "The Fallen Angel" / quête équivalente Sodomight
- **Mécanique** : transforme un item normal en un item magique de haute qualité (équivalent "magic" avec 2 affixes aléatoires de haut niveau)
- **Utilisation** : une seule fois par personnage, par difficulté (3 fois total)

#### 2.2.2 Service Socket (Sodomight : "Incrusteur")

- **NPC** : Larzuk (Act 5 → Sodomight : artisan de l'acte 5)
- **Récompense de quête** : "Rescue on Mount Arreat" / quête équivalente Sodomight
- **Mécanique** : ajoute des sockets à un item
- **Nombre de sockets** : maximum possible pour le type d'item et son ilvl (déterminé par table)
- Pour les uniques/sets : toujours 1 socket

---

### 2.3 Gambling — Mécanique Complète

| Paramètre | Valeur D2 | Valeur Sodomight |
|-----------|-----------|-----------------|
| Vendeur | Gheed (Act 1) / Alkor (Act 3) / Jamella (Act 4) | Vendeurs équivalents |
| ilvl item = | `clvl ± 5` (range clvl-5 à clvl+4, min 5) | identique |
| Magic | 89.85% | identique |
| Rare | 10.00% | identique |
| Set | 0.10% | identique |
| Unique | 0.05% | identique |

**Séquence de résolution interne :**
1. Roll `rand(2000)`
2. Si résultat < 1 → Unique
3. Si résultat < 3 → Set
4. Si résultat < 203 → Rare
5. Sinon → Magic

**Influence du MF sur le gambling** : AUCUNE. Les probabilités de qualité au gambling sont fixes et ne sont modifiées par aucun équipement ni aucune stat.

---

### 2.4 Trade Joueur-à-Joueur — Culture et Règles

**Culture du trading D2 (documentation fidèle pour Sodomight) :**

- **Monnaie de référence** : Runes de haut rang (Ber, Jah, Cham, Zod, Ohm, Lo...)
- **SOJ (Stone of Jordan)** : traditionnel dans D2 original, moins utilisé depuis D2R
- **Nomenclature des parties** : "TRADE #1", "FT ISO [item]" (For Trade / In Search Of)
- **d2jsp / forums** : sites communautaires de trading hors-jeu (Sodomight : forum intégré optionnel)

**Règles anti-scam techniques intégrées (voir Section 1.6.2).**

**Pas d'Auction House** : volontairement absent pour préserver la culture du trading humain et éviter la déflation contrôlée par bots.

---

### 2.5 Loot — Règles Complètes

| Règle | D2 Original | Sodomight |
|-------|-------------|-----------|
| Système de drop | Loot au sol, partagé | identique + option timer priorité |
| Priorité | Premier arrivé | Timer de 30s pour le tueur (configurable) |
| Gold auto-pickup | Non | Option activable |
| Drop mort joueur (SC) | Gold uniquement | identique |
| Drop mort joueur (HC) | Items équipés (si "loot" activé par le mort) | identique |

**Timer de priorité Sodomight** : configurable de 0 à 60 secondes. Pendant ce timer, seul le joueur qui a porté le coup fatal peut ramasser les items. Après expiration, ils deviennent publics.

**Drop ownership en groupe** : en D2 original, pas de mécanisme — le loot est partagé dès la mort du monstre. Sodomight adopte le système de timer prioritaire.

---

## SECTION 3 : Multijoueur

### 3.1 Structure des Parties

| Paramètre | Valeur |
|-----------|--------|
| Joueurs maximum | 8 |
| Modes | Public (visible en lobby) / Privé (mot de passe) |
| Durée de vie | La partie reste active tant qu'au moins 1 joueur est connecté |
| Types de realm | Normal Softcore, Hardcore, Ladder, Non-Ladder |

**Conventions de nommage culturelles (documentation de référence) :**

| Type de partie | Exemple de nom |
|----------------|---------------|
| Baal run | "BAAL RUN P8 01" |
| Rush | "RUSH ACT1-5 NM" |
| Trade | "TRADE FT ISO" |
| Farming | "CHAOS RUSH 01" |
| Dueling | "DUEL 1v1 HC" |
| Leveling | "LEECH 80+ HC" |

---

### 3.2 Scaling des Monstres

#### 3.2.1 HP des Monstres

**Formule :**
```
HP_monster = HP_base × (nombre_joueurs_dans_zone + 1) / 2
```

| Joueurs | Multiplicateur HP |
|---------|-------------------|
| 1 (solo) | ×1.0 (100%) |
| 2 | ×1.5 (150%) |
| 3 | ×2.0 (200%) |
| 4 | ×2.5 (250%) |
| 5 | ×3.0 (300%) |
| 6 | ×3.5 (350%) |
| 7 | ×4.0 (400%) |
| 8 | ×4.5 (450%) |

**Important** : Les stats du monstre sont fixées à sa création (spawn). Si un joueur quitte la partie après le spawn, les monstres déjà générés conservent leurs stats augmentées.

**En Nightmare/Hell** : en plus des HP, l'Attack Rating et les dégâts des monstres augmentent de +6.25% par joueur supplémentaire.

#### 3.2.2 Commande "Players X" (Solo uniquement)

En solo ou LAN, la commande `/players X` (X = 1–8) simule le scaling à X joueurs sans les joueurs réels :
- Augmente les HP/XP des monstres comme si X joueurs étaient présents
- Améliore le NoDrop (plus de loot)
- Chaque joueur ajouté via `/players` compte comme 0.5 joueur pour le calcul NoDrop (vs 1.0 pour un vrai joueur partied)

---

### 3.3 Expérience en Groupe

#### 3.3.1 Formule d'XP de Groupe

**XP total généré par le kill :**
```
XP_total = XP_base × (nombre_joueurs_dans_jeu + 1) / 2
```

**Distribution entre membres du groupe (zone commune) :**
```
XP_joueur = XP_total × (clvl_joueur / somme_clvl_tous_membres)
```

**Bonus de présence** : tous les membres partied dans la même zone reçoivent le bonus. Les membres dans une autre zone ou non-partied reçoivent 0 XP de ce kill.

#### 3.3.2 Pénalité de Niveau (Level Penalty)

Si l'écart de niveau entre le personnage et le monstre est important, l'XP est réduite. Table de pénalité en pourcentage :

| Niveau personnage - Niveau monstre | % XP reçue |
|------------------------------------|------------|
| 0 à +2 (personnage pas trop fort) | 100% |
| +3 | ~95% |
| +5 | ~85% |
| +10 | ~50% |
| +15 | ~25% |
| +20 | ~5% |
| ≥+22 | ~1% (quasi-nul) |

*(Table exacte dans `Monstats.txt`, colonne `XPL` — pénalité progressive)*

---

### 3.4 PvP — Player vs Player

#### 3.4.1 Système Hostile

| Paramètre | D2 Original | D2R | Sodomight |
|-----------|-------------|-----|-----------|
| Déclaration | Unilatérale | Mutuelle (consentement des deux) | **Unilatérale** (fidèle à D2 original) |
| Niveau minimum | 9 | 9 | 9 |
| Lieu de déclaration | Menu Party, en ville uniquement | identique | identique |
| Délai avant attaque | 10 secondes | identique | identique |
| TP fermés | Oui (portails de l'hostile) | identique | identique |
| Marqueur automap | Croix rouge | identique | identique |

**Règle importante** : déclarer hostile à un joueur rend sa PARTY entière hostile. Tous les membres de son groupe peuvent attaquer l'agresseur.

#### 3.4.2 Réduction de Dégâts PvP

| Type d'attaque | Multiplicateur dégâts |
|----------------|----------------------|
| Toutes attaques physiques et sorts | ×1/6 (≈17%) |
| Aura Thorns (Paladin) | ×1/8 (≈12.5%) |
| Malédiction Iron Maiden (Nécromancien) | ×1/4 (≈25%) |
| Dégâts des invocations/golems | ×1/2 (50%) |
| "Hit Slows Target" | Cappé à 50% de ralentissement en PvP |

#### 3.4.3 Zones Sûres et Règles Géographiques

| Zone | Attaque possible |
|------|-----------------|
| Ville (town) | Non — les attaques sont impossibles en ville |
| Zones de jeu normales | Oui, si hostilité déclarée |
| Arène de duel (Act 5, Bloody Foothills → Sodomight : "Arène des Ruines") | Oui, zone dédiée |

#### 3.4.4 Conséquences du PvP

| Mode | Mort en PvP |
|------|-------------|
| Softcore | Perte de gold (formule mort standard), aucune perte d'XP |
| Hardcore | Mort permanente du personnage |

**Trophy Ear system** : à la mort d'un joueur hostile, l'agresseur reçoit un "ear" (oreille) portant le nom, classe et niveau de la victime. Item cosmétique/social (Sodomight : peut être remplacé par un "Sceau de Défaite" thématique).

**"Ding protection"** : pas de mécanisme officiel dans D2 original. Sodomight peut implémenter une protection de 60 secondes après le level up (période d'invulnérabilité PvP).

---

### 3.5 Coopération

#### 3.5.1 Waypoints Partagés

| Version | Règle |
|---------|-------|
| D2 Original | Les waypoints activés s'appliquent uniquement au personnage qui les a activés |
| D2R | Idem — pas de partage automatique |
| Sodomight | **Option configurable** : partage auto ou non (défaut : non partagé, fidèle D2) |

#### 3.5.2 Quêtes en Groupe

| Situation | Comportement |
|-----------|-------------|
| Un joueur complete une quête | Il reçoit la récompense. Les autres doivent compléter eux-mêmes la quête pour l'obtenir |
| Boss tué en groupe | Tous les joueurs dans la partie reçoivent le crédit de kill |
| Récompenses de quête (stat points, skill points) | Chaque joueur doit récupérer sa récompense individuellement auprès du NPC |

#### 3.5.3 Town Portals en Multijoueur

| Règle | D2 Original |
|-------|-------------|
| Un seul portail actif par joueur | Oui — ouvrir un nouveau TP ferme l'ancien |
| Accessible à tous dans la partie | Oui, si partied |
| Le portail se ferme quand | Le caster retourne en jeu via son propre portail |
| Portail d'un joueur hostile | Se ferme quand la hostilité est déclarée |
| Utilisation infinie par les alliés | Oui — seule l'utilisation par le caster lui-même ferme le portail |
| En ville → portail vers zone | Portail de retour disponible en ville (lieu de sortie du TP) |

---

### 3.6 Hardcore Multijoueur

| Règle | Valeur |
|-------|--------|
| Mort = | Personnage définitivement perdu, inaccessible |
| Items équipés | Droppés sur le corpse si "loot" activé avant la mort |
| Items en inventaire/stash | Perdus si le joueur quitte avant récupération |
| Qui peut looter le corpse | Uniquement les joueurs partied qui avaient le droit de "loot" activé |
| Mélange HC/SC | Interdit — personnage HC ne peut rejoindre une partie SC et vice-versa |
| Si le host HC meurt | La partie continue pour les autres joueurs SC ou HC survivants |

**Règle de "loot" préventif** : dans le menu de groupe, chaque joueur HC peut cocher "Allow looting" pour ses alliés. Cette autorisation doit être donnée AVANT la mort.

---

## SECTION 4 : Progression et Personnage

### 4.1 Système de Niveaux

**Cap maximum** : Niveau 99

**Points distribués par niveau :**
- +5 points d'attribut par niveau
- +1 point de compétence par niveau

**Table d'XP complète (valeurs D2 1.13c / D2R) :**

| Niveau | XP Totale requise | XP vers niveau suivant |
|--------|------------------|----------------------|
| 1 | 0 | 500 |
| 2 | 500 | 1 000 |
| 3 | 1 500 | 2 250 |
| 4 | 3 750 | 4 125 |
| 5 | 7 875 | 6 300 |
| 6 | 14 175 | 8 505 |
| 7 | 22 680 | 10 206 |
| 8 | 32 886 | 11 510 |
| 9 | 44 396 | 13 319 |
| 10 | 57 715 | 14 600 |
| 15 | ~220 165 | ~40 000 |
| 20 | ~671 891 | ~100 000 |
| 25 | ~2 050 449 | ~250 000 |
| 30 | ~5 493 363 | ~500 000 |
| 35 | ~10 906 488 | ~800 000 |
| 40 | ~19 235 252 | ~1 300 000 |
| 45 | ~32 050 088 | ~2 000 000 |
| 50 | ~51 767 302 | ~4 650 593 |
| 55 | ~82 104 680 | ~7 000 000 |
| 60 | ~128 782 495 | ~10 000 000 |
| 65 | ~200 602 101 | ~15 000 000 |
| 70 | ~311 105 466 | ~25 000 000 |
| 75 | ~481 128 591 | ~40 102 443 |
| 80 | ~742 730 244 | ~60 000 000 |
| 85 | ~1 145 236 814 | ~100 000 000 |
| 90 | ~1 764 543 065 | ~150 000 000 |
| 95 | ~2 717 422 497 | ~250 000 000 |
| 98 | ~3 226 000 000 | ~290 000 000 |
| 99 | ~3 520 485 254 | — (max) |

*(Source : fichier `Experience.txt` D2 1.13c, identique pour D2R)*

---

### 4.2 Points d'Attribut

**Répartition** : libre (pas de contrainte de classe, on peut tout mettre en Force si voulu).

**Formules de dépense par stat :**

| Classe | Vie/point Vita | Mana/point Energy | Stamina/point Vita |
|--------|---------------|-------------------|-------------------|
| Amazon | +3 vie | +1.5 mana | +1 stamina |
| Necromancer | +2 vie | +2 mana | +1 stamina |
| Barbarian | +4 vie | +1 mana | +1 stamina |
| Sorceress | +2 vie | +2 mana | +1 stamina |
| Paladin | +3 vie | +1.5 mana | +1 stamina |
| Druid | +1.5 vie | +2 mana | +1 stamina |
| Assassin | +3 vie | +1.75 mana | +1 stamina |

#### 4.2.1 Points de Stat Bonus via Quêtes

| Quête | Acte | Récompense | Multiplicateur (×3 difficultés) |
|-------|------|-----------|--------------------------------|
| Lam Esen's Tome | Act 3 | +5 points d'attribut | +15 total |
| **Total** | — | — | **+15 points d'attribut** |

#### 4.2.2 Respec — Token d'Absolution (Sodomight : "Cristal de Rééquilibrage")

**Méthode 1 : Akara (Sodomight : PNJ équivalent Act 1)**
- 1 respec gratuit par difficulté (Normal/Cauchemar/Enfer) = 3 respecs totaux
- Disponible après complétion de la quête "Den of Evil" / quête équivalente

**Méthode 2 : Token d'Absolution (craft)**

Recette du Cube Alchimique :
```
Essence Tordue de la Souffrance (drop Andariel/Duriel Hell)
+ Essence Chargée de la Haine (drop Mephisto Hell)
+ Essence Brûlante de la Terreur (drop Diablo Hell)
+ Essence Putréfiée de la Destruction (drop Baal Hell)
= 1 Token d'Absolution
```

- Drop rate : faible (~1 sur 10-15 kills de boss Hell)
- Utilisations : illimitées (craftables autant de fois que possible)
- Effet : réinitialise TOUS les points de stat ET tous les points de skill en une seule utilisation

---

### 4.3 Points de Compétences

**Total par leveling** : 98 points (niveaux 2 à 99 = 98 niveaux × 1 point)

**Points bonus via quêtes :**

| Quête | Acte | Récompense | × 3 difficultés |
|-------|------|-----------|-----------------|
| Den of Evil | Act 1 | +1 skill point | +3 |
| Radament's Lair | Act 2 | +1 skill point | +3 |
| The Fallen Angel / Izual | Act 4 | +2 skill points | +6 |
| **Total quêtes** | — | +4 / difficulté | **+12 totaux** |

**Total maximum absolu : 98 + 12 = 110 skill points**

**"+skills" items** : certains items accordent des niveaux bonus (ex. "+2 à tous les skills"). Ces bonus sont temporaires (liés à l'item), poussent le skill au-delà du hard cap de 20, et sont recalculés dynamiquement.

**Hard cap** : 20 points alloués manuellement par skill (points durs). Les niveaux effectifs peuvent dépasser 20 via les bonus d'items (soft cap illimité).

---

### 4.4 Modes de Jeu

| Mode | Description | Règles spéciales |
|------|-------------|-----------------|
| Softcore (Normal) | Mode standard | Mort = respawn en ville, pénalité gold/XP |
| Hardcore | Mort permanente | 1 vie, mort = fin du personnage |
| Ladder | Saisonnier | Pool d'items spéciaux (Runewords LoD), reset périodique, classement |
| Non-Ladder | Permanent | Pas de Runewords LoD-only, pas de reset |

**Sodomight — Mode Saisonnier** :
- Reset Ladder tous les 6 mois (durée configurable)
- Les personnages Ladder deviennent Non-Ladder à la fin de la saison
- Classements : niveau max, kills de boss, time trials (optionnel)
- Pas de feature exclusive Ladder dans le MVP — à implémenter post-MVP

---

### 4.5 Mort et Résurrection

#### 4.5.1 Mort Softcore

**Séquence d'événements :**

1. HP descend à 0 → animation de mort
2. Le personnage est téléporté en ville (point de respawn de l'acte courant)
3. Le corpse reste sur place avec tous les items équipés
4. Gold perdu calculé immédiatement (voir Section 2.1.2)
5. XP perdue calculée selon la difficulté

**Perte d'XP :**

| Difficulté | Pénalité XP |
|------------|-------------|
| Normal | Aucune |
| Nightmare | 5% de l'XP requise pour le niveau suivant |
| Hell | 10% de l'XP requise pour le niveau suivant |
| Récupération (corpse dans même partie, même session) | 75% de l'XP perdue récupérée |

**Récupération du corpse :**
- Marcher sur son propre corpse dans la zone → items ré-équipés automatiquement
- Ordre de ré-équipement : casque → amulette → armure → arme/bouclier → anneaux → ceinture → bottes → gants → sets secondaires
- Si l'inventaire est plein lors du ré-équipement : les items impossibles à placer tombent au sol
- Si le personnage meurt 16 fois sans récupérer son premier corpse : les items tombent au sol définitivement

**Sécurité corpse Sodomight** : le corpse persiste tant que la session de jeu est active. En solo, le corpse est sauvegardé entre les sessions (le monde est rechargé à la reconnexion).

#### 4.5.2 Mort Hardcore

1. HP descend à 0 → animation de mort
2. Personnage marqué comme "mort" — inaccessible en lecture/écriture pour le gameplay
3. Le corpse reste avec les items équipés (si "loot" autorisé, les alliés peuvent ramasser)
4. Items en inventaire, stash et cube perdus si le joueur quitte avant récupération par un allié
5. Le personnage apparaît en grisé dans la liste des personnages (archivé, non supprimable)

---

## SECTION 5 : Audio et Atmosphère

### 5.1 Musique — Structure

**Compositeur de référence** : Matt Uelmen (D2 original). Sodomight crée ses propres compositions dans le même style : nappes atmosphériques, rythmes dépouillés, tension sans mélodie directe, instruments acoustiques + percussions ethniques.

#### 5.1.1 Pistes par Zone (D2 → Sodomight)

| Piste interne D2 | Zone(s) correspondante(s) | Durée | Ambiance |
|-----------------|--------------------------|-------|---------|
| `town1` | Rogue Encampment (Acte 1) | 4:08 | Sécurisante, mélancolique, médiéval |
| `wild` | Blood Moor, Cold Plains, Stony Field... | 8:00 | Sinistre, nappes lentes, venteuses |
| `caves` | Caves, Crypt, Mausoleum... | 3:53 | Souterrain, claustrophobique |
| `monastery` | Monastery Barracks, Cathedral... | 5:08 | Épique, sacré, orgue |
| `town2` | Lut Gholein (Acte 2) | 3:03 | Oriental, chaleur, détente relative |
| `desert` | Dry Hills, Far Oasis... | 6:35 | Aride, solo guitar, chaleur |
| `tombs` | Tal Rasha's Tombs, Chamber | 5:35 | Mystique, funèbre, lent |
| `jungle` | Spider Forest, Great Marsh... | 7:42 | Tropical, humid, tendu |
| `kurast` | Kurast Bazaar, Act 3 town | 4:58 | Urbain-décadent, cordes graves |
| `mesa` | City of the Damned, River of Flame | 5:26 | Infernal, percussions lourdes |
| `diablo` | Chaos Sanctuary (boss Act 4) | 2:35 | Intense, oppressant, climax |
| `xtown` | Harrogath (Acte 5) | 4:52 | Blizzard, isolement, nordique |
| `siege` | Arreat Plateau, Crystalline Passage... | 6:49 | Militaire, froid, tension |
| `icecaves` | Glacial Trail, Frozen Tundra... | 4:41 | Glacial, minimaliste, vide |
| `xtemple` | Worldstone Keep (intérieur) | 3:37 | Cosmique, alien, dissonant |
| `baal` | Throne of Destruction, Worldstone | 4:23 | Finale épique, orchestral complet |

**Transition entre zones** : cross-fade de 2–3 secondes (fade out piste courante, fade in nouvelle piste).

**Musique de boss** : piste dédiée jouée pendant le combat de boss, retour à la piste de zone après la mort du boss.

#### 5.1.2 Système de Lecture (mge-audio)

```toml
[audio.music]
crossfade_duration_ms = 2500
loop = true
volume_music = 0.7          # 0.0–1.0
volume_sfx = 0.8
volume_ambient = 0.6
volume_voice = 1.0

[audio.music.zones]
# Mapping zone_id → fichier audio
"act1_town" = "music/act1_town.ogg"
"act1_wilderness" = "music/act1_wild.ogg"
"act1_caves" = "music/act1_caves.ogg"
# ...
```

---

### 5.2 Sons Iconiques à Reproduire

#### 5.2.1 Sons de Loot (priorité haute — signature auditive D2)

| Son | Description | Déclencheur |
|-----|-------------|-------------|
| Drop d'item Magic | Clin métal léger | Item Magic posé au sol |
| Drop d'item Rare | Clin métallique plus riche | Item Rare posé au sol |
| Drop d'item Set | Son cristallin vert | Item Set posé au sol |
| Drop d'item Unique | Coup sourd + résonance dorée | Item Unique posé au sol |
| Drop d'une Rune | Son grave et lourd, résonant | Rune posée au sol |
| Drop de gold | Tintement de pièces | Pile de gold posée |
| Ramassage d'item | Clic sec + validation | Clic sur un item |
| Identification d'item | Éclat magique + révélation | Utilisation scroll ID |

#### 5.2.2 Sons d'Interface

| Son | Déclencheur |
|-----|-------------|
| Ouverture inventaire | Bruissement de parchemin |
| Ouverture stash | Grincement de coffre + verrou |
| Allocation d'un point de skill | Ding lumineux montant |
| Allocation d'un point de stat | Clic affirmé |
| Level up | Orchestral montant (3–4 secondes), lumière visuelle |
| Ouverture cube | Cliquetis mécanique |
| Transmutation réussie | Bourdonnement magique + flash |
| Trade accepté | Cloche douce à deux tons |
| Mort du personnage | Son de chute + silence |

#### 5.2.3 Sons de Combat par Type

| Type de dégâts | Ambiance sonore |
|----------------|----------------|
| Physique (épée, hache) | Frappe métallique, chair |
| Feu | Crépitement + souffle de chaleur |
| Glace/Froid | Impact cristallin + craquement |
| Foudre | Crépitement électrique aigu |
| Poison | Sifflement acide + glouglou |
| Magie neutre | Bourdonnement pur + impact |
| Os/Nécro | Craquement osseux + souffle grave |
| Invocation | Sons cosmiques, résonance voix |

#### 5.2.4 Sons d'Ambiance par Acte

| Acte | Ambiance sonore de fond |
|------|------------------------|
| Act 1 — Forêts/Marais | Vent froid, feuilles, corbeaux |
| Act 2 — Désert | Vent de sable, clochettes lointaines |
| Act 3 — Jungle | Insectes, pluie tropicale, eau courante |
| Act 4 — Enfer | Lave bouillonnante, rugissements lointains, chaleur |
| Act 5 — Glace | Blizzard, vent cinglant, craquements de glace |

#### 5.2.5 Sons de Monstres (exemples représentatifs)

| Type de monstre | Sons |
|----------------|------|
| Squelettes (Undead) | Craquements osseux, cliquetis |
| Démons (Fallen) | Cris aigus, jappements |
| Bêtes (Werewolves) | Grognements, rugissements |
| Nécromants boss | Voix grave, ricanements |
| Diablo | Rugissement profond, distorsion vocale |
| Baal | Tentacules, corps déformé (sons organiques) |

---

### 5.3 Son Spatial (Spatialisation)

| Propriété | Implémentation mge-audio |
|-----------|-------------------------|
| Distance d'atténuation | Linéaire de 0 à max_distance (configurable par type de son) |
| Panoration gauche/droite | Basé sur la position relative joueur ↔ source en world-space |
| Occlusion | Sons atténués (-50% volume) si murs/obstacles entre joueur et source |
| Sons d'ambiance | Full volume, non spatialisés (fond permanent) |
| Sons de monstres | Spatialisés — voix/attaques plus fortes quand proches |
| Sons d'interface | Non spatialisés (2D direct) |

**Configuration mge-audio :**

```toml
[audio.spatial]
max_distance_units = 800.0      # distance à laquelle le son est inaudible
rolloff_factor = 1.0            # courbe d'atténuation (1.0 = linéaire)
panning_enabled = true
occlusion_enabled = true
occlusion_factor = 0.5          # atténuation si objet bloquant
```

---

## SECTION 6 : Systèmes de Loot Avancés

### 6.1 Drop Tables (Treasure Classes)

#### 6.1.1 Mécanisme des Treasure Classes (TC)

Une **Treasure Class (TC)** est un nœud dans un arbre de drop. Chaque TC contient une liste d'entrées avec des probabilités associées. Quand un monstre meurt :

1. Le moteur sélectionne la TC assignée au monstre (dépend de `mlvl` et de `monstats.txt`)
2. La TC effectue `N picks` (nombre de sélections, généralement 1–5)
3. Chaque pick tire une entrée de la TC :
   - L'entrée peut être un **item base** → l'item est généré
   - L'entrée peut être une **autre TC** → on recommence récursivement
   - L'entrée peut être **NoDrop** → aucun item généré pour ce pick

#### 6.1.2 NoDrop — Formule Exacte

```
NoDrop_effectif = (ND / (ND + SommeProb)) ^ NDE / (1 - (ND / (ND + SommeProb)) ^ NDE) × SommeProb
```

Où :
- `ND` = valeur NoDrop de base de la TC
- `NDE` = NoDrop Exponent (dépend du nombre de joueurs)
- `SommeProb` = somme de toutes les probabilités des entrées de la TC (hors NoDrop)

**Calcul du NoDrop Exponent (NDE) :**

| Type de joueur | Contribution |
|----------------|-------------|
| Vous-même | 1.0 |
| Joueur partied dans la zone | 1.0 |
| Joueur non-partied dans la zone | 0.5 |
| Joueur absent (autre zone) | 0.0 |
| Simulation `/players X` | 0.5 par joueur virtuel |

**NDE = somme arrondie à l'inférieur des contributions.**

**Impact du NoDrop** : plus de joueurs → NDE plus grand → NoDrop effectif plus faible → plus d'items droppés.

#### 6.1.3 Area Level et TC

L'Area Level (`alvl`) détermine quelle TC est utilisée pour les monstres normaux de la zone. Les champions et uniques utilisent des TCs améliorées :

| Type | Bonus alvl |
|------|-----------|
| Normal | +0 |
| Champion | +2 |
| Unique | +3 |

Les **coffres** (Chests) ont leurs propres TCs, souvent différentes des monstres et parfois plus riches.

---

### 6.2 Pipeline de Génération d'Items

**Séquence complète (dans l'ordre exact d'exécution) :**

```
1. Détermination de l'ilvl
   → Monstre : ilvl = mlvl
   → Monstre champion : ilvl = mlvl + 2
   → Monstre unique : ilvl = mlvl + 3
   → Shop : ilvl = clvl + 5 (cap 99)
   → Gambling : ilvl = clvl + rand(-5, +4) (min 5)
   → Coffre : ilvl fixe par zone

2. Sélection du base type
   → Déterminé par la TC et les sous-TCs sélectionnées

3. Vérification de qualité (par ordre) :
   a. Unique check
   b. Set check
   c. Rare check
   d. Magic check
   e. Superior check
   f. Normal (par défaut)

4. Génération des affixes (si Magic/Rare/Crafted)

5. Roll des stats numériques dans les fourchettes de l'affix
```

#### 6.2.1 Formule de Vérification de Qualité

```
Chance = (BaseChance - ((ilvl - qlvl) / Divisor)) × 128
EffectiveMF = MF × Factor / (MF + Factor)    [seulement pour Unique/Set/Rare]
FinalChance = Chance - (Chance × QualityFactor / 1024)
Succès si : rand(FinalChance) < 128
```

Paramètres par qualité (issus de `ItemRatio.txt`) :

| Qualité | Factor MF | BaseChance indicatif |
|---------|-----------|---------------------|
| Unique | 250 | ~200 (variable) |
| Set | 500 | ~150 (variable) |
| Rare | 600 | ~80 (variable) |
| Magic | N/A | N/A |

#### 6.2.2 Fallback en Cascade

| Situation | Résultat |
|-----------|---------|
| Roll Unique réussi, mais aucun unique existe pour ce base type et cet ilvl | Génère un Rare avec triple durabilité |
| Roll Set réussi, mais aucun item de set existe pour ce base type et cet ilvl | Génère un Magic avec double durabilité |
| Item résultant a une durabilité anormale | Indicateur visuel de l'origine (non affiché mais logguable) |

#### 6.2.3 Roll d'Affixes

**Item Magic :**
```
25% : préfixe + suffixe
25% : préfixe seul
50% : suffixe seul
```

**Item Rare :**
```
Nombre d'affixes : 2d3+2 (entre 2 et 6, pondéré vers 4)
→ Maximum 3 préfixes et 3 suffixes
→ 50/50 pour chaque slot : préfixe ou suffixe
→ Pas de doublon dans le même "groupe" d'affixe
→ Pondéré par la fréquence de chaque affixe
```

**Calcul de l'Affix Level (alvl) :**

```
si ilvl > 99 : ilvl = 99
si qlvl > ilvl : ilvl = qlvl
si magic_lvl > 0 : alvl = ilvl + magic_lvl
sinon :
  si ilvl < (99 - qlvl/2) : alvl = ilvl - qlvl/2
  sinon : alvl = 2×ilvl - 99
si alvl > 99 : alvl = 99
```

Seuls les affixes dont le `level` ≤ `alvl` sont éligibles à la sélection.

---

### 6.3 Magic Find — Influence Exacte

**Rappel fondamental** : le MF n'influence PAS le nombre d'items droppés ni le choix de la TC. Il influence UNIQUEMENT la qualité de l'item une fois que le TC a déterminé qu'un item sera généré.

**Formules de MF effectif avec diminishing returns :**

| Qualité | Formule | Cap effectif |
|---------|---------|-------------|
| Unique | `MF_eff = MF × 250 / (MF + 250)` | 250 |
| Set | `MF_eff = MF × 500 / (MF + 500)` | 500 |
| Rare | `MF_eff = MF × 600 / (MF + 600)` | 600 |
| Magic | Aucun diminishing returns (linéaire) | Illimité (mais plafond pratique ~200%) |

**Table de MF effectif selon la valeur brute :**

| MF Brut | Eff. Magic | Eff. Rare | Eff. Set | Eff. Unique |
|---------|-----------|-----------|---------|------------|
| 0% | 0 | 0 | 0 | 0 |
| 50% | 50 | 46 | 45 | 41 |
| 100% | 100 | 85 | 83 | 71 |
| 200% | 200 | 150 | 142 | 111 |
| 300% | 300 | 200 | 187 | 136 |
| 400% | 400 | 240 | 222 | 153 |
| 500% | 500 | 272 | 250 | 166 |
| 750% | 750 | 333 | 300 | 187 |
| 1000% | 1000 | 375 | 333 | 200 |

**Interprétation pratique** : au-delà de 300% de MF brut, chaque point supplémentaire a un effet minimal sur les uniques. Le sweet spot pour les uniques se situe entre 150–300% selon le niveau de farm.

**MF n'est pas partagé en groupe** : chaque joueur utilise son propre MF pour ses propres kills. Seul le dernier coup (killing blow) détermine quel MF s'applique aux drops.

---

### 6.4 Filtre de Loot Sodomight (SDLF — Sodomight Drop Loot Filter)

Le SDLF s'inspire de la syntaxe de Project Diablo 2, adaptée au contexte Sodomight.

#### 6.4.1 Structure de Base

```
# Syntaxe : ItemDisplay[Conditions]: Sortie
# Lignes vides et commentaires (#) ignorés
# Première règle correspondante s'applique (sauf %CONTINUE%)
```

#### 6.4.2 Conditions Disponibles

**Qualité :**
```
NORMAL MAGIC RARE UNIQUE SET CRAFTED SUPERIOR
```

**Tier de base :**
```
NORM EXC ELT
```

**Catégories d'équipement :**
```
HELM CHEST SHIELD GLOVES BOOTS BELT WEAPON AXE SWORD BOW
STAFF JEWELRY RING AMULET CHARM RUNE GEM QUEST GOLD MISC
```

**Propriétés booléennes :**
```
ETH        # item ethéré
SOCK       # a des sockets
ID         # identifié
!ID        # non identifié
INF        # indestructible
```

**Valeurs numériques :**
```
ILVL > 80         # item level supérieur à 80
RUNE > 9          # rune numéro > 9 (El=1, Tir=2, ... Zod=33)
SOCKETS > 0       # au moins un socket
GOLD > 1000       # pile d'or supérieure à 1000
CLVL > 70         # niveau personnage > 70
LVLREQ < 50       # niveau requis < 50
```

**Opérateurs logiques :**
```
AND (implicite entre conditions)
OR
! (négation)
( ) pour grouper
```

#### 6.4.3 Sorties Disponibles

**Affichage :**
```
%NAME%          Nom par défaut de l'item
%BASENAME%      Nom de base sans qualificatif
%ILVL%          Item level affiché
%SOCKETS%       Nombre de sockets
%ED%            Enhanced Damage/Defense %
```

**Couleurs texte :**
```
%WHITE% %GRAY% %BLUE% %YELLOW% %GOLD% %GREEN%
%RED% %ORANGE% %PURPLE% %TAN% %BLACK%
```

**Icônes minimap :**
```
%BORDER-FF%     Grande icône (couleur hex 2 chiffres)
%MAP-FF%        Icône moyenne
%DOT-FF%        Petite icône
```

**Son personnalisé :**
```
%SOUND-unique%  Joue le son "unique_drop"
%SOUND-rune%    Joue le son "rune_drop"
%SOUND-level%   Joue le son "level_up"
```

**Spéciaux :**
```
%CONTINUE%      Ne s'arrête pas à cette règle, continue le matching
                (pour combiner plusieurs sorties)
(vide)          Cache l'item complètement
```

#### 6.4.4 Exemples Complets

```
# ═══════════════════════════════════════════════════════
# FILTRE DE LOOT SODOMIGHT — Exemple Standard
# Auteur : Développeur Sodomight
# ═══════════════════════════════════════════════════════

# --- Cacher les items normaux de bas niveau ---
ItemDisplay[NORMAL ILVL < 30 !SOCK]:

# --- Toujours afficher les runes ---
ItemDisplay[RUNE > 0]: %ORANGE%%NAME% [%ILVL%]%MAP-FF%

# --- Runes hautes (Ohm et au-dessus = RUNE > 27) ---
ItemDisplay[RUNE > 27]: %GOLD%%NAME%%BORDER-FF%%SOUND-rune%

# --- Items Unique ---
ItemDisplay[UNIQUE]: %GOLD%%NAME% [%ILVL%]%MAP-FF%%SOUND-unique%

# --- Items Set ---
ItemDisplay[SET]: %GREEN%%NAME% [%ILVL%]%MAP-FF%

# --- Items Rares niveau élevé ---
ItemDisplay[RARE ILVL > 80]: %YELLOW%%NAME% [%ILVL%]

# --- Charmes --- toujours afficher
ItemDisplay[CHARM]: %BLUE%%BASENAME%%CONTINUE%
ItemDisplay[CHARM MAGIC]: %BLUE%%BASENAME% {%ED%}

# --- Gold si pile importante ---
ItemDisplay[GOLD > 500]: %GOLD%%NAME% [%GOLD%]

# --- Cacher le reste des items normaux ---
ItemDisplay[NORMAL]:

# --- Affichage par défaut pour tout le reste ---
ItemDisplay[]: %NAME%
```

#### 6.4.5 Intégration TOML Sodomight

```toml
[loot_filter]
enabled = true
file = "filters/default.sdlf"
strictness_level = 1           # 0 = tout voir, 5 = ultra-strict

[loot_filter.sounds]
unique_drop = "sfx/ui/drop_unique.ogg"
rune_drop = "sfx/ui/drop_rune.ogg"
set_drop = "sfx/ui/drop_set.ogg"
```

---

## SECTION 7 : Systèmes de Personnage — Compléments

### 7.1 Stamina et Déplacement

#### 7.1.1 Walk vs Run

| Mode | Stamina | Vitesse (référence base) |
|------|---------|-------------------------|
| Marche (Walk) | Se régénère lentement | Vitesse de marche (≈ 6 yards/s base) |
| Course (Run) | Se vide | Vitesse de course (≈ 9 yards/s base) |

**Toggle** : touche **R** (ou clic sur l'icône de course dans le HUD en D2R).

#### 7.1.2 Drain et Régénération de Stamina

| Situation | Drain/Régén par frame |
|-----------|----------------------|
| Course | -X (drain, dépend stats) |
| Marche | +X lent (régén partielle) |
| Arrêt | +X rapide (régén complète en ≤ 512 frames ≈ 20.5 s) |
| Arrêt total jusqu'à régén complète | Maximum 256 frames (≈ 10.24 s) depuis 0 |

**Impact de l'armure sur le drain de stamina :**

| Type d'armure | Multiplicateur drain |
|---------------|---------------------|
| Légère / Robes | ×1.0 (aucun malus) |
| Moyenne (chainmail, scale...) | ×1.5 |
| Lourde (plate, banded...) | ×2.0 |

#### 7.1.3 Faster Run/Walk (FRW)

**Formule :**
```
Vitesse_effective = Vitesse_base × (150 + FRW) / 150
```

- **Pas de breakpoints** : contrairement à l'IAS ou au FCR, chaque point de FRW améliore directement la vitesse (en yards/s)
- Diminishing returns naturels par la formule (150 au dénominateur)
- Applicable à la marche ET à la course (les deux modes bénéficient du FRW)

**Vitesses de base par classe (en yards/s) :**

| Classe | Marche | Course |
|--------|--------|--------|
| Amazon | 6 | 9 |
| Necromancer | 6 | 9 |
| Barbarian | 6 | 9 |
| Sorceress | 6 | 9 |
| Paladin | 6 | 9 |
| Druid | 6 | 9 |
| Assassin | 6 | 9 |

*(Toutes les classes ont les mêmes vitesses de base. La différenciation se fait via les stats d'items et skills.)*

---

### 7.2 Identification d'Items

#### 7.2.1 Scroll of Identify (Sodomight : "Parchemin de Révélation")

| Propriété | Valeur |
|-----------|--------|
| Taille | 1×1 |
| Stack max | 20 |
| Achat | Tous les NPC marchands |
| Effet | Identifie 1 item non-identifié |

#### 7.2.2 Tome of Identify (Sodomight : "Tome de Révélation")

| Propriété | Valeur |
|-----------|--------|
| Taille | 2×2 |
| Capacité | Jusqu'à 20 Scrolls of Identify |
| Utilisation | Clic droit sur le tome → identifie un item |
| Rechargement | Glisser des scrolls dans le tome |

#### 7.2.3 Deckard Cain / Sage Aldric (Sodomight)

- Identifie gratuitement (si quête de sauvetage complétée)
- **"Identifier tout"** (D2R feature adoptée) : identifie TOUS les items non-identifiés en inventaire en un seul clic — adopter dans Sodomight

---

### 7.3 Town Portal

#### 7.3.1 Scroll of Town Portal (Sodomight : "Parchemin de Retour")

| Propriété | Valeur |
|-----------|--------|
| Taille | 1×1 |
| Stack max | 20 |
| Effet | Ouvre un portail bilatéral entre la position actuelle et la ville |

#### 7.3.2 Tome of Town Portal (Sodomight : "Tome de Retour")

| Propriété | Valeur |
|-----------|--------|
| Taille | 2×2 |
| Capacité | Jusqu'à 20 Scrolls of Town Portal |

#### 7.3.3 Règles du Portail

| Règle | Comportement |
|-------|-------------|
| Un seul portail actif par joueur | Ouvrir un nouveau TP ferme l'ancien |
| Utilisation par les alliés | Les membres partied peuvent l'utiliser à volonté |
| Le portail se ferme quand | Le propriétaire du portail l'utilise pour retourner en jeu (pas à l'aller en ville) |
| En ville | Un portail de retour est créé au même point de sortie |
| Mort du propriétaire | Le portail ne se ferme PAS automatiquement à la mort (en SC) |
| Déclaration de hostilité | Les portails du joueur hostile se ferment |
| Fin de session | Tous les portails se ferment |

**Stratégie de groupe** : avoir deux joueurs qui ouvrent chacun un portail à deux endroits différents de la zone. Si les monstres campent l'un, on entre par l'autre. Les alliés peuvent utiliser les portails sans les fermer.

---

## Schémas TOML Sodomight

### Configuration Générale

```toml
# sodomight/config.toml
[game]
name = "Sodomight"
version = "0.1.0-alpha"
max_players = 8
loot_priority_timer_sec = 30.0    # Priorité au tueur en secondes (0 = désactivé)
gold_autodrop = true              # Ramassage auto du gold
automap_mode = "tab_toggle"       # "tab_toggle" | "always" | "never"
show_hp_mana_always = false       # Affichage permanent des valeurs HP/Mana

[difficulty.normal]
resist_penalty = 0
monster_hp_multiplier = 1.0
xp_death_penalty_pct = 0.0
gold_death_penalty_pct = 0.0

[difficulty.nightmare]
resist_penalty = -40
monster_hp_multiplier = 2.0
xp_death_penalty_pct = 0.05
gold_death_penalty_pct = 0.20

[difficulty.hell]
resist_penalty = -100
monster_hp_multiplier = 4.5
xp_death_penalty_pct = 0.10
gold_death_penalty_pct = 0.20

[stash]
tabs_personal = 1
tabs_shared = 4
width = 10
height = 10
shared_between_characters = true
gold_cap_per_tab = 2_500_000

[inventory]
width = 10
height = 4
cell_size_px = 29

[belt]
columns = 4
# rows déterminé dynamiquement par le type de ceinture équipé

[cube]
# Cube Alchimique
inventory_size_w = 2
inventory_size_h = 2
internal_width = 3
internal_height = 4

[pvp]
damage_reduction_factor = 0.1666    # 1/6
thorns_pvp_factor = 0.125           # 1/8
iron_maiden_pvp_factor = 0.25       # 1/4
minion_pvp_factor = 0.50
min_level_for_hostility = 9
hostile_declaration_delay_sec = 10.0

[stamina]
walk_regen_rate = 0.5               # fraction de stamina/frame
run_drain_rate = 1.0                # fraction drainée/frame (×armor_multiplier)
stand_regen_rate = 2.0
armor_medium_multiplier = 1.5
armor_heavy_multiplier = 2.0

[xp]
max_level = 99
stat_points_per_level = 5
skill_points_per_level = 1
# Table XP dans experience.toml

[town_portal]
max_active_per_player = 1
accessible_by_party = true
closes_on_caster_return = true

[loot_filter]
enabled = true
default_file = "filters/standard.sdlf"
hide_normal_below_ilvl = 30

[audio]
music_volume = 0.7
sfx_volume = 0.8
ambient_volume = 0.6
voice_volume = 1.0
music_crossfade_ms = 2500
spatial_max_distance = 800.0
spatial_rolloff = 1.0
```

### Configuration XP (extrait)

```toml
# sodomight/data/experience.toml
[experience]
# [niveau] = XP totale requise pour atteindre ce niveau
table = [
    0,          # niveau 1  (départ)
    500,        # niveau 2
    1_500,      # niveau 3
    3_750,      # niveau 4
    7_875,      # niveau 5
    14_175,     # niveau 6
    22_680,     # niveau 7
    32_886,     # niveau 8
    44_396,     # niveau 9
    57_715,     # niveau 10
    # ... valeurs intermédiaires ...
    51_767_302,   # niveau 50
    128_782_495,  # niveau 60
    311_105_466,  # niveau 70
    742_730_244,  # niveau 80
    1_764_543_065, # niveau 90
    3_520_485_254, # niveau 99 (max)
]
```

### Configuration des Belts

```toml
# sodomight/data/items/belts.toml
[[belt]]
name = "Sash"
tier = "normal"
rows = 2
slots = 8   # 4 colonnes × 2 rangées

[[belt]]
name = "Light Belt"
tier = "normal"
rows = 2
slots = 8

[[belt]]
name = "Belt"
tier = "normal"
rows = 3
slots = 12

[[belt]]
name = "Heavy Belt"
tier = "normal"
rows = 3
slots = 12

[[belt]]
name = "Plated Belt"
tier = "normal"
rows = 4
slots = 16

# Tous les belts Exceptional et Elite ont rows = 4, slots = 16
```

### Configuration MF

```toml
# sodomight/data/magic_find.toml
[magic_find]
# Facteurs de diminishing returns par qualité
unique_factor = 250
set_factor = 500
rare_factor = 600
magic_linear = true     # Magic est linéaire, pas de diminishing returns

# Formule : mf_eff = mf_raw * factor / (mf_raw + factor)
```

### Configuration Gambling

```toml
# sodomight/data/gambling.toml
[gambling]
# Probabilités en parts sur 2000
magic_weight = 1797     # 89.85%
rare_weight = 200       # 10.00%
set_weight = 2          # 0.10%
unique_weight = 1       # 0.05%

# ilvl = clvl + rand_range(-5, +4), minimum 5
ilvl_offset_min = -5
ilvl_offset_max = 4
ilvl_minimum = 5
```

### Configuration Multijoueur

```toml
# sodomight/net/config.toml
[network]
max_players = 8
party_max_size = 8
pvp_enabled = true
ladder_enabled = false   # Implémenter post-MVP

[network.scaling]
# HP = HP_base * (n_players + 1) / 2
hp_formula = "linear"   # (n + 1) / 2
xp_formula = "linear"   # (n + 1) / 2

[network.experience]
# Réduction XP si écart de niveau important
penalty_threshold = 5   # Écart de niveau déclenchant la pénalité
penalty_rate = 0.05     # Réduction par niveau d'écart au-delà du seuil

[network.loot]
priority_timer_sec = 30.0
nodrop_party_contribution = 1.0
nodrop_unparty_contribution = 0.5
nodrop_players_cmd_contribution = 0.5
```

---

## Annexe A — Renommages D2 → Sodomight

| Élément D2 Original | Nom Sodomight |
|---------------------|---------------|
| Horadric Cube | Cube Alchimique |
| Deckard Cain | Sage Aldric |
| Token of Absolution | Cristal de Rééquilibrage |
| Scroll of Identify | Parchemin de Révélation |
| Tome of Identify | Tome de Révélation |
| Scroll of Town Portal | Parchemin de Retour |
| Tome of Town Portal | Tome de Retour |
| Stone of Jordan (SOJ) | Pierre de Jordynn (POJ) |
| Battle.net | Réseau Sodomight |
| Rogue Encampment | Camp des Éclaireurs |
| Lut Gholein | (renommé thématiquement) |
| Harrogath | (renommé thématiquement) |
| Bloody Foothills (arène) | Arène des Ruines |
| Charsi (forgeron) | Forgeron de l'acte 1 (nom propre à définir) |
| Larzuk (artisan) | Artisan de l'acte 5 (nom propre à définir) |
| Gheed (vendeur gambling) | Vendeur équivalent (nom propre à définir) |
| Imbue (service) | Trempe Alchimique |
| Socket (service Larzuk) | Incrusteur |
| Ear (PvP trophy) | Sceau de Défaite |

---

## Annexe B — Crates MGE Concernées

| Système | Crate MGE | Responsabilité |
|---------|-----------|---------------|
| HUD, paperdoll, inventaire, tooltip | `mge-ui` | Rendu et interaction UI ECS |
| Musique, sons, spatialisation | `mge-audio` | Lecture, crossfade, spatialisation |
| Sérialisation personnage, stash, waypoints | `mge-save` | Format .d2s adapté, SQLite via KindMother |
| Multijoueur, parties, trade, chat | `mge-net` | Protocole réseau P2P + serveur lobby |
| Treasure Classes, item gen, loot filter | `mge-loot` (à créer ou dans `mge-core`) | Pipeline de génération d'items |

---

## Sources

- [The Arreat Summit — Blizzard Official](https://classic.battle.net/diablo2exp/)
- [Diablo 2 Wiki — diablo2.diablowiki.net](https://diablo2.diablowiki.net/)
- [Maxroll.gg D2 Resources](https://maxroll.gg/d2/resources/)
- [The Phrozen Keep — d2mods.info](https://d2mods.info/)
- [Drop Calculator silospen](https://dropcalc.silospen.com/)
- [Project Diablo 2 Wiki — Item Filtering](https://wiki.projectdiablo2.com/wiki/Item_Filtering)
- [fabd/diablo2 GitHub — Data files 1.13c](https://github.com/fabd/diablo2)
- [PureDiablo](https://www.purediablo.com/diablo-2/)
- [Diablo 2 Wiki Fandom — Archive](https://diablo-archive.fandom.com/wiki/)
- [Wowhead D2 Guides](https://www.wowhead.com/diablo-2/guides)
