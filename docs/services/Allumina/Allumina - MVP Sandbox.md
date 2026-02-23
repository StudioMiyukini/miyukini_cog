# Allumina — MVP Sandbox

## Contexte

Ce document définit le **Minimum Viable Product** d'Allumina : la plus petite version jouable, stable et cohérente du sandbox, implémentable en **3–6 mois**, supportant **50–200 joueurs simultanés** sur un Lobby MWS unique.

Ce n'est **pas** le moteur complet décrit dans le [Blueprint Moteur Sandbox MGE](./Allumina%20-%20Blueprint%20Moteur%20Sandbox%20MGE.md). C'est une **première itération fonctionnelle** qui prouve les boucles de gameplay fondamentales et sert de fondation évolutive.

## Portée / Scope

- **Applicable à :** Plan d'implémentation, sprint planning, priorisation.
- **Audience :** Développement, game design, architecture.
- **Statut :** Spécification MVP normative.

## Principe directeur

> Chaque système inclus dans le MVP doit répondre à **au moins une** de ces questions par OUI :
> 1. Est-ce nécessaire pour que le joueur puisse **se déplacer et combattre** ?
> 2. Est-ce nécessaire pour que le joueur puisse **progresser** ?
> 3. Est-ce nécessaire pour que le monde **persiste** ?
> 4. Est-ce nécessaire pour que les joueurs **interagissent** entre eux ?

Tout le reste est **post-MVP**.

---

# I — PÉRIMÈTRE : CE QUI ENTRE, CE QUI N'ENTRE PAS

## Inclus dans le MVP

| Système | Justification |
|---------|---------------|
| **Game loop ECS** (30 TPS) | Fondation — sans ça, rien ne tourne |
| **Carte fixe** (tilemap 2D) | Le joueur a besoin d'un monde |
| **Mouvement** (clic-déplacement, A*) | Le joueur doit se déplacer |
| **Combat mêlée temps réel** | Boucle de jeu primaire (Diablo-like) |
| **IA monstres** (FSM basique) | Le joueur a besoin d'ennemis |
| **Spawn monstres** (fixe, timer) | Le monde doit contenir des monstres |
| **Troupe basique** (échelle Groupe) | Différenciateur Allumina, sensation de puissance |
| **Stats personnage** (10 caracs, aptitudes) | Fondation progression |
| **Compétences** (gain par usage, simplifié) | Boucle de progression |
| **Loot** (drop monstres, qualité simple) | Récompense de jeu |
| **Inventaire + équipement** | Le joueur doit porter des objets |
| **Récolte** (nœuds fixes, respawn timer) | Source de matériaux |
| **Craft** (recettes directes, pas de chaîne) | Transformer matériaux en équipement |
| **Or + NPC marchands** | Économie minimale |
| **Trade joueur-joueur** | Interaction sociale fondamentale |
| **Réseau** (Lobby MWS, serveur autoritaire) | Multijoueur |
| **Persistence** (snapshot périodique) | Le monde doit survivre aux redémarrages |
| **Solo** (hors-ligne, sauvegarde locale) | LOI-1, LOI-2 |

## Exclu du MVP (post-MVP)

| Système | Raison de l'exclusion | Quand |
|---------|----------------------|-------|
| **Housing / Territorial Engine** | Complexe (multi-tile, decay, ACL, impôts) | v0.2 |
| **Simulation écologique** (Lotka-Volterra) | Non critique pour jouer — respawn fixe suffit | v0.3 |
| **Chaînes de production** (craft multi-étape) | Recettes directes suffisent pour le MVP | v0.2 |
| **Nations NPC / Régions dynamiques** | 1 seule zone suffit pour tester | v0.3 |
| **Guerre de nations** | Pas de nations → pas de guerre | v0.4 |
| **Gradient de loi / PvP zones** | PvP consensuel (duel) suffit au MVP | v0.2 |
| **Enseignement joueur→joueur** | Gain par usage suffit pour progresser | v0.2 |
| **Régulation inflation automatique** | 50–200 joueurs → pas d'inflation critique | v0.3 |
| **Timer Wheel hiérarchique** | Simple timer queue suffit à 50–200 joueurs | v0.2 |
| **Persistence incrémentale** | Snapshot complet suffit à cette échelle | v0.2 |
| **Cluster multi-node** | Mono-serveur suffit pour 200 joueurs | v0.4+ |
| **Magie** (sorts, mana, écoles) | Trop de contenu — mêlée + tir suffisent | v0.2 |
| **Compagnie+ / ordres tactiques** | Groupe (suivi simple) suffit | v0.2 |
| **Dressage d'animaux** | Système social secondaire | v0.3 |
| **Caravanes** | Pas de régions multiples | v0.3 |
| **Guildes** | Trade direct suffit au MVP | v0.2 |
| **Karma / réputation / statut social** | Pas de PvP complexe | v0.2 |

---

# II — ARCHITECTURE TECHNIQUE MVP

## Stack

| Couche | Choix |
|--------|-------|
| **Moteur** | MGE (Rust, ECS pur) |
| **Rendu** | 2D isométrique, tilemap |
| **Réseau** | Protocole Allumina sur transport MWS (Lobby) |
| **Persistence** | KindMother (solo) / fichier JSON + binaire (Lobby hôte) |
| **Config** | JSON data-driven (rechargeable) |

## Game loop

```
┌──────────────────────────────────────────┐
│            GAME LOOP (30 TPS)             │
│                                            │
│  Phase 10  : Timer tick (simple queue)     │
│  Phase 50  : Network input                 │
│  Phase 100 : Movement + Pathfinding        │
│  Phase 200 : Combat resolution             │
│  Phase 300 : Skill checks + gain           │
│  Phase 400 : Harvest + Craft               │
│  Phase 500 : AI tick                       │
│  Phase 600 : Spawn system                  │
│  Phase 700 : Loot + Inventory              │
│  Phase 800 : Economy (trade, vendor)       │
│  Phase 900 : Persistence (save check)      │
│  Phase 950 : Network output                │
└──────────────────────────────────────────┘
```

