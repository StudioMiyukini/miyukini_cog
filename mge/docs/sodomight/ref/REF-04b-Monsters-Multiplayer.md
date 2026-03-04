# REF-04b -- Monstres, Boss, IA & Multiplayer -- Reference Compacte
<!-- @id REF-04b @do reference-monsters-multiplayer @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-04 le 2026-03-03. Formules D2, valeurs exactes, MGE mapping, sprints, cross-refs -->

**Projet** : Sodomight | **Moteur** : MGE | **Ref D2** : v1.14d / D2R 2.x

> **TL;DR** : Systeme complet des monstres D2 : formules HP, classification (normal/champion/unique/boss), 62 Super Uniques, stats et attaques des 5 Act Bosses, immunites Hell et breaking, archetypes IA (FSM), mercenaires, 16 familles de monstres. Plus : multiplayer (8 joueurs, XP sharing, player count scaling, NoDrop, economie runes). Tout mappe vers `mge-arpg-entity`, `mge-arpg-ai`, `mge-arpg-combat` et `mge-net`.

> **Voir aussi** : [REF-04a](REF-04a-World-Zones.md) pour les zones, area levels, waypoints et generation procedurale.

---

## 1. Monstres & IA `[S1-S2]`

> `-> MGE: mge-arpg-entity::MonsterDefinition, mge-arpg-ai::AiBehavior` | TOML: `assets/data/monsters/`

### 1.1 Formule HP Monstres `[S1]`

> `-> MGE: mge-arpg-stats::MonsterHp` | TOML: `assets/data/monsters/monlvl.toml`, `assets/data/monsters/monstats.toml`

```
HP = monlvl_base_hp[mlvl][diff] * rand(MinHP%, MaxHP%) / 100
```

- `monlvl_base_hp` : table indexee par mlvl et difficulte (ex: mlvl 88 Hell = 14685 base)
- `MinHP%/MaxHP%` : par monstre dans monstats.toml (ex: Hell Bovine = 440-540%)
- Resultat : Hell Bovine = 14685 * 440-540% = **64,614 - 79,229 HP**
- Difficulte scaling base : N=x1.0, NM=x1.75, H=x1.5 (applique sur monlvl table)

**Exemples monlvl base HP (Hell)** :

| mlvl | Base HP (Hell) | Exemples zones |
|------|----------------|----------------|
| 67 | ~3,200 | Blood Moor H |
| 75 | ~5,800 | Rocky Waste H |
| 81 | ~9,200 | Cow Level H |
| 85 | ~12,500 | alvl85 zones H |
| 88 | 14,685 | WSK/Throne H |

### 1.2 Classification `[S1]`

> `-> MGE: mge-arpg-entity::MonsterRank` | Cross-ref: **REF-03 SS2.1** (TC bonus)

| Categorie | Couleur | TC Bonus | HP mult (vs base) | Drop bonus |
|-----------|---------|----------|--------------------|------------|
| Normal | Blanc | -- | x1 | Standard |
| Champion | Bleu (groupe 3) | TC+2 | x2-x12 (voir SS1.3) | +NoDrop reduction |
| Unique/Rare | Dore + minions | TC+3 | Variable | +NoDrop reduction |
| Super Unique | Dore (nom fixe) | TC specifique | Specifique | TC table propre |
| Act Boss | Special | TC propre, quest drops | Tres eleve | Quest drop garanti 1x/diff |

Types : **Demon**, **Undead** (+50% blunt, Holy Bolt/Sanctuary eff.), **Animal** (neutre).

### 1.3 Champion sous-types `[S1]`

> `-> MGE: mge-arpg-entity::ChampionType`

| Sous-type | HP mult N/NM/H | Dmg | Speed | Special |
|-----------|-----------------|-----|-------|---------|
| Champion | x3/x2.5/x2 | x2 | +20% | Standard |
| Berserker | x0.75 | x4 | Normal | Fragile, mLvl+3 |
| Fanatic | x3/x2.5/x2 | +90% | +100% | -70% def |
| Ghostly | x3/x2.5/x2 | +cold dmg | -20% | 80% phys resist |
| Possessed | x6 | +90% | Normal | Immune curses |

