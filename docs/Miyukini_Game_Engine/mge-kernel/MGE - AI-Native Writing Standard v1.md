# MGE — AI-Native Writing Standard v1.1

Norme officielle d'ecriture AI-Native pour le Miyukini Game Engine. Ce document est normatif et obligatoire pour tous les plugins Phase 2+ et extensions.

## Statut

- **Type :** Norme obligatoire
- **Portee :** Tous les plugins Phase 2+, extensions, contributions tierces
- **Audience :** Developpeurs humains, Composer, LLM, contributeurs tiers
- **Version :** AI-Native v1.1
- **Retroactivite :** Non retroactive pour legacy (mge-core v0.1 exclu)
- **Historique :** v1.0 (2026-02-19) -> v1.1 (2026-02-19)

---

## Table des matieres

1. [Objectif fondamental](#1-objectif-fondamental)
2. [Principe directeur](#2-principe-directeur)
3. [Structure obligatoire d'un plugin](#3-structure-obligatoire-dun-plugin)
4. [Norme MSCM obligatoire](#4-norme-mscm-obligatoire)
5. [Norme fonctionnelle](#5-norme-fonctionnelle)
6. [Norme de compression token](#6-norme-de-compression-token)
7. [Norme Event](#7-norme-event)
8. [Norme MIP — Optimisation token](#8-norme-mip--optimisation-token)
9. [Norme index.md](#9-norme-indexmd)
10. [Norme Helper](#10-norme-helper)
11. [Regles strictes pour Composer](#11-regles-strictes-pour-composer)
12. [Verification automatique](#12-verification-automatique)
13. [Interdictions evolutives](#13-interdictions-evolutives)
14. [Norme No Hidden State](#14-norme-no-hidden-state)
15. [Norme One Responsibility per File](#15-norme-one-responsibility-per-file)
16. [AI-Native Score](#16-ai-native-score)
17. [Design philosophy](#17-design-philosophy)

---

## 1. Objectif fondamental

Le code MGE doit etre :

| Propriete | Signification |
|-----------|---------------|
| Lisible par IA sans parsing complet | Un LLM peut comprendre un module en lisant les annotations MSCM |
| Indexable par MIP | Chaque bloc public a un @id unique dans blocks.json |
| Token-efficient | Pas de commentaires narratifs, pas de code decoratif |
| Structurellement stable | Meme structure pour tous les plugins, pas de surprise |
| Sans ambiguite semantique | @requires, @writes, @emits explicitent les effets |
| Sans logique implicite | Pas de callbacks caches, pas de dispatch dynamique |
| Sans etat cache | Pas de static mut, pas de globales mutables |

Le code n'est pas seulement execute. Il est interroge par des agents IA.

---

## 2. Principe directeur

### 2.1 Separation stricte

| Element | Role |
|---------|------|
| Code | Execution |
| MSCM | Semantique |
| MIP | Index |
| blocks.json | Interface machine |
| domains.json | Carte des domaines |

Le code ne porte pas sa propre documentation narrative. La semantique est dans les annotations MSCM. L'index est genere par MIP. Les agents IA lisent blocks.json pour naviguer.

### 2.2 Regle des 30 lignes

> Un LLM ne doit jamais avoir besoin de lire plus de 30 lignes pour comprendre une fonctionnalite.

Si une fonction depasse 30 lignes, elle doit etre decoupee. Si un fichier depasse 300 lignes, il doit etre scinde (cf. section 15).

---

## 3. Structure obligatoire d'un plugin

```
crates/mge-plugin-{name}/
  src/
    mod.rs           # Root plugin, trait Plugin impl, re-exports
    components.rs    # Structs Component, donnees pures
    systems.rs       # Fonctions systeme, 1 fn = 1 effet
    events.rs        # Structs Event, pas de logique
    helpers.rs       # Optionnel — fonctions pures et parametrables (cf. section 10)
  index.md           # Resume compresse du plugin (max 80 lignes)
  Cargo.toml
```

Aucun autre fichier metier autorise sans justification documentee.

---

## 4. Norme MSCM obligatoire

Chaque bloc public doit etre annote. Les annotations utilisent le format `//!` (module-level) ou `///` (item-level).

### 4.1 Gouvernance des IDs (v1.1)

Le format d'@id inclut obligatoirement la version du plugin :

```
mge.plugin.{name}.v{N}.{kind}.{item}
```

Exemples :

```
mge.plugin.physics.v1.fn.apply_velocity
mge.plugin.physics.v1.component.position2d
mge.plugin.physics.v1.event.collision
```

Justification :

- Permet le versioning non destructif (@id stable entre patchs)
- Permet la coexistence v1/v2 pendant une migration
- Permet la migration douce (v1 deprecie, v2 actif)
- Le segment `v{N}` est incremente uniquement lors d'un changement breaking de l'API du plugin

Regles :

- `v{N}` est obligatoire dans les @id de plugins Phase 2+
- `v{N}` n'est PAS retroactif pour mge-core (qui utilise `mge.core.*` sans version)
- Le N commence a 1 et ne peut qu'augmenter

### 4.2 Plugin root (mod.rs)

```rust
//! @id mge.plugin.physics.v1
//! @role plugin
//! @layer plugin
//! @domain physics
//! @do provide_2d_aabb_collision
```

Le champ `@domain` est obligatoire sur le root plugin. Il permet la projection automatique vers `domains.json` lors de la generation MIP.

### 4.3 Composant

```rust
//! @id mge.plugin.physics.v1.component.position2d
//! @role data
//! @layer plugin
//! @do store_entity_2d_position
//! @fields x:f32,y:f32

pub struct Position2D {
    pub x: f32,
    pub y: f32,
}
```

Regles composant :

- 1 struct = 1 @id
- @fields obligatoire (liste des champs avec types)
- Pas de methodes complexes dans impl
- Pas de logique dans impl (donnees pures)

### 4.4 Systeme

```rust
//! @id mge.plugin.physics.v1.fn.apply_velocity
//! @role system
//! @layer plugin
//! @do update_position_from_velocity
//! @requires Position2D,Velocity2D
//! @writes Position2D
//! @emits none
//! @phase 100
//! @complexity O(n)

pub fn apply_velocity(world: &mut World, ctx: &mut Context) {
    let dt = ctx.delta_secs();
    world.for_each_mut::<Position2D, Velocity2D, _>(|_, pos, vel| {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    });
}
```

Regles systeme :

- 1 fn = 1 effet
- @requires obligatoire (composants lus)
- @writes obligatoire (composants modifies)
- @emits obligatoire (evenements emis, ou `none`)
- @phase obligatoire (PhaseId numerique, cf. Scheduler)
- @complexity obligatoire (O(n), O(n^2), etc.)

Le champ `@phase` permet a un LLM de comprendre l'ordre d'execution sans lire mod.rs. La valeur correspond au PhaseId(u32) du Scheduler.

### 4.5 Champs MSCM autorises

| Champ | Obligatoire | S'applique a | Description |
|-------|-------------|--------------|-------------|
| @id | Oui | tout | Identifiant global unique, versionne (v1.1) |
| @role | Oui | tout | plugin, data, system, event, helper |
| @layer | Oui | tout | plugin |
| @domain | plugin root | root uniquement | Domaine fonctionnel (physics, ai, render...) |
| @do | Oui | tout | Description fonctionnelle verbale |
| @requires | system/helper | systemes, helpers | Composants requis en lecture |
| @writes | system/helper | systemes, helpers | Composants/monde modifies |
| @emits | system | systemes | Evenements emis (ou `none`) |
| @phase | system | systemes | PhaseId numerique d'execution |
| @fields | data/event | composants, events | Structure interne (champs:types) |
| @complexity | system | systemes | Complexite algorithmique |

Aucun autre champ non valide par cette norme.

---

## 5. Norme fonctionnelle

### 5.1 Une fonction = un effet

Interdit :

```rust
fn physics_update(world: &mut World, ctx: &mut Context) {
    apply_velocity(world, ctx);
    resolve_collision(world, ctx);
}
```

Autorise :

```rust
fn apply_velocity(world: &mut World, ctx: &mut Context) { /* ... */ }
fn resolve_collision(world: &mut World, ctx: &mut Context) { /* ... */ }
```

Chaque systeme est enregistre separement dans le Scheduler.

### 5.2 Pas d'appel interne cache

Interdit :

```rust
fn resolve(world: &mut World) {
    helper_internal(world);
}
```

Autorise : tout doit etre visible dans le corps principal de la fonction. Les helpers sont des fonctions publiques separees, pas des appels caches.

### 5.3 Pas de logique conditionnelle complexe

Interdit :

```rust
if cond1 { /* A */ } else if cond2 { /* B */ } else { /* C */ }
```

Preferer le decoupage en fonctions distinctes avec un effet clair chacune.

---

## 6. Norme de compression token

### 6.1 Pas de commentaires narratifs

Interdit :

```rust
// Cette fonction met a jour la position en fonction de la velocite
```

Les annotations MSCM portent la semantique. Pas de commentaires redondants.

### 6.2 Noms courts mais explicites

| Autorise | Interdit |
|----------|----------|
| pos | p |
| vel | v |
| col | c |
| dt | d |

Les abreviations courantes du domaine sont autorisees. Les noms a une lettre sont interdits.

### 6.3 Pas de code decoratif

Interdit dans le hot path :

- `println!` / `dbg!`
- logs non encapsules derriere un feature flag
- allocations non necessaires
- debug non conditionnel

---

## 7. Norme Event

```rust
//! @id mge.plugin.physics.v1.event.collision
//! @role event
//! @layer plugin
//! @do notify_entity_collision
//! @fields a:EntityId,b:EntityId

pub struct CollisionEvent {
    pub a: EntityId,
    pub b: EntityId,
}
```

Regles :

- 1 struct = 1 event
- Pas de logique (pas d'impl avec methodes)
- Pas de methode
- @fields obligatoire
- Donnees pures, Send + Sync + 'static

---

## 8. Norme MIP — Optimisation token

### 8.1 Format blocks.json ultra-compresse (v1.1)

Pour les plugins Phase 2+, le generateur MIP produit un format ultra-compresse :

```json
{
  "i": "mge.plugin.physics.v1.fn.apply_velocity",
  "k": "s",
  "d": "physics",
  "r": ["Position2D", "Velocity2D"],
  "w": ["Position2D"],
  "e": [],
  "p": 100,
  "c": "O(n)"
}
```

| Abbreviation | Signification | Valeurs |
|--------------|---------------|---------|
| i | id | @id complet |
| k | kind | s=system, d=data, e=event, p=plugin, h=helper |
| d | domain | @domain du plugin parent |
| r | requires | composants lus |
| w | writes | composants modifies |
| e | emits | evenements |
| p | phase | PhaseId numerique |
| c | complexity | O(n), O(n^2), etc. |

### 8.2 Format domains.json (v1.1)

Le generateur MIP produit automatiquement un `domains.json` a partir des @domain :

```json
[
  {
    "domain": "physics",
    "plugin": "mge.plugin.physics.v1",
    "components": ["Position2D", "Velocity2D", "ColliderAABB"],
    "systems": ["apply_velocity", "resolve_collision"],
    "events": ["CollisionEvent"]
  }
]
```

### 8.3 Pas de texte long dans MIP

Pas de phrases. Pas de descriptions narratives. L'index MIP est une structure machine, pas de la documentation humaine.

---

## 9. Norme index.md

Chaque plugin doit avoir un `index.md` a la racine du crate. Format compresse, max 80 lignes :

```
Plugin: mge-plugin-physics
Version: v1
Domain: physics

Components:
- Position2D
- Velocity2D
- ColliderAABB

Systems:
- apply_velocity (phase 100)
- resolve_collision (phase 200)

Events:
- CollisionEvent

Helpers:
- spawn_movable_entity

Hot path: yes
Headless safe: yes
AI-Native Score: 10/10
```

---

## 10. Norme Helper

Les helpers sont des fonctions pures et parametrables. Ils permettent aux jeux d'etre ecrits par parametrage.

```rust
//! @id mge.plugin.physics.v1.fn.spawn_movable_entity
//! @role helper
//! @layer plugin
//! @do spawn_entity_with_position_and_velocity
//! @requires Position2D,Velocity2D
//! @writes World

pub fn spawn_movable_entity(
    world: &mut World,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
) -> EntityId {
    let id = world.spawn();
    world.insert(id, Position2D { x, y });
    world.insert(id, Velocity2D { x: vx, y: vy });
    id
}
```

### 10.1 Bornage strict des helpers (v1.1)

Les helpers sont une zone dangereuse. Sans encadrement ils deviennent des DSL deguises, des orchestrateurs caches ou des mini-moteurs dans le plugin.

Regles strictes :

| Regle | Justification |
|-------|---------------|
| Un helper ne peut PAS appeler un systeme | Pas d'orchestration cachee |
| Un helper ne peut PAS emettre d'event | L'emission est reservee aux systemes |
| Un helper ne peut PAS iterer avec iter2/iter3/for_each_mut | Pas de logique de boucle ECS — c'est le role d'un systeme |
| Un helper ne peut PAS contenir de logique conditionnelle complexe | Pas de branches if/else/match imbriquees |
| Un helper doit etre < 20 lignes | Parametrage, pas logique |

Un helper fait du **parametrage** (spawn, configure, setup). Pas du **traitement** (iterate, transform, emit).

---

## 11. Regles strictes pour Composer

Composer (et tout LLM generant du code MGE) DOIT :

| Regle | Description |
|-------|-------------|
| Toujours ajouter @requires et @writes | Chaque systeme declare ses dependances |
| Toujours ajouter @phase | Chaque systeme declare sa phase d'execution |
| Toujours ajouter @domain sur le root | Chaque plugin declare son domaine |
| Toujours versionner les @id | Format `mge.plugin.{name}.v{N}.*` |
| Ne jamais creer de fonction > 40 lignes | Decouper en fonctions atomiques |
| Ne jamais imbriquer 2 responsabilites | 1 fn = 1 effet |
| Ne jamais creer de DSL | Pas de macros generant des API implicites |
| Ne jamais creer de macro complexe | Les macros simples (derive) sont autorisees |
| Ne jamais introduire dynamic dispatch | Pas de `dyn` dans le hot path |
| Ne jamais casser la norme MSCM | Chaque bloc public est annote |
| Ne jamais introduire de static mut | Cf. section 14 |
| Respecter la structure plugin | mod.rs, components.rs, systems.rs, events.rs |
| Respecter le bornage helper | Cf. section 10.1 |

---

## 12. Verification automatique

Le generateur MIP doit refuser ou signaler :

| Violation | Consequence |
|-----------|-------------|
| @id duplique | Erreur bloquante |
| @id sans version (v{N}) sur plugin Phase 2+ | Erreur bloquante |
| Fichier plugin sans root @id | Erreur bloquante |
| Systeme sans @requires | Warning |
| Systeme sans @phase | Warning |
| Systeme sans @complexity | Warning |
| Composant sans @fields | Warning |
| Event sans @fields | Warning |
| Plugin root sans @domain | Warning |
| Fichier > 300 lignes | Warning |

---

## 13. Interdictions evolutives (v1.1)

Cette section protege MGE contre la derive architecturale a long terme. Ces interdictions sont permanentes et ne peuvent etre levees que par une revision de cette norme.

| Interdiction | Justification |
|--------------|---------------|
| Macro generative complexe | Cree du code invisible, casse l'indexation MIP |
| Trait generique lourd (>2 type params) | Explose la complexite pour humains et LLM |
| Derive custom generateur de code | Meme danger que les macros generatives |
| Type erased (`Box<dyn Any>` comme API publique) | Detruit la tracabilite statique |
| Dynamic dispatch dans le hot path | Performance impredictible, inlined impossible |
| `unsafe` dans les plugins | Reserve au kernel (mge-core) si necessaire |
| Async dans les systemes | Le Scheduler est synchrone et deterministe |
| Dependances transitives lourdes (>100 crates) | Temps de compile, surface d'attaque |
| Re-export sauvage (`pub use *`) | Pollution du namespace, ambiguite |
| Global mutable state | Cf. section 14 |

Un plugin qui viole ces interdictions sera refuse par review et non indexe par MIP.

---

## 14. Norme No Hidden State (v1.1)

Chaque systeme doit etre :

- **Stateless** : aucune variable persistante entre les ticks
- **Ou state explicite** : via des composants enregistres dans le World

### Interdit

```rust
static mut CACHE: Vec<EntityId> = Vec::new();

lazy_static! {
    static ref STATE: Mutex<HashMap<EntityId, f32>> = Mutex::new(HashMap::new());
}

thread_local! {
    static BUFFER: RefCell<Vec<f32>> = RefCell::new(Vec::new());
}
```

### Autorise

```rust
pub struct PhysicsCache {
    pub broad_phase_pairs: Vec<(EntityId, EntityId)>,
}
impl Component for PhysicsCache {}
```

L'etat est un composant, stocke dans le World, visible, serialisable, inspectable.

Justification :

- Les agents IA peuvent inspecter l'etat via les composants
- Le determinisme est garanti (pas de fuite inter-tick)
- La serialisation save/load fonctionne
- Le profiling et le debug sont possibles

---

## 15. Norme One Responsibility per File (v1.1)

| Regle | Seuil |
|-------|-------|
| Fonction max | 30 lignes (ideal), 40 lignes (absolu) |
| Fichier max | 300 lignes |
| Module max (dossier) | 5 fichiers metier |

Si `systems.rs` depasse 300 lignes, il doit etre decoupe en fichiers thematiques :

```
src/
  systems/
    mod.rs              # re-exports uniquement
    velocity.rs         # systemes lies a la velocite
    collision.rs        # systemes lies aux collisions
```

Chaque sous-fichier respecte les memes normes MSCM.

Justification :

- Un LLM ne doit jamais charger 800 lignes pour comprendre un fichier
- Le cout token est proportionnel a la taille du fichier charge
- La navigation par blocks.json est plus fine

---

## 16. AI-Native Score (v1.1)

Chaque plugin doit pouvoir etre evalue selon ces criteres. Le score est indique dans `index.md`.

| Critere | Poids | Seuil acceptable | Seuil ideal |
|---------|-------|-------------------|-------------|
| MSCM coverage | 25% | 90% des blocs publics | 100% |
| Max fn length | 15% | < 40 lignes | < 30 lignes |
| No dyn in hot path | 15% | 0 occurrence | 0 occurrence |
| No hidden state | 15% | 0 static mut / lazy_static | 0 |
| blocks.json completeness | 15% | Tous les blocs indexes | Tous + @phase + @complexity |
| Max file length | 10% | < 400 lignes | < 300 lignes |
| Helper compliance | 5% | Pas d'appel systeme | Conforme section 10.1 |

### Calcul du score

```
score = somme(critere_ok * poids) / somme(poids) * 10
```

| Score | Evaluation |
|-------|------------|
| 10/10 | Exemplaire — reference AI-Native |
| 8-9/10 | Conforme — pret pour production |
| 6-7/10 | Acceptable — ameliorations requises |
| < 6/10 | Non conforme — revision obligatoire avant merge |

Le score minimum pour un merge en Phase 2+ est **8/10**.

---

## 17. Design philosophy

MGE AI-Native v1.1 n'est pas un moteur classique. C'est :

| Concept | Description |
|---------|-------------|
| Graphe de fonctions atomiques | Chaque systeme est un noeud independant |
| Carte semantique | MSCM annote chaque noeud avec sa semantique |
| API machine | blocks.json est l'interface de navigation pour agents IA |
| Carte de domaines | domains.json projette les plugins par domaine fonctionnel |
| Base pour agents autonomes | Un LLM peut assembler un jeu par composition de systemes |

Consequences strategiques :

- Un LLM peut coder 70%+ d'un jeu par assemblage de systemes existants
- Le moteur devient un DSL fonctionnel implicite
- Le cout token est minimal grace a la compression MSCM/MIP
- Le code est stable 10+ ans grace aux interdictions evolutives
- L'audit MWS (Phase B, blocs MIP) est facilite
- Le versioning des @id permet la migration douce

---

## Changelog

| Version | Date | Modifications |
|---------|------|---------------|
| v1.0 | 2026-02-19 | Norme initiale — 13 sections |
| v1.1 | 2026-02-19 | +@domain, +@phase, +version @id, +interdictions evolutives, +bornage helpers, +no hidden state, +one responsibility per file, +AI-Native Score, +MIP ultra-compresse (k a 1 lettre), +domains.json |

---

## References

| Document | Role |
|----------|------|
| [MGE - Kernel Specification](./MGE%20-%20Kernel%20Specification.md) | Specification du microkernel |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin, enregistrement |
| [MGE - MSCM MIP Governance](../MGE%20-%20MSCM%20MIP%20Governance.md) | Politique balisage et index |
| [MGE - Roadmap](../MGE%20-%20Roadmap.md) | Phases de developpement |

---

**Document :** MGE — AI-Native Writing Standard v1.1
**Version :** 1.1
**Date :** 2026-02-19
**Statut :** Norme obligatoire
