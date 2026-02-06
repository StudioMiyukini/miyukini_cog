# JayXpose — Opérateurs et Toolkits

## Contexte

Ce document décrit les **Opérateurs** (Strate 7) et **Kits d'Outils** (Strate 6) du service **JayXpose** : profil exposant enrichi, catalogue de produits, site vitrine, coffre-fort documentaire, annuaire, et **liaison avec JayFestival**. Il s'appuie sur l'[Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) et le [Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md).

**Références** : Glossaire Miyukini (Opérateur, Outil, Kit d'Outils, Mandat de Permission) ; [JayFestival - Exposants Operateurs et Toolkits](../JayFestival/publics/Exposants/Exposants%20-%20Operateurs%20et%20Toolkits.md).

## Portée / Scope

- **Périmètre** : Opérateurs et Kits JayXpose (profil, catalogue, vitrine, documents, annuaire, intégration JayFestival).
- **Hors périmètre** : Candidatures, participations, facturation (JayFestival / JayKonta) ; spécifications API détaillées (contrats dédiés).

---

## 1. Référence glossaire Miyukini

| Concept | Définition |
|---------|------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d'Outils (Toolkit)** | Composition officielle d'Outils, validée et déclarée par l'environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

---

## 2. Opérateurs JayXpose

### 2.1 Opérateur « JayXpose Profil » (profil exposant)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **création et la mise à jour du profil exposant enrichi** (fiche entreprise complète : identité, juridique, contacts multiples, adresses, visuels, réseaux sociaux). |
| **Public servi** | Exposants authentifiés (rôle exposant, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Création du profil à l'inscription (champs minimaux) ; mise à jour complète de la fiche entreprise ; gestion des contacts multiples ; upload logo et bannière ; gestion réseaux sociaux ; gestion des informations juridiques (SIRET, SIREN, APE, immatriculation). |
| **Ne fait pas** | Catalogue produits (JayXpose Catalogue) ; documents (JayXpose Documents) ; vitrine (JayXpose Vitrine) ; candidatures (JayFestival). |

### 2.2 Opérateur « JayXpose Catalogue » (catalogue de produits)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **gestion du catalogue de produits** de l'exposant : fiches produits, catégories, visuels, mise en avant. |
| **Public servi** | Exposant (gestion de son catalogue) ; visiteurs (consultation publique du catalogue). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; écriture = propriétaire uniquement ; lecture = selon politique (public si catalogue visible). |
| **Capacités exposées** | CRUD fiches produits (nom, description, prix, catégorie, disponibilité, visuels) ; gestion catégories/collections ; désignation produits vedettes ; gestion visuels multiples par produit ; liste catalogue public (paginée, filtrée). |
| **Ne fait pas** | Vente en ligne / paiement (Miyustore) ; facturation (JayKonta). |

### 2.3 Opérateur « JayXpose Vitrine » (site vitrine)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **configuration et publication du site vitrine** de l'exposant : pages, personnalisation, SEO, activation. |
| **Public servi** | Exposant (configuration) ; visiteurs (consultation du site vitrine publié). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; écriture = propriétaire ; lecture publique si vitrine publiée. |
| **Capacités exposées** | Configuration des pages (accueil, catalogue, présentation, contact) ; personnalisation (slug URL, couleurs, SEO) ; éditeur de contenu pour la page présentation ; prévisualisation ; publication / suspension / passage en brouillon. |
| **Ne fait pas** | Gestion du profil (JayXpose Profil) ; gestion du catalogue (JayXpose Catalogue). |

### 2.4 Opérateur « JayXpose Documents » (coffre-fort documentaire)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **gestion du coffre-fort documentaire** : upload, versioning, statuts, alertes, partage gouverné. |
| **Public servi** | Exposant (gestion de ses documents) ; organisateur JayFestival (consultation des documents partagés, via Mandat). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; sécurité **Critical (3)** minimum (WorrySentinel) ; persistance sécurisée (KindMother) ; auditabilité complète. |
| **Capacités exposées** | Upload documents (types prédéfinis : RIB, assurance, KBIS, immatriculation, licence, URSSAF, carte pro, diplôme, autre) ; versioning (remplacement) ; renseignement date d'expiration ; consultation statuts ; alertes expiration ; partage gouverné (accepter/refuser/révoquer) ; centralisation (un document → N partages). |
| **Ne fait pas** | Validation réglementaire des documents (humain / processus externe) ; facturation (JayKonta). |

### 2.5 Opérateur « JayXpose Annuaire » (fiche publique et référencement)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **fiche publique**, la **liste annuaire** et la **recherche d'exposants** pour le référencement dans l'annuaire. |
| **Public servi** | Exposant (gestion de sa fiche publique, visibilité, confidentialité) ; visiteurs / utilisateurs (consultation annuaire) ; organisateur (consultation fiche exposant). |
| **Gouvernance** | Lecture publique ou selon Mandat ; écriture (visibilité, confidentialité) = propriétaire uniquement. |
| **Capacités exposées** | Liste des exposants visibles dans l'annuaire (paginée, filtrée) ; fiche détaillée (profil + catalogue aperçu + éditions participées + lien vitrine) ; recherche texte ; filtres (secteur, localisation, mots-clés, événement) ; gestion visibilité et confidentialité par champ. |
| **Ne fait pas** | Candidatures, participations (JayFestival). |

### 2.6 Synthèse des Opérateurs

| Opérateur | Usage | Besoins couverts |
|-----------|-------|------------------|
| **JayXpose Profil** | Profil exposant enrichi (fiche entreprise complète). | JXP-01 à JXP-05, JXP-13 à JXP-18. |
| **JayXpose Catalogue** | Catalogue de produits (fiches, catégories, visuels, vedettes). | JXP-20 à JXP-29. |
| **JayXpose Vitrine** | Site vitrine (pages, personnalisation, SEO, publication). | JXP-30 à JXP-38. |
| **JayXpose Documents** | Coffre-fort documentaire (upload, versioning, partage gouverné). | JXP-40 à JXP-49. |
| **JayXpose Annuaire** | Fiche publique, annuaire, recherche, référencement. | JXP-06 à JXP-09, JXP-50 à JXP-53. |

---

## 3. Kits d'Outils JayXpose

### 3.1 Kit « Profil Exposant »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **lecture et l'écriture du profil exposant enrichi** (fiche entreprise). |
| **Outils agrégés** | `exposant.profile.get` (profil par user_id), `exposant.profile.create` (création à l'inscription), `exposant.profile.update` (mise à jour complète), `exposant.logo.upload`, `exposant.banner.upload`, `exposant.contacts.update` (contacts multiples), `exposant.juridique.update` (SIRET, SIREN, APE…). |
| **Consommé par** | JayXpose Profil ; JayFestival (inscription, Mon compte, candidature pré-remplie). |
| **Composants sous-jacents** | KindMother (persistance) ; alpha = Supabase (table `exposants`). |

### 3.2 Kit « Catalogue Produits »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **gestion du catalogue** (CRUD produits, catégories, visuels). |
| **Outils agrégés** | `catalogue.produit.create`, `catalogue.produit.update`, `catalogue.produit.delete`, `catalogue.produit.get`, `catalogue.produit.list` (paginé, filtré), `catalogue.categorie.create`, `catalogue.categorie.update`, `catalogue.categorie.delete`, `catalogue.categorie.list`, `catalogue.visuel.upload`, `catalogue.visuel.delete`, `catalogue.visuel.reorder`, `catalogue.vedettes.set`. |
| **Consommé par** | JayXpose Catalogue ; JayXpose Vitrine (page catalogue) ; JayXpose Annuaire (aperçu catalogue) ; JayFestival (catalogue dans répertoire). |
| **Composants sous-jacents** | KindMother ; Miyumedia (stockage visuels) ; alpha = Supabase (`produits_catalogue`, `categories_produits`, `produits_visuels`). |

### 3.3 Kit « Vitrine Exposant »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **configuration et le rendu du site vitrine** (pages, personnalisation, publication). |
| **Outils agrégés** | `vitrine.config.get`, `vitrine.config.update` (slug, couleurs, SEO), `vitrine.page.get`, `vitrine.page.update` (contenu page présentation), `vitrine.pages.list`, `vitrine.publish`, `vitrine.unpublish`, `vitrine.preview.render`. |
| **Consommé par** | JayXpose Vitrine ; rendu public (lecture vitrine). |
| **Composants sous-jacents** | KindMother ; Miyucms (éditeur contenu) ; alpha = Supabase (`exposants` + `vitrine_pages`). |

### 3.4 Kit « Coffre-Fort Documents »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le **stockage sécurisé et le partage gouverné** des documents professionnels. |
| **Outils agrégés** | `document.upload`, `document.replace` (nouvelle version), `document.get`, `document.list`, `document.delete`, `document.status.update`, `document.expiration.check`, `document.share.request`, `document.share.respond` (accepter/refuser), `document.share.revoke`, `document.share.list` (partages actifs). |
| **Consommé par** | JayXpose Documents ; JayFestival (demande et consultation documents partagés). |
| **Composants sous-jacents** | KindMother ; WorrySentinel (sécurité Critical) ; Miyumedia (stockage sécurisé) ; alpha = Supabase (`documents_professionnels`, `documents_partages`, Storage bucket sécurisé). |

### 3.5 Kit « Annuaire Exposants »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **liste publique des exposants**, la **fiche détaillée** et la **recherche**. |
| **Outils agrégés** | `annuaire.list` (paginé, filtré : secteur, localisation, mots-clés, événement), `annuaire.search` (full-text), `annuaire.fiche.get` (fiche détaillée : profil + catalogue aperçu + éditions), `annuaire.visibility.set` (activer/désactiver), `annuaire.confidentialite.set` (politique par champ). |
| **Consommé par** | JayXpose Annuaire ; JayFestival Catalogue (répertoire) ; JayFestival Organisateur (liste exposants). |
| **Composants sous-jacents** | KindMother ; alpha = Supabase (`exposants`, `produits_catalogue`, `editions_exposants`). |

### 3.6 Kit « Liaison JayFestival »

| Attribut | Description |
|----------|-------------|
| **Rôle** | Définir le **contrat d'intégration** : données que JayFestival lit depuis JayXpose (profil, catalogue, documents partagés). |
| **Outils agrégés** | Lecture profil pour fiche exposant (organisateur, catalogue) ; lecture catalogue pour répertoire enrichi ; lecture documents partagés pour candidatures ; pré-remplissage formulaire candidature ; historique participations ; notifications croisées. |
| **Consommé par** | JayFestival (tous écrans exposant, répertoire, fiche, candidatures, demande documents). |
| **Données** | Alpha : mêmes tables Supabase ; pas de duplication ; JayFestival et JayXpose partagent la source. |

### 3.7 Synthèse des Kits d'Outils

| Kit d'Outils | Opérateur(s) consommateur(s) | Besoins couverts |
|--------------|------------------------------|--------------------|
| **Profil Exposant** | JayXpose Profil, JayFestival | Fiche entreprise enrichie. |
| **Catalogue Produits** | JayXpose Catalogue, JayXpose Vitrine, JayXpose Annuaire, JayFestival | Catalogue complet. |
| **Vitrine Exposant** | JayXpose Vitrine | Site vitrine. |
| **Coffre-Fort Documents** | JayXpose Documents, JayFestival | Documents professionnels + partage. |
| **Annuaire Exposants** | JayXpose Annuaire, JayFestival | Référencement et recherche. |
| **Liaison JayFestival** | JayFestival | Intégration complète. |

---

## 4. Matrice Parcours / Opérateurs / Kits d'Outils

| Parcours ou livrable | Opérateur | Kit(s) d'Outils |
|----------------------|-----------|------------------|
| Inscription exposant (création profil) | JayXpose Profil | Profil Exposant. |
| Fiche entreprise complète | JayXpose Profil | Profil Exposant. |
| Catalogue : création / modification produits | JayXpose Catalogue | Catalogue Produits. |
| Catalogue : catégories et vedettes | JayXpose Catalogue | Catalogue Produits. |
| Site vitrine : configuration et publication | JayXpose Vitrine | Vitrine Exposant, Catalogue Produits. |
| Site vitrine : page présentation | JayXpose Vitrine | Vitrine Exposant. |
| Documents : upload et gestion | JayXpose Documents | Coffre-Fort Documents. |
| Documents : partage pour candidature | JayXpose Documents | Coffre-Fort Documents, Liaison JayFestival. |
| Fiche publique (annuaire) | JayXpose Annuaire | Annuaire Exposants. |
| Liste annuaire + recherche | JayXpose Annuaire | Annuaire Exposants. |
| Fiche exposant dans JayFestival | JayXpose Annuaire | Annuaire Exposants, Liaison JayFestival. |
| Candidature pré-remplie | JayXpose Profil | Profil Exposant, Liaison JayFestival. |
| Catalogue dans répertoire JayFestival | JayXpose Catalogue | Catalogue Produits, Liaison JayFestival. |
| Historique participations | JayXpose Annuaire | Liaison JayFestival. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles | Miyauth, Master Butler | Compte exposant, Mandat, permissions. |
| Persistance alpha | Supabase (tables JayXpose) | Tables exposants, produits, documents, etc. |
| Persistance post-alpha | KindMother, SQLite | Migration documentée. |
| Stockage médias | Miyumedia, Supabase Storage | Logos, bannières, visuels produits, documents PDF. |
| Éditeur contenu | Miyucms | Contenu page présentation vitrine. |
| Sécurité, audit | WorrySentinel | Niveaux de sécurité ; audit documents. |
| JayFestival | JayFestival | Candidatures, participations, répertoire, notifications. |
| JayKonta | JayKonta | Facturation exposant (RIB partagé depuis coffre-fort). |
| JayRDV | JayRDV | Lien réservation depuis vitrine. |

---

## 6. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [JayFestival - Exposants Operateurs et Toolkits](../JayFestival/publics/Exposants/Exposants%20-%20Operateurs%20et%20Toolkits.md)

---

**Document** : JayXpose — Opérateurs et Toolkits
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Référence produit
