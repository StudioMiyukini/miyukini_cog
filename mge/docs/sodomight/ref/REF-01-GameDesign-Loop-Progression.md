# REF-01 -- Game Design, Boucle de Gameplay & Progression -- Reference Compacte
<!-- @id REF-01 @do reference-gamedesign @role Fabrice @layer 7 @human miyuki -->
<!-- Enrichi le 2026-03-03. Ajout: formules exactes, MGE mapping, sprint prio, cross-refs -->

**Projet** : Sodomight | **Source** : D2:LoD v1.14d / D2R 2.x

---

## 1. Piliers de design [S0]

| Pilier | Mecaniques cles | -> MGE |
|--------|-----------------|--------|
| Combat rapide | FHR/FCR/IAS breakpoints, latence < 100ms intention-feedback | `mge-arpg-combat` breakpoints TOML -> [REF-02 S4] |
| Loot addictif | Treasure Classes, MF, color coding (blanc<gris<bleu<jaune<vert<or) | `mge-arpg-loot` TC TOML -> [REF-03 S8] |
| Build diversity | 7 classes x 30 skills x synergies x items, breakpoints, runewords | `mge-arpg-skills` + `mge-arpg-items` -> [REF-02], [REF-03] |
| Social emergent | Trade, PvP, Baal runs, rushing, ladder economy | `mge-arpg-trade` + `mge-net` -> [REF-04 S3] |

## 2. Boucles de gameplay [S0]

**Micro-loop (seconde)** : Clic -> Action -> Feedback (anim+son+dmg) -> Resultat (recul/mort) -> Loot (bruit+couleur) -> Evaluation -> Decision (ramasser/ignorer) -> Repeat
-> MGE: `mge-arpg-combat::CombatLoop` + `mge-audio::SfxTrigger` + `mge-arpg-loot::DropEvent`

**Combat-loop (minute)** : Enter zone -> Scan pack -> Engage (5-15s combat) -> Loot phase -> Next pack. Rythme cible: 1 pack/10-20s, 1 elite/20-60s.
-> MGE: `mge-arpg-ai::PackSpawner` + `mge-arpg-world::ZoneTransition`

**Session-loop (heure)** : Choisir activite -> Executer runs -> Town phase (ID, stash) -> Evaluer -> Adapter

**Meta-loop (saison)** : Sem 1-2 leveling fresh -> Sem 3-4 farm Hell/builds endgame -> Mois 2-3 min-max/PvP -> Mois 4+ completionnisme -> Ladder reset

**Farming runs typiques** :

| Run | Duree | Runs/h | Drops cibles | Zone alvl |
|-----|-------|--------|-------------|-----------|
| Mephisto Hell | 1-3 min | 20-30 | TC78 (Shako, Oculus, War Traveler) | 83 |
| Pindle | 30-60s | 40-60 | TC87 (Griffon's, Death's Fathom) | 85 |
| Baal runs | 8-15 min | 4-6 | XP + TC87 | 85 |
| Chaos Sanctuary | 5-10 min | 6-10 | XP + runes + GC | 85 |
| Pit (alvl 85) | 3-5 min | 12-15 | TC87, safe all builds | 85 |
| Ancient Tunnels (alvl 85) | 3-5 min | 12-15 | TC87, no cold immune | 85 |
| Countess | 2-3 min | 15-20 | Runes (max Ist special, Lo normal) | 79 |
| Travincal Council | 1-2 min | 20-30 | High runes, gold | 82 |
| Lower Kurast (SP) | 30-60s | 40-60 | Super chests -> HR | 80 |

-> MGE: Chaque run = profil de recompense distinct dans `assets/data/farming_profiles.toml`

## 3. Difficultes -- Parametres cles [S1]

