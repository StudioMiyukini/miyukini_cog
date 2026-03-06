# Trace P4

## Statut

- Etat : Termine (GO conditionnel P5)
- Phase : P4
- Responsable principal : George/Victor
- Date : 2026-03-06

## TL;DR

P4 execute avec integration Central + audits conformite/securite.
Le service `miyukini-connect` a ete durci (lockout + expiration session), integre au runtime login Central
via affichage de l'etat Connect, puis valide par builds/tests.

## Integrations realisees

1. `apps/central` depend maintenant de `miyukini-connect`.
2. `ServiceConnections` instancie un `ConnectService` local.
3. Ecran connexion affiche l'etat runtime Connect (`ONLINE_FULL/DEGRADED/ISOLATED/SUSPICIOUS`).
4. Correction RSX hors perimetre detectee pendant build (`miyucloud/auth_security.rs`) pour debloquer la phase.
5. Correction test TLS hors perimetre (`miyucloud/web_surface/tls.rs`) pour provider rustls.

## Verifications techniques

1. `cargo build --workspace` : PASS.
2. `cargo build -p miyukini-central-native` : PASS.
3. `cargo test -p miyukini-connect` : PASS (8 tests).
4. `cargo test -p miyukini-central` : PASS.
5. `cargo test -p miyukini-central-native` : PASS.
5b. `cargo test -p miyucloud-server --bin miyucloud-server web_surface::tls::tests::test_load_existing_cert` : PASS (apres correctif provider).
6. `cargo clippy -p miyukini-connect -- -D warnings` : PASS.
7. `cargo clippy -p miyukini-central -- -D warnings` : FAIL hors perimetre (`jayrdv` lint debt existante).
8. `cargo clippy -p miyukini-central-native` : PASS avec warnings non bloquants existants.

## Gate P4

- George (conformite) : PASS conditionnel.
- Victor (securite) : PASS, score securite >= 60/100 (voir RAS).
- Defauts critiques bloquants : aucun detecte sur le perimetre Miyukini Connect.
- Dette hors perimetre : lint strict workspace non vert a cause de `jayrdv`.

## Decision

- GO conditionnel vers P5.
- Conditions a maintenir avant merge final:
1. Traiter ou accepter explicitement la dette lint hors perimetre `jayrdv`.
2. Ajouter audit dependances (cargo-audit) dans CI quand l'outil est disponible.
