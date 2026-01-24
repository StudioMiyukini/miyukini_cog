# Miyukini Core System — Revue critique des traits publics (API v0.1)

## Contexte

Ce document passe en revue chaque **trait public** du kernel (noms, signatures, types exposés, cohérence avec le contrat) afin de **geler l’API v0.1**. Références : [Definition Kernel](Miyukini%20Core%20System%20-%20Definition%20Kernel.md), [Structure du Kernel](Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md).

---

## 1. Config

### Contrat (Definition + Structure)

- Mécanismes de **chargement** et **d’accès** ; le produit décide des clés et des valeurs.
- Aucune politique (noms de variables, structure métier).
- Env, fichiers, secrets — en v0.1 seule la source « env » est fournie par l’impl par défaut.

### Implémentation actuelle

| Élément | Valeur |
|---------|--------|
| **Trait** | `Config` |
| **Méthodes** | `fn get(&self, key: &str) -> Option<&str>` |
| **Types exposés** | `Config`, `EnvConfig` |
| **Constructeur** | `EnvConfig::from_env() -> Self` |

### Revue

| Critère | Évaluation |
|---------|------------|
| **Nom** | `Config` — aligné contrat, PascalCase. OK. |
| **Signature `get`** | `key: &str` : le produit choisit les clés. `Option<&str>` : accès minimal, pas de typage (int, bool) qui figerait une politique. Le produit parse. OK. |
| **Chargement hors trait** | Le contrat parle de « chargement et accès ». Le **chargement** est dans le constructeur de l’impl (`from_env()`), pas dans le trait. Le trait ne impose pas *comment* charger (fichier, secrets, etc.). Cohérent. |
| **Types** | Pas de type d’erreur de chargement : `std::env::vars()` n’en fournit pas ; le module `error` est Phase 2. OK. |
| **Fichiers / secrets** | Non couverts en v0.1. Le produit peut implémenter un `Config` custom (fichier, vault, etc.). Le kernel reste minimal. OK. |

### Décision

- **Gel** : `Config::get(&self, key: &str) -> Option<&str>`
- **Hors gel (impl)** : `EnvConfig::from_env()` — l’impl peut évoluer ; le trait, non.

---

## 2. IdGenerator

### Contrat (Definition + Structure)

- Génération d’IDs (UUID, ULID, dérivés déterministes pour le jeu).
- Type **Id** défini par le kernel, **opaque** : format interne non garanti, ne pas persister ou interpréter sans passer par les APIs du produit.
- Pas de ré-export de `uuid::Uuid` ou `log::Level`.

### Implémentation actuelle

| Élément | Valeur |
|---------|--------|
| **Trait** | `IdGenerator` |
| **Méthodes** | `fn generate(&self) -> Id` |
| **Types exposés** | `Id`, `IdParseError`, `IdGenerator`, `UuidIdGenerator` |
| **Id** | `Display`, `Debug`, `Clone`, `Copy`, `Eq`, `Hash`, `PartialEq` ; `Id::parse(s: &str) -> Result<Id, IdParseError>` |

### Revue

| Critère | Évaluation |
|---------|------------|
| **Nom** | `IdGenerator` — clair, aligné. OK. |
| **Signature `generate`** | `(&self) -> Id`. Pas de paramètre (ex. seed pour déterministe). Le contrat évoque des « dérivés déterministes pour le jeu » : en v0.1 on reste non déterministe ; le produit peut implémenter un générateur déterministe en dehors du kernel. OK. |
| **Type `Id`** | Champ interne privé (`uuid::Uuid`), pas exposé. `Display` : le produit peut utiliser `to_string()` pour *son* API de persistance. `Id::parse` : round-trip pour la couche produit. Le format supporté (UUID aujourd’hui) peut changer — doc de `parse` le précise. OK. |
| **`IdParseError`** | Type minimal, `Debug` + `Display` + `Error`. Pas de `From<uuid::Error>` dans l’API pour ne pas lier au détail d’impl. OK. |
| **`Debug` pour `Id`** | Aujourd’hui `Id(uuid)`. Le contrat dit « format interne non garanti ». `Debug` sert au développement ; la persistance doit utiliser `Display` / `Id::parse`. **Recommandation** : documenter que `Debug` est à usage dev uniquement, ne pas s’en servir pour persistance. On garde l’impl ; si on préfère une opacité stricte : `Id({{ opaque }})`. Pour le gel : **laisser `Debug` implémenté, comportement considéré comme « best-effort pour le debug »**. |
| **`Clone, Copy, Eq, Hash, PartialEq`** | Bounds techniques (clés, collections), pas du métier. OK. |

