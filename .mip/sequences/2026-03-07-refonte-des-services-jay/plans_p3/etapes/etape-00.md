# E00 — Smoke test + Init workspace Portal

## Statut : A faire
## Depend de : --
## Agents : Denis
## Taches : 3
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

> Objectif : valider que le workspace compile, créer la structure apps/cog-web-portal, et écrire un smoke test RED qui prouve la structure du plan avant de commencer le TDD.

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E00-01 | INFRA | Créer apps/cog-web-portal avec Cargo.toml | Denis | `apps/cog-web-portal/Cargo.toml`, `apps/cog-web-portal/src/main.rs`, `Cargo.toml` (workspace member) | pending | — | — |
| E00-02 | TEST | Smoke test RED — `cargo test -p cog-web-portal` échoue intentionnellement sur test placeholder | Denis | `apps/cog-web-portal/src/main.rs` | pending | — | — |
| E00-03 | INFRA | Vérifier `cargo check --workspace` 0 erreur | Denis | workspace | pending | — | — |

## Critères de complétion
- `cargo check --workspace` : 0 erreur
- `apps/cog-web-portal/` existe dans workspace members
- Smoke test existe et est RED (échoue — attendu)

## Commit message template
`feat(cog-web-portal): E00 -- smoke test RED + init workspace`

