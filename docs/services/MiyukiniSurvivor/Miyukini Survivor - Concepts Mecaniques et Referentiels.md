# Miyukini Survivor — Concepts, mécaniques et référentiels

## Contexte

Ce document regroupe une **vue synthétique** du service Miyukini Survivor (Lord of the Castle) : **concepts fondamentaux**, **mécaniques de gameplay**, **inventaire et objets**, **bestiaire des monstres**, **mécaniques RPG** et **tableaux de référence** alignés sur le code. Il sert de point d’entrée unique pour la compréhension du jeu et de ses règles.

**Source de vérité code :** `crates/lord_of_the_castle/` (constants, game_state, game_loop, enemies, towers, troops, player, character_creation, warrior_skills, loot).

---

## Portée / Scope

- **Applicable à :** Miyukini Survivor (Lord of the Castle)
- **Contenu :** Concepts, phases, entités, combat, vagues, inventaire, bestiaire, stats, compétences, équipement
- **Statut :** Référence alignée sur le code et la doc existante
- **Documents détaillés :** [Document Fondateur](Miyukini%20Survivor%20-%20Document%20Fondateur.md), [Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md), [Bestiaire et Metriques Ennemis](reference/Miyukini%20Survivor%20-%20Bestiaire%20et%20Metriques%20Ennemis.md), [Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md)

---

## 1. Concepts fondamentaux

| Concept | Description |
|--------|-------------|
| **Zone de jeu** | Un écran ; le joueur, le Château, les ennemis, les tours et les troupes évoluent dans cet espace. |
| **Le Château** | Objectif central des ennemis ; PV et armure, pas d’attaque. À 0 PV = game over. |
| **Phase Préparation** | Le joueur dépense or, points de compétence, construit des tours dans la grille autorisée (autour du Château). |
| **Phase Bataille** | Vagues d’ennemis depuis les bords vers le Château ; le joueur, les tours et les troupes les combattent. |
| **Joueur** | Avatar déplaçable en 8 directions ; attaque auto à portée ; stats (8 caractéristiques), PV, mana, équipement. |
| **Tours** | Bâtiments construits en phase Préparation ; champ de vision 300 px, flèches 600 px de portée, 600 px/s. |
| **Ennemis** | Se dirigent vers le Château ; priorité Joueur > Tour > Château ; donnent or, XP et objets à la mort. |
| **Troupes** | Soldats recrutés (Milicien…) ; suivent le joueur dans la zone de commandement, attaquent les ennemis à portée. |
| **Projectiles alliés** | Flèches des tours : premier ennemi touché blessé, puis projectile disparaît (sauf effet type transpercer). |

---

## 2. Mécaniques de gameplay

### 2.1 Phases

- **Préparation :** compétences (arbre Guerrier), équipement (marchand), construction de tours (grille 20×20 px, bâtiments 40×40 px), recrutement de troupes. Passage en Bataille via **« Lancer la vague »**.
- **Bataille :** spawn d’ennemis (quantité et rate évolutifs), déplacement ennemis vers Château, attaque auto joueur/tours/troupes, loot au sol (or, XP, objets). Fin quand tous les ennemis sont morts (ou conditions de défaite).

### 2.2 Joueur

- **Déplacement :** 8 directions (WASD / flèches) ; vitesse **90 × (1 + Agi/100)** px/s.
- **Attaque auto :** intervalle 1 s, portée 40 px, dégâts 1–2 (arme de base).
- **Mort :** à 0 PV, bloqué jusqu’à fin de vague ; si le Château survit, revive avec **−1 PV max** (minimum 4 PV max).

### 2.3 Tours (base)

| Attribut | Valeur |
|----------|--------|
| Champ de vision (ciblage) | 300 px |
| Portée des flèches | 600 px (course max du projectile) |
| Vitesse des flèches | 600 px/s |
| Cadence | 1 projectile / 2 s |
| Dégâts par projectile | 4 |
| PV / armure | 100 PV, 0 armure ; 0 PV = tour détruite |
| Coût construction | 50 or |
| Taille / grille | 40×40 px (2×2 cases de 20 px) |

Projectile : sprite 2×2 px noir ; blesse le **premier ennemi touché** (le plus proche de l’origine sur la trajectoire), puis disparaît.

### 2.4 Troupes

