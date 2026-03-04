# REF-02b -- Les 7 Classes & 210 Skills -- Reference Compacte
<!-- @id REF-02b @do reference-classes-skills @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-02 le 2026-03-03. Enrichi: valeurs par slvl, MGE mapping, sprint prio, cross-refs -->

**Projet** : Sodomight | **Source** : D2:LoD v1.14d / D2R 2.8

> **TL;DR** : Les 7 classes D2 avec leurs 3 arbres de skills chacune (210 skills total), tables de valeurs par slvl pour les skills meta (Lightning Fury, Blessed Hammer, Blizzard, Frozen Orb, Corpse Explosion), limites de summons, et formules WW. Chaque skill mappe vers une struct dans `mge-arpg-skills` avec synergies en TOML. **Voir aussi** : [REF-02a](REF-02a-Combat-Formulas.md) pour les formules de combat, stats de base, breakpoints et immunity breaking.

---

## 1. Les 7 classes -- 210 Skills [S1-S2]

-> MGE: `mge-arpg-skills` -- chaque skill = struct dans `assets/data/skills/`. Synergies = TOML refs.
Format : `Skill (lvl_req) [mana@1/mana@20] -- description [synergies]`

### 1.1 Amazon -- Javelin & Spear [S2]

| # | Skill | Req | Mana 1/20 | Effet cle | Synergies |
|---|-------|-----|-----------|-----------|-----------|
| 1 | Jab | 1 | 2/2 | Triple melee rapide | -- |
| 2 | Power Strike | 6 | 2/2 | +1-lightning | CS +10%/pt |
| 3 | Poison Javelin | 6 | 3/3 | Trainee poison | Plague +10%/pt |
| 4 | Impale | 12 | 3/3 | +dmg, degrade arme | -- |
| 5 | Lightning Bolt | 12 | 4/4 | Javeline->eclair | LF +1%/pt |
| 6 | Charged Strike | 18 | 5/5 | Bolts foudre multiples | LF +1%/pt |
| 7 | Plague Javelin | 18 | 6/6 | Nuage poison AoE | PoisJav +10%/pt |
| 8 | Fend | 24 | 5/5 | Multi-target melee | -- |
| 9 | Lightning Strike | 30 | 9/9 | Chain lightning melee | PS/CS/LB +3%/pt |
| 10 | **Lightning Fury** | 30 | 10/19.5 | Foudre split | PS/CS/LB/LS +1%/pt |

**Lightning Fury valeurs par slvl** :

| slvl | Dmg max | Mana | Bolts |
|------|---------|------|-------|
| 1 | 1-40 | 10 | 2 |
| 5 | 1-120 | 12 | 6 |
| 10 | 1-240 | 14.5 | 11 |
| 15 | 1-390 | 17 | 16 |
| 20 | 1-580 | 19.5 | 21 |

### 1.1b Amazon -- Bow & Crossbow [S2]

| # | Skill | Req | Mana 1/20 | Effet cle | Synergies |
|---|-------|-----|-----------|-----------|-----------|
| 1 | Magic Arrow | 1 | 1.5/1.5 | 100% magic, ammo infinie | -- |
| 2 | Fire Arrow | 1 | 3/3 | Phys+feu | Exploding +12%/pt |
| 3 | Cold Arrow | 6 | 3/3 | Phys+froid+chill | Freezing +12%/pt |
| 4 | Multiple Shot | 6 | 5/5 | Multi-fleche -25% dmg | -- |
| 5 | Exploding Arrow | 12 | 5/5 | AoE feu | Immolation +8%/pt |
| 6 | Ice Arrow | 18 | 5/5 | Gel ST | Freezing +8%/pt |
| 7 | Guided Arrow | 18 | 8/8 | Auto-aim, always-hit | -- |
| 8 | Immolation Arrow | 24 | 8/8 | AoE feu + brasier | Exploding +8%/pt |
| 9 | Strafe | 24 | 11/11 | Auto-cible multi | -- |
| 10 | Freezing Arrow | 30 | 10/10 | AoE gel | ColdArrow/IceArrow +5%/pt |

