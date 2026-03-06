# P0 Temps 10 - Verification CI/CD

## Etat actuel

- Le workspace racine build/test/clippy sur Rust
- `mge/` etant independant, il devra porter ses propres commandes et pipelines

## Recommandation

- CI racine : ne pas casser le depot principal si `mge/` est absent des membres workspace
- CI `mge/` : jobs dedies `fmt`, `clippy`, `test`, `assets-verify`, `package`
- Release : produire binaires standalone et package Market par plateforme
