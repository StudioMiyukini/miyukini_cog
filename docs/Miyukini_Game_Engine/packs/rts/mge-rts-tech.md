# mge-rts-tech

> @id mge.rts.tech.v1  
> @role plugin  
> @domain rts  
> @do manage_tech_tree_research_prerequisites_unlocks  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-tech` |
| @id MSCM | `mge.rts.tech.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rts-resource` |
| Hot path | Non (recherche avance par tick mais peu de recherches simultanees) |
| Headless safe | Oui |
| Complexite globale | O(q) ou q=recherches en cours |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ResearchState` | `Locked, Available, InProgress, Completed` | Etat d'un noeud technologique |
| `TechCategory` | `Military, Economic, Infrastructure, Special` | Categorie de technologie |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TechNode` | `mge.rts.tech.v1.component.tech_node` | `tech_id: u32, category: TechCategory, state: ResearchState, progress: f32, duration: f32, cost: ResourceCost` | Noeud technologique individuel |
| `TechTree` | `mge.rts.tech.v1.component.tech_tree` | `nodes: Vec<EntityId>, edges: Vec<(EntityId, EntityId)>` | Arbre technologique complet. edges = (prerequis, dependant) |
| `TechPrerequisite` | `mge.rts.tech.v1.component.tech_prerequisite` | `required_techs: Vec<u32>, required_buildings: Vec<u32>` | Prerequis pour debloquer un noeud tech |
| `ResearchQueue` | `mge.rts.tech.v1.component.research_queue` | `current: Option<EntityId>, queue: Vec<EntityId>, max_parallel: u8` | File de recherche. max_parallel = recherches simultanees |

---

## 4. Formules

```
research_delta  = dt * research_speed_modifier
progress_new    = min(progress + research_delta, duration)
complete        = progress_new >= duration
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_research` | `mge.rts.tech.v1.fn.tick_research` | Logic (1170) | ResearchQueue, TechNode | TechNode | none | O(q) | Avance le progress des recherches en cours |
| `start_research` | `mge.rts.tech.v1.fn.start_research` | Logic (1171) | ResearchQueue, TechNode, TechPrerequisite, ResourceDepot | ResearchQueue, TechNode, ResourceDepot | ResearchStarted, ResearchCancelled | O(p) | Demarre une recherche si prerequis remplis et ressources suffisantes |
| `complete_research` | `mge.rts.tech.v1.fn.complete_research` | Logic (1172) | ResearchQueue, TechNode | ResearchQueue, TechNode | ResearchCompleted | O(q) | Finalise les recherches terminees, passe le state a Completed |
| `unlock_tech_nodes` | `mge.rts.tech.v1.fn.unlock_tech_nodes` | Logic (1173) | TechTree, TechNode, TechPrerequisite | TechNode | TechUnlocked | O(n * p) | Verifie les prerequis et debloque les noeuds disponibles |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ResearchStarted` | `mge.rts.tech.v1.event.research_started` | `tech_id: u32, category: TechCategory, duration: f32` | `start_research` | ui, audio |
| `ResearchCompleted` | `mge.rts.tech.v1.event.research_completed` | `tech_id: u32, category: TechCategory` | `complete_research` | production, building, ui, audio |
| `TechUnlocked` | `mge.rts.tech.v1.event.tech_unlocked` | `tech_id: u32, unlocked_by: Vec<u32>` | `unlock_tech_nodes` | ui |
| `ResearchCancelled` | `mge.rts.tech.v1.event.research_cancelled` | `tech_id: u32, refund: ResourceCost` | `start_research` | ui, resource |

---

## 7. Invariants