### 1.1c Amazon -- Passive & Magic [S1]

| # | Skill | Req | Effet | slvl1 | slvl10 | slvl20 |
|---|-------|-----|-------|-------|--------|--------|
| 1 | Inner Sight | 1 | Debuff -def | -40def | -175def | -355def |
| 2 | Critical Strike | 1 | Passif double dmg | 16% | 51% | 64% |
| 3 | Dodge | 6 | Evite melee (immobile) | 18% | 38% | 50% |
| 4 | Slow Missiles | 12 | Ralentit projectiles 1/3 | 12s | 30s | 50s |
| 5 | Avoid | 12 | Evite range (immobile) | 14% | 34% | 46% |
| 6 | Penetrate | 18 | +AR% | +35% | +135% | +260% |
| 7 | Decoy | 24 | Leurre | 10s | 55s | 105s |
| 8 | Evade | 24 | Evite all (mouvement) | 18% | 34% | 46% |
| 9 | Valkyrie | 30 | Guerriere invoquee (1 max) | Equip T1 | T3 | T5 |
| 10 | Pierce | 30 | Projectiles traversent | 23% | 52% | 67% |

> Dodge/Avoid/Evade = animation lock interrompant l'attaque. Bug connu.

### 1.2 Assassin -- Martial Arts [S2]

Charges : max 3/skill, timeout 15s, Finisher consomme toutes charges. Mixer possible.

| # | Skill | Req | Type | Effet cle |
|---|-------|-----|------|-----------|
| 1 | Tiger Strike | 1 | Charge-up | +100/200/300% phys (3 charges) |
| 2 | Dragon Talon | 1 | Finisher | Kick, +IAS |
| 3 | Fists of Fire | 6 | Charge-up | +fire 1/2/3 charges |
| 4 | Dragon Claw | 6 | Finisher | Double claw hit |
| 5 | Cobra Strike | 12 | Charge-up | C1=LL, C2=ML, C3=both |
| 6 | Claws of Thunder | 18 | Charge-up | +lightning |
| 7 | Dragon Tail | 18 | Finisher | Kick AoE fire(=phys*%) |
| 8 | Blades of Ice | 24 | Charge-up | +cold + freeze |
| 9 | Dragon Flight | 24 | Finisher | Teleport kick |
| 10 | Phoenix Strike | 30 | Charge-up | C1=meteor, C2=chain light, C3=chaos ice |

### 1.2b Assassin -- Traps [S2]

Traps = missiles (PAS summons), affectees par -EnemyRes. Max 5 actives simultanement.

| # | Skill | Req | Mana | Effet cle | Synergies |
|---|-------|-----|------|-----------|-----------|
| 1 | Fire Blast | 1 | 4 | Bombe feu | WoF +12%/pt |
| 2 | Shock Web | 6 | 6 | Toile electrique | CBS/LS +12%/pt |
| 3 | Blade Sentinel | 6 | 7 | Lame patrouillante | -- |
| 4 | Charged Bolt Sentry | 12 | 11 | Bolts foudre | LS +12%/pt, ShW +12%/pt |
| 5 | Wake of Fire | 12 | 10 | Vagues feu | FB +10%/pt |
| 6 | Blade Fury | 18 | 3 | Projectiles lames | -- |
| 7 | **Lightning Sentry** | 24 | 20 | Eclairs (meta) | CBS +12%/pt, ShW +12%/pt |
| 8 | Wake of Inferno | 24 | 14 | Jet flammes | FB +10%/pt |
| 9 | **Death Sentry** | 30 | 20 | Lightning + CE (40-80% HP) | LS +12%/pt |
| 10 | Blade Shield | 30 | 25 | Lames rotatives | -- |

### 1.2c Assassin -- Shadow Disciplines [S2]

