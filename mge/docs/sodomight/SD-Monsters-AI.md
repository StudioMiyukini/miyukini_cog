# SD-Monsters-AI — Référence Exhaustive Monstres, IA & Bosses

**Projet :** Sodomight (clone Diablo 2 fidèle, assets maison)
**Crate cible :** `mge-arpg-ai`, `mge-arpg-entity`
**Moteur :** MGE (ECS archetype, data-driven TOML)
**Date :** 2026-02-28
**Portée :** Tous les monstres D2 + LoD — Normal, Nightmare, Hell

---

## Contexte

Sodomight est un clone fidèle de Diablo 2 Lord of Destruction. Tous les noms propres Blizzard
sont renommés (noms Sodomight entre parenthèses). Ce document couvre :

1. Classification des types de monstres
2. Familles de monstres D2 exhaustives
3. Tous les monstres par acte (stats N/NM/H)
4. Affixes de champions et uniques (tous les ~50)
5. Super Uniques — liste complète
6. Boss d'acte — mécaniques complètes
7. Archétypes d'IA et paramètres
8. Schémas TOML Sodomight
9. Système de scaling multijoueur

---

## 1. Classification des Types de Monstres

### 1.1 Catégories par rareté

| Catégorie | Couleur nom | Description |
|-----------|-------------|-------------|
| **Normal** | Blanc | Monstre de base, aucun affix |
| **Champion** | Bleu | Groupe de 3, 1–3 affixes fixes selon type de champion |
| **Unique/Rare** | Doré | 1 monstre nommé généré + pack d'acolytes, 4–6 affixes aléatoires |
| **Super Unique** | Doré (nom fixe) | Nom fixe, affixes fixes, spawn toujours au même endroit |
| **Boss d'acte** | Spécial | Unique, mécaniques propres, pas d'affixes aléatoires |
| **Minion** | Blanc/gris | Invoqué par joueur (nécromancien) ou par monstre |
| **Über** | Spécial | Variantes endgame post-jeu (patch 1.11+) |

### 1.2 Types fondamentaux (influence les modificateurs de skills)

| Type | Description | Immunité naturelle | Bonus reçu |
|------|-------------|-------------------|------------|
| **Demon** | Créatures démoniaques | — | Holy Bolt inefficace |
| **Undead** | Morts-vivants | — | 150% dmg armes contondantes ; Holy Bolt efficace |
| **Animal** | Créatures naturelles | — | Aucun bonus/malus |

> Note : "Animal" n'a pas de type affiché en jeu. La classification interne reste importante pour
> les skills (Sanctuary vs Undead, Zeal vs Demon, etc.).

### 1.3 Sous-types Champion

| Sous-type Champion | HP | Dmg | AR | Vitesse | Notes |
|-------------------|-----|-----|-----|---------|-------|
| **Champion** | ×3 (N), ×2.5 (NM), ×2 (H) | ×2 | ×2 | +33% | Standard |
| **Berserker** | ×1.5 | ×4 | ×4 | normal | Dégâts extrêmes, peu de vie |
| **Fanatic** | ×3/2.5/2 | ×2 | ×0.3 | +100% | Rapide mais peu précis |
| **Ghostly** | ×3/2.5/2 | +cold | normal | –20% | 80% résistance physique, translucide |
| **Possessed** | ×12 | normal | normal | normal | Immunisé aux malédictions, énorme pool de vie |

---

## 2. Familles de Monstres D2 — Vue d'ensemble

| Famille | Type | Actes principaux | Particularité IA |
|---------|------|-----------------|-----------------|
| Fallen | Demon | 1, 5 | Fuite à chaque kill, shaman ressuscite |
| Zombie | Undead | 1, 2 | Lent, encaisse beaucoup |
| Skeleton | Undead | 1, 2, 3 | Archers, mages, guerriers |
| Corrupt Rogue | Demon | 1 | Archers et lancières |
| Goatman | Demon | 1, 2, 3 | Corps-à-corps brutal |
| Spike Fiend | Animal | 1 | Coureur rapide |
| Blood Hawk | Animal | 1, 2 | Vol, attaque en piqué |
| Fallen Shaman | Demon | 1, 3, 5 | Ressuscite les Fallen morts |
| Wendigo | Animal | 1 | Brute résistante |
| Giant Spider | Animal | 1, 3 | Tir de toile, ralentit |
| Wraith / Ghost | Undead | 1, 2 | Vol, drain vie |
| Sand Raider | Demon | 2 | Melee rapide |
| Mummy / Greater Mummy | Undead | 2 | Ressuscite les mummies |
| Claw Viper | Animal | 2, 3 | Crache poison |
| Sand Maggot | Animal | 2 | Pondre des œufs (larves) |
| Scarab Demon | Animal | 2 | Charge électrique |
| Blunderbore | Demon | 2, 3 | Géant, AoE stun |
| Swarm / Itchies | Animal | 2 | Empoisonnement rapide |
| Vulture Demon | Animal/Demon | 2, 3 | Vole, loot rare |
| Fetish | Demon | 3 | Blowdart, dagger sprint |
| Fetish Shaman | Demon | 3 | Ressuscite Fetish |
| Thorned Hulk | Animal | 3 | Renvoi dégâts corps-à-corps |
| Council Member | Demon | 3 | Hydra, Lightning, Holy Shield |
| Willowisp / Gloam | Undead | 3, 5 | Lightning, mana drain |
| Zakarum Zealot | Animal | 3 | Fanaticism aura |
| Finger Mage | Undead | 4 | Fissures d'os |
| Oblivion Knight | Undead | 4 | Auras maudites (Iron Maiden, Decrepify…) |
| Megademon | Demon | 4 | Inferno, corps-à-corps |
| Vile Mother | Demon | 4 | Spawne des Vile Child |
| Saber Cat | Animal | 5 | Charge rapide |
| Frozen Horror | Animal | 5 | Cold immunity, glacial |
| Succubus | Demon | 5 | Séduction, Charged Bolt |
| Overseer | Demon | 5 | Whip, buff les minions |
| Death Mauler | Demon | 5 | Terrier, rebondit |
| Reanimated Horde | Undead | 5 | Revenant barbare |
| Minion of Destruction | Animal | 5 (Baal) | Immune au feu |
| Hell Bovine | Animal | Secret Cow Level | Normal melee, drop rare |

---

## 3. Monstres par Acte — Catalogue Exhaustif

### Conventions tableaux stats

- **HP** = vie de base (1 joueur)
- **HP/+P** = vie ajoutée par joueur supplémentaire (formule : `HP × (N+1)/2`)
- **Dmg** = dégâts min–max
- **AR** = Attack Rating de base
- **Rés.** = résistances Fire/Cold/Lightning/Poison (%)
- **Immun. Hell** = immunités en difficulté Hell
- **XP** = expérience donnée au niveau équivalent

---

### 3.1 Acte 1 — Rogue Encampment & Alentours

#### Fallen (Fallen / Carver / Devilkin / Dark One / Warped One)

| Variante | D2 Name | Sodomight Name | Zones | Stats Normal | Stats NM | Stats Hell |
|----------|---------|---------------|-------|-------------|---------|-----------|
| Tier 1 | Fallen | Chu'fa | Cold Plains, Stony Field | HP:10, Dmg:1–4 | HP:110, Dmg:12–22 | HP:237, Dmg:23–44 |
| Tier 2 | Carver | Karru | Tamoe Highland | HP:18, Dmg:3–6 | HP:140, Dmg:18–32 | HP:290, Dmg:36–60 |
| Tier 3 | Devilkin | Devlin | Dark Wood | HP:22, Dmg:4–8 | HP:175, Dmg:22–38 | HP:350, Dmg:45–72 |
| Tier 4 | Dark One | Darkon | Black Marsh | HP:28, Dmg:6–10 | HP:200, Dmg:28–48 | HP:400, Dmg:56–90 |
| Tier 5 | Warped One | Warpkin | Act 5 Foothills | HP:35, Dmg:8–15 | HP:240, Dmg:35–60 | HP:480, Dmg:70–112 |

**Type :** Demon | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 0, Froid 0, Foudre 0, Poison 0
**IA :** `pack_flee` — Fuient 2s à chaque kill ennemi proche ; shaman associé ressuscite

#### Fallen Shaman (Fallen Shaman / Carver Shaman / Devilkin Shaman / Dark Shaman / Warped Shaman)

| Variante | D2 Name | Sodomight Name | HP Normal | HP Hell | Dmg Normal | Dmg Hell |
|----------|---------|---------------|-----------|---------|-----------|---------|
| Tier 1 | Fallen Shaman | Chu'fa Seer | 18 | 300 | 3–6 feu | 40–80 feu |
| Tier 2 | Carver Shaman | Karru Seer | 25 | 380 | 5–10 feu | 55–100 feu |
| Tier 3 | Devilkin Shaman | Devlin Seer | 32 | 460 | 7–14 feu | 70–125 feu |
| Tier 4 | Dark Shaman | Darkon Seer | 40 | 540 | 10–18 feu | 88–155 feu |
| Tier 5 | Warped Shaman | Warpkin Seer | 50 | 620 | 12–22 feu | 100–180 feu |

**Type :** Demon | **Peut être Revived :** Non
**Résistances (Hell) :** Feu 50, Froid 0, Foudre 0, Poison 0
**IA :** `shamanic` — Ressuscite uniquement les Fallen de sa propre variante ; ne fuit pas
**Attaque spéciale :** Fireball (portée 10 tiles)

#### Zombie (Zombie / Hungry Dead / Ghoul / Drowned Carcass / Plague Bearer)

| Variante | D2 Name | Sodomight Name | HP Normal | HP Hell | Immunités Hell |
|----------|---------|---------------|-----------|---------|---------------|
| Tier 1 | Zombie | Rampant | Burial Grounds | 45 | 800 | — |
| Tier 2 | Hungry Dead | Famished | Cold Plains | 60 | 950 | — |
| Tier 3 | Ghoul | Decayer | Dark Wood | 75 | 1100 | — |
| Tier 4 | Drowned Carcass | Sogbound | Sewers/Swamp | 90 | 1280 | Poison |
| Tier 5 | Plague Bearer | Blightmaw | Act 2 Lost City | 110 | 1500 | Poison |

**Type :** Undead | **Peut être Revived :** Oui
**IA :** `melee_slow` — Marche lentement vers le joueur ; jamais ne fuit
**Attaque spéciale :** Tier 4–5 : Cloud de poison au contact (Drowned Carcass = poison physique)

#### Skeleton / Skeleton Archer / Skeleton Mage

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Dmg Normal | Notes |
|----------|---------|-----------|-----------|---------|-----------|-------|
| Guerrier | Skeleton Warrior | Ossik Blade | 22 | 420 | 3–8 | Épée ou hache |
| Archer | Skeleton Archer | Ossik Arrow | 18 | 380 | 3–6 distance | Flèches physiques |
| Mage Feu | Returned | Ashbone Mage | 25 | 500 | 5–12 feu | Boule de feu lente |
| Mage Froid | Bone Warrior | Icecrypt Mage | 25 | 500 | 4–10 froid | Cristaux de glace |
| Mage Foudre | Burning Dead | Volt Bone Mage | 25 | 500 | 6–14 foudre | Éclair court |
| Mage Poison | Horror | Venom Bone Mage | 25 | 500 | 2–5 +poison | Nuage de poison |

