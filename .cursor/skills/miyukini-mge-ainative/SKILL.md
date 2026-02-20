---
name: miyukini-mge-ainative
description: Norme AI-Native Writing Standard v1.1 pour MGE. Structure obligatoire des plugins Phase 2+, IDs versionnes (v{N}), @domain/@phase obligatoires, regles MSCM pour plugins (composants, systemes, events, helpers bornes), regle des 30 lignes, 300 lignes max par fichier, compression token, format blocks.json ultra-compresse (i/k/d/r/w/e/p/c), domains.json, interdictions evolutives, no hidden state, AI-Native Score, regles Composer/LLM. Utiliser quand on cree ou modifie un plugin MGE, quand on ecrit des systemes/composants/events, quand on genere du code MGE via LLM, ou quand on verifie la conformite AI-Native.
---

# MGE — AI-Native Writing Standard v1.1

Norme obligatoire pour tous les plugins Phase 2+ et extensions MGE.
Document normatif complet : `docs/Miyukini_Game_Engine/mge-kernel/MGE - AI-Native Writing Standard v1.md`

## Principes fondamentaux

- Le code est interroge par des agents IA, pas seulement execute.
- Un LLM ne doit jamais lire plus de 30 lignes pour comprendre une fonctionnalite.
- Separation stricte : Code = Execution, MSCM = Semantique, MIP = Index, blocks.json = Interface machine, domains.json = Carte des domaines.

## Structure obligatoire d'un plugin

```
crates/mge-plugin-{name}/
  src/
    mod.rs           # Root plugin + @id + @role plugin + @domain
    components.rs    # Structs Component (donnees pures)
    systems.rs       # 1 fn = 1 effet (max 300 lignes, sinon decouper)
    events.rs        # Structs Event (pas de logique)
    helpers.rs       # Optionnel — fonctions pures parametrables (cf. bornage)
  index.md           # Resume compresse (max 80 lignes) + AI-Native Score
  Cargo.toml
```

Aucun autre fichier metier sans justification. Si `systems.rs` > 300 lignes, decouper en `systems/mod.rs` + sous-fichiers.

## Gouvernance des IDs (v1.1)

Format obligatoire :

```
mge.plugin.{name}.v{N}.{kind}.{item}
```

- `v{N}` obligatoire sur les @id de plugins Phase 2+
- N commence a 1, ne peut qu'augmenter
- Permet coexistence v1/v2 et migration douce
- Non retroactif pour mge-core (qui utilise `mge.core.*`)

## MSCM obligatoire par type de bloc

### Plugin root (mod.rs)

```rust
//! @id mge.plugin.{name}.v1
//! @role plugin
//! @layer plugin
//! @domain {domain}
//! @do {description_fonctionnelle}
```

`@domain` est obligatoire. Projete vers `domains.json` par MIP.

### Composant

```rust
//! @id mge.plugin.{name}.v1.component.{comp}
//! @role data
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}

pub struct MyComp { pub x: f32, pub y: f32 }
```

Regles : 1 struct = 1 @id, @fields obligatoire, pas de logique dans impl.

### Systeme

```rust
//! @id mge.plugin.{name}.v1.fn.{system}
//! @role system
//! @layer plugin
//! @do {description}
//! @requires {Comp1},{Comp2}
//! @writes {Comp1}
//! @emits none
//! @phase {PhaseId}
//! @complexity O(n)

pub fn my_system(world: &mut World, ctx: &mut Context) { /* ... */ }
```

Regles : 1 fn = 1 effet, @requires/@writes/@emits/@phase/@complexity obligatoires.
`@phase` = PhaseId(u32) du Scheduler, permet a un LLM de comprendre l'ordre sans lire mod.rs.

### Event

```rust
//! @id mge.plugin.{name}.v1.event.{event}
//! @role event
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}

pub struct MyEvent { pub a: EntityId, pub b: EntityId }
```

Regles : 1 struct = 1 event, pas de logique, @fields obligatoire.

### Helper

```rust
//! @id mge.plugin.{name}.v1.fn.{helper}
//! @role helper
//! @layer plugin
//! @do {description}
//! @requires {Comp1},{Comp2}
//! @writes World

pub fn my_helper(world: &mut World, x: f32, y: f32) -> EntityId { /* ... */ }
```

## Bornage strict des helpers (v1.1)

| Interdit | Justification |
|----------|---------------|
| Appeler un systeme | Pas d'orchestration cachee |
| Emettre un event | L'emission est reservee aux systemes |
| Iterer avec iter2/iter3/for_each_mut | Le traitement ECS est le role des systemes |
| Logique conditionnelle complexe | Pas de branches imbriquees |
| Plus de 20 lignes | Parametrage, pas logique |

