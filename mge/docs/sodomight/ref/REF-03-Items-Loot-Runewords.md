# REF-03 -- Items, Loot & Runewords -- Reference Compacte
<!-- @id REF-03 @do reference-items @role Fabrice @layer 7 @human miyuki -->
<!-- Enrichi le 2026-03-03. Ajout: formules exactes, MGE mapping, sprint prio, cross-refs -->

**Projet** : Sodomight | **Source** : D2:LoD v1.14d / D2R 2.8

---

## 1. Systeme d'items [S1]

-> MGE: `mge-arpg-items` + `assets/data/item_bases.toml`

**Inventaire** : grille 10x4=40 slots. -> MGE: `mge-arpg-items::Inventory { grid: Grid<10,4> }`, collision O(1) bitfield 40 bits.
-> Cross-ref: [REF-05 S1.5] pour UI inventaire

**Tailles items** :

| Taille | Items |
|--------|-------|
| 1x1 | Rings, gems, runes, potions, SC, scrolls, keys, jewels |
| 1x2 | Gloves, boots, belts, LC |
| 1x3 | Javelins, wands, scepters, GC, amulets |
| 1x4 | 1H swords, daggers |
| 2x2 | Helms, small shields |
| 2x3 | Armors, large shields, bows (inv) |
| 2x4 | Polearms, staves, 2H swords |

**Tiers** : Normal / Exceptional / Elite (3 par base, stats/reqs differents).
**Ethereals** : 5% chance, +50% def/dmg, non reparable (sauf Zod/self-repair). Exclusions : Phase Blade, Set items, achetes/cubed.
-> MGE: `mge-arpg-items::ItemBase { tier, ethereal_chance: 0.05 }`

### Qualites [S1]

| Qualite | Couleur | Affixes | -> MGE |
|---------|---------|---------|--------|
| Normal/Superior | Blanc | 0 (Sup: +1-15%ED) | `Quality::Normal/Superior` |
| Magic | Bleu | 1-2 | `Quality::Magic` |
| Rare | Jaune | 2-6 | `Quality::Rare` |
| Crafted | Orange | 3-8 | `Quality::Crafted` |
| Set | Vert | Fixes + bonus ensemble | `Quality::Set` |
| Unique | Or | Fixes | `Quality::Unique` |

### Sockets & Affixes [S1]

1/3 normal/superior = socketed. Larzuk = max(normal), 1(rare/unique/set).
```
Socket max = ilvl<=25 ? MaxSock1 : ilvl<=40 ? MaxSock25 : MaxSock40
alvl = magic_lvl>0 ? ilvl+magic_lvl : ilvl<(99-qlvl/2) ? ilvl-qlvl/2 : 2*ilvl-99 (cap 99)
```
Magic: 25%pre+suf, 25%pre, 50%suf. Rare: 2-6, 1/4 chance par extra. Crafted ilvl71+: 4 random.
-> MGE: `mge-arpg-items::SocketCalc` + `mge-arpg-loot::AffixGen`

### Types armes (sock max) [S1]

| Type | Max Sockets | -> MGE: `assets/data/item_bases.toml` |
|------|-------------|------|
| Swords/Axes/Bows/Xbows/Polearms/Spears/Staves | 6 | `max_sockets = 6` |
| Scepters | 5 | `max_sockets = 5` |
| Maces (varies) | 3-6 | Depends on base |
| Daggers/Claws/Orbs | 3 | `max_sockets = 3` |
| Wands | 2 | `max_sockets = 2` |
| Helms | 3 | `max_sockets = 3` |
| Body Armor/Shields | 4 | `max_sockets = 4` |
| Gloves/Boots/Belts | 0 | Non-socketable |

### Gambling [S2]

```
ilvl = clvl + random(-5, +4)
```
MF N'AFFECTE PAS le gambling. Unique: 1/2000, Set: 1/1000, Rare: 1/10, Magic: ~89.85%.
-> MGE: `mge-arpg-items::GamblingRoll`

---

