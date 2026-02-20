# MGE — Tool AI Assist Layer

## Contexte

Assistant IA connecté aux outils du Tooling Layer, jamais au runtime. Peut générer, proposer, corriger, varier — sans modifier le kernel.

## Portée / Scope

- **Applicable à :** Workflow Allumina, productivité designers.
- **Statut :** Spécification.

---

## Rôle

Assistant IA connecté aux outils, pas au runtime.

## Peut

- Générer prefabs
- Proposer équilibrage (Balance Lab)
- Corriger incohérences (Data Authoring)
- Générer variations (sprites, stats)
- Créer nouveaux ennemis
- Générer quêtes (structure, objectifs)

## Ne fait jamais

- Modifier directement le kernel
- S'exécuter dans le runtime
- Bypasser les outils (toujours via UI/API outil)

## Règles

- L'IA est un **assistant**, pas un acteur autonome
- Toute action IA passe par un outil (Data Authoring, Prefab Editor, etc.)
- L'utilisateur valide/accepte les propositions

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