| Parametre | Normal | Nightmare | Hell | -> MGE |
|-----------|--------|-----------|------|--------|
| Resistance penalty joueur | 0 | -40% | -100% | `mge-arpg-stats::DifficultyModifiers.res_penalty` |
| Death XP penalty | 0% | 5% | 10% | `mge-arpg-stats::DifficultyModifiers.death_xp_loss` |
| XP recovery cadavre | N/A | 75% | 75% | `mge-arpg-stats::DeathRecovery` |
| Mods monstres uniques | 1 | 2 | 3 | `mge-arpg-ai::MonsterAffixRoll` -> [REF-04 S2.3] |
| Mods superuniques | 0 | 1 | 2 | idem |
| Skill levels monstres bonus | +0 | +3 | +7 | `mge-arpg-ai::MonsterSkillScale` |
| Life/Mana steal efficacite | 100% | 50% | 33% | `mge-arpg-combat::LeechPenalty` -> [REF-02 S1.8] |
| Hireling dmg vs Bosses | 50% | 35% | 25% | `mge-arpg-combat::HirelingDmgScale` |
| Static Field cap | 0% HP | 33% HP | 50% HP | `mge-arpg-skills::StaticFieldCap` |
| Immunites | Quasi-inexistantes | Rares | Generalisees | `mge-arpg-combat::ImmunityCheck` -> [REF-04 S2.6] |

-> MGE: `assets/data/difficulty.toml` -- TOUTES les valeurs ci-dessus = config data-driven, JAMAIS hardcode.

## 4. XP Table (1-99) [S1]

-> MGE: `mge-arpg-stats::XpTable` charge depuis `assets/data/xp_table.toml`