## 2. Les 33 Runes [S2]

-> MGE: `mge-arpg-items::Rune` + `assets/data/runes.toml`

| # | Rune | Clvl | Arme | Armure | Bouclier |
|---|------|------|------|--------|----------|
| 1 | El | 11 | +50AR +1Light | +15Def +1Light | +15Def +1Light |
| 2 | Eld | 11 | +75%/+50AR vs Undead | -15%Stam | +7%Block |
| 3 | Tir | 13 | +2 Mana/kill | +2 Mana/kill | +2 Mana/kill |
| 4 | Nef | 13 | Knockback | +30Def vs Missile | +30Def vs Missile |
| 5 | Eth | 15 | -25%TargetDef | RegenMana 15% | RegenMana 15% |
| 6 | Ith | 15 | +9MaxDmg | 15%Dmg->Mana | 15%Dmg->Mana |
| 7 | Tal | 17 | 75Poison/5s | +30%PoisRes | +35%PoisRes |
| 8 | Ral | 19 | +5-30Fire | +30%FireRes | +35%FireRes |
| 9 | Ort | 21 | +1-50Light | +30%LightRes | +35%LightRes |
| 10 | Thul | 23 | +3-14Cold/3s | +30%ColdRes | +35%ColdRes |
| 11 | Amn | 25 | 7%LL | AtkTakes14 | AtkTakes14 |
| 12 | Sol | 27 | +9MinDmg | -7DmgTaken | -7DmgTaken |
| 13 | Shael | 29 | +20IAS | +20FHR | +20FBR |
| 14 | Dol | 31 | 25%MonFlee | +7RepLife | +7RepLife |
| 15 | Hel | -- | -20%Req | -15%Req | -15%Req |
| 16-19 | Io/Lum/Ko/Fal | 35-41 | +10 VIT/ENE/DEX/STR | idem | idem |
| 20 | Lem | 43 | +75%Gold | +50%Gold | +50%Gold |
| 21 | Pul | 45 | +75%/+100AR vs Demons | +30%Def | +30%Def |
| 22 | Um | 47 | 25%OW | +15%AllRes | +22%AllRes |
| 23 | Mal | 49 | PMH | -7MagicDmg | -7MagicDmg |
| 24 | Ist | 51 | +30%MF | +25%MF | +25%MF |
| 25 | Gul | 53 | +20%AR | +5MaxPoisRes | +5MaxPoisRes |
| 26 | Vex | 55 | 7%ML | +5MaxFireRes | +5MaxFireRes |
| 27 | Ohm | 57 | +50%ED | +5MaxColdRes | +5MaxColdRes |
| 28 | Lo | 59 | 20%DS | +5MaxLightRes | +5MaxLightRes |
| 29 | Sur | 61 | 20%HitBlinds | +5%Mana | +50Mana |
| 30 | Ber | 63 | 20%CB | DR8% | DR8% |
| 31 | Jah | 65 | ITD | +5%Life | +50Life |
| 32 | Cham | 67 | 32%Freeze/3s | CBF | CBF |
| 33 | Zod | 69 | Indestructible | Indestructible | Indestructible |

**Upgrade** : #1-9=3->+1 | #10-20=3+gem->+1 | #21-33=2+gem->+1.
**Rarete** : El-Ral=commun, Ort-Dol=peu commun, Hel-Lem=mid, Pul-Gul=rare, Vex-Lo=tres rare, Sur-Zod=ultra (Zod~1/500k).
**Countess** : 2 tables (normal+rune). Hell max: Ist(special)/Lo(normal). Optimal=players 1.

**Economie runes (valeur relative)** -> [REF-04 S3.4] :

| Rune | Valeur | Rune | Valeur |
|------|--------|------|--------|
| Ist | 1x | Lo | 8-12x |
| Gul | 2x | Ber | 15-25x |
| Vex | 4x | Jah | 15-25x |
| Ohm | 6-8x | Cham | 2-3x |
| Sur | 8-10x | Zod | 2-4x |

