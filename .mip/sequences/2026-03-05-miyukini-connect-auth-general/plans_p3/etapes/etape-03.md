# Etape 03 - Couplage Central et Origin

## Objectif

Rendre Connect effectif comme point d'entree de session Central.

## Taches

1. Handshake bootstrap Central <-> Connect.
2. Probe Origin capabilities + cache local.
3. Etats runtime (`ONLINE_FULL`, `ONLINE_DEGRADED`, `ISOLATED`) exposes.
4. Integration UI auth embed/modal/full.

## Criteres de sortie

1. Demarrage Central+Connect stable sans Origin.
2. Demarrage Central+Connect stable avec Origin.
3. Etas runtime correctement visibles en UI.

## Risques

- Couplage de demarrage fragile si timeouts/retries mal calibres.

## Execution autopilote

- Statut: Termine
- Livrables:
1. Probe Origin implemente via `origin_ping`.
2. Cache capabilities local + `bootstrap` etat runtime exposes.
3. Etats `ONLINE_FULL`, `ONLINE_DEGRADED`, `ISOLATED` pris en charge.
- Evidence:
1. test `e03_origin_probe_switches_runtime_states` PASS.
