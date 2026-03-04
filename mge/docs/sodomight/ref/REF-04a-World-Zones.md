# REF-04a -- World, Zones, Waypoints & Generation Procedurale -- Reference Compacte
<!-- @id REF-04a @do reference-world-zones @role Fabrice @layer 7 @human miyuki -->
<!-- Split de REF-04 le 2026-03-03. Formules D2, valeurs exactes, MGE mapping, sprints, cross-refs -->

**Projet** : Sodomight | **Moteur** : MGE | **Ref D2** : v1.14d / D2R 2.x

> **TL;DR** : Structure complete du monde D2 : 5 actes, area levels par zone (avec alvl85 Hell en gras), 39 waypoints, generation procedurale (outdoor random, caves, dungeons fixes), villes/NPCs, difficulte scaling, et les 27 quetes avec leurs rewards permanents. Tout mappe vers `mge-arpg-world` avec TOML data-driven.

> **Voir aussi** : [REF-04b](REF-04b-Monsters-Multiplayer.md) pour les monstres, boss, IA et multiplayer scaling.

---

## 1. Monde & Zones `[S0-S1]`

> `-> MGE: mge-arpg-world::ZoneDefinition, mge-arpg-world::ActDefinition` | TOML: `assets/data/zones/`

### 1.1 Structure des 5 Actes

| Acte | Theme | Ville | Boss | Zones | WP | alvl85 Hell |
|------|-------|-------|------|-------|----|-------------|
| 1 | Foret/monastere | Rogue Encampment | Andariel | ~25 | 9 | Pit L1/L2, Mausoleum |
| 2 | Desert/tombeaux | Lut Gholein | Duriel | ~22 | 9 | Ancient Tunnels, Maggot L3 |
| 3 | Jungle/temples | Kurast Docks | Mephisto | ~24 | 9 | Sewers L2, Forgotten Temple |
| 4 | Enfer/chaos | Pandemonium Fortress | Diablo | ~5 | 3 | River of Flame, Chaos Sanct. |
| 5 | Montagne/neige | Harrogath | Baal | ~24 | 9 | WSK L1-L3, Throne, WSC |

Progression : boss kill -> acte suivant. Baal kill -> difficulte suivante.
`-> MGE: mge-arpg-quest::ActProgression` | Cross-ref: **REF-01 SS1.1**

### 1.2 Zones par acte avec Area Levels `[S1]`

> `-> MGE: mge-arpg-world::AreaLevel` | TOML: `assets/data/zones/area_levels.toml`
> Cross-ref: **REF-01 SS2.4** (alvl85 farming), **REF-03 SS2.1** (TC/alvl -> qlvl drops)

**alvl 85 Hell en gras** = tous items droppables (TC87). Formule TC : `alvl >= qlvl_item` requis.

#### Acte 1

| Zone | N | NM | H | WP | Quete |
|------|---|-----|---|----|----|
| Blood Moor | 1 | 36 | 67 | -- | Den nearby |
| Cold Plains | 2 | 36 | 68 | Oui | -- |
| Stony Field | 4 | 37 | 68 | Oui | Cairn Stones |
| Dark Wood | 5 | 38 | 68 | Oui | Inifuss Tree |
| Black Marsh | 6 | 38 | 69 | Oui | -- |
| Tamoe Highland | 8 | 39 | 69 | -- | -- |
| Burial Grounds | 3 | 36 | 80 | -- | Blood Raven |
| Den of Evil | 1 | 36 | 79 | -- | Q1 |
| Cave L1/L2 | 2 | 36-37 | 77-78 | -- | -- |
| Underground Passage L1/L2 | 4 | 37-38 | 69/83 | -- | -- |
| The Hole L1/L2 | 5 | 38-39 | 80-81 | -- | -- |
| The Pit L1/L2 | 7 | 39-40 | **85/85** | -- | -- |
| Crypt | 3 | 37 | 83 | -- | -- |
| Mausoleum | 3 | 37 | **85** | -- | -- |
| Tower Cellar L1-L5 | 7 | 38-42 | 75-79 | -- | Q5:Countess(L5) |
| Tristram | 6 | 39 | 76 | -- | Q4:Rescue Cain |
| Monastery Gate/Outer Cloister | 8-9 | 40 | 70 | Oui(OC) | -- |
| Barracks | 9 | 40 | 70 | -- | Q3:Smith |
| Jail L1-L3 | 10 | 41 | 71-72 | Oui(L1) | Pitspawn(L2) |
| Inner Cloister/Cathedral | 10-11 | 41-42 | 72 | Oui(IC) | -- |
| Catacombs L1-L4 | 11-12 | 42-43 | 72-73 | Oui(L2) | Q6:Andariel(L4) |
| Secret Cow Level | 28 | 64 | 81 | -- | Special |