| Lvl | XP Totale | XP next | Lvl | XP Totale | XP next |
|-----|-----------|---------|-----|-----------|---------|
| 1 | 0 | 500 | 51 | 51,767,302 | 5,069,147 |
| 2 | 500 | 1,000 | 52 | 56,836,449 | 5,525,370 |
| 3 | 1,500 | 2,250 | 53 | 62,361,819 | 6,022,654 |
| 4 | 3,750 | 4,125 | 54 | 68,384,473 | 6,564,692 |
| 5 | 7,875 | 6,300 | 55 | 74,949,165 | 7,155,515 |
| 6 | 14,175 | 8,505 | 56 | 82,104,680 | 7,799,511 |
| 7 | 22,680 | 10,206 | 57 | 89,904,191 | 8,501,467 |
| 8 | 32,886 | 11,510 | 58 | 98,405,658 | 9,266,598 |
| 9 | 44,396 | 13,319 | 59 | 107,672,256 | 10,100,593 |
| 10 | 57,715 | 14,429 | 60 | 117,772,849 | 11,009,646 |
| 11 | 72,144 | 18,036 | 61 | 128,782,495 | 12,000,515 |
| 12 | 90,180 | 22,545 | 62 | 140,783,010 | 13,080,560 |
| 13 | 112,725 | 28,181 | 63 | 153,863,570 | 14,257,811 |
| 14 | 140,906 | 35,226 | 64 | 168,121,381 | 15,541,015 |
| 15 | 176,132 | 44,033 | 65 | 183,662,396 | 16,939,705 |
| 16 | 220,165 | 55,042 | 66 | 200,602,101 | 18,464,279 |
| 17 | 275,207 | 68,801 | 67 | 219,066,380 | 20,126,064 |
| 18 | 344,008 | 86,002 | 68 | 239,192,444 | 21,937,409 |
| 19 | 430,010 | 107,503 | 69 | 261,129,853 | 23,911,777 |
| 20 | 537,513 | 134,378 | 70 | 285,041,630 | 26,063,836 |
| 21 | 671,891 | 167,973 | 71 | 311,105,466 | 28,409,582 |
| 22 | 839,864 | 209,966 | 72 | 339,515,048 | 30,966,444 |
| 23 | 1,049,830 | 262,457 | 73 | 370,481,492 | 33,753,424 |
| 24 | 1,312,287 | 328,072 | 74 | 404,234,916 | 36,791,232 |
| 25 | 1,640,359 | 410,090 | 75 | 441,026,148 | 40,102,443 |
| 26 | 2,050,449 | 512,612 | 76 | 481,128,591 | 43,711,663 |
| 27 | 2,563,061 | 640,765 | 77 | 524,840,254 | 47,645,713 |
| 28 | 3,203,826 | 698,434 | 78 | 572,485,967 | 51,933,826 |
| 29 | 3,902,260 | 761,293 | 79 | 624,419,793 | 56,607,872 |
| 30 | 4,663,553 | 829,810 | 80 | 681,027,665 | 61,702,579 |
| 31 | 5,493,363 | 904,492 | 81 | 742,730,244 | 67,255,812 |
| 32 | 6,397,855 | 985,897 | 82 | 809,986,056 | 73,308,835 |
| 33 | 7,383,752 | 1,074,627 | 83 | 883,294,891 | 79,906,630 |
| 34 | 8,458,379 | 1,171,344 | 84 | 963,201,521 | 87,098,226 |
| 35 | 9,629,723 | 1,276,765 | 85 | 1,050,299,747 | 94,937,067 |
| 36 | 10,906,488 | 1,391,674 | 86 | 1,145,236,814 | 103,481,403 |
| 37 | 12,298,162 | 1,516,924 | 87 | 1,248,718,217 | 112,794,729 |
| 38 | 13,815,086 | 1,653,448 | 88 | 1,361,512,946 | 122,946,255 |
| 39 | 15,468,534 | 1,802,257 | 89 | 1,484,459,201 | 134,011,418 |
| 40 | 17,270,791 | 1,964,461 | 90 | 1,618,470,619 | 146,072,446 |
| 41 | 19,235,252 | 2,141,263 | 91 | 1,764,543,065 | 159,218,965 |
| 42 | 21,376,515 | 2,333,976 | 92 | 1,923,762,030 | 173,548,673 |
| 43 | 23,710,491 | 2,544,034 | 93 | 2,097,310,703 | 189,168,053 |
| 44 | 26,254,525 | 2,772,997 | 94 | 2,286,478,756 | 206,193,177 |
| 45 | 29,027,522 | 3,022,566 | 95 | 2,492,671,933 | 224,750,564 |
| 46 | 32,050,088 | 3,294,598 | 96 | 2,717,422,497 | 244,978,115 |
| 47 | 35,344,686 | 3,591,112 | 97 | 2,962,400,612 | 267,026,144 |
| 48 | 38,935,798 | 3,914,311 | 98 | 3,229,426,756 | 291,058,498 |
| 49 | 42,850,109 | 4,266,600 | 99 | 3,520,485,254 | --- |
| 50 | 47,116,709 | 4,650,593 | | | |

**XP totale level 99** : 3,520,485,254

## 5. Penalites XP [S1]

### 5.1 Par difference de niveau monstre

-> MGE: `mge-arpg-stats::XpPenalty` -- formules ci-dessous implementees en Rust, tables dans TOML

**Tier 1 (Clvl < 25)** :
```
delta = abs(Clvl - Mlvl)
XP_mult = max(5, 100 - 19*(delta - 5))    -- pour delta > 5, sinon 100%
```
+/-5 = 100%, +/-6 = 81%, +/-7 = 62%, +/-8 = 43%, +/-9 = 24%, +/-10+ = 5%

**Tier 2 (Clvl 25-69)** :
```
Monstres au-dessus : XP_eff = XP_base * (Clvl / Mlvl)
Monstres en-dessous : meme table que Tier 1
```

**Tier 3 (Clvl 70-99) -- Penalite haut niveau supplementaire** :
```
XP_divisor = 1024  -- 1024 = 100%
XP_eff = XP_base * (divisor_at_level / 1024)
```

