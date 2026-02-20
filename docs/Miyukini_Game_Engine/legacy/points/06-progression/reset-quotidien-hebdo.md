# Reset quotidien et hebdomadaire

**Catégorie :** 06. Progression  
**Description :** Limites par jour/semaine.

## Contexte

Les resets quotidiens et hebdomadaires définissent des limites temporelles sur certaines actions : nombre de tentatives de donjon, récompenses, quêtes journalières. À minuit (ou heure configurée), les compteurs sont réinitialisés.

**Rôle :** Limiter le farming, équilibrer la progression, créer des habitudes de jeu. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Périodes

| Période | Définition | Exemple |
|---------|------------|---------|
| Quotidien | 24 h, reset à 00:00 | Donjons 3x/jour |
| Hebdo | 7 jours, reset lundi | Raid 1x/semaine |

### Fuseau horaire

- Serveur : fixé (ex. UTC)
- Client : afficher en local

### Ce qui peut être limité

- Tentatives de donjon
- Récompenses de quêtes journalières
- Reset de talents gratuit
- Achats limités

---

## Modèle de données / API

```rust
pub struct DailyLimit {
    pub action_id: String,
    pub count: u32,
    pub last_reset: DateTime,
}

fn is_limit_reached(character: &Character, action_id: &str) -> bool;
fn reset_if_due(character: &mut Character);
```

---

## Références

- [Index 06](_index.md)
- [Gestion temps](../23-systeme/gestion-temps.md)
