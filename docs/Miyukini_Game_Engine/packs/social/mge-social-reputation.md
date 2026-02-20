# mge-social-reputation

> @id mge.social.reputation.v1  
> @role plugin  
> @domain social  
> @do manage_reputation_scores_thresholds_decay  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-reputation` |
| @id MSCM | `mge.social.reputation.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (updates evenementiels, decay periodique) |
| Headless safe | Oui |
| Complexite globale | O(n * f) ou n=entites, f=factions/regions tracees |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ReputationLevel` | `Revered, Honored, Friendly, Neutral, Unfriendly, Hostile, Hated` | Niveau de reputation derive du score |
| `ReputationSource` | `Faction, Region, Global` | Contexte de la reputation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Reputation` | `mge.social.reputation.v1.component.reputation` | `entity: EntityId, source_type: ReputationSource, source_id: EntityId, score: f32, level: ReputationLevel` | Score de reputation d'une entite aupres d'une faction/region |
| `ReputationThresholds` | `mge.social.reputation.v1.component.reputation_thresholds` | `revered: f32, honored: f32, friendly: f32, unfriendly: f32, hostile: f32, hated: f32` | Seuils de transition entre niveaux. Configurable par faction |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_reputation_score` | `mge.social.reputation.v1.fn.update_reputation_score` | Logic (1020) | Reputation | Reputation | ReputationChanged | O(n*f) | Applique les deltas de reputation accumules |
| `check_reputation_thresholds` | `mge.social.reputation.v1.fn.check_reputation_thresholds` | Logic (1021) | Reputation, ReputationThresholds | Reputation | ThresholdCrossed | O(n*f) | Detecte les changements de niveau et met a jour level |
| `decay_reputation` | `mge.social.reputation.v1.fn.decay_reputation` | Logic (1022) | Reputation | Reputation | none | O(n*f) | Declin naturel vers Neutral sur les reputations non entretenues |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ReputationChanged` | `mge.social.reputation.v1.event.reputation_changed` | `entity: EntityId, source_id: EntityId, old_score: f32, new_score: f32` | `update_reputation_score` | faction, ui, gossip |
| `ThresholdCrossed` | `mge.social.reputation.v1.event.threshold_crossed` | `entity: EntityId, source_id: EntityId, old_level: ReputationLevel, new_level: ReputationLevel` | `check_reputation_thresholds` | faction, ai, ui, dialogue |

---

## 7. Invariants

- `Reputation.level` est toujours coherent avec `score` et `ReputationThresholds` apres `check_reputation_thresholds`.
- `Reputation.score` est toujours dans [-100.0, 100.0].
- `ThresholdCrossed` n'est emis que si `old_level != new_level`.
- Le decay ne pousse jamais le score au-dela de la valeur neutre (0.0).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `reputation_decay_rate` | `f32` | 0.005 | [0.0, 0.05] | Declin par tick vers 0 |
| `default_revered_threshold` | `f32` | 90.0 | [70.0, 100.0] | Seuil Revered par defaut |
| `default_hated_threshold` | `f32` | -90.0 | [-100.0, -70.0] | Seuil Hated par defaut |
| `max_reputation_entries` | `u32` | 32 | [4, 128] | Nombre max de reputations trackees par entite |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke et met a jour les scores de reputation | Ne gere pas les factions (-> faction) |
| Detecte les franchissements de seuils | Ne gere pas les relations individuelles (-> relationship) |
| Applique le declin naturel | Ne gere pas les consequences (-> ai, dialogue) |
| Supporte reputation par faction et region | Ne gere pas la propagation (-> gossip) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Reputation, ReputationThresholds |
| Ecrit | Reputation |
| Emet | ReputationChanged, ThresholdCrossed |
| Ne touche jamais | Relationship, Faction, NeedSet, Schedule, PersonalityTraits, GossipMemory |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-reputation/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.reputation.v1
    ├── components.rs     # Reputation, ReputationThresholds
    ├── systems.rs        # update_reputation_score, check_reputation_thresholds, decay_reputation
    └── events.rs         # ReputationChanged, ThresholdCrossed
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
- [ ] 3 systemes dans `systems.rs`
- [ ] 2 evenements dans `events.rs`
- [ ] 2 enumerations (ReputationLevel, ReputationSource)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : score update, threshold crossing, decay
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.reputation.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.reputation.v1.component.reputation","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.reputation.v1.component.reputation_thresholds","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.reputation.v1.fn.update_reputation_score","k":"s","d":"social","r":["Reputation"],"w":["Reputation"],"e":["ReputationChanged"],"p":1020,"c":"O(n*f)"},
  {"i":"mge.social.reputation.v1.fn.check_reputation_thresholds","k":"s","d":"social","r":["Reputation","ReputationThresholds"],"w":["Reputation"],"e":["ThresholdCrossed"],"p":1021,"c":"O(n*f)"},
  {"i":"mge.social.reputation.v1.fn.decay_reputation","k":"s","d":"social","r":["Reputation"],"w":["Reputation"],"e":[],"p":1022,"c":"O(n*f)"},
  {"i":"mge.social.reputation.v1.event.reputation_changed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.reputation.v1.event.threshold_crossed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
let faction = world.spawn();

world.insert(world.spawn(), Reputation {
    entity,
    source_type: ReputationSource::Faction,
    source_id: faction,
    score: 0.0,
    level: ReputationLevel::Neutral,
});

world.insert(faction, ReputationThresholds {
    revered: 90.0, honored: 60.0, friendly: 20.0,
    unfriendly: -20.0, hostile: -60.0, hated: -90.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
