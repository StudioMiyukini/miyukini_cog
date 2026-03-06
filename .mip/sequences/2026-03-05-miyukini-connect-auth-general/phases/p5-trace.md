# Trace P5

## Statut

- Etat : Termine (validation utilisateur)
- Phase : P5
- Responsable principal : Denis
- Date : 2026-03-05

## TL;DR

Livrable P5 prepare et presentable.
Le lot est techniquement valide, avec conditions P4 explicites conservees:
1) dette lint `jayrdv` hors perimetre acceptee avec reserve,
2) `cargo-audit` a integrer en CI des que disponible.

## Volet 1 - Presentation du livrable

Fonctionnalites livrees:

1. Service `miyukini-connect` local-first complet (policy/session/step-up/origin/audit).
2. Durcissement P4:
   - lockout apres echecs de connexion repetes
   - expiration session idle + absolue
3. Integration Central:
   - instance `ConnectService` dans les connexions globales
   - affichage de l'etat runtime Connect sur l'ecran de connexion
4. Correctifs de stabilite rencontres pendant P4:
   - correction parse RSX `miyucloud/auth_security.rs`
   - correction provider TLS test `miyucloud/web_surface/tls.rs`

Fichiers modifies (tracked):

1. `Cargo.toml`
2. `Cargo.lock`
3. `apps/central/Cargo.toml`
4. `apps/central/src/data.rs`
5. `apps/central/src/screens/connexion.rs`
6. `apps/central/src/services/miyucloud/auth_security.rs`
7. `apps/miyucloud/src/web_surface/tls.rs`

Fichiers ajoutes (untracked):

1. `crates/miyukini-connect/*`
2. `.mip/sequences/2026-03-05-miyukini-connect-auth-general/*`

Verification executee:

1. `cargo build --workspace` : PASS
2. `cargo build -p miyukini-central-native` : PASS
3. `cargo test -p miyukini-connect` : PASS (8 tests)
4. `cargo test -p miyukini-central` : PASS
5. `cargo test -p miyukini-central-native` : PASS
6. `cargo test -p miyucloud-server --bin miyucloud-server web_surface::tls::tests::test_load_existing_cert` : PASS
7. `cargo clippy -p miyukini-connect -- -D warnings` : PASS
8. `cargo clippy -p miyukini-central -- -D warnings` : FAIL hors perimetre (`jayrdv` legacy)

## Volet 2 - Test humain

Artefact de validation utilisateur:

1. `audits/2026-03-05-miyukini-connect-auth-general-p5-validation.md`

Etat:

1. Checklist test executee.
2. Validation explicite utilisateur recue: `p5 valide`.
3. Verdict retenu: `ACCEPTE AVEC RESERVES`.

## Volet 3 - Conditions P4 respectées en P5

1. Condition 1 (dette lint `jayrdv`) : documentee et acceptee avec reserve.
2. Condition 2 (`cargo-audit` CI) : tracee comme action obligatoire avant merge final.

## Decision intermediaire P5

- Gate P5: PASSE.
- Passage P6: GO.
