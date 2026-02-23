# Allumina — Analyse Technique des Systèmes Diablo II — Transposition MGE

## Contexte

Ce document est une **analyse technique exhaustive** des systèmes mécaniques de Diablo II (2000, Blizzard North), destinée à servir de référence pour l'implémentation d'Allumina sur le moteur MGE (Miyukini Game Engine). L'analyse est fondée sur le reverse-engineering communautaire (D2MOO, Phrozen Keep, données moddées), la documentation des fichiers .txt de configuration, et des hypothèses argumentées lorsque l'information est incertaine.

## Portée / Scope

- **Applicable à :** Implémentation moteur, plugins MGE, game design technique Allumina.
- **Audience :** Développement, architecture MGE, game design.
- **Statut :** Document de référence technique.

### Hors périmètre

- Lore, narration, historique du jeu.
- Comparaison subjective entre ARPG.
- Design des assets graphiques.

---

# 1. SYSTÈME DE DÉPLACEMENT

## 1.1 Structure fondamentale

### Grille isométrique diagonale

Diablo II utilise une **grille de tuiles orientée en diagonale** pour créer sa vue isométrique. Ce n'est pas de la navigation libre : tout positionnement est ancré sur une grille de **subtiles**.

| Paramètre | Valeur |
|-----------|--------|
| **Type de grille** | Isométrique losange, tuiles diamant |
| **Taille d'une tuile** | 160×80 px (affichage) |
| **Subtile** | 32×16 px (affichage), 5 subtiles par tuile en X et Y |
| **Coordonnées** | Entières en subtiles, position serveur en subtiles |
| **Unité de distance** | 1 yard = 1.5 subtiles (24px vertical × 48px horizontal) |
| **Interpolation visuelle** | Oui — le client interpole entre les positions serveur à 25 FPS |

### Footprint des unités

Chaque personnage et la plupart des monstres occupent un **footprint en X** couvrant 5 subtiles :

```
    [W]
  [W][C][W]
    [W]

C = centre (bloquant), W = wings (non bloquant, peuvent chevaucher)
```

Le centre ne peut **jamais** chevaucher celui d'une autre unité (sauf Teleport sur unités alliées). Les ailes peuvent se superposer, permettant un placement serré.

### Transposition MGE

| Concept D2 | Équivalent MGE |
|------------|----------------|
| Position subtile (entière) | `Position2D` (Vec2, px) — coordonnées flottantes MGE |
| Footprint en X | `Hitbox` composant (cercle ou AABB configurable) |
| Interpolation client | Interpolation native via `Velocity2D` et `LocomotionParams` |
| Grille subtile | Optionnel — MGE peut utiliser une grille logique superposée ou du positionnement libre |

**Recommandation MGE :** utiliser des coordonnées flottantes (Vec2) avec une grille logique optionnelle pour le pathfinding. Le footprint en X est modélisable par un cercle de rayon configurable dans le composant hitbox.

---

## 1.2 Pathfinding

### Algorithme

Diablo II utilise un **pathfinding basé sur les SubTilesFlags** extraits des fichiers DT1 (textures de tuiles). Chaque subtile possède un champ de bits indiquant la passabilité.

| Aspect | Détail |
|--------|--------|
| **Algorithme** | A* limité (portée courte ~35 subtiles), recalcul fréquent |
| **Données de passabilité** | Bit fields par subtile, combinés sur toutes les couches de la carte |
| **Flags distincts** | Joueur-walkable vs mercenaire-walkable vs missile-passable |
| **PNJ statiques** | Chemins précalculés dans les fichiers DS1 (paths prédéfinis) |

### Gestion des collisions dynamiques

Les monstres ont une portée de pathfinding **limitée**. Quand un monstre ne peut pas atteindre sa cible :
1. Il tente un A* sur ~35 subtiles
2. Si le chemin échoue, il se déplace en ligne droite vers la cible
3. S'il est bloqué, il entre en état d'attente puis réessaie

### Gestion des obstacles destructibles

Les obstacles destructibles (barils, murs fissurés) sont traités comme des tuiles bloquantes jusqu'à destruction, puis le flag de passabilité est mis à jour.

### Recalcul

Le pathfinding est recalculé à chaque **AI tick** du monstre (contrôlé par `aidel` dans MonStats.txt). Le joueur recalcule à chaque clic ou changement de direction.

### Pseudo-algorithme de pathfinding D2

```
fn pathfind(unit, target_pos):
    if distance(unit.pos, target_pos) > MAX_PATH_RANGE:
        return move_towards_direct(unit, target_pos)
    
    path = a_star(
        start = unit.subtile_pos,
        goal = target_pos.to_subtile(),
        passability = get_subtile_flags(unit.walk_type),
        max_nodes = 200   // limite CPU
    )
    
    if path.is_empty():
        return move_towards_direct(unit, target_pos)
    
    unit.waypoints = path
    unit.waypoint_index = 0
```

### Transposition MGE

Le skill `miyukini-deplacement-orientation` définit déjà le pathfinding par waypoints avec A*. La chaîne de locomotion MGE (input → accel/friction → clamp → displacement → rotation) est directement applicable.

| Concept D2 | Plugin MGE |
|------------|------------|
| SubTilesFlags | Grille de passabilité `NavigationGrid` (composant) |
| A* limité | `mge-plugin-pathfinding` avec max_nodes configurable |
| Waypoints DS1 | `waypoints` + `waypoint_index` (déjà dans MGE) |
| Walk types distincts | `collision_layers` (layer, mask) — déjà dans MGE |

---

## 1.3 Hitbox et collision

### Formes de hitbox

Diablo II utilise **deux systèmes de hitbox** distincts (définis dans MonStats2.txt) :

| Système | Usage | Paramètres |
|---------|-------|------------|
| **SizeX/SizeY** | Collision physique (déplacement, blocage) | Diamètre en subtiles (1-3), joueur = 2 |
| **htTop/htLeft/htWidth/htHeight** | Hitbox d'attaque / sélection (graphique) | Rectangle superposé, pivot = animation pivot |

Le champ `NoGfxHitTest` contrôle quel système est utilisé pour la détection de collision :
- `0` : utilise SizeX/SizeY (standard)
- `1` : utilise le rectangle superposé (htTop/htLeft/htWidth/htHeight)

### Séparation physique vs attaque

```
┌───────────────────────────────┐
│  htWidth × htHeight           │ ← Hitbox de sélection/attaque
│  ┌─────────────┐              │
│  │ SizeX×SizeY │              │ ← Hitbox physique (collision)
│  └─────────────┘              │
└───────────────────────────────┘
```

### Priorité de collision

1. **Mur / terrain** : toujours bloquant (subtile flags)
2. **Unités vivantes** : bloquent par SizeX/SizeY (centre du footprint X)
3. **Cadavres** : non bloquants
4. **Missiles** : passent à travers les unités (sauf CollideType spécifique)
5. **Pets entre eux** : peuvent se bloquer mutuellement (problème connu des nécromanciens)

### Transposition MGE

```rust
// Composant MGE pour hitbox à la D2
pub struct HitboxD2 {
    pub physical_radius: f32,     // équivalent SizeX/SizeY
    pub selection_rect: Rect,     // équivalent htTop/htLeft/htWidth/htHeight
    pub use_gfx_hitbox: bool,     // équivalent NoGfxHitTest
}
```

Le skill `miyukini-deplacement-orientation` prévoit déjà `collision_layers (layer, mask)` pour séparer les couches physiques.

---

## 1.4 Contraintes moteur

### Tick rate

| Paramètre | Valeur |
|-----------|--------|
| **Tick rate interne** | **25 FPS** fixe — toute la logique de jeu |
| **Plus petite unité de temps** | 1/256 de seconde (pour les calculs de précision) |
| **dt par frame** | 40 ms (1/25) |
| **Animations** | Liées au tick rate — les breakpoints existent car les animations sont des frames discrètes à 25 FPS |

### Breakpoints (phénomène D2 spécifique)

Les améliorations de vitesse (FCR, IAS, FHR, FBR) ne deviennent effectives que quand elles **retirent une frame d'animation**. Les réductions partielles sont arrondies vers le haut → aucun effet.

```
Exemple Sorcière — Faster Cast Rate :
  Base : 13 frames (13 × 40ms = 520ms)
  9% FCR  → 12 frames (480ms) ← premier breakpoint
  20% FCR → 11 frames (440ms)
  ...
  200% FCR → 7 frames (280ms) ← dernier breakpoint
```

**Implication MGE :** le moteur MGE utilisant des coordonnées flottantes et un dt variable, les breakpoints ne sont **pas nécessaires**. La vitesse d'animation peut être interpolée de manière continue. Cependant, si Allumina veut reproduire le *feel* D2, un système de breakpoints optionnel peut être implémenté comme un plugin.

### Désynchronisation réseau

| Aspect | D2 Original |
|--------|-------------|
| **Architecture** | Client-serveur (Battle.net) ou peer-to-peer (TCP/IP) |
| **Prédiction client** | Limitée — le client prédit le mouvement mais le serveur fait autorité |
| **Rubber-banding** | Fréquent — le joueur « snap » à sa position serveur en cas de désync |
| **NHAM bug** | Next Hit Always Misses — désync entre animation client et état serveur lors d'interruptions |

### Transposition MGE (réseau)

Allumina utilise le MWS (Miyukini Webway System) avec un modèle Lobby (hôte = serveur). Le modèle recommandé :

| Aspect | Recommandation |
|--------|----------------|
| **Autorité** | Hôte du Lobby = serveur autoritaire |
| **Prédiction** | Client prediction avec réconciliation serveur |
| **Tick rate** | 30 FPS logique (plus fluide que D2, coût CPU acceptable) |
| **Interpolation** | Client interpole entre états serveur reçus |

---

# 2. COMPORTEMENT DES ENTITÉS (IA)

## 2.1 Architecture IA

### Modèle : Table-Driven FSM (Machine à États Finis piloté par données)

Diablo II utilise un système hybride :
- **FSM hardcodée** : états prédéfinis dans le code C++ (AiThink.cpp dans D2MOO)
- **Configuration par tables** : MonStats.txt fournit les paramètres qui pilotent les transitions et le comportement

Ce n'est **ni** un behavior tree, **ni** un script pur. C'est une FSM dont les transitions sont paramétrées par des fichiers .txt.

### États typiques (reconstitués depuis D2MOO et MonStats.txt)

```
┌─────────┐    aggro      ┌─────────┐
│  IDLE   │──────────────→│  CHASE  │
└─────────┘               └────┬────┘
     ↑                         │
     │ no target          in range
     │                         ↓
┌─────────┐               ┌─────────┐
│  WANDER │←──leash────── │ ATTACK  │
└─────────┘               └────┬────┘
     ↑                         │
     │                    hit / stun
     │                         ↓
     │                    ┌─────────┐
     └──recover────────── │  STUN   │
                          └─────────┘
                               │
                          hp < threshold
                               ↓
                          ┌─────────┐
                          │  FLEE   │
                          └─────────┘
                               │
                          hp = 0
                               ↓
                          ┌─────────┐
                          │  DEAD   │
                          └─────────┘
```

### Hardcoded States (identifiés dans le code)

| ID | État | Effet |
|----|------|-------|
| 1 | Freeze | Arrêt total animations + IA |
| 11 | Cold | Vélocité et attack rate ÷ 2 |
| 13 | Blaze | Émet des missiles en se déplaçant |
| 15 | Concentrate | Interrompt les actions sauf si interrupt=1 |
| 22 | Spiderlay | Produit missile #143 en se déplaçant |

### Paramètres IA clés (MonStats.txt)

| Colonne | Rôle |
|---------|------|
| `AI` | Identifiant de l'IA utilisée (chaque IA = code C++ spécifique) |
| `aidel` / `aidel(N)` / `aidel(H)` | Délai entre les AI ticks (plus bas = plus agressif) |
| `aidist` / `aidist(N)` / `aidist(H)` | Distance d'activation en cells (défaut : 35 ≈ 1 écran) |
| `aip1` à `aip8` | Paramètres passés à l'IA (en %, usage dépend du type d'IA) |
| `threat` | Priorité de ciblage (plus haut = ciblé en premier par les ennemis) |

### Transposition MGE

```rust
// Plugin MGE : mge-plugin-ai-monster.v1
pub struct MonsterAiState {
    pub current: AiStateId,      // Idle, Chase, Attack, Stun, Flee, Dead
    pub ai_type: AiTypeId,       // Référence vers la table d'IA
    pub ai_delay: u32,           // Frames entre chaque AI tick
    pub ai_dist: f32,            // Distance d'activation (px)
    pub ai_params: [f32; 8],     // aip1-8 equivalent
    pub target: Option<EntityId>,
    pub threat_level: f32,
    pub last_tick: u32,
}
```

---

## 2.2 Système d'aggro

### Distance d'activation

| Paramètre | Valeur par défaut | Source |
|-----------|-------------------|--------|
| **Rayon d'activation** | 35 cells ≈ ~1 écran complet | `aidist` dans MonStats.txt |
| **Ligne de vue** | Requise pour la plupart des IA (pas de détection à travers les murs) |
| **Ligne de vue (vol)** | Monstres volants (`flying=1`) ignorent certains obstacles sol |

### Mémoire de cible

- Les monstres conservent leur cible tant qu'elle est **à portée de poursuite** (pas de timer de mémoire explicite dans les fichiers de config)
- Le **leashing** se produit quand la cible sort de la portée de pathfinding (~35 cells)
- Hypothèse : la mémoire de cible dure ~3-5 secondes après perte de ligne de vue (basé sur comportement observé)

### Priorité des cibles

Contrôlée par la colonne `threat` de MonStats.txt + paramètre `petIgnore` :

