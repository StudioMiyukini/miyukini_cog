# MGE — Tool Data Authoring (SQL Editor)

## Contexte

Outil d'édition rapide du game design. Permet de modifier stats, unités, factions, loot et progression via une base SQLite locale et une UI dédiée.

## Portée / Scope

- **Applicable à :** Édition design Allumina, itération rapide.
- **Statut :** Spécification.

---

## Rôle

Édition rapide du game design sans toucher au code.

## Contient

- SQLite DB locale
- Tables : stats, unités, factions, loot, progression
- UI de modification rapide
- Historique de modifications
- Comparaison de versions

## Règles

| Règle | Description |
|-------|-------------|
| **Éditer** | Modifier données game design |
| **Tester** | Valider in-game via export |
| **Explorer** | Requêtes ad hoc, exploration |

**Ce tool ne sert qu'à éditer, tester, explorer.** Il n'exécute pas la simulation.

## Export

Vers `/export/runtime_data/` :

- JSON optimisé
- TOML structuré
- Binaire compressé (optionnel)

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
