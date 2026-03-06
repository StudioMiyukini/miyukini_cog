# Configuration environnement MIP

## TL;DR

Genere par MIP UI JSX (sans compilation).

## Metadonnees
- Date de configuration : 2026-03-05
- Version MIP : v2.1
- Reconfigurable via : `/mip_setup`

## Stack technique
- Langage(s) : Rust
- Framework(s) : axum, Dioxus 0.6
- Base(s) de donnees : KindMother (SQLite), SQLCipher

## Commandes standard
- Build : `cargo build --workspace`
- Test : `cargo test --workspace`
- Lint : `cargo clippy --workspace -- -D warnings`
- Format : `cargo fmt --all`

## Securite
- Niveau : standard

## Infrastructure
- Deploiement : local / VPS

## Outil IA
- Outil principal : Cursor (Composer)
- Modele(s) : claude-sonnet-4
- Budget : à définir
- Abonnements : desactive

## Garde-fou
- Cette interface n'ecrit que dans `.mip/`
