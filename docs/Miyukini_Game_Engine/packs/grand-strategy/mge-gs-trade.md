# mge-gs-trade

> @id mge.gs.trade.v1  
> @role plugin  
> @domain grand-strategy  
> @do manage_trade_routes_exchanges_embargo  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gs-trade` |
| @id MSCM | `mge.gs.trade.v1` |
| Domaine | grand-strategy |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gs-economy` |
| Hot path | Non (commerce calcule par tick de jour) |
| Headless safe | Oui |
| Complexite globale | O(r) ou r=routes commerciales actives |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TradeStatus` | `Active, Blocked, Embargoed` | Etat d'une route commerciale |
| `TradeGood` | `Grain, Cloth, Iron, Spice, Luxury, Custom(u16)` | Type de bien echange |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TradeRoute` | `mge.gs.trade.v1.component.trade_route` | `origin: EntityId, destination: EntityId, goods: Vec<TradeGood>, value: f32, status: TradeStatus` | Route commerciale entre deux provinces/factions |
| `TradeAgreement` | `mge.gs.trade.v1.component.trade_agreement` | `parties: (EntityId, EntityId), goods: Vec<TradeGood>, duration: f32, remaining: f32` | Accord commercial bilateral avec duree |
| `Embargo` | `mge.gs.trade.v1.component.embargo` | `enforcer: EntityId, target: EntityId, remaining: f32, severity: f32` | Embargo commercial. severity [0, 1] = pourcentage de commerce bloque |

---

## 4. Formules

```
route_income    = base_value * goods_multiplier * (1.0 - embargo_severity) * efficiency
trade_income    = sum(route.route_income for active routes)

embargo_effect  = severity * affected_routes_value
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_trade_routes` | `mge.gs.trade.v1.fn.tick_trade_routes` | Logic (1220) | TradeRoute, TradeAgreement | TradeRoute | none | O(r) | Met a jour les routes actives et leur valeur |
| `process_trade_income` | `mge.gs.trade.v1.fn.process_trade_income` | Logic (1221) | TradeRoute, Treasury | Treasury | TradeRouteEstablished | O(r) | Ajoute les revenus commerciaux au tresor |
| `apply_embargo` | `mge.gs.trade.v1.fn.apply_embargo` | Logic (1222) | Embargo, TradeRoute | TradeRoute | EmbargoImposed, EmbargoLifted | O(e) | Applique les embargos actifs sur les routes affectees |
| `check_trade_disruption` | `mge.gs.trade.v1.fn.check_trade_disruption` | Logic (1223) | TradeRoute, DiplomaticStance | TradeRoute | TradeRouteBroken | O(r) | Detecte les routes rompues par guerre ou embargo total |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TradeRouteEstablished` | `mge.gs.trade.v1.event.trade_route_established` | `origin: EntityId, destination: EntityId, value: f32` | `process_trade_income` | economy, ui |
| `TradeRouteBroken` | `mge.gs.trade.v1.event.trade_route_broken` | `route: EntityId, reason: String` | `check_trade_disruption` | economy, diplomacy, ui |
| `EmbargoImposed` | `mge.gs.trade.v1.event.embargo_imposed` | `enforcer: EntityId, target: EntityId, severity: f32` | `apply_embargo` | diplomacy, ui |
| `EmbargoLifted` | `mge.gs.trade.v1.event.embargo_lifted` | `enforcer: EntityId, target: EntityId` | `apply_embargo` | diplomacy, ui |

---

## 7. Invariants

- `TradeRoute.value` est toujours >= 0.0.
- `Embargo.severity` est toujours dans [0.0, 1.0].
- Une route `Blocked` ne genere aucun revenu.
- Un embargo expire (remaining <= 0) est supprime automatiquement.
- Deux factions en guerre n'ont aucune route `Active` entre elles.
- `TradeAgreement.remaining` ne devient jamais negatif.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `base_trade_value` | `f32` | 10.0 | [1.0, 100.0] | Valeur de base d'une route commerciale par tick |
| `embargo_default_duration` | `f32` | 60.0 | [10.0, 365.0] | Duree par defaut d'un embargo (ticks/jours) |
| `war_trade_block` | `bool` | true | — | Bloque automatiquement le commerce en cas de guerre |
| `goods_value_multiplier` | `f32` | 1.0 | [0.5, 3.0] | Multiplicateur global de valeur des biens |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les routes commerciales et leur valeur | Ne gere pas le tresor directement (→ economy) |
| Applique les embargos | Ne gere pas la diplomatie (→ diplomacy) |
| Detecte les ruptures de routes | Ne gere pas les navires/convois (→ v2) |
| Calcule les revenus commerciaux | Ne gere pas la production (→ economy) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | TradeRoute, TradeAgreement, Embargo, Treasury, DiplomaticStance |
| Ecrit | TradeRoute, Treasury |
| Emet | TradeRouteEstablished, TradeRouteBroken, EmbargoImposed, EmbargoLifted |
| Ne touche jamais | Army, Population, Province, Religion, Culture, CasusBelli, Decision |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gs-trade/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.gs.trade.v1
    ├── components.rs     # TradeRoute, TradeAgreement, Embargo
    ├── systems.rs        # tick_trade_routes, process_trade_income, apply_embargo, check_trade_disruption
    └── events.rs         # TradeRouteEstablished, TradeRouteBroken, EmbargoImposed, EmbargoLifted
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
- [ ] 3 composants avec @id et @fields
- [ ] 4 systemes avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements avec @id et @fields
- [ ] 2 enumerations (TradeStatus, TradeGood)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : trade income, embargo, route disruption
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.gs.trade.v1","k":"p","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.component.trade_route","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.component.trade_agreement","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.component.embargo","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.fn.tick_trade_routes","k":"s","d":"grand-strategy","r":["TradeRoute","TradeAgreement"],"w":["TradeRoute"],"e":[],"p":1220,"c":"O(r)"},
  {"i":"mge.gs.trade.v1.fn.process_trade_income","k":"s","d":"grand-strategy","r":["TradeRoute","Treasury"],"w":["Treasury"],"e":["TradeRouteEstablished"],"p":1221,"c":"O(r)"},
  {"i":"mge.gs.trade.v1.fn.apply_embargo","k":"s","d":"grand-strategy","r":["Embargo","TradeRoute"],"w":["TradeRoute"],"e":["EmbargoImposed","EmbargoLifted"],"p":1222,"c":"O(e)"},
  {"i":"mge.gs.trade.v1.fn.check_trade_disruption","k":"s","d":"grand-strategy","r":["TradeRoute","DiplomaticStance"],"w":["TradeRoute"],"e":["TradeRouteBroken"],"p":1223,"c":"O(r)"},
  {"i":"mge.gs.trade.v1.event.trade_route_established","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.event.trade_route_broken","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.event.embargo_imposed","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.trade.v1.event.embargo_lifted","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let silk_road = world.spawn();
world.insert(silk_road, TradeRoute {
    origin: persia,
    destination: rome,
    goods: vec![TradeGood::Spice, TradeGood::Cloth],
    value: 25.0,
    status: TradeStatus::Active,
});

let embargo = world.spawn();
world.insert(embargo, Embargo {
    enforcer: rome,
    target: carthage,
    remaining: 60.0,
    severity: 0.8,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Grand Strategy - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
