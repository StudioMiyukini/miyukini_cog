# Audit global miyukini-connect-auth-general

## Statut

- Etat : Termine
- Phase : P4
- Responsable principal : George
- Date : 2026-03-06

## TL;DR

Conformite P4 validee sur le perimetre de la sequence:
integration Connect -> Central effectuee, build workspace vert, tests cibles verts,
securite validee (RAS 71/100). Dette lint stricte detectee hors perimetre (`jayrdv`).

## Checklist conformite George

- [x] Build workspace OK (`cargo build --workspace`)
- [x] Tests perimetre OK (`miyukini-connect`, `miyukini-central`, `miyukini-central-native`)
- [x] Lint perimetre connect strict OK (`cargo clippy -p miyukini-connect -- -D warnings`)
- [x] Integration back + front verifiee (Central affiche runtime Connect)
- [x] Annotations de code presentes sur nouveaux fichiers `miyukini-connect`
- [x] Lois d'autonomie respectees
- [x] Parcours utilisateur login coherent (etat Connect visible)

## Ecarts constates

1. `cargo clippy -p miyukini-central -- -D warnings` bloque par dette existante `jayrdv` (hors perimetre).
2. `cargo-audit` indisponible localement (commande non installee).
3. Campagne `cargo test --workspace --no-fail-fast` non terminee dans la fenetre timebox (timeout local).

## Actions correctives appliquees en P4

1. Correction parse RSX bloquante dans `apps/central/src/services/miyucloud/auth_security.rs`.
2. Durcissement `miyukini-connect`:
   - lockout apres echecs repetes
   - expiration session idle + absolue
3. Integration UI login Central de l'etat runtime Connect.
4. Correction test TLS `miyucloud-server` pour installer explicitement le provider rustls en test.

## Decision

- Audit P4 : PASS conditionnel.
- Passage P5 : autorise.
