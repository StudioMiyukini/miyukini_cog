# MiyukiniAdmin

## Panneau d'Administration Souverain

**MiyukiniAdmin** est l’opérateur de la **Strate 9** : le panneau d’administration du COG. Il fait exception architecturale en ayant des privilèges directs sur la configuration et le monitoring des Cores (sans contourner leurs décisions).

## Rôle

> MiyukiniAdmin **configure** et **surveille** le système, sans remplacer la gouvernance des Cores.

## Fonctionnalités Principales

| Fonction | Description |
|----------|-------------|
| **Configuration** | Paramètres système, Cores, services |
| **Monitoring** | Métriques, logs, états (T0–T4) |
| **Sécurité** | Panneau de contrôle sécurité, audits |
| **Base de données** | Gestion DB (Supabase/SQLite selon déploiement), migrations |
| **Récupération** | Procédures de recovery, sauvegardes |

## Position Architecturale

- **Strate 9** : Au-dessus des Opérateurs (Strate 7).
- **Exception** : Accès privilégié pour configurer et observer, pas pour décider à la place des Cores.
- **Souverain** : Réservé à l’administrateur du COG ; authentification et autorisation strictes.

## Documentation

- Documentation fondatrice, architecture et flux
- Contrats (database, integration, security, testing)
- UI (design, accessibilité, dashboard, DB management)
- Opérations (serveur HTTP/HTTPS, recovery)
- Référence (vocabulaire, SQL, capacités)

Racine : `docs/admin/MiyukiniAdmin/`.