### 1.4 Affixes Unique/Elite `[S1]`

> `-> MGE: mge-arpg-entity::MonsterAffix` | TOML: `assets/data/monsters/affixes.toml`

| Affix | Effet | Res gagnee | On-Death |
|-------|-------|------------|----------|
| Extra Strong | +135% dmg, +90% AR | -- | -- |
| Extra Fast | +100% speed, x2 atk rate | -- | -- |
| Cursed | 75% Amplify Damage on hit | -- | -- |
| Magic Resistant | +40% fire/cold/light | +40% tri-res | -- |
| Stone Skin | +50% phys res, +200% def | +50% phys | -- |
| Fire Enchanted | +fire dmg (66-256 base) | +75% fire | CE fire (75-100% HP, 4yd) |
| Cold Enchanted | +cold dmg (10-50 base) | +75% cold | Frost Nova (1s freeze) |
| Lightning Enchanted | +light dmg (1-300 base) | +75% light | Charged Bolts on hit (4-6 bolts) |
| Spectral Hit | +random elem | +75% all | -- |
| Mana Burn | x4 mana drain | +20% magic | -- |
| Teleportation | TP si HP<33%, heal 4% | -- | -- |
| Multishot | 3 projectiles | -- | Ranged only |
| Aura Enchanted | Aura random | -- | -- |

Auras possibles : Might, Holy Fire, Blessed Aim, Holy Freeze, Holy Shock, Conviction, Fanaticism.
Nb affixes : N=1, NM=2(+1), H=3(+2). Minions heritent 1er affix seulement.
**LMLE** (Lightning Multi = letal) : 4-6 bolts x 3 projectiles = 12-18 bolts. Danger : one-shot possible.

### 1.5 Super Uniques (62 total) `[S2-S3]`

> `-> MGE: mge-arpg-entity::SuperUnique` | TOML: `assets/data/monsters/super_uniques.toml`

**A1 (13)** : Corpsefire(Den), Bishibosh(Cold Plains), Bonebreaker(Crypt), Blood Raven(Burial), Coldcrow(Cave), Rakanishu(Stony), Treehead(Dark Wood), Griswold(Tristram), Countess(Tower L5), Pitspawn(Jail L2), Bone Ash(Cathedral), The Smith(Barracks), **Andariel**(Cat L4)

**A2 (11)** : Radament(Sewers L3), Creeping Feature(Stony Tomb), Blood Witch(Halls Dead L3), Beetleburst(Far Oasis), Coldworm(Maggot L3), Dark Elder(Lost City), Fangskin(Claw Viper L2), Fire Eye(Palace L3), Summoner(Arcane), Ancient Kaa(Tal Rasha), **Duriel**(Chamber)

**A3 (12)** : Sszark(Spider Cavern), Endugu(Flayer Dung L3), Stormtree(Lower Kurast), Sarina(Ruined Temple), Icehawk(Sewers L1), Ismail/Geleb/Toorc(Travincal), Bremm/Wyand/Maffer(Durance L3), **Mephisto**(Durance L3)

**A4 (6)** : Izual(Plains Despair), Hephasto(River Flame), Grand Vizier/Lord de Seis/Infector(Chaos), **Diablo**(Chaos)

**A5 (20)** : Dac Farren, Shenk, Eldritch, Sharptooth, Eyeback, Thresh Socket, Frozenstein, Bonesaw Breaker, Snapchip, Pindleskin, Nihlathak(CE), Talic/Madawc/Korlic(Ancients), Colenzo/Achmel/Bartuc/Ventar/Lister(Baal waves), **Baal**(WSC)

**Baal Waves** : W1=Colenzo(Fallen), W2=Achmel(Undead), W3=Bartuc(Council+Hydras), W4=Ventar(Venom Lord), W5=Lister(Minions of Destruction). Baal cast Decrepify.

### 1.6 Act Bosses -- Stats complets `[S1]`