---

## 3. Runewords [S2]

-> MGE: `mge-arpg-items::RunewordDetection` + `assets/data/runewords.toml`
Regles : base Normal/Superior, sockets=nb runes exact, ordre exact, type autorise.

### Budget (Clvl 13-29) [S1]

| Nom | Runes | Types | Clvl | Stats cles |
|-----|-------|-------|------|-----------|
| Steel | TirEl | Swd/Axe/Mace | 13 | 20%ED, 25%IAS, 50%OW |
| Nadir | NefTir | Helms | 13 | 50%ED, CloakOfShadows L13 |
| Malice | IthElEth | Melee | 15 | 33%ED, 100%OW, PMH |
| **Stealth** | TalEth | Armor | 17 | 25%FRW/FCR/FHR |
| Leaf | TirRal | Staves | 19 | +3Fire, +3FBolt/Inferno/Warmth |
| **Ancient's Pledge** | RalOrtTal | Shields | 21 | AllRes+43-48 |
| Zephyr | OrtEth | Missile | 21 | 33%ED, 25%FRW/IAS |
| Edge | TirTalAmn | Missile | 25 | ThornsL15, -15%Vendor |
| Strength | AmnTir | Melee | 25 | 35%ED, 25%CB, +20STR |
| Honor | AmnElIthTirSol | Melee | 27 | 160%ED, 25%DS, +1All |
| **Lore** | OrtSol | Helms | 27 | +1All, 30%LightRes |
| **Insight** | RalTirTalSol | Pole/Staves | 27 | MeditationL12-17, 200-260%ED |
| Rhyme | ShaelEth | Shields | 29 | 40%FBR, AllRes+25, CBF, 25%MF |

### Mid (Clvl 35-49) [S2]

| Nom | Runes | Types | Clvl | Stats cles |
|-----|-------|-------|------|-----------|
| Black | ThulIoNef | Club/Ham/Mace | 35 | 40%CB, 120%ED |
| White | DolIo | Wands | 35 | +3P&B, 20%FCR |
| Smoke | NefLum | Armor | 37 | AllRes+50, 20%FHR |
| Lionheart | HelLumFal | Armor | 41 | +25STR +20VIT AllRes+30 |
| Obedience | HelKoThulEthFal | Pole/Spears | 41 | 370%ED, 40%CB |
| **Treachery** | ShaelThulLem | Armor | 43 | +2Asn, 45%IAS, 5%Fade |
| Wealth | LemKoTir | Armor | 43 | 300%Gold, 100%MF |
| Crescent Moon | ShaelUmTir | Axe/Swd/Pole | 47 | -35%EnemyLightRes |
| Duress | ShaelUmThul | Armor | 47 | 150-200%ED, 15%CB |
| Oath | ShaelPulMalLum | Swd/Axe/Mace | 49 | Indestr, 210-340%ED |

Aussi : Splendor(37), Memory(37), Melody(39), Harmony(39), Lawbringer(43), Passion(43), Gloom(47), Stone(47), Venom(49).

### High (Clvl 51-69) [S2]

| Nom | Runes | Types | Clvl | Stats cles | Valeur eco |
|-----|-------|-------|------|-----------|-----------|
| **HotO** | KoVexPulThul | Staves/Maces | 55 | +3All, 40%FCR, AllRes+30-40 | ~5 Ist |
| **CTA** | AmnRalMalIstOhm | Weapons | 57 | +1All, +2-6BO/BC | ~8 Ist |
| **Fortitude** | ElSolDolLo | Wpn/Armor | 59 | 300%ED(w)/200%ED(a), AllRes+25-30 | ~10 Ist |
| **Grief** | EthTirLoMalRal | Swd/Axe | 59 | +340-400 FLAT dmg, ITD, 20%DS | ~10 Ist |
| Beast | BerTirUmMalLum | Axe/Scep/Ham | 63 | FanaticismL9, 240-270%ED | ~20 Ist |
| **CoH** | DolUmBerIst | Armor | 63 | +2All, AllRes+65, DR8%, 25%MF | ~20 Ist |
| **Infinity** | BerMalBerIst | Pole/Spears | 63 | ConvictionL12, -45-55%EnemyLightRes | ~35 Ist |
| **Enigma** | JahIthBer | Armor | 65 | +2All, +1Teleport, 45%FRW, DR8% | ~35 Ist |
| Faith | OhmJahLemEld | Missile | 65 | FanaticismL12-15, 330%ED | ~25 Ist |
| Dream | IoJahPul | Helms/Shields | 65 | HolyShockL15 | ~20 Ist |
| BotD | VexHelElEldZodEth | Weapons | 69 | Indestr, 350-400%ED, 12-15%LL | ~10 Ist |

