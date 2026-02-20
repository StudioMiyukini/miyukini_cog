# mge-gfp-screen-shake

> @id mge.foundation.screen_shake.v1  
> @role plugin  
> @domain foundation  
> @do camera_shake_trauma_decay_intensity  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-screen-shake` |
| @id MSCM | `mge.foundation.screen_shake.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-camera2d` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(c), c = cameras avec shake actif |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ShakeTrauma` | `mge.foundation.screen_shake.v1.component.shake_trauma` | `current: f32, max: f32` | Niveau de trauma actuel (0.0–max). L'intensite du shake est proportionnelle a trauma² |
| `ShakeDecay` | `mge.foundation.screen_shake.v1.component.shake_decay` | `rate: f32` | Taux de decroissance du trauma par tick (unites/tick) |
| `ShakeIntensity` | `mge.foundation.screen_shake.v1.component.shake_intensity` | `offset_x: f32, offset_y: f32, max_offset: f32` | Offset de tremblement calcule. max_offset definit l'amplitude maximale en pixels |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_screen_shake` | `mge.foundation.screen_shake.v1.fn.tick_screen_shake` | 144 | ShakeTrauma, ShakeDecay, Camera2D | ShakeTrauma, ShakeIntensity | ShakeStarted, ShakeEnded | O(c) | Decroit le trauma, calcule un offset aleatoire deterministe a partir de trauma², emet les evenements de debut/fin |

---

## 5. Flux de donnees

```
ShakeRequest (event entree)
       │
       ▼
 ShakeTrauma.current += trauma ajoute
       │
       ├──── ShakeDecay (taux decroissance)
       ├──── ShakeTrauma (trauma²)
       │
       ▼
 ┌──────────────────────────┐
 │ tick_screen_shake         │  Phase 144
 │ (decay trauma, compute   │
 │  offset = trauma² × noise│
 │  × max_offset)           │
 └──────────┬───────────────┘
            │
            ├──→ ShakeIntensity (offset_x, offset_y)
            ├──→ ShakeTrauma (current decremente)
            ├──→ ShakeStarted (si trauma passe de 0 → >0)
            └──→ ShakeEnded (si trauma atteint 0)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ShakeRequest` | `mge.foundation.screen_shake.v1.event.shake_request` | `camera: EntityId, trauma: f32` | Systemes externes (combat, explosion, impact) | `tick_screen_shake` (consomme pour ajouter du trauma) |
| `ShakeStarted` | `mge.foundation.screen_shake.v1.event.shake_started` | `camera: EntityId` | `tick_screen_shake` | Systemes audio, effets visuels, UI |
| `ShakeEnded` | `mge.foundation.screen_shake.v1.event.shake_ended` | `camera: EntityId` | `tick_screen_shake` | Systemes audio, effets visuels, UI |

---

## 7. Invariants

- `ShakeTrauma.current` est toujours dans [0.0, `ShakeTrauma.max`].
- L'intensite du shake est proportionnelle a `trauma²` (non lineaire, sensation naturelle).
- Le pseudo-bruit est deterministe : seede a partir du frame courant et du trauma (reproductible).
- `ShakeDecay.rate` doit etre > 0.0 (sinon le trauma ne decroit jamais).
- `ShakeIntensity.offset_x/y` sont toujours dans [-max_offset, +max_offset].
- `ShakeStarted` est emis une seule fois quand le trauma passe de 0 a une valeur positive.
- `ShakeEnded` est emis une seule fois quand le trauma atteint 0.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_shake_decay` | f32 | 2.0 | ]0.0, +inf[ | Taux de decroissance du trauma par defaut |
| `default_max_offset` | f32 | 10.0 | ]0.0, +inf[ | Amplitude maximale du shake en pixels par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le trauma et sa decroissance | Ne deplace pas la camera (l'offset est lu par le rendu) |
| Calcule un offset de tremblement deterministe | Ne gere pas le suivi de cible (→ follow-camera) |
| Emet des evenements debut/fin de shake | Ne gere pas les limites camera (→ constraint-camera) |
| Consomme les ShakeRequest pour ajouter du trauma | Ne gere pas le rendu ni l'audio |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ShakeTrauma, ShakeDecay, Camera2D |
| Ecrit | ShakeTrauma, ShakeIntensity |
| Emet | ShakeStarted, ShakeEnded |
| Ne touche jamais | Transform2D, Viewport, Velocity2D, Collider, FollowTarget, CameraBounds |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-screen-shake/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.screen_shake.v1, trait Plugin impl
    ├── components.rs     # ShakeTrauma, ShakeDecay, ShakeIntensity
    ├── systems.rs        # tick_screen_shake
    └── events.rs         # ShakeRequest, ShakeStarted, ShakeEnded
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] Parametres GCL `default_shake_decay` et `default_max_offset` documentes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : decay trauma, trauma² → offset, ShakeStarted/ShakeEnded emis, determinisme bruit, clamp offset
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.screen_shake.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.component.shake_trauma","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.component.shake_decay","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.component.shake_intensity","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.fn.tick_screen_shake","k":"s","d":"foundation","r":["ShakeTrauma","ShakeDecay","Camera2D"],"w":["ShakeTrauma","ShakeIntensity"],"e":["ShakeStarted","ShakeEnded"],"p":144,"c":"O(c)"},
  {"i":"mge.foundation.screen_shake.v1.event.shake_request","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.event.shake_started","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.screen_shake.v1.event.shake_ended","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let camera = world.spawn();
world.insert(camera, Transform2D {
    x: 400.0, y: 300.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(camera, Camera2D { zoom: 1.0, rotation: 0.0, active: true });
world.insert(camera, ShakeTrauma { current: 0.0, max: 1.0 });
world.insert(camera, ShakeDecay { rate: 2.0 });
world.insert(camera, ShakeIntensity {
    offset_x: 0.0, offset_y: 0.0, max_offset: 10.0,
});

events.emit(ShakeRequest { camera, trauma: 0.5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-camera2d](mge-gfp-camera2d.md) | Camera de base (prerequis) |
| [mge-gfp-constraint-camera](mge-gfp-constraint-camera.md) | Limites camera (s'execute avant shake) |
