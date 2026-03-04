# REF-02a -- Formules de Combat, Stats & Breakpoints -- Reference Compacte
<!-- @id REF-02a @do reference-combat-formulas @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-02 le 2026-03-03. Formules exactes, valeurs par slvl, MGE mapping, cross-refs -->

**Projet** : Sodomight | **Source** : D2:LoD v1.14d / D2R 2.8

> **TL;DR** : Toutes les formules de combat D2 : CTH, pipeline physique/elementaire, modificateurs speciaux (CB, OW, CS/DS), IAS/block/defense, leech, poison, breakpoints FCR/FHR/FBR, stats de base des 7 classes, et immunity breaking. Chaque formule mappe vers un module Rust TDD dans `mge-arpg-combat` ou `mge-arpg-stats`.

> **Voir aussi** : [REF-02b](REF-02b-Classes-Skills.md) pour les 7 classes et leurs 210 skills.

---

## 1. Formules de combat [S1]

-> MGE: `mge-arpg-combat` -- toutes les formules ci-dessous = fonctions Rust testees TDD

### 1.1 Chance to Hit [S1]
```
CTH = clamp(200 * AR / (AR + DR) * alvl / (alvl + dlvl), 5, 95)
ITD : CTH = clamp(200 * alvl / (alvl + dlvl), 5, 95)     -- ignore DR
PvP : DR_eff = DR / 2
```
Defense=0 en course (sauf Barbarian). **Always-hit** : Guided Arrow, Smite, Blessed Hammer, Mind Blast, Conversion.
-> MGE: `mge-arpg-combat::chance_to_hit(ar, dr, clvl, mlvl) -> f32`

### 1.2 Pipeline physique [S1]
```
1. WeaponBaseDmg (min-max)
2. * 1.5 si Ethereal                        -- avant tout
3. * (1 + STR_bonus/100 + ED_on_weapon/100) -- STR/DEX factor par type arme
4. + Flat min/max (jewels/charms)
5. * (1 + ED_off_weapon/100)                -- skills, auras, additif
6. * (1 + Critical_or_Deadly/100)           -- x2 si crit
7. - flat Physical DR
8. * (1 - DR%/100)                          -- cap 50% en PvM
```

**STR/DEX factors par type d'arme** :

| Type arme | STR% | DEX% | Total |
|-----------|------|------|-------|
| Swords/Axes/Staves/Spears/Polearms | 100 | 0 | 100 |
| Maces/Scepters/Wands | 110 | 0 | 110 |
| Daggers/Throwing/Javelins | 50 | 50 | 100 |
| Bows/Crossbows | 0 | 100 | 100 |
| Claws (Assassin) | 75 | 75 | 150 |

-> MGE: `mge-arpg-combat::PhysDamagePipeline` + `assets/data/weapon_types.toml`

### 1.3 Pipeline elementaux [S1]
```
1. Skill/Item base elemental (min-max)
2. + Synergy bonus (% par hard point SEULEMENT)
3. * (1 + Mastery%/100)
4. * (1 - target_resistance/100)            -- cap -100% (= x2 dmg max)
```
6 types : Physical, Fire, Cold, Lightning, Poison, Magic.
-> MGE: `mge-arpg-combat::ElemDamagePipeline` + `mge-arpg-skills::SynergyCalc`
-> Cross-ref: [REF-04b S2.7](REF-04b-Monsters-Multiplayer.md) pour immunites

### 1.4 Modificateurs speciaux [S1]

**Critical Strike** (Ama passif) :
```
CS% = slvl * 100 / (slvl + 6)
```
slvl1=14%, slvl5=45%, slvl10=63%, slvl15=71%, slvl20=77%

**Deadly Strike** (items) : meme formule mais depuis items. Ne stack pas avec CS :
```
EffDouble% = CS + DS * (100 - CS) / 100
```

**Crushing Blow** -- % HP restante du monstre, AVANT degats normaux :

| Contexte | Melee | Ranged |
|----------|-------|--------|
| Normal mob | 1/4 (25%) | 1/8 (12.5%) |
| Champ/Unique/Boss | 1/8 (12.5%) | 1/16 (6.25%) |
| PvP | 1/10 (10%) | 1/20 (5%) |
| Hireling | 1/10 (10%) | 1/20 (5%) |

MP scaling: `CB_dmg = HP_current * fraction / (0.5 + 0.5 * players)`
-> MGE: `mge-arpg-combat::CrushingBlow`

**Open Wounds** -- DoT physique non-resistable, 200 frames (8s @25fps), bloque regen :

| Clvl range | Formule dmg/frame (/256) |
|------------|--------------------------|
| 1-15 | (9*Clvl + 31) / 256 |
| 16-30 | (18*Clvl - 104) / 256 |
| 31-45 | (27*Clvl - 374) / 256 |
| 46-60 | (36*Clvl - 779) / 256 |
| 61-99 | (45*Clvl - 1319) / 256 |

