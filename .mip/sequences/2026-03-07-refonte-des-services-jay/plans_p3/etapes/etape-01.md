# E01 — MSCM Audit complet famille Jay

## Statut : A faire
## Depend de : E00
## Agents : George (audit) + François (corrections back) + Lise (corrections front)
## Taches : 12
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Objectif : 100% conformité MSCM sur crates/jayfestival + crates/jayxpose. Audit MSCM only sur autres services Jay (jaykoa, jaykonta, jaymanga, jay1tribu). Référence : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`.

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E01-01 | AUDIT | Audit MSCM crates/jayfestival — rapport manques | George | `crates/jayfestival/src/**/*.rs` | pending | — | — |
| E01-02 | AUDIT | Audit MSCM crates/jayxpose — rapport manques | George | `crates/jayxpose/src/**/*.rs` | pending | — | — |
| E01-03 | CODE | Corriger MSCM `crates/jayfestival/src/data/` | François | `kindmother_db.rs`, `kindmother_client_db.rs`, `types.rs`, `mod.rs` | pending | — | — |
| E01-04 | CODE | Corriger MSCM `crates/jayfestival/src/auth/` | François | `mod.rs`, `permissions.rs` | pending | — | — |
| E01-05 | CODE | Corriger MSCM `crates/jayfestival/src/services/` | François | `services/**/*.rs` | pending | — | — |
| E01-06 | CODE | Corriger MSCM `crates/jayxpose/src/` | François | `data/**/*.rs`, `auth/mod.rs`, `governance.rs` | pending | — | — |
| E01-07 | CODE | Corriger MSCM `apps/central/src/services/jayfestival/` | Lise | tous `.rs` (30+ fichiers) | pending | — | — |
| E01-08 | CODE | Corriger MSCM `apps/central/src/services/jayxpose/` | Lise | tous `.rs` | pending | — | — |
| E01-09 | AUDIT | Audit MSCM only — crates/jaykoa + crates/jaykonta | George | rapport uniquement | pending | — | — |
| E01-10 | AUDIT | Audit MSCM only — crates/jaymanga + crates/jay1tribu | George | rapport uniquement | pending | — | — |
| E01-11 | TEST | `cargo clippy -p jayfestival -p jayxpose -- -D warnings` | Denis | workspace | pending | — | — |
| E01-12 | TEST | `cargo test -p jayfestival` + `cargo test -p jayxpose` | Denis | workspace | pending | — | — |

## Critères de complétion
- 0 bloc public sans `@id @role @layer @human` dans jayfestival + jayxpose
- 0 doublon `@id` dans le codebase
- `cargo clippy -p jayfestival -p jayxpose -- -D warnings` : 0 warning
- `cargo test -p jayfestival` + `cargo test -p jayxpose` : 100% passent

## Commit message template
`feat(mscm): E01 -- MSCM audit complet JayFestival + JayXpose`
