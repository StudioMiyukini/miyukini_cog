# JayManga — Plan d'implementation

## Contexte

Ce document definit le **plan d'implementation** du service JayManga : decoupage en phases, ordre de developpement des modules, dependances entre modules, jalons de validation et criteres d'acceptation. Il s'appuie sur le [Guide d'Implementation](./JayManga%20-%20Guide%20Implementation.md) pour la structure technique et sur les specifications fonctionnelles pour les exigences.

## Portee / Scope

- **Perimetre** : Planification de l'implementation du crate `jaymanga` en phases incrementales, de la fondation (Phase 0) au service complet (Phase 7).
- **Hors perimetre** : Estimations temporelles en jours/heures (dependent de l'equipe), implementation code (voir Guide Implementation).
- **References** : [JayManga - Guide Implementation](./JayManga%20-%20Guide%20Implementation.md), [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Vue d'ensemble des phases

```
Phase 0 : Fondation           ← Crate, schema, types, config vendeur
Phase 1 : Catalogue            ← Import, formats, series, metadonnees
Phase 2 : Optimisation images  ← Compression, variantes, selection
Phase 3 : Liseuse et lecture   ← Liseuse web + native, navigation, demo
Phase 4 : Paiement et licences ← Panier, checkout, licences, passerelles
Phase 5 : Favoris et MWS       ← Favoris, presence, telechargement, manifestes
Phase 6 : Gamification         ← XP, niveaux, streaks, badges, Miou
Phase 7 : Portail Agrege       ← Collecteur, cache, interface unifiee
```

Chaque phase produit un **livrable autonome et testable**. Les phases sont sequentielles dans leurs dependances mais certains modules peuvent se developper en parallele au sein d'une phase.

---

## 2. Graphe de dependances

```
Phase 0 ──→ Phase 1 ──→ Phase 2 ──→ Phase 3 ──→ Phase 4
               │                        │            │
               │                        └────→ Phase 5
               │                                     │
               └──────────────────────────────→ Phase 6
                                                     │
                                              Phase 7 (Phase 5 + Phase 3 requis)
```

| Phase | Depend de |
|-------|-----------|
| 0 — Fondation | Rien (debut) |
| 1 — Catalogue | Phase 0 |
| 2 — Optimisation | Phase 1 |
| 3 — Liseuse | Phase 2 (variantes), Phase 1 (oeuvres) |
| 4 — Paiement | Phase 3 (liseuse pour ecran fin demo) |
| 5 — Favoris et MWS | Phase 3 (lecture), Phase 4 (licences pour statut achat) |
| 6 — Gamification | Phase 3 (lecture pour attribution XP) |
| 7 — Portail Agrege | Phase 5 (MWS manifestes), Phase 3 (liseuse pour redirection) |

---

## 3. Detail des phases

### Phase 0 — Fondation

**Objectif** : Creer le crate `jaymanga`, definir le schema de base de donnees, les types de domaine et la configuration vendeur. A la fin de cette phase, le crate compile et les tests unitaires sur les types passent.

#### Modules a implementer

| Module | Fichier(s) | Description |
|--------|-----------|-------------|
| Crate skeleton | `Cargo.toml`, `src/lib.rs` | Crate avec dependances, feature flags, lints. |
| Types catalogue | `data/types.rs` | `Work`, `Chapter`, `Page`, `Series`, `SellerConfig`, `OptimizationConfig`, enums. |
| Types paiement | `data/types_payment.rs` | `PurchaseLicense`, `PaymentTransaction`, `Promotion`, enums. |
| Types lecteur | `data/types_reader.rs` | `ReaderFavorite`, `ReaderProgression`, `ReaderBadge`, enums. |
| Types aggregation | `data/types_aggregator.rs` | `AggregatedCatalogEntry`, `IndexedSeller`, `OnlineStatus`. |
| Types federation | `data/types_federation.rs` | `FederationCatalogEntry`, `FederationInfo`. |
| Schema SQL | `data/schema.sql` | Toutes les tables (catalogue, paiement, lecteur, aggregation). |
| Data module | `data/mod.rs` | Feature flags, re-exports. |
| KindMother DB | `data/kindmother_db.rs` | Implementation `legacy-sqlite` : init schema, CRUD basique pour `SellerConfig`. |
| Erreurs | `lib.rs` | `JayMangaError` avec `thiserror`. |

#### Criteres de validation

- [ ] `cargo build` et `cargo build --features kindmother-only --no-default-features` compilent sans erreur.
- [ ] `cargo clippy` sans warning (pedantic).
- [ ] Tests unitaires : serialisation/deserialisation de tous les types, construction des enums, validation des valeurs par defaut.
- [ ] Le schema SQL s'initialise correctement sur une base vide.
- [ ] `seller_config_get` et `seller_config_upsert` fonctionnent.

#### Livrable

Le crate `jaymanga` existe dans le workspace, compile, et est enregistre dans le `Cargo.toml` racine sous `[workspace.members]`.

---

### Phase 1 — Catalogue

**Objectif** : Le vendeur peut importer des oeuvres (fichiers, dossiers, archives), creer des series, gerer les metadonnees et les statuts de publication. Le catalogue est persistant.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| CRUD catalogue | `data/kindmother_db.rs` | Phase 0 (schema) |
| Import fichiers | `domain/catalog.rs` | CRUD catalogue |
| Import dossier | `domain/catalog.rs` | CRUD catalogue |
| Import archive (ZIP/CBZ) | `domain/catalog.rs` | CRUD catalogue |
| Import incremental | `domain/catalog.rs` | CRUD catalogue |
| Series et organisation | `domain/catalog.rs` | CRUD catalogue |
| Validation metadonnees | `domain/catalog.rs` | Types |
| Validation demo pages (RM-07) | `domain/catalog.rs` | Types |
| API catalogue | `api/catalog_api.rs` | CRUD catalogue |

#### Criteres de validation

- [ ] Import d'un dossier de 50 pages → oeuvre creee avec 1 chapitre et 50 pages.
- [ ] Import d'un ZIP avec 3 sous-dossiers → oeuvre avec 3 chapitres.
- [ ] Import incremental : ajout d'un chapitre a une oeuvre existante.
- [ ] Creation de serie, association d'oeuvres, reordonnement des volumes.
- [ ] Modification de statut : draft → published → archived.
- [ ] `demo_pages_count` respecte RM-07 (max 50% du total, min 1).
- [ ] API `/api/jaymanga/catalog` retourne les oeuvres publiees avec filtres.

#### Livrable

Le vendeur peut publier et gerer un catalogue de manga depuis Central. Le catalogue est consultable via l'API REST.

---

### Phase 2 — Optimisation images

**Objectif** : A l'import, les pages sont automatiquement optimisees en arriere-plan. Les variantes (HD, SD, mobile, thumb) sont generees en WebP/AVIF.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Moteur d'optimisation | `domain/optimizer.rs` | Phase 1 (pages en DB) |
| Selection de variante | `domain/optimizer.rs` | Variantes generees |
| API media | `api/media_api.rs` | Variantes generees |
| Configuration optimisation | `data/kindmother_db.rs` | Phase 0 (schema) |

#### Criteres de validation

- [ ] Import d'une page 4000x6000 JPEG → generation des 4 variantes (HD, SD, mobile, thumb) en WebP.
- [ ] Les fichiers originaux sont preserves dans `originals/`.
- [ ] L'optimisation tourne en arriere-plan (async) sans bloquer l'import.
- [ ] `optimizer_select_variant(viewport=800, ratio=1)` retourne la variante mobile.
- [ ] `optimizer_select_variant(viewport=1920, ratio=2)` retourne la variante HD.
- [ ] API `/api/jaymanga/media/page/{id}` sert la bonne variante selon `Accept` et viewport.
- [ ] Le ratio de compression est affichable (taille originale vs optimisee).
- [ ] Re-optimisation avec nouveaux parametres regenere les variantes.

#### Livrable

Les pages sont optimisees automatiquement. L'API media sert les variantes adaptees a chaque appareil.

---

### Phase 3 — Liseuse et lecture

**Objectif** : Le lecteur peut lire les oeuvres en ligne (Portail web) et localement (Central). Les 5 formats de lecture sont supportes. Les pages de demonstration sont gerees.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Logique lecteur | `domain/reader.rs` | Phase 2 (variantes) |
| API lecteur (progression) | `api/reader_api.rs` | domain/reader |
| Templates liseuse web | `web/portal.rs`, `web/templates/reader.html` | API media, domain/reader |
| Composant liseuse Dioxus | `JayMangaReader` (Central) | domain/reader |
| Verification demo pages | `domain/reader.rs` | Phase 1 (demo_pages_count) |
| Ecran fin de demonstration | Templates + composant | Verification licence |

#### Sous-modules liseuse

| Sous-module | Description | Format |
|-------------|-------------|--------|
| Mode Manga | Pages RTL, double-page desktop | `manga` |
| Mode Webtoon | Defilement vertical continu, lazy loading | `webtoon` |
| Mode Landscape | Pages LTR, plein largeur | `landscape` |
| Mode Comics | Pages LTR, double-page LTR | `comics` |
| Mode Libre | Ratio variable par page, sens configurable | `free` |

#### Criteres de validation

- [ ] Liseuse web : navigation page par page (clic zones, fleches clavier, swipe).
- [ ] Liseuse web : mode webtoon avec defilement vertical et lazy loading.
- [ ] Liseuse web : mode plein ecran, zoom, mode sombre.
- [ ] Liseuse web : pre-chargement des 2-3 pages suivantes.
- [ ] Demo : les N premieres pages sont accessibles, page N+1 retourne 403.
- [ ] Demo : ecran d'incitation a l'achat apres la derniere page de demo.
- [ ] Sauvegarde automatique de la position de lecture.
- [ ] Reprise de lecture a la derniere page lue.
- [ ] Liseuse native (Dioxus) : meme fonctionnement avec fichiers locaux.
- [ ] Table des matieres avec chapitres et statut de lecture.
- [ ] Barre de progression dans le chapitre et dans l'oeuvre.

#### Livrable

Les lecteurs peuvent lire les oeuvres dans les 5 formats, avec pages de demonstration fonctionnelles. La liseuse web et la liseuse native sont operationnelles.

---

### Phase 4 — Paiement et licences

**Objectif** : Le lecteur peut acheter des oeuvres. Le vendeur gere ses ventes, configure les passerelles de paiement, et administre les licences.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Logique paiement | `domain/payment.rs` | Phase 0 (types paiement) |
| Logique promotions | `domain/promotion.rs` | Phase 0 (types) |
| Verification licences | `auth/license_verify.rs` | Phase 3 (liseuse) |
| API paiement | `api/payment_api.rs` | domain/payment |
| CRUD licences | `data/kindmother_db.rs` | Phase 0 (schema) |
| CRUD transactions | `data/kindmother_db.rs` | Phase 0 (schema) |
| Webhook passerelle | `api/payment_api.rs` | Config passerelle |
| Export CSV/PDF ventes | `export/csv.rs`, `export/pdf.rs` | Transactions en DB |
| Auth permissions | `auth/permissions.rs` | Phase 0 |
| Templates panier/checkout | `web/templates/` | API paiement |
| Composants vente (Dioxus) | `JayMangaDashboard`, `JayMangaSalesAdmin` | domain/payment |

#### Criteres de validation

- [ ] Ajout au panier, modification, detection de doublons.
- [ ] Checkout avec calcul du total (incluant promotions).
- [ ] Paiement par carte : redirection vers passerelle, callback webhook, licence generee.
- [ ] Paiement par virement : transaction pending, confirmation manuelle par le vendeur, licence generee.
- [ ] Verification de licence : page payante accessible seulement avec licence active.
- [ ] Remboursement total : licence revoquee, acces en ligne coupe.
- [ ] Remboursement partiel : licence reste active pour le contenu non rembourse.
- [ ] Promotions : -30% applique correctement, prix barre affiche.
- [ ] Expiration transactions pending apres 30 jours.
- [ ] Tableau de bord vendeur : revenus, ventes, top oeuvres, transactions en attente.
- [ ] Export CSV : transactions filtrees par periode.

#### Livrable

Le cycle complet achat → paiement → licence → acces est fonctionnel. Le vendeur administre ses ventes depuis Central.

---

### Phase 5 — Favoris, telechargement et MWS

**Objectif** : Le lecteur peut mettre des oeuvres en favoris (cross-COG), telecharger pour lire hors-ligne, et verifier la presence des COGs vendeurs via le MWS. Le manifeste JayManga est publie sur le Tracker.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Logique favoris | `domain/favorites.rs` | Phase 4 (statut achat) |
| Logique telechargement | `domain/download.rs` | Phase 4 (licences) |
| Integration MWS presence | `services/mws/presence.rs` | miyuwebway_participant |
| Integration MWS manifestes | `services/mws/manifests.rs` | miyuwebway_participant |
| Integration MWS decouverte | `services/mws/discovery.rs` | miyuwebway_participant |
| API presence | `api/presence_api.rs` | services/mws |
| API federation | `api/federation_api.rs` | Phase 1 (catalogue) |
| CRUD favoris | `data/kindmother_db.rs` | Phase 0 (schema) |
| Sync metadonnees cache | `domain/favorites.rs` | API federation |
| Composant Bibliotheque | `JayMangaLibrary` | favoris, presence |

#### Criteres de validation

- [ ] Ajout d'un favori cross-COG : `seller_cog_id + work_id` stockes localement.
- [ ] Affichage des favoris avec metadonnees en cache (titre, couverture, progression).
- [ ] Indicateur de presence : en ligne (vert), hors-ligne (gris), inconnu (blanc).
- [ ] `QUERY_PRESENCE_BATCH` : verification de presence de tous les COGs favoris en une requete.
- [ ] Telechargement : licence valide + download autorise + COG en ligne → fichiers stockes localement.
- [ ] Verification SHA-256 de chaque fichier telecharge.
- [ ] Lecture hors-ligne : liseuse native ouvre les fichiers locaux sans connexion.
- [ ] Manifeste JayManga publie sur le Tracker avec les bonnes metadonnees.
- [ ] API federation `/api/jaymanga/federation/catalog` retourne le catalogue public.
- [ ] API federation `/api/jaymanga/federation/catalog/since/{ts}` retourne le delta.
- [ ] `allow_aggregation = false` → federation retourne 403.
- [ ] Sync metadonnees : a l'ouverture de la bibliotheque, les caches sont mis a jour pour les COGs en ligne.

#### Livrable

L'experience lecteur cross-COG est complete : favoris, presence, telechargement, lecture hors-ligne. L'infrastructure MWS (manifestes, federation) est en place pour le Portail Agrege.

---

### Phase 6 — Gamification

**Objectif** : Le systeme de progression lecteur est operationnel : XP par page/chapitre/oeuvre, niveaux, streaks, badges. Miou intervient pour les evenements de progression.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Logique gamification | `domain/gamification.rs` | Phase 3 (lecture) |
| CRUD progression | `data/kindmother_db.rs` | Phase 0 (schema) |
| CRUD badges | `data/kindmother_db.rs` | Phase 0 (schema) |
| Composant profil | `JayMangaProfile` | progression, badges |
| Notifications Miou | Integration Miou audio/text | domain/gamification |
| API XP (web) | `api/reader_api.rs` | domain/gamification |
| Onboarding Miou (vendeur) | Composant Central | Phase 1 (publication) |
| Onboarding Miou (lecteur) | Templates web + composant | Phase 3 (liseuse) |

#### Criteres de validation

- [ ] Lire une page (>3s manga, >2s webtoon) → +1 XP.
- [ ] Terminer un chapitre → +10 XP bonus.
- [ ] Terminer une oeuvre → +50 XP bonus.
- [ ] Premiere lecture du jour → +5 XP bonus quotidien.
- [ ] Lire un nouveau genre → +15 XP.
- [ ] Passage de niveau : 0→100 XP = niveau 2 (Lecteur), animation.
- [ ] Streak : 5 pages lues → +1 jour de streak, affichage flamme.
- [ ] Bouclier de streak : 1 jour manque sans perte (1 fois/semaine).
- [ ] Badge "Premier Chapitre" attribue a la fin du premier chapitre.
- [ ] Badge "Explorateur" attribue apres 3 genres differents.
- [ ] Profil lecteur affiche : niveau, XP, streak, badges, statistiques.
- [ ] Miou : toast "Niveau 3 — Passione !" a la montee de niveau.
- [ ] Onboarding lecteur : 4 etapes avec possibilite de skip.
- [ ] Onboarding vendeur : guidage import → publication → dashboard.
- [ ] Lecteur visiteur : XP en localStorage, suggestion COG apres niveau 2 (une fois).

#### Livrable

Le systeme de progression complet est operationnel sur les 3 interfaces. Miou guide les nouveaux utilisateurs et celebre les accomplissements.

---

### Phase 7 — Portail Agrege

**Objectif** : Le Portail Agrege est operationnel : un COG aggregateur collecte les catalogues des COGs JayManga via le MWS, les presente dans une interface unifiee, et grise les COGs hors-ligne.

#### Modules a implementer

| Module | Fichier(s) | Dependance |
|--------|-----------|------------|
| Logique aggregation | `domain/aggregator.rs` | Phase 5 (MWS manifestes) |
| Collecteur periodique | `domain/aggregator.rs` | services/mws/discovery |
| Cache catalogue | `data/kindmother_db.rs` | Phase 0 (schema) |
| Moteur de recherche | `domain/aggregator.rs` | Cache catalogue |
| Recommandations | `domain/aggregator.rs` | Cache catalogue |
| Templates Portail Agrege | `web/aggregator_portal.rs`, `web/templates/aggregate.html` | domain/aggregator |
| Configuration aggregateur | `data/kindmother_db.rs` | Phase 0 (schema) |
| Moderation | `domain/aggregator.rs` | Config aggregateur |
| Statistiques engagement | `data/kindmother_db.rs` | Templates |

#### Criteres de validation

- [ ] Activation du Portail Agrege : `aggregator_enabled = true` dans la config.
- [ ] Cycle de sync : `QUERY_MANIFESTS_BY_SERVICE("jaymanga")` au Tracker, filtrage `allow_aggregation`, collecte incrementielle.
- [ ] Phase 1 sync : obtention des manifestes depuis le Tracker (pas de connexion directe aux COGs).
- [ ] Phase 2 sync : connexion directe uniquement aux COGs modifies depuis le dernier sync.
- [ ] Cache : metadonnees et miniatures stockees localement.
- [ ] Interface unifiee : catalogue global avec recherche, filtres (genre, format, prix, langue, disponibilite).
- [ ] COGs hors-ligne : oeuvres grisees, bouton d'action desactive.
- [ ] Fiche oeuvre intermediaire : metadonnees en cache + bouton "Lire/Acheter sur COG X" → redirection.
- [ ] Page vendeur : nom, description, catalogue filtre, lien vers le Portail d'origine.
- [ ] Recommandations : oeuvres similaires basees sur genres, tags, auteur communs.
- [ ] Moderation : blocage d'un COG → ses oeuvres disparaissent du Portail Agrege.
- [ ] Indicateur de fraicheur du cache : "Mis a jour il y a X heures".
- [ ] Volume de donnees : 100 COGs × 50 oeuvres ≈ 155 Mo de cache (acceptable).

#### Livrable

Le Portail Agrege offre une vue unifiee de l'ecosysteme JayManga. Les lecteurs decouvrent les manga de tous les COGs depuis une seule interface.

---

## 4. Integration Miyukini Central

A chaque phase, l'integration dans Central est requise :

| Phase | Integration Central |
|-------|---------------------|
| 0 | Enregistrer `jaymanga` comme service dans `miyukini-central/src/services/`. Declaration dans le catalogue de services. |
| 1 | `JayMangaServiceCard` dans le Salon. Ecrans `CatalogAdmin` et `WorkEditor` accessibles depuis la navigation. |
| 3 | `JayMangaReader` (liseuse native) et `JayMangaLibrary` (ecran avec onglet "En cours"). |
| 4 | `JayMangaDashboard` (ventes), `JayMangaSalesAdmin`. |
| 5 | `JayMangaLibrary` complet (5 onglets), indicateurs de presence. |
| 6 | `JayMangaProfile` (progression, badges), integration Miou dans les ecrans existants. |
| 7 | Configuration du Portail Agrege dans `JayMangaSettings`. |

---

## 5. Integration apps/central (Tauri / Dioxus)

L'application Tauri (`apps/central/`) doit integrer les composants JayManga :

| Phase | Fichier apps/central | Action |
|-------|---------------------|--------|
| 0 | `Cargo.toml` | Ajouter `jaymanga` comme dependance |
| 1 | `src/services/jaymanga/mod.rs` | Creer le module service JayManga avec `JayMangaView` enum |
| 1 | `src/services/jaymanga/card.rs` | `JayMangaServiceCard` dans le Salon |
| 1 | `src/app_state.rs` | Extension `JayMangaState` dans `AppContext` |
| 3 | `src/services/jaymanga/reader.rs` | Liseuse native Dioxus |
| 6 | `src/services/jaymanga/profile.rs` | Profil lecteur avec gamification |

---

## 6. Integration Web Portal

Les templates web sont servis par le module `web/` du crate `jaymanga`, integre dans le serveur web du COG (surface web du Portail).

| Phase | Route | Template |
|-------|-------|----------|
| 1 | `/manga` | `catalog.html` — Catalogue public |
| 1 | `/manga/{work_id}` | `work.html` — Fiche oeuvre |
| 3 | `/manga/read/{chapter_id}` | `reader.html` — Liseuse web |
| 4 | `/manga/cart` | `cart.html` — Panier |
| 4 | `/manga/checkout` | `checkout.html` — Checkout |
| 7 | `/manga/aggregate` | `aggregate.html` — Portail Agrege |
| 7 | `/manga/aggregate/seller/{cog_id}` | `seller.html` — Page vendeur |

---

## 7. Ordre de developpement au sein d'une phase

Pour chaque phase, l'ordre de developpement recommande est :

```
1. Types de domaine (si nouveaux)
2. Schema SQL (si nouvelles tables)
3. CRUD data layer (kindmother_db)
4. Logique domaine (domain/)
5. Tests unitaires domaine
6. API REST (api/)
7. Templates web (web/)
8. Composants Dioxus (Central)
9. Tests d'integration
10. Integration Central / Tauri
```

---

## 8. Dependances externes a surveiller

| Dependance | Phase | Risque | Mitigation |
|------------|-------|--------|------------|
| `miyuwebway_participant` | 5 | Le crate doit supporter les manifestes de services | Developper en parallele ou utiliser des mocks MWS |
| Passerelles de paiement (Stripe, PayPal) | 4 | Configuration externe, API tierce | Commencer par le mode `manual` (virement), ajouter les passerelles incrementalement |
| `image` crate (traitement d'images) | 2 | Performance sur les gros fichiers | Limiter la concurrence (`max_concurrent_jobs`), tester avec des fichiers realistes |
| Dioxus (UI native) | 1+ | API en evolution | Suivre les patterns du skill `miyukini-dioxus-ui`, tester regulierement |

---

## 9. Jalons de livraison

| Jalon | Phase | Critere |
|-------|-------|---------|
| **M0 — Crate compilee** | 0 | Le crate existe, compile, types et schema valides. |
| **M1 — Catalogue publiable** | 1 | Le vendeur peut importer et publier des manga. Le catalogue est consultable via API. |
| **M2 — Pages optimisees** | 2 | Les images sont optimisees en arriere-plan. L'API media sert les variantes. |
| **M3 — Lecture fonctionnelle** | 3 | Le lecteur peut lire en ligne (5 formats) avec pages de demo. Liseuse web et native OK. |
| **M4 — Ventes actives** | 4 | Le cycle achat → paiement → licence → lecture est complet. Le vendeur gere ses ventes. |
| **M5 — Ecosysteme connecte** | 5 | Favoris cross-COG, presence MWS, telechargement hors-ligne, manifestes publies. |
| **M6 — Engagement lecteur** | 6 | XP, niveaux, streaks, badges. Miou guide et celebre. |
| **M7 — Portail Agrege operationnel** | 7 | Vue unifiee inter-COG, collecte automatique, COGs grises, redirection. |

---

## 10. Regles transversales

### 10.1 Conventions de code

| Regle | Description |
|-------|-------------|
| `unsafe_code = "forbid"` | Aucun code unsafe. |
| Clippy pedantic | Tous les warnings clippy pedantic actives. |
| UUIDs v4 | `uuid::Uuid::new_v4().to_string()` pour tous les identifiants primaires. |
| Timestamps ISO 8601 | `chrono::Utc::now().to_rfc3339()` pour toutes les dates. |
| Prix en centimes | Tous les montants financiers en `i64` (centimes). Jamais de `f64` pour l'argent. |
| Feature flags | `legacy-sqlite` (defaut) et `kindmother-only` pour la couche data. |
| Thread-safety | `Mutex<Connection>` pour les acces synchrones a la DB. |

### 10.2 Tests

| Type | Couverture minimale |
|------|---------------------|
| Unitaires | Tous les modules de domaine. |
| Integration | Flux complets (import → lecture, achat → licence → acces). |
| Performance | Optimisation d'images, chargement de catalogue >1000 oeuvres. |

### 10.3 Documentation

Chaque module public doit avoir un commentaire `//!` en tete de fichier et des `///` sur les fonctions publiques. Le `lib.rs` inclut la description du crate et l'annotation MSCM :

```rust
//! @id service.media.jaymanga
//! @role media_distribution
//! @layer domain
//! @human Service de lecture et vente de manga en ligne
//! @do manage_manga_catalog_reading_sales
```

---

## 11. References

| Document | Role |
|----------|------|
| [JayManga - Guide Implementation](./JayManga%20-%20Guide%20Implementation.md) | Structure technique, types, modules, APIs. |
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Vision, scope, decisions structurantes. |
| [JayManga - Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md) | Spec import, formats, optimisation (Phases 1-2). |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Spec liseuse (Phase 3). |
| [JayManga - Achat et Paiement](./JayManga%20-%20Achat%20et%20Paiement.md) | Spec paiement (Phase 4). |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Spec favoris, telechargement, presence (Phase 5). |
| [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) | Spec aggregation (Phase 7). |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Spec gamification (Phase 6). |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Spec UI Dioxus. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Spec UI mobile. |
| [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Spec UI web. |

---

**Document** : JayManga — Plan d'implementation
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Plan d'implementation — phases, ordre, dependances, jalons, criteres de validation.