## Diagramme d'architecture

```
┌────────────────────────────────────────────────┐
│            LOBBY ALLUMINA (COG Hôte)            │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │   ECS    │  │  Event   │  │  Timer Queue │  │
│  │  World   │  │   Bus    │  │  (simple)    │  │
│  └────┬─────┘  └────┬─────┘  └──────┬───────┘  │
│       │              │               │           │
│  ┌────┴──────────────┴───────────────┴────────┐ │
│  │              SYSTEMS (par phase)            │ │
│  │  movement, combat, ai, spawn, skill,       │ │
│  │  harvest, craft, loot, economy, persist    │ │
│  └────────────────────────────────────────────┘ │
│       │                              │           │
│  ┌────┴──────┐              ┌───────┴────────┐  │
│  │ Spatial   │              │   Network      │  │
│  │ Index     │              │   (MWS Lobby)  │  │
│  │ (sectors) │              │                │  │
│  └───────────┘              └────────────────┘  │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │     Persistence (KindMother / fichier)    │   │
│  └──────────────────────────────────────────┘   │
└────────────────────────────────────────────────┘

Clients (COGs joueurs) :
  → Envoi : inputs (mouvement, attaque, interaction)
  → Réception : état monde (deltas entités visibles)
  → Prédiction : mouvement local uniquement
```

---

# III — SPÉCIFICATION PAR SYSTÈME

---

## 1. CARTE ET MONDE

### Ce qu'on implémente

- **Tilemap fixe** : 512×512 tiles (ou plus petit pour commencer : 256×256)
- **Tiles** : sol, eau, mur, obstacle destructible
- **Statics** : arbres, rochers, bâtiments NPC (non joueur)
- **1 seule zone** : pas de régions, pas de facettes, pas de nations
- **Tile flags** : walkable, blockable, water, harvestable

### Ce qu'on coupe

- Pas de génération procédurale (carte faite à la main ou éditeur)
- Pas de climat / saisons
- Pas de zones multiples
- Pas de transition de zone

### Components

```rust
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

pub struct Tile {
    pub graphic_id: u16,
    pub altitude: i8,
    pub flags: TileFlags,
}

bitflags! {
    pub struct TileFlags: u16 {
        const WALKABLE    = 0x01;
        const WATER       = 0x02;
        const HARVESTABLE = 0x04;
        const BLOCKED     = 0x08;
    }
}
```

### Justification

Sans carte, rien n'existe. Une tilemap fixe est le minimum absolu et peut être créée avec un éditeur simple (ou même un fichier JSON).

---

## 2. MOUVEMENT ET PATHFINDING

### Ce qu'on implémente

- **Clic-déplacement** (point-and-click, style Diablo)
- **A* pathfinding** sur la grille de tiles
- **Collision** : entités bloquent le passage (taille 1 tile)
- **Vitesse** : déterminée par `Agi` du personnage
- **Prédiction client** : le client déplace le joueur localement, le serveur réconcilie

### Ce qu'on coupe

- Pas de formations de troupe (suivants marchent derrière)
- Pas de collision poussée entre entités (simple blocage)
- Pas de véhicules / montures (post-MVP)

### Components

```rust
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Velocity {
    pub speed: f32,          // tiles/sec, dérivé de Agi
}

pub struct PathState {
    pub path: Vec<(u32, u32)>,
    pub current_index: usize,
    pub target: Option<(f32, f32)>,
}

pub struct Collision {
    pub radius: f32,         // 0.5 tile = standard
    pub blocking: bool,
}
```

### Formule de vitesse

```
base_speed = 3.0 tiles/sec
speed = base_speed + (Agi - 5) × 0.2
// Agi 1 → 2.2 tiles/sec, Agi 5 → 3.0, Agi 10 → 4.0
```

### System

```rust
// @phase 100
pub fn movement_system(world: &mut World, ctx: &mut Context) {
    // Pour chaque entité avec Position + PathState + Velocity :
    // 1. Si path non vide : avancer vers le prochain waypoint
    // 2. Vérifier collision avec la grille (tile flags)
    // 3. Vérifier collision avec autres entités (spatial index)
    // 4. Mettre à jour Position
    // 5. Si destination atteinte : émettre ArrivedEvent
}
```

---

## 3. COMBAT MÊLÉE TEMPS RÉEL

### Ce qu'on implémente

La séquence d'attaque définie dans [Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) section 6, simplifiée :

```
Phase 1 — Toucher : atk (assaillant) vs esq (défenseur)
  → 50% + 1% par point d'écart
  → Échec → attaque ratée
  → Succès → Phase 2

Phase 2 — Parade : atk (assaillant) vs par (défenseur)
  → 50% + 1% par point d'écart
  → Parade → dégâts absorbés par bouclier/arme (résistance)
  → Pas de parade → dégâts aux PV

Dégâts = For × multiplicateur_arme × (1 - AR_type / 100)
Critique = jet ≤ Luk → dégâts × 1.5
```

- **3 types de dégâts** : Tranchant (Tranc), Contondant (Cont), Perforant (Perc)
- **3 types d'armure** : ARt, ARc, ARp
- **Vitesse d'attaque** : cooldown entre coups = 1.0 / (atk_speed / 50)
- **Portée** : 1.5 tiles (mêlée)

### Ce qu'on implémente aussi : tir à distance (simplifié)

- **Un seul type de tir** pour le MVP : `tir` (pas de distinction corde/poing/épaule)
- Portée : 6 tiles
- Projectile comme entité avec vitesse + collision
- Dégâts calculés comme la mêlée (tir vs esq, puis dégâts)

