---
name: miyukini-testing
description: Patterns de test du projet Miyukini COG. Tests unitaires (module tests, helpers), cycle MiyukiniSQLtest (test E2E MiyuSQL), test runners (jaykoa), tests d'integration KindMother Client. Utiliser quand on ecrit des tests, quand on ajoute un test runner, ou quand on travaille sur le cycle MiyukiniSQLtest.
---

# Testing — Patterns Miyukini COG

## Structure des tests

### Tests unitaires (dans chaque fichier)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn mem_db() -> ServiceDb {
        ServiceDb::open(":memory:").unwrap()
    }

    fn make_entity() -> Entity {
        Entity {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Test".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_create_and_read() {
        let db = mem_db();
        let entity = make_entity();
        db.entity_create(&entity).unwrap();
        let result = db.entity_by_id(&entity.id).unwrap();
        assert!(result.is_some());
    }
}
```

### Tests dans `tests/` (integration)

```
crates/{service}/
└── tests/
    ├── unit_tests.rs           # Tests unitaires supplementaires
    └── cycle_miyukinisqltest.rs # Cycle E2E MiyuSQL (si applicable)
```

## Helpers de test recurrents

| Helper | Pattern |
|--------|---------|
| `mem_db()` | Cree une DB en memoire (`:memory:`) |
| `make_{entity}()` | Cree une entite de test |
| `test_ctx()` | Cree un `GovernedContext` de test |
| `setup()` | Prepare l'environnement de test |

## Cycle MiyukiniSQLtest

Test E2E complet pour MiyuSQL :

```rust
#[test]
fn cycle_miyukinisqltest() {
    let ctx = GovernedContext::new("test-mandate".into(), 1);
    let executor = MemoryExecutor::new();

    // 1. CREATE TABLE
    let res = executor.execute(&ctx, "CREATE TABLE MiyukiniSQLtest (...)", &[]);
    assert!(res.is_ok());

    // 2. ALTER TABLE ADD COLUMN
    let res = executor.execute(&ctx, "ALTER TABLE ... ADD COLUMN ...", &[]);
    assert!(res.is_ok());

    // 3. INSERT
    let res = executor.execute(&ctx, "INSERT INTO MiyukiniSQLtest ...", &params);
    assert!(res.is_ok());

    // 4. SELECT
    let res = executor.execute(&ctx, "SELECT * FROM MiyukiniSQLtest", &[]);
    assert!(res.unwrap().count > 0);

    // 5. DELETE
    let res = executor.execute(&ctx, "DELETE FROM MiyukiniSQLtest", &[]);
    assert!(res.is_ok());

    // 6. DROP TABLE
    let res = executor.execute(&ctx, "DROP TABLE MiyukiniSQLtest", &[]);
    assert!(res.is_ok());
}
```

## Test runner (JayKoa)

Pattern de test runner integre au service :

```rust
pub struct TestRunner {
    db: Arc<ServiceDb>,
}

impl TestRunner {
    pub fn run_all(&self) -> Vec<TestResult> {
        vec![
            self.test_connection(),
            self.test_schema(),
            self.test_crud(),
        ]
    }
}
```

## Tests KindMother Client

```rust
#[test]
fn test_client_connection() {
    // Necessite kindmother-service en cours d'execution
    let client = KindMotherClient::connect("127.0.0.1:50051", "test", "test");
    assert!(client.is_ok());
}

#[test]
fn test_client_execute() {
    let client = setup_client();
    let result = get_runtime().block_on(
        client.execute("SELECT 1", vec![], "test_query")
    );
    assert!(result.is_ok());
}
```

## Convention de nommage

| Pattern | Exemple |
|---------|---------|
| `test_{action}` | `test_create_event` |
| `test_{action}_{condition}` | `test_create_event_duplicate_id` |
| `test_{action}_{expected}` | `test_sign_in_invalid_password` |
| `cycle_{nom}` | `cycle_miyukinisqltest` |

## Annotations MSCM dans les tests

```rust
/// @id: service_test_creation
/// @role: test
/// @layer: service
/// @human: Test de creation d'une entite.
/// @do: verify_entity_creation
#[test]
fn test_create_entity() { ... }
```

## Commandes

```bash
# Tous les tests du workspace
cargo test --workspace

# Tests d'un crate specifique
cargo test -p miyusql

# Tests avec sortie verbose
cargo test -p jayfestival -- --nocapture

# Tests correspondant a un pattern
cargo test -p miyusql cycle_
```

## Regles

1. **DB en memoire** : utiliser `:memory:` pour tests unitaires
2. **Helpers partages** : `mem_db()`, `make_*()` dans chaque module test
3. **Pas d'effets de bord** : chaque test est independant
4. **Annotations MSCM** : `@role: test` pour les tests
5. **GovernedContext** : toujours fournir un mandat valide pour les tests MiyuSQL
6. **Cycle MiyukiniSQLtest** : respecter la sequence complete (CREATE → ALTER → INSERT → SELECT → DELETE → DROP)

## References

- **Tests MiyuSQL** : `crates/miyusql/tests/`
- **Tests KindMother** : `crates/kindmother-client/tests/`
- **Tests MiyAuth** : `crates/miyauth/tests/`
- **Manifeste de test** : via `admin_cell.rs` de chaque toolkit
