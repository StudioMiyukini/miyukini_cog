# P0 Temps 10 - Verification CI/CD

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Hugo

## Commandes de validation par etape

```bash
# Apres E00 (obligatoire avant E01-E05)
cargo check -p miyukini-central
cargo clippy -p miyukini-central -- -D warnings

# Apres chaque etape E01-E05
cargo check -p miyukini-central
cargo clippy -p miyukini-central -- -D warnings

# Verification composants nouveaux
cargo test -p miyuki-ui-dioxus
cargo clippy -p miyuki-ui-dioxus -- -D warnings
```

## Pas de CI automatisee

Le workspace n'a pas de pipeline CI/CD configure. Validation manuelle via `cargo check` + `cargo clippy`. Pas de blocker CI.

## Gate qualite

- `cargo check -p miyukini-central` doit passer AVANT de passer a l'etape suivante
- Si clippy echoue : corriger avant de commiter
