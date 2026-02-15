# États de Confiance (T0 à T4)

## Vue d'ensemble

Le système peut être dans **5 états de confiance**. Ces états influencent TAMR (autorisations), BorderGuard (frontières) et WorrySentinel (alertes).

| État | Code | Description | Effet typique |
|------|------|-------------|----------------|
| **Normal** | T0 | Aucune anomalie | Accès selon permissions |
| **Instable** | T1 | Comportement inhabituel | Surveillance renforcée |
| **Dégradé** | T2 | Anomalies confirmées | Accès réduit |
| **Restreint** | T3 | Menace active | Accès minimal |
| **Bloqué** | T4 | Situation critique | Accès bloqué |

## Transitions

- **T0 vers T1** : WorrySentinel détecte des anomalies.
- **T1 vers T2** : Confirmation d'anomalie.
- **T2 vers T3** : Menace identifiée.
- **T3 vers T4** : Décision de lockdown.
- **Retour à T0** : Après investigation et validation.

## Utilisation

- **TAMR** : Adapte les autorisations selon l'état.
- **BorderGuard** : Renforce les contrôles ou ferme les frontières.
- **WorrySentinel** : Déclenche les transitions et alerte les Cores.

Documentation : Security - Operational Runbook, Threat Model Summary.
