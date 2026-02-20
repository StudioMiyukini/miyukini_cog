# Titres

**Catégorie :** 06. Progression  
**Description :** Succès débloqués ; affichage sous le nom.

## Contexte

Les titres sont des labels affichés sous ou à côté du nom du personnage (ex. "Légende", "Explorateur"). Ils sont généralement débloqués via les [achievements](achievements-succes.md) et offrent une personnalisation cosmétique et un prestige.

**Rôle dans le moteur :** Différenciation visuelle, récompense de progression, expression du joueur. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Affichage

| Emplacement | Format |
|-------------|--------|
| Sous le nom | "[Nom] le [Titre]" ou "Nom - Titre" |
| Tooltip | Au survol du joueur |
| Liste | Écran personnage, sélection |

### Un titre actif

Le joueur peut n'avoir qu'un seul titre affiché à la fois (ou aucun). Changement via menu ou raccourci.

### Déblocage

- Achievements
- Quêtes
- Niveau
- Achat boutique (si applicable)

---

## Modèle de données / API

```rust
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub unlock_source: TitleUnlockSource,
}

pub enum TitleUnlockSource {
    Achievement(AchievementId),
    Quest(QuestId),
    Level(u32),
}
```

---

## Références

- [Index 06](_index.md)
- [Achievements](achievements-succes.md)
