# Configuration environnement MIP

## TL;DR

Rust workspace Miyukini COG. Windows 11, 16 cœurs, 15 GB RAM. Cursor + Claude. Mode FULL.

## Métadonnées

- Date de configuration : 2026-03-04
- Version MIP : v2.1
- Reconfigurable via : `/mip_setup`

## Système

- OS : Windows 11 (10.0.26200)
- Shell : PowerShell 5.1
- CPU : 16 cœurs logiques
- RAM : 15 GB
- GPU : Aucune
- Disque : ~27 GB libre
- Réseau : Internet oui
- Git : 2.53.0 (StudioMiyukini, miyukini-cog)

## Stack technique

- Langage(s) : Rust
- Framework(s) : axum, Dioxus 0.6
- Base(s) de données : KindMother (SQLite), SQLCipher
- Style API : REST
- Gestionnaire de paquets : Cargo
- Monorepo : oui (workspace)
- Linter : clippy
- Formateur : rustfmt
- Framework de tests : cargo test

## Commandes standard

- Build : `cargo build --workspace`
- Test : `cargo test --workspace`
- Lint : `cargo clippy --workspace -- -D warnings`
- Format : `cargo fmt --all`
- Test unitaire : `cargo test -p {crate}`

## Sécurité

- Niveau : standard
- Conformité : RGPD
- Chiffrement : at-rest (SQLCipher), transit (TLS)
- Secrets : `.mip/secrets/` (variables d'env)

## Infrastructure

- Hébergement code : GitHub
- CI/CD : aucune
- Conteneurisation : aucune
- Déploiement : local / VPS
- Auth : JWT, OAuth2 (selon service)

## Outil IA

- Outil principal : Cursor (Composer)
- Inférence locale : aucune
- Modèle(s) : Claude Sonnet/Opus (configurable)
- Budget : à définir
- Abonnements : `.mip/config/subscriptions.md` (optionnel)
- Capacités MIP adaptées :
  - Agents parallèles : Via Agent
  - TodoWrite : non
  - Vérification docs/MCP : oui
  - Tâches arrière-plan : limité
  - Accès terminal : oui

## Dépendances

### Installées

- rustc / cargo : (rustup)
- git : 2.53.0

### Manquantes

- Docker : non installé
- CI/CD : à configurer si besoin

## Conventions du projet

- Convention commit : type(scope): message
- Annotations de code : MSCM (@id, @do, @role, @layer, @human)
- Patterns d'erreur : types explicites, pas de unwrap() en prod
- Règles NON NÉGOCIABLES : Documents max 400 lignes, Loi 9 parallélisation