Exemple Clvl 80 : (45*80-1319)/256 = 9.38 dmg/frame = 1876 total sur 8s
-> MGE: `mge-arpg-combat::OpenWounds`

**Prevent Monster Heal** : `can_regen = false` pendant 120000 frames (~80min @25fps).

### 1.5 IAS [S1]
```
EIAS = 120 * IAS / (120 + IAS)              -- diminishing returns
TotalSpeed = EIAS + SkillIAS - WSM
Frames = lookup_breakpoint_table(TotalSpeed)
```

| Arme | WSM | Notes |
|------|-----|-------|
| Phase Blade | -30 | Fastest base |
| Crystal Sword | 0 | Neutral |
| Berserker Axe | 0 | Popular Grief base |
| War Pike | +20 | Slow |
| Colossus Voulge | +10 | Insight base |
| Monarch | +10 | Spirit base (shield) |

-> MGE: `mge-arpg-combat::IasCalc` + `assets/data/weapon_speed.toml`
-> Cross-ref: [REF-03 S1] pour armes bases

### 1.6 Block [S1]
```
EffBlock% = min(shield_block% * (DEX - 15) / (Clvl * 2), 75)
DEX_for_max_block = 75 * Clvl * 2 / shield_block% + 15
```
En course : block% / 3. Holy Shield : +bonus massif block%.
-> MGE: `mge-arpg-combat::BlockCalc`

### 1.7 Defense [S1]
```
TotalDef = (DEX/4 + SumFlatDef + Charms) * (1 + SumPercentDef%/100)
```
Ethereal = +50% def base. En course : defense = 0 (sauf Barb).
-> MGE: `mge-arpg-stats::DefenseCalc`

### 1.8 Leech [S1]
```
Stolen_Life = floor(PhysDmg * LL% / 100 * diff_penalty * drain_eff)
Stolen_Mana = floor(PhysDmg * ML% / 100 * diff_penalty * drain_eff)
```

| Difficulte | Penalite leech |
|------------|---------------|
| Normal | 100% |
| Nightmare | 50% |
| Hell | 33% |

Boss drain_effectiveness souvent 0%. Life Tap = 50% phys->life, **ignore** penalites diff.
-> MGE: `mge-arpg-combat::LeechCalc` -> [REF-01 S3] pour penalites diff

### 1.9 Poison [S1]
```
Dmg_per_frame = bit_rate / 256
Total_dmg = dmg_per_frame * duration_frames   -- 25fps
Affichage = total_dmg * duration_sec / 256
```
Ne stack PAS. Nouveau remplace si `bit_rate >= ancien_bit_rate`.
-> MGE: `mge-arpg-combat::PoisonDot`

---

## 2. Stats de base [S1]

-> MGE: `mge-arpg-stats::ClassStats` dans `assets/data/classes.toml`

### 2.1 Attributs lvl 1 + gains

| Classe | STR | DEX | VIT | ENE | Life | Mana | Stam | L/VIT | M/ENE | L/lvl | M/lvl |
|--------|-----|-----|-----|-----|------|------|------|-------|-------|-------|-------|
| Amazon | 20 | 25 | 20 | 15 | 50 | 15 | 84 | +3 | +1.5 | +2 | +1.5 |
| Assassin | 20 | 20 | 20 | 25 | 50 | 25 | 95 | +3 | +1.75 | +2 | +1.5 |
| Barbarian | 30 | 20 | 25 | 10 | 55 | 10 | 91 | +4 | +1 | +2 | +1 |
| Druid | 15 | 20 | 25 | 20 | 55 | 20 | 84 | +2 | +2 | +1.5 | +2 |
| Necromancer | 15 | 25 | 15 | 25 | 45 | 25 | 79 | +2 | +2 | +1.5 | +2 |
| Paladin | 25 | 20 | 25 | 15 | 55 | 15 | 89 | +3 | +1.5 | +2 | +1.5 |
| Sorceress | 10 | 25 | 10 | 35 | 40 | 35 | 74 | +2 | +2 | +1 | +2 |

### 2.2 BaseAR & Block Modifier

`BaseAR = (DEX - 7) * 5 + ClassBaseAR`

| | Ama | Asn | Bar | Dru | Nec | Pal | Sor |
|-|-----|-----|-----|-----|-----|-----|-----|
| ClassBaseAR | 9 | 14 | 10 | 7 | 7 | 10 | 7 |
| Block Mod | 25% | 25% | 25% | 20% | 20% | 30% | 20% |

---

## 3. Breakpoints FCR/FHR/FBR [S1]