**Type :** Undead | **Peut être Revived :** Oui (Guerrier/Archer)
**Résistances (Hell) :** Feu 0/20, Froid 0/20, Foudre 0/20, Poison 50
**IA Guerrier/Archer :** `melee_chase` / `ranged_static`
**IA Mage :** `ranged_retreat` — Recule si le joueur s'approche à < 4 tiles

#### Corrupt Rogue (Rogue Archer / Rogue Spearwoman)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Attaque |
|----------|---------|-----------|-----------|---------|---------|
| Archer | Dark Hunter | Shadowbow | 30 | 520 | Flèches physiques |
| Arbalète | Vile Archer | Deathquill | 35 | 580 | Flèches + poison |
| Feu | Dark Archer | Emberbow | 32 | 550 | Flèches enflammées |
| Froid | Rotting Archer | Frostquill | 32 | 550 | Flèches givrantes |
| Lancière | Dark Spearwoman | Ashpike | 38 | 620 | Javelot |
| Lancière Foudre | Vile Lancer | Stormspear | 42 | 660 | Javelot + éclair |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `ranged_kite` — Tire à distance, recule si corps-à-corps ; Archers reculent activement
**Attaque spéciale :** Variantes élémentaires ajoutent dégâts élémentaires à chaque tir

#### Goatman / Goat Shaman

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Moon Clan | Kraghorn | 55 | 800 | Hache deux mains |
| Tier 2 | Night Clan | Duskhorn | 70 | 980 | Bouclier + hache |
| Tier 3 | Blood Clan | Redhorn | 85 | 1150 | Attaque double |
| Tier 4 | Hell Clan | Hellhorn | 100 | 1350 | Dmg élevé |
| Shaman | Goat Shaman | Bleat Witch | 45 | 700 | Lance Firewall |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `melee_charge` — Charge le joueur en ligne droite si > 8 tiles
**Attaque spéciale (Shaman) :** Firewall — mur de feu traversant la zone

#### Blood Hawk / Foul Crow

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Normal | Blood Hawk | Crimsonwing | 18 | 320 | Piqué aérien |
| Nest | Blood Hawk Nest | Crimson Nest | 12 | 200 | Spawn hawks (destructible) |
| Foul Crow | Foul Crow | Harrowcrow | 22 | 380 | Attaque en groupe |
| Nest | Foul Crow Nest | Harrow Nest | 15 | 230 | Spawn crows |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `aerial_dive` — Vole vers le joueur, atterrit, attaque, repars en vol

#### Spike Fiend / Thorn Beast

| Variante | D2 Name | Sodomight | HP Normal | HP Hell |
|----------|---------|-----------|-----------|---------|
| Tier 1 | Spike Fiend | Quillbeast | 15 | 280 |
| Tier 2 | Thorn Beast | Barbjaw | 20 | 340 |
| Tier 3 | Quill Rat | Razorback | 25 | 400 |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `melee_fast` — Très rapide, attaque frénétiquement
**Attaque spéciale :** Lancent leurs piquants en projectile (courte portée)

#### Wendigo / Carver Brute (Brute family)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Brute | Goreclub | 80 | 1200 | Masse + bouclier |
| Tier 2 | Yeti | Coldmaul | 95 | 1400 | Bonus dégâts froid |
| Tier 3 | Wendigo | Darkpelt | 110 | 1600 | Le plus fort des brutes |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `melee_follow` — Suit et attaque, bonus si plusieurs cibles

#### Giant Spider / Leaping Spider

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Attaque |
|----------|---------|-----------|-----------|---------|---------|
| Crawler | Leaping Spider | Vaultspider | 25 | 420 | Bond + morsure |
| Giant | Giant Spider | Webspinner | 35 | 560 | Toile + morsure |
| Spider Magus | Spider Magus | Weaveborn | 30 | 490 | Tir de venin |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `leap_attack` — Bond périodique sur le joueur

#### Wraith / Ghost (Undead aérien)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Attaque |
|----------|---------|-----------|-----------|---------|---------|
| Tier 1 | Wraith | Hauntshade | 20 | 380 | Drain vie |
| Tier 2 | Specter | Gloomrift | 28 | 480 | Drain vie + froid |
| Tier 3 | Apparition | Voidveil | 36 | 580 | Drain vie + téléport |

**Type :** Undead | **Peut être Revived :** Non (fantôme — pas de corps)
**Résistances (Hell) :** Feu 0, Froid 40, Foudre 0, Poison 50
**IA :** `phase_chase` — Traverse les obstacles, drain vie au contact

#### Dark Elder (Plague Bearer family)

| D2 Name | Sodomight | Zone | HP Normal | HP Hell | Notes |
|---------|-----------|------|-----------|---------|-------|
| Plague Bearer | Blightmaw | Act 2 Lost City | 110 | 1500 | Poison cloud |
| Dark Elder | Oldvenom | Act 2 Lost City (SU) | 250 | 2800 | Super Unique |

**Type :** Undead | **IA :** `slow_poison_aura` — Nuage de poison permanent autour du corps

---

### 3.2 Acte 2 — Lut Gholein & Désert

#### Sand Raider / Marauder

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Sand Raider | Duneblood | 60 | 850 | Épée rapide |
| Tier 2 | Marauder | Scarstrider | 75 | 1050 | Bouclier + hache |
| Tier 3 | Invader | Siegeblood | 90 | 1250 | Double frappe |
| Tier 4 | Infidel | Blastrider | 110 | 1450 | Attaque enflammée |
| Tier 5 | Sand Warrior | Ashwalker | 130 | 1700 | Résistance feu |

**Type :** Demon | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 40, Froid 0, Foudre 0, Poison 0
**IA :** `melee_chase` — Chasse agressivement, aggro range élevé

#### Dung Soldier / Scarab Demon

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Attaque |
|----------|---------|-----------|-----------|---------|---------|
| Tier 1 | Dung Soldier | Chitinax | 50 | 750 | Melee + charge électrique |
| Tier 2 | Death Beetle | Scarathorn | 65 | 950 | Charge + foudre |
| Tier 3 | Scarab Demon | Gloomcarab | 80 | 1150 | Charge + AoE foudre |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `charge_shock` — Charge le joueur, libère un éclair au contact

#### Claw Viper

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Claw Viper | Fangslink | 55 | 800 | Poison + morsure |
| Tier 2 | Salamander | Scaleburn | 70 | 1000 | Feu + poison |
| Tier 3 | Pit Viper | Venomscale | 85 | 1200 | Poison puissant |
| Tier 4 | Serpent Magus | Coilmaster | 100 | 1400 | Ritual Curse (ralentit) |

**Type :** Animal | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 0, Froid 0, Foudre 0, Poison 60
**IA :** `melee_poison` — Corps-à-corps avec empoisonnement passif

#### Sand Maggot / Rock Worm (famille Maggot)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Reine | Sand Maggot | Grubqueen | 120 | 1800 | Pond des œufs |
| Asticot | Rock Worm | Gritlarva | 30 | 480 | Fouille dans le sol |
| Jeune | Sand Maggot Young | Hatchmaw | 15 | 250 | Spawn depuis œuf |
| Œuf | Sand Maggot Egg | Gritwomb | 8 | 140 | Immobile, pond Young |

**Type :** Animal | **Peut être Revived :** Oui (adultes seulement)
**IA Reine :** `burrow_queen` — Se déplace en creusant, pond des œufs périodiquement
**IA Jeune :** `swarm_rush` — Fonce en masse vers le joueur

#### Blunderbore / Crusher

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Blunderbore | Bouldermaul | 200 | 3000 | Géant, AoE |
| Tier 2 | Gorbelly | Warbelly | 250 | 3500 | Variante plus forte |
| Tier 3 | Mauler | Crashfist | 300 | 4000 | Le plus puissant |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `melee_aoe` — Attaque de masse, frappe 3×3 tiles

#### Mummy / Greater Mummy

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Dried Corpse | Dessicant | 70 | 1000 | Marche lentement |
| Tier 2 | Decayed | Rotbind | 85 | 1200 | Poison contact |
| Tier 3 | Embalmed | Preserv'd | 100 | 1400 | Poison puissant |
| Tier 4 | Preserved Dead | Lacqwrap | 115 | 1600 | Armure renforcée |
| Tier 5 | Cadaver | Cryptwalker | 130 | 1800 | Résistance physique |
| Ressusciteur | Dried Corpse | Wrappedmaster | 55 | 850 | Ressuscite les mummies |
| Grand | Greater Mummy | Tomb Weaver | 140 | 2000 | Ressuscite ET attaque |

**Type :** Undead | **Peut être Revived :** Oui (standard mummies)
**Résistances (Hell) :** Feu 0, Froid 40, Foudre 0, Poison 80
**IA :** `shamanic_resurrect` (Greater Mummy) — Ressuscite les mummies mortes à portée
**Attaque spéciale :** Cloud de poison persistant

#### Bat Demon / Swarm / Itchies

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Chauve-souris | Bat Demon | Darkwing | 15 | 280 | Vol, rapide |
| Essaim | Itchies | Stingcloud | 10 | 200 | Empoisonne rapidement |
| Essaim froid | Hell Swarm | Frostmidge | 12 | 220 | Freeze court |

**Type :** Animal (Demon pour Bat Demon) | **Peut être Revived :** Non (trop petit)
**IA :** `swarm_envelop` — Entoure le joueur, attaque depuis tous côtés

#### Vulture Demon

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Vulture Demon | Beakclaw | 80 | 1200 | Attaque en vol |
| Tier 2 | Hell Buzzard | Goretalon | 100 | 1500 | Plus rapide |
| Tier 3 | Winged Nightmare | Shadowwing | 120 | 1800 | Très rapide, NM+ |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `aerial_strafe` — Passe en vol, attaque au passage

#### Baboon Demon / Night Clan

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Night Clan | Shadowclaw | 65 | 950 | Bond + griffes |
| Tier 2 | Doom Ape | Voidape | 80 | 1150 | Bond allongé |
| Tier 3 | Demon Ape | Hellgrip | 95 | 1350 | Attaque 3 fois de suite |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `leap_pack` — Bond groupé sur le joueur

#### Fire Tower / Lightning Spire (Constructs)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Feu | Fire Tower | Cinderspire | 200 | 3000 | Attaque Firewall statique |
| Foudre | Lightning Spire | Boltcrown | 200 | 3000 | Éclairs statiques |

**Type :** (Construct) | **Peut être Revived :** Non
**IA :** `static_turret` — Immobile, tire périodiquement

