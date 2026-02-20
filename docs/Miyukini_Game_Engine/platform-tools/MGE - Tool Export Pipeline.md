# MGE — Tool Export Pipeline

## Contexte

Élément critique : transformer données dynamiques (SQL, JSON éditable) en données runtime statiques. Garantit déterminisme, cohérence, versionnage, performance.

## Portée / Scope

- **Applicable à :** Build Allumina, livraison runtime.
- **Statut :** Spécification.

---

## Rôle

Transformer données dynamiques → runtime statique.

## Processus

```
SQL / JSON structuré
        │
        ▼
   Validation (schéma, contraintes)
        │
        ▼
   Optimisation (compression, déduplication)
        │
        ▼
   Export → /export/runtime_data/
```

## Garantit

| Garantie | Description |
|----------|-------------|
| **Déterminisme** | Même input → même output |
| **Cohérence** | Références valides, pas d'orphelins |
| **Versionnage** | Hash, timestamps, diff |
| **Performance** | Format optimisé pour chargement runtime |

## Formats de sortie

- JSON optimisé
- TOML structuré
- Binaire compressé (optionnel)

## Règles

- Point d'entrée unique pour toutes les données game design
- Le runtime ne consomme que `/export/runtime_data/`

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