> `-> MGE: mge-arpg-entity::ActBoss, mge-arpg-combat::BossAi` | TOML: `assets/data/monsters/bosses.toml`
> Cross-ref: [REF-02a SS1.1](REF-02a-Combat-Formulas.md) (damage pipeline), **REF-03 SS2.2** (boss drops/TC)

#### Stats de base

| Boss | Lvl N/NM/H | HP N/NM/H | Defense N/NM/H | Block N/NM/H |
|------|------------|-----------|----------------|--------------|
| Andariel | 12/49/75 | 1,024/24,800/60,031 | 60/752/1,622 | 0/20/40% |
| Duriel | 22/55/88 | 3,995/55,799/84,524 | 112/907/2,044 | 0/25/50% |
| Mephisto | 26/59/87 | 6,036/74,547/94,320 | 183/1,286/2,697 | 20/40/50% |
| Diablo | 40/62/94 | 13,818/90,749/113,812 | 208/1,176/2,534 | -- |
| Baal | 60/75/99 | 26,484/117,596/493,701 | 313/1,494/2,847 | 40/45/55% |

#### Resistances

| Boss | Phys N/NM/H | Fire N/NM/H | Cold N/NM/H | Light N/NM/H | Poison N/NM/H | Magic N/NM/H |
|------|-------------|-------------|-------------|--------------|---------------|--------------|
| Andariel | 0/0/66 | **-50/-50/-50** | 50/50/66 | 50/50/66 | 80/50/66 | 0/0/0 |
| Duriel | 0/0/50 | 20/50/75 | 50/75/95 | 20/50/75 | 20/50/75 | 0/0/33 |
| Mephisto | 0/0/20 | 33/50/75 | 25/25/75 | 33/50/75 | 50/50/75 | 0/0/50 |
| Diablo | 0/0/45 | 33/50/50 | 33/50/50 | 33/50/50 | 50/50/50 | 0/0/0 |
| Baal | 0/0/50 | 33/50/50 | 33/50/50 | 33/50/50 | 50/50/50 | 0/0/0 |

**Andariel fire weakness (-50%)** = exploit majeur (Fireball/Meteor farming speed).

#### Drain Effectiveness (Life/Mana leech)

| Boss | N | NM | H | Note |
|------|---|-----|---|------|
| Andariel | 100% | 100% | 100% | Full leech all diff |
| Duriel | 100% | 100% | 100% | Full leech all diff |
| Mephisto | 100% | **0%** | **0%** | No leech NM/H (post-1.10) |
| Diablo | 100% | 100% | 100% | Full leech all diff |
| Baal | 100% | **50%** | **20%** | Reduced leech NM/H |

#### Attaques principales

| Boss | Attaque | Dmg N | Dmg NM | Dmg H |
|------|---------|-------|--------|-------|
| Andariel | Melee (Phys+Poison) | 10-20 | 60-90 | 165-231 |
| Andariel | Poison Spray AoE | DoT | DoT | DoT (+175 psn/s, 10s) |
| Duriel | Jab | 19-25 | 63-85 | 140-190 |
| Duriel | Smite (stun) | 19-22 | 51-74 | 115-165 |
| Duriel | Charge | 57-75 | 236-318 | **665-902** |
| Mephisto | Melee | 50-75 | 78-107 | 156-215 |
| Mephisto | Lightning / Charged Bolt | Elem | Elem | Elem (heavy) |
| Diablo | Melee A1 | 19-49 | 91-112 | 192-235 |
| Diablo | Melee A2 | 28-64 | 96-127 | 203-267 |
| Diablo | Lightning Hose | Elem | Elem | **Lethal** |
| Baal | Melee A1 (Phys+Fire) | 39-66 | 73-109+61-91 | 166-210+133-222 |
| Baal | Melee A2 | 50-100 | 82-146 | 183-266 |
| Baal | Incineration Nova | Fire AoE | Fire AoE | Fire AoE (heavy) |
| Baal | Hoarfrost (knockback) | Cold AoE | Cold AoE | Cold AoE |
| Baal | Mana Rift | Mana drain | Mana drain | Mana drain |

