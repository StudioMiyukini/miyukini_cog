# mge-gs-population

> @id mge.gs.population.v1  
> @role plugin  
> @domain grand-strategy  
> @do manage_demographics_growth_migration_classes  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gs-population` |
| @id MSCM | `mge.gs.population.v1` |
| Domaine | grand-strategy |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (croissance calculee chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(p) ou p=provinces |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `PopulationClass` | `Noble, Clergy, Burgher, Peasant, Slave` | Classe sociale de la population |
| `MigrationReason` | `War, Famine, Opportunity, Persecution, Natural` | Cause de la migration |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Population` | `mge.gs.population.v1.component.population` | `count: u32, growth_rate: f32, class_distribution: HashMap<PopulationClass, f32>` | Population d'une province. class_distribution = pourcentages (somme = 1.0) |
| `Migration` | `mge.gs.population.v1.component.migration` | `origin: EntityId, destination: EntityId, count: u32, reason: MigrationReason, progress: f32` | Vague migratoire en cours entre deux provinces |
| `PopGrowth` | `mge.gs.population.v1.component.pop_growth` | `birth_rate: f32, death_rate: f32, disease_factor: f32, prosperity_factor: f32` | Facteurs de croissance et mortalite |

---

## 4. Formules

```
net_growth      = (birth_rate - death_rate) * prosperity_factor * (1.0 - disease_factor)
pop_delta       = population.count * net_growth * dt
population_new  = max(population.count + pop_delta, 0)

migration_pull  = destination_prosperity - origin_prosperity
migration_push  = war_factor + famine_factor + persecution_factor
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_population_growth` | `mge.gs.population.v1.fn.tick_population_growth` | Logic (1240) | Population, PopGrowth | Population | PopulationGrowth, PopulationDecline | O(p) | Applique la croissance/declin naturel de la population |
| `process_migration` | `mge.gs.population.v1.fn.process_migration` | Logic (1241) | Migration, Population | Migration, Population | MigrationWave | O(m) | Transfere les migrants entre provinces |
| `update_class_distribution` | `mge.gs.population.v1.fn.update_class_distribution` | Logic (1242) | Population | Population | none | O(p) | Reequilibre la distribution des classes selon l'economie |
| `check_population_events` | `mge.gs.population.v1.fn.check_population_events` | Logic (1243) | Population, PopGrowth | Population | Famine | O(p) | Detecte les famines et evenements demographiques |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `PopulationGrowth` | `mge.gs.population.v1.event.population_growth` | `province: EntityId, delta: u32, new_total: u32` | `tick_population_growth` | economy, military, ui |
| `PopulationDecline` | `mge.gs.population.v1.event.population_decline` | `province: EntityId, delta: u32, new_total: u32, cause: String` | `tick_population_growth` | economy, ui |
| `MigrationWave` | `mge.gs.population.v1.event.migration_wave` | `origin: EntityId, destination: EntityId, count: u32, reason: MigrationReason` | `process_migration` | culture, religion, ui |
| `Famine` | `mge.gs.population.v1.event.famine` | `province: EntityId, severity: f32, pop_affected: u32` | `check_population_events` | economy, military, ui |

---

## 7. Invariants

- `Population.count` ne devient jamais negatif (min 0).
- `Population.class_distribution` somme toujours a 1.0 (± epsilon).
- `PopGrowth.birth_rate` et `death_rate` sont toujours >= 0.0.
- `Migration.count` ne depasse pas la population de l'origin.
- Une famine est declenchee si food_supply < population_need.
- Les migrations terminees (progress >= 1.0) sont supprimees au tick suivant.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `base_birth_rate` | `f32` | 0.03 | [0.01, 0.1] | Taux de natalite de base par tick |
| `base_death_rate` | `f32` | 0.02 | [0.005, 0.05] | Taux de mortalite de base par tick |
| `migration_speed` | `f32` | 0.1 | [0.01, 1.0] | Vitesse de progression des migrations par tick |
| `famine_threshold` | `f32` | 0.5 | [0.1, 0.9] | Ratio food/need declenchant la famine |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la croissance et le declin de population | Ne gere pas la nourriture (→ economy, province) |
| Simule les migrations entre provinces | Ne gere pas la religion de la population (→ religion) |
| Distribue les classes sociales | Ne gere pas la culture de la population (→ culture) |
| Detecte les famines | Ne gere pas le recrutement militaire (→ military) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Population, Migration, PopGrowth |
| Ecrit | Population, Migration |
| Emet | PopulationGrowth, PopulationDecline, MigrationWave, Famine |
| Ne touche jamais | DiplomaticStance, Treasury, TradeRoute, Army, Province, Religion, Culture, CasusBelli |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gs-population/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.gs.population.v1
    ├── components.rs     # Population, Migration, PopGrowth
    ├── systems.rs        # tick_population_growth, process_migration, update_class_distribution, check_population_events
    └── events.rs         # PopulationGrowth, PopulationDecline, MigrationWave, Famine
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (tick_population_growth) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants avec @id et @fields
- [ ] 4 systemes avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements avec @id et @fields
- [ ] 2 enumerations (PopulationClass, MigrationReason)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : growth, migration, class distribution, famine
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.gs.population.v1","k":"p","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.component.population","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.component.migration","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.component.pop_growth","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.fn.tick_population_growth","k":"s","d":"grand-strategy","r":["Population","PopGrowth"],"w":["Population"],"e":["PopulationGrowth","PopulationDecline"],"p":1240,"c":"O(p)"},
  {"i":"mge.gs.population.v1.fn.process_migration","k":"s","d":"grand-strategy","r":["Migration","Population"],"w":["Migration","Population"],"e":["MigrationWave"],"p":1241,"c":"O(m)"},
  {"i":"mge.gs.population.v1.fn.update_class_distribution","k":"s","d":"grand-strategy","r":["Population"],"w":["Population"],"e":[],"p":1242,"c":"O(p)"},
  {"i":"mge.gs.population.v1.fn.check_population_events","k":"s","d":"grand-strategy","r":["Population","PopGrowth"],"w":["Population"],"e":["Famine"],"p":1243,"c":"O(p)"},
  {"i":"mge.gs.population.v1.event.population_growth","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.event.population_decline","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.event.migration_wave","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.population.v1.event.famine","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let paris = world.spawn();
world.insert(paris, Population {
    count: 50000,
    growth_rate: 0.01,
    class_distribution: HashMap::from([
        (PopulationClass::Noble, 0.05),
        (PopulationClass::Clergy, 0.05),
        (PopulationClass::Burgher, 0.20),
        (PopulationClass::Peasant, 0.70),
    ]),
});
world.insert(paris, PopGrowth {
    birth_rate: 0.03,
    death_rate: 0.02,
    disease_factor: 0.0,
    prosperity_factor: 1.2,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Grand Strategy - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
