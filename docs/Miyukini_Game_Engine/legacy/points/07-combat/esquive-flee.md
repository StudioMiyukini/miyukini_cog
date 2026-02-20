# Esquive et flee

**Catégorie :** 07. Combat  
**Description :** Formule vs précision ; réduction des dégâts.

## Contexte

L'esquive (dodge) peut signifier soit une attaque manquée (voir [chance-toucher](chance-toucher.md)), soit une réduction des dégâts. Le flee (fuite) est la capacité à sortir du combat. Les deux sont liés à la stat d'esquive.

**Rôle :** Défense, survie, builds agilité. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Esquive

- Si hit_chance échoue : 0 dégât (attaque manquée)
- Ou : % de réduction des dégâts si "touché mais esquivé partiellement"

### Flee

- Taux de fuite = f(vitesse_joueur, vitesse_ennemi, nombre_ennemis)
- Réussite : sortie du combat, pas d'aggro immédiat

---

## Références

- [Index 07](_index.md)
- [Chance toucher](chance-toucher.md)
