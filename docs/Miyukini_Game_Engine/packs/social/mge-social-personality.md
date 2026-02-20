# mge-social-personality

> @id mge.social.personality.v1  
> @role plugin  
> @domain social  
> @do manage_personality_traits_preferences_compatibility  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-social-personality` |
| @id MSCM | `mge.social.personality.v1` |
| Domaine | social |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (evaluations a la demande ou periodiques) |
| Headless safe | Oui |
| Complexite globale | O(n^2) pour calcul de compatibilite (borne par rayon social) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TraitAxis` | `Openness, Conscientiousness, Extraversion, Agreeableness, Neuroticism` | Axe Big Five |
| `PreferenceType` | `FoodPreference, ActivityPreference, SocialPreference, LocationPreference` | Type de preference |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `PersonalityTraits` | `mge.social.personality.v1.component.personality_traits` | `openness: f32, conscientiousness: f32, extraversion: f32, agreeableness: f32, neuroticism: f32` | Traits Big Five. Valeurs dans [-1.0, 1.0] |
| `Preferences` | `mge.social.personality.v1.component.preferences` | `entity: EntityId, entries: Vec<PreferenceEntry>` | Liste des preferences avec poids. PreferenceEntry = (PreferenceType, id: u32, weight: f32) |

---

## 4. Formules

```
compatibility(a, b) = 1.0 - (
    |a.openness - b.openness| +
    |a.conscientiousness - b.conscientiousness| +
    |a.extraversion - b.extraversion| +
    |a.agreeableness - b.agreeableness| +
    |a.neuroticism - b.neuroticism|
) / 10.0

personality_modifier(trait_value, axis) = 1.0 + trait_value * axis_weight
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_compatibility` | `mge.social.personality.v1.fn.compute_compatibility` | Logic (1050) | PersonalityTraits | PersonalityTraits | CompatibilityEvaluated | O(p^2) | Calcule la compatibilite entre paires proches. Borne par rayon |
| `apply_personality_modifier` | `mge.social.personality.v1.fn.apply_personality_modifier` | Logic (1051) | PersonalityTraits | PersonalityTraits | none | O(n) | Applique les modificateurs de personnalite aux interactions sociales |
| `evaluate_preference` | `mge.social.personality.v1.fn.evaluate_preference` | Logic (1052) | Preferences | Preferences | none | O(n*p) | Evalue les preferences pour tri des choix par l'AI |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CompatibilityEvaluated` | `mge.social.personality.v1.event.compatibility_evaluated` | `entity_a: EntityId, entity_b: EntityId, score: f32` | `compute_compatibility` | relationship, ai, gossip |

---

## 7. Invariants

- `PersonalityTraits` valeurs toujours dans [-1.0, 1.0] pour chaque axe.
- La compatibilite est symetrique : compatibility(A,B) == compatibility(B,A).
- Les traits ne changent pas en cours de partie (immuables apres generation).
- `Preferences.entries` est trie par weight decroissant apres `evaluate_preference`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `compatibility_radius` | `u32` | 50 | [5, 500] | Nombre max de paires evaluees par tick |
| `trait_influence_weight` | `f32` | 0.3 | [0.0, 1.0] | Poids des traits dans les modificateurs sociaux |
| `max_preferences` | `u16` | 20 | [5, 100] | Nombre max de preferences par entite |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke les traits de personnalite | Ne modifie pas les relations (-> relationship) |
| Calcule la compatibilite entre entites | Ne gere pas les besoins (-> need) |
| Evalue les preferences | Ne decide pas des actions (-> ai) |
| Fournit des modificateurs sociaux | Ne gere pas les factions (-> faction) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | PersonalityTraits, Preferences |
| Ecrit | PersonalityTraits, Preferences |
| Emet | CompatibilityEvaluated |
| Ne touche jamais | Relationship, Faction, Reputation, NeedSet, Schedule, GossipMemory, Household |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-social-personality/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.social.personality.v1
    ├── components.rs     # PersonalityTraits, Preferences
    ├── systems.rs        # compute_compatibility, apply_personality_modifier, evaluate_preference
    └── events.rs         # CompatibilityEvaluated
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
- [ ] 1 evenement dans `events.rs`
- [ ] 2 enumerations (TraitAxis, PreferenceType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : compatibility calc, modifier application, preference sort
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.social.personality.v1","k":"p","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.personality.v1.component.personality_traits","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.personality.v1.component.preferences","k":"d","d":"social","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.social.personality.v1.fn.compute_compatibility","k":"s","d":"social","r":["PersonalityTraits"],"w":["PersonalityTraits"],"e":["CompatibilityEvaluated"],"p":1050,"c":"O(p^2)"},
  {"i":"mge.social.personality.v1.fn.apply_personality_modifier","k":"s","d":"social","r":["PersonalityTraits"],"w":["PersonalityTraits"],"e":[],"p":1051,"c":"O(n)"},
  {"i":"mge.social.personality.v1.fn.evaluate_preference","k":"s","d":"social","r":["Preferences"],"w":["Preferences"],"e":[],"p":1052,"c":"O(n*p)"},
  {"i":"mge.social.personality.v1.event.compatibility_evaluated","k":"e","d":"social","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, PersonalityTraits {
    openness: 0.7,
    conscientiousness: 0.3,
    extraversion: -0.2,
    agreeableness: 0.8,
    neuroticism: -0.5,
});
world.insert(entity, Preferences {
    entity,
    entries: vec![
        PreferenceEntry { pref_type: PreferenceType::FoodPreference, id: 42, weight: 0.9 },
        PreferenceEntry { pref_type: PreferenceType::ActivityPreference, id: 7, weight: 0.6 },
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