### Ce qu'on coupe

- Pas de magie (post-MVP v0.2)
- Pas de lancer d'objets (jet)
- Pas d'effets spéciaux (poison, feu, stun — sauf mort)
- Pas de combo system

### Components

```rust
pub struct CombatStats {
    pub atk: f64,
    pub esq: f64,
    pub par: f64,
    pub atk_speed: f64,
    pub damage_base: f64,
    pub damage_type: DamageType,
}

pub enum DamageType { Tranchant, Contondant, Perforant }

pub struct Health {
    pub current: f64,
    pub max: f64,                // (For + Con) × 10
    pub regen_rate: f64,         // PV/sec (simple, pas d'End pour le MVP)
}

pub struct Armor {
    pub ar_tranchant: f64,       // % réduction
    pub ar_contondant: f64,
    pub ar_perforant: f64,
    pub resistance: f64,         // points de résistance pièce (décroît)
}

pub struct AttackCooldown {
    pub remaining: f32,
    pub base_interval: f32,      // 1.0 / (atk_speed / 50)
}

pub struct DeadTag;              // marqueur d'entité morte
```

### System

```rust
// @phase 200
pub fn combat_system(world: &mut World, ctx: &mut Context) {
    for event in world.events::<AttackRequest>() {
        let attacker = world.get::<CombatStats>(event.attacker);
        let defender_esq = world.get::<CombatStats>(event.target).esq;
        let defender_par = world.get::<CombatStats>(event.target).par;
        let defender_armor = world.get::<Armor>(event.target);
        let luk = world.get::<CharacterStats>(event.attacker).characteristics[9]; // Luk
        
        // Phase 1 : toucher (atk vs esq)
        let hit_chance = 50.0 + (attacker.atk - defender_esq);
        let roll = random_d100() - luk;
        
        if roll > hit_chance { continue; } // raté
        
        // Critique ?
        let is_crit = roll <= luk;
        
        // Phase 2 : parade (atk vs par)
        let parry_chance = 50.0 + (defender_par - attacker.atk);
        let parry_roll = random_d100();
        let parried = parry_roll <= parry_chance;
        
        // Dégâts
        let mut damage = attacker.damage_base;
        if is_crit { damage *= 1.5; }
        
        let ar = match attacker.damage_type {
            Tranchant => defender_armor.ar_tranchant,
            Contondant => defender_armor.ar_contondant,
            Perforant => defender_armor.ar_perforant,
        };
        damage *= 1.0 - (ar / 100.0);
        
        if parried {
            // Dégâts absorbés par résistance de l'arme/bouclier
            defender_armor.resistance -= damage;
            if defender_armor.resistance < 0.0 { defender_armor.resistance = 0.0; }
        } else {
            // Dégâts aux PV
            let health = world.get_mut::<Health>(event.target);
            health.current -= damage;
            if health.current <= 0.0 {
                world.add_component(event.target, DeadTag);
                emit EntityDeathEvent { entity: event.target, killer: event.attacker };
            }
        }
        
        emit DamageEvent { target: event.target, amount: damage, crit: is_crit };
    }
}
```

---

## 4. IA MONSTRES

### Ce qu'on implémente

FSM simple à 5 états :

```
┌────────┐  joueur dans rayon  ┌────────┐
│  IDLE  │────────────────────→│ CHASE  │
│        │←────────────────────│        │
└────────┘  joueur hors leash  └───┬────┘
                                   │ à portée d'attaque
                                   ↓
                              ┌────────┐
                              │ ATTACK │
                              │        │←──┐
                              └───┬────┘   │ cible vivante
                                  │        │ + à portée
                                  └────────┘
                                  │ cible morte ou hors portée
                                  ↓
                              ┌────────┐
                              │ RETURN │ → retour au spawn point
                              └───┬────┘
                                  │ arrivé
                                  ↓
                              ┌────────┐
                              │  IDLE  │
                              └────────┘

Mort → EntityDeathEvent → loot drop → respawn timer
```

- **Aggro radius** : configurable par type de monstre (ex: 8 tiles)
- **Leash radius** : 2× aggro radius — au-delà, le monstre retourne
- **Ciblage** : cible la plus proche en champ de vision, cumul max 4 ennemis par cible (section 6.1 du doc Caracs)
- **Pas de group behavior** au MVP — chaque monstre est indépendant

### Ce qu'on coupe

- Pas de boss avec IA spéciale (même FSM, juste des stats plus hautes)
- Pas de flee / retraite
- Pas de coordination de groupe
- Pas de patrouille (idle = stationnaire)

### Components

```rust
pub struct AIState {
    pub state: CreatureState,
    pub spawn_point: Position,
    pub aggro_radius: f32,
    pub leash_radius: f32,
    pub target: Option<EntityId>,
    pub attack_range: f32,
}

pub enum CreatureState {
    Idle, Chase, Attack, Return,
}

pub struct MonsterDef {
    pub type_id: u32,
    pub name: String,
    pub level: u32,
    pub stats: CombatStats,
    pub health: f64,
    pub aggro_radius: f32,
    pub loot_table: LootTableId,
    pub xp_value: u32,          // pas d'XP dans Allumina, mais sert de poids pour le skill gain
}
```

---

## 5. SPAWN MONSTRES

### Ce qu'on implémente

- **Spawners fixes** : position + type + quantité + rayon de dispersion + respawn timer
- Configurés dans le fichier de carte (JSON)
- Un spawner gère N monstres maximum
- Quand un monstre meurt : respawn timer (ex: 120s)

### Ce qu'on coupe

- Pas de densité dynamique
- Pas de seed-based generation
- Pas de champion / élite / affixes (juste des monstres normaux avec des stats variées)

### Components

