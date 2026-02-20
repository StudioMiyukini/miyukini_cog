# Tables de loot

**Catégorie :** 10. Loot et tables  
**Description :** Par type de monstre ; rareté.

## Contexte

Les tables de loot définissent quels objets un monstre peut faire tomber. Chaque type de monstre a une ou plusieurs tables, avec des probabilités et raretés. Lié au [ramassage](ramassage.md) et aux [droits de loot](droits-loot.md).

**Rôle :** Génération des récompenses combat. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Structure

- Table = liste d'entrées (objet_id, chance, min/max quantité)
- Rareté : commun, rare, épique, légendaire (modificateur chance)
- Roll par kill : 1 à N rolls selon le monstre

---

## Références

- [Index 10](_index.md)
- [Ramassage](ramassage.md)
- [Bosses](bosses.md)
