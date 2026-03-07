# Agent François — P3 Refonte Services Jay (Blocs A+B+C)

## Rôle séquence

Dev back-end — MSCM audit + corrections + hardening JayFestival + JayXpose. Intervient en Blocs A (audit), B (JayFestival backend), C (JayXpose backend + contrats).

## Contexte séquence

- Stack : Rust + Dioxus 0.7 + KindMother (rusqlite)
- MSCM obligatoire : `@id @do @role @layer @human` sur tout bloc public
- Référence sécurité : `apps/miyucloud/src/security_headers.rs`
- Checklist MSCM : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- Spec : `specs/2026-03-07-refonte-des-services-jay-spec.md`
- Anti-patterns : `drop(conn)` avant appel récursif, `?` pas de `unwrap()`, `thiserror` pour erreurs

## Fichiers à charger au démarrage

1. `specs/2026-03-07-refonte-des-services-jay-spec.md` (section Architecture + MSCM)
2. `phases/p0/temps/temps-04-inventaire.md` (crates à modifier)
3. `phases/p0/temps/temps-05-securite.md` (checklist sécu)
4. `.mip/memory/patterns-and-lessons.md` (anti-patterns)

## Tâches typiques

- Ajouter/corriger annotations MSCM sur crates/jayfestival/ et crates/jayxpose/
- Hardening `kindmother_db.rs` : validation entrées, PRAGMA, Result partout
- Créer `crates/jayfestival/src/portal_contract.rs` (impl PortalContract)
- Créer `crates/jayxpose/src/portal_contract.rs` (impl PortalContract)

## Critères de complétion par tâche

- 0 `unwrap()` dans le code modifié
- Tous blocs pub annotés MSCM
- Tests passent : `cargo test -p jayfestival` + `cargo test -p jayxpose`
- Clippy clean : `cargo clippy -p jayfestival -p jayxpose -- -D warnings`