```rust
pub struct Spawner {
    pub monster_type: u32,
    pub position: Position,
    pub radius: f32,
    pub max_count: u32,
    pub current_count: u32,
    pub respawn_delay_sec: f32,
    pub pending_respawns: Vec<f32>,  // timers restants
}
```

---

## 6. TROUPES (échelle Groupe)

### Ce qu'on implémente

Le différenciateur d'Allumina dès le MVP. Un joueur peut recruter 1–5 suivants (troupes PNJ) selon son `Cmd` :

- **Comportement** : suivi automatique (comme mercenaires Diablo 2)
- **Combat** : attaquent en cas d'agression (pas d'attaque à vue, conforme au doc Combat et Troupes section 6.2)
- **Balise** : clic droit → suivants se dirigent vers le point balisé, puis reprennent le suivi
- **Pool Cmd** : `Cmd ≤ Cha × 10 + 20`, chaque troupe coûte X pts Cmd

### Ce qu'on coupe

- Pas de formations (Compagnie+)
- Pas de sous-groupes (types)
- Pas de vue RTS
- Pas de nécromancie
- Pas de dressage d'animaux

### Components

```rust
pub struct TroopOwner {
    pub cmd_value: f64,
    pub cmd_used: f64,
    pub troops: Vec<EntityId>,
    pub rally_point: Option<Position>,  // balise clic droit
}

pub struct TroopFollower {
    pub owner: EntityId,
    pub troop_type: u32,
    pub cmd_cost: f64,
    pub follow_distance: f32,    // 2–4 tiles derrière l'owner
    pub state: TroopState,
}

pub enum TroopState {
    Following,        // suit le joueur
    MovingToRally,    // se dirige vers la balise
    Fighting,         // engagé en combat
    Returning,        // retour vers le joueur après combat
}
```

### System

```rust
// @phase 510
pub fn troop_follow_system(world: &mut World, ctx: &mut Context) {
    for (follower, troop) in world.query::<(&mut Position, &TroopFollower)>() {
        let owner_pos = world.get::<Position>(troop.owner);
        
        match troop.state {
            Following => {
                // Suivre le joueur à follow_distance
                // Utiliser A* si distance > 3 tiles
                // Sinon mouvement direct
            }
            MovingToRally => {
                let rally = world.get::<TroopOwner>(troop.owner).rally_point;
                // Se déplacer vers la balise
                // À l'arrivée → retour en Following
            }
            Fighting => {
                // IA combat (même FSM que les monstres, mais déclenchée par agression)
                // Si plus de cible → Returning
            }
            Returning => {
                // Retour vers le joueur
                // Si distance < follow_distance → Following
            }
        }
    }
}
```

### Recrutement (MVP)

- PNJ recruteurs fixes dans le monde (comme des marchands)
- Le joueur paie de l'or → reçoit une troupe si Cmd suffisant
- Types MVP : **Milicien** (coût Cmd 5, stats faibles) et **Garde** (coût Cmd 10, stats moyennes)

---

## 7. STATS ET PROGRESSION

### Ce qu'on implémente

Les 10 caractéristiques (For, Con, Agi, Dex, Per, Vol, Int, Sag, Cha, Luk) telles que définies dans le doc normé.

**Formules à la création :**

| Statistique | Formule |
|-------------|---------|
| PV max | (For + Con) × 10 |
| End max | (For + Con × 2) × 10 |
| Aggro | Con + For |
| Pds max | (For + Con) × 5 |
| atk | Dex × 10 |
| atk speed | Agi × 10 |
| par | (For + Con) / 2 × 10 |
| esq | Agi × 10 |
| tir | (Dex + Per) / 2 × 10 |

**Compétences MVP** (sous-ensemble du doc complet) :

| Compétence | Carac | Justification MVP |
|------------|-------|-------------------|
| **Minage** | For | Récolte minerais |
| **Bûcheronnage** | For | Récolte bois |
| **Mécanique** (craft) | Sag | Fabriquer des objets |
| **Marchandage** | Cha | Prix achat/vente chez NPC |
| **Commandement** (Cmd) | Cha | Pool troupes |
| **Athlétisme** | Con | Vitesse déplacement bonus |
| **Combat** (aptitude, pas compétence) | — | Monte en combattant |

### Gain par usage (simplifié)

```
fn try_skill_gain(character, skill_id, difficulty, success):
    let skill = character.skills[skill_id]
    let cap = get_cap(character, skill_id)  // carac × 10 + 20
    
    if skill.base >= cap: return  // au plafond
    
    // Chance de gain simplifiée
    let room = (cap - skill.base) / cap
    let gc = room * 0.25 * if success { 1.0 } else { 0.3 }
    
    if random() < gc:
        skill.base += 0.1
        emit SkillGainEvent
```

Pas de GGS, pas d'anti-macro, pas d'enseignement au MVP. Le gain par usage seul suffit pour 3–6 mois de jeu.

### Ce qu'on coupe

- PM (mana) — pas de magie au MVP
- End consommation détaillée (End existe mais simplifié : surcharge seulement)
- Enseignement joueur→joueur
- Anti-macro (50–200 joueurs → supervision manuelle)

---

## 8. LOOT ET INVENTAIRE

### Ce qu'on implémente

- **Drop table par monstre** : liste d'items possibles avec probabilités
- **Qualité simplifiée** : Normal, Bon, Excellent (3 niveaux, pas de spectre continu)
- **Inventaire** : grille simple (20 slots)
- **Équipement** : 6 slots (tête, torse, jambes, main droite, main gauche, accessoire)
- **Poids** : chaque item a un poids, cap = Pds max du personnage
- **Or** : drop de monstres + vente NPC

### Items MVP

