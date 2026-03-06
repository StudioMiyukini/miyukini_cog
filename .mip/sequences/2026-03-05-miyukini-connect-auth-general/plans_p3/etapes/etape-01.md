# Etape 01 - Foundation locale

## Objectif

Mettre en place Miyukini Connect minimal viable en local/offline.

## Taches

1. Creer structure service `miyukini-connect` (api/policy/session).
2. Exposer endpoints minimaux (`bootstrap`, `auth/verify`, `session/current`, `introspect`).
3. Implementer login local et claims session normalises.
4. Ajouter TOTP de base.

## Criteres de sortie

1. Login offline fonctionnel.
2. Session introspectable par un service consommateur.
3. Tests unitaires policy de base verts.

## Risques

- Dette technique si schema session non stable des E01.

## Execution autopilote

- Statut: Termine
- Livrables:
1. crate `crates/miyukini-connect` cree.
2. Contrats locaux exposes: bootstrap, auth_verify, session_current, session_introspect.
3. Login offline password+TOTP operationnel avec claims normalises.
- Evidence:
1. test `e01_offline_login_totp_and_introspect` PASS.
