# mge-idle-offline

> @id mge.idle.offline.v1  
> @role plugin  
> @domain idle  
> @do manage_offline_progression_and_catchup_earnings  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-idle-offline` |
| @id MSCM | `mge.idle.offline.v1` |
| Domaine | idle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-idle-producer` |
| Hot path | Non (execute une seule fois au login) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites Producer |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `OfflineMode` | `Full, Capped, Percentage` | Mode de calcul des gains hors-ligne |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `OfflineState` | `mge.idle.offline.v1.component.offline_state` | `mode: OfflineMode, efficiency: f64, processed: bool` | Configuration du mode offline et flag de traitement |
| `LastPlayedTimestamp` | `mge.idle.offline.v1.component.last_played_timestamp` | `timestamp_secs: u64` | Dernier timestamp de jeu en secondes Unix |
| `OfflineEarnings` | `mge.idle.offline.v1.component.offline_earnings` | `earnings_by_resource: Vec<(ResourceType, f64)>, duration_secs: u64` | Gains calcules au retour, par type de ressource |

---

## 4. Formules

```
elapsed_secs = min(current_time - last_played_timestamp, max_offline_duration)
offline_ticks = elapsed_secs / tick_duration

Mode Full:       earnings = production_rate * offline_ticks * efficiency
Mode Capped:     earnings = min(production_rate * offline_ticks * efficiency, cap)
Mode Percentage: earnings = production_rate * offline_ticks * efficiency_percentage
```

L'`efficiency` est un facteur de reduction (0.0 a 1.0). Les gains offline sont generalement inferieurs aux gains online pour eviter l'exploitation.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_offline_duration` | `mge.idle.offline.v1.fn.compute_offline_duration` | 2140 | LastPlayedTimestamp, OfflineState | OfflineState | none | O(1) | Calcule la duree ecoulee. Clampe a max_offline_duration |
| `calculate_offline_earnings` | `mge.idle.offline.v1.fn.calculate_offline_earnings` | 2141 | OfflineState, Producer, ProductionRate, GlobalMultiplier | OfflineEarnings | none | O(n) | Simule la production pour chaque producteur sur la duree offline |
| `apply_offline_earnings` | `mge.idle.offline.v1.fn.apply_offline_earnings` | 2142 | OfflineEarnings, OfflineState | ResourceOutput, LastPlayedTimestamp, OfflineState | OfflineEarningsCalculated | O(n) | Ajoute les gains aux ResourceOutput. Met a jour le timestamp. Marque processed = true |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `OfflineEarningsCalculated` | `mge.idle.offline.v1.event.offline_earnings_calculated` | `duration_secs: u64, earnings_by_resource: Vec<(ResourceType, f64)>` | `apply_offline_earnings` | UI (ecran de retour), analytics |

---

## 7. Invariants

- `compute_offline_duration` ne s'execute que si `processed = false`.
- Les 3 systemes ne s'executent qu'une seule fois par session (guard `processed`).
- `OfflineEarnings.duration_secs` ne depasse jamais `max_offline_duration`.
- Les gains offline sont toujours >= 0.0.
- `LastPlayedTimestamp` est mis a jour en fin de chaque session et apres apply.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_offline_duration` | `u64` | 28800 | [60, 604800] | Duree max offline en secondes (defaut : 8h, max : 7 jours) |
| `offline_efficiency` | `f64` | 0.5 | [0.0, 1.0] | Rendement offline par rapport au online (0.5 = 50%) |
| `offline_mode` | `OfflineMode` | `Full` | Full, Capped, Percentage | Mode de calcul |
| `offline_cap` | `f64` | 1_000_000.0 | [0.0, 1e18] | Plafond de gains en mode Capped |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule la duree offline | Ne gere pas la sauvegarde du timestamp (→ save-load) |
| Simule les gains des producteurs | Ne modifie pas les producteurs (lecture seule) |
| Applique les gains aux ResourceOutput | Ne gere pas l'UI de retour (→ UI layer) |
| Respecte le plafond et l'efficience | Ne gere pas le prestige offline (→ prestige) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | LastPlayedTimestamp, OfflineState, Producer, ProductionRate, GlobalMultiplier, OfflineEarnings |
| Ecrit | OfflineState, OfflineEarnings, ResourceOutput, LastPlayedTimestamp |
| Emet | OfflineEarningsCalculated |
| Ne touche jamais | UpgradeState, PrestigeState, Multiplier |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-idle-offline/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.idle.offline.v1, trait Plugin impl
    ├── components.rs     # OfflineState, LastPlayedTimestamp, OfflineEarnings
    ├── systems.rs        # compute_offline_duration, calculate_offline_earnings, apply_offline_earnings
    └── events.rs         # OfflineEarningsCalculated
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (OfflineMode)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : duree offline, gains calcules, cap, efficience, guard processed
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.idle.offline.v1","k":"p","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.offline.v1.component.offline_state","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.offline.v1.component.last_played_timestamp","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.offline.v1.component.offline_earnings","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.offline.v1.fn.compute_offline_duration","k":"s","d":"idle","r":["LastPlayedTimestamp","OfflineState"],"w":["OfflineState"],"e":[],"p":2140,"c":"O(1)"},
  {"i":"mge.idle.offline.v1.fn.calculate_offline_earnings","k":"s","d":"idle","r":["OfflineState","Producer","ProductionRate","GlobalMultiplier"],"w":["OfflineEarnings"],"e":[],"p":2141,"c":"O(n)"},
  {"i":"mge.idle.offline.v1.fn.apply_offline_earnings","k":"s","d":"idle","r":["OfflineEarnings","OfflineState"],"w":["ResourceOutput","LastPlayedTimestamp","OfflineState"],"e":["OfflineEarningsCalculated"],"p":2142,"c":"O(n)"},
  {"i":"mge.idle.offline.v1.event.offline_earnings_calculated","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, OfflineState { mode: OfflineMode::Full, efficiency: 0.5, processed: false });
world.insert(player, LastPlayedTimestamp { timestamp_secs: 1708300000 });
world.insert(player, OfflineEarnings { earnings_by_resource: vec![], duration_secs: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Idle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
