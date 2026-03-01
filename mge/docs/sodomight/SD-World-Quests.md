# SD — Monde & Quetes : Documentation Exhaustive

> **Projet** : Sodomight (clone spirituel Diablo 2 + LoD)
> **Moteur** : Miyukini Game Engine (MGE) — ECS Rust
> **Langue** : Explications en francais, termes techniques en anglais
> **Version document** : 1.0 — 2026-02-28

---

## Table des matieres

1. [Mecaniques Monde](#1-mecaniques-monde)
2. [Acte 1 — Les Ruines de Karak](#2-acte-1--les-ruines-de-karak)
3. [Acte 2 — Les Sables de Khemra](#3-acte-2--les-sables-de-khemra)
4. [Acte 3 — La Jungle de Vezir](#4-acte-3--la-jungle-de-vezir)
5. [Acte 4 — Les Plaines Maudites](#5-acte-4--les-plaines-maudites)
6. [Acte 5 — Les Bastions de Glace](#6-acte-5--les-bastions-de-glace)
7. [Schemas TOML](#7-schemas-toml)

---

## 1. Mecaniques Monde

### 1.1 Generation procedurale des cartes

Sodomight (comme D2) utilise un systeme **Deck-of-Cards (DOC)** : les cartes sont composees de tuiles pre-construites assemblees aleatoirement a chaque partie. Chaque zone est regeneree a chaque nouvelle partie (sauf exceptions ci-dessous).

**Cartes fixes (non regenerees) :**
- Catacomb Niveau 4 (boss Andariel) — D2 : Catacombs L4
- Sanctuaire du Chaos (boss Diablo) — D2 : Chaos Sanctuary
- Trone de la Destruction (boss Baal) — D2 : Throne of Destruction
- Village des Damnes (Tristram equivalent) — D2 : Tristram
- Place du Conseil (Travincal equivalent) — D2 : Travincal
- Demeure du Supplice N3 (Mephisto) — D2 : Durance of Hate L3
- Salles du Vaught (Nihlathak) — D2 : Halls of Vaught
- Canyon des Mages — D2 : Canyon of the Magi

**Toutes les autres zones** sont procedurales : layout different a chaque run, mais les connections entre zones restent fixes.

### 1.2 Systeme de Waypoints

Les waypoints permettent le voyage instantane entre zones decouverts. Il y a **39 waypoints** en tout (Normal, Nightmare, Hell partagent la meme activation).

| Acte | Nombre | Waypoints |
|------|--------|-----------|
| Acte 1 | 9 | Ville du Refuge, Plaine de Sang, Plaine Froide, Tour de Guet, Catacombes N1, Monastere, Baraquements, Ja-Pho L2, Prison N3 |
| Acte 2 | 9 | Lut Gholein, Plaine Aride, Repaire du Seweur, Allee du Portail, Dedale des Tombes, Vallee des Rois, Monastere Defile, Khorog L1, Oasis Sombre |
| Acte 3 | 9 | Kurast Docks, Bazaar Inferieur, Bazaar Superieur, Grand Marche, Flats, Precinct, Temple Ruine, Mephistorum L2, Durance of Hate L2 |
| Acte 4 | 3 | Pandemonium Fortress, City of the Damned, River of Flame |
| Acte 5 | 9 | Harrogath, Bloody Foothills, Frigid Highlands, Arreat Plateau, Crystalline Passage, Glacial Trail, Halls of Pain, Frozen Tundra, The Ancients Way |

**Noms Sodomight proposes :**
- Waypoint = **Nexus-pierre** (Nexus Stone)
- Acte 1 Ville = **Fort Karak**
- Acte 2 Ville = **Oasis de Khemra**
- Acte 3 Ville = **Port de Vezir**
- Acte 4 Ville = **Forteresse du Pandemonium**
- Acte 5 Ville = **Harrogath** (garde le nom, ville des barbares)

### 1.3 Portails de Ville (Town Portal)

- Consommable : Parchemin de Portail ou compétence (gratuite en Sodomight)
- Crée un portail bleu bidirectionnel vers la ville de l'acte actif
- Le portail reste ouvert jusqu'a : nouvelle partie, mort du personnage, ou ouverture d'un second portail par le meme personnage
- Les autres joueurs (coop) peuvent emprunter le portail
- Impossible dans les zones de boss fixes (Sanctuaire du Chaos, Trone)

**Noms Sodomight :**
- Town Portal = **Portail Refuge**

### 1.4 Systeme de Difficulte

| Difficulte | Nom SD | Penalite Resistances | Niveau min requis |
|------------|--------|---------------------|-------------------|
| Normal | Ordinaire | 0% | 1 |
| Nightmare | Cauchemar | -40% toutes resistances | 20 |
| Hell | Enfer | -100% toutes resistances | 40 |

Les resistances maximales restent plafonnees a 75% (sauf bonus d'equipement jusqu'a 95%).

### 1.5 Niveaux de Zone (Area Level)

Le niveau de zone (area level / alvl) determine :
- Le niveau des items droppes (item level = alvl)
- La formule to-hit des monstres vs joueur
- Le niveau des super-uniques (alvl +3 en NM, alvl+5 en Hell approximatif)

Zones cles :

| Zone (D2) | Normal | Nightmare | Hell |
|-----------|--------|-----------|------|
| Blood Moor | 1 | 36 | 67 |
| Cold Plains | 2 | 36 | 68 |
| The Pit L1 | 12 | 43 | 85 |
| The Pit L2 | 13 | 43 | 85 |
| Chaos Sanctuary | 28 | 58 | 85 |
| Throne of Destruction | 43 | 66 | 85 |
| Worldstone Keep L3 | 43 | 66 | 85 |
| Halls of Vaught | 45 | 67 | 85 |

### 1.6 Sanctuaires (Shrines)

Les sanctuaires sont des objets interactables fixes dans les zones. Chaque activation applique un effet temporaire au joueur (ou permanent pour certains).

| Type (D2) | Nom SD propose | Effet | Duree |
|-----------|---------------|-------|-------|
| Experience Shrine | Sanctuaire d'Eveil | +50% XP gagne | 96s |
| Mana Recharge Shrine | Sanctuaire de Flux | Regeneration mana x5 | 96s |
| Health Shrine | Sanctuaire de Seve | Regeneration vie x5 | 96s |
| Combat Shrine | Sanctuaire de Rage | +200% degats, +200% AR | 96s |
| Skill Shrine | Sanctuaire de Maitrise | +2 tous skills | 96s |
| Resistance Shrine | Sanctuaire de Blindage | +75% toutes resistances | 96s |
| Fire Shrine | Sanctuaire de Braise | Fireball automatique | 96s |
| Monster Shrine | Sanctuaire Maudit | Transforme un monstre en super-unique | Instantane |
| Poison Shrine | Sanctuaire Venen | Empoisonne le joueur | Duree variable |
| Exploding Shrine | Sanctuaire Volatile | Explose au contact | Instantane |
| Gem Shrine | Sanctuaire de Pierre | Upgrade une gemme random | Instantane |
| Recharging Shrine | Sanctuaire de Recharge | Recharge items | Instantane |
| Stamina Shrine | Sanctuaire d'Endurance | Stamina infinie | 96s |
| Portal Shrine | Sanctuaire Nexus | Ouvre portail aléatoire | Instantane |
| Armor Shrine | Sanctuaire de Cuirasse | +100% defense | 96s |

### 1.7 Coffres et Interactables

**Types de coffres :**
- Coffre Ordinaire (Chest) : loot aleatoire selon alvl
- Grand Coffre (Large Chest) : meilleur loot, souvent verouille
- Coffre Unique (Special Chest) : fixe dans certaines zones, meilleur loot garanti
- Charnier (Bone Pile) : loot squelettes, commun en acte 1-2
- Urne (Urn) : petits objets, commun en acte 3
- Tonneau (Barrel) : commun partout, parfois monstre pop-out

**Interactables speciaux :**
- Portail de Tristram (Acte 1, Q1) : ouvre acces au village
- Horadric Malus (Acte 1, Q3) : item de quete sur enclume
- Cube Horadrique (Acte 2, Q2) : dans Halls of the Dead L3
- Amulet de Tal Rasha (Acte 2, Q5) : dans tomb correcte parmi 7
- Flambeau du Zakarum (Acte 3, Q1) : Khalim flail craftee
- Steles Carrees (Acte 5, Q2) : activation des 4 steles barrieres

---

## 2. Acte 1 — Les Ruines de Karak

**Nom D2 original :** Act 1 — Rogue Encampment / Wilderness
**Nom SD :** Acte 1 — Les Ruines de Karak
**Ville :** Fort Karak (D2 : Rogue Encampment)
**Theme :** Forets obscures, marais, monastere en ruines, catacombes

### 2.1 NPCs de Fort Karak

| NPC (D2) | Nom SD propose | Role / Services |
|----------|---------------|----------------|
| Akara | Syrene | Healer, vendeuse parchemins/potions/staves/wands/scepters, identifie items, repare (late) |
| Charsi | Forga | Forgeron, repare, rachete items, recompense Q3 (imbue) |
| Gheed | Toval | Marchand generique, achete/vend items, jeux de sort |
| Kashya | Brova | Chef Rogue, donne Q2 (Tomb), loue mercenaires Rogue (archers) |
| Warriv | Darak | Caravannier, donne acces acte 2 apres Q6 |
| Flavie | Yris | Garde sentinelle (dialogue uniquement, N/A services) |
| Deckard Cain | Elias | Lore-keeper, identifie items gratuitement apres Q5 |
| Wirt (Tristram) | Thorn | Vendeur jambe (leg item), items uniques overprices |

**Services vendeur Syrene (Akara) :**
- Potions de Vie (toutes tailles)
- Potions de Mana (toutes tailles)
- Parchemins ID et Portal
- Baguettes, Batons, Sceptres (Normal+)
- Identifie items (gratuit si Elias present, sinon 100 gold)

**Mercenaires Brova (Kashya) :**
- Type : Archers Rogue
- Variantes : Cold Arrow (gel), Fire Arrow (feu)
- Stats : base defense, degats physiques + element
- Niveaux : scalent avec le joueur
- Nombre : 1 actif a la fois, peut etre loue/relance

### 2.2 Zones de l'Acte 1

| Zone (D2) | Nom SD | Type | Connections | alvl N/NM/H | Waypoint |
|-----------|--------|------|-------------|-------------|----------|
| Rogue Encampment | Fort Karak | Ville | Blood Moor | — | Oui |
| Blood Moor | Lande de Sang | Exterieur | Cold Plains, Den of Evil | 1/36/67 | Oui |
| Den of Evil | Antre du Mal | Grotte | Blood Moor | 1/36/67 | Non |
| Cold Plains | Plaine Froide | Exterieur | Blood Moor, Burial Grounds, Stony Field, Cave | 2/36/68 | Oui |
| Cave L1 | Caverne N1 | Grotte | Cold Plains | 2/36/68 | Non |
| Cave L2 | Caverne N2 | Grotte | Cave L1 | 5/37/68 | Non |
| Burial Grounds | Cimetiere | Exterieur | Cold Plains, Crypt, Mausoleum | 3/36/68 | Non |
| Crypt | Crypte | Souterrain | Burial Grounds | 3/36/68 | Non |
| Mausoleum | Mausolee | Souterrain | Burial Grounds | 3/36/68 | Non |
| Stony Field | Champ de Pierre | Exterieur | Cold Plains, Underground Passage, Tristram (portail) | 4/37/68 | Non |
| Underground Passage L1 | Passage Souterrain N1 | Grotte | Stony Field, Dark Wood | 5/38/68 | Non |
| Underground Passage L2 | Passage Souterrain N2 | Grotte | Underground Passage L1 | 8/38/68 | Non |
| Dark Wood | Bois Sombre | Exterieur | Underground Passage, Black Marsh | 5/38/68 | Non |
| Black Marsh | Marecage Noir | Exterieur | Dark Wood, Forgotten Tower, Tamoe Highland | 6/38/69 | Oui (Watchtower) |
| Forgotten Tower L1-L5 | Tour Oubliee N1-N5 | Souterrain | Black Marsh (L1 entree) | 7-10/38/69 | Non |
| Tamoe Highland | Lande de Tamoe | Exterieur | Black Marsh, Monastery Gate, Pit L1 | 8/39/69 | Non |
| The Pit L1 | La Fosse N1 | Souterrain | Tamoe Highland | 12/43/85 | Non |
| The Pit L2 | La Fosse N2 | Souterrain | Pit L1 | 13/43/85 | Non |
| Monastery Gate | Porte du Monastere | Exterieur | Tamoe Highland, Outer Cloister | 8/39/69 | Non |
| Outer Cloister | Cloitre Exterieur | Monastere | Monastery Gate, Barracks | 9/40/70 | Non |
| Barracks | Baraquements | Monastere | Outer Cloister, Jail L1 | 9/40/70 | Oui |
| Jail L1 | Prison N1 | Monastere | Barracks | 10/40/71 | Non |
| Jail L2 | Prison N2 | Monastere | Jail L1 | 10/40/71 | Oui (Jap L2) |
| Jail L3 | Prison N3 | Monastere | Jail L2, Inner Cloister | 11/40/71 | Oui |
| Inner Cloister | Cloitre Interieur | Monastere | Jail L3, Cathedral | 11/40/71 | Non |
| Cathedral | Cathedrale | Monastere | Inner Cloister, Catacombs L1 | 11/41/71 | Non |
| Catacombs L1 | Catacombes N1 | Souterrain | Cathedral | 11/41/71 | Oui |
| Catacombs L2 | Catacombes N2 | Souterrain | Catacombs L1 | 11/41/72 | Non |
| Catacombs L3 | Catacombes N3 | Souterrain | Catacombs L2 | 12/42/72 | Non |
| Catacombs L4 | Catacombes N4 | Souterrain FIXE | Catacombs L3 | 12/42/72 | Non |
| Tristram | Village des Damnes | Special FIXE | Stony Field (portail) | 2/36/67 | Non |

### 2.3 Super-Uniques de l'Acte 1

| Super-Unique (D2) | Nom SD | Zone | Modifieurs fixes |
|-------------------|--------|------|-----------------|
| Corpsefire | Corpse-Feu | Den of Evil | Spectral Hit, Extra Fast |
| Bishibosh | Sorcier Brulant | Cold Plains | Fire Enchanted, Extra Strong |
| Bloodraven | Corbeau-Sang | Burial Grounds | Cold Enchanted, Cursed (boss Q2) |
| Griswold | Forgeron Damne | Tristram | Spectral Hit, Cursed, Magic Resistant |
| Rakanishu | Rakanishi | Stony Field | Lightning Enchanted, Extra Fast, Cursed |
| The Countess | La Comtesse | Forgotten Tower L5 | Fire Enchanted, Cold Enchanted, Cursed (boss Q4) |
| Pitspawn Fouldog | Chien-Fosse | Pit L1 | Cold Enchanted, Stone Skin |
| Bone Ash | Cendre d'Os | Cathedral | Fire Enchanted, Extra Strong |
| Flamespike the Crawler | Rampant-Flamme | Pit L2 | Fire Enchanted, Extra Strong |
| Bonebreaker | Brise-Os | Cathedral | Spectral Hit, Stone Skin |
| Coldcrow | Corneille Glacee | Cave L2 | Cold Enchanted, Extra Fast |
| Treehead WoodFist | Tete-d'Arbre | Dark Wood | Extra Strong, Stone Skin |
| Andariel | Andaria | Catacombs L4 | BOSS (voir 2.5) |

### 2.4 Quetes de l'Acte 1 (6 quetes)

#### Q1 — Den of Evil (Antre du Mal)

**Nom SD :** Purge de l'Antre
**Donneur :** Syrene (Akara) — dialogue automatique a l'entree en jeu
**Zone :** Lande de Sang → Antre du Mal
**Objectif :** Tuer tous les monstres dans l'Antre du Mal (grotte adjacente a la Lande de Sang)
**Trigger :** Entrer dans l'Antre du Mal et tuer le dernier monstre
**Completion :** Retour a Syrene
**Recompense :** 1 point de skill supplementaire + XP selon niveau

**Mecanique :** Le compteur de monstres restants s'affiche. La grotte contient environ 30-50 monstres + Corpse-Feu (super-unique). Completion obligatoire pour le first skill point bonus.

**Multi-difficulte :** Quete repete en Cauchemar et Enfer, chaque completion donne +1 skill point.

---

#### Q2 — Sisters' Burial Grounds (Cimetiere des Soeurs)

**Nom SD :** Profanation du Cimetiere
**Donneur :** Brova (Kashya)
**Zone :** Cimetiere → boss Corbeau-Sang (Bloodraven)
**Objectif :** Tuer Corbeau-Sang, ancienne commandante Rogue maintenant non-morte
**Trigger :** Entrer dans le Cimetiere
**Completion :** Mort de Corbeau-Sang
**Recompense :** Mercenaire gratuit (1 Rogue loue gratuitement par Brova)

**Mecanique :** Corbeau-Sang attaque a distance (arc), se deplace rapidement, peut ressusciter des monstres. Elle est Cold Enchanted + Cursed. Elle ne drop pas d'objet de quete.

---

#### Q3 — The Search for Cain (La Recherche d'Elias)

**Nom SD :** Le Sage Perdu
**Donneur :** Syrene (Akara) — apres avoir vu le portail dans Champ de Pierre
**Zones :** Champ de Pierre → Village des Damnes (Tristram) → Fort Karak
**Objectif :** Activer le Cercle de Pierres Levees dans Champ de Pierre pour ouvrir un portail, entrer dans le Village des Damnes, sauver Elias (Deckard Cain)
**Trigger :** Activer le portail en cliquant sur Rakanishi (l'activation tue Rakanishi et ouvre le portail)
**Completion :** Retour a Fort Karak avec Elias sauve
**Recompense :** Elias identifie tous items gratuitement (sinon 100 gold/item)

**Mecanique :** Le Village des Damnes contient Forgeron Damne (Griswold), boss puissant. Elias est dans une cage au centre du village. Wirt (Thorn) est aussi dans le village — il vend des items mais est un vendeur unique avec 1 item par session.

**Note :** Si Elias n'est pas sauve, il reste disponible en ville mais facture l'identification.

---

#### Q4 — The Forgotten Tower (La Tour Oubliee)

**Nom SD :** Les Secrets de la Tour
**Donneur :** Note de journal (Letter from Griswold) trouvee dans le Marecage Noir
**Zone :** Marecage Noir → Tour Oubliee N1-N5
**Objectif :** Descendre les 5 niveaux de la Tour Oubliee et tuer La Comtesse
**Trigger :** Trouver/lire la note de journal (facultatif — peut aller directement)
**Completion :** Mort de La Comtesse
**Recompense :** La Comtesse drope toujours plusieurs runes (quantite et qualite selon difficulte)

**Mecanique — La Comtesse (Countess) :**
- Fire Enchanted + Cold Enchanted + Cursed
- Drop garanti : 3-6 runes en Normal, jusqu'a Io (r16) ; NM jusqu'a Ist (r24) ; Hell jusqu'a Gul (r25)
- Peut drop des runes dans le chest de la chambre aussi

**Noms SD runetable La Comtesse :**
- Normal : runes niveau 1-16 (El a Io)
- Nightmare : runes niveau 1-24 (El a Ist)
- Hell : runes niveau 1-25 (El a Gul)

---

#### Q5 — Tools of the Trade (Outils du Metier)

**Nom SD :** L'Enclume du Forgeron
**Donneur :** Forga (Charsi)
**Zone :** Baraquements → Enclume Horadrique (item fixe dans Baraquements)
**Objectif :** Recuperer l'Enclume Horadrique (Horadric Malus) dans les Baraquements
**Trigger :** Parler a Forga
**Completion :** Rapporter l'Enclume a Forga
**Recompense :** Forga peut maintenant "imbuer" (socketiser en unique) n'importe quel item normal — 1 utilisation par difficulte

**Mecanique Imbue :**
- Transforme un item normal en item unique de meme type
- Si l'item a deja un unique, peut generer un double
- Ne fonctionne pas sur les socketed items
- Recommandation : utiliser sur coronet ou circlet de haut niveau

---

#### Q6 — Sisters to the Slaughter (Les Soeurs au Massacre)

**Nom SD :** La Chute d'Andaria
**Donneur :** Auto (apres Catacombes N3)
**Zone :** Catacombes N4 — Zone FIXE
**Objectif :** Tuer Andaria (Andariel), demon de la Douleur
**Trigger :** Entrer dans Catacombes N4
**Completion :** Mort d'Andaria
**Recompense :** Acces Acte 2 via Darak (Warriv) + XP + Gold

### 2.5 Boss — Andaria (Andariel)

**Nom D2 :** Andariel, Maiden of Anguish
**Nom SD :** Andaria, Vierge du Supplice

**Localisation :** Catacombes N4 (carte fixe)
**Type :** Evil Boss — immune au poison, tres faible au feu

**Stats (Normal) :**
- Vie : ~3000 HP
- Defense : 200
- AR : 250
- Resistances : Poison Immune, Feu -50% (faiblesse), Froid 0%, Eclair 0%, Physique 0%

**Attaques :**
1. **Nova Venin** : projette un anneau de boules de poison, degats sur duree, portee moyenne
2. **Crachat Acide** : projectile direct, poison fort + ralentissement
3. **Coup de Griffe** : melee rapide, degats physiques + petit poison
4. **Nuage Mephitique** (enrage) : aura de poison en melee

**Mecanique de combat :**
- Immunite totale au poison (toutes attaques poison = 0 degat)
- Tres sensible au feu (degats x1.5 environ)
- Invocations : spawne de petits scorpions venimeux autour d'elle
- Ne charge pas, reste a portee moyenne-courte
- Enrage si > 50% monstres proches tues

**Drops garantis :**
- Ear de boss (PvP token, inutile en solo)
- Items magiques/rares + gold
- Possibilite de drop Set/Unique selon tier Normal

**Lore SD :** Andaria, demon de la douleur, a corrompu le monastere et transforme les Soeurs Rogue en ses servantes. Elle fut lachee en avant-garde par Belzaroth (D2 : Diablo) pour bloquer les heros.

---

## 3. Acte 2 — Les Sables de Khemra

**Nom D2 original :** Act 2 — Lut Gholein
**Nom SD :** Acte 2 — Les Sables de Khemra
**Ville :** Oasis de Khemra (D2 : Lut Gholein)
**Theme :** Desert, cites antiques, catacombes de sable, cryptes de pharaons

### 3.1 NPCs de l'Oasis de Khemra

| NPC (D2) | Nom SD | Role / Services |
|----------|--------|----------------|
| Atma | Nasha | Taverne, donne Q1 (Radament), achete/vend potions |
| Drognan | Seraph | Mage, vend sorts/staves/wands, identifie, Q4 (Staff) |
| Fara | Alima | Forgeron/Paladin, repare, rachete, vend armures/armes |
| Elzix | Barak | Marchand, vend casques/boucliers/armures lourdes |
| Greiz | Comandar | Chef mercenaires, loue Desert Wolves (lanciers) |
| Jerhyn | Vizir | Gouverneur, donne acces Sewers, lore |
| Lysander | Veritas | Alchimiste, vend potions/mana/antidotes |
| Meshif | Kaptan | Marin, transport acte 3 apres Q6 |
| Warriv | Darak | Caravannier, retour acte 1 |
| Deckard Cain | Elias | Identification gratuite (si sauve acte 1) |

**Mercenaires Comandar (Greiz) :**
- Type : Desert Wolves (lanciers)
- Variantes : Offensive (degats+), Defensive (life+), Combat (balanced)
- Attaque : lance physique, pas de magie elemental de base
- Aura disponible : Prayer (regen vie)

### 3.2 Zones de l'Acte 2

| Zone (D2) | Nom SD | Type | alvl N/NM/H | Waypoint |
|-----------|--------|------|-------------|----------|
| Lut Gholein | Oasis de Khemra | Ville | — | Oui |
| Lut Gholein Sewers L1 | Egouts de Khemra N1 | Souterrain | 11/42/70 | Non |
| Lut Gholein Sewers L2 | Egouts de Khemra N2 | Souterrain | 12/42/70 | Non |
| Lut Gholein Sewers L3 | Egouts de Khemra N3 | Souterrain | 12/42/70 | Oui (Repaire) |
| Rocky Waste | Plaine Aride | Exterieur | 12/40/68 | Oui |
| Dry Hills | Collines Seches | Exterieur | 13/40/68 | Non |
| Halls of the Dead L1 | Salles des Morts N1 | Souterrain | 13/40/68 | Non |
| Halls of the Dead L2 | Salles des Morts N2 | Souterrain | 14/40/69 | Non |
| Halls of the Dead L3 | Salles des Morts N3 | Souterrain | 15/40/69 | Non |
| Far Oasis | Oasis Lointaine | Exterieur | 13/40/68 | Oui (Oasis Sombre) |
| Lost City | Cite Perdue | Exterieur | 16/41/69 | Non |
| Ancient Tunnels | Tunnels Antiques | Souterrain | 17/43/85 | Non |
| Valley of Snakes | Vallee des Serpents | Exterieur | 16/41/70 | Non |
| Claw Viper Temple L1 | Temple des Viperes N1 | Souterrain | 17/42/70 | Non |
| Claw Viper Temple L2 | Temple des Viperes N2 | Souterrain | 17/42/70 | Non |
| Harem L1-L2 | Harem N1-N2 | Palais | 18/42/70 | Non |
| Palace Cellar L1-L3 | Cave du Palais N1-N3 | Souterrain | 18-20/42-44/70-72 | Oui (Allee Portail) |
| Arcane Sanctuary | Sanctuaire Arcanique | Magique | 20/43/72 | Oui (Dedale) |
| Canyon of the Magi | Canyon des Mages | Exterieur FIXE | 20/43/72 | Oui (Vallee Rois) |
| Tal Rasha's Tomb (false x6) | Tombes de Tal-Rashim (fausses) | Souterrain | 20/43/72 | Non |
| Tal Rasha's Tomb (true) | Tombe Veritable de Tal-Rashim | Souterrain FIXE | 20/43/72 | Non |
| Tal Rasha's Chamber | Chambre de Tal-Rashim | Souterrain | 20/43/72 | Non |

### 3.3 Super-Uniques de l'Acte 2

| Super-Unique (D2) | Nom SD | Zone | Modifieurs |
|-------------------|--------|------|------------|
| Radament | Radament | Sewers L3 | Extra Strong, Magic Resistant (boss Q1) |
| The Summoner | L'Invocateur | Arcane Sanctuary | Magic Resistant, Teleportation (boss Q4) |
| Ancient Kaa the Soulless | Kaa l'Ame-Morte | Halls of Dead L3 | Extra Strong, Magic Resistant |
| Creeping Feature | Rampant Hideux | Dry Hills | Extra Strong, Cursed |
| Blood Witch the Wild | Sorciere Sauvage | Halls of Dead L2 | Fire Enchanted, Cursed |
| Beetleburst | Scarabee-Eclat | Far Oasis | Stone Skin, Extra Strong |
| Leatherarm | Bras-de-Cuir | Lost City | Cold Enchanted, Stone Skin |
| Coldworm the Burrower | Froid-Ver | Maggot Lair L3 | Cold Enchanted, Extra Fast |
| Fire Eye | Oeil-de-Feu | Halls of Dead (outer) | Fire Enchanted, Extra Fast |
| Dark Elder | Ancien Sombre | Lost City | Lightning Enchanted |
| Duriel | Dourial | Tal Rasha Chamber | BOSS (voir 3.5) |

### 3.4 Quetes de l'Acte 2 (6 quetes)

#### Q1 — Radament's Lair (Le Repaire de Radament)

**Nom SD :** La Menace des Egouts
**Donneur :** Nasha (Atma)
**Zone :** Egouts de Khemra N1-N3 → boss Radament
**Objectif :** Tuer Radament dans les Egouts de Khemra
**Trigger :** Parler a Nasha en entrant en ville
**Completion :** Mort de Radament
**Recompense :** Livre de Skills (permet d'apprendre 1 skill supplementaire dans l'arbre), reduction prix Nasha

**Mecanique :** Radament est un mummy champion tres puissant. Il lance des sorts de necromancien (revive, bone spear). Il drope automatiquement le Livre de la Loi (Book of Skill).

---

#### Q2 — The Horadric Staff (Le Baton Horadrique)

**Nom SD :** L'Assemblee du Baton
**Donneur :** Seraph (Drognan)
**Zones multiples :** Salles des Morts N3 (Cube Horadrique) + Temple des Viperes N2 (Amulet Viper) + Oasis Lointaine (Maggot Lair, Horadric Shaft)
**Objectif :**
1. Trouver le Cube Horadrique dans Salles des Morts N3
2. Trouver le Shaft (manche) du Baton Horadrique dans le Repaire des Asticots N3
3. Trouver la Tete (head) du Baton dans Temple des Viperes N2
4. Combiner dans le Cube pour creer le Baton Horadrique
**Trigger :** Trouver le Cube Horadrique
**Completion :** Possession du Baton Horadrique
**Recompense :** Cube Horadrique (objet de crafting permanent), XP

**Mecanique Cube Horadrique :**
- Inventaire 3x4 en grille
- Formules de crafting : transmutation d'items, fusion de gemmes, crafting runes
- Reste dans l'inventaire de facon permanente
- Central au gameplay de crafting en Sodomight

---

#### Q3 — Tainted Sun (Le Soleil Souille)

**Nom SD :** L'Eclipse Maudite
**Donneur :** Auto (l'eclipse se declenche a l'entree en Vallee des Serpents)
**Zone :** Vallee des Serpents → Temple des Viperes N2 → Amulet a detruire
**Objectif :** Detruire l'Amulet de la Vipere dans le Temple des Viperes N2 (zone + boss Fangskin gardien)
**Trigger :** Entrer en Vallee des Serpents (le ciel devient rouge/eclipse)
**Completion :** Destruction de l'Amulet par Fangskin
**Recompense :** Eclipse levee, Seraph donne Antidote Potions + XP

**Mecanique :** Fangskin est un gardien de l'amulet. L'Amulet est sur un autel dans Temple N2. La detruire restaure le soleil et debarre l'eclipse.

---

#### Q4 — Arcane Sanctuary (Le Sanctuaire Arcanique)

**Nom SD :** Le Labyrinthe du Sorcier
**Donneur :** Vizir (Jerhyn)
**Zone :** Palais → Harem → Cave du Palais → Sanctuaire Arcanique → Invocateur
**Objectif :** Trouver le Sanctuaire Arcanique en traversant le Palais, puis tuer l'Invocateur et recuperer le Journal de Tal-Rashim
**Trigger :** Parler au Vizir apres avoir fini Q1-Q2
**Completion :** Mort de l'Invocateur + journal recupere
**Recompense :** Connaissance de la tombe correcte parmi 7 dans Canyon des Mages (symbol sur journal)

**Mecanique :** Le Sanctuaire est une zone flottante avec 4 bras levitant. L'Invocateur est un necromancien. Il drope le Journal de Tal-Rashim qui indique par un glyphe quelle des 7 tombes dans le Canyon est la vraie.

---

#### Q5 — The Tomb of Tal Rasha (La Tombe de Tal-Rashim)

**Nom SD :** Le Sacrifice Eternel
**Donneur :** Elias (Deckard Cain) apres Q4
**Zone :** Canyon des Mages → Tombe Veritable de Tal-Rashim → Chambre de Tal-Rashim
**Objectif :** Trouver la bonne tombe (indiquee par le journal), y entrer, utiliser le Baton Horadrique sur le Cube-Orifice pour ouvrir la chambre de Tal-Rashim
**Trigger :** Avoir le Baton Horadrique + journal
**Completion :** Ouverture de la chambre (Dourial spawne)
**Recompense :** Acces au boss Dourial (Duriel) — necessaire pour l'acte 3

**Mecanique :** Inseerer le Baton dans l'orifice detruit le Baton (il disparait de l'inventaire). La chambre de Tal-Rashim est une salle fixe avec Dourial en embuscade.

---

#### Q6 — Seven Tombs (Les Sept Tombes)

**Nom SD :** La Delivrance de Tal-Rashim
**Donneur :** Auto (completion logique de Q5)
**Zone :** Chambre de Tal-Rashim
**Objectif :** Tuer Dourial (Duriel) pour liberer Tal-Rashim et trouver Tyrael (Tyr-Ael)
**Trigger :** Entrer dans la chambre
**Completion :** Mort de Dourial + dialogue Tyr-Ael
**Recompense :** Acces acte 3 via Kaptan (Meshif) + dialogue Tyr-Ael + XP massif

### 3.5 Boss — Dourial (Duriel)

**Nom D2 :** Duriel, Lord of Pain
**Nom SD :** Dourial, Seigneur de la Souffrance

**Localisation :** Chambre de Tal-Rashim (chambre etroite, pas d'espace pour s'eloigner)
**Type :** Evil Boss — immunite physique partielle, aura Holy Freeze

**Stats (Normal) :**
- Vie : ~6000 HP
- Defense : 400
- AR : 400
- Resistances : Froid 110% (immune), Feu 50%, Eclair 50%, Physique 0%, Magie 0%

**Attaques :**
1. **Charge** : fonce sur le joueur, degats eleves, renverse
2. **Griffe Rapide** : combo 2-3 coups melee rapides, physique
3. **Holy Freeze Aura** : aura permanente, ralentit tout autour (comme paladin aura)
4. **Knockback** : certains coups projettent le joueur

**Mecanique de combat :**
- La chambre est tres petite — impossible de fuir ou kiter facilement
- Immunite au froid (Holy Freeze ne s'applique pas sur Dourial lui-meme)
- Pas de regeneration de mana dans la chambre (anti-spam sorts)
- Strategie classique : Town Portal pres de l'entree pour fuir, preparer potions avant, utiliser mercenaire comme tank
- Utiliser Tomes de TP avant d'entrer (stock max)

**Drops garantis :**
- Items magiques/rares de bon niveau
- Bonne chance de drop Set/Unique acte 2

**Lore SD :** Dourial fut place par Belzaroth dans la tombe pour garder son frere Tal-Rashim emprisonne. Tuer Dourial libere l'ame de Tal-Rashim et permet a Tyr-Ael de guider les heros vers Vezir.

---

## 4. Acte 3 — La Jungle de Vezir

**Nom D2 original :** Act 3 — Kurast
**Nom SD :** Acte 3 — La Jungle de Vezir
**Ville :** Port de Vezir (D2 : Kurast Docks)
**Theme :** Jungle tropicale, cite zakarum corrompue, temples profanes, Durance de Haine

### 4.1 NPCs du Port de Vezir

| NPC (D2) | Nom SD | Role / Services |
|----------|--------|----------------|
| Hratli | Veran | Forgeron, repare, rachete, vend armures/armes |
| Alkor | Ziko | Alchimiste, vend potions/antidotes/throws, identifie |
| Ormus | Phaelos | Mage, vend sorts Sorceress/Necromancer, identifie |
| Asheara | Dalya | Chef mercenaires, loue Iron Wolves (mages) |
| Cain | Elias | Identification gratuite |
| Meshif | Kaptan | Transport, retour acte 2 |
| Natalya | Veria | Assassin legendaire (dialogue uniquement, loot unique si trop pres boss) |

**Mercenaires Dalya (Asheara) :**
- Type : Iron Wolves (mages)
- Variantes : Feu, Froid, Eclair
- Attaque : sort elementaire a distance
- Aura : Vigor (froid) ou Holy Freeze (feu) selon variante

### 4.2 Zones de l'Acte 3

| Zone (D2) | Nom SD | Type | alvl N/NM/H | Waypoint |
|-----------|--------|------|-------------|----------|
| Kurast Docks | Port de Vezir | Ville | — | Oui |
| Spider Forest | Foret des Araignees | Exterieur | 21/45/74 | Non |
| Great Marsh | Grand Marecage | Exterieur | 21/45/74 | Non |
| Flayer Jungle | Jungle des Ecorcheurs | Exterieur | 22/47/74 | Oui (Flats) |
| Lower Kurast | Kurast Inferieure | Cite | 22/47/75 | Oui (Bazaar Inf.) |
| Kurast Bazaar | Bazaar de Kurast | Cite | 23/48/75 | Oui (Bazaar Sup.) |
| Upper Kurast | Kurast Superieure | Cite | 24/49/75 | Oui (Grand Marche) |
| Kurast Causeway | Chaussee de Kurast | Cite | 25/50/75 | Non |
| Travincal | Place du Conseil FIXE | Cite | 25/50/75 | Non |
| Spider Cavern | Caverne des Araignees | Grotte | 21/45/74 | Non |
| Flayer Dungeon L1-L3 | Donjon des Ecorcheurs N1-N3 | Souterrain | 22-24/47-49/74-76 | Non |
| Sewers L1-L2 | Egouts de Kurast N1-N2 | Souterrain | 23/48/75 | Oui (Precinct) |
| Ruined Temple | Temple Ruine | Exterieur | 24/49/75 | Oui |
| Disused Fane | Sanctuaire Abandonne | Exterieur | 24/49/75 | Non |
| Forgotten Reliquary | Reliquaire Oublie | Exterieur | 24/49/75 | Non |
| Forgotten Temple | Temple Oublie | Souterrain | 24/49/75 | Non |
| Ruined Fane | Fane en Ruines | Souterrain | 24/49/75 | Non |
| Disused Reliquary | Reliquaire Desaffecte | Souterrain | 24/49/75 | Non |
| Durance of Hate L1 | Demeure du Supplice N1 | Souterrain | 27/55/79 | Non |
| Durance of Hate L2 | Demeure du Supplice N2 | Souterrain | 27/55/79 | Oui (Mephistorum L2) |
| Durance of Hate L3 | Demeure du Supplice N3 | Souterrain FIXE | 28/56/79 | Non |

### 4.3 Super-Uniques de l'Acte 3

| Super-Unique (D2) | Nom SD | Zone | Modifieurs |
|-------------------|--------|------|------------|
| Sszark the Burning | Sszark l'Ardent | Spider Cavern | Fire Enchanted, Cursed |
| Icehawk Riftwing | Riftwing Givrant | Spider Forest | Cold Enchanted, Extra Fast |
| Stormtree | Arbre-Tempete | Lower Kurast | Lightning Enchanted, Stone Skin |
| Battlemaid Sarina | Sarina Battlemaid | Kurast Sewers | Fire Enchanted, Extra Strong |
| Toorc Icefist | Poing-de-Glace | Flayer Jungle | Cold Enchanted, Stone Skin |
| Ismail Vilehand | Villemain | Travincal | Fire Enchanted, Cursed (Council) |
| Geleb Flamefinger | Doigt-de-Flamme | Travincal | Fire Enchanted, Extra Strong (Council) |
| Bremm Sparkfist | Poing-Etincelle | Durance of Hate L2 | Lightning Enchanted (Council) |
| Wyand Voidbringer | Voide-Porteur | Durance of Hate L2 | Magic Resistant (Council) |
| Maffer Dragonhand | Main-de-Dragon | Durance of Hate L2 | Fire Enchanted (Council) |
| Mephisto | Mephistor | Durance L3 | BOSS (voir 4.5) |

### 4.4 Quetes de l'Acte 3 (6 quetes)

#### Q1 — The Golden Bird (L'Oiseau d'Or)

**Nom SD :** Le Talisman de Viz-Jun
**Donneur :** Auto (drop d'un oiseau dore tue par monstre en Jungle)
**Zone :** Jungle de Vezir → Port de Vezir → Kaptan → Phaelos
**Objectif :** Ramasser l'Oiseau Dore (token drop), le remettre a Kaptan pour un parchemin, puis au Phaelos
**Trigger :** Tuer le monstre qui drope l'Oiseau Dore
**Completion :** Phaelos donne la Jade Figurine → Kaptan donne la Golden Bird → Elias
**Recompense :** Elias donne une Potion Permanente de Vie (+20 Life permanents au personnage)

---

#### Q2 — Blade of the Old Religion (Lame de l'Ancienne Foi)

**Nom SD :** Le Flambeau de Khalim
**Donneur :** Veran (Hratli)
**Zones :** Jungle, Cites de Kurast — 4 items de quete a recuperer
**Objectif :** Assembler le Flambeau de Khalim depuis 4 reliques :
1. Coeur de Khalim : Araignee Caverne (boss Sszark)
2. Cerveau de Khalim : Flayer Dungeon N3
3. Oeil de Khalim : Sewers de Kurast N2
4. Flambeau de Khalim (item de base) : Kurast Inferieure (chest fixe)
**Assemblee dans Cube :** Coeur + Cerveau + Oeil + Flambeau → Flambeau de Khalim
**Trigger :** Parler a Veran
**Completion :** Frapper la Warp-Orbe avec le Flambeau dans Travincal
**Recompense :** La Warp-Orbe detruite ouvre l'acces a la Demeure du Supplice

**Mecanique :** La Warp-Orbe est indestructible par les sorts normaux — uniquement le Flambeau la detruit. Frapper la Warp-Orbe est l'action de fin de quete. Le Flambeau disparait ensuite.

---

#### Q3 — Khalim's Will (La Volonte de Khalim)

**Nom SD :** (integre dans Q2 ci-dessus — SD fusionnera Q2+Q3)
**Note :** En D2 original, Q2 = trouver les 3 reliques, Q3 = assembler et detruire l'orbe. Pour Sodomight, fusionner en une seule quete pour la clarte.

---

#### Q4 — Lam Esen's Tome (Le Tome de Lam-Esen)

**Nom SD :** Le Livre du Savoir Zakarum
**Donneur :** Ziko (Alkor)
**Zone :** L'un des 6 petits temples/reliquaires autour de Kurast Superieure/Bazaar
**Objectif :** Trouver le Tome de Lam-Esen dans l'un des 6 petits temples aleatoires
**Trigger :** Parler a Ziko
**Completion :** Remettre le Tome a Ziko
**Recompense :** +5 points de stats libres (redistribuables)

**Mecanique :** Les 6 petits temples (Ruined Temple, Disused Fane, Forgotten Reliquary, Forgotten Temple, Ruined Fane, Disused Reliquary) sont generes aleatoirement autour de Kurast. Le Tome est dans l'un d'eux.

---

#### Q5 — The Blackened Temple (Le Conseil du Temple Noirci)

**Nom SD :** Le Conseil des Corrompus
**Donneur :** Veran (Hratli) — apres decouverte de Travincal
**Zone :** Travincal (carte fixe)
**Objectif :** Tuer les 3 membres du Conseil du Zakarum corrompus dans Travincal
**Trigger :** Entrer dans Travincal
**Completion :** Mort des 3 membres du Conseil (dont Ismail Vilehand, Geleb Flamefinger, un 3e aleatoire)
**Recompense :** XP + items

**Mecanique :** Les membres du Conseil sont tres puissants : ils lancent des hydras, ont immunity au feu pour certains, font des degats massifs. Ils respawnent si le joueur quitte la zone. La Warp-Orbe est dans Travincal.

---

#### Q6 — The Guardian (Le Gardien de l'Enfer)

**Nom SD :** La Chute de Mephistor
**Donneur :** Auto
**Zone :** Demeure du Supplice N3 (carte fixe)
**Objectif :** Tuer Mephistor (Mephisto)
**Trigger :** Entrer dans Demeure N3
**Completion :** Mort de Mephistor
**Recompense :** Acces acte 4 + stone of jordan possible + drops rares

### 4.5 Boss — Mephistor (Mephisto)

**Nom D2 :** Mephisto, Lord of Hatred
**Nom SD :** Mephistor, Seigneur de la Haine

**Localisation :** Demeure du Supplice N3 (carte fixe)
**Type :** Evil Boss — presence d'un fosse d'eau autour de lui (moat)

**Stats (Normal) :**
- Vie : ~10000 HP
- Defense : 500
- AR : 500
- Resistances : Froid 75%, Eclair 75%, Feu 0%, Poison 0%, Physique 33%

**Attaques :**
1. **Eclair en Chaine** : eclair qui rebondit entre cibles multiples
2. **Gel** : projectile de glace, ralentit + degats froid
3. **Griffe Venin** : melee, poison sur duree
4. **Nova de Froid** : anneau de froid autour de lui
5. **Malemalefice** : peut maudire (Decreptify) les joueurs proches

**Mecanique Moat Trick :**
- Mephistor est entoure d'un fosse d'eau (moat) qui le protege
- Les sorts a longue portee peuvent l'atteindre par dessus le fosse
- Les attaques corps-a-corps necessitent de traverser le fosse
- Les archers/lanceurs de sort peuvent l'attaquer en securite depuis le bord
- Strategie classique : se positionner au bord du fosse, attaque a distance

**Drops garantis :**
- Stone of Jordan (chance augmentee significativement)
- Items de set/unique de niveau acte 3
- Le Tome de Lamentation (quest item pour acte 4 dans certaines versions)

**Lore SD :** Mephistor, Seigneur de la Haine, controla la cite de Vezir via le Conseil du Zakarum corrompu. Sa mort libere Kurast mais permet aux forces des Enfers de se concentrer sur Pandemonium.

---

## 5. Acte 4 — Les Plaines Maudites

**Nom D2 original :** Act 4 — Pandemonium Fortress
**Nom SD :** Acte 4 — Les Plaines Maudites
**Ville :** Forteresse du Pandemonium (D2 : Pandemonium Fortress)
**Theme :** Enfers, lave, zones maudites, sanctuaire du chaos

### 5.1 NPCs de la Forteresse du Pandemonium

| NPC (D2) | Nom SD | Role / Services |
|----------|--------|----------------|
| Tyrael | Tyr-Ael | Archange, lore, donne Q1 |
| Jamella | Sevra | Pretre, vend sorts/staves/wands, potions, identifie |
| Halbu | Baldar | Forgeron, repare, rachete, vend armes/armures |
| Deckard Cain | Elias | Identification gratuite |

**Acte 4 : PAS de mercenaire disponible pour louage.** Les mercenaires existants continuent si actifs.

### 5.2 Zones de l'Acte 4

| Zone (D2) | Nom SD | Type | alvl N/NM/H | Waypoint |
|-----------|--------|------|-------------|----------|
| Pandemonium Fortress | Forteresse du Pandemonium | Ville | — | Oui |
| Outer Steppes | Etapes Exterieures | Enfers | 25/55/80 | Non |
| Plains of Despair | Plaines du Desespoir | Enfers | 26/56/80 | Non |
| City of the Damned | Cite des Damnes | Enfers | 27/57/80 | Oui |
| River of Flame | Fleuve de Flamme | Enfers | 28/58/80 | Oui |
| Chaos Sanctuary | Sanctuaire du Chaos FIXE | Enfers | 28/58/85 | Non |

### 5.3 Super-Uniques de l'Acte 4

| Super-Unique (D2) | Nom SD | Zone | Modifieurs |
|-------------------|--------|------|------------|
| Grand Vizier of Chaos | Grand Vizir du Chaos | Chaos Sanctuary | Fire Enchanted, Extra Fast (seal boss) |
| Lord De Seis | Seigneur De Seis | Chaos Sanctuary | Magic Resistant, Cursed (seal boss) |
| Infector of Souls | Infecteur d'Ames | Chaos Sanctuary | Fire Enchanted, Extra Strong (seal boss) |
| Izual | Izual | Plains of Despair | Extra Strong, Magic Resistant (Q2) |
| Hephasto the Armorer | Hephasto | River of Flame | Extra Strong, Stone Skin (Q3 item) |
| Diablo | Belzaroth | Chaos Sanctuary | BOSS (voir 5.5) |

### 5.4 Quetes de l'Acte 4 (3 quetes)

#### Q1 — The Fallen Angel (L'Ange Dechu)

**Nom SD :** La Redemption d'Izual
**Donneur :** Tyr-Ael
**Zone :** Plaines du Desespoir
**Objectif :** Trouver et tuer Izual (ancien ange de Tyr-Ael, maintenant possede)
**Trigger :** Parler a Tyr-Ael
**Completion :** Mort d'Izual + dialogue de son ame liberee
**Recompense :** +2 points de skill (de l'ame d'Izual reconnaissante)

**Mecanique :** Izual est un tres grand boss volant dans Plaines du Desespoir. Il est Extra Strong, Magic Resistant. Il revele (lore) que c'est lui qui a transmis les plans pour les Pierres de l'Ame aux Evils.

---

#### Q2 — Hell's Forge (La Forge des Enfers)

**Nom SD :** La Forge Infernale
**Donneur :** Tyr-Ael (apres Q1)
**Zone :** Fleuve de Flamme → boss Hephasto → Forge Infernale
**Objectif :**
1. Tuer Hephasto dans Fleuve de Flamme (drope le Marteau Mephisto)
2. Aller a la Forge Infernale (structure dans Fleuve de Flamme)
3. Placer la Pierre de l'Ame de Mephisto dans la Forge + frapper avec le Marteau
**Trigger :** Avoir la Pierre de l'Ame de Mephisto (item inventaire)
**Completion :** Destruction de la Pierre de Mephisto
**Recompense :** Gems (toutes parfaites) + Runes (jusqu'a Gul en Normal, meilleures en NM/Hell)

**Mecanique :** La Pierre de l'Ame ne peut etre detruite que dans la Forge Infernale avec le Marteau Mephisto. La destruction libere une explosion de gemmes et runes.

---

#### Q3 — Terror's End (La Fin de la Terreur)

**Nom SD :** La Defaite de Belzaroth
**Donneur :** Auto
**Zone :** Etapes Exterieures → Cite des Damnes → Fleuve de Flamme → Sanctuaire du Chaos
**Objectif :** Traverser les zones, ouvrir les 5 Sceaux dans le Sanctuaire du Chaos, tuer Belzaroth (Diablo)
**Trigger :** Entrer dans le Sanctuaire du Chaos
**Completion :** Mort de Belzaroth
**Recompense :** Acces Acte 5 (si LoD) + XP massif + drops endgame

### 5.5 Boss — Belzaroth (Diablo)

**Nom D2 :** Diablo, Lord of Terror
**Nom SD :** Belzaroth, Seigneur de la Terreur

**Localisation :** Sanctuaire du Chaos (carte fixe)
**Type :** Evil Boss — 5 sceaux, 3 gardes super-uniques

**Mecanisme des 5 Sceaux :**
Le Sanctuaire a 5 sceaux a activer dans 3 bras de la croix :
- **Bras gauche** : 2 sceaux, gardien Grand Vizir du Chaos
- **Bras droit** : 2 sceaux, gardien Infecteur d'Ames
- **Bras bas** : 1 sceau, gardien Seigneur De Seis
- Activer tous les sceaux fait spawner Belzaroth au centre

**Attention :** L'activation de sceaux peut spawner des vagues de monstres supplementaires.

**Stats (Normal) :**
- Vie : ~15000 HP
- Defense : 700
- AR : 700
- Resistances : Feu 50%, Froid 50%, Eclair 50%, Poison 0%, Physique 33%

**Attaques :**
1. **Lightning Hose** : rayon d'eclair continu, tourne autour de lui, degats massifs
2. **Fire Nova** : anneau de feu qui s'expand
3. **Bone Prison** (emprisonnement) : enferme le joueur dans une cage d'os temporaire
4. **Red Lightning Hose** : version renforcee du rayon, plus rapide
5. **Cold Touch** : melee, cold hit + slow
6. **Charge** : fonce sur le joueur, renverse

**Mecanique :**
- Lightning Hose est l'attaque la plus mortelle — s'esquive en sortant du cone
- Fire Nova est evitable avec bonne mobilite
- Bone Prison isole le joueur — briser vite ou utiliser TP
- Belzaroth est tres grand et lent a la course

**Drops garantis :**
- Tres bonne chance Set/Unique
- Ear du boss
- Items de niveau 28 (Normal)

---

## 6. Acte 5 — Les Bastions de Glace

**Nom D2 original :** Act 5 — Harrogath (Lord of Destruction)
**Nom SD :** Acte 5 — Les Bastions de Glace
**Ville :** Harrogath (conserve le nom — cite barbare)
**Theme :** Montagnes glaciales, forteresses de Baal, monde de la Pierrre Mondiale

### 6.1 NPCs de Harrogath

| NPC (D2) | Nom SD | Role / Services |
|----------|--------|----------------|
| Larzuk | Lazgar | Forgeron, repare, socketise items (Q1), vend armes/armures |
| Malah | Veda | Guerisseuse, vend potions/antidotes, identifie, soin |
| Qual-Kehk | Kragath | Chef de guerre, loue mercenaires barbares (Q3), donne Q3 |
| Nihlathak | Niharak | Alchimiste (traitre — disparait apres Q2), echange items |
| Anya | Lyra | Marchande (apres Q4), vend armures/armes haut niveau |
| Deckard Cain | Elias | Identification gratuite |

**Mercenaires Kragath (Qual-Kehk) :**
- Type : Barbares
- Variantes : Slayers, Guard, Combat
- Attaque : physique + Warcry (cri de guerre)
- Auras : Holy Freeze, Defiance, Prayer

**Note sur Niharak (Nihlathak) :**
- Echange la Relic (item special) contre des Runes
- Disparait de la ville apres la quete Q2 (il trahit)
- Devient boss dans Halls of Vaught

### 6.2 Zones de l'Acte 5

| Zone (D2) | Nom SD | Type | alvl N/NM/H | Waypoint |
|-----------|--------|------|-------------|----------|
| Harrogath | Harrogath | Ville | — | Oui |
| Bloody Foothills | Contreforts Sanglants | Exterieur | 24/58/80 | Oui |
| Frigid Highlands | Hautes Terres Frigides | Exterieur | 25/59/81 | Oui |
| Arreat Plateau | Plateau d'Arreat | Exterieur | 26/60/82 | Oui |
| Crystalline Passage | Passage Cristallin | Caverne glace | 29/63/83 | Oui |
| Frozen River | Riviere Gelée | Caverne glace | 29/63/83 | Non |
| Glacial Trail | Piste Glaciale | Caverne glace | 30/64/83 | Oui |
| Drifting Sands (Dunes) | Dunes Glaciales | Caverne glace | 27/61/82 | Non |
| Frozen Tundra | Toundra Glacee | Exterieur | 27/61/82 | Oui |
| The Ancients' Way | Chemin des Anciens | Exterieur | 37/63/83 | Oui |
| Icy Cellar | Cave Glaciale | Caverne glace | 38/64/83 | Non |
| Arreat Summit | Sommet d'Arreat | Zone speciale FIXE | 37/63/83 | Non |
| Nihlathak's Temple | Temple de Niharak | Souterrain | 38/64/83 | Non |
| Halls of Anguish | Salles du Tourment | Souterrain | 39/64/83 | Non |
| Halls of Pain | Salles de la Douleur | Souterrain | 40/65/83 | Oui |
| Halls of Vaught | Salles de Vaught FIXE | Souterrain | 45/67/85 | Non |
| Worldstone Keep L1 | Citadelle de la Pierre Mondiale N1 | Souterrain | 40/65/85 | Non |
| Worldstone Keep L2 | Citadelle de la Pierre Mondiale N2 | Souterrain | 41/66/85 | Non |
| Worldstone Keep L3 | Citadelle de la Pierre Mondiale N3 | Souterrain | 43/66/85 | Non |
| Throne of Destruction | Trone de la Destruction FIXE | Souterrain | 43/66/85 | Non |
| Worldstone Chamber | Chambre de la Pierre Mondiale | Zone speciale FIXE | 45/66/85 | Non |

### 6.3 Super-Uniques de l'Acte 5

| Super-Unique (D2) | Nom SD | Zone | Modifieurs |
|-------------------|--------|------|------------|
| Shenk the Overseer | Shenk le Surveillant | Bloody Foothills | Extra Strong, Cursed (Q1 boss) |
| Eldritch the Rectifier | Eldritch le Correcteur | Frigid Highlands | Extra Fast, Stone Skin |
| Eyeback the Unleashed | Oeil-Lache | Frigid Highlands | Extra Strong, Extra Fast |
| Thresh Socket | Seuil-Emboiture | Crystalline Passage | Cold Enchanted, Stone Skin |
| Pindleskin | Peau-Broches | Nihlathak's Temple | Cursed, Extra Strong |
| Nihlathak | Niharak | Halls of Vaught | Magic Resistant, Cold Enchanted (Q5 boss) |
| The Ancients (3) | Les Anciens (3) | Arreat Summit | BOSS Q6 (voir 6.4) |
| Lister the Tormentor | Lister le Tourmenteur | Throne of Destruction | Stone Skin, Magic Resistant (wave 5 Baal) |
| Colenzo the Annihilator | Colenzo l'Annihilateur | Throne of Destruction | Fire Enchanted (wave 1 Baal) |
| Achmel the Cursed | Achmel le Maudit | Throne of Destruction | Cursed (wave 2 Baal) |
| Bartuc the Bloody | Bartuc le Sanglant | Throne of Destruction | Extra Strong (wave 3 Baal) |
| Ventar the Unholy | Ventar l'Impie | Throne of Destruction | Extra Fast, Cursed (wave 4 Baal) |
| Baal | Baalthar | Worldstone Chamber | BOSS (voir 6.5) |

### 6.4 Quetes de l'Acte 5 (6 quetes)

#### Q1 — Siege on Harrogath (Siege de Harrogath)

**Nom SD :** Repousser l'Assaut
**Donneur :** Lazgar (Larzuk)
**Zone :** Contreforts Sanglants → boss Shenk le Surveillant
**Objectif :** Tuer Shenk le Surveillant dans les Contreforts Sanglants
**Trigger :** Parler a Lazgar
**Completion :** Mort de Shenk
**Recompense :** Lazgar peut maintenant ajouter des sockets a 1 item normal (le nombre de sockets est maximal selon le type d'item — deterministe selon la formule D2)

**Mecanique Sockets Lazgar :**
- Fonctionne uniquement sur items normaux (pas magiques/rares/uniques)
- Donne le maximum de sockets possible pour le type d'item
- 1 utilisation par difficulte
- Tres utile pour les items runeword

---

#### Q2 — Rescue on Mount Arreat (Sauvetage sur Arreat)

**Nom SD :** Les Barbares Prisonniers
**Donneur :** Kragath (Qual-Kehk)
**Zones :** Hautes Terres Frigides + Plateau d'Arreat
**Objectif :** Liberer 15 prisonniers barbares dans les deux zones (ils sont dans des cages)
**Trigger :** Parler a Kragath
**Completion :** Liberer les 15 prisonniers
**Recompense :** Runes (2-3 runes aleatoires selon difficulte)

**Mecanique :** Les prisonniers sont dans des cages interactables dans les deux zones exterieures. Les monstres les gardent. Les interagir les libere instantanement.

---

#### Q3 — Prison of Ice (Prison de Glace)

**Nom SD :** La Trahison de Niharak
**Donneur :** Lyra (Anya) — via parchemin ou vision
**Zone :** Passage Cristallin → Riviere Gelee (boss Niharak Tome)
**Objectif :** Trouver Lyra prisonniere dans la Riviere Gelee, la liberer avec le Parchemin de Kha'Daath (donne par Elias)
**Trigger :** Trouver la note de Lyra
**Completion :** Lyra liberee + retour en ville
**Recompense :** Lyra ouvre un magasin (armures/armes haut niveau) + Niharak fugit en ville avant de disparaitre + Pots d'Antidote + Thawing Potions de masse

**Mecanique :** Niharak trahit la ville. Il etait en contact avec Baalthar (Baal). Lyra est piege dans un block de glace dans Riviere Gelee.

---

#### Q4 — Betrayal of Harrogath (La Trahison de Harrogath)

**Nom SD :** La Traque du Traitre
**Donneur :** Kragath (Qual-Kehk) apres Q3
**Zone :** Temple de Niharak → Salles du Tourment → Salles de la Douleur → Salles de Vaught
**Objectif :** Retrouver et tuer Niharak dans les Salles de Vaught
**Trigger :** Q3 completee
**Completion :** Mort de Niharak
**Recompense :** Lyra peut maintenant personnaliser un item (ajoute le nom du personnage a l'item) — 1 par difficulte

**Mecanique Niharak (Pindleskin connexe) :**
- Niharak est dans Salles de Vaught (carte fixe)
- Il lance des explosions de cadavres massives (Corpse Explosion)
- Il est tres dangereux : ses corpse explosions one-shot facilement
- Peau-Broches (Pindleskin) garde l'entree du Temple
- Salles de Vaught = carte fixe optimale pour le farming (alvl 85)

---

#### Q5 — Rite of Passage (Le Rite de Passage)

**Nom SD :** Le Jugement des Anciens
**Donneur :** Elias (Deckard Cain) ou auto
**Zone :** Chemin des Anciens → Sommet d'Arreat (carte fixe)
**Objectif :** Interagir avec la Stele des Anciens sur le Sommet d'Arreat, puis vaincre les 3 Anciens : Madawc, Korlic, Talic
**Trigger :** Activer la stele
**Completion :** Mort des 3 Anciens
**Recompense :** XP enorme + acces a la Citadelle de la Pierre Mondiale

**Mecanique des 3 Anciens :**
Les Anciens sont 3 champions barbares avec des affinites aleatoires en NM/Hell (resistances/immunites generees aleatoirement a chaque partie) :
- **Madawc** (hache) : lance des haches, attaque a distance, dodge efficace
- **Korlic** (spear) : charge puissante, Leap Attack
- **Talic** (epee+bouclier) : Whirlwind, tres rapide
- Ils ne peuvent pas mourir en meme temps — si l'un meurt, les autres enragent
- Strategie : focaliser un a la fois, separer si possible

---

#### Q6 — Eve of Destruction (L'Eve de la Destruction)

**Nom SD :** La Fin de Baalthar
**Donneur :** Auto
**Zone :** Citadelle N1-N3 → Trone de la Destruction → Chambre de la Pierre Mondiale
**Objectif :**
1. Traverser la Citadelle de la Pierre Mondiale (3 niveaux)
2. Entrer dans le Trone de la Destruction, survivre aux 5 vagues
3. Tuer Baalthar (Baal) dans la Chambre de la Pierre Mondiale
**Trigger :** Entrer dans le Trone de la Destruction
**Completion :** Mort de Baalthar
**Recompense :** Credits de fin + XP massif + items endgame

### 6.5 Boss — Baalthar (Baal)

**Nom D2 :** Baal, Lord of Destruction
**Nom SD :** Baalthar, Seigneur de la Destruction

**Phase 1 — Trone de la Destruction : Les 5 Vagues**

Avant d'entrer dans la Chambre, Baalthar envoie 5 vagues de monstres depuis le trone :

| Vague | Boss de vague | Composition |
|-------|--------------|-------------|
| 1 | Colenzo l'Annihilateur | Shaman + monstres feu |
| 2 | Achmel le Maudit | Undeads maudits + wraiths |
| 3 | Bartuc le Sanglant | Berserkers physiques puissants |
| 4 | Ventar l'Impie | Wailing Beasts rapides |
| 5 | Lister le Tourmenteur | Champion ultra-puissant + minions |

Apres la vague 5, Baalthar disparait dans un portail vers la Chambre de la Pierre Mondiale.

**Phase 2 — Chambre de la Pierre Mondiale : Baalthar**

**Stats (Normal) :**
- Vie : ~70000 HP (le boss le plus coriace)
- Defense : 1000
- AR : 1000
- Resistances : Feu 20%, Froid 20%, Eclair 20%, Poison 0%, Physique 33%

**Attaques :**
1. **Clone** : cree une copie de lui-meme (decoy) qui attaque aussi — tuer le bon
2. **Incineration Nova** : nova de flammes spiralees en rotation, evitable avec mouvement perpendiculaire
3. **Hoarfrost** : cone de glace puissant, degats enormes
4. **Vortex** : tourbillon de vent qui aspire le joueur vers lui
5. **Tentacules** : invoque des tentacules du sol qui frappent
6. **Mana Burn** : attaque qui detruit le mana du joueur
7. **Melee puissante** : coup de poing geant, knockback

**Mecanique de combat :**
- **Clone** : Baalthar cree une copie parfaite. Les deux semblent identiques. La vraie copie est determinee par laquelle a ete attaquee en premier (la vraie perd de la vie). L'autre disparait si la vraie meurt.
- **Incineration Nova** : tourner autour de Baalthar dans le meme sens que la rotation annule les degats
- **Hoarfrost** : ne jamais etre dans le cone frontal
- Le combat se passe dans une grande salle — assez d'espace pour manoeuvrer

**Drops garantis :**
- Tres haut chance d'items Sets et Uniques de haut niveau
- XP bonus enorme
- Credits de fin de jeu

**Lore SD :** Baalthar, dernier des Trois Mauvais, tente de corrompre la Pierre Mondiale pour dominer toute creation. Sa mort par le heros declenche la destruction de la Pierre Mondiale par Tyr-Ael.

---

## 7. Schemas TOML

### 7.1 Schema Zone

```toml
# schema: zone.toml
# Utilisé dans mge/data/zones/

[zone]
id = "act1_blood_moor"           # @id UUID ou slug unique
sd_name = "Lande de Sang"        # Nom Sodomight
d2_name = "Blood Moor"           # Reference D2 originale
act = 1                          # Acte 1-5
type = "outdoor"                 # outdoor | dungeon | town | fixed

[zone.area_level]
normal = 1
nightmare = 36
hell = 67

[zone.connections]
# Zones adjacentes (slugs)
from = ["fort_karak"]
to = ["act1_cold_plains", "act1_den_of_evil"]

[zone.waypoint]
has_waypoint = true
waypoint_id = "wp_act1_blood_moor"

[zone.generation]
type = "procedural"              # procedural | fixed
tileset = "wilderness"           # wilderness | monastery | caves | desert | jungle | hell | ice

[zone.super_uniques]
# Liste des super-uniques fixes dans cette zone
ids = []

[zone.objects]
# Interactables dans la zone
chests = ["chest_generic", "chest_large"]
shrines = ["shrine_random"]
special = []

[zone.monsters]
# Families de monstres qui peuvent spawner
families = ["fallen", "zombie", "quill_rat"]
density_base = 0.6               # 0.0-1.0, densite de base
champion_chance = 0.05           # Chance de pack champion
```

### 7.2 Schema Quete

```toml
# schema: quest.toml
# Utilisé dans mge/data/quests/

[quest]
id = "act1_q1_den_of_evil"       # @id
sd_name = "Purge de l'Antre"
d2_name = "Den of Evil"
act = 1
quest_number = 1                  # 1-6 par acte

[quest.giver]
npc_id = "npc_syrene"            # NPC qui donne la quete
trigger_zone = "act1_blood_moor" # Zone d'entree qui trigger l'offre

[quest.objectives]
type = "kill_all"                # kill_all | kill_boss | retrieve_item | escort | activate
target_zone = "act1_den_of_evil"
target_id = ""                   # Pour kill_boss: id du boss
required_kills = -1              # -1 = tous les monstres de la zone

[quest.completion]
return_to_npc = "npc_syrene"
auto_complete = false            # true si completion sans retour NPC

[quest.rewards]
xp_base = 3000
gold_base = 0
skill_points = 1                 # Points de skill bonus
stat_points = 0
item_rewards = []                # Items garantis
npc_unlocks = []                 # NPCs debloquees
zone_unlocks = []                # Zones debloquees

[quest.difficulty]
# La quete se repete en NM et Hell avec les memes recompenses
repeatable_per_difficulty = true
reward_per_difficulty = { skill_points = 1 }
```

### 7.3 Schema Super-Unique

```toml
# schema: super_unique.toml

[super_unique]
id = "su_corpsefire"
sd_name = "Corpse-Feu"
d2_name = "Corpsefire"
zone_id = "act1_den_of_evil"
base_monster = "zombie"          # Famille de base

[super_unique.modifiers]
# Modifieurs fixes (toujours presents)
fixed = ["spectral_hit", "extra_fast"]
# Modifieurs aleatoires additionnels en NM/Hell (0-2 extras)
random_nm = 1
random_hell = 2

[super_unique.stats]
life_multiplier = 3.5            # Multiplicateur de vie vs monstre base
damage_multiplier = 2.0
defense_multiplier = 2.0
area_level_bonus = 3             # alvl base + 3 pour drop table

[super_unique.drops]
# Super-uniques droppent toujours plus que monstres normaux
treasure_class_bonus = 3         # TC bonus vs monstre normal de la zone
unique_chance_multiplier = 10.0  # Chance x10 de drop unique/set
```

### 7.4 Schema Boss d'Acte

```toml
# schema: act_boss.toml

[act_boss]
id = "boss_andaria"
sd_name = "Andaria"
d2_name = "Andariel"
act = 1
zone_id = "act1_catacombs_l4"
quest_id = "act1_q6_slaughter"

[act_boss.stats.normal]
life = 3000
defense = 200
attack_rating = 250
experience = 15000

[act_boss.stats.nightmare]
life = 45000
defense = 900
attack_rating = 1200
experience = 100000

[act_boss.stats.hell]
life = 130000
defense = 2500
attack_rating = 3500
experience = 400000

[act_boss.resistances]
fire = -50      # Negatif = faiblesse
cold = 0
lightning = 0
poison = 110    # >= 100 = immune
physical = 33
magic = 0

[act_boss.attacks]
# Liste des attaques avec parametres
[[act_boss.attacks.list]]
id = "nova_poison"
type = "projectile_ring"
damage_type = "poison"
damage_base = [50, 80]
poison_duration = 4.0
cooldown = 3.0

[[act_boss.attacks.list]]
id = "acid_spit"
type = "projectile"
damage_type = "poison"
damage_base = [30, 60]
slow_duration = 2.0
cooldown = 1.5

[[act_boss.attacks.list]]
id = "claw_strike"
type = "melee"
damage_type = "physical"
damage_base = [40, 70]
poison_bonus = [5, 10]
cooldown = 0.8

[act_boss.drops]
# Recompenses de mort garanties
guaranteed = []
treasure_class = "TreasureClassBossAct1"
unique_set_chance_multiplier = 15.0
```

---

*Document genere : 2026-02-28*
*Source : Recherche exhaustive D2 + LoD (Arreat Summit, PureDiablo, DiabloWiki)*
*Usage : Reference de conception pour Sodomight — clone spirituel non-commercial*
