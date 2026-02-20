# mge-gs-diplomacy

> @id mge.gs.diplomacy.v1  
> @role plugin  
> @domain grand-strategy  
> @do manage_diplomatic_relations_treaties_alliances  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gs-diplomacy` |
| @id MSCM | `mge.gs.diplomacy.v1` |
| Domaine | grand-strategy |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-social-faction`, `mge-social-reputation` |
| Hot path | Non (diplomatie evaluee par tick de jour, pas par frame) |
| Headless safe | Oui |
| Complexite globale | O(f^2) ou f=nombre de factions |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `DiplomaticRelation` | `War, Truce, Neutral, NonAggression, Alliance, Vassal, Overlord` | Nature de la relation diplomatique |
| `TreatyType` | `Peace, Trade, Military, Vassalage, Marriage, NonAggression` | Type de traite entre factions |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `DiplomaticStance` | `mge.gs.diplomacy.v1.component.diplomatic_stance` | `faction_a: EntityId, faction_b: EntityId, relation: DiplomaticRelation, opinion: f32, tension: f32` | Relation diplomatique entre deux factions. opinion [-100, 100], tension [0, 100] |
| `Treaty` | `mge.gs.diplomacy.v1.component.treaty` | `treaty_type: TreatyType, parties: (EntityId, EntityId), duration: f32, remaining: f32, terms: Vec<String>` | Traite actif entre deux parties avec duree et termes |
| `DiplomaticAction` | `mge.gs.diplomacy.v1.component.diplomatic_action` | `actor: EntityId, target: EntityId, action_type: TreatyType, acceptance_chance: f32` | Action diplomatique en attente d'evaluation |

---

## 4. Formules

```
opinion_delta   = base_opinion_change * modifier
opinion_new     = clamp(opinion + opinion_delta, -100.0, 100.0)

tension_delta   = provocation_value - diplomacy_cooldown
tension_new     = clamp(tension + tension_delta, 0.0, 100.0)

acceptance      = base_acceptance + opinion_bonus + power_ratio_bonus - war_exhaustion_malus
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_treaties` | `mge.gs.diplomacy.v1.fn.tick_treaties` | Logic (1200) | Treaty | Treaty | TreatyExpired | O(t) | Decremente la duree restante des traites actifs |
| `process_diplomatic_action` | `mge.gs.diplomacy.v1.fn.process_diplomatic_action` | Logic (1201) | DiplomaticAction, DiplomaticStance | DiplomaticStance, Treaty | WarDeclared, PeaceSigned, AllianceFormed | O(a) | Evalue et applique les actions diplomatiques en attente |
| `update_opinion` | `mge.gs.diplomacy.v1.fn.update_opinion` | Logic (1202) | DiplomaticStance | DiplomaticStance | none | O(f^2) | Applique les modificateurs d'opinion (declin naturel, evenements) |
| `check_treaty_expiration` | `mge.gs.diplomacy.v1.fn.check_treaty_expiration` | Logic (1203) | Treaty | Treaty, DiplomaticStance | AllianceBroken, TreatyExpired | O(t) | Supprime les traites expires et met a jour les stances |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `WarDeclared` | `mge.gs.diplomacy.v1.event.war_declared` | `aggressor: EntityId, defender: EntityId, casus_belli: Option<u32>` | `process_diplomatic_action` | military, cb, ui |
| `PeaceSigned` | `mge.gs.diplomacy.v1.event.peace_signed` | `parties: (EntityId, EntityId), terms: Vec<String>` | `process_diplomatic_action` | military, trade, ui |
| `AllianceFormed` | `mge.gs.diplomacy.v1.event.alliance_formed` | `parties: (EntityId, EntityId), duration: f32` | `process_diplomatic_action` | military, ui |
| `AllianceBroken` | `mge.gs.diplomacy.v1.event.alliance_broken` | `parties: (EntityId, EntityId), reason: String` | `check_treaty_expiration` | military, opinion, ui |
| `TreatyExpired` | `mge.gs.diplomacy.v1.event.treaty_expired` | `treaty_type: TreatyType, parties: (EntityId, EntityId)` | `tick_treaties` | ui |

---

## 7. Invariants