#### Sand Leaper / Hell Spawn (Leaper family)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Sand Leaper | Gritjump | 40 | 600 | Bond rapide |
| Tier 2 | Cave Leaper | Stoneleap | 55 | 800 | Bond fort |
| Tier 3 | Tomb Creeper | Cryptleap | 65 | 950 | NM/Hell |
| Tier 4 | Tree Lurker | Woodeaper | 75 | 1100 | Act 3 |
| Tier 5 | Invader | Blazeleap | 90 | 1300 | Act 4 |
| Tier 6 | Hell Slinger | Hellbound | 105 | 1500 | Act 4 renforcé |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `leap_chase` — Bond répété vers le joueur, imprévisible

---

### 3.3 Acte 3 — Kurast Jungles

#### Fetish / Fetish Shaman (famille Fetish)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Dague | Fetish | Pinyata | 30 | 500 | Sprint-dague |
| Sarbacane | Fetish Blowdart | Toxiblower | 25 | 440 | Tir poison |
| Shaman | Fetish Shaman | Pinyata Seer | 40 | 620 | Ressuscite Fetish |
| Âme | Soul Killer | Shrieker | 35 | 560 | Variante NM |
| Shaman Âme | Soul Killer Shaman | Shrieker Seer | 50 | 720 | Ressuscite Soul Killers |
| Flayer | Flayer | Flayblade | 38 | 590 | NM Hell only |
| Shaman Flayer | Flayer Shaman | Flayblade Seer | 55 | 780 | Ressuscite Flayers |
| Danse | Undead Stygian Doll | Reanidoll | 28 | 470 | Variante morte-vivante |

**Type :** Demon (vivants) / Undead (morts-vivants) | **Peut être Revived :** Oui (Fetish)
**Résistances (Hell) :** Feu 0, Froid 0, Foudre 0, Poison 40
**IA Fetish :** `sprint_melee` — Sprint frénétique vers le joueur, poignard
**IA Blowdart :** `ranged_poison` — Reste à distance, tire dard empoisonné
**IA Shaman :** `shamanic` — Ressuscite les fetish morts de sa propre variante uniquement

#### Thorned Hulk / Bramble Hulk

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Thorned Hulk | Spikedom | 250 | 3800 | Épines reflètent dégâts |
| Tier 2 | Bramble Hulk | Thornmaul | 300 | 4500 | Plus fort, résistance phys |
| Tier 3 | Swamp Dweller | Bogcrusher | 350 | 5200 | NM renforcé |
| Tier 4 | Thrasher | Slashbriar | 400 | 6000 | Très rapide pour sa taille |

**Type :** Animal | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 0, Froid 40, Foudre 0, Poison 60
**IA :** `melee_thorns` — Melee puissant ; les épines causent des dégâts en retour physique

#### Willowisp / Gloam (famille fantôme électrique)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Gloam | Sparkshade | 25 | 430 | Éclair, drain mana 40% |
| Tier 2 | Black Soul | Voidzap | 35 | 580 | Éclair puissant |
| Tier 3 | Burning Soul | Scorchsoul | 45 | 730 | Foudre + feu |

**Type :** Undead | **Peut être Revived :** Non
**Résistances (Hell) :** Feu 0, Froid 0, Foudre IMMUNE, Poison 50
**IA :** `ranged_lightning` — Tire des éclairs à distance, drain mana passif au contact
**Attaque spéciale :** Drain mana 40% chance sur melee

#### Council Member (Zakarum Council)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Sorts |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Zakarum Zealot | Goldmask Zealot | 120 | 1800 | Melee fanatique |
| Tier 2 | Faithful | Goldmask Faith | 140 | 2100 | Holy Fire aura |
| Tier 3 | Templar | Goldmask Templar | 160 | 2400 | Holy Shield |
| Elite | Council Member | Warped Elder | 300 | 4500 | Hydra, Lightning, Holy Shield |

**Type :** Demon | **Peut être Revived :** Oui (Zealots/Faithful/Templar)
**Résistances (Hell) :** Feu 50, Froid 0, Foudre 75, Poison 0
**IA :** `mage_melee` — Lance des sorts, utilise Holy Shield, approche si sorts insuffisants
**Attaque spéciale (Council Member) :** Hydra (invoque une Hydra de feu), Lightning Bolt, Blizzard

#### Zakarum Priest (Shaman du Conseil)

| D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|---------|-----------|-----------|---------|-------|
| Zakarum Priest | Warpedpope | 80 | 1200 | Ressuscite les Zealots |

**Type :** Demon | **IA :** `shamanic` — Ressuscite les Zealots alentour

#### Rat Man (Swamp)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell |
|----------|---------|-----------|-----------|---------|
| Tier 1 | Rat Man | Skeezrat | 45 | 680 |
| Tier 2 | Plague Rat | Plaguewhisker | 58 | 870 |
| Tier 3 | Tomb Rat | Cryptnibble | 72 | 1080 |
| Tier 4 | Black Raptor | Shadowclaw | 88 | 1290 |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `swarm_melee` — Attaque en meute, fuit si seul

#### Giant Mosquito / Frog Demon

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Moustique | Giant Mosquito | Bloodmosq | 20 | 360 | Drain vie au contact |
| Grenouille | Frog Demon | Bulkfrog | 65 | 980 | Bond + acide |

**Type :** Animal | **Peut être Revived :** Oui
**IA Mosquito :** `drain_life_fly` — Vol, drain vie passif contact
**IA Frog :** `leap_acid` — Bond, crache acide (poison) à l'atterrissage

---

### 3.4 Acte 4 — Chaos Sanctuary

#### Finger Mage / Vile Mother

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Mage osseux | Finger Mage | Bonedigit | 80 | 1200 | Bone Prison, Teeth |
| Mère immonde | Vile Mother | Spawncyst | 150 | 2200 | Spawn des Vile Child |
| Enfant immonde | Vile Child | Cystkin | 15 | 250 | Minion de la Vile Mother |

**Type :** Undead (Finger Mage) / Demon (Vile) | **Peut être Revived :** Oui (Finger Mage)
**IA Finger Mage :** `mage_bone` — Lance Bone Prison, Teeth, se téléporte périodiquement
**IA Vile Mother :** `spawn_burst` — Spawn des enfants puis melee

#### Oblivion Knight (Malédiction en aura)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Aura Curse |
|----------|---------|-----------|-----------|---------|-----------|
| Tier 1 | Oblivion Knight | Voidlord | 280 | 4200 | Iron Maiden / Decrepify |
| Tier 2 | Doom Knight | Blight Paladin | 320 | 4800 | Life Tap / Amplify Damage |
| Tier 3 | Abyss Knight | Abyssward | 360 | 5400 | Confuse / Attract |

**Type :** Undead | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 0, Froid IMMUNE, Foudre 0, Poison 50
**IA :** `curse_caster` — Lance une malédiction de type Nécromancien en aura permanente
**Attaque spéciale :** Malédiction change selon le timer ; Iron Maiden est mortelle pour le joueur melee

#### Megademon / Venom Lord (famille Demon géant)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Megademon | Gorehull | 400 | 6000 | Inferno, melee puissant |
| Tier 2 | Balrog | Brimhull | 480 | 7200 | Inferno étendu |
| Tier 3 | Pit Lord | Abysshull | 560 | 8400 | Résistance feu totale |
| Tier 4 | Venom Lord | Venomhull | 500 | 7500 | Inferno + poison |
| Tier 5 | Hell Lord | Hellhull | 600 | 9000 | Double Inferno |

**Type :** Demon | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu IMMUNE, Froid 0, Foudre 0, Poison 50
**IA :** `inferno_melee` — Inferno en approche, melee au contact

#### Storm Caster (Sorcier tempête)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Sorts |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Storm Caster | Tempestcrypt | 120 | 1800 | Firewall, Lightning |
| Tier 2 | Warped Shaman | Twistedcaster | 150 | 2200 | Firewall, Lightning, Glacial |

**Type :** Demon | **IA :** `ranged_mage` — Lance sorts en alternance, se téléporte si pris corps-à-corps

#### Trapped Soul

| D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|---------|-----------|-----------|---------|-------|
| Trapped Soul | Shacklebound | 35 | 540 | Âme piégée dans les flammes |

**Type :** Animal (classification interne D2) | **IA :** `melee_burning`

---

### 3.5 Acte 5 — Harrogath & Montagnes

#### Saber Cat / Death Bringer (famille félin)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Saber Cat | Raventooth | 75 | 1100 | Charge rapide |
| Tier 2 | Night Tiger | Shadowtooth | 90 | 1350 | Plus rapide |
| Tier 3 | Hell Cat | Infernopaw | 108 | 1600 | Résistance feu |
| Tier 4 | Death Bringer | Grimfang | 130 | 1900 | Attaque éventrante |

**Type :** Animal | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 40, Froid 0, Foudre 0, Poison 0
**IA :** `charge_sprint` — Charge initiale, puis melee frénétique

#### Demon Imp (téléporteur)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Demon Imp | Flickfiend | 40 | 620 | Téléporte, frappe |
| Tier 2 | Demon Gremlin | Twistwick | 52 | 800 | Téléporte + poison |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `teleport_harass` — Téléporte périodiquement autour du joueur, frappe et s'éloigne

#### Frozen Horror / Frozen Creeper (famille glace)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Frozen Creeper | Glacivine | 90 | 1350 | Gel contact |
| Tier 2 | Frozen Scourge | Permaclaw | 110 | 1650 | Gel + dégâts |
| Tier 3 | Frozen Horror | Cryoterror | 130 | 1950 | Immune au froid |
| Tier 4 | Frozen Abyss | Void Glacier | 160 | 2400 | Mana burn + gel |

**Type :** Animal | **Peut être Revived :** Oui
**Résistances (Hell) :** Feu 0, Froid IMMUNE, Foudre 0, Poison 0
**IA :** `cold_freeze_melee` — Gel les cibles proches, melee lent mais puissant

#### Ice Boar / Fire Boar (famille sanglier)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Feu | Fire Boar | Embertusk | 85 | 1280 | Charge + feu |
| Glace | Ice Boar | Glacialtusk | 85 | 1280 | Charge + gel |

**Type :** Animal | **IA :** `charge_element` — Charge et libère élément à l'impact

#### Overseer (Contremaitre)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Overseer | Lashmaster | 200 | 3000 | Fouet, boost alliés |
| Tier 2 | Lasher | Whipfiend | 240 | 3600 | Boost vitesse alliés |
| Tier 3 | Overlord | Tyrantwhip | 280 | 4200 | Boost dégâts alliés |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `commander_aura` — Fouette les alliés proches pour booster leur vitesse ; attaque si isolé

#### Succubus / Hell Temptress (famille démone)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Sorts |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Hell Temptress | Luredemon | 90 | 1350 | Charged Bolt |
| Tier 2 | Blood Temptress | Crimsonlure | 110 | 1650 | Charged Bolt fort |
| Tier 3 | Succubus | Shadowsiren | 130 | 1950 | Charged Bolt + Mana Burn |

**Type :** Demon | **Peut être Revived :** Oui
**IA :** `ranged_cb` — Tire des Charged Bolts à distance, recule si corps-à-corps

#### Death Mauler / Siege Beast

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Death Mauler | Crashbeast | 180 | 2700 | Burrow + frappe |
| Tier 2 | Siege Beast | Rampartbeast | 220 | 3300 | Géant destructeur |

