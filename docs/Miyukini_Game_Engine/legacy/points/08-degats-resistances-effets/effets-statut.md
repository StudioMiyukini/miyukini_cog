# Effets de statut (buffs, debuffs)

**Catégorie :** 08. Dégâts, résistances et effets  
**Description :** Buffs, debuffs ; stack ; durée ; dispel.

## Contexte

Les effets de statut modifient les stats ou le comportement d'une entité pendant une durée. Buffs = effets positifs ; debuffs = négatifs. Le stack définit si plusieurs instances s'empilent. Le dispel retire les effets.

**Rôle :** Profondeur du combat. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Stack

| Mode | Description |
|------|-------------|
| Refresh | Nouvelle application reset la durée |
| Stack | Plusieurs instances, effets cumulés |
| Cap | Max N stacks |

### Dispel

- Dispel tous les debuffs
- Dispel un type (magie, poison)
- Purge une cible

---

## Références

- [Index 08](_index.md)
- [Crowd control](crowd-control-cc.md)
