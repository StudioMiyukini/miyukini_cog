# Vol de vie et mana

**Catégorie :** 08. Dégâts, résistances et effets  
**Description :** Sur les coups.

## Contexte

Le lifesteal (vol de vie) et le manasteal restaurent des ressources au lanceur en fonction des dégâts infligés. Typiquement un % des dégâts. Peut être limité par entrée (cap par coup).

**Rôle :** Survie, sustain. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

```
vie_recup = dégâts * lifesteal_percent / 100
mana_recup = dégâts * manasteal_percent / 100
```

---

## Références

- [Index 08](_index.md)