Commun : Jamais immunes. Pas de regen. Demons. Quest drop garanti 1er kill/diff.
**Diablo** : x7 dmg vs player minions (mercs/summons). **Baal** : clone illusion + teleport.
**Duriel Holy Freeze aura** : ralentit 50% -- critique pour melee builds, prevoir Cannot Be Frozen.

### 1.7 Immunites Hell `[S1]`

> `-> MGE: mge-arpg-combat::ImmunityBreaking` | Cross-ref: [REF-02a SS4](REF-02a-Combat-Formulas.md) (Lower Resist/Conviction formules)

`resistance > 99% = IMMUNE`. Magic immunity **inbrisable**.

| Skill | Max reduction | Vs Immune (1/5 eff) | Resultat |
|-------|-------------|---------------------|----------|
| Lower Resist (Necro) | -70% | -14% | Casse immunite si res <= 113 |
| Conviction (Paladin) | -150% resist | -30% | Casse immunite si res <= 129 |
| Amplify Damage (Necro) | -100% phys | -20% phys | Casse phys immune si res <= 119 |
| Decrepify (Necro) | -50% phys | -10% phys | Casse phys immune si res <= 109 |

```
Reduction_necessaire = (Res_monstre - 99) * 5
Exemple : 110% fire res -> (110-99)*5 = 55 reduction -> Conviction(-150/5=-30) suffit
```

Sunder Charms (2.5+) : immunite -> 95% res, puis reductions normales appliquent.

**Immunites communes Hell** : Fire(~40% monstres), Cold(~35%), Light(~25%), Poison(~55%), Phys(~15%).
Dual-immune zones : Chaos Sanct. (MSLE+Stone Skin), WSK (multi-type packs).

### 1.8 Archetypes IA `[S1]`

> `-> MGE: mge-arpg-ai::AiArchetype` | TOML: `assets/data/ai/archetypes.toml`

| Archetype | Comportement | Aggro range | Exemples |
|-----------|-------------|-------------|----------|
| Melee Charge | Droit vers joueur | ~20 tiles | Zombie, Skeleton, Goatman |
| Melee Retreat | Attaque puis recule 3-5 tiles | ~15 tiles | Fallen |
| Ranged Keep-distance | Distance fixe 8-12 tiles, projectiles | ~25 tiles | Skeleton Archer, Corrupt Rogue |
| Caster Reposition | Sort + deplacement random 5-8 tiles | ~20 tiles | Oblivion Knight, Council |
| Summoner Resurrect | Ressuscite allies, fuit si menace | ~15 tiles | Fallen Shaman, Greater Mummy |
| Fleeer | Fuit a chaque kill allie, rallied par Shaman | ~10 tiles | Fallen (rallied par Shaman) |
| Boss Multi-phase | Pattern sequentiel, abilities rotation | Full zone | Act bosses |

**FSM minimal** : `Idle -> Aggro -> (Attack | Cast | Flee | Resurrect) -> Idle`. Transition par distance, HP%, allies.
Pathfinding : A* sur grille sub-tiles. Stuck timer : 2s -> teleport si boss/unique.

### 1.9 Mercenaires `[S1-S2]`

> `-> MGE: mge-arpg-entity::MercenaryDefinition, mge-arpg-ai::MercAi`

| Acte | Type | Equip slots | Variants | Auras/Skills |
|------|------|-------------|----------|--------------|
| 1 | Rogue Scout (ranged) | Bow,Helm,Armor | Cold Arrow / Fire Arrow | -- |
| 2 | Desert Guard (**meta**) | Polearm,Helm,Armor | Combat/Defense/Offense | Might / Holy Freeze / Thorns(N), Prayer/Defiance/Blessed Aim(NM), Might/Holy Freeze/Thorns(H) |
| 3 | Iron Wolf (caster) | Sword,Shield,Helm,Armor | Lightning / Fire / Cold | Respective element skills |
| 5 | Barbarian (melee) | Sword(s),Helm,Armor | Frenzy / Bash | Stun |

A2 meta runewords : Insight(Meditation), Infinity(Conviction), Reaper's Toll(Decrepify).
A2 Defense(Holy Freeze) = choix universel : ralentit monstres, survie. Cross-ref: **REF-03 SS3.5** (runewords merc)