| Catégorie | Exemples |
|-----------|----------|
| **Armes mêlée** | Épée (Tranc), Masse (Cont), Lance (Perc) |
| **Arme à distance** | Arc (Perc) |
| **Armures** | Cuir (ARt faible), Mailles (ARc moyen), Plate (ARp haut) |
| **Consommables** | Potion de soin (restaure PV) |
| **Matériaux** | Minerai, Bois, Cuir brut |
| **Or** | Monnaie empilable |

### Components

```rust
pub struct Inventory {
    pub slots: Vec<Option<EntityId>>,
    pub capacity: usize,             // 20 pour le MVP
}

pub struct Equipment {
    pub head: Option<EntityId>,
    pub torso: Option<EntityId>,
    pub legs: Option<EntityId>,
    pub main_hand: Option<EntityId>,
    pub off_hand: Option<EntityId>,
    pub accessory: Option<EntityId>,
}

pub struct Item {
    pub type_id: ItemTypeId,
    pub quality: ItemQuality,
    pub weight: f32,
    pub stack_count: u32,
    pub stackable: bool,
}

pub enum ItemQuality { Normal, Good, Excellent }

pub struct WeaponData {
    pub damage_min: f64,
    pub damage_max: f64,
    pub damage_type: DamageType,
    pub attack_speed_modifier: f64,
    pub range: f32,
}

pub struct ArmorData {
    pub ar_tranchant: f64,
    pub ar_contondant: f64,
    pub ar_perforant: f64,
    pub resistance: f64,
}

pub struct LootTable {
    pub entries: Vec<LootEntry>,
}

pub struct LootEntry {
    pub item_type: ItemTypeId,
    pub quality_weights: [f32; 3],  // [Normal, Good, Excellent]
    pub probability: f32,            // 0.0–1.0
    pub count_range: (u32, u32),
}
```

---

## 9. RÉCOLTE (SIMPLIFIÉ)

### Ce qu'on implémente