| Lvl | %XP | /1024 | Lvl | %XP | /1024 | Lvl | %XP | /1024 |
|-----|-----|-------|-----|-----|-------|-----|-----|-------|
| 70 | 95.31 | 976 | 80 | 48.44 | 496 | 90 | 5.96 | 61 |
| 71 | 90.63 | 928 | 81 | 43.75 | 448 | 91 | 4.49 | 46 |
| 72 | 85.94 | 880 | 82 | 39.06 | 400 | 92 | 3.42 | 35 |
| 73 | 81.25 | 832 | 83 | 34.38 | 352 | 93 | 2.54 | 26 |
| 74 | 76.56 | 784 | 84 | 29.69 | 304 | 94 | 1.95 | 20 |
| 75 | 71.88 | 736 | 85 | 25.00 | 256 | 95 | 1.46 | 15 |
| 76 | 67.19 | 688 | 86 | 18.75 | 192 | 96 | 1.07 | 11 |
| 77 | 62.50 | 640 | 87 | 14.06 | 144 | 97 | 0.78 | 8 |
| 78 | 57.81 | 592 | 88 | 10.55 | 108 | 98 | 0.59 | 6 |
| 79 | 53.13 | 544 | 89 | 7.91 | 81 | | | |

### 5.2 XP en groupe [S2]

-> MGE: `mge-arpg-stats::PartyXp` + `mge-net::PartySystem`

```
XP_totale_monstre = XP_base * (n + 1) / 2
XP_par_membre = XP_totale * (Clvl_membre / Somme_Clvl_proches)
Bonus_meme_zone = +35%
Rayon_proximite = 53.33 yards (~2 ecrans)
```

| Joueurs | Multi XP total | XP vs Solo |
|---------|---------------|------------|
| 1 | 100% | 100% |
| 2 | 150% | 75% |
| 3 | 200% | 67% |
| 4 | 250% | 63% |
| 5 | 300% | 60% |
| 6 | 350% | 58% |
| 7 | 400% | 57% |
| 8 | 450% | 56% |

### 5.3 Penalite de mort [S1]

```
XP_perdue = XP_next_level * penalty%
XP_recoverable = XP_perdue * 0.75   -- toucher cadavre dans meme partie
XP_nette_perdue = XP_perdue * 0.25
```

| Difficulte | Perte | Recovery cadavre | Perte nette |
|------------|-------|-----------------|-------------|
| Normal | 0% | N/A | 0% |
| Nightmare | 5% next lvl | 75% | 1.25% |
| Hell | 10% next lvl | 75% | 2.5% |

Perte ne fait jamais perdre un niveau. Recovery uniquement dans la meme partie.

## 6. Stat/Skill points [S1]

-> MGE: `mge-arpg-stats::LevelUpReward` + `mge-arpg-quest::QuestReward`

- **Par level** : +5 stat, +1 skill
- **Total level 99 (sans quetes)** : 495 stat, 98 skill

### Quetes permanentes (x3 difficultes)

| Quete | Acte | Reward | Total x3 | -> MGE |
|-------|------|--------|----------|--------|
| Den of Evil | 1 | +1 skill + 1 respec | 3 skill + 3 respec | `mge-arpg-quest::DenReward` |
| Radament's Lair | 2 | +1 skill | 3 skill | `mge-arpg-quest::RadamentReward` |
| Izual (Fallen Angel) | 4 | +2 skill | 6 skill | `mge-arpg-quest::IzualReward` |
| Lam Esen's Tome | 3 | +5 stat | 15 stat | `mge-arpg-quest::LamEsenReward` |
| Golden Bird | 3 | +20 Max Life | +60 Life | `mge-arpg-quest::GoldenBirdReward` |
| Prison of Ice | 5 | +10 All Res | +30 All Res | `mge-arpg-quest::AnyaReward` -> [REF-04 S1.7] |
| Tools of the Trade | 1 | Imbue 1 item | 3 imbues | `mge-arpg-quest::ImbueReward` -> [REF-03 S4] |
| Hell's Forge | 4 | Gems + rune (El-Sol/Sol-Um/Hel-Gul) | 3 forges | `mge-arpg-quest::ForgeReward` -> [REF-03 S2] |
| Siege of Harrogath | 5 | Socket 1 item | 3 sockets | `mge-arpg-quest::LarzukReward` -> [REF-03 S1] |
| Rescue Mt Arreat | 5 | Ral+Ort+Tal | 3x3 runes | `mge-arpg-quest::RescueReward` |

