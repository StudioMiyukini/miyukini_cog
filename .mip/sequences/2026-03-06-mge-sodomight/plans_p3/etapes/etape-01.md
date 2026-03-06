# Etape 01 - Fondation workspace et gouvernance technique

## Objectif

Poser `mge/` comme workspace Rust autonome, compilable et lisible.

## Taches

1. Creer `mge/Cargo.toml` avec un workspace isole du workspace racine.
2. Creer les crates fondatrices minimales: `mge-core`, `mge-content`, `mge-render`, `games/sodomight`.
3. Poser l'arborescence `crates/`, `tools/`, `games/`, `assets-src/`, `assets-baked/`, `data/`.
4. Definir la convention de nommage des crates, modules, packages et bins.
5. Poser `rustfmt`, `clippy`, editions Rust, versions minimales et policies de dependances.
6. Poser une commande de build locale unique pour compiler tout `mge/`.
7. Poser une commande de test locale unique pour les crates fondatrices.
8. Poser les fichiers de configuration communs: lint, tests, licenses internes, README de workspace.
9. Verifier que `mge/` build seul depuis sa racine sans dependre du monorepo COG.
10. Verifier qu'un build depuis la racine COG ne casse pas l'isolement de `mge/`.
11. Renseigner la carte initiale des responsabilites crate par crate.
12. Geler les conventions de structure avant ouverture des etapes runtime/render.

## Documentation de soutien

1. Rediger la carte du workspace et les responsabilites des crates.
2. Documenter les conventions de code, CI locale et policies de dependances.

## Criteres de sortie

1. Workspace `mge/` compilable.
2. Structure de base stable pour la suite.
3. Documentation d'architecture initiale disponible.
