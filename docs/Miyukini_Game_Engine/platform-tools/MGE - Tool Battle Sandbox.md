# MGE — Tool Battle Sandbox

## Contexte

Simulateur visuel isolé pour tester formations, morale, LOD comportemental. Mini runtime dédié aux batailles, sans jeu complet.

## Portée / Scope

- **Applicable à :** Test formations Massive Battle, tuning LOD.
- **Statut :** Spécification.

---

## Rôle

Simulateur visuel isolé pour batailles.

## Permet

- Charger ~100 soldats (ou plus)
- Tester morale
- Tester formations (ligne, carré, etc.)
- Tester LOD comportemental (Full/Reduced/Sleep)
- Visualiser résultats

## Important

C'est un **mini runtime isolé**. Il :

- Utilise le Kernel + plugins nécessaires (MB, spatial, etc.)
- Ne charge pas le jeu complet (Allumina)
- S'exécute en mode dédié pour tests
- Ne modifie pas le kernel

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
