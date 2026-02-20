# mge-social-gossip

> @id mge.social.gossip.v1  
> @role plugin  
> @domain social  
> @do manage_information_propagation_rumors_social_memory  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-gossip` |
| @id MSCM | `mge.social.gossip.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-social-relationship`, `mge-social-reputation` |
| Hot path | Non (propagation par vagues, pas chaque frame) |
| Headless safe | Oui |
| Complexite globale | O(g * c) ou g=gossips actifs, c=contacts par entite |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `GossipType` | `Fact, Rumor, Opinion, Secret` | Nature de l'information propagee |
| `GossipReliability` | `Confirmed, Likely, Uncertain, False` | Fiabilite percue |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `GossipMemory` | `mge.social.gossip.v1.component.gossip_memory` | `entity: EntityId, items: Vec<GossipItem>, max_items: u16` | Memoire de rumeurs d'une entite. FIFO si max depasse |
| `GossipItem` | `mge.social.gossip.v1.component.gossip_item` | `gossip_type: GossipType, subject: EntityId, content_id: u32, reliability: GossipReliability, received_tick: u64, source: EntityId` | Information memorisee avec source et fiabilite |
| `GossipSpreadState` | `mge.social.gossip.v1.component.gossip_spread_state` | `gossip_id: u32, origin: EntityId, spread_count: u32, max_spread: u32, active: bool` | Etat de propagation d'un gossip dans le reseau |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `spread_gossip` | `mge.social.gossip.v1.fn.spread_gossip` | Logic (1060) | GossipMemory, GossipSpreadState | GossipMemory, GossipSpreadState | GossipSpread | O(g*c) | Propage les gossips actifs aux contacts proches |
| `decay_gossip` | `mge.social.gossip.v1.fn.decay_gossip` | Logic (1061) | GossipMemory | GossipMemory | GossipForgotten | O(n*g) | Supprime les gossips trop anciens de la memoire |
| `memorize_event` | `mge.social.gossip.v1.fn.memorize_event` | Logic (1062) | GossipMemory | GossipMemory | none | O(e) | Convertit les evenements observes en GossipItems |
| `evaluate_gossip_impact` | `mge.social.gossip.v1.fn.evaluate_gossip_impact` | Logic (1063) | GossipMemory | GossipMemory | RumorConfirmed | O(n*g) | Evalue l'impact des gossips sur les opinions. Confirme rumeurs convergentes |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `GossipSpread` | `mge.social.gossip.v1.event.gossip_spread` | `gossip_id: u32, from: EntityId, to: EntityId, gossip_type: GossipType` | `spread_gossip` | relationship, reputation, ui |
| `GossipForgotten` | `mge.social.gossip.v1.event.gossip_forgotten` | `entity: EntityId, gossip_id: u32` | `decay_gossip` | ui |
| `RumorConfirmed` | `mge.social.gossip.v1.event.rumor_confirmed` | `gossip_id: u32, subject: EntityId, confirmed_by_count: u32` | `evaluate_gossip_impact` | reputation, relationship, ai |

---

## 7. Invariants

- `GossipMemory.items.len()` ne depasse jamais `max_items`. FIFO en cas de debordement.
- `GossipSpreadState.spread_count` ne depasse jamais `max_spread`.
- `GossipSpreadState.active = false` quand `spread_count >= max_spread`.
- La fiabilite ne peut que baisser lors de la propagation (Confirmed -> Likely -> Uncertain).
- `RumorConfirmed` n'est emis que si au moins `confirmation_threshold` sources independantes.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `max_gossip_per_entity` | `u16` | 30 | [5, 200] | Taille max de la memoire gossip |
| `gossip_decay_ticks` | `u64` | 5000 | [100, 50000] | Duree avant oubli d'un gossip |
| `max_spread_hops` | `u32` | 5 | [1, 20] | Nombre max de relais pour un gossip |
| `confirmation_threshold` | `u32` | 3 | [2, 10] | Sources independantes requises pour confirmer |
| `spread_probability` | `f32` | 0.3 | [0.05, 1.0] | Probabilite de partager un gossip a chaque contact |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Propage les informations entre entites | Ne modifie pas les relations (-> relationship) |
| Gere la memoire de rumeurs | Ne modifie pas la reputation (-> reputation) |
| Confirme les rumeurs par convergence | Ne genere pas de dialogue (-> rpg-dialogue) |
| Oublie les gossips anciens | Ne decide pas des actions (-> ai) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | GossipMemory, GossipItem, GossipSpreadState |
| Ecrit | GossipMemory, GossipSpreadState |
| Emet | GossipSpread, GossipForgotten, RumorConfirmed |
| Ne touche jamais | Relationship, Faction, Reputation, NeedSet, Schedule, PersonalityTraits, Household |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-gossip/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.gossip.v1
    ├── components.rs     # GossipMemory, GossipItem, GossipSpreadState
    ├── systems.rs        # spread_gossip, decay_gossip, memorize_event, evaluate_gossip_impact
    └── events.rs         # GossipSpread, GossipForgotten, RumorConfirmed
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs`
- [ ] 3 evenements dans `events.rs`
- [ ] 2 enumerations (GossipType, GossipReliability)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : spread, decay, memorize, confirm
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.gossip.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.component.gossip_memory","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.component.gossip_item","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.component.gossip_spread_state","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.fn.spread_gossip","k":"s","d":"social","r":["GossipMemory","GossipSpreadState"],"w":["GossipMemory","GossipSpreadState"],"e":["GossipSpread"],"p":1060,"c":"O(g*c)"},
  {"i":"mge.social.gossip.v1.fn.decay_gossip","k":"s","d":"social","r":["GossipMemory"],"w":["GossipMemory"],"e":["GossipForgotten"],"p":1061,"c":"O(n*g)"},
  {"i":"mge.social.gossip.v1.fn.memorize_event","k":"s","d":"social","r":["GossipMemory"],"w":["GossipMemory"],"e":[],"p":1062,"c":"O(e)"},
  {"i":"mge.social.gossip.v1.fn.evaluate_gossip_impact","k":"s","d":"social","r":["GossipMemory"],"w":["GossipMemory"],"e":["RumorConfirmed"],"p":1063,"c":"O(n*g)"},
  {"i":"mge.social.gossip.v1.event.gossip_spread","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.event.gossip_forgotten","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.gossip.v1.event.rumor_confirmed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let alice = world.spawn();
world.insert(alice, GossipMemory {
    entity: alice,
    items: vec![
        GossipItem {
            gossip_type: GossipType::Rumor,
            subject: bob,
            content_id: 42,
            reliability: GossipReliability::Uncertain,
            received_tick: 100,
            source: carol,
        },
    ],
    max_items: 30,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