#### Acte 2

| Zone | N | NM | H | WP | Quete |
|------|---|-----|---|----|----|
| Rocky Waste | 14 | 43 | 75 | -- | -- |
| Dry Hills | 15 | 44 | 76 | Oui | -- |
| Far Oasis | 16 | 45 | 76 | Oui | Beetleburst |
| Lost City | 17 | 46 | 77 | Oui | Dark Elder |
| Valley of Snakes | 18 | 46 | 77 | -- | -- |
| Canyon of the Magi | 16 | 48 | 79 | -- | 7 tombes |
| Sewers L1-L3 | 13-14 | 43-44 | 74-75 | Oui(L2) | Q1:Radament(L3) |
| Harem L2/Palace Cellar L1-L3 | 13 | 47-48 | 78 | Oui(PC-L1) | -- |
| Stony Tomb L1-L2 | 12 | 44 | 78-79 | -- | Creeping Feature(L2) |
| Halls of Dead L1-L3 | 12-13 | 44-45 | 79-82 | Oui(L2) | Q3:Cube(L3) |
| Claw Viper Temple L1-L2 | 14 | 47 | 82-83 | -- | Q2:Amulet(L2) |
| Maggot Lair L1-L3 | 17 | 45-46 | 84-**85** | -- | Q3:Staff(L3) |
| Ancient Tunnels | 17 | 46 | **85** | -- | -- |
| Arcane Sanctuary | 14 | 48 | 79 | Oui | Q5:Summoner |
| Tal Rasha's Tomb (x7)/Chamber | 17 | 49 | 80 | -- | Q6:Duriel |

#### Acte 3

| Zone | N | NM | H | WP | Quete |
|------|---|-----|---|----|----|
| Spider Forest | 21 | 49 | 79 | Oui | -- |
| Great Marsh | 21 | 50 | 80 | Oui | -- |
| Flayer Jungle | 22 | 50 | 80 | Oui | Gidbinn |
| Lower Kurast | 22 | 52 | 80 | Oui | -- |
| Kurast Bazaar | 22 | 52 | 81 | Oui | -- |
| Upper Kurast | 23 | 52 | 81 | Oui | -- |
| Kurast Causeway | 24 | 53 | 81 | -- | -- |
| Travincal | 24 | 54 | 82 | Oui | Q5:Council |
| Spider Cave/Cavern | 21 | 50 | 79 | -- | Q2:Eye(Cavern) |
| Swampy Pit L1-L3 | 21-22 | 50-51 | 80-83 | -- | -- |
| Flayer Dungeon L1-L3 | 22 | 51 | 81-83 | -- | Q2:Brain(L3) |
| Sewers L1-L2 (Kurast) | 23-24 | 52-53 | 84-**85** | -- | Q4:Heart(L2) |
| Ruined/Disused/Forgotten Temple | 23 | 53 | 84 | -- | Q3:Lam Esen |
| Forgotten Temple/Ruined Fane | 24 | 54 | **85** | -- | Q3:Lam Esen |
| Durance of Hate L1-L3 | 25 | 55 | 83 | Oui(L2) | Q6:Mephisto(L3) |

#### Acte 4

| Zone | N | NM | H | WP | Quete |
|------|---|-----|---|----|----|
| Outer Steppes | 26 | 56 | 82 | -- | -- |
| Plains of Despair | 26 | 56 | 83 | -- | Q2:Izual |
| City of the Damned | 27 | 57 | 84 | Oui | -- |
| River of Flame | 27 | 57 | **85** | Oui | Q2:Hephasto |
| Chaos Sanctuary | 28 | 58 | **85** | -- | Q3:5 seals+Diablo |