| # | Skill | Req | Effet | slvl20 |
|---|-------|-----|-------|--------|
| 1 | Claw Mastery | 1 | +dmg/AR/crit claws | +188%dmg, +190%AR, 28%crit |
| 2 | Psychic Hammer | 1 | Knockback magic dmg | -- |
| 3 | Burst of Speed | 6 | +IAS +FRW | +52%IAS, +41%FRW, 327s |
| 4 | Weapon Block | 12 | Block dual claws | 56% block |
| 5 | Cloak of Shadows | 12 | Aveuglement -def | 1 cast max |
| 6 | Fade | 18 | +AllRes, -curse dur. | +60%AllRes, DR 16%, 319s |
| 7 | Shadow Warrior | 18 | Clone (vos skills, 1 max) | -- |
| 8 | Mind Blast | 24 | Stun + conversion | Always-hit, 2s stun |
| 9 | Venom | 30 | +poison dmg armes | 400 poison/0.4s |
| 10 | Shadow Master | 30 | Clone autonome (1 max) | Uses own AI, better gear |

BoS et Fade = **mutuellement exclusifs**.
### 1.3 Barbarian -- Combat Skills [S2]

| # | Skill | Req | Mana | Effet cle | Notes |
|---|-------|-----|------|-----------|-------|
| 1 | Bash | 1 | 2 | Knockback +ED | -- |
| 2 | Double Swing | 6 | 1 | Dual double frappe | Mana-positive avec leech |
| 3 | Leap | 6 | 2 | Saut, evite obstacles | -- |
| 4 | Double Throw | 12 | 2 | Dual lancer ranged | -- |
| 5 | Stun | 12 | 2 | Melee stun 2-5.5s | -- |
| 6 | Leap Attack | 18 | 7 | Saut + frappe | -- |
| 7 | Concentrate | 18 | 2 | Non-interruptible, +def | -- |
| 8 | Frenzy | 24 | 3 | Dual accelerant | +IAS+FRW cumulatif |
| 9 | **Whirlwind** | 30 | 25 | Tourbillon | WSM-based, IAS items ignorees |
| 10 | Berserk | 30 | 4 | +dmg 100% magic, def=0 | -- |

**WW frames par WSM** : WSM>=15->12f, >=10->10f, >=-10->8f, >=-34->6f, <-34->4f.
-> MGE: `mge-arpg-skills::WhirlwindCalc` lookup table

### 1.3b Barbarian -- Combat Masteries [S1]

| Skill | Req | slvl20 | Notes |
|-------|-----|--------|-------|
| Sword/Axe/Mace Mastery | 1 | +128%dmg, +128%AR, 32%crit | 1 mastery par type arme |
| Polearm/Spear/Throwing | 6 | idem | -- |
| Inc. Stamina | 12 | +120% stam | -- |
| Iron Skin | 18 | +200% def | Additif avec autres %def |
| Inc. Speed | 24 | +29% FRW | -- |
| Natural Resistance | 30 | +48% AllRes | Additif avec res gear |

### 1.3c Barbarian -- Warcries [S1]

| # | Skill | Req | Mana | Effet | slvl1 / slvl20 |
|---|-------|-----|------|-------|----------------|
| 1 | Howl | 1 | 4 | AoE fuite | 4s / 23s dur |
| 2 | Find Potion | 1 | 2 | Corpse->potion | 23% / 55% chance |
| 3 | Taunt | 6 | 3 | Force melee | -15%dmg, -25%AR |
| 4 | Shout | 6 | 6 | +defense groupe | +100% / +270%def, 120-300s |
| 5 | **Find Item** | 12 | 7 | "Hork" re-roll loot+MF | 12% / ~50% chance |
| 6 | Battle Cry | 18 | 11 | Debuff -def -dmg | -50%def/-25%dmg |
| 7 | **Battle Orders** | 24 | 7 | +Life/Mana groupe | +35% / +107%, 140-480s |
| 8 | Grim Ward | 24 | 4 | Totem fuite | -- |
| 9 | War Cry | 30 | 10 | AoE dmg + stun | 12-66 dmg, 2-5s stun |
| 10 | Battle Command | 30 | 11 | +1 all skills groupe | 12-168s dur |

