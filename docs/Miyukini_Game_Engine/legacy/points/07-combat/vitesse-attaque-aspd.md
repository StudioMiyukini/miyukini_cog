# Vitesse d'attaque (ASPD)

**Catégorie :** 07. Combat  
**Description :** Vitesse d'attaque ; cap ; bonus.

## Contexte

L'ASPD (Attack Speed) détermine la cadence des attaques : plus elle est élevée, plus le personnage attaque rapidement. Un cap évite les builds extrêmes. Les bonus viennent des stats, équipements, buffs.

**Rôle :** DPS, feel du combat. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Formule

```
delay_ms = base_delay / (1 + aspd_bonus)
aspd_bonus = sum(bonus from stats, gear, buffs)
```

### Cap

- Ex. : 200 % ASPD max (attaque 2x plus rapide que la base)
- Ou : délai minimum (ex. 100 ms entre deux coups)

---

## Références

- [Index 07](_index.md)
- [Auto-attaque base](auto-attaque-base.md)
