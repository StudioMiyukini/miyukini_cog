# Saisons et battle pass

**Catégorie :** 06. Progression  
**Description :** Contenu limité dans le temps.

## Contexte

Les saisons et le battle pass offrent une progression temporelle : des récompenses échelonnées sur une période (ex. 3 mois). Le joueur progresse via des points de saison (XP, objectifs) et débloque des paliers (gratuit et/ou premium).

**Rôle :** Rétention, monétisation optionnelle, contenu frais périodique. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Structure

| Élément | Description |
|--------|-------------|
| Saison | Période (date début, date fin) |
| Paliers | 50–100 niveaux typiquement |
| Voie gratuite | Récompenses sans achat |
| Voie premium | Récompenses additionnelles (achat pass) |

### Progression

- XP de saison par quêtes, daily, gameplay
- Chaque palier débloque les récompenses correspondantes

---

## Modèle de données / API

```rust
pub struct Season {
    pub id: SeasonId,
    pub start: DateTime,
    pub end: DateTime,
    pub tiers: Vec<SeasonTier>,
}

pub struct SeasonTier {
    pub level: u32,
    pub free_reward: Option<Reward>,
    pub premium_reward: Option<Reward>,
}
```

---

## Références

- [Index 06](_index.md)
- [Achievements](achievements-succes.md)