### 1.4 Druid -- Elemental [S2]

| Skill | Req | Type | Synergies | Notes |
|-------|-----|------|-----------|-------|
| Firestorm | 1 | Fire DoT | Molten Boulder +23%/pt | -- |
| Molten Boulder | 6 | Fire+knockback | Firestorm +23%/pt | -- |
| Arctic Blast | 6 | Cold beam | -- | -- |
| Fissure | 12 | Fire AoE | Firestorm +23%/pt | -- |
| Cyclone Armor | 12 | Absorb elem | Twister/Tornado/Hurricane +4%/pt | -- |
| Twister | 18 | Phys stun | Tornado +1%/pt | -- |
| Volcano | 24 | Fire burst | Firestorm/MB +23%/pt each | -- |
| **Tornado** | 24 | **Physical** | Twister +1%/pt, CA +1%/pt, Hurricane +1%/pt | Degats PHYSIQUES |
| Hurricane | 30 | Cold AoE | Arctic Blast +1%/pt | slvl20: 310-340 cold |
| Armageddon | 30 | Fire rain | -- | -- |

### 1.4b Druid -- Shape Shifting [S2]

| Forme | slvl20 bonus | Breakpoints FHR/FCR | Notes |
|-------|-------------|--------------------|----|
| Werewolf | +120%AR, +30%IAS | **Differents** du human | Fury = meta |
| Werebear | +268%Life, +186%Dmg | **Differents** du human | Tank, Maul |
| Lycanthropy | +200% Life | Passif | Boost les 2 formes |

Forme change TOUS les breakpoints -> [REF-02a S3](REF-02a-Combat-Formulas.md) pour tables completes.

### 1.4c Druid -- Summoning [S2]

| Summon | Max | slvl20 | Notes |
|--------|-----|--------|-------|
| Raven | 5 | ~24 dmg, limited hits | Anti-flee utility |
| Spirit Wolf | 5 | ~120 HP each | -- |
| Dire Wolf | 3 | ~320 HP, eats corpse | -- |
| Grizzly | 1 | ~1800 HP | Tank, knockback |
| Oak Sage | 1 (spirit) | +50% Life | Best spirit for most |
| Heart of Wolverine | 1 (spirit) | +155%Dmg, +155%AR | DPS spirit |
| Spirit of Barbs | 1 (spirit) | Thorns | Niche |
| Poison/Carrion/Solar Creeper | 1 (vine) | Poison/LL/ML | -- |

1 Spirit + 1 Vine max. Ravens+Wolves+DireWolves+Grizzly coexistent.

### 1.5 Necromancer -- Summoning [S2]

| # | Skill | Req | Count formula | slvl20 |
|---|-------|-----|---------------|--------|
| 1 | Raise Skeleton | 1 | 1+floor(slvl/3) | 7 skel, ~150 HP each (w/ mastery) |
| 2 | Skeleton Mastery | 1 | Passif | +200%HP, +150%dmg aux skel |
| 3 | Clay Golem | 6 | 1 golem | Slow on hit, ~2000 HP |
| 4 | Golem Mastery | 12 | Passif | +300%HP, +90% speed |
| 5 | Skeletal Mage | 12 | 1+floor(slvl/3) | 7 mages, elem random |
| 6 | Blood Golem | 18 | 1 golem | Life sharing, ~1500 HP |
| 7 | Summon Resist | 24 | Passif | +65% AllRes summons |
| 8 | Iron Golem | 24 | 1 golem | Herite props item metal (perd si meurt!) |
| 9 | Fire Golem | 30 | 1 golem | Holy Fire aura, ~3500 HP |
| 10 | Revive | 30 | 1/pt | 180s duree, gardent skills monstre |

### 1.5b Necromancer -- Poison & Bone [S1]