Aussi : Delirium(51), Rift(53), Kingslayer(53), Silence(55), Death(55), Chaos(57), Exile(57), Wind(61), Bramble(61), Dragon(61), Brand(65), Ice(65), LastWish(65), Phoenix(65), Doom(67), Pride(67).

---

## 4. Horadric Cube [S2]

-> MGE: `mge-arpg-items::CubeRecipe` + `assets/data/cube_recipes.toml`
-> Cross-ref: [REF-04 S1.7] pour quetes liees

### Upgrade

| Direction | Unique (Runes+Gem) | Rare (Runes+Gem) |
|-----------|-------------------|-----------------|
| NormWpn->Exc | Ral+Sol+P.Emerald | Ort+Amn+P.Sapphire |
| NormArm->Exc | Tal+Shael+P.Diamond | Ral+Thul+P.Amethyst |
| ExcWpn->Elite | Lum+Pul+P.Emerald | Fal+Um+P.Sapphire |
| ExcArm->Elite | Ko+Lem+P.Diamond | Ko+Pul+P.Amethyst |

### Sockets

| Base | Recipe | Result |
|------|--------|--------|
| Armor(Norm) | Tal+Thul+P.Topaz | 1-4 sockets |
| Weapon(Norm) | Ral+Amn+P.Amethyst | 1-6 sockets |
| Helm(Norm) | Ral+Thul+P.Sapphire | 1-3 sockets |
| Shield(Norm) | Tal+Amn+P.Ruby | 1-4 sockets |
| Remove sockets | Hel+TPScroll+item | Vide (contenu detruit) |

### Autres recettes

| Type | Recipe | Notes |
|------|--------|-------|
| Repair weapon | Ort+Wpn | — |
| Repair armor | Ral+Arm | — |
| Reroll magic | 3P.Gems+MagicItem | ilvl preserve |
| Reroll rare | 6P.Skulls+RareItem | ilvl=40%clvl+40%ilvl |
| Convert rings->amu | 3MagicRings | ilvl=75%clvl |
| Convert amu->ring | 3MagicAmu | ilvl=75%clvl |
| Cow Level | Wirt'sLeg+TPTome | A1 Rogue Enc |
| Pandemonium | 3 Keys (T/H/D) | Uber portals |
| Uber Tristram | 3 Organs | Hellfire Torch |
| Token of Absolution | 4 Essences | Full respec |

---

## 5. Crafting [S3]

-> MGE: `mge-arpg-items::CraftingRecipe` + `assets/data/crafting.toml`

```
craft_ilvl = floor(clvl/2) + floor(ilvl/2)
Optimal : clvl >= 93 + ilvl >= 49 -> craft_ilvl >= 71 -> 4 random affixes
```

| Cat. | P.Gem | Garantis | Rune (varies par slot) |
|------|-------|----------|----------------------|
| Blood | Ruby | 1-4%LL, +10-20Life | Wpn=Ort, Shield=Ith, Helm=Ral, Armor=Thul |
| Caster | Amethyst | 4-10%MRegen, +10-20Mana | Belt=Ith, Amu=Ral, Ring=Amn, Gloves=Ort |
| Hitpower | Sapphire | 5%FrostNova, AtkTakes3-10 | — |
| Safety | Emerald | 10-30%ED, 1-4DR, 1-2MDR | — |

