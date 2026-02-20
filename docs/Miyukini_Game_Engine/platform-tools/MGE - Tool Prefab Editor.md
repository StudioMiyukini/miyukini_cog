# MGE — Tool Prefab Editor

## Contexte

Outil de création visuelle d'entités (prefabs). Permet d'assembler des composants, gérer l'héritage et visualiser la hiérarchie sans modifier le kernel.

## Portée / Scope

- **Applicable à :** Création entités Allumina, réutilisation assets.
- **Statut :** Spécification.

---

## Rôle

Créer visuellement des entités (prefabs).

## Permet

- Ajouter composants
- Héritage prefab (parent → enfant)
- Visualiser hiérarchie
- Tester overrides
- Voir conflits entre overrides

## Fonctionnement

**Lit :**

- Plugins actifs
- Composants disponibles (depuis Kernel/Plugins)

**Produit :**

- Fichiers prefabs exportables

## Règles

- Ne modifie jamais le kernel
- Consomme uniquement les composants déclarés par les plugins
- Export vers Export Pipeline

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