| # | Skill | Req | Mana | Effet | Synergies |
|---|-------|-----|------|-------|-----------|
| 1 | Teeth | 1 | 3 | Projectiles magic | BSpear/BSpirit +7%/pt |
| 2 | Bone Armor | 1 | 11 | Absorbe phys | BW/BP/Teeth +15/pt flat |
| 3 | Poison Dagger | 6 | 4 | +poison melee | PoisExp/PoisNova +15%/pt |
| 4 | **Corpse Explosion** | 6 | 15-34 | 70-120% CorpseHP, radius | 50%phys + 50%fire |
| 5 | Bone Wall | 12 | 17 | Mur | BSpear/BSpirit +15%/pt |
| 6 | Poison Explosion | 18 | 8 | AoE poison corpse | PD/PN +15%/pt |
| 7 | **Bone Spear** | 18 | 12-31 | Perforant magic | Teeth/BW/BP +7%/pt each |
| 8 | Bone Prison | 24 | 27 | Cage | BSpear/BSpirit +15%/pt |
| 9 | Poison Nova | 30 | 20 | AoE poison radiale | PD/PE +15%/pt |
| 10 | **Bone Spirit** | 30 | 12 | Traceur magic | BSpear/BW/BP +7%/pt each |

**Corpse Explosion valeurs par slvl** :

| slvl | Mana | Radius (phys/fire) |
|------|------|--------------------|
| 1 | 15 | 2.67 / 2.67 yd |
| 5 | 19 | 4.0 / 4.0 yd |
| 10 | 24 | 5.33 / 6.0 yd |
| 15 | 29 | 6.67 / 7.33 yd |
| 20 | 34 | 8.67 / 9.33 yd |

Degats = **toujours** 70-120% du MaxHP du cadavre. Scale avec /players HP!

### 1.5c Necromancer -- Curses [S1]

| # | Curse | Req | Mana | Effet exact | Duree slvl1/20 |
|---|-------|-----|------|-------------|----------------|
| 1 | **Amplify Damage** | 1 | 4 | -100% Phys Res | 8s / 27s |
| 2 | Dim Vision | 6 | 9 | -vision AI | 6s / 25s |
| 3 | Weaken | 6 | 4 | -33% dmg monstre | 8s / 27s |
| 4 | Iron Maiden | 12 | 5 | Thorns 200-715% | 12s / 31s |
| 5 | Terror | 12 | 7 | Fuite | 4s / 23s |
| 6 | Confuse | 18 | 13 | Attaque aleatoire | 10s / 29s |
| 7 | **Life Tap** | 18 | 9 | 50% phys->vie (ignore penalites) | 16s / 35s |
| 8 | Attract | 24 | 17 | Focalise aggro | 12s / 31s |
| 9 | **Decrepify** | 24 | 11 | -50% speed, -50% dmg, +50% dmg pris | 4s / 23s |
| 10 | **Lower Resist** | 30 | 22 | -res elem (brise immun 1/5) | slvl1:-25%, slvl20:-70% |

UNE SEULE curse active par monstre. Nouvelle = remplace.

### 1.6 Paladin -- Combat Skills [S1]

| # | Skill | Req | Mana | Effet | Notes |
|---|-------|-----|------|-------|-------|
| 1 | Sacrifice | 1 | 0 | +dmg massif, perd 8% vie | -- |
| 2 | Smite | 1 | 2 | Always-hit, stun, unblockable | CB/DS/OW applicables |
| 3 | Holy Bolt | 6 | 2-8 | Dmg undead, heal allie | -- |
| 4 | Zeal | 12 | 2 | Multi-hit (5 max) | AR bonus intrinseque |
| 5 | Charge | 12 | 9 | Dash + frappe | -- |
| 6 | Vengeance | 18 | 4 | +fire/cold/lightning | -- |
| 7 | **Blessed Hammer** | 18 | 5-9.7 | Spirale magic | Voir formule ci-dessous |
| 8 | Conversion | 24 | 4 | Convertir monstre | Always-hit, 90%+chance |
| 9 | Holy Shield | 24 | 35 | +block% +def +smite dmg | slvl20: +43% block, 245s |
| 10 | Fist of Heavens | 30 | 25 | Lightning + Holy Bolts AoE | 1s cooldown |

