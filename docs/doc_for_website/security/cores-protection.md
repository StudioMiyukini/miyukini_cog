# Protection par les Cores

## Gouvernance Cores et protection des données

La **protection des données** et des frontières est assurée par les **Cores** : ils décident ; ils ne stockent pas les données utilisateur en dehors de ce que KindMother gère sous leur gouvernance.

## Rôles par Core

| Core | Rôle de protection |
|------|---------------------|
| **TAMR** | Qui peut accéder à quoi ; révocation en cas de menace |
| **BorderGuard** | Ce qui entre et sort ; quarantaine ; blocage d'exfiltration |
| **KindMother** | Chiffrement au repos ; intégrité ; sauvegardes |
| **WorrySentinel** | Détection d'anomalies ; alertes ; états de confiance |
| **StrongFather** | Coordination des réponses (lockdown, dégradation) |
| **LogisticsSteward** | Limitation des ressources (abus, DoS) |

## Flux de décision

- Aucune donnée sensible sans **validation TAMR**.
- Aucun flux réseau sans **contrôle BorderGuard**.
- Toute anomalie remonte à **WorrySentinel** et peut déclencher des actions des autres Cores.

## Référence

Security - Gouvernance Cores Protection Données.
