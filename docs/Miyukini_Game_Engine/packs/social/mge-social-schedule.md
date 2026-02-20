# mge-social-schedule

> @id mge.social.schedule.v1  
> @role plugin  
> @domain social  
> @do manage_daily_schedules_routines_activities  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-schedule` |
| @id MSCM | `mge.social.schedule.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-social-need` |
| Hot path | Non (avance par slot temporel, pas chaque frame) |
| Headless safe | Oui |
| Complexite globale | O(n * s) ou n=entites, s=slots par planning |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ActivityType` | `Sleep, Eat, Work, Socialize, Leisure, Travel, Idle` | Type d'activite planifiable |
| `ScheduleState` | `Active, Paused, Completed, Interrupted` | Etat du planning courant |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Schedule` | `mge.social.schedule.v1.component.schedule` | `entity: EntityId, slots: Vec<ScheduleSlot>, current_index: u16, state: ScheduleState` | Planning quotidien d'une entite |
| `ScheduleSlot` | `mge.social.schedule.v1.component.schedule_slot` | `activity: ActivityType, start_tick: u32, duration_ticks: u32, priority: u8, location_hint: Option<EntityId>` | Creneau horaire avec activite et duree |
| `CurrentActivity` | `mge.social.schedule.v1.component.current_activity` | `entity: EntityId, activity: ActivityType, remaining_ticks: u32, started_tick: u64` | Activite en cours d'execution |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `advance_schedule` | `mge.social.schedule.v1.fn.advance_schedule` | Logic (1040) | Schedule, CurrentActivity | Schedule, CurrentActivity | ActivityStarted, ActivityCompleted | O(n) | Avance le planning. Passe au slot suivant si duree ecoulee |
| `assign_activity` | `mge.social.schedule.v1.fn.assign_activity` | Logic (1041) | Schedule | CurrentActivity | ActivityStarted | O(a) | Assigne une activite forcee hors planning (besoin critique) |
| `check_schedule_conflict` | `mge.social.schedule.v1.fn.check_schedule_conflict` | Logic (1042) | Schedule, CurrentActivity | Schedule | ScheduleConflict | O(n) | Detecte les conflits entre activite en cours et prochain slot |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ActivityStarted` | `mge.social.schedule.v1.event.activity_started` | `entity: EntityId, activity: ActivityType, duration: u32` | `advance_schedule`, `assign_activity` | need, ai, ui |
| `ActivityCompleted` | `mge.social.schedule.v1.event.activity_completed` | `entity: EntityId, activity: ActivityType` | `advance_schedule` | need, ai |
| `ScheduleConflict` | `mge.social.schedule.v1.event.schedule_conflict` | `entity: EntityId, current: ActivityType, scheduled: ActivityType` | `check_schedule_conflict` | ai, ui |

---

## 7. Invariants

- `Schedule.current_index` est toujours dans [0, slots.len()).
- `CurrentActivity.remaining_ticks` est decremente chaque tick et ne descend pas sous 0.
- `ActivityCompleted` est emis exactement une fois a la fin de chaque activite.
- Un slot avec `priority > current.priority` interrompt l'activite en cours.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `max_slots_per_schedule` | `u16` | 24 | [4, 96] | Nombre max de creneaux par planning |
| `idle_activity_duration` | `u32` | 60 | [10, 600] | Duree par defaut de l'activite Idle en ticks |
| `interrupt_priority_threshold` | `u8` | 200 | [100, 255] | Priorite minimale pour interrompre un slot actif |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les plannings quotidiens | Ne deplace pas l'entite (-> spatial, ai) |
| Avance d'activite en activite | Ne satisfait pas les besoins (-> need) |
| Detecte les conflits de planning | Ne decide pas quelle activite choisir (-> ai) |
| Supporte l'interruption par priorite | Ne gere pas les lieux (-> spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Schedule, ScheduleSlot, CurrentActivity |
| Ecrit | Schedule, CurrentActivity |
| Emet | ActivityStarted, ActivityCompleted, ScheduleConflict |
| Ne touche jamais | Relationship, Faction, Reputation, NeedSet, PersonalityTraits, GossipMemory |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-schedule/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.schedule.v1
    ├── components.rs     # Schedule, ScheduleSlot, CurrentActivity
    ├── systems.rs        # advance_schedule, assign_activity, check_schedule_conflict
    └── events.rs         # ActivityStarted, ActivityCompleted, ScheduleConflict
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
- [ ] 3 systemes dans `systems.rs`
- [ ] 3 evenements dans `events.rs`
- [ ] 2 enumerations (ActivityType, ScheduleState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : schedule advance, assign, conflict detection
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.schedule.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.component.schedule","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.component.schedule_slot","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.component.current_activity","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.fn.advance_schedule","k":"s","d":"social","r":["Schedule","CurrentActivity"],"w":["Schedule","CurrentActivity"],"e":["ActivityStarted","ActivityCompleted"],"p":1040,"c":"O(n)"},
  {"i":"mge.social.schedule.v1.fn.assign_activity","k":"s","d":"social","r":["Schedule"],"w":["CurrentActivity"],"e":["ActivityStarted"],"p":1041,"c":"O(a)"},
  {"i":"mge.social.schedule.v1.fn.check_schedule_conflict","k":"s","d":"social","r":["Schedule","CurrentActivity"],"w":["Schedule"],"e":["ScheduleConflict"],"p":1042,"c":"O(n)"},
  {"i":"mge.social.schedule.v1.event.activity_started","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.event.activity_completed","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.schedule.v1.event.schedule_conflict","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Schedule {
    entity,
    slots: vec![
        ScheduleSlot { activity: ActivityType::Sleep, start_tick: 0, duration_ticks: 480, priority: 10, location_hint: None },
        ScheduleSlot { activity: ActivityType::Eat, start_tick: 480, duration_ticks: 60, priority: 20, location_hint: None },
        ScheduleSlot { activity: ActivityType::Work, start_tick: 540, duration_ticks: 480, priority: 15, location_hint: None },
    ],
    current_index: 0,
    state: ScheduleState::Active,
});
world.insert(entity, CurrentActivity {
    entity,
    activity: ActivityType::Sleep,
    remaining_ticks: 480,
    started_tick: 0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Social Simulation - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
