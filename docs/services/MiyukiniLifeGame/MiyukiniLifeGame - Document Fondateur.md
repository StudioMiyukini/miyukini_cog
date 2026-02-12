# MiyukiniLifeGame — Document Fondateur

## Contexte

**Miyukini Life Game** est un simulateur de vie (god simulator) sandbox où le joueur incarne une divinité omnipotente capable de créer des mondes, d'y placer des créatures et des civilisations, et d'observer leur évolution autonome. Inspiré de WorldBox, ce jeu démontre la capacité de Miyukini COG à héberger des simulations complexes multi-agents avec gouvernance intégrée.

## Portée / Scope

Ce document définit la **vision fondamentale** de Miyukini Life Game :
- Raison d'être du jeu
- Genre et gameplay principal
- Inspiration WorldBox
- Intégration dans l'écosystème Miyukini COG
- Stack technique
- Roadmap (MVP → v1.0 → extensions)

## Vision

### Raison d'être

Créer un **simulateur de vie sandbox** où :
1. Le joueur est un **dieu observateur et interventionniste**
2. Les civilisations évoluent de manière **autonome et émergente**
3. Aucun objectif imposé — **liberté totale d'expérimentation**
4. Le jeu démontre la **puissance de la stack Miyukini COG** pour des simulations complexes

### Philosophie de jeu

> "Vous n'êtes pas un joueur avec des missions. Vous êtes un dieu avec des pouvoirs."

- **Pas de game over** — Le monde continue tant que vous ne le détruisez pas
- **Pas de quêtes** — Vous définissez vos propres objectifs
- **Pas de tutoriel obligatoire** — Découverte par l'expérimentation
- **Émergence > Scripting** — Les comportements émergent de règles simples

## Genre et gameplay

### Genre principal

**God Simulator / Sandbox / Simulation de civilisations**

Sous-genres :
- Simulation de vie (Life Sim)
- Simulation politique (Political Sim)
- Simulation de colonies (Colony Sim)
- Jeu de création (Building Game)
- Jeu de destruction (Destruction Game)

### Gameplay principal

#### 1. Création de monde

**Outils de terrain :**
- Pinceaux pour dessiner continents, océans, montagnes, forêts
- Biomes : Plaines, Désert, Toundra, Jungle, Marais, Volcan
- Taille du monde : Petit (256×256), Moyen (512×512), Grand (1024×1024)
- Génération procédurale ou création manuelle

**Ressources naturelles :**
- Minerais (fer, or, diamant)
- Faune (cerfs, loups, ours)
- Flore (arbres, buissons, fleurs)

#### 2. Placement de créatures

**Races civilisées :**
- **Humains** — Équilibrés, adaptables, expansion rapide
- **Orcs** — Guerriers, agressifs, faible technologie
- **Elfes** — Longue vie, forêts, magie, pacifiques
- **Nains** — Montagnes, technologie, défensifs

**Créatures hostiles :**
- Zombies, Squelettes, Démons
- Dragons (contrôlables par le joueur)
- Tumeurs (infectent le terrain)
- Cold Ones (créatures de glace)

**Créatures neutres :**
- Animaux sauvages (loups, ours, cerfs, moutons)
- Créatures magiques (licornes, treants)

#### 3. Observation et évolution

**Les civilisations font :**
- Construire des villages, routes, fortifications
- Former des royaumes avec frontières
- Développer des technologies (âge de pierre → fer → médiéval)
- Établir des religions et cultes
- Créer des familles et lignées nobles
- Coloniser de nouveaux territoires
- Naviguer vers d'autres continents

**Diplomatie émergente :**
- Alliances et ennemis
- Guerres territoriales
- Rébellions et révolutions
- Empires qui s'étendent et s'effondrent

#### 4. Intervention divine

**Pouvoirs créatifs :**
- Donner des ressources, de la nourriture, des armes
- Bénir des unités (immortalité, force, vitesse)
- Guérir des maladies, réparer des bâtiments
- Créer des barrières magiques