- `DiplomaticStance.opinion` est toujours dans [-100.0, 100.0].
- `DiplomaticStance.tension` est toujours dans [0.0, 100.0].
- Un traite de paix implique `relation != War` entre les parties.
- `Treaty.remaining` ne devient jamais negatif (min 0.0).
- Deux factions ne peuvent avoir qu'un seul `DiplomaticStance` actif (paire unique).
- Une declaration de guerre annule automatiquement tous les traites positifs entre les parties.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `opinion_decay_rate` | `f32` | 0.5 | [0.0, 5.0] | Declin naturel de l'opinion par tick vers 0 |
| `truce_duration` | `f32` | 60.0 | [10.0, 365.0] | Duree par defaut d'une treve (en ticks/jours) |
| `alliance_opinion_bonus` | `f32` | 25.0 | [0.0, 50.0] | Bonus d'opinion pour les allies |
| `war_declaration_tension` | `f32` | 50.0 | [10.0, 100.0] | Tension ajoutee par une declaration de guerre |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les relations et traites diplomatiques | Ne gere pas les factions (→ social-faction) |
| Evalue les actions diplomatiques | Ne gere pas le combat (→ military, rpg-combat) |
| Suit l'opinion et la tension | Ne gere pas la reputation (→ social-reputation) |
| Expire et annule les traites | Ne gere pas les casus belli (→ cb) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DiplomaticStance, Treaty, DiplomaticAction |
| Ecrit | DiplomaticStance, Treaty |
| Emet | WarDeclared, PeaceSigned, AllianceFormed, AllianceBroken, TreatyExpired |
| Ne touche jamais | Treasury, Army, Population, Province, TradeRoute, CasusBelli, Religion, Culture |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gs-diplomacy/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.gs.diplomacy.v1, trait Plugin impl
    ├── components.rs     # DiplomaticStance, Treaty, DiplomaticAction
    ├── systems.rs        # tick_treaties, process_diplomatic_action, update_opinion, check_treaty_expiration
    └── events.rs         # WarDeclared, PeaceSigned, AllianceFormed, AllianceBroken, TreatyExpired
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 5 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (DiplomaticRelation, TreatyType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : treaty tick, diplomatic action, opinion update, expiration
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.gs.diplomacy.v1","k":"p","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.component.diplomatic_stance","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.component.treaty","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.component.diplomatic_action","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.fn.tick_treaties","k":"s","d":"grand-strategy","r":["Treaty"],"w":["Treaty"],"e":["TreatyExpired"],"p":1200,"c":"O(t)"},
  {"i":"mge.gs.diplomacy.v1.fn.process_diplomatic_action","k":"s","d":"grand-strategy","r":["DiplomaticAction","DiplomaticStance"],"w":["DiplomaticStance","Treaty"],"e":["WarDeclared","PeaceSigned","AllianceFormed"],"p":1201,"c":"O(a)"},
  {"i":"mge.gs.diplomacy.v1.fn.update_opinion","k":"s","d":"grand-strategy","r":["DiplomaticStance"],"w":["DiplomaticStance"],"e":[],"p":1202,"c":"O(f^2)"},
  {"i":"mge.gs.diplomacy.v1.fn.check_treaty_expiration","k":"s","d":"grand-strategy","r":["Treaty"],"w":["Treaty","DiplomaticStance"],"e":["AllianceBroken","TreatyExpired"],"p":1203,"c":"O(t)"},
  {"i":"mge.gs.diplomacy.v1.event.war_declared","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.event.peace_signed","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.event.alliance_formed","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.event.alliance_broken","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.diplomacy.v1.event.treaty_expired","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let france = world.spawn();
let england = world.spawn();

let stance = world.spawn();
world.insert(stance, DiplomaticStance {
    faction_a: france,
    faction_b: england,
    relation: DiplomaticRelation::Neutral,
    opinion: -15.0,
    tension: 30.0,
});

let treaty = world.spawn();
world.insert(treaty, Treaty {
    treaty_type: TreatyType::NonAggression,
    parties: (france, england),
    duration: 120.0,
    remaining: 120.0,
    terms: vec!["no_border_incident".into()],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Grand Strategy - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
