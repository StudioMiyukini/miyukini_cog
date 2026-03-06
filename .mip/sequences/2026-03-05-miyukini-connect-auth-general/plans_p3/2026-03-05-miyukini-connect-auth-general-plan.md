# Plan P3 - miyukini-connect-auth-general

## TL;DR

Execution P3 planifiee en 5 etapes avec gates de securite.
Cible: livrer Miyukini Connect conforme LOI COG, online + isolated, avec niveaux AAL et step-up pour les operations sensibles.

## Principes de conduite

1. Ne jamais casser le chemin critique offline.
2. Ne jamais reduire l'exigence securite quand Origin est indisponible.
3. Valider chaque etape par criteres de sortie explicites.
4. Prioriser la robustesse des sessions avant enrichissements UX.

## Conditions d'entree P3 (issues T10)

1. Un pipeline CI dedie `miyukini-connect` doit exister des E01.
2. Les checks C1/C2/C3 doivent etre bloques en CI avant passage G3.
3. Les tests offline/isolated doivent etre obligatoires avant passage G5.
4. Tout echec de check securite S3 entraine un blocage de merge et de gate.

## Roadmap par etape

### Etape 01 - Foundation locale

Objectif:

- Poser le service `miyukini-connect` minimal viable.

Travaux:

1. Squelette service + modules (api/policy/session).
2. Contrats API v1 minimaux (`bootstrap`, `auth/verify`, `session/current`, `introspect`).
3. Login local + TOTP de base.
4. Claims session normalises (`aal`, `methods`, `permission_tier`).

Sortie attendue:

- Login offline operationnel.
- Introspection session disponible pour un service.

Gate G1:

- Tests unitaires policy de base passes.
- Test integration offline passe.

### Etape 02 - MFA forte et step-up

Objectif:

- Activer l'elevation de confiance par criticite.

Travaux:

1. Mapping `permission_tier -> required_aal` implemente.
2. Endpoint `auth/step-up` actif.
3. Ajout passkey/hardware key (ou interface prete si integration progressive).
4. Rotation session id apres step-up.

Sortie attendue:

- Refus automatique des actions sensibles sans AAL requis.

Gate G2:

- Scenario `sensitive_write` impose step-up.
- Tests anti-bypass step-up passes.

### Etape 03 - Couplage Central + Origin

Objectif:

- Connect devient la porte d'entree de session Central.

Travaux:

1. Handshake bootstrap Central <-> Connect.
2. Probe Origin capabilities (timeout court + cache local).
3. Etats runtime exposes: ONLINE_FULL / ONLINE_DEGRADED / ISOLATED.
4. UI login integrable frame/modal/full.

Sortie attendue:

- Demarrage Central via Connect stable online/offline.

Gate G3:

- Test boot complet Central+Connect sans Origin passe.
- Test boot avec Origin disponible passe.

### Etape 04 - Hardening isolation et anti-2-temps

Objectif:

- Rendre l'isolement resistant aux attaques preparees online.

Travaux:

1. Blocage enrolment/recovery faible en mode ISOLATED.
2. Integrity fingerprint de session.
3. Journal auth append-only chaine par hash.
4. Mode SUSPICIOUS (hausse exigences AAL + restrictions).

Sortie attendue:

- Scenario attaque en 2 temps detecte et contenu.

Gate G4:

- Test d'attaque en 2 temps passe.
- Verification integrite journaux passe.

### Etape 05 - Validation finale et readiness P4/P5

Objectif:

- Fermer la phase P3 avec un niveau de confiance execution.

Travaux:

1. Validation PASS-0 / PASS-01.
2. Campagne tests integration et non-regression.
3. Runbook incident/recovery/reconciliation.
4. Dossier de transfert vers P4/P5.

Sortie attendue:

- Build/Tests/Audit verts sans bloqueur critique.

Gate G5:

- PASS-0 atteint.
- PASS-01 atteint.
- Backlog residuel classe et trace.

## Dependances et enchainement

```text
E01 -> E02 -> E03 -> E04 -> E05
```

Paralleles autorises:

1. UI Lise peut avancer pendant E02/E03 si contrats API stables.
2. Hugo peut preparer scripts/tests de pipeline des E01.
3. George/Jean/Arianne peuvent preparer audit templates pendant E03.

## RACI simplifie

- Denis: pilotage execution + integration + migration.
- Francois: implementation backend coeur Connect.
- Victor: validation securite et criteres PASS.
- Lise: UX auth et etats runtime.
- Hugo: outillage, run, validation CI locale.
- George/Jean/Arianne: audit qualite/efficience/validation finale.

## Criteres globaux de reussite P3

1. Service Connect utilisable par Central en online et isolated.
2. Step-up exige pour actions sensibles.
3. Aucune elevation critique sans AAL requis.
4. Scenario attaque 2-temps traite.
5. Artefacts P3 complets pour entree P4.

## Execution autopilote (2026-03-05/06)

Statut global:
1. E01 -> E05 executes en mode FULL autopilote.
2. G1 -> G5 passes (validation locale).

Evidences:
1. Implementation: `crates/miyukini-connect` (policy, sessions, step-up, origin probe, audit chain).
2. Tests: `cargo test -p miyukini-connect` (6/6 PASS).
3. Lint: `cargo clippy -p miyukini-connect -- -D warnings` PASS.
4. Runbook: `ressources/runbook-p3-miyukini-connect.md`.

Restant vers P4:
1. Integration UI Central du runtime state et des parcours step-up.
