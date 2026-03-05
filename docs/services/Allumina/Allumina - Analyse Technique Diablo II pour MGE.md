# Allumina â€” Analyse Technique des SystÃ¨mes Diablo II â€” Transposition MGE

## Contexte

Ce document est une **analyse technique exhaustive** des systÃ¨mes mÃ©caniques de Diablo II (2000, Blizzard North), destinÃ©e Ã  servir de rÃ©fÃ©rence pour l'implÃ©mentation d'Allumina sur le moteur MGE (Miyukini Game Engine). L'analyse est fondÃ©e sur le reverse-engineering communautaire (D2MOO, Phrozen Keep, donnÃ©es moddÃ©es), la documentation des fichiers .txt de configuration, et des hypothÃ¨ses argumentÃ©es lorsque l'information est incertaine.

## PortÃ©e / Scope

- **Applicable Ã  :** ImplÃ©mentation moteur, plugins MGE, game design technique Allumina.
- **Audience :** DÃ©veloppement, architecture MGE, game design.
- **Statut :** Document de rÃ©fÃ©rence technique.

### Hors pÃ©rimÃ¨tre

- Lore, narration, historique du jeu.
- Comparaison subjective entre ARPG.
- Design des assets graphiques.

---

# 1. SYSTÃˆME DE DÃ‰PLACEMENT

## 1.1 Structure fondamentale

### Grille isomÃ©trique diagonale

Diablo II utilise une **grille de tuiles orientÃ©e en diagonale** pour crÃ©er sa vue isomÃ©trique. Ce n'est pas de la navigation libre : tout positionnement est ancrÃ© sur une grille de **subtiles**.

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Type de grille** | IsomÃ©trique losange, tuiles diamant |
| **Taille d'une tuile** | 160Ã—80 px (affichage) |
| **Subtile** | 32Ã—16 px (affichage), 5 subtiles par tuile en X et Y |
| **CoordonnÃ©es** | EntiÃ¨res en subtiles, position serveur en subtiles |
| **UnitÃ© de distance** | 1 yard = 1.5 subtiles (24px vertical Ã— 48px horizontal) |
| **Interpolation visuelle** | Oui â€” le client interpole entre les positions serveur Ã  25 FPS |

### Footprint des unitÃ©s

Chaque personnage et la plupart des monstres occupent un **footprint en X** couvrant 5 subtiles :

```
    [W]
  [W][C][W]
    [W]

C = centre (bloquant), W = wings (non bloquant, peuvent chevaucher)
```

Le centre ne peut **jamais** chevaucher celui d'une autre unitÃ© (sauf Teleport sur unitÃ©s alliÃ©es). Les ailes peuvent se superposer, permettant un placement serrÃ©.

### Transposition MGE

| Concept D2 | Ã‰quivalent MGE |
|------------|----------------|
| Position subtile (entiÃ¨re) | `Position2D` (Vec2, px) â€” coordonnÃ©es flottantes MGE |
| Footprint en X | `Hitbox` composant (cercle ou AABB configurable) |
| Interpolation client | Interpolation native via `Velocity2D` et `LocomotionParams` |
| Grille subtile | Optionnel â€” MGE peut utiliser une grille logique superposÃ©e ou du positionnement libre |

**Recommandation MGE :** utiliser des coordonnÃ©es flottantes (Vec2) avec une grille logique optionnelle pour le pathfinding. Le footprint en X est modÃ©lisable par un cercle de rayon configurable dans le composant hitbox.

---

## 1.2 Pathfinding

### Algorithme

Diablo II utilise un **pathfinding basÃ© sur les SubTilesFlags** extraits des fichiers DT1 (textures de tuiles). Chaque subtile possÃ¨de un champ de bits indiquant la passabilitÃ©.

| Aspect | DÃ©tail |
|--------|--------|
| **Algorithme** | A* limitÃ© (portÃ©e courte ~35 subtiles), recalcul frÃ©quent |
| **DonnÃ©es de passabilitÃ©** | Bit fields par subtile, combinÃ©s sur toutes les couches de la carte |
| **Flags distincts** | Joueur-walkable vs mercenaire-walkable vs missile-passable |
| **PNJ statiques** | Chemins prÃ©calculÃ©s dans les fichiers DS1 (paths prÃ©dÃ©finis) |

### Gestion des collisions dynamiques

Les monstres ont une portÃ©e de pathfinding **limitÃ©e**. Quand un monstre ne peut pas atteindre sa cible :
1. Il tente un A* sur ~35 subtiles
2. Si le chemin Ã©choue, il se dÃ©place en ligne droite vers la cible
3. S'il est bloquÃ©, il entre en Ã©tat d'attente puis rÃ©essaie

### Gestion des obstacles destructibles

Les obstacles destructibles (barils, murs fissurÃ©s) sont traitÃ©s comme des tuiles bloquantes jusqu'Ã  destruction, puis le flag de passabilitÃ© est mis Ã  jour.

### Recalcul

Le pathfinding est recalculÃ© Ã  chaque **AI tick** du monstre (contrÃ´lÃ© par `aidel` dans MonStats.txt). Le joueur recalcule Ã  chaque clic ou changement de direction.

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

Le skill `miyukini-deplacement-orientation` dÃ©finit dÃ©jÃ  le pathfinding par waypoints avec A*. La chaÃ®ne de locomotion MGE (input â†’ accel/friction â†’ clamp â†’ displacement â†’ rotation) est directement applicable.

| Concept D2 | Plugin MGE |
|------------|------------|
| SubTilesFlags | Grille de passabilitÃ© `NavigationGrid` (composant) |
| A* limitÃ© | `mge-plugin-pathfinding` avec max_nodes configurable |
| Waypoints DS1 | `waypoints` + `waypoint_index` (dÃ©jÃ  dans MGE) |
| Walk types distincts | `collision_layers` (layer, mask) â€” dÃ©jÃ  dans MGE |

---

## 1.3 Hitbox et collision

### Formes de hitbox

Diablo II utilise **deux systÃ¨mes de hitbox** distincts (dÃ©finis dans MonStats2.txt) :

| SystÃ¨me | Usage | ParamÃ¨tres |
|---------|-------|------------|
| **SizeX/SizeY** | Collision physique (dÃ©placement, blocage) | DiamÃ¨tre en subtiles (1-3), joueur = 2 |
| **htTop/htLeft/htWidth/htHeight** | Hitbox d'attaque / sÃ©lection (graphique) | Rectangle superposÃ©, pivot = animation pivot |

Le champ `NoGfxHitTest` contrÃ´le quel systÃ¨me est utilisÃ© pour la dÃ©tection de collision :
- `0` : utilise SizeX/SizeY (standard)
- `1` : utilise le rectangle superposÃ© (htTop/htLeft/htWidth/htHeight)

### SÃ©paration physique vs attaque

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  htWidth Ã— htHeight           â”‚ â† Hitbox de sÃ©lection/attaque
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”              â”‚
â”‚  â”‚ SizeXÃ—SizeY â”‚              â”‚ â† Hitbox physique (collision)
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### PrioritÃ© de collision

1. **Mur / terrain** : toujours bloquant (subtile flags)
2. **UnitÃ©s vivantes** : bloquent par SizeX/SizeY (centre du footprint X)
3. **Cadavres** : non bloquants
4. **Missiles** : passent Ã  travers les unitÃ©s (sauf CollideType spÃ©cifique)
5. **Pets entre eux** : peuvent se bloquer mutuellement (problÃ¨me connu des nÃ©cromanciens)

### Transposition MGE

```rust
// Composant MGE pour hitbox Ã  la D2
pub struct HitboxD2 {
    pub physical_radius: f32,     // Ã©quivalent SizeX/SizeY
    pub selection_rect: Rect,     // Ã©quivalent htTop/htLeft/htWidth/htHeight
    pub use_gfx_hitbox: bool,     // Ã©quivalent NoGfxHitTest
}
```

Le skill `miyukini-deplacement-orientation` prÃ©voit dÃ©jÃ  `collision_layers (layer, mask)` pour sÃ©parer les couches physiques.

---

## 1.4 Contraintes moteur

### Tick rate

| ParamÃ¨tre | Valeur |
|-----------|--------|
| **Tick rate interne** | **25 FPS** fixe â€” toute la logique de jeu |
| **Plus petite unitÃ© de temps** | 1/256 de seconde (pour les calculs de prÃ©cision) |
| **dt par frame** | 40 ms (1/25) |
| **Animations** | LiÃ©es au tick rate â€” les breakpoints existent car les animations sont des frames discrÃ¨tes Ã  25 FPS |

### Breakpoints (phÃ©nomÃ¨ne D2 spÃ©cifique)

Les amÃ©liorations de vitesse (FCR, IAS, FHR, FBR) ne deviennent effectives que quand elles **retirent une frame d'animation**. Les rÃ©ductions partielles sont arrondies vers le haut â†’ aucun effet.

```
Exemple SorciÃ¨re â€” Faster Cast Rate :
  Base : 13 frames (13 Ã— 40ms = 520ms)
  9% FCR  â†’ 12 frames (480ms) â† premier breakpoint
  20% FCR â†’ 11 frames (440ms)
  ...
  200% FCR â†’ 7 frames (280ms) â† dernier breakpoint
```

**Implication MGE :** le moteur MGE utilisant des coordonnÃ©es flottantes et un dt variable, les breakpoints ne sont **pas nÃ©cessaires**. La vitesse d'animation peut Ãªtre interpolÃ©e de maniÃ¨re continue. Cependant, si Allumina veut reproduire le *feel* D2, un systÃ¨me de breakpoints optionnel peut Ãªtre implÃ©mentÃ© comme un plugin.

### DÃ©synchronisation rÃ©seau

| Aspect | D2 Original |
|--------|-------------|
| **Architecture** | Client-serveur (Battle.net) ou peer-to-peer (TCP/IP) |
| **PrÃ©diction client** | LimitÃ©e â€” le client prÃ©dit le mouvement mais le serveur fait autoritÃ© |
| **Rubber-banding** | FrÃ©quent â€” le joueur Â« snap Â» Ã  sa position serveur en cas de dÃ©sync |
| **NHAM bug** | Next Hit Always Misses â€” dÃ©sync entre animation client et Ã©tat serveur lors d'interruptions |

### Transposition MGE (rÃ©seau)

Allumina utilise le MWS (Miyukini Webway System) avec un modÃ¨le Lobby (hÃ´te = serveur). Le modÃ¨le recommandÃ© :

| Aspect | Recommandation |
|--------|----------------|
| **AutoritÃ©** | HÃ´te du Lobby = serveur autoritaire |
| **PrÃ©diction** | Client prediction avec rÃ©conciliation serveur |
| **Tick rate** | 30 FPS logique (plus fluide que D2, coÃ»t CPU acceptable) |
| **Interpolation** | Client interpole entre Ã©tats serveur reÃ§us |

---

# 2. COMPORTEMENT DES ENTITÃ‰S (IA)

## 2.1 Architecture IA

### ModÃ¨le : Table-Driven FSM (Machine Ã  Ã‰tats Finis pilotÃ© par donnÃ©es)

Diablo II utilise un systÃ¨me hybride :
- **FSM hardcodÃ©e** : Ã©tats prÃ©dÃ©finis dans le code C++ (AiThink.cpp dans D2MOO)
- **Configuration par tables** : MonStats.txt fournit les paramÃ¨tres qui pilotent les transitions et le comportement

Ce n'est **ni** un behavior tree, **ni** un script pur. C'est une FSM dont les transitions sont paramÃ©trÃ©es par des fichiers .txt.

### Ã‰tats typiques (reconstituÃ©s depuis D2MOO et MonStats.txt)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    aggro      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  IDLE   â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’â”‚  CHASE  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜               â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜
     â†‘                         â”‚
     â”‚ no target          in range
     â”‚                         â†“
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”               â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  WANDER â”‚â†â”€â”€leashâ”€â”€â”€â”€â”€â”€ â”‚ ATTACK  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜               â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜
     â†‘                         â”‚
     â”‚                    hit / stun
     â”‚                         â†“
     â”‚                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
     â””â”€â”€recoverâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ â”‚  STUN   â”‚
                          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                               â”‚
                          hp < threshold
                               â†“
                          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                          â”‚  FLEE   â”‚
                          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                               â”‚
                          hp = 0
                               â†“
                          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                          â”‚  DEAD   â”‚
                          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### Hardcoded States (identifiÃ©s dans le code)

| ID | Ã‰tat | Effet |
|----|------|-------|
| 1 | Freeze | ArrÃªt total animations + IA |
| 11 | Cold | VÃ©locitÃ© et attack rate Ã· 2 |
| 13 | Blaze | Ã‰met des missiles en se dÃ©plaÃ§ant |
| 15 | Concentrate | Interrompt les actions sauf si interrupt=1 |
| 22 | Spiderlay | Produit missile #143 en se dÃ©plaÃ§ant |

### ParamÃ¨tres IA clÃ©s (MonStats.txt)

| Colonne | RÃ´le |
|---------|------|
| `AI` | Identifiant de l'IA utilisÃ©e (chaque IA = code C++ spÃ©cifique) |
| `aidel` / `aidel(N)` / `aidel(H)` | DÃ©lai entre les AI ticks (plus bas = plus agressif) |
| `aidist` / `aidist(N)` / `aidist(H)` | Distance d'activation en cells (dÃ©faut : 35 â‰ˆ 1 Ã©cran) |
| `aip1` Ã  `aip8` | ParamÃ¨tres passÃ©s Ã  l'IA (en %, usage dÃ©pend du type d'IA) |
| `threat` | PrioritÃ© de ciblage (plus haut = ciblÃ© en premier par les ennemis) |

### Transposition MGE

```rust
// Plugin MGE : mge-plugin-ai-monster.v1
pub struct MonsterAiState {
    pub current: AiStateId,      // Idle, Chase, Attack, Stun, Flee, Dead
    pub ai_type: AiTypeId,       // RÃ©fÃ©rence vers la table d'IA
    pub ai_delay: u32,           // Frames entre chaque AI tick
    pub ai_dist: f32,            // Distance d'activation (px)
    pub ai_params: [f32; 8],     // aip1-8 equivalent
    pub target: Option<EntityId>,
    pub threat_level: f32,
    pub last_tick: u32,
}
```

---

## 2.2 SystÃ¨me d'aggro

### Distance d'activation

| ParamÃ¨tre | Valeur par dÃ©faut | Source |
|-----------|-------------------|--------|
| **Rayon d'activation** | 35 cells â‰ˆ ~1 Ã©cran complet | `aidist` dans MonStats.txt |
| **Ligne de vue** | Requise pour la plupart des IA (pas de dÃ©tection Ã  travers les murs) |
| **Ligne de vue (vol)** | Monstres volants (`flying=1`) ignorent certains obstacles sol |

### MÃ©moire de cible

- Les monstres conservent leur cible tant qu'elle est **Ã  portÃ©e de poursuite** (pas de timer de mÃ©moire explicite dans les fichiers de config)
- Le **leashing** se produit quand la cible sort de la portÃ©e de pathfinding (~35 cells)
- HypothÃ¨se : la mÃ©moire de cible dure ~3-5 secondes aprÃ¨s perte de ligne de vue (basÃ© sur comportement observÃ©)

### PrioritÃ© des cibles

ContrÃ´lÃ©e par la colonne `threat` de MonStats.txt + paramÃ¨tre `petIgnore` :