**Type :** Animal (Demon pour Siege Beast) | **Peut être Revived :** Oui
**IA Death Mauler :** `burrow_attack` — Disparaît, réapparaît sous le joueur

#### Reanimated Horde (Barbare réanimé)

| Variante | D2 Name | Sodomight | HP Normal | HP Hell | Notes |
|----------|---------|-----------|-----------|---------|-------|
| Tier 1 | Reanimated Horde | Revnant Berserker | 150 | 2200 | Attaque barbare |
| Tier 2 | Frenzied Horde | Maddened Revenant | 180 | 2700 | Frenzy aura |

**Type :** Undead | **Peut être Revived :** Oui
**IA :** `berserk_undead` — Attaque frénétique, bonus dégâts en groupe

#### Minion of Destruction (Serviteur de Baal)

| D2 Name | Sodomight | HP Normal | HP Hell | Immunité |
|---------|-----------|-----------|---------|---------|
| Minion of Destruction | Worldstone Minion | 220 | 3300 | Feu (en Hell) |

**Type :** Animal | **Peut être Revived :** Oui
**IA :** `pack_rush` — Fonce en masse, immunité feu en Hell

#### Hell Bovine (Vache Infernale — Secret Cow Level)

| D2 Name | Sodomight | Zone | HP Normal | HP Hell | Notes |
|---------|-----------|------|-----------|---------|-------|
| Hell Bovine | Slaughterfiend | Moo Moo Farm | 50 | 1200 | Hache, melee simple |
| The Cow King | Bovine Sovereign | Moo Moo Farm | 250 | — | Super Unique, Ne peut pas être tué en Normal |

**Type :** Animal | **Peut être Revived :** Oui (Bovines normaux)
**IA :** `melee_pack` — Melee simple, toujours en groupe dense

---

## 4. Affixes de Champions et Uniques

### 4.1 Tableau complet des affixes (~50)

| # | Nom D2 | Nom Sodomight | Effet Mécanique | Dangerosité (1–10) |
|---|--------|--------------|----------------|-------------------|
| 1 | **Extra Fast** | Foudroyant | +100% vitesse marche/course ; boss : ×2 vitesse d'attaque | 8 |
| 2 | **Extra Strong** | Surpuissant | ×2.5 dégâts min/max ; +25% AR ; minions +75% dégâts | 7 |
| 3 | **Extra Life** | Tenace | ×3 HP en Normal ; ×2.5 NM ; ×2 Hell | 4 |
| 4 | **Cursed** | Maudit | 50% chance d'appliquer *Amplify Damage* sur hit (×2 dégâts physiques reçus) | 9 |
| 5 | **Magic Resistant** | Résistant aux arts | +40% Feu, +40% Froid, +40% Foudre simultanément | 7 |
| 6 | **Fire Enchanted** | Embrasé | +66–100% dégâts feu ; ×2 AR ; +75% rés. feu ; explose à la mort (75–100% HP max en feu) | 9 |
| 7 | **Cold Enchanted** | Glaciaire | +66–100% dégâts froid ; ×2 AR ; +75% rés. froid ; Frost Nova à la mort | 7 |
| 8 | **Lightning Enchanted** | Fulgurant | +66–100% dégâts foudre ; ×2 AR ; +75% rés. foudre ; Charged Bolts quand frappé | 8 |
| 9 | **Mana Burn** | Cannibale de mana | ×4 dégâts ; ×2 AR ; +20% rés. magie ; convertit mana brûlé en HP pour le monstre | 10 |
| 10 | **Teleportation** | Téléporteur | Se téléporte si HP < 33%, récupère jusqu'à 30% HP (nerfé v1.09) | 5 |
| 11 | **Stone Skin** | Peau de pierre | +50% résistance physique ; ×2 défense | 7 |
| 12 | **Spectral Hit** | Frappe spectrale | Chaque hit ajoute dégâts élémentaires aléatoires ; +20% rés. Feu/Froid/Foudre ; ×2 AR | 8 |
| 13 | **Multishot** | Multijets | ×3 projectiles par attaque (monstres à distance uniquement) ; portée identique | 8 |
| 14 | **Aura Enchanted** | Imbu d'aura | Génère une aura de Paladin (voir sous-tableau) | 7–10 |
| 15 | **Immune to Fire** | Immunisé au feu | 100% résistance au feu (immunité complète) | 5 |
| 16 | **Immune to Cold** | Immunisé au froid | 100% résistance au froid | 4 |
| 17 | **Immune to Lightning** | Immunisé à la foudre | 100% résistance à la foudre | 5 |
| 18 | **Immune to Poison** | Immunisé au poison | 100% résistance au poison | 3 |
| 19 | **Immune to Magic** | Immunisé à la magie | 100% résistance aux dégâts magiques | 6 |
| 20 | **Immune to Physical** | Immunisé au physique | 100% résistance physique (Ghost champion uniquement naturellement) | 10 |
| 21 | **Reflect Missiles** | Renvoi de projectiles | Renvoie les projectiles du joueur vers lui | 9 |
| 22 | **Fanaticism** (aura) | Fanatisme | Boost vitesse d'attaque alliés +75%, dégâts +300%, AR +300% (rayon 20) | 10 |
| 23 | **Conviction** (aura) | Conviction | Réduit résistances joueur de –125% (Feu/Froid/Foudre) et défense de –70% | 10 |
| 24 | **Holy Freeze** (aura) | Gel Sacré | Ralentit joueur et alliés –70% vitesse dans le rayon | 8 |
| 25 | **Holy Fire** (aura) | Feu Sacré | Dégâts de feu AoE autour du monstre ; bonus dégâts feu aux alliés | 7 |
| 26 | **Holy Shock** (aura) | Choc Sacré | Dégâts de foudre AoE autour du monstre | 7 |
| 27 | **Might** (aura) | Puissance | Augmente dégâts physiques alliés proches +300% | 8 |
| 28 | **Blessed Aim** (aura) | Bénédiction des archers | Augmente AR alliés proches +300% | 6 |
| 29 | **Health Link** | Vie partagée | Pool de vie commun entre les champions du pack ; dégâts sur l'un = dégâts sur tous | 9 |
| 30 | **Jailer** | Geôlier | Emprisonne le joueur dans une prison magique 3–5 secondes | 8 |
| 31 | **Avenger** | Vengeur | Champions uniquement ; immunité à 2 éléments aléatoires | 7 |
| 32 | **Fire Chains** | Chaînes de feu | Champions uniquement ; chaînes de feu entre les 3 membres du pack | 8 |
| 33 | **Possessed** (type) | Possédé | ×12 HP ; immunisé aux malédictions | 7 |
| 34 | **Ghostly** (type) | Spectral | 80% résistance physique ; dégâts de froid passifs | 8 |

### 4.2 Détail des Auras Enchantées

| Aura | Niveau scalé | Effet principal | Combinaison dangereuse |
|------|-------------|----------------|----------------------|
| **Fanaticism** | = mLevel/2 | +AS, +Dmg, +AR tous alliés proches | + Oblivion Knight = alliés avec Iron Maiden |
| **Conviction** | = mLevel/2 | –résistances joueur (–125% max) | + Fire/Lightning Enchanted = one-shot |
| **Holy Freeze** | = mLevel/2 | Slow AoE massif (–70% vit.) | + Mana Burn = joueur immobile et sans mana |
| **Holy Fire** | = mLevel/2 | Dégâts feu AoE continus | + Fire Enchanted minions = zone mortelle |
| **Holy Shock** | = mLevel/2 | Dégâts foudre AoE continus | + Lightning Enchanted = chain death |
| **Might** | = mLevel/2 | +300% dégâts phys alliés | + Extra Strong = dégâts physiques absurdes |
| **Blessed Aim** | = mLevel/2 | +300% AR alliés | + Stone Skin = impossible à esquiver |

### 4.3 Combinaisons d'Affixes Particulièrement Dangereuses

| Combinaison | Pourquoi c'est fatal |
|-------------|---------------------|
| Cursed + Fire Enchanted | Dégâts physiques doublés ET explosion à la mort |
| Mana Burn + Extra Fast | Joueur à sec de mana avant de pouvoir réagir |
| Conviction Aura + Lightning Enchanted | Résistances joueur négatives + éclairs = mort instantanée |
| Stone Skin + Magic Resistant | Quasi-indestructible — aucun type de dégâts ne passe |
| Fanaticism + Extra Strong | Alliés qui one-shot via dégâts boostés ×3 ×2.5 |
| Teleportation + Mana Burn | Impossible d'attraper, se régénère, vide le mana |
| Health Link + 3x Fire Enchanted | Triple explosion de mort en même temps |
| Ghostly + Immune to Magic | 80% phys + 100% magie = seuls élémentaires passent |

---

## 5. Super Uniques — Catalogue Complet

> Les Super Uniques ont des affixes **fixes**. En Nightmare, ils gagnent **+1 affix aléatoire**.
> En Hell, ils gagnent **+2 affixes aléatoires** supplémentaires.

### 5.1 Acte 1 — 13 Super Uniques

| Nom D2 | Nom Sodomight | Zone | Base Monster | Affixes Fixes | Farm ? |
|--------|--------------|------|-------------|--------------|--------|
| **Corpsefire** | Cryptworm | Den of Evil | Zombie | Spectral Hit | Non (quête seulement) |
| **Bishibosh** | Ashseer | Cold Plains | Fallen Shaman | Magic Resistant, Fire Enchanted | Non |
| **Bonebreaker** | Tombgrinder | The Crypt | Skeleton | Extra Strong, Magic Resistant | Coffre d'or derrière lui |
| **Blood Raven** | Crimsonwitch | Burial Grounds | Corrupt Rogue | Fire Arrow (unique) | Non — quête |
| **Coldcrow** | Icefeather | The Cave Level 2 | Dark Stalker | Cold Enchanted | Oui — peut drop Gull |
| **Rakanishu** | Voltking | Stony Field | Carver | Lightning Enchanted, Extra Fast | Non |
| **Treehead Woodfist** | Knottlord | Dark Wood | Brute | Extra Strong, Extra Fast | Non |
| **Griswold** | Ironforge | Tristram | Skeleton | Cursed | Optionnel, loot moyen |
| **The Countess** | La Tisserune | Forgotten Tower L5 | Dark Stalker | Fire Enchanted | **OUI** — Runes uniques |
| **Pitspawn Fouldog** | Bilepool | Jail Level 2 | Bone Fetish | Cursed, Cold Enchanted | Non |
| **Bone Ash** | Ashpyre | Cathedral | Burning Dead Mage | Extra Strong, Cold Enchanted, Magic Resistant | Non |
| **The Smith** | Le Marteleur | Barracks | Blunderbore | Extra Strong | Quête : Horadric Malus |
| **Andariel** | Andara | Catacombs L4 | Act Boss | Voir §6.1 | **OUI** — Boss farming |