#### Acte 5

| Zone | N | NM | H | WP | Quete |
|------|---|-----|---|----|----|
| Bloody Foothills | 24 | 58 | 80 | -- | Q1:Shenk |
| Frigid Highlands | 25 | 59 | 81 | Oui | Q2:Rescue |
| Arreat Plateau | 26 | 60 | 81 | Oui | -- |
| Crystalline Passage | 29 | 61 | 82 | Oui | -- |
| Frozen River | 29 | 61 | 83 | -- | Q3:Anya |
| Glacial Trail | 29 | 61 | 83 | Oui | -- |
| Drifter Cavern | 29 | 61 | 84 | -- | -- |
| Frozen Tundra | 27 | 60 | 81 | Oui | -- |
| Ancients' Way | 29 | 62 | 82 | Oui | -- |
| Arreat Summit | 37 | 68 | 87 | -- | Q5:Ancients |
| Icy Cellar | 29 | 62 | 83 | -- | Snapchip |
| Nihlathak's Temple | 32 | 63 | 83 | -- | Q4 |
| Halls of Anguish/Pain/Vaught | 33-36 | 63-64 | 83-84 | Oui(Pain) | Q4:Nihlathak(Vaught) |
| Abaddon/Pit of Acheron/Infernal Pit | 39 | 60-62 | 81-83 | -- | -- |
| WSK L1-L3 | 39-42 | 65-66 | **85** | Oui(L2) | -- |
| Throne of Destruction | 43 | 66 | **85** | -- | Q6:Baal waves |
| Worldstone Chamber | 43 | 66 | **85** | -- | Q6:Baal |

### 1.3 Waypoints (39 total) `[S1]`

> `-> MGE: mge-arpg-world::Waypoint` | TOML: `assets/data/zones/waypoints.toml`

A1(9): Rogue Enc., Cold Plains, Stony Field, Dark Wood, Black Marsh, Outer Cloister, Jail L1, Inner Cloister, Catacombs L2.
A2(9): Lut Gholein, Sewers L2, Dry Hills, Halls Dead L2, Far Oasis, Lost City, Palace Cellar L1, Arcane Sanctuary, Canyon of Magi.
A3(9): Kurast Docks, Spider Forest, Great Marsh, Flayer Jungle, Lower Kurast, Kurast Bazaar, Upper Kurast, Travincal, Durance L2.
A4(3): Pandemonium Fort., City of Damned, River of Flame.
A5(9): Harrogath, Frigid Highlands, Arreat Plateau, Crystalline Pass., Halls of Pain, Glacial Trail, Frozen Tundra, Ancients' Way, WSK L2.

Activation : contact physique. Persistant/difficulte. Gratuit/illimite. Villes = auto-actif.

### 1.4 Generation procedurale `[S1-S2]`

> `-> MGE: mge-arpg-world::MapGenerator, mge-arpg-world::TileAssembler`

| Type | Methode | Algo | Exemples |
|------|---------|------|----------|
| Outdoor random | Tuiles assemblees | Deck of Cards (sans remise) | Blood Moor, Cold Plains |
| Outdoor fixed | Layout fixe | Static | Travincal, Tristram, Arreat Summit |
| Indoor random | Prefabs + variations | Random pick + rotation | Jail, Maggot Lair |
| Cave | Dungeon procedural | Room+corridor + dead-ends | The Pit, Ancient Tunnels |
| Dungeon fixed | Toujours identique | Static | Catacombs L4, Chaos Sanct., Durance L3, Throne |

Connexions inter-zones toujours fixes meme si layout interne varie.
TOML: `assets/data/zones/map_presets.toml` (prefabs), `assets/data/zones/tile_decks.toml` (outdoor)

### 1.5 Towns & NPCs `[S1]`

> `-> MGE: mge-arpg-entity::NpcDefinition, mge-arpg-trade::VendorInventory`

Pattern constant : **Blacksmith + Healer + Gambler + Merc Captain + Cain + Transport**

