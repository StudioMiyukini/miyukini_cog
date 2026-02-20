# Jobs et changement de classe

**Catégorie :** 06. Progression  
**Description :** Quêtes de changement ; évolution de carrière.

## Contexte

Le système de jobs (classes) et de changement de classe permet au personnage d'évoluer : guerrier → paladin, mage → sorcier. Le changement s'effectue souvent via une quête ou un PNJ, avec conservation optionnelle des compétences de l'ancienne classe.

**Rôle :** Diversité de gameplay, évolution narrative, rejouabilité. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Prérequis au changement

| Prérequis | Exemple |
|-----------|---------|
| Niveau | Niveau 50 guerrier |
| Quête | "L'épreuve du paladin" |
| Objet | "Parchemin de promotion" |
| Stat | STR 30, VIT 20 |

### Conséquences

- Nouveaux sorts/compétences
- Arbre de talents différent
- Stats de base modifiées
- Ancienne classe : désactivée mais mémorisée (ou perdue)

---

## Modèle de données / API

```rust
pub struct JobChange {
    pub from_class: ClassId,
    pub to_class: ClassId,
    pub prereqs: Vec<JobPrereq>,
}
```

---

## Références

- [Index 06](_index.md)
- [Gain compétences](gain-competences-aptitudes.md)
- [Quêtes](../19-quetes-missions/quetes.md)