```
Resurrect_cost = min((hlvl^2 / 2) * 15, 50000)    // hlvl = hire level
```

Ethereaux = pas de perte durabilite sur merc (multiplicateur dmg/def gratuit).
**Merc IA** : suit joueur (2-5 tiles), aggro le monstre le plus proche, ne fuit jamais, pas de potion auto.

### 1.10 Familles monstres `[S2-S3]`

> `-> MGE: mge-arpg-entity::MonsterFamily` | TOML: `assets/data/monsters/families.toml`

| Famille | Type | Actes | IA | Special |
|---------|------|-------|----|---------|
| Fallen | Demon | 1,5 | Fuite/rallied Shaman | Shaman kill = permanent death |
| Zombie | Undead | 1,2 | Lent, charge if close | -- |
| Skeleton | Undead | 1,2,3 | Archers/mages/guerriers | Mages = elem dmg |
| Corrupt Rogue | Demon | 1 | Archers/lanciers | -- |
| Goatman | Demon | 1,2,3 | Melee brutal | -- |
| Fallen Shaman | Demon | 1,3,5 | Resurrect, fireball | Priority target |
| Scarab | Animal | 2 | Lightning charge | Lightning Enchanted variant |
| Mummy/Greater Mummy | Undead | 2,3 | Poison, resurrect | Unravel = resurrector |
| Flayer | Demon | 3 | Petit, nombreux, blow dart | Poison blow dart |
| Vampire | Undead | 3,4 | Caster, fireball, meteor | Fire immune variants |
| Council Member | Demon | 3,5 | Hydra, heal, melee | Heal = must kill fast |
| Doom/Oblivion Knight | Demon | 4 | Caster, curses | Iron Maiden (letal melee) |
| Venom Lord | Demon | 4,5 | Melee, inferno | High HP, fire immune possible |
| Succubus | Demon | 5 | Ranged, Blood Star | Blood Star = magic dmg |
| Reanimated Horde | Undead | 5 | Revient si non-shattered | Cold kill = permanent |
| Minion of Destruction | Demon | 5 | Knockback, regen, Wave 5 | Highest non-boss HP |

---

## 2. Multijoueur & Economie `[S2-S3]`

> `-> MGE: mge-net::GameSession, mge-net::PlayerSync` | `sodomight-server/`, `sodomight-client/`

### 2.1 Regles generales `[S2]`

> `-> MGE: mge-net::LobbyManager, mge-arpg-trade::TradeSession`

- Max **8 joueurs**/partie. Nom+mdp optionnel. Difficulte definie a la creation.
- Loot : Free-for-all. Timer priorite ~10s pour le tueur (en party). Cross-ref: **REF-03 SS2.1**
- Trade : Double confirmation. Modification -> reset Accept. Items quest non-tradables.
- PvP : Degats = **1/6** (17%). Death = drop "ear". Town = safe. Lvl min 9.
- Ladder : Reset ~6 mois, runewords/uniques exclusifs, economies separees.
- Hardcore : Mort permanente, economie separee HC/SC.
- Hostility : declaration depuis party menu, 10s timer avant PvP actif.

### 2.2 XP Sharing `[S2]`

> `-> MGE: mge-arpg-stats::XpDistribution` | Cross-ref: **REF-01 SS2.2** (XP formulas)

```
XP_totale = XP_base * (Nb_joueurs + 1) / 2
XP_joueur = XP_totale * (Niveau_joueur / Somme_niveaux_proches)
```

Bonus party : +35% si meme zone. Proximite : ~53.33 yards (~2 ecrans).

**Penalite post-70** (XP divisor from REF-01):

| clvl | XP% effectif | clvl | XP% effectif |
|------|-------------|------|-------------|
| 70 | 95.3% | 85 | 25.0% |
| 75 | 71.9% | 90 | 6.0% |
| 80 | 48.4% | 95 | 0.6% |

### 2.3 Player Count Scaling `[S1]`

> `-> MGE: mge-arpg-world::PlayerCountScaling`

