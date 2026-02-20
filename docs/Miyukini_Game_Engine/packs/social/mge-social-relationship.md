# mge-social-relationship

> @id mge.social.relationship.v1  
> @role plugin  
> @domain social  
> @do manage_relationships_affinity_interaction_history  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-relationship` |
| @id MSCM | `mge.social.relationship.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (decay et affinite mis a jour chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(r) ou r=nombre de relations actives |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RelationType` | `Friend, Rival, Neutral, Romantic, Family, Professional` | Nature de la relation |
| `InteractionKind` | `Talk, Gift, Insult, Help, Trade, Fight` | Type d'interaction sociale |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Relationship` | `mge.social.relationship.v1.component.relationship` | `source: EntityId, target: EntityId, relation_type: RelationType, affinity: f32, trust: f32` | Lien oriente entre deux entites. affinity [-100, 100], trust [0, 100] |
| `RelationshipHistory` | `mge.social.relationship.v1.component.relationship_history` | `source: EntityId, target: EntityId, interactions: Vec<InteractionRecord>, max_records: u16` | Historique borne des interactions entre deux entites |
| `SocialLink` | `mge.social.relationship.v1.component.social_link` | `entity: EntityId, relationships: Vec<EntityId>` | Index des relations d'une entite pour acces rapide |

---

## 4. Formules

```
affinity_delta   = base_impact * personality_modifier * context_bonus
affinity_new     = clamp(affinity + affinity_delta, -100.0, 100.0)

trust_delta      = interaction_trust_value * familiarity_bonus
trust_new        = clamp(trust + trust_delta, 0.0, 100.0)

decay_per_tick   = decay_rate * (1.0 - familiarity_factor)
affinity_decayed = affinity * (1.0 - decay_per_tick)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_affinity` | `mge.social.relationship.v1.fn.update_affinity` | Logic (1000) | Relationship | Relationship | AffinityChanged | O(r) | Applique les deltas d'affinite accumules ce tick |
| `decay_relationships` | `mge.social.relationship.v1.fn.decay_relationships` | Logic (1001) | Relationship | Relationship | none | O(r) | Applique le declin naturel des relations non entretenues |
| `process_interaction` | `mge.social.relationship.v1.fn.process_interaction` | Logic (1002) | Relationship, RelationshipHistory | Relationship, RelationshipHistory | RelationshipFormed | O(i) | Traite les interactions en attente. Cree la relation si inexistante |
| `check_relationship_threshold` | `mge.social.relationship.v1.fn.check_relationship_threshold` | Logic (1003) | Relationship | Relationship | RelationshipBroken | O(r) | Detecte les relations passant sous le seuil de rupture |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `RelationshipFormed` | `mge.social.relationship.v1.event.relationship_formed` | `source: EntityId, target: EntityId, relation_type: RelationType` | `process_interaction` | faction, gossip, ui |
| `RelationshipBroken` | `mge.social.relationship.v1.event.relationship_broken` | `source: EntityId, target: EntityId, reason: String` | `check_relationship_threshold` | faction, gossip, ai |
| `AffinityChanged` | `mge.social.relationship.v1.event.affinity_changed` | `source: EntityId, target: EntityId, old_value: f32, new_value: f32` | `update_affinity` | gossip, ui |

---

## 7. Invariants

- `Relationship.affinity` est toujours dans [-100.0, 100.0] apres `update_affinity`.
- `Relationship.trust` est toujours dans [0.0, 100.0].
- `RelationshipHistory.interactions.len()` ne depasse jamais `max_records`.
- Une relation est toujours orientee : (A→B) et (B→A) sont deux `Relationship` distinctes.
- `SocialLink.relationships` est toujours synchronise avec les `Relationship` existantes.
- `RelationshipFormed` n'est emis qu'une seule fois par paire et direction.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `decay_rate` | `f32` | 0.01 | [0.0, 0.1] | Taux de declin d'affinite par tick |
| `break_threshold` | `f32` | -80.0 | [-100.0, -10.0] | Seuil d'affinite declenchant RelationshipBroken |
| `max_history_records` | `u16` | 50 | [10, 500] | Nombre max d'interactions memorisees par paire |
| `initial_affinity` | `f32` | 0.0 | [-50.0, 50.0] | Affinite initiale a la creation d'une relation |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les relations entre entites | Ne gere pas les factions (-> faction) |
| Stocke et decroit l'affinite | Ne gere pas la reputation globale (-> reputation) |
| Enregistre l'historique d'interactions | Ne gere pas la propagation (-> gossip) |
| Detecte les ruptures de relation | Ne gere pas les besoins sociaux (-> need) |
| Supporte 6 types de relation | Ne gere pas la personnalite (-> personality) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Relationship, RelationshipHistory |
| Ecrit | Relationship, RelationshipHistory, SocialLink |
| Emet | RelationshipFormed, RelationshipBroken, AffinityChanged |
| Ne touche jamais | Faction, Reputation, NeedSet, Schedule, PersonalityTraits, GossipMemory, Household |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-relationship/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.social.relationship.v1, trait Plugin impl
    ├── components.rs     # Relationship, RelationshipHistory, SocialLink
    ├── systems.rs        # update_affinity, decay_relationships, process_interaction, check_relationship_threshold
    └── events.rs         # RelationshipFormed, RelationshipBroken, AffinityChanged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire (update_affinity, decay_relationships) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (RelationType, InteractionKind)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : affinity update, decay, interaction, threshold break
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.relationship.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.component.relationship","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.component.relationship_history","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.component.social_link","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.fn.update_affinity","k":"s","d":"social","r":["Relationship"],"w":["Relationship"],"e":["AffinityChanged"],"p":1000,"c":"O(r)"},
  {"i":"mge.social.relationship.v1.fn.decay_relationships","k":"s","d":"social","r":["Relationship"],"w":["Relationship"],"e":[],"p":1001,"c":"O(r)"},
  {"i":"mge.social.relationship.v1.fn.process_interaction","k":"s","d":"social","r":["Relationship","RelationshipHistory"],"w":["Relationship","RelationshipHistory"],"e":["RelationshipFormed"],"p":1002,"c":"O(i)"},
  {"i":"mge.social.relationship.v1.fn.check_relationship_threshold","k":"s","d":"social","r":["Relationship"],"w":["Relationship"],"e":["RelationshipBroken"],"p":1003,"c":"O(r)"},
  {"i":"mge.social.relationship.v1.event.relationship_formed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.event.relationship_broken","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.relationship.v1.event.affinity_changed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let alice = world.spawn();
let bob = world.spawn();

let rel = world.spawn();
world.insert(rel, Relationship {
    source: alice,
    target: bob,
    relation_type: RelationType::Friend,
    affinity: 25.0,
    trust: 50.0,
});
world.insert(rel, RelationshipHistory {
    source: alice,
    target: bob,
    interactions: vec![],
    max_records: 50,
});

world.insert(alice, SocialLink { entity: alice, relationships: vec![rel] });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
