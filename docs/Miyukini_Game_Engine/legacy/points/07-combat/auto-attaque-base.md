# Auto-attaque de base

**Catégorie :** 07. Combat  
**Description :** Attaque automatique ; portée ; cadence ; dégâts.

## Contexte

L'auto-attaque (basic attack) est l'attaque par défaut lorsqu'aucune compétence n'est utilisée. Elle a une portée, une cadence (attacks per second) et inflige des dégâts. Voir [vitesse-attaque-aspd](vitesse-attaque-aspd.md) et [chance-toucher](chance-toucher.md).

**Rôle :** Dégâts de fond, DPS de base. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Portée

- Mélé : 1–2 tiles
- À distance : Selon l'arme (arc, arbalète)

### Cadence

Liée à l'ASPD (vitesse d'attaque). Voir [vitesse-attaque-aspd](vitesse-attaque-aspd.md).

### Dégâts

- Formule : `(arme_dmg + stat_bonus) * modificateurs`
- Modificateurs : taille, race ; voir [modificateurs-taille](modificateurs-taille.md), [modificateurs-race](modificateurs-race.md).

---

## Modèle de données / API

```rust
pub struct BasicAttackConfig {
    pub range: f32,
    pub base_delay_ms: u32,
}
```

---

## Références

- [Index 07](_index.md)
- [Vitesse attaque ASPD](vitesse-attaque-aspd.md)
- [Chance de toucher](chance-toucher.md)
