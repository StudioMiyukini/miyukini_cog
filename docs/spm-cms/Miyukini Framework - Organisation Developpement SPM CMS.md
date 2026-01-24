# Organisation du développement SPM CMS

> Plan d'organisation modulaire et incrémentale pour développer un Socle Produit Mutualisé (SPM) dédié à un mini CMS.
> Réutilisable par ≥3 types de produits, avec frontières claires kernel/SPM/produit et règles d'évolution anti-bloat.

---

## Contexte

Le SPM CMS est un socle produit mutualisé construit **au-dessus du kernel technique minimal** (infra uniquement). Il fournit des capacités CMS génériques réutilisables par plusieurs produits (CMS, Event, SaaS, Jeux) sans logique métier spécifique.

**Principe :** Le SPM n'est PAS un framework, PAS un produit final. Il expose des contrats fonctionnels que les produits consomment et adaptent à leur contexte.

---

## 1. Découpage du SPM en modules produit

### Modules fondation (Phase 0)

**Module Contenu**
- **Responsabilité :** Gestion des entités de contenu (pages, articles, blocs) : CRUD, statuts (brouillon/publié/archivé), relations, versioning, métadonnées.
- **Dépendances :** Kernel (Id, Clock, Logger), Module Utilisateurs (optionnel pour attribution)

**Module Hiérarchie**
- **Responsabilité :** Organisation hiérarchique des contenus (arborescence, navigation, breadcrumbs).
- **Dépendances :** Module Contenu

**Module Taxonomies**
- **Responsabilité :** Classification des contenus (catégories, tags, taxonomies personnalisées).
- **Dépendances :** Module Contenu

### Modules cœur (Phase 1)

**Module Références Média**
- **Responsabilité :** Référencement de médias (images, vidéos, fichiers) : métadonnées, relations média ↔ contenu, états fonctionnels. Le stockage réel reste dans le produit ou dans un module infra futur (hors SPM).
- **Dépendances :** Kernel (Id, Clock, Logger)

**Module Publication**
- **Responsabilité :** Cycle de publication : brouillons, prévisualisation, publication programmée, états de workflow génériques. **Garde-fou :** Le module ne contient que des états et transitions génériques. Toute règle métier conditionnelle (ex. validation par X rôles) appartient au produit.
- **Dépendances :** Module Contenu, Kernel (Clock)

**Module Blocs**
- **Responsabilité :** Système de blocs de contenu modulaires et réutilisables.
- **Dépendances :** Module Contenu, Module Références Média

### Modules complémentaires (Phase 2)

**Module Recherche**
- **Responsabilité :** Recherche fonctionnelle, full-text, filtres, tris sur les contenus.
- **Dépendances :** Module Contenu, Module Taxonomies

**Module Historique**
- **Responsabilité :** Traçabilité des opérations sur contenus : historique des modifications, audit.
- **Dépendances :** Module Contenu, Kernel (Clock)

### Modules optionnels (Phase 3)

**Module Thèmes (structure)**
- **Responsabilité :** Structure fonctionnelle des thèmes (pas le design) : zones, layouts, régions.
- **Dépendances :** Module Contenu, Module Blocs

**Module Import/Export**
- **Responsabilité :** Import/export de contenus, mapping fonctionnel, synchronisation.
- **Dépendances :** Module Contenu, Module Taxonomies

---

## 2. Ordre de développement recommandé

### Phase 0 : Fondations SPM (MVP minimal)

**Modules :** Contenu, Hiérarchie, Taxonomies

**Justification :** Permet de créer et organiser des contenus de base. Minimum viable pour valider l'architecture SPM et tester avec un produit pilote CMS.

**Validation :** Un produit CMS peut créer des pages/articles, les organiser en hiérarchie, les classifier avec des taxonomies.

---

### Phase 1 : Modules cœur (fonctionnalités CMS essentielles)

**Modules :** Références Média, Publication, Blocs

