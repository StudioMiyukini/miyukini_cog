# Allumina — Blueprint Moteur Sandbox MGE

## Contexte

Ce document transforme les patterns sandbox extraits d'Ultima Online (voir [Extraction Architecture UO](./Allumina%20-%20Extraction%20Architecture%20UO%20pour%20MGE.md)) en **architecture moteur concrète** pour Allumina sur MGE. Il ne s'agit pas de reproduire UO mais d'abstraire ses concepts fondamentaux, d'éliminer les limitations héritées de 1997, et de les moderniser pour le gameplay spécifique d'Allumina : progression esclave→héros, troupes à échelle variable (Dynasty Warriors), combat temps réel (Diablo), économie player-driven (UO), monde persistant souverain (LOI-1 à LOI-3 MWS).

**Allumina n'est PAS une copie d'UO.** C'est un monde systémique moderne inspiré des principes sandbox profonds.

## Portée / Scope

- **Applicable à :** Architecture complète du sandbox Allumina, plugins MGE, game design technique.
- **Audience :** Développement moteur, architecture serveur, game design.
- **Statut :** Blueprint normatif.

## Documents sources

| Document | Rôle |
|----------|------|
| [Extraction Architecture UO pour MGE](./Allumina%20-%20Extraction%20Architecture%20UO%20pour%20MGE.md) | Reverse-engineering UO complet (ServUO, ModernUO, ClassicUO, UOX3) |
| [Extraction Systèmes D2 pour MGE](./Allumina%20-%20Extraction%20Systemes%20D2%20OpenDiablo2%20pour%20MGE.md) | Systèmes Diablo II (combat, items, loot, projectiles) |
| [Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | 10 caracs, aptitudes combat, compétences, formules |
| [Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Échelles, voies, ordres tactiques, Dynasty Warriors |
| [Compétences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Progression par usage, enseignement PNJ/joueur |
| [Vision Gameplay et Ambition](./Concept/Allumina%20-%20Vision%20Gameplay%20et%20Ambition.md) | UO/Diablo/DW, progression esclave→héros, nations NPC |
| [Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Solo + Lobby MWS, persistance LOI-3 |

---

# PARTIE A — FONDATIONS

## Principes systémiques d'Allumina

| Principe | UO (1997) | Allumina (2026) |
|----------|-----------|-----------------|
| **Monde persistant** | Un shard = un monde | Un Lobby MWS = un monde souverain (LOI-3) |
| **Économie player-driven** | Économie fermée, pas de régulation | Régulation automatique, sinks dynamiques |
| **Progression par usage** | 58 skills, cap 720, GGS | 10 caracs + aptitudes + compétences, plafond `carac×10+20` |
| **Housing** | Décoratif, decay par inactivité | Fonctionnel (atelier, fort, ferme), entretien actif |
| **PvP** | Binaire (Felucca/Trammel) | Gradient de loi par région (nation) |
| **Craft** | Recette directe → item | Chaîne de production multi-étape |
| **Combat** | Tour-par-tour masqué | Temps réel Diablo-like + troupes DW |
| **Troupes** | Inexistant | Groupe→Compagnie→Régiment (Cha/Cmd) |
| **Écologie** | Respawn fixe | Simulation dynamique (fertilité, climat, population) |

## Invariants architecturaux MGE

```
1. ECS pur — pas d'héritage OOP, pas de God Objects
2. Serveur 100% autoritaire (Lobby hôte, LOI-1)
3. Simulation déterministe tick-based (30 TPS logique)
4. Persistence incrémentale (KindMother, pas de pause réseau)
5. Event Bus central — découplage total inter-systèmes
6. Scheduler Timer Wheel — O(1) insert/remove
7. Séparation Simulation / Network / Rendering
8. Client non autoritaire (prédiction mouvement uniquement)
9. Hot reload data-driven (config JSON/RON)
10. Multi-thread safe (Rust ownership)
```

---

# PARTIE B — LES 8 ENGINES

---

# I — ALLUMINA TERRITORIAL ENGINE

## 1. Ce qu'UO faisait

- Housing monde ouvert : `BaseHouse` (6 types, multi-tile via `BaseMulti`)
- Decay par inactivité (6 stades, IDOC → collapse → barils pillables)
- Access control : Owner / Co-Owner / Friend / Access / Banned / Public
- Lockdown + Secure containers (capacité limitée par type de maison)
- Placement validation : zone, espace, terrain plat, distance autres maisons

**Pattern identifié :** Objet-monde persistant avec cycle de vie, ACL, et contraintes spatiales.

**Limites héritées :**
- Héritage OOP profond (`Item` → `BaseMulti` → `BaseHouse` → `SmallHouse` → `SmallOldHouse`)
- Pas de fonction économique (décoratif uniquement)
- Terrain consommé sans limite → pénurie de terrain
- Pas de système d'impôts
- Decay passif sans entretien actif

## 2. Version modernisée pour Allumina

Allumina transforme le housing en **Territorial Engine** :

| Fonction | Mécanisme |
|----------|-----------|
| **Habitation** | Repos (regen End), stockage personnel, point de respawn |
| **Atelier** | Stations de craft (forge, établi, scierie), bonus craft |
| **Commerce** | Comptoir de vente (vendor joueur), vitrine |
| **Agriculture** | Champs cultivables, enclos animaux, récolte périodique |
| **Fortification** | Tours de défense, murs, pièges, gardes PNJ recrutés |
| **Entrepôt** | Stockage de masse (caravanes, guildes) |

### Gradient de terrain

```
┌───────────────────────────────────────────────┐
│         GRADIENT DE TERRAIN ALLUMINA           │
│                                                │
│  Ville (nation)                                │
│  ├── Parcelles prédéfinies → achat à la nation │
│  ├── Impôt fixe + protection gardes            │
│  └── Taille limitée, fonctions limitées        │
│                                                │
│  Province                                      │
│  ├── Terrain libre → placement validation      │
│  ├── Impôt modéré, patrouilles rares           │
│  └── Taille moyenne, toutes fonctions          │
│                                                │
│  Frontière / Terre sauvage                     │
│  ├── Terrain libre, pas d'impôt                │
│  ├── Pas de protection → attaquable            │
│  └── Grande taille, fortification possible     │
└───────────────────────────────────────────────┘
```

## 3. Components MGE

```rust
pub struct Territory {
    pub serial: u64,
    pub owner: EntityId,
    pub structure_type: StructureType,
    pub nation_zone: Option<NationZoneId>,
    pub tax_rate: f32,
    pub last_maintenance: u64,          // game_time du dernier paiement
    pub maintenance_cost: MaintenanceCost,
    pub decay_stage: DecayStage,
    pub integrity: f32,                 // 0.0–100.0 (siège)
    pub tiles: Vec<TerritoryTile>,
    pub functional_slots: Vec<FunctionalSlot>,
}

pub struct TerritoryTile {
    pub offset: IVec3,
    pub tile_id: u16,
    pub flags: TileFlags,
    pub walkable: bool,
}

pub struct FunctionalSlot {
    pub slot_type: FunctionalType,
    pub position: IVec3,
    pub active: bool,
    pub upgrade_level: u8,
}

pub enum FunctionalType {
    Forge, Workbench, Sawmill, Alchemy, Enchanting,
    StorageVault, Bed, TradeCounter, CropField,
    StableSlot, DefenseTower, TrapSlot, Well,
}

pub struct MaintenanceCost {
    pub gold_per_cycle: u64,
    pub materials: Vec<(ItemTypeId, u32)>,
    pub cycle_duration: u64,            // en game_ticks
}

pub enum StructureType {
    House, Workshop, Shop, Farm, Fort, Warehouse, Inn,
}

pub enum DecayStage {
    Pristine,     // 100% — entretien à jour
    Worn,         // 80%  — cosmétique
    Damaged,      // 60%  — fonctions dégradées (-20% craft)
    Crumbling,    // 40%  — fonctions limitées
    Condemned,    // 20%  — accès forcé public
    Collapsed,    // 0%   — terrain libéré, contenu en barils
}

pub struct TerritoryAccess {
    pub owner: EntityId,
    pub co_owners: Vec<EntityId>,
    pub employees: Vec<EntityId>,
    pub guests: Vec<EntityId>,
    pub bans: Vec<EntityId>,
    pub door_locks: HashMap<IVec3, LockLevel>,
    pub is_public: bool,
}
```

## 4. Systems MGE

```rust
// @phase 350
pub fn territory_maintenance_system(world: &mut World, ctx: &mut Context) {
    // Pour chaque Territory:
    // - Vérifier si le cycle de maintenance est échu
    // - Vérifier si owner possède les ressources (or + matériaux)
    // - Si oui: prélever, reset decay timer, emit MaintenancePaidEvent
    // - Si non: avancer decay_stage d'un cran
    // - Appliquer les malus selon decay_stage (craft -20%, etc.)
}

// @phase 351
pub fn territory_decay_system(world: &mut World, ctx: &mut Context) {
    // Pour chaque Territory en Condemned depuis > collapse_delay:
    // - Émettre TerritoryCollapseEvent
    // - Convertir contenu en barils pillables (ItemSpawnEvent)
    // - Supprimer les tiles du monde
    // - Libérer le terrain
}

// @phase 352
pub fn territory_siege_system(world: &mut World, ctx: &mut Context) {
    // En zones PvP/Frontier:
    // - Si des entités hostiles attaquent la structure
    // - Réduire integrity selon les dégâts
    // - Si integrity <= 0 → structure détruite (pas decay, destruction)
    // - Emit TerritorySiegeEvent, TerritoryDestroyedEvent
}
```

## 5. Event flow

```
[Joueur place deed] → TerritoryPlacementRequest
  → validate_placement() : zone, espace, terrain, limite par statut
  → TerritoryPlacedEvent → spawn tiles dans le monde
  
[Cycle maintenance] → territory_maintenance_system
  → MaintenancePaidEvent | DecayAdvancedEvent
  
[Siège PvP] → territory_siege_system
  → TerritorySiegeEvent → TerritoryDestroyedEvent
  
[Collapse] → territory_decay_system
  → TerritoryCollapseEvent → ItemSpawnEvent (barils)
```

## 6. Anti-exploit

| Exploit | Solution |
|---------|----------|
| Spam de structures | Impôt progressif : 2e structure ×1.5, 3e ×2.5, etc. |
| Bloquer positions stratégiques | Limite par statut social + validation nation |
| Structure PvP indestructible | Integrity + siège + decay si pas maintenue |
| Alt accounts pour bypass | Limite par COG (pas par personnage) |
| Stockage infini | Capacité par type + entretien Stockage Vault |

## 7. Data schema JSON

```json
{
  "structure_types": {
    "workshop": {
      "base_tiles": 64,
      "functional_slots": [
        { "type": "forge", "position": [3, 0, 2] },
        { "type": "workbench", "position": [5, 0, 2] }
      ],
      "maintenance": { "gold": 500, "materials": [{ "type": "wood", "qty": 10 }], "cycle_hours": 168 },
      "max_storage": 200,
      "required_status": "citizen"
    }
  }
}
```

---

# II — ALLUMINA USAGE-BASED PROGRESSION ENGINE

## 1. Ce qu'UO faisait

- 58 skills, cap total 720 points, cap individuel 100 (extensible à 120 via Power Scrolls)
- Gain probabiliste par usage : formule `gc = ((cap-total)/cap + (cap_skill-base)/cap_skill) / 4`
- Facteurs : room-to-cap, difficulty, success, GainFactor, level penalty (÷2 à 70, ÷2 à 80, ÷2 à 90)
- GGS (Guaranteed Gain System) : gain garanti après cooldown (0s à 50, 12h à 110+)
- Anti-macro : détection location + target répétitifs
- Skill locks : Up / Down / Locked

**Pattern identifié :** Progression continue probabiliste avec fenêtre de difficulté et caps imbriqués.

**Limites héritées :**
- Macroing facile (AFK skill gain) malgré anti-macro
- Cap total force des sacrifices arbitraires
- Pas d'enseignement structuré
- GainFactor fixe par skill (pas d'influence du gameplay)

## 2. Version Allumina

Allumina utilise le système [Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) :

| Couche | Échelle | Plafond |
|--------|---------|---------|
| **Caractéristiques** (For, Con, Agi, Dex, Per, Vol, Int, Sag, Cha, Luk) | 1–10 | Fixe à la création + progression lente |
| **Aptitudes de combat** (atk, par, esq, tirC, tirP, tirE, cast speed...) | 1–100 | Dérivé des caracs |
| **Compétences** (Saut, Fouille, Marchandage, Cmd, Dressage...) | min=`carac×5` — max=`carac×10+20` | Plafond par caractéristique |

### Deux voies de progression

1. **Usage** (inspiré UO) : pratiquer la compétence fait monter le skill
2. **Enseignement** (spécifique Allumina) : cycles de test maître→apprenti (1 pt par cycle réussi)

### Formule de gain Allumina

```
fn calculate_gain_chance(character, skill_id, difficulty, success) -> f64:
    let skill = character.skills[skill_id]
    let carac = character.get_primary_carac(skill_id)
    let individual_cap = carac.base * 10.0 + 20.0
    
    // Facteur espace disponible (inspiré UO room-to-cap)
    let room = (individual_cap - skill.base) / individual_cap
    
    // Facteur difficulté (action plus difficile = plus de gain)
    let diff_factor = (1.0 - difficulty) * if success { 0.5 } else { 0.1 }
    
    // Gain brut
    let mut gc = (room + diff_factor) / 4.0
    
    // Pénalités par palier (comme UO mais adaptées au cap variable)
    let ratio = skill.base / individual_cap
    if ratio > 0.70 { gc /= 2.0 }
    if ratio > 0.85 { gc /= 2.0 }
    if ratio > 0.95 { gc /= 3.0 }
    
    // Facteur par type de compétence
    gc *= skill.gain_factor
    
    // Anti-macro : si même location + même action < 5min → gc /= 10
    if is_macro_detected(character, skill_id) { gc /= 10.0 }
    
    // Variation d'activité bonus : actions différentes dans l'heure → gc × 1.2
    if activity_variety_bonus(character) { gc *= 1.2 }
    
    gc.max(0.005) // minimum 0.5%
```

### Enseignement joueur→joueur

```
fn teaching_cycle(teacher, student, skill_id) -> TeachResult:
    let t_skill = teacher.skills[skill_id].base
    let s_skill = student.skills[skill_id].base
    
    // Le maître doit avoir ≥ 20 pts de plus
    if t_skill < s_skill + 20.0:
        return TeachResult::InsufficientMastery
    
    // Opposition : Enseignement du maître vs difficulté (ratio élève/maître)
    let teach_val = teacher.skills[ENSEIGNEMENT].base  // ou Sag-based
    let difficulty = (s_skill / t_skill) * 100.0
    
    // Jet opposé (section 5.1 du doc Caracs)
    let chance = 50.0 + (teach_val - difficulty)
    let roll = random_d100()
    
    // Luk du maître réduit le jet
    let effective_roll = roll - teacher.characteristics.luk as f64
    
    if effective_roll <= chance:
        student.skills[skill_id].base += 1.0
        emit SkillGainEvent { entity: student.id, skill: skill_id, source: Teaching }
        return TeachResult::Success
    else:
        return TeachResult::Failed
```

## 3. Components MGE

```rust
pub struct CharacterStats {
    pub characteristics: [f64; 10],
    // index: 0=For, 1=Con, 2=Agi, 3=Dex, 4=Per, 5=Vol, 6=Int, 7=Sag, 8=Cha, 9=Luk
}

pub struct DerivedStats {
    pub hp_max: f64,        // (For + Con) × 10
    pub mp_max: f64,        // (Int + Sag) × 10
    pub end_max: f64,       // (For + Con×2) × 10
    pub aggro: f64,         // Con + For
    pub weight_max: f64,    // (For + Con) × 5
}

pub struct CombatAptitudes {
    pub atk: f64,           // Dex × 10
    pub atk_speed: f64,     // Agi × 10
    pub par: f64,           // (For + Con)/2 × 10
    pub esq: f64,           // Agi × 10
    pub tir_c: f64,         // (For/2 + Per) × 10
    pub tir_p: f64,         // (Agi + Per) × 10
    pub tir_e: f64,         // (For + Per) × 10
    pub cast_speed: f64,    // (Int + Sag) / 2
}

pub struct SkillSet {
    pub skills: Vec<SkillValue>,
}

pub struct SkillValue {
    pub id: SkillId,
    pub base: f64,
    pub gain_factor: f64,
    pub lock: SkillLock,
    pub primary_carac: u8,       // index dans characteristics
    pub last_gain_time: u64,
    pub last_use_location: Option<IVec3>,
    pub last_use_action: Option<u32>,
    pub variety_actions: Vec<(u32, u64)>,  // (action_hash, timestamp) pour bonus variété
}

pub enum SkillLock { Up, Down, Locked }

pub struct TeachingSession {
    pub teacher: EntityId,
    pub student: EntityId,
    pub skill: SkillId,
    pub cycles_completed: u32,
    pub cycles_total: u32,
    pub cooldown_remaining: f32,  // temps avant prochain cycle
}

pub struct TroopCommand {
    pub cmd_value: f64,           // compétence Commandement
    pub troops: Vec<TroopSlot>,
    pub cmd_used: f64,            // somme des coûts
}

pub struct TroopSlot {
    pub entity: EntityId,
    pub troop_type: TroopTypeId,
    pub cmd_cost: f64,
}
```

## 4. Systems MGE

```rust
// @phase 500
pub fn skill_check_system(world: &mut World, ctx: &mut Context);
// Traite les SkillCheckRequest → calcule succès → émet SkillCheckResult
// Applique la formule d'opposition (50% + 1%/pt écart) du doc Caracs section 5.1
// Applique Luk (résultat effectif = dé − Luk)
// Vérifie critique (jet ≤ Luk + mod → critik / réussite auto)

// @phase 501
pub fn skill_gain_system(world: &mut World, ctx: &mut Context);
// Pour chaque SkillCheckResult avec lock == Up :
// 1. calculate_gain_chance()
// 2. Roll → si gain : skill.base += 0.1
// 3. Anti-macro check
// 4. Émet SkillGainEvent

// @phase 502
pub fn teaching_system(world: &mut World, ctx: &mut Context);
// Pour chaque TeachingSession active :
// 1. Vérifier cooldown
// 2. teaching_cycle()
// 3. Émettre TeachCycleEvent
```

## 5. Config JSON

```json
{
  "skills": {
    "saut": {
      "id": 1,
      "primary_carac": "for",
      "gain_factor": 1.0,
      "description": "Distance de saut, franchissement d'obstacles"
    },
    "marchandage": {
      "id": 20,
      "primary_carac": "cha",
      "gain_factor": 0.8,
      "description": "Influence prix vente/achat, 1% par pt écart"
    },
    "commandement": {
      "id": 25,
      "primary_carac": "cha",
      "gain_factor": 0.5,
      "description": "Pool pts troupes, prérequis types"
    }
  },
  "teaching": {
    "min_skill_gap": 20.0,
    "cycle_cooldown_sec": 300,
    "max_cycles_per_day": 10
  },
  "anti_macro": {
    "same_location_penalty_delay_sec": 300,
    "same_action_penalty_delay_sec": 180,
    "variety_bonus_window_sec": 3600,
    "variety_bonus_min_actions": 3
  }
}
```

---

# III — ALLUMINA ECOLOGICAL RESOURCE ENGINE

## 1. Ce qu'UO faisait

- `HarvestSystem` abstrait avec `HarvestDefinition` + `HarvestResource` + `HarvestVein`
- Banques de ressources invisibles (position spatiale, mais montant caché)
- Respawn fixe par timer (banque se remplit après X minutes)
- Distribution probabiliste des veines (Iron 50%, Bronze 20%, Valorite 1%)
- Validation : tile type + outil + skill check
- Bonus Felucca (+100% drop)

**Pattern identifié :** Banque de ressources spatiale avec distribution probabiliste et respawn par timer.

**Limites héritées :**
- Respawn fixe → pas de surexploitation réelle
- Pas de simulation écologique
- Distribution identique partout (pas de géographie)
- Pas de raréfaction dynamique
- Pas d'impact joueur sur l'écosystème

## 2. Version Allumina : simulation écologique dynamique

```
┌──────────────────────────────────────────────────────┐
│              SIMULATION ÉCOLOGIQUE ALLUMINA            │
│                                                        │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐        │
│  │ Climat   │───→│ Fertilité│───→│ Végétation│        │
│  │ (saison, │    │ du sol   │    │ (densité) │        │
│  │  météo)  │    │          │    │           │        │
│  └──────────┘    └──────────┘    └─────┬─────┘        │
│                                        │               │
│                          ┌─────────────┼────────┐     │
│                          ↓             ↓        ↓     │
│                    ┌──────────┐  ┌──────────┐  ┌────┐ │
│                    │Herbivores│  │   Bois   │  │Herb│ │
│                    │(cerfs,   │  │(arbres,  │  │(pl │ │
│                    │ lapins)  │  │ buissons)│  │antes│ │
│                    └────┬─────┘  └──────────┘  └────┘ │
│                         │                              │
│                    ┌────┴─────┐                        │
│                    │Carnivores│                        │
│                    │(loups,   │                        │
│                    │ ours)    │                        │
│                    └────┬─────┘                        │
│                         │                              │
│  ┌──────────────────────┴─────────────────────────┐   │
│  │              JOUEURS                            │   │
│  │  ┌────────┐  ┌────────┐  ┌────────┐           │   │
│  │  │Récolte │  │ Chasse │  │ Minage │           │   │
│  │  └───┬────┘  └───┬────┘  └───┬────┘           │   │
│  │      ↓           ↓           ↓                 │   │
│  │  [Déplétion] [Pop baisse] [Nœud épuisé]       │   │
│  │      ↓           ↓           ↓                 │   │
│  │  [Stress écologique croissant]                 │   │
│  │      ↓                                         │   │
│  │  [Régénération dynamique = f(fertilité,climat)]│   │
│  │      ↓                                         │   │
│  │  [Si stress > seuil → DÉSERTIFICATION]         │   │
│  │  [Migration animaux vers zones saines]         │   │
│  └────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────┘
```

### Modèle Lotka-Volterra simplifié

```
dV/dt = r_v × V × (1 - V/K_v) - α × H × V - harvest_vegetation
dH/dt = r_h × H × (1 - H / (β × V)) - γ × C × H - harvest_herbivores
dC/dt = r_c × C × (1 - C / (δ × H)) - μ × C - harvest_carnivores

V = végétation, H = herbivores, C = carnivores
r = taux de croissance, K = capacité, α/β/γ/δ = coefficients d'interaction
harvest_* = prélèvement joueur
```

## 3. Components MGE

```rust
pub struct EcologicalZone {
    pub zone_id: ZoneId,
    pub bounds: IRect,
    pub climate: Climate,
    pub fertility_base: f32,         // sol de base (géographie)
    pub fertility_current: f32,      // 0.0–1.0 (dynamique)
    pub vegetation_density: f32,     // 0.0–1.0
    pub depletion_stress: f32,       // 0.0–1.0 (surexploitation cumulée)
    pub desertification: bool,       // stress > 0.8 pendant > 30 jours
}

pub struct Climate {
    pub temperature: f32,
    pub rainfall: f32,
    pub season: Season,
    pub weather: Weather,
}

pub enum Season { Spring, Summer, Autumn, Winter }
pub enum Weather { Clear, Rain, Storm, Drought, Snow }

pub struct AnimalPopulation {
    pub zone: ZoneId,
    pub species: SpeciesId,
    pub count: u32,
    pub max_capacity: u32,
    pub growth_rate: f32,
    pub death_rate: f32,
    pub diet: Diet,
    pub migration_threshold: f32,  // si food < threshold → migre
}

pub enum Diet { Herbivore, Carnivore, Omnivore }

pub struct ResourceNode {
    pub node_id: u64,
    pub resource_type: ResourceTypeId,
    pub position: IVec3,
    pub zone: ZoneId,
    pub current_amount: u32,
    pub max_capacity: u32,
    pub base_regen_rate: f32,        // unités/heure (base)
    pub effective_regen_rate: f32,    // unités/heure (dynamique)
    pub depletion_stress: f32,       // 0.0–1.0 (nœud spécifique)
    pub required_skill: SkillId,
    pub min_skill: f64,
    pub required_tool: ItemTypeId,
}

pub struct HarvestAttempt {
    pub player: EntityId,
    pub node: EntityId,
    pub tool: EntityId,
}
```

## 4. Systems MGE

```rust
// @phase 100 (basse fréquence : toutes les 10 min game-time)
pub fn ecological_simulation_system(world: &mut World, ctx: &mut Context) {
    for zone in world.query::<&mut EcologicalZone>() {
        // 1. Mettre à jour climat (saison, météo aléatoire)
        update_climate(&mut zone.climate, ctx.game_time);
        
        // 2. Calculer fertilité dynamique
        let climate_mod = match zone.climate.weather {
            Rain => 1.3, Clear => 1.0, Drought => 0.4, Storm => 0.8, Snow => 0.2,
        };
        let season_mod = match zone.climate.season {
            Spring => 1.4, Summer => 1.0, Autumn => 0.7, Winter => 0.3,
        };
        zone.fertility_current = zone.fertility_base
            * climate_mod * season_mod
            * (1.0 - zone.depletion_stress);
        
        // 3. Végétation (Lotka-Volterra simplifié)
        let dv = zone.fertility_current * 0.01 * zone.vegetation_density
            * (1.0 - zone.vegetation_density);
        zone.vegetation_density = (zone.vegetation_density + dv).clamp(0.0, 1.0);
        
        // 4. Décroissance stress naturelle
        zone.depletion_stress *= 0.995;
        
        // 5. Désertification
        zone.desertification = zone.depletion_stress > 0.8;
    }
    
    // Populations animales
    for pop in world.query::<&mut AnimalPopulation>() {
        let zone = world.get::<EcologicalZone>(pop.zone);
        let food = match pop.diet {
            Herbivore => zone.vegetation_density,
            Carnivore => herbivore_density(world, pop.zone),
            Omnivore => (zone.vegetation_density + herbivore_density(world, pop.zone)) / 2.0,
        };
        
        let growth = pop.count as f32 * pop.growth_rate * food
            - pop.count as f32 * pop.death_rate;
        pop.count = (pop.count as f32 + growth).max(0.0).min(pop.max_capacity as f32) as u32;
        
        // Migration si nourriture insuffisante
        if food < pop.migration_threshold {
            emit MigrationEvent { species: pop.species, from: pop.zone };
        }
    }
}

// @phase 300
pub fn harvest_system(world: &mut World, ctx: &mut Context) {
    for attempt in world.events::<HarvestAttempt>() {
        let node = world.get_mut::<ResourceNode>(attempt.node);
        let player = world.get::<CharacterStats>(attempt.player);
        
        // 1. Validation (outil, skill minimum)
        // 2. Skill check (fenêtre de difficulté)
        // 3. Si succès : prélever, émettre HarvestSuccessEvent
        // 4. Augmenter depletion_stress du nœud et de la zone
        // 5. Mettre à jour effective_regen_rate
        
        node.depletion_stress += 0.01;
        let zone = world.get_mut::<EcologicalZone>(node.zone);
        zone.depletion_stress += 0.001;
    }
}
```

## 5. Rareté géographique

| Ressource | Zones privilégiées | Rareté |
|-----------|-------------------|--------|
| **Fer** | Montagnes du nord, cavernes | Commune |
| **Bois dur** | Forêts profondes du sud | Commune |
| **Herbes médicinales** | Prairies, marais | Moyenne |
| **Mithril** | Mines profondes, volcans | Rare |
| **Cristal arcane** | Ruines magiques | Très rare |
| **Valorite** | Profondeurs volcaniques | Légendaire |

---

# IV — ALLUMINA PRODUCTION CHAIN ENGINE

## 1. Ce qu'UO faisait

- `CraftSystem` abstrait avec héritage (`Blacksmithy`, `Carpentry`, `Tailoring`...)
- Recette directe : X lingots → épée
- Qualité binaire : normal ou exceptional (skill check bonus)
- Signature crafter (nom)
- Material type modifie les propriétés (Agapite, Valorite...)
- UI → server pipeline : joueur sélectionne recette, serveur valide

**Limites héritées :**
- Pas de chaîne de production
- Qualité binaire (pas de granularité)
- Pas d'outils avec qualité
- Pas de spécialisation encouragée

## 2. Version Allumina : chaînes de production multi-étape

```
CHAÎNE DE PRODUCTION — ÉPÉE EN ACIER ENCHANTÉ

[Minerai de fer]──→(Fonderie)──→[Lingot de fer]
                                       │
[Charbon]──────────→(Forge)────→[Acier]←┘
                                   │
                          (Forge)──┤──→[Lame brute d'acier]
                                   │
[Bois dur]──→(Scierie)──→[Planche]─┤
                                   │
                   (Menuiserie)────┤──→[Pommeau en bois dur]
                                   │
[Cuir]─────────────────────────────┤
                                   │
                    (Assemblage)────┤──→[Épée en acier]
                                   │           │
[Gemme magique]                    │    (Enchantement)──→[Épée en acier enchantée]
[Réactif arcane]───────────────────┘           │
                                         Qualité = f(skills,
                                           outils, matériaux)
```

### Formule de qualité

```
fn calculate_craft_quality(crafter, recipe, materials, tool) -> f64:
    // Facteur skill (jet Mécanique vs difficulté recette)
    let skill = crafter.skills[recipe.primary_skill].base
    let cap = recipe.max_skill
    let skill_factor = ((skill - recipe.min_skill) / (cap - recipe.min_skill)).clamp(0.0, 1.0)
    
    // Facteur outil (qualité de l'outil utilisé)
    let tool_factor = tool.quality / 100.0
    
    // Facteur matériaux (moyenne pondérée des qualités)
    let mat_factor = materials.iter()
        .map(|m| m.quality * m.weight)
        .sum::<f64>() / materials.iter().map(|m| m.weight).sum::<f64>()
        / 100.0
    
    // Pondération
    let base_quality = skill_factor * 50.0 + tool_factor * 20.0 + mat_factor * 30.0
    
    // Variation (±8%)
    let variation = random_range(-8.0, 8.0)
    
    let mut final_quality = (base_quality + variation).clamp(0.0, 100.0)
    
    // Critique craft (Luk du crafter)
    let roll = random_d100()
    if roll <= crafter.characteristics.luk as f64 {
        final_quality = (final_quality * 1.20).min(100.0)  // +20%
    }
    
    final_quality
```

## 3. Components MGE

```rust
pub struct CraftRecipe {
    pub id: RecipeId,
    pub name: String,
    pub category: CraftCategory,
    pub inputs: Vec<CraftInput>,
    pub output: ItemTypeId,
    pub output_quantity: u32,
    pub station_required: Option<FunctionalType>,
    pub skill_checks: Vec<(SkillId, f64, f64)>,   // (skill, min, max)
    pub base_time_sec: f32,
    pub failure_consequence: FailureConsequence,
}

pub struct CraftInput {
    pub item_type: ItemTypeId,
    pub quantity: u32,
    pub min_quality: Option<f64>,
    pub consumed_on_failure: bool,
}

pub enum FailureConsequence {
    LoseMaterials,            // matériaux perdus
    LosePartialMaterials(f32), // % perdus
    ProduceLowerQuality,      // produit dégradé
    Nothing,                  // tentative gratuite
}

pub struct CraftedItem {
    pub quality: f64,           // 0.0–100.0
    pub crafter: EntityId,
    pub crafter_name: String,
    pub material_types: Vec<MaterialId>,
    pub crafted_at: u64,
}

pub struct CrafterReputation {
    pub specialties: HashMap<CraftCategory, f64>,
    pub items_crafted: u64,
    pub exceptional_count: u64,  // qualité > 80
    pub masterwork_count: u64,   // qualité > 95
}

pub enum CraftCategory {
    Blacksmithing, Carpentry, Tailoring, Alchemy,
    Cooking, Enchanting, Engineering, Leatherworking,
}
```

## 4. Impact économique

La chaîne de production crée des **interdépendances** entre crafters :

```
Mineur ──→ Fondeur ──→ Forgeron ──→ Enchanteur
                                         ↑
Herboriste ──→ Alchimiste ──→ Réactifs ──┘

Bûcheron ──→ Menuisier ──→ Manches, arcs
                                ↑
Chasseur ──→ Tanneur ──→ Cuir──┘
```

Chaque maillon peut être un joueur différent → commerce obligatoire → économie vivante.

---

# V — ALLUMINA REGIONAL GOVERNANCE ENGINE

## 1. Ce qu'UO faisait

- 6 maps (Felucca, Trammel, Ilshenar, Malas, Tokuno, TerMur)
- Règles PvP binaires par map (`MapRules`)
- Spawn séparé par map
- Pas de contrôle territorial joueur

**Limites héritées :**
- Fracture de la communauté (Felucca vs Trammel)
- Pas de gradient de loi
- Maps isolées (pas de transition seamless)
- Pas de gouvernance dynamique

## 2. Version Allumina : monde unique avec régions dynamiques

```
┌─────────────────────────────────────────────────────────┐
│              MONDE UNIQUE ALLUMINA                        │
│                                                           │
│  ┌────────────────────┐                                  │
│  │  NATION AETHORIA   │                                  │
│  │  (civilisée)       │                                  │
│  │  ┌──────────┐      │                                  │
│  │  │ Capitale │ ← No-PvP, gardes puissants, impôts     │
│  │  │ (Metro)  │                                         │
│  │  └──────────┘      │                                  │
│  │  ┌──────────┐      │                                  │
│  │  │ Province │ ← PvP limité (duels, guildes)          │
│  │  │          │   patrouilles, impôt modéré             │
│  │  └──────────┘      │                                  │
│  │  ┌──────────┐      │                                  │
│  │  │ Frontière│ ← PvP libre, gardes absents             │
│  │  │          │   impôt faible, ressources riches        │
│  │  └──────────┘      │                                  │
│  └────────────────────┘                                  │
│                                                           │
│  ┌────────────────────┐                                  │
│  │ TERRE SAUVAGE      │ ← Aucune loi, aucun impôt       │
│  │ (wilderness)       │   PvP total, ressources rares    │
│  │                    │   donjons, ruines, créatures      │
│  └────────────────────┘                                  │
│                                                           │
│  ┌────────────────────┐                                  │
│  │  NATION KELVARIS   │                                  │
│  │  (en guerre)       │   ← Guerre avec Aethoria         │
│  │  Front de guerre   │   PvP total entre nationaux       │
│  └────────────────────┘                                  │
│                                                           │
│  ┌────────────────────┐                                  │
│  │ ZONE CONTESTÉE     │ ← Contrôle change selon guerres  │
│  │ (dynamique)        │   Impôt du contrôleur actuel     │
│  └────────────────────┘                                  │
└─────────────────────────────────────────────────────────┘
```

## 3. Components MGE

```rust
pub struct Region {
    pub id: RegionId,
    pub name: String,
    pub bounds: Vec<IRect>,          // polygone de la région
    pub nation: Option<NationId>,
    pub law_level: LawLevel,
    pub pvp_rules: PvpRules,
    pub tax_rate: f32,               // prélevé par la nation
    pub guard_strength: f32,         // 0.0–1.0 (densité patrouilles)
    pub resource_bonus: f32,         // multiplicateur récolte
    pub housing_rules: HousingRules,
    pub contest_status: ContestStatus,
}

pub enum LawLevel {
    Metropolitan,   // loi stricte, gardes puissants, réponse immédiate
    Provincial,     // loi partielle, patrouilles
    Frontier,       // loi minimale, patrouilles rares
    Lawless,        // aucune loi
    WarZone,        // zone de guerre active
}

pub enum PvpRules {
    Forbidden,      // sanctuaire
    WarOnly,        // entre nations en guerre uniquement
    Consensual,     // duels + guildes uniquement
    Free,           // PvP total
}

pub struct HousingRules {
    pub allowed: bool,
    pub predefined_parcels: bool,
    pub max_size: u32,
    pub functional_restrictions: Vec<FunctionalType>,
}

pub enum ContestStatus {
    Stable,
    Contested { attackers: Vec<NationId>, progress: f32 },
    UnderSiege { attacker: NationId, siege_duration: f32 },
    Occupied { new_controller: NationId, since: u64 },
}

pub struct Nation {
    pub id: NationId,
    pub name: String,
    pub relations: HashMap<NationId, NationRelation>,
    pub treasury: u64,
    pub military_strength: f32,
}

pub enum NationRelation {
    Allied, Neutral, Hostile, AtWar,
}
```

## 4. Machine d'état — Guerre de nations

```
[Paix]
  │ tensions croissantes (événements, quêtes, accumulation)
  ↓
[Tensions]
  │ seuil atteint → déclaration de guerre (nation NPC)
  ↓
[Guerre déclarée]
  │ zones frontières deviennent WarZone
  │ PvP libre entre nationaux
  │ quêtes de guerre, objectifs militaires
  ↓
[Siège]
  │ forces attaquent une ville/fort
  │ joueurs peuvent participer (DW-style)
  ↓
[Résolution]
  │ victoire → zone contestée change de contrôleur
  │ défaite → statu quo
  │ armistice → retour graduel à la paix
  ↓
[Paix] (avec nouveau contrôle territorial)
```

---

# VI — ALLUMINA SIMULATION SCHEDULER (Timer Wheel)

## 1. Pourquoi le Timer Wheel remplace un scheduler classique

| Scheduler classique | Timer Wheel |
|--------------------|----|
| Priority queue (heap) | Ring buffer |
| Insert : O(log n) | Insert : O(1) |
| Remove : O(log n) | Remove : O(1) |
| Execute : O(1) | Execute : O(1) amortized |
| Mémoire : heap allocations | Mémoire : pré-alloué |
| Threading : locks nécessaires | Threading : single-threaded possible |

Pour un MMO avec 100k+ timers actifs (decay, respawn, regen, buffs, debuffs, AI ticks, craft...), la différence O(log n) vs O(1) est critique.

## 2. Architecture (inspirée ModernUO)

```
┌─────────────────────────────────────────────────────┐
│              TIMER WHEEL HIÉRARCHIQUE                 │
│                                                       │
│  Ring 0 (core)  : 4096 slots × 8ms  = ~32.8s        │
│  Ring 1 (mid)   : 4096 slots × 32.8s = ~37.3h       │
│  Ring 2 (outer) : 4096 slots × 37.3h = ~6.3 jours   │
│                                                       │
│  Résolution : 8ms (125 ticks/sec)                    │
│  Portée max : ~6.3 jours                             │
│  Au-delà : overflow queue (trié, O(log n))           │
│                                                       │
│  ┌─Ring 0────────────────────────────────────┐       │
│  │ [0][1][2]...[slot]...[4095]              │       │
│  │       ↑ current                           │       │
│  └───────────────────────────────────────────┘       │
│  ┌─Ring 1────────────────────────────────────┐       │
│  │ [0][1][2]...[mid_slot]...[4095]          │       │
│  │ Promotion vers Ring 0 quand Ring 0 wrappe│       │
│  └───────────────────────────────────────────┘       │
│  ┌─Ring 2────────────────────────────────────┐       │
│  │ [0][1]...[outer_slot]...[4095]           │       │
│  │ Promotion vers Ring 1 quand Ring 1 wrappe│       │
│  └───────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────┘
```

## 3. Pseudo-code Rust MGE

```rust
const RING_SIZE: usize = 4096;
const TICK_MS: u64 = 8;

pub struct TimerWheel {
    ring0: Vec<Vec<TimerEntry>>,       // 4096 slots
    ring1: Vec<Vec<TimerEntry>>,       // 4096 slots
    ring2: Vec<Vec<TimerEntry>>,       // 4096 slots
    overflow: BinaryHeap<TimerEntry>,  // > 6.3 jours
    current_tick: u64,
    next_id: u64,
}

pub struct TimerEntry {
    pub id: TimerId,
    pub deadline_tick: u64,
    pub event: ScheduledEvent,
    pub repeat_interval: Option<u64>,  // ticks entre répétitions
    pub priority: u8,                  // 0 = haute, 255 = basse
}

pub enum ScheduledEvent {
    DecayCheck(EntityId),
    ResourceRegen(EntityId),
    BuffExpire(EntityId, BuffId),
    AITick(EntityId),
    CraftComplete(EntityId, RecipeId),
    EcologicalTick(ZoneId),
    MaintenanceDue(EntityId),
    Custom(Box<dyn FnOnce(&mut World)>),
}

impl TimerWheel {
    pub fn new() -> Self {
        Self {
            ring0: vec![Vec::new(); RING_SIZE],
            ring1: vec![Vec::new(); RING_SIZE],
            ring2: vec![Vec::new(); RING_SIZE],
            overflow: BinaryHeap::new(),
            current_tick: 0,
            next_id: 0,
        }
    }
    
    pub fn schedule(&mut self, delay_ms: u64, event: ScheduledEvent) -> TimerId {
        let delay_ticks = delay_ms / TICK_MS;
        let deadline = self.current_tick + delay_ticks;
        let id = TimerId(self.next_id);
        self.next_id += 1;
        
        let entry = TimerEntry {
            id, deadline_tick: deadline, event, repeat_interval: None, priority: 128,
        };
        
        self.insert_entry(entry);
        id
    }
    
    pub fn schedule_repeating(
        &mut self, delay_ms: u64, interval_ms: u64, event: ScheduledEvent
    ) -> TimerId {
        let delay_ticks = delay_ms / TICK_MS;
        let interval_ticks = interval_ms / TICK_MS;
        let deadline = self.current_tick + delay_ticks;
        let id = TimerId(self.next_id);
        self.next_id += 1;
        
        let entry = TimerEntry {
            id, deadline_tick: deadline, event,
            repeat_interval: Some(interval_ticks), priority: 128,
        };
        
        self.insert_entry(entry);
        id
    }
    
    fn insert_entry(&mut self, entry: TimerEntry) {
        let delta = entry.deadline_tick.saturating_sub(self.current_tick);
        
        if delta < RING_SIZE as u64 {
            let slot = (entry.deadline_tick % RING_SIZE as u64) as usize;
            self.ring0[slot].push(entry);
        } else if delta < (RING_SIZE * RING_SIZE) as u64 {
            let slot = ((entry.deadline_tick / RING_SIZE as u64) % RING_SIZE as u64) as usize;
            self.ring1[slot].push(entry);
        } else if delta < (RING_SIZE * RING_SIZE * RING_SIZE) as u64 {
            let slot = ((entry.deadline_tick / (RING_SIZE * RING_SIZE) as u64) % RING_SIZE as u64) as usize;
            self.ring2[slot].push(entry);
        } else {
            self.overflow.push(entry);
        }
    }
    
    pub fn advance(&mut self) -> Vec<ScheduledEvent> {
        self.current_tick += 1;
        let slot0 = (self.current_tick % RING_SIZE as u64) as usize;
        
        // Exécuter ring 0
        let mut events = Vec::new();
        let entries = std::mem::take(&mut self.ring0[slot0]);
        for entry in entries {
            if entry.deadline_tick <= self.current_tick {
                events.push(entry.event);
                if let Some(interval) = entry.repeat_interval {
                    let next = TimerEntry {
                        deadline_tick: self.current_tick + interval,
                        ..entry
                    };
                    self.insert_entry(next);
                }
            } else {
                self.ring0[slot0].push(entry);
            }
        }
        
        // Promouvoir ring 1 → ring 0 (quand ring 0 wrappe)
        if slot0 == 0 {
            let slot1 = ((self.current_tick / RING_SIZE as u64) % RING_SIZE as u64) as usize;
            let entries = std::mem::take(&mut self.ring1[slot1]);
            for entry in entries {
                let new_slot = (entry.deadline_tick % RING_SIZE as u64) as usize;
                self.ring0[new_slot].push(entry);
            }
            
            // Promouvoir ring 2 → ring 1 (quand ring 1 wrappe)
            if slot1 == 0 {
                let slot2 = ((self.current_tick / (RING_SIZE * RING_SIZE) as u64) % RING_SIZE as u64) as usize;
                let entries = std::mem::take(&mut self.ring2[slot2]);
                for entry in entries {
                    let new_slot = ((entry.deadline_tick / RING_SIZE as u64) % RING_SIZE as u64) as usize;
                    self.ring1[new_slot].push(entry);
                }
                
                // Promouvoir overflow → ring 2
                while let Some(entry) = self.overflow.peek() {
                    let delta = entry.deadline_tick.saturating_sub(self.current_tick);
                    if delta < (RING_SIZE * RING_SIZE * RING_SIZE) as u64 {
                        let entry = self.overflow.pop().unwrap();
                        self.insert_entry(entry);
                    } else {
                        break;
                    }
                }
            }
        }
        
        // Trier par priorité
        events.sort_by_key(|e| e.priority());
        events
    }
    
    pub fn cancel(&mut self, id: TimerId) -> bool {
        // Parcourir les rings et retirer l'entrée (lazy cancel via tombstone est plus efficace)
        // En production : marquer comme cancelled, filtrer à l'exécution
        todo!()
    }
}
```

## 4. Intégration ECS

```rust
// Le TimerWheel est un Resource global
pub struct SimulationScheduler {
    pub timer_wheel: TimerWheel,
}

// @phase 10 (premier system exécuté)
pub fn scheduler_tick_system(world: &mut World, ctx: &mut Context) {
    let scheduler = world.resource_mut::<SimulationScheduler>();
    let events = scheduler.timer_wheel.advance();
    
    for event in events {
        match event {
            ScheduledEvent::DecayCheck(id) => emit DecayCheckEvent(id),
            ScheduledEvent::ResourceRegen(id) => emit ResourceRegenEvent(id),
            ScheduledEvent::BuffExpire(id, buff) => emit BuffExpireEvent(id, buff),
            ScheduledEvent::AITick(id) => emit AITickEvent(id),
            ScheduledEvent::CraftComplete(id, recipe) => emit CraftCompleteEvent(id, recipe),
            ScheduledEvent::EcologicalTick(zone) => emit EcologicalTickEvent(zone),
            ScheduledEvent::MaintenanceDue(id) => emit MaintenanceDueEvent(id),
            ScheduledEvent::Custom(f) => f(world),
        }
    }
}
```

## 5. Multi-thread safety

En Rust, le `TimerWheel` est un `Resource` du World ECS. L'accès est contrôlé par le borrow checker :
- **Single-writer** : seul `scheduler_tick_system` (phase 10) modifie le wheel
- **Schedule depuis d'autres systems** : via une `CommandQueue` thread-safe qui est vidée au début du prochain tick
- Pas de `Mutex` nécessaire si l'architecture ECS garantit l'exclusivité par phase

```rust
pub struct TimerCommandQueue {
    queue: crossbeam::queue::SegQueue<TimerCommand>,
}

pub enum TimerCommand {
    Schedule { delay_ms: u64, event: ScheduledEvent },
    Cancel { id: TimerId },
}

// Début de scheduler_tick_system :
while let Some(cmd) = timer_queue.queue.pop() {
    match cmd {
        TimerCommand::Schedule { delay_ms, event } => {
            scheduler.timer_wheel.schedule(delay_ms, event);
        }
        TimerCommand::Cancel { id } => {
            scheduler.timer_wheel.cancel(id);
        }
    }
}
```

---

# VII — ALLUMINA WORLD STATE ENGINE (Persistence)

## 1. Ce qu'UO faisait

- `World.Save()` : sérialisation complète de toutes les entités
- ServUO : single-threaded, pause réseau complète pendant le save (~2–5s)
- ModernUO : snapshot rapide + écriture async multi-thread
- Format : `.idx` (index serial→offset) + `.tdb` (type database) + `.bin` (données)
- Versioned serialization : chaque type a un numéro de version, migration manuelle

**Limites héritées :**
- ServUO : pause réseau inacceptable
- Pas d'incremental save
- Pas d'event sourcing
- Migration manuelle des versions

## 2. Version Allumina : persistence incrémentale souveraine

### Architecture (conforme LOI-3)

| Mode | Stockage | Stratégie |
|------|----------|-----------|
| **Solo** | KindMother locale (COG joueur) | Snapshot périodique (auto-save) |
| **Lobby (hôte)** | KindMother (COG hôte) | Incrémental + snapshot périodique |
| **Lobby (client)** | Personnage local + cache monde | Sync au connect/disconnect |

### Dirty tracking

```rust
pub struct WorldPersistence {
    pub save_interval: Duration,           // ex: 5 minutes
    pub snapshot_interval: Duration,       // ex: 30 minutes
    pub last_incremental: Instant,
    pub last_snapshot: Instant,
    pub dirty_entities: HashSet<EntityId>,
    pub deleted_entities: Vec<EntityId>,
    pub change_log: Vec<ChangeEntry>,      // pour event sourcing futur
}

pub struct ChangeEntry {
    pub tick: u64,
    pub entity: EntityId,
    pub component: ComponentTypeId,
    pub old_hash: u64,
    pub new_hash: u64,
}
```

### Système de sauvegarde

```rust
// @phase 990
pub fn incremental_save_system(world: &mut World, ctx: &mut Context) {
    let persistence = world.resource_mut::<WorldPersistence>();
    
    if ctx.now - persistence.last_incremental < persistence.save_interval {
        return;
    }
    
    // 1. Collecter les entités dirty
    let dirty: Vec<EntityId> = persistence.dirty_entities.drain().collect();
    let deleted: Vec<EntityId> = persistence.deleted_entities.drain(..).collect();
    
    if dirty.is_empty() && deleted.is_empty() {
        persistence.last_incremental = ctx.now;
        return;
    }
    
    // 2. Sérialiser (snapshot des composants dirty)
    let mut buffer = Vec::with_capacity(dirty.len() * 256);
    for entity_id in &dirty {
        serialize_entity_components(world, *entity_id, &mut buffer);
    }
    
    // 3. Écriture async (pas de pause du game loop)
    let save_task = SaveTask {
        data: buffer,
        deleted,
        timestamp: ctx.game_time,
    };
    ctx.spawn_async(async move {
        write_incremental_save(save_task).await;
    });
    
    persistence.last_incremental = ctx.now;
}

// @phase 991
pub fn full_snapshot_system(world: &mut World, ctx: &mut Context) {
    let persistence = world.resource::<WorldPersistence>();
    
    if ctx.now - persistence.last_snapshot < persistence.snapshot_interval {
        return;
    }
    
    // Snapshot complet en background thread
    let snapshot = snapshot_entire_world(world);
    ctx.spawn_async(async move {
        write_full_snapshot(snapshot).await;
    });
}
```

### Format de fichiers

```
allumina_save/
├── meta.json                    (seed, game_time, version, server_id)
├── snapshot/
│   ├── entities.idx             (serial → offset)
│   ├── entities.bin             (données sérialisées)
│   ├── regions.bin              (terrain, régions, nations)
│   └── ecology.bin              (zones écologiques, populations)
├── incremental/
│   ├── inc_00001.bin            (delta depuis snapshot)
│   ├── inc_00002.bin
│   └── ...
├── economy/
│   └── economy_state.json       (inflation, supply/demand, treasury)
└── characters/
    └── char_{serial}.bin         (personnage exportable)
```

---

# VIII — ALLUMINA DISTRIBUTED AUTHORITY MODEL (Network)

## 1. Ce qu'UO faisait

- Login Gateway → Game Server (séparation authentification / gameplay)
- Serveur 100% autoritaire (pas de client prediction)
- Protocole binaire custom, Huffman compression
- ModernUO : `stackalloc` zero-alloc, `epoll`/`kqueue`

## 2. Version Allumina : intégration MWS

### Flux de connexion

```
┌─────────┐    ┌──────────┐    ┌──────────────┐    ┌──────────────┐
│ Client  │    │ Tracker  │    │  Origin/     │    │ Lobby Hôte   │
│ (COG    │───→│  MWS     │───→│  Relay MWS   │───→│ (COG Hôte)   │
│ joueur) │    │(catalogue)│   │(tunnel)      │    │              │
└─────────┘    └──────────┘    └──────────────┘    └──────────────┘

Étapes :
1. Client interroge le Tracker MWS → liste des Lobbys Allumina
2. Client sélectionne un Lobby
3. Demande de Permis de circulation (MWS)
4. Relay établit le tunnel vers le COG hôte
5. COG hôte délivre l'accord d'hôte
6. Connexion établie → flux de jeu Allumina
```

### Modèle d'autorité

| Aspect | Qui décide | Client prediction |
|--------|-----------|-------------------|
| **Position** | Serveur (hôte) | Oui (réconciliation) |
| **Combat** | Serveur | Non |
| **Skill check** | Serveur | Non |
| **Inventaire** | Serveur | Non |
| **Trade** | Serveur + validation bilatérale | Non |
| **Craft** | Serveur | Non |
| **Troupes** | Serveur (ordres → serveur) | Ordres optimistes |
| **Chat** | Serveur (relay) | Optimiste |

### Interest Management

```
Zone d'intérêt = rayon autour du joueur (configurable, ex: 64 tiles)

Chaque tick réseau :
  Pour chaque joueur :
    1. Calculer la zone d'intérêt (secteurs dans le rayon)
    2. Collecter les entités dans ces secteurs
    3. Diff avec l'état précédent envoyé au client
    4. Envoyer les deltas uniquement
    
Optimisations :
  - Entités statiques → envoyées une fois, pas de delta
  - Entités hors zone → DestroyEntity packet
  - Troupes du joueur → toujours dans la zone (même si loin)
  - Batailles DW → élargir la zone temporairement
```

---

# PARTIE C — SYSTÈMES TRANSVERSAUX

---

# IX — ALLUMINA LIVING ECONOMY ENGINE

## Boucle économique

```
┌────────────────────────────────────────────────────────────┐
│              BOUCLE ÉCONOMIQUE ALLUMINA                      │
│                                                              │
│  FAUCETS (sources d'or)              SINKS (drains d'or)    │
│  ─────────────────────              ──────────────────────    │
│  • Loot monstres                    • Impôts nations         │
│  • Quêtes                           • Entretien structures   │
│  • Vente NPC (matériaux)            • Réparation équipement  │
│  • Trésors explorés                 • Consommables           │
│  • Récompenses guerre               • Frais mercenaires NPC  │
│                                     • Taxe marchands joueur  │
│                                     • Perte à la mort (PvP)  │
│                                     • Enseignement PNJ       │
│                                     • Achat terrain          │
│                                                              │
│  MARCHÉ JOUEUR-JOUEUR                                        │
│  ─────────────────────                                       │
│  Échange direct (trade window)                               │
│  Comptoir de vente (territory)                               │
│  Caravanes (transport inter-régions)                         │
│                                                              │
│  RÉGULATION                                                  │
│  ──────────                                                  │
│  Si inflation > seuil :                                      │
│    → NPC augmentent les prix                                 │
│    → Impôts augmentent                                       │
│    → Faucets diminuent (loot réduit)                         │
│  Si déflation < seuil :                                      │
│    → NPC baissent les prix                                   │
│    → Quêtes donnent plus d'or                                │
│    → Impôts baissent                                         │
└────────────────────────────────────────────────────────────┘
```

## Components MGE

```rust
pub struct EconomyState {
    pub gold_in_circulation: u64,
    pub active_players: u32,
    pub target_gold_per_player: u64,
    pub inflation_index: f64,          // 1.0 = neutre, >1 = inflation
    pub sink_multiplier: f64,
    pub faucet_multiplier: f64,
    pub price_history: Vec<PriceSnapshot>,
    pub resource_supply: HashMap<ResourceTypeId, u64>,
    pub resource_demand: HashMap<ResourceTypeId, u64>,
}

pub struct PriceSnapshot {
    pub tick: u64,
    pub avg_prices: HashMap<ItemTypeId, f64>,
}

pub struct NationTreasury {
    pub nation: NationId,
    pub gold: u64,
    pub tax_income_per_cycle: u64,
    pub military_expense: u64,
    pub infrastructure_expense: u64,
}
```

## Régulateur automatique

```rust
// @phase 600
pub fn economy_regulation_system(world: &mut World, ctx: &mut Context) {
    let econ = world.resource_mut::<EconomyState>();
    let target = econ.target_gold_per_player * econ.active_players as u64;
    let ratio = econ.gold_in_circulation as f64 / target.max(1) as f64;
    
    econ.inflation_index = ratio;
    
    if ratio > 1.3 {
        // Forte inflation → sinks agressifs
        econ.sink_multiplier = 1.0 + (ratio - 1.0) * 0.8;
        econ.faucet_multiplier = 0.5;
    } else if ratio > 1.1 {
        // Inflation modérée
        econ.sink_multiplier = 1.0 + (ratio - 1.0) * 0.3;
        econ.faucet_multiplier = 0.8;
    } else if ratio < 0.7 {
        // Forte déflation → faucets agressifs
        econ.faucet_multiplier = 1.0 + (1.0 - ratio) * 0.8;
        econ.sink_multiplier = 0.5;
    } else if ratio < 0.9 {
        // Déflation modérée
        econ.faucet_multiplier = 1.0 + (1.0 - ratio) * 0.3;
        econ.sink_multiplier = 0.8;
    } else {
        econ.sink_multiplier = 1.0;
        econ.faucet_multiplier = 1.0;
    }
}
```

## Rareté géographique et guerre économique

La rareté géographique (section III) crée naturellement des routes commerciales :

```
[Nord : minerais] ←── caravanes ──→ [Sud : bois, herbes]
        ↕                                    ↕
[Est : cristaux] ←── caravanes ──→ [Ouest : cuir, nourriture]

Guerre économique :
- Bloquer les routes → pénurie chez l'ennemi
- Taxer les caravanes traversant votre territoire
- Monopoliser une ressource rare → contrôle du marché
```

---

# X — SIMULATION ÉCOLOGIQUE (détails avancés)

## Régénération et désertification

```
Fertilité d'une zone = f(climat, sol, stress)

Si stress > 0.8 pendant 30 jours simulés :
  → Zone entre en DÉSERTIFICATION
  → Végétation tombe à 0
  → Animaux migrent
  → Nœuds de ressources ne se régénèrent plus
  → Seul le temps (sans exploitation) restaure la zone

Temps de restauration :
  stress 0.8 → 1.0 : ~60 jours simulés pour revenir à 0.5
  Désertification complète : ~180 jours simulés

Joueurs doivent GÉRER les ressources :
  → Ne pas surexploiter → rotation des zones
  → Coopération inter-guildes pour préserver
  → Terre sauvage : pas de régulation → surexploitation rapide
```

## Migration des animaux

```rust
// Quand food < migration_threshold dans une zone :
pub fn animal_migration_system(world: &mut World, ctx: &mut Context) {
    for event in world.events::<MigrationEvent>() {
        let source_zone = world.get::<EcologicalZone>(event.from);
        
        // Trouver zone adjacente avec meilleure nourriture
        let adjacent = get_adjacent_zones(event.from);
        let best_zone = adjacent.iter()
            .max_by_key(|z| {
                let zone = world.get::<EcologicalZone>(**z);
                (zone.vegetation_density * 1000.0) as i32
            });
        
        if let Some(target) = best_zone {
            // Transférer une partie de la population
            let pop = world.get_mut::<AnimalPopulation>(event.from, event.species);
            let migrants = pop.count / 4;  // 25% migre
            pop.count -= migrants;
            
            let target_pop = world.get_mut::<AnimalPopulation>(*target, event.species);
            target_pop.count += migrants;
            
            emit AnimalMigrationEvent {
                species: event.species,
                from: event.from,
                to: *target,
                count: migrants,
            };
        }
    }
}
```

---

# XI — TERRITORIALITÉ ET CONTRÔLE DE ZONES

## Taxation locale

Les nations NPC prélèvent des impôts sur les activités dans leurs régions :

| Activité | Impôt Metropolitan | Provincial | Frontier |
|----------|-------------------|------------|----------|
| Vente marchand joueur | 10% | 5% | 0% |
| Entretien structure | 100% coût | 70% coût | 30% coût |
| Récolte ressources | 0% | 0% | 0% |
| Caravane traversante | 5% valeur | 2% valeur | 0% |

## Guerre de guildes et influence

Les guildes peuvent acquérir de l'influence auprès des nations NPC :

```rust
pub struct GuildInfluence {
    pub guild: GuildId,
    pub nation: NationId,
    pub influence: f64,         // 0.0–100.0
    pub rank: GuildNationRank,  // Unknown, Known, Allied, Favored
}
```

Un seuil d'influence élevé peut :
- Réduire les impôts pour les membres
- Accorder des terrains en ville
- Obtenir des contrats militaires (guerre)
- Obtenir des routes commerciales protégées

---

# XII — SANDBOX ÉMERGENT

## Boucles systémiques interconnectées

```
┌──────────────────────────────────────────────────────────────┐
│              BOUCLES D'ÉMERGENCE ALLUMINA                     │
│                                                                │
│  [Surexploitation forêt]                                      │
│       ↓                                                       │
│  [Désertification → bois rare]                                │
│       ↓                                                       │
│  [Prix du bois augmente]                                      │
│       ↓                                                       │
│  [Caravanes de bois depuis d'autres régions]                  │
│       ↓                                                       │
│  [Bandits attaquent les caravanes (PvP émergent)]             │
│       ↓                                                       │
│  [Guildes engagent des escortes (mercenaires joueurs)]        │
│       ↓                                                       │
│  [Économie d'escorte émerge]                                  │
│       ↓                                                       │
│  [Nation déclare la zone protégée → patrouilles]              │
│       ↓                                                       │
│  [Impôts augmentent pour financer les patrouilles]            │
│       ↓                                                       │
│  [Joueurs migrent vers une autre région]                      │
│       ↓                                                       │
│  [Forêt se régénère → cycle recommence]                       │
└──────────────────────────────────────────────────────────────┘
```

## Anti-dérives sandbox

| Dérive | Mécanisme de régulation |
|--------|------------------------|
| **Grief PvP** | Karma + murder count → statut criminel → chasseurs de prime NPC + joueurs |
| **Monopole économique** | Sinks naturels + régulation inflation + concurrence NPC |
| **Exploitation abusive** | Désertification → pénurie → auto-régulation |
| **Alt-abuse** | Limite par COG, pas par personnage |
| **Botting** | Anti-macro (variété action requise) + détection patterns |
| **Housing spam** | Impôt progressif + limite par statut |
| **Grief siège** | Cooldown entre sièges + coût militaire (troupes) |
| **Duplication** | Serveur autoritaire + sérialisation vérifiée |

---

# PARTIE D — BLUEPRINT GLOBAL

---

# XIII — ARCHITECTURE MODULAIRE

## Arborescence dossier Allumina

```
allumina/
├── core/
│   ├── simulation_scheduler/        → Timer wheel + tick loop
│   │   ├── timer_wheel.rs
│   │   ├── scheduled_event.rs
│   │   └── command_queue.rs
│   ├── world_state/                 → Persistence (KindMother)
│   │   ├── dirty_tracker.rs
│   │   ├── incremental_save.rs
│   │   ├── full_snapshot.rs
│   │   └── serialization.rs
│   ├── event_bus/                   → Event dispatch central
│   │   ├── event_types.rs
│   │   └── dispatcher.rs
│   └── spatial_index/               → Sector grid + queries
│       ├── sector_grid.rs
│       └── spatial_query.rs
│
├── world/
│   ├── map_engine/                  → Tuiles, terrain, statics
│   │   ├── tile_map.rs
│   │   ├── tile_flags.rs
│   │   └── pathfinding.rs
│   ├── region_governance/           → Nations, lois, PvP rules
│   │   ├── region.rs
│   │   ├── nation.rs
│   │   ├── war_system.rs
│   │   └── pvp_flagging.rs
│   ├── ecological_simulation/       → Climat, fertilité, populations
│   │   ├── eco_zone.rs
│   │   ├── climate.rs
│   │   ├── population.rs
│   │   └── migration.rs
│   └── territory_engine/            → Housing, structures, decay, siège
│       ├── territory.rs
│       ├── access_control.rs
│       ├── functional_slot.rs
│       ├── decay.rs
│       └── siege.rs
│
├── entity/
│   ├── character/                   → Stats, skills, equipment
│   │   ├── stats.rs
│   │   ├── derived.rs
│   │   ├── combat_aptitudes.rs
│   │   └── equipment.rs
│   ├── creature_ai/                 → IA monstres + PNJ
│   │   ├── fsm.rs
│   │   ├── aggro.rs
│   │   ├── group_behavior.rs
│   │   └── boss_modifiers.rs
│   ├── troop_system/                → Troupes, ordres tactiques
│   │   ├── troop_command.rs
│   │   ├── formation.rs
│   │   ├── orders.rs
│   │   └── dw_battle.rs            → Dynasty Warriors mode
│   └── follower_ai/                 → Mercenaires, invocations, animaux
│       ├── follow.rs
│       ├── target_selection.rs
│       └── tamed_animal.rs
│
├── interaction/
│   ├── combat_engine/               → Dégâts, résistances, aptitudes
│   │   ├── attack_sequence.rs       → atk vs esq → atk vs par → dégâts
│   │   ├── damage_types.rs          → Tranc/Cont/Perc vs ARt/ARc/ARp
│   │   ├── critical.rs             → Luk seuil, critik 150%+mod
│   │   └── ranged.rs               → tirC/tirP/tirE
│   ├── skill_engine/                → Progression par usage + enseignement
│   │   ├── skill_check.rs           → Opposition 50%+1%/pt
│   │   ├── skill_gain.rs           → Formule gain probabiliste
│   │   ├── teaching.rs             → Cycles enseignement joueur→joueur
│   │   └── anti_macro.rs
│   ├── harvest_engine/              → Récolte écologique
│   │   ├── harvest.rs
│   │   └── tool_validation.rs
│   ├── craft_engine/                → Chaînes de production
│   │   ├── recipe.rs
│   │   ├── quality.rs
│   │   ├── production_chain.rs
│   │   └── crafter_reputation.rs
│   └── magic_engine/                → Sorts, mana, écoles
│       ├── spell.rs
│       ├── cast.rs
│       └── necromancy.rs            → Pool morts-vivants séparé
│
├── economy/
│   ├── currency/                    → Or, banque
│   │   └── gold.rs
│   ├── trade/                       → Échange joueur-joueur
│   │   └── trade_session.rs
│   ├── vendor/                      → Marchands NPC + joueur
│   │   ├── npc_vendor.rs
│   │   └── player_vendor.rs
│   ├── market/                      → Offre/demande dynamique
│   │   └── price_tracker.rs
│   ├── inflation_control/           → Régulation automatique
│   │   └── regulator.rs
│   └── caravan/                     → Transport inter-régions
│       └── caravan.rs
│
├── social/
│   ├── karma/                       → Karma, fame, murder counts
│   │   └── karma.rs
│   ├── guild/                       → Guildes, alliances, guerres
│   │   ├── guild.rs
│   │   └── guild_influence.rs
│   ├── reputation/                  → Statut social (esclave→héros)
│   │   └── social_status.rs
│   └── nation_relation/             → Relations joueur-nation
│       └── nation_rep.rs
│
├── network/
│   ├── mws_integration/             → Lobby, Tracker, découverte
│   │   ├── lobby.rs
│   │   └── tracker_client.rs
│   ├── packet_handler/              → Packets Allumina
│   │   ├── packets.rs
│   │   └── handlers.rs
│   ├── interest_management/         → Filtrage par zone d'intérêt
│   │   └── interest.rs
│   └── authority_model/             → Serveur autoritaire + prédiction
│       ├── server_authority.rs
│       └── client_prediction.rs
│
└── config/
    ├── world.json
    ├── skills.json
    ├── recipes.json
    ├── resource_nodes.json
    ├── ecological_zones.json
    ├── regions.json
    ├── nations.json
    ├── troop_types.json
    └── economy.json
```

## Dépendances entre modules

```
core/simulation_scheduler ← (TOUS les modules)
core/event_bus            ← (TOUS les modules)
core/spatial_index        ← world/*, entity/*, interaction/*
core/world_state          ← (sauvegarde de tous les composants)

world/map_engine          ← region_governance, territory_engine, ecological_simulation
world/region_governance   ← entity/creature_ai (spawn), social/pvp_flagging
world/ecological_simulation ← interaction/harvest_engine
world/territory_engine    ← economy/currency (impôts), social/reputation

entity/character          ← interaction/* (toutes les interactions)
entity/creature_ai        ← world/region_governance (spawn rules)
entity/troop_system       ← entity/follower_ai, interaction/combat_engine
entity/follower_ai        ← entity/character (owner)

interaction/combat_engine ← entity/character (stats), world/region_governance (PvP rules)
interaction/skill_engine  ← entity/character (skills)
interaction/harvest_engine ← world/ecological_simulation (nœuds)
interaction/craft_engine  ← economy/* (consommation, prix)

economy/inflation_control ← economy/currency, economy/market
economy/vendor            ← economy/inflation_control (prix dynamiques)
economy/caravan           ← world/region_governance (taxes), interaction/combat_engine (attaques)

social/karma              ← interaction/combat_engine (kills PvP)
social/guild              ← world/region_governance (influence)
social/reputation         ← social/karma, economy/trade (transactions)

network/mws_integration   ← (indépendant, couche transport)
network/interest_management ← core/spatial_index
network/authority_model   ← (toutes les interactions passent par le serveur)
```

---

# XIV — CONFIGURATIONS

## world.json

```json
{
  "world": {
    "name": "Allumina Prime",
    "seed": 42,
    "tick_rate": 30,
    "time_scale": 24.0,
    "map_size": [4096, 4096],
    "sector_size": 16,
    "save_interval_sec": 300,
    "snapshot_interval_sec": 1800
  },
  "ecology": {
    "simulation_interval_sec": 600,
    "desertification_threshold": 0.8,
    "desertification_duration_days": 30,
    "recovery_rate": 0.005,
    "migration_threshold": 0.3
  },
  "economy": {
    "target_gold_per_player": 50000,
    "inflation_high_threshold": 1.3,
    "inflation_low_threshold": 0.7,
    "sink_multiplier_max": 2.0,
    "faucet_multiplier_max": 2.0
  }
}
```

## troop_types.json

```json
{
  "troop_types": [
    {
      "id": "militia",
      "name": "Milicien",
      "cmd_prerequisite": 20,
      "cmd_cost": 5,
      "stats": { "hp": 50, "atk": 15, "esq": 10, "par": 12 },
      "equipment": "light_armor",
      "recruitment": { "source": "town", "gold_cost": 100 },
      "maintenance_gold_per_cycle": 10
    },
    {
      "id": "guard",
      "name": "Garde",
      "cmd_prerequisite": 25,
      "cmd_cost": 10,
      "stats": { "hp": 80, "atk": 25, "esq": 15, "par": 20 },
      "equipment": "medium_armor",
      "recruitment": { "source": "town", "gold_cost": 300 },
      "maintenance_gold_per_cycle": 25
    },
    {
      "id": "knight",
      "name": "Chevalier",
      "cmd_prerequisite": 50,
      "cmd_cost": 25,
      "stats": { "hp": 150, "atk": 45, "esq": 20, "par": 35 },
      "equipment": "heavy_armor",
      "recruitment": { "source": "nation_army", "rank_required": "officer" },
      "maintenance_gold_per_cycle": 80
    }
  ]
}
```

## resource_nodes.json

```json
{
  "resource_nodes": [
    {
      "type": "iron_vein",
      "base_regen_rate": 2.0,
      "max_capacity": 50,
      "required_skill": "mining",
      "min_skill": 0,
      "required_tool": "pickaxe",
      "geographic_zones": ["northern_mountains", "deep_caves"],
      "rarity": "common",
      "harvest_amount": { "min": 1, "max": 3 }
    },
    {
      "type": "mithril_vein",
      "base_regen_rate": 0.2,
      "max_capacity": 10,
      "required_skill": "mining",
      "min_skill": 80,
      "required_tool": "adamantine_pickaxe",
      "geographic_zones": ["volcanic_depths", "ancient_mines"],
      "rarity": "rare",
      "harvest_amount": { "min": 1, "max": 1 }
    },
    {
      "type": "oak_tree",
      "base_regen_rate": 1.0,
      "max_capacity": 30,
      "required_skill": "lumberjacking",
      "min_skill": 20,
      "required_tool": "axe",
      "geographic_zones": ["southern_forest", "central_woods"],
      "rarity": "common",
      "harvest_amount": { "min": 2, "max": 5 }
    }
  ]
}
```

---

# XV — VERSIONS MONO-SERVEUR ET CLUSTER

## Version mono-serveur (Lobby MWS)

```
┌──────────────────────────────────────────┐
│          COG HÔTE (Lobby Allumina)        │
│                                            │
│  ┌────────────────────────────────────┐   │
│  │        GAME LOOP (30 TPS)          │   │
│  │                                    │   │
│  │  [Input] → [Simulation] → [Output] │   │
│  │                                    │   │
│  │  Phase 10  : Scheduler tick        │   │
│  │  Phase 50  : Network input         │   │
│  │  Phase 100 : Ecological sim        │   │
│  │  Phase 200 : Movement              │   │
│  │  Phase 300 : Combat, Harvest, Craft│   │
│  │  Phase 400 : Economy               │   │
│  │  Phase 500 : Skills                │   │
│  │  Phase 600 : AI, Economy regul     │   │
│  │  Phase 700 : Social, PvP flags     │   │
│  │  Phase 900 : Persistence           │   │
│  │  Phase 950 : Network output        │   │
│  └────────────────────────────────────┘   │
│                                            │
│  ┌────────────┐  ┌────────────────────┐   │
│  │ KindMother │  │ MWS Integration    │   │
│  │ (DB locale)│  │ (Lobby + Tracker)  │   │
│  └────────────┘  └────────────────────┘   │
│                                            │
│  Capacité : ~200 joueurs (estimation)      │
│  Limitation : single-threaded ECS          │
└──────────────────────────────────────────┘
```

## Version cluster MMO (2030-ready)

```
┌─────────────────────────────────────────────────────────────┐
│                    CLUSTER ALLUMINA                           │
│                                                               │
│  ┌─────────┐         ┌──────────────┐                        │
│  │ Gateway │←────────→│ Auth Service │                        │
│  │ (MWS    │         │ (comptes,    │                        │
│  │ Tracker)│         │  personnages)│                        │
│  └────┬────┘         └──────────────┘                        │
│       │                                                       │
│  ┌────┴────────────────────────────────────┐                 │
│  │           ZONE SERVERS                   │                 │
│  │                                          │                 │
│  │  ┌──────────┐  ┌──────────┐  ┌────────┐ │                 │
│  │  │ Zone A   │  │ Zone B   │  │ Zone C │ │                 │
│  │  │(Aethoria │  │(Sauvage  │  │(Kelvar │ │                 │
│  │  │  Nord)   │  │  Ouest)  │  │  is)   │ │                 │
│  │  └────┬─────┘  └────┬─────┘  └───┬────┘ │                 │
│  │       │              │             │      │                 │
│  │  ┌────┴──────────────┴─────────────┴────┐ │                │
│  │  │        MESSAGE BUS (inter-zone)       │ │                │
│  │  │  (transitions, caravanes, guerres)    │ │                │
│  │  └──────────────────────────────────────┘ │                │
│  └──────────────────────────────────────────┘                │
│                                                               │
│  ┌──────────────────────────────────────────┐                │
│  │           SERVICES PARTAGÉS               │                │
│  │                                           │                │
│  │  ┌──────────┐  ┌──────────┐  ┌────────┐ │                │
│  │  │ Economy  │  │ Ecology  │  │ Nation │  │                │
│  │  │ Service  │  │ Service  │  │ War    │  │                │
│  │  │(global)  │  │(global)  │  │Service │  │                │
│  │  └──────────┘  └──────────┘  └────────┘ │                │
│  └──────────────────────────────────────────┘                │
│                                                               │
│  ┌──────────────────────────────────────────┐                │
│  │           PERSISTENCE CLUSTER             │                │
│  │                                           │                │
│  │  ┌──────────┐  ┌──────────┐  ┌────────┐ │                │
│  │  │ DB Zone A│  │ DB Zone B│  │DB Zones│ │                │
│  │  │          │  │          │  │ C...N  │ │                │
│  │  └──────────┘  └──────────┘  └────────┘ │                │
│  └──────────────────────────────────────────┘                │
│                                                               │
│  Zone handoff :                                               │
│  Joueur quitte Zone A → sérialise → Message Bus → Zone B     │
│  → désérialise → spawn dans Zone B                            │
│                                                               │
│  Capacité cible : 10k+ joueurs                                │
└─────────────────────────────────────────────────────────────┘
```

### Zone handoff

```
fn zone_handoff(player, from_zone, to_zone):
    // 1. Freeze player sur from_zone
    freeze_entity(from_zone, player)
    
    // 2. Sérialiser l'état complet du joueur
    let snapshot = serialize_player_full(player)
    // Inclut : stats, skills, inventory, equipment, troops, buffs, position
    
    // 3. Envoyer via Message Bus
    message_bus.send(ZoneTransfer {
        player_id: player.id,
        target_zone: to_zone,
        data: snapshot,
        entry_point: calculate_entry_point(to_zone, from_zone),
    })
    
    // 4. to_zone reçoit et spawn
    // to_zone:
    let player = deserialize_player(snapshot)
    spawn_at(player, entry_point)
    
    // 5. Client reçoit la nouvelle zone
    send_to_client(player, ZoneChangePacket {
        new_zone: to_zone,
        position: entry_point,
    })
```

---

# XVI — COMPARAISONS FINALES

## UO Original vs Allumina

| Aspect | UO (1997) | Allumina (2026+) |
|--------|-----------|------------------|
| **Architecture** | Monolithique C#, héritage OOP | ECS Rust, modulaire, data-driven |
| **Threading** | Single-threaded | Multi-thread safe (ownership Rust) |
| **Timer** | Priority queue (ServUO) | Timer Wheel O(1) |
| **Persistence** | Pause réseau (ServUO) | Incrémental async |
| **Skill** | 58 skills, cap 720, GGS | 10 caracs + aptitudes + compétences, enseignement |
| **Housing** | Décoratif, decay passif | Fonctionnel, entretien actif, siège |
| **Économie** | Non régulée, inflation | Régulateur automatique, sinks dynamiques |
| **Écologie** | Respawn fixe | Simulation Lotka-Volterra, désertification |
| **Craft** | Recette directe | Chaîne de production, qualité continue |
| **PvP** | Binaire (Felucca/Trammel) | Gradient de loi, nations en guerre |
| **Troupes** | Inexistant | Groupe→Régiment (Dynasty Warriors) |
| **Combat** | Tour-par-tour masqué | Temps réel (Diablo-like) |
| **Network** | Protocole propriétaire | MWS (Lobby, Tracker, tunnel) |
| **Scalabilité** | ~2000 joueurs/shard | 10k+ (cluster) |

## ModernUO vs Allumina

| Aspect | ModernUO | Allumina |
|--------|----------|----------|
| **Langage** | C# .NET 8 | Rust |
| **Architecture** | Héritage OOP modernisé | ECS pur |
| **Timer** | Timer Wheel 3 anneaux | Timer Wheel 3 anneaux + overflow |
| **Persistence** | Multi-thread round-robin | Incrémental + snapshot async |
| **Network** | stackalloc zero-alloc | MWS tunnel + authority model |
| **Config** | C# scripts | JSON/RON data-driven, hot reload |
| **Gameplay** | Fidèle UO | UO + Diablo + Dynasty Warriors |

## Ce qu'il faut conserver d'UO

1. **Progression par usage** — la sensation que pratiquer = progresser
2. **Économie player-driven** — les joueurs sont le moteur économique
3. **Housing monde ouvert** — ancrage émotionnel dans le monde
4. **Craft significatif** — les objets craftés ont de la valeur
5. **Monde unique persistant** — sentiment d'appartenance communautaire
6. **PvP à conséquences** — karma, réputation, chasseurs de prime
7. **Serveur autoritaire** — intégrité du gameplay

## Ce qu'il faut abandonner

1. **Single-threaded monolithique** — ne scale pas
2. **Héritage OOP profond** — maintenance impossible
3. **Skill gain purement aléatoire** — frustrant
4. **Économie sans régulation** — inflation terminale
5. **Maps séparées** — fracture communauté
6. **Housing sans fonction** — gaspillage de terrain
7. **Respawn fixe** — monde mort
8. **Pause réseau au save** — inacceptable

## Ce qu'il faut radicalement moderniser

1. **Écologie dynamique** → monde vivant, conséquences réelles
2. **Chaînes de production** → profondeur craft, interdépendances joueurs
3. **Régulation économique automatique** → stabilité long terme
4. **Gradient de loi** → nuances au lieu de binaire PvP/PvE
5. **Troupes et batailles** → échelle épique (Dynasty Warriors)
6. **Enseignement joueur→joueur** → lien social profond
7. **Architecture cluster** → 10k+ joueurs
8. **Persistence incrémentale** → zéro interruption
9. **Nations NPC dynamiques** → monde politiquement vivant
10. **Timer Wheel** → performance MMO

---

## Références complètes

| Document | Rôle |
|----------|------|
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision service, MWS, LOI |
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Genre, monde, solo/multi |
| [Allumina - Vision Gameplay et Ambition](./Concept/Allumina%20-%20Vision%20Gameplay%20et%20Ambition.md) | UO/Diablo/DW, progression esclave→héros |
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Échelles, voies, ordres tactiques |
| [Allumina - Compétences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Progression par usage, enseignement |
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Stats, aptitudes, plafonds, formules |
| [Allumina - Extraction Architecture UO pour MGE](./Allumina%20-%20Extraction%20Architecture%20UO%20pour%20MGE.md) | Reverse-engineering UO complet |
| [Allumina - Extraction Systèmes D2 pour MGE](./Allumina%20-%20Extraction%20Systemes%20D2%20OpenDiablo2%20pour%20MGE.md) | Systèmes Diablo II |
| [Allumina - Analyse Technique D2 pour MGE](./Allumina%20-%20Analyse%20Technique%20Diablo%20II%20pour%20MGE.md) | Analyse moteur D2 |

---

**Document** : Allumina — Blueprint Moteur Sandbox MGE  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Blueprint normatif
