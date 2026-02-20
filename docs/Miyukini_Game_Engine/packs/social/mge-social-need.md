# mge-social-need

> @id mge.social.need.v1  
> @role plugin  
> @domain social  
> @do manage_entity_needs_hunger_rest_social  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-need` |
| @id MSCM | `mge.social.need.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (besoins degradent chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * k) ou n=entites, k=types de besoins |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `NeedType` | `Hunger, Rest, Social, Hygiene, Fun, Comfort` | Type de besoin trackable |
| `NeedUrgency` | `Satisfied, Low, Critical, Emergency` | Niveau d'urgence derive de la valeur |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `NeedSet` | `mge.social.need.v1.component.need_set` | `entity: EntityId, needs: Vec<NeedState>` | Ensemble des besoins d'une entite |
| `NeedState` | `mge.social.need.v1.component.need_state` | `need_type: NeedType, value: f32, decay_rate: f32, urgency: NeedUrgency` | Etat d'un besoin. value [0, 100], decay_rate par tick |

---

## 4. Formules

```
value_new    = clamp(value - decay_rate * dt, 0.0, 100.0)

urgency =
  if value >= 70.0   => Satisfied
  if value >= 40.0   => Low
  if value >= 15.0   => Critical
  else               => Emergency

priority_score = (100.0 - value) * need_weight
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_needs` | `mge.social.need.v1.fn.tick_needs` | Logic (1030) | NeedSet | NeedSet | none | O(n*k) | Decremente la valeur de chaque besoin selon decay_rate |
| `satisfy_need` | `mge.social.need.v1.fn.satisfy_need` | Logic (1031) | NeedSet | NeedSet | NeedSatisfied | O(s) | Traite les satisfactions en attente. Augmente value |
| `check_critical_need` | `mge.social.need.v1.fn.check_critical_need` | Logic (1032) | NeedSet | NeedSet | NeedCritical | O(n*k) | Detecte les besoins passant en Critical ou Emergency. Met a jour urgency |
| `evaluate_need_priority` | `mge.social.need.v1.fn.evaluate_need_priority` | Logic (1033) | NeedSet | NeedSet | NeedChanged | O(n*k) | Trie les besoins par priorite pour l'AI scheduling |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `NeedCritical` | `mge.social.need.v1.event.need_critical` | `entity: EntityId, need_type: NeedType, value: f32` | `check_critical_need` | schedule, ai, ui |
| `NeedSatisfied` | `mge.social.need.v1.event.need_satisfied` | `entity: EntityId, need_type: NeedType, amount: f32` | `satisfy_need` | ui, schedule |
| `NeedChanged` | `mge.social.need.v1.event.need_changed` | `entity: EntityId, need_type: NeedType, old_urgency: NeedUrgency, new_urgency: NeedUrgency` | `evaluate_need_priority` | schedule, ai |

---

## 7. Invariants

- `NeedState.value` est toujours dans [0.0, 100.0] apres `tick_needs` et `satisfy_need`.
- `NeedState.urgency` est toujours coherent avec `value` apres `check_critical_need`.
- `NeedCritical` n'est emis qu'a la transition vers Critical ou Emergency, pas a chaque tick.
- `NeedSet.needs` contient au plus un `NeedState` par `NeedType`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `hunger_decay_rate` | `f32` | 0.05 | [0.01, 1.0] | Declin faim par tick |
| `rest_decay_rate` | `f32` | 0.03 | [0.01, 1.0] | Declin repos par tick |
| `social_decay_rate` | `f32` | 0.02 | [0.01, 1.0] | Declin besoin social par tick |
| `critical_threshold` | `f32` | 15.0 | [5.0, 30.0] | Seuil Emergency |
| `satisfied_threshold` | `f32` | 70.0 | [50.0, 90.0] | Seuil Satisfied |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Degrade les besoins chaque tick | Ne planifie pas les activites (-> schedule) |
| Detecte les etats critiques | Ne deplace pas l'entite vers une ressource (-> ai) |
| Satisfait les besoins sur demande | Ne gere pas l'inventaire nourriture (-> rpg-inventory) |
| Calcule les priorites de besoins | Ne gere pas les emotions (-> personality) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | NeedSet, NeedState |
| Ecrit | NeedSet, NeedState |
| Emet | NeedCritical, NeedSatisfied, NeedChanged |
| Ne touche jamais | Relationship, Faction, Reputation, Schedule, PersonalityTraits, GossipMemory |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-need/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.need.v1
    ├── components.rs     # NeedSet, NeedState
    ├── systems.rs        # tick_needs, satisfy_need, check_critical_need, evaluate_need_priority
    └── events.rs         # NeedCritical, NeedSatisfied, NeedChanged
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs`
- [ ] 3 evenements dans `events.rs`
- [ ] 2 enumerations (NeedType, NeedUrgency)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : decay, satisfy, critical detection, priority
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.need.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.need.v1.component.need_set","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.need.v1.component.need_state","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.need.v1.fn.tick_needs","k":"s","d":"social","r":["NeedSet"],"w":["NeedSet"],"e":[],"p":1030,"c":"O(n*k)"},
  {"i":"mge.social.need.v1.fn.satisfy_need","k":"s","d":"social","r":["NeedSet"],"w":["NeedSet"],"e":["NeedSatisfied"],"p":1031,"c":"O(s)"},
  {"i":"mge.social.need.v1.fn.check_critical_need","k":"s","d":"social","r":["NeedSet"],"w":["NeedSet"],"e":["NeedCritical"],"p":1032,"c":"O(n*k)"},
  {"i":"mge.social.need.v1.fn.evaluate_need_priority","k":"s","d":"social","r":["NeedSet"],"w":["NeedSet"],"e":["NeedChanged"],"p":1033,"c":"O(n*k)"},
  {"i":"mge.social.need.v1.event.need_critical","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.need.v1.event.need_satisfied","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.need.v1.event.need_changed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, NeedSet {
    entity,
    needs: vec![
        NeedState { need_type: NeedType::Hunger, value: 80.0, decay_rate: 0.05, urgency: NeedUrgency::Satisfied },
        NeedState { need_type: NeedType::Rest, value: 60.0, decay_rate: 0.03, urgency: NeedUrgency::Low },
        NeedState { need_type: NeedType::Social, value: 45.0, decay_rate: 0.02, urgency: NeedUrgency::Low },
    ],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
