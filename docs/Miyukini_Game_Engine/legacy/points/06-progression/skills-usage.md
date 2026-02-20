# Skills par usage (skill usage)

**Catégorie :** 06. Progression  
**Description :** Compétences montent à l'utilisation ; peuvent baisser.

## Contexte

Dans ce système, les compétences ne montent pas via des points dépensables mais par l'utilisation répétée. Plus le joueur utilise une compétence, plus elle progresse. À l'inverse, l'absence d'utilisation peut entraîner une dégénérescence (skill decay). Inspiré de jeux comme Ultima Online, RuneScape.

**Rôle :** Progression organique, spécialisation naturelle, évite les builds identiques. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Gain par utilisation

- Chaque utilisation réussie accorde des points de progression
- Le gain dépend du [skill gains dégressifs](skill-gains-degressifs.md) : plus la skill est haute, plus les gains sont lents
- Les échecs (cible ratée, interrompu) peuvent donner une fraction réduite ou rien

### Dégénérescence (decay)

| Option | Description |
|--------|-------------|
| Aucune | Les skills ne baissent jamais |
| Lent | -X points/jour si non utilisée |
| Par dépassement | Les skills au-dessus du cap total baissent ; voir [cap-total-skills](cap-total-skills.md) |

### Cap et plafond

Voir [cap-total-skills](cap-total-skills.md) pour le plafond global réparti entre toutes les compétences. Voir [skill-gains-degressifs](skill-gains-degressifs.md) pour la courbe de gain.

---

## Modèle de données / API

```rust
pub struct SkillUsageProgress {
    pub skill_id: SkillId,
    pub level: f32,       // ou entier avec sous-niveaux
    pub xp: u64,
}
```

---

## Exemples

- **UO-style** : Chaque utilisation = gain. 700 pts max répartis. Decay lent au-dessus du cap.
- **RuneScape-style** : XP par action, pas de decay. Compétences 1-99.

---

## Références

- [Cap total skills](cap-total-skills.md)
- [Skill gains dégressifs](skill-gains-degressifs.md)
- [Gain compétences](gain-competences-aptitudes.md)