| Règle | Détail |
|-------|--------|
| `threat` élevé | Ciblé en priorité (ex : Maggot Eggs à threat=25 → merc les cible d'abord) |
| `petIgnore=1` | Le monstre ignore totalement les invocations et mercenaires → va directement au joueur |
| `primeevil=1` | +300% dégâts contre mercenaires et invocations (Diablo, Baal) |
| Fallback | Cible la plus proche si pas de priorité |

### Pseudo-algorithme d'aggro

```
fn ai_think(monster, world, dt):
    if monster.ai_tick_cooldown > 0:
        monster.ai_tick_cooldown -= 1
        return
    monster.ai_tick_cooldown = monster.ai_delay
    
    match monster.state:
        Idle | Wander =>
            targets = find_units_in_range(monster.pos, monster.ai_dist)
            targets = filter_line_of_sight(monster.pos, targets)
            if monster.pet_ignore:
                targets = targets.filter(|t| t.is_player())
            target = targets.sort_by(|t| -t.threat).first()
            if target.is_some():
                monster.target = target
                monster.state = Chase
        
        Chase =>
            if target_out_of_range(monster, LEASH_DIST):
                monster.target = None
                monster.state = Wander
            elif in_attack_range(monster, monster.target):
                monster.state = Attack
            else:
                pathfind_towards(monster, monster.target.pos)
        
        Attack =>
            execute_attack(monster, monster.target)
            monster.state = Chase  // retour à poursuite
        
        Flee =>
            move_away_from(monster, monster.target.pos)
            if monster.hp > flee_threshold:
                monster.state = Chase
```

---

## 2.3 Variantes d'IA

### Table des comportements par type

| Type | Comportement | Particularités |
|------|-------------|----------------|
| **Mob standard** | Chase → Attack → Chase, fuite rare | aidel moyen (~8-12), distance standard |
| **Champion** | Plus agressif, même IA que standard | aidel réduit, +2 niveaux, HP ×3/2.5/2 |
| **Unique** | IA standard + affixes | +3 niveaux, HP ×4/3/2, 1-3 affixes selon difficulté |
| **Super Unique** | IA spécifique ou standard | Niveaux fixés (boss=1), hcIdx spécial, skills dédiés |
| **Boss (Act)** | IA hardcodée spécifique | primeevil=1 (+300% dmg vs pets), immunités, phases |
| **Ranged** | Maintient distance, fuit si approché | `rangedtype=1`, IA différente (kite) |
| **Melee** | Charge directe | Pas de multishot, priorité rapprochement |
| **Spawner** | Reste en arrière, produit des monstres | `placespawn=1`, utilise skill Nest/Minion Spawner |
| **Resurrecting** | Relève les morts de son type | Shamans (Fallen), Defilers — lié à BaseId |
| **Critter** | Fuit le joueur, non agressif | `critter=1`, `inert=1` |

### IA des Shamans (exemple complexe)

```
État: GUARD (près du camp)
  → Joueur détecté dans aidist
  → Transition: FLEE (s'éloigne du joueur)
  → Si allié mort détecté dans le rayon:
    → Transition: RESURRECT (caste S1 sur le cadavre)
  → Si menacé directement:
    → Transition: ATTACK (attaque à distance)
  → Si plus d'alliés morts:
    → Transition: FLEE
```

---

## 2.4 Gestion des groupes

### Coordination

| Mécanisme | Détail |
|-----------|--------|
| **Packs** | MinGrp/MaxGrp définit la taille du groupe à la génération |
| **Boss + Minions** | SetBoss=1 permet au "chef" de coordonner (ex : ordre de raid pour Scarabs) |
| **BossXfer** | Si le chef meurt, le leadership passe à un de ses minions |
| **Pas de coordination active** | Les mobs d'un même pack ne communiquent pas leur cible — chacun a sa propre boucle IA |

### Leashing

- Distance de leash : ~35 cells (distance d'activation IA)
- Pas de leash hard-reset : le monstre retourne à sa position de spawn en mode Wander
- Le monstre ne se soigne **pas** en retournant (contrairement à des ARPG modernes comme D3)

### Limite de poursuite

- Limitée par le pathfinding (portée A* ~35 subtiles)
- Les monstres volants (`flying=1`) ont une portée de poursuite plus grande (pas bloqués par obstacles sol)
- `opendoors=1/0` : contrôle si le monstre peut ouvrir les portes (lobotomisation si 0)

### Transposition MGE (groupes)

Voir `docs/Miyukini_Game_Engine/MGE - Pathfinding Collisions - Guide Entites Groupes.md` pour la gestion des groupes MGE (déjà documenté pour les scénarios musou/RTS).

```rust
pub struct PackLeader {
    pub minion_ids: Vec<EntityId>,
    pub boss_xfer: bool,       // leadership transférable
    pub raid_chance: f32,      // % chance d'ordonner un raid (aip5)
}

pub struct PackMember {
    pub leader: Option<EntityId>,
    pub pack_id: u32,
}
```

---

# 3. SYSTÈME DE SPAWN

## 3.1 Génération des monstres

### Architecture de la génération

Le spawn dans D2 est entièrement **table-driven** via plusieurs fichiers interconnectés :

```
Levels.txt          → Définit quels monstres peuvent spawner dans une zone
  ↓
MonStats.txt        → Définit les propriétés de chaque monstre
  ↓
MonType.txt         → Catégorie (super-groupe : skeleton, demon, etc.)
  ↓
ActInfo.txt         → Contrôle les monstres errants (wandering)
  ↓
TreasureClass.txt   → Contrôle le loot (séparé du spawn)
```

### Pondération probabiliste

Le champ `Rarity` de MonStats.txt contrôle la probabilité relative de spawn :

```
Exemple : 2 monstres éligibles pour une zone
  Monster A : Rarity = 10
  Monster B : Rarity = 1
  Total = 11
  
  Chance Monster A = 10/11 = 91%
  Chance Monster B = 1/11 = 9%
  
  Rarity = 0 → jamais sélectionné par Levels.txt
```

### Seed et génération procédurale

| Aspect | Détail |
|--------|--------|
| **Carte** | Générée procéduralement à partir d'un seed (stocké dans la save) |
| **Seed** | Détermine le layout des tuiles (DS1 presets combinés aléatoirement) |
| **Monstres** | Placés APRÈS la génération de carte, selon les tables |
| **Sparse populate** | `sparsePopulate` (0-100%) = chance qu'un monstre choisi soit effectivement placé |

### Influence du niveau de zone

| Difficulté | Niveau monstre | Source |
|------------|---------------|--------|
| **Normal** | Fixe (colonne Level de MonStats.txt) | MonStats.txt |
| **Nightmare** | = Area Level de Levels.txt | Levels.txt |
| **Hell** | = Area Level de Levels.txt | Levels.txt |
| **Boss (boss=1)** | Toujours depuis MonStats.txt | MonStats.txt (indépendant de la zone) |

### Monstres errants (Wandering)

Contrôlés par ActInfo.txt :

| Paramètre | Rôle |
|-----------|------|
| `wanderingMonsterPopulateChance` | % chance (0-100) de spawner un monstre errant |
| `wanderingMonsterRegionTotal` | Max de monstres errants simultanés |
| `wanderingNpcStart/Range` | Sélection aléatoire de la classe de monstre errant |

---

## 3.2 Pack generation

### Taille des groupes

Contrôlée par 4 colonnes dans MonStats.txt :

| Colonne | Rôle |
|---------|------|
| `MinGrp` / `MaxGrp` | Nombre d'unités de base spawned ensemble |
| `PartyMin` / `PartyMax` | Nombre de minions (Minion1/Minion2) accompagnant l'unité |

### Composition

| Type | Composition |
|------|-------------|
| **Pack standard** | Homogène : MinGrp-MaxGrp du même BaseId |
| **Pack avec minions** | Chef (unité principale) + PartyMin-PartyMax minions (type Minion1/Minion2) |
| **Champion pack** | 2-4 du même type, tous Champions (pas de chef) |
| **Unique pack** | 1 Unique + minions de son propre type (ou Minion1/2 si défini) |

### Placement

Les monstres sont placés aux positions disponibles (subtiles passables) autour d'un point de spawn, avec offset `spawnx`/`spawny` pour éviter l'empilement. Le système de collision empêche la superposition.

```
fn spawn_pack(zone, monster_id, count):
    center = find_valid_spawn_point(zone)
    for i in 0..count:
        pos = center + random_offset(spawnx, spawny)
        if is_walkable(pos) and no_collision(pos):
            spawn_monster(monster_id, pos, spawnmode)
```

---

## 3.3 Respawn

| Aspect | D2 comportement |
|--------|-----------------|
| **Respawn en jeu** | NON — les monstres tués restent morts pour la session |
| **Spawners** | Certaines unités (nids, etc.) produisent continuellement de nouveaux monstres — ce n'est pas du respawn mais du spawn dynamique |
| **Reset** | En quittant et recréant la partie, la carte est régénérée (nouveau seed) et les monstres réapparaissent |
| **Exception** | Certains monstres sont re-spawnable si un Shaman les ressuscite (morts-vivants bas → relèvement par morts-vivants hauts) |

### Transposition MGE

Pour Allumina (monde persistant type UO) : le respawn est nécessaire, contrairement à D2. Implémenter un timer de respawn par zone avec pondération :

```rust
pub struct SpawnZone {
    pub zone_id: u32,
    pub monster_table: Vec<SpawnEntry>,   // monster_id + rarity
    pub max_population: u32,
    pub respawn_delay: f32,               // secondes
    pub current_population: u32,
}

pub struct SpawnEntry {
    pub monster_id: u32,
    pub rarity: u32,
    pub min_group: u32,
    pub max_group: u32,
    pub sparse_chance: f32,    // 0.0-1.0
}
```

---

## 3.4 Spawn des élites

### Hiérarchie

```
┌─────────────────────────────────────────────────┐
│                 Super Unique                     │
│  (Noms fixes : Lord De Seis, Rakanishu, etc.)   │
│  Boss=1, niveau fixe, skills dédiés             │
├─────────────────────────────────────────────────┤
│              Unique (boss doré)                  │
│  +3 niveaux, HP ×4/3/2, 1-3 affixes            │
│  Entouré de minions de son type                 │
├─────────────────────────────────────────────────┤
│            Champion (bleu)                       │
│  +2 niveaux, HP ×3/2.5/2, variantes            │
│  Pack de 2-4 du même type                       │
├─────────────────────────────────────────────────┤
│           Monstre standard (blanc)               │
│  Stats de base, MinGrp-MaxGrp                   │
└─────────────────────────────────────────────────┘
```

### Affixes des Uniques

En génération, un Unique reçoit des affixes aléatoires :

| Difficulté | Nombre d'affixes |
|------------|-----------------|
| Normal | 1 |
| Nightmare | 2 |
| Hell | 3 |

### Pool d'affixes connu

| Affix | Effet |
|-------|-------|
| **Extra Strong** | +90/75/66% dégâts (N/NM/H) |
| **Extra Fast** | Vitesse de déplacement et d'attaque augmentée |
| **Cursed** | Applique Amplify Damage aux joueurs touchés |
| **Fire Enchanted** | Dégâts feu ajoutés, explosion à la mort |
| **Cold Enchanted** | Dégâts froid ajoutés, nova de froid à la mort |
| **Lightning Enchanted** | Charged Bolts émis quand frappé et à la mort |
| **Spectral Hit** | Dégâts aléatoires (feu/froid/foudre/poison) |
| **Stone Skin** | +80% résistance physique, -50% vitesse |
| **Multishot** | Tire plusieurs projectiles (ranged seulement) |
| **Aura Enchanted** | Aura aléatoire (Might, Holy Fire, Conviction, Fanaticism) |
| **Mana Burn** | Drain de mana massif |
| **Teleportation** | Se téléporte aléatoirement |
| **Magic Resistant** | +20/40/60% toutes résistances (N/NM/H) |
| **Conviction** | Aura réduisant les résistances des joueurs |

### Règles de combinaison

- Les affixes de résistance ne peuvent pas créer une **3e immunité** ni augmenter une immunité existante
- Certaines combinaisons sont interdites implicitement (pas de Fire Enchanted + Cold Enchanted — vérifié expérimentalement)
- L'aura est choisie parmi : Might, Holy Fire, Blessed Aim, Holy Freeze, Holy Shock, Conviction, Fanaticism

### Champions : variantes

| Variante | HP | Dégâts | Spécial |
|----------|-----|--------|---------|
| **Standard** | ×3/2.5/2 | +90/75/66% | — |
| **Berserker** | ×0.75 (du champion) | +270/225/198% | Glass cannon |
| **Fanatic** | ×3/2.5/2 | +90/75/66% | Similaire standard |
| **Ghostly** | ×3/2.5/2 | — | 80% résistance physique, +33-50% cold dmg |
| **Possessed** | ×6 (du champion) | Standard | Immune aux malédictions |

### Pseudo-algorithme de génération d'élite

```
fn generate_elite(zone, difficulty):
    roll = random(0..100)
    
    if roll < 8:   // ~8% unique
        monster = pick_base_monster(zone)
        elite = spawn_unique(monster, difficulty)
        num_affixes = match difficulty:
            Normal => 1, Nightmare => 2, Hell => 3
        elite.affixes = pick_affixes(num_affixes, monster.flags)
        elite.level += 3
        elite.hp *= match difficulty:
            Normal => 4.0, Nightmare => 3.0, Hell => 2.0
        // Spawn minions
        for _ in 0..random(elite.party_min..=elite.party_max):
            spawn_minion(elite, zone)
    
    elif roll < 28:  // ~20% champion
        monster = pick_base_monster(zone)
        count = random(2..=4)
        variant = pick_champion_variant()
        for _ in 0..count:
            champ = spawn_champion(monster, variant, difficulty)
            champ.level += 2
            champ.hp *= match difficulty:
                Normal => 3.0, Nightmare => 2.5, Hell => 2.0
    
    else:  // pack standard
        generate_standard_pack(zone)

fn pick_affixes(count, monster_flags):
    pool = AFFIX_POOL.clone()
    if monster_flags.is_melee:
        pool.remove(Multishot)
    selected = []
    for _ in 0..count:
        affix = pool.pick_random()
        if !creates_third_immunity(selected + [affix], monster_flags):
            selected.push(affix)
            pool.remove(affix)
    return selected
```

---

# 4. SYSTÈME DE PROJECTILES

## 4.1 Modèle de projectile

### Architecture : Entités physiques (pas de hitscan)

Diablo II n'utilise **pas** de hitscan. Tous les projectiles sont des **entités discrètes** qui se déplacent sur la carte frame par frame.

| Paramètre | Source | Détail |
|-----------|--------|--------|
| **Vel** | Missiles.txt | Vitesse initiale (pixels/frame à 25 FPS) |
| **MaxVel** | Missiles.txt | Vitesse maximale |
| **Accel** | Missiles.txt | Accélération par frame |
| **Range** | Missiles.txt | Durée de vie en frames |
| **LevRange** | Missiles.txt | Bonus de range par niveau |
| **VelLev** | Missiles.txt | Bonus de vitesse par niveau |
| **Size** | Missiles.txt | Rayon de collision en subtiles |

### Fonctions serveur/client séparées

Le système de missiles sépare **strictement** le client et le serveur :

| Fonction | Client | Serveur |
|----------|--------|---------|
| **DoFunc** (mouvement) | `pCltDoFunc` — graphisme, effets visuels | `pSrvDoFunc` — logique, collision |
| **HitFunc** (impact) | `pCltHitFunc` — particules, son | `pSrvHitFunc` — dégâts, effets |
| **DmgFunc** | — | `pSrvDmgFunc` — modifie les dégâts avant calcul |

Les fonctions client et serveur **doivent** être synchronisées pour éviter les désync.

### Move Function standard (type 1)

```
fn missile_move_basic(missile, dt):
    missile.vel += missile.accel
    missile.vel = min(missile.vel, missile.max_vel)
    missile.pos += missile.direction * missile.vel
    missile.range -= 1
    if missile.range <= 0:
        destroy(missile)
```

---

## 4.2 Collision

### 9 types de collision (CollideType)

| ID | Comportement |
|----|-------------|
| 0 | Pas de collision — traverse tout jusqu'à expiration |
| 1 | Collide joueurs uniquement (bug : ne collide pas les monstres malgré l'intention) |
| 2 | Collide monstres ennemis uniquement |
| 3 | Collide monstres + joueurs |
| 4 | Expire toujours (missile fantôme) |
| 5 | Copie du type 2 |
| 6 | Collide murs uniquement (impacts verticaux) |
| 7 | Collide missiles destructibles (déprécié) |
| 8 | Collide monstres/joueurs + bloqué par terrain |

### Détection : par tick (pas continue)

La collision est vérifiée **à chaque frame** (25 FPS), pas en continu. Un projectile très rapide peut donc **traverser** une cible fine (problème de tunneling).

### Collision murale

Un missile est détruit s'il finit sa frame sur une subtile ayant des **wall bits** activés. La vérification est sur la position finale, pas sur le trajet.

### Piercing

| Paramètre | Effet |
|-----------|-------|
| `Pierce=1` | Le missile continue après collision (traverse les ennemis) |
| `CollideKill=1` | Le missile est détruit après collision |
| `LastCollide=1` | Le missile s'arrête après collision |
| `NextHit/NextDelay` | Multi-hit avec délai entre chaque impact |

### Mémoire anti-spam

Les missiles se souviennent de leur **dernière cible** et ne la refrappent pas immédiatement. Cela permet à Fissure de multi-hit quand plusieurs monstres sont sur la même tile.

```rust
pub struct MissileState {
    pub last_hit_entity: Option<EntityId>,
    pub next_hit_delay: u32,
    pub pierce: bool,
    pub collide_type: CollideType,
}
```

---

## 4.3 Sorts spéciaux

### Projectiles guidés (homing)

Les missiles guidés utilisent une **Move Function spéciale** qui ajuste la direction vers la cible à chaque frame :

```
fn missile_move_guided(missile, dt):
    if missile.target.is_alive():
        desired_dir = (missile.target.pos - missile.pos).normalize()
        missile.direction = lerp(missile.direction, desired_dir, missile.turn_rate)
    missile.pos += missile.direction * missile.vel
    missile.range -= 1
```

Exemples : Guided Arrow, Bone Spirit.

### AoE à l'impact

Quand `pSrvHitFunc` est déclenché, certains missiles spawn un **sous-missile** AoE :

```
fn on_hit_aoe(missile, hit_pos):
    spawn_missile(
        type = missile.sub_missile,
        pos = hit_pos,
        collide_type = 3,  // touche tout
        range = 1,         // instantané
        size = aoe_radius
    )
```

Exemples : Fireball (explosion), Frozen Orb (nova de froid).

### Effets persistants

| Sort | Mécanisme |
|------|-----------|
| **Firewall** | Série de missiles statiques alignés sur la grille, chacun applique des dégâts par tick |
| **Blizzard** | Missiles tombant à des positions aléatoires dans une zone |
| **Poison Nova** | Ring de missiles partant dans toutes les directions |
| **Meteor** | Missile invisible descendant + AoE persistant au sol (Molten Boulder) |

### Fissure (cas d'étude technique intéressant)

Fissure émet des missiles le long de la grille. Quand plusieurs monstres chevauchent la même subtile, chaque missile peut frapper un monstre différent grâce à la mémoire anti-spam, causant des dégâts massifs.

---

## 4.4 Synchronisation réseau

| Aspect | Implémentation D2 |
|--------|-------------------|
| **Autorité** | Serveur autoritaire pour les dégâts et les collisions |
| **Client** | Affiche les missiles localement (interpolation graphique) |
| **Désync possible** | Si le client et le serveur divergent sur la position d'une cible, le missile client peut « rater » visuellement alors que le serveur a enregistré un hit (ou l'inverse) |
| **pCltDoFunc vs pSrvDoFunc** | Les deux doivent être cohérents sinon désync visuelle |

### Transposition MGE

```rust
// Plugin MGE : mge-plugin-projectile.v1

pub struct Projectile {
    pub velocity: f32,
    pub max_velocity: f32,
    pub acceleration: f32,
    pub range_remaining: u32,      // frames de vie restantes
    pub size: f32,                 // rayon collision (px)
    pub collide_type: CollideType,
    pub pierce: bool,
    pub last_hit: Option<EntityId>,
    pub next_hit_delay: u32,
    pub guided: bool,
    pub guide_target: Option<EntityId>,
    pub turn_rate: f32,            // pour missiles guidés
}

pub enum CollideType {
    None,           // traverse tout
    EnemyOnly,      // D2 type 2
    AllUnits,       // D2 type 3
    WallsOnly,      // D2 type 6
    UnitsAndWalls,  // D2 type 8
}
```

### Coût CPU estimé (projectiles)

| Opération | Coût par frame |
|-----------|---------------|
| Déplacement (par missile) | O(1) — simple addition vectorielle |
| Collision broadphase | O(n×m) naïf, O(n log n) avec spatial hash |
| Collision narrowphase | O(1) — test cercle/subtile |
| Total (100 missiles, 200 monstres) | ~0.5ms avec spatial hash |

**Recommandation MGE :** utiliser un spatial hash (grille de cellules) pour la broadphase. La grille de passabilité MGE peut servir de base.

---

# 5. FOLLOWERS / MERCENAIRES / INVOCATIONS

## 5.1 Architecture comportementale

### IA simplifiée

Les mercenaires et invocations utilisent des **IA similaires aux monstres** mais avec des priorités différentes :

| Entité | IA | Particularités |
|--------|-----|----------------|
| **Mercenaire** | Propre IA (melee ou ranged selon acte) | Suit le joueur, engage les ennemis proches |
| **Squelettes** | IA similaire mercenaire | Poursuivent ennemis visibles, restent près du joueur |
| **Golems** | IA spéciale par type | 1 seul actif à la fois |
| **Revives** | IA originale du monstre | Conservent leurs attaques spéciales, comportement moins prévisible |

### Priorité joueur

```
fn follower_ai_think(follower, owner, world):
    distance_to_owner = distance(follower.pos, owner.pos)
    
    // Priorité 1 : rester près du joueur
    if distance_to_owner > MAX_FOLLOW_DIST:
        pathfind_towards(follower, owner.pos)
        return
    
    // Priorité 2 : même cible que le joueur (si visible)
    if owner.target.is_some() and can_see(follower, owner.target):
        follower.target = owner.target
        engage(follower)
        return
    
    // Priorité 3 : ennemi le plus proche
    nearest = find_nearest_enemy(follower.pos, FOLLOWER_AGGRO_RANGE)
    if nearest.is_some():
        follower.target = nearest
        engage(follower)
        return
    
    // Priorité 4 : suivre le joueur
    if distance_to_owner > FOLLOW_THRESHOLD:
        pathfind_towards(follower, owner.pos)
    else:
        idle(follower)
```

---

## 5.2 Téléportation automatique

### Conditions de téléportation

| Condition | Comportement |
|-----------|-------------|
| **Distance excessive** | Si le follower est trop loin du joueur (hors écran + marge), téléportation invisible vers un point proche du joueur |
| **Changement de zone** | Le follower est instantanément repositionné dans la nouvelle zone |
| **Joueur utilise Teleport** | Le mercenaire et les invocations sont téléportés au point d'arrivée |
| **Stuck** | Pas de téléportation automatique de déblocage — le joueur doit se déplacer ou utiliser Teleport |

### Distance de téléport estimée

| Seuil | Valeur estimée |
|-------|---------------|
| **Distance de suivi normal** | ~10-15 subtiles |
| **Distance de téléportation** | >40-50 subtiles (environ 1.5 écran) |

### Problèmes connus

- **Arcane Sanctuary** : terrain complexe → followers se bloquent fréquemment
- **Pathfinding limité** : les mercenaires melee souffrent plus que les ranged dans les couloirs étroits
- Pas de commande "rappel" — les workarounds sont : fuir pour les faire suivre, ou utiliser Teleport

---

## 5.3 Sélection de cible

### Priorité de ciblage des followers

| Priorité | Critère |
|----------|---------|
| 1 | Cible actuellement attaquée par le joueur (si visible) |
| 2 | Ennemi le plus proche du follower |
| 3 | Ennemi le plus proche du joueur |
| 4 | Comportement propre (Revives conservent l'IA du monstre original) |

### Cas spéciaux

- **Revives** : utilisent l'IA originale du monstre → peuvent s'éloigner, utiliser des attaques spéciales, et sont moins obéissants
- **Golems** : IA agressive (Clay = lent mais tanky, Iron = thorns, Fire = charge)
- **Mercenaire Act 2 avec aura** : reste en formation → l'aura bénéficie au joueur et aux autres suivants

---

## 5.4 Pathfinding spécifique

### Différences avec le pathfinding monstre

| Aspect | Monstres | Followers |
|--------|----------|-----------|
| **Walk flags** | Player-walkable OU monster-walkable | Mercenary-walkable (flags distincts dans les subtiles) |
| **Collision pets** | — | Les pets se bloquent entre eux (SizeX/SizeY) |
| **Collision avec autres monstres** | Oui | Oui (pas d'immunité) |
| **Passage portes** | Configurable (`opendoors`) | Suivent le joueur (passent les portes ouvertes) |

### InTown et collision

D'après MonStats.txt : `InTown` contrôle si les pets ont une collision en ville :
- **Singleplayer** : collision activable/désactivable
- **Multiplayer** : collision toujours désactivée pour les pets en ville

### Transposition MGE

```rust
pub struct Follower {
    pub owner: EntityId,
    pub follow_distance: f32,        // distance idéale au joueur
    pub teleport_threshold: f32,     // distance de téléportation auto
    pub targeting_mode: TargetingMode,
    pub retain_original_ai: bool,    // pour les Revives
}

pub enum TargetingMode {
    FollowOwnerTarget,   // priorité cible du joueur
    NearestEnemy,        // cible la plus proche
    OriginalAi,          // IA du monstre original (Revives)
}
```

---

# 6. ANALYSE MOTEUR SOUS-JACENT

## 6.1 Architecture probable du moteur original

| Couche | Technologie |
|--------|-------------|
| **Langage** | C/C++ (confirmé par D2MOO) |
| **Rendering** | DirectDraw (2D sprites), résolution 640×480 (800×600 en LoD) |
| **Game loop** | Fixed timestep à 25 FPS |
| **Données** | Fichiers .txt (TSV) chargés en RAM → tables indexées par hcIdx |
| **Assets** | Formats propriétaires : DCC/DC6 (sprites), DT1 (tuiles), DS1 (presets carte), COF (animations) |
| **Réseau** | TCP/IP, architecture client-serveur pour Battle.net, peer-to-peer pour LAN |
| **Audio** | DirectSound |

## 6.2 Limites hardware 2000

| Contrainte | Impact |
|------------|--------|
| **CPU** | Pentium II/III ~500MHz → tick rate limité à 25 FPS |
| **RAM** | 64-256 MB → cartes générées procéduralement, pas pré-chargées |
| **GPU** | 2D uniquement (pas de GPU computing) → tout sur CPU |
| **Réseau** | Modems 56k → minimum de données réseau, pas de streaming |
| **Stockage** | CD-ROM → assets compressés, streaming minimal |

## 6.3 Pourquoi certains comportements existent (limitations techniques)

| Comportement | Raison technique |
|-------------|-----------------|
| **Breakpoints** | Tick fixe 25 FPS → animations en frames discrètes |
| **Pathfinding limité (35 cells)** | CPU trop faible pour A* longue distance sur 200+ monstres |
| **Monstres bloqués** | Pas de système de déblocage automatique (coût CPU) |
| **Pas de respawn** | RAM insuffisante pour tracker les respawn timers de centaines de monstres |
| **Téléportation followers** | Solution bon marché au pathfinding défaillant |
| **IA table-driven** | Pas assez de CPU pour du behavior tree complexe par entité |
| **Collision par subtile** | Plus rapide que du calcul flottant point par point |
| **CollideType #1 bug** | Code jamais corrigé car le jeu fonctionne malgré tout |

## 6.4 Estimation coût CPU par système

| Système | Coût estimé (par frame, 2000) | Coût estimé (2026, MGE) |
|---------|-------------------------------|-------------------------|
| **Pathfinding** (200 mobs) | ~8ms (A* limité) | ~0.5ms (A* optimisé + spatial hash) |
| **IA** (200 mobs) | ~3ms (FSM simple) | ~0.2ms |
| **Projectiles** (50 actifs) | ~1ms | ~0.05ms |
| **Collision** (globale) | ~4ms | ~0.3ms (broadphase spatial hash) |
| **Rendering** (sprites) | ~15ms | ~2ms (GPU batched) |
| **Réseau** (sync) | ~2ms | ~1ms |
| **Total** | ~33ms (budgeable sur 40ms) | ~4ms (largement sous les 33ms à 30 FPS) |

## 6.5 Vulnérabilités potentielles du moteur

| Vulnérabilité | Description | Exploitation connue |
|---------------|-------------|---------------------|
| **Desync client** | Le client prédit localement → position manipulable | Maphack, teleport hack |
| **Tables .txt modifiables** | Fichiers de configuration en clair → moddable | Modification de stats, résistances |
| **Collision par tick** | Projectiles rapides traversent les hitbox | Trivial à reproduire en jeu |
| **Memory editing** | Pas de protection mémoire côté client | Duplication d'items, modification de gold |
| **TCP/IP peer-to-peer** | Pas de serveur autoritaire en LAN | Triche libre en LAN |
| **Seed prédictible** | Le seed de carte est partagé | Cartes prévisibles avec le même seed |

---

# 7. RÉINTERPRÉTATION MODERNE — TRANSPOSITION MGE/ALLUMINA

## 7.1 Architecture recommandée pour Allumina

| Couche | Choix MGE | Justification |
|--------|-----------|---------------|
| **Game loop** | Fixed timestep 30 FPS logique + rendering découplé | Plus fluide que D2 (25 FPS), budget CPU confortable |
| **Positionnement** | Vec2 flottant avec grille logique optionnelle | Flexibilité + compatibilité pathfinding |
| **IA** | Table-driven FSM avec composants ECS | Même approche que D2 mais via composants MGE au lieu de fichiers .txt |
| **Projectiles** | Entités physiques (pas de hitscan) + spatial hash | Reproduit le feel D2 avec meilleure performance |
| **Collision** | Broadphase spatial hash + narrowphase cercle/AABB | Standard moderne, O(n log n) |
| **Réseau** | Lobby hôte autoritaire + client prediction + réconciliation | MWS comme transport, LOI-1 respectée (solo jouable) |
| **Données** | Composants ECS + tables de configuration (RON/JSON) | Équivalent des .txt D2 mais sérialisable et typé |

## 7.2 Mapping D2 → Plugins MGE

| Système D2 | Plugin MGE | Composants |
|------------|------------|------------|
| MonStats.txt | `mge-plugin-monster-stats.v1` | `MonsterDef`, `MonsterInstance` |
| MonStats2.txt | `mge-plugin-monster-collision.v1` | `MonsterHitbox`, `MonsterSize` |
| Missiles.txt | `mge-plugin-projectile.v1` | `Projectile`, `ProjectileDef` |
| Levels.txt | `mge-plugin-zone-spawn.v1` | `SpawnZone`, `SpawnTable` |
| Skills.txt | `mge-plugin-skills.v1` | `SkillDef`, `SkillInstance` |
| AI (AiThink.cpp) | `mge-plugin-ai-monster.v1` | `MonsterAiState`, `AiConfig` |
| SuperUniques.txt | `mge-plugin-elite-gen.v1` | `EliteDef`, `AffixPool` |
| TreasureClass.txt | `mge-plugin-loot.v1` | `LootTable`, `TreasureClass` |

## 7.3 Difficultés principales

| Difficulté | Détail |
|------------|--------|
| **Reproduire le "feel" D2** | Le tick à 25 FPS crée une sensation spécifique ; un tick plus rapide sera plus fluide mais différent |
| **Breakpoints optionnels** | Si on veut les breakpoints, il faut un système de quantification des vitesses d'animation |
| **IA table-driven fidèle** | Nécessite un système de configuration robuste avec 8+ paramètres par IA |
| **Collision subtile vs flottante** | Le passage de coordonnées entières à flottantes change les edge cases de collision |
| **Followers décents** | D2 avait un pathfinding médiocre pour les followers — il faut faire mieux sans perdre le feel |
| **Multijoueur souverain** | D2 repose sur Battle.net ; Allumina doit fonctionner via MWS (Lobby P2P) sans serveur central |

## 7.4 Pièges à éviter

| Piège | Explication |
|-------|-------------|
| **Copier les bugs de D2** | Les CollideType bugs, le NHAM, les followers bloqués → ne pas reproduire |
| **Tick rate trop élevé** | 60 FPS logique serait overkill pour un ARPG isométrique et coûteux en réseau |
| **Pathfinding global** | A* sur toute la carte est inutile et coûteux — garder la portée limitée de D2 |
| **IA trop complexe** | Behavior trees par mob = overkill. La FSM table-driven de D2 est suffisante et performante |
| **Oublier le leashing** | Sans leashing, les mobs suivent indéfiniment → train de monstres exploit |
| **Collision trop réaliste** | D2 autorise le chevauchement des "ailes" → ne pas bloquer trop strictement ou les combats deviennent impossibles |
| **Négliger le spatial hash** | Sans broadphase, la collision de 200+ mobs + 50 projectiles est O(n²) |

## 7.5 Comparaison implicite avec PoE et D3

| Aspect | D2 (2000) | Path of Exile | Diablo 3 | Allumina (cible) |
|--------|-----------|---------------|----------|-------------------|
| **Tick rate** | 25 FPS | 30 FPS serveur | 60 FPS | 30 FPS |
| **Pathfinding** | A* limité | NavMesh + A* | NavMesh | A* sur grille logique |
| **IA** | FSM table-driven | FSM + scripts | Behavior tree | FSM table-driven (MGE) |
| **Projectiles** | Entités physiques | Entités physiques | Entités + hitscan | Entités physiques |
| **Collision** | Subtile grid | Continuous | Capsule + spatial hash | Cercle/AABB + spatial hash |
| **Réseau** | Client-serveur/P2P | Serveur autoritaire | Serveur autoritaire | Lobby autoritaire (MWS) |
| **Élites** | Affixes simples | Affixes + mods carte | Affixes + Nephalem | Affixes (pool configurable) |
| **Followers** | IA basique, teleport | IA basique | IA basique | IA améliorée (chaîne locomotion MGE) |
| **Troupes** | Mercenaire + summons | Spectres/zombies | 1 follower | Multi-échelles (Charisme cap) |

---

# 8. TABLES RÉCAPITULATIVES

## 8.1 Constantes fondamentales D2

| Constante | Valeur |
|-----------|--------|
| Tick rate | 25 FPS |
| Frame duration | 40 ms |
| Subtile size (affichage) | 32×16 px |
| Tile size (affichage) | 160×80 px |
| Subtiles par tile | 5×5 |
| 1 yard | 1.5 subtiles = 48×24 px |
| Distance d'activation IA (défaut) | 35 cells ≈ 1 écran |
| Portée pathfinding | ~35 subtiles |
| Player SizeX/SizeY | 2 subtiles |
| Knockback range | 7×7 subtiles (centré) |
| Regen formule | (REGEN × HP) / 4096 par frame |
| Block cap | 75% |
| Resistance immunité | ≥100% |
| Break immunité | 5 pts résistance réduite = 1% brisé |

## 8.2 Multiplicateurs d'élite par difficulté

| Type | HP (N/NM/H) | Niveau bonus | XP bonus |
|------|-------------|-------------|----------|
| Minion | ×2 / ×1.75 / ×1.5 | +3 | ×5 |
| Champion | ×3 / ×2.5 / ×2 | +2 | ×3 |
| Berserker | ×0.75 champion | +2 | ×5 |
| Possessed | ×6 champion | +2 | ×3 |
| Unique | ×4 / ×3 / ×2 | +3 | ×5 |

## 8.3 Structure de données recommandée (MGE)

```rust
// Table de spawn — équivalent Levels.txt + MonStats.txt
pub struct ZoneSpawnConfig {
    pub zone_id: u32,
    pub area_level: [u32; 3],          // Normal, Nightmare, Hell
    pub eligible_monsters: Vec<MonsterSpawnEntry>,
    pub elite_chance: f32,              // % chance par pack
    pub champion_ratio: f32,            // ratio champion vs unique
    pub max_population: u32,
    pub wandering_chance: f32,
    pub wandering_max: u32,
}

pub struct MonsterSpawnEntry {
    pub monster_def_id: u32,
    pub rarity: u32,
    pub min_group: u32,
    pub max_group: u32,
    pub party_min: u32,
    pub party_max: u32,
    pub sparse_chance: f32,
    pub is_ranged: bool,
}

// Définition monstre — équivalent MonStats.txt complet
pub struct MonsterDef {
    pub id: u32,
    pub base_id: u32,
    pub ai_type: AiTypeId,
    pub ai_delay: [u32; 3],            // par difficulté
    pub ai_dist: [f32; 3],
    pub ai_params: [[f32; 8]; 3],
    pub threat: f32,
    pub velocity: f32,
    pub run_velocity: f32,
    pub skills: [Option<SkillRef>; 8],
    pub resistances: ResistanceSet,
    pub hp_range: [(u32, u32); 3],      // (min, max) par difficulté
    pub damage_a1: [(u32, u32); 3],
    pub damage_a2: [(u32, u32); 3],
    pub flags: MonsterFlags,
}

pub struct MonsterFlags {
    pub is_boss: bool,
    pub is_prime_evil: bool,            // +300% dmg vs pets
    pub is_undead_low: bool,
    pub is_undead_high: bool,
    pub is_demon: bool,
    pub is_flying: bool,
    pub can_open_doors: bool,
    pub pet_ignore: bool,
    pub is_spawner: bool,
    pub is_ranged: bool,
    pub is_melee: bool,
    pub no_aura: bool,
}
```

---

# 9. GÉNÉRATION PROCÉDURALE DE CARTES

## 9.1 Système de seed

### Seed global

Diablo II utilise un **seed 32 bits** (entier non signé) comme source de toute la génération procédurale. Le RNG (Random Number Generator) est un générateur séquentiel déterministe : chaque nombre aléatoire est calculé à partir du précédent, de sorte qu'un même seed produit toujours le même flux de valeurs "aléatoires".

| Paramètre | Détail |
|-----------|--------|
| **Taille** | 32 bits (uint32), Little Endian |
| **Stockage** | Offset `0xAB` (171 octets) dans le fichier `.d2s` (save du personnage) |
| **Génération** | Automatique à la création d'une partie (basé sur l'horloge système) |
| **Override** | Paramètre CLI `-seed <valeur>` pour forcer un seed spécifique |
| **Plage** | 0 à 4 294 967 295 |

### Ce que le seed contrôle

| Élément | Contrôlé par le seed |
|---------|---------------------|
| **Layout des tuiles** | Oui — positions des DS1 dans le monde, choix des variantes |
| **Sélection des presets** | Oui — quel DS1 parmi File1-File6 est choisi |
| **Positions des sorties** | Oui — emplacements des warps entre zones |
| **Densité de monstres** | Partiellement — le seed affecte les positions de spawn, des seeds très petits peuvent produire une densité anormalement élevée |
| **Types de monstres** | Indirectement — la sélection aléatoire parmi M1-M25 dépend du flux RNG |
| **Drops d'items** | Non — les drops sont calculés indépendamment au moment du kill |
| **Waypoints** | Oui — position dans la zone (sauf certains waypoints fixes) |
| **Objets de quête** | Oui — position dans le preset DS1 désigné |

### Multijoueur et partage de seed

| Aspect | Comportement |
|--------|-------------|
| **Création de partie** | Le créateur de la partie génère le seed ; tous les joueurs utilisent le même seed |
| **Persistance** | Le seed est écrit dans le `.d2s` de chaque joueur qui rejoint la partie |
| **Partie permanente** | Le seed reste identique tant que la partie existe sur le serveur (realm) |
| **Rejointure** | Un joueur qui rejoint retrouve la même carte (même seed) |
| **Nouvelle partie** | Un nouveau seed est généré à chaque création de partie |

### Structure de la chaîne DRLG

```
Game Seed (32-bit)
  └─→ Act Seed (dérivé)
       └─→ Level Seed (dérivé par level ID)
            └─→ Room Seeds (dérivés séquentiellement)
                 └─→ Sub-element placement (monstres, objets, etc.)
```

Chaque niveau dans un acte reçoit un seed dérivé du seed de l'acte. Cela garantit que la modification d'un niveau n'affecte pas la génération des autres niveaux du même acte.

### Structures DRLG internes (reverse-engineered)

```c
// D2DrlgActStrc — représente un acte complet
struct D2DrlgActStrc {
    D2RoomStrc* pRoom;           // liste chaînée de rooms
    uint32_t    dwSeed;          // seed de l'acte
    D2DrlgDataStrc* pDrlgData;   // données de génération
};

// D2DrlgDataStrc — données de génération aléatoire
struct D2DrlgDataStrc {
    uint32_t dwSeed;             // seed courant (état RNG)
    uint32_t dwRoomCount;        // nombre de rooms générées
};

// D2DrlgLevelStrc — un niveau individuel
struct D2DrlgLevelStrc {
    uint32_t dwLevelType;        // type de niveau
    uint32_t dwSeed;             // seed du niveau
    uint32_t dwSizeX, dwSizeY;   // dimensions
};
```

---

## 9.2 Types de génération (DrlgType)

Diablo II utilise **trois algorithmes de génération distincts**, sélectionnés par la colonne `DrlgType` dans `Levels.txt` :

| DrlgType | Nom | Usage | Exemples |
|----------|-----|-------|----------|
| **1** | Random Maze | Donjons composés de rooms assemblées | Caves, Cryptes, Arcane Sanctuary, Maggot Lair |
| **2** | Preset | Carte fixe (un seul DS1) | Catacombes Niv. 4, Pandemonium Fortress, Villes |
| **3** | Random Wilderness | Zones extérieures de taille fixe | Blood Moor, Stony Field, déserts Acte 2 |

### DrlgType 1 — Random Maze (Donjons)

Le système de labyrinthe assemble des **rooms individuelles** (chacune étant un fichier DS1) en un réseau connecté.

**Fichier de contrôle : `LvlMaze.txt`**

| Colonne | Rôle |
|---------|------|
| `Rooms` | Nombre **minimum** de DS1 composant le labyrinthe |
| `SizeX` / `SizeY` | Coordonnées du coin inférieur-droit de chaque room (en tiles, base 0) |
| `Merge` | Contrôle la fusion de certaines rooms |

**Algorithme de génération des labyrinthes :**

```
fn generate_maze(level, seed):
    rng = init_rng(seed)
    grid = empty_grid(level.max_size)
    room_count = 0
    
    // 1. Placer la room d'entrée
    start_pos = get_start_position(level)  // centre ou bord selon LevelType
    grid[start_pos] = ENTRY_ROOM
    room_count += 1
    
    // 2. Expansion par croissance
    while room_count < level.min_rooms:
        // Sélectionner une room existante avec un côté libre
        source = pick_room_with_free_edge(grid, rng)
        direction = pick_free_direction(source, rng)  // N, S, E, W
        new_pos = source.pos + direction
        
        if is_valid_position(new_pos, grid):
            grid[new_pos] = ROOM
            add_connection(source, new_pos, direction)
            room_count += 1
    
    // 3. Placer la room spéciale (boss, quête) à l'extrémité
    endpoint = find_deepest_leaf(grid)
    grid[endpoint] = SPECIAL_ROOM
    
    // 4. Résoudre les types de DS1 par ouvertures
    for each cell in grid:
        openings = compute_openings(cell)  // bitmask NSEW
        cell.ds1_type = openings           // index 1-15 dans LvlPrest.txt
        cell.ds1_file = pick_variant(level.presets[openings], rng)
```

**Convention de nommage des DS1 de labyrinthe :**

Les DS1 sont nommés selon leurs ouvertures, encodées sur 4 bits (N=8, S=4, E=2, W=1) :

```
Bitmask  Valeur  Nom DS1          Ouvertures
........
00000001 =  1    caveW.ds1        Ouest
00000010 =  2    caveE.ds1        Est
00000011 =  3    caveEW.ds1       Est + Ouest
00000100 =  4    caveS.ds1        Sud
00000101 =  5    caveSW.ds1       Sud + Ouest
00000110 =  6    caveSE.ds1       Sud + Est
00000111 =  7    caveSEW.ds1      Sud + Est + Ouest
00001000 =  8    caveN.ds1        Nord
00001001 =  9    caveNW.ds1       Nord + Ouest
00001010 = 10    caveNE.ds1       Nord + Est
00001011 = 11    caveNEW.ds1      Nord + Est + Ouest
00001100 = 12    caveNS.ds1       Nord + Sud
00001101 = 13    caveNSW.ds1      Nord + Sud + Ouest
00001110 = 14    caveNSE.ds1      Nord + Sud + Est
00001111 = 15    caveNSEW.ds1     Toutes directions
```

Chaque type de DS1 dans LvlPrest.txt peut avoir jusqu'à **6 variantes** (colonnes File1-File6), sélectionnées aléatoirement pour apporter de la variété visuelle.

**Contraintes de connexion entre rooms :**

- Les sorties de deux rooms adjacentes **doivent** être aux mêmes coordonnées relatives pour assurer la continuité du passage
- La largeur de passage est de 1+ tuile (minimum 4-5 subtiles pour le joueur)
- Le labyrinthe doit tenir dans les limites `SizeX × SizeY` de `Levels.txt`
- Certains LevelTypes imposent le placement de l'entrée (centre pour Arcane Sanctuary, bord pour la plupart des caves)

**Exemple concret — Den of Evil avec Rooms=6 :**

```
caveSpre2.ds1     ← Room d'entrée (depuis la surface)
  (DEF 85)
     |
caveNE2.ds1 ─── caveSW.ds1
  (DEF 62)       (DEF 57)
                    |
caveSE2.ds1 ─── caveNW2.ds1
  (DEF 58)       (DEF 61)
     |
caveNE.ds1  ─── caveWspec.ds1    ← Corpsefire (room spéciale)
  (DEF 62)       (DEF 95)
```

7 DS1 (25×25 tuiles chacun) pour un Rooms=6, car le DRLG peut ajouter des rooms supplémentaires pour satisfaire les contraintes de connexion.

### DrlgType 2 — Preset (Carte fixe)

Les niveaux preset sont des **DS1 uniques non randomisés** (le layout est toujours identique). Cependant, `LvlPrest.txt` offre une randomisation limitée :

- **File1 à File6** : jusqu'à 6 variantes DS1 d'un même preset, sélectionnées aléatoirement par le seed
- **Populate** : contrôle si les monstres sont placés aléatoirement dans la zone
- La taille du DS1 correspond exactement à `SizeX × SizeY` de `Levels.txt`

Exemples : Catacombes Niveau 4 (Andariel), Pandemonium Fortress, les villes.

### DrlgType 3 — Random Wilderness (Zones extérieures)

La génération des zones extérieures (Acte 1 wilderness, Acte 2 déserts) est la plus complexe, composée de **4 étapes séquentielles** :

**Étape 1 — Bordures et sorties**

```
fn generate_wilderness_borders(level, seed):
    rng = init_rng(seed)
    
    // Calculer la forme et les bordures
    borders = select_border_ds1(level)  // BordX.ds1, StnClfX.ds1, XRiverX.ds1
    place_borders(level.grid, borders)
    
    // Placer les sorties vers les zones adjacentes
    for vis_index in level.vis_links:
        exit_pos = compute_exit_position(vis_index, rng)
        place_exit_warp(level.grid, exit_pos, vis_index)
```

Les bordures sont composées de DS1 simples :
- `BordX.ds1` : bordures normales (arbres + mur de pierre)
- `StnClfX.ds1` : falaises (Stony Field, Dark Woods)
- `XRiverX.ds1` : rivières en bordure

**Étape 2 — Chemins, waypoints et warps internes**

- Les **waypoints** sont placés aléatoirement dans la zone (sauf exceptions comme Cold Plains)
- Les **chemins** sont générés dynamiquement (pas de DS1 pour les éléments de chemin) — l'algorithme relie les entrées, caves, ponts entre eux
- Les **warps internes** (entrées de caves, Tour de la Comtesse) sont positionnés

**Étape 3 — Placement des DS1 thématiques**

```
fn place_themed_presets(level, rng):
    // DS1 thématiques placés aléatoirement dans les espaces vides
    for preset in level.themed_presets:
        pos = find_empty_area(level.grid, preset.size, rng)
        place_ds1(level.grid, preset, pos)
```

Exemples de DS1 thématiques :
- `circle.ds1` : cercle de petites pierres
- `arrow.ds1` : flèche en pierres
- `pond1.ds1` : étang
- Camps de monstres (Bishibosh dans Cold Plains)
- Objets de quête (Cairn Stones dans Stony Field, Inifuss Tree dans Dark Woods)

Le nombre de presets thématiques est **fixe par zone** (probablement hardcodé ou dans une table DLL). Chaque zone contient toujours : quelques blocs bordés, 1-2 maisons, parfois une maison en feu, 5 shrines.

**Étape 4 — Remplissage par objets aléatoires (LvlSub.txt)**

```
fn fill_with_random_objects(level, rng):
    // Le jeu extrait des éléments INDIVIDUELS depuis les DS1 "filler"
    for filler_ds1 in level.sub_theme_fillers:
        elements = extract_individual_elements(filler_ds1)
        for element in elements:
            if rng.roll() < element.probability:
                pos = find_empty_position(level.grid, rng)
                place_element(level.grid, element, pos)
    
    // Combler les espaces restants avec le sol standard (herbe, sable...)
    fill_remaining_with_base_floor(level.grid)
```

Les DS1 "filler" sont des fichiers étranges qui ressemblent à des planches de conception :
- `stone.ds1` : collection de formations rocheuses individuelles
- `trees.ds1` : collection d'arbres individuels
- `swamp.ds1` / `swamp2.ds1` : éléments marécageux
- `pud.ds1` : flaques et mares

Le jeu **extrait des éléments individuels** de ces DS1 et les place séparément sur la carte. La colonne `SubTheme` de `Levels.txt` contrôle quels fillers sont utilisés (le Dark Wood utilise plus d'arbres, le Blood Moor pas de marécages).

Ces DS1 sont référencés dans `LvlSub.txt` (pas `LvlPrest.txt`), qui fonctionne de manière similaire à `TCex.txt` avec des colonnes `Prob` (probabilité), `Trials` (picks) et un maximum.

---

## 9.3 Système de tuiles — Format DT1

### Vue d'ensemble

Les fichiers DT1 (Diablo Tile 1) contiennent toutes les **tuiles graphiques** utilisées pour les sols, murs, ombres et toits des cartes. Il y a 256 fichiers DT1 dans le jeu (~157 MB total), organisés par acte et thème dans `Data\Global\Tiles\Act{N}\`.

Un même DT1 est partagé par **plusieurs cartes** (les murs de pierre du Rogue Encampment apparaissent aussi dans Cold Plains, Stony Field, Tristram).

### Structure du fichier DT1

```
┌─────────────────────────────────┐
│ File Header (276 octets)        │
├─────────────────────────────────┤
│ Tile Header #0 (96 octets)     │
│ Tile Header #1 (96 octets)     │
│ ...                             │
│ Tile Header #N (96 octets)     │
├─────────────────────────────────┤
│ Tile Data #0                    │
│   ├─ Block Headers (20 oct/blk)│
│   └─ Block Data (pixels)       │
│ Tile Data #1                    │
│ ...                             │
└─────────────────────────────────┘
```

**File Header (276 octets) :**

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 4 | Version 1 (= 7) |
| 0x04 | 4 | Version 2 (= 6) |
| 0x08 | 260 | Réservé (tout à zéro) |
| 0x10C | 4 | Nombre de tuiles |
| 0x110 | 4 | Pointeur vers les Tile Headers (= 0x114) |

**Tile Header (96 octets) :**

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 4 | Direction (orientation générale, 1-5) |
| 0x04 | 2 | Hauteur de toit (pixels au-dessus du sol) |
| 0x06 | 1 | Index sonore (bois, pierre, boue, etc.) |
| 0x07 | 1 | Flag animé (bit 0 = tuile animée) |
| 0x08 | 4 | Hauteur (pixels, toujours négatif, puissance de 32) |
| 0x0C | 4 | Largeur (pixels, max 160, puissance de 32) |
| 0x10 | 4 | Zéros |
| 0x14 | 4 | **Orientation** (type de tuile, 0-19) |
| 0x18 | 4 | **Main Index** (0-63) |
| 0x1C | 4 | **Sub Index** (0-63) |
| 0x20 | 4 | Rarity / Frame index |
| 0x24 | 4 | Unknown 1-4 (même valeur pour toutes les tuiles d'un DT1) |
| 0x28 | 25 | **Sub-tile flags** (passabilité, 5×5 = 25 subtiles) |
| 0x41 | 7 | Zéros |
| 0x48 | 4 | Pointeur vers les Block Headers |
| 0x4C | 4 | Taille totale des Block Headers + Block Data |
| 0x50 | 4 | Nombre de blocs |
| 0x54 | 12 | Zéros |

### Identification des tuiles : 3 index

Chaque tuile est identifiée de manière unique par la combinaison **(Orientation, Main Index, Sub Index)** :

- **Orientation** (0-19) : détermine le type de tuile
- **Main Index** (0-63) : identifiant principal dans le tileset
- **Sub Index** (0-63) : sous-variante

Soit jusqu'à 64 × 64 = **4096 tuiles distinctes** par orientation.

### Types de tuiles (Orientation)

| Orientation | Type | Rendu |
|-------------|------|-------|
| 0 | **Sol** (statique ou animé) | Dessiné en premier |
| 1 | Mur gauche | |
| 2 | Mur droit (supérieur) | |
| 3 | Partie droite du coin nord | |
| 4 | Partie gauche du coin nord | |
| 5 | Coin supérieur-droit | |
| 6 | Coin inférieur-gauche | |
| 7 | Coin inférieur-droit | |
| 8 | Mur gauche avec porte | |
| 9 | Mur droit avec porte | |
| 10-11 | **Tuiles spéciales** (warps, TP, entrées) | |
| 12 | Piliers, colonnes, objets autonomes | |
| 13 | **Ombres** | |
| 14 | **Arbres** (objets avec ombre précédente) | |
| 15 | **Toits** | Au-dessus du sol (roof_y dans le .ini) |
| 16-19 | **Murs bas** (équivalents de 1, 2, 3/4, 7) | Dessinés sous les sols |

### Direction et éclairage

La **Direction** (1-9) contrôle comment la lumière affecte la tuile. Chaque Direction doit être associée aux bonnes Orientations :

| Direction | Orientations compatibles |
|-----------|------------------------|
| 1 | 1, 5, 8 |
| 2 | 2, 6, 9 |
| 3 | 0, 3, 4, 12, 14 |
| 4 | 7 |
| 5 | 15 (toits) |
| 6-9 | 16-19 (murs bas) |

### Sub-tile flags (passabilité)

Chaque tuile possède **25 flags** (grille 5×5) définissant la passabilité subtile par subtile. L'ordre est gauche→droite, bas→haut :

| Bit | Effet |
|-----|-------|
| 0 | **Block walk** (bloque le déplacement piéton) |
| 1 | **Block light + LOS** (bloque lumière et ligne de vue) |
| 2 | **Block jump** (bloque saut et téléportation) |
| 3 | **Block player walk only** (pas le mercenaire — usage étrange) |
| 4 | Inconnu |
| 5 | **Block light only** (pas la LOS) |
| 6-7 | Inconnus |

### Rarity / Random Sets

Quand plusieurs tuiles dans un DT1 partagent le même triplet (Orientation, Main Index, Sub Index), elles forment un **random set**. Le jeu choisit aléatoirement parmi elles selon leur **Rarity** :

```
Exemple : 4 tuiles de sol "terre brûlée" avec même identifiant
  Tuile A : Rarity = 1  → 1/37 chance (beaucoup de sang)
  Tuile B : Rarity = 2  → 2/37 chance (un peu de sang)
  Tuile C : Rarity = 10 → 10/37 chance (pas de sang)
  Tuile D : Rarity = 0  → jamais affichée (si total > 0)
```

Si toutes les Rarity sont à 0, seule la **dernière tuile** du premier DT1 est utilisée.

### Encodage graphique des blocs

Deux formats de compression :

**Format 1 — Sol isométrique (RAW, 256 octets fixe) :**

```c
// Dessin d'un bloc isométrique 3D (losange de 32×15 pixels)
int xjump[15] = {14, 12, 10, 8, 6, 4, 2, 0, 2, 4, 6, 8, 10, 12, 14};
int nbpix[15] = {4,  8,  12, 16, 20, 24, 28, 32, 28, 24, 20, 16, 12, 8,  4};
// Chaque ligne : sauter xjump pixels, dessiner nbpix pixels RAW
```

**Format 0 — Murs (RLE, taille variable) :**

```c
// Blocs 32×32 en Run Length Encoding
// Lecture par paires : (skip, count)
// skip = pixels transparents, count = pixels opaques qui suivent
// (0, 0) = saut à la ligne suivante
```

### Block Header (20 octets)

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 2 | Position X dans le bitmap |
| 0x02 | 2 | Position Y dans le bitmap |
| 0x04 | 2 | Zéros |
| 0x06 | 1 | Grid X (0-4, position dans la grille subtile) |
| 0x07 | 1 | Grid Y (0-4) |
| 0x08 | 2 | Format (1 = isométrique RAW, 0 = RLE 32×32) |
| 0x0A | 4 | Longueur des données encodées (octets) |
| 0x0E | 2 | Zéros |
| 0x10 | 4 | Offset fichier des données encodées |

---

## 9.4 Système de presets — Format DS1

### Vue d'ensemble

Les fichiers DS1 (Diablo Scene 1) sont des **configurations prédéfinies de cartes isométriques multi-couches**. Chaque DS1 définit l'arrangement des tuiles DT1, les objets, les PNJ et leurs chemins. C'est le format de base pour tous les niveaux, qu'ils soient fixes ou assemblés en labyrinthe.

### Structure multi-couches

| Composant | Nombre max | Ordre de rendu |
|-----------|-----------|----------------|
| **Couches de sol** | 4 | Rendues en premier |
| **Couches de mur** | 4 | Rendues après le sol |
| **PNJ / Monstres** | Variable | Placés après le terrain |
| **Objets** (feu, drapeaux, etc.) | Variable | Placés selon le type |
| **Chemins de PNJ** | Variable | Paths prédéfinis dans le DS1 |

### Référencement des tuiles

Chaque cellule du DS1 référence des tuiles DT1 via leurs 3 index :
- **Main Index** (6 bits dans le DS1, soit 0-63)
- **Sub Index** (6 bits, soit 0-63)
- **Orientation** (détermine le type : sol, mur, ombre, etc.)

### Tuiles spéciales dans les DS1

Les tuiles spéciales (Orientation 10-11) contrôlent des fonctionnalités gameplay :

| Index | Fonction |
|-------|----------|
| #00-46 | **Vis** (warps entre zones — passage d'une zone à une autre) |
| #47-74 | **Area** (suppression de murs et toits pour révéler l'intérieur) |
| #75 | Town Entry |
| #76 | Map Entry |
| #77 | Town Entry 2 |
| #78 | Corpse Location |
| #79 | Teleport Location |
| #80 | Unknown |
| #82-83 | Vis supplémentaires |

### PopPads et suppression de toits

Les tuiles spéciales #47-74 contrôlent la **suppression dynamique de murs/toits** quand le joueur approche d'un bâtiment. Deux tuiles spéciales identiques sont placées pour former un rectangle définissant la zone de suppression.

```
LvlPrest.txt colonnes associées :
  Pops    = nombre de zones de suppression dans le DS1
  PopPad  = offset en subtiles du trigger par rapport à la zone définie
            (0 = exact, +N = zone trigger agrandie, -N = rétrécie)
```

Le mécanisme est piloté par le **Sub Index** de la tuile spéciale : il correspond au **Main Index** des tuiles qui seront supprimées. Un groupe de tuiles spéciales (même Group) supprime ensemble ; des tuiles de groupes différents supprimées indépendamment, permettant jusqu'à **4 zones de suppression séparées simultanément**.

| Groupe | Main Index des spéciales | Peuvent supprimer indépendamment |
|--------|-------------------------|--------------------------------|
| 1 | 8, 9, 10 | Oui (entre eux, même cibles) |
| 2 | 12, 13 | Oui |
| 3 | 16 | Oui |
| 4 | 20 | Oui |

### Transparence des murs

La transparence (fading) des murs se produit quand les murs forment une **chaîne fermée** (box) :

- Tous les murs doivent être connectés sans interruption
- La colonne `Logicals` dans `LvlPrest.txt` active/désactive la transparence
- Le paramètre `Tile Sound = 0` dans le fichier `.ini` du tileset est requis
- Les coins nord doivent être séparés en deux frames (Orientation 3 + Orientation 4)

---

## 9.5 Fichiers de configuration — LvlPrest.txt

### Colonnes principales

| Colonne | Type | Description |
|---------|------|-------------|
| `Def` | int | Identifiant unique du preset |
| `LevelId` | int | Référence vers `Levels.txt` |
| `Populate` | bool | Placement aléatoire de monstres dans le preset |
| `Logicals` | bool | Active la transparence des murs |
| `Outdoors` | bool | Zone extérieure |
| `Animate` | bool | Active les animations de tuiles |
| `KillEdge` | bool | Supprime les tuiles en bordure |
| `FillBlanks` | bool | Remplit les espaces vides avec le sol par défaut |
| `Expansion` | bool | Extension Lord of Destruction |
| `SizeX` / `SizeY` | int | Dimensions en cells (coordonnées, base 0) |
| `AutoMap` | bool | Génère l'automap pour ce preset |
| `Scan` | bool | Scan des tuiles VIS |
| `Pops` | int | Nombre de PopPads (zones de suppression de toits) |
| `PopPad` | int | Offset du trigger PopPad (en subtiles) |
| `Files` | int | Nombre de variantes DS1 utilisées (1-6) |
| `File1`-`File6` | string | Chemins vers les fichiers DS1 (jusqu'à 6 variantes) |
| `Dt1Mask` | uint32 | Bitmask des fichiers DT1 à charger |

### Dt1Mask — Calcul

Le `Dt1Mask` est un **bitmask 32 bits** qui détermine quels fichiers DT1 de `LvlTypes.txt` sont chargés pour ce preset.

```
Formule : Dt1Mask = 2^(nombre de DT1 à charger) - 1

Exemple :
  LvlTypes.txt pour LevelType 5 liste 16 fichiers DT1
  Dt1Mask = 2^16 - 1 = 65535 (0xFFFF)
  → Charge les 16 DT1

Pour charger seulement les DT1 #0, #2, #5 :
  Dt1Mask = (1 << 0) | (1 << 2) | (1 << 5) = 0b00100101 = 37
```

Chaque bit correspond à une colonne File dans `LvlTypes.txt`. Bit 0 = File1, bit 1 = File2, etc.

---

## 9.6 Fichiers de configuration — LvlTypes.txt

`LvlTypes.txt` associe un **identifiant de type de niveau** à une liste de fichiers DT1. Chaque ligne définit un ensemble visuel complet (tileset).

| Colonne | Description |
|---------|-------------|
| `Id` | Identifiant du LevelType (référencé par Levels.txt) |
| `File1`-`File32` | Chemins vers les fichiers DT1 composant le tileset |
| `Act` | Acte auquel le type appartient |

Le `LevelType` de `Levels.txt` pointe vers une ligne de `LvlTypes.txt`, et le `Dt1Mask` de `LvlPrest.txt` sélectionne quels DT1 de cette ligne sont effectivement chargés.

```
Pipeline :
  Levels.txt[LevelType] → LvlTypes.txt[Id] → Liste de DT1
  LvlPrest.txt[Dt1Mask] → Filtre les DT1 à charger
  DS1 → Référence les tuiles par (Orientation, MainIndex, SubIndex) dans les DT1 chargés
```

---

## 9.7 Composition des niveaux — Levels.txt

### Colonnes complètes

**Identification :**

| Colonne | Description |
|---------|-------------|
| `Name` | Nom interne du niveau |
| `Id` | Identifiant unique (référencé partout) |
| `Pal` | Palette de couleurs (une par acte) |
| `Act` | Acte d'appartenance (0-4) |

**Dimensions et positionnement :**

| Colonne | Description |
|---------|-------------|
| `SizeX` / `SizeY` | Dimensions horizontale/verticale en subtiles |
| `OffsetX` / `OffsetY` | Position dans le worldspace (-1/-1 = position hardcodée) |
| `Depend` | ID du niveau dont le warp dépend pour l'alignement |

**Génération :**

| Colonne | Description |
|---------|-------------|
| `DrlgType` | Type de génération : 1=maze, 2=preset, 3=wilderness |
| `LevelType` | Référence vers LvlTypes.txt (tileset) |
| `SubType` | Sous-type de niveau (influence la variété) |
| `SubTheme` | Thème de remplissage (arbres, marécages...) |

**Connexions :**

| Colonne | Description |
|---------|-------------|
| `Vis0`-`Vis7` | IDs des niveaux connectés visuellement |
| `Warp0`-`Warp7` | IDs des warps dans LvlWarp.txt (affichage des entrées) |
| `WarpDist` | Taille de la zone de sécurité autour des entrées (en subtiles, défaut ~2025) |

**Éclairage et atmosphère :**

| Colonne | Description |
|---------|-------------|
| `LOSDraw` | 0 = pas de Line Of Sight (extérieur), 1 = LoS active (grottes) |
| `IsInside` | 0 = cycle jour/nuit, 1 = toujours jour |
| `Rain` | Peut-il pleuvoir (0/1) |
| `NoPer` | 0 = perspective autorisée, 1 = perspective interdite |
| `Intensity` / `Red` / `Green` / `Blue` | Contrôle RGB de l'éclairage (0-255) |

**Densité de monstres :**

| Colonne | Description |
|---------|-------------|
| `MonDen` | Densité de monstres (Normal) — valeur relative, pas un compte absolu |
| `MonDen(N)` | Densité Nightmare |
| `MonDen(H)` | Densité Hell |
| `MonUMin` / `MonUMax` | Min/Max de boss et champions dans la zone |
| `MonWndr` | 0 = monstres immobiles avant activation, 1 = IA active (errance) |
| `MonSpcWalk` | Comportement spécial de déplacement (1 = grilles métalliques, 5 = tuiles liquides) |

**Sélection des monstres :**

| Colonne | Description |
|---------|-------------|
| `Mtot` | Nombre total de types de monstres différents (max 4 simultanés par jeu) |
| `M1`-`M25` | IDs des monstres éligibles au spawn aléatoire |
| `S1`-`S25` | Monstres "satellites" : quand M1 spawn, S1 spawn aussi automatiquement |
| `Utot` | Nombre total de types de monstres éligibles comme boss/champions |
| `U1`-`U25` | IDs des monstres pouvant apparaître comme boss |

**Algorithme de sélection :**

```
fn select_monsters_for_zone(level, difficulty):
    // 1. Sélectionner Mtot types parmi M1-M25 (aléatoire seed-dépendant)
    pool = level.M1_to_M25.filter(|m| m != 0)
    selected = random_pick(pool, level.Mtot, rng)
    
    // 2. Vérifier que chaque monstre peut spawner
    for monster_id in selected:
        mon = MonStats[monster_id]
        if mon.spawn != 1:
            continue  // MonStats.txt "spawn" doit être 1
        
        // 3. Niveau de zone (difficulté)
        monster_level = match difficulty:
            Normal    => mon.Level           // niveau fixe depuis MonStats.txt
            Nightmare => level.MonLvl2       // niveau de zone
            Hell      => level.MonLvl3       // niveau de zone
```

Le jeu ne peut pas spawner plus de **4 types de monstres différents simultanément** dans une zone. Si `Mtot` > 4, les types sont sélectionnés aléatoirement à chaque génération, apportant de la variété entre les parties.

**Critters et objets de décor :**

| Colonne | Description |
|---------|-------------|
| `C1`-`C5` | Types de critters (serpents, poulets, chameaux...) |
| `CA1`-`CA5` | % chance de spawn de chaque critter (typiquement 30) |
| `objGrp0`-`objGrp7` | Groupes d'objets décoratifs/shrines (réf. Objgroup.txt) |
| `objPct0`-`objPct7` | % chance de spawn pour chaque groupe (max 100) |

**Waypoints et quêtes :**

| Colonne | Description |
|---------|-------------|
| `Waypoint` | Index du waypoint dans la zone (255 = aucun) |
| `SubWaypoint` | Gestion du waypoint dans les niveaux non-preset |
| `SubShrine` | Influence le spawn aléatoire de shrines en wilderness |
| `Quest` | ID de la quête liée à cette zone |
| `SaveMonster` | 1 = les monstres tués restent morts en revenant dans la zone |
| `Portal` / `Position` | Contrôle du repositionnement de portails |

---

## 9.8 Waypoints et objets de quête

### Placement des waypoints

| Contexte | Mécanisme de placement |
|----------|----------------------|
| **Preset (DrlgType 2)** | Position fixe dans le DS1 — toujours au même endroit |
| **Maze (DrlgType 1)** | Placé dans une room spécifique du labyrinthe (room désignée par `SubWaypoint`) |
| **Wilderness (DrlgType 3)** | Placé aléatoirement dans un espace vide de la zone |

La colonne `Waypoint` de `Levels.txt` détermine l'**index** du waypoint. La valeur 255 signifie qu'il n'y a pas de waypoint dans la zone.

### Zone de sécurité

La colonne `WarpDist` définit une zone tampon (en subtiles) autour des entrées de niveau et des waypoints pour **empêcher les stair-traps** (monstres campant directement sur les entrées). La valeur par défaut est ~2025 subtiles.

### Objets de quête

Les objets de quête sont placés via trois fichiers interconnectés :

| Fichier | Rôle |
|---------|------|
| `Objects.txt` | Définition de tous les objets plaçables (propriétés, animations) |
| `Objgroup.txt` | Groupes d'objets associés à des zones (shrines, coffres, etc.) |
| `Objpreset.txt` | Placement fixe d'objets dans des DS1 spécifiques |

**Types de placement :**

| Type | Mécanisme |
|------|-----------|
| **Type 1 (DS1 direct)** | Objets placés manuellement dans le DS1 via le DS1 Editor |
| **Type 2 (Objpreset)** | Objets référencés par ID et placés dans les DS1 selon l'acte |
| **Aléatoire (Objgroup)** | Objets sélectionnés depuis Objgroup.txt et placés par le DRLG |

**Spawn garanti :**

Certains objets de quête ont un **spawn garanti** car ils sont encodés directement dans le DS1 du preset :

| Objet | Zone | Mécanisme |
|-------|------|-----------|
| Cairn Stones | Stony Field | DS1 thématique dédié (toujours présent) |
| Inifuss Tree | Dark Woods | DS1 thématique dédié |
| Horadric Cube pedestal | Halls of the Dead Niv. 3 | DS1 preset (DrlgType 2) |
| Altars de quête | Zones preset | Encodé dans le DS1 |
| Super Uniques (Corpsefire, etc.) | Room spéciale du labyrinthe | DS1 "spec" (caveWspec.ds1) |

Les DS1 "spec" (comme `caveWspec.ds1` pour Corpsefire) ne peuvent spawn qu'**une seule fois** dans un labyrinthe et sont toujours placés à l'extrémité la plus profonde.

---

## 9.9 Zones spéciales

### Arcane Sanctuary (Acte 2)

| Aspect | Détail |
|--------|--------|
| **DrlgType** | 1 (Maze) |
| **Particularité** | L'entrée est toujours au **centre** du labyrinthe (hardcodé) |
| **Layout** | 4 branches partant du centre, chacune menant à un portail |
| **Pathfinding** | Extrêmement étroit — les followers se bloquent fréquemment |

### Maggot Lair (Acte 2)

| Aspect | Détail |
|--------|--------|
| **DrlgType** | 1 (Maze) |
| **Particularité** | Couloirs très étroits (2-3 subtiles de passage) |
| **Impact gameplay** | Les invocations et mercenaires bloquent le joueur |
| **Rooms** | Petites, avec des connexions sinueuses |

### Jungle Acte 3

| Aspect | Détail |
|--------|--------|
| **DrlgType** | 3 (Wilderness) |
| **Particularité** | Malgré DrlgType 3, la génération ressemble davantage aux caves (DrlgType 1) |
| **Complexité** | Zones denses avec beaucoup d'obstacles visuels et de passages étroits |

---

## 9.10 Transposition MGE — Génération procédurale

### Architecture recommandée

```rust
// Plugin MGE : mge-plugin-mapgen.v1

pub enum MapGenType {
    RandomMaze,       // DrlgType 1
    Preset,           // DrlgType 2
    RandomWilderness, // DrlgType 3
}

pub struct MapSeed {
    pub game_seed: u32,
    pub act_seeds: [u32; 5],
    pub level_seeds: HashMap<u32, u32>,
}

pub struct LevelDef {
    pub id: u32,
    pub gen_type: MapGenType,
    pub size: (u32, u32),
    pub level_type: u32,        // référence tileset
    pub connections: Vec<LevelConnection>,
    pub monster_density: f32,
    pub monster_pool: Vec<u32>,
    pub waypoint_index: Option<u8>,
}

pub struct MazeRoom {
    pub position: (i32, i32),   // position dans la grille de rooms
    pub openings: u8,           // bitmask NSEW (4 bits)
    pub preset_variant: u32,    // index de variante DS1
    pub is_special: bool,       // room de boss/quête
    pub tiles: TileGrid,
}

pub struct WildernessZone {
    pub borders: Vec<BorderSegment>,
    pub exits: Vec<ExitPoint>,
    pub paths: Vec<PathSegment>,
    pub themed_presets: Vec<PlacedPreset>,
    pub filler_elements: Vec<FillerElement>,
    pub base_floor: TileId,
}
```

### Mapping D2 → MGE

| Fichier D2 | Équivalent MGE |
|-----------|----------------|
| `Levels.txt` | `LevelDef` composant (table RON/JSON) |
| `LvlPrest.txt` | `PresetDef` composant |
| `LvlTypes.txt` | `TilesetDef` composant |
| `LvlMaze.txt` | `MazeDef` composant |
| `LvlSub.txt` | `FillerDef` composant |
| `LvlWarp.txt` | `WarpDef` composant |
| Fichiers DT1 | `TileAtlas` asset (conversion à l'import) |
| Fichiers DS1 | `RoomPreset` asset (conversion à l'import) |

### Différences clés avec D2

| Aspect | D2 Original | Allumina (MGE) |
|--------|-------------|----------------|
| **Seed** | 32-bit, stocké dans le .d2s | 64-bit recommandé (plus d'entropie) |
| **Taille de grille** | Subtiles entières | Grille logique + coordonnées flottantes |
| **Rooms max** | Limité par RAM (~200×200 tiles par niveau) | Beaucoup plus large (streaming possible) |
| **Persistance** | Régénéré à chaque partie | Persistant si monde ouvert (sérialisé) |
| **Variantes** | 6 max par preset | Illimité (chargement dynamique) |
| **Tuiles** | Palette 8-bit, 160×80px | Sprites modernes, résolution libre |

---

## 10. Références

| Document | Rôle |
|----------|------|
| [D2MOO (GitHub)](https://github.com/ThePhrozenKeep/D2MOO) | Reverse-engineering et réimplémentation de Diablo II |
| [Phrozen Keep — MonStats.txt](https://d2mods.info/forum/kb/viewarticle?a=360) | Documentation exhaustive de MonStats.txt |
| [Phrozen Keep — MonStats2.txt](https://d2mods.info/forum/kb/viewarticle?a=359) | Collision, taille, paramètres graphiques |
| [Phrozen Keep — Missiles.txt](https://d2mods.info/forum/kb/viewarticle?a=364) | Système de projectiles complet |
| [Phrozen Keep — Levels.txt](https://d2mods.info/forum/kb/viewarticle?a=384) | Référence complète Levels.txt (par Nefarius) |
| [Phrozen Keep — Advanced DT1](https://d2mods.info/forum/kb/viewarticle?a=468) | Orientations, directions, transparence murs, PopPads |
| [Paul Siramy — DT1 Format](http://paul.siramy.free.fr/_divers/dt1_doc/) | Spécification complète du format DT1 |
| [Paul Siramy — MAZE and DS1 Mechanisms](https://d2mods.info/forum/viewtopic.php?t=13427) | Algorithme de génération de labyrinthes |
| [Phrozen Keep — Randomizing Levels](https://d2mods.info/forum/kb/viewarticle?a=29) | Randomisation de niveaux preset (par Kingpin) |
| [OpenDiablo2/ds1 (GitHub)](https://github.com/OpenDiablo2/ds1) | Décodeur DS1 en Go (reverse-engineering) |
| [d2-map-investigation (GitHub)](https://github.com/squeek502/d2-map-investigation) | Corrélations de génération de cartes |
| [diablo-mapgen (GitHub)](https://github.com/Matthew-petroff/diablo-mapgen) | Outil de génération de cartes D2 |
| [D2 Tile Grid Guide](http://www.dos486.com/diablo/grid/) | Grille isométrique et footprints |
| [Frames and Animations](https://mannm.org/d2library/faqtoids/frames_eng.html) | Tick rate, breakpoints, animations |
| [D2R Map Seed Extraction](https://noobient.com/2025/11/21/finding-the-map-seed-in-diablo-ii-resurrected/) | Extraction du seed dans les .d2s |
| [Allumina — Document Fondateur](../Allumina%20-%20Document%20Fondateur.md) | Vision service Allumina |
| [Allumina — Combat et Troupes](./Allumina%20-%20Combat%20et%20Troupes.md) | Troupes et échelles de combat |
| [MGE — Skill Déplacement](../../../Miyukini_Game_Engine/deplacement-orientation/_index.md) | Chaîne locomotion MGE |
| [MGE — Hitbox et Collisions](../../../Miyukini_Game_Engine/MGE%20-%20Hitbox%20et%20Collisions%20-%20Reference.md) | Référence collision MGE |
| [MGE — Guide Groupes](../../../Miyukini_Game_Engine/MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md) | Pathfinding groupes MGE |

---

# 10. SYSTÈME DE GÉNÉRATION D'ITEMS — TREASURE CLASS

## 10.1 Vue d'ensemble du pipeline de génération

Quand un monstre meurt, le jeu exécute le pipeline suivant dans cet ordre exact :

```
Monster Kill
  ↓
[1] Déterminer le TC du monstre (monstats.txt → TreasureClassEx.txt)
  ↓
[2] TC Upgrade (NM/Hell : monter le TC si mlvl > TC level)
  ↓
[3] Pour chaque Pick (nombre = colonne Picks du TC) :
  │   ↓
  │   [3a] Roll NoDrop vs Prob1..Prob10
  │   ↓
  │   [3b] Si NoDrop → rien, passer au pick suivant
  │   [3c] Si TC enfant sélectionné → descendre récursivement dans le TC enfant
  │   [3d] Si item atomique sélectionné → continuer vers [4]
  ↓
[4] Détermination de la qualité (Unique → Set → Rare → Magic → Superior → Normal → Low)
  ↓
[5] Sélection Unique/Set spécifique (si qualité Unique ou Set)
  ↓
[6] Génération des affixes (si Magic, Rare ou Crafted)
  ↓
[7] Roll Éthéré (5% pour items éligibles)
  ↓
[8] Roll Sockets (si applicable)
  ↓
[9] Roll des valeurs de propriétés (ranges des mods)
  ↓
Item final généré
```

## 10.2 Structure des Treasure Classes

### Fichier TreasureClassEx.txt

Chaque ligne définit un TC avec :

| Colonne | Rôle |
|---------|------|
| `Treasure Class` | Nom du TC |
| `Picks` | Nombre de tentatives de drop |
| `group` | Groupe pour TC upgrade (NM/Hell) |
| `level` | Niveau du TC (pour TC upgrade) |
| `Unique` | Bonus qualité Unique (QualityFactor, 0-1024) |
| `Set` | Bonus qualité Set (QualityFactor, 0-1024) |
| `Rare` | Bonus qualité Rare (QualityFactor, 0-1024) |
| `Magic` | Bonus qualité Magic (QualityFactor, 0-1024) |
| `NoDrop` | Poids de NoDrop |
| `Item1..Item10` | Entrées (items ou sous-TCs) |
| `Prob1..Prob10` | Poids de probabilité de chaque entrée |

### Hiérarchie récursive

Les TCs forment un **arbre récursif**. Chaque entrée dans un TC peut être :
- Un **item atomique** (ex : `gld`, `amu`, un code d'item spécifique)
- Un **TC enfant** (ex : `Act 5 (H) Equip C`)
- Un **TC auto-généré** (ex : `weap87`, `armo84` — créés au runtime depuis weapons.txt/armor.txt)

### TCs auto-générés (Atomic TCs)

Le jeu génère au runtime des TCs `WeapXX` et `ArmoXX` (XX = 03 à 87) en regroupant les items par qlvl :

```
Armo03 = tous les items d'armure avec qlvl 1-3
  → Cap (qlvl 1), Quilted Armor (qlvl 1), Buckler (qlvl 1), etc.

Armo87 = tous les items d'armure avec qlvl 85-87
  → Diadem (qlvl 85), Corona (qlvl 85), Sacred Armor (qlvl 85), etc.

Weap87 = tous les items d'arme avec qlvl 85-87
  → Phase Blade (qlvl 73 → NON, pas dans weap87), etc.
```

### Pondération dans les TCs atomiques

Dans les TCs atomiques, chaque item a un poids `Rarity` (défini dans ItemTypes.txt) :

| Type d'item | Rarity |
|-------------|--------|
| Items normaux (épées, armures...) | 3 |
| Assassin claws | 2 |
| Wands / Staves / Scepters | 1 |
| Autres class-specific | 1 |

**Probabilité d'un item** = `ItemRarity / TotalRarity`

Exemple `armo87` (13 de total rarity) :
- Diadem : 3/22
- Corona : 3/22
- Sacred Armor : 3/22
- Class-specific items : 1/22 chacun

## 10.3 Picks et sélection

### Picks positifs

Si `Picks > 0`, le jeu effectue `Picks` tentatives indépendantes. Chaque tentative :
1. Calcule la somme totale : `Total = NoDrop + Prob1 + Prob2 + ... + Prob10`
2. Tire un nombre aléatoire `R` dans `[0, Total)`
3. Sélectionne l'entrée correspondante au poids cumulé

### Picks négatifs

Si `Picks < 0`, le jeu garantit exactement `|Picks|` items, en **ignorant NoDrop**. Les items sont distribués proportionnellement aux poids Prob.

Exemple : `Picks = -3`, `Prob1=2`, `Prob2=1` → toujours 2 de Item1 + 1 de Item2.

### Limite physique

Un monstre ne peut pas drop plus de **6 items** au sol (limitation du moteur). Si un boss a `Picks=7` (comme Mephisto), un des rolls est gaspillé si tous réussissent.

### Exemple complet : Mephisto Hell

```
Mephisto (H) :
  Picks = 7
  NoDrop = 15
  gld,mul=2048     → Prob = 5
  Act 4 (H) Equip A → Prob = 52
  Act 4 (H) Junk    → Prob = 5
  Act 4 (H) Good    → Prob = 3
  
  Bonus qualité : Unique=983, Set=983, Rare=983, Magic=1024
  Total = 15 + 5 + 52 + 5 + 3 = 80

Pour CHAQUE pick (7 fois) :
  15/80 = 18.75% → NoDrop
  5/80  = 6.25%  → Gold
  52/80 = 65%    → Equipment
  5/80  = 6.25%  → Junk (potions, flèches)
  3/80  = 3.75%  → Good (gemmes, runes, jewels, charmes, anneaux, amulettes)
```

## 10.4 TC Upgrade (Nightmare / Hell)

En Normal, le TC du monstre est utilisé directement. En NM/Hell, le TC peut être **upgradé** :

```
Algorithme TC Upgrade :
  1. Trouver le TC de base du monstre (monstats.txt)
  2. Vérifier si le TC a un "group" défini
  3. Si oui : trouver le TC le plus élevé dans le même group
     dont level ≤ mlvl du monstre
  4. Utiliser ce TC upgradé
  5. Les TCs enfants (inclus dans le TC sélectionné) ne sont PAS upgradés
```

Exemple :
- Devilkin dans The Pit (Hell) : mlvl = 85
- TC de base : "Act 1 (H) H2H B" (group=7, level=66)
- TC upgradé : "Act 5 (H) H2H C" (group=7, level=85 ≤ mlvl)

## 10.5 Formule NoDrop et scaling multijoueur

### Formule NoDrop en solo

```
P(NoDrop) = NoDrop / (NoDrop + ProbSum)
```

Où `ProbSum = Prob1 + Prob2 + ... + Prob10`

### Formule NoDrop multijoueur

```
NewNoDrop = int( ProbSum / ( 1/( (NoDrop/(NoDrop+ProbSum))^N ) - 1 ) )
```

Où :
```
N = int(1 + AdditionalPlayers/2 + ClosePartiedPlayers/2)

- AdditionalPlayers : tous les autres joueurs dans la partie
- ClosePartiedPlayers : joueurs dans votre party ET à moins de 2 écrans
- int() : troncature (pas d'arrondi)
```

### Commande /players

La commande `/playersX` ajoute des joueurs "non-partied". Le mapping effectif :

| Commande | N effectif (NoDrop exponent) |
|----------|------------------------------|
| /players 1 ou 2 | 1 |
| /players 3 ou 4 | 2 |
| /players 5 ou 6 | 3 |
| /players 7 ou 8 | 4 |

### Exemple : Mephisto

```
NoDrop = 15, ProbSum = 65, Total = 80

Solo (N=1) :
  NoDrop rate = (15/80)^1 = 0.1875
  NewNoDrop = 15 → 15/80 = 18.75% NoDrop par pick

N=2 :
  NoDrop rate = (15/80)^2 = 0.03516
  NewNoDrop = int(65 × 0.03516 / (1 - 0.03516)) = int(2.366) = 2
  → 2/67 = 2.99% NoDrop par pick

N=3 :
  NoDrop rate = (15/80)^3 = 0.00659
  NewNoDrop = int(65 × 0.00659 / (1 - 0.00659)) = int(0.431) = 0
  → 0% NoDrop (full drops garantis)
```

## 10.6 Boss d'acte — Règles spéciales

### Bonus de niveau des élites

| Type de monstre | Bonus mlvl |
|----------------|-----------|
| Champion | +2 |
| Boss / Unique / Minion | +3 |

### Règles spécifiques aux boss d'acte

| Règle | Effet |
|-------|-------|
| **Force Magic** | Les boss d'acte et les boss aléatoires forcent un drop minimum de qualité Magic (ne s'applique pas aux items non-magiques : runes, potions, etc.) |
| **QualityFactor élevé** | Mephisto/Diablo/Baal (H) ont `Unique=983, Set=983, Rare=983, Magic=1024` → énorme bonus qualité |
| **Quest Drop** | Premier kill d'un boss d'acte = drops améliorés (pas de white items, pas de potions dans le drop principal) |

### Boss levels (monstats.txt)

| Boss | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Andariel | 12 | 49 | 75 |
| Duriel | 22 | 55 | 88 |
| Mephisto | 26 | 59 | 87 |
| Diablo | 40 | 62 | 94 |
| Baal | 60 | 75 | 99 |
| Nihlathak | 65 | 70 | 92 |
| Uber Bosses | — | — | 110 |

### Quest Drop Bug (Andariel)

Andariel peut être **permanentement buguée** en quest drop :
1. Tuer Andariel pour la première fois
2. Aller directement à l'Acte 2 (via le portail de Warriv)
3. Sauvegarder et quitter dans l'Acte 2

Si cela est fait correctement, **toutes les exécutions futures** d'Andariel en cette difficulté utiliseront la table de quest drop (pas de white items/potions).

Pour les autres boss (Duriel, Mephisto, Diablo, Baal), le quest drop bug nécessite un personnage secondaire n'ayant pas complété la quête pour porter le coup final.

---

# 11. DÉTERMINATION DE LA QUALITÉ D'ITEM

## 11.1 Niveaux fondamentaux

| Niveau | Abréviation | Source | Description |
|--------|-------------|--------|-------------|
| **Item Level** | ilvl | = mlvl du monstre | Niveau caché de l'item, déterminé à la création |
| **Monster Level** | mlvl | monstats.txt / area level | Niveau du monstre qui drop |
| **Quality Level** | qlvl | weapons.txt / armor.txt | Niveau intrinsèque du type d'item de base |
| **Area Level** | alvl (zone) | levels.txt | Niveau de la zone (= mlvl des monstres normaux en NM/Hell) |
| **Affix Level** | alvl (affix) | Calculé | Niveau déterminant quels affixes peuvent apparaître |
| **Character Level** | clvl | Joueur | Niveau du personnage (utilisé pour gambling) |

### Relation ilvl ↔ mlvl

```
ilvl = mlvl (pour drops de monstres)
ilvl = area level (pour drops de coffres/conteneurs)

Champions : mlvl = area_level + 2
Uniques/Boss random : mlvl = area_level + 3
Boss d'acte : mlvl fixe (voir table §10.6)
```

## 11.2 Algorithme de détermination de qualité

Le jeu teste les qualités **dans cet ordre exact** et s'arrête au premier succès :

```
1. Test UNIQUE    → Si succès → générer Unique (ou downgrade)
2. Test SET       → Si succès → générer Set (ou downgrade)
3. Test RARE      → Si succès → générer Rare
4. Test MAGIC     → Si succès → générer Magic
5. Test SUPERIOR  → Si succès → générer Superior
6. Test NORMAL    → Si succès → générer Normal
7. Fallback       → Low Quality (cracked, crude, etc.)
```

### Formule complète de chaque test

```
ÉTAPE 1 : Sélectionner la ligne correcte dans ItemRatio.txt
  → Version (0=Classic, 1=LoD)
  → Uber (0=Normal tier, 1=Exceptional/Elite tier)
  → Class Specific (0=non, 1=oui)

ÉTAPE 2 : Calculer Chance
  Chance = (BaseChance - ((ilvl - qlvl) / Divisor)) × 128

ÉTAPE 3 : Appliquer Magic Find (seulement pour Unique, Set, Rare)
  EffectiveMF = MF × Factor / (MF + Factor)
  
  Factors (rendements décroissants) :
    Unique : Factor = 250
    Set    : Factor = 500
    Rare   : Factor = 600
    Magic  : Pas de diminishing returns → EffectiveMF = MF
  
  Chance = Chance × 100 / (100 + EffectiveMF)

ÉTAPE 4 : Appliquer le minimum
  if (Chance < MinChance) then Chance = MinChance

ÉTAPE 5 : Appliquer le QualityFactor du TC
  FinalChance = Chance - (Chance × QualityFactor / 1024)
  
  QualityFactor = valeur Unique/Set/Rare/Magic du TC dans TreasureClassEx.txt
  (valeur maximale rencontrée dans toute la chaîne de TCs traversée)

ÉTAPE 6 : Roll final
  Générer un nombre aléatoire R dans [0, FinalChance)
  if (R < 128) → SUCCÈS (item de cette qualité)
  else         → ÉCHEC (passer au test suivant)

Probabilité finale = 128 / FinalChance
```

## 11.3 Valeurs de ItemRatio.txt (v1.13 LoD)

### Items NON class-specific, tier Normal (Version=1, Uber=0, ClassSpecific=0)

| Qualité | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 400 | 1 | 6400 |
| **Set** | 160 | 2 | 5600 |
| **Rare** | 100 | 2 | 3200 |
| **Magic** | 34 | 3 | 192 |
| **HiQuality** | 12 | 8 | — |
| **Normal** | 2 | 2 | — |

### Items NON class-specific, tier Exceptional/Elite (Version=1, Uber=1, ClassSpecific=0)

| Qualité | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 400 | 1 | 6400 |
| **Set** | 160 | 2 | 5600 |
| **Rare** | 100 | 2 | 3200 |
| **Magic** | 34 | 3 | 192 |
| **HiQuality** | 12 | 8 | — |
| **Normal** | 1 | 1 | — |

### Items CLASS-SPECIFIC, tier Normal (Version=1, Uber=0, ClassSpecific=1)

| Qualité | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 240 | 3 | 6400 |
| **Set** | 120 | 3 | 5600 |
| **Rare** | 80 | 3 | 3200 |
| **Magic** | 17 | 6 | 192 |
| **HiQuality** | 9 | 8 | — |
| **Normal** | 2 | 2 | — |

### Items CLASS-SPECIFIC, tier Exceptional/Elite (Version=1, Uber=1, ClassSpecific=1)

| Qualité | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 240 | 3 | 6400 |
| **Set** | 120 | 3 | 5600 |
| **Rare** | 80 | 3 | 3200 |
| **Magic** | 17 | 6 | 192 |
| **HiQuality** | 9 | 8 | — |
| **Normal** | 1 | 1 | — |

## 11.4 Formules Magic Find — Rendements décroissants

### Formule Effective Magic Find (EMF)

```
Pour Unique : EMF = MF × 250 / (MF + 250)
Pour Set    : EMF = MF × 500 / (MF + 500)
Pour Rare   : EMF = MF × 600 / (MF + 600)
Pour Magic  : EMF = MF (pas de diminishing returns)

Exception : si MF ≤ 10, alors EMF = MF (pas de DR appliqué)
```

### Table de référence EMF

| MF réel | EMF Unique | EMF Set | EMF Rare | EMF Magic |
|---------|-----------|---------|----------|-----------|
| 0 | 0 | 0 | 0 | 0 |
| 50 | 42 | 45 | 46 | 50 |
| 100 | 71 | 83 | 86 | 100 |
| 200 | 111 | 143 | 150 | 200 |
| 300 | 136 | 188 | 200 | 300 |
| 400 | 154 | 222 | 240 | 400 |
| 500 | 167 | 250 | 273 | 500 |
| 700 | 184 | 292 | 323 | 700 |
| 1000 | 200 | 333 | 375 | 1000 |

### Caps effectifs

- EMF Unique ne peut jamais dépasser 250 (atteint à MF → ∞)
- EMF Set ne peut jamais dépasser 500
- EMF Rare ne peut jamais dépasser 600
- EMF Magic n'a pas de cap

## 11.5 Exemple complet : Baal Hell drop Unearthed Wand (200% MF)

```
Item : Unearthed Wand (wand elite, class-specific=Non*, Uber=1)
  * Les wands ne sont PAS class-specific dans ItemTypes.txt
ilvl = 99 (Baal Hell mlvl)
qlvl = 86 (Unearthed Wand dans weapons.txt)
MF = 200%

--- TEST UNIQUE ---
Ligne ItemRatio.txt : Version=1, Uber=1, ClassSpecific=0
BaseChance = 400, Divisor = 1, MinChance = 6400

Étape 2 : Chance = (400 - ((99-86)/1)) × 128 = (400-13) × 128 = 387 × 128 = 49536
Étape 3 : Factor = 250 (unique)
  EMF = 200 × 250 / (200+250) = 50000/450 = 111
  Chance = 49536 × 100 / (100+111) = 49536 × 100/211 = 23476
Étape 4 : 23476 > MinChance(6400) → pas de clamp
Étape 5 : QualityFactor = 983 (Baal Hell TC)
  FinalChance = 23476 - (23476 × 983/1024) = 23476 - 22530 = 946 ≈ 939*

Probabilité Unique = 128/939 = 13.6%

* La légère différence vient de l'arithmétique entière (int truncation à chaque étape)
```

---

# 12. GÉNÉRATION DES ITEMS UNIQUES ET SET

## 12.1 Sélection d'un Unique spécifique

Après que le test de qualité a déterminé "Unique", le jeu :

```
1. Construire la liste de tous les Uniques du même type de base
   dont qlvl_unique ≤ ilvl de l'item

2. Si la liste est VIDE :
   → Downgrade en RARE avec durabilité × 3
   → (Si le type ne peut pas être Rare → Magic avec durabilité × 3)

3. Si la liste contient UN seul item :
   → Cet Unique est sélectionné

4. Si la liste contient PLUSIEURS items :
   → Sélection pondérée par le champ "rarity" de UniqueItems.txt
   → P(item) = item.rarity / Σ(all_rarities)
```

### Champ Rarity des Uniques

| Item | Rarity | Commentaire |
|------|--------|-------------|
| Manald Heal (anneau unique) | 15 | Très commun |
| Nagelring (anneau unique) | 15 | Très commun |
| Stone of Jordan | 1 | 15× plus rare que Manald |
| Bul-Kathos' Wedding Band | 1 | Rare |

Exemple : 9 anneaux uniques éligibles, total rarity = 59
- SoJ = 1/59 ≈ 1.7%
- Manald = 15/59 ≈ 25.4%

## 12.2 Prévention des doublons Uniques

### Champ "nolimit" de UniqueItems.txt

| Valeur nolimit | Comportement |
|----------------|-------------|
| 0 (ou vide) | L'Unique ne peut drop qu'**une seule fois** par partie |
| 1 | Pas de limite (peut drop plusieurs fois) |

### Mécanisme de prévention

Quand un Unique avec `nolimit=0` est sélectionné :

```
1. Vérifier si cet Unique a déjà été :
   a. Droppé comme Unique dans cette partie
   b. Droppé comme failed unique (rare 3× durabilité) dans cette partie
   c. Généré dans l'écran de gambling dans cette partie

2. Si déjà généré → Downgrade en Rare avec durabilité × 3

3. Si non → Drop l'Unique et marquer comme "généré" pour cette partie
```

C'est pourquoi les uniques communs (Manald, Nagelring) "bloquent" le drop de SoJ : une fois Manald généré, la prochaine tentative de Manald downgrade en rare, mais le flag empêche aussi sa re-sélection.

## 12.3 Conditions de downgrade

### Downgrade Unique → Rare

Un Unique est downgrade en **Rare avec durabilité × 3** si :
1. Le qlvl de l'Unique > ilvl (monstre pas assez haut niveau)
2. Aucun Unique n'existe pour ce type de base
3. L'Unique a déjà été généré dans cette partie (`nolimit=0`)

### Downgrade Set → Magic

Un Set est downgrade en **Magic avec durabilité × 2** si :
1. Le qlvl du Set > ilvl
2. Aucun Set n'existe pour ce type de base

### Cas notable : Pindleskin et Arachnid Mesh

```
Pindleskin (Hell) : mlvl = 86
Arachnid Mesh (unique Spiderweb Sash) : qlvl = 87

86 < 87 → qlvl > mlvl → IMPOSSIBLE pour Pindleskin de drop Arachnid Mesh
→ Toute tentative de Unique Spiderweb Sash = Rare 3× durabilité
```

### Cas notable : Tyrael's Might et Templar's Might

```
Sacred Armor uniques :
  Templar's Might : qlvl = 85
  Tyrael's Might  : qlvl = 87

Pindleskin (mlvl=86) :
  → Peut drop Templar's Might (85 ≤ 86)
  → Ne peut PAS drop Tyrael's Might (87 > 86)
  
Baal Hell (mlvl=99) :
  → Peut drop les deux
```

## 12.4 Sélection d'un Set spécifique

Même algorithme que pour les Uniques :

```
1. Construire la liste de tous les items Set du même type de base
   dont qlvl_set ≤ ilvl
2. Si vide → Magic avec durabilité × 2
3. Si un seul → sélectionné
4. Si plusieurs → pondération par rarity
```

---

# 13. SYSTÈME DE GÉNÉRATION DES AFFIXES

## 13.1 Calcul du Affix Level (alvl)

Le alvl détermine quels affixes (prefixes/suffixes) sont disponibles pour un item :

```
Algorithme (arithmétique entière, pas de fractions) :

1. if (ilvl > 99) then ilvl = 99    // Cap à 99
2. if (qlvl > ilvl) then ilvl = qlvl // qlvl minimum
   // (Note : ce ilvl modifié est temporaire, ne change pas l'item)

3. if (magic_lvl > 0) then
     alvl = ilvl + magic_lvl
   else
     if (ilvl < (99 - qlvl/2)) then
       alvl = ilvl - qlvl/2          // int division
     else
       alvl = 2 × ilvl - 99

4. if (alvl > 99) then alvl = 99    // Cap à 99
```

### Magic Level (maglvl)

Le `magic_lvl` est un attribut de certains types d'items dans weapons.txt/armor.txt. Les items avec un magic level non-nul :

| Type | Magic Level |
|------|-------------|
| Wands (non-elite) | Variable (1-18) |
| Staves (non-elite) | Variable (1-18) |
| Orbs | Variable (1-18) |
| Circlets | Variable (3-8) |
| Autres items | 0 |

### Exemple : Small Charm droppé par Pindleskin Normal

```
ilvl = 45 (Pindleskin Normal mlvl)
qlvl = 28 (Small Charm qlvl)
magic_lvl = 0

qlvl(28) < ilvl(45) → pas de clamp
magic_lvl = 0 → branche else
ilvl(45) < 99 - qlvl/2 = 99 - 14 = 85 → oui
alvl = 45 - 28/2 = 45 - 14 = 31
```

## 13.2 Sélection des affixes pour items Magic

### Nombre d'affixes (items Magic)

```
Roll aléatoire :
  50% → Suffix seulement
  25% → Prefix seulement
  25% → Prefix ET Suffix

Maximum : 1 Prefix + 1 Suffix
```

### Sélection d'un affix

Pour chaque slot (prefix/suffix) :

```
1. Construire la liste des affixes éligibles :
   - spawnable = 1
   - itype correspond au type de l'item
   - etype ne contient PAS le type de l'item
   - level ≤ alvl (affix level min)
   - maxlevel ≥ alvl OU maxlevel = 0 (pas de cap)
   - version correcte (Classic/LoD)

2. Sélection pondérée par "frequency" :
   P(affix) = affix.frequency / Σ(frequencies de tous les affixes éligibles)
   
   frequency = 0 → ne peut JAMAIS apparaître en drop
   (seulement via cube recipes)
```

## 13.3 Sélection des affixes pour items Rare

### Nombre d'affixes (items Rare)

```
Les Rare Items ont entre 3 et 6 affixes.
Probabilité 1/4 (25%) pour chaque nombre : 3, 4, 5, ou 6.

Maximum : 3 Prefixes + 3 Suffixes

Restriction Jewels Rare : maximum 4 affixes total
```

### Note sur ilvl et nombre d'affixes des Crafted items

```
ilvl 1-30  : 40% → 1 affix, 20% → 2, 20% → 3, 20% → 4
ilvl 31-50 : 60% → 2 affixes, 20% → 3, 20% → 4
ilvl 51-70 : 80% → 3 affixes, 20% → 4
ilvl 71+   : 100% → 4 affixes
```

### Algorithme de sélection (Rare)

```
fn generate_rare_affixes(item, alvl):
  num_affixes = random_choice([3, 4, 5, 6], weights=[1,1,1,1])
  prefix_count = 0
  suffix_count = 0
  used_groups = HashSet::new()
  affixes = []
  
  for i in 0..num_affixes:
    // 50/50 prefix ou suffix
    is_prefix = random_bool()
    
    // Vérifier les caps
    if is_prefix and prefix_count >= 3:
      is_prefix = false  // forcer suffix
    if !is_prefix and suffix_count >= 3:
      is_prefix = true   // forcer prefix
    
    // Construire la liste des affixes éligibles
    pool = get_eligible_affixes(
      item.type,
      alvl,
      is_prefix,
      exclude_groups = used_groups,
      rare_only = true  // certains affixes sont "magic only"
    )
    
    if pool.is_empty():
      break
    
    // Sélection pondérée par frequency
    affix = weighted_random(pool, |a| a.frequency)
    
    affixes.push(affix)
    used_groups.insert(affix.group)
    if is_prefix: prefix_count += 1
    else: suffix_count += 1
  
  return affixes
```

### Contraintes de groupes

```
- Un item ne peut pas avoir 2 affixes du même "group" (MagicPrefix.txt / MagicSuffix.txt)
- Exemple : "Wyrm" (+41-60 Mana, group 55) et "Dragon's" (+31-40 Mana, group 55) 
  sont mutuellement exclusifs
- Des affixes du même TYPE mais de GROUPS différents peuvent coexister
  (ex : +life prefix et +life suffix)
```

### Affixes "magic only"

Certains affixes ont `rare=0` dans MagicPrefix.txt/MagicSuffix.txt :
- Ces affixes ne peuvent apparaître **que** sur des items Magic
- Ils ne peuvent **pas** apparaître sur des Rare ou Crafted
- Exemple : Jeweler's (4 sockets), certains +3 skill tree prefixes

## 13.4 Staffmods et Automods

### Staffmods

Les staffmods sont des bonus de compétences **intrinsèques** à certains types d'items, générés comme des propriétés de base (comme la durabilité) et non comme des affixes :

| Type d'item | Skills possibles |
|-------------|-----------------|
| Wands | Skills Nécromancien |
| Staves | Skills Sorcière |
| Scepters | Skills Paladin |
| Claws (assassin) | Skills Assassin |
| Orbs | Skills Sorcière |
| Druid helms | Skills Druide |
| Barbarian helms | Skills Barbare |

Les staffmods **ne sont pas supprimés** par les runewords — un item base avec de bons staffmods les conserve.

### Automods

Les automods sont des propriétés automatiques définies par `auto prefix` dans weapons.txt/armor.txt :
- Orbs → bonus mana
- Nécromancien heads → poison damage
- Paladin shields → résistances

---

# 14. SYSTÈME DE SOCKETS

## 14.1 Détermination du nombre de sockets (items normaux/supérieurs)

### Items éligibles aux sockets

Seuls ces types peuvent avoir des sockets :
- Casques
- Boucliers
- Armures corporelles
- Armes (sauf armes de jet)

### Probabilité d'avoir des sockets (normal/superior)

```
1/3 de tous les items normaux et supérieurs sont générés socketed.
Les items low quality ne peuvent PAS avoir de sockets.
```

### Nombre de sockets : ItemTypes.txt

Le fichier ItemTypes.txt définit 3 caps par type d'item :

| Colonne | Applicable si |
|---------|---------------|
| `MaxSock1` | ilvl ≤ 25 |
| `MaxSock25` | ilvl 26-40 |
| `MaxSock40` | ilvl ≥ 41 |

Le nombre de sockets est un random dans `[1, min(MaxSockX, gemsockets)]` :
- `MaxSockX` = cap par ilvl du type (ItemTypes.txt)
- `gemsockets` = cap absolu de l'item spécifique (weapons.txt / armor.txt)

### Exemple : Crystal Sword

```
gemsockets (weapons.txt) = 6
Type = "swor" → MaxSock1=3, MaxSock25=4, MaxSock40=6

ilvl 1-25  : random [1, min(3, 6)] = [1, 3] sockets
ilvl 26-40 : random [1, min(4, 6)] = [1, 4] sockets
ilvl 41+   : random [1, min(6, 6)] = [1, 6] sockets
```

### Exemple : Monarch Shield

```
gemsockets (armor.txt) = 4
Type = "shie" → MaxSock1=3, MaxSock25=3, MaxSock40=4

ilvl 1-25  : random [1, min(3, 4)] = [1, 3] sockets
ilvl 26-40 : random [1, min(3, 4)] = [1, 3] sockets
ilvl 41+   : random [1, min(4, 4)] = [1, 4] sockets
```

## 14.2 Larzuk (quête Siege on Harrogath)

### Règles de Larzuk

| Qualité de l'item | Sockets ajoutés |
|-------------------|----------------|
| **Normal (white/grey)** | Maximum possible = `min(MaxSockX_pour_ilvl, gemsockets)` |
| **Superior** | Maximum possible (même formule que normal) |
| **Magic** | 1 ou 2 sockets (50/50) |
| **Rare** | 1 socket (toujours) |
| **Set** | 1 socket (toujours) |
| **Unique** | 1 socket (toujours) |
| **Crafted** | 1 socket (toujours) |

Larzuk est **déterministe** pour les items normaux : toujours le maximum.

### Utilisations

3 utilisations par personnage : 1 en Normal, 1 en Nightmare, 1 en Hell.

## 14.3 Recettes Cube pour sockets

### Recettes pour items normaux (non-socketed, non-superior)

| Type | Recette | Range de sockets |
|------|---------|-----------------|
| **Arme** | Ral + Amn + Perfect Amethyst + arme | 1-6 (aléatoire) |
| **Armure corporelle** | Tal + Thul + Perfect Topaz + armure | 1-4 (aléatoire) |
| **Casque** | Ral + Thul + Perfect Sapphire + casque | 1-3 (aléatoire) |
| **Bouclier** | Tal + Amn + Perfect Ruby + bouclier | 1-4 (aléatoire) |

### Algorithme cube socket

```
Le cube roll un nombre entre 1 et 6 (distribution uniforme).
Si le résultat dépasse le maximum de l'item → clamp au maximum.

Exemple : Claws (max 3 sockets)
  Roll 1 → 1 socket  (1/6)
  Roll 2 → 2 sockets (1/6)
  Roll 3 → 3 sockets (1/6)
  Roll 4 → 3 sockets (1/6)  // clamp
  Roll 5 → 3 sockets (1/6)  // clamp
  Roll 6 → 3 sockets (1/6)  // clamp
  
  Résultat : 1/6 → 1os, 1/6 → 2os, 4/6 → 3os
```

### Recette socket pour items Rare

```
3× Perfect Skulls + Stone of Jordan + item Rare → item Rare avec 1 socket
(Détruit les stats existantes et re-roll le Rare, puis ajoute 1 socket)
```

## 14.4 Sockets sur items Rare (natifs)

Les items Rare peuvent avoir des sockets naturels si l'affix **Mechanist's** (prefix) est sélectionné :
- 1 ou 2 sockets (50/50)
- Clampé au maximum du type de base
- Exemple : Rare Buckler avec Mechanist's → toujours 1 socket (max sockets buckler = 1)

## 14.5 Astuce Low Quality → Normal → Socket

```
1. Trouver un item Low Quality elite (ex : Crude Phase Blade)
2. Cube : El + Chipped Gem + item Low Quality → item Normal de ilvl 1
3. Larzuk : socket l'item → min(MaxSock1, gemsockets) sockets
   Pour Phase Blade : min(3, 6) = 3 sockets (au lieu de 6 avec un ilvl élevé)
4. Utile pour les runewords à 3 sockets dans un Phase Blade
```

---

# 15. ITEMS ÉTHÉRÉS

## 15.1 Probabilité et éligibilité

### Chance de base

```
P(Éthéré) = 5% (1/20) pour tous les items éligibles
Indépendant du Magic Find et de tout autre modificateur.
Le roll est effectué APRÈS la détermination de la qualité.
```

### Items qui NE PEUVENT PAS être éthérés

| Catégorie | Raison |
|-----------|--------|
| **Items Set** | Flag éthéré désactivé dans le code |
| **Anneaux** | Pas de flag éthéré |
| **Amulettes** | Pas de flag éthéré |
| **Charmes** | Pas de flag éthéré (sauf exceptions : SoJ, Annihilus, Hellfire Torch, Gheed's → toujours éthérés) |
| **Arcs (Bows)** | Flag éthéré désactivé |
| **Arbalètes (Crossbows)** | Flag éthéré désactivé |
| **Items Crafted** | Flag éthéré désactivé |
| **Phase Blade** | Indestructible par nature → pas d'éthéré |
| **Items Low Quality** | Flag éthéré impossible |

### Items toujours éthérés

Certains uniques sont **toujours** éthérés :
- Stone of Jordan
- Annihilus
- Hellfire Torch
- Gheed's Fortune
- Ghostflame (unique War Sword) — éthéré ET indestructible

## 15.2 Bonus éthéré

| Bonus | Valeur |
|-------|--------|
| **Dégâts de base** | +50% (multiplicateur sur min/max damage) |
| **Défense de base** | +50% (multiplicateur sur min/max defense) |
| **Durabilité** | -50% (arrondi inférieur) puis -1 supplémentaire |
| **Requirements STR/DEX** | -10 chacun |
| **Réparation** | Impossible (aucun PNJ ne peut réparer un item éthéré) |
| **Valeur marchande** | -75% ou plus |

### Formule durabilité éthérée

```
eth_durability = floor(base_durability / 2) - 1
Si eth_durability < 1 → eth_durability = 1
```

### Interactions spéciales

| Situation | Comportement |
|-----------|-------------|
| **Éthéré sur Mercenaire** | La durabilité ne diminue PAS quand équipé sur un mercenaire |
| **Éthéré + Zod Rune** | L'item devient indestructible (durabilité ne diminue plus jamais) |
| **Éthéré + Self-repair mod** | L'item se répare automatiquement (mod "Repairs 1 durability in X seconds") |
| **Éthéré + "Indestructible" mod** | L'item ne perd pas de durabilité |
| **Éthéré vendu à un PNJ** | Item détruit immédiatement, ne peut PAS être racheté |

## 15.3 Rune Zod

| Propriété | Valeur |
|-----------|--------|
| **Effet** | "Indestructible" — annule toute perte de durabilité |
| **Level requirement** | 69 (ou le level req de l'item si supérieur) |
| **Rareté** | 2ème item le plus rare du jeu |
| **Usage principal** | Socketer dans un item éthéré pour le rendre permanent |

---

# 16. RÉCAPITULATIF DES FORMULES — RÉFÉRENCE RAPIDE

## 16.1 Quality Roll

```
Chance = (BaseChance - (ilvl - qlvl) / Divisor) × 128
Chance = Chance × 100 / (100 + EMF)
if Chance < MinChance → Chance = MinChance
FinalChance = Chance - (Chance × QualityFactor / 1024)
P(qualité) = 128 / FinalChance
```

## 16.2 Effective Magic Find

```
EMF_unique = MF × 250 / (MF + 250)
EMF_set    = MF × 500 / (MF + 500)
EMF_rare   = MF × 600 / (MF + 600)
EMF_magic  = MF
```

## 16.3 Affix Level

```
if ilvl > 99 → ilvl = 99
if qlvl > ilvl → ilvl = qlvl
if magic_lvl > 0 → alvl = ilvl + magic_lvl
else if ilvl < (99 - qlvl/2) → alvl = ilvl - qlvl/2
else → alvl = 2×ilvl - 99
if alvl > 99 → alvl = 99
```

## 16.4 NoDrop Multijoueur

```
N = int(1 + OtherPlayers/2 + ClosePartyMembers/2)
NewNoDrop = int(ProbSum / (1/((NoDrop/(NoDrop+ProbSum))^N) - 1))
```

## 16.5 Sockets

```
max_sockets = min(MaxSockX[ilvl_bracket], gemsockets)
  où X = 1 si ilvl≤25, 25 si ilvl≤40, 40 si ilvl≥41

Larzuk (Normal/Superior) : toujours max_sockets
Larzuk (Magic) : random(1, 2)
Larzuk (Rare/Set/Unique/Crafted) : toujours 1
Cube (Normal) : random(1, 6) clampé à max_sockets
```

## 16.6 Éthéré

```
P(ethereal) = 5% (flat, non-modifiable)
Dégâts/Défense éthéré = base × 1.5
Durabilité éthérée = floor(base/2) - 1
STR/DEX req éthéré = base - 10
```

## 16.7 Downgrade

```
Unique fail → Rare × 3 durabilité
Set fail    → Magic × 2 durabilité
Duplicate unique (nolimit=0) → Rare × 3 durabilité
```

---

## 17. Transposition MGE — Système de loot Allumina

### Architecture recommandée

```rust
// Plugin MGE : mge-plugin-loot-engine.v1
// Équivalent de TreasureClassEx.txt + ItemRatio.txt + affixation

pub struct TreasureClass {
    pub name: String,
    pub picks: i32,                    // négatif = garanti
    pub group: Option<u32>,
    pub level: u32,
    pub quality_bonus: QualityBonus,
    pub no_drop: u32,
    pub entries: Vec<TcEntry>,
}

pub struct TcEntry {
    pub target: TcTarget,              // item code ou TC enfant
    pub prob: u32,
}

pub enum TcTarget {
    Item(ItemCode),
    SubTc(String),
}

pub struct QualityBonus {
    pub unique_factor: u32,            // 0-1024
    pub set_factor: u32,
    pub rare_factor: u32,
    pub magic_factor: u32,
}

pub struct ItemRatioConfig {
    pub rows: Vec<ItemRatioRow>,       // 1 par combinaison uber/class_specific
}

pub struct ItemRatioRow {
    pub is_uber: bool,
    pub is_class_specific: bool,
    pub unique: QualityParams,
    pub set: QualityParams,
    pub rare: QualityParams,
    pub magic: QualityParams,
    pub hi_quality: QualityParams,
    pub normal: QualityParams,
}

pub struct QualityParams {
    pub base_chance: u32,
    pub divisor: u32,
    pub min_chance: u32,
}

pub struct AffixPool {
    pub prefixes: Vec<AffixDef>,
    pub suffixes: Vec<AffixDef>,
}

pub struct AffixDef {
    pub id: u32,
    pub name: String,
    pub group: u32,
    pub level: u32,
    pub max_level: u32,
    pub frequency: u32,
    pub rare_eligible: bool,
    pub item_types: Vec<ItemTypeCode>,
    pub exclude_types: Vec<ItemTypeCode>,
    pub mods: Vec<ModDef>,
    pub level_req: u32,
}

// Algorithme principal
pub fn generate_item_quality(
    ilvl: u32,
    qlvl: u32,
    mf: u32,
    ratio: &ItemRatioRow,
    tc_quality: &QualityBonus,
) -> ItemQuality {
    // Test dans l'ordre : Unique → Set → Rare → Magic → Superior → Normal → Low
    for quality in [Unique, Set, Rare, Magic, Superior, Normal] {
        let params = ratio.get_params(quality);
        let factor = get_mf_factor(quality);
        
        let mut chance = (params.base_chance as i64
            - (ilvl as i64 - qlvl as i64) / params.divisor as i64) * 128;
        
        let emf = if factor > 0 {
            mf * factor / (mf + factor)
        } else {
            mf
        };
        
        chance = chance * 100 / (100 + emf as i64);
        
        if chance < params.min_chance as i64 {
            chance = params.min_chance as i64;
        }
        
        let qf = tc_quality.get_factor(quality) as i64;
        let final_chance = chance - (chance * qf / 1024);
        
        if random(0..final_chance) < 128 {
            return quality;
        }
    }
    ItemQuality::LowQuality
}
```

---

## 18. Références (complémentaires au §9)

| Document | Rôle |
|----------|------|
| [PureDiablo — Item Generation](https://www.purediablo.com/diablo-2/item-generation) | Guide complet de génération d'items (base de ce chapitre) |
| [The Amazon Basin — Item Drop Procedure](https://theamazonbasin.com/wiki/index.php?title=Diablo_II_Item_Drop_Procedure) | Procédure de drop complète avec ordre des vérifications |
| [Phrozen Keep — TreasureClassEx.txt](https://d2mods.info/forum/viewtopic.php?t=67310) | Documentation NoDrop et Picks |
| [Phrozen Keep — ItemRatio.txt](https://d2mods.info/forum/kb/viewarticle?a=320) | Guide du fichier ItemRatio.txt |
| [GitHub fabd/diablo2 — ItemRatio.txt](https://github.com/fabd/diablo2/blob/master/code/d2_113_data/ItemRatio.txt) | Données brutes ItemRatio.txt v1.13 |
| [Phrozen Keep — UniqueItems.txt](https://d2mods.info/forum/viewtopic.php?t=38595) | Guide Rarity et nolimit des Uniques |
| [PureDiablo — Magic Find Diminishing Returns](https://www.purediablo.com/diablo-2/magic-find-diminishing-returns) | Formules EMF |
| [Arreat Summit — Items Basics](https://classic.battle.net/diablo2exp/items/basics.shtml) | Référence officielle Blizzard |
| [diablo2.io — Larzuk Calculator](https://diablo2.io/larzuksockets.php) | Calculateur de sockets Larzuk |

---

**Document** : Allumina — Analyse Technique Diablo II pour MGE  
**Version** : 2.1  
**Date** : 2026-02-22  
**Statut** : Document de référence technique  
**Changelog** : v2.1 — Ajout section 9 (Génération procédurale de cartes : seed, DT1/DS1, DRLG, Levels.txt, waypoints) | v2.0 — Ajout sections 10-18 (système complet de génération d'items)