**The Countess — Notes spéciales :**
- Drop des runes jusqu'à **Ist** (Hell)
- Son loot table de runes est unique dans le jeu
- En Hell, pack de Dark Stalkers très dense dans la tour

**Blood Raven — Notes spéciales :**
- Invoque des Zombies en permanence pendant le combat
- Quête obligatoire (acte 1, quête 2)
- Immunisée aux projectiles physiques (tire des flèches de feu)

### 5.2 Acte 2 — 11 Super Uniques

| Nom D2 | Nom Sodomight | Zone | Base Monster | Affixes Fixes | Farm ? |
|--------|--------------|------|-------------|--------------|--------|
| **Radament** | Ossikar | Sewers Level 3 | Greater Mummy | Extra Fast | Quête — drop livre de skill |
| **Creeping Feature** | Mouldcreep | Stony Tomb Level 2 | Decayed | Extra Strong, Cold Enchanted | Non |
| **Blood Witch the Wild** | Bloodweave | Halls of the Dead L3 | Huntress | Extra Strong, Cursed | Quête — Horadric Cube |
| **Beetleburst** | Shockbeetle | Far Oasis | Death Beetle | Magic Resistant | Non |
| **Coldworm the Burrower** | Iceworm | Maggot Lair Level 3 | Sand Maggot | Cold Enchanted, Magic Resistant | Non |
| **Dark Elder** | Oldvenom | Lost City | Plague Bearer | Extra Fast, Magic Resistant | Non |
| **Fangskin** | Slitherhook | Claw Viper Temple L2 | Salamander | Lightning Enchanted, Extra Fast | Non |
| **Fire Eye** | Embereye | Palace Cellar Level 3 | Invader | Fire Enchanted, Extra Fast | Non |
| **The Summoner** | L'Invocateur | Arcane Sanctuary | Unique | Voir §5 notes | Quête — Key of Hate |
| **Ancient Kaa the Soulless** | Kaa l'Âme Vide | Tal Rasha's Tomb | Unraveler | Magic Resistant, Extra Strong, Lightning Enchanted | Non |
| **Duriel** | Durael | Duriel's Lair | Act Boss | Voir §6.2 | Peu — passage unique |

**The Summoner — Notes spéciales :**
- Stats complètement fixes, pas de modificateurs aléatoires
- Lance des Firewall redoutables
- Drop la **Key of Hate** pour le système Pandémonium

### 5.3 Acte 3 — 12 Super Uniques

| Nom D2 | Nom Sodomight | Zone | Base Monster | Affixes Fixes | Farm ? |
|--------|--------------|------|-------------|--------------|--------|
| **Sszark the Burning** | Sszark l'Ardent | Spider Cavern | Flame Spider | Extra Strong, Cursed | Non |
| **Witch Doctor Endugu** | Doc Endugu | Flayer Dungeon L3 | Fetish Shaman | Magic Resistant, Fire Enchanted | Très rare |
| **Stormtree** | Tempêtebois | Flayer Jungle | Thrasher | Extra Fast, Lightning Enchanted | Non |
| **Battlemaid Sarina** | Guerrière Sarina | Ruined Temple | Flesh Hunter | Extra Fast, Spectral Hit | Non |
| **Icehawk Riftwing** | Glacioangle | Kurast Sewers L1 | Gloombat | Cold Enchanted, Teleportation | Non |
| **Ismail Vilehand** | Mainfiel | Travincal | Council Member | Extra Fast, Cursed | Part du Conseil |
| **Geleb Flamefinger** | Geleb Feudigit | Travincal | Council Member | Extra Strong, Fire Enchanted | Explosion mortelle |
| **Toorc Icefist** | Toorc Glacepoing | Travincal | Council Member | Cold Enchanted, Stone Skin | Souvent immun phys |
| **Bremm Sparkfist** | Bremm Éclairclair | Durance of Hate L3 | Council Member | Aura Enchanted, Fire Enchanted | Avoid |
| **Wyand Voidbringer** | Wyand le Vide | Durance of Hate L3 | Council Member | Mana Burn, Teleportation | Peu dangereux |
| **Maffer Dragonhand** | Maffer Dragomain | Durance of Hate L3 | Council Member | Extra Fast, Extra Strong | Très dangereux |
| **Mephisto** | Méphikar | Durance of Hate L3 | Act Boss | Voir §6.3 | **OUI** — Boss #1 farmé |

**Bremm Sparkfist — Notes spéciales :**
- Aura Enchanted = Holy Shock ou Fanaticism selon spawn
- Fire Enchanted = explosion à la mort PLUS aura
- Zone Durance L3 contient 3 Council Members Super Uniques simultanément

### 5.4 Acte 4 — 6 Super Uniques

| Nom D2 | Nom Sodomight | Zone | Base Monster | Affixes Fixes | Farm ? |
|--------|--------------|------|-------------|--------------|--------|
| **Izual** | Izaël | Plains of Despair | Angel (Demon) | Chilling Hit, Frost Nova | Quête — 2 skill points |
| **Hephasto the Armorer** | Hephasto le Forgeur | River of Flame | Blood Lord | Conviction Aura, Spectral Hit | Quête — Hellforge Hammer |
| **Grand Vizier of Chaos** | Grand Vizir du Chaos | Chaos Sanctuary | Storm Caster | Extra Strong, Fire Enchanted | Seal boss Diablo |
| **Lord de Seis** | Seigneur de Seis | Chaos Sanctuary | Oblivion Knight | Aura (Fanaticism), Extra Strong | Seal boss Diablo |
| **Infector of Souls** | Infecteur des Âmes | Chaos Sanctuary | Venom Lord | Extra Fast, Spectral Hit | Seal boss Diablo |
| **Diablo** | Diablon | Chaos Sanctuary | Act Boss | Voir §6.4 | **OUI** — Runes/Set items |

**Lord de Seis — Notes spéciales :**
- Aura Fanaticism booste TOUS les monstres du Chaos Sanctuary proches
- Sur Hell, gagne Cold Immunity naturellement
- Peut changer son aura si laissé tranquille trop longtemps

**Hephasto — Notes spéciales :**
- Conviction Aura permanente = dangereux pour tous les joueurs élémentaires
- Quête obligatoire pour forger la Hellforge

### 5.5 Acte 5 — 20+ Super Uniques

| Nom D2 | Nom Sodomight | Zone | Base Monster | Affixes Fixes | Farm ? |
|--------|--------------|------|-------------|--------------|--------|
| **Dac Farren** | Dac Farren | Bloody Foothills | Demon Gremlin | Cold Enchanted | Non |
| **Shenk the Overseer** | Shenk le Maître | Bloody Foothills | Overseer | Extra Strong | Quête |
| **Eldritch the Rectifier** | Eldritch le Recteur | Frigid Highlands | Enslaved | Extra Fast | **OUI** — Speed farm |
| **Sharptooth Slayer** | Sharptooth | Frigid Highlands | Overlord | Extra Fast | Non |
| **Eyeback the Unleashed** | Œilvif | Frigid Highlands | Death Mauler | Extra Fast, Extra Strong | Non |
| **Thresh Socket** | Thresh l'Alvéole | Arreat Plateau | Blood Bringer | Cursed | Non |
| **Frozenstein** | Frozzix | Frozen River | Frozen Abyss | Cold Enchanted, Mana Burn | Blocage Anya |
| **Bonesaw Breaker** | Scie Osseuse | Glacial Trail | Reanimated Horde | Extra Strong, Magic Resistant | Coffre d'or proche |
| **Snapchip Shatter** | Snapchip l'Éclateur | Icy Cellar | Frozen Creeper | Cursed, Cold Enchanted | Non |
| **Pindleskin** | Pindleskin | Nihlathak's Temple | Defiled Warrior | Fire Enchanted | **OUI** — Farm #2 |
| **Nihlathak** | Nihilkar | Halls of Vaught | Unique | Corpse Explosion (actif) | Quête — Key of Destruction |
| **Talic** | Talic | Arreat Summit | Barbarian | Whirlwind | Quête Rite of Passage |
| **Madawc** | Madawc | Arreat Summit | Barbarian | Shout, Double Throw | Quête Rite of Passage |
| **Korlic** | Korlic | Arreat Summit | Barbarian | Leap Attack | Quête Rite of Passage |
| **Colenzo the Annihilator** | Colenzo l'Anéantisseur | Throne of Destruction | Warped Shaman | Fire Enchanted | Wave 1 Baal |
| **Achmel the Cursed** | Achmel le Maudit | Throne of Destruction | Greater Mummy | Poison Immune (innée) | Wave 2 Baal |
| **Bartuc the Bloody** | Bartuc le Sanguinaire | Throne of Destruction | Council Member | Lightning Enchanted | Wave 3 Baal |
| **Ventar the Unholy** | Ventar le Profane | Throne of Destruction | Venom Lord | Extra Fast | Wave 4 Baal |
| **Lister the Tormentor** | Lister le Tourmenteur | Throne of Destruction | Minion of Destruction | Spectral Hit, Régénération | Wave 5 Baal |
| **Baal** | Baalrok | Worldstone Chamber | Act Boss | Voir §6.5 | **OUI** — XP + Runes |

**Pindleskin — Notes spéciales :**
- Accessible via portail rouge juste après le waypoint Nihlathak's Temple
- Run ultra-rapide : portal > 2s kill > loot > retour en ville
- Drop toutes les Set/Unique de son niveau (Hell = mLevel 86)
- Minions = Defiled Warriors (Reanimated Horde) aussi dangereux

**Eldritch + Shenk — Run combiné :**
- Deux Super Uniques dans la même zone (Frigid Highlands)
- Run très rapide, accessible depuis Harrogath

**The Ancients (Talic, Madawc, Korlic) — Mécaniques spéciales :**
- Équipement aléatoire avec affixes selon leur mLevel
- Si un équipement donne une immunité, ils DEVIENNENT immunisés
- Doivent tous être tués pour activer le portail Baal
- Si le joueur meurt, ils respawn avec de NOUVEAUX affixes d'équipement

---

## 6. Boss d'Acte — Mécaniques Complètes

### 6.1 Andariel — Reine des Anguilles (Acte 1)

**Sodomight Name :** Andara l'Infestée
**Zone :** Catacombs Level 4
**Type :** Demon | **Regen HP :** Non

#### Stats par difficulté

| Stat | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Niveau | 12 | 49 | 75 |
| HP | 1 024 | 24 800 | 60 031 |
| XP | 1 282 | 92 295 | 561 066 |
| Défense | 75 | 420 | 760 |

#### Résistances

| Type | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Feu | **–50%** (vulnérable) | **–50%** | **–50%** |
| Froid | 50% | 50% | 66% |
| Foudre | 50% | 50% | 66% |
| Poison | 50% | 50% | 66% |
| Physique | 0% | 0% | 66% |
| Magie | 0% | 0% | 0% |

#### Attaques et phases

1. **Scythe Strike** — Melee en arc large, 3 coups rapides ; dégâts physiques + poison
2. **Poison Cloud** — Nuage de poison zone, durée 3s, radius 3 tiles ; damage over time massif
3. **Poison Spray** — Jet de poison directionnel, portée 8 tiles
4. **Charge** — Charge en ligne droite si cible à > 10 tiles
5. **Phase Aggressive** (HP < 50%) — Accélère toutes les attaques ×1.5

