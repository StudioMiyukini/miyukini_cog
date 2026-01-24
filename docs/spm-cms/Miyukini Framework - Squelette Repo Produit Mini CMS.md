# Squelette du repo produit mini CMS

> Structure recommandée pour un produit CMS minimal consommant le SPM CMS.
> Exemple de référence pour valider l'architecture SPM.

---

## Contexte

Ce squelette définit la structure d'un **produit CMS minimal** qui consomme le SPM CMS. Il sert de référence pour :

- Valider que le SPM peut être consommé par un produit réel
- Démontrer l'articulation SPM ↔ Produit
- Tester les modules Phase 0 (et suivantes)

**Principe :** Le produit CMS ajoute la logique métier spécifique (SEO, templates, intégrations) au-dessus du SPM.

---

## Structure de dépôt

```
cms-mini/
├── Cargo.toml                    # Workspace ou crate unique
├── README.md                      # Documentation produit
│
├── crates/                        # Si workspace
│   ├── cms-mini-core/            # Crate principale
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # Point d'entrée
│   │       ├── lib.rs            # API publique (si library)
│   │       ├── config.rs         # Configuration produit
│   │       ├── adapters/         # Adaptateurs SPM → stack produit
│   │       │   ├── content.rs    # Adaptateur Module Contenu
│   │       │   ├── hierarchy.rs  # Adaptateur Module Hiérarchie
│   │       │   └── taxonomies.rs # Adaptateur Module Taxonomies
│   │       ├── domain/           # Logique métier CMS
│   │       │   ├── seo.rs        # SEO, meta tags
│   │       │   ├── templates.rs  # Templates de rendu
│   │       │   └── integrations.rs # Intégrations externes
│   │       └── persistence/      # Persistance (si nécessaire)
│   │           └── repository.rs # Repository DB (produit)
│   │
│   └── cms-mini-api/             # API HTTP (optionnel)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── routes.rs         # Routes HTTP
│           └── handlers.rs       # Handlers API
│
└── docs/
    ├── README.md                  # Documentation produit
    ├── architecture.md            # Architecture produit
    └── integration-spm.md         # Guide d'intégration SPM
```

---

## Dépendances (Cargo.toml)

### Crate principale

```toml
[package]
name = "cms-mini-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Kernel
miyukini-kernel = { path = "../../crates/miyukini-kernel" }

# SPM CMS
miyukini-spm-cms = { path = "../../crates/miyukini-spm-cms" }

# Stack technique (choix du produit)
# Exemples (à adapter) :
# sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
# tokio = { version = "1", features = ["full"] }
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"
```

**Principe :** Le produit choisit sa stack technique. Le SPM et le kernel n'imposent rien.

---

## Adaptateurs SPM → Stack produit

### Exemple : Adaptateur Module Contenu

**Fichier :** `src/adapters/content.rs`

**Responsabilité :** Adapter le contrat fonctionnel du Module Contenu vers la stack technique du produit (DB, sérialisation, etc.).

**Structure :**
```rust
// Exemple conceptuel (pas d'implémentation complète)
use miyukini_spm_cms::content::ContentManager;
use miyukini_kernel::{Id, Clock};

pub struct ContentAdapter {
    // Repository DB (choix du produit)
    // db: Database,
    // clock: Clock,
}

impl ContentAdapter {
    pub fn new(/* db: Database, clock: Clock */) -> Self {
        // Initialisation
    }
}

// Implémentation du trait ContentManager du SPM
impl ContentManager for ContentAdapter {
    fn create_content(&self, input: ContentInput) -> Result<ContentId, ContentError> {
        // 1. Utiliser kernel pour générer ID et horodatage
        // 2. Adapter vers format DB
        // 3. Persister
        // 4. Retourner ID
    }
    
    // ... autres opérations
}
```

**Principe :** L'adaptateur traduit les contrats fonctionnels du SPM vers l'implémentation technique du produit.

---

## Logique métier CMS (produit)

### Exemple : SEO

**Fichier :** `src/domain/seo.rs`