- **Zone de commandement :** rayon 200 px autour du joueur ; les troupes restent et combattent dans cette zone.
- **Type Milicien :** 100 PV, 20 % blocage, 3 dégâts, portée 25 px, 1 attaque/s, vitesse 50 px/s.
- **États :** InZone, OutOfZone, Dead (respawn 10 s au château), AtCastle (récupération).
- **Vision :** 100 px pour repérer un ennemi ; max 3 troupes par ennemi ciblé.

### 2.5 Vagues

- **Vague 1 :** 5 ennemis toutes les 3 s.
- **Évolution :** spawn_quantity = ⌈ prev × 1,1 ⌉ + 1 ; spawn_rate = prev × 0,99.
- **Boss :** vagues 10, 20, 30… (1 Boss par vague Boss).
- **Mini-boss :** ~15 % de chance par spawn (hors vague Boss).

---

## 3. Inventaire et objets

### 3.1 Slots d’équipement (code)

| Slot | Libellé |
|------|---------|
| Head | Tête |
| Neck | Collier |
| Shoulders | Épaules |
| Bracer | Brassard |
| Gloves | Gants |
| Ring1 / Ring2 | Bague 1 / 2 |
| Chest | Torse |
| Belt | Ceinture |
| Legs | Jambes |
| Feet | Pieds |
| MainHand / OffHand | Main droite / gauche |
| Ammo | Munitions |
| Consumable | Consommable |

Inventaire max **20 slots** (constante `INVENTORY_MAX_SLOTS`). Objets **non identifiés** (type/slot seulement) ou **identifiés** (nom, rareté, effets).

### 3.2 Loot au sol

- **Or :** 30 % + chance de drop ; quantité 50 % à 100 % des PV du monstre.
- **XP :** 100 % drop ; quantité 10 %(+chance) à 200 %(+chance) des PV, min 1.
- **Objet :** 10 %(+chance) de drop ; type (slot) seulement jusqu’à identification.
- **Ramassage :** rayon 30 px autour du joueur.

### 3.3 Raretés

Commun, Peu commun, Rare, Magique, Ultra rare, Unique (code : `ItemRarity`).

### 3.4 Marchand

- Pools (armes, armures, accessoires) renouvelées en fin de vague.
- Identification par expert : coût 20 or (constante).
- Voir [Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md) pour préfixes/suffixes et tables.

---

## 4. Bestiaire des monstres

### 4.1 Types

| Type | Rôle | Taille (px) | Vitesse base (px/s) | Vitesse max | Dégâts contact | PV max (formule) |
|------|------|-------------|----------------------|-------------|----------------|------------------|
| **Normal** | Foule | 10×10 | 16 × (1+vague/100) | 50 | 1 | ½×C |
| **MiniBoss** | Renforcé | 20×20 | 8 × (1+vague/100) | 40 | 3 | 2,5×C |
| **Boss** | Élite | 30×30 | 6 × (1+vague/100) | 30 | 10 | 10×C |

**C** = Constitution effective du joueur au spawn : `(10 + Con).max(1)` (stats de base + bonus compétences).  
PV normal = `(C/2).max(1)` ; Normal = cette valeur, MiniBoss = 5×, Boss = 20×.

### 4.2 Comportement

- **Champ de vision :** 60 px (ciblage Joueur > Tour > Château).
- **Pushback :** 4 px appliqués au joueur/troupe au contact.
- **Détail complet :** [Miyukini Survivor - Bestiaire et Metriques Ennemis](reference/Miyukini%20Survivor%20-%20Bestiaire%20et%20Metriques%20Ennemis.md).

---

## 5. Mécaniques RPG

### 5.1 Les 8 caractéristiques (stats)

| Stat | Nom | Effets (doc / code) |
|------|-----|----------------------|
| **For** | Force | Dégâts mêlée, prérequis équipement |
| **Con** | Constitution | PV max, récupération PV |
| **Agi** | Agilité | Vitesse déplacement (90×(1+Agi/100) px/s) |
| **Dex** | Dextérité | Portée, % crochetage coffres |
| **Int** | Intelligence | Mana, puissance sorts |
| **Sag** | Sagesse | Slots de sorts, identification |
| **Cha** | Charisme | Nombre max de troupes, prix marchand |
| **Luk** | Chance | Critique, drop, coffres |

