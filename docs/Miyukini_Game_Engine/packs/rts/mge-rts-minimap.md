# mge-rts-minimap

> @id mge.rts.minimap.v1  
> @role plugin  
> @domain rts  
> @do manage_minimap_entries_icons_pings  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-minimap` |
| @id MSCM | `mge.rts.minimap.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Oui (mise a jour positions chaque tick) |
| Headless safe | Oui (donnees seulement, pas de rendu) |
| Complexite globale | O(e) ou e=entites sur la minimap |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `MinimapIcon` | `Unit, Building, Resource, Alert, Custom(u16)` | Type d'icone affichee sur la minimap |
| `PingType` | `Attack, Defend, Gather, Warning, Custom(u16)` | Type de ping signalant un evenement |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `MinimapEntry` | `mge.rts.minimap.v1.component.minimap_entry` | `icon: MinimapIcon, color: u32, visible: bool, team: u8` | Marque une entite comme representee sur la minimap |
| `MinimapPing` | `mge.rts.minimap.v1.component.minimap_ping` | `ping_type: PingType, position: (f32, f32), duration: f32, remaining: f32, sender: EntityId` | Ping temporaire sur la minimap. remaining decremente chaque tick |

---

## 4. Formules

```
minimap_x = (world_x / world_width) * minimap_width
minimap_y = (world_y / world_height) * minimap_height
remaining = max(remaining - dt, 0.0)
expired   = remaining <= 0.0
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_minimap_entries` | `mge.rts.minimap.v1.fn.update_minimap_entries` | PostLogic (1150) | MinimapEntry, Position2D | MinimapEntry | none | O(e) | Synchronise les positions minimap avec les positions monde |
| `tick_minimap_pings` | `mge.rts.minimap.v1.fn.tick_minimap_pings` | PostLogic (1151) | MinimapPing | MinimapPing | none | O(p) | Decremente le timer des pings actifs |
| `remove_expired_pings` | `mge.rts.minimap.v1.fn.remove_expired_pings` | PostLogic (1152) | MinimapPing | MinimapPing | none | O(p) | Supprime les pings dont remaining <= 0 |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `MinimapPinged` | `mge.rts.minimap.v1.event.minimap_pinged` | `sender: EntityId, ping_type: PingType, position: (f32, f32)` | externe (input joueur) | ui, audio, allies |

---

## 7. Invariants

- `MinimapPing.remaining` est toujours dans [0.0, duration].
- `MinimapEntry.visible` respecte le brouillard de guerre si le plugin fog-of-war est actif.
- Un ping expire est supprime au tick suivant son expiration.
- `MinimapEntry.color` est un RGBA encode en u32.
- Les positions minimap sont toujours dans les bornes [0, minimap_width] x [0, minimap_height].

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `minimap_width` | `u16` | 256 | [64, 512] | Largeur logique de la minimap en pixels |
| `minimap_height` | `u16` | 256 | [64, 512] | Hauteur logique de la minimap en pixels |
| `default_ping_duration` | `f32` | 5.0 | [1.0, 30.0] | Duree par defaut d'un ping en secondes |
| `max_active_pings` | `u16` | 20 | [5, 100] | Nombre max de pings simultanes |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Synchronise les positions monde vers minimap | Ne gere pas le rendu de la minimap (→ ui) |
| Gere les pings temporaires | Ne gere pas la visibilite (→ fog-of-war) |
| Supporte les icones par type d'entite | Ne gere pas les clics sur la minimap (→ input) |
| Gere l'expiration des pings | Ne gere pas la camera (→ ui) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | MinimapEntry, MinimapPing, Position2D |
| Ecrit | MinimapEntry, MinimapPing |
| Emet | MinimapPinged |
| Ne touche jamais | Selection, ProductionQueue, ResourceNode, Building, OrderQueue, FogGrid, TechNode |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-minimap/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.minimap.v1, trait Plugin impl
    ├── components.rs     # MinimapEntry, MinimapPing
    ├── systems.rs        # update_minimap_entries, tick_minimap_pings, remove_expired_pings
    └── events.rs         # MinimapPinged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (update_minimap_entries) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (MinimapIcon, PingType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : position sync, ping tick, ping expiration
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.minimap.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.minimap.v1.component.minimap_entry","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.minimap.v1.component.minimap_ping","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.minimap.v1.fn.update_minimap_entries","k":"s","d":"rts","r":["MinimapEntry","Position2D"],"w":["MinimapEntry"],"e":[],"p":1150,"c":"O(e)"},
  {"i":"mge.rts.minimap.v1.fn.tick_minimap_pings","k":"s","d":"rts","r":["MinimapPing"],"w":["MinimapPing"],"e":[],"p":1151,"c":"O(p)"},
  {"i":"mge.rts.minimap.v1.fn.remove_expired_pings","k":"s","d":"rts","r":["MinimapPing"],"w":["MinimapPing"],"e":[],"p":1152,"c":"O(p)"},
  {"i":"mge.rts.minimap.v1.event.minimap_pinged","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let soldier = world.spawn();
world.insert(soldier, MinimapEntry {
    icon: MinimapIcon::Unit,
    color: 0x00FF00FF,
    visible: true,
    team: 1,
});

let ping = world.spawn();
world.insert(ping, MinimapPing {
    ping_type: PingType::Attack,
    position: (150.0, 80.0),
    duration: 5.0,
    remaining: 5.0,
    sender: player_id,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
