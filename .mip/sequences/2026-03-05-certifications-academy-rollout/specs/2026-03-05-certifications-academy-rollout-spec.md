# Spec - Certifications Academy Rollout

## Architecture

- Source de verite certs: `.mip/certifications/registry.json`
- Formation locale: `.mip/certifications/{folder}/academy/`
- Sources web compactes: `.mip/certifications/sources/{agent}.md`
- Passeports agents: `.mip/certifications/diplomas/{agent}.md`
- Snapshot machine: `.mip/certifications/diplomas/STATUS.json`

## Regles

1. Modularite stricte (pas de monolithique).
2. Assessment = signal de statut.
3. READY_EXTERNE != certif reelle: readiness interne uniquement.

## Script de sync

- Script: `.mip/scripts/sync-cert-diplomas.ps1`
- Entree: registry + assessments
- Sortie: passports + index + status.json
