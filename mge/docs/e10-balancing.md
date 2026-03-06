# E10 -- Notes d'equilibrage Acte 1 -- P3

**version**: P3-freeze
**perimetre**: Camp de depart + Acte 1 (zones 0 a 42)

---

## Zones et niveaux de monstre

| Zone | ID | area_level | Biome | Waypoint |
|------|----|------------|-------|----------|
| Rogue Encampment | 0 | 1 | TownCamp | oui |
| Blood Moor | 10 | 1 | Wilderness | non |
| Den of Evil | 11 | 1 | Cave | non |
| Cold Plains | 20 | 2 | Wilderness | oui |
| Burial Grounds | 21 | 2 | Forest | non |
| Cave | 22 | 3 | Cave | non |
| Stony Field | 30 | 4 | Wilderness | oui |
| Underground Passage | 31 | 5 | Dungeon | non |
| Cathedral | 40 | 7 | Cathedral | oui |
| Catacombs | 41 | 8 | Catacomb | oui |
| Andariel's Chamber | 42 | 9 | Catacomb | non |

Progression area_level : 1 -> 1 -> 1 -> 2 -> 2 -> 3 -> 4 -> 5 -> 7 -> 8 -> 9
Pas de saut brutal; la progression est lisse vers le boss.

---

## Roster Acte 1 -- donnees de base

| Monstre | ID | level | HP | Damage | XP | Drop% | Zone(s) |
|---------|-------|-------|----|--------|----|-------|---------|
| Fallen | 1 | 1 | 30 | 5 | 10 | 30 | 10, 20, 30 |
| Fallen Shaman | 2 | 2 | 40 | 8 | 20 | 35 | 10, 20, 30 |
| Zombie | 3 | 2 | 60 | 10 | 18 | 25 | 21 |
| Skeleton | 4 | 3 | 50 | 12 | 22 | 28 | 31 |
| Dark Ranger | 5 | 3 | 55 | 15 | 30 | 40 | 21 |
| Spike Fiend | 6 | 4 | 70 | 18 | 35 | 32 | 11, 22 |
| Skeleton Mage | 7 | 7 | 80 | 22 | 55 | 38 | 41, 42 |
| Blood Raven | 100 | 3 | 500 | 40 | 300 | 100 | 21 |
| Andariel | 101 | 9 | 3 000 | 120 | 5 000 | 100 | 42 |

Formule HP a haut niveau : `base_hp + base_hp * (area_level - base_level) / 4`
Exemple : Fallen en zone area_level 4 -> HP = 30 + 30 * 3 / 4 = 52 (+73%)

---

## Boss Andariel -- 3 phases

| Phase | Declenchement | Description |
|-------|---------------|-------------|
| 0 | 100% HP | Normal (melee + poison) |
| 1 | <= 60% HP | Enrage -- dommages +50% |
| 2 | <= 25% HP | Frenetique -- vitesse +30%, deluge poison |

HP total : 3 000 / Damage base : 120
XP recompense : 5 000 (cap theorique par kill solo)
Drop garanti : 100% (is_boss = true)

---

## Mercenaires -- couts et stats de base

| Merc | ID | Type | Cout embauche | HP | Damage | Cout ress. |
|------|----|------|---------------|----|--------|-----------|
| Flavie | 1 | RogueScout | 500 | 200 | 25 | 250 |
| Lysa | 2 | RogueScout | 500 | 180 | 30 | 250 |

Cout resurrection = 50% du cout d'embauche (base), croissant selon le niveau.
Note : Le cout base de resurrection est une valeur initiale; il doit etre ajuste par `level * multiplier` en P4.

---

## Equilibrage XP -- party

| Joueurs en zone | Mode Split | XP par joueur |
|-----------------|-----------|---------------|
| 1 | -- | 100% |
| 2 | Split | 50% |
| 4 | Split | 25% |
| 8 | Split | 12.5% |

Mode `Individual` : chaque membre recoit 100% independamment.
Mode `LeaderControlled` : meme reduction que Split (P3 = placeholder; logique leader differee).

Recommandation : En mode `Split`, un bonus de synergie (+10% par membre additionnel) est souhaitable en P4 pour ne pas penaliser les grosses parties.

---

## Economie -- prix de base

La formule generale (definie dans `mge-items::economy`) est :

- `base_price` = valeur intrinseque de l'item
- `buy_price` = `base_price * 1.25`
- `sell_price` = `base_price * 0.25`
- `repair_cost` = `base_price * 0.15`

Ecart achat/vente intentionnellement eleve (ratio 5:1) conforme a D2. Pas de correction prevue pour P3.

---

## Tables de loot -- probabilites camp + Acte 1

Monstres normaux : drop_chance 25-40% selon type.
Fallen Shaman : 35% -- intentionnel, les shamans doivent recompenser leur elimination en priorite.
Elite variants : multiplicateur hp_multiplier x2, xp_multiplier x3 -- standard D2.
Champions : multiplicateur hp x1.5, xp x2.

Note : Le simulateur LCG (`mge-items::simulator`) est deterministe (seed fixe). En production, la seed doit etre fournie par le serveur par session pour garantir la non-predictibilite.

---

## Progression de quetes -- temps estime Acte 1

| Quete | Zone cible | Prereq |
|-------|------------|--------|
| 1 - Den of Evil | 11 | -- |
| 2 - Blood Raven | 21 | -- |
| 3 - Rescue Cain | Cairn Stones (30) | -- |
| 4 - Forgotten Tower | Catacombs (41) | -- |
| 5 - The Malus | Underground Passage (31) | -- |
| 6 - Sisters to the Slaughter | 42 | quete 3 completee |

Parcours recommande (walkthrough validator step order) :
Camp -> Blood Moor -> Den -> Cold Plains -> Burial -> Stony -> Underground -> Cathedral -> Catacombs -> Boss

---

## Points d'attention equilibrage P4

1. **HP Andariel en party 8** : 3 000 HP base ne scale pas avec la taille du groupe. En P4, ajouter `boss_hp *= 1 + (party_size - 1) * 0.5`.
2. **Mercenaire niveau** : Pas de scaling level en P3; les stats sont statiques. A implanter en P4.
3. **Mode Individual XP** : En party 8, chacun recoive 100% -- trop genereux. Envisager un cap 200% (4 joueurs max full XP) en P4.
4. **LCG seed reseau** : Seed fixe en tests; en prod, seed doit etre injectee par `ZoneServer` au moment de la generation.
5. **Ladder reset policy** : "6 mois + top 20 cosmetiques" est un placeholder; a valider avec l'equipe produit en P4.
