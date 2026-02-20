# Aggro et menace

**Catégorie :** 07. Combat  
**Description :** Gestion de la menace ; priorité de ciblage des ennemis.

## Contexte

L'aggro (agro, threat) détermine quelle cible les ennemis attaquent. Chaque action (dégâts, soins, compétences de menace) génère de la menace. L'ennemi cible le joueur avec la menace la plus haute. Voir [rôles](roles.md) (tank = main aggro).

**Rôle :** Combat de groupe, rôle tank. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Génération de menace

- Dégâts : 1:1 (1 dégât = 1 menace)
- Soins : 0.5:1 ou variable (soigner génère de l'aggro)
- Compétences taunt : fixe ou multiplier

### Perte d'aggro

- Cible meurt
- Drop de l'aggro après X secondes sans action

---

## Références

- [Index 07](_index.md)
- [Rôles](roles.md)
- [Officiers alliés](officiers-allies.md)