**Blessed Hammer formule & valeurs** :
```
Hammer_Dmg = Base * (1 + Vigor_syn*14%/pt + BAim_syn*14%/pt) * (1 + Conc_bonus/2)
```

| slvl | Base min-max | Mana | Full synergy (20 BAim, 20 Vigor) |
|------|-------------|------|----------------------------------|
| 1 | 12-16 | 5 | ~72-96 + Conc |
| 5 | 44-48 | 6 | ~264-288 + Conc |
| 10 | 88-92 | 7.2 | ~528-552 + Conc |
| 15 | 140-144 | 8.5 | ~840-864 + Conc |
| 20 | 196-200 | 9.7 | ~1176-1200 + Conc |

Avec slvl20 Concentration (+300% listed -> +150% to Hammer) : **~18-20k** dmg endgame.
150% bonus vs Undead. -> MGE: `mge-arpg-skills::BlessedHammer`

### 1.6b Paladin -- Offensive Auras [S1]

| Aura | Req | slvl20 | Synergy | Notes |
|------|-----|--------|---------|-------|
| Might | 1 | +280% ED | -- | -- |
| Holy Fire | 6 | 150-175 pulse | -- | -- |
| Thorns | 6 | ~800% retour | -- | -- |
| Blessed Aim | 12 | +255% AR | BHammer +14%/pt | -- |
| **Concentration** | 18 | +300% ED | BHammer: **50% du listed** | Meta Hammerdin |
| Holy Freeze | 18 | 80-100 pulse + slow | -- | Merc A2 Def meta |
| Holy Shock | 24 | 1-700 pulse | -- | Dream runeword -> [REF-03] |
| Sanctuary | 24 | Knockback undead | **Ignore** undead immunite | -- |
| **Fanaticism** | 30 | +317%ED, +35%IAS | -- | Meta physique |
| **Conviction** | 30 | -150% def/-150% res | Brise immun 1/5 | Infinity merc -> [REF-03] |

1 aura skill active a la fois. Auras items (Faith, Dream, Infinity) = separees, stackent.

### 1.6c Paladin -- Defensive Auras [S2]

Prayer(1,regen), ResistFire(1), Defiance(6,+def%), ResistCold(6), Cleansing(12,-dur poison/curse), ResistLightning(12), Vigor(18,+FRW [synBHammer+14%/pt]), Meditation(24,+mana regen -- Insight runeword -> [REF-03]), Redemption(30,consume corps->life/mana), Salvation(30,+all res slvl20:+60%).

### 1.7 Sorceress -- Fire [S1]

| Skill | Req | Mana@20 | Base@20 | Synergies |
|-------|-----|---------|---------|-----------|
| Fire Bolt | 1 | 4.5 | 118-132 | FBall/Meteor +14%/pt |
| Warmth | 1 | Passif | +162% mana regen | -- |
| Inferno | 6 | 12 | ~220/s | -- |
| Blaze | 12 | 18 | ~230 trail | -- |
| **Fire Ball** | 12 | 11 | 199-225 | FBolt/Meteor +14%/pt |
| Fire Wall | 18 | 22 | 466-503 DoT | -- |
| Enchant | 18 | 25 | +244-260 fire melee | -- |
| **Meteor** | 24 | 17 | 444-485 + brasier | FBolt/FBall +14%/pt |
| Fire Mastery | 30 | Passif | +306% fire dmg | -- |
| Hydra | 30 | 20 | ~134-157 per head (x3) | -- |

### 1.7b Sorceress -- Cold [S1]