#### Stratégie de farm

- **Vulnérabilité feu** (–50%) = Sorceress Fire, Paladin Holy Fire idéaux
- En Hell : Fire Sorceress recommandée (résistances basses au feu)
- Route recommandée : Cairn Stones WP > Tamoe Highland > Monastery Gate > Barracks > Jail > Cathedral > Catacombs
- Drop notable : Andariel peut dropper des uniques haut niveau en Hell (mLevel 75)
- Loot inclut : Andariels Visage (unique helm, meilleures casques du jeu)

---

### 6.2 Duriel — Seigneur de la Douleur (Acte 2)

**Sodomight Name :** Durael le Supplicieur
**Zone :** Duriel's Lair (Tal Rasha's Tomb)
**Type :** Demon | **Regen HP :** Non

#### Stats par difficulté

| Stat | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Niveau | 22 | 55 | 88 |
| HP | 3 995 | 55 799 | 84 524 |
| Défense | 200 | 800 | 1 200 |

#### Résistances

| Type | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Physique | 0% | 0% | 50% |
| Feu | 20% | 50% | 75% |
| Froid | 50% | 75% | **95%** |
| Foudre | 20% | 50% | 75% |
| Poison | 20% | 50% | 75% |
| Magie | 0% | 0% | 33% |

#### Attaques et mécaniques

1. **Jab** — 3 coups rapides successifs ; dégâts 19–25 (N) à 140–190 (H) physique
2. **Smite** — Coup avec stun ; dégâts 19–22 (N) à 115–165 (H) physique
3. **Charge** — Charge linéaire longue portée ; dégâts 57–75 (N) à **665–902 (H)** physique
4. **Holy Freeze Aura (passive)** — Ralentit marche et attaque du joueur de 30–46% ; **ne peut PAS être résisté** ; "Cannot be Frozen" n'aide PAS contre cette aura
5. **Dégâts aux minions** — Inflige ×7 ses dégâts normaux aux minions de joueurs

#### Stratégies

- **Le boss le plus brutal en Normal/NM** si le joueur est sous-équipé
- En Hell : charge à 665–902 physique peut one-shot sans armure adéquate
- Contre-stratégie : apporter des **Thawing Potions** (réduisent durée gel)
- Les archers et lanceurs de sorts peuvent reculer dans les coins pour éviter l'aura Holy Freeze
- Après patch v1.10 : rayon de l'aura réduit — archers peuvent s'en sortir en coins

---

### 6.3 Mephisto — Seigneur de la Haine (Acte 3)

**Sodomight Name :** Méphikar le Haïssant
**Zone :** Durance of Hate Level 3
**Type :** Demon | **Regen HP :** Non

#### Stats par difficulté

| Stat | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Niveau | 26 | 59 | 87 |
| HP | 6 036 | 74 547 | 94 320 |
| XP | 10 718 | 240 504 | 1 148 886 |

#### Résistances

| Type | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Feu | 33% | 50% | 75% |
| Froid | 25% | 25% | 40% |
| Foudre | 33% | 50% | 75% |
| Poison | 50% | 50% | 75% |
| Physique | 0% | 0% | 20% |
| Magie | 0% | 0% | 50% |

#### Attaques et sorts

1. **Lightning Bolt** — Éclair direct haute fréquence ; son attaque principale
2. **Charged Bolt** — Rafale de petits éclairs en éventail
3. **Chain Lightning** — Éclair qui rebondit entre plusieurs cibles
4. **Frozen Orb** (Iceball) — Projectile de glace qui explose en cristaux ; dégâts physiques + froid
5. **Frost Nova** — Nova de glace radius 4 tiles si joueur < 3 tiles
6. **Poison Nova** — Nova de poison radius 6 tiles si joueur très proche
7. **Blizzard** — Rarement utilisé ; AoE de glace
8. **Bone Prison** — Emprisonne le joueur dans une cage d'os

#### Le Moat Trick — Mécanique emblématique

```
Disposition de la salle Durance L3 :
[ Joueur ] ← [ Fossé d'eau ] → [ Mephisto ]
```

- Mephisto spawn au nord de la salle
- La salle possède un **fossé d'eau** traversant la zone
- Procédure : Agro Mephisto → Téléporter au-dessus du fossé → Se positionner derrière la ligne
- Mephisto ne peut pas traverser le fossé ; il fait des allers-retours au bord
- Le joueur peut ensuite utiliser des sorts à AOE (Nova de glace, Blizzard, Corpse Explosion) depuis l'autre côté
- **Résultat :** 0 dégâts reçus, kill assuré

#### Popularité farm (raison #1)

1. **Accessible rapidement** — Durance L3 WP est proche, run < 45s
2. **Moat trick** — Kill sans risque pour presque toutes les classes
3. **Loot exceptionnel** — Mephisto droppe les meilleures Unique/Set en Act 3
4. **Items notables :** Stone of Jordan, Harlequin Crest (Shako), Ondal's Wisdom, Tal Rasha's set pièces, Arachnid Mesh
5. **Stackable avec Andariel run** — Même session peut tuer les deux

---

### 6.4 Diablo — Prince de la Terreur (Acte 4)

**Sodomight Name :** Diablon le Seigneur de Terreur
**Zone :** Chaos Sanctuary (après activation des 5 sceaux)
**Type :** Demon | **Regen HP :** Non

#### Stats par difficulté

| Stat | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Niveau | 40 | 62 | 94 |
| HP | 13 818 | 90 749 | 113 812 |

#### Résistances

| Type | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Feu | 33% | 50% | 50% |
| Froid | 33% | 50% | 50% |
| Foudre | 33% | 50% | 50% |
| Poison | 50% | 50% | 50% |
| Physique | 0% | 0% | 45% |
| Magie | 0% | 0% | 0% |

#### Les 5 Sceaux du Chaos Sanctuary

La salle du Chaos Sanctuary contient **5 sceaux** à activer. Chaque activation spawn des monstres spéciaux. Après tous les 5, Diablo apparaît.

| Sceau | Monstre spawné | Notes |
|-------|---------------|-------|
| Sceau A-1 | Grand Vizier of Chaos (Storm Caster SU) | Fire Enchanted, Extra Strong |
| Sceau A-2 | Pack de Storm Casters | 8–12 monstres normaux |
| Sceau B-1 | Lord de Seis (Oblivion Knight SU) | Fanaticism Aura, Extra Strong |
| Sceau B-2 | Pack d'Oblivion Knights | Curseurs de Iron Maiden |
| Sceau C | Infector of Souls (Venom Lord SU) | Extra Fast, Spectral Hit |

#### Attaques et sorts

1. **Lightning Hose (Red Lightning)** — Jet continu d'éclairs rouges en rotation ; sa signature
2. **Fire Inferno** — Inferno de feu directionnel, longue portée
3. **Cold Fingers** — Projectiles de glace en arc large
4. **Firestorm** — AoE de flammes au sol sous le joueur
5. **Bone Prison** — Emprisonne le joueur
6. **Armageddon** — Météores qui tombent en zone large (charge spéciale)
7. **Telekinesis** — Pousse/repousse le joueur à distance
8. **Melee** — Corps-à-corps puissant si adjacent ; ×7 dégâts aux minions

#### Clone Diablo (mécaniques Battle.net originales)

- Apparaît dans Battle.net public quand un certain nombre de **Stones of Jordan** sont vendus à un marchand dans un realm
- Message d'annonce : *"Diablo walks the earth"*
- Clone Diablo spawn dans toutes les parties Hell ouvertes ce realm
- Drop : **Annihilus** (unique small charm — +1 All Skills, +All Stats, +All Resistances)
- Non reproductible en single-player sans mods

---

### 6.5 Baal — Seigneur de la Destruction (Acte 5)

**Sodomight Name :** Baalrok le Destructeur
**Zone :** Worldstone Chamber (après les 5 vagues)
**Type :** Demon | **Regen HP :** Non

#### Stats par difficulté

| Stat | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Niveau | 60 | 75 | 99 |
| HP | 26 484 | 117 596 | **493 701** |
| XP | 216 862 | 1 619 522 | 4 536 276 |
| Rune max drop | Io | Vex | **Zod** |

#### Résistances

| Type | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Feu | 25% | 50% | 50% |
| Froid | 25% | 50% | 50% |
| Foudre | 25% | 50% | 50% |
| Poison | 25% | 50% | 50% |
| Physique | 0% | 0% | 50% |
| Magie | 0% | 0% | 0% |

#### Les 5 Vagues du Trône de Destruction

| Vague | Leader (SU) | Monstre de base | Immunité monstre | Notes |
|-------|-------------|----------------|-----------------|-------|
| 1 | **Colenzo the Annihilator** | Warped One + Warped Shaman | Aucune | Ressuscitement constant |
| 2 | **Achmel the Cursed** | Greater Mummy + Unravelers | Magie | Aura poison au contact |
| 3 | **Bartuc the Bloody** | Council Members | Aucune (rés. foudre élevée) | Erratique, se disperse |
| 4 | **Ventar the Unholy** | Venom Lords | Aucune | Inferno rapide, très mobile |
| 5 | **Lister the Tormentor** | Minions of Destruction | **Feu** | Spectral Hit + régénération HP |

#### Baal Clone

- Avant d'affronter Baal, un **clone de Baal** apparaît comme distraction
- Le clone absorbe des attaques mais est moins puissant
- Après sa mort, le vrai Baal spawn

#### Attaques et sorts de Baal