- **Nœuds de ressource fixes** sur la carte (minerai, arbre, buisson d'herbes)
- **Skill check** : Minage ou Bûcheronnage vs difficulté du nœud
- **Banque de ressources** : chaque nœud a un stock, se vide, respawn après timer fixe
- **Outils requis** : pioche pour minage, hache pour bois
- **Gain de compétence** sur récolte

### Ce qu'on coupe

- Pas de simulation écologique (respawn fixe)
- Pas de rareté géographique (1 seule zone)
- Pas de veines multiples (un type de minerai par nœud)
- Pas de surexploitation / désertification

### Components

```rust
pub struct HarvestNode {
    pub resource_type: ResourceTypeId,
    pub current_stock: u32,
    pub max_stock: u32,
    pub respawn_delay_sec: f32,
    pub respawn_timer: Option<f32>,  // None si stock > 0
    pub required_skill: SkillId,
    pub skill_difficulty: f64,
    pub required_tool: ItemTypeId,
    pub harvest_amount: (u32, u32),  // min, max par récolte
}
```

### Nœuds MVP

| Nœud | Ressource | Skill | Outil | Stock | Respawn |
|------|-----------|-------|-------|-------|---------|
| Veine de fer | Minerai de fer | Minage 0 | Pioche | 10 | 120s |
| Veine d'acier | Minerai d'acier | Minage 40 | Pioche | 5 | 300s |
| Chêne | Bois de chêne | Bûcheronnage 0 | Hache | 8 | 90s |
| Orme dur | Bois dur | Bûcheronnage 40 | Hache | 4 | 240s |

---

## 10. CRAFT (RECETTES DIRECTES)

### Ce qu'on implémente

- **Recettes directes** : matériaux → item (pas de chaîne, pas d'intermédiaire)
- **Skill check** : Mécanique vs difficulté de la recette
- **Qualité** : déterminée par le résultat du skill check (Normal < 50%, Bon 50–80%, Excellent 80%+)
- **Station requise** : NPC "forge" dans le monde (pas de housing) — le joueur se rend à la forge et ouvre l'interface

### Ce qu'on coupe

- Pas de chaîne de production (lingot→lame→épée)
- Pas de qualité continue (3 paliers suffisent)
- Pas de réputation crafter
- Pas de signature crafter

### Recettes MVP

| Recette | Inputs | Skill min | Station |
|---------|--------|-----------|---------|
| Épée en fer | 5 minerai fer | Mécanique 0 | Forge |
| Masse en fer | 4 minerai fer | Mécanique 0 | Forge |
| Lance en fer | 3 minerai fer + 2 bois | Mécanique 10 | Forge |
| Arc simple | 3 bois | Mécanique 0 | Atelier |
| Armure cuir | 5 cuir brut | Mécanique 20 | Atelier |
| Armure mailles | 8 minerai fer | Mécanique 40 | Forge |
| Épée en acier | 5 minerai acier | Mécanique 50 | Forge |
| Potion de soin | 3 herbes | Mécanique 10 | Atelier |

### Components

```rust
pub struct CraftRecipe {
    pub id: RecipeId,
    pub name: String,
    pub inputs: Vec<(ItemTypeId, u32)>,     // (type, quantité)
    pub output: ItemTypeId,
    pub output_quantity: u32,
    pub skill_required: SkillId,
    pub skill_min: f64,
    pub station: StationType,
}

pub enum StationType { Forge, Atelier }
```

---

## 11. ÉCONOMIE (MINIMALE)

### Ce qu'on implémente

- **Or** : monnaie unique, stack empilable, drop de monstres
- **NPC marchands** : achat/vente à prix fixes (modifiés par Marchandage : 1% par point d'écart, max 50%)
- **Trade joueur-joueur** : fenêtre d'échange (les deux joueurs doivent confirmer)

### Ce qu'on coupe

- Pas de régulation inflation
- Pas de marchands joueur (vendor)
- Pas de caravanes
- Pas d'enchères
- Pas de taxe

### Formule marchandage

```
prix_effectif = prix_base × (1 - écart_marchandage / 100)
écart = joueur.marchandage - npc.marchandage
écart = clamp(écart, -50, 50)

// Ex: joueur 30, NPC 45 → écart -15 → prix × 1.15 (15% plus cher)
// Ex: joueur 45, NPC 30 → écart +15 → prix × 0.85 (15% moins cher)
```

---

## 12. RÉSEAU (LOBBY MWS)

### Ce qu'on implémente

- **Mode solo** : hors-ligne complet, sauvegarde locale KindMother
- **Mode Lobby** : un COG hôte expose un Lobby Allumina via MWS
- **Découverte** : clients interrogent le Tracker MWS → liste des Lobbys Allumina
- **Connexion** : Permis de circulation + accord d'hôte → tunnel MWS
- **Serveur autoritaire** : toutes les actions sont validées côté hôte
- **Prédiction client** : mouvement uniquement (réconciliation serveur)
- **Interest management** : envoi des entités dans un rayon de 32 tiles autour du joueur

### Protocole MVP (packets)

| Packet (client → serveur) | Contenu |
|---------------------------|---------|
| `MoveRequest` | target_x, target_y |
| `AttackRequest` | target_entity_id |
| `InteractRequest` | target_entity_id, action_type |
| `CraftRequest` | recipe_id |
| `TradeRequest` | target_player_id |
| `TradeConfirm` | trade_session_id |
| `RallyPoint` | x, y |

| Packet (serveur → client) | Contenu |
|---------------------------|---------|
| `EntitySpawn` | entity_id, type, position, components_data |
| `EntityDespawn` | entity_id |
| `EntityUpdate` | entity_id, changed_components |
| `DamageNumber` | target, amount, crit |
| `SkillGain` | skill_id, new_value |
| `InventoryUpdate` | slot, item_data |
| `TradeWindow` | items_offered, items_requested |
| `ChatMessage` | sender, message |

### Tick réseau

- **Serveur → client** : 10 updates/sec (chaque 3e tick du game loop à 30 TPS)
- **Client → serveur** : sur action (pas de polling)

---

## 13. PERSISTENCE (SNAPSHOT)

### Ce qu'on implémente

- **Sauvegarde complète** toutes les 5 minutes (pas d'incrémental au MVP)
- **Solo** : fichier local dans KindMother
- **Lobby** : fichier sur le COG hôte
- **Format** : binaire (serde + bincode) pour les entités, JSON pour la config monde

### Contenu de la sauvegarde

```
allumina_save_mvp/
├── meta.json              (version, game_time, tick_count)
├── world.bin              (tilemap — rarement modifié)
├── entities.bin           (toutes les entités : joueurs, monstres, items au sol, spawners)
└── player_characters/
    └── char_{id}.bin      (personnage exportable pour changement de Lobby)
```

### Ce qu'on coupe

- Pas de sauvegarde incrémentale
- Pas d'event sourcing
- Pas de delta compression
- La sauvegarde complète à 50–200 joueurs prend <1s → acceptable

---

# IV — CONTENU MINIMAL

## Carte MVP

```
┌───────────────────────────────────────────┐
│                                             │
│        ┌──────────┐                        │
│        │  VILLAGE │  (PNJ : marchands,     │
│        │          │   forge, atelier,      │
│        │          │   recruteur)           │
│        └──────────┘                        │
│              │                              │
│         route │                             │
│              │                              │
│    ┌─────────┴─────────┐                   │
│    │                    │                   │
│  ┌─┴────────┐   ┌──────┴───┐              │
│  │  FORÊT   │   │  MINES   │              │
│  │ (bois,   │   │ (minerai,│              │
│  │  bêtes)  │   │  monstres│              │
│  └──────────┘   └──────────┘              │
│                                             │
│         ┌──────────────┐                   │
│         │   DONJON     │                   │
│         │ (monstres    │                   │
│         │  forts, loot │                   │
│         │  rare)       │                   │
│         └──────────────┘                   │
│                                             │
└───────────────────────────────────────────┘
```

## Monstres MVP

| Type | Zone | Niveau | Aggressif | Loot |
|------|------|--------|-----------|------|
| Rat | Village (environs) | 1 | Non | Or, cuir brut |
| Loup | Forêt | 3 | Oui (rayon 6) | Or, cuir brut, herbes |
| Gobelin | Mines | 5 | Oui (rayon 8) | Or, minerai fer, arme basique |
| Bandit | Routes | 7 | Oui (rayon 10) | Or, équipement aléatoire |
| Troll | Donjon | 12 | Oui (rayon 12) | Or, minerai acier, bon équipement |
| Chef Troll | Donjon (fond) | 20 | Oui (rayon 15) | Or ×5, excellent équipement |

## NPC MVP

| NPC | Lieu | Fonction |
|-----|------|----------|
| Marchand général | Village | Achat/vente items de base |
| Forgeron | Village (forge) | Station craft forge |
| Artisan | Village (atelier) | Station craft atelier |
| Recruteur | Village | Recruter Miliciens (Cmd 20, coût 5) et Gardes (Cmd 25, coût 10) |
| Guérisseur | Village | Restaurer PV complet (gratuit) |

---

# V — PLAN D'IMPLÉMENTATION

## Phases de développement

### Phase 1 — Fondations (mois 1–2)

| Tâche | Durée estimée | Dépendances |
|-------|---------------|-------------|
| Game loop ECS (30 TPS) | 1 semaine | — |
| Tilemap + rendu 2D | 1 semaine | Game loop |
| Mouvement (clic, A*) | 1 semaine | Tilemap |
| Spatial index (sectors) | 3 jours | Tilemap |
| Stats personnage (10 caracs) | 3 jours | Game loop |
| Combat mêlée (atk/esq/par/dégâts) | 2 semaines | Stats, Mouvement |
| Tir à distance (projectile) | 1 semaine | Combat |
| Health, mort, respawn joueur | 3 jours | Combat |
| IA monstres (FSM 5 états) | 1 semaine | Combat, Mouvement |
| Spawners | 3 jours | IA |

### Phase 2 — Progression et économie (mois 2–3)

| Tâche | Durée estimée | Dépendances |
|-------|---------------|-------------|
| Inventaire + équipement | 1 semaine | Stats |
| Items (armes, armures, conso) | 1 semaine | Inventaire |
| Loot (drop tables) | 3 jours | Items, IA |
| Compétences (gain par usage) | 1 semaine | Stats |
| Récolte (nœuds + skill check) | 1 semaine | Compétences, Inventaire |
| Craft (recettes directes) | 1 semaine | Récolte, Inventaire |
| Or + NPC marchands | 3 jours | Inventaire |
| Trade joueur-joueur | 3 jours | Inventaire |

### Phase 3 — Troupes et réseau (mois 3–4)

| Tâche | Durée estimée | Dépendances |
|-------|---------------|-------------|
| Troupes (suivi, balise, combat) | 2 semaines | IA, Combat |
| Recrutement NPC | 3 jours | Troupes, Or |
| Réseau : protocole packets | 1 semaine | Game loop |
| Réseau : serveur autoritaire | 2 semaines | Protocole |
| Réseau : prédiction client (mouvement) | 1 semaine | Serveur auth |
| Réseau : interest management | 3 jours | Spatial index |
| Intégration MWS Lobby | 1 semaine | Réseau |

### Phase 4 — Persistence et polish (mois 4–5)

| Tâche | Durée estimée | Dépendances |
|-------|---------------|-------------|
| Persistence (snapshot complet) | 1 semaine | Tout |
| Solo (sauvegarde locale) | 3 jours | Persistence |
| Carte complète (village + forêt + mines + donjon) | 2 semaines | Tilemap |
| Contenu (monstres, items, recettes) | 2 semaines | Tout |
| UI minimale (inventaire, craft, stats, trade) | 2 semaines | Tout |
| Balancing (stats, dégâts, loot rates) | 1 semaine | Contenu |

### Phase 5 — Test et stabilisation (mois 5–6)

| Tâche | Durée estimée | Dépendances |
|-------|---------------|-------------|
| Tests multiplayer (10 joueurs) | 1 semaine | Phase 4 |
| Tests charge (50–200 joueurs) | 1 semaine | Tests multi |
| Fix bugs critiques | 2 semaines | Tests |
| Optimisation (si nécessaire) | 1 semaine | Tests charge |

**Total estimé : ~22 semaines (5.5 mois)**

---

# VI — CRITÈRES DE RÉUSSITE DU MVP

Le MVP est considéré comme **réussi** quand les critères suivants sont remplis :

| Critère | Mesure |
|---------|--------|
| **Jouable** | Un joueur peut : se déplacer, combattre, recruter des troupes, récolter, crafter, commercer |
| **Multijoueur** | 2+ joueurs sur le même Lobby voient les mêmes entités et peuvent interagir |
| **Stable** | Le serveur tient 1h+ sans crash avec 50 joueurs |
| **Persistant** | Le monde survit à un redémarrage du serveur |
| **Solo** | Le jeu fonctionne hors-ligne (LOI-1, LOI-2) |
| **Progression** | Les compétences montent par usage, le joueur s'équipe, ses troupes grandissent |
| **Fun** | La boucle combat + troupes + loot + craft produit un cycle de jeu satisfaisant |

---

# VII — ÉVOLUTION POST-MVP

## Roadmap après le MVP

| Version | Contenu | Estimation |
|---------|---------|------------|
| **v0.2** | Housing basique, magie (sorts simples), Compagnie (formations), enseignement, guildes, PvP consensuel (duel), timer wheel, persistence incrémentale | +3 mois |
| **v0.3** | Simulation écologique, régions multiples, nations NPC, rareté géographique, caravanes, dressage, régulation inflation | +3 mois |
| **v0.4** | Guerre de nations, gradient de loi, siège de structures, chaînes de production, cluster multi-node, karma/réputation | +3 mois |
| **v1.0** | Version complète du Blueprint Moteur Sandbox MGE | +6 mois |

Chaque version s'appuie sur la précédente. Le MVP est conçu pour que chaque system puisse être remplacé ou étendu sans réécriture du reste (découplage ECS + Event Bus).

---

# VIII — CONFIGURATIONS MVP

## world_mvp.json

```json
{
  "world": {
    "name": "Allumina MVP",
    "seed": 1,
    "tick_rate": 30,
    "map_width": 256,
    "map_height": 256,
    "save_interval_sec": 300
  },
  "gameplay": {
    "base_move_speed": 3.0,
    "agi_speed_bonus": 0.2,
    "combat_range_melee": 1.5,
    "combat_range_ranged": 6.0,
    "projectile_speed": 8.0,
    "interest_radius": 32,
    "max_inventory_slots": 20,
    "respawn_delay_player_sec": 10,
    "troop_follow_distance": 3.0,
    "troop_max_per_player": 5
  },
  "economy": {
    "monster_gold_min": 1,
    "monster_gold_max": 50,
    "npc_vendor_base_marchandage": 30,
    "marchandage_max_discount_pct": 50
  }
}
```

## monsters_mvp.json

```json
{
  "monsters": [
    {
      "type_id": 1,
      "name": "Rat",
      "level": 1,
      "hp": 20,
      "atk": 8, "esq": 5, "par": 2, "atk_speed": 30,
      "damage_min": 1, "damage_max": 3, "damage_type": "tranchant",
      "aggro_radius": 0, "leash_radius": 10,
      "loot_table": "rat_loot",
      "respawn_sec": 60
    },
    {
      "type_id": 2,
      "name": "Loup",
      "level": 3,
      "hp": 50,
      "atk": 18, "esq": 15, "par": 5, "atk_speed": 40,
      "damage_min": 4, "damage_max": 8, "damage_type": "perforant",
      "aggro_radius": 6, "leash_radius": 12,
      "loot_table": "wolf_loot",
      "respawn_sec": 120
    },
    {
      "type_id": 3,
      "name": "Gobelin",
      "level": 5,
      "hp": 80,
      "atk": 25, "esq": 20, "par": 12, "atk_speed": 35,
      "damage_min": 6, "damage_max": 14, "damage_type": "tranchant",
      "aggro_radius": 8, "leash_radius": 16,
      "loot_table": "goblin_loot",
      "respawn_sec": 180
    },
    {
      "type_id": 4,
      "name": "Bandit",
      "level": 7,
      "hp": 120,
      "atk": 35, "esq": 25, "par": 18, "atk_speed": 40,
      "damage_min": 10, "damage_max": 20, "damage_type": "tranchant",
      "aggro_radius": 10, "leash_radius": 20,
      "loot_table": "bandit_loot",
      "respawn_sec": 240
    },
    {
      "type_id": 5,
      "name": "Troll",
      "level": 12,
      "hp": 300,
      "atk": 50, "esq": 15, "par": 30, "atk_speed": 25,
      "damage_min": 20, "damage_max": 40, "damage_type": "contondant",
      "aggro_radius": 12, "leash_radius": 24,
      "loot_table": "troll_loot",
      "respawn_sec": 300
    },
    {
      "type_id": 6,
      "name": "Chef Troll",
      "level": 20,
      "hp": 800,
      "atk": 70, "esq": 20, "par": 40, "atk_speed": 20,
      "damage_min": 35, "damage_max": 70, "damage_type": "contondant",
      "aggro_radius": 15, "leash_radius": 30,
      "loot_table": "troll_boss_loot",
      "respawn_sec": 600
    }
  ]
}
```

## troops_mvp.json

```json
{
  "troop_types": [
    {
      "type_id": 1,
      "name": "Milicien",
      "cmd_prerequisite": 20,
      "cmd_cost": 5,
      "hp": 60,
      "atk": 15, "esq": 10, "par": 12, "atk_speed": 30,
      "damage_min": 3, "damage_max": 7, "damage_type": "tranchant",
      "ar_tranc": 5, "ar_cont": 5, "ar_perc": 5,
      "recruit_gold_cost": 200,
      "follow_distance": 2.5
    },
    {
      "type_id": 2,
      "name": "Garde",
      "cmd_prerequisite": 25,
      "cmd_cost": 10,
      "hp": 120,
      "atk": 28, "esq": 15, "par": 22, "atk_speed": 35,
      "damage_min": 8, "damage_max": 16, "damage_type": "tranchant",
      "ar_tranc": 12, "ar_cont": 10, "ar_perc": 8,
      "recruit_gold_cost": 500,
      "follow_distance": 3.0
    }
  ]
}
```

## recipes_mvp.json

```json
{
  "recipes": [
    {
      "id": 1,
      "name": "Épée en fer",
      "inputs": [{ "type": "iron_ore", "qty": 5 }],
      "output": "iron_sword",
      "skill": "mecanique",
      "skill_min": 0,
      "station": "forge"
    },
    {
      "id": 2,
      "name": "Masse en fer",
      "inputs": [{ "type": "iron_ore", "qty": 4 }],
      "output": "iron_mace",
      "skill": "mecanique",
      "skill_min": 0,
      "station": "forge"
    },
    {
      "id": 3,
      "name": "Lance en fer",
      "inputs": [{ "type": "iron_ore", "qty": 3 }, { "type": "oak_wood", "qty": 2 }],
      "output": "iron_spear",
      "skill": "mecanique",
      "skill_min": 10,
      "station": "forge"
    },
    {
      "id": 4,
      "name": "Arc simple",
      "inputs": [{ "type": "oak_wood", "qty": 3 }],
      "output": "simple_bow",
      "skill": "mecanique",
      "skill_min": 0,
      "station": "atelier"
    },
    {
      "id": 5,
      "name": "Armure de cuir",
      "inputs": [{ "type": "raw_leather", "qty": 5 }],
      "output": "leather_armor",
      "skill": "mecanique",
      "skill_min": 20,
      "station": "atelier"
    },
    {
      "id": 6,
      "name": "Armure de mailles",
      "inputs": [{ "type": "iron_ore", "qty": 8 }],
      "output": "chainmail",
      "skill": "mecanique",
      "skill_min": 40,
      "station": "forge"
    },
    {
      "id": 7,
      "name": "Épée en acier",
      "inputs": [{ "type": "steel_ore", "qty": 5 }],
      "output": "steel_sword",
      "skill": "mecanique",
      "skill_min": 50,
      "station": "forge"
    },
    {
      "id": 8,
      "name": "Potion de soin",
      "inputs": [{ "type": "herbs", "qty": 3 }],
      "output": "health_potion",
      "output_quantity": 3,
      "skill": "mecanique",
      "skill_min": 10,
      "station": "atelier"
    }
  ]
}
```

---

## Références

| Document | Rôle |
|----------|------|
| [Allumina - Blueprint Moteur Sandbox MGE](./Allumina%20-%20Blueprint%20Moteur%20Sandbox%20MGE.md) | Architecture cible complète (vision finale) |
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision, LOI, MWS |
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Genre, monde, solo/multi |
| [Allumina - Prototype Premier Playable](./Allumina%20-%20Prototype%20Premier%20Playable.md) | Précédent prototype (2 joueurs) |
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Formules stats, combat, compétences |
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Troupes, ordres, échelles |
| [Allumina - Compétences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Progression par usage |

---

**Document** : Allumina — MVP Sandbox  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Spécification MVP normative
