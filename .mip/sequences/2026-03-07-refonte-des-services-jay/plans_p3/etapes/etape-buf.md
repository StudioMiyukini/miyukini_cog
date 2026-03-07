# BUF — Buffer corrections + Audit P4 préparation

## Statut : A faire
## Depend de : E04
## Agents : Denis (intégration) + George (conformité) + Victor (sécurité) + Jean (efficience)
## Taches : 6 (+ corrections dynamiques selon E01-E04)
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Buffer 20% : corrections issues des étapes précédentes. Audit intégration workspace. Préparation P4.

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| BUF-01 | FIX | Corrections issues E01-E04 (définies dynamiquement) | Denis | selon besoin | pending | — | — |
| BUF-02 | AUDIT | `cargo build --workspace` + `cargo test --workspace` | Denis | workspace | pending | — | — |
| BUF-03 | AUDIT | Audit conformité George — MSCM final, Lois d'Autonomie, UX | George | tous fichiers modifiés | pending | — | — |
| BUF-04 | AUDIT | Audit sécurité Victor — score /100 (PASS-0 + PASS-01 + RAS) | Victor | Portal + Jay | pending | — | — |
| BUF-05 | AUDIT | Audit efficience Jean — tokens consommés vs budget | Jean | `metrics/2026-03-07-refonte-des-services-jay.json` | pending | — | — |
| BUF-06 | FIX | Corrections défauts audit BUF-03/BUF-04 (si non CRIT) | Denis+François | selon audit | pending | — | — |

## Critères de complétion
- `cargo build --workspace` : 0 erreur
- `cargo test --workspace` : tous passent
- `cargo clippy --workspace -- -D warnings` : 0 warning
- Score sécurité Victor : ≥ 90/100
- 0 défaut BLOQUANT George
- Rapport efficience Jean disponible

## Commit message template
`fix(workspace): BUF -- corrections post-intégration + audit P4`

