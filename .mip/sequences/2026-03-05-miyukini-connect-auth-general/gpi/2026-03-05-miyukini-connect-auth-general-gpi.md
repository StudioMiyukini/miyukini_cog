# GPI - Miyukini Connect

## Objectif global

Livrer un service d'authentification transverse COG, robuste, standardise et conforme LOI.

## Priorite 0 - Corrections bloquantes audit T9 (C1/C2/C3)

Ces corrections sont a executer **avant tout lancement P3 effectif**.

| ID | Correction | Cible | Critere de cloture |
|----|------------|-------|--------------------|
| C1 | Migration hash legacy SHA256 -> Argon2id | E01 (au plus tard) | Migration sur login valide + retrocompat testee |
| C2 | Policy AAL normative gelee/versionnee | E01-E02 | Table versionnee + tests non-regression verts |
| C3 | Integrite session + audit chain v1 | E02-E03 | Fingerprint session + hash-chain locale + verification post-reconnexion |

Regles de gate:

1. Pas d'ouverture E02 sans C1/C2 engages.
2. Pas de passage G3 sans C3 implemente au minimum.
3. Pas de passage G5 si une condition C1/C2/C3 reste ouverte.

Etat d'avancement apres lancement P3 autopilote:

| ID | Etat | Evidence |
|----|------|----------|
| C1 | Ferme (local) | Hash password Argon2id (`$argon2id$`) + test `c1_password_is_argon2id...` |
| C2 | Ferme (local) | Policy engine versionne (`policy_version`) + mappings AAL testes |
| C3 | Ferme (local) | `integrity_fingerprint` session + `audit_chain.verify_integrity()` + tests E04 |

## Conditions readiness T10 (obligatoires)

Ces conditions sont issues de `P0 T10 - Verification CI/CD` et deviennent contraignantes pour l'execution:

1. Pipeline CI dedie `miyukini-connect` cree des E01.
2. Checks C1/C2/C3 rendus bloquants dans la pipeline avant passage G3.
3. Scenario offline/isolated rendu obligatoire avant passage G5.
4. Echec d'un check S3 (C1/C2/C3) = merge bloque.

## Jalons

| Jalon | Cible | Critere |
|-------|-------|---------|
| J1 - Foundation | API + policy engine + login local | AAL1/AAL2 operationnels offline |
| J2 - Step-up | Passkey/hardware key + session elevation | AAL3/AAL4 + step-up valide |
| J3 - Origin integration | Probe + capabilities cache | degradation online/offline stable |
| J4 - Central coupling | Boot couple + UI embed | login Central via Connect 100% |
| J5 - Hardening | audit + tests securite + P5 | zero bloqueur critique |

## KPI securite

1. Taux de succes login legitime >= 98%.
2. Taux de blocage brute-force >= 99% des tentatives anormales.
3. Taux de bypass step-up = 0.
4. Couverture tests policy AAL >= 90%.

## KPI resilience

1. Login local en mode isolation: 100% des parcours eligibles.
2. Delai bootstrap Connect <= 1.5s sans Origin.
3. Reprise online apres retour reseau <= 10s pour refresh capabilities.

## KPI UX

1. Temps median login standard < 20s.
2. Temps median step-up < 30s.
3. Taux abandon login < 8%.

## Risques GPI

| Risque | Niveau | Action |
|--------|--------|--------|
| UX trop contraignante en haut niveau | Haut | Step-up cible uniquement sur actions sensibles |
| Ambiguite entre tiers permission et role metier | Moyen | Contrat API `authorize-hint` + doc integration |
| Dependance Origin non maitrisee | Moyen | Cache capacites + mode local par defaut |