**Pouvoirs destructifs :**
- Éclairs, tornades, tremblements de terre
- Météorites, volcans, lave
- Pluie acide, bombes atomiques
- Plagues (peste, zombification)

**Pouvoirs de manipulation :**
- Forcer la paix ou la guerre
- Déplacer des unités avec l'aimant divin
- Contrôler directement un dragon ou Crabzilla
- Accélérer/ralentir/pauser le temps

## Inspiration : WorldBox

### Ce que nous reprenons de WorldBox

1. **Philosophie god simulator** — Pas d'objectifs imposés
2. **Autonomie des civilisations** — Comportements émergents
3. **Diversité des pouvoirs** — Création ET destruction
4. **Pixel art** — Style visuel simple et performant
5. **Sandbox complet** — Génération procédurale + création manuelle
6. **Races distinctes** — Humains, Orcs, Elfes, Nains
7. **Diplomatie et guerres** — Émergentes, non scriptées
8. **Effets visuels** — Particules, explosions, trainées

### Ce que nous améliorons

1. **Architecture Miyukini COG** — Gouvernance intégrée
2. **Sauvegarde synchronisée** — Via KindMother
3. **Multi-device** — Desktop + Web (WASM)
4. **Extensibilité** — Nouveaux pouvoirs via Toolkits
5. **Performance** — Optimisations Rust
6. **UI moderne** — Interface Dioxus réactive
7. **Historique détaillé** — Timeline des événements
8. **Statistiques avancées** — CaringNanny pour métriques

## Intégration Miyukini COG

### Type de Service

**Service interne COG (Type 1)**
- Aucune surface web externe
- Accès via Miyukini Central uniquement
- Sauvegarde locale + sync KindMother

### Architecture pyramidale

**Strate 7 — Service**
```
MiyukiniLifeGame
└── Interface utilisateur (Dioxus)
```

**Strate 7 — Opérateurs**
```
LifeGame.Simulation     → Tick du monde, physique, AI
LifeGame.Entities       → Gestion entités (créatures, unités)
LifeGame.World          → Terrain, biomes, ressources
LifeGame.Powers         → Pouvoirs divins, effets
```

**Strate 6 — Toolkits requis**
```
MiyuWorldGen       → Génération procédurale de terrain
MiyuPixelCanvas    → Rendu pixel art et carte
MiyuEntitySim      → Simulation d'entités autonomes
MiyuDiplomacy      → Relations entre royaumes
MiyuPathfinding    → Calcul de chemins (A*, flow fields)
MiyuParticles      → Effets visuels des pouvoirs
MiyuSaveFormat     → Sérialisation/désérialisation mondes
MiyuTimeControl    → Contrôle vitesse de simulation
```

**Strate 4 — Cores utilisés**
```
StrongFather       → Autorisations pour pouvoirs destructifs
KindMother         → Sauvegarde et sync des mondes
CaringNanny        → Observation statistiques (pop, ressources)
MasterButler       → Orchestration des Toolkits
EverBuddy          → Versions de mondes, migrations
WorrySentinel      → Limites (taille monde, nb entités)
```

### Flux de gouvernance

**Exemple : Joueur lance une bombe atomique**
```
1. UI (Dioxus) → Clic sur pouvoir "Bombe Atomique"
2. Service MiyukiniLifeGame → Demande au Core StrongFather
3. StrongFather → Vérifie que le joueur a ce pouvoir
4. StrongFather → Approuve (ou refuse si limites atteintes)
5. MasterButler → Orchestre les Toolkits concernés
6. LifeGame.Powers → Crée l'explosion via MiyuParticles
7. LifeGame.Entities → Détruit unités dans le rayon
8. LifeGame.World → Crée un cratère dans le terrain
9. CaringNanny → Met à jour les statistiques
10. KindMother → Sauvegarde l'état du monde
```

## Stack technique

### Langage et frameworks