| NPC | A1 | A2 | A3 | A4 | A5 |
|-----|----|----|----|----|-----|
| Blacksmith | Charsi | Fara | Hratli | Halbu | Larzuk |
| Healer | Akara | Fara/Atma | -- | -- | Malah |
| Gambler | Gheed | Elzix | Alkor | Jamella | Anya |
| Merc Captain | Kashya | Greiz | Asheara | -- | Qual-Kehk |

**Gambling** : `ilvl = clvl-5 a clvl+4`. Couts : 5954-35,898 or selon type. Chances : Unique=0.05%, Set=0.10%, Rare=9.85%, Magic=90%. Cross-ref: **REF-03 SS4.3**

Town Portals : Scroll/Tome(20 max), bidirectionnel, ferme si mort/nouvelle partie/nouveau TP.

### 1.6 Difficulty Scaling `[S0]`

> `-> MGE: mge-arpg-world::DifficultyConfig` | TOML: `assets/data/difficulty.toml`

| Diff | Res Penalty | XP Death Penalty | Lvl min | Immunites | Monster HP mult |
|------|-------------|------------------|---------|-----------|-----------------|
| Normal | 0% | 0% | 1 | Non | x1.0 |
| Nightmare | -40% all | -5% next lvl | 20 | Rares | x1.75 (base monlvl) |
| Hell | -100% all | -10% next lvl | 40 | Tres frequentes | x1.5 (vs NM base) |

Res max : 75% defaut (95% max). Champions/Uniques : +0/+1/+2 affixes extra (N/NM/H).
Cross-ref: **REF-02a SS1.3** (resistance penalty pipeline), **REF-01 SS1.3** (XP penalty)

### 1.7 Quetes (27 + Cow Level) `[S2-S3]`

> `-> MGE: mge-arpg-quest::QuestDefinition` | Scripting: Rhai `assets/scripts/quests/`

| A | Q# | Quete | Reward | Permanent |
|---|-----|-------|--------|-----------|
| 1 | Q1 | Den of Evil | +1 SP, respec | x3 diff |
| 1 | Q2 | Blood Raven | Merc gratuite | x1 |
| 1 | Q3 | Horadric Malus | Imbue (Charsi) | x3 diff |
| 1 | Q4 | Search for Cain | Identify gratuit | Permanent |
| 1 | Q5 | Countess | Runes drop | Farmable |
| 1 | Q6 | Andariel | Acces A2 | x1/diff |
| 2 | Q1 | Radament | +1 SP | x3 diff |
| 2 | Q2 | Horadric Staff | Requis boss | x1 |
| 2 | Q3 | Claw Viper | Soleil | x1 |
| 2 | Q4 | Arcane Sanctuary | Tombe | x1 |
| 2 | Q5 | Summoner | Acces Canyon | x1 |
| 2 | Q6 | Duriel | Acces A3 | x1/diff |
| 3 | Q1 | Golden Bird | +20 HP permanent | x3 diff |
| 3 | Q2 | Gidbinn | Ring rare | x1 |
| 3 | Q3 | Khalim's Will | Mephisto path | x1 |
| 3 | Q4 | Lam Esen | +5 Stat Points | x3 diff |
| 3 | Q5 | Council | Ring rare | x1 |
| 3 | Q6 | Mephisto | Acces A4 | x1/diff |
| 4 | Q1 | Izual | +2 SP | x3 diff |
| 4 | Q2 | Hellforge | Gemmes + runes | x1/diff |
| 4 | Q3 | Diablo | Acces A5 | x1/diff |
| 5 | Q1 | Shenk | Larzuk +sockets | x3 diff |
| 5 | Q2 | Rescue Mt Arreat | 3 mercs + Runes | x1 |
| 5 | Q3 | Anya | Personnalise + res | x3 diff |
| 5 | Q4 | Nihlathak | -- | x1 |
| 5 | Q5 | Ancients | XP massif, acces WSK | x1/diff |
| 5 | Q6 | Baal | Diff suivante | x1/diff |

**Totaux x3 diff** : +12 SP, +15 Stat Points, +60 HP, +30 res all(Anya).
