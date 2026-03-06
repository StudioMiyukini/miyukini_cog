# Trace P3

## Statut

- Etat : Termine (autopilote FULL)
- Phase : P3
- Responsable principal : Denis

## TL;DR

P3 execute en mode FULL autopilote avec livraison d'un crate `miyukini-connect` couvrant
E01 a E04 (auth locale offline, step-up AAL, probe Origin/runtime, hardening isolation,
audit chain). Validation E05 effectuee via tests/lint locaux et runbook P3.

## Timeline execution

1. E01 termine : fondation locale `api/policy/session` + login offline password+TOTP.
2. E02 termine : step-up `auth_step_up`, rotation `session_id`, controles anti-bypass AAL.
3. E03 termine : `origin_ping`, cache capabilities, `runtime_state` (ONLINE_FULL/DEGRADED/ISOLATED).
4. E04 termine : blocage facteurs faibles en ISOLATED, mode SUSPICIOUS, audit chain hash.
5. E05 termine : campagne locale PASS-0/PASS-01 via tests + lint crate + runbook.

## Gates

- G1 : PASS
- G2 : PASS
- G3 : PASS
- G4 : PASS
- G5 : PASS (local)

## Evidences techniques

1. Nouveau crate: `crates/miyukini-connect`.
2. Tests executes: `cargo test -p miyukini-connect` -> 6 passes.
3. Lint execute: `cargo clippy -p miyukini-connect -- -D warnings` -> PASS.
4. Runbook P3: `ressources/runbook-p3-miyukini-connect.md`.

## Decision

- Passage P4 : GO conditionnel (integration runtime dans UI Central a finaliser en P4).
