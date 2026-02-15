# Sécurité — Vue d'Ensemble

## Principes de Sécurité Miyukini

La sécurité dans Miyukini repose sur les **Cores** (gouvernance), les **invariants** (Lois d’Autonomie, contrats) et une **défense en profondeur** : Kernel, Cores, BorderGuard, TAMR, WorrySentinel.

## Architecture de Sécurité

| Couche | Rôle |
|--------|------|
| **Kernel** | Pas de logique métier ; `unsafe_code = "forbid"` |
| **Cores** | Décisions (TAMR, BorderGuard, KindMother, WorrySentinel) |
| **BorderGuard** | Contrôle des frontières (entrée/sortie) |
| **TAMR** | Authentification et autorisation |
| **KindMother** | Chiffrement au repos (SQLCipher) |
| **WorrySentinel** | Détection d’anomalies et alertes |

## Niveaux de Sécurité des Données

| Niveau | Nom | Usage |
|--------|-----|--------|
| 0 | Public | Données non sensibles |
| 1 | Standard | Authentification requise |
| 2 | Sensible | Chiffrement renforcé |
| 3 | Critique | Accès très restreint |
| 4 | Maximum | Protection maximale |

## États de Confiance (T0–T4)

| État | Code | Description |
|------|------|-------------|
| Normal | T0 | Fonctionnement nominal |
| Instable | T1 | Anomalies mineures |
| Dégradé | T2 | Anomalies confirmées |
| Restreint | T3 | Menace active, accès réduit |
| Bloqué | T4 | Accès bloqué |

## Mesures Principales

- **Liste des mesures** : Document de référence (Security - Liste des Mesures).
- **Gouvernance Cores** : Protection des données et des frontières par les Cores.
- **MIP Security** : Contrat de sécurité MIP (balisage, index, conformité).
- **Invariants** : Contrats d’invariants et anti-patterns documentés.

Documentation détaillée : `docs/security/`.
