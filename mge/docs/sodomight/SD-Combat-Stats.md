# SD-Combat-Stats — Référence Exhaustive Combat, Stats & Formules

**Projet :** Sodomight (clone Diablo 2 LoD fidèle, assets maison)
**Moteur :** MGE (Miyukini Game Engine) — ECS archetype, data-driven TOML
**Crates cibles :** `mge-arpg-stats`, `mge-arpg-combat`
**Version de référence :** Diablo II + Lord of Destruction v1.14d (vanilla)
**Date :** 2026-02-28

> **Avertissement sur les sources :** Toutes les formules et valeurs de ce document
> sont issues de Diablo II vanilla v1.13/v1.14. Les variantes Project Diablo 2 (PD2)
> sont explicitement signalées `[PD2-ONLY]` et NE doivent PAS être implémentées
> dans Sodomight sans décision explicite du game design.

---

## Table des matières

1. [Attributs de base par classe](#1-attributs-de-base-par-classe)
2. [Formules de combat](#2-formules-de-combat)
3. [Défense et blocage](#3-défense-et-blocage)
4. [Breakpoints — Tables complètes](#4-breakpoints--tables-complètes)
5. [Vie, Mana, Stamina](#5-vie-mana-stamina)
6. [Effets de statut](#6-effets-de-statut)
7. [Systèmes de résistance avancés](#7-systèmes-de-résistance-avancés)
8. [Magic Find](#8-magic-find)
9. [Expérience et leveling](#9-expérience-et-leveling)
10. [Stamina — Système détaillé](#10-stamina--système-détaillé)
11. [Niveaux de zones et items](#11-niveaux-de-zones-et-items)
12. [Schémas d'implémentation TOML/Rust](#12-schémas-dimplémentation-tomlrust)

---

## 1. Attributs de base par classe

### 1.1 Stats de départ

| Classe       | STR | DEX | VIT | ENE | Life (départ) | Mana (départ) | Stamina (départ) |
|--------------|-----|-----|-----|-----|---------------|---------------|------------------|
| Amazon       |  20 |  25 |  20 |  15 |            50 |            15 |               84 |
| Assassin     |  20 |  20 |  20 |  25 |            50 |            25 |               95 |
| Barbarian    |  30 |  20 |  25 |  10 |            55 |            10 |               91 |
| Druid        |  15 |  20 |  25 |  20 |            55 |            20 |               84 |
| Necromancer  |  15 |  25 |  15 |  25 |            45 |            25 |               79 |
| Paladin      |  25 |  20 |  25 |  15 |            55 |            15 |               89 |
| Sorceress    |  10 |  25 |  10 |  35 |            40 |            35 |               74 |

Chaque montée de niveau octroie **5 points de stat** à répartir librement.

### 1.2 Gains par point de Vitality

| Classe       | Life/VIT | Stamina/VIT | Life/niveau | Stamina/niveau |
|--------------|----------|-------------|-------------|----------------|
| Amazon       |     +3   |      +1     |      +2     |       +1       |
| Assassin     |     +3   |     +1.25   |      +2     |      +1.25     |
| Barbarian    |     +4   |      +1     |      +2     |       +1       |
| Druid        |     +2   |      +1     |     +1.5    |       +1       |
| Necromancer  |     +2   |      +1     |     +1.5    |       +1       |
| Paladin      |     +3   |      +1     |      +2     |       +1       |
| Sorceress    |     +2   |      +1     |      +1     |       +1       |

> **Note implémentation :** Life et Stamina sont stockés en valeur entière mais les
> gains fractionnaires (1.5, 1.25) s'accumulent en virgule flottante interne.
> Arrondir vers le bas à chaque niveau pour l'affichage.

### 1.3 Gains par point d'Energy (Mana)

| Classe       | Mana/ENE | Mana/niveau |
|--------------|----------|-------------|
| Amazon       |    +1.5  |     +1.5    |
| Assassin     |   +1.75  |     +1.5    |
| Barbarian    |    +1.0  |     +1.0    |
| Druid        |    +2.0  |     +2.0    |
| Necromancer  |    +2.0  |     +2.0    |
| Paladin      |    +1.5  |     +1.5    |
| Sorceress    |    +2.0  |     +2.0    |

### 1.4 Bonus de dommages selon Strength et Dexterity par type d'arme

Le bonus de dommage physique appliqué par les attributs est calculé ainsi :

```
DmgBonus% = (STR * strFactor + DEX * dexFactor) / 100
```

où `strFactor` et `dexFactor` sont définis par le **type d'arme** (non la classe) :

| Catégorie d'arme             | STR% | DEX% | Notes                              |
|------------------------------|------|------|------------------------------------|
| Swords (épées 1H et 2H)      | 100  |    0 | 1% dmg par point de STR            |
| Axes (haches)                | 100  |    0 |                                    |
| Maces / Scepters / Wands     | 110  |    0 | Hammerdin: 110% STR bonus          |
| Staves (bâtons 2H)           | 100  |    0 |                                    |
| Spears (lances 2H)           | 100  |    0 | Spear = 100% STR uniquement        |
| Polearms (hallebardes)       | 100  |    0 |                                    |
| Daggers (dagues)             |  50  |   50 | Mix STR/DEX 50/50                  |
| Throwing (armes de lancer)   |  50  |   50 | LoD uniquement — Classic: 100 STR  |
| Bows (arcs)                  |    0 |  100 | 100% DEX uniquement                |
| Crossbows (arbalètes)        |    0 |  100 |                                    |
| Javelins (javelines)         |  50  |   50 |                                    |
| Claws (griffes Assassin)     |  75  |   75 | Total 150% (75 STR + 75 DEX)       |
| Amazon Spears/Javelins       | 100  |    0 | Quand utilisées en mêlée           |

**Formule complète du DmgBonus intégré au calcul :**

```
PhysMin = floor(WeaponMin * (1 + DmgBonus%/100 + ED_on_weapon/100)
               + FlatMin)
         * (1 + ED_off_weapon/100)

PhysMax = floor(WeaponMax * (1 + DmgBonus%/100 + ED_on_weapon/100)
               + FlatMax)
         * (1 + ED_off_weapon/100)
```

### 1.5 Attack Rating par point de Dexterity

La formule de l'AR de base est :

```
BaseAR = (Dexterity - 7) * 5 + ClassBaseAR
```

Chaque point de DEX apporte **5 AR** (identique pour toutes les classes).
Le `ClassBaseAR` est une constante par classe :

| Classe       | ClassBaseAR |
|--------------|-------------|
| Amazon       |          9  |
| Assassin     |         14  |
| Barbarian    |         10  |
| Druid        |          7  |
| Necromancer  |          7  |
| Paladin      |         10  |
| Sorceress    |          7  |

> Ces constantes proviennent des fichiers de données D2 (`Characters.txt`).

---

## 2. Formules de combat

### 2.1 Chance de toucher (Chance to Hit)

```
CTH = 200 * AR / (AR + DR) * clvl / (clvl + mlvl)
```

- **AR** = Attack Rating de l'attaquant (joueur ou monstre)
- **DR** = Defense Rating du défenseur
- **clvl** = niveau de l'attaquant
- **mlvl** = niveau du défenseur
- **Résultat cappé :** min = 5%, max = 95%
- **Unités :** résultat en pourcentage (0–100)

**Variante "Ignore Target's Defense" (ITD) :**

```
CTH_ITD = 200 * clvl / (clvl + mlvl)
```

Le DR est ignoré, seul l'écart de niveau compte. Toujours cappé [5%, 95%].

**Variante PvP :** Même formule. Le défenseur (joueur) a sa défense réduite de 50%
dans les calculs PvP (`DR_effective = DR / 2`).

**Implémentation Rust :**

```rust
pub fn chance_to_hit(ar: u32, dr: u32, clvl: u8, mlvl: u8) -> f32 {
    let ar = ar as f32;
    let dr = dr as f32;
    let clvl = clvl as f32;
    let mlvl = mlvl as f32;
    let cth = 200.0 * (ar / (ar + dr)) * (clvl / (clvl + mlvl));
    cth.clamp(5.0, 95.0)
}
```

### 2.2 Calcul des dommages physiques

**Ordre de calcul (pipeline) :**

```
1. WeaponMin / WeaponMax         — dommage de base de l'arme
2. × 1.5 si Ethereal             — bonus 50% éthéré (avant tout)
3. × (1 + ED_on_weapon/100)      — Enhanced Damage sur l'arme (multiplicatif)
4. + FlatMin / FlatMax           — bonus plats (+X dommages)
5. × (1 + ED_off_weapon/100)     — Enhanced Damage hors arme (additionné, puis ×)
6. × (1 + StrDexBonus/100)       — bonus STR/DEX selon type d'arme
7. × 2  si Critical Strike       — double les dommages physiques
8. + ElementalDamage             — dommages élémentaires additionnés séparément
```

**ED On-Weapon vs Off-Weapon :**

- **On-weapon ED** : Affixe directement sur l'arme. S'applique en étape 3,
  avant les bonus plats. Multiplicatif avec lui-même si plusieurs sources.
- **Off-weapon ED** : Items (armure, bijoux, capacités, auras). Tous additionnés
  ensemble puis appliqués comme un seul multiplicateur en étape 5.

```
ED_off_total = sum(ED_from_armor, ED_from_rings, ED_from_skills, ED_from_auras)
```

**Exemple de calcul :**

```
Arme : 50-100 dommages de base
ED on-weapon : +100%
+20 dommages plats
STR bonus : +200% (200 STR × 100%)
ED off-weapon : +150% (skills + items)
No critical

PhysMin = floor((50 × 2.0) + 20) × 3.5 = floor(120) × 3.5 = 420
PhysMax = floor((100 × 2.0) + 20) × 3.5 = floor(220) × 3.5 = 770
```

### 2.3 Critical Strike et Deadly Strike

**Critical Strike (CS)** — Compétence passive (Amazon, Barbarian...) :
- Chance de doubler les dommages physiques totaux
- Source : skill uniquement

**Deadly Strike (DS)** — Stat d'item :
- Même effet : double les dommages physiques totaux
- Cumulable avec CS

**Interaction CS + DS :**

```
P_double = 1 - ((1 - CS/100) × (1 - DS/100))
```

Exemple : CS 50% + DS 50% → P = 1 - (0.5 × 0.5) = 75% de chance de doubler.

> [PD2-ONLY] En PD2, les crits multiplient par ×1.5 au lieu de ×2.
> **Sodomight utilise ×2 (vanilla).**

**Quadruple damage (Mercenaire/Summon) :** Les suivants ont 5% de chance fixe
d'appliquer un effet "quad damage" (×4). C'est une mécanique interne distincte de CS/DS.

### 2.4 Open Wounds

Open Wounds inflige des dommages physiques sur la durée (DoT), non soignables
pendant la durée de l'effet.

**Durée :** 8 secondes (200 frames à 25 fps)

**Dommage par frame en fonction du niveau du joueur (clvl) :**

```
Si clvl ∈ [1,15]  : DpF = (9×clvl + 31) / 256
Si clvl ∈ [16,30] : DpF = (18×clvl - 104) / 256
Si clvl ∈ [31,45] : DpF = (27×clvl - 374) / 256
Si clvl ∈ [46,60] : DpF = (36×clvl - 779) / 256
Si clvl ∈ [61,99] : DpF = (45×clvl - 1319) / 256
```

**DPS = DpF × 25** (25 frames/seconde)

**Tableaux de référence rapide :**

| clvl | DPS (approx.) |
|------|---------------|
|    1 |           1.5 |
|   10 |          12.8 |
|   20 |          21.9 |
|   30 |          34.2 |
|   40 |          51.2 |
|   50 |          74.2 |
|   60 |         103.5 |
|   70 |         138.7 |
|   80 |         175.8 |
|   90 |         212.9 |
|   99 |         241.2 |

**Modificateurs de cible :**

| Cible                  | Mêlée | Distance |
|------------------------|-------|----------|
| Monstres normaux       | ×1    | ×1/2     |
| Champions / Super-unique | ×1/2 | ×1/4   |
| Acte-Boss              | ×1/2  | ×1/4     |
| Joueur / Mercenaire    | ×1/4  | ×1/8     |

### 2.5 Crushing Blow

Crushing Blow retire une fraction de la **vie actuelle** de la cible (non la vie max).

| Cible                    | Mêlée  | Distance |
|--------------------------|--------|----------|
| Monstres normaux         | 1/4    | 1/8      |
| Champions / Super-unique | 1/8    | 1/16     |
| Acte-Boss                | 1/8    | 1/16     |
| Joueur / Mercenaire      | 1/10   | 1/20     |

```rust
pub fn crushing_blow_damage(current_life: i32, target_type: TargetType,
                             is_ranged: bool) -> i32 {
    let fraction = match (target_type, is_ranged) {
        (TargetType::Normal, false)          => (1, 4),
        (TargetType::Normal, true)           => (1, 8),
        (TargetType::ChampionBoss, false)    => (1, 8),
        (TargetType::ChampionBoss, true)     => (1, 16),
        (TargetType::ActBoss, false)         => (1, 8),
        (TargetType::ActBoss, true)          => (1, 16),
        (TargetType::PlayerHireling, false)  => (1, 10),
        (TargetType::PlayerHireling, true)   => (1, 20),
    };
    current_life * fraction.0 / fraction.1
}
```

### 2.6 Dommages élémentaires

Les dommages élémentaires (Fire, Cold, Lightning, Poison, Magic) sont **additionnés**
au dommage physique en dernière étape et traités séparément.

**Formule générale :**

```
ElemDmgFinal = ElemDmg × (1 - Resistance/100)
```

- Les résistances **négatives** amplifient les dommages (minimum -100%).
- Si la résistance est ≥ 100% → le type élémentaire est **immune** (voir §7).

**Feu (Fire) :**
- Dommage instantané
- Résistance Fire du monstre/joueur appliquée directement

**Froid (Cold) :**
- Dommage instantané + effet de ralentissement
- Durée de gel/ralentissement proportionnelle au skill
- Bonus de dommages Cold contre cibles Frozen (certains skills)

**Foudre (Lightning) :**
- Dommage instantané, variance élevée (min–max très large)
- Chain Lightning : splash sur cibles adjacentes

**Poison :**
- DoT sur durée variable selon source
- Formule : `PoisonDPS = PoisonDamage / Duration`
- Les résistances Poison s'appliquent au **total** du dommage (non par tick)
- Effet : réduit la régénération de vie pendant la durée (empoisonnement)

**Formule Poison complète :**

```
PoisonTotal = BasePoisonDmg × (1 + SkillBonus/100)
PoisonDuration = BaseDuration × (1 + DurationBonus/100)
PoisonDPS = PoisonTotal / PoisonDuration

Pour chaque frame :
PoisonDmgPerFrame = PoisonTotal / (PoisonDuration × 25)
TotalApplied = PoisonDmgPerFrame × (1 - PoisonResist/100)
```

**Magie (Magic) :**
- Ignore l'armure physique
- N'est PAS affectée par les résistances physiques
- Peut être réduite par des items spécifiques (+x% Magic Resist — très rare)

**Dommages convertis :**
Certains skills convertissent une portion du dommage physique en élémentaire.

```
PhysicalPortion = WeaponDmg × (1 - ConversionRate/100)
ElementalPortion = WeaponDmg × (ConversionRate/100)
```

Exemple : Phoenix Strike — le dommage de base de l'arme est partiellement converti.
Les dommages convertis **ne** bénéficient **pas** des bonus Enhanced Damage physique.

---

## 3. Défense et blocage

### 3.1 Défense (Defense Rating)

La défense **ne réduit PAS les dommages** reçus directement.
Elle affecte uniquement la **Chance to Hit** de l'attaquant (via la formule CTH §2.1).

```
CTH = 200 × AR/(AR+DR) × clvl/(clvl+mlvl)
```

Un DR élevé oblige l'attaquant à avoir beaucoup d'AR pour toucher.

**Spécialité Smite (Paladin) :**
Smite touche **toujours** (100% CTH) et le blocage ne s'applique pas contre Smite.

### 3.2 Formule de Chance to Block

```
CTB = floor((ShieldBlock + Bonus) × (DEX - 15) / (clvl × 2))
CTB = min(CTB, 75)
```

- **ShieldBlock** : Valeur de blocage de base du bouclier + bonus de classe
- **Bonus** : Somme de tous les "+x% Increased Chance of Blocking" + Holy Shield
- **DEX** : Dextérité totale du personnage
- **clvl** : Niveau du personnage
- **Cap maximum : 75%**

**DEX requise pour 75% de blocage :**

```
DEX_pour_75% = 15 + ceil(150 × clvl / (ShieldBlock + Bonus))
```

**Malus en run :**

Lorsque le personnage court (`Run`), le CTB est divisé par 3, et cappé à 25%.

```
CTB_run = min(floor(CTB / 3), 25)
```

**Holy Shield (Paladin) :**

Le skill Holy Shield ajoute directement au `Bonus` de la formule.
Niveau 20 Holy Shield : +203% blocking. Le paladin peut atteindre le cap 75%
avec très peu de DEX en combinant Holy Shield et un bon bouclier paladin.

### 3.3 Classes de boucliers et bonus de blocage

| Classe       | Bonus de blocage inhérent | Notes                        |
|--------------|---------------------------|------------------------------|
| Amazon       | +25%                      |                              |
| Assassin     | +25%                      |                              |
| Barbarian    | +25%                      |                              |
| Druid        | +20%                      | Formes animales: voir §3.4   |
| Necromancer  | +20%                      |                              |
| Paladin      | +30%                      | Holy Shield s'additionne     |
| Sorceress    | +20%                      |                              |

### 3.4 Particularités par classe

**Paladin + Holy Shield :**
- La Smite du paladin ignore complètement le block du défenseur
- Holy Shield permet des frames de block très rapides (voir §4.3 FBR)

**Formes animales Druid (Werewolf / Werebear) :**
- Peuvent équiper un bouclier et bloquer
- Les tables FBR sont distinctes par forme (voir §4.3)

**Sorceress / Necromancer :**
- Blocage fonctionnel mais pénalisé par le faible bonus (20%)
- Très haute DEX nécessaire pour 75% block à haut niveau

---

## 4. Breakpoints — Tables complètes

> Les frames indiqués sont à 25 fps (Diablo 2 standard).
> Un frame = 1/25e de seconde = 0.04 seconde.

### 4.1 FCR — Faster Cast Rate (frames pour lancer un sort)

#### Amazon

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     19 |   68 |     13 |
|    7 |     18 |   99 |     12 |
|   14 |     17 |  152 |     11 |
|   22 |     16 |      |        |
|   32 |     15 |      |        |
|   48 |     14 |      |        |

> Note : Les valeurs légèrement différentes selon le type d'arme (arc vs javeline etc.)
> ne changent les seuils que par 1-3% FCR dans certains cas — la table ci-dessus
> représente le cas général.

#### Assassin

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     16 |   65 |     11 |
|    8 |     15 |  102 |     10 |
|   16 |     14 |  174 |      9 |
|   27 |     13 |      |        |
|   42 |     12 |      |        |

#### Barbarian

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     13 |   63 |      9 |
|    9 |     12 |  105 |      8 |
|   20 |     11 |  200 |      7 |
|   37 |     10 |      |        |

#### Druid (forme humaine)

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     18 |   68 |     12 |
|    4 |     17 |   99 |     11 |
|   10 |     16 |  163 |     10 |
|   19 |     15 |      |        |
|   30 |     14 |      |        |
|   46 |     13 |      |        |

#### Druid (Werewolf / formes werecreature)

| FCR% | Frames (WW) | FCR% | Frames (WB) |
|------|-------------|------|-------------|
|    0 |          16 |    0 |          16 |
|    6 |          15 |   15 |          14 |
|   14 |          14 |   26 |          13 |
|   26 |          13 |   40 |          12 |
|   40 |          12 |   63 |          11 |
|   60 |          11 |   99 |          10 |
|   95 |          10 |  163 |           9 |
|  157 |           9 |      |             |

WW = Werewolf, WB = Werebear

#### Necromancer (forme normale)

| FCR% | Frames |
|------|--------|
|    0 |     15 |
|    9 |     14 |
|   18 |     13 |
|   30 |     12 |
|   48 |     11 |
|   75 |     10 |
|  125 |      9 |

#### Necromancer (forme Vampire — Iron Golem / Bone Spirit certains skills)

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     23 |   86 |     15 |
|    6 |     22 |  120 |     14 |
|   11 |     21 |  180 |     13 |
|   18 |     20 |      |        |
|   24 |     19 |      |        |
|   35 |     18 |      |        |
|   48 |     17 |      |        |
|   65 |     16 |      |        |

#### Paladin

| FCR% | Frames |
|------|--------|
|    0 |     15 |
|    9 |     14 |
|   18 |     13 |
|   30 |     12 |
|   48 |     11 |
|   75 |     10 |
|  125 |      9 |

#### Sorceress (sorts standards)

| FCR% | Frames |
|------|--------|
|    0 |     13 |
|    9 |     12 |
|   20 |     11 |
|   37 |     10 |
|   63 |      9 |
|  105 |      8 |
|  200 |      7 |

#### Sorceress (Lightning / Chain Lightning uniquement)

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     19 |   78 |     13 |
|    7 |     18 |  117 |     12 |
|   15 |     17 |  194 |     11 |
|   23 |     16 |      |        |
|   35 |     15 |      |        |
|   52 |     14 |      |        |

#### Mercenaires Acte 3 (cast)

| FCR% | Frames | FCR% | Frames |
|------|--------|------|--------|
|    0 |     17 |   58 |     12 |
|    8 |     16 |   86 |     11 |
|   15 |     15 |  138 |     10 |
|   26 |     14 |      |        |
|   39 |     13 |      |        |

### 4.2 FHR — Faster Hit Recovery (frames de stagger)

Le stagger est déclenché quand les dommages reçus en un coup dépassent
un seuil de `1/12` de la vie maximale du personnage (Stun Threshold).

#### Amazon

| FHR% | Frames |
|------|--------|
|    0 |     11 |
|    6 |     10 |
|   13 |      9 |
|   20 |      8 |
|   32 |      7 |
|   52 |      6 |
|   86 |      5 |
|  174 |      4 |
|  600 |      3 |

#### Assassin

| FHR% | Frames |
|------|--------|
|    0 |      9 |
|    7 |      8 |
|   15 |      7 |
|   27 |      6 |
|   48 |      5 |
|   86 |      4 |
|  200 |      3 |

#### Barbarian

| FHR% | Frames |
|------|--------|
|    0 |      9 |
|    7 |      8 |
|   15 |      7 |
|   27 |      6 |
|   48 |      5 |
|   86 |      4 |
|  200 |      3 |

#### Druid (arme 1H oscillante)

| FHR% | Frames |
|------|--------|
|    0 |     14 |
|    3 |     13 |
|    7 |     12 |
|   13 |     11 |
|   19 |     10 |
|   29 |      9 |
|   42 |      8 |
|   63 |      7 |
|   99 |      6 |
|  174 |      5 |

#### Druid (autres armes / forme humaine)

| FHR% | Frames |
|------|--------|
|    0 |     13 |
|    5 |     12 |
|   10 |     11 |
|   16 |     10 |
|   26 |      9 |
|   39 |      8 |
|   56 |      7 |
|   86 |      6 |
|  152 |      5 |

#### Druid (Werewolf)

| FHR% | Frames |
|------|--------|
|    0 |      7 |
|    9 |      6 |
|   20 |      5 |
|   42 |      4 |
|   86 |      3 |
|  280 |      2 |

#### Druid (Werebear)

| FHR% | Frames |
|------|--------|
|    0 |     13 |
|    5 |     12 |
|   10 |     11 |
|   16 |     10 |
|   24 |      9 |
|   37 |      8 |
|   54 |      7 |
|   86 |      6 |
|  152 |      5 |

#### Necromancer (forme normale)

| FHR% | Frames |
|------|--------|
|    0 |     13 |
|    5 |     12 |
|   10 |     11 |
|   16 |     10 |
|   26 |      9 |
|   39 |      8 |
|   56 |      7 |
|   86 |      6 |
|  152 |      5 |

#### Necromancer (forme Vampire)

| FHR% | Frames |
|------|--------|
|    0 |     15 |
|    2 |     14 |
|    6 |     13 |
|   10 |     12 |
|   16 |     11 |
|   24 |     10 |
|   34 |      9 |
|   48 |      8 |
|   72 |      7 |
|  117 |      6 |
|  208 |      5 |

#### Paladin (lance / bâton)

| FHR% | Frames |
|------|--------|
|    0 |     13 |
|    3 |     12 |
|    7 |     11 |
|   13 |     10 |
|   20 |      9 |
|   32 |      8 |
|   48 |      7 |
|   75 |      6 |
|  129 |      5 |
|  280 |      4 |

#### Paladin (autres armes)

| FHR% | Frames |
|------|--------|
|    0 |      9 |
|    7 |      8 |
|   15 |      7 |
|   27 |      6 |
|   48 |      5 |
|   86 |      4 |
|  200 |      3 |

#### Sorceress

| FHR% | Frames |
|------|--------|
|    0 |     15 |
|    5 |     14 |
|    9 |     13 |
|   14 |     12 |
|   20 |     11 |
|   30 |     10 |
|   42 |      9 |
|   60 |      8 |
|   86 |      7 |
|  142 |      6 |
|  280 |      5 |

#### Mercenaires

| FHR% | Act 1 | Act 2 | Act 3 | Act 5 |
|------|-------|-------|-------|-------|
|    0 |    11 |    15 |    17 |     9 |
|    5 |    — |    14 |    16 |     — |
|    6 |    10 |    — |    — |     — |
|    7 |    — |    — |    — |     8 |
|    8 |    — |    — |    15 |     — |
|   13 |    — |    — |    14 |     — |
|   14 |    — |    12 |    — |     — |
|   15 |     — |    — |    — |     7 |
|   20 |     8 |    11 |    13 |     — |
|   27 |    — |    — |    — |     6 |
|   30 |    — |    10 |    — |     — |
|   32 |     7 |    — |    11 |     — |
|   42 |    — |     9 |    — |     — |
|   46 |    — |    — |    10 |     — |
|   48 |    — |    — |    — |     5 |
|   52 |     6 |    — |    — |     — |
|   60 |    — |     8 |    — |     — |
|   63 |    — |    — |     9 |     — |
|   86 |     5 |     7 |     8 |     4 |
|  133 |    — |    — |     7 |     — |
|  142 |    — |     6 |    — |     — |
|  174 |     4 |    — |    — |     — |
|  200 |    — |    — |    — |     3 |
|  232 |    — |    — |     6 |     — |

### 4.3 FBR — Faster Block Rate (frames de blocage)

#### Amazon (arme 1H oscillante)

| FBR% | Frames |
|------|--------|
|    0 |     17 |
|    4 |     16 |
|    6 |     15 |
|   11 |     14 |
|   15 |     13 |
|   23 |     12 |
|   29 |     11 |
|   40 |     10 |
|   56 |      9 |
|   80 |      8 |
|  120 |      7 |
|  200 |      6 |

#### Amazon (autres armes / bouclier)

| FBR% | Frames |
|------|--------|
|    0 |      5 |
|   13 |      4 |
|   32 |      3 |
|   86 |      2 |

#### Assassin

| FBR% | Frames |
|------|--------|
|    0 |      5 |
|   13 |      4 |
|   32 |      3 |
|   86 |      2 |

#### Barbarian

| FBR% | Frames |
|------|--------|
|    0 |      7 |
|    9 |      6 |
|   20 |      5 |
|   42 |      4 |
|   86 |      3 |

#### Druid (forme humaine)

| FBR% | Frames |
|------|--------|
|    0 |     11 |
|    6 |     10 |
|   13 |      9 |
|   20 |      8 |
|   32 |      7 |
|   52 |      6 |
|   86 |      5 |

#### Druid (Werewolf)

| FBR% | Frames |
|------|--------|
|    0 |      9 |
|    7 |      8 |
|   15 |      7 |
|   27 |      6 |
|   48 |      5 |
|   86 |      4 |

#### Druid (Werebear)

| FBR% | Frames |
|------|--------|
|    0 |     12 |
|    5 |     11 |
|   10 |     10 |
|   16 |      9 |
|   27 |      8 |
|   40 |      7 |
|   65 |      6 |
|  109 |      5 |

#### Necromancer

| FBR% | Frames |
|------|--------|
|    0 |     11 |
|    6 |     10 |
|   13 |      9 |
|   20 |      8 |
|   32 |      7 |
|   52 |      6 |
|   86 |      5 |

#### Paladin (sans Holy Shield)

| FBR% | Frames |
|------|--------|
|    0 |      5 |
|   13 |      4 |
|   32 |      3 |
|   86 |      2 |

#### Paladin (avec Holy Shield actif)

| FBR% | Frames |
|------|--------|
|    0 |      2 |
|   86 |      1 |

#### Sorceress

| FBR% | Frames |
|------|--------|
|    0 |      9 |
|    7 |      8 |
|   15 |      7 |
|   27 |      6 |
|   48 |      5 |
|   86 |      4 |

### 4.4 IAS — Increased Attack Speed

#### Formule principale

L'IAS suit une logique de **diminishing returns** via la conversion en EIAS :

```
EIAS = floor(120 × IAS / (120 + IAS))
```

où `IAS` = total IAS des items équipés uniquement (pas les skills).

**Formule de durée d'animation :**

```
AnimDuration = ceil((AnimLength × 256) /
    (AnimSpeed × (AnimRate + SIAS + EIAS - WSM) / 100)) - 1
```

Variables :
- `AnimLength` : Nombre de frames de l'animation de base
- `AnimSpeed` : Valeur HitShift (256 pour les attaques normales)
- `AnimRate` : Taux d'animation de base (100 par défaut)
- `SIAS` : IAS de skill (Fanaticism, Burst of Speed, Werewolf, Frenzy, etc.)
- `EIAS` : IAS effectif depuis les items (formule ci-dessus)
- `WSM` : Weapon Speed Modifier de l'arme

**Note importante :** SIAS (IAS de skill) et EIAS (IAS item) se **somment**
avant la conversion. L'EIAS ne s'applique qu'aux items.

#### Weapon Speed Modifier (WSM) par type d'arme

| Type d'arme         | WSM typique | Notes                           |
|---------------------|-------------|---------------------------------|
| Phase Blade         |         -30 | Arme la plus rapide             |
| Crystal Sword       |         -10 |                                 |
| War Sword           |          10 |                                 |
| Colossus Sword      |          10 |                                 |
| Phase Blade (Sword) |         -30 |                                 |
| Berserker Axe       |           0 |                                 |
| Champion Axe        |          20 |                                 |
| War Hammer          |          20 | Marteaux = lents                |
| Maul                |          20 |                                 |
| Scythe (Polearm)    |           0 |                                 |
| War Scythe          |          20 |                                 |
| Short Bow           |           0 |                                 |
| Long Bow            |          10 |                                 |
| Short Battle Bow    |           0 |                                 |
| Grand Matron Bow    |          10 |                                 |
| Maiden Javelin      |         -10 |                                 |
| War Javelin         |           0 |                                 |
| Wand                |         -20 | Armes de cast rapides           |
| Bone Wand           |         -20 |                                 |
| Orb                 |         -30 | Plus rapide pour Sorceress      |
| Claw (type Katar)   |         -20 |                                 |
| Greater Claw        |         -10 |                                 |

> La liste complète des WSM est encodée dans `WeaponTypes.toml` du projet.
> Les valeurs sont issues de `weapons.txt` du jeu original.

**Cas spéciaux :**

- **Attaque séquentielle** (Jab, Impale, Leap Attack, Charge) : WSM +30 penalty
- **Armes de lancer** (quand lancées) : +30 penalty supplémentaire
- **Dual-Wield** (Frenzy, Double Swing) :
  `WSM_effective = (WSM_left + WSM_right) / 2`

---

## 5. Vie, Mana, Stamina

### 5.1 Régénération de Mana

```
ManaRegenPerSec = MaxMana / 120 × (1 + RegenMana% / 100)
```

- Taux de base : pleine régénération en **120 secondes** (~0.833% par seconde)
- `RegenMana%` : bonus depuis items (skulls, préfixe "Replenish Mana", auras)
- **Warmth (Sorceress)** : +x% Mana Regen (bonus efficace et scalable)
- **Meditation (Paladin)** : x% bonus — le seul skill non-Sorceress affectant le regen

**Par frame :**

```
ManaRegenPerFrame = MaxMana × (100 + RegenMana%) / (120 × 25 × 100)
```

### 5.2 Régénération de Life

La vie ne se régénère **pas** passivement de façon intrinsèque.
Seuls ces sources octroient de la régénération :

- **Replenish Life** (items) : `LifeRegenPerSec = ReplenishLife × 25 / 256`
- **Prayer Aura** (Paladin) : régénération active en aura
- **Potions de vie**
- **Life Steal** (vol de vie au contact — voir §5.4)

### 5.3 Régénération de Stamina

La stamina se régénère quand le personnage **marche** ou **reste immobile**.

- **Temps plein rechargement (marche) :** ~10.24 secondes (256 frames)
- **Temps plein rechargement (debout immobile) :** ~20.48 secondes (512 frames)
- **Armure lourde :** double la consommation de stamina en sprint

### 5.4 Life Steal (Vol de vie)

```
LifeStolen = PhysDmg × LeechRate × AttackPenalty × DifficultyPenalty
             × DrainEffectiveness
```

- **LeechRate** : % indiqué sur l'item (Life Stolen Per Hit)
- **AttackPenalty** : 1.0 (mêlée), 0.5 (splash/zone), 0.33 (Leap Attack)
- **DifficultyPenalty** : Normal = 1.0, Nightmare = 0.5, Hell = 0.333
- **DrainEffectiveness** : stat par monstre (0% pour squelettes, golems)

> Le Life Steal ne fonctionne que sur les **dommages physiques**.
> Aucun steal sur les dommages élémentaires ou magiques.

**Efficacité réduite par le Drain Effectiveness du monstre :**
Chaque monstre possède un paramètre `drain_effectiveness` dans ses données.
- Squelettes, golems : 0% → aucun steal possible
- La plupart des monstres : 100%
- Quelques boss : valeurs réduites

### 5.5 Mana Steal

Même formule que le Life Steal, mais appliquée au mana.
Le Mana Steal s'applique aussi uniquement aux dommages physiques.

```
ManaStolen = PhysDmg × ManaLeechRate × AttackPenalty × DifficultyPenalty
             × DrainEffectiveness
```

### 5.6 Energy Shield (Sorceress)

**Mécanisme :**
Energy Shield redirige une fraction des dommages reçus de la vie vers le mana.

```
DmgAbsorbedByMana = TotalDamage × AbsorbRate/100
DmgToLife = TotalDamage × (1 - AbsorbRate/100)
ManaConsumed = DmgAbsorbedByMana × ManaConversionRatio
```

**Taux d'absorption par niveau de skill :**

| Niveau | Absorption |
|--------|-----------|
|      1 |       20% |
|      5 |       36% |
|     10 |       52% |
|     15 |       68% |
|     20 |       80% |
|     25 |       88% |
|     30 |       92% |
|     35 |       94% |
|     40 |       95% (cap) |

**Ratio Mana/Dommage (ManaConversionRatio) :**

Sans Telekinesis : **2.0** (2 mana consommés par 1 point absorbé)

Avec niveaux de Telekinesis (synergie) :

| Niv. Telekinesis | Ratio Mana |
|------------------|-----------|
|                0 |       2.0 |
|                4 |       1.75 |
|                8 |       1.5 |
|               12 |       1.25 |
|               16 |       1.0 |
|               20 |       0.75 |

**Formule exacte :**

```
ManaRatio = 2.0 - (TelekinesisLevel × 0.0625)
```

**Interactions importantes :**
- Energy Shield absorbe les dommages **après** application des résistances
- Si le mana atteint 0, les dommages restants vont directement à la vie
- L'absorption ne s'applique pas aux dommages Poison (DoT)
- L'absorption ne réduit pas les effets procéduraux (Open Wounds, Crushing Blow)

---

## 6. Effets de statut

### 6.1 Effets du joueur sur les monstres

#### Frozen (Gelé)

- **Source :** Cold damage suffisant, Nova de glace, Blizzard, Arctic Blast Druid
- **Effet :** Immobilité totale du monstre
- **Durée :** Variable selon le skill/source (en frames)
- **Bonus :** Certains skills infligent +X% Cold damage contre cibles Frozen
- **Résistance :** Cold Resist réduit la durée (non les dommages)
  - `Duration_effective = BaseDuration × (1 - ColdResist/100)`
- **Immunité :** Si Cold Resist > 99% → non geable

#### Chilled (Ralenti par le froid)

- **Source :** Tout cold damage inférieur au seuil de gel
- **Effet :** Ralentissement du mouvement et des attaques (~50% vitesse)
- **Durée :** Proportionnelle au cold damage et à la résistance

#### Stunned (Étourdi)

- **Source :** Bash (Barbarian), Stun (Barbarian skill), certains items
- **Effet :** Paralysie temporaire — le monstre ne peut ni bouger ni attaquer
- **Durée :** Fixe par skill
- **Stun Lock :** Mécanisme par lequel des hits répétés maintiennent un monstre
  dans un état de stagger permanent. Nécessite un FHR élevé du monstre et une
  cadence d'attaque suffisante de l'attaquant.

#### Knockback

- **Source :** Items avec Knockback, Bash, Fist of the Heavens, etc.
- **Effet :** Déplacement forcé du monstre dans la direction de l'attaque
- **Distance :** Fixe (environ 4-6 tiles selon la source)
- **Immunité :** Boss Act et certains monstres "Large" sont résistants

#### Poison

- Voir §2.6 pour la formule complète
- Pile sur la même cible (multiple stacks de poison possible)
- Empêche la régénération de vie pendant la durée

#### Open Wounds

- Voir §2.4 pour la formule complète
- Un seul stack actif par attaquant (le refresh repart de zéro)
- Non soignable par régénération pendant la durée (8s)
- Immunité : cibles avec résistance physique très élevée (mécaniquement non bloqué,
  mais l'effet peut être de 0 si le monstre a immunity physique — rare)

#### Cursed (Malédictions — Necromancer)

Amplify Damage :
- Double les dommages **physiques** reçus par la cible
- Durée : skill-dépendante
- Stack avec autres sources de vulnérabilité

Decrepify :
- Réduit la vitesse d'attaque/déplacement du monstre de 50%
- Réduit la résistance physique du monstre (-50%)
- Durée : skill-dépendante

Life Tap :
- 50% du dommage physique infligé par N'IMPORTE QUI à la cible est volé
- Permet le Life Steal même contre les immunités drain (squelettes, etc.)
- Durée : skill-dépendante

Iron Maiden :
- Les dommages physiques infligés par la cible lui reviennent amplifiés
- Très efficace contre les monstres à forte attaque physique

### 6.2 Effets des monstres sur le joueur

#### Conviction Aura (Paladin-monstre)

- Réduit toutes les résistances des joueurs proches
- Peut faire passer les résistances **en dessous de 0%** (aucune limite minimale
  autre que -100%)
- Force d'une aura de monstre Conviction : valeur fixe selon le mlvl du monstre
- **Cumulatif** avec d'autres réductions de résistance

#### Mana Burn

- Certains monstres volent du mana lors des attaques
- Quantité : fixe ou proportionnelle
- Contre-jeu : items "Cannot be Frozen" n'aide pas ; besoin de mana élevé ou
  de régénération rapide

#### Stone Skin (affix monstre)

- Réduit les dommages physiques reçus par le monstre
- Ne s'applique pas aux dommages élémentaires ou magiques
- Valeur : réduction significative mais pas immunité totale

#### Extra Fast

- Augmente la vitesse d'attaque et de déplacement du monstre de 50-100%
- Ne change pas les mécaniques de combat, juste les timings

#### Extra Strong

- Augmente les dommages d'attaque du monstre (×2 dommages environ)

#### Spectral Hit

- Le monstre inflige un type de dommage élémentaire **aléatoire** à chaque coup
- Types possibles : Fire, Cold, Lightning, Poison
- Utile : le joueur doit avoir des résistances élémentaires équilibrées

#### Fire/Cold/Lightning Enchanted

- À la mort, le monstre explose en infligeant des dommages de l'élément correspondant
- Zone d'effet autour du point de mort
- **Danger critique** en Hell où les résistances du joueur sont réduites (-100%)

#### Cursed (affix monstre Amplify Damage)

- Le monstre maudit les joueurs qui le touchent
- Effet : Amplify Damage appliqué au joueur (double les dommages physiques reçus)
- Durée : courte, mais se réapplique à chaque coup du monstre

#### Teleportation (affix monstre)

- Le monstre se téléporte aléatoirement quand touché ou après un délai
- Frustrant pour les builds à courte portée
- Les Fanatisme-Boss combinés avec Teleport sont particulièrement dangereux

---

## 7. Systèmes de résistance avancés

### 7.1 Résistances des joueurs

**Cap de base :** 75% pour tous les éléments (Fire, Cold, Lightning, Poison)

**Modificateurs du cap :**

| Source                            | Bonus au cap |
|-----------------------------------|-------------|
| "+x% Maximum Fire/Cold/Light/Poison Resist" (items) | +x% (empilable) |
| Salvation Aura (Paladin, lvl 20) | +5% all max resist |
| Maximum global possible           | 95%         |

**Pénalités de difficulté (s'appliquent à la résistance actuelle) :**

| Difficulté | Pénalité toutes résistances |
|------------|---------------------------|
| Normal     | 0%                        |
| Nightmare  | -40%                      |
| Hell       | -100%                     |

Exemple : Un personnage avec 75% résistance Feu en Normal aura :
- Nightmare : 75 - 40 = 35% résistance Feu
- Hell : 75 - 100 = -25% résistance Feu (amplifie les dommages !)

La pénalité s'applique à la **résistance finale** (après items et bonifications).

**Résistance minimale :** -100% (plancher absolu).

### 7.2 Immunités des monstres

Un monstre devient **immune** à un élément quand sa résistance dépasse **99%**.
Les dommages de l'élément concerné sont réduits à 0 (aucun dommage infligé).

### 7.3 Bris d'immunité

Seuls **Conviction** (Paladin) et **Lower Resist** (Necromancer) peuvent briser
une immunité. Ils fonctionnent à **1/5 de leur efficacité normale** contre les immunités.

**Formule :**

```
Si resist_monstre > 99 (immune) :
    reduction_effective = reduction_du_skill / 5
    resist_finale = resist_monstre - reduction_effective

Si resist_finale ≤ 99 → immunité brisée
    → les dommages de l'élément sont à nouveau infligés normalement

Si resist_finale reste > 99 → toujours immune
```

**Exemples détaillés :**

*Exemple 1 — Immunité NON brisée :*
```
Monstre : 110% Fire resist (immune)
Lower Resist niveau 20 : -80% resist
Reduction effective = -80 / 5 = -16
Resist finale = 110 - 16 = 94%
→ Immunité BRISÉE (94 ≤ 99)
→ Le monstre subit les dommages Fire avec 94% résistance
```

*Exemple 2 — Immunité brisée par Conviction :*
```
Monstre : 130% Lightning resist (immune)
Conviction niveau 25 : -150% resist
Reduction effective = -150 / 5 = -30
Resist finale = 130 - 30 = 100%
→ Toujours immune (100 ≥ 100) ... ou pile exactement à la limite ?
Note : le seuil est STRICTEMENT > 99%, donc 100% = IMMUNE
```

*Exemple 3 — Double débuff :*
```
Monstre : 120% Cold resist (immune)
Conviction -100% + Lower Resist -70%
Total = -170%
Reduction effective = -170 / 5 = -34
Resist finale = 120 - 34 = 86%
→ Immunité BRISÉE (86 ≤ 99)
Note : les deux skills contribuent à 1/5 efficacité en zone immune
```

**Règles supplémentaires :**
- Une fois l'immunité brisée, les réductions supplémentaires s'appliquent
  **à pleine efficacité** sur la portion excédant 99% déjà traitée
- Le plancher de résistance après bris reste -100%
- Cold Mastery et Penetration (Fireball) ne brisent **pas** les immunités,
  ils s'appliquent seulement si l'immunité est déjà brisée

### 7.4 Résistances des joueurs — absorbances

L'absorbance (Absorb) est une mécanique distincte des résistances :

```
DmgAfterAbsorb = max(0, DmgAfterResist - FlatAbsorb)
                 × (1 - %Absorb/100)
```

Ou dans certains cas l'ordre est inversé selon la source.
Les items "Absorb x% Fire damage" réduisent le dommage après résistance.
Maximum d'absorption : peut théoriquement rendre des dommages négatifs (soins).

---

## 8. Magic Find

### 8.1 Mécanisme général

Le Magic Find (MF) n'affecte **pas** ce qui tombe (les loot tables),
mais la **qualité** de ce qui tombe (Normal → Magic → Rare → Set → Unique).

À chaque drop, le jeu vérifie successivement :
1. Est-ce un Unique ? (base chance ~1/400 selon l'item)
2. Sinon, est-ce un Set ? (base chance ~1/160)
3. Sinon, est-ce un Rare ? (base chance ~1/100)
4. Sinon, est-ce un Magic ? (base chance ~1/34)
5. Sinon : Normal (base)

Le MF modifie ces chances de base.

### 8.2 Formules MF avec diminishing returns

**Magic items :** Aucun diminishing return. MF appliqué linéairement.

**Rare items :**
```
EffectiveMF_Rare = MF × 600 / (MF + 600)
```

**Set items :**
```
EffectiveMF_Set = MF × 500 / (MF + 500)
```

**Unique items :**
```
EffectiveMF_Unique = MF × 250 / (MF + 250)
```

### 8.3 Tableaux de valeurs

| MF%  | Eff. Unique | Eff. Set | Eff. Rare |
|------|-------------|----------|-----------|
|    0 |           0 |        0 |         0 |
|   50 |          41 |       45 |        47 |
|  100 |          71 |       77 |        80 |
|  150 |          94 |      102 |       107 |
|  200 |         111 |      122 |       129 |
|  300 |         136 |      150 |       160 |
|  400 |         154 |      170 |       182 |
|  500 |         167 |      185 |       198 |
|  700 |         183 |      204 |       219 |
| 1000 |           — |      238 |       257 |

**Conclusion pratique :**
- Jusqu'à ~200% MF : gains significatifs pour les uniques
- 200–500% : gains diminuants mais encore valables
- Au-delà de 500% : rendements très faibles sur les uniques (~15% de gain
  pour doubler le MF de 500% à 1000%)

### 8.4 Interférences avec les loot tables

Le MF **ne change pas** :
- Quels items peuvent tomber (déterminé par le mlvl et les loot tables)
- La quantité d'items droppés
- Le nombre de drops par kill

Le MF **change uniquement** :
- La probabilité qu'un item qui allait être Normal devienne Magic, Rare, Set, ou Unique

---

## 9. Expérience et leveling

### 9.1 Formule XP par niveau (approximation)

Les valeurs exactes sont encodées dans la table `Experience.txt` du jeu.
Les valeurs ci-dessous sont exactes pour D2 v1.14d.

| Niveau | XP requis    | Niveau | XP requis      |
|--------|-------------|--------|----------------|
|      1 |           0 |     51 |   18,262,891   |
|      2 |         500 |     52 |   19,800,432   |
|      3 |       1,500 |     53 |   21,429,957   |
|      4 |       3,750 |     54 |   23,154,706   |
|      5 |       7,875 |     55 |   24,978,246   |
|      6 |      14,175 |     56 |   26,903,395   |
|      7 |      22,680 |     57 |   28,933,237   |
|      8 |      32,886 |     58 |   31,071,136   |
|      9 |      44,396 |     59 |   33,319,738   |
|     10 |      57,715 |     60 |   35,681,972   |
|     15 |     156,250 |     70 |   60,000,000   |
|     20 |     483,125 |     80 |  151,250,000   |
|     25 |   1,250,000 |     85 |  225,000,000   |
|     30 |   2,750,000 |     90 |  360,000,000   |
|     35 |   5,250,000 |     95 |  550,000,000   |
|     40 |   8,750,000 |     97 |  750,000,000   |
|     45 |  13,250,000 |     98 |  900,000,000   |
|     50 |  18,000,000 |     99 | 3,520,485,254  |

> Le niveau 99 nécessite ~3.5 milliards d'XP cumulés. La progression aux
> niveaux 90+ est exponentiellement lente par design.

### 9.2 Malus/Bonus d'XP selon l'écart de niveau

```
Si |clvl - mlvl| ≤ 5 : XP × 1.0 (plein XP)
Si |clvl - mlvl| = 6 : XP × 0.81
Si |clvl - mlvl| = 7 : XP × 0.62
Si |clvl - mlvl| = 8 : XP × 0.43
Si |clvl - mlvl| = 9 : XP × 0.24
Si |clvl - mlvl| ≥ 10 : XP × 0.05 (5% minimum)
```

**Cas : monstre trop faible (mlvl < clvl - 10) :**
Seulement 5% de l'XP normal. Pas de sens à farmer des zones de bas niveau.

**Cas : monstre trop fort (mlvl > clvl + 10) :**
Pas de bonus XP supplémentaire au-delà du plein XP. Le malus ne s'applique
que dans le sens joueur-plus-fort-que-monstre.

### 9.3 Partage d'XP en groupe

```
XP_total_monstre = XP_base × 1.35  (si ≥1 coéquipier dans la même zone)

XP_joueurI = XP_total × (clvl_I / sum(clvl_J pour J dans le groupe))
```

**Conditions du bonus de groupe :**
- Un coéquipier doit être dans la **même zone nommée** (même level de la map)
- Les joueurs hors de 2 écrans du kill ne reçoivent pas d'XP
- Le joueur trop loin est exclu du calcul mais peut recevoir un minimum

**Note :** La répartition par niveau favorise les joueurs de haut niveau dans
un groupe mixte, ce qui crée une dynamique de powerleveling naturelle.

### 9.4 Perte d'XP à la mort

| Difficulté | Perte XP                    | Récupération via corpse |
|------------|-----------------------------|------------------------|
| Normal     | 0%                          | N/A                    |
| Nightmare  | 5% de l'XP vers level suiv. | 75% récupéré si corpse |
| Hell       | 10% de l'XP vers level suiv.| 75% récupéré si corpse |

**Important :** On ne peut jamais descendre de niveau à cause d'une mort.
La perte est limitée à l'XP dans le niveau actuel (plancher = 0 XP dans ce niveau).

---

## 10. Stamina — Système détaillé

### 10.1 Fonctionnement général

- **Marcher** : Ne consomme pas de stamina. Vitesse de déplacement réduite (~60% du sprint).
- **Courir** : Consomme de la stamina progressivement.
- **Stamina à 0** : Le personnage revient automatiquement en **walk**. Vitesse réduite.

### 10.2 Consommation de stamina

La consommation de stamina pendant le sprint dépend de :
- Type d'armure équipée
- Classe du personnage

| Condition              | Drain multiplier |
|------------------------|-----------------|
| Armure légère / Medium | ×1.0            |
| Armure lourde          | ×2.0            |
| Assassin + claw        | ×1.0            |

### 10.3 Vitesses de régénération

```
Stamina_RegenPerFrame = MaxStamina / 256  (en marche ou immobile)
```

- Pleine régénération en ~10.24 secondes (256 frames) en marchant
- En courant : aucune régénération
- En idle (debout): ~20.48 secondes pour pleine régénération

### 10.4 Faster Run/Walk (FRW)

FRW augmente la **vitesse de déplacement** (pas la stamina elle-même).
Il n'y a pas de table de breakpoints pour FRW — c'est un bonus linéaire.

```
Speed_effective = BaseSpeed × (1 + FRW/100)
```

Plafonné à une vitesse maximale hardcodée par le moteur.

---

## 11. Niveaux de zones et items

### 11.1 Définitions

| Terme | Définition                                                           |
|-------|----------------------------------------------------------------------|
| clvl  | Character Level — niveau du joueur                                   |
| mlvl  | Monster Level — niveau du monstre                                    |
| alvl  | Area Level — niveau de la zone (détermine le mlvl des monstres)      |
| ilvl  | Item Level — niveau de l'item (généralement = mlvl du monstre qui drope) |
| qlvl  | Quality Level — niveau de qualité intrinsèque du type d'objet        |
| ALvl  | Affix Level — niveau qui détermine quels affixes peuvent spawner     |

**ilvl selon la source :**

| Source       | ilvl calculé                     |
|--------------|----------------------------------|
| Monster drop | ilvl = mlvl                      |
| Chest / Pot  | ilvl = alvl                      |
| Shop buy     | ilvl = clvl + 5                  |
| Gambling     | ilvl = aléatoire [clvl-5, clvl+4] |
| Crafting     | ilvl = floor((ilvl_item + clvl) / 2) |

### 11.2 Formule Affix Level (ALvl)

```
Si ilvl < (99 - floor(qlvl / 2)) :
    ALvl = ilvl - floor(qlvl / 2)
Sinon :
    ALvl = 2 × ilvl - 99
```

L'ALvl détermine quels affixes (préfixes/suffixes) peuvent spawner sur l'item.
Plus l'ALvl est élevé, plus les affixes puissants sont accessibles.

**Magic Level (maglvl) de certains items :**

| Type d'item | maglvl |
|-------------|--------|
| Baguette normale | 1 |
| Bâton normal     | 1 |
| Orbe normal      | 1 |
| Circlet          | 3 |
| Coronet          | 8 |
| Tiara            | 13 |
| Diadème          | 18 |

Le maglvl s'ajoute à l'ilvl pour le calcul des affixes, permettant des affixes
plus puissants sur ces types spécifiques.

### 11.3 Players X et impact sur le loot

`/players X` (ou lobby multiplayer avec X joueurs) :

- **Impact sur le spawn :** Augmente le nombre de monstres spawnés proportionnellement
- **Impact sur le loot :** Les tableaux de drops par monstre ne changent PAS
  mais le **nombre de monstres tués** augmente, donc plus de drops globalement
- **Impact sur l'XP :** Réduction de l'XP selon le nombre de joueurs
  (XP divisée entre joueurs proches)
- **Impact sur la difficulté :** Monstres avec plus de vie et de dommages

Les drops d'un monstre individuel en `/players 8` sont **identiques** à `/players 1`.
Seul le volume global augmente grâce aux monstres supplémentaires.

---

## 12. Schémas d'implémentation TOML/Rust

### 12.1 Encodage des breakpoints en TOML

Les breakpoints sont des **tables de lookup**, non des formules dynamiques.
Pour chaque animation (FCR/FHR/FBR), on stocke un tableau de paires (threshold, frames).

```toml
# mge-arpg-combat/data/breakpoints/fcr.toml

[fcr.sorceress.normal]
# Paires (fcr_threshold, frames_requis)
# Format : tableau trié par threshold croissant
breakpoints = [
  { threshold =   0, frames = 13 },
  { threshold =   9, frames = 12 },
  { threshold =  20, frames = 11 },
  { threshold =  37, frames = 10 },
  { threshold =  63, frames =  9 },
  { threshold = 105, frames =  8 },
  { threshold = 200, frames =  7 },
]

[fcr.sorceress.lightning]
breakpoints = [
  { threshold =   0, frames = 19 },
  { threshold =   7, frames = 18 },
  { threshold =  15, frames = 17 },
  { threshold =  23, frames = 16 },
  { threshold =  35, frames = 15 },
  { threshold =  52, frames = 14 },
  { threshold =  78, frames = 13 },
  { threshold = 117, frames = 12 },
  { threshold = 194, frames = 11 },
]

[fcr.paladin.normal]
breakpoints = [
  { threshold =   0, frames = 15 },
  { threshold =   9, frames = 14 },
  { threshold =  18, frames = 13 },
  { threshold =  30, frames = 12 },
  { threshold =  48, frames = 11 },
  { threshold =  75, frames = 10 },
  { threshold = 125, frames =  9 },
]
```

### 12.2 Lookup de breakpoint en Rust

```rust
/// Recherche dichotomique dans une table de breakpoints.
/// Retourne le nombre de frames pour la valeur `stat` donnée.
pub fn lookup_breakpoint(table: &[(u16, u8)], stat: u16) -> u8 {
    // table = &[(threshold, frames)], trié par threshold croissant
    // On cherche le dernier threshold ≤ stat
    let mut frames = table[0].1;
    for &(threshold, f) in table {
        if stat >= threshold {
            frames = f;
        } else {
            break;
        }
    }
    frames
}

// Exemple d'utilisation :
// let fcr_table = config.fcr.sorceress.normal.breakpoints.as_slice();
// let frames = lookup_breakpoint(fcr_table, player_fcr);
```

### 12.3 Structure TOML pour les stats de classes

```toml
# mge-arpg-stats/data/classes/sorceress.toml

[class.sorceress]
id = "sorceress"

[class.sorceress.base_stats]
strength  = 10
dexterity = 25
vitality  = 10
energy    = 35

[class.sorceress.base_pools]
life    = 40
mana    = 35
stamina = 74

[class.sorceress.per_vitality]
life    = 2.0
stamina = 1.0

[class.sorceress.per_energy]
mana = 2.0

[class.sorceress.per_level]
life    = 1.0
mana    = 2.0
stamina = 1.0

[class.sorceress.base_ar_constant]
value = 7  # ClassBaseAR pour la formule AR = (DEX - 7) * 5 + constant
```

### 12.4 Structure TOML pour les types d'armes et bonus de dommage

```toml
# mge-arpg-combat/data/weapon_types.toml

[[weapon_type]]
id = "sword_1h"
str_factor = 100   # 1% dommage par point de STR
dex_factor = 0
wsm = 0           # Weapon Speed Modifier de base (variable par item)

[[weapon_type]]
id = "mace_scepter_wand"
str_factor = 110   # 1.1% dommage par point de STR
dex_factor = 0
wsm = -20

[[weapon_type]]
id = "bow"
str_factor = 0
dex_factor = 100   # 1% dommage par point de DEX
wsm = 0

[[weapon_type]]
id = "dagger"
str_factor = 50
dex_factor = 50
wsm = -20

[[weapon_type]]
id = "claw_katar"
str_factor = 75
dex_factor = 75
wsm = -20

[[weapon_type]]
id = "javelin_throwing"
str_factor = 50
dex_factor = 50
wsm = 0
throw_wsm_penalty = 30   # Pénalité supplémentaire quand lancée
```

### 12.5 Pipeline de calcul de dommage en Rust (ECS)

```rust
/// Composants ECS nécessaires pour un calcul de dommage complet
/// dans mge-arpg-combat

pub struct WeaponStats {
    pub min_dmg: f32,
    pub max_dmg: f32,
    pub is_ethereal: bool,
    pub ed_on_weapon: f32,   // Enhanced Damage on-weapon (%)
    pub flat_min: f32,
    pub flat_max: f32,
}

pub struct AttackerStats {
    pub strength: u32,
    pub dexterity: u32,
    pub ed_off_weapon: f32,  // Somme de tous les ED hors-arme
    pub weapon_type_id: &'static str,
    pub critical_strike_chance: f32,
    pub deadly_strike_chance: f32,
    pub open_wounds_chance: f32,
    pub crushing_blow_chance: f32,
    pub leech_life_pct: f32,
    pub leech_mana_pct: f32,
}

pub fn calculate_physical_damage(
    weapon: &WeaponStats,
    attacker: &AttackerStats,
    weapon_type: &WeaponTypeData,
    rng: &mut impl Rng,
) -> PhysicalDamageResult {
    // Étape 1 : Base
    let mut min = weapon.min_dmg;
    let mut max = weapon.max_dmg;

    // Étape 2 : Éthéré
    if weapon.is_ethereal {
        min *= 1.5;
        max *= 1.5;
    }

    // Étape 3 : ED on-weapon
    let ed_on = 1.0 + weapon.ed_on_weapon / 100.0;
    min *= ed_on;
    max *= ed_on;

    // Étape 4 : Flat damage
    min += weapon.flat_min;
    max += weapon.flat_max;

    // Étape 5 : ED off-weapon
    let ed_off = 1.0 + attacker.ed_off_weapon / 100.0;
    min *= ed_off;
    max *= ed_off;

    // Étape 6 : STR/DEX bonus
    let str_bonus = attacker.strength as f32 * weapon_type.str_factor / 100.0;
    let dex_bonus = attacker.dexterity as f32 * weapon_type.dex_factor / 100.0;
    let attr_bonus = 1.0 + (str_bonus + dex_bonus) / 100.0;
    min *= attr_bonus;
    max *= attr_bonus;

    // Roll du dommage final
    let base_phys = rng.gen_range(min..=max) as u32;

    // Étape 7 : Critical Strike / Deadly Strike
    let p_crit = 1.0 - (1.0 - attacker.critical_strike_chance / 100.0)
                     * (1.0 - attacker.deadly_strike_chance / 100.0);
    let is_crit = rng.gen_bool(p_crit as f64);
    let final_phys = if is_crit { base_phys * 2 } else { base_phys };

    PhysicalDamageResult {
        damage: final_phys,
        is_critical: is_crit,
    }
}
```

### 12.6 Encodage des formules Open Wounds en TOML

```toml
# mge-arpg-combat/data/effects/open_wounds.toml

[open_wounds]
duration_frames = 200          # 8 secondes × 25 fps
max_stacks = 1                 # Vanilla D2 : 1 seul stack, refresh

# Formule DpF = (A × clvl + B) / 256
# Définie par tranches de niveaux
[[open_wounds.formula_segments]]
clvl_min = 1
clvl_max = 15
a_coeff = 9
b_coeff = 31

[[open_wounds.formula_segments]]
clvl_min = 16
clvl_max = 30
a_coeff = 18
b_coeff = -104

[[open_wounds.formula_segments]]
clvl_min = 31
clvl_max = 45
a_coeff = 27
b_coeff = -374

[[open_wounds.formula_segments]]
clvl_min = 46
clvl_max = 60
a_coeff = 36
b_coeff = -779

[[open_wounds.formula_segments]]
clvl_min = 61
clvl_max = 99
a_coeff = 45
b_coeff = -1319

# Modificateurs par type de cible
[open_wounds.target_modifiers]
normal_melee   = [1, 1]     # fraction num/denom
normal_ranged  = [1, 2]
champ_boss_melee  = [1, 2]
champ_boss_ranged = [1, 4]
player_melee   = [1, 4]
player_ranged  = [1, 8]
```

### 12.7 Stacking des modificateurs de stats

**Règle générale :**

```
ED (Enhanced Damage) :
- On-weapon : multiplicatif avec off-weapon, additif entre plusieurs sources on-weapon
- Off-weapon : ADDITIF entre toutes les sources off-weapon, puis appliqué une fois

Résistances :
- Items : ADDITIF (tous sommés)
- Pénalités de difficulté : ADDITIF avec les résistances

FCR / FHR / FBR / IAS :
- ADDITIF entre toutes sources (excepté IAS → converti en EIAS par diminishing returns)

Life / Mana bonus % :
- ADDITIF entre toutes sources items/skills
- Appliqué sur la valeur de base
```

**Exemple de stacking multiplicatif off-weapon ED :**

```
// INCORRECT (multiplicatif — vanilla D2 ne fait PAS ça pour off-weapon) :
total_mult = 1.2 × 1.5 × 1.3 = 2.34

// CORRECT (additif pour off-weapon) :
total_ed = 20% + 50% + 30% = 100%
total_mult = 1 + 100/100 = 2.0
```

---

## Sources de référence

- [Arreat Summit — Blizzard Official](http://classic.battle.net/diablo2exp/)
- [Diablo 2 Wiki (diablo2.diablowiki.net)](https://diablo2.diablowiki.net/)
- [Maxroll D2 Resources](https://maxroll.gg/d2/resources/)
- [Breakpoints — Diablo Wiki](https://diablo2.diablowiki.net/Breakpoints)
- [Attack Rating — Diablo Wiki](https://diablo2.diablowiki.net/Attack_Rating)
- [Open Wounds — Diablo Wiki](https://diablo2.diablowiki.net/Open_Wounds)
- [Resistance — Diablo Wiki](https://diablo2.diablowiki.net/Resistance)
- [Magic Find DR — Diablo Wiki](https://diablo2.diablowiki.net/Magic_find_diminishing_returns)
- [Experience — Diablo Wiki](https://diablo2.diablowiki.net/Experience)
- [Maxroll Breakpoints & Animations](https://maxroll.gg/d2/resources/breakpoints-animations)
- [Maxroll Life & Mana Mechanics](https://maxroll.gg/d2/resources/life-mana-mechanics)
- [Maxroll Damage Calculation](https://maxroll.gg/d2/resources/damage-calculation)
- [Maxroll Block Mechanics](https://maxroll.gg/d2/resources/block-mechanics)
- [Maxroll Hit Chance Mechanics](https://maxroll.gg/d2/resources/hit-chance-mechanics)
- [IAS Formula — mann.org](https://www.mannm.org/d2library/faqtoids/ias_eng.html)
- [Gamer Guides — Stat Points](https://www.gamerguides.com/diablo-ii-resurrected/guide/characters/builds/stat-points)
- [Project Diablo 2 — Game Mechanics](https://wiki.projectdiablo2.com/wiki/Game_Mechanics) (PD2 uniquement — ne pas implémenter sans décision)
