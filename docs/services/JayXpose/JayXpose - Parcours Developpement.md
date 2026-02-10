# JayXpose — Parcours de Développement

> **Template v1.0** — Service complexe multi-modules  
> **Date de création :** 2026-02-07  
> **Responsable :** Miyukini Team  
> **Type d'entité :** [X] Service | [ ] Opérateur | [ ] Toolkit | [ ] Outil  
> **Strate cible :** [ ] 6 (Outils) | [X] 7 (Opérateurs) | [ ] 9 (Admin)

---

## 📋 Vue d'ensemble

**Description en une phrase :**  
JayXpose est le **WordPress + Elementor + WooCommerce + Figma** de l'écosystème Miyukini : identité professionnelle, page builder complet, catalogue produits avec sync stocks MiyukiniPoS, et sites vitrine/e-shop/service-shop personnalisables.

**Objectif principal :**  
Permettre à des exposants (artisans, artistes, marques, entreprises) de créer et gérer une présence professionnelle en ligne complète avec mini-site, catalogue de produits, e-shop, et intégration dans l'écosystème Jay (JayFestival, JayKonta, MiyukiniPoS).

**Équivalent fonctionnel :**
| Fonctionnalité | Équivalent Externe |
|----------------|-------------------|
| Page Builder | **Elementor** (drag & drop, blocs visuels) |
| CMS | **WordPress** (pages, articles, médias) |
| Catalogue & E-shop | **WooCommerce** (produits, stocks, commandes) |
| Design System | **Figma** (composants, styles, preview) |
| Sync Stocks | **MiyukiniPoS** (inventaire centralisé) |