```
Monster_HP  = Base_HP * (Players + 1) / 2          // 1P=100%, 3P=200%, 5P=300%, 8P=450%
Monster_XP  = Base_XP * (Players + 1) / 2          // meme formule que HP
Monster_DMG = Base_DMG * (1 + 0.0625*(Players-1))  // 1P=100%, 8P=143.75%
Monster_AR  = Base_AR * (1 + 0.0625*(Players-1))   // meme formule que DMG
CB_Dmg      = CB_Base / (0.5 + 0.5 * Players)      // 1P=100%, 3P=50%, 8P=22.2%
```

| /players | HP mult | XP mult | DMG mult | CB eff |
|----------|---------|---------|----------|--------|
| 1 | 100% | 100% | 100% | 100% |
| 3 | 200% | 200% | 112.5% | 50% |
| 5 | 300% | 300% | 125% | 33.3% |
| 7 | 400% | 400% | 137.5% | 25% |
| 8 | 450% | 450% | 143.75% | 22.2% |

### 2.4 NoDrop Scaling `[S1]`

> `-> MGE: mge-arpg-loot::NoDropScaling` | Cross-ref: **REF-03 SS2.1** (NoDrop formula)

```
N = int(1 + floor(AdditionalPlayers/2) + floor(ClosePartiedPlayers/2))
NewNoDrop = int( ProbSum / (1 / ((NoDrop / (NoDrop + ProbSum))^N) - 1) )
```

Effectif aux players impairs : /p1=N1, /p3=N2, /p5=N3, /p7=N4.

| /players | N effective | NoDrop reduction | Best for |
|----------|-----------|-----------------|----------|
| 1 | 1 | Baseline | -- |
| 3 | 2 | ~50% | Solo farming optimal |
| 5 | 3 | ~75% | Diminishing returns start |
| 7 | 4 | ~87% | Sweet spot farming |
| 8 | 4 | ~87% | Same as /p7 |

**/p7 = sweet spot** : meme NoDrop que /p8 mais HP 400% vs 450%.

### 2.5 Economie meta-game `[S3]`

> `-> MGE: mge-arpg-trade::EconomyConfig` | Cross-ref: **REF-03 SS3.6** (rune values)

Hierarchie runes (Ist = 1x) : Ist=1, Gul=2, Vex=4, Ohm=6-8, Lo=8-12, Ber=15-25, Jah=15-25, Cham=2-3, Zod=2-4.

**Currency tiers** :
- Low : Pgems, Ral/Amn/Sol (barter)
- Mid : Pul-Gul (mid uniques, niche runewords)
- High : Vex-Lo (Hoto, Fort, CTA)
- Ultra : Ber-Jah (Enigma, Infinity, BoTD)

---

## 3. Implementation MGE Summary `[S0-S3]`

| Systeme | Crate MGE | TOML data | Sprint |
|---------|-----------|-----------|--------|
| Monster stats (HP/def/res) | `mge-arpg-entity` + `mge-arpg-stats` | `monsters/monstats.toml`, `monsters/monlvl.toml` | S1 |
| Monster affixes | `mge-arpg-entity` | `monsters/affixes.toml` | S1 |
| Super Uniques + Act Bosses | `mge-arpg-entity` | `monsters/super_uniques.toml`, `monsters/bosses.toml` | S1-S2 |
| Monster AI (FSM) | `mge-arpg-ai` | `ai/archetypes.toml` | S1 |
| Mercenaries | `mge-arpg-entity` + `mge-arpg-ai` | `mercs/definitions.toml` | S1-S2 |
| Immunity breaking | `mge-arpg-combat` | `combat/immunity.toml` | S1 |
| Quests (Rhai scripting) | `mge-arpg-quest` | `scripts/quests/*.rhai` | S2-S3 |
| Player count scaling | `mge-arpg-world` | `difficulty.toml` | S1 |
| NoDrop scaling | `mge-arpg-loot` | `loot/nodrop.toml` | S1 |
| Multiplayer sessions | `mge-net` | `server/config.toml` | S2 |
| Trade + economy | `mge-arpg-trade` | `economy/rune_values.toml` | S3 |
