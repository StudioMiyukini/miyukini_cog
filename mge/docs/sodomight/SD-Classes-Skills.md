# SD - Classes & Skills — Référence Exhaustive

## Contexte

Document de référence technique pour le projet **Sodomight** — clone fidèle de Diablo 2: Lord of Destruction (v1.14d/D2R 2.8) intégré au moteur MGE (Miyukini Game Engine, ECS archetype, data-driven TOML).

Les mécaniques de jeu reproduisent Diablo 2 à l'identique. Les noms propres liés à l'univers Blizzard/Diablo sont remplacés par des noms Sodomight originaux. Les termes techniques anglais (FCR, FHR, IAS, etc.) sont conservés tels quels car ils désignent des mécaniques universelles.

## Portée / Scope

Ce document couvre :
- Les 7 classes jouables avec stats complètes
- Les 210 skills (30 par classe × 7 classes), organisés en 3 arbres par classe
- Les formules de dégâts par niveau
- Les synergies entre skills
- Les tables de breakpoints FCR / FHR / FBR par classe
- Des exemples TOML MGE pour intégration engine
- Le nommage Sodomight pour chaque classe et skill

---

## Table des matières

1. [Tableau global des stats de classe](#1-tableau-global-des-stats-de-classe)
2. [Amazon → Sarith (Archère)](#2-amazon--sarith-larchère)
3. [Necromancer → Mortecian](#3-necromancer--mortecian)
4. [Barbarian → Ravageur](#4-barbarian--ravageur)
5. [Sorceress → Arcaniste](#5-sorceress--larcaniste)
6. [Paladin → Croisé-Solaire](#6-paladin--croisé-solaire)
7. [Druid → Animiste](#7-druid--lanimiste)
8. [Assassin → Ombrelame](#8-assassin--lombrelame)
9. [Breakpoints — Tables complètes](#9-breakpoints--tables-complètes)
10. [Exemples TOML MGE](#10-exemples-toml-mge)

---

## 1. Tableau global des stats de classe

### 1.1 Attributs de départ (Level 1)

| Classe (D2) | Nom Sodomight | STR | DEX | VIT | ENE | Life | Mana | Stamina |
|-------------|---------------|-----|-----|-----|-----|------|------|---------|
| Amazon | Sarith | 20 | 25 | 20 | 15 | 50 | 15 | 84 |
| Necromancer | Mortecian | 15 | 25 | 15 | 25 | 45 | 25 | 79 |
| Barbarian | Ravageur | 30 | 20 | 25 | 10 | 55 | 10 | 92 |
| Sorceress | Arcaniste | 10 | 25 | 10 | 35 | 40 | 35 | 74 |
| Paladin | Croisé-Solaire | 25 | 20 | 25 | 15 | 55 | 15 | 89 |
| Druid | Animiste | 15 | 20 | 25 | 20 | 55 | 20 | 84 |
| Assassin | Ombrelame | 20 | 20 | 20 | 25 | 50 | 25 | 95 |

### 1.2 Gains par niveau

| Classe | Life/lvl | Mana/lvl | Stamina/lvl |
|--------|----------|----------|-------------|
| Sarith | +2 | +1.5 | +1 |
| Mortecian | +1.5 | +2 | +1 |
| Ravageur | +2 | +1 | +1 |
| Arcaniste | +2 | +2 | +1 |
| Croisé-Solaire | +3 | +1.5 | +1 |
| Animiste | +1.5 | +2 | +1 |
| Ombrelame | +2 | +1.5 | +1.25 |

> Note : les valeurs Paladin (+3 life/lvl) sont les plus hautes du jeu, reflétant son rôle de tank.

### 1.3 Gains par point d'attribut

| Classe | Life/VIT | Mana/ENE | Stamina/VIT | Mana supplémentaire |
|--------|----------|----------|-------------|---------------------|
| Sarith | +3 | +1.5 | +1 | — |
| Mortecian | +3 | +1.5 | +1 | — |
| Ravageur | +4 | +1.5 | +1 | — |
| Arcaniste | +3 | +1.5 | +1 | — |
| Croisé-Solaire | +3 | +1.5 | +1 | — |
| Animiste | +2 | +2 | +1 | — |
| Ombrelame | +3 | +1.75 | +1.5 | — |

> Note : le Ravageur obtient 4 Life par point de Vitalité (le plus élevé), et l'Animiste obtient 2 Mana par point d'Energie (plus efficient).

### 1.4 Modificateur de blocage (Block Chance Modifier)

Le blocage effectif est calculé ainsi :
```
Chance de blocage = (Shield Block% * Class Modifier * 2) / (Character Level + 25)
Cap absolu : 75%
```

| Classe | Block Chance Modifier |
|--------|-----------------------|
| Sarith | 25% |
| Mortecian | 20% |
| Ravageur | 25% |
| Arcaniste | 20% |
| Croisé-Solaire | 30% |
| Animiste | 20% |
| Ombrelame | 25% |

Le Croisé-Solaire (Paladin) a le modificateur le plus élevé (30%), facilitant l'atteinte du cap de 75%.

---

## 2. Amazon → Sarith, l'Archère

### Description de classe
La Sarith est une guerrière agile spécialisée dans l'arc, le javelot et les techniques passives défensives. Elle excelle à distance avec des projectiles enchantés et peut invoquer une puissante alliée (Valkyrie). Classe de choix pour les builds à dégâts élémentaux à distance.

### Nommage Sodomight
- **Amazon** → **Sarith** (guerrière des hauts plateaux, nomade)
- L'univers D2 parle d'Amazones de l'île de Skovos — remplacé par les Sariths, tribus nomades des Plaines-Hautes de Solmer.

---

### Arbre 1 : Flèches & Carreaux (Bow and Crossbow Skills)

#### 2.1.1 Flèche Magique (Magic Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Magic Arrow |
| **Nom Sodomight** | Trait Ensorcelé |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 1.5 (base) — ne s'améliore pas |
| **Type de dégâts** | Magique (non-résistible) |
| **Range** | Portée d'arc/arbalète |
| **Frames de cast** | Dépend IAS |

**Description mécanique :** Crée une flèche/carreau magique qui inflige des dégâts supplémentaires magiques. Les dégâts magiques contournent les résistances élémentales. Pas de composante physique — 100% magique.

**Formule de dégâts par niveau :**
| Niveau | Dégâts min | Dégâts max |
|--------|-----------|-----------|
| 1 | 3 | 4 |
| 5 | 9 | 12 |
| 10 | 18 | 23 |
| 15 | 28 | 35 |
| 20 | 38 | 47 |

Formule : min = 2 + (slvl × 2), max = 3 + (slvl × 2.2) (approximation)

**Synergies reçues :** Aucune.
**Synergies données :** Aucune.
**Niveau 20 (hard cap) :** ~38-47 dégâts magiques supplémentaires.
**Avec +skills :** Scaling continu sans cap dur sur les dégâts.
**Notes gameplay :** Utile pour régénérer les flèches physiques (génère des flèches infinies). Quasi inutilisé en builds endgame mais essentiel pour ne jamais manquer de munitions. Aucune interaction build.

---

#### 2.1.2 Flèche de Feu (Fire Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Arrow |
| **Nom Sodomight** | Trait Ardent |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 3 |
| **Type de dégâts** | Physique + Feu (conversion partielle) |
| **Range** | Portée d'arc/arbalète |

**Description mécanique :** Enchante une flèche pour qu'elle inflige des dégâts de feu supplémentaires. Depuis le patch 1.09, une portion des dégâts physiques est convertie en dégâts de feu. Le pourcentage de conversion augmente avec le niveau du skill.

**Formule de dégâts par niveau :**
| Niveau | Feu min | Feu max | % conv. physique→feu |
|--------|---------|---------|----------------------|
| 1 | 4 | 6 | 3% |
| 5 | 20 | 28 | 11% |
| 10 | 44 | 62 | 21% |
| 15 | 68 | 96 | 31% |
| 20 | 92 | 130 | 41% |

**Synergies reçues :**
- Exploding Arrow : +12% Fire Damage par niveau de base

**Synergies données :**
- Exploding Arrow reçoit des dégâts via cette synérgie

**Niveau 20 :** ~92-130 feu + 41% conversion physique.
**Notes gameplay :** Base du build "Bowazon de Feu". Combiné à Exploding Arrow (maxé), les synergies font exploser les dégâts de feu. Peu utilisé seul mais essentiel comme synérgie.

---

#### 2.1.3 Flèche de Glace (Cold Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cold Arrow |
| **Nom Sodomight** | Trait Givrant |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 6 |
| **Prérequis** | Trait Ardent (Fire Arrow) |
| **Coût mana** | 3.5 |
| **Type de dégâts** | Physique + Froid (conversion + chill) |
| **Range** | Portée d'arc/arbalète |
| **Effet spécial** | Ralentit les ennemis (Chill) |

**Description mécanique :** Enchante une flèche pour inflige des dégâts de froid et ralentit les ennemis touchés. Similaire à Fire Arrow : conversion croissante des dégâts physiques en froid.

**Formule de dégâts :**
| Niveau | Froid min | Froid max | Durée chill | % conv. |
|--------|-----------|-----------|-------------|---------|
| 1 | 2 | 4 | 2s | 3% |
| 5 | 12 | 20 | 2.6s | 11% |
| 10 | 28 | 44 | 3.3s | 21% |
| 15 | 44 | 68 | 4s | 31% |
| 20 | 60 | 92 | 4.6s | 41% |

Durée de freeze/chill : divisée par 2 en Nightmare, par 4 en Hell.

**Synergies reçues :**
- Ice Arrow : +12% Cold Damage par niveau de base

**Synergies données :**
- Freezing Arrow reçoit +12% Cold Damage par niveau de Cold Arrow

**Notes gameplay :** Synérgie principale pour Freezing Arrow. Le chill aide à contrôler les packs mais la durée s'effondre en Hell.

---

#### 2.1.4 Flèches Multiples (Multiple Shot)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Multiple Shot |
| **Nom Sodomight** | Volée de Traits |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 6 |
| **Prérequis** | Trait Ensorcelé (Magic Arrow) |
| **Coût mana** | 3 (base), +0.25/lvl |
| **Type de dégâts** | Physique (réduit à 75%) |
| **Range** | Portée d'arc/arbalète |
| **Effet spécial** | Tire plusieurs flèches en éventail |

**Description mécanique :** Tire un éventail de flèches ou carreaux en une seule attaque. Le nombre de flèches commence à 2 et augmente. Les dégâts de chaque flèche individuelle sont réduits à 75% des dégâts normaux. Toutes les propriétés de l'arc/arbalète s'appliquent à chaque flèche.

**Formule nombre de flèches :**
| Niveau | Flèches |
|--------|---------|
| 1 | 3 |
| 4 | 4 |
| 8 | 5 |
| 12 | 6 |
| 16 | 7 |
| 20 | 8 |
| 24 | 9 |

Formula : 2 + floor(slvl / 4) flèches latérales + 1 centrale = total variable.

**Synergies reçues :** Aucune synérgie directe de dégâts.
**Synergies données :** Aucune.
**Notes gameplay :** Excellent pour les builds physiques (Bowazon MS/GA). Ne bénéficie pas des synergies élémentales mais la base physique est forte. Le bonus d'Attack Rating augmente avec le niveau.

---

#### 2.1.5 Flèche Explosive (Exploding Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Exploding Arrow |
| **Nom Sodomight** | Trait Déflagrant |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 12 |
| **Prérequis** | Trait Ardent (Fire Arrow) |
| **Coût mana** | 5 (base) |
| **Type de dégâts** | Physique + Feu (AoE à l'impact) |
| **Range** | Portée d'arc, explosion de rayon ~2.6m |
| **Effet spécial** | Explosion de zone au point d'impact |

**Description mécanique :** La flèche explose à l'impact, infligeant des dégâts de feu en zone autour du point de contact. Les dégâts de feu bénéficient de la synérgie de Fire Arrow.

**Formule de dégâts :**
| Niveau | Feu min | Feu max |
|--------|---------|---------|
| 1 | 13 | 25 |
| 5 | 37 | 61 |
| 10 | 77 | 117 |
| 15 | 117 | 173 |
| 20 | 157 | 229 |

**Synergies reçues :**
- Fire Arrow : +12% Fire Damage par niveau de base
- Immolation Arrow : +12% Fire Damage par niveau de base

**Synergies données :**
- Fire Arrow : +12% Fire Damage (synérgie croisée)

**Notes gameplay :** Skill de choix pour le "Fire Bowazon". Les synergies croisées entre Fire Arrow et Exploding Arrow permettent de maximiser les deux pour des explosions massives.

---

#### 2.1.6 Flèche de Glace Pure (Ice Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Ice Arrow |
| **Nom Sodomight** | Trait Glacial |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 18 |
| **Prérequis** | Trait Givrant (Cold Arrow) |
| **Coût mana** | 4 |
| **Type de dégâts** | Physique + Froid (Freeze) |
| **Range** | Portée d'arc |
| **Effet spécial** | **Freeze** (immobilisation) sur la cible unique |

**Description mécanique :** Fige une cible sur place (Freeze, plus puissant que Chill). La durée de freeze est fixe mais réduite par les difficultés supérieures.

**Formule de dégâts :**
| Niveau | Froid min | Froid max | Durée freeze |
|--------|-----------|-----------|--------------|
| 1 | 6 | 10 | 2s |
| 5 | 22 | 34 | 2.6s |
| 10 | 46 | 70 | 3.3s |
| 15 | 70 | 106 | 4s |
| 20 | 94 | 142 | 4.6s |

Durée freeze : ÷2 Nightmare, ÷4 Hell.

**Synergies reçues :**
- Cold Arrow : +12% Cold Damage par niveau
- Freezing Arrow : +12% Cold Damage par niveau

**Synergies données :**
- Freezing Arrow : +12% Cold Damage par niveau
- Cold Arrow : +12% Cold Damage par niveau

**Notes gameplay :** Skill de support pour Freezing Arrow build. Maxé principalement pour les synergies qu'il apporte à Freezing Arrow.

---

#### 2.1.7 Flèche Guidée (Guided Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Guided Arrow |
| **Nom Sodomight** | Trait Traqueur |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 18 |
| **Prérequis** | Flèches Multiples (Multiple Shot) |
| **Coût mana** | 8 (base) |
| **Type de dégâts** | Physique (100% dégâts normaux) |
| **Range** | Portée illimitée (homing) |
| **Effet spécial** | **Auto-seeking** — la flèche suit la cible désignée |

**Description mécanique :** Tire une flèche enchantée qui traque automatiquement la cible désignée. Ne peut manquer sauf si la cible est hors de la map. Inflige 100% des dégâts de l'arc (contrairement à Multiple Shot à 75%).

**Formule :** Dégâts physiques intégraux de l'arc. Le skill ne scale pas directement en dégâts mais améliore le bonus d'Attack Rating.
| Niveau | AR Bonus |
|--------|----------|
| 1 | 0% |
| 5 | +30% |
| 10 | +65% |
| 20 | +135% |

**Synergies reçues :** Aucune.
**Synergies données :** Aucune.
**Notes gameplay :** Skill single-target de choix pour Bowazon physique (MultiShot + Guided Arrow combo). Jamais besoin de viser précisément. Inestimable contre les boss.

---

#### 2.1.8 Flèche Embrasée (Immolation Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Immolation Arrow |
| **Nom Sodomight** | Trait Embrasé |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 24 |
| **Prérequis** | Trait Déflagrant (Exploding Arrow) |
| **Coût mana** | 6 (base) |
| **Type de dégâts** | Feu (explosion + flaque de feu persistante) |
| **Range** | Portée d'arc, zone de feu de ~2.6m de rayon |
| **Effet spécial** | Crée une flaque de feu persistante pendant plusieurs secondes |

**Description mécanique :** La flèche explose à l'impact ET crée une flaque de feu persistante qui continue de brûler les ennemis. Dégâts initiaux d'explosion + dégâts par seconde de la flaque.

**Formule de dégâts :**
| Niveau | Explosion feu | Feu/sec (flaque) | Durée flaque |
|--------|---------------|-----------------|--------------|
| 1 | 25-50 | 62-125 | 4s |
| 5 | 65-118 | 148-255 | 4.4s |
| 10 | 128-223 | 283-468 | 4.9s |
| 20 | 253-433 | 550-895 | 6s |

**Synergies reçues :**
- Fire Arrow : +12% Fire Damage par niveau
- Exploding Arrow : +12% Fire Damage par niveau

**Synergies données :**
- Exploding Arrow : +12% Fire Damage par niveau

**Notes gameplay :** Excellent pour contrôler les zones. La flaque persiste et draine la vie des ennemis qui restent dessus. Synergies croisées avec Exploding Arrow et Fire Arrow.

---

#### 2.1.9 Tir en Rafale (Strafe)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Strafe |
| **Nom Sodomight** | Salve Rapide |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 24 |
| **Prérequis** | Flèches Multiples + Trait Traqueur (Multiple Shot + Guided Arrow) |
| **Coût mana** | 11 (base), +0.5/lvl |
| **Type de dégâts** | Physique (75% par flèche individuelle) |
| **Range** | Portée d'arc |
| **Effet spécial** | Tire rapidement jusqu'à 10 flèches sur tous les ennemis à portée |

**Description mécanique :** Tire jusqu'à 10 flèches/carreaux en succession rapide sur tous les ennemis à portée. La Sarith ne peut pas se déplacer pendant la durée du Strafe. Toutes les propriétés de l'arc s'appliquent à chaque flèche.

**Formule nombre de flèches max par cible :**
Max flèches par cible = 2 + floor(slvl / 4)

| Niveau | Flèches max/cible | AR Bonus |
|--------|-------------------|----------|
| 1 | 2 | +10% |
| 5 | 3 | +30% |
| 10 | 4 | +65% |
| 20 | 6 | +135% |

**Synergies reçues :** Aucune (dégâts physiques purs).
**Synergies données :** Aucune.
**Notes gameplay :** Skill signature de la Bowazon physique. Excellent DPS en zone, se combine idéalement avec des arcs à high damage. Le lock de mouvement est la principale contrainte. Incontournable dans MS/Strafe builds.

---

#### 2.1.10 Flèche Gelante (Freezing Arrow)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Freezing Arrow |
| **Nom Sodomight** | Trait Blizzard |
| **Arbre** | Flèches & Carreaux |
| **Niveau requis** | 30 |
| **Prérequis** | Trait Glacial (Ice Arrow) |
| **Coût mana** | 9 (base) |
| **Type de dégâts** | Froid (AoE — freeze de zone) |
| **Range** | Portée d'arc, explosion de ~5.3m de rayon |
| **Effet spécial** | Freeze de zone (tous les ennemis dans le rayon) |

**Description mécanique :** Tire une flèche enchantée qui explose en une nova de froid au point d'impact, gelant tous les ennemis dans le rayon. C'est l'une des rares compétences de freeze de zone dans le jeu.

**Formule de dégâts :**
| Niveau | Froid min | Froid max | Durée freeze |
|--------|-----------|-----------|--------------|
| 1 | 25 | 50 | 2s |
| 5 | 68 | 120 | 2.6s |
| 10 | 133 | 225 | 3.3s |
| 15 | 198 | 330 | 4s |
| 20 | 263 | 435 | 4.6s |

**Synergies reçues :**
- Cold Arrow : +12% Cold Damage par niveau
- Ice Arrow : +5% durée de freeze par niveau (en plus des dégâts)

**Synergies données :**
- Ice Arrow : +12% Cold Damage par niveau

**Notes gameplay :** Build "Freezing Arrow Bowazon" — l'un des builds les plus fun pour contrôler les foules. La combinaison Cold Arrow + Ice Arrow en synergies peut porter les dégâts à des niveaux élevés. Faiblesse : Fire Immune monstres résistants au froid en Hell.

---

### Arbre 2 : Passives & Magie (Passive and Magic Skills)

#### 2.2.1 Vision Intérieure (Inner Sight)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Inner Sight |
| **Nom Sodomight** | Oeil de Solmer |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 5 |
| **Type** | Debuff (Défense réduite) |
| **Range** | ~9m de rayon autour de la cible |
| **Durée** | 10s (base) |
| **Effet spécial** | Illumine les ennemis (visibilité), réduit leur défense |

**Description mécanique :** Illumine les ennemis dans la zone de cible et réduit leur valeur de défense, facilitant l'atteinte des coups physiques pour toute l'équipe.

**Formule — Réduction de Défense :**
| Niveau | Défense réduite de | Durée |
|--------|-------------------|-------|
| 1 | -40 | 10s |
| 5 | -80 | 15s |
| 10 | -130 | 22s |
| 20 | -230 | 37s |

**Synergies reçues :** Aucune.
**Notes gameplay :** Skill de support en groupe. La réduction de défense bénéficie à tout le groupe et aux mercenaires. Peu utilisé car 1 point suffit pour l'utilité de base.

---

#### 2.2.2 Frappe Critique (Critical Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Critical Strike |
| **Nom Sodomight** | Coup Critique |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |
| **Type** | Passif (chance de double dégâts) |

**Description mécanique :** Chaque attaque physique a une chance de faire le double de dégâts. Probabilité fixe par niveau, avec rendements décroissants.

**Formule — Probabilité de critique :**
| Niveau | Chance |
|--------|--------|
| 1 | 16% |
| 2 | 23% |
| 3 | 29% |
| 5 | 39% |
| 7 | 47% |
| 10 | 55% |
| 15 | 65% |
| 20 | 73% |

Formule approx : Chance = 100 × slvl / (slvl + 6)

**Synergies reçues :** Aucune.
**Synergies données :** Aucune.
**Notes gameplay :** Skill essentiel pour TOUT build Sarith physique. Rend les dégâts physiques beaucoup plus consistants. 5-7 points couvrent l'essentiel (>45%). Maxer pour build physique pur.

---

#### 2.2.3 Esquive (Dodge)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dodge |
| **Nom Sodomight** | Esquive Instinctive |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 6 |
| **Prérequis** | Oeil de Solmer (Inner Sight) |
| **Coût mana** | Passif |
| **Type** | Passif (chance d'esquiver les attaques de mêlée) |

**Description mécanique :** Chance passive d'esquiver entièrement une attaque de mêlée. Lorsque l'esquive se déclenche, une animation d'esquive est jouée pendant laquelle la Sarith ne peut pas attaquer (interaction problématique avec Strafe/Valkyrie).

**Formule — Probabilité d'esquive mêlée :**
| Niveau | Chance |
|--------|--------|
| 1 | 18% |
| 3 | 26% |
| 5 | 33% |
| 10 | 45% |
| 15 | 54% |
| 20 | 60% |

**Note critique :** Dodge, Avoid et Evade partagent une mécanique interdépendante. Si l'une se déclenche pendant Strafe, l'attaque est interrompue. Cela crée le fameux "Dodge Bug" — mettre trop de points dans ces skills peut interrompre Strafe constamment. La plupart des builds recommandent 1 seul point dans chacun.

**Notes gameplay :** 1 point pour le bénéfice de l'efficacité marginale ; ne jamais maxer si on joue Strafe.

---

#### 2.2.4 Missiles Lents (Slow Missiles)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Slow Missiles |
| **Nom Sodomight** | Ralentissement de Projectiles |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 12 |
| **Prérequis** | Oeil de Solmer (Inner Sight) |
| **Coût mana** | 5 |
| **Type** | Debuff actif (aura de zone) |
| **Range** | ~9.3m de rayon |
| **Durée** | 12s (base) |
| **Effet spécial** | Réduit la vitesse des projectiles ennemis de 33% |

**Description mécanique :** Ralentit les projectiles ennemis dans un rayon autour du curseur. Très utile contre les archers ennemis, les serpents crachers, etc.

**Synergies reçues :** Aucune.
**Notes gameplay :** 1 point suffit. Utile dans des zones d'archers intenses (Andariel, Arcane Sanctuary, etc.). Rarement maxé.

---

#### 2.2.5 Esquive de Missiles (Avoid)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Avoid |
| **Nom Sodomight** | Dérobade |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 12 |
| **Prérequis** | Esquive Instinctive (Dodge) |
| **Coût mana** | Passif |
| **Type** | Passif (chance d'esquiver les projectiles) |

**Description mécanique :** Chance passive d'esquiver les projectiles (missiles, flèches, sorts à projectile). Même mécanique d'interruption que Dodge pour Strafe.

**Formule — Probabilité :**
| Niveau | Chance |
|--------|--------|
| 1 | 24% |
| 5 | 39% |
| 10 | 52% |
| 20 | 65% |

**Notes gameplay :** Même règle : 1 point maximum pour Strafe builds. Utile pour les builds passifs/défensifs.

---

#### 2.2.6 Pénétration (Penetrate)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Penetrate |
| **Nom Sodomight** | Percement |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 18 |
| **Prérequis** | Dérobade (Avoid) |
| **Coût mana** | Passif |
| **Type** | Passif (bonus d'Attack Rating) |

**Description mécanique :** Augmente passivement l'Attack Rating (précision) de toutes les attaques. L'AR est crucial dans D2 car la formule de toucher requiert souvent des valeurs élevées en Hell.

**Formule — Bonus AR :**
| Niveau | Bonus AR |
|--------|----------|
| 1 | +35% |
| 5 | +75% |
| 10 | +130% |
| 15 | +185% |
| 20 | +240% |

**Notes gameplay :** Skill essentiel pour builds physiques (MS/Strafe). L'AR est problématique en Hell et quelques points dans Penetrate compensent sans investissement lourd.

---

#### 2.2.7 Leurre (Decoy)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Decoy |
| **Nom Sodomight** | Effigie de Combat |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 24 |
| **Prérequis** | Missiles Lents (Slow Missiles) |
| **Coût mana** | 19 (base) |
| **Type** | Invocation (clone défensif statique) |
| **Durée** | 10s (base), augmente avec niveau |
| **Effet spécial** | Crée un clone qui attire les ennemis et peut attaquer |

**Description mécanique :** Invoque un clone stationnaire de la Sarith qui attire l'attention des ennemis et peut attaquer. Le Leurre a les mêmes stats que la Sarith (vie, équipement) mais ne se déplace pas.

**Formule — Points de vie du Leurre :**
| Niveau | Vie (% de la Sarith) |
|--------|----------------------|
| 1 | 100% |
| 10 | 145% |
| 20 | 195% |

**Notes gameplay :** Excellent pour distraire les boss et les packs d'élites. Utile en groupe pour attirer les monstres loin du groupe. Le Leurre absorbe énormément de dégâts. 1 point suffit pour l'utilité de distraction.

---

#### 2.2.8 Evitement Combiné (Evade)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Evade |
| **Nom Sodomight** | Pas de l'Ombre |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 24 |
| **Prérequis** | Dérobade + Esquive Instinctive (Avoid + Dodge) |
| **Coût mana** | Passif |
| **Type** | Passif (chance d'esquiver mêlée ET missiles en mouvement) |

**Description mécanique :** Chance d'esquiver toutes les attaques (mêlée et projectiles) **lorsque la Sarith est en mouvement**. Complément de Dodge (stationnaire mêlée) et Avoid (stationnaire projectile).

**Formule — Probabilité :**
| Niveau | Chance |
|--------|--------|
| 1 | 18% |
| 5 | 33% |
| 10 | 45% |
| 20 | 60% |

**Notes gameplay :** Même avertissement que Dodge : 1 point recommandé pour Strafe builds. Builds défensifs peuvent investir davantage.

---

#### 2.2.9 Valkyrie
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Valkyrie |
| **Nom Sodomight** | Gardienne de Solmer |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 30 |
| **Prérequis** | Effigie de Combat + Pas de l'Ombre (Decoy + Evade) |
| **Coût mana** | 25 (base) |
| **Type** | Invocation puissante (alliée persistante) |
| **Effet spécial** | Invoque une alliée guerrière très puissante |

**Description mécanique :** Invoque une guerrière spectrale (la Valkyrie) qui combat aux côtés de la Sarith. La Valkyrie a d'excellentes stats de base, porte un équipement aléatoire et améliore sa résistance et ses dégâts avec le niveau du skill. Elle persiste jusqu'à la mort.

**Formule — Stats de la Valkyrie :**
| Niveau skill | Vie | Dégâts (min-max) | Résistances |
|-------------|-----|-----------------|-------------|
| 1 | 255 | 12-20 | 50% all |
| 5 | 610 | 35-55 | 60% all |
| 10 | 1155 | 74-112 | 65% all |
| 20 | 2245 | 152-228 | 75% all |

La Valkyrie gagne aussi de l'équipement selon le niveau du skill (lance, bouclier, armure progressivement meilleures).

**Synergies reçues :**
- Critical Strike : +5% Life par niveau de base de Critical Strike
- Dodge/Avoid/Evade : Améliore les résistances et défense de la Valkyrie

**Notes gameplay :** L'une des meilleures invocations du jeu. Servir de tank, absorber les coups, et libérer la Sarith pour faire des dégâts. Même 1 point donne une alliée utile, mais maxer donne une Valkyrie quasi-indestructible.

---

#### 2.2.10 Transpercement (Pierce)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Pierce |
| **Nom Sodomight** | Perforation |
| **Arbre** | Passives & Magie |
| **Niveau requis** | 30 |
| **Prérequis** | Percement (Penetrate) |
| **Coût mana** | Passif |
| **Type** | Passif (chance de traversée de cibles) |

**Description mécanique :** Chance passive que chaque flèche/carreau traverse la première cible et continue pour toucher d'autres ennemis derrière. La flèche peut toucher chaque ennemi une seule fois par tir.

**Formule — Probabilité de traversée :**
| Niveau | Chance |
|--------|--------|
| 1 | 23% |
| 3 | 33% |
| 5 | 40% |
| 7 | 47% |
| 10 | 55% |
| 15 | 65% |
| 20 | 73% |

Formule : Chance = 100 × slvl / (slvl + 6) — identique à Critical Strike.

**Notes gameplay :** Synergique avec TOUS les skills de projectile. Notamment avec Lightning Fury (où chaque "split" peut aussi percer) et Multiple Shot. Essentiel pour tout build utilisant des projectiles multiples.

---

### Arbre 3 : Javelines & Lances (Javelin and Spear Skills)

#### 2.3.1 Coup de Javelot (Jab)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Jab |
| **Nom Sodomight** | Estoc Rapide |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2 |
| **Type de dégâts** | Physique (3 frappes rapides) |
| **Range** | Mêlée (lance/javeline) |

**Description mécanique :** Effectue 3 frappes rapides en succession. Chaque frappe a son propre calcul d'AR (Attack Rating). Les 3 touches sont indépendantes — si la première manque, les autres peuvent quand même toucher.

**Formule — Bonus de dégâts par frappe :**
| Niveau | Damage bonus | AR Bonus |
|--------|-------------|----------|
| 1 | +35% | +15% |
| 5 | +65% | +35% |
| 10 | +107% | +60% |
| 20 | +192% | +110% |

**Notes gameplay :** Skill de mêlée rapide pour builds lancier. Rarement utilisé en endgame (remplacé par Fend ou Lightning Strike) mais utile pour les premiers actes.

---

#### 2.3.2 Frappe de Puissance (Power Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Power Strike |
| **Nom Sodomight** | Frappe Tonnante |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 6 |
| **Prérequis** | Estoc Rapide (Jab) |
| **Coût mana** | 2 |
| **Type de dégâts** | Physique + Foudre |
| **Range** | Mêlée (javeline/lance) |

**Description mécanique :** Attaque de mêlée puissante ajoutant des dégâts de foudre à la frappe physique. Les dégâts de foudre scalent bien avec le niveau.

**Formule de dégâts :**
| Niveau | Foudre min | Foudre max | AR Bonus |
|--------|-----------|-----------|----------|
| 1 | 1 | 12 | +20% |
| 5 | 5 | 36 | +40% |
| 10 | 11 | 72 | +70% |
| 20 | 23 | 144 | +130% |

**Synergies reçues :**
- Lightning Bolt : +20% Lightning Damage par niveau
- Charged Strike : +20% Lightning Damage par niveau
- Lightning Strike : +20% Lightning Damage par niveau
- Lightning Fury : +20% Lightning Damage par niveau

**Notes gameplay :** Skill important comme synérgie pour les builds foudre. Quelques points pour les synergies maximisent les dégâts de foudre des skills de haut niveau.

---

#### 2.3.3 Javeline Empoisonnée (Poison Javelin)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Poison Javelin |
| **Nom Sodomight** | Javeline Vipérine |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 6 |
| **Prérequis** | Estoc Rapide (Jab) |
| **Coût mana** | 2 |
| **Type de dégâts** | Poison (nuage persistant) |
| **Range** | Lancé — crée un nuage de poison en ligne |
| **Durée** | 2s (base) |

**Description mécanique :** Lance une javeline qui laisse un nuage de poison derrière elle au fil de sa trajectoire. Le nuage persiste et empoisonne les ennemis qui le traversent.

**Formule de dégâts :**
| Niveau | Poison/sec | Durée |
|--------|-----------|-------|
| 1 | 20 | 2s |
| 5 | 74 | 2.8s |
| 10 | 162 | 3.8s |
| 20 | 338 | 5.8s |

**Synergies reçues :**
- Plague Javelin : +20% Poison Damage par niveau

**Synergies données :**
- Plague Javelin : +20% Poison Damage par niveau

**Notes gameplay :** Peu utilisé seul. Synérgie pour Plague Javelin builds mais les dégâts poison de base sont trop faibles pour un build dédié.

---

#### 2.3.4 Empalage (Impale)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Impale |
| **Nom Sodomight** | Embrochement |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 12 |
| **Prérequis** | Frappe Tonnante (Power Strike) |
| **Coût mana** | 3 |
| **Type de dégâts** | Physique (très élevé, arme-based) |
| **Range** | Mêlée |
| **Effet spécial** | Ignore la défense, chance de Stun, détruit l'arme |

**Description mécanique :** Attaque unique mais extrêmement puissante. Dégâts massifs, ignore la défense de l'ennemi (équivalent à toujours toucher). Cependant chaque usage coûte 46% de durabilité de l'arme, la rendant peu pratique.

**Formule de dégâts :**
| Niveau | Damage bonus | Durabilité arme consommée |
|--------|-------------|--------------------------|
| 1 | +300% | 46% |
| 5 | +440% | 38% |
| 10 | +620% | 28% |
| 20 | +980% | 8% |

**Notes gameplay :** Skill situationnel — la consommation de durabilité le rend impraticable sauf avec des armes indestructibles. 1 point pour les situations exceptionnelles.

---

#### 2.3.5 Eclair de Javeline (Lightning Bolt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning Bolt |
| **Nom Sodomight** | Eclair de Javeline |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 12 |
| **Prérequis** | Javeline Vipérine (Poison Javelin) |
| **Coût mana** | 6 |
| **Type de dégâts** | Foudre (100% conversion physique) |
| **Range** | Lancé (projection) |

**Description mécanique :** Lance une javeline convertie en éclair pur — 100% des dégâts sont de la foudre, aucune composante physique. La javeline voyage en ligne droite et touche la première cible.

**Formule de dégâts :**
| Niveau | Foudre min | Foudre max |
|--------|-----------|-----------|
| 1 | 1 | 40 |
| 5 | 5 | 80 |
| 10 | 11 | 134 |
| 20 | 23 | 242 |

**Synergies reçues :**
- Power Strike : +20% Lightning Damage par niveau
- Charged Strike : +20% Lightning Damage par niveau
- Lightning Strike : +20% Lightning Damage par niveau
- Lightning Fury : +20% Lightning Damage par niveau

**Notes gameplay :** Maxé principalement pour ses synergies qui boostent toute la chaîne foudre. Les dégâts bruts sont corrects mais ce n'est pas le skill principal d'un build foudre.

---

#### 2.3.6 Frappe Chargée (Charged Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Charged Strike |
| **Nom Sodomight** | Frappe Electrisée |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 18 |
| **Prérequis** | Frappe Tonnante (Power Strike) |
| **Coût mana** | 4 (base) |
| **Type de dégâts** | Physique + Foudre (+ Charged Bolts émis) |
| **Range** | Mêlée + émission de Charged Bolts en zone |

**Description mécanique :** Frappe de mêlée qui inflige des dégâts de foudre ET libère plusieurs Charged Bolts autour de la cible. Les Charged Bolts rebondissent et peuvent toucher d'autres ennemis. Très puissant contre les packs denses.

**Formule de dégâts :**
| Niveau | Foudre (frappe) min-max | Bolts émis |
|--------|------------------------|-----------|
| 1 | 1-50 | 3 |
| 5 | 10-100 | 5 |
| 10 | 23-170 | 8 |
| 20 | 49-310 | 14 |

**Synergies reçues :**
- Power Strike : +20% Lightning Damage par niveau
- Lightning Bolt : +20% Lightning Damage par niveau
- Lightning Strike : +20% Lightning Damage par niveau
- Lightning Fury : +20% Lightning Damage par niveau

**Notes gameplay :** L'un des meilleurs skills single-target pour la Sarith foudre. Les Charged Bolts créent un AoE de fait. Souvent maxé dans les builds "Javazon" avec Lightning Fury.

---

#### 2.3.7 Javeline de Peste (Plague Javelin)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Plague Javelin |
| **Nom Sodomight** | Javeline Pestilentielle |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 18 |
| **Prérequis** | Eclair de Javeline (Lightning Bolt) |
| **Coût mana** | 7 |
| **Type de dégâts** | Poison (nuage AoE expansif) |
| **Durée** | 3s (base), s'étend progressivement |

**Description mécanique :** Lance une javeline qui crée un nuage de poison expansif qui s'élargit progressivement sur sa durée. Le nuage peut toucher plusieurs ennemis simultanément.

**Formule de dégâts :**
| Niveau | Poison/sec | Durée |
|--------|-----------|-------|
| 1 | 62 | 3s |
| 5 | 188 | 3.6s |
| 10 | 394 | 4.4s |
| 20 | 806 | 6s |

**Synergies reçues :**
- Poison Javelin : +20% Poison Damage par niveau

**Synergies données :**
- Poison Javelin : +20% Poison Damage par niveau

**Notes gameplay :** Skill de niche pour builds poison-Sarith. Rarement utilisé en endgame compétitif mais amusant en solo.

---

#### 2.3.8 Défense Tourbillonnante (Fend)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fend |
| **Nom Sodomight** | Danse de Lance |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 24 |
| **Prérequis** | Embrochement (Impale) |
| **Coût mana** | 5 (base) |
| **Type de dégâts** | Physique (attaque tous les ennemis adjacents) |
| **Range** | Mêlée (AoE adjacente) |

**Description mécanique :** Attaque rapidement toutes les cibles adjacentes. Similaire à Zeal du Paladin. Excellent pour la mêlée AoE. La Sarith ne peut pas se déplacer pendant Fend.

**Formule de dégâts et cibles :**
| Niveau | Damage bonus | AR Bonus | Cibles max |
|--------|-------------|----------|-----------|
| 1 | +70% | +40% | 3 |
| 5 | +110% | +70% | 4 |
| 10 | +163% | +108% | 5 |
| 20 | +270% | +185% | 7 |

**Notes gameplay :** Skill de mêlée AoE pour "Spearazon". Moins populaire que les builds foudre mais viables.

---

#### 2.3.9 Frappe d'Eclair (Lightning Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning Strike |
| **Nom Sodomight** | Coup de Tonnerre |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 30 |
| **Prérequis** | Frappe Electrisée (Charged Strike) |
| **Coût mana** | 9 |
| **Type de dégâts** | Physique + Foudre (Chain Lightning) |
| **Range** | Mêlée + Chain Lightning rebondissant |
| **Effet spécial** | L'éclair rebondit sur plusieurs ennemis |

**Description mécanique :** Frappe de mêlée qui libère un Chain Lightning rebondissant. L'éclair rebondit entre les ennemis proches, touchant jusqu'à plusieurs cibles.

**Formule :**
| Niveau | Foudre min-max | Rebonds |
|--------|---------------|---------|
| 1 | 1-80 | 2 |
| 5 | 12-152 | 4 |
| 10 | 28-248 | 7 |
| 20 | 60-440 | 13 |

**Synergies reçues :**
- Power Strike : +20% Lightning Damage par niveau
- Lightning Bolt : +20% Lightning Damage par niveau
- Charged Strike : +20% Lightning Damage par niveau
- Lightning Fury : +20% Lightning Damage par niveau

**Notes gameplay :** Alternative à Charged Strike pour les packs. Certains builds mixent les deux.

---

#### 2.3.10 Fureur de Foudre (Lightning Fury)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning Fury |
| **Nom Sodomight** | Fureur Foudroyante |
| **Arbre** | Javelines & Lances |
| **Niveau requis** | 30 |
| **Prérequis** | Javeline Pestilentielle (Plague Javelin) |
| **Coût mana** | 10 (base), augmente avec niveau |
| **Type de dégâts** | Foudre (split en plusieurs éclairs à l'impact) |
| **Range** | Lancé |
| **Effet spécial** | **Split** — se divise en plusieurs éclairs à l'impact ; les éclairs peuvent piercer avec Pierce |

**Description mécanique :** Lance une javeline qui, à l'impact, se divise en un grand nombre de Charged Bolts. Chaque bolt peut percer avec le passif Pierce, créant une dévastation de zone monstrueuse. Skill signature de la "Javazon".

**Formule — Nombre de Charged Bolts :**
| Niveau | Bolts à l'impact | Foudre par bolt (min-max) |
|--------|-----------------|--------------------------|
| 1 | 2 | 1-30 |
| 5 | 5 | 1-65 |
| 10 | 9 | 1-110 |
| 15 | 13 | 1-155 |
| 20 | 17 | 1-200 |

**Synergies reçues :**
- Power Strike : +20% Lightning Damage par niveau
- Lightning Bolt : +20% Lightning Damage par niveau
- Charged Strike : +20% Lightning Damage par niveau
- Lightning Strike : +20% Lightning Damage par niveau

**Avec Pierce maxé (73%) :** chaque bolt peut traverser plusieurs ennemis, multipliant drastiquement les dégâts effectifs sur les packs.

**Notes gameplay :** **Skill le plus puissant de la Sarith pour le farming.** La combinaison Lightning Fury + Pierce + synergies maxées = dégâts AoE phénoménaux. Build standard : 20 LF, 20 CS, 20 PS, reste en synergies. Faiblesse : Lightning Immune en Hell.

---

## 3. Necromancer → Mortecian

### Description de classe
Le Mortecian est un invocateur et manipulateur de forces obscures. Il commande des légions de revenants et de golems, lance des malédictions affaiblissant les ennemis, et maîtrise les sorts d'os et de poison.

### Nommage Sodomight
- **Necromancer** → **Mortecian** (maître des morts, terme générique)
- Skeletons → **Revenants** ; Golems restent **Golems**

---

### Arbre 1 : Invocations (Summoning Spells)

#### 3.1.1 Invoquer Revenant (Raise Skeleton)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Raise Skeleton |
| **Nom Sodomight** | Invoquer Revenant |
| **Arbre** | Invocations |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 6 (base) |
| **Type** | Invocation (réanimation de cadavre) |

**Description mécanique :** Réanime le cadavre d'un ennemi tué comme revenant guerrier combattant. Le nombre maximum de revenants augmente avec le niveau.

**Formule — Stats des revenants :**
| Niveau | Vie | Dégâts min-max | Max revenants |
|--------|-----|----------------|---------------|
| 1 | 120 | 3-8 | 1 |
| 5 | 300 | 14-25 | 4 |
| 10 | 600 | 38-60 | 7 |
| 20 | 1200 | 86-132 | 13 |

**Synergies reçues :** Skeleton Mastery : +20% Life et +20% Damage par niveau de base.

**Notes gameplay :** Fondement du "Summonmancer". Maxer conjointement avec Skeleton Mastery.

---

#### 3.1.2 Maîtrise des Revenants (Skeleton Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Skeleton Mastery |
| **Nom Sodomight** | Maîtrise des Revenants |
| **Arbre** | Invocations |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Life bonus | Damage bonus |
|--------|-----------|-------------|
| 1 | +20% | +20% |
| 10 | +200% | +200% |
| 20 | +400% | +400% |

**Notes gameplay :** À maxer en priorité absolue pour tout build invocateur.

---

#### 3.1.3 Golem de Terre (Clay Golem)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Clay Golem |
| **Nom Sodomight** | Golem de Terre |
| **Arbre** | Invocations |
| **Niveau requis** | 6 |
| **Prérequis** | Invoquer Revenant |
| **Coût mana** | 15 |
| **Effet spécial** | Ralentit les ennemis à l'impact (50%) |

**Formule :**
| Niveau | Vie | Dégâts |
|--------|-----|--------|
| 1 | 100 | 5-9 |
| 10 | 730 | 39-63 |
| 20 | 1510 | 83-131 |

**Synergies reçues :** Golem Mastery +20% Life/niveau. Tous les autres golem skills +4% Life/niveau.

**Notes gameplay :** Le Slow 50% permanent est très utile. Facilement re-invocable (pas de composant requis).

---

#### 3.1.4 Maîtrise des Golems (Golem Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Golem Mastery |
| **Nom Sodomight** | Maîtrise des Golems |
| **Arbre** | Invocations |
| **Niveau requis** | 12 |
| **Prérequis** | Golem de Terre |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Life bonus | Speed bonus |
|--------|-----------|------------|
| 1 | +20% | +8% |
| 10 | +200% | +80% |
| 20 | +400% | +160% |

---

#### 3.1.5 Invoquer Mage Revenant (Raise Skeletal Mage)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Raise Skeletal Mage |
| **Nom Sodomight** | Invoquer Mage Revenant |
| **Arbre** | Invocations |
| **Niveau requis** | 12 |
| **Prérequis** | Invoquer Revenant |
| **Coût mana** | 8 |
| **Effet spécial** | Élément aléatoire (feu, froid, foudre, poison) |

**Formule :**
| Niveau | Vie | Dégâts élémentaux |
|--------|-----|-------------------|
| 1 | 60 | 2-8 |
| 10 | 420 | 34-70 |
| 20 | 900 | 78-158 |

**Synergies reçues :** Skeleton Mastery +20% Life et Damage/niveau.

**Notes gameplay :** Utiles pour contourner les immunités élémentales grâce à la diversité des types d'attaque.

---

#### 3.1.6 Golem de Sang (Blood Golem)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blood Golem |
| **Nom Sodomight** | Golem de Sang |
| **Arbre** | Invocations |
| **Niveau requis** | 18 |
| **Prérequis** | Golem de Terre |
| **Coût mana** | 25 |
| **Effet spécial** | Vol de vie — partage avec le Mortecian |

**Formule :**
| Niveau | Vie | Dégâts | Life Steal |
|--------|-----|--------|-----------|
| 1 | 150 | 9-15 | 35% |
| 10 | 890 | 55-85 | 45% |
| 20 | 1830 | 115-175 | 55% |

---

#### 3.1.7 Résistance des Invocations (Summon Resist)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Summon Resist |
| **Nom Sodomight** | Résistance des Invocations |
| **Arbre** | Invocations |
| **Niveau requis** | 24 |
| **Prérequis** | Maîtrise des Golems |
| **Coût mana** | Passif |

**Formule :**
| Niveau | All Resist bonus (invocations) |
|--------|-------------------------------|
| 1 | +28% |
| 10 | +73% |
| 20 | +123% |

**Notes gameplay :** Essentiel pour maintenir l'armée en vie dans Nightmare/Hell.

---

#### 3.1.8 Golem de Métal (Iron Golem)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Iron Golem |
| **Nom Sodomight** | Golem de Métal |
| **Arbre** | Invocations |
| **Niveau requis** | 24 |
| **Prérequis** | Golem de Sang |
| **Coût mana** | 35 |
| **Effet spécial** | Forgé depuis un item — hérite de TOUTES ses propriétés magiques |

**Formule :**
| Niveau | Vie | Aura Épines |
|--------|-----|------------|
| 1 | 200 | 200% retour |
| 10 | 1200 | 350% retour |
| 20 | 2400 | 500% retour |

**Notes gameplay :** Le golem le plus puissant. Forger avec un runeword "Insight" donne l'Aura de Méditation (regen mana pour tout le groupe).

---

#### 3.1.9 Golem de Flammes (Fire Golem)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Golem |
| **Nom Sodomight** | Golem de Flammes |
| **Arbre** | Invocations |
| **Niveau requis** | 30 |
| **Prérequis** | Golem de Métal |
| **Coût mana** | 50 |
| **Effet spécial** | Absorbe 50%+ des dégâts de feu reçus ; aura de feu |

**Formule :**
| Niveau | Vie | Feu AoE dégâts |
|--------|-----|---------------|
| 1 | 250 | 15-25 |
| 20 | 2850 | 175-275 |

---

#### 3.1.10 Résurrection Servile (Revive)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Revive |
| **Nom Sodomight** | Résurrection Servile |
| **Arbre** | Invocations |
| **Niveau requis** | 30 |
| **Prérequis** | Résistance des Invocations |
| **Coût mana** | 45 |
| **Durée** | 180 secondes |
| **Effet spécial** | Le monstre revivifié conserve TOUTES ses capacités originales |

**Formule :**
| Niveau | Max revivifiés |
|--------|---------------|
| 1 | 1 |
| 10 | 8 |
| 20 | 16 |

**Notes gameplay :** Complète l'armée. Revivifier des monstres puissants (avec immunités, capacités spéciales) est extrêmement puissant.

---

### Arbre 2 : Os & Poison (Poison and Bone Spells)

#### 3.2.1 Dents d'Os (Teeth)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Teeth |
| **Nom Sodomight** | Dents d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 3 |
| **Type de dégâts** | Magique (éventail de projectiles) |

**Formule :**
| Niveau | Dégâts min-max | Projectiles |
|--------|---------------|------------|
| 1 | 2-4 | 3 |
| 10 | 17-27 | 11 |
| 20 | 35-53 | 19 |

**Synergies données :** Lance d'Os +15% Magic Damage/niveau ; Âme d'Os +15% Magic Damage/niveau.

---

#### 3.2.2 Bouclier d'Os (Bone Armor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bone Armor |
| **Nom Sodomight** | Bouclier d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 11 |
| **Type** | Défensif (absorbe les dégâts physiques) |
| **Durée** | 144s |

**Formule :**
| Niveau | Dégâts absorbés |
|--------|----------------|
| 1 | 20 |
| 10 | 110 |
| 20 | 215 |

**Synergies reçues :** Muraille d'Os +4%/niveau ; Cage d'Os +4%/niveau.

---

#### 3.2.3 Dague Virulente (Poison Dagger)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Poison Dagger |
| **Nom Sodomight** | Dague Virulente |
| **Arbre** | Os & Poison |
| **Niveau requis** | 6 |
| **Prérequis** | Dents d'Os |
| **Coût mana** | 3 |
| **Type** | Physique + Poison (mêlée) |

**Formule :**
| Niveau | Poison/sec | Durée |
|--------|-----------|-------|
| 1 | 15 | 2s |
| 10 | 115 | 4.2s |
| 20 | 235 | 6.5s |

**Synergies données :** Nova Vénéneuse +10%/niveau ; Explosion Vénéneuse +8%/niveau.

---

#### 3.2.4 Explosion Cadavérique (Corpse Explosion)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Corpse Explosion |
| **Nom Sodomight** | Explosion Cadavérique |
| **Arbre** | Os & Poison |
| **Niveau requis** | 6 |
| **Prérequis** | Bouclier d'Os |
| **Coût mana** | 15 |
| **Type de dégâts** | 50% Physique + 50% Feu (basé sur la vie max du cadavre) |
| **Range** | Rayon d'explosion croissant |

**Formule :**
| Niveau | % vie max (dégâts totaux) | Rayon |
|--------|--------------------------|-------|
| 1 | 60-100% | 2.6m |
| 10 | 78-118% | 4m |
| 20 | 98-138% | 5.3m |

**Notes gameplay :** **Skill le plus important du Mortecian.** Utilisé par TOUS les builds. La chaîne de réaction (chaque mort = nouveau cadavre) est dévastatrice en Hell.

---

#### 3.2.5 Muraille d'Os (Bone Wall)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bone Wall |
| **Nom Sodomight** | Muraille d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 12 |
| **Prérequis** | Explosion Cadavérique |
| **Coût mana** | 17 |
| **Type** | Contrôle (barrière physique) |

**Formule :**
| Niveau | Vie du mur | Durée |
|--------|-----------|-------|
| 1 | 30 | 12s |
| 10 | 270 | 30s |
| 20 | 570 | 50s |

**Synergies données :** Bouclier d'Os +4% absorption/niveau ; Lance d'Os +15% Magic Damage/niveau.

---

#### 3.2.6 Explosion Vénéneuse (Poison Explosion)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Poison Explosion |
| **Nom Sodomight** | Explosion Vénéneuse |
| **Arbre** | Os & Poison |
| **Niveau requis** | 18 |
| **Prérequis** | Muraille d'Os |
| **Coût mana** | 8 |
| **Type** | Poison AoE (explosion de cadavre) |

**Formule :**
| Niveau | Poison/sec | Durée | Rayon |
|--------|-----------|-------|-------|
| 1 | 28 | 2s | 2.6m |
| 10 | 212 | 4.2s | 4m |
| 20 | 436 | 6.5s | 5.3m |

**Synergies données :** Nova Vénéneuse +15% Poison Damage/niveau.

---

#### 3.2.7 Lance d'Os (Bone Spear)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bone Spear |
| **Nom Sodomight** | Lance d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 18 |
| **Prérequis** | Dents d'Os |
| **Coût mana** | 7 (base), +0.125/niveau |
| **Type de dégâts** | Magique (traverse tous les ennemis) |

**Formule :**
| Niveau | Magique min | Magique max |
|--------|------------|------------|
| 1 | 17 | 25 |
| 10 | 117 | 143 |
| 20 | 237 | 279 |

**Synergies reçues :** Dents d'Os +15%/niveau ; Muraille d'Os +15%/niveau ; Cage d'Os +15%/niveau ; Âme d'Os +15%/niveau.

**Notes gameplay :** **Skill offensif principal du Bonemancer.** Dégâts magiques non-résistibles. Avec 4 synergies à 20 = +120% dégâts multiplicateurs.

---

#### 3.2.8 Cage d'Os (Bone Prison)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bone Prison |
| **Nom Sodomight** | Cage d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 24 |
| **Prérequis** | Muraille d'Os |
| **Coût mana** | 27 |
| **Effet spécial** | Emprisonne une cible — immobilisation complète |

**Formule :**
| Niveau | Vie de la cage | Durée |
|--------|---------------|-------|
| 1 | 40 | 8s |
| 10 | 290 | 18s |
| 20 | 590 | 30s |

**Synergies données :** Bouclier d'Os +4%/niveau ; Lance d'Os +15% Magic Damage/niveau.

---

#### 3.2.9 Nova Vénéneuse (Poison Nova)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Poison Nova |
| **Nom Sodomight** | Nova Vénéneuse |
| **Arbre** | Os & Poison |
| **Niveau requis** | 30 |
| **Prérequis** | Explosion Vénéneuse |
| **Coût mana** | 20 |
| **Type** | Poison nova 360° |
| **Range** | ~8m de rayon |
| **Durée poison** | 2s base |

**Formule :**
| Niveau | Poison total min (sur 2s) | Poison total max |
|--------|--------------------------|-----------------|
| 1 | 104 | 187 |
| 10 | 868 | 1318 |
| 20 | 1816 | 2666 |

**Synergies reçues :** Dague Virulente +10%/niveau ; Explosion Vénéneuse +10%/niveau.

---

#### 3.2.10 Âme d'Os (Bone Spirit)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bone Spirit |
| **Nom Sodomight** | Âme d'Os |
| **Arbre** | Os & Poison |
| **Niveau requis** | 30 |
| **Prérequis** | Lance d'Os |
| **Coût mana** | 12 (base), +0.25/niveau |
| **Type** | Magique (homing — suit la cible) |

**Formule :**
| Niveau | Magique min | Magique max |
|--------|------------|------------|
| 1 | 22 | 33 |
| 10 | 122 | 153 |
| 20 | 242 | 293 |

**Synergies reçues :** Dents d'Os/Muraille/Cage +15%/niveau chacun.
**Synergies données :** Lance d'Os +15%/niveau.

**Notes gameplay :** Single-target homing. Idéal contre les boss — ne rate jamais.

---

### Arbre 3 : Malédictions (Curses)

#### 3.3.1 Amplification des Dégâts (Amplify Damage)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Amplify Damage |
| **Nom Sodomight** | Amplification des Dégâts |
| **Arbre** | Malédictions |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 4 |
| **Durée** | 8s (base) |
| **Effet** | Réduit les résistances physiques ennemies de 100% (double les dégâts physiques reçus) |

**Notes gameplay :** **Malédiction la plus importante pour groupes physiques.** 1 point suffit — augmenter le niveau prolonge seulement la durée.

---

#### 3.3.2 Vision Eclipsée (Dim Vision)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dim Vision |
| **Nom Sodomight** | Vision Eclipsée |
| **Arbre** | Malédictions |
| **Niveau requis** | 6 |
| **Prérequis** | Amplification des Dégâts |
| **Coût mana** | 9 |
| **Durée** | 12s |
| **Effet** | Aveugle les ennemis — ils errent sans attaquer |

---

#### 3.3.3 Epuisement (Weaken)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Weaken |
| **Nom Sodomight** | Epuisement |
| **Arbre** | Malédictions |
| **Niveau requis** | 6 |
| **Prérequis** | Amplification des Dégâts |
| **Coût mana** | 4 |
| **Durée** | 16s |
| **Effet** | Réduit les dégâts ennemis de 33% (base) à 53% (niveau 20) |

---

#### 3.3.4 Rétribution de Fer (Iron Maiden)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Iron Maiden |
| **Nom Sodomight** | Rétribution de Fer |
| **Arbre** | Malédictions |
| **Niveau requis** | 12 |
| **Prérequis** | Epuisement |
| **Coût mana** | 5 |
| **Effet** | Renvoi de 200% des dégâts physiques reçus à l'attaquant |

**Formule :**
| Niveau | % dégâts renvoyés |
|--------|-------------------|
| 1 | 200% |
| 10 | 350% |
| 20 | 530% |

---

#### 3.3.5 Effroi (Terror)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Terror |
| **Nom Sodomight** | Effroi |
| **Arbre** | Malédictions |
| **Niveau requis** | 12 |
| **Prérequis** | Vision Eclipsée |
| **Coût mana** | 7 |
| **Durée** | 8s |
| **Effet** | Force les ennemis à fuir en panique |

---

#### 3.3.6 Confusion Mentale (Confuse)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Confuse |
| **Nom Sodomight** | Confusion Mentale |
| **Arbre** | Malédictions |
| **Niveau requis** | 18 |
| **Prérequis** | Effroi |
| **Coût mana** | 13 |
| **Durée** | 12s |
| **Effet** | Ennemis attaquent des cibles aléatoires (s'entre-tuent) |

---

#### 3.3.7 Drain Vital (Life Tap)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Life Tap |
| **Nom Sodomight** | Drain Vital |
| **Arbre** | Malédictions |
| **Niveau requis** | 18 |
| **Prérequis** | Rétribution de Fer |
| **Coût mana** | 9 |
| **Durée** | 16s |
| **Effet** | 50-88% des dégâts physiques infligés à la cible récupérés comme vie |

**Formule :**
| Niveau | Vol de vie |
|--------|-----------|
| 1 | 50% |
| 10 | 68% |
| 20 | 88% |

**Notes gameplay :** **Malédiction essentielle contre les boss.** Rend l'armée autosuffisante en vie.

---

#### 3.3.8 Aimantation (Attract)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Attract |
| **Nom Sodomight** | Aimantation |
| **Arbre** | Malédictions |
| **Niveau requis** | 24 |
| **Prérequis** | Confusion Mentale |
| **Coût mana** | 17 |
| **Durée** | 16s |
| **Effet** | Tous les ennemis proches attaquent la cible maudite |

---

#### 3.3.9 Décrépitude (Decrepify)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Decrepify |
| **Nom Sodomight** | Décrépitude |
| **Arbre** | Malédictions |
| **Niveau requis** | 24 |
| **Prérequis** | Drain Vital |
| **Coût mana** | 11 |
| **Durée** | 12s |
| **Effets combinés** | -75% vitesse, -50% dégâts, -50% résistances physiques, -50% vitesse attaque |

**Notes gameplay :** **Malédiction la plus importante pour le boss-killing.** 1 point suffit pour la mécanique complète.

---

#### 3.3.10 Dissolution des Résistances (Lower Resist)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lower Resist |
| **Nom Sodomight** | Dissolution des Résistances |
| **Arbre** | Malédictions |
| **Niveau requis** | 30 |
| **Prérequis** | Aimantation |
| **Coût mana** | 22 |
| **Durée** | 16s |
| **Effet** | Réduit TOUTES les résistances élémentales |

**Formule :**
| Niveau | Réduction all resist |
|--------|---------------------|
| 1 | -31% |
| 10 | -58% |
| 20 | -88% |

**Notes gameplay :** **Inestimable en groupe.** Peut briser les immunités élémentales en Hell si la réduction dépasse la valeur de résistance (ex. : Immune to Fire = 100% → Lower Resist -88% → 12% resist = plus immunisé).

---

## 4. Barbarian → Ravageur

### Description de classe
Le Ravageur est un guerrier de mêlée pur, spécialisé dans l'utilisation simultanée de deux armes, les cris de guerre boostant le groupe, et les maîtrises passives d'armes. Il possède également la capacité unique **Dual Wield** (deux armes différentes en même temps) et **Battle Orders** pour booster massivement le groupe.

### Nommage Sodomight
- **Barbarian** → **Ravageur** (guerrier sauvage des territoires nordiques de Solmer)
- Warcries → **Cris de Guerre** ; Masteries → **Maîtrises**

---

### Arbre 1 : Maîtrises de Combat (Combat Masteries)

#### 4.1.1 Maîtrise des Épées (Sword Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Sword Mastery |
| **Nom Sodomight** | Maîtrise des Lames |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Description :** Augmente les dégâts, l'AR et la chance de coup critique pour toutes les épées.

**Formule :**
| Niveau | Damage bonus | AR bonus | Crit chance |
|--------|-------------|---------|------------|
| 1 | +28% | +30% | 5% |
| 10 | +145% | +165% | 14% |
| 20 | +280% | +315% | 24% |

**Notes gameplay :** 1 seule maîtrise d'arme est utile par build — choisir selon le type d'arme principal.

---

#### 4.1.2 Maîtrise des Haches (Axe Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Axe Mastery |
| **Nom Sodomight** | Maîtrise des Haches |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :** Identique à Maîtrise des Lames (même scaling).

---

#### 4.1.3 Maîtrise des Masses (Mace Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Mace Mastery |
| **Nom Sodomight** | Maîtrise des Masses |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :** Identique aux autres maîtrises d'armes.

---

#### 4.1.4 Maîtrise des Hallebardes (Pole Arm Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Pole Arm Mastery |
| **Nom Sodomight** | Maîtrise des Hampes |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

---

#### 4.1.5 Maîtrise des Lances (Throwing Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Throwing Mastery |
| **Nom Sodomight** | Maîtrise du Lancer |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |
| **Effet spécial** | Ajoute aussi une chance de récupérer les armes lancées |

**Formule (chance de récupération) :**
| Niveau | Récupération |
|--------|-------------|
| 1 | 5% |
| 10 | 32% |
| 20 | 50% |

---

#### 4.1.6 Maîtrise des Javelines (Spear Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Spear Mastery |
| **Nom Sodomight** | Maîtrise des Javelines |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

---

#### 4.1.7 Endurance Accrue (Increased Stamina)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Increased Stamina |
| **Nom Sodomight** | Endurance Accrue |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 6 |
| **Prérequis** | N'importe quelle Maîtrise niveau 1 |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Stamina bonus | Regen bonus |
|--------|--------------|------------|
| 1 | +15% | +15% |
| 10 | +100% | +100% |
| 20 | +200% | +200% |

---

#### 4.1.8 Peau de Fer (Iron Skin)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Iron Skin |
| **Nom Sodomight** | Peau de Fer |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 12 |
| **Prérequis** | Endurance Accrue |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Defense bonus |
|--------|--------------|
| 1 | +30% |
| 10 | +165% |
| 20 | +315% |

---

#### 4.1.9 Vitesse Accrue (Increased Speed)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Increased Speed |
| **Nom Sodomight** | Vitesse Accrue |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 18 |
| **Prérequis** | Endurance Accrue |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Walk/Run Speed |
|--------|---------------|
| 1 | +13% |
| 10 | +30% |
| 20 | +48% |

---

#### 4.1.10 Résistance Naturelle (Natural Resistance)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Natural Resistance |
| **Nom Sodomight** | Résistance Naturelle |
| **Arbre** | Maîtrises de Combat |
| **Niveau requis** | 24 |
| **Prérequis** | Peau de Fer |
| **Coût mana** | Passif |

**Formule :**
| Niveau | All Resist bonus |
|--------|-----------------|
| 1 | +8% |
| 10 | +32% |
| 20 | +52% |

**Notes gameplay :** Utile pour atteindre les caps de résistance (75% en Normal, 75% en Nightmare/Hell nécessite des items). Quelques points aident significativement.

---

### Arbre 2 : Compétences de Combat (Combat Skills)

#### 4.2.1 Coup Brutal (Bash)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Bash |
| **Nom Sodomight** | Coup Brutal |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2 |
| **Type** | Physique (knockback) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Knockback |
|--------|-------------|---------|----------|
| 1 | +50% | +20% | Oui |
| 10 | +230% | +110% | Oui |
| 20 | +430% | +210% | Oui |

**Synergies données :** Stun +14% Damage/niveau ; Concentrate +14% Damage/niveau.

---

#### 4.2.2 Double Swing
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Double Swing |
| **Nom Sodomight** | Double Frappe |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 6 |
| **Prérequis** | Coup Brutal |
| **Coût mana** | 2 (0 si les deux mains touchent) |
| **Type** | Physique (2 frappes simultanées, dual wield requis) |

**Formule :**
| Niveau | Damage bonus (chaque frappe) | AR bonus |
|--------|------------------------------|---------|
| 1 | +30% | +20% |
| 10 | +165% | +110% |
| 20 | +315% | +210% |

**Synergies données :** Frenzy +8% Damage/niveau.

**Notes gameplay :** Prérequis pour Frenzy. Peu utilisé seul.

---

#### 4.2.3 Bond (Leap)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Leap |
| **Nom Sodomight** | Bond |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 6 |
| **Prérequis** | Coup Brutal |
| **Coût mana** | 2 |
| **Type** | Déplacement (saut) |
| **Cooldown** | ~2s |
| **Effet spécial** | Knockback à l'atterrissage |

**Description :** Saute jusqu'à une distance cible, knockbackant les ennemis à l'atterrissage. Moyen de déplacement rapide et d'échappement.

**Formule :**
| Niveau | Rayon de knockback | Distance max |
|--------|-------------------|-------------|
| 1 | 2m | ~8m |
| 10 | 3m | ~10m |
| 20 | 4.6m | ~13m |

**Synergies données :** Leap Attack +10% Damage/niveau.

---

#### 4.2.4 Double Lancer (Double Throw)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Double Throw |
| **Nom Sodomight** | Double Lancer |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 12 |
| **Prérequis** | Double Frappe |
| **Coût mana** | 2 |
| **Type** | Physique (2 projectiles, armes de lancer) |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +75% | +30% |
| 10 | +345% | +165% |
| 20 | +645% | +315% |

---

#### 4.2.5 Assommoir (Stun)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Stun |
| **Nom Sodomight** | Assommoir |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 12 |
| **Prérequis** | Coup Brutal |
| **Coût mana** | 3 |
| **Type** | Physique + Stun |
| **Durée stun** | 1.6s base |

**Formule :**
| Niveau | Damage bonus | AR bonus | Durée stun |
|--------|-------------|---------|-----------|
| 1 | +125% | +30% | 1.6s |
| 10 | +575% | +165% | 4s |
| 20 | +1075% | +315% | 7.4s |

**Synergies données :** Whirlwind +10% Damage/niveau ; War Cry +6% Damage/niveau (via Howl).

---

#### 4.2.6 Bond d'Attaque (Leap Attack)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Leap Attack |
| **Nom Sodomight** | Bond Destructeur |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 18 |
| **Prérequis** | Bond |
| **Coût mana** | 4 |
| **Type** | Physique (saut + frappe AoE à l'atterrissage) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Rayon AoE |
|--------|-------------|---------|----------|
| 1 | +200% | +60% | 2.6m |
| 10 | +1100% | +330% | 3.3m |
| 20 | +2100% | +630% | 4m |

**Synergies reçues :** Bond +10% Damage/niveau.

**Notes gameplay :** Excellent pour initier des combats contre des packs. Le saut permet de passer par-dessus les obstacles. Builds "Leap Attack Barb" viables.

---

#### 4.2.7 Concentration (Concentrate)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Concentrate |
| **Nom Sodomight** | Frappe Concentrée |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 18 |
| **Prérequis** | Assommoir |
| **Coût mana** | 4 |
| **Type** | Physique (non-interruptible) |
| **Effet spécial** | L'attaque ne peut pas être interrompue par les hits |

**Formule :**
| Niveau | Damage bonus | AR bonus | Defense bonus |
|--------|-------------|---------|--------------|
| 1 | +175% | +60% | +100% |
| 10 | +1050% | +330% | +550% |
| 20 | +2050% | +630% | +1050% |

**Synergies reçues :** Battle Orders +10% Damage/niveau ; Coup Brutal +14% Damage/niveau.

**Notes gameplay :** Skill de choix pour les Ravageurs utilisant un bouclier (1 main + bouclier). Le bonus de défense est massif.

---

#### 4.2.8 Frénésie (Frenzy)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Frenzy |
| **Nom Sodomight** | Frénésie |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 24 |
| **Prérequis** | Double Frappe |
| **Coût mana** | 4 |
| **Type** | Physique (dual wield, stacks de vitesse) |
| **Effet spécial** | Chaque usage augmente la vitesse d'attaque et de déplacement (jusqu'à 5 stacks) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Speed (par stack) |
|--------|-------------|---------|------------------|
| 1 | +135% | +60% | +8% |
| 10 | +765% | +330% | +14% |
| 20 | +1465% | +630% | +20% |

**Synergies reçues :** Double Frappe +8% Damage/niveau ; Taunt +8% Damage/niveau.

**Notes gameplay :** Build "Frenzy Barb" — l'un des plus puissants pour le farming. Le stacking de vitesse rend le Ravageur extrêmement rapide et dangereux.

---

#### 4.2.9 Tourbillon (Whirlwind)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Whirlwind |
| **Nom Sodomight** | Tourbillon |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 30 |
| **Prérequis** | Frappe Concentrée |
| **Coût mana** | 25 (base) |
| **Type** | Physique (AoE — frappe tout sur le passage) |
| **Effet spécial** | Le Ravageur tourne sur lui-même en se déplaçant, frappant tout ce qu'il croise |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +100% | +50% |
| 10 | +450% | +270% |
| 20 | +850% | +490% |

**Synergies reçues :** Assommoir +10% Damage/niveau ; Battle Orders +10% Damage/niveau.

**Notes gameplay :** **Skill signature du Ravageur.** "Whirlwind Barb" est le build endgame classique. Frapper en se déplaçant permet de couvrir énormément de terrain. Excellent IAS requis pour un bon Whirlwind.

---

#### 4.2.10 Berserk
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Berserk |
| **Nom Sodomight** | Berserk |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 30 |
| **Prérequis** | Frénésie |
| **Coût mana** | 8 |
| **Type** | Magique (100% conversion physique → magique) |
| **Effet spécial** | 100% conversion physique en dégâts magiques ; défense réduite à 0 pendant l'attaque |

**Formule :**
| Niveau | Damage bonus (magique) | AR bonus |
|--------|----------------------|---------|
| 1 | +100% | +40% |
| 10 | +700% | +220% |
| 20 | +1400% | +420% |

**Synergies reçues :** Howl +10% Damage magique/niveau ; Battle Orders +10% Damage magique/niveau.

**Notes gameplay :** Spécialisé pour tuer les ennemis immunisés aux dégâts physiques. La vulnérabilité défensive impose une gestion prudente. Souvent 1 point pour les situations nécessitant du dégât magique.

---

### Arbre 3 : Cris de Guerre (Warcries)

#### 4.3.1 Rugissement (Howl)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Howl |
| **Nom Sodomight** | Rugissement |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 4 |
| **Type** | Cri de guerre (fuite forcée AoE) |
| **Range** | ~7.3m rayon |
| **Durée** | 8s base |

**Description :** Force tous les ennemis proches à fuir en panique.

**Synergies données :** War Cry +6% Damage/niveau ; Berserk +10% Magic Damage/niveau.

---

#### 4.3.2 Trouver Potion (Find Potion)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Find Potion |
| **Nom Sodomight** | Fouille Vitale |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2 |
| **Type** | Utilitaire |
| **Effet spécial** | Chance de trouver une potion en frappant les cadavres |

**Formule :**
| Niveau | Chance de trouver |
|--------|------------------|
| 1 | 50% |
| 10 | 57% |
| 20 | 63% |

---

#### 4.3.3 Cri de Défense (Shout)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shout |
| **Nom Sodomight** | Cri de Défense |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 6 |
| **Prérequis** | Rugissement |
| **Coût mana** | 6 |
| **Type** | Buff (défense du groupe) |
| **Durée** | 144s base |

**Formule :**
| Niveau | Defense bonus (groupe) | Durée |
|--------|----------------------|-------|
| 1 | +100% | 144s |
| 10 | +250% | 252s |
| 20 | +450% | 372s |

**Synergies données :** Battle Orders +7% Defense/niveau.

---

#### 4.3.4 Cri de Guerre (War Cry)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | War Cry |
| **Nom Sodomight** | Cri de Guerre |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 6 |
| **Prérequis** | Fouille Vitale |
| **Coût mana** | 11 |
| **Type** | Offensif AoE (dégâts magiques + stun) |
| **Range** | ~3.3m rayon |

**Formule :**
| Niveau | Dégâts magiques | Durée stun |
|--------|----------------|-----------|
| 1 | 5-9 | 1.6s |
| 10 | 38-60 | 3.5s |
| 20 | 83-125 | 6s |

**Synergies reçues :** Rugissement, Cri de Guerre, Provocation, Battle Orders +6% Damage chacun/niveau.

**Notes gameplay :** Build "Singer Barb" — maxer War Cry et ses synergies pour un Ravageur qui tue avec ses cris.

---

#### 4.3.5 Trouver Objet (Find Item)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Find Item |
| **Nom Sodomight** | Fouille Profonde |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 12 |
| **Prérequis** | Fouille Vitale |
| **Coût mana** | 8 |
| **Type** | Utilitaire (MF supplémentaire) |
| **Effet spécial** | Chance de trouver un objet supplémentaire dans les cadavres |

**Formule :**
| Niveau | Chance |
|--------|--------|
| 1 | 12% |
| 10 | 23% |
| 20 | 34% |

**Notes gameplay :** Skill de farming très utile. Le Ravageur peut farmer le Magic Find plus efficacement que d'autres classes grâce à cela.

---

#### 4.3.6 Provocation (Taunt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Taunt |
| **Nom Sodomight** | Provocation |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 12 |
| **Prérequis** | Cri de Défense |
| **Coût mana** | 3 |
| **Type** | Debuff + aggro |
| **Effet** | Attire les ennemis ; réduit leurs dégâts et vitesse d'attaque |

**Formule :**
| Niveau | Réduction dégâts ennemis | Réduction vitesse attaque |
|--------|--------------------------|--------------------------|
| 1 | -10% | -10% |
| 10 | -28% | -28% |
| 20 | -48% | -48% |

**Synergies données :** Frénésie +8% Damage/niveau ; War Cry +6% Damage/niveau.

---

#### 4.3.7 Ordres de Bataille (Battle Orders)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Battle Orders |
| **Nom Sodomight** | Ordres de Bataille |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 18 |
| **Prérequis** | Cri de Guerre |
| **Coût mana** | 7 |
| **Type** | Buff de groupe (Life/Mana/Stamina) |
| **Durée** | 144s base |

**Formule :**
| Niveau | Life/Mana/Stamina bonus (groupe) | Durée |
|--------|----------------------------------|-------|
| 1 | +50% | 144s |
| 5 | +68% | 180s |
| 10 | +100% | 228s |
| 20 | +163% | 324s |

**Synergies reçues :** Cri de Défense +7% Duration/niveau.
**Synergies données :** Frappe Concentrée +10% Damage/niveau ; Tourbillon +10% Damage/niveau ; Berserk +10% Magic Damage/niveau.

**Notes gameplay :** **Skill le plus important du Ravageur pour le groupe.** Battle Orders niveau 20 double pratiquement la vie de tout le groupe. Utilisé dans TOUS les builds Ravageur comme buff de groupe. Le mercenaire en bénéficie aussi.

---

#### 4.3.8 Trophée Funèbre (Grim Ward)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Grim Ward |
| **Nom Sodomight** | Trophée Macabre |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 24 |
| **Prérequis** | Fouille Profonde |
| **Coût mana** | 5 |
| **Type** | Totem (ralentit et effraie) |
| **Durée** | 24s base |
| **Effet spécial** | Transforme un cadavre en totème qui fait fuir les ennemis proches |

**Formule :**
| Niveau | Rayon de terreur | Durée |
|--------|-----------------|-------|
| 1 | 4m | 24s |
| 10 | 6m | 36s |
| 20 | 8m | 50s |

---

#### 4.3.9 Commandement de Bataille (Battle Command)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Battle Command |
| **Nom Sodomight** | Commandement de Bataille |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 30 |
| **Prérequis** | Ordres de Bataille |
| **Coût mana** | 9 |
| **Type** | Buff de groupe (+1 skill level à tous) |
| **Durée** | 144s base |
| **Effet spécial** | Ajoute +1 niveau à TOUS les skills de tout le groupe |

**Notes gameplay :** **Extrêmement puissant en groupe.** +1 à tous les skills pour tout le groupe pendant 144s+. Utilisé en rotation avec Battle Orders pour maintenir les deux buffs.

---

#### 4.3.10 Cri de Bataille (Battle Cry)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Battle Cry |
| **Nom Sodomight** | Cri de Bataille |
| **Arbre** | Cris de Guerre |
| **Niveau requis** | 24 |
| **Prérequis** | Provocation |
| **Coût mana** | 7 |
| **Type** | Debuff AoE |
| **Durée** | 10s base |
| **Effets** | Réduit la défense et les dégâts des ennemis |

**Formule :**
| Niveau | Defense réduite | Dégâts réduits |
|--------|----------------|----------------|
| 1 | -50% | -25% |
| 10 | -75% | -35% |
| 20 | -100% | -45% |

**Synergies données :** War Cry +6% Damage/niveau.

---

## 5. Sorceress → Arcaniste

### Description de classe
L'Arcaniste est la classe de lancer de sorts la plus puissante du jeu. Elle maîtrise trois éléments : Feu, Froid et Foudre. Elle possède Téléporation, une capacité de déplacement instantané unique. Classe fragile en points de vie mais dévastatrice en dégâts élémentaux.

### Nommage Sodomight
- **Sorceress** → **Arcaniste** (tisseuse des arcanes élémentaux)
- Les trois arbres restent thématiquement Fire/Cold/Lightning mais renommés : **Pyromance / Cryomance / Fulguromance**

---

### Arbre 1 : Sorts de Feu (Fire Spells / Pyromance)

#### 5.1.1 Eclair de Feu (Fire Bolt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Bolt |
| **Nom Sodomight** | Eclair de Feu |
| **Arbre** | Pyromance |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2.5 (base) |
| **Type** | Feu (projectile) |

**Formule :**
| Niveau | Feu min | Feu max |
|--------|---------|---------|
| 1 | 3 | 6 |
| 10 | 30 | 45 |
| 20 | 72 | 99 |

**Synergies reçues :** Boule de Feu +16% Damage/niveau ; Météore +16% Damage/niveau.

---

#### 5.1.2 Chaleur (Warmth)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Warmth |
| **Nom Sodomight** | Chaleur Intérieure |
| **Arbre** | Pyromance |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |
| **Type** | Passif (régénération de mana) |

**Formule :**
| Niveau | Regen mana bonus |
|--------|-----------------|
| 1 | +30% |
| 10 | +120% |
| 20 | +230% |

**Notes gameplay :** Quelques points accélèrent la régénération de mana entre les combats. Peu prioritaire avec des potions de mana.

---

#### 5.1.3 Inferno
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Inferno |
| **Nom Sodomight** | Inferno |
| **Arbre** | Pyromance |
| **Niveau requis** | 6 |
| **Prérequis** | Eclair de Feu |
| **Coût mana** | 7 (puis continu : 2/s) |
| **Type** | Feu (jet continu) |

**Formule :**
| Niveau | Feu/sec min | Feu/sec max |
|--------|------------|------------|
| 1 | 12 | 25 |
| 10 | 72 | 110 |
| 20 | 152 | 215 |

**Notes gameplay :** Skill de bas niveau, peu utilisé en endgame. Prérequis pour Blaze.

---

#### 5.1.4 Brasier (Blaze)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blaze |
| **Nom Sodomight** | Traînée de Feu |
| **Arbre** | Pyromance |
| **Niveau requis** | 12 |
| **Prérequis** | Inferno |
| **Coût mana** | 11 |
| **Type** | Feu (traînée de feu au sol sur le chemin de l'Arcaniste) |
| **Durée traînée** | 6s base |

**Formule :**
| Niveau | Feu/sec min-max | Durée |
|--------|----------------|-------|
| 1 | 18-37 | 6s |
| 10 | 98-165 | 9s |
| 20 | 198-315 | 13s |

---

#### 5.1.5 Boule de Feu (Fire Ball)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Ball |
| **Nom Sodomight** | Boule de Feu |
| **Arbre** | Pyromance |
| **Niveau requis** | 12 |
| **Prérequis** | Eclair de Feu |
| **Coût mana** | 6 (base), +0.125/niveau |
| **Type** | Feu (AoE à l'impact) |

**Formule :**
| Niveau | Feu min | Feu max |
|--------|---------|---------|
| 1 | 6 | 15 |
| 10 | 62 | 88 |
| 20 | 150 | 196 |

**Synergies reçues :** Eclair de Feu +16% Damage/niveau ; Météore +14% Damage/niveau.
**Synergies données :** Eclair de Feu +16% Damage/niveau ; Météore +5% Damage/niveau ; Hydre +3% Damage/niveau.

**Notes gameplay :** Skill principal du build "Fireball Sorc". Excellent AoE, excellent scaling.

---

#### 5.1.6 Mur de Feu (Fire Wall)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Wall |
| **Nom Sodomight** | Mur de Flammes |
| **Arbre** | Pyromance |
| **Niveau requis** | 18 |
| **Prérequis** | Traînée de Feu |
| **Coût mana** | 22 |
| **Type** | Feu (barrière de feu persistante) |
| **Durée** | 6s base |

**Formule :**
| Niveau | Feu/sec | Durée |
|--------|---------|-------|
| 1 | 70-94 | 6s |
| 10 | 290-370 | 9s |
| 20 | 570-710 | 13s |

**Notes gameplay :** Excellent pour le kiting (forcer les ennemis à traverser le mur). Peu utile en mob clearing rapide mais dévastateur en solo boss.

---

#### 5.1.7 Enchantement (Enchant)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Enchant |
| **Nom Sodomight** | Enchantement Igné |
| **Arbre** | Pyromance |
| **Niveau requis** | 18 |
| **Prérequis** | Boule de Feu |
| **Coût mana** | 26 |
| **Type** | Buff (ajoute des dégâts de feu aux attaques physiques) |
| **Durée** | 144s base |

**Formule :**
| Niveau | Feu ajouté min | Feu ajouté max | AR bonus |
|--------|---------------|---------------|---------|
| 1 | 7 | 11 | +70% |
| 10 | 87 | 115 | +340% |
| 20 | 207 | 267 | +640% |

**Notes gameplay :** Excellent pour booster un mercenaire ou un allié de mêlée. Build "Enchantress" possible mais de niche.

---

#### 5.1.8 Météore (Meteor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Meteor |
| **Nom Sodomight** | Météore |
| **Arbre** | Pyromance |
| **Niveau requis** | 24 |
| **Prérequis** | Mur de Flammes |
| **Coût mana** | 17 |
| **Type** | Feu (impact AoE + cratère de feu persistant) |
| **Délai** | ~1.6s avant l'impact |

**Formule :**
| Niveau | Impact feu min-max | Feu/sec (cratère) | Durée cratère |
|--------|-------------------|------------------|--------------|
| 1 | 80-100 | 40-50 | 4s |
| 10 | 400-500 | 200-250 | 6s |
| 20 | 800-1000 | 400-500 | 9s |

**Synergies reçues :** Boule de Feu +14% Damage/niveau ; Eclair de Feu +14% Damage/niveau.
**Synergies données :** Boule de Feu +5% Damage/niveau ; Eclair de Feu +16% Damage/niveau.

**Notes gameplay :** Build "Meteorb" (Meteor + Frozen Orb) — l'un des builds les plus polyvalents de l'Arcaniste.

---

#### 5.1.9 Maîtrise du Feu (Fire Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Mastery |
| **Nom Sodomight** | Maîtrise Pyromantique |
| **Arbre** | Pyromance |
| **Niveau requis** | 30 |
| **Prérequis** | Enchantement Igné |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Bonus dégâts feu | Pénétration résistance |
|--------|-----------------|----------------------|
| 1 | +30% | -5% |
| 10 | +165% | -14% |
| 20 | +315% | -24% |

**Notes gameplay :** Booster passif obligatoire pour tout build feu. Réduit les résistances au feu des ennemis (pénétration), ce qui est précieux en Hell.

---

#### 5.1.10 Hydre
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Hydra |
| **Nom Sodomight** | Hydre Pyromantique |
| **Arbre** | Pyromance |
| **Niveau requis** | 30 |
| **Prérequis** | Météore |
| **Coût mana** | 20 |
| **Type** | Feu (invocation de tourelles de feu) |
| **Durée** | 10s base |
| **Effet spécial** | Invoque une Hydre qui tire automatiquement des projectiles de feu sur les ennemis |

**Formule :**
| Niveau | Feu par projectile | Projectiles/sec | Durée |
|--------|--------------------|----------------|-------|
| 1 | 14-19 | ~2/s | 10s |
| 10 | 66-87 | ~2/s | 14s |
| 20 | 134-175 | ~2/s | 18s |

**Synergies reçues :** Boule de Feu +3% Damage/niveau ; Maîtrise Pyromantique +2% Damage/niveau.

**Notes gameplay :** Excellent pour les zones d'attente (poser l'Hydre et laisser les ennemis venir). Peut avoir plusieurs Hydres simultanées à haut niveau.

---

### Arbre 2 : Sorts de Froid (Cold Spells / Cryomance)

#### 5.2.1 Eclat de Glace (Ice Bolt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Ice Bolt |
| **Nom Sodomight** | Eclat de Glace |
| **Arbre** | Cryomance |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 3 |
| **Type** | Froid (chill) |

**Formule :**
| Niveau | Froid min | Froid max |
|--------|-----------|-----------|
| 1 | 3 | 5 |
| 10 | 24 | 33 |
| 20 | 57 | 73 |

**Synergies données :** Explosion de Glace +8%/niveau ; Pointe Glaciale +5%/niveau ; Blizzard +5%/niveau ; Orbe Gelée +2%/niveau.

---

#### 5.2.2 Armure de Givre (Frozen Armor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Frozen Armor |
| **Nom Sodomight** | Armure de Givre |
| **Arbre** | Cryomance |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 7 |
| **Type** | Défensif (gèle les attaquants en mêlée) |
| **Durée** | 144s |

**Formule :**
| Niveau | Defense bonus | Durée freeze attaquant |
|--------|--------------|----------------------|
| 1 | +30% | 3s |
| 10 | +165% | 5s |
| 20 | +315% | 7s |

**Notes gameplay :** 1 point pour le bonus de défense passif. Remplaçable par Shiver Armor ou Chilling Armor selon le build.

---

#### 5.2.3 Nova de Givre (Frost Nova)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Frost Nova |
| **Nom Sodomight** | Nova de Givre |
| **Arbre** | Cryomance |
| **Niveau requis** | 6 |
| **Prérequis** | Eclat de Glace |
| **Coût mana** | 9 |
| **Type** | Froid AoE (nova 360°) |

**Formule :**
| Niveau | Froid min | Froid max |
|--------|-----------|-----------|
| 1 | 2 | 6 |
| 10 | 14 | 28 |
| 20 | 30 | 56 |

**Synergies données :** Orbe Gelée +5%/niveau.

---

#### 5.2.4 Explosion de Glace (Ice Blast)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Ice Blast |
| **Nom Sodomight** | Explosion de Glace |
| **Arbre** | Cryomance |
| **Niveau requis** | 6 |
| **Prérequis** | Eclat de Glace |
| **Coût mana** | 6 |
| **Type** | Froid (gèle la cible) |

**Formule :**
| Niveau | Froid min | Froid max | Durée freeze |
|--------|-----------|-----------|-------------|
| 1 | 8 | 12 | 2s |
| 10 | 55 | 72 | 3.3s |
| 20 | 128 | 160 | 4.6s |

**Synergies reçues :** Eclat de Glace +8%/niveau ; Pointe Glaciale +6%/niveau.

---

#### 5.2.5 Armure de Frisson (Shiver Armor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shiver Armor |
| **Nom Sodomight** | Armure de Frisson |
| **Arbre** | Cryomance |
| **Niveau requis** | 12 |
| **Prérequis** | Nova de Givre |
| **Coût mana** | 11 |
| **Type** | Défensif + riposte froid |
| **Durée** | 144s |

**Formule :**
| Niveau | Defense bonus | Dommage froid riposte |
|--------|--------------|----------------------|
| 1 | +45% | 6-8 |
| 10 | +225% | 52-68 |
| 20 | +435% | 120-152 |

---

#### 5.2.6 Pointe Glaciale (Glacial Spike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Glacial Spike |
| **Nom Sodomight** | Pointe Glaciale |
| **Arbre** | Cryomance |
| **Niveau requis** | 18 |
| **Prérequis** | Explosion de Glace |
| **Coût mana** | 10 |
| **Type** | Froid AoE (éclats à l'impact, gèle) |

**Formule :**
| Niveau | Froid min | Froid max |
|--------|-----------|-----------|
| 1 | 17 | 26 |
| 10 | 100 | 138 |
| 20 | 225 | 295 |

**Synergies reçues :** Eclat de Glace +5%/niveau ; Explosion de Glace +6%/niveau ; Blizzard +3%/niveau.
**Synergies données :** Explosion de Glace +6%/niveau ; Blizzard +4%/niveau.

---

#### 5.2.7 Blizzard
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blizzard |
| **Nom Sodomight** | Blizzard |
| **Arbre** | Cryomance |
| **Niveau requis** | 24 |
| **Prérequis** | Armure de Frisson |
| **Coût mana** | 23 |
| **Type** | Froid AoE (pluie de glace sur une zone) |
| **Durée** | 3.2s (pluie continue) |

**Formule :**
| Niveau | Froid/s min | Froid/s max |
|--------|------------|------------|
| 1 | 51 | 86 |
| 10 | 275 | 430 |
| 20 | 580 | 880 |

**Synergies reçues :** Eclat de Glace +5%/niveau ; Pointe Glaciale +4%/niveau.

**Notes gameplay :** Skill AoE puissant du build "Blizzard Sorc". Avec la Maîtrise du Froid, les résistances ennemies s'effondrent.

---

#### 5.2.8 Armure Glaciale (Chilling Armor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Chilling Armor |
| **Nom Sodomight** | Armure Glaciale |
| **Arbre** | Cryomance |
| **Niveau requis** | 24 |
| **Prérequis** | Pointe Glaciale |
| **Coût mana** | 17 |
| **Type** | Défensif + riposte projectile |
| **Durée** | 144s |
| **Effet spécial** | Riposte automatique aux projectiles ennemis avec des éclairs de froid |

**Formule :**
| Niveau | Defense bonus | Dommage riposte |
|--------|--------------|----------------|
| 1 | +45% | 4-6 |
| 10 | +225% | 35-52 |
| 20 | +435% | 85-120 |

---

#### 5.2.9 Orbe Gelée (Frozen Orb)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Frozen Orb |
| **Nom Sodomight** | Orbe Gelée |
| **Arbre** | Cryomance |
| **Niveau requis** | 30 |
| **Prérequis** | Armure Glaciale |
| **Coût mana** | 25 |
| **Type** | Froid (orbe qui tourne et éjecte des Eclats de Glace) |
| **Cooldown** | 1s |

**Formule :**
| Niveau | Froid par Eclat | Eclats émis |
|--------|----------------|------------|
| 1 | 40-45 | 16 |
| 10 | 40-45 | 16 (inchangé) |
| 20 | 40-45 | 16 (inchangé) |

Note : La puissance de l'Orbe vient quasi exclusivement des synergies — les dégâts de base restent presque constants.

**Synergies reçues :** Eclat de Glace +2%/niveau ; Nova de Givre +5%/niveau.

**Notes gameplay :** **Build "Frozen Orb Sorc"** — l'un des builds les plus populaires et polyvalents. Combiné avec Meteor = build "Meteorb". Le cooldown d'1s impose un rythme cadencé.

---

#### 5.2.10 Maîtrise du Froid (Cold Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cold Mastery |
| **Nom Sodomight** | Maîtrise Cryomantique |
| **Arbre** | Cryomance |
| **Niveau requis** | 30 |
| **Prérequis** | Eclat de Glace (niveau 1) |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Pénétration résistance froid |
|--------|------------------------------|
| 1 | -20% |
| 5 | -40% |
| 10 | -65% |
| 20 | -115% |

**Notes gameplay :** **La maîtrise la plus précieuse du jeu.** Contrairement à Fire/Lightning Mastery qui booste les dégâts, Cold Mastery pénètre les résistances — ce qui peut briser les immunités au froid (résistance 100% → 100% - 115% = en dessous de 100% = plus immunisé). Maxer en priorité pour les builds froid en Hell.

---

### Arbre 3 : Sorts de Foudre (Lightning Spells / Fulguromance)

#### 5.3.1 Eclair Chargé (Charged Bolt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Charged Bolt |
| **Nom Sodomight** | Eclats de Foudre |
| **Arbre** | Fulguromance |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 3 |
| **Type** | Foudre (multiple Charged Bolts aléatoires) |

**Formule :**
| Niveau | Foudre par bolt | Bolts émis |
|--------|----------------|-----------|
| 1 | 2-4 | 3 |
| 10 | 12-22 | 9 |
| 20 | 26-46 | 17 |

**Synergies reçues :** Foudre +6% Damage/niveau ; Chaîne d'Eclairs +4%/niveau.

---

#### 5.3.2 Champ Statique (Static Field)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Static Field |
| **Nom Sodomight** | Champ Statique |
| **Arbre** | Fulguromance |
| **Niveau requis** | 6 |
| **Prérequis** | Eclats de Foudre |
| **Coût mana** | 9 |
| **Type** | Foudre AoE (retire % de la vie actuelle) |
| **Limites** | Plafond en difficulté : 33% de la vie actuelle max en Normal, 50% en Nightmare/Hell |

**Description :** Retire 25% de la vie actuelle de TOUS les ennemis dans le rayon. Ne peut pas tuer — réduit jusqu'à 1 HP minimum. Extrêmement puissant pour descendre rapidement les boss.

**Notes gameplay :** **Skill essentiel pour tout build Arcaniste.** 1 point suffit pour l'effet. Le % retiré diminue en difficulté supérieure mais reste excellent pour préparer un boss kill.

---

#### 5.3.3 Télékinésie (Telekinesis)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Telekinesis |
| **Nom Sodomight** | Télékinésie |
| **Arbre** | Fulguromance |
| **Niveau requis** | 6 |
| **Prérequis** | Eclats de Foudre |
| **Coût mana** | 7 |
| **Type** | Télékinésie (ramasser items à distance, pousser ennemis) |
| **Effet spécial** | Avec Energy Shield : réduit le coût du dégât transféré en mana |

**Notes gameplay :** Principalement utilisé pour ramasser des items sans se déplacer. Essentiel pour optimiser Energy Shield (chaque point réduit le ratio mana/dégât).

---

#### 5.3.4 Foudre (Lightning)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning |
| **Nom Sodomight** | Foudre |
| **Arbre** | Fulguromance |
| **Niveau requis** | 12 |
| **Prérequis** | Champ Statique |
| **Coût mana** | 8 (base) |
| **Type** | Foudre (projectile ligne droite, dégâts variables) |

**Formule :**
| Niveau | Foudre min | Foudre max |
|--------|-----------|-----------|
| 1 | 1 | 43 |
| 10 | 1 | 253 |
| 20 | 1 | 537 |

**Synergies données :** Eclats de Foudre +6%/niveau ; Chaîne d'Eclairs +4%/niveau.
**Synergies reçues :** Chaîne d'Eclairs +7%/niveau ; Orage +4%/niveau.

**Notes gameplay :** Dégâts max très élevés mais minimum toujours à 1 — variance extrême. Base du "Lightning Sorc".

---

#### 5.3.5 Nova
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Nova |
| **Nom Sodomight** | Nova de Foudre |
| **Arbre** | Fulguromance |
| **Niveau requis** | 12 |
| **Prérequis** | Télékinésie |
| **Coût mana** | 15 |
| **Type** | Foudre AoE 360° |

**Formule :**
| Niveau | Foudre min | Foudre max |
|--------|-----------|-----------|
| 1 | 1 | 20 |
| 10 | 1 | 110 |
| 20 | 1 | 230 |

---

#### 5.3.6 Chaîne d'Eclairs (Chain Lightning)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Chain Lightning |
| **Nom Sodomight** | Chaîne d'Eclairs |
| **Arbre** | Fulguromance |
| **Niveau requis** | 18 |
| **Prérequis** | Foudre |
| **Coût mana** | 9 |
| **Type** | Foudre (rebondit entre ennemis) |

**Formule :**
| Niveau | Foudre min | Foudre max | Rebonds |
|--------|-----------|-----------|---------|
| 1 | 1 | 40 | 2 |
| 10 | 1 | 260 | 7 |
| 20 | 1 | 540 | 15 |

**Synergies reçues :** Foudre +7%/niveau ; Orage +4%/niveau.
**Synergies données :** Foudre +4%/niveau ; Eclats de Foudre +4%/niveau.

---

#### 5.3.7 Téléporation (Teleport)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Teleport |
| **Nom Sodomight** | Téléporation |
| **Arbre** | Fulguromance |
| **Niveau requis** | 18 |
| **Prérequis** | Télékinésie |
| **Coût mana** | 24 (base), diminue avec le niveau |
| **Type** | Utilitaire (déplacement instantané) |
| **Cooldown** | Aucun |

**Description :** Téléporte instantanément l'Arcaniste à l'endroit ciblé. Skill le plus important de la classe pour la mobilité et la sécurité.

**Formule — coût mana :**
| Niveau | Coût mana |
|--------|----------|
| 1 | 24 |
| 5 | 20 |
| 10 | 17 |
| 20 | 11 |

**Notes gameplay :** **Skill absolument indispensable.** Permet de traverser les maps à une vitesse folle, d'échapper aux situations dangereuses, et de se positionner stratégiquement. 1 point suffit — le coût de mana reste gérable.

---

#### 5.3.8 Orage (Thunder Storm)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Thunder Storm |
| **Nom Sodomight** | Orage |
| **Arbre** | Fulguromance |
| **Niveau requis** | 24 |
| **Prérequis** | Chaîne d'Eclairs |
| **Coût mana** | 19 |
| **Type** | Foudre (frappe automatique toutes les 2-4s) |
| **Durée** | 40s base |

**Formule :**
| Niveau | Foudre min | Foudre max | Délai entre frappes |
|--------|-----------|-----------|---------------------|
| 1 | 1 | 100 | 4s |
| 10 | 1 | 550 | 2.5s |
| 20 | 1 | 1100 | 1.5s |

**Synergies données :** Foudre +4%/niveau ; Chaîne d'Eclairs +4%/niveau.

---

#### 5.3.9 Bouclier d'Energie (Energy Shield)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Energy Shield |
| **Nom Sodomight** | Bouclier d'Energie |
| **Arbre** | Fulguromance |
| **Niveau requis** | 24 |
| **Prérequis** | Nova de Foudre |
| **Coût mana** | 6 |
| **Type** | Défensif (absorbe une portion des dégâts comme mana) |
| **Durée** | 144s |

**Formule :**
| Niveau | % dégâts absorbés comme mana | Durée |
|--------|------------------------------|-------|
| 1 | 20% | 144s |
| 10 | 40% | 220s |
| 20 | 65% | 310s |

Avec Télékinésie, le ratio de conversion mana→dégâts est amélioré (moins de mana consommé par dégât).

**Notes gameplay :** Build "ES Sorc" — maximiser Energy Shield + Télékinésie + grande réserve de mana pour avoir une défense massive. Permet de jouer avec très peu de vie car les dégâts passent en mana.

---

#### 5.3.10 Maîtrise de la Foudre (Lightning Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning Mastery |
| **Nom Sodomight** | Maîtrise Fulguromantique |
| **Arbre** | Fulguromance |
| **Niveau requis** | 30 |
| **Prérequis** | Orage (Thunder Storm) |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Bonus dégâts foudre | Pénétration résistance |
|--------|--------------------|-----------------------|
| 1 | +50% | -5% |
| 10 | +275% | -14% |
| 20 | +525% | -24% |

**Notes gameplay :** Booster passif essentiel pour tout build foudre. Le multiplicateur de dégâts est le plus élevé des trois Masteries (50% base vs 30% Fire), reflétant la variance plus haute de la foudre.

---

## 6. Paladin → Croisé-Solaire

### Description de classe
Le Croisé-Solaire est un guerrier-prêtre combinant des capacités de mêlée puissantes, des auras bénéficiant à tout le groupe, et des bénédictions divines. Il a le modificateur de blocage le plus élevé et les plus hauts gains de vie par niveau. Classe de choix pour le tanking et le support de groupe.

### Nommage Sodomight
- **Paladin** → **Croisé-Solaire** (serviteur du Soleil Eternel, gardien de la lumière)
- Les auras restent des auras ; les skills de combat restent nommés en anglais dans les termes techniques

---

### Arbre 1 : Auras Défensives (Defensive Auras)

#### 6.1.1 Prière (Prayer)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Prayer |
| **Nom Sodomight** | Prière |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2/s |
| **Type** | Aura (soin passif du groupe) |

**Formule :**
| Niveau | Soin/s (groupe) |
|--------|----------------|
| 1 | 4 |
| 10 | 22 |
| 20 | 42 |

**Synergies données :** Méditation +7% Heal/niveau.

---

#### 6.1.2 Résistance au Feu (Resist Fire)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Resist Fire |
| **Nom Sodomight** | Résistance au Feu |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif (aura permanente) |

**Formule :**
| Niveau | Fire Resist bonus (groupe) | Max Fire Resist bonus |
|--------|---------------------------|----------------------|
| 1 | +16% | +3% |
| 10 | +67% | +12% |
| 20 | +117% | +22% |

**Synergies données :** Salut +4% All Resist/niveau ; Vengeance +21% Fire Damage/niveau.

---

#### 6.1.3 Défiance (Defiance)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Defiance |
| **Nom Sodomight** | Défiance |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 6 |
| **Prérequis** | Prière |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Defense bonus (groupe) |
|--------|----------------------|
| 1 | +70% |
| 10 | +350% |
| 20 | +670% |

**Synergies données :** Bouclier Sacré (Holy Shield) +15% Defense/niveau.

---

#### 6.1.4 Résistance au Froid (Resist Cold)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Resist Cold |
| **Nom Sodomight** | Résistance au Froid |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 6 |
| **Prérequis** | Résistance au Feu |
| **Coût mana** | Passif |

**Formule :** Identique à Resist Fire (même scaling).

**Synergies données :** Salut +4%/niveau ; Vengeance +21% Cold Damage/niveau.

---

#### 6.1.5 Soin (Cleansing)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cleansing |
| **Nom Sodomight** | Purification |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 12 |
| **Prérequis** | Prière |
| **Coût mana** | 2/s |
| **Type** | Aura (réduit les durées de poison et de malédiction) |

**Formule :**
| Niveau | Réduction durée poison | Réduction durée malédictions |
|--------|----------------------|------------------------------|
| 1 | -35% | -17% |
| 10 | -62% | -35% |
| 20 | -86% | -52% |

---

#### 6.1.6 Résistance à la Foudre (Resist Lightning)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Resist Lightning |
| **Nom Sodomight** | Résistance à la Foudre |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 12 |
| **Prérequis** | Résistance au Froid |
| **Coût mana** | Passif |

**Formule :** Identique aux deux autres résistances.

**Synergies données :** Salut +4%/niveau ; Vengeance +21% Lightning Damage/niveau.

---

#### 6.1.7 Vigueur (Vigor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Vigor |
| **Nom Sodomight** | Vigueur |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 18 |
| **Prérequis** | Défiance |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Stamina bonus | Run Speed bonus | Regen Life bonus |
|--------|--------------|----------------|-----------------|
| 1 | +50% | +13% | +20% |
| 10 | +140% | +26% | +65% |
| 20 | +250% | +40% | +120% |

**Synergies données :** Charge +20% Damage/niveau ; Marteau Béni +14% Magic Damage/niveau.

---

#### 6.1.8 Méditation (Meditation)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Meditation |
| **Nom Sodomight** | Méditation |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 24 |
| **Prérequis** | Purification |
| **Coût mana** | 2/s |

**Formule :**
| Niveau | Regen mana/s (groupe) |
|--------|----------------------|
| 1 | +60% |
| 10 | +200% |
| 20 | +400% |

**Notes gameplay :** L'aura de Méditation (disponible via le runeword "Insight" sur un mercenaire) révolutionne la régénération de mana pour tout le groupe. Build "Insight Merc" est standard pour permettre aux casters de maintenir leur mana.

---

#### 6.1.9 Rédemption (Redemption)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Redemption |
| **Nom Sodomight** | Rédemption |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 30 |
| **Prérequis** | Vigueur |
| **Coût mana** | 2/s |
| **Effet spécial** | Convertit les cadavres proches en vie et mana |

**Formule :**
| Niveau | % vie/mana récupéré par cadavre |
|--------|--------------------------------|
| 1 | 20% |
| 10 | 29% |
| 20 | 38% |

---

#### 6.1.10 Salut (Salvation)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Salvation |
| **Nom Sodomight** | Salut Divin |
| **Arbre** | Auras Défensives |
| **Niveau requis** | 30 |
| **Prérequis** | Résistance à la Foudre |
| **Coût mana** | Passif |

**Formule :**
| Niveau | All Resist bonus (groupe) | Max All Resist bonus |
|--------|--------------------------|---------------------|
| 1 | +60% | +12% |
| 10 | +150% | +25% |
| 20 | +250% | +35% |

**Notes gameplay :** Excellent pour les builds de support pur — augmenter toutes les résistances du groupe simultanément.

---

### Arbre 2 : Auras Offensives (Offensive Auras)

#### 6.2.1 Puissance (Might)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Might |
| **Nom Sodomight** | Puissance Sacrée |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Damage bonus (groupe) |
|--------|----------------------|
| 1 | +40% |
| 10 | +220% |
| 20 | +420% |

---

#### 6.2.2 Feu Sacré (Holy Fire)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Holy Fire |
| **Nom Sodomight** | Flamme Sacrée |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 6 |
| **Prérequis** | Puissance Sacrée |
| **Coût mana** | 2/s |
| **Type** | Aura (dégâts de feu périodiques dans un rayon) |

**Formule :**
| Niveau | Feu min-max (périodique) | Rayon |
|--------|-------------------------|-------|
| 1 | 3-5 | 4.6m |
| 10 | 50-68 | 6.6m |
| 20 | 138-174 | 9.3m |

**Synergies reçues :** Résistance au Feu +21% Damage/niveau ; Vengeance +21% Damage/niveau.
**Synergies données :** Froid Sacré +7% Damage/niveau ; Choc Sacré +7% Damage/niveau.

---

#### 6.2.3 Visée Bénie (Blessed Aim)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blessed Aim |
| **Nom Sodomight** | Précision Bénie |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 12 |
| **Prérequis** | Puissance Sacrée |
| **Coût mana** | Passif |

**Formule :**
| Niveau | AR bonus (groupe) |
|--------|------------------|
| 1 | +50% |
| 10 | +275% |
| 20 | +525% |

**Synergies données :** Marteau Béni +14% Magic Damage/niveau.

---

#### 6.2.4 Concentration
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Concentration |
| **Nom Sodomight** | Concentration Divine |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 18 |
| **Prérequis** | Précision Bénie |
| **Coût mana** | 2/s |

**Formule :**
| Niveau | Damage bonus (groupe) | Interrupted attack protection |
|--------|----------------------|-------------------------------|
| 1 | +125% | Oui |
| 10 | +675% | Oui |
| 20 | +1275% | Oui |

**Synergies données :** Marteau Béni +14% Magic Damage/niveau.

---

#### 6.2.5 Froid Sacré (Holy Freeze)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Holy Freeze |
| **Nom Sodomight** | Gel Sacré |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 18 |
| **Prérequis** | Flamme Sacrée |
| **Coût mana** | 2/s |
| **Effet spécial** | Ralentit TOUS les ennemis dans le rayon en permanence |

**Formule :**
| Niveau | Froid min-max | Slow % | Rayon |
|--------|--------------|--------|-------|
| 1 | 4-6 | 30% | 5.3m |
| 10 | 50-70 | 60% | 7.3m |
| 20 | 132-172 | 90% | 9.3m |

**Synergies reçues :** Résistance au Froid +21%/niveau ; Flamme Sacrée +7%/niveau.
**Synergies données :** Choc Sacré +7%/niveau.

**Notes gameplay :** Aura de contrôle exceptionnelle. Ralentir tous les ennemis augmente massivement la survie du groupe.

---

#### 6.2.6 Choc Sacré (Holy Shock)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Holy Shock |
| **Nom Sodomight** | Choc Sacré |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 24 |
| **Prérequis** | Gel Sacré |
| **Coût mana** | 2/s |

**Formule :**
| Niveau | Foudre min-max | Rayon |
|--------|---------------|-------|
| 1 | 1-20 | 5.3m |
| 10 | 1-200 | 7.3m |
| 20 | 1-470 | 9.3m |

**Synergies reçues :** Résistance à la Foudre +21%/niveau ; Flamme Sacrée +7%/niveau ; Gel Sacré +7%/niveau.

---

#### 6.2.7 Fanatisme (Fanaticism)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fanaticism |
| **Nom Sodomight** | Fanatisme Sacré |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 30 |
| **Prérequis** | Précision Bénie |
| **Coût mana** | 2/s |

**Formule :**
| Niveau | Damage bonus | AR bonus | IAS bonus |
|--------|-------------|---------|----------|
| 1 | +50% | +50% | +14% |
| 10 | +215% | +215% | +32% |
| 20 | +390% | +390% | +50% |

**Notes gameplay :** **L'aura offensive la plus puissante du jeu.** Le build "Hammerdin" utilise Fanatisme ou Concentration selon la situation. Bénéficie à tout le groupe.

---

#### 6.2.8 Conviction
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Conviction |
| **Nom Sodomight** | Conviction Divine |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 30 |
| **Prérequis** | Choc Sacré |
| **Coût mana** | 2/s |
| **Effet spécial** | Réduit la défense ET toutes les résistances élémentales des ennemis |

**Formule :**
| Niveau | Defense réduite | All Resist réduite |
|--------|----------------|-------------------|
| 1 | -45% | -30% |
| 10 | -81% | -66% |
| 20 | -90% (cap) | -150% |

**Notes gameplay :** **L'aura la plus précieuse pour les groupes élémentaux.** Conviction peut briser les immunités élémentales. Build "Conviction Paladin" = support essentiel pour les groupes Arcaniste/Sarith foudre.

---

#### 6.2.9 Sanctification (Sanctuary)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Sanctuary |
| **Nom Sodomight** | Sanctuaire |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 24 |
| **Prérequis** | Concentration Divine |
| **Coût mana** | 2/s |
| **Effet spécial** | Dégâts magiques et knockback sur les morts-vivants uniquement |

**Formule :**
| Niveau | Dégâts magiques (mort-vivants) |
|--------|-------------------------------|
| 1 | 4-7 |
| 10 | 32-48 |
| 20 | 80-112 |

**Notes gameplay :** Très niche — efficace uniquement contre les morts-vivants. 1 point peut servir dans certaines zones.

---

#### 6.2.10 Feu Sacré Amélioré (Holy Nova — alias)
*Note : cet emplacement est occupé par une aura de niveau 24 selon la version. Dans LoD v1.14, l'arbre Offensif comprend : Might, Holy Fire, Blessed Aim, Concentration, Holy Freeze, Holy Shock, Fanaticism, Conviction, Sanctuary. Le 10e emplacement est **Thorns**.*

#### 6.2.10 Epines (Thorns)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Thorns |
| **Nom Sodomight** | Aura d'Epines |
| **Arbre** | Auras Offensives |
| **Niveau requis** | 6 |
| **Prérequis** | Puissance Sacrée |
| **Coût mana** | Passif |
| **Effet spécial** | Renvoie les dégâts physiques aux attaquants |

**Formule :**
| Niveau | % dégâts renvoyés |
|--------|-------------------|
| 1 | 200% |
| 10 | 650% |
| 20 | 1250% |

---

### Arbre 3 : Compétences de Combat (Combat Skills)

#### 6.3.1 Sacrifice
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Sacrifice |
| **Nom Sodomight** | Sacrifice Divin |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 0 (coût en vie) |
| **Coût vie** | 8% de la vie du Croisé-Solaire |
| **Type** | Physique (dégâts élevés, coût en vie) |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +180% | +50% |
| 10 | +1080% | +275% |
| 20 | +2080% | +525% |

**Synergies données :** Zèle +8% Damage/niveau.

---

#### 6.3.2 Frappe de Bouclier (Smite)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Smite |
| **Nom Sodomight** | Frappe Sacrée |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 2 |
| **Type** | Physique (toujours touche, stun) |
| **Durée stun** | 0.6s base |

**Formule :**
| Niveau | Damage (shield-based) | Stun durée |
|--------|----------------------|-----------|
| 1 | +30% shield dmg | 0.6s |
| 10 | +165% | 2.5s |
| 20 | +315% | 4.6s |

**Synergies reçues :** Bouclier Sacré +7% Damage/niveau.

**Notes gameplay :** Frappe garantie (toujours touche, ignore l'AR). Utilisé par "Smitenecro" hybrid et les builds tanking pour le stun.

---

#### 6.3.3 Eclair Sacré (Holy Bolt)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Holy Bolt |
| **Nom Sodomight** | Eclair de Lumière |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 6 |
| **Prérequis** | Sacrifice Divin |
| **Coût mana** | 2 |
| **Type** | Magique (dégâts sur morts-vivants) / Soin (alliés) |
| **Effet spécial** | Soigne les alliés touchés OU blesse les morts-vivants |

**Formule :**
| Niveau | Dégâts mort-vivants | Soin allié |
|--------|--------------------|-----------|
| 1 | 4-7 | 10-13 |
| 10 | 32-48 | 50-65 |
| 20 | 80-112 | 110-140 |

**Synergies données :** Poing des Cieux +18% Damage/niveau.

---

#### 6.3.4 Zèle (Zeal)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Zeal |
| **Nom Sodomight** | Zèle Sacré |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 12 |
| **Prérequis** | Frappe Sacrée |
| **Coût mana** | 2 |
| **Type** | Physique (5 frappes rapides sur cibles adjacentes) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Frappes |
|--------|-------------|---------|---------|
| 1 | +35% | +35% | 3 |
| 5 | +70% | +70% | 5 |
| 20 | +245% | +245% | 5 (cap) |

**Synergies reçues :** Sacrifice Divin +8% Damage/niveau.

**Notes gameplay :** Skill de mêlée AoE principal pour les Croisés-Solaires offensifs. Le nombre de frappes plafonne à 5. Synergique avec toutes les auras de dégâts.

---

#### 6.3.5 Charge
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Charge |
| **Nom Sodomight** | Charge Sacrée |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 12 |
| **Prérequis** | Zèle Sacré |
| **Coût mana** | 9 |
| **Type** | Physique (dash + frappe) |
| **Effet spécial** | Knockback, vitesse de course boostée pendant la charge |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +100% | +50% |
| 10 | +640% | +275% |
| 20 | +1240% | +525% |

**Synergies reçues :** Vigueur +20% Damage/niveau.

---

#### 6.3.6 Vengeance (Vengeance)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Vengeance |
| **Nom Sodomight** | Vengeance Divine |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 18 |
| **Prérequis** | Charge Sacrée |
| **Coût mana** | 4 |
| **Type** | Physique + Feu + Froid + Foudre (tous à la fois) |
| **Effet spécial** | Frappe avec les trois éléments simultanément |

**Formule :**
| Niveau | Elemental ajouté | AR bonus |
|--------|-----------------|---------|
| 1 | +120% | +30% |
| 10 | +660% | +165% |
| 20 | +1260% | +315% |

**Synergies reçues :** Resist Fire +21% Fire Damage/niveau ; Resist Cold +21% Cold/niveau ; Resist Lightning +21% Lightning/niveau.

**Notes gameplay :** Build "Vengeance Paladin" (alias "Avenger") — tri-élémental, idéal contre les ennemis immunisés à un seul élément.

---

#### 6.3.7 Marteau Béni (Blessed Hammer)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blessed Hammer |
| **Nom Sodomight** | Marteau Béni |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 18 |
| **Prérequis** | Eclair de Lumière |
| **Coût mana** | 5 |
| **Type** | Magique (spirale sortant du Croisé-Solaire) |
| **Effet spécial** | Dégâts de +150% contre les morts-vivants |

**Description :** Lance un marteau magique qui spirale vers l'extérieur en tournant. Il touche tout sur son passage. Dégâts magiques = non résistibles. Les morts-vivants reçoivent 150% de dégâts supplémentaires.

**Formule :**
| Niveau | Magique min | Magique max |
|--------|------------|------------|
| 1 | 20 | 30 |
| 10 | 150 | 185 |
| 20 | 330 | 395 |

**Synergies reçues :** Vigueur +14% Magic Damage/niveau ; Concentration +14%/niveau ; Précision Bénie +14%/niveau.

**Notes gameplay :** **Le skill le plus puissant du jeu en solo pour le farming.** Build "Hammerdin" = le build le plus populaire et efficace de D2. Dégâts magiques = aucune immunité à craindre (sauf Physical Immune). Avec toutes synergies à 20 = dégâts astronomiques.

---

#### 6.3.8 Conversion
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Conversion |
| **Nom Sodomight** | Conversion Divine |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 24 |
| **Prérequis** | Vengeance Divine |
| **Coût mana** | 4 |
| **Type** | Conversion (ennemi devient allié temporaire) |
| **Durée** | 26s base |

**Formule :**
| Niveau | Chance de conversion |
|--------|---------------------|
| 1 | 26% |
| 5 | 46% |
| 10 | 61% |
| 20 | 86% |

**Notes gameplay :** Utile pour retourner des monstres puissants. Les monstres convertis comptent dans l'équipe pendant la durée.

---

#### 6.3.9 Bouclier Sacré (Holy Shield)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Holy Shield |
| **Nom Sodomight** | Bouclier Divin |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 24 |
| **Prérequis** | Frappe Sacrée (Smite) |
| **Coût mana** | 35 |
| **Type** | Buff défensif majeur |
| **Durée** | 180s base |

**Formule :**
| Niveau | Defense bonus | Block bonus | Smite damage |
|--------|--------------|------------|-------------|
| 1 | +25% | +14% | +0% |
| 10 | +160% | +59% | +100% |
| 20 | +310% | +104% | +200% |

**Synergies reçues :** Défiance +15% Defense/niveau.

**Notes gameplay :** **Skill essentiel pour tous les builds Croisé-Solaire de mêlée.** Avec Holy Shield, atteindre le cap de 75% de blocage est trivial. Améliore aussi massivement la défense et les dégâts de Smite.

---

#### 6.3.10 Poing des Cieux (Fist of the Heavens)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fist of the Heavens |
| **Nom Sodomight** | Poing des Cieux |
| **Arbre** | Compétences de Combat |
| **Niveau requis** | 30 |
| **Prérequis** | Marteau Béni |
| **Coût mana** | 25 |
| **Type** | Foudre + Eclairs de Lumière AoE |
| **Effet spécial** | Impact de foudre + libère des Eclairs de Lumière (Holy Bolts) en AoE |

**Formule :**
| Niveau | Foudre min-max | Holy Bolts émis |
|--------|---------------|----------------|
| 1 | 1-150 | 4 |
| 10 | 1-810 | 12 |
| 20 | 1-1590 | 22 |

**Synergies reçues :** Eclair de Lumière +18% Damage/niveau.

**Notes gameplay :** Build "FoH Paladin" — les Holy Bolts soignent les alliés et blessent les morts-vivants. Dévastateur contre les morts-vivants. Viabilité en Hell dépend des immunités.

---

## 7. Druid → Animiste

### Description de classe
L'Animiste est un homme des bois maîtrisant les forces de la nature. Il peut se transformer en loup-garou ou en werebear pour le combat au corps à corps, invoquer des esprits de la nature et des créatures sauvages, et lancer des sorts élémentaux (vent, feu, glace). Classe très polyvalente mais difficile à optimiser.

### Nommage Sodomight
- **Druid** → **Animiste** (maître des animaux et des éléments naturels)
- Werewolf → **Métamorphe Loup** ; Werebear → **Métamorphe Ours**

---

### Arbre 1 : Elémentaire (Elemental)

#### 7.1.1 Tempête de Feu (Firestorm)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Firestorm |
| **Nom Sodomight** | Tempête de Feu |
| **Arbre** | Elémentaire |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 4 |
| **Type** | Feu (jets de feu jaillissant du sol) |

**Formule :**
| Niveau | Feu/s min | Feu/s max |
|--------|----------|----------|
| 1 | 3 | 7 |
| 10 | 30 | 60 |
| 20 | 75 | 140 |

**Synergies données :** Fissure +11%/niveau ; Volcan +11%/niveau.

---

#### 7.1.2 Boulder Enflammé (Molten Boulder)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Molten Boulder |
| **Nom Sodomight** | Boulder Enflammé |
| **Arbre** | Elémentaire |
| **Niveau requis** | 6 |
| **Prérequis** | Tempête de Feu |
| **Coût mana** | 10 |
| **Type** | Physique + Feu (boulder roulant qui traverseles ennemis) |

**Formule :**
| Niveau | Physique min-max | Feu |
|--------|-----------------|-----|
| 1 | 6-12 | 3-5 |
| 10 | 55-95 | 28-44 |
| 20 | 130-210 | 70-104 |

**Synergies données :** Volcan +9%/niveau.

---

#### 7.1.3 Souffle Arctique (Arctic Blast)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Arctic Blast |
| **Nom Sodomight** | Souffle Arctique |
| **Arbre** | Elémentaire |
| **Niveau requis** | 6 |
| **Prérequis** | Tempête de Feu |
| **Coût mana** | 4/s (continu) |
| **Type** | Froid (jet continu de glace) |

**Formule :**
| Niveau | Froid/s min | Froid/s max |
|--------|------------|------------|
| 1 | 8 | 16 |
| 10 | 60 | 110 |
| 20 | 145 | 255 |

**Synergies données :** Cyclone +13% Cold Damage/niveau ; Tornade +13%/niveau.

---

#### 7.1.4 Fissure (Fissure)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fissure |
| **Nom Sodomight** | Fissure Volcanique |
| **Arbre** | Elémentaire |
| **Niveau requis** | 12 |
| **Prérequis** | Boulder Enflammé |
| **Coût mana** | 16 |
| **Type** | Feu (fissures dans le sol qui brûlent) |

**Formule :**
| Niveau | Feu min | Feu max |
|--------|---------|---------|
| 1 | 16 | 26 |
| 10 | 90 | 128 |
| 20 | 210 | 285 |

**Synergies reçues :** Tempête de Feu +11%/niveau.

---

#### 7.1.5 Armure Cyclonique (Cyclone Armor)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cyclone Armor |
| **Nom Sodomight** | Armure de Vent |
| **Arbre** | Elémentaire |
| **Niveau requis** | 12 |
| **Prérequis** | Souffle Arctique |
| **Coût mana** | 6 |
| **Type** | Défensif (absorbe les dégâts élémentaux) |
| **Durée** | 200s |

**Formule :**
| Niveau | Absorption élémentale |
|--------|----------------------|
| 1 | 40 |
| 10 | 250 |
| 20 | 560 |

**Synergies reçues :** Tornade +10%/niveau ; Ouragan +10%/niveau.

---

#### 7.1.6 Tourbillon Venteux (Twister)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Twister |
| **Nom Sodomight** | Tourbillon |
| **Arbre** | Elémentaire |
| **Niveau requis** | 18 |
| **Prérequis** | Armure de Vent |
| **Coût mana** | 7 |
| **Type** | Physique + stun (mini tornades) |

**Formule :**
| Niveau | Physique min-max | Stun durée |
|--------|-----------------|-----------|
| 1 | 6-8 | 0.4s |
| 10 | 42-56 | 0.8s |
| 20 | 100-128 | 1.4s |

**Synergies données :** Tornade +20% Damage/niveau.

---

#### 7.1.7 Volcan (Volcano)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Volcano |
| **Nom Sodomight** | Volcan |
| **Arbre** | Elémentaire |
| **Niveau requis** | 24 |
| **Prérequis** | Fissure Volcanique |
| **Coût mana** | 26 |
| **Type** | Physique + Feu (eruption volcanique) |

**Formule :**
| Niveau | Physique min-max | Feu min-max |
|--------|-----------------|------------|
| 1 | 8-10 | 20-30 |
| 10 | 60-82 | 150-210 |
| 20 | 150-195 | 380-520 |

**Synergies reçues :** Tempête de Feu +11%/niveau ; Boulder Enflammé +9%/niveau.

---

#### 7.1.8 Tornade (Tornado)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Tornado |
| **Nom Sodomight** | Tornade |
| **Arbre** | Elémentaire |
| **Niveau requis** | 24 |
| **Prérequis** | Tourbillon |
| **Coût mana** | 10 |
| **Type** | Physique (trajectoire erratique) |

**Formule :**
| Niveau | Physique min | Physique max |
|--------|------------|-------------|
| 1 | 26 | 36 |
| 10 | 155 | 200 |
| 20 | 365 | 465 |

**Synergies reçues :** Tourbillon +20%/niveau.
**Synergies données :** Ouragan +12%/niveau ; Armure de Vent +10%/niveau.

---

#### 7.1.9 Ouragan (Hurricane)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Hurricane |
| **Nom Sodomight** | Ouragan |
| **Arbre** | Elémentaire |
| **Niveau requis** | 30 |
| **Prérequis** | Tornade |
| **Coût mana** | 30 |
| **Type** | Froid AoE (tempête permanente autour de l'Animiste) |
| **Durée** | 40s base |

**Formule :**
| Niveau | Froid min-max | Rayon |
|--------|--------------|-------|
| 1 | 25-50 | 8m |
| 10 | 175-310 | 9.3m |
| 20 | 425-740 | 10.6m |

**Synergies reçues :** Tornade +12%/niveau ; Armure de Vent +10%/niveau.

**Notes gameplay :** Excellent pour les builds élémentaires en forme de loup. L'ouragan tourne autour de l'Animiste en permanence, ralentissant et blessant tout ce qui s'approche.

---

#### 7.1.10 Armageddon
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Armageddon |
| **Nom Sodomight** | Armageddon |
| **Arbre** | Elémentaire |
| **Niveau requis** | 30 |
| **Prérequis** | Volcan |
| **Coût mana** | 36 |
| **Type** | Feu (pluie de météores en cercle) |
| **Durée** | 40s base |

**Formule :**
| Niveau | Feu min-max par météore | Météores/s |
|--------|------------------------|-----------|
| 1 | 10-18 | ~2 |
| 10 | 78-130 | ~2 |
| 20 | 193-315 | ~2 |

---

### Arbre 2 : Métamorphose (Shape Shifting)

#### 7.2.1 Métamorphe Loup (Werewolf)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Werewolf |
| **Nom Sodomight** | Métamorphe Loup |
| **Arbre** | Métamorphose |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 10 |
| **Type** | Transformation (forme loup-garou) |
| **Effet** | Boost AR, vitesse attaque, life ; accès aux skills de loup |

**Formule :**
| Niveau | AR bonus | Attack speed | Life bonus |
|--------|---------|-------------|-----------|
| 1 | +60% | +30% | +25% |
| 10 | +150% | +60% | +50% |
| 20 | +250% | +90% | +75% |

**Synergies données :** Lycanthropie +11% Duration/niveau.

---

#### 7.2.2 Lycanthropie (Lycanthropy)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lycanthropy |
| **Nom Sodomight** | Lycanthropie |
| **Arbre** | Métamorphose |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Life bonus (formes) | Durée transformation |
|--------|--------------------|-----------------------|
| 1 | +20% | +40s |
| 10 | +110% | +220s |
| 20 | +210% | +420s |

**Notes gameplay :** Maxer pour maintenir les transformations indéfiniment et booster la vie en forme animale.

---

#### 7.2.3 Métamorphe Ours (Werebear)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Werebear |
| **Nom Sodomight** | Métamorphe Ours |
| **Arbre** | Métamorphose |
| **Niveau requis** | 6 |
| **Prérequis** | Métamorphe Loup |
| **Coût mana** | 10 |
| **Type** | Transformation (forme ours) |
| **Effet** | Boost damage, defense, life ; accès aux skills d'ours |

**Formule :**
| Niveau | Damage bonus | Defense bonus | Life bonus |
|--------|-------------|--------------|-----------|
| 1 | +66% | +26% | +50% |
| 10 | +216% | +134% | +140% |
| 20 | +416% | +264% | +250% |

---

#### 7.2.4 Broyage (Maul)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Maul |
| **Nom Sodomight** | Broyage |
| **Arbre** | Métamorphose |
| **Niveau requis** | 12 |
| **Prérequis** | Métamorphe Ours |
| **Coût mana** | 6 |
| **Type** | Physique + stacks de dégâts (Werebear uniquement) |
| **Effet spécial** | Chaque usage stack un bonus de dégâts |

**Formule :**
| Niveau | Damage bonus | Stun durée |
|--------|-------------|-----------|
| 1 | +125% | 1s |
| 10 | +575% | 2s |
| 20 | +1075% | 3.3s |

---

#### 7.2.5 Rage Sauvage (Feral Rage)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Feral Rage |
| **Nom Sodomight** | Rage Férale |
| **Arbre** | Métamorphose |
| **Niveau requis** | 12 |
| **Prérequis** | Broyage |
| **Coût mana** | 4 |
| **Type** | Physique + life steal (Werebear) |

**Formule :**
| Niveau | Damage bonus | Life steal | Move speed |
|--------|-------------|-----------|-----------|
| 1 | +60% | 4% | +8% |
| 10 | +330% | 12% | +24% |
| 20 | +630% | 20% | +40% |

---

#### 7.2.6 Griffes de Feu (Fire Claws)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Claws |
| **Nom Sodomight** | Griffes Enflammées |
| **Arbre** | Métamorphose |
| **Niveau requis** | 18 |
| **Prérequis** | Métamorphe Loup OU Broyage |
| **Coût mana** | 5 |
| **Type** | Physique + Feu (les deux formes) |

**Formule :**
| Niveau | Feu ajouté min-max |
|--------|-------------------|
| 1 | 16-20 |
| 10 | 100-130 |
| 20 | 240-310 |

**Synergies reçues :** Tempête de Feu +14%/niveau ; Fissure +14%/niveau ; Volcan +14%/niveau.

---

#### 7.2.7 Rage (Rabies)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Rabies |
| **Nom Sodomight** | Rage Vénéneuse |
| **Arbre** | Métamorphose |
| **Niveau requis** | 18 |
| **Prérequis** | Rage Férale |
| **Coût mana** | 5 |
| **Type** | Poison (se propage de cible en cible) |
| **Effet spécial** | Les ennemis touchés infectent ceux qu'ils touchent |

**Formule :**
| Niveau | Poison/s | Durée | Propagation |
|--------|---------|-------|------------|
| 1 | 40 | 4s | Oui |
| 10 | 230 | 8s | Oui |
| 20 | 540 | 13s | Oui |

---

#### 7.2.8 Onde de Choc (Shock Wave)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shock Wave |
| **Nom Sodomight** | Onde de Choc |
| **Arbre** | Métamorphose |
| **Niveau requis** | 24 |
| **Prérequis** | Broyage |
| **Coût mana** | 5 |
| **Type** | Physique + stun AoE (Werebear) |
| **Stun durée** | 1.6s base |

**Formule :**
| Niveau | Physique min-max | Stun durée | Rayon |
|--------|-----------------|-----------|-------|
| 1 | 10-20 | 1.6s | 4m |
| 10 | 80-140 | 3.3s | 5.3m |
| 20 | 195-325 | 5.6s | 6.6m |

---

#### 7.2.9 Faim (Hunger)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Hunger |
| **Nom Sodomight** | Faim Sauvage |
| **Arbre** | Métamorphose |
| **Niveau requis** | 24 |
| **Prérequis** | Griffes Enflammées |
| **Coût mana** | 0 (vol de vie et mana) |
| **Effet** | Vole massivement vie et mana en frappant |

**Formule :**
| Niveau | Life steal | Mana steal |
|--------|-----------|-----------|
| 1 | 72% | 12% |
| 10 | 90% | 20% |
| 20 | 114% | 30% |

---

#### 7.2.10 Fureur (Fury)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fury |
| **Nom Sodomight** | Fureur Animale |
| **Arbre** | Métamorphose |
| **Niveau requis** | 30 |
| **Prérequis** | Rage Vénéneuse |
| **Coût mana** | 3 |
| **Type** | Physique (multifrappes — Werewolf) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Frappes |
|--------|-------------|---------|---------|
| 1 | +210% | +30% | 2 |
| 5 | +330% | +70% | 3 |
| 10 | +510% | +130% | 4 |
| 20 | +910% | +250% | 5 |

**Notes gameplay :** Build "Fury Druid" (Werewolf) — l'un des builds de mêlée les plus puissants. Combine Fury avec Fanaticism d'un allié Croisé-Solaire = dévastateur.

---

### Arbre 3 : Invocations Naturelles (Summoning)

#### 7.3.1 Corbeau (Raven)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Raven |
| **Nom Sodomight** | Corbeau |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 6 |
| **Type** | Invocation (corbeau attaquant) |
| **Effet spécial** | Les corbeaux aveuglent les ennemis |

**Formule :**
| Niveau | Corbeaux max | Damage |
|--------|------------|--------|
| 1 | 1 | 1-5 |
| 5 | 4 | 4-14 |
| 10 | 6 | 9-25 |
| 20 | 12 | 19-45 |

---

#### 7.3.2 Liane Vénéneuse (Poison Creeper)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Poison Creeper |
| **Nom Sodomight** | Liane Vénéneuse |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 5 |
| **Type** | Invocation (vigne se déplaçant et empoisonnant) |

**Formule :**
| Niveau | Poison/s | Durée poison |
|--------|---------|-------------|
| 1 | 4-6 | 4s |
| 10 | 40-58 | 7s |
| 20 | 100-138 | 10s |

---

#### 7.3.3 Sage du Chêne (Oak Sage)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Oak Sage |
| **Nom Sodomight** | Esprit du Chêne |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 6 |
| **Prérequis** | Liane Vénéneuse |
| **Coût mana** | 7 |
| **Type** | Esprit de soutien (boost life) |

**Formule :**
| Niveau | Life bonus (groupe) |
|--------|-------------------|
| 1 | +30% |
| 10 | +100% |
| 20 | +200% |

**Notes gameplay :** **Skill essentiel pour la survie du groupe.** L'Esprit du Chêne quasi-double la vie de tout le groupe. 1 point suffit pour un bonus significatif.

---

#### 7.3.4 Loup Esprit (Summon Spirit Wolf)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Summon Spirit Wolf |
| **Nom Sodomight** | Loup Spectral |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 6 |
| **Prérequis** | Corbeau |
| **Coût mana** | 10 |
| **Type** | Invocation (loup spectral téléportant) |
| **Effet spécial** | Les loups peuvent se téléporter vers leurs cibles |

**Formule :**
| Niveau | Vie | Damage | Max loups |
|--------|-----|--------|----------|
| 1 | 80 | 4-9 | 1 |
| 10 | 440 | 35-70 | 3 |
| 20 | 1040 | 95-175 | 5 |

---

#### 7.3.5 Liane de Charogne (Carrion Vine)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Carrion Vine |
| **Nom Sodomight** | Liane de Charogne |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 12 |
| **Prérequis** | Liane Vénéneuse |
| **Coût mana** | 8 |
| **Type** | Invocation (vigne qui dévore les cadavres pour soigner) |
| **Effet** | Consomme les cadavres et régénère la vie de l'Animiste |

**Formule :**
| Niveau | % vie max régénérée/cadavre |
|--------|---------------------------|
| 1 | 25% |
| 10 | 34% |
| 20 | 44% |

---

#### 7.3.6 Coeur du Loir (Heart of Wolverine)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Heart of Wolverine |
| **Nom Sodomight** | Esprit du Loir |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 18 |
| **Prérequis** | Esprit du Chêne |
| **Coût mana** | 7 |
| **Type** | Esprit offensif (damage + AR) |

**Formule :**
| Niveau | Damage bonus (groupe) | AR bonus (groupe) |
|--------|----------------------|------------------|
| 1 | +20% | +26% |
| 10 | +75% | +116% |
| 20 | +155% | +236% |

---

#### 7.3.7 Loup Sauvage (Summon Dire Wolf)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Summon Dire Wolf |
| **Nom Sodomight** | Loup des Profondeurs |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 18 |
| **Prérequis** | Loup Spectral |
| **Coût mana** | 15 |
| **Effet spécial** | Les loups dévorent les cadavres pour se soigner |

**Formule :**
| Niveau | Vie | Damage | Max loups |
|--------|-----|--------|----------|
| 1 | 200 | 12-25 | 2 |
| 10 | 900 | 78-140 | 3 |
| 20 | 2000 | 198-330 | 5 |

---

#### 7.3.8 Liane Solaire (Solar Creeper)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Solar Creeper |
| **Nom Sodomight** | Liane Solaire |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 24 |
| **Prérequis** | Liane de Charogne |
| **Coût mana** | 12 |
| **Effet** | Consomme les cadavres et régénère le mana de l'Animiste |

**Formule :**
| Niveau | % mana max régénéré/cadavre |
|--------|---------------------------|
| 1 | 2% |
| 10 | 5% |
| 20 | 9% |

---

#### 7.3.9 Esprit d'Epines (Spirit of Barbs)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Spirit of Barbs |
| **Nom Sodomight** | Esprit des Epines |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 30 |
| **Prérequis** | Esprit du Loir |
| **Coût mana** | 7 |
| **Type** | Esprit défensif (réflexion de dégâts) |

**Formule :**
| Niveau | % dégâts physiques renvoyés |
|--------|---------------------------|
| 1 | 50% |
| 10 | 140% |
| 20 | 280% |

---

#### 7.3.10 Grizzli Invoqué (Summon Grizzly)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Summon Grizzly |
| **Nom Sodomight** | Invocation du Grizzli |
| **Arbre** | Invocations Naturelles |
| **Niveau requis** | 30 |
| **Prérequis** | Loup des Profondeurs |
| **Coût mana** | 25 |
| **Type** | Invocation (grizzli puissant) |

**Formule :**
| Niveau | Vie | Damage min-max |
|--------|-----|---------------|
| 1 | 650 | 37-76 |
| 10 | 2450 | 157-286 |
| 20 | 5050 | 377-636 |

**Notes gameplay :** Le grizzli est l'un des tanks les plus résistants du jeu. Avec l'Esprit du Chêne, il peut absorber d'énormes quantités de dégâts.

---

## 8. Assassin → Ombrelame

### Description de classe
L'Ombrelame est une guerrière agile spécialisée dans les arts martiaux (charge-ups + finishing moves), les pièges automatiques à distance (sentinelles de foudre, feu), et les disciplines de l'ombre (buffs, invocations de clone, poison). Classe technique avec une courbe d'apprentissage élevée.

### Nommage Sodomight
- **Assassin** → **Ombrelame** (chasseuse de l'ombre, maîtresse des lames et des pièges)

---

### Arbre 1 : Arts Martiaux (Martial Arts)

#### 8.1.1 Frappe du Tigre (Tiger Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Tiger Strike |
| **Nom Sodomight** | Frappe du Tigre |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 1 |
| **Type** | Charge-Up Skill (physique) |
| **Effet** | Accumule 1-3 charges qui boostent les finishing moves |

**Description :** Les charge-up skills s'accumulent jusqu'à 3 charges. Les finishing moves libèrent toutes les charges accumulées.

**Formule (dégâts bonus par charge) :**
| Niveau | Dégâts bonus par charge | AR bonus |
|--------|------------------------|---------|
| 1 | +150% | +20% |
| 10 | +860% | +110% |
| 20 | +1660% | +210% |

**Synergies données :** Coup du Cobra +5%/niveau.

---

#### 8.1.2 Dragon Talon
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dragon Talon |
| **Nom Sodomight** | Serre du Dragon |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 6 |
| **Type** | Finishing Move (coups de pied) |

**Formule :**
| Niveau | Damage bonus | AR bonus | Kicks |
|--------|-------------|---------|-------|
| 1 | +75% | +20% | 1 |
| 5 | +155% | +60% | 2 |
| 12 | +295% | +130% | 3 |
| 20 | +455% | +210% | 4 |

**Notes gameplay :** Skill d'ouverture efficace. Dragon Talon + Tiger Strike charges = burst de dégâts massif.

---

#### 8.1.3 Poings de Feu (Fists of Fire)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fists of Fire |
| **Nom Sodomight** | Poings de Feu |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 6 |
| **Prérequis** | Frappe du Tigre |
| **Coût mana** | 2 |
| **Type** | Charge-Up (feu, griffes uniquement) |

**Formule (par charge) :**
| Niveau | Feu bonus par charge |
|--------|---------------------|
| 1 | 1-4 feu |
| 10 | 15-40 feu |
| 20 | 40-100 feu |

**Synergies données :** Frappe du Phénix +10% Damage/niveau.

---

#### 8.1.4 Griffe du Dragon (Dragon Claw)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dragon Claw |
| **Nom Sodomight** | Griffe du Dragon |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 6 |
| **Prérequis** | Serre du Dragon |
| **Coût mana** | 2 |
| **Type** | Finishing Move (double griffe) |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +80% | +20% |
| 10 | +440% | +110% |
| 20 | +840% | +210% |

---

#### 8.1.5 Coup du Cobra (Cobra Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cobra Strike |
| **Nom Sodomight** | Coup du Cobra |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 12 |
| **Prérequis** | Frappe du Tigre |
| **Coût mana** | 2 |
| **Type** | Charge-Up (vol de vie et mana) |

**Formule (par charge) :**
| Niveau | Life steal | Mana steal |
|--------|-----------|-----------|
| 1 | 15% | 12% |
| 10 | 24% | 20% |
| 20 | 35% | 28% |

---

#### 8.1.6 Griffes du Tonnerre (Claws of Thunder)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Claws of Thunder |
| **Nom Sodomight** | Griffes du Tonnerre |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 18 |
| **Prérequis** | Poings de Feu |
| **Coût mana** | 4 |
| **Type** | Charge-Up (foudre, griffes uniquement) |

**Formule :**
| Niveau | Foudre bonus par charge (min-max) |
|--------|----------------------------------|
| 1 | 1-20 |
| 10 | 1-130 |
| 20 | 1-290 |

**Synergies données :** Frappe du Phénix +13% Damage/niveau.

---

#### 8.1.7 Queue du Dragon (Dragon Tail)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dragon Tail |
| **Nom Sodomight** | Queue du Dragon |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 18 |
| **Prérequis** | Serre du Dragon |
| **Coût mana** | 10 |
| **Type** | Finishing Move (coup de pied + explosion de feu) |

**Formule :**
| Niveau | Feu AoE min-max | AR bonus |
|--------|----------------|---------|
| 1 | 40-65 | +40% |
| 10 | 280-420 | +220% |
| 20 | 660-980 | +420% |

---

#### 8.1.8 Lames de Glace (Blades of Ice)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blades of Ice |
| **Nom Sodomight** | Lames de Glace |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 24 |
| **Prérequis** | Griffes du Tonnerre |
| **Coût mana** | 3 |
| **Type** | Charge-Up (froid, griffes uniquement) |

**Formule :**
| Niveau | Froid bonus par charge (min-max) |
|--------|----------------------------------|
| 1 | 8-18 |
| 10 | 60-110 |
| 20 | 140-250 |

**Synergies données :** Frappe du Phénix +10% Damage/niveau.

---

#### 8.1.9 Vol du Dragon (Dragon Flight)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Dragon Flight |
| **Nom Sodomight** | Vol du Dragon |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 24 |
| **Prérequis** | Queue du Dragon |
| **Coût mana** | 15 |
| **Type** | Finishing Move (téléportation + coup de pied) |
| **Effet** | Téléporte vers un ennemi distant et le frappe |

**Formule :**
| Niveau | Damage bonus | AR bonus |
|--------|-------------|---------|
| 1 | +150% | +40% |
| 10 | +850% | +220% |
| 20 | +1650% | +420% |

**Notes gameplay :** Excellent skill d'initiation. Téléporter sur un ennemi et libérer toutes les charges = burst énorme.

---

#### 8.1.10 Frappe du Phénix (Phoenix Strike)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Phoenix Strike |
| **Nom Sodomight** | Frappe du Phénix |
| **Arbre** | Arts Martiaux |
| **Niveau requis** | 30 |
| **Prérequis** | Lames de Glace |
| **Coût mana** | 4 |
| **Type** | Charge-Up tri-élémental (libère nova selon le nombre de charges) |
| **Effet spécial** | 1 charge = Meteor ; 2 charges = Chain Lightning ; 3 charges = nova de glace |

**Formule :**
| Niveau | Feu (meteor) | Foudre (chain) | Froid (nova) |
|--------|-------------|---------------|-------------|
| 1 | 50-100 | 1-80 | 40-75 |
| 10 | 330-610 | 1-480 | 265-470 |
| 20 | 790-1430 | 1-1080 | 640-1100 |

**Synergies reçues :** Poings de Feu +10%/niveau ; Griffes du Tonnerre +13%/niveau ; Lames de Glace +10%/niveau.

**Notes gameplay :** Build "Phoenix Strike Assassin" — tri-élémental, contourne les immunités. Complexe à utiliser mais très efficace.

---

### Arbre 2 : Pièges (Traps)

#### 8.2.1 Bombe de Feu (Fire Blast)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fire Blast |
| **Nom Sodomight** | Bombe de Feu |
| **Arbre** | Pièges |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 3 |
| **Type** | Feu lancé (pas un piège) |

**Formule :**
| Niveau | Feu min | Feu max |
|--------|---------|---------|
| 1 | 6 | 10 |
| 10 | 44 | 68 |
| 20 | 104 | 156 |

**Synergies données :** Sillage de Feu +14%/niveau ; Sillage d'Enfer +14%/niveau.

---

#### 8.2.2 Toile de Foudre (Shock Web)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shock Web |
| **Nom Sodomight** | Toile de Foudre |
| **Arbre** | Pièges |
| **Niveau requis** | 6 |
| **Prérequis** | Bombe de Feu |
| **Coût mana** | 6 |
| **Type** | Foudre (piège qui tire des étoiles de foudre) |

**Formule :**
| Niveau | Foudre min-max | Max pièges |
|--------|---------------|-----------|
| 1 | 1-35 | 3 |
| 10 | 1-200 | 5 |
| 20 | 1-420 | 5 |

**Synergies données :** Sentinelle à Eclairs Chargés +14%/niveau.

---

#### 8.2.3 Lame Sentinelle (Blade Sentinel)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blade Sentinel |
| **Nom Sodomight** | Sentinelle Lame |
| **Arbre** | Pièges |
| **Niveau requis** | 6 |
| **Prérequis** | Bombe de Feu |
| **Coût mana** | 7 |
| **Type** | Physique (lame tournoyante patrouillant une zone) |

**Formule :**
| Niveau | Damage min-max |
|--------|---------------|
| 1 | 12-24 |
| 10 | 82-148 |
| 20 | 196-342 |

**Synergies données :** Bouclier de Lames +6%/niveau.

---

#### 8.2.4 Sentinelle à Eclairs Chargés (Charged Bolt Sentry)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Charged Bolt Sentry |
| **Nom Sodomight** | Sentinelle Electrique |
| **Arbre** | Pièges |
| **Niveau requis** | 12 |
| **Prérequis** | Toile de Foudre |
| **Coût mana** | 13 |
| **Type** | Foudre (piège tirant des Charged Bolts) |

**Formule :**
| Niveau | Foudre par bolt | Bolts émis | Max pièges |
|--------|----------------|-----------|-----------|
| 1 | 1-12 | 6 | 3 |
| 10 | 1-68 | 9 | 5 |
| 20 | 1-148 | 13 | 5 |

**Synergies reçues :** Toile de Foudre +14%/niveau.

---

#### 8.2.5 Sillage de Feu (Wake of Fire)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Wake of Fire |
| **Nom Sodomight** | Sillage de Feu |
| **Arbre** | Pièges |
| **Niveau requis** | 12 |
| **Prérequis** | Sentinelle Lame |
| **Coût mana** | 13 |
| **Type** | Feu (piège qui émet des vagues de feu) |

**Formule :**
| Niveau | Feu min-max | Max pièges |
|--------|------------|-----------|
| 1 | 6-10 | 3 |
| 10 | 42-66 | 5 |
| 20 | 100-154 | 5 |

**Synergies reçues :** Bombe de Feu +14%/niveau.
**Synergies données :** Sillage d'Enfer +12%/niveau.

---

#### 8.2.6 Fureur de Lames (Blade Fury)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blade Fury |
| **Nom Sodomight** | Fureur de Lames |
| **Arbre** | Pièges |
| **Niveau requis** | 18 |
| **Prérequis** | Sentinelle Lame |
| **Coût mana** | 1-3 |
| **Type** | Physique (lames tournoyantes lancées) |

**Formule :**
| Niveau | Damage min-max | AR bonus |
|--------|---------------|---------|
| 1 | 60-100% weapon | +50% |
| 10 | 60-100% weapon | +275% |
| 20 | 60-100% weapon | +525% |

---

#### 8.2.7 Sentinelle de Foudre (Lightning Sentry)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Lightning Sentry |
| **Nom Sodomight** | Sentinelle de Foudre |
| **Arbre** | Pièges |
| **Niveau requis** | 24 |
| **Prérequis** | Sentinelle Electrique |
| **Coût mana** | 20 |
| **Type** | Foudre (piège tirant des éclairs puissants) |

**Formule :**
| Niveau | Foudre min | Foudre max | Max pièges |
|--------|-----------|-----------|-----------|
| 1 | 1 | 100 | 3 |
| 10 | 1 | 600 | 5 |
| 20 | 1 | 1300 | 5 |

**Synergies reçues :** Sillage de Feu +12%/niveau ; Sentinelle de Mort +20%/niveau.

**Notes gameplay :** **Le skill de piège le plus puissant du jeu.** Build "Trapsin" = spam de Sentinelles de Foudre + Sentinelles de Mort. DPS exceptionnel, très safe (l'Ombrelame reste à distance).

---

#### 8.2.8 Sillage d'Enfer (Wake of Inferno)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Wake of Inferno |
| **Nom Sodomight** | Sillage d'Enfer |
| **Arbre** | Pièges |
| **Niveau requis** | 24 |
| **Prérequis** | Sillage de Feu |
| **Coût mana** | 20 |
| **Type** | Feu (piège à jet continu de feu) |

**Formule :**
| Niveau | Feu/s min-max | Max pièges |
|--------|--------------|-----------|
| 1 | 12-18 | 3 |
| 10 | 82-116 | 5 |
| 20 | 192-268 | 5 |

**Synergies reçues :** Sillage de Feu +12%/niveau ; Bombe de Feu +14%/niveau.

---

#### 8.2.9 Sentinelle de Mort (Death Sentry)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Death Sentry |
| **Nom Sodomight** | Sentinelle de Mort |
| **Arbre** | Pièges |
| **Niveau requis** | 30 |
| **Prérequis** | Sentinelle de Foudre |
| **Coût mana** | 20 |
| **Type** | Foudre + Corpse Explosion (hybride) |
| **Effet spécial** | Tire des éclairs ET fait exploser les cadavres proches |

**Formule :**
| Niveau | Foudre min-max | Corpse Explosion % |
|--------|---------------|-------------------|
| 1 | 1-80 | 40-60% vie max |
| 10 | 1-460 | 58-78% |
| 20 | 1-960 | 78-98% |

**Synergies reçues :** Sentinelle de Foudre +20%/niveau.

**Notes gameplay :** **Build "Deathsin"** — combinaison Sentinelle de Foudre + Sentinelle de Mort = DPS + Corpse Explosion en masse. Extrêmement puissant pour le clearing de zones.

---

#### 8.2.10 Bouclier de Lames (Blade Shield)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Blade Shield |
| **Nom Sodomight** | Bouclier de Lames |
| **Arbre** | Pièges |
| **Niveau requis** | 30 |
| **Prérequis** | Fureur de Lames |
| **Coût mana** | 27 |
| **Type** | Physique (lames tournant autour de l'Ombrelame) |
| **Durée** | 60s base |

**Formule :**
| Niveau | Damage min-max | Durée |
|--------|---------------|-------|
| 1 | 20-40 | 60s |
| 10 | 140-240 | 100s |
| 20 | 340-560 | 150s |

**Synergies reçues :** Sentinelle Lame +6%/niveau.

---

### Arbre 3 : Disciplines de l'Ombre (Shadow Disciplines)

#### 8.3.1 Maîtrise des Griffes (Claw Mastery)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Claw Mastery |
| **Nom Sodomight** | Maîtrise des Griffes |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | Passif |

**Formule :**
| Niveau | Damage bonus | AR bonus | Crit chance |
|--------|-------------|---------|------------|
| 1 | +28% | +30% | 5% |
| 10 | +145% | +165% | 14% |
| 20 | +280% | +315% | 24% |

---

#### 8.3.2 Marteau Psychique (Psychic Hammer)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Psychic Hammer |
| **Nom Sodomight** | Frappe Psychique |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 1 |
| **Prérequis** | — |
| **Coût mana** | 4 |
| **Type** | Magique (knockback) |

**Formule :**
| Niveau | Dégâts magiques | Knockback |
|--------|----------------|----------|
| 1 | 5-7 | Oui |
| 10 | 37-49 | Oui |
| 20 | 85-109 | Oui |

---

#### 8.3.3 Burst de Vitesse (Burst of Speed)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Burst of Speed |
| **Nom Sodomight** | Burst de Vitesse |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 6 |
| **Prérequis** | Maîtrise des Griffes |
| **Coût mana** | 10 |
| **Type** | Buff (vitesse d'attaque et de déplacement) |
| **Durée** | 144s base |

**Formule :**
| Niveau | Attack speed bonus | Move speed bonus | Durée |
|--------|-------------------|-----------------|-------|
| 1 | +15% | +13% | 144s |
| 10 | +33% | +22% | 225s |
| 20 | +51% | +31% | 315s |

**Notes gameplay :** **Skill essentiel pour tous les builds Ombrelame.** Maintenu en permanence. Le bonus IAS est exceptionnel pour les builds de mêlée et de pièges.

---

#### 8.3.4 Blocage d'Arme (Weapon Block)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Weapon Block |
| **Nom Sodomight** | Blocage aux Griffes |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 12 |
| **Prérequis** | Burst de Vitesse |
| **Coût mana** | Passif |
| **Prérequis équipement** | Dual Claw (deux griffes) |

**Formule :**
| Niveau | Block chance |
|--------|-------------|
| 1 | 18% |
| 10 | 33% |
| 20 | 48% |

---

#### 8.3.5 Voile d'Ombres (Cloak of Shadows)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Cloak of Shadows |
| **Nom Sodomight** | Manteau d'Ombres |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 12 |
| **Prérequis** | Frappe Psychique |
| **Coût mana** | 13 |
| **Type** | Debuff (réduit la défense et vision des ennemis) |
| **Durée** | 8s base |

**Formule :**
| Niveau | Defense réduite | Durée |
|--------|----------------|-------|
| 1 | -100% | 8s |
| 10 | -100% | 13s |
| 20 | -100% | 18s |

**Notes gameplay :** Réduit la défense ennemie à 0 (pratiquement) pendant la durée. Excellent en combinaison avec des attaques physiques.

---

#### 8.3.6 Disparition (Fade)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Fade |
| **Nom Sodomight** | Disparition |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 18 |
| **Prérequis** | Burst de Vitesse |
| **Coût mana** | 10 |
| **Type** | Buff (résistances + réduction durée des malédictions) |
| **Durée** | 144s base |

**Formule :**
| Niveau | All Resist bonus | Curse duration reduction | Durée |
|--------|-----------------|--------------------------|-------|
| 1 | +15% | -20% | 144s |
| 10 | +33% | -36% | 225s |
| 20 | +51% | -54% | 315s |

**Notes gameplay :** Alternatif à Burst of Speed selon le besoin. En Hell avec des malédictions fréquentes, Fade est souvent préféré.

---

#### 8.3.7 Guerrière de l'Ombre (Shadow Warrior)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shadow Warrior |
| **Nom Sodomight** | Ombre Guerrière |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 18 |
| **Prérequis** | Manteau d'Ombres |
| **Coût mana** | 27 |
| **Type** | Invocation (clone qui utilise 1 skill de l'Ombrelame) |

**Formule :**
| Niveau | Vie du clone | Skills utilisés |
|--------|------------|----------------|
| 1 | 60% player | 1 (aléatoire) |
| 10 | 110% player | 1 |
| 20 | 160% player | 1 |

---

#### 8.3.8 Explosion Mentale (Mind Blast)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Mind Blast |
| **Nom Sodomight** | Explosion Mentale |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 24 |
| **Prérequis** | Ombre Guerrière |
| **Coût mana** | 15 |
| **Type** | Magique + stun + conversion |
| **Effet spécial** | Stun garanti ; chance de convertir l'ennemi |

**Formule :**
| Niveau | Dégâts magiques | Stun durée | Conversion chance |
|--------|----------------|-----------|-----------------|
| 1 | 4-7 | 2s | 22% |
| 10 | 30-48 | 4s | 40% |
| 20 | 74-112 | 6.6s | 58% |

**Notes gameplay :** Excellent pour le crowd control. La conversion transforme temporairement les ennemis en alliés.

---

#### 8.3.9 Venin (Venom)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Venom |
| **Nom Sodomight** | Venin |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 30 |
| **Prérequis** | Explosion Mentale |
| **Coût mana** | 12 |
| **Type** | Buff poison (applique du poison sur toutes les attaques) |
| **Durée** | 0.4s (très courte mais très puissante) |

**Formule :**
| Niveau | Poison ajouté | Durée |
|--------|--------------|-------|
| 1 | 15-25 | 0.4s |
| 10 | 115-175 | 0.4s |
| 20 | 265-395 | 0.4s |

Note : La durée très courte concentre tout le poison en 0.4 secondes — DPS instantané massif.

**Notes gameplay :** Build "Venom Assassin" — le poison en 0.4s est quasi-instantané donc ne souffre pas des réductions de durée en Hell. Combiné avec des attaques rapides = DPS poison exceptionnel.

---

#### 8.3.10 Maître de l'Ombre (Shadow Master)
| Attribut | Valeur |
|----------|--------|
| **Nom D2** | Shadow Master |
| **Nom Sodomight** | Maître de l'Ombre |
| **Arbre** | Disciplines de l'Ombre |
| **Niveau requis** | 30 |
| **Prérequis** | Ombre Guerrière |
| **Coût mana** | 35 |
| **Type** | Invocation (clone avancé utilisant tous les skills) |

**Formule :**
| Niveau | Vie du clone | Skills utilisés |
|--------|------------|----------------|
| 1 | 80% player | Tous |
| 10 | 170% player | Tous |
| 20 | 260% player | Tous |

**Notes gameplay :** Le clone le plus puissant du jeu. Utilise tous les skills de l'Ombrelame de façon intelligente. Peut poser des pièges, utiliser des buffs, etc. Indispensable pour les builds Trapsin en groupe.

---

## PARTIE 9 — BREAKPOINTS ET EXEMPLES TOML

Les breakpoints sont les seuils exacts de pourcentage (FCR, FHR, FBR, IAS) au-delà desquels la vitesse d'animation s'améliore d'une frame. Diablo 2 fonctionne à **25 FPS**. En deçà d'un seuil, le bonus n'a aucun effet sur la vitesse d'animation.

> **Rappel technique :** Seul le palier atteint compte. Passer de 99% à 100% FCR n'a d'effet que si ce seuil est un breakpoint listé.

---

### 9.1 — FCR (Faster Cast Rate)

Le FCR réduit le nombre de frames de cast. Valeur de base = 25 FPS.

#### Sarith (Amazon)

| Frames | 19 | 18 | 17 | 16 | 15 | 14 | 13 |
|--------|----|----|----|----|----|----|-----|
| FCR %  | 0  | 7  | 14 | 22 | 32 | 48 | 68 |

> Prochain seuil non atteignable légitimement : 152% (frame 12) — hors portée équipement standard.

#### Mortecian (Necromancer)

| Frames | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|---|
| FCR %  | 0  | 9  | 18 | 30 | 48 | 75 | 125 |

#### Ravageur (Barbarian)

| Frames | 13 | 12 | 11 | 10 | 9 | 8 | 7 |
|--------|----|----|----|----|---|---|---|
| FCR %  | 0  | 9  | 20 | 37 | 63 | 105 | 200 |

#### Arcaniste (Sorceress) — Éclairs

| Frames | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 |
|--------|----|----|----|----|----|----|----|----|-----|
| FCR %  | 0  | 7  | 15 | 23 | 35 | 52 | 78 | 117 | 194 |

#### Arcaniste (Sorceress) — Autres sorts

| Frames | 13 | 12 | 11 | 10 | 9 | 8 | 7 |
|--------|----|----|----|----|---|---|---|
| FCR %  | 0  | 9  | 20 | 37 | 63 | 105 | 200 |

#### Croisé-Solaire (Paladin)

| Frames | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|---|
| FCR %  | 0  | 9  | 18 | 30 | 48 | 75 | 125 |

#### Animiste (Druid) — Forme Humaine

| Frames | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 |
|--------|----|----|----|----|----|----|----|----|-----|
| FCR %  | 0  | 4  | 10 | 19 | 30 | 46 | 68 | 99 | 163 |

#### Animiste (Druid) — Forme Ours (Werebear)

| Frames | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|----|----|---|
| FCR %  | 0  | 7  | 15 | 26 | 40 | 63 | 99 | 163 | — |

> En forme Ours, le FCR est réduit à l'IAS du porteur d'arme transformation.

#### Animiste (Druid) — Forme Loup (Werewolf)

| Frames | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|----|----|---|
| FCR %  | 0  | 6  | 14 | 26 | 40 | 60 | 95 | 157 | — |

#### Ombrelame (Assassin)

| Frames | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|----|----|---|
| FCR %  | 0  | 8  | 16 | 27 | 42 | 65 | 102 | 174 | — |

---

### 9.2 — FHR (Faster Hit Recovery)

Le FHR réduit le temps de récupération après un coup encaissé.

#### Sarith (Amazon)

| Frames | 11 | 10 | 9 | 8 | 7 | 6 | 5 |
|--------|----|----|---|---|---|---|---|
| FHR %  | 0  | 6  | 13 | 20 | 32 | 52 | 86 |

#### Mortecian (Necromancer)

| Frames | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|---|
| FHR %  | 0  | 5  | 10 | 16 | 26 | 39 | 86 |

#### Ravageur (Barbarian)

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FHR %  | 0 | 7 | 15 | 27 | 48 |

#### Arcaniste (Sorceress)

| Frames | 15 | 14 | 13 | 12 | 11 | 10 | 9 |
|--------|----|----|----|----|----|----|---|
| FHR %  | 0  | 5  | 9  | 14 | 20 | 30 | 42 |

#### Croisé-Solaire (Paladin)

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FHR %  | 0 | 7 | 15 | 27 | 48 |

#### Animiste (Druid) — Forme Humaine

| Frames | 13 | 12 | 11 | 10 | 9 | 8 | 7 |
|--------|----|----|----|----|---|---|---|
| FHR %  | 0  | 3  | 7  | 13 | 19 | 29 | 42 |

#### Animiste (Druid) — Forme Ours

| Frames | 13 | 12 | 11 | 10 | 9 | 8 | 7 |
|--------|----|----|----|----|---|---|---|
| FHR %  | 0  | 5  | 10 | 16 | 24 | 37 | 54 |

#### Animiste (Druid) — Forme Loup

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FHR %  | 0 | 7 | 15 | 27 | 48 |

#### Ombrelame (Assassin)

| Frames | 11 | 10 | 9 | 8 | 7 | 6 | 5 |
|--------|----|----|---|---|---|---|---|
| FHR %  | 0  | 7  | 14 | 22 | 32 | 48 | 75 |

---

### 9.3 — FBR (Faster Block Rate)

Le FBR réduit le temps d'animation de blocage avec un bouclier.

#### Sarith (Amazon)

| Frames | 5 | 4 | 3 | 2 |
|--------|---|---|---|---|
| FBR %  | 0 | 13 | 32 | 86 |

#### Mortecian (Necromancer)

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FBR %  | 0 | 13 | 32 | 86 | — |

#### Ravageur (Barbarian)

| Frames | 5 | 4 | 3 | 2 |
|--------|---|---|---|---|
| FBR %  | 0 | 9 | 20 | 86 |

#### Arcaniste (Sorceress)

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FBR %  | 0 | 13 | 32 | 86 | — |

#### Croisé-Solaire (Paladin)

| Frames | 5 | 4 | 3 | 2 |
|--------|---|---|---|---|
| FBR %  | 0 | 13 | 32 | 86 |

#### Animiste (Druid) — Forme Humaine

| Frames | 11 | 10 | 9 | 8 | 7 | 6 | 5 |
|--------|----|----|---|---|---|---|---|
| FBR %  | 0  | 6  | 13 | 20 | 32 | 52 | 86 |

#### Animiste (Druid) — Forme Ours

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FBR %  | 0 | 5 | 11 | 20 | 37 |

#### Animiste (Druid) — Forme Loup

| Frames | 9 | 8 | 7 | 6 | 5 |
|--------|---|---|---|---|---|
| FBR %  | 0 | 7 | 15 | 27 | 48 |

#### Ombrelame (Assassin)

| Frames | 5 | 4 | 3 | 2 |
|--------|---|---|---|---|
| FBR %  | 0 | 9 | 20 | 86 |

---

### 9.4 — IAS (Increased Attack Speed)

L'IAS est **dépendant de l'arme**. Le calcul exact varie selon :
- La vitesse de base de l'arme (WSM — Weapon Speed Modifier, de -60 à +20)
- La classe du personnage (chaque classe a son propre EIAS)
- Les auras / buffs externes (Fanaticism, Might — non stackable avec l'IAS équipement)

**Formula EIAS :**
```
EIAS = floor(120 * IAS / (120 + IAS)) - WSM + SIAS
```
Où `SIAS` = IAS provenant de skills (ex: Fanaticism, Frenzy bonus).

> Pour les calculs exacts, utiliser le **D2 Speed Calculator** (Chambers / Jarulf). Les tables IAS sont trop nombreuses pour être listées ici (varie par arme × classe × animation).

**Breakpoints IAS courants (exemples Ravageur avec hache):**

| Animation | Frames | IAS requis |
|-----------|--------|------------|
| 1-hand axe | 13 | 0 |
| 1-hand axe | 12 | 10 |
| 1-hand axe | 11 | 20 |
| 1-hand axe | 10 | 35 |
| 1-hand axe | 9  | 55 |
| 1-hand axe | 8  | 90 |
| 1-hand axe | 7  | 160 |

---

### 9.5 — Résumé Tableau Bloc par Classe

**Formule générale :**
```
Chance Blocage = (Block% Bouclier × Modificateur Classe × 2) / (Niveau + 25)
Cap = 75%
```

| Classe | Modificateur Bloc |
|--------|-------------------|
| Sarith (Amazon) | 25% |
| Mortecian (Necromancer) | 20% |
| Ravageur (Barbarian) | 25% |
| Arcaniste (Sorceress) | 20% |
| Croisé-Solaire (Paladin) | 30% |
| Animiste (Druid) | 20% |
| Ombrelame (Assassin) | 25% |

---

## PARTIE 10 — EXEMPLES TOML MGE

Les compétences sont définies en TOML dans le moteur MGE. Chaque fichier `.toml` décrit un skill avec ses métadonnées, formules, synergies et comportements.

### Format général d'un skill

```toml
# @id   skill:<id_unique>
# @do   Définir les paramètres d'une compétence active/passive/aura
# @role data
# @layer 3
# @human Données de skill importables par le moteur MGE

[skill]
id          = "skill:sarith_furie_eclair"
name        = "Furie des Éclairs"       # Nom Sodomight
name_d2     = "Lightning Fury"          # Référence D2 originale
class       = "sarith"
tree        = "combat_javelines"
tier        = 3                         # Rang dans l'arbre (1=haut, 3=bas)
level_req   = 30
max_level   = 20
skill_type  = "active"                  # active | passive | aura | toggle

[skill.prerequisites]
skills = ["lance_foudre", "greve_tonnerre"]

[skill.cost]
mana_base   = 11
mana_per_lvl = 1                        # Coût mana augmente de 1 par niveau

[skill.cast]
frames      = 13                        # Frames de cast à 0% FCR
type        = "throw"                   # throw | projectile | nova | aura | melee

[skill.damage]
type        = ["physical", "lightning"]
# Dégâts physiques (javelot)
phys_min_base = 1
phys_max_base = 1
# Dégâts foudre par éclair
lightning_min_base = 1
lightning_max_base = 40
lightning_min_per_lvl = 0
lightning_max_per_lvl = 40
# Nombre d'éclairs générés
bolts_base  = 3
bolts_per_lvl = 1                       # +1 éclair par niveau jusqu'à niveau 14

# Résumé niveau 20
# Mana : 30, Éclairs : 20, Dégâts foudre : 1–800 par éclair

[skill.synergies]
# Chaque synergie ajoute un bonus aux dégâts foudre
[[skill.synergies.received]]
from_skill  = "impulsion_foudre"        # Impulsion Foudre (Charged Strike)
bonus_type  = "lightning_dmg_pct"
per_hard_point = 0.02                   # +2% dégâts foudre par point dur

[[skill.synergies.received]]
from_skill  = "greve_tonnerre"          # Grève du Tonnerre (Thunder Strike)
bonus_type  = "lightning_dmg_pct"
per_hard_point = 0.02

[[skill.synergies.given]]
to_skill    = "lance_foudre"            # Lance Foudre (Lightning Bolt)
bonus_type  = "lightning_dmg_pct"
per_hard_point = 0.02

[skill.flags]
requires_javelins = true
pierces_enemies   = true               # Les éclairs traversent
lvl20_note = "20 éclairs, 1-800 dég foudre/éclair, 10 synergies possibles"
```

---

### Exemple 2 — Lance d'Os (Bone Spear) — Mortecian

```toml
# @id   skill:mortecian_lance_os
# @do   Définir les paramètres du skill Lance d'Os (Bone Spear)
# @role data
# @layer 3
# @human Données de skill importables par le moteur MGE

[skill]
id          = "skill:mortecian_lance_os"
name        = "Lance d'Os"
name_d2     = "Bone Spear"
class       = "mortecian"
tree        = "venin_os"
tier        = 2
level_req   = 18
max_level   = 20
skill_type  = "active"

[skill.prerequisites]
skills = ["fleche_os"]                  # Flèche d'Os (Bone Arrow / Teeth)

[skill.cost]
mana_base   = 18
mana_per_lvl = 0

[skill.cast]
frames      = 13
type        = "projectile"
pierces     = true                      # Traverse les ennemis

[skill.damage]
type        = ["magic"]
# Dégâts magiques
magic_min_base   = 80
magic_max_base   = 100
magic_min_per_lvl = 20
magic_max_per_lvl = 30

# Résumé niveau 20 (avant synergies) : 460–680 dégâts magiques

[skill.synergies]
[[skill.synergies.received]]
from_skill  = "fleche_os"               # Flèche d'Os (Teeth)
bonus_type  = "magic_dmg_pct"
per_hard_point = 0.10                   # +10% dégâts magiques

[[skill.synergies.received]]
from_skill  = "tempete_os"              # Tempête d'Os (Bone Spirit)
bonus_type  = "magic_dmg_pct"
per_hard_point = 0.10

[[skill.synergies.given]]
to_skill    = "tempete_os"
bonus_type  = "magic_dmg_pct"
per_hard_point = 0.10

[skill.flags]
non_resistable = true                   # Les dégâts magiques ne sont pas résistés
lvl20_note = "460-680 dégâts magiques + jusqu'à +200% via synergies"
```

---

### Exemple 3 — Appel aux Armes (Battle Orders) — Ravageur

```toml
# @id   skill:ravageur_appel_armes
# @do   Définir les paramètres du skill Appel aux Armes (Battle Orders)
# @role data
# @layer 3
# @human Données de skill importables par le moteur MGE

[skill]
id          = "skill:ravageur_appel_armes"
name        = "Appel aux Armes"
name_d2     = "Battle Orders"
class       = "ravageur"
tree        = "cris_guerre"
tier        = 2
level_req   = 24
max_level   = 20
skill_type  = "active"                  # Shout — buff AoE

[skill.prerequisites]
skills = ["cri_guerre"]                 # Cri de Guerre (War Cry) — non, correct: Shout

[skill.cost]
mana_base   = 11
mana_per_lvl = 1

[skill.cast]
frames      = 13
type        = "shout"
radius      = 13.3                      # Yards
affects     = ["self", "party", "mercenary"]

[skill.effect]
# Durée en secondes
duration_base   = 120
duration_per_lvl = 12                   # +12s par niveau

# Bonus Vie
life_bonus_base_pct  = 0.50            # +50% Vie à niveau 1
life_bonus_per_lvl   = 0.05            # +5% par niveau

# Bonus Mana
mana_bonus_base_pct  = 0.50
mana_bonus_per_lvl   = 0.05

# Bonus Stamina
stamina_bonus_base_pct = 0.50
stamina_bonus_per_lvl  = 0.05

# Résumé niveau 20 : +148% Vie/Mana/Stamina, durée 348s
# Exemple : Sarith avec 1000 Vie de base -> 2480 Vie avec BO niveau 20

[skill.synergies]
[[skill.synergies.received]]
from_skill  = "cri_bataille"            # Cri de Bataille (Battle Cry)
bonus_type  = "duration_pct"
per_hard_point = 0.0                    # Pas de synergie de durée directe — synergie via Battle Command

[skill.flags]
affects_hirelings = true
affects_party     = true
stacks_with_shout = false               # Ne stack pas avec Shout (même buff type)
lvl20_note = "+148% Vie/Mana/Stamina, durée 348 secondes"
```

---

### Exemple 4 — Nova Glaciale (Blizzard) — Arcaniste

```toml
# @id   skill:arcaniste_blizzard
# @do   Définir les paramètres du skill Blizzard
# @role data
# @layer 3
# @human Données de skill importables par le moteur MGE

[skill]
id          = "skill:arcaniste_blizzard"
name        = "Tempête de Glace"
name_d2     = "Blizzard"
class       = "arcaniste"
tree        = "froid"
tier        = 3
level_req   = 30
max_level   = 20
skill_type  = "active"

[skill.prerequisites]
skills = ["tempete_grelons", "vague_froid"]

[skill.cost]
mana_base   = 25
mana_per_lvl = 0

[skill.cast]
frames      = 13                        # Arcaniste autres sorts (non-éclairs)
type        = "area_delayed"            # Pluie de stalactites sur zone
duration_base = 2.8                     # Secondes de chute
area_radius  = 4.6                      # Yards

[skill.damage]
type        = ["cold"]
# Dégâts par stalactite
cold_min_base   = 75
cold_max_base   = 125
cold_min_per_lvl = 26
cold_max_per_lvl = 26
# Nombre de stalactites par seconde : environ 25 hits total sur durée
cold_length_base   = 2                  # Secondes de gel
cold_length_per_lvl = 0.2

[skill.synergies]
[[skill.synergies.received]]
from_skill  = "vague_froid"             # Vague de Froid (Glacial Spike)
bonus_type  = "cold_dmg_pct"
per_hard_point = 0.18                   # +18% dégâts froid

[[skill.synergies.received]]
from_skill  = "armure_glace"            # Armure de Glace (Ice Blast)
bonus_type  = "cold_dmg_pct"
per_hard_point = 0.18

[skill.flags]
cold_immune_bypass = false             # Pas de bypass immunité froid
lvl20_note = "101-151 dég/stalactite, +360% max via synergies"
```

---

## PARTIE 11 — INDEX DE RÉFÉRENCE RAPIDE

### 11.1 — Index des Skills par Arbre

#### Sarith (Amazon)

**Arbalètes & Arcs**
1. Flèche de Feu (Fire Arrow)
2. Frappe Longue (Long Battle Bow skill — n/a direct)
3. Flèche Multiple (Multiple Shot)
4. Pluie de Flèches (Strafe)
5. Flèche Glaciale (Cold Arrow)
6. Flèche Empoisonnée (Poison Javelin — arc variant)
7. Flèche de Glace Explosive (Exploding Arrow → Freezing Arrow)

**Javelines & Sorts**
1. Javeline de Foudre (Lightning Javelin)
2. Impulsion Foudre (Charged Strike)
3. Grève du Tonnerre (Thunder Strike)
4. Furie des Éclairs (Lightning Fury)

**Passifs & Magie**
1. Esquive (Dodge)
2. Évitement (Avoid)
3. Parade (Evade)
4. Force Intérieure (Inner Sight)
5. Valkyrie

#### Mortecian (Necromancer)

**Invocations**
1. Squelettes Guerriers (Raise Skeleton)
2. Mages Squelettes (Raise Skeleton Mage)
3. Golem de Glaise (Clay Golem)
4. Golem de Feu (Fire Golem)
5. Revenant (Revive)
6. Commandement des Ossements (Skeleton Mastery)

**Venin & Os**
1. Flèche d'Os (Teeth)
2. Mur d'Os (Bone Wall)
3. Lance d'Os (Bone Spear)
4. Tempête d'Os (Bone Spirit)
5. Nova Empoisonnée (Poison Nova)
6. Dague Empoisonnée (Poison Dagger)

**Malédictions**
1. Affaiblir (Amplify Damage)
2. Terreur (Terror)
3. Malédiction du Vide (Dim Vision)
4. Déclin (Weaken)
5. Fléau Sanguin (Iron Maiden)
6. Décréptitude (Decrepify)
7. Lenteur (Lower Resist)

#### Ravageur (Barbarian)

**Combats**
1. Coup Assourdissant (Bash)
2. Concentration (Concentrate)
3. Mêlée Sauvage (Frenzy)
4. Maelstrom (Whirlwind)
5. Saut de Combat (Combat Leap — Leap Attack)

**Maîtrises**
1. Maîtrise des Épées (Sword Mastery)
2. Maîtrise des Haches (Axe Mastery)
3. Maîtrise des Masses (Mace Mastery)
4. Maîtrise des Lances (Spear/Polearm Mastery)
5. Maîtrise Combat à Deux Armes (Two-Handed Sword / Increased Stamina)
6. Résistance Naturelle (Natural Resistance)

**Cris de Guerre**
1. Cri de Guerre (Shout)
2. Appel aux Armes (Battle Orders)
3. Commandement de Bataille (Battle Command)
4. Cri de Guerre (War Cry)
5. Berserker

#### Arcaniste (Sorceress)

**Foudre**
1. Nova
2. Éclairs en Chaîne (Chain Lightning)
3. Bouclier Statique (Static Field)
4. Vitesse de Télékinésie (Telekinesis)
5. Téléportation (Teleport)

**Froid**
1. Flèche de Glace (Ice Bolt)
2. Explosion de Glace (Ice Blast)
3. Pointe Glaciale (Glacial Spike)
4. Blizzard → Tempête de Glace
5. Nova Glaciale (Frozen Orb)
6. Armure Glaciale (Frozen Armor / Chilling Armor / Shiver Armor)

**Feu**
1. Boule de Feu (Fireball)
2. Nova de Feu (Fire Nova — Fire Wall)
3. Météore (Meteor)
4. Hydre (Hydra)
5. Enchantement (Enchant)

#### Croisé-Solaire (Paladin)

**Auras Offensives**
1. Fanatisme (Fanaticism)
2. Conviction
3. Concentration
4. Sacrifice
5. Zèle (Zeal)

**Auras Défensives**
1. Prière (Prayer)
2. Résistances (Resist Cold / Fire / Lightning)
3. Salut (Salvation)
4. Méditation (Meditation)
5. Rédemption (Redemption)
6. Santé Divine (Holy Bolt / Divine Strength)

**Sorts Offensifs**
1. Trombe Sacrée (Holy Bolt)
2. Vengeance (Vengeance)
3. Saint Feu (Holy Fire)
4. Tempête Sacrée (Holy Shock)
5. Frost Sacré (Holy Freeze)
6. Lumière Sacrée (Holy Light — FoH)
7. Marteau Sacré (Blessed Hammer)

#### Animiste (Druid)

**Invocations**
1. Loup-garou (Werewolf)
2. Ours-garou (Werebear)
3. Totem Carbone (Oak Sage)
4. Esprit de la Meute (Heart of Wolverine)
5. Esprit de la Forêt (Spirit of Barbs)
6. Corbeau (Raven)
7. Loups (Summon Wolves)
8. Ours Invoqué (Summon Grizzly)

**Sorts Élémentaires**
1. Tornades (Tornado)
2. Ouragan (Hurricane)
3. Tremblement de Terre (Earthquake — Armageddon)
4. Fissure (Fissure)
5. Eruption Volcanique (Volcano)
6. Pluie de Feu (Firestorm)
7. Pyro-Souffle (Rabies — non, Molten Boulder)

**Formes & Passifs**
1. Forme Lycanthrope (Lycanthropy)
2. Frénesie Sauvage (Feral Rage)
3. Rage de Meute (Rabies)
4. Griffe de Roche (Maul)
5. Charge Bestiale (Fury)
6. Tremblement (Shockwave)

#### Ombrelame (Assassin)

**Arts Martiales**
1. Griffe de Tigre (Tiger Strike)
2. Dragon Enflammé (Dragon Talon — Kick)
3. Griffe du Cobra (Cobra Strike)
4. Griffe de Phoenix (Phoenix Strike)
5. Frappe Clairière (Claws of Thunder)
6. Coup de Queue (Dragon Tail)
7. Vol de Dragon (Dragon Flight)

**Pièges**
1. Piège Lame (Blade Sentinel)
2. Charge d'Éclair (Charged Bolt Sentry)
3. Bouclier de Mort (Death Sentry)
4. Tour Infernale (Wake of Fire Sentry)
5. Tour Foudre (Lightning Sentry)
6. Inferno Embrasé (Wake of Inferno)

**Disciplines de l'Ombre**
1. Arme Empoisonnée (Poison Dagger — Blade Fury)
2. Cloak of Shadows → Voile de l'Ombre
3. Discret (Fade)
4. Venin (Venom)
5. Ombre Guerrière (Shadow Warrior)
6. Maître de l'Ombre (Shadow Master)

---

## PARTIE 12 — NOTES DE CONCEPTION SODOMIGHT

### 12.1 — Philosophie des Renommages

Les noms Sodomight conservent l'essence des mécaniques D2 tout en s'intégrant dans l'univers lore du jeu :

- **Classes** : Les noms évoquent le rôle et l'archétype plutôt que la culture d'origine (Amazon → Sarith, guerrière chasseresse)
- **Skills offensifs** : Noms évocateurs de l'effet visuel ou de la puissance
- **Skills défensifs/auras** : Noms liés à la protection, la lumière, la foi
- **Skills d'invocation** : Noms liés à la nécromancie, les esprits, la forêt

### 12.2 — Mécaniques à Préserver dans MGE

Les mécaniques suivantes sont fondamentales au ressenti D2 et doivent être implémentées fidèlement :

1. **Système de frames** : 25 FPS d'animation, breakpoints exacts
2. **Hard points vs Soft points** : Seuls les points durs activent les synergies
3. **Résistances immunités** : Enemies avec 100%+ résistance = immunité perçable par -res skills
4. **Life/Mana per level** : Gains stricts selon la classe (Life = Vit points × multiplicateur)
5. **Charge-up system** (Ombrelame) : 3 niveaux de charge, libération par finishing moves
6. **Auras** : Rayon d'aura fixe, une aura active à la fois par paladin (sauf items)
7. **Corpse mechanics** : Mortecian nécessite des cadavres pour certains skills
8. **Forme Lycanthrope** : Stats séparées, breakpoints séparés, items équipés persistent

### 12.3 — Paramètres d'Équilibrage

Pour l'implémentation dans MGE, les paramètres suivants devront être ajustés lors du balancing :

| Paramètre | Valeur D2 | Note Sodomight |
|-----------|-----------|----------------|
| Experience curve | Exponentielle | À calibrer selon progression cible |
| Max level | 99 | Peut être réduit à 80 pour un endgame plus accessible |
| Synergy cap | Illimité | Considérer un cap à 20 points pour éviter les builds mono-skill |
| Resistance cap | 75% | Peut être monté à 80% pour le build définitif |
| Life globe size | Base + Vit | Conserver la formule exacte D2 |
| FCR caps by class | Voir tables | Implémenter les breakpoints exacts |

---

*Fin du document SD-Classes-Skills.md — Référence Sodomight complète v1.0*

*Généré pour le projet Miyukini COG / Sodomight — Moteur MGE*
*Basé sur les données Diablo 2: Lord of Destruction v1.14d*
