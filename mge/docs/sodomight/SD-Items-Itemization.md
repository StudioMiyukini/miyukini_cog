# SD — Système d'Items et d'Itemisation (Sodomight)

> Document de référence exhaustif pour le projet Sodomight — clone fidèle de Diablo 2 LoD.
> Moteur : MGE (ECS archetype, data-driven TOML).
> Tous les noms propres Blizzard sont remplacés par des équivalents Sodomight.

---

## Table des matières

1. [Hiérarchie et qualités d'items](#1-hiérarchie-et-qualités-ditems)
2. [Types de bases d'items](#2-types-de-bases-ditems)
3. [Système d'affixes](#3-système-daffixes)
4. [Les 33 Runes](#4-les-33-runes)
5. [Runewords — Mots de Runes](#5-runewords--mots-de-runes)
6. [Set Items — Items de Set](#6-set-items--items-de-set)
7. [Unique Items — Items Uniques](#7-unique-items--items-uniques)
8. [Cube Alchimique](#8-cube-alchimique)
9. [Gemmes](#9-gemmes)
10. [Charmes](#10-charmes)
11. [Joyaux (Jewels)](#11-joyaux-jewels)
12. [Système de Durabilité](#12-système-de-durabilité)
13. [Système de Sockets](#13-système-de-sockets)
14. [Schémas TOML Sodomight](#14-schémas-toml-sodomight)

---

## 1. Hiérarchie et qualités d'items

### 1.1 Les 7 qualités d'items

| Qualité | Couleur affichage | Abrév. | Description |
|---------|-------------------|--------|-------------|
| Normal (Ordinaire) | Blanc | NRM | Item de base sans affixes magiques |
| Supérieur (Superior) | Blanc | SUP | +1 à +15% Enhanced Defense/Damage, +1 à +3 Durabilité |
| Magique (Magic) | Bleu | MAG | 1 préfixe + 1 suffixe maximum |
| Rare | Jaune | RAR | 2-3 préfixes + 2-3 suffixes, nom généré |
| Crafted (Fabriqué) | Orange | CRF | Via Cube Alchimique, affixes garantis + aléatoires |
| Set (Ensemble) | Vert | SET | Fait partie d'un set nommé, bonus partiels et complets |
| Unique | Or/Doré | UNQ | Stats fixes prédéfinies, le plus souvent build-defining |

> **Note Sodomight** : Les Runewords (Mots de Runes) forment une catégorie à part — techniquement des items normaux/gris avec runes sockétées.

### 1.2 Algorithme de détermination de la qualité

La génération d'un item suit une séquence de checks **en cascade** (premier succès = qualité finale) :

```
Unique → Set → Rare → Magique → Haute Qualité → Normal
```

#### Niveaux clés

| Niveau | Sigle | Définition |
|--------|-------|-----------|
| Item Level (ilvl) | ilvl | = niveau du monstre qui l'a droppé (mlvl) |
| Quality Level (qlvl) | qlvl | Inherent au type d'item (stocké dans weapons.txt / armor.txt) |
| Affix Level (alvl) | alvl | Détermine quels affixes peuvent spawner |
| Monster Level (mlvl) | mlvl | Champions : mlvl+2, Uniques : mlvl+3 |

#### Formule alvl

```
si magic_lvl > 0 :
    alvl = ilvl + magic_lvl
sinon :
    si ilvl < (99 - qlvl/2) :
        alvl = ilvl - qlvl/2
    sinon :
        alvl = 2*ilvl - 99
```

> Les items avec magic_lvl non-nul (baguettes, orbes, sceptres, circlets non-élites) peuvent obtenir des affixes plus élevés plus tôt.

#### Formule de chance de qualité

```
Chance = (BaseChance - ((ilvl - qlvl) / Diviseur)) × 128
FinalChance = Chance - (Chance × QualityFactor / 1024)
```

Succès si `random(FinalChance) < 128`.

| Qualité | BaseChance | Diviseur | Facteur MF (diminishing returns) |
|---------|-----------|----------|----------------------------------|
| Unique | 200 | 0 | 250 |
| Set | 150 | 0 | 500 |
| Rare | 80 | 0 | 600 |
| Magique | 40 | 0 | linéaire |

### 1.3 Magic Find (MF) — Formule complète

Le MF applique des **diminishing returns** par la formule :

```
MF_effectif = MF × Facteur / (MF + Facteur)
```

| MF brut | Rare effectif | Set effectif | Unique effectif |
|---------|--------------|-------------|-----------------|
| 50% | +46% | +45% | +41% |
| 100% | +85% | +83% | +71% |
| 200% | +150% | +142% | +111% |
| 300% | +200% | +187% | +136% |
| 500% | +272% | +250% | +166% |
| 1000% | +375% | +333% | +200% |

**Ce que le MF N'affecte PAS :**
- Quantité totale de drops
- Drops de runes et gemmes
- Drops de charmes et joyaux
- Taux d'items Exceptionnel/Élite

**Plage optimale recommandée :** 250–350% MF pour le meilleur ratio effet/sacrifices d'équipement.

### 1.4 Règles de génération par qualité

#### Normal / Supérieur
- qlvl ≤ ilvl
- Supérieur : chance de +ED et/ou +Durabilité (rare, ~1/200)

#### Magique
- alvl détermine le pool d'affixes disponibles
- 50% suffix only, 25% prefix only, 25% les deux
- Maximum : 1 préfixe + 1 suffixe

#### Rare
- Maximum : 3 préfixes + 3 suffixes (6 affixes total)
- Minimum garanti : 2 affixes
- Affixes tirés dans le même pool que magic mais avec restrictions de famille (pas deux affixes de la même famille)
- Nom généré à partir d'une table de noms Rare (2 mots combinés)

#### Crafted
- Input : Magic item + Perfect Gem + Jewel + Rune spécifique
- Affixes garantis (selon recette) + 1 à 4 affixes aléatoires
- Nombre d'affixes aléatoires : `floor((clvl/2 + qlvl/2) / 10)` — capped à 4
- `clvl` = niveau du personnage au moment de la transmutation

#### Set / Unique
- Stats fixes prédéfinies dans les data files
- Drop conditionné par ilvl ≥ qlvl de l'item

---

## 2. Types de bases d'items

### 2.1 Structure de progression : Normal → Exceptionnel → Élite

Chaque item de base existe en 3 tiers. L'Exceptionnel et l'Élite ont des stats plus élevées et des prérequis accrus.

### 2.2 Armes — Épées (Swords)

#### 1H Swords

| Nom Normal | Nom Exceptionnel | Nom Élite | Dégâts N | Dégâts Ex | Dégâts El | Str/Dex req (El) |
|------------|-----------------|-----------|----------|-----------|-----------|-----------------|
| Hand Axe → Short Sword | Falchion → Broad Sword | Fal. → Champion Sword | 2-6 | 13-24 | 24-54 | 108/64 |
| Short Sword | Sabre | Falcata | 2-7 | 3-14 | 23-49 | 128/18 |
| Scimitar | Shamshir | Ataghan | 2-6 | 7-20 | 26-46 | 138/95 |
| Broad Sword | Crystal Sword | Dimensional Blade | 7-14 | 5-15 | 13-35 | 85/- |
| Long Sword | Battle Sword | Gothic Sword | 3-16 | 6-21 | 14-45 | 113/- |
| War Sword | Rune Sword | Ancient Sword | 8-20 | 10-26 | 43-57 | 127/88 |
| Two-Handed Sword* | Giant Sword | Espandon | 4-24 | 9-28 | — | — |
| Flamberge* | Zweihander* | Grandfather* | — | — | — | — |

> * Certaines épées sont utilisables en 1H ou 2H selon la classe

#### 2H Swords

| Nom Normal | Nom Exceptionnel | Nom Élite | Dégâts (2H) El | Str req |
|------------|-----------------|-----------|----------------|---------|
| Two-Handed Sword | Giant Sword | Espandon | 8-26 | 73 |
| Flamberge | Zweihander | Caduceus* | 9-40 | 97 |
| Great Sword | Executioner Sword | Colossus Blade | 25-65 | 189 |
| * Caduceus = sceptre élite |

### 2.3 Armes — Haches (Axes)

| Nom Normal | Nom Exceptionnel | Nom Élite | Type | Dégâts El | Str/Dex req |
|------------|-----------------|-----------|------|-----------|-------------|
| Hand Axe | Hatchet | Tomahawk | 1H | 33-58 | 125/67 |
| Axe | Cleaver | Small Crescent | 1H | 38-60 | 115/83 |
| Double Axe | Twin Axe | Ettin Axe | 1H | 33-94 | 145/45 |
| Military Pick | Military Axe | War Spike | 1H | 30-48 | 73/110 |
| War Axe | Bearded Axe | Berserker Axe | 1H | 24-71 | 138/75 |
| Large Axe | Tabar | Feral Axe | 2H | 22-53 | 196/30 |
| Broad Axe | Gothic Axe | Silver-Edged Ax | 2H | 38-100 | 166/65 |
| Battle Axe | Ancient Axe | Decapitator | 2H | 49-137 | 189/33 |
| Great Axe | Giant Axe | Champion Axe | 2H | 59-137 | 167/59 |
| Giant Axe | Glorious Axe | Glorious Axe | 2H | 60-124 | 182/42 |

### 2.4 Armes — Masses et Gourdins (Maces)

| Nom Normal | Nom Exceptionnel | Nom Élite | Type | Dégâts El |
|------------|-----------------|-----------|------|-----------|
| Club | Cudgel | Truncheon | 1H | 35-43 |
| Spiked Club | Barbed Club | Tyrant Club | 1H | 32-58 |
| Mace | Flanged Mace | Reinforced Mace | 1H | 41-49 |
| Morning Star | Jagged Star | Devil Star | 1H | 43-53 |
| Flail | Battle Hammer | Scourge | 1H | 38-70 |
| War Hammer | War Club | Legendary Mallet | 2H | 50-61 |
| Maul | Great Maul | Ogre Maul | 2H | 77-106 |
| Great Maul | Thunder Maul | Thunder Maul | 2H | 33-171 |

### 2.5 Armes — Lances et Hast (Polearms / Spears)

| Nom Normal | Nom Exceptionnel | Nom Élite | Type | Dégâts El |
|------------|-----------------|-----------|------|-----------|
| Spear | War Spear | Hyperion Spear | 2H | 35-119 |
| Trident | Fuscina | Stygian Pike | 2H | 29-144 |
| Brandistock | War Fork | Mancatcher | 2H | 42-92 |
| Spetum | Yari | Ghost Spear | 2H | 18-155 |
| Pike | Pike | War Pike | 2H | 14-63 |
| Bardiche | Partizan | Ogre Axe | Polearm | 28-68 |
| Voulge | Bec-de-Corbin | Colossus Voulge | Polearm | 17-165 |
| Scythe | Grim Scythe | Thresher | Polearm | 12-141 |
| Poleaxe | Lochaber Axe | Giant Thresher | Polearm | 40-114 |
| Halberd | Bill | Cryptic Axe | Polearm | 33-150 |
| War Scythe | War Scythe | Colossus Blade | Polearm | 25-100 |

### 2.6 Armes — Arcs et Arbalètes

| Nom Normal | Nom Exceptionnel | Nom Élite | Type | Dégâts El |
|------------|-----------------|-----------|------|-----------|
| Short Bow | Edge Bow | Spider Bow | Arc | 23-50 |
| Hunter's Bow | Razor Bow | Blade Bow | Arc | 21-57 |
| Long Bow | Cedar Bow | Shadow Bow | Arc | 15-59 |
| Composite Bow | Double Bow | Great Bow | Arc | 32-55 |
| Short Battle Bow | Short Siege Bow | Diamond Bow | Arc | 33-60 |
| Long Battle Bow | Long Siege Bow | Crusader Bow | Arc | 15-71 |
| Short War Bow | Rune Bow | Ward Bow | Arc | 20-72 |
| Long War Bow | Gothic Bow | Hydra Bow | Arc | 10-86 |
| Light Crossbow | Arbalest | Pellet Bow | Arbal. | 28-76 |
| Crossbow | Siege Crossbow | Gorgon Crossbow | Arbal. | 33-104 |
| Heavy Crossbow | Ballista | Colossus Crossbow | Arbal. | 32-91 |
| Repeating Crossbow | Chu-Ko-Nu | Demon Crossbow | Arbal. | 26-40 |

### 2.7 Armes — Baguettes, Orbes, Sceptres, Bâtons

| Catégorie | Nom Normal | Nom Exceptionnel | Nom Élite | Notes |
|-----------|------------|-----------------|-----------|-------|
| Wand | Wand | Burnt Wand | Polished Wand | Nécromancien |
| Wand | Bone Wand | Petrified Wand | Ghost Wand | Nécro |
| Wand | Grim Wand | Tomb Wand | Lich Wand | Nécro |
| Wand | Lich Wand | Grave Wand | Unearthed Wand | Nécro |
| Orb | Eagle Orb | Glowing Orb | Heavenly Stone | Sorcière |
| Orb | Sacred Globe | Crystalline Globe | Eldritch Orb | Sorcière |
| Orb | Smoked Sphere | Cloudy Sphere | Demon Heart | Sorcière |
| Orb | Clasped Orb | Sparkling Ball | Vortex Orb | Sorcière |
| Orb | Jared's Stone | Swirling Crystal | Dimensional Shard | Sorcière |
| Scepter | Scepter | Rune Scepter | Mighty Scepter | Paladin |
| Scepter | Grand Scepter | Holy Water Sprinkler | Seraph Rod | Paladin |
| Scepter | War Scepter | Divine Scepter | Caduceus | Paladin |
| Staff | Short Staff | Jo Staff | Walking Stick | Druide/Sorcière |
| Staff | Long Staff | Quarterstaff | Stalagmite | — |
| Staff | Gnarled Staff | Cedar Staff | Elder Staff | — |
| Staff | Battle Staff | Gothic Staff | Shillelagh | — |
| Staff | War Staff | War Staff | Archon Staff | — |

### 2.8 Armes — Daggers et Griffes

| Catégorie | Nom Normal | Nom Exceptionnel | Nom Élite | Classe |
|-----------|------------|-----------------|-----------|--------|
| Dagger | Dagger | Poignard | Bone Knife | — |
| Dagger | Dirk | Rondel | Mithral Point | — |
| Dagger | Kris | Cinquedeas | Fanged Knife | — |
| Dagger | Blade | Stiletto | Legend Spike | — |
| Katar | Katar | Quhab | Suwayyah | Assassine |
| Katar | Wrist Blade | Demon Claw | Wrist Sword | Assassine |
| Katar | Hatchet Hands | Scissors Quhab | War Fist | Assassine |
| Katar | Cestus | Fascia | Battle Cestus | Assassine |
| Katar | Claws | Hand Scythe | Feral Claws | Assassine |
| Katar | Greater Claws | Greater Talons | Runic Talons | Assassine |
| Katar | Greater Talons | Scissors Suwayyah | Scissors Suwayyah | Assassine |

### 2.9 Armes — Javelines (Amazon)

| Nom Normal | Nom Exceptionnel | Nom Élite | Type |
|------------|-----------------|-----------|------|
| Javelin | Maiden Javelin | Hyperion Javelin | Amazon |
| Pilum | Ceremonial Javelin | Thundermaids | Amazon |
| Short Spear | Matriarchal Javelin | Grand Matron Bow | Amazon |
| Glaive | Scissors Quhab | — | Amazon |

### 2.10 Armures — Corps (Body Armor)

| Nom Normal | Nom Exceptionnel | Nom Élite | Def Min/Max N | Def El | Str req El |
|------------|-----------------|-----------|--------------|--------|------------|
| Quilted Armor | Ghost Armor | Dusk Shroud | 8-11 | 361-467 | 77 |
| Leather Armor | Serpentskin Armor | Wyrmhide | 14-17 | 368-478 | 84 |
| Hard Leather Armor | Demonhide Armor | Scarab Husk | 21-24 | 380-492 | 95 |
| Studded Leather | Trellised Armor | Wire Fleece | 32-35 | 400-506 | 111 |
| Ring Mail | Linked Mail | Diamond Mail | 45-48 | 446-524 | 131 |
| Scale Mail | Tigulated Mail | Loricated Mail | 57-60 | 410-524 | 149 |
| Chain Mail | Mesh Armor | Boneweave | 72-75 | 399-487 | 158 |
| Light Plate | Cuirass | Great Hauberk | 90-93 | 456-542 | 118 |
| Breast Plate | Mage Plate | Archon Plate | 65-68 | 410-524 | 103 |
| Splint Mail | Russet Armor | Kraken Shell | 90-95 | 417-550 | 174 |
| Plate Mail | Templar Coat | Hellforge Plate | 108-116 | 521-599 | 196 |
| Field Plate | Sharktooth Armor | Lacquered Plate | 116-128 | 433-500 | 208 |
| Gothic Plate | Embossed Plate | Shadow Plate | 127-139 | 413-487 | 230 |
| Full Plate Mail | Chaos Armor | Sacred Armor | 150-165 | 487-600 | 232 |

### 2.11 Armures — Casques (Helms)

| Nom Normal | Nom Exceptionnel | Nom Élite | Def El | Str req El |
|------------|-----------------|-----------|--------|------------|
| Cap | War Hat | Shako (Harlequin Crest base) | 12-15 | 20 |
| Skull Cap | Sallet | Hydraskull | 20-24 | 28 |
| Helm | Casque | Armet | 29-34 | 41 |
| Full Helm | Basinet | Giant Conch | 38-44 | 63 |
| Great Helm | Winged Helm | Spired Helm | 60-66 | 96 |
| Crown | Grand Crown | Corona | 72-78 | 103 |
| Mask | Death Mask | Demonhead | 27-30 | 55 |
| Bone Helm | Grim Helm | Bone Visage | 33-36 | 106 |

**Casques de Classe :**

| Classe | Nom Normal | Nom Exceptionnel | Nom Élite |
|--------|------------|-----------------|-----------|
| Barbarian | Wolf Head | Jawbone Cap | Carnage Helm |
| Barbarian | Hawk Helm | Fanged Helm | Fury Visor |
| Barbarian | Antlers | Horned Helm | Destroyer Helm |
| Barbarian | Falcon Mask | Assault Helmet | Conquerer Crown |
| Barbarian | Spirit Mask | Avenger Guard | Slayer Guard |
| Druid | Pelt | Tarnhelm (base) | Blood Spirit |
| Druid | Preserved Head | Totemic Mask | Sun Spirit |
| Druid | Zombie Head | Savage Helmet | Earth Spirit |
| Druid | Unraveller Head | Citadel Helm | Sky Spirit |
| Druid | Gargoyle Head | Minion Skull | Dream Spirit |
| Necromancer | Preserved Head | Dried Corpse Head | Mummy Head |
| Necromancer | Zombie Head | Decapitated Head | Vampire Head |
| Necromancer | Unraveller Head | Voodoo Head | Medusa Head |
| Necromancer | Gargoyle Head | Minion Skull | Worm Head |
| Necromancer | Demon Head | Overlord Skull | Demon Head |

**Circlets (non-classe) :**

| Nom | Str req | Notes |
|-----|---------|-------|
| Circlet | — | magic_lvl = 2 |
| Coronet | — | magic_lvl = 3 |
| Tiara | — | magic_lvl = 5 |
| Diadem | — | magic_lvl = 5 |

### 2.12 Armures — Boucliers

| Catégorie | Nom Normal | Nom Exceptionnel | Nom Élite | Block El | Str req El |
|-----------|------------|-----------------|-----------|----------|------------|
| Shield | Small Shield | Tiger Shield | Aegis | 52% | 219 |
| Shield | Large Shield | Defenders | Ward | 52% | 185 |
| Shield | Buckler | Round Shield | Hyperion Shield | 52% | 127 |
| Shield | Kite Shield | Scutum | Monarch (4 sockets) | 46% | 156 |
| Shield | Tower Shield | Gothic Shield | Colossus Shield | 48% | 255 |
| Paladin | Targe | Hyperion Targe | Zakarum Shield | 50% | 110 |
| Paladin | Rondache | Aegis (Pal) | Kurast Shield | 52% | 121 |
| Paladin | Heraldic Shield | Ancient Shield | Hierarch Shield | 48% | 163 |
| Paladin | Aerin Shield | Pavise | Sacred Rondache | 56% | 109 |
| Paladin | Crown Shield | Ancient Armor | Vortex Shield | 56% | 148 |
| Nécromancien | Preserved Head | Homunculus (base) | Blood Lord Skull | — | — |

### 2.13 Armures — Gants, Bottes, Ceintures

#### Gants (Gloves)

| Nom Normal | Nom Exceptionnel | Nom Élite | Def El |
|------------|-----------------|-----------|--------|
| Leather Gloves | Demonhide Gloves | Bramble Mitts | 8-9 |
| Heavy Gloves | Sharkskin Gloves | Vampirebone Gloves | 21-23 |
| Chain Gloves | Heavy Bracers | Vambraces | 28-30 |
| Light Gauntlets | Battle Gauntlets | Crusader Gauntlets | 37-42 |
| Gauntlets | War Gauntlets | Ogre Gauntlets | 55-62 |

#### Bottes (Boots)

| Nom Normal | Nom Exceptionnel | Nom Élite | Def El |
|------------|-----------------|-----------|--------|
| Boots | Demonhide Boots | Wyrmhide Boots | 12-15 |
| Heavy Boots | Sharkskin Boots | Scarabshell Boots | 24-27 |
| Chain Boots | Mesh Boots | Boneweave Boots | 29-32 |
| Light Plated Boots | Battle Boots | Mirrored Boots | 37-42 |
| Greaves | War Boots | Myrmidon Greaves | 53-62 |

#### Ceintures (Belts) — 4 tailles

| Nom Normal | Nom Exceptionnel | Nom Élite | Lignes potion | Def El |
|------------|-----------------|-----------|--------------|--------|
| Sash | Demonhide Sash | Spiderweb Sash | 1 | 55-62 |
| Light Belt | Sharkskin Belt | Vampirefang Belt | 2 | 50-65 |
| Belt | Mesh Belt | Mithril Coil | 3 | 58-70 |
| Heavy Belt | Battle Belt | Troll Belt | 3 | 62-76 |
| Plated Belt | War Belt | Colossus Girdle | 4 | 65-78 |

> La taille de la ceinture détermine le nombre de rangées de potions accessibles.

---

## 3. Système d'affixes

### 3.1 Mécanisme général

- **Préfixe** : modifie le début du nom de l'item (ex : "Godly Plate of the Whale")
- **Suffixe** : modifie la fin du nom de l'item
- **Items Magiques** : 1 préfixe + 1 suffixe maximum
- **Items Rares** : jusqu'à 3 préfixes + 3 suffixes (6 max), jamais 2 affixes de la même "famille"
- **Limite ilvl** : affix ne peut spawner que si `alvl ≥ affix_level`

**Codage couleur :**
- Jaune dans les tables = disponible sur Magic ET Rare
- Bleu = Magic uniquement

### 3.2 Préfixes — Table complète

#### Catégorie : Dommages améliorés (Enhanced Damage)

| Nom préfixe | Effet | Range | ilvl min | Types |
|-------------|-------|-------|----------|-------|
| Sharp | +50-60% Enhanced Damage | 50-60% | 1 | Armes |
| Fine | +61-80% Enhanced Damage | 61-80% | 11 | Armes |
| Gleaming | +81-100% Enhanced Damage | 81-100% | 21 | Armes |
| Honed | +101-130% Enhanced Damage | 101-130% | 31 | Armes |
| Polished | +131-160% Enhanced Damage | 131-160% | 41 | Armes |
| Tempered | +161-190% Enhanced Damage | 161-190% | 51 | Armes |
| Razor-sharp | +191-220% Enhanced Damage | 191-220% | 61 | Armes |
| Keen | +221-250% Enhanced Damage | 221-250% | 71 | Armes |
| Ferocious | +251-300% Enhanced Damage | 251-300% | 81 | Armes |
| Cruel | +201-300% Enhanced Damage | 201-300% | 65 | Armes (Rare only) |

#### Catégorie : Attaque contre Démons / Morts-vivants

| Nom préfixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| Slaying | +50-100% Damage to Undead | 50-100% | 1 |
| Venomous | +100-150% Damage to Undead | 100-150% | 20 |
| Mephitic | +150-200% Damage to Undead | 150-200% | 40 |
| Baneful | +200-250% Damage to Undead | 200-250% | 60 |
| Pestilent | +100-150% Damage to Demons | 100-150% | 20 |
| Demonic | +150-200% Damage to Demons | 150-200% | 40 |

#### Catégorie : Défense améliorée (Enhanced Defense)

| Nom préfixe | Effet | Range | ilvl min | Types |
|-------------|-------|-------|----------|-------|
| Sturdy | +10-20% Enhanced Defense | 10-20% | 1 | Armures |
| Strong | +21-30% Enhanced Defense | 21-30% | 11 | Armures |
| Glorious | +31-40% Enhanced Defense | 31-40% | 21 | Armures |
| Blessed | +41-50% Enhanced Defense | 41-50% | 31 | Armures |
| Saintly | +51-65% Enhanced Defense | 51-65% | 41 | Armures |
| Holy | +66-80% Enhanced Defense | 66-80% | 51 | Armures |
| Godly | +81-100% Enhanced Defense | 81-100% | 61 | Armures |

#### Catégorie : Dommages élémentaires (Préfixes)

| Nom | Effet | Range | ilvl min |
|-----|-------|-------|----------|
| Smoky | +1-24 Fire Damage | 1-24 | 1 |
| Ember | +25-49 Fire Damage | 25-49 | 11 |
| Fiery | +50-99 Fire Damage | 50-99 | 21 |
| Smoking | +100-149 Fire Damage | 100-149 | 31 |
| Flaming | +150-199 Fire Damage | 150-199 | 41 |
| Burning | +200-299 Fire Damage | 200-299 | 55 |
| Blazing | +300-400 Fire Damage | 300-400 | 68 |
| Volcanic | +401-600 Fire Damage | 401-600 | 82 |
| Shivering | +1-24 Cold Damage | 1-24 | 1 |
| Boreal | +25-49 Cold Damage | 25-49 | 11 |
| Hibernal | +50-99 Cold Damage | 50-99 | 21 |
| Freezing | +100-149 Cold Damage | 100-149 | 31 |
| Glacial | +150-199 Cold Damage | 150-199 | 41 |
| Bitter | +200-299 Cold Damage | 200-299 | 55 |
| Frosted | +300-400 Cold Damage | 300-400 | 68 |
| Icy | +401-600 Cold Damage | 401-600 | 82 |
| Buzzing | +1-24 Lightning Damage | 1-24 | 1 |
| Arcing | +25-49 Lightning Damage | 25-49 | 11 |
| Shocking | +50-99 Lightning Damage | 50-99 | 21 |
| Electrical | +100-149 Lightning Damage | 100-149 | 31 |
| Charged | +150-199 Lightning Damage | 150-199 | 41 |
| Powered | +200-299 Lightning Damage | 200-299 | 55 |
| Grounding | +300-400 Lightning Damage | 300-400 | 68 |
| Zapping | +401-600 Lightning Damage | 401-600 | 82 |
| Septic | +6-24 Poison Dmg/2sec | 6-24 | 1 |
| Foul | +25-74 Poison Dmg/2sec | 25-74 | 11 |
| Toxic | +75-124 Poison Dmg/2sec | 75-124 | 21 |
| Pestilential | +125-174 Poison Dmg/2sec | 125-174 | 31 |
| Venomous | +175-249 Poison Dmg/2sec | 175-249 | 41 |
| Envenomed | +250-374 Poison Dmg/2sec | 250-374 | 55 |
| Noxious | +375-500 Poison Dmg/2sec | 375-500 | 68 |
| Deadly | +501-700 Poison Dmg/2sec | 501-700 | 82 |

#### Catégorie : Skills (Préfixes)

| Nom | Effet | Range | ilvl min | Types |
|-----|-------|-------|----------|-------|
| Acrobat's | +1 to Amazon Passive & Magic Skills | +1 | 30 | Arcs, Armes Amazon |
| Archer's | +1 to Amazon Bow and Crossbow Skills | +1 | 30 | Arcs, Armes Amazon |
| Avenger's | +1 to Paladin Offensive Auras | +1 | 30 | Armes Paladin |
| Entrapping | +1 to Assassin Traps | +1 | 30 | Katar |
| Shaman's | +1 to Druid Elemental Skills | +1 | 30 | Pelts |
| Necromancer's | +1 to Necromancer Summoning | +1 | 30 | Wands, Heads |
| Sorcerer's | +1 to Sorceress Elemental Skills | +1 | 30 | Orbs, Staves |
| Berserker's | +1 to Barbarian Warcries | +1 | 30 | Barb helms |

### 3.3 Suffixes — Table complète

#### Catégorie : Attributs

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of Strength | +1-3 Strength | 1-3 | 1 |
| of Might | +4-7 Strength | 4-7 | 9 |
| of the Ox | +8-13 Strength | 8-13 | 26 |
| of the Giant | +14-19 Strength | 14-19 | 45 |
| of the Titan | +20-26 Strength | 20-26 | 66 |
| of the Leviathan | +27-35 Strength | 27-35 | 83 |
| of Skill | +1-3 Dexterity | 1-3 | 1 |
| of Accuracy | +4-7 Dexterity | 4-7 | 9 |
| of Perfection | +8-13 Dexterity | 8-13 | 26 |
| of Precision | +14-19 Dexterity | 14-19 | 45 |
| of Certainty | +20-26 Dexterity | 20-26 | 66 |
| of Transcendence | +27-35 Dexterity | 27-35 | 83 |
| of Energy | +1-3 Energy | 1-3 | 1 |
| of the Mind | +4-7 Energy | 4-7 | 9 |
| of Brilliance | +8-13 Energy | 8-13 | 26 |
| of Sorcery | +14-19 Energy | 14-19 | 45 |
| of Wizardry | +20-26 Energy | 20-26 | 66 |
| of Zeal | +4-7 Vitality | 4-7 | 9 |
| of Life | +8-13 Vitality | 8-13 | 26 |
| of the Jackal | +1-5 Life | 1-5 | 1 |
| of the Fox | +6-10 Life | 6-10 | 6 |
| of the Wolf | +11-20 Life | 11-20 | 11 |
| of the Tiger | +21-30 Life | 21-30 | 16 |
| of the Mammoth | +31-40 Life | 31-40 | 21 |
| of the Colossus | +41-60 Life | 41-60 | 26 |
| of the Elephant | +61-80 Life | 61-80 | 31 |
| of the Whale | +81-100 Life | 81-100 | 36 |
| of Everlasting | +101-120 Life | 101-120 | 42 |
| of Life Everlasting | +121-150 Life | 121-150 | 50 |

#### Catégorie : Resistances

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of Flame Resistance | +5-10% Fire Res | 5-10% | 1 |
| of Fire Resistance | +11-20% Fire Res | 11-20% | 7 |
| of Blaze Resistance | +21-30% Fire Res | 21-30% | 14 |
| of Incineration | +31-40% Fire Res | 31-40% | 21 |
| of Frost Resistance | +5-10% Cold Res | 5-10% | 1 |
| of Cold Resistance | +11-20% Cold Res | 11-20% | 7 |
| of Ice Resistance | +21-30% Cold Res | 21-30% | 14 |
| of Glacial Resistance | +31-40% Cold Res | 31-40% | 21 |
| of Shock Resistance | +5-10% Ltng Res | 5-10% | 1 |
| of Lightning Resistance | +11-20% Ltng Res | 11-20% | 7 |
| of Thunder Resistance | +21-30% Ltng Res | 21-30% | 14 |
| of Storms | +31-40% Ltng Res | 31-40% | 21 |
| of Blight Resistance | +5-10% Poison Res | 5-10% | 1 |
| of Poison Resistance | +11-20% Poison Res | 11-20% | 7 |
| of Venom Resistance | +21-30% Poison Res | 21-30% | 14 |
| of Pestilence | +31-40% Poison Res | 31-40% | 21 |
| of Resist All | +4-7% All Res | 4-7% | 7 |
| of Balance | +8-12% All Res | 8-12% | 14 |
| of Equilibrium | +13-17% All Res | 13-17% | 21 |
| of Stability | +18-22% All Res | 18-22% | 28 |
| of Harmony | +23-27% All Res | 23-27% | 35 |
| of Purity | +28-32% All Res | 28-32% | 42 |
| of the Elements | +33-37% All Res | 33-37% | 50 |
| of the Arcane | +38-42% All Res | 38-42% | 60 |

#### Catégorie : Vitesse et Combat

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of Quickness | +10% Faster Attack Speed | 10% | 5 |
| of Swiftness | +20% Faster Attack Speed | 20% | 20 |
| of Alacrity | +30% Faster Attack Speed | 30% | 37 |
| of Speed | +40% Faster Attack Speed | 40% | 56 |
| of Fervor | +20% Faster Cast Rate | 20% | 5 |
| of the Apprentice | +10% Faster Cast Rate | 10% | 5 |
| of the Magus | +20% Faster Cast Rate | 20% | 25 |
| of the Sorcerer | +30% Faster Cast Rate | 30% | 45 |
| of the Wizard | +40% Faster Cast Rate | 40% | 65 |
| of Balance | +10% Faster Hit Recovery | 10% | 5 |
| of Stability | +20% Faster Hit Recovery | 20% | 20 |
| of Harmony | +30% Faster Hit Recovery | 30% | 40 |
| of Stability | +40% Faster Hit Recovery | 40% | 62 |
| of Blocking | +10% Faster Block Rate | 10% | 5 |
| of Deflecting | +20% Faster Block Rate | 20% | 25 |
| of Run/Walk | +10% Faster Run/Walk | 10% | 5 |
| of Acceleration | +20% Faster Run/Walk | 20% | 25 |
| of Speed | +30% Faster Run/Walk | 30% | 45 |

#### Catégorie : Life/Mana Leech

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of the Lamprey | 2-3% Life Stolen | 2-3% | 4 |
| of the Leech | 4-5% Life Stolen | 4-5% | 15 |
| of the Locust | 6-7% Life Stolen | 6-7% | 30 |
| of the Wraith | 8-9% Life Stolen | 8-9% | 47 |
| of the Vampire | 10-11% Life Stolen | 10-11% | 63 |
| of the Bat | 2-3% Mana Stolen | 2-3% | 4 |
| of the Spider | 4-5% Mana Stolen | 4-5% | 15 |

#### Catégorie : Magic Find / Gold Find

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of Luck | +5-10% Better Chance of MF | 5-10% | 1 |
| of Fortune | +11-15% Better Chance of MF | 11-15% | 12 |
| of Good Luck | +16-20% Better Chance of MF | 16-20% | 24 |
| of Chance | +21-25% Better Chance of MF | 21-25% | 38 |
| of Greed | +25-30% Better Chance of MF | 25-30% | 50 |
| of Wealth | +31-40% Better Chance of MF | 31-40% | 65 |
| of Prosperity | +21-30% Extra Gold | 21-30% | 1 |
| of Avarice | +31-50% Extra Gold | 31-50% | 10 |
| of Greed (Gold) | +51-70% Extra Gold | 51-70% | 22 |
| of Riches | +71-100% Extra Gold | 71-100% | 38 |
| of Plenty | +101-150% Extra Gold | 101-150% | 54 |
| of Treasure | +151-200% Extra Gold | 151-200% | 72 |

#### Catégorie : Dommages min/max

| Nom suffixe | Effet | Range | ilvl min |
|-------------|-------|-------|----------|
| of Craftsmanship | +1 Minimum Damage | 1 | 1 |
| of Quality | +2 Minimum Damage | 2 | 19 |
| of Maiming | +3 Minimum Damage | 3 | 38 |
| of Slaughter | +4-5 Minimum Damage | 4-5 | 57 |
| of Carnage | +6-9 Minimum Damage | 6-9 | 79 |
| of Worth | +2-4 Maximum Damage | 2-4 | 1 |
| of Measure | +5-10 Maximum Damage | 5-10 | 15 |
| of Excellence | +11-17 Maximum Damage | 11-17 | 30 |
| of Performance | +18-25 Maximum Damage | 18-25 | 49 |
| of Importance | +26-35 Maximum Damage | 26-35 | 69 |
| of Mastery | +36-49 Maximum Damage | 36-49 | 89 |


---

## 4. Les 33 Runes

### 4.1 Vue d'ensemble

Les runes sont des objets socketables qui ajoutent des modificateurs à l'item dans lequel elles sont insérées. Leur principal intérêt réside dans la formation de **Mots de Runes** (Runewords) en les combinant dans l'ordre exact dans un item à sockets multiples.

**Règles fondamentales :**
- L'ordre d'insertion des runes dans un runeword est strictement obligatoire
- Les runes seules (hors runeword) apportent leurs effets individuels
- Les runes ne peuvent PAS être insérées dans des items Magiques, Rares, Sets ou Uniques pour créer un Runeword (uniquement items normaux/gris/blancs)
- La rune Hel est la seule sans prérequis de niveau de personnage

### 4.2 Table complète des 33 runes

> Noms Sodomight proposés entre parenthèses.

| # | Nom D2 | Nom Sodomight | ilvl zone drop | Arme | Armure / Casque | Bouclier | Recette upgrade |
|---|--------|--------------|---------------|------|-----------------|----------|-----------------|
| 01 | El | Lux | 11 | +50 Attack Rating, +1 Light Radius | +15 Defense, +1 Light Radius | +15 Defense, +1 Light Radius | 3× El |
| 02 | Eld | Crep | 11 | +75% Dmg Undead, +50 AR Undead | 15% Slower Stamina Drain | +7% Chance to Block | 3× Eld |
| 03 | Tir | Sael | 13 | +2 Mana After Each Kill | +2 Mana After Each Kill | +2 Mana After Each Kill | 3× Tir |
| 04 | Nef | Bren | 13 | Knockback | +30 Defense vs. Missile | +30 Defense vs. Missile | 3× Nef |
| 05 | Eth | Vex | 15 | -25% Target Defense | Regenerate Mana 15% | Regenerate Mana 15% | 3× Eth |
| 06 | Ith | Dorn | 15 | +9 Max Damage | 15% Damage Taken Goes to Mana | 15% Damage Taken Goes to Mana | 3× Ith |
| 07 | Tal | Aes | 17 | +75 Poison Dmg over 5sec | Poison Resist +30% | Poison Resist +35% | 3× Tal |
| 08 | Ral | Fera | 19 | Add 5-30 Fire Dmg | Fire Resist +30% | Fire Resist +35% | 3× Ral |
| 09 | Ort | Cael | 21 | +1-50 Lightning Dmg | Lightning Resist +30% | Lightning Resist +35% | 3× Ort |
| 10 | Thul | Ysal | 23 | +3-14 Cold Dmg (3 sec) | Cold Resist +30% | Cold Resist +35% | 3× Thul + Chipped Topaz |
| 11 | Amn | Vor | 25 | 7% Life Stolen per Hit | Attacker Takes Dmg of 14 | Attacker Takes Dmg of 14 | 3× Amn + Chipped Amethyst |
| 12 | Sol | Keth | 27 | +9 Minimum Dmg | Damage Reduced by 7 | Damage Reduced by 7 | 3× Sol + Chipped Sapphire |
| 13 | Shael | Larn | 29 | +20% Increased Attack Speed | +20% Faster Hit Recovery | +20% Faster Block Rate | 3× Shael + Chipped Ruby |
| 14 | Dol | Mist | 31 | Hit Causes Monster to Flee 25% | Replenish Life +7 | Replenish Life +7 | 3× Dol + Chipped Emerald |
| 15 | Hel | Bhal | — | -20% Requirements | -15% Requirements | -15% Requirements | 3× Hel + Chipped Diamond |
| 16 | Io | Reth | 35 | +7 Vitality | +7 Vitality | +7 Vitality | 3× Io + Flawed Topaz |
| 17 | Lum | Sorn | 37 | +10 Energy | +10 Energy | +10 Energy | 3× Lum + Flawed Amethyst |
| 18 | Ko | Wael | 39 | +10 Dexterity | +10 Dexterity | +10 Dexterity | 3× Ko + Flawed Sapphire |
| 19 | Fal | Drael | 41 | +10 Strength | +10 Strength | +10 Strength | 3× Fal + Flawed Ruby |
| 20 | Lem | Orin | 43 | 75% Extra Gold from Monsters | 50% Extra Gold from Monsters | 50% Extra Gold from Monsters | 3× Lem + Flawed Emerald |
| 21 | Pul | Naer | 45 | +75% Damage to Demons, +100 AR Demons | +30% Enhanced Defense | +30% Enhanced Defense | 2× Pul + Flawed Diamond |
| 22 | Um | Gaul | 47 | 25% Chance of Open Wounds | Cold, Fire, Ltng, Poison Resist +15% | Cold, Fire, Ltng, Poison Resist +22% | 2× Um + Jewel |
| 23 | Mal | Thal | 49 | Prevent Monster Heal | Magic Dmg Reduced by 7 | Magic Dmg Reduced by 7 | 2× Mal + Perfect Amethyst |
| 24 | Ist | Odel | 51 | 30% Better Chance of MF | 25% Better Chance of MF | 25% Better Chance of MF | 2× Ist + Perfect Sapphire |
| 25 | Gul | Vaen | 53 | 20% Bonus to Attack Rating | +5% Max Poison Resist | +5% Max Poison Resist | 2× Gul + Perfect Ruby |
| 26 | Vex | Zael | 55 | 7% Mana Stolen per Hit | +5% Max Fire Resist | +5% Max Fire Resist | 2× Vex + Perfect Emerald |
| 27 | Ohm | Khor | 57 | +50% Enhanced Damage | +5% Max Cold Resist | +5% Max Cold Resist | 2× Ohm + Perfect Diamond |
| 28 | Lo | Mael | 59 | 20% Deadly Strike | +5% Max Lightning Resist | +5% Max Lightning Resist | 2× Lo + Flawless Topaz |
| 29 | Sur | Rhal | 61 | 20% Chance of Hit Blinding Target | +5% Max Mana | +50 Mana | 2× Sur + Flawless Amethyst |
| 30 | Ber | Naos | 63 | 20% Chance of Crushing Blow | Damage Reduced 8% | Damage Reduced 8% | 2× Ber + Flawless Sapphire |
| 31 | Jah | Ebal | 65 | Ignore Target's Defense | +5% Max Life | +5% Max Life | 2× Jah + Flawless Ruby |
| 32 | Cham | Thren | 67 | Freeze Target +3 | Cannot Be Frozen | Cannot Be Frozen | 2× Cham + Flawless Emerald |
| 33 | Zod | Aeon | 69 | Indestructible | Indestructible | Indestructible | Impossible (rune ultime) |

### 4.3 Raretés relatives

| Tier | Runes | Disponibilité |
|------|-------|--------------|
| Commun | El–Ort | Normal Act 1–3 |
| Peu commun | Thul–Lum | Normal Act 4–5 / Nightmare |
| Rare | Ko–Gul | Nightmare / Hell début |
| Très rare | Vex–Lo | Hell milieu/fin |
| Légendaire | Sur–Cham | Hell fin, farming intensif |
| Mythique | Zod | Extrait du tableau Rune TC87 uniquement |

---

## 5. Runewords — Mots de Runes

### 5.1 Règles de création

1. L'item doit être **Normal** (blanc/gris) — pas Magic, Rare, Set ou Unique
2. Le nombre de sockets doit être **exactement** celui requis par le runeword
3. Les runes doivent être insérées dans **l'ordre exact** indiqué
4. L'item doit être du **type correct** (arme, armure, casque, bouclier, etc.)
5. Certains runewords sont **Ladder only** (exclusif au mode en ligne classé)

### 5.2 Table des Runewords — 2 sockets

| Nom | Runes | Type | ilvl | Propriétés principales |
|-----|-------|------|------|------------------------|
| Ancient's Pledge | Ral+Ort+Tal | Bouclier 3S | 21 | +50% ED, Cold/Fire/Ltng/Poison Res +43-48%, 10% Dmg to Mana |
| Black | Thul+Io+Nef | Masses 1H | 35 | +120% ED, +200 AR, Adds 3-14 Cold Dmg, 40% Crushing Blow, Knockback, +10 Vit |
| Lore | Ort+Sol | Casque | 27 | +1 All Skills, +10 Energy, +2 Mana/Kill, Ltng Res +30%, Dmg Red 7 |
| Leaf | Tir+Ral | Baton 2S | 19 | +3 Fire Skills, +3 Inferno, +3 Warmth, +3 Fire Bolt, Adds 5-30 Fire, +33 Mana |
| Malice | Ith+El+Eth | Armes melee | 15 | +33% ED, Ignore Def, +200 AR, -100 Monster Def/Hit, Prevent Heal, Open Wounds 100% |
| Melody | Shael+Ko+Nef | Arcs | 39 | +50% ED Missile, +300% Dmg Undead, +3 Bow Skills, +3 Passive/Magic, Knockback |
| Memory | Lum+Io+Sol+Eth | Batons | 37 | +3 Sorc Elemental, +33% FCR, +9 Min Dmg, -25% Target Def, +9 Vit, +10 Energy |
| Nadir | Nef+Tir | Casque | 13 | +50% ED, +10 Def, +30 Def vs Missile, +5 Str, +2 Mana/Kill, -33% Gold Drop |
| Rhyme | Shael+Eth | Bouclier | 29 | +40% FBR, 20% Regen Mana, 15% Dmg to Mana, All Res +25%, Cannot Be Frozen, 50% Gold |
| Smoke | Nef+Lum | Armure | 37 | +75% ED, +280 Defense, Blinds, Slows 33%, All Res +50, 20% FCR |
| Splendor | Eth+Lum | Bouclier | 37 | +1 All Skills, +10% FCR, +20% FBR, +60-100% ED, +4 Mana/Kill, All Res +10%, +20% MF |
| Stealth | Tal+Eth | Armure | 17 | +25% FRW, +25% FCR, +25% FHR, +6 Dex, Regen Mana 15%, Poison Res +30% |
| Steel | Tir+El | Epees/Haches/Masses | 13 | +25% IAS, +20% ED, +3 Min, +3 Max, +50 AR, 50% Open Wounds, Repair Durability |
| Strength | Amn+Tir | Armes melee | 25 | +35% ED, 25% Crushing Blow, 7% Life Steal, +2 Mana/Kill, +20 Str |
| Venom | Tal+Dol+Mal | Armes | 49 | 312 Poison Dmg/sec, Prevent Heal, 7% Mana Steal, Lvl13 Poison Nova (11 chgs) |
| Wealth | Lem+Ko+Tir | Armure | 43 | +10% FCR, +2 Mana/Kill, +300% Extra Gold, 100% Better MF |
| White | Dol+Io | Baguette | 35 | +3 Bone/Tooth, +20% FCR, +2 Skel Mastery, +4 Skel Summon chgs, +2 Bone Spear |
| Zephyr | Ort+Eth | Arcs/Javelines | 21 | +25% FRW, +25% IAS, +33% ED, -8% Target Def, Twister (Tornado chgs) |

### 5.3 Table des Runewords — 3-4 sockets

| Nom | Runes | Type | ilvl | Propriétés principales |
|-----|-------|------|------|------------------------|
| Beast | Ber+Tir+Um+Mal+Lum | Haches/Masses/Sceptres | 63 | Level 9 Fanaticism Aura, +40% IAS, +240-270% ED, 20% Crushing Blow |
| Bone | Sol+Um+Um | Armure (Necro) | 47 | 15% Chance Bone Armor on Strike, 15% Chance Bone Spear, +2 Necro Skills, All Res +30 |
| Chains of Honor | Dol+Um+Ber+Ist | Armure 4S | 63 | +2 All Skills, +200% Dmg Demons, 8% Life Steal, +70% ED, All Res +65, Dmg Red 8%, 25% MF |
| Chaos | Fal+Ohm+Um | Griffes | 57 | +35% IAS, +290-340% ED, +1 Whirlwind, 25% Deadly Strike, Ignore Def, Frozen Orb chance |
| Crescent Moon | Shael+Um+Tir | Haches/Masses/Epees | 47 | +20% IAS, +180-220% ED, -35% Enemy Ltng Res, 25% Open Wounds, +2 Mana/Kill |
| Delirium | Lem+Ist+Io | Casque | 51 | +2 All Skills, +261 Defense, +10 Vit, 50% Extra Gold, 25% MF, Attract chgs |
| Doom | Hel+Ohm+Um+Lo+Pul | Haches/Masses/Lances | 67 | Level 12 Holy Freeze Aura, +2 All Skills, +45% IAS, +330-370% ED, -40-60% Enemy Cold Res |
| Dragon | Sur+Lo+Sol | Armure/Bouclier | 61 | +360 Def, Level 14 Holy Fire Aura, +3-5 All Skills, +5% Max Fire Res, +50 Str |
| Duress | Shael+Um+Thul | Armure | 47 | +40% FHR, +10-20% ED, 15% Crushing Blow, 33% Open Wounds, All Res +xx |
| Edge | Tir+Tal+Amn | Arcs | 25 | Level 15 Thorns Aura, +35% IAS, +320-380% Dmg Undead, 7% Life Steal |
| Enigma | Jah+Ith+Ber | Armure 3S | 65 | +2 All Skills, +45% FRW, +750-775% ED, Teleport Oskill, 45% MF, +Str per clvl |
| Eternity | Amn+Ber+Ist+Sol+Sur | Armes melee | 63 | Indestructible, +260-310% ED, 7% Life Steal, 20% Crushing Blow, Revive chgs |
| Exile | Vex+Ohm+Ist+Dol | Paladin Shield | 57 | Level 13-16 Defiance Aura, +2 Offensive Auras, +30% FBR, Freezes, +220-260% ED |
| Famine | Fal+Ohm+Ort+Jah | Haches/Masses | 65 | +30% IAS, +320-370% ED, Ignore Def, Add Magic+Fire+Ltng+Cold Dmg, 12% Life Steal |
| Fortitude | El+Sol+Dol+Lo | Armes/Armure | 59 | +25% FCR (arme), +200% ED (armure), +300% ED (arme), All Res +25-30, 20% DS (arme) |
| Grief | Eth+Tir+Lo+Mal+Ral | Epees/Haches 5S | 59 | +35% IAS, +30-40% Deadly Strike, Ignore Def, +340-400 flat Dmg, -25% Target Def |
| Hand of Justice | Sur+Cham+Amn | Armes | 67 | Level 16 Holy Fire Aura, +33% IAS, +280-330% ED, Ignore Def, -20% Enemy Fire Res |
| Harmony | Tir+Ith+Sol+Ko | Arcs | 39 | Level 10 Vigor Aura, +200-275% ED, +2-6 Valkyrie, Regen Mana 20% |
| Heart of the Oak | Ko+Vex+Pul+Thul | Batons/Masses | 55 | +3 All Skills, +40% FCR, +75% ED, All Res +30-40, Oak Sage chgs |
| Holy Thunder | Eth+Ral+Ort+Tal | Sceptres | 23 | +60% ED, -25% Target Def, Ltng Res +60%, +5 Max Ltng Res, Chain Ltng chgs |
| Infinity | Ber+Mal+Ber+Ist | Lances/Halbards (Ladder) | 63 | Level 12 Conviction Aura, +35% IAS, +255-325% ED, -45-55% Enemy Ltng Res |
| Insight | Ral+Tir+Tal+Sol | Lances/Halbards/Batons | 27 | Level 12-17 Meditation Aura, +35% FCR, +200-260% ED, +1-6 Critical Strike |
| King's Grace | Amn+Ral+Thul | Epees/Sceptres | 25 | +100% ED, +100% Dmg Demons, +100% Dmg Undead, 7% Life Steal |
| Kingslayer | Mal+Um+Gul+Fal | Epees/Haches | 53 | +30% IAS, +230-270% ED, 33% Crushing Blow, 50% Open Wounds |
| Last Wish | Jah+Mal+Jah+Sur+Jah+Ber | Epees/Masses/Haches 6S | 65 | Level 17 Might Aura, 60-70% Crushing Blow, +330-375% ED, Ignore Def |
| Lawbringer | Amn+Lem+Ko | Epees/Masses/Sceptres | 43 | Level 16-18 Sanctuary Aura, 20% Decrepify chance, -50% Slain Rest in Peace |
| Lionheart | Hel+Lum+Fal | Armure | 41 | +20% ED, +25 Str, +15 Dex, +20 Vit, +10 Energy, +50 Life, All Res +30% |
| Myth | Hel+Amn+Nef | Armure | 25 | +2 Barb Warcries, +30 Def vs Missile, Replenish +10, Attacker Takes Dmg 14 |
| Oath | Shael+Pul+Mal+Lum | Epees/Haches/Masses | 59 | Indestructible, +50% IAS, +210-340% ED, 30% Bone Spirit chance |
| Obedience | Hel+Ko+Thul+Eth+Fal | Lances/Halbards | 41 | +40% FHR, +370% ED, -25% Target Def, -25% Enemy Fire Res, 40% Crushing Blow |
| Passion | Dol+Ort+Eld+Lem | Armes | 43 | +25% IAS, +160-210% ED, +1 Berserk, +1 Zeal, 75% Extra Gold |
| Peace | Shael+Thul+Amn | Armure | 29 | +2 Amazon Passive, +3 Critical Strike, 2% Valkyrie chance, 7% Life Steal |
| Phoenix | Vex+Vex+Lo+Jah | Armes/Boucliers (Ladder) | 65 | Level 10-15 Redemption Aura, +350-400% ED, -28% Enemy Fire Res, 45% MF |
| Pride | Cham+Sur+Io+Lo | Lances/Halbards (Ladder) | 67 | Level 16-20 Concentration Aura, 260-300% Bonus AR, 20% Deadly Strike |
| Principle | Ral+Gul+Eld | Armure (Paladin) | 53 | +2 Paladin Combat Skills, +50% ED Undead, +100-150 Life |
| Rift | Hel+Ko+Ber+Pul | Lances/Sceptres (Ladder) | 53 | Tornado chance, Frozen Orb chance, +20% IAS, +5-10 All Attribs, 75% Dmg Demons |
| Sanctuary | Ko+Ko+Mal | Boucliers | 49 | +20% FBR, +20% FHR, +20% IAS, +130-160% ED Undead, All Res +50-70%, Slows 20% |
| Silence | Dol+Eld+Hel+Ist+Tir+Nef | Armes | 55 | +2 All Skills, +20% IAS, +20% FHR, 200% ED, +75 MF, All Res +75 |
| Spirit | Tal+Thul+Ort+Amn | Epees/Boucliers | 25 | +2 All Skills, +25-35% FCR, +55% FHR, +250 Def vs Missile, +22 Vit, +89-112 Mana |
| Stone | Shael+Um+Pul+Lum | Armure | 47 | +60% FHR, +250-290% ED, +300 Defense, +16 Str, +16 Vit, All Res +15% |
| Treachery | Shael+Thul+Lem | Armure | 43 | 5% Chance Fade, +45% IAS, +30% FHR, Cold Res +30%, 50% Extra Gold |
| Voice of Reason | Lem+Ko+El+Eld | Epees/Masses | 43 | +220-350% ED, +75-100 Cold Dmg, Freeze Target lvl4, -24% Enemy Cold Res |
| Wind | Sur+El | Armes melee | 61 | +10% FHR, +30% FRW, +15 Dex, 120-160% ED, -50% Target Def, Blinds, Tornado chance |
| Wrath | Pul+Lum+Ber+Mal | Arcs (Ladder) | 63 | +375% Dmg Undead, +85-120 Magic Dmg, +41-240 Ltng Dmg, 33% Deadly Strike |

### 5.4 Noms Sodomight des Runewords majeurs

| Nom D2 | Nom Sodomight | Runes |
|--------|--------------|-------|
| Enigma | Arcanum | Jah+Ith+Ber |
| Infinity | Abyssal Crown | Ber+Mal+Ber+Ist |
| Spirit | Echo | Tal+Thul+Ort+Amn |
| Grief | Wrath's Edge | Eth+Tir+Lo+Mal+Ral |
| Fortitude | Bastion | El+Sol+Dol+Lo |
| Call to Arms | War Hymn | Amn+Ral+Mal+Ist+Ohm |
| Insight | Oracle | Ral+Tir+Tal+Sol |
| Chains of Honor | Dominion | Dol+Um+Ber+Ist |
| Breath of the Dying | Death Breath | Vex+Hel+El+Eld+Zod+Eth |
| Heart of the Oak | Arborvitae | Ko+Vex+Pul+Thul |
| Infinity | Abyssal Crown | Ber+Mal+Ber+Ist |
| Last Wish | Crimson Pact | Jah+Mal+Jah+Sur+Jah+Ber |
| Phoenix | Ember Rebirth | Vex+Vex+Lo+Jah |

---

## 6. Set Items — Items de Set

### 6.1 Mécanisme des bonus de set

- Les bonus **partiels** s'accumulent en portant plusieurs pièces du même set
- Le bonus **complet** s'ajoute aux bonus partiels (ils se cumulent tous)
- Les stats vertes sur un item = stats liées au set (actives selon le nombre de pièces portées)
- Les stats dorées = stats fixes de la pièce individuelle

### 6.2 Sets Normaux (16 sets)

#### Angelic Raiment (Sodomight : Celestial Vestments)

| Piece | Base | Stats individuelles |
|-------|------|---------------------|
| Angelic Sickle | Sabre | +50% ED, +75 AR, 50% Dmg Undead, 7% Life Steal |
| Angelic Mantle | Ring Mail | +100 Defense, Lightning Res +20%, Fire Res +25% |
| Angelic Halo | Ring | Replenish Life +6, +20 Life |
| Angelic Wings | Light Gauntlets | +3 Life/Kill, +25 AR, +50 Mana |

**Bonus partiels :**
- 2 pieces : +50 Life
- 3 pieces : 1 Light Radius, Half Freeze Duration, Attack Rating +75%
- 4 pieces (complet) : +2 All Skills

#### Arcanna's Tricks (Sodomight : Weaver's Enigma)

| Piece | Base | Stats |
|-------|------|-------|
| Arcanna's Deathwand | Wand | +1 Sorceress Skills, +10% FCR, Slain Rest in Peace |
| Arcanna's Head | Skull Cap | +5% FCR, +25 Mana, Magic Dmg Reduced by 3 |
| Arcanna's Flesh | Light Plate | +10 Energy, Lightning Res +30%, Fire Res +25% |
| Arcanna's Sign | Amulet | Attacker Takes Dmg 8, Replenish Life +4, +20 Mana |

**Bonus partiels :**
- 2 pieces : +50 Mana
- 3 pieces : 5% Mana Steal
- 4 pieces (complet) : +2 Sorceress Skills

#### Berserker's Arsenal (Sodomight : Ravager's Fury)

| Piece | Base | Stats |
|-------|------|-------|
| Berserker's Headgear | Cap | +30% ED, +2 Barbarian Skills |
| Berserker's Hauberk | Splint Mail | +100 Defense, All Res +10% |
| Berserker's Hatchet | Double Axe | +50% ED, +1 Whirlwind |

**Bonus complet :** +3 Barbarian Skills, Replenish Life +25, Half Freeze Duration

#### Cathan's Traps (Sodomight : Shadowbane Relics)

| Piece | Base | Stats |
|-------|------|-------|
| Cathan's Rule | Battle Staff | +1 Fire Skills, +50% ED, 10% FCR |
| Cathan's Seal | Ring | Damage Reduced by 2, Attacker Takes Dmg 5, 6% Life Steal |
| Cathan's Sigil | Amulet | Attacker Takes Dmg 8, Half Freeze Duration |
| Cathan's Visage | Death Mask | +25 Mana, 4% Life Steal, Lightning Res +25% |
| Cathan's Mesh | Chain Mail | +50 Defense, Magic Dmg Reduced by 2, Cold Res +25% |

**Bonus complet :** 10% FCR, +2 Fire Skills

#### Civerb's Vestments (Sodomight : Ironwood Regalia)

| Piece | Base | Stats |
|-------|------|-------|
| Civerb's Cudgel | Grand Scepter | +150% ED vs. Undead, +1 to Holy Bolt charges |
| Civerb's Icon | Amulet | +10 Energy, Replenish Life +4 |
| Civerb's Ward | Large Shield | +15 Defense, +5 Energy, Cold Res +20%, Poison Res +20% |

**Bonus complet :** +35% ED

#### Cleglaw's Brace (Sodomight : Ironjaw Set)

| Piece | Base | Stats |
|-------|------|-------|
| Cleglaw's Tooth | Short Sword | 50% Deadly Strike, 30% Open Wounds, 35% Chance CB |
| Cleglaw's Claw | Small Shield | +5 Dex, +10 Life, All Res +20% |
| Cleglaw's Pincers | Chain Gloves | Slows 25%, +10 Str, Magic Dmg Reduced 2 |

**Bonus complet :** +2 Kick, Knockback

#### Death's Disguise (Sodomight : Grimveil Set)

| Piece | Base | Stats |
|-------|------|-------|
| Death's Hand | Leather Gloves | Poison Res +50%, Half Freeze Duration |
| Death's Guard | Sash | All Res +15%, Cannot Be Frozen |
| Death's Touch | War Sword | 25% Deadly Strike, 8% Life Steal |

**Bonus complet :** +3 Min Dmg, Replenish Life +10

#### Hsaru's Defense (Sodomight : Warden's Bulwark)

| Piece | Base | Stats |
|-------|------|-------|
| Hsaru's Iron Heel | Chain Boots | +20 Life, +10% FRW, Cold Res +25% |
| Hsaru's Iron Stay | Belt | +25 Life, +10 Str |
| Hsaru's Iron Fist | Buckler | +30 Defense, +10 Dex |

**Bonus complet :** +2 All Skills, +80 AR, +10% FRW

#### Infernal Tools (Sodomight : Hellbound Implements)

| Piece | Base | Stats |
|-------|------|-------|
| Infernal Cranium | Cap | +5 Energy, Attacker Takes Dmg 8, Regen Mana 15% |
| Infernal Torch | Gnarled Staff | +1 Necro Skills, +10% FCR, Attacker Takes Dmg 8 |
| Infernal Sign | Belt | Replenish Life +5, Magic Dmg Reduced 2 |

**Bonus complet :** +40 Life, Half Freeze Duration

#### Isenhart's Armory (Sodomight : Frostguard Armory)

| Piece | Base | Stats |
|-------|------|-------|
| Isenhart's Lightbrand | Broad Sword | +30% IAS, +35 AR, Adds 5-10 Cold Dmg |
| Isenhart's Parry | Gothic Shield | +24 Defense, +30% FBR |
| Isenhart's Case | Breast Plate | +40 Defense, All Res +20% |
| Isenhart's Horns | Full Helm | +10 Dex, Damage Reduced 2 |

**Bonus complet :** 50% Deadly Strike, +35 Life

#### Milabrega's Regalia (Sodomight : Luminary Regalia)

| Piece | Base | Stats |
|-------|------|-------|
| Milabrega's Orb | Round Shield (Pal) | +50% ED Undead, +100 AR Undead |
| Milabrega's Robe | Ancient Armor | +150 Defense, Regen Mana 15% |
| Milabrega's Rod | War Scepter | +2 Paladin Skills, +50% ED Undead |
| Milabrega's Diadem | Crown | +1 Light Radius, Cold Res +25% |

**Bonus complet :** +2 Paladin Combat Skills, Heal 15%

#### Sigon's Complete Steel (Sodomight : Ironclad Panoply)

| Piece | Base | Stats |
|-------|------|-------|
| Sigon's Gage | Gauntlets | +20 IAS, +20 Str |
| Sigon's Wrap | Plated Belt | +20 Life, Lightning Res +20% |
| Sigon's Sabot | Greaves | +40% FRW, Cold Res +40% |
| Sigon's Visor | Great Helm | +30 AR, +3 Light Radius |
| Sigon's Guard | Tower Shield | +25 Defense, +30% FBR |
| Sigon's Shelter | Gothic Plate | +170 Defense, All Res +25% |

**Bonus complet :** +2 All Skills, +100 Life, +20 AR, Attacker Takes Dmg 18

#### Tancred's Battlegear (Sodomight : Soulreaper's Battlegear)

| Piece | Base | Stats |
|-------|------|-------|
| Tancred's Crowbill | Military Pick | +80% ED, +75 AR |
| Tancred's Spine | Full Plate Mail | +40 Defense, Slows 35% |
| Tancred's Hobnails | Boots | +25 Life, +10% FRW |
| Tancred's Weird | Amulet | +75 Mana, All Res +10% |
| Tancred's Skull | Bone Helm | +40 Life, Attacker Takes Dmg 20, 10% MF |

**Bonus complet :** 78% MF, +100 AR

#### Vidala's Rig (Sodomight : Windrunner's Rig)

| Piece | Base | Stats |
|-------|------|-------|
| Vidala's Barb | Long Battle Bow | +75% ED Missile, +75 AR, Adds Ltng Dmg |
| Vidala's Fetlock | Light Plated Boots | +30% FRW, +15 Max Stamina |
| Vidala's Ambush | Leather Armor | +11 Dex, Cold Res +35% |
| Vidala's Snare | Amulet | +20 Life, +5 Dex, Lightning Res +20% |

**Bonus complet :** +2 Amazon Skills, Walk/Run Unlimited Stamina

### 6.3 Sets LoD — Normaux/Exceptionnels/Elites (18 sets)

#### Aldur's Watchtower (Sodomight : Stormcaller's Watchtower) — Druide

| Piece | Base | Stats individuelles |
|-------|------|---------------------|
| Aldur's Stony Gaze | Hunter's Guise | +15% FCR, +90 Mana, Cold Res +55% |
| Aldur's Deception | Shadow Plate | +300 Defense, +15 Str, Lightning Res +40% |
| Aldur's Rhythm | Jagged Star | +50% IAS, +200-250% ED, 6% Life Steal, +50 AR |
| Aldur's Advance | Battle Boots | +50 Life, +40% FRW, Fire Res +40% |

**Bonus complet :** +2 Druid Skills, +15 Dex, +10 Energy, +50 Life, Fire Res +50%

#### Bul-Kathos' Children (Sodomight : Titan's Lineage) — Barbare

| Piece | Base | Stats |
|-------|------|-------|
| Bul-Kathos' Sacred Charge | Colossus Blade | +200% ED, +50 Max Dmg, 20% IAS, 35% CB |
| Bul-Kathos' Tribal Guardian | Mythical Sword | +150% ED, 50% Poison Dmg, +150 AR, 30% DS, +2 Barb |

**Bonus complet :** +2 All Skills, +200 Life

#### Cow King's Leathers (Sodomight : Bovine King's Leathers)

| Piece | Base | Stats |
|-------|------|-------|
| Cow King's Horns | War Hat | +25% MF, +15 Str, Half Freeze |
| Cow King's Hide | Studded Leather | +100 Defense, +20 Vit, All Res +30% |
| Cow King's Hooves | Heavy Boots | +30 Life, +20% FRW, +25% MF, +15 Dex |

**Bonus complet :** +100 AR, 15% CB, +20 Life

#### The Disciple (Sodomight : The Devoted)

| Piece | Base | Stats |
|-------|------|-------|
| Dark Adherent | Dusk Shroud | +60-150% ED, Poison Res +24%, Fire Res +24% |
| Telling of Beads | Amulet | +1 All Skills, All Res +15% |
| Credendum | Mithril Coil | +25 Str, All Res +15% |
| Laying of Hands | Bramble Mitts | +20% IAS, +350% Dmg Demons, Fire Res +50% |
| Rite of Passage | Demonhide Boots | +50% FRW, Half Freeze |

**Bonus complet :** +2 All Skills, Poison Res +50%, All Res +50%

#### Griswold's Legacy (Sodomight : Hammerfall Legacy) — Paladin

| Piece | Base | Stats |
|-------|------|-------|
| Griswold's Valor | Corona | +30% FBR, +1-148 Ltng Dmg, All Res +30-50%, Socket |
| Griswold's Heart | Ornate Plate | +400 Defense, +30% FHR, Cannot Be Frozen |
| Griswold's Redemption | Caduceus | +2 Paladin Skills, 40% IAS, +220-240% ED |
| Griswold's Honor | Vortex Shield | +20% FBR, +70-100% ED, +2-3 Combat Skills |

**Bonus complet :** +3 Paladin Skills, +100% MF

#### Guillaume's Face (Sodomight : Bloodfang Warrior Set) — Barbare

Unique pièce seule (Winged Helm) — souvent utilisé en solo hors set.

#### Hwanin's Majesty (Sodomight : Stormcrown's Majesty)

| Piece | Base | Stats |
|-------|------|-------|
| Hwanin's Justice | Bill | +50% IAS, +200% ED, Adds Ltng Dmg |
| Hwanin's Refuge | Tigulated Mail | +200 Defense, Ltng Res +20%, Cold Res +15% |
| Hwanin's Splendor | Grand Crown | +1-99 Ltng Dmg, +88 Mana |
| Hwanin's Seal | Belt | +30 Life, Replenish +5, Poison Dmg +76/5sec |

**Bonus complet :** +3 All Skills, +1-499 Ltng Dmg (1-5/clvl)

#### Immortal King (Sodomight : Eternal Sovereign) — Barbare

| Piece | Base | Stats |
|-------|------|-------|
| Immortal King's Will | Avenger Guard | +150 AR, +37 Str, +2 Barb Combat |
| Immortal King's Soul Cage | Sacred Armor | +500 Defense, All Res +40%, Sockets |
| Immortal King's Detail | War Belt | +25 Str, +36-40% FHR |
| Immortal King's Forge | War Gauntlets | +65 AR, +20 Str, +20 Dex |
| Immortal King's Pillar | War Boots | +40% FRW, +44-46% FRW bonus |
| Immortal King's Stone Crusher | Ogre Maul | 40% IAS, +200-250% ED, 50% CB, +200 AR |

**Bonus complet :** +2 All Skills, +50 Defense, +75 AR, 4% Life Steal, +50 Mana, 50% MF

#### M'avina's Battle Hymn (Sodomight : Moonshard Hymn) — Amazone

| Piece | Base | Stats |
|-------|------|-------|
| M'avina's True Sight | Diamond Bow | +30% IAS, +188-240% ED, +50 AR, Cannot Be Frozen |
| M'avina's Tenet | Sharkskin Belt | +25 Dex, Cold Res +25%, 10% MF |
| M'avina's Embrace | Kraken Shell | +100% ED, +350 Defense, Cold Res +30% |
| M'avina's Caster | Grand Matron Bow | +30% IAS, Adds Cold Dmg, +1-2 Amazon Bow Skills |
| M'avina's Icy Clutch | Battle Gauntlets | +16-20 Cold Dmg, +30 Str, +30 Dex, +10% FBR |

**Bonus complet :** +2 Amazon Skills, Freezes Target, Regenerate Mana 10%

#### Natalya's Odium (Sodomight : Shadowstrike's Odium) — Assassine

| Piece | Base | Stats |
|-------|------|-------|
| Natalya's Totem | Grim Helm | +30% FHR, All Res +25-30%, +2 Assassin Skills |
| Natalya's Mark | Scissors Suwayyah | +200-250% ED, 30% DS, -20% Enemy Cold/Ltng Res |
| Natalya's Soul | Mesh Boots | +40 Life, +25% FRW, Cold Res +25%, Lightning Res +25% |
| Natalya's Shadow | Loricated Mail | +150 Defense, +2 Assassin Skills, Poison Res +40% |

**Bonus complet :** +3 Assassin Skills, All Res +35%

#### Orphan's Call (Sodomight : Orphan's Legacy)

| Piece | Base | Stats |
|-------|------|-------|
| Guillaume's Face | Winged Helm | 35% DS, 15% CB, +15 Str |
| Wilhelm's Pride | Battle Belt | 4% Life Steal, 2% Mana Steal |
| Magnus' Skin | Sharkskin Gloves | +20% IAS, 10% Life Steal |
| Whitstan's Guard | Round Shield | +55% FBR, +50% ED vs Undead |

**Bonus complet :** +2 All Skills, All Res +30%

#### Sander's Folly (Sodomight : Fool's Legacy)

| Piece | Base | Stats |
|-------|------|-------|
| Sander's Riprap | Heavy Boots | +40 Life, +10% FRW, +10 Str |
| Sander's Taboo | Heavy Gloves | +6% IAS, +20 Mana |
| Sander's Superstition | Bone Wand | +75% ED, 25% DS, +10 Dex, +100 AR |
| Sander's Paragon | Skull Cap | +5% MF, +1 Light, +20 Mana |

**Bonus complet :** +3 All Skills

#### Tal Rasha's Wrappings (Sodomight : Wraithweave Vestments) — Sorciere

| Piece | Base | Stats |
|-------|------|-------|
| Tal Rasha's Lidless Eye | Swirling Crystal | +1-2 Sorc Skills, +33% FCR, +77 Mana |
| Tal Rasha's Guardianship | Lacquered Plate | All Res +65-68%, 88% MF, +30% FHR |
| Tal Rasha's Horadric Crest | Death Mask | 10% Life Steal, 10% Mana Steal, +45 Life, +30 Mana |
| Tal Rasha's Fine-Spun Cloth | Mesh Belt | +20 Dex, +30 Mana, 10% Dmg to Mana |
| Tal Rasha's Adjudication | Amulet | +2 Sorc Skills, +57 Life, +50 Mana, Ltng Res +33% |

**Bonus complet :** +3 Sorc Skills, +150 Mana, +50 Life, +5% Max Cold/Ltng/Fire Res

#### Trang-Oul's Avatar (Sodomight : Bloodlord's Avatar) — Nécromancien

| Piece | Base | Stats |
|-------|------|-------|
| Trang-Oul's Guise | Bone Visage | 8% Life Steal, +25 Str, +2 All Skills |
| Trang-Oul's Scales | Chaos Armor | +150 Defense, Poison Res +40%, All Res +30% |
| Trang-Oul's Wing | Cantor Trophy (Head) | +25% Blocked, +20% FBR, Cold Res +40%, +3 Bone Prison |
| Trang-Oul's Claws | Heavy Bracers | 20% FCR, +2 Nécro Curses, +30 Mana, Cold Res +40% |
| Trang-Oul's Girth | Troll Belt | +66 Life, Replenish +15, Poison Res +40%, Cannot Be Frozen |

**Bonus complet :** +3 Necro Skills, +150 Mana, +200% ED (Vampire forme si 3+ pièces)
> **Note** : Avec 3-4 pièces le Nécro se transforme en Vampire et accède à Meteor, Fireball, Firewall

---

## 7. Unique Items — Items Uniques Majeurs

### 7.1 Principe

Les Uniques ont des stats **fixes prédéfinies** (sauf ranges indiquées). Ils sont build-defining pour la plupart des builds endgame.

### 7.2 Casques

#### Harlequin Crest — Shako (Sodomight : Mirth's Crown)

| Propriete | Valeur |
|-----------|--------|
| Base | Shako (War Hat elite) |
| +2 All Skills | fixe |
| +1-148 Life (0.75 per clvl) | variable |
| +1-148 Mana (0.75 per clvl) | variable |
| Damage Reduced by 10% | fixe |
| 50% Better Chance of MF | fixe |
| +2 All Attributes | fixe |
| ilvl drop min | 76 |

**Builds :** Toutes les classes, indispensable pour MF/farming

#### Vampire Gaze (Sodomight : Sanguine Gaze)

| Propriete | Valeur |
|-----------|--------|
| Base | Grim Helm |
| Adds 6-22 Cold Damage | fixe |
| 8% Mana Stolen per Hit | fixe |
| 6-8% Life Stolen per Hit | range |
| 15-20% Slower Stamina Drain | range |
| Damage Reduced by 15-20% | range |
| Magic Damage Reduced by 12-15 | range |
| ilvl drop min | 41 |

#### Crown of Ages (Sodomight : Ageless Crown)

| Propriete | Valeur |
|-----------|--------|
| Base | Corona |
| +50-100% ED | range |
| +100-150 Defense | range |
| All Resistances +20-30% | range |
| Damage Reduced by 10-15% | range |
| +1-2 Sockets | sockets |
| Cannot Be Frozen | fixe |
| ilvl drop min | 82 |

### 7.3 Armures

#### Chains of Honor (Runeword, voir section 5)

#### Fortitude (Runeword, voir section 5)

#### Enigma (Runeword, voir section 5)

#### Skin of the Vipermagi (Sodomight : Serpent's Mantle)

| Propriete | Valeur |
|-----------|--------|
| Base | Serpentskin Armor |
| +120% Enhanced Defense | fixe |
| +1 All Skills | fixe |
| +20-35% Faster Cast Rate | range |
| Magic Damage Reduced by 9-11 | range |
| All Resistances +20-35% | range |
| ilvl drop min | 29 |

#### Skullder's Ire (Sodomight : Bonehoard Ire)

| Propriete | Valeur |
|-----------|--------|
| Base | Russet Armor |
| +1 All Skills | fixe |
| 0.5% MF per clvl (max ~49) | per level |
| Repairs durability 1/20 | fixe |
| ilvl drop min | 46 |

#### Skin of Flayed One (Sodomight : Flayed Carapace)

| Propriete | Valeur |
|-----------|--------|
| Base | Demonhide Armor |
| Replenish Life +20 | fixe |
| +2 to Amazon Skills | fixe |
| +10% Life Stolen per Hit | fixe |
| Fire Resist +30% | fixe |

### 7.4 Ceintures

#### Arachnid Mesh (Sodomight : Silkweb Girdle)

| Propriete | Valeur |
|-----------|--------|
| Base | Spiderweb Sash (elite) |
| +1 All Skills | fixe |
| +20% Faster Cast Rate | fixe |
| Slows Target by 10% | fixe |
| +5% Max Mana | fixe |
| Level 3 Venom (11 charges) | fixe |
| ilvl drop min | 80 |

**Builds :** Toutes les classes casters

#### Thundergod's Vigor (Sodomight : Stormforged Vigor)

| Propriete | Valeur |
|-----------|--------|
| Base | War Belt |
| +1-50 Lightning Damage | variable |
| 5% Chance to Cast Level 7 Fist of Heavens on Striking | fixe |
| +20 Strength | fixe |
| +20 Vitality | fixe |
| +3-5 Lightning Fury | range |
| +3-5 Lightning Strike | range |
| Lightning Absorb 20% | fixe |
| +10% Max Lightning Resist | fixe |
| ilvl drop min | 65 |

### 7.5 Bottes

#### Waterwalk (Sodomight : Tidesurge Boots)

| Propriete | Valeur |
|-----------|--------|
| Base | Sharkskin Boots |
| 100 Max Stamina | fixe |
| 5% Chance Dodge | fixe |
| +15 Dexterity | fixe |
| +40-50 Life | range |
| +15% Faster Run/Walk | fixe |
| Fire Resist +20% | fixe |

#### War Traveler (Sodomight : Wanderer's Pilgrim)

| Propriete | Valeur |
|-----------|--------|
| Base | Battle Boots |
| +25% Faster Run/Walk | fixe |
| +10 Vitality | fixe |
| +10 Strength | fixe |
| 40-50% Better Chance of MF | range |
| Adds 15-25 Damage | range |
| Attacker Takes Damage of 5-10 | range |
| ilvl drop min | 47 |

### 7.6 Gants

#### Magefist (Sodomight : Emberweave Gloves)

| Propriete | Valeur |
|-----------|--------|
| Base | Light Gauntlets |
| +1 Fire Skills | fixe |
| +20% Faster Cast Rate | fixe |
| +Fire Bolt charges | charges |
| Regenerate Mana 25% | fixe |
| ilvl drop min | 23 |

#### Laying of Hands (Set piece — voir Disciple)

#### Dracul's Grasp (Sodomight : Wyrmfist Grasp)

| Propriete | Valeur |
|-----------|--------|
| Base | Vampirebone Gloves |
| +90-120% Enhanced Defense | range |
| +10-15 Strength | range |
| 25% Chance to Cast Level 15 Life Tap on Striking | fixe |
| 5% Life Stolen per Hit | fixe |
| +5-10 Life After Each Kill | range |
| ilvl drop min | 71 |

### 7.7 Boucliers

#### Stormshield (Sodomight : Tempest Shield)

| Propriete | Valeur |
|-----------|--------|
| Base | Monarch (Bone Shield) |
| 35% Chance of Blocking | fixe |
| +3-5 Strength | range |
| Cold Resist +60% | fixe |
| Lightning Resist +25% | fixe |
| Damage Reduced by 35% | fixe |
| +25% Faster Block Rate | fixe |
| Indestructible | fixe |
| ilvl drop min | 73 |

#### Lidless Wall (Sodomight : Eyeless Ward)

| Propriete | Valeur |
|-----------|--------|
| Base | Grim Shield |
| +1 All Skills | fixe |
| 20% FCR | fixe |
| +3-5 Mana After Kill | range |
| +10% Max Energy | fixe |
| +80-130 Mana | range |
| ilvl drop min | 41 |

### 7.8 Anneaux

#### Stone of Jordan (Sodomight : Eternity Sigil)

| Propriete | Valeur |
|-----------|--------|
| Base | Ring |
| +1 All Skills | fixe |
| Increase Max Mana 25% | fixe |
| +1-12 Lightning Damage | range |
| +20 Mana | fixe |
| ilvl drop min | 39 |

#### Raven Frost (Sodomight : Frostmark Band)

| Propriete | Valeur |
|-----------|--------|
| Base | Ring |
| +150-250 Attack Rating | range |
| Adds 15-45 Cold Damage (4 sec) | range |
| +15-20 Dexterity | range |
| +40 Mana | fixe |
| Cold Absorb 20% | fixe |
| Cannot Be Frozen | fixe |
| ilvl drop min | 45 |

#### Bul-Kathos' Wedding Band (Sodomight : Titan's Oath Ring)

| Propriete | Valeur |
|-----------|--------|
| Base | Ring |
| +1 All Skills | fixe |
| 3-5% Life Stolen per Hit | range |
| +0.5 Life per clvl | per level |
| +50 Max Stamina | fixe |
| ilvl drop min | 58 |

### 7.9 Amulettes

#### Mara's Kaleidoscope (Sodomight : Prism of Mara)

| Propriete | Valeur |
|-----------|--------|
| Base | Amulet |
| +2 All Skills | fixe |
| All Resistances +20-30% | range |
| +5 All Attributes | fixe |
| ilvl drop min | 67 |

**Builds :** Toutes les classes

#### The Rising Sun (Sodomight : Solstice Pendant)

| Propriete | Valeur |
|-----------|--------|
| Base | Amulet |
| +2 Fire Skills | fixe |
| 2% Chance to Cast Level 13 Meteor | fixe |
| +1-2 Absorb Fire | range |
| Adds 24-48 Fire Damage | range |
| Replenish Life +10 | fixe |
| +5% Max Fire Resist | fixe |

### 7.10 Armes uniques majeures

#### Windforce (Sodomight : Galestrike Bow)

| Propriete | Valeur |
|-----------|--------|
| Base | Hydra Bow |
| +30% IAS | fixe |
| +250% Enhanced Damage | fixe |
| Knocks Target Back | fixe |
| +10 Strength | fixe |
| 6-8% Life Stolen | range |
| ilvl drop min | 73 |

#### Grandfather (Sodomight : Patriarch's Blade)

| Propriete | Valeur |
|-----------|--------|
| Base | Colossus Blade |
| +150-200% Enhanced Damage | range |
| +35-50 Max Damage | range |
| +20 Strength | fixe |
| +20 Dexterity | fixe |
| +5 All Attributes | fixe |
| +50 Life | fixe |
| Deadly Strike (level/2)% | per level |
| ilvl drop min | 73 |

#### Oculus (Sodomight : Arcane Sphere)

| Propriete | Valeur |
|-----------|--------|
| Base | Swirling Crystal |
| +3 Sorceress Skills | fixe |
| 30% FCR | fixe |
| +20 All Attributes | fixe |
| All Resistances +20% | fixe |
| 50% Better Chance of MF | fixe |
| +20 Mana | fixe |
| ilvl drop min | 47 |

#### Herald of Zakarum (Sodomight : Sanctified Herald)

| Propriete | Valeur |
|-----------|--------|
| Base | Gilded Shield |
| +2 Paladin Combat Skills | fixe |
| +2 Offensive Auras | fixe |
| +130-160% Enhanced Defense | range |
| +20% Faster Block | fixe |
| 30% FBR | fixe |
| All Resistances +50-65% | range |
| +3-4 Combat Skills | range |
| ilvl drop min | 67 |

#### Homunculus (Sodomight : Homunculus Idol)

| Propriete | Valeur |
|-----------|--------|
| Base | Hierophant Trophy (Shrunken Head) |
| +2 Necromancer Skills | fixe |
| +40% Faster Block | fixe |
| +2-3 Summoning | range |
| +20% Bonus Blocking | fixe |
| All Resistances +40% | fixe |
| Replenish Life +10 | fixe |
| +150 Mana | fixe |
| ilvl drop min | 73 |

---

## 8. Cube Alchimique

> Sodomight renomme le Horadric Cube en **Cube Alchimique**.

### 8.1 Recettes de Gemmes

3 gemmes du meme type et qualite = 1 gemme du meme type de qualite superieure.

| Recette | Resultat |
|---------|---------|
| 3 Chipped Gem | 1 Flawed Gem (meme type) |
| 3 Flawed Gem | 1 Standard Gem (meme type) |
| 3 Standard Gem | 1 Flawless Gem (meme type) |
| 3 Flawless Gem | 1 Perfect Gem (meme type) |

### 8.2 Recettes de Runes (Upgrade)

| Input | Output |
|-------|--------|
| 3 El | 1 Eld |
| 3 Eld | 1 Tir |
| 3 Tir | 1 Nef |
| 3 Nef | 1 Eth |
| 3 Eth | 1 Ith |
| 3 Ith | 1 Tal |
| 3 Tal | 1 Ral |
| 3 Ral | 1 Ort |
| 3 Ort | 1 Thul |
| 3 Thul + Chipped Topaz | 1 Amn |
| 3 Amn + Chipped Amethyst | 1 Sol |
| 3 Sol + Chipped Sapphire | 1 Shael |
| 3 Shael + Chipped Ruby | 1 Dol |
| 3 Dol + Chipped Emerald | 1 Hel |
| 3 Hel + Chipped Diamond | 1 Io |
| 3 Io + Flawed Topaz | 1 Lum |
| 3 Lum + Flawed Amethyst | 1 Ko |
| 3 Ko + Flawed Sapphire | 1 Fal |
| 3 Fal + Flawed Ruby | 1 Lem |
| 3 Lem + Flawed Emerald | 1 Pul |
| 2 Pul + Flawed Diamond | 1 Um |
| 2 Um + Jewel | 1 Mal |
| 2 Mal + Perfect Amethyst | 1 Ist |
| 2 Ist + Perfect Sapphire | 1 Gul |
| 2 Gul + Perfect Ruby | 1 Vex |
| 2 Vex + Perfect Emerald | 1 Ohm |
| 2 Ohm + Perfect Diamond | 1 Lo |
| 2 Lo + Flawless Topaz | 1 Sur |
| 2 Sur + Flawless Amethyst | 1 Ber |
| 2 Ber + Flawless Sapphire | 1 Jah |
| 2 Jah + Flawless Ruby | 1 Cham |
| 2 Cham + Flawless Emerald | 1 Zod |

### 8.3 Recettes de Sockets

| Recette | Resultat | Notes |
|---------|---------|-------|
| Tal + Thul + Perfect Topaz + Normal Armor | Ajoute 1-4 sockets (armure) | Aleatoire, cap par item |
| Ral + Amn + Perfect Amethyst + Normal Weapon | Ajoute 1-6 sockets (arme) | Aleatoire |
| Tal + Amn + Perfect Ruby + Normal Helm | Ajoute 1-2 sockets (casque) | Aleatoire |
| Hel + Scroll of Town Portal + Normal Shield | Ajoute 1-4 sockets (bouclier) | Aleatoire |
| 3 Perfect Skull + 1 Rare Item + Stone of Jordan | Ajoute 1 socket (item Rare) | Ne fonctionne pas si deja socket |

> **Larzuk (Quete 1 Acte 5)** : Donne le MAXIMUM de sockets possibles pour l'ilvl de l'item (item normal), 1-2 pour magic, 1 pour rare/set/unique.

### 8.4 Upgrade de tier d'items

#### Normal to Exceptionnel

| Recette | Types |
|---------|-------|
| Tal + Shael + Perfect Topaz + Normal Unique Armor | Unique Armor Normal vers Exceptional |
| Ral + Sol + Perfect Emerald + Normal Rare Armor | Rare Armor Normal vers Exceptional |
| Amn + Shael + Perfect Ruby + Normal Unique Weapon | Unique Weapon Normal vers Exceptional |

#### Exceptionnel to Elite

| Recette | Types |
|---------|-------|
| Ko + Lem + Perfect Diamond + Exceptional Unique Armor | Unique Armor Exc vers Elite |
| Lem + Ko + Gul + Exceptional Rare Armor | Rare Armor Exc vers Elite |
| Pul + Fal + Perfect Emerald + Exceptional Unique Weapon | Unique Weapon Exc vers Elite |

> Note : Les items Crafted, Runewords et Sets ne peuvent pas etre upgrade.

### 8.5 Recettes de Crafting (4 types)

Ingredients communs a chaque craft : Magic item (slot cible) + Perfect Gem (type selon craft) + Jewel + Rune (selon slot et type)

#### Blood Crafting (life steal)

| Slot | Rune | Gem Parfaite | Effets garantis |
|------|------|-------------|-----------------|
| Helm | Nef | Ruby | 1-3% Life Stolen, +10-20 Life, 5-10% FHR |
| Armor | Ort | Ruby | 1-3% Life Stolen, +10-20 Life, 5-10% FHR |
| Gloves | Ral | Ruby | 1-3% Life Stolen, +10-20 Life, 10-20% IAS |
| Shield | Thul | Ruby | 1-3% Life Stolen, +10-20 Life, 5-10% FHR |
| Belt | Nef | Ruby | 1-3% Life Stolen, +10-20 Life, +5-10 Replenish |
| Boots | Ral | Ruby | 1-3% Life Stolen, +10-20 Life, 10-20% FRW |
| Amulet | Ort | Ruby | 1-3% Life Stolen, +10-20 Life, +5-10 Life/Kill |
| Ring | Sol | Ruby | 1-3% Life Stolen, +10-20 Life, +5-10 Life/Kill |
| Weapon | Ral | Ruby | 1-3% Life Stolen, +10-20 Life, +50-80 AR |

#### Caster Crafting (FCR, mana)

| Slot | Rune | Gem Parfaite | Effets garantis |
|------|------|-------------|-----------------|
| Helm | Thul | Amethyst | 4-10% Regen Mana, +10-20 Mana, +5-10% FCR |
| Armor | Ral | Amethyst | 4-10% Regen Mana, +10-20 Mana, +5-10% FCR |
| Gloves | Ort | Amethyst | 4-10% Regen Mana, +10-20 Mana, +10-20% FCR |
| Shield | Eth | Amethyst | 4-10% Regen Mana, +10-20 Mana, +5-10% FCR |
| Belt | Sol | Amethyst | 4-10% Regen Mana, +10-20 Mana, +5-10 Replenish |
| Boots | Nef | Amethyst | 4-10% Regen Mana, +10-20 Mana, +10-20% FRW |
| Amulet | Eth | Amethyst | 4-10% Regen Mana, +10-20 Mana, +1 Skills |
| Ring | Sol | Amethyst | 4-10% Regen Mana, +10-20 Mana, +10-20% FCR |
| Weapon | Tir | Amethyst | 4-10% Regen Mana, +10-20 Mana, +2-3 Mana/Kill |

#### Hit Power Crafting (defensive reactif)

| Slot | Rune | Gem Parfaite | Effets garantis |
|------|------|-------------|-----------------|
| Helm | Tir | Sapphire | 5% Chance Frost Nova (Lvl4), +Attacker Takes Dmg, +Defense |
| Armor | Ort | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg, +Defense |
| Gloves | Tir | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg, +IAS |
| Shield | Tal | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg, +FBR |
| Belt | Eth | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg |
| Boots | Tir | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg, +FRW |
| Amulet | Nef | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg |
| Ring | Ral | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg |
| Weapon | Ort | Sapphire | 5% Chance Frost Nova, +Attacker Takes Dmg, +AR |

#### Safety Crafting (damage reduction)

| Slot | Rune | Gem Parfaite | Effets garantis |
|------|------|-------------|-----------------|
| Helm | Nef | Emerald | +10-30% ED, Magic Dmg Red 1-2, Dmg Red 1-4 |
| Armor | Sol | Emerald | +10-30% ED, Magic Dmg Red 1-2, Dmg Red 1-4 |
| Gloves | Thul | Emerald | +10-30% ED, Magic Dmg Red 1-2 |
| Shield | Tal | Emerald | +10-30% ED, Magic Dmg Red 1-2, +FBR |
| Belt | Nef | Emerald | +10-30% ED, Magic Dmg Red 1-2 |
| Boots | Eth | Emerald | +10-30% ED, Magic Dmg Red 1-2, +FRW |
| Amulet | Sol | Emerald | +10-30% ED, Magic Dmg Red 1-2 |
| Ring | Tir | Emerald | +10-30% ED, Magic Dmg Red 1-2 |
| Weapon | Sol | Emerald | +10-30% ED, Magic Dmg Red 1-2 |

### 8.6 Reroll d'affixes

| Recette | Effet |
|---------|-------|
| 6 Perfect Skulls + 1 Rare Armor | Reroll tous les affixes (nouveau rare aleatoire, meme base) |
| 6 Perfect Skulls + 1 Rare Weapon | Reroll tous les affixes |

### 8.7 Recettes speciales

| Recette | Effet |
|---------|-------|
| Wirt's Leg + Tome of Town Portal | Ouvre portail Secret Cow Level |
| Hel + Scroll of Town Portal + Socketed Item | Retire toutes gems/runes sans detruire l'item |
| Tir + Perfect Ruby + Rechargeable Item | Recharge charges d'un item |
| Ral + Scroll of Town Portal + Any Item | Repare toute la durabilite |

---

## 9. Gemmes

> Sodomight : Les gemmes conservent leurs noms mais les noms de qualite sont adaptes.

### 9.1 Noms de qualite (Sodomight)

| D2 | Sodomight |
|----|-----------|
| Chipped | Brisee |
| Flawed | Imparfaite |
| Standard | Normale |
| Flawless | Parfaite |
| Perfect | Immaculee |

### 9.2 Table complete des effets par type et qualite

#### Rubis (Ruby) — Sodomight : Pierre de Feu

| Qualite | Arme (Fire Dmg) | Armure/Casque (Max Life) | Bouclier (Fire Res) |
|---------|----------------|--------------------------|---------------------|
| Chipped | Adds 3-4 Fire Dmg | +10 to Maximum Life | Fire Resist +12% |
| Flawed | Adds 5-8 Fire Dmg | +17 to Maximum Life | Fire Resist +16% |
| Standard | Adds 8-12 Fire Dmg | +24 to Maximum Life | Fire Resist +22% |
| Flawless | Adds 10-16 Fire Dmg | +31 to Maximum Life | Fire Resist +28% |
| Perfect | Adds 15-20 Fire Dmg | +38 to Maximum Life | Fire Resist +40% |

#### Saphir (Sapphire) — Sodomight : Pierre de Glace

| Qualite | Arme (Cold Dmg) | Armure/Casque (Max Mana) | Bouclier (Cold Res) |
|---------|----------------|--------------------------|---------------------|
| Chipped | Adds 1-3 Cold Dmg (1 sec) | +10 to Maximum Mana | Cold Resist +12% |
| Flawed | Adds 3-5 Cold Dmg (1.4 sec) | +17 to Maximum Mana | Cold Resist +16% |
| Standard | Adds 4-7 Cold Dmg (2 sec) | +24 to Maximum Mana | Cold Resist +22% |
| Flawless | Adds 6-10 Cold Dmg (2.4 sec) | +31 to Maximum Mana | Cold Resist +28% |
| Perfect | Adds 10-14 Cold Dmg (3 sec) | +38 to Maximum Mana | Cold Resist +40% |

#### Topaze (Topaz) — Sodomight : Pierre d'Eclair

| Qualite | Arme (Ltng Dmg) | Armure/Casque (MF) | Bouclier (Ltng Res) |
|---------|----------------|---------------------|---------------------|
| Chipped | Adds 1-8 Lightning Dmg | +9% Better MF | Lightning Resist +12% |
| Flawed | Adds 1-14 Lightning Dmg | +13% Better MF | Lightning Resist +16% |
| Standard | Adds 1-22 Lightning Dmg | +16% Better MF | Lightning Resist +22% |
| Flawless | Adds 1-30 Lightning Dmg | +20% Better MF | Lightning Resist +28% |
| Perfect | Adds 1-40 Lightning Dmg | +24% Better MF | Lightning Resist +40% |

#### Emeraude (Emerald) — Sodomight : Pierre de Poison

| Qualite | Arme (Poison Dmg) | Armure/Casque (Dexterity) | Bouclier (Poison Res) |
|---------|-------------------|---------------------------|----------------------|
| Chipped | +10 Poison Dmg over 3 sec | +3 to Dexterity | Poison Resist +12% |
| Flawed | +20 Poison Dmg over 4 sec | +4 to Dexterity | Poison Resist +16% |
| Standard | +40 Poison Dmg over 5 sec | +6 to Dexterity | Poison Resist +22% |
| Flawless | +60 Poison Dmg over 6 sec | +8 to Dexterity | Poison Resist +28% |
| Perfect | +100 Poison Dmg over 8 sec | +15 to Dexterity | Poison Resist +40% |

#### Diamant (Diamond) — Sodomight : Pierre de Lumiere

| Qualite | Arme (+Dmg Undead) | Armure/Casque (+AR) | Bouclier (All Res) |
|---------|-------------------|---------------------|---------------------|
| Chipped | +28% Dmg to Undead | +20 to AR | All Res +6% |
| Flawed | +34% Dmg to Undead | +40 to AR | All Res +8% |
| Standard | +44% Dmg to Undead | +60 to AR | All Res +11% |
| Flawless | +54% Dmg to Undead | +80 to AR | All Res +14% |
| Perfect | +68% Dmg to Undead | +100 to AR | All Res +19% |

#### Amethyste (Amethyst) — Sodomight : Pierre de Force

| Qualite | Arme (+AR) | Armure/Casque (+Str) | Bouclier (+Defense) |
|---------|------------|----------------------|---------------------|
| Chipped | +40 to AR | +3 to Strength | +8 Defense |
| Flawed | +60 to AR | +4 to Strength | +12 Defense |
| Standard | +80 to AR | +6 to Strength | +18 Defense |
| Flawless | +100 to AR | +8 to Strength | +24 Defense |
| Perfect | +150 to AR | +15 to Strength | +30 Defense |

#### Crane (Skull) — Sodomight : Pierre de Mort

| Qualite | Arme (Life+Mana Steal) | Armure/Casque (Regen) | Bouclier (Dmg Return) |
|---------|----------------------|----------------------|----------------------|
| Chipped | 2% Life Steal, 1% Mana Steal | Regenerate Life 2 | Attacker Takes 4 Dmg |
| Flawed | 2.5% Life Steal, 1% Mana Steal | Regenerate Life 3 | Attacker Takes 8 Dmg |
| Standard | 3% Life Steal, 2% Mana Steal | Regenerate Life 5 | Attacker Takes 12 Dmg |
| Flawless | 3.5% Life Steal, 2% Mana Steal | Regenerate Life 8 | Attacker Takes 16 Dmg |
| Perfect | 4% Life Steal, 3% Mana Steal | Regenerate Life 12 | Attacker Takes 20 Dmg |

---

## 10. Charmes

> Sodomight : Les charmes gardent leurs proprietes passives dans l'inventaire sans necessite d'equipement.

### 10.1 Tailles et mecanisme

| Taille | Espace | Nom D2 | Prefixe | Suffixe |
|--------|--------|--------|---------|---------|
| 1x1 | 1 case | Small Charm | Oui | Oui |
| 1x2 | 2 cases | Large Charm | Oui | Oui |
| 1x3 | 3 cases | Grand Charm | Oui | Oui |

**Distribution magique :** 50% suffix only, 25% prefix only, 25% les deux.

### 10.2 Affixes des Small Charms

| Affix | Effet | Range | ilvl min |
|-------|-------|-------|----------|
| Fire Damage | Adds X-Y Fire Dmg | 1-3 / 2-6 | 5 / 16 |
| Cold Damage | Adds X-Y Cold Dmg | 1-2 / 2-5 | 5 / 16 |
| Lightning Dmg | Adds X-Y Ltng Dmg | 1-7 / 2-15 | 5 / 16 |
| Poison Damage | Adds X Poison over Y sec | 25-75 | 10 |
| Attack Rating | +XX to AR | 5-20 | 5 |
| Max Damage | +1-3 to Max Damage | 1-3 | 5 |
| Life | +X to Life | 1-20 | 5 |
| Mana | +X to Mana | 1-17 | 5 |
| Stamina | +X to Stamina | 10-30 | 5 |
| Fire Resistance | +X% Fire Res | 3-11% | 5 |
| Cold Resistance | +X% Cold Res | 3-11% | 5 |
| Lightning Resistance | +X% Ltng Res | 3-11% | 5 |
| Poison Resistance | +X% Poison Res | 3-11% | 5 |
| All Resistances | +X% All Res | 2-5% | 5 |
| Magic Find | +X% Better MF | 3-7% | 5 |
| Gold Find | +X% Extra Gold | 10-30% | 5 |
| Faster Run/Walk | +X% FRW | 3-5% | 5 |

### 10.3 Affixes des Large Charms

| Affix | Effet | Range | ilvl min |
|-------|-------|-------|----------|
| Fire Resistance | +X% Fire Res | 6-18% | 5 |
| Cold Resistance | +X% Cold Res | 6-18% | 5 |
| Lightning Resistance | +X% Ltng Res | 6-18% | 5 |
| Poison Resistance | +X% Poison Res | 6-18% | 5 |
| All Resistances | +X% All Res | 4-8% | 5 |
| Life | +X to Life | 6-30 | 5 |
| Mana | +X to Mana | 6-24 | 5 |
| Attack Rating | +X to AR | 25-75 | 5 |
| Max Damage | +X to Max Damage | 2-6 | 5 |
| Magic Find | +X% Better MF | 5-11% | 5 |
| Gold Find | +X% Extra Gold | 25-60% | 5 |
| Faster Run/Walk | +X% FRW | 5-8% | 5 |

### 10.4 Affixes des Grand Charms

| Affix | Effet | Range | ilvl min |
|-------|-------|-------|----------|
| +1 Amazon Bow/Xbow Skills | Classe specifique | +1 | 50 |
| +1 Amazon Passive/Magic | Classe specifique | +1 | 50 |
| +1 Amazon Javelin/Spear | Classe specifique | +1 | 50 |
| +1 Barb Warcries | Classe specifique | +1 | 50 |
| +1 Barb Combat Skills | Classe specifique | +1 | 50 |
| +1 Barb Masteries | Classe specifique | +1 | 50 |
| +1 Necro Summoning | Classe specifique | +1 | 50 |
| +1 Necro Bone/Tooth | Classe specifique | +1 | 50 |
| +1 Necro Curses | Classe specifique | +1 | 50 |
| +1 Paladin Combat Skills | Classe specifique | +1 | 50 |
| +1 Paladin Offensive Auras | Classe specifique | +1 | 50 |
| +1 Paladin Defensive Auras | Classe specifique | +1 | 50 |
| +1 Sorc Fire Skills | Classe specifique | +1 | 50 |
| +1 Sorc Cold Skills | Classe specifique | +1 | 50 |
| +1 Sorc Lightning Skills | Classe specifique | +1 | 50 |
| +1 Druid Elemental Skills | Classe specifique | +1 | 50 |
| +1 Druid Shapeshifting | Classe specifique | +1 | 50 |
| +1 Druid Summoning | Classe specifique | +1 | 50 |
| +1 Assassin Traps | Classe specifique | +1 | 50 |
| +1 Assassin Shadow Disc. | Classe specifique | +1 | 50 |
| +1 Assassin Martial Arts | Classe specifique | +1 | 50 |
| Life | +X to Life | 10-45 | 5 |
| Mana | +X to Mana | 10-35 | 5 |
| Fire Resistance | +X% Fire Res | 10-25% | 5 |
| Cold Resistance | +X% Cold Res | 10-25% | 5 |
| Lightning Resistance | +X% Ltng Res | 10-25% | 5 |
| Poison Resistance | +X% Poison Res | 10-25% | 5 |
| All Resistances | +X% All Res | 5-10% | 5 |

### 10.5 Charmes Uniques

#### Gheed's Fortune (Sodomight : Vagabond's Fortune)

| Propriete | Range |
|-----------|-------|
| 80-160% Extra Gold from Monsters | range |
| Reduces all Vendor Prices 10-15% | range |
| 20-40% Better Chance of Getting Magic Items | range |

> 1 seul Gheed peut etre dans l'inventaire a la fois (si 2 sont dropp, un est ignore).

#### Annihilus (Sodomight : Nihilstone) — Small Charm

Droppable uniquement en tuant Uber Diablo (event rare en ligne).

| Propriete | Range |
|-----------|-------|
| +1 to All Skills | fixe |
| +10-20 to All Attributes | range |
| All Resistances +10-20% | range |
| 5-10% Better Chance of MF | range |

#### Hellfire Torch (Sodomight : Infernostone) — Large Charm

Droppable en tuant les Uber Bosses (Uber Tristram).

| Propriete | Range |
|-----------|-------|
| +3 to [Class-specific skill tree] | fixe (classe aleatoire) |
| +10-20 to All Attributes | range |
| All Resistances +10-20% | range |
| Level 30 Hydra (10 charges) | fixe |

---

## 11. Joyaux (Jewels)

### 11.1 Mecanisme

Les Jewels (Joyaux) sont socketables comme les runes et les gemmes mais offrent des affixes magiques/rares au lieu d'effets fixes.

- **Jewel Magique** : 1 prefix + 1 suffix
- **Jewel Rare** : 2-4 proprietes
- **Jewel Unique** : Rainbow Facet (stats fixes speciales)

### 11.2 Ce qu'un Jewel PEUT avoir

| Categorie | Examples |
|-----------|---------|
| Enhanced Damage | +40-100% Enhanced Damage |
| Min/Max Damage | +1-3 Min, +2-10 Max |
| IAS | +5-15% Increased Attack Speed |
| FCR | +5-10% Faster Cast Rate |
| Elemental Damage | Adds fire/cold/ltng/poison Dmg |
| Attack Rating | +10-75 AR |
| Life | +5-20 Life |
| Mana | +5-15 Mana |
| Resistances | Fire/Cold/Ltng/Poison Res +5-15% |
| All Resistances | All Res +3-7% |
| Magic Find | +1-5% MF |
| Defense | +10-40 Defense |
| Requirements | -10-15% Requirements |
| Damage to Demons/Undead | +10-50% |

### 11.3 Ce qu'un Jewel NE PEUT PAS avoir

- Life/Mana steal
- Faster Run/Walk
- Vitality, Energy bonuses directs
- +Skills ou +Class Skills
- Faster Hit Recovery (FHR)
- Faster Block Rate (FBR)

### 11.4 Jewel Uniques — Rainbow Facets (Sodomight : Prismatic Crystals)

Chaque Rainbow Facet existe en 4 versions elementaires et 2 sous-types (Level-Up / Die).

| Jewel | Effet Level-Up | Effet Die | Bonus passive |
|-------|---------------|-----------|--------------|
| Fire Facet (Level-Up) | 5% Chance Cast Lvl18 Blaze | — | -5% Enemy Fire Res, +5% Fire Skill Dmg |
| Fire Facet (Die) | — | 5% Chance Cast Lvl18 Meteor | -5% Enemy Fire Res, +5% Fire Skill Dmg |
| Cold Facet (Level-Up) | 5% Chance Cast Lvl15 Blizzard | — | -5% Enemy Cold Res, +5% Cold Skill Dmg |
| Cold Facet (Die) | — | 5% Chance Cast Lvl15 Glacial Spike | -5% Enemy Cold Res, +5% Cold Skill Dmg |
| Lightning Facet (Level-Up) | 5% Chance Cast Lvl15 Charged Bolt | — | -5% Enemy Ltng Res, +5% Ltng Skill Dmg |
| Lightning Facet (Die) | — | 5% Chance Cast Lvl25 Chain Lightning | -5% Enemy Ltng Res, +5% Ltng Skill Dmg |
| Poison Facet (Level-Up) | 5% Chance Cast Lvl15 Poison Nova | — | -5% Enemy Poison Res, +5% Poison Skill Dmg |
| Poison Facet (Die) | — | 5% Chance Cast Lvl15 Poison Nova | -5% Enemy Poison Res, +5% Poison Skill Dmg |

---

## 12. Systeme de Durabilite

### 12.1 Mecanisme general

- Chaque item (sauf missiles, Phase Blade, Indestructible) possede une **durabilite max** definie
- La durabilite diminue au combat et plus rapidement a la mort
- A 0 durabilite : l'item est **casse** (defenses et bonus annules, mais toujours en inventaire)
- La durabilite ne se regenere PAS seule (sauf Repair 1/20 sur certains items)

### 12.2 Perte de durabilite

| Evenement | Perte |
|-----------|-------|
| Combat normal (arme) | -1 durabilite / X attaques |
| Etre touche (armure/casque) | -1 durabilite parfois |
| Mort du personnage | -75 durabilite (items equipes) |
| Items magiques recement repares | Potentielle perte de quelques points max durabilite |

### 12.3 Reparation

| Methode | Disponibilite | Cout |
|---------|--------------|------|
| NPC Blacksmith (Charsi, Fara, etc.) | En ville chaque acte | En or (proportionnel a la valeur) |
| NPC Hratli (Acte 3) | Armures/Boucliers | En or |
| Ral + Scroll of Town Portal (Cube) | Partout | Rune + Scroll |
| Modifier "Repairs X Durability per Second" | Automatique | Aucun |

### 12.4 Indestructible

Un item avec le modificateur **Indestructible** n'a pas de durabilite et ne peut jamais etre casse. Il ne necessite aucune reparation.

Sources d'Indestructible :
- **Rune Zod** (r33) socketee dans l'item
- Modifier natif de certains Uniques (Stormshield, etc.)
- Certains Runewords (Oath, Eternity, Breath of the Dying)

### 12.5 Items Ethereaux

Les items **Ethereaux** (gris clair) ont :
- +50% Enhanced Defense (armures) ou +50% Enhanced Damage (armes)
- Durabilite reduite de moitie
- Ne peuvent PAS etre repares par les NPCs (sauf enchantement Zod ou Runeword specifique)
- Ideal pour equiper les Mercenaires (ils ne meurent pas de durabilite 0)

---

## 13. Systeme de Sockets

### 13.1 Sources de sockets

| Source | Methode | Notes |
|--------|---------|-------|
| Drop naturel | L'item droppee avec sockets | Max: Normal=3, NM=4, Hell=6 |
| Larzuk (Quete A5 Q1) | Recompense de quete | Donne le max possible selon ilvl |
| Cube Alchimique | Recette specifique (voir section 8.3) | 1-6 aleatoire, cap par item |
| Bonus max bouclier Unique (Stormshield) | Drop natif | Fixe |

### 13.2 Nombre maximum de sockets par type et ilvl

#### Armures (Body Armor)

| ilvl | Max Sockets |
|------|-------------|
| 1-25 | 1 socket |
| 26-40 | 2 sockets |
| 41+ | 4 sockets |

> Certaines armures ont un max absolu inferieur (ex: mage plates max 3)

#### Armes

| Categorie | Max Sockets |
|-----------|-------------|
| 1H Swords | 6 |
| 2H Swords | 6 |
| Axes | 6 |
| Maces | 6 |
| Polearms | 6 |
| Bows / Xbows | 6 |
| Staves | 6 |
| Wands | 2 |
| Orbs | 3 |
| Scepters | 5 |

#### Casques

| Type | Max Sockets |
|------|-------------|
| Normal Helms | 2 |
| Great Helm | 3 |
| Barb Helms | 3 |
| Druid Pelts | 3 |
| Circletes | 2 |

#### Boucliers

| Type | Max Sockets |
|------|-------------|
| Small/Buckler | 3 |
| Large/Kite | 4 |
| Tower/Bone | 4 |
| Monarch | 4 (cle pour Spirit) |
| Paladin Shields | 4 |
| Shrunken Heads | 3 |

### 13.3 Regles de Runeword

Pour qu'un Runeword fonctionne :
1. Item de type **Normal** (pas Magic, Rare, Set, Unique) — la base doit etre blanche/grise
2. Item doit avoir **exactement** le bon nombre de sockets (ni plus ni moins)
3. Runes insertees dans **l'ordre exact** (de gauche a droite dans l'interface)
4. L'item doit correspondre au **type requis** par le runeword

> Les items **Etheres Normaux** peuvent recevoir des runewords et obtiennent les bonus de durabilite 0 (brisable) — SAUF si le runeword confere Indestructible.

### 13.4 Sockets : Larzuk par ilvl (item Normal)

| ilvl de l'item | Sockets donnes (Larzuk) |
|----------------|------------------------|
| 1-25 | Max de 1-3 selon type |
| 26-40 | Max de 2-4 selon type |
| 41+ | Max absolu du type |

### 13.5 Retrait des socketed items

- Recette Cube : Hel + Scroll of Town Portal + item socket = retire toutes les runes/gems/jewels sans detruire l'item
- ATTENTION : les runes/gems/jewels retires sont DETRUITS

---

## 14. Schemas TOML Sodomight

### 14.1 Definition d'un item unique complet

```toml
# sodomight/data/items/unique/shako.toml
# @id: item-unique-shako
# @do: define unique helm Mirth's Crown
# @role: data
# @layer: 6

[item]
id = "mirths_crown"
base_item = "war_hat"
display_name = "Mirth's Crown"
original_name = "Harlequin Crest (Shako)"
quality = "Unique"
ilvl_drop_min = 76

[item.display]
color = "Gold"
flavor_text = "Shining in the darkness..."

[item.requirements]
strength = 20
level = 62

[item.properties]
# Stats fixes
all_skills = 2
damage_reduced_pct = 10
better_mf_pct = 50
all_attributes = 2
# Stats per level (0.75 per clvl)
life_per_clvl = 75   # stored as fixed-point x100 → 0.75
mana_per_clvl = 75

[item.sockets]
max_sockets = 2
```

### 14.2 Definition d'un set complet

```toml
# sodomight/data/items/sets/tal_rasha.toml
# @id: item-set-tal-rasha
# @do: define Tal Rasha set (Wraithweave Vestments)
# @role: data
# @layer: 6

[set]
id = "tal_rashas_wrappings"
display_name = "Wraithweave Vestments"
original_name = "Tal Rasha's Wrappings"
class = "Sorceress"
pieces = [
    "tal_rashas_lidless_eye",
    "tal_rashas_guardianship",
    "tal_rashas_horadric_crest",
    "tal_rashas_fine_spun_cloth",
    "tal_rashas_adjudication",
]

[[set.partial_bonus]]
pieces_required = 2
effect = { type = "all_skills", value = 1 }

[[set.partial_bonus]]
pieces_required = 3
effect = { type = "max_mana_pct", value = 5 }

[[set.partial_bonus]]
pieces_required = 4
effect = { type = "life_flat", value = 50 }

[set.full_bonus]
all_skills = 3
max_mana = 150
max_life = 50
max_cold_resist = 5
max_lightning_resist = 5
max_fire_resist = 5

[[set.item]]
id = "tal_rashas_lidless_eye"
base_item = "swirling_crystal"
display_name = "Tal Rasha's Lidless Eye"

[[set.item.properties]]
key = "sorceress_skills"
value = 2
condition = "per_piece_1"

[[set.item.properties]]
key = "faster_cast_rate_pct"
value = 33

[[set.item.properties]]
key = "mana"
value = 77
```

### 14.3 Definition d'un runeword

```toml
# sodomight/data/items/runewords/spirit.toml
# @id: item-runeword-spirit
# @do: define Spirit runeword (Echo)
# @role: data
# @layer: 6

[runeword]
id = "spirit"
display_name = "Echo"
original_name = "Spirit"
runes = ["Tal", "Thul", "Ort", "Amn"]
rune_count = 4
ladder_only = false

[runeword.valid_types]
swords = true           # 4-socket swords
shields = true          # 4-socket shields (Monarch preferred)
spirit_shields = false

[runeword.properties]
all_skills = 2
faster_cast_rate_pct = { min = 25, max = 35 }
faster_hit_recovery_pct = 55
defense_vs_missile = 250
vitality = 22
mana = { min = 89, max = 112 }
cold_resist = 35
lightning_resist = 35
poison_resist = 35
life_stolen_pct = 4
attacker_takes_damage = 14
```

### 14.4 Definition des affixes prefixes/suffixes

```toml
# sodomight/data/affixes/prefixes.toml
# @id: affix-prefixes-weapons
# @do: define weapon prefixes pool
# @role: data
# @layer: 6

[[prefix]]
id = "cruel"
display_name = "Cruel"
affix_level = 65
item_types = ["swords", "axes", "maces", "spears", "polearms"]
magic_only = false
rare_allowed = true
effect = { type = "enhanced_damage_pct", min = 201, max = 300 }

[[prefix]]
id = "ferocious"
display_name = "Ferocious"
affix_level = 81
item_types = ["swords", "axes", "maces", "spears", "polearms"]
magic_only = false
rare_allowed = true
effect = { type = "enhanced_damage_pct", min = 251, max = 300 }

[[prefix]]
id = "shimmering"
display_name = "Shimmering"
affix_level = 7
item_types = ["helms", "armor", "shields", "boots", "gloves", "belts"]
magic_only = false
rare_allowed = true
effect = { type = "all_resist_pct", min = 4, max = 7 }

[[prefix]]
id = "of_balance"
display_name = "of Balance"    # suffixe
affix_level = 14
item_types = ["helms", "armor", "shields", "gloves"]
magic_only = false
rare_allowed = true
effect = { type = "all_resist_pct", min = 8, max = 12 }
```

### 14.5 Definition des 33 runes

```toml
# sodomight/data/items/runes.toml
# @id: items-runes-definitions
# @do: define all 33 runes with effects and upgrade recipes
# @role: data
# @layer: 6

[[rune]]
id = "el"
display_name = "Lux"      # Sodomight name
original_name = "El"
rune_number = 1
ilvl_drop = 11
[rune.weapon_effect]
attack_rating = 50
light_radius = 1
[rune.armor_helm_effect]
defense = 15
light_radius = 1
[rune.shield_effect]
defense = 15
light_radius = 1
[rune.upgrade]
input = { rune = "el", count = 3 }
output = "eld"

[[rune]]
id = "eld"
display_name = "Crep"
original_name = "Eld"
rune_number = 2
ilvl_drop = 11
[rune.weapon_effect]
enhanced_damage_undead_pct = 75
attack_rating_undead = 50
[rune.armor_helm_effect]
stamina_drain_slower_pct = 15
[rune.shield_effect]
block_chance_pct = 7
[rune.upgrade]
input = { rune = "eld", count = 3 }
output = "tir"

[[rune]]
id = "tir"
display_name = "Sael"
original_name = "Tir"
rune_number = 3
ilvl_drop = 13
[rune.all_effect]
mana_after_kill = 2
[rune.upgrade]
input = { rune = "tir", count = 3 }
output = "nef"

# ... (runes 4 a 33 suivent le meme schema)

[[rune]]
id = "zod"
display_name = "Aeon"
original_name = "Zod"
rune_number = 33
ilvl_drop = 69
[rune.all_effect]
indestructible = true
[rune.upgrade]
# Aucune recette possible : rune ultime
input = null
output = null
```

---

## Sources et References

Ce document a ete elabore a partir des sources suivantes :

- [The Arreat Summit (source officielle Blizzard)](https://classic.battle.net/diablo2exp/)
- [Diablo 2 Wiki — diablo2.diablowiki.net](https://diablo2.diablowiki.net/)
- [Diablo Wiki Fandom](https://diablo.fandom.com/wiki/)
- [Maxroll.gg D2 Resources](https://maxroll.gg/d2/)
- [diablo2.io Database](https://diablo2.io/)
- [Icy Veins Diablo 2 Guides](https://www.icy-veins.com/d2/)
- [Project Diablo 2 Wiki](https://wiki.projectdiablo2.com/)
- [PureDiablo Diablo 2 Guides](https://www.purediablo.com/diablo-2/)
- [D2tomb.com Item Database](https://www.d2tomb.com/)

---

*Document Sodomight — Version 1.0*
*Genere pour le projet Miyukini-COG / MGE*