- **Rust** — Langage principal
- **Dioxus** — Framework UI (Desktop + WASM)
- **wgpu** — Rendu graphique (optionnel pour effets avancés)
- **serde** — Sérialisation
- **rayon** — Parallélisation des calculs de simulation

### Toolkits à créer

| Toolkit | Description | Priorité |
|---------|-------------|----------|
| **MiyuWorldGen** | Génération procédurale Perlin noise, biomes | MVP |
| **MiyuPixelCanvas** | Affichage pixel art, zoom, pan | MVP |
| **MiyuEntitySim** | Simulation autonome (pathfinding, besoins) | MVP |
| **MiyuDiplomacy** | Relations, guerres, alliances | v1.0 |
| **MiyuPathfinding** | A*, flow fields pour grandes armées | v1.0 |
| **MiyuParticles** | Effets visuels (explosions, feu, magie) | v1.0 |
| **MiyuTimeControl** | Pause, vitesse 1×-10×, avance rapide | MVP |
| **MiyuSaveFormat** | Format binaire compressé pour mondes | MVP |

### Toolkits à réutiliser

- **Dioxus** (UI)
- **MiyuSave** (sauvegarde générique)
- **MiyuAssets** (chargement sprites/textures)

### Format de données

**Structure d'un monde :**
```rust
struct World {
    metadata: WorldMetadata,
    terrain: TerrainGrid,
    entities: EntityManager,
    kingdoms: Vec<Kingdom>,
    history: EventLog,
}

struct WorldMetadata {
    id: WorldId,
    name: String,
    size: (u32, u32),
    seed: u64,
    created_at: Timestamp,
    playtime: Duration,
}

struct TerrainGrid {
    tiles: Vec<Vec<Tile>>,
    biomes: BiomeMap,
    resources: ResourceMap,
}

struct Tile {
    terrain_type: TerrainType, // Grass, Water, Mountain, etc.
    elevation: u8,
    temperature: i8,
    moisture: u8,
}

struct EntityManager {
    units: HashMap<EntityId, Unit>,
    buildings: HashMap<EntityId, Building>,
    creatures: HashMap<EntityId, Creature>,
}

struct Kingdom {
    id: KingdomId,
    name: String,
    race: Race,
    capital: BuildingId,
    territories: Vec<TerritoryId>,
    population: u32,
    relations: HashMap<KingdomId, Relation>,
    tech_level: TechLevel,
}
```

## Roadmap

### MVP (Version 0.1) — "Proof of Concept"

**Objectif :** Démontrer la faisabilité technique

**Fonctionnalités :**
- ✅ Création d'un monde (256×256)
- ✅ Pinceaux terrain de base (terre, eau, montagne)
- ✅ Placement d'Humains (1 race)
- ✅ Villages de base (maisons, routes simples)
- ✅ 10 pouvoirs divins (5 créatifs, 5 destructifs)
- ✅ Sauvegarde locale (1 slot)
- ✅ UI minimale (palette de pouvoirs, carte)
- ✅ Simulation simple (population, nourriture)

**Non inclus :**
- ❌ Diplomatie
- ❌ Guerres
- ❌ Autres races (Orcs, Elfes, Nains)
- ❌ Créatures hostiles
- ❌ Effets visuels avancés
- ❌ Génération procédurale

**Durée estimée :** 4-6 semaines (1 développeur)

### Beta v1.0 — "Jeu complet"

**Objectif :** Jeu jouable avec toutes les fonctionnalités principales

**Fonctionnalités ajoutées :**
- ✅ 4 races jouables (Humains, Orcs, Elfes, Nains)
- ✅ Système de royaumes et diplomatie
- ✅ Guerres et batailles
- ✅ 50+ pouvoirs divins (7 catégories)
- ✅ Créatures hostiles (zombies, démons, dragons)
- ✅ Génération procédurale de mondes
- ✅ Effets visuels (particules, explosions)
- ✅ Historique et timeline
- ✅ Statistiques détaillées (CaringNanny)
- ✅ Sauvegarde synchronisée (KindMother)
- ✅ 3 tailles de monde (256, 512, 1024)

