# Skill gains dégressifs

**Catégorie :** 06. Progression  
**Description :** Plus une skill est haute, plus les gains sont lents.

## Contexte

Les gains dégressifs signifient que la progression d'une compétence ralentit à mesure qu'elle monte. Passer de niveau 1 à 2 est rapide ; passer de 98 à 99 prend beaucoup plus d'utilisations ou de points. Cela évite les max trop rapides et valorise la maîtrise.

**Rôle :** Courbe de progression réaliste, prestige des hauts niveaux. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Formule typique

```
xp_requis(niveau) = base * niveau^exposant
gain_per_use = base_gain / (1 + k * niveau)
```

Plus le niveau est élevé, plus `gain_per_use` diminue.

### Paramètres

| Paramètre | Effet |
|-----------|-------|
| Exposant | Plus élevé = courbe plus raide |
| k | Facteur de réduction par niveau |

---

## Modèle de données

```rust
fn xp_required_for_skill_level(level: u32, config: &SkillCurveConfig) -> u64;
fn effective_gain(level: u32, base_gain: f32, config: &SkillCurveConfig) -> f32;
```

---

## Références

- [Index 06](_index.md)
- [Skills usage](skills-usage.md)
- [Cap total skills](cap-total-skills.md)
