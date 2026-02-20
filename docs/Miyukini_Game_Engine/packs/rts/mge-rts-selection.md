# mge-rts-selection

> @id mge.rts.selection.v1  
> @role plugin  
> @domain rts  
> @do manage_unit_selection_box_selection_control_groups  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-selection` |
| @id MSCM | `mge.rts.selection.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-plugin-input` |
| Hot path | Oui (selection traitee chaque frame sur input joueur) |
| Headless safe | Oui |
| Complexite globale | O(n) ou n=entites selectionnables dans la zone |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `SelectionMode` | `Single, Box, AddSingle, AddBox` | Mode de selection actif (simple, rectangle, ajout) |
| `GroupSlot` | `G1, G2, G3, G4, G5, G6, G7, G8, G9, G0` | Slot de control group (touches 0-9) |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Selection` | `mge.rts.selection.v1.component.selection` | `entities: Vec<EntityId>, mode: SelectionMode` | Ensemble des entites actuellement selectionnees par le joueur |
| `SelectionBox` | `mge.rts.selection.v1.component.selection_box` | `start_x: f32, start_y: f32, end_x: f32, end_y: f32, active: bool` | Rectangle de selection en coordonnees ecran. active=true pendant le drag |
| `ControlGroup` | `mge.rts.selection.v1.component.control_group` | `slot: GroupSlot, entities: Vec<EntityId>` | Groupe de controle memorise sur un slot clavier |
| `Selectable` | `mge.rts.selection.v1.component.selectable` | `team: u8, priority: u8, selected: bool` | Marque une entite comme selectionnable. priority pour le tri |

---

## 4. Formules

Non applicable. La selection est booleenne (dans la box ou non).

```
in_box = entity.world_x >= min_x && entity.world_x <= max_x
      && entity.world_y >= min_y && entity.world_y <= max_y
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_box_selection` | `mge.rts.selection.v1.fn.process_box_selection` | Input (1100) | SelectionBox, Selectable, Position2D | Selection, Selectable | SelectionChanged | O(n) | Resout la box selection : collecte les entites dans le rectangle |
| `update_selection` | `mge.rts.selection.v1.fn.update_selection` | Input (1101) | Selection, Selectable | Selectable | none | O(s) | Met a jour le flag selected sur les entites apres changement de selection |
| `assign_control_group` | `mge.rts.selection.v1.fn.assign_control_group` | Input (1102) | Selection | ControlGroup | ControlGroupAssigned | O(s) | Memorise la selection courante dans un slot de control group |
| `recall_control_group` | `mge.rts.selection.v1.fn.recall_control_group` | Input (1103) | ControlGroup | Selection, Selectable | ControlGroupRecalled | O(g) | Restaure la selection depuis un control group memorise |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `SelectionChanged` | `mge.rts.selection.v1.event.selection_changed` | `old_entities: Vec<EntityId>, new_entities: Vec<EntityId>, mode: SelectionMode` | `process_box_selection` | ui, unit-ai |
| `ControlGroupAssigned` | `mge.rts.selection.v1.event.control_group_assigned` | `slot: GroupSlot, entities: Vec<EntityId>` | `assign_control_group` | ui |
| `ControlGroupRecalled` | `mge.rts.selection.v1.event.control_group_recalled` | `slot: GroupSlot, entities: Vec<EntityId>` | `recall_control_group` | ui, unit-ai |

---

## 7. Invariants

- `Selection.entities` ne contient que des `EntityId` avec un composant `Selectable`.
- Un control group ne contient que des entites vivantes ; les mortes sont purgees au recall.
- `SelectionBox.active` est `false` quand aucun drag n'est en cours.
- `Selectable.selected` est toujours synchronise avec `Selection.entities` apres `update_selection`.
- Une entite ne peut apparaitre qu'une seule fois dans `Selection.entities` (pas de doublons).
- Les control groups sont independants : une entite peut etre dans plusieurs groupes.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `max_selection_size` | `u32` | 200 | [1, 1000] | Nombre max d'entites selectionnables simultanement |
| `box_min_size` | `f32` | 5.0 | [1.0, 50.0] | Taille min du rectangle pour declencher box selection (pixels) |
| `double_click_select_type` | `bool` | true | — | Active la selection de toutes les unites du meme type par double-clic |
| `control_group_count` | `u8` | 10 | [1, 10] | Nombre de control groups disponibles |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la selection d'entites par clic et rectangle | Ne gere pas les ordres (→ unit-ai) |
| Memorise et rappelle les control groups | Ne gere pas le deplacement des unites (→ spatial) |
| Filtre par equipe les entites selectionnables | Ne gere pas la visibilite (→ fog-of-war) |
| Supporte les modes single et box | Ne gere pas le rendu du rectangle (→ ui) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | SelectionBox, Selectable, Position2D, ControlGroup |
| Ecrit | Selection, Selectable, ControlGroup |
| Emet | SelectionChanged, ControlGroupAssigned, ControlGroupRecalled |
| Ne touche jamais | ProductionQueue, ResourceNode, Building, OrderQueue, FogGrid, TechNode |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-selection/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.selection.v1, trait Plugin impl
    ├── components.rs     # Selection, SelectionBox, ControlGroup, Selectable
    ├── systems.rs        # process_box_selection, update_selection, assign_control_group, recall_control_group
    └── events.rs         # SelectionChanged, ControlGroupAssigned, ControlGroupRecalled
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire (process_box_selection) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (SelectionMode, GroupSlot)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : box selection, control group assign/recall, selection sync
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.selection.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.component.selection","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.component.selection_box","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.component.control_group","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.component.selectable","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.fn.process_box_selection","k":"s","d":"rts","r":["SelectionBox","Selectable","Position2D"],"w":["Selection","Selectable"],"e":["SelectionChanged"],"p":1100,"c":"O(n)"},
  {"i":"mge.rts.selection.v1.fn.update_selection","k":"s","d":"rts","r":["Selection","Selectable"],"w":["Selectable"],"e":[],"p":1101,"c":"O(s)"},
  {"i":"mge.rts.selection.v1.fn.assign_control_group","k":"s","d":"rts","r":["Selection"],"w":["ControlGroup"],"e":["ControlGroupAssigned"],"p":1102,"c":"O(s)"},
  {"i":"mge.rts.selection.v1.fn.recall_control_group","k":"s","d":"rts","r":["ControlGroup"],"w":["Selection","Selectable"],"e":["ControlGroupRecalled"],"p":1103,"c":"O(g)"},
  {"i":"mge.rts.selection.v1.event.selection_changed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.event.control_group_assigned","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.selection.v1.event.control_group_recalled","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player_sel = world.spawn();
world.insert(player_sel, Selection {
    entities: vec![],
    mode: SelectionMode::Single,
});
world.insert(player_sel, SelectionBox {
    start_x: 0.0, start_y: 0.0,
    end_x: 0.0, end_y: 0.0,
    active: false,
});

let soldier = world.spawn();
world.insert(soldier, Selectable { team: 1, priority: 0, selected: false });
world.insert(soldier, Position2D { x: 100.0, y: 50.0 });

for slot in [GroupSlot::G1, GroupSlot::G2, GroupSlot::G3] {
    let cg = world.spawn();
    world.insert(cg, ControlGroup { slot, entities: vec![] });
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