| RÃ¨gle | DÃ©tail |
|-------|--------|
| `threat` Ã©levÃ© | CiblÃ© en prioritÃ© (ex : Maggot Eggs Ã  threat=25 â†’ merc les cible d'abord) |
| `petIgnore=1` | Le monstre ignore totalement les invocations et mercenaires â†’ va directement au joueur |
| `primeevil=1` | +300% dÃ©gÃ¢ts contre mercenaires et invocations (Diablo, Baal) |
| Fallback | Cible la plus proche si pas de prioritÃ© |

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
            monster.state = Chase  // retour Ã  poursuite
        
        Flee =>
            move_away_from(monster, monster.target.pos)
            if monster.hp > flee_threshold:
                monster.state = Chase
```

---

## 2.3 Variantes d'IA

### Table des comportements par type

| Type | Comportement | ParticularitÃ©s |
|------|-------------|----------------|
| **Mob standard** | Chase â†’ Attack â†’ Chase, fuite rare | aidel moyen (~8-12), distance standard |
| **Champion** | Plus agressif, mÃªme IA que standard | aidel rÃ©duit, +2 niveaux, HP Ã—3/2.5/2 |
| **Unique** | IA standard + affixes | +3 niveaux, HP Ã—4/3/2, 1-3 affixes selon difficultÃ© |
| **Super Unique** | IA spÃ©cifique ou standard | Niveaux fixÃ©s (boss=1), hcIdx spÃ©cial, skills dÃ©diÃ©s |
| **Boss (Act)** | IA hardcodÃ©e spÃ©cifique | primeevil=1 (+300% dmg vs pets), immunitÃ©s, phases |
| **Ranged** | Maintient distance, fuit si approchÃ© | `rangedtype=1`, IA diffÃ©rente (kite) |
| **Melee** | Charge directe | Pas de multishot, prioritÃ© rapprochement |
| **Spawner** | Reste en arriÃ¨re, produit des monstres | `placespawn=1`, utilise skill Nest/Minion Spawner |
| **Resurrecting** | RelÃ¨ve les morts de son type | Shamans (Fallen), Defilers â€” liÃ© Ã  BaseId |
| **Critter** | Fuit le joueur, non agressif | `critter=1`, `inert=1` |

### IA des Shamans (exemple complexe)

```
Ã‰tat: GUARD (prÃ¨s du camp)
  â†’ Joueur dÃ©tectÃ© dans aidist
  â†’ Transition: FLEE (s'Ã©loigne du joueur)
  â†’ Si alliÃ© mort dÃ©tectÃ© dans le rayon:
    â†’ Transition: RESURRECT (caste S1 sur le cadavre)
  â†’ Si menacÃ© directement:
    â†’ Transition: ATTACK (attaque Ã  distance)
  â†’ Si plus d'alliÃ©s morts:
    â†’ Transition: FLEE
```

---

## 2.4 Gestion des groupes

### Coordination

| MÃ©canisme | DÃ©tail |
|-----------|--------|
| **Packs** | MinGrp/MaxGrp dÃ©finit la taille du groupe Ã  la gÃ©nÃ©ration |
| **Boss + Minions** | SetBoss=1 permet au "chef" de coordonner (ex : ordre de raid pour Scarabs) |
| **BossXfer** | Si le chef meurt, le leadership passe Ã  un de ses minions |
| **Pas de coordination active** | Les mobs d'un mÃªme pack ne communiquent pas leur cible â€” chacun a sa propre boucle IA |

### Leashing

- Distance de leash : ~35 cells (distance d'activation IA)
- Pas de leash hard-reset : le monstre retourne Ã  sa position de spawn en mode Wander
- Le monstre ne se soigne **pas** en retournant (contrairement Ã  des ARPG modernes comme D3)

### Limite de poursuite

- LimitÃ©e par le pathfinding (portÃ©e A* ~35 subtiles)
- Les monstres volants (`flying=1`) ont une portÃ©e de poursuite plus grande (pas bloquÃ©s par obstacles sol)
- `opendoors=1/0` : contrÃ´le si le monstre peut ouvrir les portes (lobotomisation si 0)

### Transposition MGE (groupes)

Voir `docs/Miyukini_Game_Engine/MGE - Pathfinding Collisions - Guide Entites Groupes.md` pour la gestion des groupes MGE (dÃ©jÃ  documentÃ© pour les scÃ©narios musou/RTS).

```rust
pub struct PackLeader {
    pub minion_ids: Vec<EntityId>,
    pub boss_xfer: bool,       // leadership transfÃ©rable
    pub raid_chance: f32,      // % chance d'ordonner un raid (aip5)
}

pub struct PackMember {
    pub leader: Option<EntityId>,
    pub pack_id: u32,
}
```

---

# 3. SYSTÃˆME DE SPAWN

## 3.1 GÃ©nÃ©ration des monstres

### Architecture de la gÃ©nÃ©ration

Le spawn dans D2 est entiÃ¨rement **table-driven** via plusieurs fichiers interconnectÃ©s :

```
Levels.txt          â†’ DÃ©finit quels monstres peuvent spawner dans une zone
  â†“
MonStats.txt        â†’ DÃ©finit les propriÃ©tÃ©s de chaque monstre
  â†“
MonType.txt         â†’ CatÃ©gorie (super-groupe : skeleton, demon, etc.)
  â†“
ActInfo.txt         â†’ ContrÃ´le les monstres errants (wandering)
  â†“
TreasureClass.txt   â†’ ContrÃ´le le loot (sÃ©parÃ© du spawn)
```

### PondÃ©ration probabiliste

Le champ `Rarity` de MonStats.txt contrÃ´le la probabilitÃ© relative de spawn :

```
Exemple : 2 monstres Ã©ligibles pour une zone
  Monster A : Rarity = 10
  Monster B : Rarity = 1
  Total = 11
  
  Chance Monster A = 10/11 = 91%
  Chance Monster B = 1/11 = 9%
  
  Rarity = 0 â†’ jamais sÃ©lectionnÃ© par Levels.txt
```

### Seed et gÃ©nÃ©ration procÃ©durale

| Aspect | DÃ©tail |
|--------|--------|
| **Carte** | GÃ©nÃ©rÃ©e procÃ©duralement Ã  partir d'un seed (stockÃ© dans la save) |
| **Seed** | DÃ©termine le layout des tuiles (DS1 presets combinÃ©s alÃ©atoirement) |
| **Monstres** | PlacÃ©s APRÃˆS la gÃ©nÃ©ration de carte, selon les tables |
| **Sparse populate** | `sparsePopulate` (0-100%) = chance qu'un monstre choisi soit effectivement placÃ© |

### Influence du niveau de zone

| DifficultÃ© | Niveau monstre | Source |
|------------|---------------|--------|
| **Normal** | Fixe (colonne Level de MonStats.txt) | MonStats.txt |
| **Nightmare** | = Area Level de Levels.txt | Levels.txt |
| **Hell** | = Area Level de Levels.txt | Levels.txt |
| **Boss (boss=1)** | Toujours depuis MonStats.txt | MonStats.txt (indÃ©pendant de la zone) |

### Monstres errants (Wandering)

ContrÃ´lÃ©s par ActInfo.txt :

| ParamÃ¨tre | RÃ´le |
|-----------|------|
| `wanderingMonsterPopulateChance` | % chance (0-100) de spawner un monstre errant |
| `wanderingMonsterRegionTotal` | Max de monstres errants simultanÃ©s |
| `wanderingNpcStart/Range` | SÃ©lection alÃ©atoire de la classe de monstre errant |

---

## 3.2 Pack generation

### Taille des groupes

ContrÃ´lÃ©e par 4 colonnes dans MonStats.txt :

| Colonne | RÃ´le |
|---------|------|
| `MinGrp` / `MaxGrp` | Nombre d'unitÃ©s de base spawned ensemble |
| `PartyMin` / `PartyMax` | Nombre de minions (Minion1/Minion2) accompagnant l'unitÃ© |

### Composition

| Type | Composition |
|------|-------------|
| **Pack standard** | HomogÃ¨ne : MinGrp-MaxGrp du mÃªme BaseId |
| **Pack avec minions** | Chef (unitÃ© principale) + PartyMin-PartyMax minions (type Minion1/Minion2) |
| **Champion pack** | 2-4 du mÃªme type, tous Champions (pas de chef) |
| **Unique pack** | 1 Unique + minions de son propre type (ou Minion1/2 si dÃ©fini) |

### Placement

Les monstres sont placÃ©s aux positions disponibles (subtiles passables) autour d'un point de spawn, avec offset `spawnx`/`spawny` pour Ã©viter l'empilement. Le systÃ¨me de collision empÃªche la superposition.

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
| **Respawn en jeu** | NON â€” les monstres tuÃ©s restent morts pour la session |
| **Spawners** | Certaines unitÃ©s (nids, etc.) produisent continuellement de nouveaux monstres â€” ce n'est pas du respawn mais du spawn dynamique |
| **Reset** | En quittant et recrÃ©ant la partie, la carte est rÃ©gÃ©nÃ©rÃ©e (nouveau seed) et les monstres rÃ©apparaissent |
| **Exception** | Certains monstres sont re-spawnable si un Shaman les ressuscite (morts-vivants bas â†’ relÃ¨vement par morts-vivants hauts) |

### Transposition MGE

Pour Allumina (monde persistant type UO) : le respawn est nÃ©cessaire, contrairement Ã  D2. ImplÃ©menter un timer de respawn par zone avec pondÃ©ration :

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

## 3.4 Spawn des Ã©lites

### HiÃ©rarchie

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                 Super Unique                     â”‚
â”‚  (Noms fixes : Lord De Seis, Rakanishu, etc.)   â”‚
â”‚  Boss=1, niveau fixe, skills dÃ©diÃ©s             â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚              Unique (boss dorÃ©)                  â”‚
â”‚  +3 niveaux, HP Ã—4/3/2, 1-3 affixes            â”‚
â”‚  EntourÃ© de minions de son type                 â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚            Champion (bleu)                       â”‚
â”‚  +2 niveaux, HP Ã—3/2.5/2, variantes            â”‚
â”‚  Pack de 2-4 du mÃªme type                       â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚           Monstre standard (blanc)               â”‚
â”‚  Stats de base, MinGrp-MaxGrp                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### Affixes des Uniques

En gÃ©nÃ©ration, un Unique reÃ§oit des affixes alÃ©atoires :

| DifficultÃ© | Nombre d'affixes |
|------------|-----------------|
| Normal | 1 |
| Nightmare | 2 |
| Hell | 3 |

### Pool d'affixes connu

| Affix | Effet |
|-------|-------|
| **Extra Strong** | +90/75/66% dÃ©gÃ¢ts (N/NM/H) |
| **Extra Fast** | Vitesse de dÃ©placement et d'attaque augmentÃ©e |
| **Cursed** | Applique Amplify Damage aux joueurs touchÃ©s |
| **Fire Enchanted** | DÃ©gÃ¢ts feu ajoutÃ©s, explosion Ã  la mort |
| **Cold Enchanted** | DÃ©gÃ¢ts froid ajoutÃ©s, nova de froid Ã  la mort |
| **Lightning Enchanted** | Charged Bolts Ã©mis quand frappÃ© et Ã  la mort |
| **Spectral Hit** | DÃ©gÃ¢ts alÃ©atoires (feu/froid/foudre/poison) |
| **Stone Skin** | +80% rÃ©sistance physique, -50% vitesse |
| **Multishot** | Tire plusieurs projectiles (ranged seulement) |
| **Aura Enchanted** | Aura alÃ©atoire (Might, Holy Fire, Conviction, Fanaticism) |
| **Mana Burn** | Drain de mana massif |
| **Teleportation** | Se tÃ©lÃ©porte alÃ©atoirement |
| **Magic Resistant** | +20/40/60% toutes rÃ©sistances (N/NM/H) |
| **Conviction** | Aura rÃ©duisant les rÃ©sistances des joueurs |

### RÃ¨gles de combinaison

- Les affixes de rÃ©sistance ne peuvent pas crÃ©er une **3e immunitÃ©** ni augmenter une immunitÃ© existante
- Certaines combinaisons sont interdites implicitement (pas de Fire Enchanted + Cold Enchanted â€” vÃ©rifiÃ© expÃ©rimentalement)
- L'aura est choisie parmi : Might, Holy Fire, Blessed Aim, Holy Freeze, Holy Shock, Conviction, Fanaticism

### Champions : variantes

| Variante | HP | DÃ©gÃ¢ts | SpÃ©cial |
|----------|-----|--------|---------|
| **Standard** | Ã—3/2.5/2 | +90/75/66% | â€” |
| **Berserker** | Ã—0.75 (du champion) | +270/225/198% | Glass cannon |
| **Fanatic** | Ã—3/2.5/2 | +90/75/66% | Similaire standard |
| **Ghostly** | Ã—3/2.5/2 | â€” | 80% rÃ©sistance physique, +33-50% cold dmg |
| **Possessed** | Ã—6 (du champion) | Standard | Immune aux malÃ©dictions |

### Pseudo-algorithme de gÃ©nÃ©ration d'Ã©lite

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

# 4. SYSTÃˆME DE PROJECTILES

## 4.1 ModÃ¨le de projectile

### Architecture : EntitÃ©s physiques (pas de hitscan)

Diablo II n'utilise **pas** de hitscan. Tous les projectiles sont des **entitÃ©s discrÃ¨tes** qui se dÃ©placent sur la carte frame par frame.

| ParamÃ¨tre | Source | DÃ©tail |
|-----------|--------|--------|
| **Vel** | Missiles.txt | Vitesse initiale (pixels/frame Ã  25 FPS) |
| **MaxVel** | Missiles.txt | Vitesse maximale |
| **Accel** | Missiles.txt | AccÃ©lÃ©ration par frame |
| **Range** | Missiles.txt | DurÃ©e de vie en frames |
| **LevRange** | Missiles.txt | Bonus de range par niveau |
| **VelLev** | Missiles.txt | Bonus de vitesse par niveau |
| **Size** | Missiles.txt | Rayon de collision en subtiles |

### Fonctions serveur/client sÃ©parÃ©es

Le systÃ¨me de missiles sÃ©pare **strictement** le client et le serveur :

| Fonction | Client | Serveur |
|----------|--------|---------|
| **DoFunc** (mouvement) | `pCltDoFunc` â€” graphisme, effets visuels | `pSrvDoFunc` â€” logique, collision |
| **HitFunc** (impact) | `pCltHitFunc` â€” particules, son | `pSrvHitFunc` â€” dÃ©gÃ¢ts, effets |
| **DmgFunc** | â€” | `pSrvDmgFunc` â€” modifie les dÃ©gÃ¢ts avant calcul |

Les fonctions client et serveur **doivent** Ãªtre synchronisÃ©es pour Ã©viter les dÃ©sync.

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
| 0 | Pas de collision â€” traverse tout jusqu'Ã  expiration |
| 1 | Collide joueurs uniquement (bug : ne collide pas les monstres malgrÃ© l'intention) |
| 2 | Collide monstres ennemis uniquement |
| 3 | Collide monstres + joueurs |
| 4 | Expire toujours (missile fantÃ´me) |
| 5 | Copie du type 2 |
| 6 | Collide murs uniquement (impacts verticaux) |
| 7 | Collide missiles destructibles (dÃ©prÃ©ciÃ©) |
| 8 | Collide monstres/joueurs + bloquÃ© par terrain |

### DÃ©tection : par tick (pas continue)

La collision est vÃ©rifiÃ©e **Ã  chaque frame** (25 FPS), pas en continu. Un projectile trÃ¨s rapide peut donc **traverser** une cible fine (problÃ¨me de tunneling).

### Collision murale

Un missile est dÃ©truit s'il finit sa frame sur une subtile ayant des **wall bits** activÃ©s. La vÃ©rification est sur la position finale, pas sur le trajet.

### Piercing

| ParamÃ¨tre | Effet |
|-----------|-------|
| `Pierce=1` | Le missile continue aprÃ¨s collision (traverse les ennemis) |
| `CollideKill=1` | Le missile est dÃ©truit aprÃ¨s collision |
| `LastCollide=1` | Le missile s'arrÃªte aprÃ¨s collision |
| `NextHit/NextDelay` | Multi-hit avec dÃ©lai entre chaque impact |

### MÃ©moire anti-spam

Les missiles se souviennent de leur **derniÃ¨re cible** et ne la refrappent pas immÃ©diatement. Cela permet Ã  Fissure de multi-hit quand plusieurs monstres sont sur la mÃªme tile.

```rust
pub struct MissileState {
    pub last_hit_entity: Option<EntityId>,
    pub next_hit_delay: u32,
    pub pierce: bool,
    pub collide_type: CollideType,
}
```

---

## 4.3 Sorts spÃ©ciaux

### Projectiles guidÃ©s (homing)

Les missiles guidÃ©s utilisent une **Move Function spÃ©ciale** qui ajuste la direction vers la cible Ã  chaque frame :

```
fn missile_move_guided(missile, dt):
    if missile.target.is_alive():
        desired_dir = (missile.target.pos - missile.pos).normalize()
        missile.direction = lerp(missile.direction, desired_dir, missile.turn_rate)
    missile.pos += missile.direction * missile.vel
    missile.range -= 1
```

Exemples : Guided Arrow, Bone Spirit.

### AoE Ã  l'impact

Quand `pSrvHitFunc` est dÃ©clenchÃ©, certains missiles spawn un **sous-missile** AoE :

```
fn on_hit_aoe(missile, hit_pos):
    spawn_missile(
        type = missile.sub_missile,
        pos = hit_pos,
        collide_type = 3,  // touche tout
        range = 1,         // instantanÃ©
        size = aoe_radius
    )
```

Exemples : Fireball (explosion), Frozen Orb (nova de froid).

### Effets persistants

| Sort | MÃ©canisme |
|------|-----------|
| **Firewall** | SÃ©rie de missiles statiques alignÃ©s sur la grille, chacun applique des dÃ©gÃ¢ts par tick |
| **Blizzard** | Missiles tombant Ã  des positions alÃ©atoires dans une zone |
| **Poison Nova** | Ring de missiles partant dans toutes les directions |
| **Meteor** | Missile invisible descendant + AoE persistant au sol (Molten Boulder) |

### Fissure (cas d'Ã©tude technique intÃ©ressant)

Fissure Ã©met des missiles le long de la grille. Quand plusieurs monstres chevauchent la mÃªme subtile, chaque missile peut frapper un monstre diffÃ©rent grÃ¢ce Ã  la mÃ©moire anti-spam, causant des dÃ©gÃ¢ts massifs.

---

## 4.4 Synchronisation rÃ©seau

| Aspect | ImplÃ©mentation D2 |
|--------|-------------------|
| **AutoritÃ©** | Serveur autoritaire pour les dÃ©gÃ¢ts et les collisions |
| **Client** | Affiche les missiles localement (interpolation graphique) |
| **DÃ©sync possible** | Si le client et le serveur divergent sur la position d'une cible, le missile client peut Â« rater Â» visuellement alors que le serveur a enregistrÃ© un hit (ou l'inverse) |
| **pCltDoFunc vs pSrvDoFunc** | Les deux doivent Ãªtre cohÃ©rents sinon dÃ©sync visuelle |

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
    pub turn_rate: f32,            // pour missiles guidÃ©s
}

pub enum CollideType {
    None,           // traverse tout
    EnemyOnly,      // D2 type 2
    AllUnits,       // D2 type 3
    WallsOnly,      // D2 type 6
    UnitsAndWalls,  // D2 type 8
}
```

### CoÃ»t CPU estimÃ© (projectiles)

| OpÃ©ration | CoÃ»t par frame |
|-----------|---------------|
| DÃ©placement (par missile) | O(1) â€” simple addition vectorielle |
| Collision broadphase | O(nÃ—m) naÃ¯f, O(n log n) avec spatial hash |
| Collision narrowphase | O(1) â€” test cercle/subtile |
| Total (100 missiles, 200 monstres) | ~0.5ms avec spatial hash |

**Recommandation MGE :** utiliser un spatial hash (grille de cellules) pour la broadphase. La grille de passabilitÃ© MGE peut servir de base.

---

# 5. FOLLOWERS / MERCENAIRES / INVOCATIONS

## 5.1 Architecture comportementale

### IA simplifiÃ©e

Les mercenaires et invocations utilisent des **IA similaires aux monstres** mais avec des prioritÃ©s diffÃ©rentes :

| EntitÃ© | IA | ParticularitÃ©s |
|--------|-----|----------------|
| **Mercenaire** | Propre IA (melee ou ranged selon acte) | Suit le joueur, engage les ennemis proches |
| **Squelettes** | IA similaire mercenaire | Poursuivent ennemis visibles, restent prÃ¨s du joueur |
| **Golems** | IA spÃ©ciale par type | 1 seul actif Ã  la fois |
| **Revives** | IA originale du monstre | Conservent leurs attaques spÃ©ciales, comportement moins prÃ©visible |

### PrioritÃ© joueur

```
fn follower_ai_think(follower, owner, world):
    distance_to_owner = distance(follower.pos, owner.pos)
    
    // PrioritÃ© 1 : rester prÃ¨s du joueur
    if distance_to_owner > MAX_FOLLOW_DIST:
        pathfind_towards(follower, owner.pos)
        return
    
    // PrioritÃ© 2 : mÃªme cible que le joueur (si visible)
    if owner.target.is_some() and can_see(follower, owner.target):
        follower.target = owner.target
        engage(follower)
        return
    
    // PrioritÃ© 3 : ennemi le plus proche
    nearest = find_nearest_enemy(follower.pos, FOLLOWER_AGGRO_RANGE)
    if nearest.is_some():
        follower.target = nearest
        engage(follower)
        return
    
    // PrioritÃ© 4 : suivre le joueur
    if distance_to_owner > FOLLOW_THRESHOLD:
        pathfind_towards(follower, owner.pos)
    else:
        idle(follower)
```

---

## 5.2 TÃ©lÃ©portation automatique

### Conditions de tÃ©lÃ©portation

| Condition | Comportement |
|-----------|-------------|
| **Distance excessive** | Si le follower est trop loin du joueur (hors Ã©cran + marge), tÃ©lÃ©portation invisible vers un point proche du joueur |
| **Changement de zone** | Le follower est instantanÃ©ment repositionnÃ© dans la nouvelle zone |
| **Joueur utilise Teleport** | Le mercenaire et les invocations sont tÃ©lÃ©portÃ©s au point d'arrivÃ©e |
| **Stuck** | Pas de tÃ©lÃ©portation automatique de dÃ©blocage â€” le joueur doit se dÃ©placer ou utiliser Teleport |

### Distance de tÃ©lÃ©port estimÃ©e

| Seuil | Valeur estimÃ©e |
|-------|---------------|
| **Distance de suivi normal** | ~10-15 subtiles |
| **Distance de tÃ©lÃ©portation** | >40-50 subtiles (environ 1.5 Ã©cran) |

### ProblÃ¨mes connus

- **Arcane Sanctuary** : terrain complexe â†’ followers se bloquent frÃ©quemment
- **Pathfinding limitÃ©** : les mercenaires melee souffrent plus que les ranged dans les couloirs Ã©troits
- Pas de commande "rappel" â€” les workarounds sont : fuir pour les faire suivre, ou utiliser Teleport

---

## 5.3 SÃ©lection de cible

### PrioritÃ© de ciblage des followers

| PrioritÃ© | CritÃ¨re |
|----------|---------|
| 1 | Cible actuellement attaquÃ©e par le joueur (si visible) |
| 2 | Ennemi le plus proche du follower |
| 3 | Ennemi le plus proche du joueur |
| 4 | Comportement propre (Revives conservent l'IA du monstre original) |

### Cas spÃ©ciaux

- **Revives** : utilisent l'IA originale du monstre â†’ peuvent s'Ã©loigner, utiliser des attaques spÃ©ciales, et sont moins obÃ©issants
- **Golems** : IA agressive (Clay = lent mais tanky, Iron = thorns, Fire = charge)
- **Mercenaire Act 2 avec aura** : reste en formation â†’ l'aura bÃ©nÃ©ficie au joueur et aux autres suivants

---

## 5.4 Pathfinding spÃ©cifique

### DiffÃ©rences avec le pathfinding monstre

| Aspect | Monstres | Followers |
|--------|----------|-----------|
| **Walk flags** | Player-walkable OU monster-walkable | Mercenary-walkable (flags distincts dans les subtiles) |
| **Collision pets** | â€” | Les pets se bloquent entre eux (SizeX/SizeY) |
| **Collision avec autres monstres** | Oui | Oui (pas d'immunitÃ©) |
| **Passage portes** | Configurable (`opendoors`) | Suivent le joueur (passent les portes ouvertes) |

### InTown et collision

D'aprÃ¨s MonStats.txt : `InTown` contrÃ´le si les pets ont une collision en ville :
- **Singleplayer** : collision activable/dÃ©sactivable
- **Multiplayer** : collision toujours dÃ©sactivÃ©e pour les pets en ville

### Transposition MGE

```rust
pub struct Follower {
    pub owner: EntityId,
    pub follow_distance: f32,        // distance idÃ©ale au joueur
    pub teleport_threshold: f32,     // distance de tÃ©lÃ©portation auto
    pub targeting_mode: TargetingMode,
    pub retain_original_ai: bool,    // pour les Revives
}

pub enum TargetingMode {
    FollowOwnerTarget,   // prioritÃ© cible du joueur
    NearestEnemy,        // cible la plus proche
    OriginalAi,          // IA du monstre original (Revives)
}
```

---

# 6. ANALYSE MOTEUR SOUS-JACENT

## 6.1 Architecture probable du moteur original

| Couche | Technologie |
|--------|-------------|
| **Langage** | C/C++ (confirmÃ© par D2MOO) |
| **Rendering** | DirectDraw (2D sprites), rÃ©solution 640Ã—480 (800Ã—600 en LoD) |
| **Game loop** | Fixed timestep Ã  25 FPS |
| **DonnÃ©es** | Fichiers .txt (TSV) chargÃ©s en RAM â†’ tables indexÃ©es par hcIdx |
| **Assets** | Formats propriÃ©taires : DCC/DC6 (sprites), DT1 (tuiles), DS1 (presets carte), COF (animations) |
| **RÃ©seau** | TCP/IP, architecture client-serveur pour Battle.net, peer-to-peer pour LAN |
| **Audio** | DirectSound |

## 6.2 Limites hardware 2000

| Contrainte | Impact |
|------------|--------|
| **CPU** | Pentium II/III ~500MHz â†’ tick rate limitÃ© Ã  25 FPS |
| **RAM** | 64-256 MB â†’ cartes gÃ©nÃ©rÃ©es procÃ©duralement, pas prÃ©-chargÃ©es |
| **GPU** | 2D uniquement (pas de GPU computing) â†’ tout sur CPU |
| **RÃ©seau** | Modems 56k â†’ minimum de donnÃ©es rÃ©seau, pas de streaming |
| **Stockage** | CD-ROM â†’ assets compressÃ©s, streaming minimal |

## 6.3 Pourquoi certains comportements existent (limitations techniques)

| Comportement | Raison technique |
|-------------|-----------------|
| **Breakpoints** | Tick fixe 25 FPS â†’ animations en frames discrÃ¨tes |
| **Pathfinding limitÃ© (35 cells)** | CPU trop faible pour A* longue distance sur 200+ monstres |
| **Monstres bloquÃ©s** | Pas de systÃ¨me de dÃ©blocage automatique (coÃ»t CPU) |
| **Pas de respawn** | RAM insuffisante pour tracker les respawn timers de centaines de monstres |
| **TÃ©lÃ©portation followers** | Solution bon marchÃ© au pathfinding dÃ©faillant |
| **IA table-driven** | Pas assez de CPU pour du behavior tree complexe par entitÃ© |
| **Collision par subtile** | Plus rapide que du calcul flottant point par point |
| **CollideType #1 bug** | Code jamais corrigÃ© car le jeu fonctionne malgrÃ© tout |

## 6.4 Estimation coÃ»t CPU par systÃ¨me

| SystÃ¨me | CoÃ»t estimÃ© (par frame, 2000) | CoÃ»t estimÃ© (2026, MGE) |
|---------|-------------------------------|-------------------------|
| **Pathfinding** (200 mobs) | ~8ms (A* limitÃ©) | ~0.5ms (A* optimisÃ© + spatial hash) |
| **IA** (200 mobs) | ~3ms (FSM simple) | ~0.2ms |
| **Projectiles** (50 actifs) | ~1ms | ~0.05ms |
| **Collision** (globale) | ~4ms | ~0.3ms (broadphase spatial hash) |
| **Rendering** (sprites) | ~15ms | ~2ms (GPU batched) |
| **RÃ©seau** (sync) | ~2ms | ~1ms |
| **Total** | ~33ms (budgeable sur 40ms) | ~4ms (largement sous les 33ms Ã  30 FPS) |

## 6.5 VulnÃ©rabilitÃ©s potentielles du moteur

| VulnÃ©rabilitÃ© | Description | Exploitation connue |
|---------------|-------------|---------------------|
| **Desync client** | Le client prÃ©dit localement â†’ position manipulable | Maphack, teleport hack |
| **Tables .txt modifiables** | Fichiers de configuration en clair â†’ moddable | Modification de stats, rÃ©sistances |
| **Collision par tick** | Projectiles rapides traversent les hitbox | Trivial Ã  reproduire en jeu |
| **Memory editing** | Pas de protection mÃ©moire cÃ´tÃ© client | Duplication d'items, modification de gold |
| **TCP/IP peer-to-peer** | Pas de serveur autoritaire en LAN | Triche libre en LAN |
| **Seed prÃ©dictible** | Le seed de carte est partagÃ© | Cartes prÃ©visibles avec le mÃªme seed |

---

# 7. RÃ‰INTERPRÃ‰TATION MODERNE â€” TRANSPOSITION MGE/ALLUMINA

## 7.1 Architecture recommandÃ©e pour Allumina

| Couche | Choix MGE | Justification |
|--------|-----------|---------------|
| **Game loop** | Fixed timestep 30 FPS logique + rendering dÃ©couplÃ© | Plus fluide que D2 (25 FPS), budget CPU confortable |
| **Positionnement** | Vec2 flottant avec grille logique optionnelle | FlexibilitÃ© + compatibilitÃ© pathfinding |
| **IA** | Table-driven FSM avec composants ECS | MÃªme approche que D2 mais via composants MGE au lieu de fichiers .txt |
| **Projectiles** | EntitÃ©s physiques (pas de hitscan) + spatial hash | Reproduit le feel D2 avec meilleure performance |
| **Collision** | Broadphase spatial hash + narrowphase cercle/AABB | Standard moderne, O(n log n) |
| **RÃ©seau** | Lobby hÃ´te autoritaire + client prediction + rÃ©conciliation | MWS comme transport, LOI-1 respectÃ©e (solo jouable) |
| **DonnÃ©es** | Composants ECS + tables de configuration (RON/JSON) | Ã‰quivalent des .txt D2 mais sÃ©rialisable et typÃ© |

## 7.2 Mapping D2 â†’ Plugins MGE

| SystÃ¨me D2 | Plugin MGE | Composants |
|------------|------------|------------|
| MonStats.txt | `mge-plugin-monster-stats.v1` | `MonsterDef`, `MonsterInstance` |
| MonStats2.txt | `mge-plugin-monster-collision.v1` | `MonsterHitbox`, `MonsterSize` |
| Missiles.txt | `mge-plugin-projectile.v1` | `Projectile`, `ProjectileDef` |
| Levels.txt | `mge-plugin-zone-spawn.v1` | `SpawnZone`, `SpawnTable` |
| Skills.txt | `mge-plugin-skills.v1` | `SkillDef`, `SkillInstance` |
| AI (AiThink.cpp) | `mge-plugin-ai-monster.v1` | `MonsterAiState`, `AiConfig` |
| SuperUniques.txt | `mge-plugin-elite-gen.v1` | `EliteDef`, `AffixPool` |
| TreasureClass.txt | `mge-plugin-loot.v1` | `LootTable`, `TreasureClass` |

## 7.3 DifficultÃ©s principales

| DifficultÃ© | DÃ©tail |
|------------|--------|
| **Reproduire le "feel" D2** | Le tick Ã  25 FPS crÃ©e une sensation spÃ©cifique ; un tick plus rapide sera plus fluide mais diffÃ©rent |
| **Breakpoints optionnels** | Si on veut les breakpoints, il faut un systÃ¨me de quantification des vitesses d'animation |
| **IA table-driven fidÃ¨le** | NÃ©cessite un systÃ¨me de configuration robuste avec 8+ paramÃ¨tres par IA |
| **Collision subtile vs flottante** | Le passage de coordonnÃ©es entiÃ¨res Ã  flottantes change les edge cases de collision |
| **Followers dÃ©cents** | D2 avait un pathfinding mÃ©diocre pour les followers â€” il faut faire mieux sans perdre le feel |
| **Multijoueur souverain** | D2 repose sur Battle.net ; Allumina doit fonctionner via MWS (Lobby P2P) sans serveur central |

## 7.4 PiÃ¨ges Ã  Ã©viter

| PiÃ¨ge | Explication |
|-------|-------------|
| **Copier les bugs de D2** | Les CollideType bugs, le NHAM, les followers bloquÃ©s â†’ ne pas reproduire |
| **Tick rate trop Ã©levÃ©** | 60 FPS logique serait overkill pour un ARPG isomÃ©trique et coÃ»teux en rÃ©seau |
| **Pathfinding global** | A* sur toute la carte est inutile et coÃ»teux â€” garder la portÃ©e limitÃ©e de D2 |
| **IA trop complexe** | Behavior trees par mob = overkill. La FSM table-driven de D2 est suffisante et performante |
| **Oublier le leashing** | Sans leashing, les mobs suivent indÃ©finiment â†’ train de monstres exploit |
| **Collision trop rÃ©aliste** | D2 autorise le chevauchement des "ailes" â†’ ne pas bloquer trop strictement ou les combats deviennent impossibles |
| **NÃ©gliger le spatial hash** | Sans broadphase, la collision de 200+ mobs + 50 projectiles est O(nÂ²) |

## 7.5 Comparaison implicite avec PoE et D3

| Aspect | D2 (2000) | Path of Exile | Diablo 3 | Allumina (cible) |
|--------|-----------|---------------|----------|-------------------|
| **Tick rate** | 25 FPS | 30 FPS serveur | 60 FPS | 30 FPS |
| **Pathfinding** | A* limitÃ© | NavMesh + A* | NavMesh | A* sur grille logique |
| **IA** | FSM table-driven | FSM + scripts | Behavior tree | FSM table-driven (MGE) |
| **Projectiles** | EntitÃ©s physiques | EntitÃ©s physiques | EntitÃ©s + hitscan | EntitÃ©s physiques |
| **Collision** | Subtile grid | Continuous | Capsule + spatial hash | Cercle/AABB + spatial hash |
| **RÃ©seau** | Client-serveur/P2P | Serveur autoritaire | Serveur autoritaire | Lobby autoritaire (MWS) |
| **Ã‰lites** | Affixes simples | Affixes + mods carte | Affixes + Nephalem | Affixes (pool configurable) |
| **Followers** | IA basique, teleport | IA basique | IA basique | IA amÃ©liorÃ©e (chaÃ®ne locomotion MGE) |
| **Troupes** | Mercenaire + summons | Spectres/zombies | 1 follower | Multi-Ã©chelles (Charisme cap) |

---

# 8. TABLES RÃ‰CAPITULATIVES

## 8.1 Constantes fondamentales D2

| Constante | Valeur |
|-----------|--------|
| Tick rate | 25 FPS |
| Frame duration | 40 ms |
| Subtile size (affichage) | 32Ã—16 px |
| Tile size (affichage) | 160Ã—80 px |
| Subtiles par tile | 5Ã—5 |
| 1 yard | 1.5 subtiles = 48Ã—24 px |
| Distance d'activation IA (dÃ©faut) | 35 cells â‰ˆ 1 Ã©cran |
| PortÃ©e pathfinding | ~35 subtiles |
| Player SizeX/SizeY | 2 subtiles |
| Knockback range | 7Ã—7 subtiles (centrÃ©) |
| Regen formule | (REGEN Ã— HP) / 4096 par frame |
| Block cap | 75% |
| Resistance immunitÃ© | â‰¥100% |
| Break immunitÃ© | 5 pts rÃ©sistance rÃ©duite = 1% brisÃ© |

## 8.2 Multiplicateurs d'Ã©lite par difficultÃ©

| Type | HP (N/NM/H) | Niveau bonus | XP bonus |
|------|-------------|-------------|----------|
| Minion | Ã—2 / Ã—1.75 / Ã—1.5 | +3 | Ã—5 |
| Champion | Ã—3 / Ã—2.5 / Ã—2 | +2 | Ã—3 |
| Berserker | Ã—0.75 champion | +2 | Ã—5 |
| Possessed | Ã—6 champion | +2 | Ã—3 |
| Unique | Ã—4 / Ã—3 / Ã—2 | +3 | Ã—5 |

## 8.3 Structure de donnÃ©es recommandÃ©e (MGE)

```rust
// Table de spawn â€” Ã©quivalent Levels.txt + MonStats.txt
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

// DÃ©finition monstre â€” Ã©quivalent MonStats.txt complet
pub struct MonsterDef {
    pub id: u32,
    pub base_id: u32,
    pub ai_type: AiTypeId,
    pub ai_delay: [u32; 3],            // par difficultÃ©
    pub ai_dist: [f32; 3],
    pub ai_params: [[f32; 8]; 3],
    pub threat: f32,
    pub velocity: f32,
    pub run_velocity: f32,
    pub skills: [Option<SkillRef>; 8],
    pub resistances: ResistanceSet,
    pub hp_range: [(u32, u32); 3],      // (min, max) par difficultÃ©
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

# 9. GÃ‰NÃ‰RATION PROCÃ‰DURALE DE CARTES

## 9.1 SystÃ¨me de seed

### Seed global

Diablo II utilise un **seed 32 bits** (entier non signÃ©) comme source de toute la gÃ©nÃ©ration procÃ©durale. Le RNG (Random Number Generator) est un gÃ©nÃ©rateur sÃ©quentiel dÃ©terministe : chaque nombre alÃ©atoire est calculÃ© Ã  partir du prÃ©cÃ©dent, de sorte qu'un mÃªme seed produit toujours le mÃªme flux de valeurs "alÃ©atoires".

| ParamÃ¨tre | DÃ©tail |
|-----------|--------|
| **Taille** | 32 bits (uint32), Little Endian |
| **Stockage** | Offset `0xAB` (171 octets) dans le fichier `.d2s` (save du personnage) |
| **GÃ©nÃ©ration** | Automatique Ã  la crÃ©ation d'une partie (basÃ© sur l'horloge systÃ¨me) |
| **Override** | ParamÃ¨tre CLI `-seed <valeur>` pour forcer un seed spÃ©cifique |
| **Plage** | 0 Ã  4 294 967 295 |

### Ce que le seed contrÃ´le

| Ã‰lÃ©ment | ContrÃ´lÃ© par le seed |
|---------|---------------------|
| **Layout des tuiles** | Oui â€” positions des DS1 dans le monde, choix des variantes |
| **SÃ©lection des presets** | Oui â€” quel DS1 parmi File1-File6 est choisi |
| **Positions des sorties** | Oui â€” emplacements des warps entre zones |
| **DensitÃ© de monstres** | Partiellement â€” le seed affecte les positions de spawn, des seeds trÃ¨s petits peuvent produire une densitÃ© anormalement Ã©levÃ©e |
| **Types de monstres** | Indirectement â€” la sÃ©lection alÃ©atoire parmi M1-M25 dÃ©pend du flux RNG |
| **Drops d'items** | Non â€” les drops sont calculÃ©s indÃ©pendamment au moment du kill |
| **Waypoints** | Oui â€” position dans la zone (sauf certains waypoints fixes) |
| **Objets de quÃªte** | Oui â€” position dans le preset DS1 dÃ©signÃ© |

### Multijoueur et partage de seed

| Aspect | Comportement |
|--------|-------------|
| **CrÃ©ation de partie** | Le crÃ©ateur de la partie gÃ©nÃ¨re le seed ; tous les joueurs utilisent le mÃªme seed |
| **Persistance** | Le seed est Ã©crit dans le `.d2s` de chaque joueur qui rejoint la partie |
| **Partie permanente** | Le seed reste identique tant que la partie existe sur le serveur (realm) |
| **Rejointure** | Un joueur qui rejoint retrouve la mÃªme carte (mÃªme seed) |
| **Nouvelle partie** | Un nouveau seed est gÃ©nÃ©rÃ© Ã  chaque crÃ©ation de partie |

### Structure de la chaÃ®ne DRLG

```
Game Seed (32-bit)
  â””â”€â†’ Act Seed (dÃ©rivÃ©)
       â””â”€â†’ Level Seed (dÃ©rivÃ© par level ID)
            â””â”€â†’ Room Seeds (dÃ©rivÃ©s sÃ©quentiellement)
                 â””â”€â†’ Sub-element placement (monstres, objets, etc.)
```

Chaque niveau dans un acte reÃ§oit un seed dÃ©rivÃ© du seed de l'acte. Cela garantit que la modification d'un niveau n'affecte pas la gÃ©nÃ©ration des autres niveaux du mÃªme acte.

### Structures DRLG internes (reverse-engineered)

```c
// D2DrlgActStrc â€” reprÃ©sente un acte complet
struct D2DrlgActStrc {
    D2RoomStrc* pRoom;           // liste chaÃ®nÃ©e de rooms
    uint32_t    dwSeed;          // seed de l'acte
    D2DrlgDataStrc* pDrlgData;   // donnÃ©es de gÃ©nÃ©ration
};

// D2DrlgDataStrc â€” donnÃ©es de gÃ©nÃ©ration alÃ©atoire
struct D2DrlgDataStrc {
    uint32_t dwSeed;             // seed courant (Ã©tat RNG)
    uint32_t dwRoomCount;        // nombre de rooms gÃ©nÃ©rÃ©es
};

// D2DrlgLevelStrc â€” un niveau individuel
struct D2DrlgLevelStrc {
    uint32_t dwLevelType;        // type de niveau
    uint32_t dwSeed;             // seed du niveau
    uint32_t dwSizeX, dwSizeY;   // dimensions
};
```

---

## 9.2 Types de gÃ©nÃ©ration (DrlgType)

Diablo II utilise **trois algorithmes de gÃ©nÃ©ration distincts**, sÃ©lectionnÃ©s par la colonne `DrlgType` dans `Levels.txt` :

| DrlgType | Nom | Usage | Exemples |
|----------|-----|-------|----------|
| **1** | Random Maze | Donjons composÃ©s de rooms assemblÃ©es | Caves, Cryptes, Arcane Sanctuary, Maggot Lair |
| **2** | Preset | Carte fixe (un seul DS1) | Catacombes Niv. 4, Pandemonium Fortress, Villes |
| **3** | Random Wilderness | Zones extÃ©rieures de taille fixe | Blood Moor, Stony Field, dÃ©serts Acte 2 |

### DrlgType 1 â€” Random Maze (Donjons)

Le systÃ¨me de labyrinthe assemble des **rooms individuelles** (chacune Ã©tant un fichier DS1) en un rÃ©seau connectÃ©.

**Fichier de contrÃ´le : `LvlMaze.txt`**

| Colonne | RÃ´le |
|---------|------|
| `Rooms` | Nombre **minimum** de DS1 composant le labyrinthe |
| `SizeX` / `SizeY` | CoordonnÃ©es du coin infÃ©rieur-droit de chaque room (en tiles, base 0) |
| `Merge` | ContrÃ´le la fusion de certaines rooms |

**Algorithme de gÃ©nÃ©ration des labyrinthes :**

```
fn generate_maze(level, seed):
    rng = init_rng(seed)
    grid = empty_grid(level.max_size)
    room_count = 0
    
    // 1. Placer la room d'entrÃ©e
    start_pos = get_start_position(level)  // centre ou bord selon LevelType
    grid[start_pos] = ENTRY_ROOM
    room_count += 1
    
    // 2. Expansion par croissance
    while room_count < level.min_rooms:
        // SÃ©lectionner une room existante avec un cÃ´tÃ© libre
        source = pick_room_with_free_edge(grid, rng)
        direction = pick_free_direction(source, rng)  // N, S, E, W
        new_pos = source.pos + direction
        
        if is_valid_position(new_pos, grid):
            grid[new_pos] = ROOM
            add_connection(source, new_pos, direction)
            room_count += 1
    
    // 3. Placer la room spÃ©ciale (boss, quÃªte) Ã  l'extrÃ©mitÃ©
    endpoint = find_deepest_leaf(grid)
    grid[endpoint] = SPECIAL_ROOM
    
    // 4. RÃ©soudre les types de DS1 par ouvertures
    for each cell in grid:
        openings = compute_openings(cell)  // bitmask NSEW
        cell.ds1_type = openings           // index 1-15 dans LvlPrest.txt
        cell.ds1_file = pick_variant(level.presets[openings], rng)
```

**Convention de nommage des DS1 de labyrinthe :**

Les DS1 sont nommÃ©s selon leurs ouvertures, encodÃ©es sur 4 bits (N=8, S=4, E=2, W=1) :

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

Chaque type de DS1 dans LvlPrest.txt peut avoir jusqu'Ã  **6 variantes** (colonnes File1-File6), sÃ©lectionnÃ©es alÃ©atoirement pour apporter de la variÃ©tÃ© visuelle.

**Contraintes de connexion entre rooms :**

- Les sorties de deux rooms adjacentes **doivent** Ãªtre aux mÃªmes coordonnÃ©es relatives pour assurer la continuitÃ© du passage
- La largeur de passage est de 1+ tuile (minimum 4-5 subtiles pour le joueur)
- Le labyrinthe doit tenir dans les limites `SizeX Ã— SizeY` de `Levels.txt`
- Certains LevelTypes imposent le placement de l'entrÃ©e (centre pour Arcane Sanctuary, bord pour la plupart des caves)

**Exemple concret â€” Den of Evil avec Rooms=6 :**

```
caveSpre2.ds1     â† Room d'entrÃ©e (depuis la surface)
  (DEF 85)
     |
caveNE2.ds1 â”€â”€â”€ caveSW.ds1
  (DEF 62)       (DEF 57)
                    |
caveSE2.ds1 â”€â”€â”€ caveNW2.ds1
  (DEF 58)       (DEF 61)
     |
caveNE.ds1  â”€â”€â”€ caveWspec.ds1    â† Corpsefire (room spÃ©ciale)
  (DEF 62)       (DEF 95)
```

7 DS1 (25Ã—25 tuiles chacun) pour un Rooms=6, car le DRLG peut ajouter des rooms supplÃ©mentaires pour satisfaire les contraintes de connexion.

### DrlgType 2 â€” Preset (Carte fixe)

Les niveaux preset sont des **DS1 uniques non randomisÃ©s** (le layout est toujours identique). Cependant, `LvlPrest.txt` offre une randomisation limitÃ©e :

- **File1 Ã  File6** : jusqu'Ã  6 variantes DS1 d'un mÃªme preset, sÃ©lectionnÃ©es alÃ©atoirement par le seed
- **Populate** : contrÃ´le si les monstres sont placÃ©s alÃ©atoirement dans la zone
- La taille du DS1 correspond exactement Ã  `SizeX Ã— SizeY` de `Levels.txt`

Exemples : Catacombes Niveau 4 (Andariel), Pandemonium Fortress, les villes.

### DrlgType 3 â€” Random Wilderness (Zones extÃ©rieures)

La gÃ©nÃ©ration des zones extÃ©rieures (Acte 1 wilderness, Acte 2 dÃ©serts) est la plus complexe, composÃ©e de **4 Ã©tapes sÃ©quentielles** :

**Ã‰tape 1 â€” Bordures et sorties**

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

Les bordures sont composÃ©es de DS1 simples :
- `BordX.ds1` : bordures normales (arbres + mur de pierre)
- `StnClfX.ds1` : falaises (Stony Field, Dark Woods)
- `XRiverX.ds1` : riviÃ¨res en bordure

**Ã‰tape 2 â€” Chemins, waypoints et warps internes**

- Les **waypoints** sont placÃ©s alÃ©atoirement dans la zone (sauf exceptions comme Cold Plains)
- Les **chemins** sont gÃ©nÃ©rÃ©s dynamiquement (pas de DS1 pour les Ã©lÃ©ments de chemin) â€” l'algorithme relie les entrÃ©es, caves, ponts entre eux
- Les **warps internes** (entrÃ©es de caves, Tour de la Comtesse) sont positionnÃ©s

**Ã‰tape 3 â€” Placement des DS1 thÃ©matiques**

```
fn place_themed_presets(level, rng):
    // DS1 thÃ©matiques placÃ©s alÃ©atoirement dans les espaces vides
    for preset in level.themed_presets:
        pos = find_empty_area(level.grid, preset.size, rng)
        place_ds1(level.grid, preset, pos)
```

Exemples de DS1 thÃ©matiques :
- `circle.ds1` : cercle de petites pierres
- `arrow.ds1` : flÃ¨che en pierres
- `pond1.ds1` : Ã©tang
- Camps de monstres (Bishibosh dans Cold Plains)
- Objets de quÃªte (Cairn Stones dans Stony Field, Inifuss Tree dans Dark Woods)

Le nombre de presets thÃ©matiques est **fixe par zone** (probablement hardcodÃ© ou dans une table DLL). Chaque zone contient toujours : quelques blocs bordÃ©s, 1-2 maisons, parfois une maison en feu, 5 shrines.

**Ã‰tape 4 â€” Remplissage par objets alÃ©atoires (LvlSub.txt)**

```
fn fill_with_random_objects(level, rng):
    // Le jeu extrait des Ã©lÃ©ments INDIVIDUELS depuis les DS1 "filler"
    for filler_ds1 in level.sub_theme_fillers:
        elements = extract_individual_elements(filler_ds1)
        for element in elements:
            if rng.roll() < element.probability:
                pos = find_empty_position(level.grid, rng)
                place_element(level.grid, element, pos)
    
    // Combler les espaces restants avec le sol standard (herbe, sable...)
    fill_remaining_with_base_floor(level.grid)
```

Les DS1 "filler" sont des fichiers Ã©tranges qui ressemblent Ã  des planches de conception :
- `stone.ds1` : collection de formations rocheuses individuelles
- `trees.ds1` : collection d'arbres individuels
- `swamp.ds1` / `swamp2.ds1` : Ã©lÃ©ments marÃ©cageux
- `pud.ds1` : flaques et mares

Le jeu **extrait des Ã©lÃ©ments individuels** de ces DS1 et les place sÃ©parÃ©ment sur la carte. La colonne `SubTheme` de `Levels.txt` contrÃ´le quels fillers sont utilisÃ©s (le Dark Wood utilise plus d'arbres, le Blood Moor pas de marÃ©cages).

Ces DS1 sont rÃ©fÃ©rencÃ©s dans `LvlSub.txt` (pas `LvlPrest.txt`), qui fonctionne de maniÃ¨re similaire Ã  `TCex.txt` avec des colonnes `Prob` (probabilitÃ©), `Trials` (picks) et un maximum.

---

## 9.3 SystÃ¨me de tuiles â€” Format DT1

### Vue d'ensemble

Les fichiers DT1 (Diablo Tile 1) contiennent toutes les **tuiles graphiques** utilisÃ©es pour les sols, murs, ombres et toits des cartes. Il y a 256 fichiers DT1 dans le jeu (~157 MB total), organisÃ©s par acte et thÃ¨me dans `Data\Global\Tiles\Act{N}\`.

Un mÃªme DT1 est partagÃ© par **plusieurs cartes** (les murs de pierre du Rogue Encampment apparaissent aussi dans Cold Plains, Stony Field, Tristram).

### Structure du fichier DT1

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ File Header (276 octets)        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ Tile Header #0 (96 octets)     â”‚
â”‚ Tile Header #1 (96 octets)     â”‚
â”‚ ...                             â”‚
â”‚ Tile Header #N (96 octets)     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ Tile Data #0                    â”‚
â”‚   â”œâ”€ Block Headers (20 oct/blk)â”‚
â”‚   â””â”€ Block Data (pixels)       â”‚
â”‚ Tile Data #1                    â”‚
â”‚ ...                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**File Header (276 octets) :**

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 4 | Version 1 (= 7) |
| 0x04 | 4 | Version 2 (= 6) |
| 0x08 | 260 | RÃ©servÃ© (tout Ã  zÃ©ro) |
| 0x10C | 4 | Nombre de tuiles |
| 0x110 | 4 | Pointeur vers les Tile Headers (= 0x114) |

**Tile Header (96 octets) :**

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 4 | Direction (orientation gÃ©nÃ©rale, 1-5) |
| 0x04 | 2 | Hauteur de toit (pixels au-dessus du sol) |
| 0x06 | 1 | Index sonore (bois, pierre, boue, etc.) |
| 0x07 | 1 | Flag animÃ© (bit 0 = tuile animÃ©e) |
| 0x08 | 4 | Hauteur (pixels, toujours nÃ©gatif, puissance de 32) |
| 0x0C | 4 | Largeur (pixels, max 160, puissance de 32) |
| 0x10 | 4 | ZÃ©ros |
| 0x14 | 4 | **Orientation** (type de tuile, 0-19) |
| 0x18 | 4 | **Main Index** (0-63) |
| 0x1C | 4 | **Sub Index** (0-63) |
| 0x20 | 4 | Rarity / Frame index |
| 0x24 | 4 | Unknown 1-4 (mÃªme valeur pour toutes les tuiles d'un DT1) |
| 0x28 | 25 | **Sub-tile flags** (passabilitÃ©, 5Ã—5 = 25 subtiles) |
| 0x41 | 7 | ZÃ©ros |
| 0x48 | 4 | Pointeur vers les Block Headers |
| 0x4C | 4 | Taille totale des Block Headers + Block Data |
| 0x50 | 4 | Nombre de blocs |
| 0x54 | 12 | ZÃ©ros |

### Identification des tuiles : 3 index

Chaque tuile est identifiÃ©e de maniÃ¨re unique par la combinaison **(Orientation, Main Index, Sub Index)** :

- **Orientation** (0-19) : dÃ©termine le type de tuile
- **Main Index** (0-63) : identifiant principal dans le tileset
- **Sub Index** (0-63) : sous-variante

Soit jusqu'Ã  64 Ã— 64 = **4096 tuiles distinctes** par orientation.

### Types de tuiles (Orientation)

| Orientation | Type | Rendu |
|-------------|------|-------|
| 0 | **Sol** (statique ou animÃ©) | DessinÃ© en premier |
| 1 | Mur gauche | |
| 2 | Mur droit (supÃ©rieur) | |
| 3 | Partie droite du coin nord | |
| 4 | Partie gauche du coin nord | |
| 5 | Coin supÃ©rieur-droit | |
| 6 | Coin infÃ©rieur-gauche | |
| 7 | Coin infÃ©rieur-droit | |
| 8 | Mur gauche avec porte | |
| 9 | Mur droit avec porte | |
| 10-11 | **Tuiles spÃ©ciales** (warps, TP, entrÃ©es) | |
| 12 | Piliers, colonnes, objets autonomes | |
| 13 | **Ombres** | |
| 14 | **Arbres** (objets avec ombre prÃ©cÃ©dente) | |
| 15 | **Toits** | Au-dessus du sol (roof_y dans le .ini) |
| 16-19 | **Murs bas** (Ã©quivalents de 1, 2, 3/4, 7) | DessinÃ©s sous les sols |

### Direction et Ã©clairage

La **Direction** (1-9) contrÃ´le comment la lumiÃ¨re affecte la tuile. Chaque Direction doit Ãªtre associÃ©e aux bonnes Orientations :

| Direction | Orientations compatibles |
|-----------|------------------------|
| 1 | 1, 5, 8 |
| 2 | 2, 6, 9 |
| 3 | 0, 3, 4, 12, 14 |
| 4 | 7 |
| 5 | 15 (toits) |
| 6-9 | 16-19 (murs bas) |

### Sub-tile flags (passabilitÃ©)

Chaque tuile possÃ¨de **25 flags** (grille 5Ã—5) dÃ©finissant la passabilitÃ© subtile par subtile. L'ordre est gaucheâ†’droite, basâ†’haut :

| Bit | Effet |
|-----|-------|
| 0 | **Block walk** (bloque le dÃ©placement piÃ©ton) |
| 1 | **Block light + LOS** (bloque lumiÃ¨re et ligne de vue) |
| 2 | **Block jump** (bloque saut et tÃ©lÃ©portation) |
| 3 | **Block player walk only** (pas le mercenaire â€” usage Ã©trange) |
| 4 | Inconnu |
| 5 | **Block light only** (pas la LOS) |
| 6-7 | Inconnus |

### Rarity / Random Sets

Quand plusieurs tuiles dans un DT1 partagent le mÃªme triplet (Orientation, Main Index, Sub Index), elles forment un **random set**. Le jeu choisit alÃ©atoirement parmi elles selon leur **Rarity** :

```
Exemple : 4 tuiles de sol "terre brÃ»lÃ©e" avec mÃªme identifiant
  Tuile A : Rarity = 1  â†’ 1/37 chance (beaucoup de sang)
  Tuile B : Rarity = 2  â†’ 2/37 chance (un peu de sang)
  Tuile C : Rarity = 10 â†’ 10/37 chance (pas de sang)
  Tuile D : Rarity = 0  â†’ jamais affichÃ©e (si total > 0)
```

Si toutes les Rarity sont Ã  0, seule la **derniÃ¨re tuile** du premier DT1 est utilisÃ©e.

### Encodage graphique des blocs

Deux formats de compression :

**Format 1 â€” Sol isomÃ©trique (RAW, 256 octets fixe) :**

```c
// Dessin d'un bloc isomÃ©trique 3D (losange de 32Ã—15 pixels)
int xjump[15] = {14, 12, 10, 8, 6, 4, 2, 0, 2, 4, 6, 8, 10, 12, 14};
int nbpix[15] = {4,  8,  12, 16, 20, 24, 28, 32, 28, 24, 20, 16, 12, 8,  4};
// Chaque ligne : sauter xjump pixels, dessiner nbpix pixels RAW
```

**Format 0 â€” Murs (RLE, taille variable) :**

```c
// Blocs 32Ã—32 en Run Length Encoding
// Lecture par paires : (skip, count)
// skip = pixels transparents, count = pixels opaques qui suivent
// (0, 0) = saut Ã  la ligne suivante
```

### Block Header (20 octets)

| Offset | Taille | Description |
|--------|--------|-------------|
| 0x00 | 2 | Position X dans le bitmap |
| 0x02 | 2 | Position Y dans le bitmap |
| 0x04 | 2 | ZÃ©ros |
| 0x06 | 1 | Grid X (0-4, position dans la grille subtile) |
| 0x07 | 1 | Grid Y (0-4) |
| 0x08 | 2 | Format (1 = isomÃ©trique RAW, 0 = RLE 32Ã—32) |
| 0x0A | 4 | Longueur des donnÃ©es encodÃ©es (octets) |
| 0x0E | 2 | ZÃ©ros |
| 0x10 | 4 | Offset fichier des donnÃ©es encodÃ©es |

---

## 9.4 SystÃ¨me de presets â€” Format DS1

### Vue d'ensemble

Les fichiers DS1 (Diablo Scene 1) sont des **configurations prÃ©dÃ©finies de cartes isomÃ©triques multi-couches**. Chaque DS1 dÃ©finit l'arrangement des tuiles DT1, les objets, les PNJ et leurs chemins. C'est le format de base pour tous les niveaux, qu'ils soient fixes ou assemblÃ©s en labyrinthe.

### Structure multi-couches

| Composant | Nombre max | Ordre de rendu |
|-----------|-----------|----------------|
| **Couches de sol** | 4 | Rendues en premier |
| **Couches de mur** | 4 | Rendues aprÃ¨s le sol |
| **PNJ / Monstres** | Variable | PlacÃ©s aprÃ¨s le terrain |
| **Objets** (feu, drapeaux, etc.) | Variable | PlacÃ©s selon le type |
| **Chemins de PNJ** | Variable | Paths prÃ©dÃ©finis dans le DS1 |

### RÃ©fÃ©rencement des tuiles

Chaque cellule du DS1 rÃ©fÃ©rence des tuiles DT1 via leurs 3 index :
- **Main Index** (6 bits dans le DS1, soit 0-63)
- **Sub Index** (6 bits, soit 0-63)
- **Orientation** (dÃ©termine le type : sol, mur, ombre, etc.)

### Tuiles spÃ©ciales dans les DS1

Les tuiles spÃ©ciales (Orientation 10-11) contrÃ´lent des fonctionnalitÃ©s gameplay :

| Index | Fonction |
|-------|----------|
| #00-46 | **Vis** (warps entre zones â€” passage d'une zone Ã  une autre) |
| #47-74 | **Area** (suppression de murs et toits pour rÃ©vÃ©ler l'intÃ©rieur) |
| #75 | Town Entry |
| #76 | Map Entry |
| #77 | Town Entry 2 |
| #78 | Corpse Location |
| #79 | Teleport Location |
| #80 | Unknown |
| #82-83 | Vis supplÃ©mentaires |

### PopPads et suppression de toits

Les tuiles spÃ©ciales #47-74 contrÃ´lent la **suppression dynamique de murs/toits** quand le joueur approche d'un bÃ¢timent. Deux tuiles spÃ©ciales identiques sont placÃ©es pour former un rectangle dÃ©finissant la zone de suppression.

```
LvlPrest.txt colonnes associÃ©es :
  Pops    = nombre de zones de suppression dans le DS1
  PopPad  = offset en subtiles du trigger par rapport Ã  la zone dÃ©finie
            (0 = exact, +N = zone trigger agrandie, -N = rÃ©trÃ©cie)
```

Le mÃ©canisme est pilotÃ© par le **Sub Index** de la tuile spÃ©ciale : il correspond au **Main Index** des tuiles qui seront supprimÃ©es. Un groupe de tuiles spÃ©ciales (mÃªme Group) supprime ensemble ; des tuiles de groupes diffÃ©rents supprimÃ©es indÃ©pendamment, permettant jusqu'Ã  **4 zones de suppression sÃ©parÃ©es simultanÃ©ment**.

| Groupe | Main Index des spÃ©ciales | Peuvent supprimer indÃ©pendamment |
|--------|-------------------------|--------------------------------|
| 1 | 8, 9, 10 | Oui (entre eux, mÃªme cibles) |
| 2 | 12, 13 | Oui |
| 3 | 16 | Oui |
| 4 | 20 | Oui |

### Transparence des murs

La transparence (fading) des murs se produit quand les murs forment une **chaÃ®ne fermÃ©e** (box) :

- Tous les murs doivent Ãªtre connectÃ©s sans interruption
- La colonne `Logicals` dans `LvlPrest.txt` active/dÃ©sactive la transparence
- Le paramÃ¨tre `Tile Sound = 0` dans le fichier `.ini` du tileset est requis
- Les coins nord doivent Ãªtre sÃ©parÃ©s en deux frames (Orientation 3 + Orientation 4)

---

## 9.5 Fichiers de configuration â€” LvlPrest.txt

### Colonnes principales

| Colonne | Type | Description |
|---------|------|-------------|
| `Def` | int | Identifiant unique du preset |
| `LevelId` | int | RÃ©fÃ©rence vers `Levels.txt` |
| `Populate` | bool | Placement alÃ©atoire de monstres dans le preset |
| `Logicals` | bool | Active la transparence des murs |
| `Outdoors` | bool | Zone extÃ©rieure |
| `Animate` | bool | Active les animations de tuiles |
| `KillEdge` | bool | Supprime les tuiles en bordure |
| `FillBlanks` | bool | Remplit les espaces vides avec le sol par dÃ©faut |
| `Expansion` | bool | Extension Lord of Destruction |
| `SizeX` / `SizeY` | int | Dimensions en cells (coordonnÃ©es, base 0) |
| `AutoMap` | bool | GÃ©nÃ¨re l'automap pour ce preset |
| `Scan` | bool | Scan des tuiles VIS |
| `Pops` | int | Nombre de PopPads (zones de suppression de toits) |
| `PopPad` | int | Offset du trigger PopPad (en subtiles) |
| `Files` | int | Nombre de variantes DS1 utilisÃ©es (1-6) |
| `File1`-`File6` | string | Chemins vers les fichiers DS1 (jusqu'Ã  6 variantes) |
| `Dt1Mask` | uint32 | Bitmask des fichiers DT1 Ã  charger |

### Dt1Mask â€” Calcul

Le `Dt1Mask` est un **bitmask 32 bits** qui dÃ©termine quels fichiers DT1 de `LvlTypes.txt` sont chargÃ©s pour ce preset.

```
Formule : Dt1Mask = 2^(nombre de DT1 Ã  charger) - 1

Exemple :
  LvlTypes.txt pour LevelType 5 liste 16 fichiers DT1
  Dt1Mask = 2^16 - 1 = 65535 (0xFFFF)
  â†’ Charge les 16 DT1

Pour charger seulement les DT1 #0, #2, #5 :
  Dt1Mask = (1 << 0) | (1 << 2) | (1 << 5) = 0b00100101 = 37
```

Chaque bit correspond Ã  une colonne File dans `LvlTypes.txt`. Bit 0 = File1, bit 1 = File2, etc.

---

## 9.6 Fichiers de configuration â€” LvlTypes.txt

`LvlTypes.txt` associe un **identifiant de type de niveau** Ã  une liste de fichiers DT1. Chaque ligne dÃ©finit un ensemble visuel complet (tileset).

| Colonne | Description |
|---------|-------------|
| `Id` | Identifiant du LevelType (rÃ©fÃ©rencÃ© par Levels.txt) |
| `File1`-`File32` | Chemins vers les fichiers DT1 composant le tileset |
| `Act` | Acte auquel le type appartient |

Le `LevelType` de `Levels.txt` pointe vers une ligne de `LvlTypes.txt`, et le `Dt1Mask` de `LvlPrest.txt` sÃ©lectionne quels DT1 de cette ligne sont effectivement chargÃ©s.

```
Pipeline :
  Levels.txt[LevelType] â†’ LvlTypes.txt[Id] â†’ Liste de DT1
  LvlPrest.txt[Dt1Mask] â†’ Filtre les DT1 Ã  charger
  DS1 â†’ RÃ©fÃ©rence les tuiles par (Orientation, MainIndex, SubIndex) dans les DT1 chargÃ©s
```

---

## 9.7 Composition des niveaux â€” Levels.txt

### Colonnes complÃ¨tes

**Identification :**

| Colonne | Description |
|---------|-------------|
| `Name` | Nom interne du niveau |
| `Id` | Identifiant unique (rÃ©fÃ©rencÃ© partout) |
| `Pal` | Palette de couleurs (une par acte) |
| `Act` | Acte d'appartenance (0-4) |

**Dimensions et positionnement :**

| Colonne | Description |
|---------|-------------|
| `SizeX` / `SizeY` | Dimensions horizontale/verticale en subtiles |
| `OffsetX` / `OffsetY` | Position dans le worldspace (-1/-1 = position hardcodÃ©e) |
| `Depend` | ID du niveau dont le warp dÃ©pend pour l'alignement |

**GÃ©nÃ©ration :**

| Colonne | Description |
|---------|-------------|
| `DrlgType` | Type de gÃ©nÃ©ration : 1=maze, 2=preset, 3=wilderness |
| `LevelType` | RÃ©fÃ©rence vers LvlTypes.txt (tileset) |
| `SubType` | Sous-type de niveau (influence la variÃ©tÃ©) |
| `SubTheme` | ThÃ¨me de remplissage (arbres, marÃ©cages...) |

**Connexions :**

| Colonne | Description |
|---------|-------------|
| `Vis0`-`Vis7` | IDs des niveaux connectÃ©s visuellement |
| `Warp0`-`Warp7` | IDs des warps dans LvlWarp.txt (affichage des entrÃ©es) |
| `WarpDist` | Taille de la zone de sÃ©curitÃ© autour des entrÃ©es (en subtiles, dÃ©faut ~2025) |

**Ã‰clairage et atmosphÃ¨re :**

| Colonne | Description |
|---------|-------------|
| `LOSDraw` | 0 = pas de Line Of Sight (extÃ©rieur), 1 = LoS active (grottes) |
| `IsInside` | 0 = cycle jour/nuit, 1 = toujours jour |
| `Rain` | Peut-il pleuvoir (0/1) |
| `NoPer` | 0 = perspective autorisÃ©e, 1 = perspective interdite |
| `Intensity` / `Red` / `Green` / `Blue` | ContrÃ´le RGB de l'Ã©clairage (0-255) |

**DensitÃ© de monstres :**

| Colonne | Description |
|---------|-------------|
| `MonDen` | DensitÃ© de monstres (Normal) â€” valeur relative, pas un compte absolu |
| `MonDen(N)` | DensitÃ© Nightmare |
| `MonDen(H)` | DensitÃ© Hell |
| `MonUMin` / `MonUMax` | Min/Max de boss et champions dans la zone |
| `MonWndr` | 0 = monstres immobiles avant activation, 1 = IA active (errance) |
| `MonSpcWalk` | Comportement spÃ©cial de dÃ©placement (1 = grilles mÃ©talliques, 5 = tuiles liquides) |

**SÃ©lection des monstres :**

| Colonne | Description |
|---------|-------------|
| `Mtot` | Nombre total de types de monstres diffÃ©rents (max 4 simultanÃ©s par jeu) |
| `M1`-`M25` | IDs des monstres Ã©ligibles au spawn alÃ©atoire |
| `S1`-`S25` | Monstres "satellites" : quand M1 spawn, S1 spawn aussi automatiquement |
| `Utot` | Nombre total de types de monstres Ã©ligibles comme boss/champions |
| `U1`-`U25` | IDs des monstres pouvant apparaÃ®tre comme boss |

**Algorithme de sÃ©lection :**

```
fn select_monsters_for_zone(level, difficulty):
    // 1. SÃ©lectionner Mtot types parmi M1-M25 (alÃ©atoire seed-dÃ©pendant)
    pool = level.M1_to_M25.filter(|m| m != 0)
    selected = random_pick(pool, level.Mtot, rng)
    
    // 2. VÃ©rifier que chaque monstre peut spawner
    for monster_id in selected:
        mon = MonStats[monster_id]
        if mon.spawn != 1:
            continue  // MonStats.txt "spawn" doit Ãªtre 1
        
        // 3. Niveau de zone (difficultÃ©)
        monster_level = match difficulty:
            Normal    => mon.Level           // niveau fixe depuis MonStats.txt
            Nightmare => level.MonLvl2       // niveau de zone
            Hell      => level.MonLvl3       // niveau de zone
```

Le jeu ne peut pas spawner plus de **4 types de monstres diffÃ©rents simultanÃ©ment** dans une zone. Si `Mtot` > 4, les types sont sÃ©lectionnÃ©s alÃ©atoirement Ã  chaque gÃ©nÃ©ration, apportant de la variÃ©tÃ© entre les parties.

**Critters et objets de dÃ©cor :**

| Colonne | Description |
|---------|-------------|
| `C1`-`C5` | Types de critters (serpents, poulets, chameaux...) |
| `CA1`-`CA5` | % chance de spawn de chaque critter (typiquement 30) |
| `objGrp0`-`objGrp7` | Groupes d'objets dÃ©coratifs/shrines (rÃ©f. Objgroup.txt) |
| `objPct0`-`objPct7` | % chance de spawn pour chaque groupe (max 100) |

**Waypoints et quÃªtes :**

| Colonne | Description |
|---------|-------------|
| `Waypoint` | Index du waypoint dans la zone (255 = aucun) |
| `SubWaypoint` | Gestion du waypoint dans les niveaux non-preset |
| `SubShrine` | Influence le spawn alÃ©atoire de shrines en wilderness |
| `Quest` | ID de la quÃªte liÃ©e Ã  cette zone |
| `SaveMonster` | 1 = les monstres tuÃ©s restent morts en revenant dans la zone |
| `Portal` / `Position` | ContrÃ´le du repositionnement de portails |

---

## 9.8 Waypoints et objets de quÃªte

### Placement des waypoints

| Contexte | MÃ©canisme de placement |
|----------|----------------------|
| **Preset (DrlgType 2)** | Position fixe dans le DS1 â€” toujours au mÃªme endroit |
| **Maze (DrlgType 1)** | PlacÃ© dans une room spÃ©cifique du labyrinthe (room dÃ©signÃ©e par `SubWaypoint`) |
| **Wilderness (DrlgType 3)** | PlacÃ© alÃ©atoirement dans un espace vide de la zone |

La colonne `Waypoint` de `Levels.txt` dÃ©termine l'**index** du waypoint. La valeur 255 signifie qu'il n'y a pas de waypoint dans la zone.

### Zone de sÃ©curitÃ©

La colonne `WarpDist` dÃ©finit une zone tampon (en subtiles) autour des entrÃ©es de niveau et des waypoints pour **empÃªcher les stair-traps** (monstres campant directement sur les entrÃ©es). La valeur par dÃ©faut est ~2025 subtiles.

### Objets de quÃªte

Les objets de quÃªte sont placÃ©s via trois fichiers interconnectÃ©s :

| Fichier | RÃ´le |
|---------|------|
| `Objects.txt` | DÃ©finition de tous les objets plaÃ§ables (propriÃ©tÃ©s, animations) |
| `Objgroup.txt` | Groupes d'objets associÃ©s Ã  des zones (shrines, coffres, etc.) |
| `Objpreset.txt` | Placement fixe d'objets dans des DS1 spÃ©cifiques |

**Types de placement :**

| Type | MÃ©canisme |
|------|-----------|
| **Type 1 (DS1 direct)** | Objets placÃ©s manuellement dans le DS1 via le DS1 Editor |
| **Type 2 (Objpreset)** | Objets rÃ©fÃ©rencÃ©s par ID et placÃ©s dans les DS1 selon l'acte |
| **AlÃ©atoire (Objgroup)** | Objets sÃ©lectionnÃ©s depuis Objgroup.txt et placÃ©s par le DRLG |

**Spawn garanti :**

Certains objets de quÃªte ont un **spawn garanti** car ils sont encodÃ©s directement dans le DS1 du preset :

| Objet | Zone | MÃ©canisme |
|-------|------|-----------|
| Cairn Stones | Stony Field | DS1 thÃ©matique dÃ©diÃ© (toujours prÃ©sent) |
| Inifuss Tree | Dark Woods | DS1 thÃ©matique dÃ©diÃ© |
| Horadric Cube pedestal | Halls of the Dead Niv. 3 | DS1 preset (DrlgType 2) |
| Altars de quÃªte | Zones preset | EncodÃ© dans le DS1 |
| Super Uniques (Corpsefire, etc.) | Room spÃ©ciale du labyrinthe | DS1 "spec" (caveWspec.ds1) |

Les DS1 "spec" (comme `caveWspec.ds1` pour Corpsefire) ne peuvent spawn qu'**une seule fois** dans un labyrinthe et sont toujours placÃ©s Ã  l'extrÃ©mitÃ© la plus profonde.

---

## 9.9 Zones spÃ©ciales

### Arcane Sanctuary (Acte 2)

| Aspect | DÃ©tail |
|--------|--------|
| **DrlgType** | 1 (Maze) |
| **ParticularitÃ©** | L'entrÃ©e est toujours au **centre** du labyrinthe (hardcodÃ©) |
| **Layout** | 4 branches partant du centre, chacune menant Ã  un portail |
| **Pathfinding** | ExtrÃªmement Ã©troit â€” les followers se bloquent frÃ©quemment |

### Maggot Lair (Acte 2)

| Aspect | DÃ©tail |
|--------|--------|
| **DrlgType** | 1 (Maze) |
| **ParticularitÃ©** | Couloirs trÃ¨s Ã©troits (2-3 subtiles de passage) |
| **Impact gameplay** | Les invocations et mercenaires bloquent le joueur |
| **Rooms** | Petites, avec des connexions sinueuses |

### Jungle Acte 3

| Aspect | DÃ©tail |
|--------|--------|
| **DrlgType** | 3 (Wilderness) |
| **ParticularitÃ©** | MalgrÃ© DrlgType 3, la gÃ©nÃ©ration ressemble davantage aux caves (DrlgType 1) |
| **ComplexitÃ©** | Zones denses avec beaucoup d'obstacles visuels et de passages Ã©troits |

---

## 9.10 Transposition MGE â€” GÃ©nÃ©ration procÃ©durale

### Architecture recommandÃ©e

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
    pub level_type: u32,        // rÃ©fÃ©rence tileset
    pub connections: Vec<LevelConnection>,
    pub monster_density: f32,
    pub monster_pool: Vec<u32>,
    pub waypoint_index: Option<u8>,
}

pub struct MazeRoom {
    pub position: (i32, i32),   // position dans la grille de rooms
    pub openings: u8,           // bitmask NSEW (4 bits)
    pub preset_variant: u32,    // index de variante DS1
    pub is_special: bool,       // room de boss/quÃªte
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

### Mapping D2 â†’ MGE

| Fichier D2 | Ã‰quivalent MGE |
|-----------|----------------|
| `Levels.txt` | `LevelDef` composant (table RON/JSON) |
| `LvlPrest.txt` | `PresetDef` composant |
| `LvlTypes.txt` | `TilesetDef` composant |
| `LvlMaze.txt` | `MazeDef` composant |
| `LvlSub.txt` | `FillerDef` composant |
| `LvlWarp.txt` | `WarpDef` composant |
| Fichiers DT1 | `TileAtlas` asset (conversion Ã  l'import) |
| Fichiers DS1 | `RoomPreset` asset (conversion Ã  l'import) |

### DiffÃ©rences clÃ©s avec D2

| Aspect | D2 Original | Allumina (MGE) |
|--------|-------------|----------------|
| **Seed** | 32-bit, stockÃ© dans le .d2s | 64-bit recommandÃ© (plus d'entropie) |
| **Taille de grille** | Subtiles entiÃ¨res | Grille logique + coordonnÃ©es flottantes |
| **Rooms max** | LimitÃ© par RAM (~200Ã—200 tiles par niveau) | Beaucoup plus large (streaming possible) |
| **Persistance** | RÃ©gÃ©nÃ©rÃ© Ã  chaque partie | Persistant si monde ouvert (sÃ©rialisÃ©) |
| **Variantes** | 6 max par preset | IllimitÃ© (chargement dynamique) |
| **Tuiles** | Palette 8-bit, 160Ã—80px | Sprites modernes, rÃ©solution libre |

---

## 10. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [D2MOO (GitHub)](https://github.com/ThePhrozenKeep/D2MOO) | Reverse-engineering et rÃ©implÃ©mentation de Diablo II |
| [Phrozen Keep â€” MonStats.txt](https://d2mods.info/forum/kb/viewarticle?a=360) | Documentation exhaustive de MonStats.txt |
| [Phrozen Keep â€” MonStats2.txt](https://d2mods.info/forum/kb/viewarticle?a=359) | Collision, taille, paramÃ¨tres graphiques |
| [Phrozen Keep â€” Missiles.txt](https://d2mods.info/forum/kb/viewarticle?a=364) | SystÃ¨me de projectiles complet |
| [Phrozen Keep â€” Levels.txt](https://d2mods.info/forum/kb/viewarticle?a=384) | RÃ©fÃ©rence complÃ¨te Levels.txt (par Nefarius) |
| [Phrozen Keep â€” Advanced DT1](https://d2mods.info/forum/kb/viewarticle?a=468) | Orientations, directions, transparence murs, PopPads |
| [Paul Siramy â€” DT1 Format](http://paul.siramy.free.fr/_divers/dt1_doc/) | SpÃ©cification complÃ¨te du format DT1 |
| [Paul Siramy â€” MAZE and DS1 Mechanisms](https://d2mods.info/forum/viewtopic.php?t=13427) | Algorithme de gÃ©nÃ©ration de labyrinthes |
| [Phrozen Keep â€” Randomizing Levels](https://d2mods.info/forum/kb/viewarticle?a=29) | Randomisation de niveaux preset (par Kingpin) |
| [OpenDiablo2/ds1 (GitHub)](https://github.com/OpenDiablo2/ds1) | DÃ©codeur DS1 en Go (reverse-engineering) |
| [d2-map-investigation (GitHub)](https://github.com/squeek502/d2-map-investigation) | CorrÃ©lations de gÃ©nÃ©ration de cartes |
| [diablo-mapgen (GitHub)](https://github.com/Matthew-petroff/diablo-mapgen) | Outil de gÃ©nÃ©ration de cartes D2 |
| [D2 Tile Grid Guide](http://www.dos486.com/diablo/grid/) | Grille isomÃ©trique et footprints |
| [Frames and Animations](https://mannm.org/d2library/faqtoids/frames_eng.html) | Tick rate, breakpoints, animations |
| [D2R Map Seed Extraction](https://noobient.com/2025/11/21/finding-the-map-seed-in-diablo-ii-resurrected/) | Extraction du seed dans les .d2s |
| [Allumina â€” Document Fondateur](Allumina%20-%20Document%20Fondateur.md) | Vision service Allumina |
| [Allumina â€” Combat et Troupes](Concept//Allumina%20-%20Combat%20et%20Troupes.md) | Troupes et Ã©chelles de combat |
| [MGE â€” Skill DÃ©placement](..//..//..//README.md) | ChaÃ®ne locomotion MGE |
| [MGE â€” Hitbox et Collisions](..//..//..//README.md) | RÃ©fÃ©rence collision MGE |
| [MGE â€” Guide Groupes](..//..//..//README.md) | Pathfinding groupes MGE |

---

# 10. SYSTÃˆME DE GÃ‰NÃ‰RATION D'ITEMS â€” TREASURE CLASS

## 10.1 Vue d'ensemble du pipeline de gÃ©nÃ©ration

Quand un monstre meurt, le jeu exÃ©cute le pipeline suivant dans cet ordre exact :

```
Monster Kill
  â†“
[1] DÃ©terminer le TC du monstre (monstats.txt â†’ TreasureClassEx.txt)
  â†“
[2] TC Upgrade (NM/Hell : monter le TC si mlvl > TC level)
  â†“
[3] Pour chaque Pick (nombre = colonne Picks du TC) :
  â”‚   â†“
  â”‚   [3a] Roll NoDrop vs Prob1..Prob10
  â”‚   â†“
  â”‚   [3b] Si NoDrop â†’ rien, passer au pick suivant
  â”‚   [3c] Si TC enfant sÃ©lectionnÃ© â†’ descendre rÃ©cursivement dans le TC enfant
  â”‚   [3d] Si item atomique sÃ©lectionnÃ© â†’ continuer vers [4]
  â†“
[4] DÃ©termination de la qualitÃ© (Unique â†’ Set â†’ Rare â†’ Magic â†’ Superior â†’ Normal â†’ Low)
  â†“
[5] SÃ©lection Unique/Set spÃ©cifique (si qualitÃ© Unique ou Set)
  â†“
[6] GÃ©nÃ©ration des affixes (si Magic, Rare ou Crafted)
  â†“
[7] Roll Ã‰thÃ©rÃ© (5% pour items Ã©ligibles)
  â†“
[8] Roll Sockets (si applicable)
  â†“
[9] Roll des valeurs de propriÃ©tÃ©s (ranges des mods)
  â†“
Item final gÃ©nÃ©rÃ©
```

## 10.2 Structure des Treasure Classes

### Fichier TreasureClassEx.txt

Chaque ligne dÃ©finit un TC avec :

| Colonne | RÃ´le |
|---------|------|
| `Treasure Class` | Nom du TC |
| `Picks` | Nombre de tentatives de drop |
| `group` | Groupe pour TC upgrade (NM/Hell) |
| `level` | Niveau du TC (pour TC upgrade) |
| `Unique` | Bonus qualitÃ© Unique (QualityFactor, 0-1024) |
| `Set` | Bonus qualitÃ© Set (QualityFactor, 0-1024) |
| `Rare` | Bonus qualitÃ© Rare (QualityFactor, 0-1024) |
| `Magic` | Bonus qualitÃ© Magic (QualityFactor, 0-1024) |
| `NoDrop` | Poids de NoDrop |
| `Item1..Item10` | EntrÃ©es (items ou sous-TCs) |
| `Prob1..Prob10` | Poids de probabilitÃ© de chaque entrÃ©e |

### HiÃ©rarchie rÃ©cursive

Les TCs forment un **arbre rÃ©cursif**. Chaque entrÃ©e dans un TC peut Ãªtre :
- Un **item atomique** (ex : `gld`, `amu`, un code d'item spÃ©cifique)
- Un **TC enfant** (ex : `Act 5 (H) Equip C`)
- Un **TC auto-gÃ©nÃ©rÃ©** (ex : `weap87`, `armo84` â€” crÃ©Ã©s au runtime depuis weapons.txt/armor.txt)

### TCs auto-gÃ©nÃ©rÃ©s (Atomic TCs)

Le jeu gÃ©nÃ¨re au runtime des TCs `WeapXX` et `ArmoXX` (XX = 03 Ã  87) en regroupant les items par qlvl :

```
Armo03 = tous les items d'armure avec qlvl 1-3
  â†’ Cap (qlvl 1), Quilted Armor (qlvl 1), Buckler (qlvl 1), etc.

Armo87 = tous les items d'armure avec qlvl 85-87
  â†’ Diadem (qlvl 85), Corona (qlvl 85), Sacred Armor (qlvl 85), etc.

Weap87 = tous les items d'arme avec qlvl 85-87
  â†’ Phase Blade (qlvl 73 â†’ NON, pas dans weap87), etc.
```

### PondÃ©ration dans les TCs atomiques

Dans les TCs atomiques, chaque item a un poids `Rarity` (dÃ©fini dans ItemTypes.txt) :

| Type d'item | Rarity |
|-------------|--------|
| Items normaux (Ã©pÃ©es, armures...) | 3 |
| Assassin claws | 2 |
| Wands / Staves / Scepters | 1 |
| Autres class-specific | 1 |

**ProbabilitÃ© d'un item** = `ItemRarity / TotalRarity`

Exemple `armo87` (13 de total rarity) :
- Diadem : 3/22
- Corona : 3/22
- Sacred Armor : 3/22
- Class-specific items : 1/22 chacun

## 10.3 Picks et sÃ©lection

### Picks positifs

Si `Picks > 0`, le jeu effectue `Picks` tentatives indÃ©pendantes. Chaque tentative :
1. Calcule la somme totale : `Total = NoDrop + Prob1 + Prob2 + ... + Prob10`
2. Tire un nombre alÃ©atoire `R` dans `[0, Total)`
3. SÃ©lectionne l'entrÃ©e correspondante au poids cumulÃ©

### Picks nÃ©gatifs

Si `Picks < 0`, le jeu garantit exactement `|Picks|` items, en **ignorant NoDrop**. Les items sont distribuÃ©s proportionnellement aux poids Prob.

Exemple : `Picks = -3`, `Prob1=2`, `Prob2=1` â†’ toujours 2 de Item1 + 1 de Item2.

### Limite physique

Un monstre ne peut pas drop plus de **6 items** au sol (limitation du moteur). Si un boss a `Picks=7` (comme Mephisto), un des rolls est gaspillÃ© si tous rÃ©ussissent.

### Exemple complet : Mephisto Hell

```
Mephisto (H) :
  Picks = 7
  NoDrop = 15
  gld,mul=2048     â†’ Prob = 5
  Act 4 (H) Equip A â†’ Prob = 52
  Act 4 (H) Junk    â†’ Prob = 5
  Act 4 (H) Good    â†’ Prob = 3
  
  Bonus qualitÃ© : Unique=983, Set=983, Rare=983, Magic=1024
  Total = 15 + 5 + 52 + 5 + 3 = 80

Pour CHAQUE pick (7 fois) :
  15/80 = 18.75% â†’ NoDrop
  5/80  = 6.25%  â†’ Gold
  52/80 = 65%    â†’ Equipment
  5/80  = 6.25%  â†’ Junk (potions, flÃ¨ches)
  3/80  = 3.75%  â†’ Good (gemmes, runes, jewels, charmes, anneaux, amulettes)
```

## 10.4 TC Upgrade (Nightmare / Hell)

En Normal, le TC du monstre est utilisÃ© directement. En NM/Hell, le TC peut Ãªtre **upgradÃ©** :

```
Algorithme TC Upgrade :
  1. Trouver le TC de base du monstre (monstats.txt)
  2. VÃ©rifier si le TC a un "group" dÃ©fini
  3. Si oui : trouver le TC le plus Ã©levÃ© dans le mÃªme group
     dont level â‰¤ mlvl du monstre
  4. Utiliser ce TC upgradÃ©
  5. Les TCs enfants (inclus dans le TC sÃ©lectionnÃ©) ne sont PAS upgradÃ©s
```

Exemple :
- Devilkin dans The Pit (Hell) : mlvl = 85
- TC de base : "Act 1 (H) H2H B" (group=7, level=66)
- TC upgradÃ© : "Act 5 (H) H2H C" (group=7, level=85 â‰¤ mlvl)

## 10.5 Formule NoDrop et scaling multijoueur

### Formule NoDrop en solo

```
P(NoDrop) = NoDrop / (NoDrop + ProbSum)
```

OÃ¹ `ProbSum = Prob1 + Prob2 + ... + Prob10`

### Formule NoDrop multijoueur

```
NewNoDrop = int( ProbSum / ( 1/( (NoDrop/(NoDrop+ProbSum))^N ) - 1 ) )
```

OÃ¹ :
```
N = int(1 + AdditionalPlayers/2 + ClosePartiedPlayers/2)

- AdditionalPlayers : tous les autres joueurs dans la partie
- ClosePartiedPlayers : joueurs dans votre party ET Ã  moins de 2 Ã©crans
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
  NewNoDrop = 15 â†’ 15/80 = 18.75% NoDrop par pick

N=2 :
  NoDrop rate = (15/80)^2 = 0.03516
  NewNoDrop = int(65 Ã— 0.03516 / (1 - 0.03516)) = int(2.366) = 2
  â†’ 2/67 = 2.99% NoDrop par pick

N=3 :
  NoDrop rate = (15/80)^3 = 0.00659
  NewNoDrop = int(65 Ã— 0.00659 / (1 - 0.00659)) = int(0.431) = 0
  â†’ 0% NoDrop (full drops garantis)
```

## 10.6 Boss d'acte â€” RÃ¨gles spÃ©ciales

### Bonus de niveau des Ã©lites

| Type de monstre | Bonus mlvl |
|----------------|-----------|
| Champion | +2 |
| Boss / Unique / Minion | +3 |

### RÃ¨gles spÃ©cifiques aux boss d'acte

| RÃ¨gle | Effet |
|-------|-------|
| **Force Magic** | Les boss d'acte et les boss alÃ©atoires forcent un drop minimum de qualitÃ© Magic (ne s'applique pas aux items non-magiques : runes, potions, etc.) |
| **QualityFactor Ã©levÃ©** | Mephisto/Diablo/Baal (H) ont `Unique=983, Set=983, Rare=983, Magic=1024` â†’ Ã©norme bonus qualitÃ© |
| **Quest Drop** | Premier kill d'un boss d'acte = drops amÃ©liorÃ©s (pas de white items, pas de potions dans le drop principal) |

### Boss levels (monstats.txt)

| Boss | Normal | Nightmare | Hell |
|------|--------|-----------|------|
| Andariel | 12 | 49 | 75 |
| Duriel | 22 | 55 | 88 |
| Mephisto | 26 | 59 | 87 |
| Diablo | 40 | 62 | 94 |
| Baal | 60 | 75 | 99 |
| Nihlathak | 65 | 70 | 92 |
| Uber Bosses | â€” | â€” | 110 |

### Quest Drop Bug (Andariel)

Andariel peut Ãªtre **permanentement buguÃ©e** en quest drop :
1. Tuer Andariel pour la premiÃ¨re fois
2. Aller directement Ã  l'Acte 2 (via le portail de Warriv)
3. Sauvegarder et quitter dans l'Acte 2

Si cela est fait correctement, **toutes les exÃ©cutions futures** d'Andariel en cette difficultÃ© utiliseront la table de quest drop (pas de white items/potions).

Pour les autres boss (Duriel, Mephisto, Diablo, Baal), le quest drop bug nÃ©cessite un personnage secondaire n'ayant pas complÃ©tÃ© la quÃªte pour porter le coup final.

---

# 11. DÃ‰TERMINATION DE LA QUALITÃ‰ D'ITEM

## 11.1 Niveaux fondamentaux

| Niveau | AbrÃ©viation | Source | Description |
|--------|-------------|--------|-------------|
| **Item Level** | ilvl | = mlvl du monstre | Niveau cachÃ© de l'item, dÃ©terminÃ© Ã  la crÃ©ation |
| **Monster Level** | mlvl | monstats.txt / area level | Niveau du monstre qui drop |
| **Quality Level** | qlvl | weapons.txt / armor.txt | Niveau intrinsÃ¨que du type d'item de base |
| **Area Level** | alvl (zone) | levels.txt | Niveau de la zone (= mlvl des monstres normaux en NM/Hell) |
| **Affix Level** | alvl (affix) | CalculÃ© | Niveau dÃ©terminant quels affixes peuvent apparaÃ®tre |
| **Character Level** | clvl | Joueur | Niveau du personnage (utilisÃ© pour gambling) |

### Relation ilvl â†” mlvl

```
ilvl = mlvl (pour drops de monstres)
ilvl = area level (pour drops de coffres/conteneurs)

Champions : mlvl = area_level + 2
Uniques/Boss random : mlvl = area_level + 3
Boss d'acte : mlvl fixe (voir table Â§10.6)
```

## 11.2 Algorithme de dÃ©termination de qualitÃ©

Le jeu teste les qualitÃ©s **dans cet ordre exact** et s'arrÃªte au premier succÃ¨s :

```
1. Test UNIQUE    â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Unique (ou downgrade)
2. Test SET       â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Set (ou downgrade)
3. Test RARE      â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Rare
4. Test MAGIC     â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Magic
5. Test SUPERIOR  â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Superior
6. Test NORMAL    â†’ Si succÃ¨s â†’ gÃ©nÃ©rer Normal
7. Fallback       â†’ Low Quality (cracked, crude, etc.)
```

### Formule complÃ¨te de chaque test

```
Ã‰TAPE 1 : SÃ©lectionner la ligne correcte dans ItemRatio.txt
  â†’ Version (0=Classic, 1=LoD)
  â†’ Uber (0=Normal tier, 1=Exceptional/Elite tier)
  â†’ Class Specific (0=non, 1=oui)

Ã‰TAPE 2 : Calculer Chance
  Chance = (BaseChance - ((ilvl - qlvl) / Divisor)) Ã— 128

Ã‰TAPE 3 : Appliquer Magic Find (seulement pour Unique, Set, Rare)
  EffectiveMF = MF Ã— Factor / (MF + Factor)
  
  Factors (rendements dÃ©croissants) :
    Unique : Factor = 250
    Set    : Factor = 500
    Rare   : Factor = 600
    Magic  : Pas de diminishing returns â†’ EffectiveMF = MF
  
  Chance = Chance Ã— 100 / (100 + EffectiveMF)

Ã‰TAPE 4 : Appliquer le minimum
  if (Chance < MinChance) then Chance = MinChance

Ã‰TAPE 5 : Appliquer le QualityFactor du TC
  FinalChance = Chance - (Chance Ã— QualityFactor / 1024)
  
  QualityFactor = valeur Unique/Set/Rare/Magic du TC dans TreasureClassEx.txt
  (valeur maximale rencontrÃ©e dans toute la chaÃ®ne de TCs traversÃ©e)

Ã‰TAPE 6 : Roll final
  GÃ©nÃ©rer un nombre alÃ©atoire R dans [0, FinalChance)
  if (R < 128) â†’ SUCCÃˆS (item de cette qualitÃ©)
  else         â†’ Ã‰CHEC (passer au test suivant)

ProbabilitÃ© finale = 128 / FinalChance
```

## 11.3 Valeurs de ItemRatio.txt (v1.13 LoD)

### Items NON class-specific, tier Normal (Version=1, Uber=0, ClassSpecific=0)

| QualitÃ© | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 400 | 1 | 6400 |
| **Set** | 160 | 2 | 5600 |
| **Rare** | 100 | 2 | 3200 |
| **Magic** | 34 | 3 | 192 |
| **HiQuality** | 12 | 8 | â€” |
| **Normal** | 2 | 2 | â€” |

### Items NON class-specific, tier Exceptional/Elite (Version=1, Uber=1, ClassSpecific=0)

| QualitÃ© | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 400 | 1 | 6400 |
| **Set** | 160 | 2 | 5600 |
| **Rare** | 100 | 2 | 3200 |
| **Magic** | 34 | 3 | 192 |
| **HiQuality** | 12 | 8 | â€” |
| **Normal** | 1 | 1 | â€” |

### Items CLASS-SPECIFIC, tier Normal (Version=1, Uber=0, ClassSpecific=1)

| QualitÃ© | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 240 | 3 | 6400 |
| **Set** | 120 | 3 | 5600 |
| **Rare** | 80 | 3 | 3200 |
| **Magic** | 17 | 6 | 192 |
| **HiQuality** | 9 | 8 | â€” |
| **Normal** | 2 | 2 | â€” |

### Items CLASS-SPECIFIC, tier Exceptional/Elite (Version=1, Uber=1, ClassSpecific=1)

| QualitÃ© | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| **Unique** | 240 | 3 | 6400 |
| **Set** | 120 | 3 | 5600 |
| **Rare** | 80 | 3 | 3200 |
| **Magic** | 17 | 6 | 192 |
| **HiQuality** | 9 | 8 | â€” |
| **Normal** | 1 | 1 | â€” |

## 11.4 Formules Magic Find â€” Rendements dÃ©croissants

### Formule Effective Magic Find (EMF)

```
Pour Unique : EMF = MF Ã— 250 / (MF + 250)
Pour Set    : EMF = MF Ã— 500 / (MF + 500)
Pour Rare   : EMF = MF Ã— 600 / (MF + 600)
Pour Magic  : EMF = MF (pas de diminishing returns)

Exception : si MF â‰¤ 10, alors EMF = MF (pas de DR appliquÃ©)
```

### Table de rÃ©fÃ©rence EMF

| MF rÃ©el | EMF Unique | EMF Set | EMF Rare | EMF Magic |
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

- EMF Unique ne peut jamais dÃ©passer 250 (atteint Ã  MF â†’ âˆž)
- EMF Set ne peut jamais dÃ©passer 500
- EMF Rare ne peut jamais dÃ©passer 600
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

Ã‰tape 2 : Chance = (400 - ((99-86)/1)) Ã— 128 = (400-13) Ã— 128 = 387 Ã— 128 = 49536
Ã‰tape 3 : Factor = 250 (unique)
  EMF = 200 Ã— 250 / (200+250) = 50000/450 = 111
  Chance = 49536 Ã— 100 / (100+111) = 49536 Ã— 100/211 = 23476
Ã‰tape 4 : 23476 > MinChance(6400) â†’ pas de clamp
Ã‰tape 5 : QualityFactor = 983 (Baal Hell TC)
  FinalChance = 23476 - (23476 Ã— 983/1024) = 23476 - 22530 = 946 â‰ˆ 939*

ProbabilitÃ© Unique = 128/939 = 13.6%

* La lÃ©gÃ¨re diffÃ©rence vient de l'arithmÃ©tique entiÃ¨re (int truncation Ã  chaque Ã©tape)
```

---

# 12. GÃ‰NÃ‰RATION DES ITEMS UNIQUES ET SET

## 12.1 SÃ©lection d'un Unique spÃ©cifique

AprÃ¨s que le test de qualitÃ© a dÃ©terminÃ© "Unique", le jeu :

```
1. Construire la liste de tous les Uniques du mÃªme type de base
   dont qlvl_unique â‰¤ ilvl de l'item

2. Si la liste est VIDE :
   â†’ Downgrade en RARE avec durabilitÃ© Ã— 3
   â†’ (Si le type ne peut pas Ãªtre Rare â†’ Magic avec durabilitÃ© Ã— 3)

3. Si la liste contient UN seul item :
   â†’ Cet Unique est sÃ©lectionnÃ©

4. Si la liste contient PLUSIEURS items :
   â†’ SÃ©lection pondÃ©rÃ©e par le champ "rarity" de UniqueItems.txt
   â†’ P(item) = item.rarity / Î£(all_rarities)
```

### Champ Rarity des Uniques

| Item | Rarity | Commentaire |
|------|--------|-------------|
| Manald Heal (anneau unique) | 15 | TrÃ¨s commun |
| Nagelring (anneau unique) | 15 | TrÃ¨s commun |
| Stone of Jordan | 1 | 15Ã— plus rare que Manald |
| Bul-Kathos' Wedding Band | 1 | Rare |

Exemple : 9 anneaux uniques Ã©ligibles, total rarity = 59
- SoJ = 1/59 â‰ˆ 1.7%
- Manald = 15/59 â‰ˆ 25.4%

## 12.2 PrÃ©vention des doublons Uniques

### Champ "nolimit" de UniqueItems.txt

| Valeur nolimit | Comportement |
|----------------|-------------|
| 0 (ou vide) | L'Unique ne peut drop qu'**une seule fois** par partie |
| 1 | Pas de limite (peut drop plusieurs fois) |

### MÃ©canisme de prÃ©vention

Quand un Unique avec `nolimit=0` est sÃ©lectionnÃ© :

```
1. VÃ©rifier si cet Unique a dÃ©jÃ  Ã©tÃ© :
   a. DroppÃ© comme Unique dans cette partie
   b. DroppÃ© comme failed unique (rare 3Ã— durabilitÃ©) dans cette partie
   c. GÃ©nÃ©rÃ© dans l'Ã©cran de gambling dans cette partie

2. Si dÃ©jÃ  gÃ©nÃ©rÃ© â†’ Downgrade en Rare avec durabilitÃ© Ã— 3

3. Si non â†’ Drop l'Unique et marquer comme "gÃ©nÃ©rÃ©" pour cette partie
```

C'est pourquoi les uniques communs (Manald, Nagelring) "bloquent" le drop de SoJ : une fois Manald gÃ©nÃ©rÃ©, la prochaine tentative de Manald downgrade en rare, mais le flag empÃªche aussi sa re-sÃ©lection.

## 12.3 Conditions de downgrade

### Downgrade Unique â†’ Rare

Un Unique est downgrade en **Rare avec durabilitÃ© Ã— 3** si :
1. Le qlvl de l'Unique > ilvl (monstre pas assez haut niveau)
2. Aucun Unique n'existe pour ce type de base
3. L'Unique a dÃ©jÃ  Ã©tÃ© gÃ©nÃ©rÃ© dans cette partie (`nolimit=0`)

### Downgrade Set â†’ Magic

Un Set est downgrade en **Magic avec durabilitÃ© Ã— 2** si :
1. Le qlvl du Set > ilvl
2. Aucun Set n'existe pour ce type de base

### Cas notable : Pindleskin et Arachnid Mesh

```
Pindleskin (Hell) : mlvl = 86
Arachnid Mesh (unique Spiderweb Sash) : qlvl = 87

86 < 87 â†’ qlvl > mlvl â†’ IMPOSSIBLE pour Pindleskin de drop Arachnid Mesh
â†’ Toute tentative de Unique Spiderweb Sash = Rare 3Ã— durabilitÃ©
```

### Cas notable : Tyrael's Might et Templar's Might

```
Sacred Armor uniques :
  Templar's Might : qlvl = 85
  Tyrael's Might  : qlvl = 87

Pindleskin (mlvl=86) :
  â†’ Peut drop Templar's Might (85 â‰¤ 86)
  â†’ Ne peut PAS drop Tyrael's Might (87 > 86)
  
Baal Hell (mlvl=99) :
  â†’ Peut drop les deux
```

## 12.4 SÃ©lection d'un Set spÃ©cifique

MÃªme algorithme que pour les Uniques :

```
1. Construire la liste de tous les items Set du mÃªme type de base
   dont qlvl_set â‰¤ ilvl
2. Si vide â†’ Magic avec durabilitÃ© Ã— 2
3. Si un seul â†’ sÃ©lectionnÃ©
4. Si plusieurs â†’ pondÃ©ration par rarity
```

---

# 13. SYSTÃˆME DE GÃ‰NÃ‰RATION DES AFFIXES

## 13.1 Calcul du Affix Level (alvl)

Le alvl dÃ©termine quels affixes (prefixes/suffixes) sont disponibles pour un item :

```
Algorithme (arithmÃ©tique entiÃ¨re, pas de fractions) :

1. if (ilvl > 99) then ilvl = 99    // Cap Ã  99
2. if (qlvl > ilvl) then ilvl = qlvl // qlvl minimum
   // (Note : ce ilvl modifiÃ© est temporaire, ne change pas l'item)

3. if (magic_lvl > 0) then
     alvl = ilvl + magic_lvl
   else
     if (ilvl < (99 - qlvl/2)) then
       alvl = ilvl - qlvl/2          // int division
     else
       alvl = 2 Ã— ilvl - 99

4. if (alvl > 99) then alvl = 99    // Cap Ã  99
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

### Exemple : Small Charm droppÃ© par Pindleskin Normal

```
ilvl = 45 (Pindleskin Normal mlvl)
qlvl = 28 (Small Charm qlvl)
magic_lvl = 0

qlvl(28) < ilvl(45) â†’ pas de clamp
magic_lvl = 0 â†’ branche else
ilvl(45) < 99 - qlvl/2 = 99 - 14 = 85 â†’ oui
alvl = 45 - 28/2 = 45 - 14 = 31
```

## 13.2 SÃ©lection des affixes pour items Magic

### Nombre d'affixes (items Magic)

```
Roll alÃ©atoire :
  50% â†’ Suffix seulement
  25% â†’ Prefix seulement
  25% â†’ Prefix ET Suffix

Maximum : 1 Prefix + 1 Suffix
```

### SÃ©lection d'un affix

Pour chaque slot (prefix/suffix) :

```
1. Construire la liste des affixes Ã©ligibles :
   - spawnable = 1
   - itype correspond au type de l'item
   - etype ne contient PAS le type de l'item
   - level â‰¤ alvl (affix level min)
   - maxlevel â‰¥ alvl OU maxlevel = 0 (pas de cap)
   - version correcte (Classic/LoD)

2. SÃ©lection pondÃ©rÃ©e par "frequency" :
   P(affix) = affix.frequency / Î£(frequencies de tous les affixes Ã©ligibles)
   
   frequency = 0 â†’ ne peut JAMAIS apparaÃ®tre en drop
   (seulement via cube recipes)
```

## 13.3 SÃ©lection des affixes pour items Rare

### Nombre d'affixes (items Rare)

```
Les Rare Items ont entre 3 et 6 affixes.
ProbabilitÃ© 1/4 (25%) pour chaque nombre : 3, 4, 5, ou 6.

Maximum : 3 Prefixes + 3 Suffixes

Restriction Jewels Rare : maximum 4 affixes total
```

### Note sur ilvl et nombre d'affixes des Crafted items

```
ilvl 1-30  : 40% â†’ 1 affix, 20% â†’ 2, 20% â†’ 3, 20% â†’ 4
ilvl 31-50 : 60% â†’ 2 affixes, 20% â†’ 3, 20% â†’ 4
ilvl 51-70 : 80% â†’ 3 affixes, 20% â†’ 4
ilvl 71+   : 100% â†’ 4 affixes
```

### Algorithme de sÃ©lection (Rare)

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
    
    // VÃ©rifier les caps
    if is_prefix and prefix_count >= 3:
      is_prefix = false  // forcer suffix
    if !is_prefix and suffix_count >= 3:
      is_prefix = true   // forcer prefix
    
    // Construire la liste des affixes Ã©ligibles
    pool = get_eligible_affixes(
      item.type,
      alvl,
      is_prefix,
      exclude_groups = used_groups,
      rare_only = true  // certains affixes sont "magic only"
    )
    
    if pool.is_empty():
      break
    
    // SÃ©lection pondÃ©rÃ©e par frequency
    affix = weighted_random(pool, |a| a.frequency)
    
    affixes.push(affix)
    used_groups.insert(affix.group)
    if is_prefix: prefix_count += 1
    else: suffix_count += 1
  
  return affixes
```

### Contraintes de groupes

```
- Un item ne peut pas avoir 2 affixes du mÃªme "group" (MagicPrefix.txt / MagicSuffix.txt)
- Exemple : "Wyrm" (+41-60 Mana, group 55) et "Dragon's" (+31-40 Mana, group 55) 
  sont mutuellement exclusifs
- Des affixes du mÃªme TYPE mais de GROUPS diffÃ©rents peuvent coexister
  (ex : +life prefix et +life suffix)
```

### Affixes "magic only"

Certains affixes ont `rare=0` dans MagicPrefix.txt/MagicSuffix.txt :
- Ces affixes ne peuvent apparaÃ®tre **que** sur des items Magic
- Ils ne peuvent **pas** apparaÃ®tre sur des Rare ou Crafted
- Exemple : Jeweler's (4 sockets), certains +3 skill tree prefixes

## 13.4 Staffmods et Automods

### Staffmods

Les staffmods sont des bonus de compÃ©tences **intrinsÃ¨ques** Ã  certains types d'items, gÃ©nÃ©rÃ©s comme des propriÃ©tÃ©s de base (comme la durabilitÃ©) et non comme des affixes :

| Type d'item | Skills possibles |
|-------------|-----------------|
| Wands | Skills NÃ©cromancien |
| Staves | Skills SorciÃ¨re |
| Scepters | Skills Paladin |
| Claws (assassin) | Skills Assassin |
| Orbs | Skills SorciÃ¨re |
| Druid helms | Skills Druide |
| Barbarian helms | Skills Barbare |

Les staffmods **ne sont pas supprimÃ©s** par les runewords â€” un item base avec de bons staffmods les conserve.

### Automods

Les automods sont des propriÃ©tÃ©s automatiques dÃ©finies par `auto prefix` dans weapons.txt/armor.txt :
- Orbs â†’ bonus mana
- NÃ©cromancien heads â†’ poison damage
- Paladin shields â†’ rÃ©sistances

---

# 14. SYSTÃˆME DE SOCKETS

## 14.1 DÃ©termination du nombre de sockets (items normaux/supÃ©rieurs)

### Items Ã©ligibles aux sockets

Seuls ces types peuvent avoir des sockets :
- Casques
- Boucliers
- Armures corporelles
- Armes (sauf armes de jet)

### ProbabilitÃ© d'avoir des sockets (normal/superior)

```
1/3 de tous les items normaux et supÃ©rieurs sont gÃ©nÃ©rÃ©s socketed.
Les items low quality ne peuvent PAS avoir de sockets.
```

### Nombre de sockets : ItemTypes.txt

Le fichier ItemTypes.txt dÃ©finit 3 caps par type d'item :

| Colonne | Applicable si |
|---------|---------------|
| `MaxSock1` | ilvl â‰¤ 25 |
| `MaxSock25` | ilvl 26-40 |
| `MaxSock40` | ilvl â‰¥ 41 |

Le nombre de sockets est un random dans `[1, min(MaxSockX, gemsockets)]` :
- `MaxSockX` = cap par ilvl du type (ItemTypes.txt)
- `gemsockets` = cap absolu de l'item spÃ©cifique (weapons.txt / armor.txt)

### Exemple : Crystal Sword

```
gemsockets (weapons.txt) = 6
Type = "swor" â†’ MaxSock1=3, MaxSock25=4, MaxSock40=6

ilvl 1-25  : random [1, min(3, 6)] = [1, 3] sockets
ilvl 26-40 : random [1, min(4, 6)] = [1, 4] sockets
ilvl 41+   : random [1, min(6, 6)] = [1, 6] sockets
```

### Exemple : Monarch Shield

```
gemsockets (armor.txt) = 4
Type = "shie" â†’ MaxSock1=3, MaxSock25=3, MaxSock40=4

ilvl 1-25  : random [1, min(3, 4)] = [1, 3] sockets
ilvl 26-40 : random [1, min(3, 4)] = [1, 3] sockets
ilvl 41+   : random [1, min(4, 4)] = [1, 4] sockets
```

## 14.2 Larzuk (quÃªte Siege on Harrogath)

### RÃ¨gles de Larzuk

| QualitÃ© de l'item | Sockets ajoutÃ©s |
|-------------------|----------------|
| **Normal (white/grey)** | Maximum possible = `min(MaxSockX_pour_ilvl, gemsockets)` |
| **Superior** | Maximum possible (mÃªme formule que normal) |
| **Magic** | 1 ou 2 sockets (50/50) |
| **Rare** | 1 socket (toujours) |
| **Set** | 1 socket (toujours) |
| **Unique** | 1 socket (toujours) |
| **Crafted** | 1 socket (toujours) |

Larzuk est **dÃ©terministe** pour les items normaux : toujours le maximum.

### Utilisations

3 utilisations par personnage : 1 en Normal, 1 en Nightmare, 1 en Hell.

## 14.3 Recettes Cube pour sockets

### Recettes pour items normaux (non-socketed, non-superior)

| Type | Recette | Range de sockets |
|------|---------|-----------------|
| **Arme** | Ral + Amn + Perfect Amethyst + arme | 1-6 (alÃ©atoire) |
| **Armure corporelle** | Tal + Thul + Perfect Topaz + armure | 1-4 (alÃ©atoire) |
| **Casque** | Ral + Thul + Perfect Sapphire + casque | 1-3 (alÃ©atoire) |
| **Bouclier** | Tal + Amn + Perfect Ruby + bouclier | 1-4 (alÃ©atoire) |

### Algorithme cube socket

```
Le cube roll un nombre entre 1 et 6 (distribution uniforme).
Si le rÃ©sultat dÃ©passe le maximum de l'item â†’ clamp au maximum.

Exemple : Claws (max 3 sockets)
  Roll 1 â†’ 1 socket  (1/6)
  Roll 2 â†’ 2 sockets (1/6)
  Roll 3 â†’ 3 sockets (1/6)
  Roll 4 â†’ 3 sockets (1/6)  // clamp
  Roll 5 â†’ 3 sockets (1/6)  // clamp
  Roll 6 â†’ 3 sockets (1/6)  // clamp
  
  RÃ©sultat : 1/6 â†’ 1os, 1/6 â†’ 2os, 4/6 â†’ 3os
```

### Recette socket pour items Rare

```
3Ã— Perfect Skulls + Stone of Jordan + item Rare â†’ item Rare avec 1 socket
(DÃ©truit les stats existantes et re-roll le Rare, puis ajoute 1 socket)
```

## 14.4 Sockets sur items Rare (natifs)

Les items Rare peuvent avoir des sockets naturels si l'affix **Mechanist's** (prefix) est sÃ©lectionnÃ© :
- 1 ou 2 sockets (50/50)
- ClampÃ© au maximum du type de base
- Exemple : Rare Buckler avec Mechanist's â†’ toujours 1 socket (max sockets buckler = 1)

## 14.5 Astuce Low Quality â†’ Normal â†’ Socket

```
1. Trouver un item Low Quality elite (ex : Crude Phase Blade)
2. Cube : El + Chipped Gem + item Low Quality â†’ item Normal de ilvl 1
3. Larzuk : socket l'item â†’ min(MaxSock1, gemsockets) sockets
   Pour Phase Blade : min(3, 6) = 3 sockets (au lieu de 6 avec un ilvl Ã©levÃ©)
4. Utile pour les runewords Ã  3 sockets dans un Phase Blade
```

---

# 15. ITEMS Ã‰THÃ‰RÃ‰S

## 15.1 ProbabilitÃ© et Ã©ligibilitÃ©

### Chance de base

```
P(Ã‰thÃ©rÃ©) = 5% (1/20) pour tous les items Ã©ligibles
IndÃ©pendant du Magic Find et de tout autre modificateur.
Le roll est effectuÃ© APRÃˆS la dÃ©termination de la qualitÃ©.
```

### Items qui NE PEUVENT PAS Ãªtre Ã©thÃ©rÃ©s

| CatÃ©gorie | Raison |
|-----------|--------|
| **Items Set** | Flag Ã©thÃ©rÃ© dÃ©sactivÃ© dans le code |
| **Anneaux** | Pas de flag Ã©thÃ©rÃ© |
| **Amulettes** | Pas de flag Ã©thÃ©rÃ© |
| **Charmes** | Pas de flag Ã©thÃ©rÃ© (sauf exceptions : SoJ, Annihilus, Hellfire Torch, Gheed's â†’ toujours Ã©thÃ©rÃ©s) |
| **Arcs (Bows)** | Flag Ã©thÃ©rÃ© dÃ©sactivÃ© |
| **ArbalÃ¨tes (Crossbows)** | Flag Ã©thÃ©rÃ© dÃ©sactivÃ© |
| **Items Crafted** | Flag Ã©thÃ©rÃ© dÃ©sactivÃ© |
| **Phase Blade** | Indestructible par nature â†’ pas d'Ã©thÃ©rÃ© |
| **Items Low Quality** | Flag Ã©thÃ©rÃ© impossible |

### Items toujours Ã©thÃ©rÃ©s

Certains uniques sont **toujours** Ã©thÃ©rÃ©s :
- Stone of Jordan
- Annihilus
- Hellfire Torch
- Gheed's Fortune
- Ghostflame (unique War Sword) â€” Ã©thÃ©rÃ© ET indestructible

## 15.2 Bonus Ã©thÃ©rÃ©

| Bonus | Valeur |
|-------|--------|
| **DÃ©gÃ¢ts de base** | +50% (multiplicateur sur min/max damage) |
| **DÃ©fense de base** | +50% (multiplicateur sur min/max defense) |
| **DurabilitÃ©** | -50% (arrondi infÃ©rieur) puis -1 supplÃ©mentaire |
| **Requirements STR/DEX** | -10 chacun |
| **RÃ©paration** | Impossible (aucun PNJ ne peut rÃ©parer un item Ã©thÃ©rÃ©) |
| **Valeur marchande** | -75% ou plus |

### Formule durabilitÃ© Ã©thÃ©rÃ©e

```
eth_durability = floor(base_durability / 2) - 1
Si eth_durability < 1 â†’ eth_durability = 1
```

### Interactions spÃ©ciales

| Situation | Comportement |
|-----------|-------------|
| **Ã‰thÃ©rÃ© sur Mercenaire** | La durabilitÃ© ne diminue PAS quand Ã©quipÃ© sur un mercenaire |
| **Ã‰thÃ©rÃ© + Zod Rune** | L'item devient indestructible (durabilitÃ© ne diminue plus jamais) |
| **Ã‰thÃ©rÃ© + Self-repair mod** | L'item se rÃ©pare automatiquement (mod "Repairs 1 durability in X seconds") |
| **Ã‰thÃ©rÃ© + "Indestructible" mod** | L'item ne perd pas de durabilitÃ© |
| **Ã‰thÃ©rÃ© vendu Ã  un PNJ** | Item dÃ©truit immÃ©diatement, ne peut PAS Ãªtre rachetÃ© |

## 15.3 Rune Zod

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| **Effet** | "Indestructible" â€” annule toute perte de durabilitÃ© |
| **Level requirement** | 69 (ou le level req de l'item si supÃ©rieur) |
| **RaretÃ©** | 2Ã¨me item le plus rare du jeu |
| **Usage principal** | Socketer dans un item Ã©thÃ©rÃ© pour le rendre permanent |

---

# 16. RÃ‰CAPITULATIF DES FORMULES â€” RÃ‰FÃ‰RENCE RAPIDE

## 16.1 Quality Roll

```
Chance = (BaseChance - (ilvl - qlvl) / Divisor) Ã— 128
Chance = Chance Ã— 100 / (100 + EMF)
if Chance < MinChance â†’ Chance = MinChance
FinalChance = Chance - (Chance Ã— QualityFactor / 1024)
P(qualitÃ©) = 128 / FinalChance
```

## 16.2 Effective Magic Find

```
EMF_unique = MF Ã— 250 / (MF + 250)
EMF_set    = MF Ã— 500 / (MF + 500)
EMF_rare   = MF Ã— 600 / (MF + 600)
EMF_magic  = MF
```

## 16.3 Affix Level

```
if ilvl > 99 â†’ ilvl = 99
if qlvl > ilvl â†’ ilvl = qlvl
if magic_lvl > 0 â†’ alvl = ilvl + magic_lvl
else if ilvl < (99 - qlvl/2) â†’ alvl = ilvl - qlvl/2
else â†’ alvl = 2Ã—ilvl - 99
if alvl > 99 â†’ alvl = 99
```

## 16.4 NoDrop Multijoueur

```
N = int(1 + OtherPlayers/2 + ClosePartyMembers/2)
NewNoDrop = int(ProbSum / (1/((NoDrop/(NoDrop+ProbSum))^N) - 1))
```

## 16.5 Sockets

```
max_sockets = min(MaxSockX[ilvl_bracket], gemsockets)
  oÃ¹ X = 1 si ilvlâ‰¤25, 25 si ilvlâ‰¤40, 40 si ilvlâ‰¥41

Larzuk (Normal/Superior) : toujours max_sockets
Larzuk (Magic) : random(1, 2)
Larzuk (Rare/Set/Unique/Crafted) : toujours 1
Cube (Normal) : random(1, 6) clampÃ© Ã  max_sockets
```

## 16.6 Ã‰thÃ©rÃ©

```
P(ethereal) = 5% (flat, non-modifiable)
DÃ©gÃ¢ts/DÃ©fense Ã©thÃ©rÃ© = base Ã— 1.5
DurabilitÃ© Ã©thÃ©rÃ©e = floor(base/2) - 1
STR/DEX req Ã©thÃ©rÃ© = base - 10
```

## 16.7 Downgrade

```
Unique fail â†’ Rare Ã— 3 durabilitÃ©
Set fail    â†’ Magic Ã— 2 durabilitÃ©
Duplicate unique (nolimit=0) â†’ Rare Ã— 3 durabilitÃ©
```

---

## 17. Transposition MGE â€” SystÃ¨me de loot Allumina

### Architecture recommandÃ©e

```rust
// Plugin MGE : mge-plugin-loot-engine.v1
// Ã‰quivalent de TreasureClassEx.txt + ItemRatio.txt + affixation

pub struct TreasureClass {
    pub name: String,
    pub picks: i32,                    // nÃ©gatif = garanti
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
    // Test dans l'ordre : Unique â†’ Set â†’ Rare â†’ Magic â†’ Superior â†’ Normal â†’ Low
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

## 18. RÃ©fÃ©rences (complÃ©mentaires au Â§9)

| Document | RÃ´le |
|----------|------|
| [PureDiablo â€” Item Generation](https://www.purediablo.com/diablo-2/item-generation) | Guide complet de gÃ©nÃ©ration d'items (base de ce chapitre) |
| [The Amazon Basin â€” Item Drop Procedure](https://theamazonbasin.com/wiki/index.php?title=Diablo_II_Item_Drop_Procedure) | ProcÃ©dure de drop complÃ¨te avec ordre des vÃ©rifications |
| [Phrozen Keep â€” TreasureClassEx.txt](https://d2mods.info/forum/viewtopic.php?t=67310) | Documentation NoDrop et Picks |
| [Phrozen Keep â€” ItemRatio.txt](https://d2mods.info/forum/kb/viewarticle?a=320) | Guide du fichier ItemRatio.txt |
| [GitHub fabd/diablo2 â€” ItemRatio.txt](https://github.com/fabd/diablo2/blob/master/code/d2_113_data/ItemRatio.txt) | DonnÃ©es brutes ItemRatio.txt v1.13 |
| [Phrozen Keep â€” UniqueItems.txt](https://d2mods.info/forum/viewtopic.php?t=38595) | Guide Rarity et nolimit des Uniques |
| [PureDiablo â€” Magic Find Diminishing Returns](https://www.purediablo.com/diablo-2/magic-find-diminishing-returns) | Formules EMF |
| [Arreat Summit â€” Items Basics](https://classic.battle.net/diablo2exp/items/basics.shtml) | RÃ©fÃ©rence officielle Blizzard |
| [diablo2.io â€” Larzuk Calculator](https://diablo2.io/larzuksockets.php) | Calculateur de sockets Larzuk |

---

**Document** : Allumina â€” Analyse Technique Diablo II pour MGE  
**Version** : 2.1  
**Date** : 2026-02-22  
**Statut** : Document de rÃ©fÃ©rence technique  
**Changelog** : v2.1 â€” Ajout section 9 (GÃ©nÃ©ration procÃ©durale de cartes : seed, DT1/DS1, DRLG, Levels.txt, waypoints) | v2.0 â€” Ajout sections 10-18 (systÃ¨me complet de gÃ©nÃ©ration d'items)


