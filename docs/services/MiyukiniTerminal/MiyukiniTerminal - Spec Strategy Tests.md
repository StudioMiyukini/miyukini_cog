# MiyukiniTerminal — Spécification Stratégie Tests

## Contexte

Ce document décrit la **stratégie de tests** pour MiyukiniTerminal : tests unitaires (Rust), tests d'intégration (Relay mock), tests E2E (émulateur), couverture cible et outils.

**Références :**

- [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md)
- [Spec CI CD](./MiyukiniTerminal%20-%20Spec%20CI%20CD.md)

---

## Portée / Scope

- Tests unitaires
- Tests intégration
- Tests E2E
- Couverture
- Outils

---

## 1. Tests unitaires (Rust)

| Cible | Outil | Description |
|-------|-------|-------------|
| Logique métier | `#[test]` | Parsing token, validation payload |
| Queue | cargo test | Enqueue, retry, statuts |
| Storage | cargo test | CRUD identity, cache |
| Utilitaires | cargo test | Helpers, conversions |

### 1.1 Structure

```
src/
  foo.rs
  foo.rs  # #[cfg(test)] mod tests { ... }
```

Ou `tests/` pour intégration.

---

## 2. Tests d'intégration

| Cible | Mock | Description |
|-------|------|-------------|
| Relay | Serveur mock TCP | Répond REGISTER_OK, REGISTER_ERR |
| Sync API | mockito / wiremock | Réponses JSON préenregistrées |
| Storage | SQLite in-memory | Pas de fichier réel |

### 2.1 Relay mock

- Écouter sur port aléatoire
- Accepter REGISTER avec parent_cog_id
- Répondre REGISTER_OK ou REGISTER_ERR selon config
- Permet tester le flow sans Origin réel

---

## 3. Tests E2E

| Outil | Usage |
|-------|-------|
| Émulateur Android | Lancer app |
| ADB | Install, instrumentation |
| Espresso (Kotlin) / UI Automator | Optionnel si Dioxus expose UIA |
| Dioxus | Vérifier support test renderer |

### 3.1 Scénarios critiques

| Scénario | Étapes |
|----------|--------|
| Liaison | Lancer app → écran Liaison → saisie token mock → Salon |
| Consultation | Salon → JayKonta → affichage soldes |
| Offline | Couper réseau → saisie dépense → queue → rétablir → sync |

---

## 4. Couverture

| Objectif | Cible |
|----------|-------|
| Unitaires | 80%+ sur logique critique |
| Intégration | Flux principaux couverts |
| E2E | Happy paths + 2–3 edge cases |

### 4.1 Outil

- `cargo-tarpaulin` ou `cargo-llvm-cov` pour couverture Rust

---

## 5. Outils

| Outil | Usage |
|-------|-------|
| cargo test | Tests Rust |
| cargo clippy | Lint |
| dx | Build, run (manuel E2E) |
| GitHub Actions | Exécution CI |

---

## 6. Références

- [Spec CI CD](./MiyukiniTerminal%20-%20Spec%20CI%20CD.md)
- [cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
