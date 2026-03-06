# Etape 04 - Hardening isolation et anti-2-temps

## Objectif

Durcir le mode isole face a une compromission preparee online.

## Taches

1. Bloquer enrolment/recovery faible en `ISOLATED`.
2. Ajouter integrity fingerprint de session.
3. Ajouter audit chain hash append-only.
4. Activer mode `SUSPICIOUS` (hausse AAL + restrictions).

## Criteres de sortie

1. Test scenario attaque 2-temps passe.
2. Tampering logs detecte.
3. Revocation preventive sessions suspectes operationnelle.

## Risques

- Faux positifs en mode suspicious si seuils mal regles.

## Execution autopilote

- Statut: Termine
- Livrables:
1. Blocage facteurs faibles en `ISOLATED` (`EmailOtp`, `QrSigned`).
2. Integrity fingerprint par session implemente.
3. Audit chain hash append-only + verification integrite.
4. Mode `SUSPICIOUS` avec hausse exigences AAL.
- Evidence:
1. test `e04_isolated_blocks_weak_factors_and_suspicious_hardens_policy` PASS.
2. test `e04_audit_chain_detects_tampering` PASS.