---

## 6. Gemmes (7x5=35) [S2]

-> MGE: `mge-arpg-items::Gem` + `assets/data/gems.toml`
Upgrade: 3 identiques -> +1 qualite. Effets Perfect :

| Gem | Arme | Armure | Bouclier |
|-----|------|--------|----------|
| Amethyst | +150AR | +10STR | +30Def |
| Diamond | +68%vsUndead +100AR | +19%AllRes | +19%AllRes |
| Emerald | +29Poison/1s | +10DEX | +40%PoisRes |
| Ruby | +15-20Fire | +38Life | +40%FireRes |
| Sapphire | +10-14Cold/3s | +38Mana | +40%ColdRes |
| Topaz | +1-40Light | +24%MF | +40%LightRes |
| Skull | 4%LL/3%ML | +5Life 19%MRegen | AtkTakes20 |

## 7. Charmes & Joyaux [S2]

-> MGE: `mge-arpg-items::CharmPassive` + `assets/data/charms.toml`

SC(1x1), LC(1x2), GC(1x3) : 1-2 affixes. Actifs dans inventaire uniquement.

| Charm special | Taille | Source | Stats | Limite |
|--------------|--------|--------|-------|--------|
| **Annihilus** | SC | Uber Diablo | +1All, +10-20Stats, AllRes+10-20, +5-10%XP | 1/perso |
| **Hellfire Torch** | LC | Uber Tristram | +3class, +10-20Stats, AllRes+10-20 | 1/perso |
| **Gheed's Fortune** | GC | Drop | 80-160%Gold, 20-40%MF, -10-15%Vendor | 1/perso |

**Rainbow Facets** (jewels) : +3-5%SkillDmg, -3-5%EnemyRes. 4 elem x 2 variantes (Die/LvlUp) = 8.
-> Cross-ref: [REF-01 S9] pour XP bonus Annihilus

---

## 8. Loot System [S1]

-> MGE: `mge-arpg-loot` -- TOUT le loot = data-driven TOML, JAMAIS hardcode
-> Cross-ref: [REF-01 S7] pour MF formules, [REF-04 S2.1] pour TC monstres

### TC & Selection

29 paliers (TC3-TC87 par 3). Chaque monstre -> TC. Par pick: roll NoDrop vs items, recurse si sous-TC.

**NoDrop formula (players scaling)** :
```
N = 1 + floor(AddPlayers/2) + floor(ClosePartied/2)
NewNoDrop = floor(ProbSum / (1/((NoDrop/(NoDrop+ProbSum))^N) - 1))
```

| Players | Drop mult vs p1 |
|---------|-----------------|
| 1 | 100% (base) |
| 3 | +50% |
| 5 | +70% |
| 7 | +85% (sweet spot farming) |
| 8 | +85% (meme que 7) |

-> MGE: `mge-arpg-loot::NoDropCalc`

### Cascade qualite [S1]

```
Roll order : Unique -> Set -> Rare -> Magic -> Superior -> Normal
Chance = (BaseChance - (ilvl - qlvl) / Divisor) * 128
EffMF = MF * Factor / (MF + Factor)    -- U:250, S:500, R:600, M:lineaire
FinalChance = Chance * 100 / (100 + EffMF) - Chance * QF / 1024
random < 128 -> SUCCES sinon -> suivant
```
Fallback : Unique sans candidat -> Rare 3x durabilite. Set sans candidat -> Magic 2x durabilite.
-> MGE: `mge-arpg-loot::QualityCascade`

### Boss drops (Hell) [S1]

| Boss | mlvl | TC | HP Hell | Quest drop | Notes |
|------|------|----|---------|------------|-------|
| Andariel | 75 | Act1 | 60,031 | Oui (quest bug = perma) | Fire weakness -50% |
| Duriel | 88 | Act2 | 84,524 | Oui | Stun-lock melee |
| Mephisto | 87 | TC78 | 94,320 | Oui | Moat trick |
| Diablo | 94 | TC84 | 113,812 | Oui | Multi-phase |
| Baal | 99 | TC87 | 493,701 | Oui | Seul TC87 natif |