| Skill | Req | Mana@20 | Base@20 | Synergies |
|-------|-----|---------|---------|-----------|
| Ice Bolt | 1 | 3.5 | 78-85 | Bliz/FO/GS +5%/pt |
| Frozen Armor | 1 | 7 | +200%def, freeze | -- |
| Frost Nova | 6 | 10 | 55-68 | Blizzard +5%/pt |
| Ice Blast | 6 | 7 | 119-130 | Bliz +5%/pt |
| Shiver Armor | 12 | 11 | +165%def | -- |
| Glacial Spike | 18 | 10 | 105-117 AoE freeze | Bliz +5%/pt |
| **Blizzard** | 24 | 42 | 570-619 | IB/IBl/GS +5%/pt each |
| Chilling Armor | 24 | 17 | Riposte ranged cold | -- |
| **Frozen Orb** | 30 | 34.5 | 262-277 + shards | IceBolt +2%/pt seulement |
| **Cold Mastery** | 30 | Passif | -165% cold res | NE brise PAS immunites |

**Blizzard valeurs par slvl** :

| slvl | Min-Max | Mana | +syn @20 IB+IBl+GS |
|------|---------|------|--------------------|
| 1 | 45-75 | 23 | +300% -> 180-300 |
| 10 | 210-249 | 32 | +300% -> 840-996 |
| 20 | 570-619 | 42 | +300% -> 2280-2476 |

**Frozen Orb valeurs par slvl** :

| slvl | Min-Max | Mana | +syn @20 IB |
|------|---------|------|-------------|
| 1 | 40-45 | 25 | +40% -> 56-63 |
| 10 | 134-144 | 29.5 | +40% -> 188-201 |
| 20 | 262-277 | 34.5 | +40% -> 367-387 |

### 1.7c Sorceress -- Lightning [S1]

| # | Skill | Req | Mana@20 | Effet | Notes |
|---|-------|-----|---------|-------|-------|
| 1 | Charged Bolt | 1 | 8 | Multi-bolts | Syn Lightning +4%/pt |
| 2 | **Static Field** | 6 | 9 | -25% HP courant | Cap N=0%, NM=33%, H=50% |
| 3 | Telekinesis | 6 | 7 | Knockback | syn E.Shield ratio |
| 4 | **Lightning** | 12 | 12 | 1-max dmg | slvl20: 1-717, syn CB +8%/pt |
| 5 | Nova | 12 | 21 | AoE foudre | -- |
| 6 | Chain Lightning | 18 | 13 | Bouncing | slvl20: 1-510 |
| 7 | **Teleport** | 18 | 24 | Instant move | Enigma -> tous -> [REF-03] |
| 8 | Thunder Storm | 24 | 19 | Eclairs auto | -- |
| 9 | **Energy Shield** | 24 | 5 | Mana absorbe dmg | slvl1=20%, slvl40+=95% |
| 10 | Lightning Mastery | 30 | Passif | +298% light dmg | -- |

E.Shield TK ratio : `mana_per_dmg = max(0.0625, 1.0 - 0.0625 * TK_slvl)`. Avec TK20 = 0.0625 mana/dmg.

---

## 2. Summon limits [S2]

-> MGE: `mge-arpg-skills::SummonLimits` dans TOML

| Type | Limite | Notes | -> MGE |
|------|--------|-------|--------|
| Skeletons/Mages (Necro) | 1+floor(slvl/3) | HP/dmg via Mastery | `SummonCap::Formula` |
| Revives | ~1/pt | 180s, gardent skills | `SummonCap::PerPoint` |
| Golem | 1 | Iron herite props item | `SummonCap::Fixed(1)` |
| Valkyrie (Ama) | 1 | Equip level-dependent | `SummonCap::Fixed(1)` |
| Shadow W/M (Asn) | 1 | Warrior=vos skills, Master=autonome | `SummonCap::Fixed(1)` |
| Spirit Wolf | 1-5 | -- | `SummonCap::PerPoint` |
| Dire Wolf | 1-3 | -- | `SummonCap::PerPoint` |
| Grizzly | 1 | -- | `SummonCap::Fixed(1)` |
| Spirit (Druid) | 1 total | Oak/HoW/SoB exclusifs | `SummonCap::Exclusive` |
| Vine (Druid) | 1 total | -- | `SummonCap::Exclusive` |
| Ravens | 1-5 | Frappes limitees | `SummonCap::PerPoint` |
| Traps (Asn) | 5 max | = missiles, pas summons | `SummonCap::Fixed(5)` |