**Totaux max** : 510 stat points, 110 skill points

## 7. Magic Find & Treasure Classes [S2]

-> MGE: `mge-arpg-loot::MagicFind` + `mge-arpg-loot::TreasureClass`
-> Cross-ref: [REF-03 S8-9] pour details loot system complet

### Formules MF (rendements decroissants)

```
EffMF_unique  = floor(MF * 250 / (MF + 250))
EffMF_set     = floor(MF * 500 / (MF + 500))
EffMF_rare    = floor(MF * 600 / (MF + 600))
EffMF_magic   = MF                              -- lineaire, pas de DR
```

| MF | Magic | Rare | Set | Unique |
|----|-------|------|-----|--------|
| 50 | 50% | 46% | 45% | 42% |
| 100 | 100% | 86% | 83% | 71% |
| 200 | 200% | 150% | 143% | 111% |
| 300 | 300% | 200% | 188% | 136% |
| 400 | 400% | 240% | 222% | 154% |
| 500 | 500% | 273% | 250% | 167% |
| 700 | 700% | 323% | 292% | 184% |
| 1000 | 1000% | 375% | 333% | 200% |

Sweet spot : **250-350% MF**. Au-dela, rendements negligeables sur uniques.
N'affecte PAS : quantite drops, runes, gems, gambling, tier Exc/Elite.

### Treasure Classes (29 paliers : TC3 -> TC87)

| TC | Items | Ou (Hell) | -> MGE |
|----|-------|-----------|--------|
| TC3-24 | Normal bases | Partout | `assets/data/treasure_classes.toml` |
| TC27-54 | Exceptional | Act 1-3 Hell | idem |
| TC63-75 | Elite mid | Mephisto, Act Bosses | idem |
| TC78-84 | Elite haut | Baal, Diablo | idem |
| TC87 | Elite top (tout droppable) | Zones alvl 85 uniquement | idem |

Champions : TC area+2, +2 picks. Uniques : TC area+3, +3-4 picks.

## 8. Area Levels complets [S1]

-> MGE: `mge-arpg-world::AreaLevel` charge depuis `assets/data/areas.toml`
-> Cross-ref: [REF-04 S1.2] pour zones detaillees avec quetes/WP

### Zones alvl 85 Hell (TC87 droppable)

| Zone | Acte | Immunites predominantes | Builds recommandes |
|------|------|------------------------|-------------------|
| The Pit L1-2 | 1 | Fire/Light/Cold | Hammerdin, Javazon |
| Mausoleum | 1 | Light | Blizzard Sorc, Hammerdin |
| Ancient Tunnels | 2 | Pois/Light/Fire (PAS Cold) | Blizzard Sorc |
| Maggot Lair L3 | 2 | Varies | Hammerdin (tight) |
| Sewers L2 (A3) | 3 | Varies | All |
| Forgotten Temple | 3 | Varies | All |
| Ruined Fane | 3 | Varies | All |
| Disused Reliquary | 3 | Varies | All |
| River of Flame | 4 | Fire/Light | Hammerdin, Fishymancer |
| Chaos Sanctuary | 4 | Cold/Fire/Light | Hammerdin (meta) |
| WSK L1-3 | 5 | Varies (tout) | Hammerdin, Javazon |
| Throne of Destruction | 5 | Varies | Hammerdin, Javazon |
| Worldstone Chamber | 5 | Varies | All |