- `TechNode.progress` est toujours dans [0.0, duration].
- Un noeud `Locked` ne peut pas etre recherche sans satisfaire ses prerequis.
- Un noeud `Completed` ne peut pas etre recherche a nouveau.
- `TechTree.edges` forme un DAG (graphe dirige acyclique) — pas de cycles.
- Les ressources sont deduites au demarrage de la recherche.
- `ResearchQueue.queue.len()` ne depasse jamais une limite raisonnable (configurable).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `research_speed_modifier` | `f32` | 1.0 | [0.1, 5.0] | Multiplicateur global de vitesse de recherche |
| `max_parallel_research` | `u8` | 1 | [1, 5] | Nombre max de recherches simultanees |
| `cancel_refund_pct` | `f32` | 0.5 | [0.0, 1.0] | Pourcentage rembourse a l'annulation |
| `auto_unlock_check` | `bool` | true | — | Verifie automatiquement les deblocages chaque tick |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere l'arbre technologique et les recherches | Ne gere pas les effets des technologies (→ gameplay externe) |
| Verifie les prerequis de deblocage | Ne gere pas les ressources (→ resource) |
| Supporte la recherche en queue | Ne gere pas l'UI de l'arbre tech (→ ui) |
| Annule et rembourse les recherches | Ne gere pas les bonus de production lies a la tech (→ production) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ResearchQueue, TechNode, TechPrerequisite, TechTree, ResourceDepot |
| Ecrit | ResearchQueue, TechNode, ResourceDepot |
| Emet | ResearchStarted, ResearchCompleted, TechUnlocked, ResearchCancelled |
| Ne touche jamais | Selection, ProductionQueue, Building, OrderQueue, FogGrid, MinimapEntry |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-tech/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.tech.v1, trait Plugin impl
    ├── components.rs     # TechNode, TechTree, TechPrerequisite, ResearchQueue
    ├── systems.rs        # tick_research, start_research, complete_research, unlock_tech_nodes
    └── events.rs         # ResearchStarted, ResearchCompleted, TechUnlocked, ResearchCancelled
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ResearchState, TechCategory)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : research tick, prerequisite check, completion, unlock chain
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.tech.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.component.tech_node","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.component.tech_tree","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.component.tech_prerequisite","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.component.research_queue","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.fn.tick_research","k":"s","d":"rts","r":["ResearchQueue","TechNode"],"w":["TechNode"],"e":[],"p":1170,"c":"O(q)"},
  {"i":"mge.rts.tech.v1.fn.start_research","k":"s","d":"rts","r":["ResearchQueue","TechNode","TechPrerequisite","ResourceDepot"],"w":["ResearchQueue","TechNode","ResourceDepot"],"e":["ResearchStarted","ResearchCancelled"],"p":1171,"c":"O(p)"},
  {"i":"mge.rts.tech.v1.fn.complete_research","k":"s","d":"rts","r":["ResearchQueue","TechNode"],"w":["ResearchQueue","TechNode"],"e":["ResearchCompleted"],"p":1172,"c":"O(q)"},
  {"i":"mge.rts.tech.v1.fn.unlock_tech_nodes","k":"s","d":"rts","r":["TechTree","TechNode","TechPrerequisite"],"w":["TechNode"],"e":["TechUnlocked"],"p":1173,"c":"O(n*p)"},
  {"i":"mge.rts.tech.v1.event.research_started","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.event.research_completed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.event.tech_unlocked","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.tech.v1.event.research_cancelled","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let tree = world.spawn();
let archery = world.spawn();
world.insert(archery, TechNode {
    tech_id: 1,
    category: TechCategory::Military,
    state: ResearchState::Available,
    progress: 0.0,
    duration: 45.0,
    cost: ResourceCost { amounts: HashMap::from([(ResourceKind::Gold, 200), (ResourceKind::Food, 100)]) },
});
world.insert(archery, TechPrerequisite {
    required_techs: vec![],
    required_buildings: vec![5],
});

let crossbow = world.spawn();
world.insert(crossbow, TechNode {
    tech_id: 2,
    category: TechCategory::Military,
    state: ResearchState::Locked,
    progress: 0.0,
    duration: 60.0,
    cost: ResourceCost { amounts: HashMap::from([(ResourceKind::Gold, 300)]) },
});
world.insert(crossbow, TechPrerequisite {
    required_techs: vec![1],
    required_buildings: vec![],
});

world.insert(tree, TechTree {
    nodes: vec![archery, crossbow],
    edges: vec![(archery, crossbow)],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
