# Etape 02 - MFA forte et step-up

## Objectif

Activer la logique AAL et le step-up sur operations sensibles.

## Taches

1. Implementer mapping `permission_tier -> required_aal`.
2. Implementer endpoint `auth/step-up`.
3. Integrer passkey/hardware key (ou adaptation progressive).
4. Rotation session id apres elevation.

## Criteres de sortie

1. Action `sensitive_write` bloquee sans step-up valide.
2. Tests anti-bypass step-up passes.
3. Claims AAL mis a jour apres elevation.

## Risques

- UX trop contraignante sans orchestration claire des parcours.

## Execution autopilote

- Statut: Termine
- Livrables:
1. Mapping `permission_tier -> required_aal` implemente.
2. Endpoint logique `auth_step_up` implemente.
3. Rotation session id appliquee apres elevation.
- Evidence:
1. test `e02_step_up_rotates_session_id_and_unlocks_sensitive_write` PASS.
