# Chance de toucher

**Catégorie :** 07. Combat  
**Description :** Formule attaque vs défense ; précision vs esquive.

## Contexte

La chance de toucher (hit chance, accuracy) détermine si une attaque réussit ou est esquivée. Elle oppose la précision de l'attaquant à l'esquive du défenseur. Voir [esquive-flee](esquive-flee.md) pour la réduction des dégâts.

**Rôle :** Équilibrage combat, importance des stats. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Formule typique

```
hit_chance = base + (precision_att - esquive_def) / facteur
hit_chance = clamp(hit_chance, min_hit, max_hit)
```

- **Base** : 75–90 % (toujours une chance min/max)
- **Précision** : Stat ou dérivée
- **Esquive** : Stat du défenseur
- **Facteur** : Pour aplatir la courbe

---

## Modèle de données / API

```rust
fn hit_chance(attacker: &Character, defender: &Character) -> f32;
fn roll_hit(attacker: &Character, defender: &Character, rng: &mut Rng) -> bool;
```

---

## Références

- [Index 07](_index.md)
- [Esquive flee](esquive-flee.md)
- [Stats](../05-joueur-personnage/stats.md)
