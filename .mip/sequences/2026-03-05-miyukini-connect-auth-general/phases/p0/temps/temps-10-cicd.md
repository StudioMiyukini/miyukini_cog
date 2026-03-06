# P0 Temps 10 - Verification CI/CD

## Statut

- Etat : Termine
- Phase : P0 Temps 10
- Responsable principal : Hugo
- Date : 2026-03-05

## TL;DR

Verification CI/CD realisee: le repo dispose d'un pipeline ciblant MiyuCloud, mais aucun pipeline dedie a Miyukini Connect pour couvrir les gates securite C1/C2/C3. Readiness CI/CD est **conditionnelle**: execution P3 possible si les checks bloquants proposes ci-dessous sont integres des E01 et activement executes avant G3/G5.

## 1) Etat CI/CD constate

### Existant

- Workflow present: `.github/workflows/miyucloud-ci.yml`
- Jobs couverts: `fmt`, `clippy`, `tests`, `cargo audit`, `build-release`.

### Manques pour Miyukini Connect

1. Aucun workflow `miyukini-connect` dedie.
2. Aucun check automatique C1 (migration hash legacy).
3. Aucun check contractuel C2 (policy AAL versionnee).
4. Aucun check C3 (integrite session + audit chain) en pipeline.
5. Aucun scenario de test offline/isolated en gate bloquante.

## 2) Blueprint pipeline cible (a activer P3)

### Stage S1 - Qualite code

1. `cargo fmt --check`
2. `cargo clippy -D warnings`
3. build debug des cibles connect

### Stage S2 - Tests fonctionnels

1. tests unitaires policy engine (AAL mapping)
2. tests integration auth/session/introspection
3. tests offline/isolated obligatoires

### Stage S3 - Securite blocante (C1/C2/C3)

1. **C1 check**: tests migration SHA256 -> Argon2id + retrocompat
2. **C2 check**: validation policy AAL versionnee + non-regression contractuelle
3. **C3 check**: verification integrity fingerprint + hash-chain audit + test post-reconnexion

### Stage S4 - Supply chain / build

1. `cargo audit`
2. build release artifacts
3. smoke test runtime minimal

Regle: echec S3 => pipeline rouge et merge bloque.

## 3) Mapping gates P3 -> CI/CD

| Gate P3 | Check CI/CD minimal |
|---------|----------------------|
| G1 | S1 + S2 offline basique |
| G2 | S2 step-up + C2 |
| G3 | C1 + C2 + C3 minimum |
| G4 | test attaque 2-temps automatise |
| G5 | PASS-0 + PASS-01 full verts |

## 4) Commandes de reference (local/CI)

```powershell
cargo fmt --all -- --check
cargo clippy -p miyukini-connect --all-targets -- -D warnings
cargo test -p miyukini-connect
cargo test -p miyukini-connect -- --ignored offline
cargo audit
```

Note: les cibles exactes seront stabilisees a la creation effective du crate/service `miyukini-connect`.

## 5) Verdict readiness T10

- **Verdict:** READY CONDITIONNEL
- Conditions:
  1. Pipeline dedie Connect cree a E01.
  2. Checks C1/C2/C3 rendus bloquants avant G3.
  3. Scenario offline/isolated rendu obligatoire avant G5.

## Decision T10

- Verification CI/CD terminee.
- Readiness conditionnelle validee.
- Passage recommande vers T11 (synthese et brief final P0).
