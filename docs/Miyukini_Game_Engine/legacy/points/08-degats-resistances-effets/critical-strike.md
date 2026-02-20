# Critical strike (coup critique)

**Catégorie :** 08. Dégâts, résistances et effets  
**Description :** Coup critique ; dégâts amplifiés.

## Contexte

Les coups critiques ont une chance de se déclencher et infligent des dégâts multipliés (ex. x2). La chance et le multiplicateur sont des stats. Lié aux [stats](../05-joueur-personnage/stats.md).

**Rôle :** DPS, builds critique. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Formule

```
si random() < crit_chance : dégâts *= crit_multiplier
crit_multiplier typique : 1.5 à 2.5
```

---

## Références

- [Index 08](_index.md)
- [Stats](../05-joueur-personnage/stats.md)