1. **Tentacles (Baal's Tentacles)** — Tentacules qui sortent du sol en zone AoE ; dégâts physiques + ralentissement
2. **Mana Rift** — Drain massif de mana + dégâts ; régénération accélérée pour Baal
3. **Hoarfrost** — Nova de givre massive ; dégâts de froid AoE
4. **Cold Meteorite** — Météorites de glace qui tombent aléatoirement
5. **Vile Effigy** — Invoque une effigie qui lance Inferno + Charged Bolts
6. **Clone Wave** — Reproduit une version miroir de lui-même temporairement
7. **Decrepify Curse** — Lance la malédiction Decrepify (–50% vitesse, –50% dégâts)

#### Popularité farm

- **XP le plus élevé** de tous les bosses en Hell (4.5M)
- Vagues du Trône donnent également beaucoup d'XP
- Run "Baal run" = farm XP pour niveau 90–99
- Loot endgame : Tyrael's Might, The Grandfather, Windforce, Crown of Ages

---

## 7. Système Über et Pandémonium

### 7.1 Conditions d'accès

```
Système Key → Organes → Über Tristram
```

#### Étape 1 : Collecte des 9 Clés

| Clé | Source | Boss Source | Sodomight |
|-----|--------|------------|-----------|
| Key of Terror (×3) | The Countess | Forgotten Tower L5 | Clé de Terreur |
| Key of Hate (×3) | The Summoner | Arcane Sanctuary | Clé de Haine |
| Key of Destruction (×3) | Nihlathak | Halls of Vaught | Clé de Destruction |

#### Étape 2 : Pré-Über Bosses (3 Clés → 1 Organe)

Transmutation Horadric Cube :
- **1 Key of Terror + 1 Key of Hate + 1 Key of Destruction** → Portail aléatoire vers l'un des 3 pré-Über

| Pré-Über | Boss | Zone | Drop |
|----------|------|------|------|
| **Lilith** | Démon féminin unique | Matron's Den | Diablo's Horn |
| **Über Duriel** | Duriel renforcé | Forgotten Sands | Baal's Eye |
| **Über Izual** | Izual renforcé | Furnace of Pain | Mephisto's Brain |

#### Étape 3 : Über Tristram

Transmutation dans Harrogath (Hell) :
- **Diablo's Horn + Baal's Eye + Mephisto's Brain** → Portail rouge vers Über Tristram

### 7.2 Les Trois Über Bosses (simultanés)

| Boss | Sodomight | Mécanique principale | Immunité |
|------|-----------|---------------------|---------|
| **Über Mephisto** | Méphikar Über | Conviction Aura lv20 (–125% rés.) + Spawn Skeleton Mages | Physique |
| **Pandemonium Diablo** | Diablon Über | Tank massif ; spawn Pit Lords immuns physique | Magie |
| **Über Baal** | Baalrok Über | Toutes attaques Baal ×5 vitesse ; Mana Rift frénétique ; régénère HP vite | Froid |

#### Stratégies

1. **Séparer les bosses** — Entrer dans Tristram, attirer Über Mephisto vers le coin gauche
2. **Tuer Über Mephisto en premier** — Sa Conviction Aura est la plus dangereuse
3. **Gérer la Conviction** — Résistances doivent être > 125% avec buff pour ne pas être à –% en présence
4. **Pit Lords de Pandemonium Diablo** — Immunisés sauf physique ; utiliser un barbare ou paladin smite
5. **Über Baal en dernier** — Régénération HP massive si Mana Rift, besoin de builds haute DPS

### 7.3 Récompense

- **Hellfire Torch** — Unique Large Charm
  - +3 à tous les skills d'**une classe aléatoire**
  - +10 à 20 à tous les attributs
  - +10 à 20 à toutes les résistances
  - Standard of Heroes (trophée cosmétique)

---

## 8. Archétypes d'IA — Comportements

### 8.1 Catalogue des comportements IA

| ID IA | Nom | Paramètres configurables | Description |
|-------|-----|--------------------------|-------------|
| `melee_chase` | Poursuite corps-à-corps | `aggro_range`, `walk_speed`, `run_speed`, `attack_speed` | Suit le joueur et attaque au contact |
| `melee_slow` | Lent corps-à-corps | `walk_speed`, `attack_damage_mul` | Marche vers le joueur, jamais ne fuit (Zombie) |
| `melee_fast` | Rapide corps-à-corps | `run_speed`, `attack_rate` | Sprint frénétique, attaque rapide (Spike Fiend) |
| `melee_charge` | Charge linéaire | `charge_range`, `charge_speed`, `charge_damage_mul` | Charge si cible à > N tiles (Goatman) |
| `melee_aoe` | Corps-à-corps zone | `aoe_radius`, `aoe_damage` | Frappe touchant plusieurs cibles (Blunderbore) |
| `melee_thorns` | Épines reflétées | `thorns_damage`, `thorns_radius` | Dégâts en retour sur melee (Thorned Hulk) |
| `ranged_static` | Tir statique | `fire_rate`, `projectile_speed`, `range` | Reste immobile, tire (Skeleton Archer) |
| `ranged_kite` | Kite et tir | `kite_distance`, `fire_rate`, `flee_speed` | Tire, recule si trop proche (Corrupt Rogue) |
| `ranged_retreat` | Retraite active | `safe_distance`, `retreat_speed` | Recule si joueur < N tiles (Skeleton Mage) |
| `ranged_lightning` | Tir foudre | `fire_rate`, `chain_lightning`, `mana_drain_chance` | Tire éclairs, drain mana (Gloam) |
| `ranged_poison` | Tir poison | `fire_rate`, `poison_duration`, `poison_damage` | Tir de dards empoisonnés (Fetish Blowdart) |
| `ranged_cb` | Charged Bolts | `bolt_count`, `bolt_spread`, `fire_rate` | Éventail de petits éclairs (Succubus) |
| `pack_flee` | Fuite en groupe | `flee_trigger_radius`, `flee_duration`, `rally_delay` | Fuit à chaque kill proche, se retourne après N secondes (Fallen) |
| `shamanic` | Ressurrection chamanique | `resurrect_range`, `resurrect_count`, `resurrect_cooldown`, `target_family` | Ressuscite les alliés morts de sa propre famille |
| `shamanic_resurrect` | Résurrection avancée | `can_resurrect_shamans`, `max_risen` | Peut ressusciter d'autres shamans (Boss Shaman uniquement) |
| `aura_bearer` | Porteur d'aura | `aura_type`, `aura_level`, `aura_radius` | Génère une aura Paladin passive (Aura Enchanted) |
| `curse_caster` | Lanceur de malédiction | `curse_type`, `curse_duration`, `curse_interval`, `switch_interval` | Lance malédiction Nécromancien en boucle (Oblivion Knight) |
| `bomber` | Explosion suicide | `arm_distance`, `run_speed`, `explosion_radius`, `explosion_damage` | Court vers le joueur et explose (Suicide Minion) |
| `teleport_harass` | Harcèlement par téléport | `teleport_range`, `teleport_cooldown`, `attack_window` | Téléporte autour du joueur, frappe, disparaît |
| `leap_attack` | Bond d'attaque | `leap_range`, `leap_cooldown`, `land_damage` | Bond périodique vers le joueur (Spider, Leaper) |
| `leap_pack` | Bond groupé | `pack_leap_sync`, `leap_spread` | Plusieurs monstres bondissent simultanément |
| `aerial_dive` | Piqué aérien | `dive_speed`, `hover_height`, `land_attack` | Vol, atterrissage + attaque, redécollage (Hawk) |
| `aerial_strafe` | Strafe aérien | `strafe_speed`, `attack_on_pass` | Passe en vol, attaque au passage (Vulture) |
| `phase_chase` | Traversée de terrain | `phase_obstacles`, `drain_on_touch` | Ignore les murs, drain vie contact (Wraith) |
| `burrow_attack` | Terrier | `burrow_time`, `surface_radius`, `surface_damage` | Disparaît dans le sol, réapparaît sous le joueur |
| `burrow_queen` | Reine terricole | `egg_lay_interval`, `egg_count`, `travel_speed` | Pond des œufs en se déplaçant (Sand Maggot) |
| `swarm_envelop` | Enveloppement en essaim | `swarm_count`, `envelop_radius`, `sting_damage` | Entoure le joueur, multiples petites attaques |
| `swarm_rush` | Ruée en essaim | `rush_speed`, `swarm_bonus_near_allies` | Fonce en masse, bonus si nombreux (Sand Maggot Young) |
| `swarm_melee` | Melee en meute | `flee_if_alone`, `pack_bonus` | Normal si en groupe, fuit si seul (Rat Man) |
| `spawn_burst` | Spawn en rafale | `spawn_count`, `spawn_interval`, `spawn_type` | Spawne des minions périodiquement (Vile Mother) |
| `summon_minions` | Invocation | `summon_type`, `summon_count`, `summon_cooldown` | Invoque des monstres (Boss Goat Shaman) |
| `mage_bone` | Mage osseux | `bone_prison_cooldown`, `teeth_count`, `teleport_escape_hp` | Bone Prison, Teeth, téléport si attaqué |
| `mage_melee` | Mage corps-à-corps | `cast_range`, `approach_threshold`, `cast_cooldown` | Lance sorts à distance, approche si sorts insuffisants |
| `ranged_mage` | Mage pur | `safe_distance`, `teleport_on_approach`, `spell_rotation` | Reste loin, téléporte si approché |
| `inferno_melee` | Inferno + melee | `inferno_range`, `inferno_duration`, `melee_damage` | Inferno en approche, melee au contact |
| `commander_aura` | Commandant | `whip_cooldown`, `ally_buff_range`, `buff_type` | Fouette les alliés pour les booster (Overseer) |
| `cold_freeze_melee` | Gel corps-à-corps | `freeze_radius`, `freeze_duration`, `melee_cold_damage` | Gèle les proches, melee lent puissant |
| `drain_life_fly` | Drain vie volant | `drain_rate`, `fly_speed`, `hover_pattern` | Vol, drain vie passif au contact |
| `charge_element` | Charge élémentaire | `charge_speed`, `element_type`, `aoe_on_land` | Charge et libère un élément à l'impact |
| `static_turret` | Tourelle statique | `fire_rate`, `fire_type`, `range` | Immobile, tire périodiquement (Fire Tower) |
| `berserk_undead` | Berserker mort-vivant | `frenzy_bonus_nearby`, `melee_speed` | Attaque frénétique, bonus en groupe |
| `sprint_melee` | Sprint poignard | `sprint_speed`, `attack_on_reach` | Sprint frénétique direct vers le joueur (Fetish) |
| `leap_acid` | Bond acide | `leap_range`, `acid_splash_radius`, `poison_damage` | Bond, crache acide à l'atterrissage (Frog Demon) |

### 8.2 Paramètres globaux IA

```toml
[ai.global]
# Rayon de détection de base (tiles)
default_aggro_range = 8
# Rayon de désengagement
default_deaggro_range = 20
# Comportement si hors de portée
leash_return = true
leash_return_hp_percent = 100  # Régénère si retourne à spawn

# Comportement des groupes
pack_shared_aggro = true       # Un monstre aggro = tous aggrés
pack_flee_radius = 5.0         # Rayon fuite Fallen
```

---

## 9. Schémas TOML Sodomight

### 9.1 Monstre de base

```toml
# @id monster.fallen_shaman
# @do define_base_monster
# @role entity_definition
# @layer 4
# @human Fallen Shaman — Shaman ressusciteur de la famille Fallen

[monster.fallen_shaman]
id = "fallen_shaman"
sodomight_name = "Chu'fa Seer"
d2_name = "Fallen Shaman"
act = 1
family = "fallen"
monster_type = "Demon"
can_revive = false          # Le nécromancien ne peut pas Revive un shaman

[monster.fallen_shaman.stats.normal]
hp_base = 18
hp_per_extra_player = 3      # Formule: hp_base * (N+1)/2 avec N joueurs
defense = 20
damage_min = 3
damage_max = 6
attack_rating = 50
experience = 85

[monster.fallen_shaman.stats.nightmare]
hp_base = 160
hp_per_extra_player = 28
defense = 180
damage_min = 20
damage_max = 38
attack_rating = 350
experience = 2800

[monster.fallen_shaman.stats.hell]
hp_base = 420
hp_per_extra_player = 74
defense = 520
damage_min = 55
damage_max = 100
attack_rating = 900
experience = 14000

[monster.fallen_shaman.resistances.normal]
fire = 50
cold = 0
lightning = 0
poison = 0
physical = 0
magic = 0

[monster.fallen_shaman.resistances.hell]
fire = 50
cold = 0
lightning = 0
poison = 0
physical = 0
magic = 0
immunities = []

[monster.fallen_shaman.ai]
type = "shamanic"
aggro_range = 10.0
resurrect_range = 8.0
resurrect_count = 3            # Max morts ressuscités simultanément
resurrect_cooldown = 8.0       # Secondes entre résurrections
target_family = "fallen"       # Ne ressuscite QUE les Fallen / Carver de sa famille
flee_on_low_hp = false
fireball_range = 10.0
fireball_cooldown = 2.0

[monster.fallen_shaman.loot]
treasure_class = "Act 1 (Shaman)"
gold_drop_min = 5
gold_drop_max = 20
```

### 9.2 Affix Champion

```toml
# @id champion_affix.cursed
# @do define_affix
# @role game_mechanic
# @layer 4
# @human Affix Maudit — inverse les soins en dégâts

[champion_affix.cursed]
id = "cursed"
sodomight_name = "Maudit"
d2_name = "Cursed"
effect_type = "on_hit_curse"

[champion_affix.cursed.effect]
curse = "amplify_damage"
curse_chance = 0.50           # 50% de chance par hit
curse_duration_seconds = 8.0
amplify_damage_multiplier = 2.0  # Les dégâts physiques reçus sont doublés

[champion_affix.cursed.meta]
stack = false
danger_rating = 9
can_appear_on_champion = true
can_appear_on_unique = true
can_appear_on_super_unique = true
dangerous_combos = ["fire_enchanted", "extra_strong", "stone_skin"]
```

### 9.3 Super Unique

```toml
# @id super_unique.the_countess
# @do define_super_unique
# @role entity_definition
# @layer 4
# @human The Countess — Super Unique de la Tour Oubliée, source de runes

[super_unique.the_countess]
id = "the_countess"
sodomight_name = "La Tisserune"
d2_name = "The Countess"
base_monster = "dark_stalker"
act = 1
zone = "forgotten_tower_level_5"
spawn_guaranteed = true       # Spawn TOUJOURS, même jeu fixé

[super_unique.the_countess.affixes_fixed]
affixes = ["fire_enchanted"]

[super_unique.the_countess.affixes_random]
nightmare_bonus = 1           # +1 affix aléatoire en NM
hell_bonus = 2                # +2 affixes aléatoires en Hell

[super_unique.the_countess.stats.hell]
hp_base = 2800
minion_count = 8              # 8 Dark Stalkers comme acolytes
minion_type = "dark_stalker"

[super_unique.the_countess.loot_special]
rune_drop = true
max_rune_normal = "Ral"       # Rune max en Normal
max_rune_nightmare = "Ko"     # Rune max en NM
max_rune_hell = "Ist"         # Rune max en Hell
rune_drop_weight = 3.0        # 3× plus de chance de dropper des runes
```

### 9.4 Boss d'Acte

```toml
# @id act_boss.andariel
# @do define_act_boss
# @role entity_definition
# @layer 4
# @human Andariel — Boss Acte 1, Reine des Anguilles

[act_boss.andariel]
id = "andariel"
sodomight_name = "Andara l'Infestée"
d2_name = "Andariel"
act = 1
zone = "catacombs_level_4"
monster_type = "Demon"
hp_regen = false
random_modifiers = false

[act_boss.andariel.stats.normal]
level = 12
hp = 1024
defense = 75
experience = 1282

[act_boss.andariel.stats.nightmare]
level = 49
hp = 24800
defense = 420
experience = 92295

[act_boss.andariel.stats.hell]
level = 75
hp = 60031
defense = 760
experience = 561066

[act_boss.andariel.resistances.all_difficulties]
fire = -50          # Vulnérable au feu !
cold = 50
lightning = 50
poison = 50
physical = 0
magic = 0

[act_boss.andariel.resistances.hell_override]
cold = 66
lightning = 66
poison = 66
physical = 66

[act_boss.andariel.ai]
type = "multi_phase_boss"
phases = [
  { hp_threshold = 1.0, mode = "standard" },
  { hp_threshold = 0.5, mode = "enraged", speed_multiplier = 1.5 }
]

[act_boss.andariel.attacks]
scythe_strike = { damage_min = 20, damage_max = 40, poison_damage = 50, poison_duration = 3.0 }
poison_cloud = { radius = 3.0, damage_per_sec = 80, duration = 3.0, cooldown = 4.0 }
poison_spray = { range = 8.0, damage = 100, poison_duration = 5.0, cooldown = 3.0 }
charge = { range = 10.0, speed = 3.0, damage_multiplier = 2.0, cooldown = 6.0 }
```

### 9.5 Archétype IA complet

```toml
# @id ai_archetype.shamanic
# @do define_ai_behavior
# @role ai_definition
# @layer 4
# @human IA Shamanique — ressuscite les alliés morts de sa famille

[ai_archetype.shamanic]
id = "shamanic"
description = "Ressuscite les alliés morts de sa propre famille"

[ai_archetype.shamanic.params]
aggro_range = 10.0            # Distance détection joueur
resurrect_range = 8.0         # Portée de résurrection
resurrect_count = 3           # Max simultanés ressuscités
resurrect_cooldown = 8.0      # Cooldown résurrection (secondes)
target_family = ""            # Vide = famille du shaman lui-même
flee_on_low_hp = false        # Ne fuit jamais
can_resurrect_shamans = false # Seuls les boss shamans peuvent ressusciter d'autres shamans
fireball_range = 10.0
fireball_cooldown = 2.0
move_to_corpse = true         # Se déplace vers les corps pour ressusciter
max_chase_range = 20.0        # Distance max poursuite avant leash

[ai_archetype.shamanic.state_machine]
states = ["idle", "alert", "chase", "resurrect", "attack", "flee"]
transitions = [
  { from = "idle", to = "alert", condition = "player_in_aggro_range" },
  { from = "alert", to = "chase", condition = "player_confirmed" },
  { from = "chase", to = "resurrect", condition = "dead_ally_in_resurrect_range" },
  { from = "resurrect", to = "attack", condition = "no_dead_allies_nearby" },
  { from = "attack", to = "resurrect", condition = "dead_ally_detected" },
]
```

---

## 10. Système de Scaling Multijoueur

### 10.1 Formule HP

```
HP_final = HP_base × (N_joueurs + 1) / 2
```

| Joueurs (N) | Multiplicateur HP |
|-------------|------------------|
| 1 | ×1.0 (100%) |
| 2 | ×1.5 (150%) |
| 3 | ×2.0 (200%) |
| 4 | ×2.5 (250%) |
| 5 | ×3.0 (300%) |
| 6 | ×3.5 (350%) |
| 7 | ×4.0 (400%) |
| 8 | ×4.5 (450%) |

> Note : Les HP d'un monstre sont **fixés au moment du spawn**. Ajouter des joueurs après le
> spawn d'un monstre ne change PAS ses HP (sauf si le monstre respawn).

### 10.2 Formule Expérience

```
XP_final = XP_base × (N_joueurs + 1) / 2
```

L'expérience scale identiquement aux HP. Chaque joueur supplémentaire augmente l'XP de 50%.

### 10.3 Dégâts et Attack Rating (Nightmare/Hell uniquement)

```
AR_final = AR_base × (1 + 0.0625 × (N-1))
Dmg_final = Dmg_base × (1 + 0.0625 × (N-1))
```

En Nightmare et Hell : +6.25% dégâts et AR par joueur supplémentaire.

| Joueurs | Bonus Dmg/AR (NM/H) |
|---------|---------------------|
| 1 | +0% |
| 2 | +6.25% |
| 3 | +12.5% |
| 4 | +18.75% |
| 5 | +25% |
| 6 | +31.25% |
| 7 | +37.5% |
| 8 | +43.75% |

### 10.4 Réduction XP en groupe

- **Pénalité de groupe** : L'XP est divisé entre tous les joueurs qui ont participé au kill
- Un joueur seul → 100% de l'XP
- 2 joueurs → 2 × (75% / 2) = 75% chacun
- Le bonus de pool partiellement compense la pénalité

### 10.5 Comportements spéciaux en groupe

- **Aggro partagé** : Si un monstre du pack aggro, tout le pack aggro simultanément
- **Fallen pack flee** : La fuite Fallen se propage aux Fallen voisins (rayon 5 tiles)
- **Shamanic priorité** : Le shaman cible en priorité les corps quand des joueurs sont présents
- **Commande d'Overseer** : L'aura de boost de l'Overseer s'applique à tous les monstres dans le rayon, pas seulement son pack

### 10.6 Implémentation TOML

```toml
[multiplayer_scaling]
hp_formula = "base * (players + 1) / 2"
xp_formula = "base * (players + 1) / 2"
ar_bonus_per_player = 0.0625     # NM/Hell seulement
dmg_bonus_per_player = 0.0625    # NM/Hell seulement
scaling_applies_to_nightmare = true
scaling_applies_to_hell = true
scaling_applies_to_normal = false  # Dégâts/AR pas scalés en Normal

# HP fixés au spawn
hp_fixed_at_spawn = true
respawn_recalculates_hp = true
```

---

## 11. Références et Sources

- [The Arreat Summit — Monsters (officiel Blizzard)](https://classic.battle.net/diablo2exp/monsters/)
- [Superuniques — Diablo2 Wiki](https://diablo2.diablowiki.net/Superuniques)
- [Monster Modifiers — Diablo2 Wiki](https://diablo2.diablowiki.net/Monster_modifier)
- [Elite Monster Affixes — Maxroll.gg](https://maxroll.gg/d2/resources/elite-monster)
- [All Monsters Database — diablo2.io](https://diablo2.io/monsters/)
- [Bosses & Super Uniques — Maxroll.gg](https://maxroll.gg/d2/resources/bosses)
- [Andariel Stats — RankedBoost](https://rankedboost.com/diablo-2/bosses/andariel/)
- [Mephisto Stats — RankedBoost](https://rankedboost.com/diablo-2/bosses/mephisto/)
- [Baal Stats — RankedBoost](https://rankedboost.com/diablo-2/bosses/baal/)
- [Duriel Stats — Diablo2 Wiki](https://diablo2.diablowiki.net/Duriel)
- [Über Tristram Guide — Maxroll.gg](https://maxroll.gg/d2/meta/ubers-explained)
- [Monster HP Scaling — Phrozen Keep](https://d2mods.info/forum/viewtopic.php?t=63578)
- [Monster AI Types — Phrozen Keep](https://d2mods.info/forum/viewtopic.php?t=11660)
- [Act I–V Bestiaries — Diablo Fandom](https://diablo.fandom.com/wiki/Diablo_II_Bestiary)
- [Mephisto Moat Trick — PureDiablo](https://www.purediablo.com/forums/threads/explain-the-moat-trick-plz.134127/)

---

*Document généré pour le projet Sodomight — MGE ECS archetype data-driven*
*Crate cible : `mge-arpg-ai`, `mge-arpg-entity`*
