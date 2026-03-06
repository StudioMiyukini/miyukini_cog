# E0 -- Fork OxiCloud & Fondations

## Statut : Termine
## Depend de : --
## Agents : Francois, Denis, Victor
## Taches : 8

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E0-01 | INFRA | Creer crate `crates/miyucloud-dav/` avec Cargo.toml | Francois | crates/miyucloud-dav/Cargo.toml | done |
| E0-02 | INFRA | Configurer workspace Cargo.toml (ajouter membre) | Francois | Cargo.toml | done |
| E0-03 | CODE | Creer `crates/miyucloud-dav/src/lib.rs` avec modules vides | Francois | crates/miyucloud-dav/src/lib.rs | done |
| E0-04 | INFRA | Ajouter dependances: quick-xml, mime_guess, moka, image, percent-encoding | Francois | crates/miyucloud-dav/Cargo.toml | done |
| E0-05 | CODE | Creer `crates/miyucloud-dav/src/common/mod.rs` (types partages) | Francois | crates/miyucloud-dav/src/common/ | done |
| E0-06 | TEST-S | Verifier version rusqlite pour CVE-2025-6965 et bumper si necessaire | Victor | Cargo.toml, Cargo.lock | done |
| E0-07 | CODE | Ajouter `#[derive(Zeroize, ZeroizeOnDrop)]` sur KeyManager | Victor | crates/miyucloud/src/ | done |
| E0-08 | INFRA | Configurer CI: paths triggers, clippy -p, test -p pour miyucloud-dav | Denis | .github/workflows/ | done |

## Commit message template
`feat(miyucloud-dav): E0 -- fork OxiCloud fondations, crate setup`