Un helper fait du **parametrage** (spawn, configure, setup). Pas du **traitement**.

## Champs MSCM autorises

| Champ | Obligatoire | S'applique a |
|-------|-------------|--------------|
| @id | Oui | tout (versionne v{N} pour Phase 2+) |
| @role | Oui | tout (plugin/data/system/event/helper) |
| @layer | Oui | tout |
| @domain | plugin root | root uniquement |
| @do | Oui | tout |
| @requires | system/helper | composants lus |
| @writes | system/helper | composants/monde modifies |
| @emits | system | evenements emis (ou `none`) |
| @phase | system | PhaseId numerique |
| @fields | data/event | structure interne |
| @complexity | system | O(n), O(n^2), etc. |

Aucun autre champ non valide.

## Regles fonctionnelles

1. **1 fn = 1 effet** — pas de fonctions orchestrantes.
2. **Pas d'appel interne cache** — tout visible dans le corps principal.
3. **Pas de logique conditionnelle complexe** — decoupage en fonctions distinctes.
4. **Max 30 lignes par fonction** (ideal), 40 lignes (absolu).
5. **Max 300 lignes par fichier** — au-dela, decoupage obligatoire.

## Compression token

- Pas de commentaires narratifs (MSCM seul porte la semantique).
- Noms courts explicites : `pos`, `vel`, `col`, `dt` (pas de noms a 1 lettre).
- Pas de `println!`/`dbg!` dans le hot path.

## No Hidden State (v1.1)

Chaque systeme doit etre stateless ou utiliser de l'etat explicite via composants.

Interdit : `static mut`, `lazy_static!`, `thread_local!`.
Autorise : composant dedie dans le World (visible, serialisable, inspectable).

## Format blocks.json ultra-compresse (MIP v1.1)

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

i=id, k=kind (s/d/e/p/h), d=domain, r=requires, w=writes, e=emits, p=phase, c=complexity.

## domains.json (v1.1)

Genere automatiquement par MIP a partir des @domain :

```json
[{"domain":"physics","plugin":"mge.plugin.physics.v1","components":["Position2D"],"systems":["apply_velocity"],"events":["CollisionEvent"]}]
```

## index.md (max 80 lignes)

```
Plugin: mge-plugin-{name}
Version: v1
Domain: {domain}

Components:
- {Comp1}
- {Comp2}

Systems:
- {system1} (phase {N})
- {system2} (phase {N})

Events:
- {Event1}

Helpers:
- {helper1}

Hot path: yes/no
Headless safe: yes/no
AI-Native Score: X/10
```

## Interdictions evolutives (v1.1)

Permanentes, ne peuvent etre levees que par revision de la norme :

| Interdit |
|----------|
| Macro generative complexe |
| Trait generique lourd (>2 type params) |
| Derive custom generateur de code |
| Type erased (`Box<dyn Any>` comme API publique) |
| Dynamic dispatch dans le hot path |
| `unsafe` dans les plugins |
| Async dans les systemes |
| Dependances transitives lourdes (>100 crates) |
| Re-export sauvage (`pub use *`) |
| Global mutable state (static mut, lazy_static) |

## AI-Native Score (v1.1)

| Critere | Poids |
|---------|-------|
| MSCM coverage 100% | 25% |
| Max fn length < 30 | 15% |
| No dyn in hot path | 15% |
| No hidden state | 15% |
| blocks.json completeness | 15% |
| Max file length < 300 | 10% |
| Helper compliance | 5% |

Score minimum pour merge Phase 2+ : **8/10**.

## Regles strictes pour Composer / LLM

| Regle |
|-------|
| Toujours @requires, @writes, @phase sur les systemes |
| Toujours @domain sur le root plugin |
| Toujours versionner les @id (v{N}) |
| Ne jamais creer de fonction > 40 lignes |
| Ne jamais imbriquer 2 responsabilites |
| Ne jamais creer de DSL ou macro complexe |
| Ne jamais introduire dynamic dispatch hot path |
| Ne jamais introduire static mut / lazy_static |
| Ne jamais casser la norme MSCM |
| Respecter la structure plugin obligatoire |
| Respecter le bornage strict des helpers |

## Verification automatique MIP

Le generateur MIP doit refuser/signaler :

| Violation | Consequence |
|-----------|-------------|
| @id duplique | Erreur bloquante |
| @id sans version (v{N}) Phase 2+ | Erreur bloquante |
| Plugin sans root @id | Erreur bloquante |
| Systeme sans @requires | Warning |
| Systeme sans @phase | Warning |
| Systeme sans @complexity | Warning |
| Composant/event sans @fields | Warning |
| Plugin root sans @domain | Warning |
| Fichier > 300 lignes | Warning |
