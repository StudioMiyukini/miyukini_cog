# mge-social-faction

> @id mge.social.faction.v1  
> @role plugin  
> @domain social  
> @do manage_factions_membership_hierarchy_ranks  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-faction` |
| @id MSCM | `mge.social.faction.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-social-reputation` |
| Hot path | Non (changements de faction peu frequents) |
| Headless safe | Oui |
| Complexite globale | O(f * m) ou f=factions, m=membres par faction |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `FactionStanding` | `Allied, Friendly, Neutral, Hostile, AtWar` | Relation entre factions |
| `RankTier` | `Leader, Officer, Member, Recruit, Outcast` | Niveau hierarchique |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Faction` | `mge.social.faction.v1.component.faction` | `name_id: u32, leader: Option<EntityId>, member_count: u32, founded_tick: u64` | Entite faction avec identifiant et statistiques |
| `FactionMember` | `mge.social.faction.v1.component.faction_member` | `entity: EntityId, faction_id: EntityId, rank: RankTier, joined_tick: u64` | Appartenance d'une entite a une faction |
| `FactionRank` | `mge.social.faction.v1.component.faction_rank` | `faction_id: EntityId, tier: RankTier, permissions: u32, title_id: u32` | Definition d'un rang dans une faction |
| `FactionRelation` | `mge.social.faction.v1.component.faction_relation` | `faction_a: EntityId, faction_b: EntityId, standing: FactionStanding, score: f32` | Relation entre deux factions |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_join_faction` | `mge.social.faction.v1.fn.process_join_faction` | Logic (1010) | Faction | Faction, FactionMember | FactionJoined | O(j) | Traite les demandes d'adhesion. Cree FactionMember |
| `update_faction_rank` | `mge.social.faction.v1.fn.update_faction_rank` | Logic (1011) | FactionMember, Faction | FactionMember | RankChanged | O(m) | Evalue et met a jour les rangs des membres |
| `process_leave_faction` | `mge.social.faction.v1.fn.process_leave_faction` | Logic (1012) | FactionMember, Faction | Faction, FactionMember | FactionLeft, FactionDissolved | O(l) | Traite les departs. Dissout si dernier membre |
| `update_faction_relations` | `mge.social.faction.v1.fn.update_faction_relations` | Logic (1013) | FactionRelation | FactionRelation | none | O(f^2) | Met a jour les standings entre factions selon les scores |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `FactionJoined` | `mge.social.faction.v1.event.faction_joined` | `entity: EntityId, faction: EntityId, rank: RankTier` | `process_join_faction` | reputation, ui, gossip |
| `FactionLeft` | `mge.social.faction.v1.event.faction_left` | `entity: EntityId, faction: EntityId, reason: String` | `process_leave_faction` | reputation, ui, gossip |
| `RankChanged` | `mge.social.faction.v1.event.rank_changed` | `entity: EntityId, faction: EntityId, old_rank: RankTier, new_rank: RankTier` | `update_faction_rank` | ui, gossip |
| `FactionDissolved` | `mge.social.faction.v1.event.faction_dissolved` | `faction: EntityId, reason: String` | `process_leave_faction` | reputation, gossip, ai |

---

## 7. Invariants

- Une entite ne peut appartenir qu'a une seule faction a la fois (`FactionMember` unique par entite).
- `Faction.member_count` est toujours egal au nombre reel de `FactionMember` referençant cette faction.
- `Faction.leader` est toujours `Some` sauf si la faction est en dissolution.
- `FactionRelation` est symetrique : (A,B) et (B,A) partagent le meme score.
- `FactionDissolved` n'est emis qu'une fois. La faction est ensuite retiree du monde.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `max_members_per_faction` | `u32` | 100 | [2, 10000] | Capacite max de membres par faction |
| `auto_promote_threshold` | `f32` | 75.0 | [10.0, 100.0] | Score reputation declenchant promotion auto |
| `dissolution_min_members` | `u32` | 1 | [0, 10] | Seuil de membres sous lequel la faction se dissout |
| `standing_war_threshold` | `f32` | -75.0 | [-100.0, 0.0] | Score relation declenchant AtWar |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les factions et leurs membres | Ne gere pas les relations individuelles (-> relationship) |
| Attribue et met a jour les rangs | Ne gere pas la reputation (-> reputation) |
| Suit les relations inter-factions | Ne gere pas la diplomatie (-> grand-strategy) |
| Dissout les factions vides | Ne gere pas le combat de faction (-> rpg-combat) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Faction, FactionMember, FactionRank, FactionRelation |
| Ecrit | Faction, FactionMember, FactionRelation |
| Emet | FactionJoined, FactionLeft, RankChanged, FactionDissolved |
| Ne touche jamais | Relationship, NeedSet, Schedule, PersonalityTraits, GossipMemory, Household |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-faction/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.faction.v1, trait Plugin impl
    ├── components.rs     # Faction, FactionMember, FactionRank, FactionRelation
    ├── systems.rs        # process_join_faction, update_faction_rank, process_leave_faction, update_faction_relations
    └── events.rs         # FactionJoined, FactionLeft, RankChanged, FactionDissolved
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
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs`
- [ ] 4 evenements dans `events.rs`
- [ ] 2 enumerations (FactionStanding, RankTier)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : join, leave, promote, dissolve, relations
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.faction.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.component.faction","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.component.faction_member","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.component.faction_rank","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.component.faction_relation","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.fn.process_join_faction","k":"s","d":"social","r":["Faction"],"w":["Faction","FactionMember"],"e":["FactionJoined"],"p":1010,"c":"O(j)"},
  {"i":"mge.social.faction.v1.fn.update_faction_rank","k":"s","d":"social","r":["FactionMember","Faction"],"w":["FactionMember"],"e":["RankChanged"],"p":1011,"c":"O(m)"},
  {"i":"mge.social.faction.v1.fn.process_leave_faction","k":"s","d":"social","r":["FactionMember","Faction"],"w":["Faction","FactionMember"],"e":["FactionLeft","FactionDissolved"],"p":1012,"c":"O(l)"},
  {"i":"mge.social.faction.v1.fn.update_faction_relations","k":"s","d":"social","r":["FactionRelation"],"w":["FactionRelation"],"e":[],"p":1013,"c":"O(f^2)"},
  {"i":"mge.social.faction.v1.event.faction_joined","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.event.faction_left","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.event.rank_changed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.faction.v1.event.faction_dissolved","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let faction = world.spawn();
world.insert(faction, Faction {
    name_id: 1001,
    leader: None,
    member_count: 0,
    founded_tick: ctx.current_tick(),
});

let entity = world.spawn();
world.insert(entity, FactionMember {
    entity,
    faction_id: faction,
    rank: RankTier::Leader,
    joined_tick: ctx.current_tick(),
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