**Justification :** Complète les capacités CMS de base. Références Média est mutualisable (5 domaines). Publication et Blocs sont spécifiques CMS mais indispensables.

**Validation :** Le produit CMS peut référencer des médias, publier des contenus avec workflow, composer des pages avec des blocs.

---

### Phase 2 : Modules complémentaires (amélioration UX et traçabilité)

**Modules :** Recherche, Historique

**Justification :** Recherche améliore la découverte de contenus. Historique est mutualisable (6 domaines) et nécessaire pour l'audit.

**Validation :** Le produit CMS peut rechercher des contenus efficacement et tracer toutes les modifications.

---

### Phase 3 : Modules optionnels (features avancées)

**Modules :** Thèmes (structure), Import/Export

**Justification :** Thèmes structurels si besoin de personnalisation poussée. Import/Export si besoin d'interopérabilité. À développer uniquement si validé par ≥2 produits.

**Validation :** Produits CMS peuvent personnaliser la structure de rendu et échanger des contenus avec d'autres systèmes.

---

## 3. Règles d'évolution du SPM

### Quand ajouter un module

1. **Validation produit :** ≥2 produits distincts (CMS, Event, SaaS, etc.) ont besoin de la même capacité.
2. **Généricité :** La capacité peut être formulée sans logique métier spécifique.
3. **Indépendance :** Le module peut être consommé sans imposer de dépendances lourdes.
4. **Pérennité :** La capacité est stable et ne changera pas radicalement dans les 2-3 ans.

### Quand refuser une généralisation

1. **Spécificité métier :** La capacité contient de la logique métier propre à un seul domaine (ex. SEO, référencement).
2. **Couplage fort :** La capacité nécessite des dépendances techniques lourdes (ex. moteur de rendu, framework UI).
3. **Anticipation :** Aucun produit réel n'a encore besoin de la capacité.
4. **Bloat potentiel :** La capacité ajouterait de la complexité sans bénéfice clair pour ≥3 produits.

### Quand laisser une feature au produit CMS

1. **Rendu visuel :** Templates, thèmes visuels, CSS, design system.
2. **SEO technique :** Meta tags, sitemaps, robots.txt, optimisations référencement.
3. **Workflow métier :** Circuits d'approbation spécifiques, règles de publication métier conditionnelles (ex. validation par X rôles, règles métier complexes).
4. **Intégrations spécifiques :** Connecteurs tiers (ex. Google Analytics, CDN), APIs externes.
5. **UI/UX :** Interface d'administration, éditeurs WYSIWYG, prévisualisation en temps réel.

---

## 4. Articulation SPM ↔ Produits

### Comment un produit CMS consomme le SPM

**Principe :** Le produit CMS dépend du SPM. Le SPM dépend du kernel. Dépendance unidirectionnelle stricte.

**Exemple d'usage :**
```
Produit CMS
  ├─ Module Contenu (SPM) → crée/gère pages/articles
  ├─ Module Références Média (SPM) → référence assets
  ├─ Module Publication (SPM) → gère workflow publication
  └─ Logique métier CMS (produit)
      ├─ SEO, meta tags
      ├─ Templates de rendu
      └─ Intégrations externes
```

**Contrat SPM :** Le SPM expose des contrats fonctionnels. Ces contrats peuvent être représentés par des traits/types (ex. `ContentManager`, `MediaReferenceManager`, `Publisher`) dans une implémentation Rust, mais le contrat est d'abord fonctionnel, pas technique. Le produit implémente les adaptateurs vers sa stack technique (DB, HTTP, etc.).

### Comment un autre produit (Event, SaaS, Jeu) pourrait consommer le SPM

**Produit Event :**
- Module Contenu → gère descriptions d'événements
- Module Taxonomies → catégorise événements (type, genre, public)
- Module Références Média → référence photos/vidéos d'événements
- Module Hiérarchie → organise programmation (scènes, horaires)

**Produit SaaS :**
- Module Contenu → gère pages de documentation, articles de blog
- Module Publication → gère workflow de publication de contenu
- Module Recherche → permet de rechercher dans la documentation