**Responsabilité :** Logique métier spécifique CMS (meta tags, sitemaps, etc.).

**Hors-scope SPM :** Le SPM ne connaît pas le SEO. C'est la responsabilité du produit.

**Structure :**
```rust
// Exemple conceptuel
pub struct SeoService {
    // Dépend du Module Contenu (SPM) pour lire les contenus
    // content_manager: Arc<dyn ContentManager>,
}

impl SeoService {
    pub fn generate_meta_tags(&self, content_id: ContentId) -> MetaTags {
        // 1. Lire le contenu via SPM
        // 2. Générer meta tags (logique métier produit)
        // 3. Retourner
    }
    
    pub fn generate_sitemap(&self) -> Sitemap {
        // Logique métier produit
    }
}
```

---

## Point d'entrée

**Fichier :** `src/main.rs`

**Responsabilité :** Initialisation du produit, intégration kernel + SPM, démarrage.

**Structure :**
```rust
// Exemple conceptuel
use miyukini_kernel::{Config, Logger, Clock, IdGenerator, Lifecycle};
use miyukini_spm_cms::content::ContentManager;

fn main() {
    // 1. Initialiser kernel
    let config = EnvConfig::from_env();
    let logger = /* logger produit */;
    let clock = DefaultClock::new();
    let id_gen = UuidIdGenerator::new();
    let mut lifecycle = DefaultLifecycle::new();
    
    // 2. Initialiser adaptateurs SPM
    let content_adapter = ContentAdapter::new(/* ... */);
    let hierarchy_adapter = HierarchyAdapter::new(/* ... */);
    let taxonomies_adapter = TaxonomiesAdapter::new(/* ... */);
    
    // 3. Initialiser services métier produit
    let seo_service = SeoService::new(/* ... */);
    
    // 4. Démarrer produit (routes HTTP, workers, etc.)
    // ...
    
    // 5. Shutdown propre
    lifecycle.shutdown();
}
```

---

## Règles d'organisation

### Séparation des responsabilités

**SPM :** Contrats fonctionnels, logique générique.

**Produit :**
- Adaptateurs SPM → stack technique
- Logique métier spécifique (SEO, templates, intégrations)
- Persistance et infrastructure
- API et UI

### Dépendances

**Flux :** Produit → SPM → Kernel

**Interdiction :** SPM ne dépend jamais du produit. Kernel ne dépend jamais du SPM ni du produit.

### Tests

**Tests unitaires :** Chaque adaptateur, chaque service métier.

**Tests d'intégration :** Produit complet avec SPM + kernel.

**Tests E2E :** Scénarios utilisateur complets.

---

## Exemple de scénario complet

### Créer une page CMS

```
1. Utilisateur → API produit (POST /pages)
2. Handler produit → ContentAdapter.create_content()
3. ContentAdapter → SPM ContentManager.create()
4. SPM → Kernel (Id.generate(), Clock.now())
5. ContentAdapter → DB (persister)
6. Handler → Logique métier (SEO, templates)
7. Handler → Réponse API
```

**Séparation :**
- **Kernel :** ID, horodatage
- **SPM :** Contrat création contenu
- **Produit :** Persistance DB, logique métier, API

---

## Critères de validation

**Architecture :**
- Dépendances unidirectionnelles respectées
- Adaptateurs SPM clairement identifiés
- Logique métier séparée du SPM

**Fonctionnel :**
- Produit peut créer/organiser/classifier des contenus
- Produit peut ajouter logique métier (SEO, templates)
- Produit peut exposer une API fonctionnelle

**Technique :**
- Intégration kernel validée
- Intégration SPM validée
- Stack technique du produit fonctionnelle

---

## Notes

**Ce squelette est un exemple.** Chaque produit adapte selon ses besoins (stack technique, architecture, organisation).

**Objectif :** Démontrer que le SPM peut être consommé par un produit réel sans couplage fort ni logique métier spécifique.

**Évolution :** Le squelette évolue avec les phases du SPM (ajout de modules Phase 1, 2, 3).