-> MGE: `assets/data/breakpoints.toml` charge par `mge-arpg-combat::BreakpointTable`
-> Format TOML : `[fhr.amazon] frames=[11,10,...] thresholds=[0,6,...]`

### 3.1 FHR (seuil declenchement : dmg > max_hp/12)

| Classe | Frames (seuils%) |
|--------|-----------------|
| Amazon | 11(0) 10(6) 9(13) 8(20) 7(32) 6(52) 5(86) 4(174) 3(600) |
| Assassin | 9(0) 8(7) 7(15) 6(27) 5(48) 4(86) 3(200) |
| Barbarian | 9(0) 8(7) 7(15) 6(27) 5(48) 4(86) 3(200) |
| Druid human | 15(0) 14(3) 13(7) 12(13) 11(19) 10(29) 9(42) 8(63) 7(99) |
| Druid bear | 14(0) 13(5) 12(10) 11(16) 10(24) 9(37) 8(54) 7(86) 6(152) |
| Druid wolf | 7(0) 6(9) 5(20) 4(42) 3(86) 2(280) |
| Necromancer | 14(0) 13(5) 12(10) 11(16) 10(26) 9(39) 8(56) 7(86) 6(152) |
| Paladin | 9(0) 8(7) 7(15) 6(27) 5(48) 4(86) 3(200) |
| Sorceress | 15(0) 14(5) 13(9) 12(14) 11(20) 10(30) 9(42) 8(60) 7(86) |

### 3.2 FCR

| Classe | Frames (seuils%) |
|--------|-----------------|
| Amazon | 19(0) 18(7) 17(14) 16(22) 15(32) 14(48) 13(68) |
| Assassin | 18(0) 17(8) 16(16) 15(27) 14(42) 13(65) 12(102) |
| Barbarian | 15(0) 14(9) 13(20) 12(37) 11(63) 10(105) 9(200) |
| Druid human | 19(0) 18(4) 17(10) 16(19) 15(30) 14(46) 13(68) |
| Druid bear | 18(0) 17(7) 16(15) 15(26) 14(40) 13(63) 12(99) |
| Druid wolf | 18(0) 17(6) 16(14) 15(26) 14(40) 13(60) 12(95) |
| Necromancer | 17(0) 16(9) 15(18) 14(30) 13(48) 12(75) 11(125) |
| Paladin | 17(0) 16(9) 15(18) 14(30) 13(48) 12(75) 11(125) |
| Sorc (Lightning) | 19(0) 18(7) 17(15) 16(23) 15(35) 14(52) 13(78) 12(117) 11(194) |
| Sorc (other) | 16(0) 15(9) 14(20) 13(37) 12(63) 11(105) 10(200) |

### 3.3 FBR

| Classe | Frames (seuils%) |
|--------|-----------------|
| Amazon | 5(0) 4(13) 3(32) 2(86) 1(600) |
| Assassin | 5(0) 4(13) 3(32) 2(86) 1(600) |
| Barbarian | 8(0) 7(9) 6(20) 5(42) 4(86) |
| Druid human/bear | 12(0) 11(5) 10(10) 9(16) 8(27) |
| Druid wolf | 9(0) 8(7) 7(15) 6(27) 5(48) |
| Necromancer | 11(0) 10(5) 9(10) 8(16) 7(26) |
| Paladin | 5(0) 4(13) 3(32) 2(86) 1(600) |
| Paladin (HS) | 2(0) 1(86) |
| Sorceress | 9(0) 8(7) 7(15) 6(27) 5(48) |

---

## 4. Immunite breaking [S1]

-> MGE: `mge-arpg-combat::ImmunityBreak`
-> Cross-ref: [REF-04b S2.7](REF-04b-Monsters-Multiplayer.md) pour liste immunites par zone

```
resistance > 99% = IMMUNE
Breaking : skill_reduction * (1/5) applique sur la portion au-dessus de 99%
Reduction_necessaire = (Res_monstre - 99) * 5
```

| Skill | Max red | Vs Immune (1/5 eff) | Brise si res < |
|-------|---------|--------------------|----|
| Lower Resist (Necro slvl20) | -70% | -14% | 113% |
| Conviction (Paladin slvl25) | -150% | -30% | 129% |
| Amplify Damage (Necro) | -100% phys | -20% phys | 119% |
| Decrepify (Necro) | -50% phys | -10% phys | 109% |
| Cold Mastery (vanilla) | N/A | **NE brise PAS** | N/A |
| Sunder Charms (D2R 2.5+) | Set a 95% | Casse immunite -> 95% | Always |

Exemple : 105% fire + Conviction -150 -> 105 - 30 = 75% (broken).
115% fire + Conviction -150 -> 115 - 30 = 85% (still immune at 85%).