PV max joueur : formule **Con×2 + For** (avec plancher 4). Mana : pool et régen selon Intelligence (doc).

### 5.2 Création de personnage

- **Nouvelle partie :** écran de personnalisation ; **8 stats** tirées dans une fourchette (ex. [-3, 6] « Que Nawak décide ») + **5 points libres** à répartir.
- **25 phrases humoristiques** avec effets (tirage aléatoire, parcours 4 étapes).

### 5.3 Level up et compétences

- **XP requise :** 100 + niveau actuel pour le niveau suivant.
- **Level up :** +1 niveau, XP remise à zéro, **1 point de compétence** ; tous les 5 niveaux **+1 point de statistique** (à placer en For, Con, Agi, etc.).
- **Arbre Guerrier :** compétences (Baston, Plus fort, Tireur, Musculation, GigaChad, etc.) avec **prérequis** et **rang max** ; les points sont dépensés en phase Préparation.
- **Bonus stats (compétences) :** EncorePlusFort → For, Musculation → Con, Tireur → Dex ; Pot de Wey et stats effectives pour PV max.

### 5.4 Compétences Guerrier (extrait)

| Compétence | Effet type |
|------------|------------|
| Baston | +1 Force/niveau sans arme (max 10) |
| Plus fort | +1 dégât mêlée/niveau (max 3) |
| Musculation | +1 Con/niveau |
| Tireur | Bonus dégâts tir |
| GigaChad | Régénération 1 PV/s (effet passif) |
| … | Voir `warrior_skills.rs` pour la liste complète |

---

## 6. Dimensions et constantes (référence code)

### 6.1 Tailles (px)

| Entité | Constante | Valeur |
|--------|----------|--------|
| Château | size::CASTLE | 40 |
| Tours | size::TOWER | 40 |
| Case grille construction | CONSTRUCTION_CELL_SIZE | 20 |
| Mobiles (joueur, ennemis normaux, troupes) | size::MOBILE | 10 |
| Mini-boss | size::MINI_BOSS | 20 |
| Boss | size::BOSS | 30 |
| Projectile tour | TOWER_PROJECTILE_SIZE | 2 |

### 6.2 PV max de référence

| Entité | Constante / formule | Valeur |
|--------|---------------------|--------|
| Joueur | hp::PLAYER_MAX / formule Con×2+For | 10 (ref) / min 4 |
| Château | hp::CASTLE_MAX | 50 |
| Tour base | hp::TOWER_BASE | 100 |

### 6.3 Combat

| Constante | Valeur |
|-----------|--------|
| AUTO_ATTACK_INTERVAL_S | 1 s |
| AUTO_ATTACK_RANGE | 40 px |
| AUTO_ATTACK_DAMAGE_MIN/MAX | 1–2 |
| ENEMY_CONTACT_NORMAL / MINI_BOSS / BOSS | 1 / 3 / 10 |
| ENEMY_PUSHBACK_ON_CONTACT_PX | 4 |
| PICKUP_RADIUS | 30 px |
| COMMAND_ZONE_RADIUS | 200 px |
| TROOP_VISION_RADIUS | 100 px |
| TROOP_RESPAWN_DELAY_S | 10 s |

---

## 7. Documents liés

| Document | Contenu |
|----------|---------|
| [Document Fondateur](Miyukini%20Survivor%20-%20Document%20Fondateur.md) | Raison d’être, scope, décisions structurantes |
| [Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md) | Règles détaillées (phases, joueur, Château, ennemis, tours, troupes, inventaire, modes) |
| [Ecrans et UI](Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md) | Layout, barre haute, zone de jeu, sidebar |
| [Bestiaire et Metriques Ennemis](reference/Miyukini%20Survivor%20-%20Bestiaire%20et%20Metriques%20Ennemis.md) | Types ennemis, formules PV/vitesse, spawn |
| [Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md) | Loot, préfixes/suffixes, slots, raretés |
| [Audit Mecaniques Gameplay](Miyukini%20Survivor%20-%20Audit%20Mecaniques%20Gameplay.md) | Manques et priorités de doc |

---

**Date :** 2026-02-06  
**Version :** 1.0  
**Alignement :** `crates/lord_of_the_castle` (constants, game_state, game_loop, enemies, towers, troops, player, character_creation, warrior_skills, loot)