**Non-objectifs :**
- ❌ Boutique marketplace (vente multi-vendeurs) — renvoi vers Miyustore
- ❌ Hébergement domaine personnalisé (alpha)
- ❌ Blog intégré (won't have cette version)
- ❌ Paiement en ligne intégré (renvoi vers JayKonta)
- ❌ Avis/témoignages (won't have)
- ❌ JavaScript/CSS personnalisé (sécurité)

**Statut actuel :** [ ] Idéation | [X] Conception | [ ] Implémentation | [ ] Raffinement | [ ] Livré

---

## 📌 Ordre des tâches à faire

Liste des tâches restantes, dans l’ordre recommandé du parcours (à mettre à jour au fil de l’avancement).

**Déjà fait :** Auth/session multi-exposants (profil Central ↔ exposant), Module M8 CMS Articles (types, persistance, CRUD, XP-E13/E14, intégration dashboard). Le présent ordre reprend uniquement ce qui reste à faire.

### Priorité immédiate (backlog)

| # | Tâche | Phase | Réf. |
|---|--------|--------|------|
| 1 | **Sync PoS bidirectionnelle complète** (pull + conflits) | 3 / M7 | Backlog |
| 2 | **Guide utilisateur et FAQ** (Phase 5) | 5 | Backlog |

### Phase 2 — Architecture & Contrats (à finaliser)

| # | Tâche | Section |
|---|--------|---------|
| 3 | Schéma d’architecture (Mermaid), flux de données, décisions techniques | 2.1 Architecture |
| 4 | **Page Builder** : architecture, types de blocs, drag & drop, stockage JSON, moteur de rendu, preview, templates | 2.1 M3 |
| 5 | **Sync MiyukiniPoS** : protocole, mapping catalogue ↔ inventaire, conflits, modes (temps réel / batch / manuel) | 2.1 M7 |
| 6 | M8 CMS : types de contenu, workflow publication, intégration Page Builder (doc) | 2.1 M8 |
| 7 | Plan persistance : `vitrine_blocs`, `vitrine_templates`, stratégie backup/restore | 2.1 Plan Persistance |
| 8 | **UX/UI Page Builder** : workflow, catalogue de blocs, panneau propriétés, styles, responsive, undo/redo | 2.2 |
| 9 | Wireframes Page Builder (éditeur, sidebar, toolbar, preview, responsive) | 2.2 |
| 10 | Design System (palette, typo, composants, grid, icônes) | 2.2 |
| 11 | Wireframes publics (PUB-E01 à PUB-E08) | 2.2 |
| 12 | **Contrats** : API Page Builder, API Sync MiyukiniPoS | 2.3 |
| 13 | Permissions Page Builder, tests d’acceptation Page Builder & Sync PoS, edge cases conflits sync | 2.3 |
| 14 | Livrables Phase 2 : Contrats Page Builder, Contrats Sync MiyukiniPoS, Tests Acceptation Complets | 2.3 |

### Phase 3 — Implémentation (itératif)

| # | Tâche | Itération |
|---|--------|-----------|
| 15 | Structure crate (admin_cell, context, errors, lib, modules profil/catalogue/builder/vitrine/documents/annuaire/sync_pos/cms) | It.1 |
| 16 | M1 : tests, CRUD exposants, contacts, visuels, réseaux sociaux, juridique, SIRET, UI XP-E01/E02 | It.1 |
| 17 | M2 : tests, schéma produits/catégories/visuels, CRUD, upload, vedettes, recherche, UI XP-E03/E04/E05 | It.1 |
| 18 | **M3 Page Builder MVP** : tests, blocs, drag & drop, panneau propriétés, JSON, rendu, preview, undo/redo, 4 templates de base | It.2 |
| 19 | M4 Vitrine : tests, pages (Accueil, Catalogue, Présentation, Contact), slug, SEO, responsive, statuts, formulaire contact, UI XP-E06/E07/E08, pages PUB | It.3 |
| 20 | M5 Coffre-Fort : tests, upload, versioning, statuts, alertes expiration, partage gouverné, WorrySentinel, UI XP-E09/E10/E11 | It.3 |
| 21 | M6 Annuaire : tests, filtres, recherche, fiche publique, JayFestival, UI PUB-E01/E02, XP-E12 | It.4 |
| 22 | **M7 Sync MiyukiniPoS** : tests, protocole, mapping, conflits, modes, logs, UI config sync | It.4 |
| 23 | M8 CMS : tests, intégration Page Builder, catégories/tags, pagination, polish (performance, accessibilité) | It.5 (déjà partiellement fait) |
| 24 | **Qualité** : types de tests (unitaires, intégration, gouvernance, persistance, UX, perf, sécurité, sync), validation, rapport | 3.2 |
| 25 | **Intégration** : enregistrement opérateur, permissions, JayFestival, JayKonta, MiyukiniPoS, docs d’intégration | 3.3 |

### Phase 4 — Raffinement

| # | Tâche | Section |
|---|--------|---------|
| 26 | Gamification : niveaux, badges, feedback, dashboard de progression, livrable Systeme Gamification | 4.1 |
| 27 | Polish UI : animations, responsive, thème, accessibilité WCAG 2.1 AA, rapport accessibilité | 4.2 |

### Phase 5 — Livraison

| # | Tâche | Section |
|---|--------|---------|
| 28 | **Documentation utilisateur** : Guide démarrage, Guide Page Builder, Guide Catalogue, Guide Coffre-Fort, FAQ, tutoriels | 5.1 |
| 29 | **Documentation technique** : Architecture finale, API Reference, Guide maintenance, index MIP | 5.2 |
| 30 | **Release** : tous tests verts, doc complète, conformité, linter, performance | 5.3 |

---

## 🏗️ Architecture Modulaire

JayXpose est structuré en **8 modules** qui peuvent être développés de manière relativement indépendante :

```
┌─────────────────────────────────────────────────────────────────────┐
│                          JayXpose Service                            │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌─────────────┐│
│  │   Module 1   │ │   Module 2   │ │   Module 3   │ │  Module 4   ││
│  │   Profil     │ │  Catalogue   │ │ Page Builder │ │   Vitrine   ││
│  │  Entreprise  │ │  Produits    │ │  (Elementor) │ │  (Sites)    ││
│  └──────────────┘ └──────────────┘ └──────────────┘ └─────────────┘│
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌─────────────┐│
│  │   Module 5   │ │   Module 6   │ │   Module 7   │ │  Module 8   ││
│  │  Coffre-Fort │ │  Annuaire    │ │  Sync        │ │  CMS        ││
│  │  Documents   │ │  Exposants   │ │  MiyukiniPoS │ │  Articles   ││
│  └──────────────┘ └──────────────┘ └──────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│                    Intégrations Externes                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                │
│  │ JayFestival  │ │  JayKonta    │ │ MiyukiniPoS  │                │
│  │  (Sync)      │ │ (Facturation)│ │  (Stocks)    │                │
│  └──────────────┘ └──────────────┘ └──────────────┘                │
└─────────────────────────────────────────────────────────────────────┘
```

### Modules et Priorités

| Module | Nom | Description | Priorité | Dépendances |
|--------|-----|-------------|----------|-------------|
| **M1** | Profil Entreprise | Fiche entreprise enrichie, contacts, juridique | P0 | — |
| **M2** | Catalogue Produits | Fiches produits, catégories, visuels, stocks | P0 | M1 |
| **M3** | Page Builder | Éditeur drag & drop type Elementor | P0 | M1 |
| **M4** | Vitrine & Sites | Mini-sites, e-shop, service-shop | P1 | M1, M2, M3 |
| **M5** | Coffre-Fort Documents | Documents professionnels, partage gouverné | P1 | M1 |
| **M6** | Annuaire Exposants | Référencement, filtres, fiche publique | P2 | M1, M2 |
| **M7** | Sync MiyukiniPoS | Synchronisation stocks, catalogue centralisé | P2 | M2 |
| **M8** | CMS Articles | Gestion articles/blog, contenu éditorial | P3 | M3 |

---

## Phase 1 : Conception & Fondations 🎯

### 1.1 Idéation & Cadrage

**Date de début :** 2026-01-15  
**Date de fin :** 2026-01-25

#### Checklist
- [✅] Idée formulée et documentée
- [✅] Type d'entité identifié (Service JayXpose, Strate 7)
- [✅] Strate dans la pyramide confirmée
- [✅] Conformité aux 8 Lois d'Autonomie vérifiée
  - [✅] LOI-1 : Aucune dépendance externe critique (DB locale KindMother)
  - [✅] LOI-2 : Accepte l'isolement comme état normal (vitrine offline possible)
  - [✅] LOI-3 : État local souverain
  - [✅] LOI-4 : Pas de temps global requis
  - [✅] LOI-5 : Coût proportionnel au hardware
  - [✅] LOI-6 : Autonomie compatible avec fédération (sync MiyukiniPoS)
  - [✅] LOI-7 : Strate Cores immuable
  - [✅] LOI-8 : Migration = diplomatie entre environnements
- [✅] Portée définie et bornée
- [✅] Architecture modulaire définie (8 modules)

#### Livrables
- [✅] `JayXpose - Document Fondateur.md` (v2.0)
- [✅] Architecture modulaire validée

**Critères de passage :** ✅ L'idée est claire, bornée, et architecturalement valide.

---

### 1.2 Documentation Fondatrice

**Date de début :** 2026-01-26  
**Date de fin :** 2026-02-06

#### Checklist
- [✅] Document Fondateur rédigé (v2.0)
  - [✅] Section **Contexte** : Identité professionnelle exposant
  - [✅] Section **Portée / Scope** : Profil, catalogue, vitrine, documents, annuaire
  - [✅] Section **Vision & Objectifs** : WordPress+Elementor+WooCommerce de Miyukini
  - [✅] Section **Non-objectifs** : Marketplace, blog, paiement direct
  - [✅] Section **Dépendances**
- [✅] Bornage fonctionnel documenté (IN / OUT)
- [✅] Analyse des dépendances complète
  - [✅] Cores sollicités identifiés
    - StrongFather : Décisions, permissions, mandats
    - KindMother : Persistance (exposants, produits, pages, documents)
    - Border Guard : Sécurité niveau 2-3, confidentialité granulaire
    - Master Butler : Capacités exposées (API vitrine, catalogue)
    - Caring Nanny : Monitoring (alertes expiration documents)
    - Ever Buddy : Cycle de vie (versions documents, migrations)
    - WorrySentinel : Audit (opérations documents, partages)
    - TAMR : Validation manuelle (documents par organisateur)
    - BondingBrother : Sync JayFestival, JayKonta, MiyukiniPoS
  - [✅] Outils requis listés (Toolkits MiyuXxx)
  - [✅] Opérateurs liés documentés
- [⏳] Première réflexion sur la gamification
  - Idées : Complétion profil, premiers produits ajoutés, première vente, vitrine publiée

#### Livrables
- [✅] `JayXpose - Document Fondateur.md` (v2.0)
- [✅] `JayXpose - Analyse des besoins.md` (v2.0)
- [✅] `JayXpose - Ecrans et UI.md` (v2.0)
- [✅] `JayXpose - Site Vitrine Specification.md` (v1.0)
- [✅] `JayXpose - Catalogue Produits.md`
- [✅] `JayXpose - Documents Professionnels et Coffre-Fort.md`
- [✅] `JayXpose - Operateurs et Toolkits.md`
- [✅] `JayXpose - Synchronisation JayFestival.md`
- [✅] `JayXpose - Confidentialite et Partage Inter-Services.md`

**Critères de passage :** ✅ La portée est définie, les frontières sont claires.

---

## Phase 2 : Architecture & Contrats 🏗️

### 2.1 Architecture Technique

**Date de début :** 2026-02-07  
**Date de fin :** ___________

#### Checklist : Architecture Globale
- [⏳] Schéma d'architecture créé (Mermaid ou diagramme)
- [⏳] Flux de données documenté
- [⏳] Interactions entre modules définies
- [⏳] Décisions techniques prises
  - [⏳] Structure du crate définie (`crates/jayxpose`)
  - [⏳] Modules Rust identifiés
  - [✅] Patterns Rust sélectionnés (admin_cell, context, errors)

#### Checklist : Module M1 — Profil Entreprise
- [⏳] Schéma de données `exposants` défini
- [⏳] Contrats d'interface rédigés (JXP-01 à JXP-18)
- [⏳] Migrations KindMother planifiées
- [⏳] Interfaces avec les Cores définies

#### Checklist : Module M2 — Catalogue Produits
- [⏳] Schéma de données `produits_catalogue`, `categories_produits`, `produits_visuels` défini
- [⏳] Contrats d'interface rédigés (JXP-20 à JXP-29)
- [⏳] Logique de sync stocks MiyukiniPoS spécifiée
- [⏳] Interface avec M7 définie

#### Checklist : Module M3 — Page Builder ⚠️ CRITIQUE
- [ ] Architecture du Page Builder définie
- [ ] Types de blocs identifiés (texte, image, vidéo, produit, galerie, formulaire, etc.)
- [ ] Système drag & drop spécifié
- [ ] Format de stockage JSON des pages défini
- [ ] Moteur de rendu spécifié
- [ ] Preview temps réel spécifiée
- [ ] Bibliothèque de templates définie

#### Checklist : Module M4 — Vitrine & Sites
- [⏳] Architecture multi-pages définie
- [✅] Types de sites identifiés (mini-site, e-shop, service-shop)
- [✅] Routing et URL structure définis
- [✅] SEO et données structurées spécifiés
- [✅] Responsive design spécifié

#### Checklist : Module M5 — Coffre-Fort Documents
- [⏳] Schéma de données `documents_professionnels`, `documents_partages` défini
- [⏳] Workflow de partage gouverné spécifié (Mandats de Permission)
- [⏳] Alertes expiration définies
- [⏳] Intégration WorrySentinel (audit) spécifiée

#### Checklist : Module M6 — Annuaire Exposants
- [⏳] Filtres et recherche spécifiés
- [⏳] Fiche publique et confidentialité granulaire définis
- [⏳] Intégration répertoire JayFestival spécifiée

#### Checklist : Module M7 — Sync MiyukiniPoS
- [ ] Protocole de synchronisation défini
- [ ] Mapping données catalogue ↔ inventaire MiyukiniPoS
- [ ] Gestion des conflits de stock spécifiée
- [ ] Modes de sync (temps réel, batch, manuel)

#### Checklist : Module M8 — CMS Articles
- [ ] Types de contenu définis (article, actualité, événement)
- [ ] Workflow publication spécifié
- [ ] Intégration avec Page Builder (M3) définie

#### Checklist : Plan de Persistance KindMother
- [⏳] Tables définies
  - [✅] `exposants` (profil enrichi)
  - [✅] `produits_catalogue`
  - [✅] `categories_produits`
  - [✅] `produits_visuels`
  - [✅] `documents_professionnels`
  - [✅] `documents_partages`
  - [✅] `vitrine_pages`
  - [ ] `vitrine_blocs` (Page Builder)
  - [ ] `vitrine_templates` (bibliothèque)
  - [ ] `cms_articles` (CMS)
  - [ ] `sync_logs` (MiyukiniPoS)
- [⏳] Migrations planifiées
- [ ] Stratégie de backup/restore

#### Livrables
- [ ] `JayXpose - Architecture Technique.md` (À CRÉER)
- [ ] `JayXpose - Architecture Page Builder.md` (À CRÉER — CRITIQUE)
- [ ] `JayXpose - Schema Persistance KindMother.md` (À CRÉER)
- [ ] `JayXpose - Sync MiyukiniPoS Specification.md` (À CRÉER)
- [ ] Diagrammes d'architecture (Mermaid)

**Critères de passage :** ⏳ L'architecture est validée, stable et documentée. **EN COURS**

---

### 2.2 Conception UX/UI ⚠️ CRITIQUE — Page Builder

**Date de début :** ___________  
**Date de fin :** ___________

Ce module est le **cœur différenciant** de JayXpose. Il doit offrir une expérience proche d'Elementor/Figma.

#### Checklist : Page Builder UX
- [ ] Workflow utilisateur documenté
  - [ ] Création d'une page vierge
  - [ ] Ajout de blocs (sidebar ou palette)
  - [ ] Configuration des blocs (panneau propriétés)
  - [ ] Réorganisation (drag & drop)
  - [ ] Preview et publication
- [ ] Catalogue des blocs défini
  - [ ] **Blocs de contenu** : Texte, Titre, Image, Vidéo, Galerie, Icône
  - [ ] **Blocs de mise en page** : Section, Colonnes, Espacement, Séparateur
  - [ ] **Blocs interactifs** : Bouton, Lien, Formulaire, Accordéon, Onglets
  - [ ] **Blocs catalogue** : Produit, Grille produits, Carrousel produits
  - [ ] **Blocs contact** : Coordonnées, Formulaire contact, Carte
  - [ ] **Blocs sociaux** : Réseaux sociaux, Partage
- [ ] Panneau de configuration par bloc
- [ ] Système de styles (couleurs, typographie, espacement)
- [ ] Responsive preview (mobile, tablette, desktop)
- [ ] Historique undo/redo

#### Checklist : Wireframes Page Builder
- [ ] Vue d'ensemble éditeur (canvas + sidebar + panneau propriétés)
- [ ] Sidebar blocs (catégories, recherche, preview miniature)
- [ ] Panneau propriétés (formulaire dynamique par type de bloc)
- [ ] Toolbar (undo, redo, preview, publish, settings)
- [ ] Mode preview (simulation visiteur)
- [ ] Mode responsive (switcher device)

#### Checklist : Design System JayXpose
- [ ] Palette de couleurs exposant (configurable)
- [ ] Typographie système
- [ ] Composants UI réutilisables
  - [ ] Carte exposant (annuaire)
  - [ ] Carte produit
  - [ ] Bloc fiche entreprise
  - [ ] Formulaire fiche entreprise
  - [ ] Formulaire produit
  - [ ] Ligne document
  - [ ] Bandeau alerte
  - [ ] Sélecteur confidentialité
- [ ] Grid system (responsive)
- [ ] Icônes et illustrations

#### Checklist : Wireframes Espace Exposant
- [✅] Dashboard exposant (XP-E01)
- [✅] Fiche entreprise (XP-E02)
- [✅] Liste catalogue (XP-E03)
- [✅] Fiche produit (XP-E04)
- [✅] Gestion catégories (XP-E05)
- [✅] Vitrine paramètres (XP-E06)
- [✅] Vitrine page présentation (XP-E07)
- [✅] Vitrine prévisualisation (XP-E08)
- [✅] Coffre-fort documents (XP-E09)
- [✅] Upload document (XP-E10)
- [✅] Demande de partage (XP-E11)
- [✅] Fiche publique (XP-E12)

#### Checklist : Wireframes Publics
- [ ] Annuaire exposants (PUB-E01)
- [ ] Fiche exposant détail (PUB-E02)
- [ ] Vitrine accueil (PUB-E03)
- [ ] Vitrine catalogue (PUB-E04)
- [ ] Vitrine présentation (PUB-E05)
- [ ] Vitrine contact (PUB-E06)
- [ ] Fiche produit détail (PUB-E07)
- [ ] E-shop panier (PUB-E08) — si applicable

#### Checklist : Maquettes Interactives
- [ ] Prototype Page Builder (cliquable)
- [ ] Prototype Espace Exposant
- [ ] Prototype Vitrine Publique
- [ ] User testing préliminaire

#### Checklist : Gamification
- [ ] Système de progression défini
  - [ ] **Niveau 1** : Profil créé
  - [ ] **Niveau 2** : Premier produit ajouté
  - [ ] **Niveau 3** : 10 produits au catalogue
  - [ ] **Niveau 4** : Vitrine publiée
  - [ ] **Niveau 5** : Première visite externe
  - [ ] **Niveau 6** : Premier contact reçu
  - [ ] **Niveau 7** : Première vente (si e-shop)
  - [ ] **Expert** : 100 produits, profil complet, documents validés
- [ ] Badges et achievements
  - [ ] "Premier pas" : Profil complété
  - [ ] "Vitrine star" : 100 visites
  - [ ] "Catalogue complet" : 50 produits
  - [ ] "Documenté" : Tous documents validés
  - [ ] "Festival ready" : Première participation JayFestival
- [ ] Récompenses visuelles et feedback

#### Livrables
- [ ] `JayXpose - Wireframes Page Builder.md` (À CRÉER — CRITIQUE)
- [ ] `JayXpose - Design System.md` (À CRÉER)
- [ ] `JayXpose - Parcours Utilisateur.md` (À CRÉER)
- [ ] `JayXpose - Gamification Design.md` (À CRÉER)
- [ ] Maquettes interactives (Figma/Penpot ou MiyuLayoutBuilder)

**⚠️ BLOCAGE POTENTIEL :** Le Page Builder est complexe. Envisager un **prototype minimal** d'abord.

**Critères de passage :** ⏳ L'UI est conçue, validée, et prête pour l'implémentation. **EN ATTENTE**

---

### 2.3 Spécifications & Contrats

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Contrats d'Interface
- [✅] API profil exposant documentée (JXP-01 à JXP-18)
- [✅] API catalogue produits documentée (JXP-20 à JXP-29)
- [✅] API vitrine documentée (JXP-30 à JXP-38)
- [✅] API coffre-fort documents documentée (JXP-40 à JXP-49)
- [✅] API annuaire documentée (JXP-50 à JXP-53)
- [✅] API sync JayFestival documentée (JXP-60 à JXP-64)
- [ ] API Page Builder (à créer)
- [ ] API Sync MiyukiniPoS (à créer)

#### Checklist : Règles de Gouvernance
- [✅] Niveaux de sécurité par catégorie de données
  - Public (0) : Vitrine publique
  - Standard (1) : Profil entreprise public
  - Sensitive (2) : Contacts, données juridiques
  - Critical (3) : Documents professionnels, RIB
- [✅] Politique de confidentialité granulaire (par champ)
- [✅] Mandats de Permission (partage documents)
- [ ] Permissions Page Builder (qui peut éditer quoi)

#### Checklist : Tests d'Acceptation
- [✅] Critères d'acceptation par besoin (CA-1 à CA-4)
- [ ] Tests d'acceptation Page Builder
- [ ] Tests d'acceptation Sync MiyukiniPoS

#### Checklist : Edge Cases
- [✅] Comportements en conditions dégradées
- [✅] Gestion des erreurs
- [✅] Fallbacks et modes offline
- [ ] Conflits de sync MiyukiniPoS

#### Livrables
- [✅] Contrats existants dans documentation
- [ ] `JayXpose - Contrats Page Builder.md` (À CRÉER)
- [ ] `JayXpose - Contrats Sync MiyukiniPoS.md` (À CRÉER)
- [ ] `JayXpose - Tests Acceptation Complets.md` (À CRÉER)

**Critères de passage :** ⏳ Les contrats sont validés, les interfaces sont stables. **EN COURS**

---

## Phase 3 : Implémentation & Validation 💻

### 3.1 Développement Itératif (TDD) — Par Module

**Approche recommandée :** Développer par **itérations modulaires** :

```
Itération 1 (P0) : M1 (Profil) + M2 (Catalogue basique)
Itération 2 (P0) : M3 (Page Builder MVP)
Itération 3 (P1) : M4 (Vitrine) + M5 (Coffre-Fort)
Itération 4 (P2) : M6 (Annuaire) + M7 (Sync MiyukiniPoS)
Itération 5 (P3) : M8 (CMS Articles) + Polish
```

---

#### Itération 1 : Profil + Catalogue (P0)

**Date de début :** ___________  
**Date de fin :** ___________

##### Checklist : Structure du Crate
- [ ] `crates/jayxpose/src/admin_cell.rs` créé
- [ ] `crates/jayxpose/src/context.rs` créé
- [ ] `crates/jayxpose/src/errors.rs` créé
- [ ] `crates/jayxpose/src/lib.rs` créé
- [ ] `crates/jayxpose/Cargo.toml` configuré
- [ ] Structure de modules définie :
  - [ ] `crates/jayxpose/src/profil/` (M1)
  - [ ] `crates/jayxpose/src/catalogue/` (M2)
  - [ ] `crates/jayxpose/src/builder/` (M3)
  - [ ] `crates/jayxpose/src/vitrine/` (M4)
  - [ ] `crates/jayxpose/src/documents/` (M5)
  - [ ] `crates/jayxpose/src/annuaire/` (M6)
  - [ ] `crates/jayxpose/src/sync_pos/` (M7)
  - [ ] `crates/jayxpose/src/cms/` (M8)

##### Checklist : M1 — Profil Entreprise
- [ ] Tests unitaires écrits
- [ ] Schéma KindMother `exposants` créé
- [ ] CRUD profil exposant implémenté
- [ ] Contacts multiples implémentés
- [ ] Visuels (logo, bannière) implémentés
- [ ] Réseaux sociaux implémentés
- [ ] Informations juridiques implémentées
- [ ] Validation format SIRET/SIREN
- [ ] UI Dashboard exposant (XP-E01)
- [ ] UI Fiche entreprise (XP-E02)

##### Checklist : M2 — Catalogue Produits (basique)
- [ ] Tests unitaires écrits
- [ ] Schéma KindMother `produits_catalogue`, `categories_produits`, `produits_visuels`
- [ ] CRUD produits implémenté
- [ ] CRUD catégories implémenté
- [ ] Upload visuels multiples
- [ ] Produits vedettes implémentés
- [ ] Recherche et filtres
- [ ] UI Liste catalogue (XP-E03)
- [ ] UI Fiche produit (XP-E04)
- [ ] UI Gestion catégories (XP-E05)

##### Livrables Itération 1
- [ ] Code M1 + M2 fonctionnel avec tests passants
- [ ] Documentation inline (MSCM tags)
- [ ] UI opérationnel pour profil et catalogue

---

#### Itération 2 : Page Builder MVP (P0) ⚠️ CRITIQUE

**Date de début :** ___________  
**Date de fin :** ___________

##### Checklist : M3 — Page Builder MVP
- [ ] Tests unitaires écrits
- [ ] Architecture technique validée
- [ ] Schéma KindMother `vitrine_blocs`, `vitrine_templates`
- [ ] Système de blocs implémenté
  - [ ] Bloc Texte
  - [ ] Bloc Titre
  - [ ] Bloc Image
  - [ ] Bloc Section
  - [ ] Bloc Colonnes
  - [ ] Bloc Bouton
  - [ ] Bloc Produit
  - [ ] Bloc Grille Produits
- [ ] Drag & drop implémenté
- [ ] Panneau propriétés dynamique
- [ ] Sauvegarde JSON
- [ ] Moteur de rendu
- [ ] Preview temps réel
- [ ] Undo/Redo
- [ ] UI Éditeur Page Builder

##### Checklist : Templates de Base
- [ ] Template "Mini-site vitrine"
- [ ] Template "E-shop"
- [ ] Template "Service-shop"
- [ ] Template "Landing page"

##### Livrables Itération 2
- [ ] Code M3 fonctionnel avec tests passants
- [ ] Page Builder MVP opérationnel
- [ ] 4 templates de base disponibles

---

#### Itération 3 : Vitrine + Coffre-Fort (P1)

**Date de début :** ___________  
**Date de fin :** ___________

##### Checklist : M4 — Vitrine & Sites
- [ ] Tests unitaires écrits
- [ ] Schéma KindMother `vitrine_pages`
- [ ] Pages vitrine implémentées (Accueil, Catalogue, Présentation, Contact)
- [ ] URL slug personnalisable
- [ ] Personnalisation couleurs
- [ ] SEO (title, description, keywords, schema.org)
- [ ] Responsive design
- [ ] Statuts publication (brouillon, publiée, suspendue)
- [ ] Formulaire contact
- [ ] UI Paramètres vitrine (XP-E06)
- [ ] UI Page présentation (XP-E07)
- [ ] UI Prévisualisation (XP-E08)
- [ ] Pages publiques (PUB-E03 à PUB-E06)

##### Checklist : M5 — Coffre-Fort Documents
- [ ] Tests unitaires écrits
- [ ] Schéma KindMother `documents_professionnels`, `documents_partages`
- [ ] Upload document (types, validation)
- [ ] Versioning documents
- [ ] Statuts (en_attente, validé, expiré, rejeté)
- [ ] Alertes expiration (30j, 15j, 7j)
- [ ] Partage gouverné (Mandats de Permission)
- [ ] Intégration WorrySentinel (audit)
- [ ] UI Coffre-fort (XP-E09)
- [ ] UI Upload document (XP-E10)
- [ ] UI Demande de partage (XP-E11)

##### Livrables Itération 3
- [ ] Code M4 + M5 fonctionnel avec tests passants
- [ ] Vitrines publiques opérationnelles
- [ ] Coffre-fort sécurisé opérationnel

---

#### Itération 4 : Annuaire + Sync MiyukiniPoS (P2)

**Date de début :** ___________  
**Date de fin :** ___________

##### Checklist : M6 — Annuaire Exposants
- [ ] Tests unitaires écrits
- [ ] Filtres (secteur, localisation, mots-clés, événement)
- [ ] Recherche full-text
- [ ] Fiche publique et confidentialité granulaire
- [ ] Intégration répertoire JayFestival
- [ ] Multi-événements
- [ ] UI Annuaire (PUB-E01)
- [ ] UI Fiche exposant détail (PUB-E02)
- [ ] UI Fiche publique config (XP-E12)

##### Checklist : M7 — Sync MiyukiniPoS ⚠️ NOUVEAU
- [ ] Tests unitaires écrits
- [ ] Protocole de synchronisation implémenté
- [ ] Mapping catalogue ↔ inventaire
- [ ] Gestion des conflits de stock
- [ ] Modes de sync (temps réel, batch, manuel)
- [ ] Logs de synchronisation
- [ ] UI Configuration sync

##### Livrables Itération 4
- [ ] Code M6 + M7 fonctionnel avec tests passants
- [ ] Annuaire opérationnel
- [ ] Sync MiyukiniPoS opérationnelle

---

#### Itération 5 : CMS Articles + Polish (P3)

**Date de début :** ___________  
**Date de fin :** ___________

##### Checklist : M8 — CMS Articles
- [ ] Tests unitaires écrits
- [ ] Types de contenu (article, actualité, événement)
- [ ] Workflow publication (brouillon, relecture, publié)
- [ ] Intégration Page Builder (M3)
- [ ] Catégories et tags articles
- [ ] Pagination et archives
- [ ] UI Liste articles
- [ ] UI Éditeur article

##### Checklist : Polish Global
- [ ] Optimisation performance
- [ ] Tests de charge
- [ ] Accessibilité WCAG 2.1 AA
- [ ] Internationalisation (préparation i18n)

##### Livrables Itération 5
- [ ] Code M8 fonctionnel avec tests passants
- [ ] CMS opérationnel
- [ ] Performance et accessibilité validées

---

### 3.2 Tests & Qualité

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Types de Tests
- [ ] **Tests unitaires** : Chaque fonction critique
- [ ] **Tests d'intégration** : Interactions entre modules
- [ ] **Tests de gouvernance** : Validation décisions StrongFather
- [ ] **Tests de persistance** : KindMother, migrations, rollback
- [ ] **Tests UX** : Parcours utilisateur, Page Builder
- [ ] **Tests de performance** :
  - [ ] Chargement fiche exposant < 1s (NFR-JXP-01)
  - [ ] Liste annuaire < 2s (NFR-JXP-01)
  - [ ] Vitrine < 3s premier chargement (NFR-JXP-01)
  - [ ] Page Builder réactif (< 100ms feedback)
- [ ] **Tests de sécurité** : Niveaux 0-3 validés
- [ ] **Tests Sync MiyukiniPoS** : Conflits, modes, logs

#### Checklist : Validation Qualité
- [ ] Aucun linter error
- [ ] `cargo clippy` passe sans warnings critiques
- [ ] `cargo fmt` appliqué
- [ ] Code review effectuée
- [ ] Documentation complète

#### Livrables
- [ ] Rapport de tests (coverage, résultats)
- [ ] `JayXpose - Rapport Qualite.md`

**Critères de passage :** ⏳ Tous les tests passent, qualité validée. **EN ATTENTE**

---

### 3.3 Intégration dans le Central

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Intégration Interne
- [ ] Enregistrement de l'Opérateur JayXpose
- [ ] Configuration des permissions
- [ ] Niveaux de sécurité configurés (0-3)
- [ ] Vérification conformité LOI-1

#### Checklist : Intégration JayFestival
- [ ] Profil unique (pas de duplication)
- [ ] Pré-remplissage candidatures
- [ ] Partage documents gouverné
- [ ] Catalogue visible dans répertoire
- [ ] Notifications croisées
- [ ] Tests d'intégration JayFestival

#### Checklist : Intégration JayKonta
- [ ] RIB partagé depuis coffre-fort (avec Mandat)
- [ ] Facturation exposant
- [ ] Tests d'intégration JayKonta

#### Checklist : Intégration MiyukiniPoS
- [ ] Sync catalogue ↔ inventaire
- [ ] Gestion des stocks temps réel
- [ ] Tests d'intégration MiyukiniPoS

#### Livrables
- [ ] `JayXpose - Integration Central.md`
- [ ] `JayXpose - Integration JayFestival.md`
- [ ] `JayXpose - Integration MiyukiniPoS.md`
- [ ] Configuration de déploiement

**Critères de passage :** ⏳ L'Opérateur est intégré et fonctionnel dans le COG. **EN ATTENTE**

---

## Phase 4 : Raffinement & Gamification 🎮

### 4.1 Gamification

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Système de Progression
- [ ] **Niveaux implémentés**
  - [ ] Niveau 1 "Débutant" : Compte créé
  - [ ] Niveau 2 "Artisan" : Profil complété (nom, description, logo)
  - [ ] Niveau 3 "Producteur" : 5 produits au catalogue
  - [ ] Niveau 4 "Exposant" : Vitrine publiée
  - [ ] Niveau 5 "Visible" : 50 visites sur la vitrine
  - [ ] Niveau 6 "Communicant" : 10 messages reçus
  - [ ] Niveau 7 "Vendeur" : Première vente (si e-shop)
  - [ ] Niveau 8 "Expert" : 100 produits + profil complet + documents validés

#### Checklist : Badges et Achievements
- [ ] **"Premier pas"** : Profil créé
- [ ] **"Vitrine publiée"** : Première publication
- [ ] **"Catalogue complet"** : 50 produits
- [ ] **"Documenté"** : Tous documents professionnels validés
- [ ] **"Festival ready"** : Première candidature JayFestival acceptée
- [ ] **"Vitrine star"** : 500 visites
- [ ] **"Super vendeur"** : 10 ventes
- [ ] **"Designer"** : 10 pages créées avec le Page Builder
- [ ] **"Influenceur"** : 100 partages réseaux sociaux

#### Checklist : Feedback et Récompenses
- [ ] Barre de progression visible
- [ ] Notifications de déblocage
- [ ] Animations de célébration
- [ ] Messages d'encouragement contextuels
- [ ] Dashboard de progression

#### Livrables
- [ ] `JayXpose - Systeme Gamification.md`
- [ ] Code de gamification testé

**Critères de passage :** ⏳ La gamification enrichit l'expérience sans nuire à l'utilisabilité. **EN ATTENTE**

---

### 4.2 Polish UI/UX

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist : Animations
- [ ] Transitions fluides entre écrans
- [ ] Feedback visuel (boutons, validations)
- [ ] Loading states (spinner, skeleton screens)
- [ ] Animations Page Builder (drag & drop, preview)
- [ ] Animations vitrine (scroll, hover)

#### Checklist : Responsive Design
- [ ] Mobile (< 768px)
- [ ] Tablette (768px - 1023px)
- [ ] Desktop (>= 1024px)
- [ ] Page Builder mobile-friendly

#### Checklist : Thème
- [ ] Cohérence visuelle Miyukini
- [ ] Mode clair / sombre (exposant)
- [ ] Thèmes vitrine (palette configurable)

#### Checklist : Accessibilité
- [ ] WCAG 2.1 niveau AA minimum
- [ ] Navigation au clavier (Page Builder inclus)
- [ ] Screen readers compatibles
- [ ] Contrastes suffisants
- [ ] Focus visible

#### Livrables
- [ ] UI polie et testée
- [ ] `JayXpose - Rapport Accessibilite.md`

**Critères de passage :** ⏳ L'UI est polie, fluide, et accessible. **EN ATTENTE**

---

## Phase 5 : Livraison & Documentation 📦

### 5.1 Documentation Utilisateur

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] **Guide de démarrage**
  - [ ] Création de compte exposant
  - [ ] Compléter son profil
  - [ ] Ajouter ses premiers produits
  - [ ] Publier sa vitrine
- [ ] **Guide Page Builder**
  - [ ] Introduction aux blocs
  - [ ] Créer une page de A à Z
  - [ ] Personnaliser son design
  - [ ] Utiliser les templates
- [ ] **Guide Catalogue**
  - [ ] Gérer ses produits
  - [ ] Organiser par catégories
  - [ ] Sync avec MiyukiniPoS
- [ ] **Guide Coffre-Fort**
  - [ ] Gérer ses documents
  - [ ] Partager avec un organisateur
- [ ] **FAQ**
  - [ ] "Comment créer une vitrine ?"
  - [ ] "Comment utiliser le Page Builder ?"
  - [ ] "Comment synchroniser mon catalogue avec MiyukiniPoS ?"
  - [ ] "Comment partager un document ?"
- [ ] **Tutoriels vidéo** (optionnel)

#### Livrables
- [ ] `JayXpose - Guide Demarrage.md`
- [ ] `JayXpose - Guide Page Builder.md`
- [ ] `JayXpose - Guide Catalogue.md`
- [ ] `JayXpose - Guide Coffre-Fort.md`
- [ ] `JayXpose - FAQ.md`
- [ ] `JayXpose - Tutoriels.md`

---

### 5.2 Documentation Technique

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist
- [ ] Architecture finale documentée
- [ ] API Reference complète (tous modules)
- [ ] Guide de maintenance
- [ ] Index MIP à jour

#### Livrables
- [ ] `JayXpose - Architecture Finale.md`
- [ ] `JayXpose - API Reference.md`
- [ ] `JayXpose - Guide Maintenance.md`
- [ ] Index MIP mis à jour

---

### 5.3 Release

**Date de début :** ___________  
**Date de fin :** ___________

#### Checklist Finale
- [ ] ✅ Tous les tests passent
- [ ] ✅ Documentation complète
- [ ] ✅ Conformité architecturale validée
- [ ] ✅ Aucun linter error
- [ ] ✅ Performance acceptable (NFR-JXP-01)
- [ ] ✅ Sécurité auditée (niveaux 0-3)
- [ ] ✅ Code review finale
- [ ] ✅ Validation par les pairs
- [ ] ✅ Approbation StrongFather
- [ ] ✅ Intégrations validées (JayFestival, JayKonta, MiyukiniPoS)

#### Checklist Release
- [ ] Version taggée (v1.0.0)
- [ ] Notes de release rédigées
- [ ] Migration guide créé
- [ ] Changelog mis à jour
- [ ] Commit de release créé
- [ ] Push vers le repository

#### Livrables
- [ ] **Version 1.0.0** déployée
- [ ] `JayXpose - Notes Release v1.0.0.md`
- [ ] `JayXpose - Migration Guide.md`
- [ ] `CHANGELOG.md` mis à jour

**Critères de passage :** ⏳ Le projet est livré, documenté, et prêt pour la production. **EN ATTENTE**

---

## 📊 Métriques de Suivi

### Avancement Global
- **Phase 1 :** [X] 100% ✅
- **Phase 2 :** [X] 55% ⏳ (Architecture + Contrats en cours, UI en attente)
- **Phase 3 :** [X] 70% ⏳ (Itération 1-5 en cours, M8 CMS + Auth multi-exposants implémentés)
- **Phase 4 :** [ ] 0%
- **Phase 5 :** [ ] 0%

### Avancement par Module
| Module | Phase 2 | Phase 3 | Phase 4 |
|--------|---------|---------|---------|
| M1 Profil | 60% | 95% | 0% |
| M2 Catalogue | 60% | 90% | 0% |
| M3 Page Builder | 60% | 70% | 0% |
| M4 Vitrine | 50% | 60% | 0% |
| M5 Coffre-Fort | 50% | 85% | 0% |
| M6 Annuaire | 30% | 50% | 0% |
| M7 Sync PoS | 40% | 40% | 0% |
| M8 CMS | 100% | 80% | 0% |

### Temps Estimé vs Réel

| Phase | Estimé | Réel | Delta |
|-------|--------|------|-------|
| Phase 1 | 32 h | 28 h | -4 h |
| Phase 2 | 80 h | ___ h (en cours) | ___ h |
| Phase 3 | 400 h | ___ h | ___ h |
| Phase 4 | 80 h | ___ h | ___ h |
| Phase 5 | 40 h | ___ h | ___ h |
| **TOTAL** | **632 h** | **___ h** | **___ h** |

### Qualité
- **Couverture de tests :** N/A (pas encore implémenté)
- **Linter errors :** 0
- **Performance :** N/A
- **Score accessibilité :** N/A

---

## 🚨 Blocages & Risques

### Blocages Actuels
| Date | Blocage | Impact | Solution proposée | Statut |
|------|---------|--------|-------------------|--------|
| 2026-02-07 | Page Builder UI complexe | 🔴 Haut | Prototype MVP d'abord, puis itérer | ⏳ Planifié |
| 2026-02-07 | Sync MiyukiniPoS non spécifiée | 🟡 Moyen | Documenter protocole de sync | ⏳ À faire |
| 2026-02-07 | Manque d'outils pour builder des layouts | 🔴 Haut | Créer MiyuLayoutBuilder ou utiliser Figma | ⏳ Identifié |

### Risques Identifiés
| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Page Builder trop complexe | 🔴 Haute | 🔴 Haut | MVP minimal d'abord, itérations progressives |
| Performance Page Builder | 🟡 Moyenne | 🔴 Haut | Optimisation dès le début, tests de charge |
| Conflits sync MiyukiniPoS | 🟡 Moyenne | 🟡 Moyen | Stratégie de résolution automatique + manuelle |
| Complexité UI | 🔴 Haute | 🔴 Haut | Design System strict, composants réutilisables |
| Intégration JayFestival | 🟢 Basse | 🟡 Moyen | Documentation existante, tests précoces |
| Scalabilité catalogue (500 produits) | 🟡 Moyenne | 🟡 Moyen | Pagination, lazy loading, indexation |
| Stockage documents (50 Mo/exposant) | 🟢 Basse | 🟢 Bas | Quotas, compression, alertes |

---

## 📝 Notes & Apprentissages

### Ce qui a bien fonctionné
- Documentation fondatrice très complète
- Définition claire des besoins (65+ JXP-xx)
- Spécification UI/écrans détaillée
- Niveaux de sécurité bien définis

### Ce qui peut être amélioré
- **Page Builder non encore conçu** : priorité absolue pour Phase 2.2
- **Sync MiyukiniPoS non spécifiée** : documenter le protocole
- **Gamification à détailler** : intégrer dans Phase 2

### Leçons apprises
- ✅ Pour un projet de cette complexité, **l'approche modulaire est essentielle**
- ✅ Le **Page Builder est le cœur différenciant** — investir temps et design
- ⚠️ **Ne pas sous-estimer la complexité UI** du Page Builder

---

## 🔗 Références

### Documents JayXpose
- [Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [Écrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [Opérateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md)
- [Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md)
- [Confidentialité et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [Niveaux Sécurité](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md)

### Dépendances
- **Cores :** StrongFather, KindMother, Border Guard, Master Butler, Caring Nanny, Ever Buddy, WorrySentinel, TAMR, BondingBrother
- **Services :** JayFestival, JayKonta, JayKoa, JayRDV
- **Outils :** MiyukiniPoS (sync stocks)
- **Toolkits :** Miyauth, Miyuprofile, Miyucms, Miyumedia, Miyucontacts

### Contacts
- **Responsable technique :** Miyukini Team
- **Responsable UX/UI :** À définir
- **Validation architecture :** Miyukini Team

---

## 🎯 Prochaines Actions

### Priorité Immédiate (Cette Semaine)
1. **[CRITIQUE]** Concevoir l'architecture du **Page Builder** (M3)
   - Définir les types de blocs
   - Spécifier le format JSON
   - Créer les wireframes de l'éditeur
   
2. **[HAUTE]** Documenter le protocole **Sync MiyukiniPoS** (M7)
   - Mapping données catalogue ↔ inventaire
   - Modes de synchronisation
   - Gestion des conflits

3. **[HAUTE]** Créer `JayXpose - Architecture Technique.md`
   - Schéma d'architecture globale
   - Interactions entre modules
   - Plan de persistance complet

### Priorité Haute (Prochaines 2 Semaines)
4. Finaliser la spécification des contrats Page Builder
5. Créer les wireframes Page Builder (éditeur, sidebar, propriétés)
6. Définir le système de gamification complet
7. Valider l'architecture avec les Cores (StrongFather, KindMother)

### Priorité Moyenne (Mois Prochain)
8. Démarrer l'implémentation Itération 1 (M1 + M2)
9. Prototyper le Page Builder MVP
10. Tests d'intégration JayFestival

---

## 📈 Comparaison avec Équivalents Externes

### JayXpose vs WordPress + Elementor + WooCommerce

| Fonctionnalité | WordPress/Elementor/WC | JayXpose (Cible) |
|----------------|------------------------|------------------|
| Page Builder | ✅ Elementor (puissant) | ⚠️ MVP puis itérer |
| CMS | ✅ WordPress (mature) | ⏳ M8 (P3) |
| E-commerce | ✅ WooCommerce (complet) | ⚠️ Catalogue + sync PoS |
| Thèmes | ✅ Milliers de thèmes | ⚠️ Templates de base |
| Plugins | ✅ Écosystème riche | ❌ Pas de plugins |
| Hébergement | 🔄 À gérer soi-même | ✅ Intégré Miyukini |
| Sécurité | 🔄 Variable | ✅ Gouvernance COG |
| Autonomie | ❌ Dépend de services | ✅ LOI-1 respectée |
| Intégration | 🔄 Via plugins | ✅ Native (JayFestival, PoS) |

### Stratégie de Différenciation

1. **Simplicité** : JayXpose est plus simple que WordPress (pas de courbe d'apprentissage)
2. **Intégration** : Sync native avec MiyukiniPoS, JayFestival, JayKonta
3. **Gouvernance** : Sécurité et confidentialité par design (COG)
4. **Autonomie** : Fonctionne hors-ligne (LOI-2)
5. **Cible** : Exposants, artisans, petites structures — pas les entreprises avec besoins complexes

---

**✨ Projet créé avec le Template Parcours Développement Miyukini COG v1.0**

---

## Annexe A : Catalogue des Blocs Page Builder (Proposition)

### Blocs de Contenu
| Bloc | Description | Propriétés |
|------|-------------|------------|
| `text` | Paragraphe de texte formaté | content, alignment, color, font-size |
| `heading` | Titre (H1-H6) | level, text, alignment, color |
| `image` | Image avec alt text | src, alt, width, alignment, link |
| `video` | Vidéo intégrée (YouTube/Vimeo) | provider, embed_id, autoplay |
| `gallery` | Galerie d'images | images[], columns, gap |
| `icon` | Icône avec texte optionnel | icon, size, color, label |

### Blocs de Mise en Page
| Bloc | Description | Propriétés |
|------|-------------|------------|
| `section` | Conteneur avec fond | background, padding, margin |
| `columns` | Colonnes responsives | layout (1/2, 1/3, 2/3), gap |
| `spacer` | Espacement vertical | height |
| `divider` | Ligne de séparation | style, color, width |

### Blocs Interactifs
| Bloc | Description | Propriétés |
|------|-------------|------------|
| `button` | Bouton CTA | text, link, style, color, size |
| `form` | Formulaire de contact | fields[], submit_text, redirect |
| `accordion` | Accordéon (FAQ) | items[{title, content}] |
| `tabs` | Onglets | tabs[{title, content}] |

### Blocs Catalogue
| Bloc | Description | Propriétés |
|------|-------------|------------|
| `product` | Fiche produit unique | product_id, show_price, show_button |
| `product_grid` | Grille de produits | category_id, limit, columns |
| `product_carousel` | Carrousel produits | product_ids[], autoplay, arrows |
| `featured_products` | Produits vedettes | limit, style |

### Blocs Contact
| Bloc | Description | Propriétés |
|------|-------------|------------|
| `contact_info` | Coordonnées | show_email, show_phone, show_address |
| `social_links` | Liens réseaux sociaux | networks[], style, size |
| `map` | Carte (si adresse publique) | zoom, style |

---

## Annexe B : Format JSON Page Builder (Proposition)

```json
{
  "version": "1.0",
  "page_id": "uuid",
  "title": "Accueil",
  "slug": "accueil",
  "settings": {
    "background_color": "#ffffff",
    "max_width": "1200px",
    "padding": "20px"
  },
  "blocks": [
    {
      "id": "block-1",
      "type": "section",
      "props": {
        "background": "#f5f5f5",
        "padding": "60px 20px"
      },
      "children": [
        {
          "id": "block-2",
          "type": "heading",
          "props": {
            "level": 1,
            "text": "Bienvenue chez Mon Atelier",
            "alignment": "center",
            "color": "#333333"
          }
        },
        {
          "id": "block-3",
          "type": "text",
          "props": {
            "content": "Découvrez nos créations artisanales uniques.",
            "alignment": "center"
          }
        }
      ]
    },
    {
      "id": "block-4",
      "type": "product_grid",
      "props": {
        "category_id": null,
        "limit": 6,
        "columns": 3,
        "show_price": true,
        "show_button": true
      }
    },
    {
      "id": "block-5",
      "type": "button",
      "props": {
        "text": "Voir tout le catalogue",
        "link": "/catalogue",
        "style": "primary",
        "alignment": "center"
      }
    }
  ]
}
```

---

## Annexe C : Templates de Base (Proposition)

### Template 1 : Mini-Site Vitrine
```
[Section Hero]
  - Heading: Nom entreprise
  - Text: Slogan
  - Image: Bannière

[Section Présentation]
  - Heading: À propos
  - Text: Description
  - Image: Photo atelier

[Section Produits Vedettes]
  - Heading: Nos créations
  - Product Grid: 6 vedettes
  - Button: Voir le catalogue

[Section Contact]
  - Contact Info
  - Form: Contact simple
```

### Template 2 : E-Shop
```
[Section Hero]
  - Image: Bannière
  - Heading: Nouveautés
  - Product Carousel: Nouveaux produits

[Section Catégories]
  - Columns (3)
    - Category Card x3

[Section Tous Produits]
  - Product Grid: Tous produits, paginé
  - Filters: Catégorie, prix

[Section Promo]
  - Section avec fond
  - Heading: Offre spéciale
  - Product: Produit promo
  - Button: Commander
```

### Template 3 : Service-Shop
```
[Section Hero]
  - Heading: Nos Services
  - Text: Présentation
  - Button: Prendre RDV (lien JayRDV)

[Section Services]
  - Columns (2)
    - Service Card x4 (image + titre + description + prix)

[Section Témoignages] (phase 2)
  - Carousel témoignages

[Section Contact]
  - Contact Info
  - Form: Demande de devis
```

### Template 4 : Landing Page
```
[Section Hero Full]
  - Image plein écran
  - Heading overlay
  - Button CTA

[Section Avantages]
  - Columns (3)
    - Icon + Heading + Text x3

[Section Produit Phare]
  - Product détaillé
  - Gallery
  - Button: Commander

[Section CTA Final]
  - Heading
  - Form: Newsletter ou contact
```

---

## Mise a jour implementation - 2026-02-07

### Travaux realises (MVP execute)

- [x] JayXpose accessible depuis Central via `JayXposeService` embarque.
- [x] Persistance locale SQLite/KindMother active.
- [x] Module M3 page builder structure:
- [x] `PageBuilderDocument` et `PageBuilderBlock` types
- [x] Persistance `vitrine_pages` + `vitrine_blocs`
- [x] Templates `vitrine_templates` seeds
- [x] Preview bloc par bloc dans XP-E08
- [x] Module M7 sync PoS MVP:
- [x] Mapping `pos_stock_links`
- [x] Action `Sync stock PoS` dans XP-E03
- [x] Journal d'audit `sync_logs`
- [x] Pulse inter-services (JayFestival/JayKonta/JayRDV) journalise dans XP-E01

### Livrables documentaires crees

- [x] `JayXpose - Architecture Technique.md`
- [x] `JayXpose - Architecture Page Builder.md`
- [x] `JayXpose - Schema Persistance KindMother.md`
- [x] `JayXpose - Sync MiyukiniPoS Specification.md`
- [x] `JayXpose - Integration Central.md`
- [x] `JayXpose - Interfaces Inter-Services.md`

### Points restant en backlog

- [x] Auth/session multi-exposants complete — implémenté 2026-02-08
- [x] CMS Articles (M8) — implémenté 2026-02-08
- [x] Sync PoS bidirectionnelle complete (pull + conflits) — implémente 2026-02-08
- [ ] Guide utilisateur et FAQ Phase 5

---

## Mise a jour implementation - 2026-02-08

### Module M8 CMS Articles implémenté

- [x] Types de données CMS:
  - [x] `CmsArticle` — article avec titre, slug, extrait, contenu, SEO
  - [x] `CmsCategory` — catégorie d'articles
  - [x] `ArticleType` enum (article, actualite, evenement)
  - [x] `ArticleStatus` enum (brouillon, relecture, publie, archive)
- [x] Persistance KindMother:
  - [x] Table `cms_articles` avec tous les champs
  - [x] Table `cms_categories`
  - [x] Index de performance `idx_cms_articles_exposant`, `idx_cms_articles_slug`
- [x] Opérations CRUD:
  - [x] `cms_article_insert`, `cms_article_update`, `cms_article_delete`
  - [x] `cms_article_by_id`, `cms_articles_by_exposant`, `cms_articles_published`
  - [x] `cms_article_increment_views`
  - [x] `cms_category_insert`, `cms_category_update`, `cms_category_delete`
  - [x] `cms_categories_by_exposant`
- [x] Écrans UI:
  - [x] XP-E13 — Liste des articles CMS avec filtres et stats
  - [x] XP-E14 — Éditeur d'article CMS (création/modification)
- [x] Intégration:
  - [x] `ScreenId::ExpCmsArticles` et `ScreenId::ExpCmsEditArticle`
  - [x] `ScreenId::PubArticles` pour le blog public
  - [x] Bouton "CMS Articles" dans le dashboard (XP-E01)
  - [x] Chargement des données CMS dans `load_dashboard_data`
  - [x] `ExpState` enrichi avec champs CMS (articles, categories, filtres)

---

## Mise a jour implementation - 2026-02-08 (Auth Multi-Exposants)

### Intégration Profil Central → Exposant JayXpose

JayXpose utilise maintenant le profil connecté à Central au lieu d'un exposant par défaut.

#### Architecture

```
Central (CentralProfile)  →  profile_service_refs  →  JayXpose (ExposantProfile)
         ↓                     service_key="jayxpose"           ↓
      profile_id           →        ref_id         →      exposant_id
```

#### Fichiers modifiés

**`crates/jayxpose/src/app.rs`**
- [x] `set_central_profile(profile_id, email, nom, prenom, exposant_id)` — liaison profil Central
- [x] `get_current_exposant_id()` — retourne l'exposant courant pour enregistrement du lien
- [x] Création automatique de l'exposant si aucun lien existant :
  - ID : `exp-central-{profile_id}`
  - Nom entreprise : prénom + nom du profil Central
  - Email contact : email du profil Central

**`crates/miyukini-central/src/services/mod.rs`**
- [x] Trait `ServiceUi` étendu :
  - [x] `set_jayxpose_profile(profile_id, email, nom, prenom, exposant_id)`
  - [x] `get_jayxpose_exposant_id() -> Option<String>`

**`crates/miyukini-central/src/services/jayxpose_service.rs`**
- [x] Implémentation `set_jayxpose_profile` délègue à `JayXposeApp`
- [x] Implémentation `get_jayxpose_exposant_id` retourne l'ID exposant courant
- [x] Constante `SERVICE_KEY = "jayxpose"` pour `profile_service_refs`

**`crates/miyukini-central/src/app.rs`**
- [x] `ui_service_content` synchronise le profil à chaque affichage de JayXpose
- [x] Utilise `profile_service_refs` pour lier profil Central ↔ exposant
- [x] Enregistre automatiquement le lien après création d'un nouvel exposant

#### Flux de connexion

1. Utilisateur se connecte à Central → `current_profile = Some(profile)`
2. Utilisateur ouvre JayXpose → `ui_service_content(ServiceId::JayXpose)`
3. Central vérifie `profile_service_refs` pour lien existant
4. **Si lien existe** : JayXpose utilise l'exposant lié
5. **Si pas de lien** :
   - JayXpose crée un nouvel exposant depuis le profil Central
   - Central enregistre le lien dans `profile_service_refs`
6. Déconnexion : JayXpose revient en mode standalone (exposant par défaut)

#### Table `profile_service_refs`

```sql
profile_service_refs (
    profile_id TEXT NOT NULL,      -- ID profil Central
    service_key TEXT NOT NULL,     -- "jayxpose"
    ref_id TEXT NOT NULL,          -- ID exposant JayXpose
    PRIMARY KEY (profile_id, service_key)
)
```

---

## Mise a jour implementation - 2026-02-08 (Lot parcours vers 100%)

### M7 Sync PoS avance

- [x] Pull stock PoS implemente (direction `stock_pull`)
- [x] Detection de conflits de stock
- [x] Politique de conflits configurable (`manual_review`, `prefer_pos`, `prefer_local`)
- [x] UI de configuration sync dans XP-E03 (mode + politique)
- [x] Resolution de conflits depuis l'UI (`Resoudre conflits`)
- [x] Audit des actions (`sync_logs`)

### Phase 5 Docs utilisateur completes

- [x] `JayXpose - Guide Demarrage.md`
- [x] `JayXpose - Guide Page Builder.md`
- [x] `JayXpose - Guide Catalogue.md`
- [x] `JayXpose - Guide Coffre-Fort.md`
- [x] `JayXpose - FAQ.md`
- [x] `JayXpose - Tutoriels.md`

### Docs techniques complementaires

- [x] `JayXpose - API Reference.md`
- [x] `JayXpose - Guide Maintenance.md`
- [x] `JayXpose - Architecture Finale.md`
- [x] `JayXpose - Rapport Qualite.md`
- [x] `JayXpose - Rapport Accessibilite.md`
- [x] `JayXpose - Integration JayFestival.md`
- [x] `JayXpose - Integration MiyukiniPoS.md`

---

## Mise a jour implementation - 2026-02-08 (Refonte Builder orientee Elementor)

### Corrections fonctionnelles builder

- [x] Selection de bloc dans canvas (navigator)
- [x] Panneau proprietes par onglets: Content / Style / Advanced
- [x] Edition reelle des champs metier:
  - [x] Hero (title/subtitle/cta)
  - [x] Heading
  - [x] Text
  - [x] Button (texte + lien)
  - [x] Image
  - [x] Product Grid
  - [x] Features
  - [x] FAQ
- [x] Actions de composition: ajouter, dupliquer, supprimer, reordonner
- [x] Persistance fiable page + blocs
- [x] Preview utile avec rendu par type de bloc et styles de base

### Documentation de refonte

- [x] `JayXpose - Refonte Page Builder Elementor.md`

---

## Mise a jour implementation - 2026-02-08 (Page Builder complet — 18 blocs + DnD natif)

### Drag-and-Drop natif souris

- [x] Glisser-deposer natif sur chaque widget du canvas (clic + drag souris)
- [x] Cible de drop inter-colonnes et intra-colonne avec ajustement d'index
- [x] Indicateur d'insertion (ligne avant/apres l'item survole)
- [x] Surbrillance de colonne si drop "append"
- [x] Ghost (etiquette flottante) pendant le drag
- [x] Commit mutation reelle du document (parent_id + repositionnement Vec)
- [x] Snapshot undo + trace gouvernance action="move_widget"
- [x] Remplacement du "drag-assist" par DnD natif

### 9 nouveaux types de blocs (18 blocs total)

Librairie de widgets categorisee (Mise en page / Contenu / Interactif / Catalogue / Contact):

- [x] **Spacer** : espacement vertical configurable
- [x] **Divider** : separateur horizontal (style solid/dashed/dotted, couleur, epaisseur)
- [x] **Video** : video integree YouTube/Vimeo (provider + embed_id + autoplay)
- [x] **Gallery** : galerie d'images multi-colonnes (URLs, colonnes, espacement)
- [x] **Icon** : icone avec texte (star/heart/check/info/warning/mail/phone/map)
- [x] **Form** : formulaire de contact (champs configurables, *=obligatoire, bouton submit)
- [x] **Accordion** : accordeon FAQ (items JSON [{title, content}])
- [x] **Tabs** : contenu a onglets (tabs JSON [{title, content}])
- [x] **Contact Info** : coordonnees entreprise (email/phone/adresse, fallback profil exposant)
- [x] **Social Links** : reseaux sociaux (Facebook/Instagram/LinkedIn/TikTok/YouTube/X, fallback profil)

Chaque bloc dispose de:
- Edition Content complete dans le panneau proprietes
- Edition Style (couleurs, padding, font_size responsive)
- Edition Advanced (css_class, anchor, margins)
- Rendu preview complet dans XP-E08
- Etiquette descriptive dans le canvas

### Templates enrichis

- [x] **E-Shop** : Hero + Grille produits + Features + Formulaire + Reseaux sociaux (2 sections)
- [x] **Service Shop** : Hero + Accordion services + Contact Info + Formulaire devis + Social (2 sections)
- [x] Templates existants conserves (Mini Site, Landing, Storytelling, Catalogue Focus)

### Librairie widgets categorisee

- [x] Categories: Mise en page / Contenu / Interactif / Catalogue / Contact
- [x] Recherche filtrante dans la librairie
- [x] Affichage par categorie avec labels (style Elementor BASIC/PRO)

### Preview XP-E08 enrichie

- [x] Rendu Spacer (espacement vertical)
- [x] Rendu Divider (ligne separatrice coloree)
- [x] Rendu Video (placeholder avec provider + ID)
- [x] Rendu Gallery (grille multi-colonnes avec placeholders images)
- [x] Rendu Icon (caractere Unicode + label)
- [x] Rendu Form (champs du formulaire + bouton, champs obligatoires marques *)
- [x] Rendu Accordion (items depliables avec titres et contenus)
- [x] Rendu Tabs (onglets avec 1er onglet actif)
- [x] Rendu Contact Info (email/phone/adresse avec fallback profil exposant)
- [x] Rendu Social Links (boutons reseaux avec fallback profil exposant)

### Fichiers modifies

- `crates/jayxpose/src/screens/exp/e07_vitrine_presentation.rs`
- `crates/jayxpose/src/screens/exp/e08_vitrine_preview.rs`
- `crates/jayxpose/src/screens/exp/mod.rs`

### Validation build

- [x] `cargo check -p jayxpose` OK
- [x] `cargo check -p miyukini-central` OK
- [x] Aucun linter error