### Recapitulatif Area Levels par acte (Hell)

| Acte | Range alvl | alvl 85 zones |
|------|-----------|---------------|
| 1 | 67-85 | Pit, Mausoleum |
| 2 | 74-85 | Ancient Tunnels, Maggot L3 |
| 3 | 79-85 | Sewers L2, Forgotten T., Ruined F., Disused R. |
| 4 | 82-85 | River of Flame, Chaos Sanctuary |
| 5 | 80-85 | WSK 1-3, Throne, Worldstone Chamber |

Table alvl complete par zone : voir [REF-04 S1.2].

## 9. XP shrines & Players X [S2]

-> MGE: `mge-arpg-stats::XpShrineBonus` + `mge-arpg-world::PlayerDifficulty`

| Source | Bonus | Duree | -> MGE |
|--------|-------|-------|--------|
| Experience Shrine | +50% | 144s solo, /n joueurs en multi | `mge-arpg-world::Shrine::Experience` |
| Annihilus charm | +5-10% permanent | Permanent | `mge-arpg-items::CharmPassive` -> [REF-03 S7] |
| Ondal's Wisdom staff | +5% equipe | Equipe | `mge-arpg-items::UniqueBonus` |

**Players X (SP)** :
```
XP_mult = (PlayersSetting + 1) / 2
HP_mult = (PlayersSetting + 1) / 2
```

| /pX | XP mult | HP mult | Optimal pour |
|-----|---------|---------|--------------|
| /p1 | 100% | 100% | Speed farming, Countess |
| /p3 | 200% | 200% | Balance XP/speed |
| /p5 | 300% | 300% | XP grind endgame |
| /p7 | 400% | 400% | Max NoDrop reduction, sweet spot |
| /p8 | 450% | 450% | Max HP/XP, durability test |

## 10. Grind 95-99 [S3]

| De-A | XP requise | % XP totale | Estimation | Meilleur spot |
|------|-----------|-------------|-----------|---------------|
| 1-85 | 1,050M | 29.8% | 20-40h | Progression normale |
| 85-90 | 568M | 16.1% | 50-100h | Baal/Chaos /p5-7 |
| 90-95 | 874M | 24.8% | 200-400h | Baal runs /p7-8 |
| 95-99 | 1,028M | 29.2% | 500-1000h+ | Baal runs /p8 |
| 98-99 | 291M | 8.3% | 200-400h | Baal runs /p8 |

## 11. Ladder/Saisons [S3]

-> MGE: `mge-save::LadderSystem` + `mge-net::SeasonManager`

- Duree saison : ~4 mois (configurable `assets/data/ladder.toml`)
- Ladder reset transfere personnages ladder -> non-ladder
- Contenu ladder-only : runewords exclusifs (deviennent non-ladder en fin de saison)
- Terror Zones (D2R) : zones rotatives alvl = Clvl (+2 champ, +5 unique)
```
TerrorZone.alvl_normal = max(zone_alvl, Clvl)
TerrorZone.alvl_champion = alvl_normal + 2
TerrorZone.alvl_unique = alvl_normal + 5
```

## 12. Level requirements items (fourchettes) [S2]

-> Cross-ref: [REF-03 S3] pour runewords detaillees, [REF-03 S1] pour items

| Categorie | rlvl | Exemples |
|-----------|------|----------|
| Normal uniques | 1-30 | Sigon's (6), Angelic (12) |
| Exceptional uniques | 25-50 | Vipermagi (29), Goldwrap (27) |
| Elite uniques | 42-77 | Shako (62), Arachnid (80), Griffon's (76) |
| Low runewords | 13-27 | Spirit (25), Insight (27), Lore (27) |
| Mid runewords | 35-55 | Smoke (43), Treachery (43), Duress (47) |
| High runewords | 47-67 | Enigma (65), Grief (59), Infinity (63), CoH (63) |