### Décision

- **Gel** : `IdGenerator::generate(&self) -> Id`
- **Gel** : `Id` — opaque, `Display`, `Id::parse(&str) -> Result<Id, IdParseError>`, `Clone`, `Copy`, `Eq`, `Hash`, `PartialEq`. `Debug` : défini, à considérer comme aide au debug, pas pour persistance.
- **Gel** : `IdParseError` — `Debug`, `Display`, `Error`.

---

## 3. Clock

### Contrat (Definition + Structure)

- `now()`, timezone, horodatage. Injection en test.
- **Timezone = choix du produit.** `std::time` suffit ; pas de `chrono`.

### Implémentation actuelle

| Élément | Valeur |
|---------|--------|
| **Trait** | `Clock` |
| **Méthodes** | `fn now(&self) -> SystemTime` |
| **Types exposés** | `Clock`, `DefaultClock` |
| **Type de retour** | `std::time::SystemTime` |

### Revue

| Critère | Évaluation |
|---------|------------|
| **Nom** | `Clock` — aligné. OK. |
| **Signature `now`** | `(&self) -> SystemTime`. `SystemTime` est dans `std`, pas une dépendance. Le produit qui a besoin de timezone fait la conversion (chrono, etc.) en dehors du kernel. OK. |
| **Injection en test** | Le produit peut implémenter un `FakeClock` avec `now()` constant. OK. |
| **Horodatage** | Pour un timestamp (ex. secondes depuis epoch), le produit fait `now().duration_since(UNIX_EPOCH)`. Le kernel n’ajoute pas de méthode. OK. |
| **Ré-export `SystemTime`** | Le kernel ne ré-exporte pas `SystemTime` ; le produit l’importe de `std::time` si besoin. Le trait référence ce type — suffisant. OK. |

### Décision

- **Gel** : `Clock::now(&self) -> SystemTime` avec `SystemTime = std::time::SystemTime` (référence std, pas type kernel).

---

## 4. Logger

### Contrat (Definition + Structure)

- Logs par **niveau**, « format structuré (JSON ou clé‑valeur) » — le produit choisit backends et formats.
- Contrat + impl par défaut **minimale et remplaçable** ; pas d’imposition de backend ou de format.
- Le kernel **ne ré-exporte pas** `log::Level` — d’où le type `Level` propre.

### Implémentation actuelle

| Élément | Valeur |
|---------|--------|
| **Trait** | `Logger` |
| **Méthodes** | `fn log(&self, level: Level, message: &str)` |
| **Types exposés** | `Logger`, `Level`, `DefaultLogger` |
| **Level** | `Error`, `Warn`, `Info`, `Debug`, `Trace` |

### Revue

| Critère | Évaluation |
|---------|------------|
| **Nom** | `Logger` — aligné. OK. |
| **Signature `log`** | `(&self, level: Level, message: &str)`. Une seule méthode avec niveau explicite — évite `info`, `warn`, etc. et reste minimal. OK. |
| **`message: &str`** | Le contrat parle de « format structuré (JSON, clé‑valeur) ». Le kernel ne impose pas : le produit peut formatter en JSON (ou autre) avant l’appel ; le trait ne prend que `&str`. OK. |
| **`Level`** | Enum kernel, pas de ré-export de `log::Level`. Valeurs alignées avec la façade log. Pas de niveau custom en v0.1. OK. |
| **`log_with_fields` / structure** | Pas ajouté. Pour rester minimal, le produit qui veut du structuré formate `message` ou implémente un `Logger` custom. **Gel** : une seule méthode `log(level, message)`. |

### Décision

- **Gel** : `Logger::log(&self, level: Level, message: &str)`
- **Gel** : `Level` — `Error`, `Warn`, `Info`, `Debug`, `Trace`.

---

## 5. Lifecycle

### Contrat (Definition + Structure)

- **Boot / shutdown** : ordre d’init, hooks d’**arrêt** des briques techniques.
- **Pas** d’orchestration de workflows métier, **ni** jobs métier, **ni** hooks applicatifs.

### Implémentation actuelle

| Élément | Valeur |
|---------|--------|
| **Trait** | `Lifecycle` |
| **Méthodes** | `fn register_shutdown_hook<F>(&mut self, f: F) where F: FnMut() + 'static` ; `fn shutdown(&mut self)` |
| **Types exposés** | `Lifecycle`, `DefaultLifecycle` |

### Revue

