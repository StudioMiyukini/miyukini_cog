# Dégâts en pourcentage

**Catégorie :** 08. Dégâts, résistances et effets  
**Description :** Pourcentage de vie enlevé en un coup.

## Contexte

Certaines attaques ou effets enlèvent un % des PV max de la cible plutôt qu'un montant fixe. Utile pour les boss à haute vie. Peut avoir un cap (ex. max 10 % des PV par coup).

**Rôle :** Équilibrage boss, anti-tank. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

```
dégâts = pv_max_cible * (pourcent / 100)
cap optionnel : min(dégâts, pv_max * max_percent / 100)
```

---

## Références

- [Index 08](_index.md)