**Produit Jeu :**
- Module Contenu → gère descriptions de niveaux, guides
- Module Références Média → référence assets graphiques, sons
- Module Taxonomies → catégorise contenus (niveaux, items, quêtes)

**Principe de non-dépendance inverse :** Le SPM ne connaît pas les produits. Il expose uniquement des contrats fonctionnels. Les produits adaptent ces contrats à leur contexte métier.

---

## 5. Livrables attendus pour chaque module

### Contrat fonctionnel

**Format :** Document markdown dans `docs/spm-cms/modules/<nom-module>/contrat.md`

**Contenu minimal :**
- Responsabilité du module (1 phrase)
- Contrats d'opérations (créer, lire, modifier, supprimer, etc.)
- Contraintes et invariants
- Exemples d'usage (pseudo-code)

**Note importante :** Les traits/types sont une représentation possible du contrat dans une implémentation Rust. Le contrat est d'abord fonctionnel, pas technique. Cela protège contre les changements de langage, d'exposition ou de runtime futur.

**Exemple (Module Contenu) :**
```rust
// Contrat fonctionnel (représentation Rust possible, pas d'implémentation)
pub trait ContentManager {
    fn create_content(&self, content: ContentInput) -> Result<ContentId, ContentError>;
    fn get_content(&self, id: ContentId) -> Result<Content, ContentError>;
    fn update_content(&self, id: ContentId, updates: ContentUpdates) -> Result<(), ContentError>;
    fn delete_content(&self, id: ContentId) -> Result<(), ContentError>;
    fn list_contents(&self, filters: ContentFilters) -> Result<Vec<Content>, ContentError>;
}
```

### Documentation minimale

**Format :** Document markdown dans `docs/spm-cms/modules/<nom-module>/README.md`

**Contenu minimal :**
- Description du module
- Cas d'usage principaux
- Dépendances (autres modules SPM, kernel)
- Guide d'intégration pour un produit
- Exemples de consommation

### Démo ou produit pilote validant le module

**Format :** Crate Rust `demos/cms-<nom-module>` ou intégration dans un produit pilote existant

**Contenu minimal :**
- Démo fonctionnelle montrant le module en action
- Validation des contrats
- Tests d'intégration avec le kernel
- Documentation d'exécution

**Critère de validation :** Le module peut être consommé par un produit CMS minimal qui démontre la capacité fonctionnelle.

---

## 6. Structure de dépôt recommandée

```
miyukini-core-system/
├── crates/
│   ├── miyukini-kernel/          # Kernel (existant, gelé)
│   └── miyukini-spm-cms/         # SPM CMS
│       ├── content/              # Module Contenu
│       ├── hierarchy/            # Module Hiérarchie
│       ├── taxonomies/           # Module Taxonomies
│       ├── media-references/     # Module Références Média
│       ├── publication/          # Module Publication
│       ├── blocks/               # Module Blocs
│       ├── search/               # Module Recherche
│       ├── history/              # Module Historique
│       ├── themes/               # Module Thèmes (structure)
│       └── import-export/        # Module Import/Export
├── demos/
│   └── cms-pilot/                # Produit pilote CMS
└── docs/
    └── spm-cms/
        ├── README.md             # Vue d'ensemble SPM CMS
        └── modules/
            └── <nom-module>/
                ├── contrat.md
                └── README.md
```

---

## 7. Critères de succès par phase

**Phase 0 :** Un produit CMS minimal peut créer, organiser et classifier des contenus.

**Phase 1 :** Un produit CMS complet peut référencer des médias, publier des contenus avec workflow et composer des pages avec blocs.

**Phase 2 :** Un produit CMS peut rechercher efficacement et tracer toutes les opérations.

**Phase 3 :** Un produit CMS peut personnaliser la structure de rendu et échanger des contenus.

**Critère transversal :** ≥2 produits distincts (CMS + Event ou CMS + SaaS) consomment au moins un module du SPM.