| Critère | Évaluation |
|---------|------------|
| **Nom** | `Lifecycle` — aligné. OK. |
| **Shutdown uniquement** | Le contrat parle d’« ordre d’init » et d’« arrêt ». L’**init** reste au produit (il enchaîne config, log, etc.). Aucun `register_init_hook` dans le kernel — évite de glisser vers de l’orchestration. **Gel** : seulement shutdown. OK. |
| **`register_shutdown_hook`** | `F: FnMut() + 'static`. `FnMut` au lieu de `FnOnce` : contrainte d’exécution en Rust (`Box<dyn FnOnce>`). Les hooks sont appelés une seule fois. `'static` : pas de capture de références éphémères. OK. |
| **`shutdown(&mut self)`** | Exécute les hooks en LIFO. Pas de `Result` ; une panic dans un hook se propage. Comportement d’impl pour un second `shutdown` : non spécifié (no-op pour `DefaultLifecycle`). Documenter. OK pour v0.1. |

### Décision

- **Gel** : `Lifecycle::register_shutdown_hook<F>(&mut self, f: F) where F: FnMut() + 'static`
- **Gel** : `Lifecycle::shutdown(&mut self)`
- **Documentation** : préciser qu’un second appel à `shutdown` est implémentation-dépendant.

---

## 6. Spécification gelée (API v0.1)

Ce qui suit est **figé** pour la v0.1. Tout changement de signature ou de type exposé est un **breaking change** à documenter et versionner.

### config

```
trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}
```

- Types publics : `Config`, `EnvConfig`.
- `EnvConfig::from_env() -> Self` : constructeur, hors contrat du trait ; l’impl peut évoluer.

### id

```
trait IdGenerator {
    fn generate(&self) -> Id;
}

struct Id { /* opaque */ }
impl Id {
    pub fn parse(s: &str) -> Result<Id, IdParseError>;
}
// Display, Debug, Clone, Copy, Eq, Hash, PartialEq pour Id

struct IdParseError;
// Debug, Display, Error pour IdParseError
```

- Types publics : `Id`, `IdParseError`, `IdGenerator`, `UuidIdGenerator`.

### time

```
trait Clock {
    fn now(&self) -> SystemTime;  // std::time::SystemTime
}
```

- Types publics : `Clock`, `DefaultClock`.
- Le kernel ne ré-exporte pas `SystemTime`.

### log

```
enum Level { Error, Warn, Info, Debug, Trace }

trait Logger {
    fn log(&self, level: Level, message: &str);
}
```

- Types publics : `Level`, `Logger`, `DefaultLogger`.

### lifecycle

```
trait Lifecycle {
    fn register_shutdown_hook<F>(&mut self, f: F)
    where
        F: FnMut() + 'static;

    fn shutdown(&mut self);
}
```

- Types publics : `Lifecycle`, `DefaultLifecycle`.

---

## 7. Synthèse des décisions

| Trait | Méthodes gelées | Types gelés | Remarques |
|-------|-----------------|-------------|-----------|
| **Config** | `get(&self, key: &str) -> Option<&str>` | `Config`, `EnvConfig` | Pas de `load` dans le trait. |
| **IdGenerator** | `generate(&self) -> Id` | `Id`, `IdParseError`, `IdGenerator`, `UuidIdGenerator` | `Id::parse` ; `Debug` = usage dev. |
| **Clock** | `now(&self) -> SystemTime` | `Clock`, `DefaultClock` | `SystemTime` = std. |
| **Logger** | `log(&self, level: Level, message: &str)` | `Level`, `Logger`, `DefaultLogger` | Une seule méthode ; pas de `log_with_fields`. |
| **Lifecycle** | `register_shutdown_hook`, `shutdown` | `Lifecycle`, `DefaultLifecycle` | Shutdown uniquement ; pas d’init. |

---

## 8. À ne pas faire en v0.1

- Ajouter `Config::load`, `Config::get_i64`, `Config::get_bool`, etc.
- Ajouter `IdGenerator::generate_with_seed` ou paramètres.
- Ajouter un type `Timestamp` ou des méthodes de conversion dans `Clock`.
- Ajouter `Logger::info`, `Logger::warn`, `Logger::log_with_fields`, ou un type `Fields`.
- Ajouter `Lifecycle::register_init_hook` ou `Lifecycle::run`.
- Ré-exporter `uuid::Uuid`, `log::Level`, ou `log::Record`.
- Changer les signatures ou les types listés sans procédure de breaking change documentée.

Ce document doit être mis à jour si un trait ou un type gelé est modifié.