-> Cross-ref: [REF-04 S2.5] pour boss patterns complets

### ilvl

```
Normal monster : ilvl = mlvl
Champion : ilvl = area_level + 2
Unique : ilvl = area_level + 3
Shop : ilvl = clvl + 5
Gambling : ilvl = clvl + random(-5, +4)
```

### Staffmods [S3]

Wands/Scepters/Staves/Claws. Tier par ilvl (1-11->T1, 12-19->T2, 20-26->T3, 27-36->T4, 37+->T5). 0-3 skills +1/+2/+3.
-> MGE: `mge-arpg-loot::StaffmodGen`

---

## 9. Magic Find [S1]

-> MGE: `mge-arpg-loot::MagicFind`
-> Cross-ref: [REF-01 S7] pour tables detaillees

```
EffMF_unique = MF * 250 / (MF + 250)
EffMF_set    = MF * 500 / (MF + 500)
EffMF_rare   = MF * 600 / (MF + 600)
EffMF_magic  = MF                        -- lineaire
```

Optimal: **250-350%**. N'affecte PAS : quantite drops, runes, gems, gambling, tier Exc/Elite.

---

## 10. Zones level 85 (Hell) [S1]

-> Cross-ref: [REF-01 S8] pour table complete, [REF-04 S1.2] pour zones avec quetes

Seules zones ou TOUS items peuvent dropper (normal mlvl85->TC87, champ87, unique88).

| Zone | Act | Immunites | Meilleur pour |
|------|-----|-----------|--------------|
| Mausoleum | 1 | Light | Blizz Sorc |
| The Pit L1-2 | 1 | Fire/Light/Cold | Hammerdin, Javazon |
| Ancient Tunnels | 2 | Pois/Light/Fire (PAS Cold) | Blizz Sorc (meta) |
| Chaos Sanctuary | 4 | Cold/Fire/Light | Hammerdin (meta) |
| WSK L1-3 + Throne | 5 | Varies | All |

---

## 11. MGE Implementation Summary

| Systeme | Crate | TOML data | Sprint |
|---------|-------|-----------|--------|
| Item bases/tiers | `mge-arpg-items` | `item_bases.toml` | S1 |
| Socket/affix gen | `mge-arpg-items` | `affixes.toml` | S1 |
| Inventory grid | `mge-arpg-items` | — (runtime) | S1 |
| Quality cascade | `mge-arpg-loot` | `loot_config.toml` | S1 |
| TC system | `mge-arpg-loot` | `treasure_classes.toml` | S1 |
| NoDrop scaling | `mge-arpg-loot` | `loot_config.toml` | S2 |
| Runes | `mge-arpg-items` | `runes.toml` | S2 |
| Runewords | `mge-arpg-items` | `runewords.toml` | S2 |
| Gems | `mge-arpg-items` | `gems.toml` | S2 |
| Charms/Jewels | `mge-arpg-items` | `charms.toml` | S2 |
| Cube recipes | `mge-arpg-items` | `cube_recipes.toml` | S2 |
| Crafting | `mge-arpg-items` | `crafting.toml` | S3 |
| Gambling | `mge-arpg-items` | `gambling.toml` | S2 |
| Staffmods | `mge-arpg-loot` | `staffmods.toml` | S3 |
| Unique/Set items | `mge-arpg-items` | `unique_items.toml`, `set_items.toml` | S2 |
| MF calc | `mge-arpg-loot` | — (formula) | S1 |

**Pieges** : (1) TC data-driven JAMAIS hardcode (2) Cascade qualite=params par etape (3) MF DR obligatoires (4) Gems 3 tables weapon/armor/shield (5) NoDrop player scaling (6) Ethereal non-reparable=equilibre (7) Runeword ordre strict + type strict (8) Fallbacks U->R3x/S->M2x.