**Durée estimée :** 12-16 semaines supplémentaires

### Extensions futures

**v1.1 — Religions et cultures**
- Systèmes de croyances
- Prophètes et miracles
- Cultes et temples

**v1.2 — Magie et technologies avancées**
- Âges technologiques (pierre → bronze → fer → industriel)
- Systèmes de magie pour Elfes
- Inventions et découvertes

**v1.3 — Créatures légendaires**
- Crabzilla (crabe géant contrôlable)
- Titans élémentaires
- Monstres de boss

**v1.4 — Modding**
- Workshop de pouvoirs personnalisés
- Nouvelles races via Toolkits
- Biomes personnalisés

## Comparaison avec autres jeux Miyukini

| Aspect | MiyuClicker | MiyukiniLifeGame | MiyukiniSurvivor |
|--------|-------------|------------------|------------------|
| Genre | Idle/Clicker | God Simulator | Survivor-like |
| Rythme | Passif | Observateur | Action intense |
| Contrôle | Direct (clic) | Indirect (pouvoirs) | Direct (WASD) |
| Objectif | Conquérir la carte | Libre | Survivre 30min |
| Simulation | Simple | Complexe (multi-agents) | Simple (vagues) |
| Rejouabilité | Slots + prestige | Mondes infinis | Runs courts |
| Complexité | Moyenne | Élevée | Faible |

## Inspirations et références

### Jeux de référence

1. **WorldBox** (principal)
   - God simulator sandbox
   - Pixel art
   - Civilisations autonomes
   - 230+ pouvoirs divins

2. **The Powder Toy**
   - Simulation physique
   - Outils de dessin
   - Effets en chaîne

3. **Dwarf Fortress**
   - Simulation profonde
   - Émergence narrative
   - Systèmes interconnectés

4. **Conway's Game of Life**
   - Automates cellulaires
   - Règles simples → comportements complexes
   - Motifs émergents

### Inspirations visuelles

- **Pixel art** — Style rétro, lisible, performant
- **Terraria** — Rendu de terrain 2D
- **Stardew Valley** — Animation des personnages
- **Noita** — Effets de particules

### Inspirations mécaniques

- **RimWorld** — Simulation de besoins et relations
- **Civilization** — Technologies et diplomatie
- **Age of Empires** — Batailles et unités
- **Spore** — Évolution et créatures

## Définition de réussite

Le jeu sera considéré comme **réussi** si :

1. ✅ **Performance** — 60 FPS avec 1000+ entités
2. ✅ **Autonomie** — Civilisations évoluent sans intervention
3. ✅ **Émergence** — Récits uniques dans chaque partie
4. ✅ **Pouvoirs** — Sensations de puissance divine
5. ✅ **Intégration COG** — Gouvernance transparente
6. ✅ **Rejouabilité** — Envie de créer de nouveaux mondes
7. ✅ **Stabilité** — Pas de crashes même après 10h+ de jeu
8. ✅ **Sauvegarde** — Mondes persistent et synchronisent

## Métriques de succès

**Métriques techniques :**
- FPS moyen > 60
- Temps de sauvegarde < 1s
- Temps de chargement < 3s
- Mémoire < 500 MB (monde moyen)

**Métriques de jeu :**
- Durée de vie moyenne > 10h par monde
- Nombre de mondes créés > 5 par utilisateur
- Taux de rétention J7 > 60%
- Satisfaction utilisateur > 4/5

## Conclusion

Miyukini Life Game est un **projet ambitieux** qui démontre la puissance de Miyukini COG pour des simulations complexes. En s'inspirant de WorldBox tout en apportant une architecture gouvernée et extensible, nous créons un god simulator unique dans l'écosystème Miyukini.

**Phase suivante :** Lire les documents détaillés sur les mécaniques de jeu et l'architecture technique.
