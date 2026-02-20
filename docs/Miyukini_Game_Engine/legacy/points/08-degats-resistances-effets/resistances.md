# Résistances

**Catégorie :** 08. Dégâts, résistances et effets  
**Description :** Feu, froid, éclair, poison, physique, magique ; cap ; immunités.

## Contexte

Les résistances réduisent les dégâts d'un type donné (élémental ou physique). Chaque entité peut avoir des résistances par élément. Un cap évite les réductions totales. Voir [éléments](elements.md) et [immunités](immunites.md).

**Rôle :** Équilibrage, builds spécialisés. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Types de résistance

| Type | Description |
|------|-------------|
| Physique | Armes, coups |
| Magique | Sorts génériques |
| Feu, Froid, Éclair | Éléments |
| Poison | DoT, venin |
| Saint, Obscur | Éléments avancés |

### Formule

```
dégâts_finaux = dégâts_base * (1 - min(résistance, cap) / 100)
```

Cap typique : 75–90 %.

---

## Modèle de données / API

```rust
pub struct Resistances {
    pub physical: i32,
    pub fire: i32,
    pub cold: i32,
    pub lightning: i32,
    pub poison: i32,
    pub holy: i32,
    pub dark: i32,
}
```

---

## Références

- [Index 08](_index.md)
- [Éléments](elements.md)
- [Immunités](immunites.md)
