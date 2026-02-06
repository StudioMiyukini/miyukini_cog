# JayXpose — Base de données Supabase et migration SQLite

## Contexte

**JayXpose** s'appuie en **version alpha** sur **Supabase** (PostgreSQL + Auth + Storage). Ce document décrit (1) le **schéma complet des tables** JayXpose enrichies (profil, catalogue, documents, vitrine, partages), (2) les **règles RLS**, (3) les **requêtes SQL** pour les parcours exposant, puis (4) la **stratégie de migration** vers **SQLite + KindMother**.

**Références** : [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md), [JayXpose - Analyse des besoins](../JayXpose%20-%20Analyse%20des%20besoins.md), [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](../../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).

## Portée / Scope

- **Périmètre** : Tables Supabase complètes (exposants enrichi, produits_catalogue, categories_produits, produits_visuels, documents_professionnels, documents_versions, documents_partages, documents_audit, vitrine_pages, confidentialite_profil) ; RLS ; requêtes SQL alpha ; buckets Storage ; stratégie migration SQLite + KindMother.
- **Hors périmètre** : Tables purement JayFestival (editions, stands, budget_entries, invoices) — documentées dans la référence JayFestival.

---

## 1. Exception pré-COG : Supabase comme backend alpha

- **Alpha** : Backend = Supabase (PostgreSQL, Auth, Storage). JayXpose lit/écrit toutes les tables exposant ; JayFestival consomme en lecture.
- **Migration obligatoire** : Post-alpha → **SQLite + KindMother** (LOI-1, LOI-2, LOI-3).

---

## 2. Schéma des tables

### 2.1 Table `profiles` (existante — Miyauth)

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, FK auth.users | Identifiant utilisateur. |
| email | TEXT | | Email (Auth). |
| username | TEXT | | Nom d'affichage. |
| user_type | TEXT | | `exhibitor`, `admin`, `manager`, `volunteer`, `visitor`. |
| avatar_url | TEXT | | Avatar. |
| created_at | TIMESTAMPTZ | | Création. |
| updated_at | TIMESTAMPTZ | | Modification. |

### 2.2 Table `exposants` (enrichie)

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, FK profiles(id) | = auth.uid(). |
| company_name | TEXT | NOT NULL | Raison sociale. |
| legal_form | TEXT | | Forme juridique. |
| slogan | TEXT | | Accroche (max 100 car.). |
| description_short | TEXT | | Description courte (max 200 car.). |
| description_long | TEXT | | Description longue (texte formaté). |
| stand_name | TEXT | | Nom du stand (par édition). |
| contact_email | TEXT | | Email contact principal. |
| contact_phone | TEXT | | Téléphone principal. |
| adresse_siege | JSONB | | `{rue, cp, ville, pays}`. |
| adresse_correspondance | JSONB | | Idem, nullable. |
| contact_facturation_nom | TEXT | | Nom contact facturation. |
| contact_facturation_email | TEXT | | Email facturation. |
| contact_facturation_phone | TEXT | | Téléphone facturation. |
| contact_logistique_nom | TEXT | | Nom contact logistique. |
| contact_logistique_email | TEXT | | Email logistique. |
| contact_logistique_phone | TEXT | | Téléphone logistique. |
| logo_url | TEXT | | URL logo (Storage). |
| banner_url | TEXT | | URL bannière (Storage). |
| site_web | TEXT | | Site web externe. |
| siret | TEXT | CHECK (length = 14) | SIRET. |
| siren | TEXT | CHECK (length = 9) | SIREN. |
| code_ape | TEXT | | Code APE/NAF. |
| num_immatriculation | TEXT | | N° immatriculation RM/RCS. |
| secteur | TEXT | | Secteur d'activité. |
| tags | JSONB | | Mots-clés `["artisanat", "bois"]`. |
| social_facebook | TEXT | | URL Facebook. |
| social_instagram | TEXT | | URL Instagram. |
| social_linkedin | TEXT | | URL LinkedIn. |
| social_tiktok | TEXT | | URL TikTok. |
| social_youtube | TEXT | | URL YouTube. |
| social_pinterest | TEXT | | URL Pinterest. |
| social_x | TEXT | | URL X (Twitter). |
| visible_annuaire | BOOLEAN | DEFAULT true | Visible dans l'annuaire. |
| vitrine_slug | TEXT | UNIQUE | Slug URL vitrine. |
| vitrine_status | TEXT | DEFAULT 'brouillon' | `brouillon` / `publiee` / `suspendue`. |
| vitrine_colors | JSONB | | `{primary, secondary, background, text}`. |
| seo_title | TEXT | | Titre SEO. |
| seo_description | TEXT | | Meta description. |
| seo_keywords | TEXT | | Mots-clés SEO. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Création. |
| updated_at | TIMESTAMPTZ | DEFAULT now() | Modification. |

### 2.3 Table `produits_catalogue`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Identifiant produit. |
| exposant_id | UUID | FK exposants(id), NOT NULL | Propriétaire. |
| name | TEXT | NOT NULL | Nom du produit. |
| description | TEXT | | Description. |
| price | NUMERIC | | Prix en centimes (nullable). |
| currency | TEXT | DEFAULT 'EUR' | Devise. |
| category_id | UUID | FK categories_produits(id) | Catégorie. |
| availability | TEXT | DEFAULT 'disponible' | `disponible` / `rupture` / `sur_commande`. |
| is_featured | BOOLEAN | DEFAULT false | Produit vedette. |
| sort_order | INTEGER | DEFAULT 0 | Ordre affichage. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Création. |
| updated_at | TIMESTAMPTZ | DEFAULT now() | Modification. |

### 2.4 Table `categories_produits`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Identifiant. |
| exposant_id | UUID | FK exposants(id), NOT NULL | Propriétaire. |
| name | TEXT | NOT NULL | Nom catégorie. |
| description | TEXT | | Description. |
| sort_order | INTEGER | DEFAULT 0 | Ordre. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Création. |

### 2.5 Table `produits_visuels`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Identifiant. |
| produit_id | UUID | FK produits_catalogue(id) ON DELETE CASCADE, NOT NULL | Produit. |
| url | TEXT | NOT NULL | URL image (Storage). |
| alt_text | TEXT | | Texte alternatif. |
| is_primary | BOOLEAN | DEFAULT false | Image principale. |
| sort_order | INTEGER | DEFAULT 0 | Ordre galerie. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Création. |

### 2.6 Table `documents_professionnels`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, DEFAULT gen_random_uuid() | Identifiant. |
| exposant_id | UUID | FK exposants(id), NOT NULL | Propriétaire. |
| type | TEXT | NOT NULL | `rib` / `assurance` / `kbis` / `immatriculation` / `licence` / `urssaf` / `carte_pro` / `diplome` / `sanitaire` / `autre`. |
| label | TEXT | | Libellé (si type = 'autre'). |
| file_url | TEXT | NOT NULL | URL fichier (Storage sécurisé). |
| file_name | TEXT | NOT NULL | Nom fichier original. |
| file_size | INTEGER | NOT NULL | Taille octets. |
| mime_type | TEXT | NOT NULL | Type MIME. |
| status | TEXT | DEFAULT 'en_attente' | `en_attente` / `valide` / `expire` / `rejete`. |
| expires_at | TIMESTAMPTZ | | Date expiration. |
| version | INTEGER | DEFAULT 1 | N° version. |
| notes | TEXT | | Notes exposant. |
| rejection_reason | TEXT | | Motif rejet. |
| uploaded_at | TIMESTAMPTZ | DEFAULT now() | Upload. |
| validated_at | TIMESTAMPTZ | | Validation. |
| validated_by | UUID | | Validateur. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Création. |
| updated_at | TIMESTAMPTZ | DEFAULT now() | Modification. |

### 2.7 Table `documents_versions`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK | Identifiant. |
| document_id | UUID | FK documents_professionnels(id) | Document parent. |
| version | INTEGER | NOT NULL | N° version. |
| file_url | TEXT | NOT NULL | URL fichier. |
| file_name | TEXT | NOT NULL | Nom fichier. |
| file_size | INTEGER | NOT NULL | Taille. |
| uploaded_at | TIMESTAMPTZ | DEFAULT now() | Upload. |

### 2.8 Table `documents_partages`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK | Identifiant. |
| document_id | UUID | FK documents_professionnels(id) | Document. |
| exposant_id | UUID | FK exposants(id) | Propriétaire. |
| target_user_id | UUID | NOT NULL | Destinataire. |
| target_context_type | TEXT | NOT NULL | `candidature` / `edition` / `administratif`. |
| target_context_id | UUID | | ID contexte. |
| status | TEXT | DEFAULT 'demande' | `demande` / `accepte` / `refuse` / `revoque` / `expire`. |
| message | TEXT | | Message demandeur. |
| requested_at | TIMESTAMPTZ | DEFAULT now() | Demande. |
| responded_at | TIMESTAMPTZ | | Réponse. |
| revoked_at | TIMESTAMPTZ | | Révocation. |
| expires_at | TIMESTAMPTZ | | Expiration partage. |

### 2.9 Table `documents_audit`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK | Identifiant. |
| document_id | UUID | FK | Document. |
| actor_id | UUID | NOT NULL | Auteur action. |
| action | TEXT | NOT NULL | `upload` / `replace` / `validate` / `reject` / `delete` / `share_request` / `share_accept` / `share_refuse` / `share_revoke` / `view`. |
| details | JSONB | | Détails. |
| created_at | TIMESTAMPTZ | DEFAULT now() | Horodatage. |

### 2.10 Table `vitrine_pages`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK | Identifiant. |
| exposant_id | UUID | FK exposants(id) | Propriétaire. |
| page_type | TEXT | NOT NULL | `accueil` / `presentation` / `contact`. |
| content | JSONB | | Contenu (blocs structurés). |
| is_visible | BOOLEAN | DEFAULT true | Page activée. |
| sort_order | INTEGER | DEFAULT 0 | Ordre navigation. |
| updated_at | TIMESTAMPTZ | DEFAULT now() | Modification. |

### 2.11 Table `confidentialite_profil`

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK | Identifiant. |
| exposant_id | UUID | FK exposants(id), UNIQUE | Propriétaire. |
| contact_email_visibility | TEXT | DEFAULT 'authentifie' | Visibilité email. |
| contact_phone_visibility | TEXT | DEFAULT 'organisateur' | Visibilité téléphone. |
| adresse_siege_visibility | TEXT | DEFAULT 'organisateur' | Visibilité adresse. |
| adresse_correspondance_visibility | TEXT | DEFAULT 'prive' | Visibilité adresse 2. |
| description_long_visibility | TEXT | DEFAULT 'public' | Visibilité description. |
| legal_form_visibility | TEXT | DEFAULT 'organisateur' | Visibilité forme juridique. |
| siret_visibility | TEXT | DEFAULT 'organisateur' | Visibilité SIRET. |
| siren_visibility | TEXT | DEFAULT 'prive' | Visibilité SIREN. |
| code_ape_visibility | TEXT | DEFAULT 'prive' | Visibilité APE. |
| num_immatriculation_visibility | TEXT | DEFAULT 'prive' | Visibilité immatriculation. |
| contact_logistique_visibility | TEXT | DEFAULT 'organisateur' | Visibilité contact logistique. |
| prix_catalogue_visibility | TEXT | DEFAULT 'public' | Visibilité prix catalogue. |
| updated_at | TIMESTAMPTZ | DEFAULT now() | Modification. |

### 2.12 Table `editions_exposants` (existante — JayFestival)

| Colonne | Type | Rôle |
|---------|------|------|
| id | UUID | PK. |
| exposant_id | UUID | FK exposants(id). |
| edition_id | UUID | FK editions(id). |
| is_accepted | BOOLEAN | Candidature acceptée. |
| is_validated | BOOLEAN | Participation validée. |
| is_paid | BOOLEAN | Payé. |
| assigned_stand | TEXT | Stand attribué. |
| size_meters | NUMERIC | Surface. |
| created_at, updated_at | TIMESTAMPTZ | Horodatage. |

---

## 3. Index recommandés

```sql
CREATE INDEX idx_exposants_visible ON exposants(visible_annuaire) WHERE visible_annuaire = true;
CREATE INDEX idx_exposants_secteur ON exposants(secteur);
CREATE INDEX idx_exposants_slug ON exposants(vitrine_slug);
CREATE INDEX idx_produits_exposant ON produits_catalogue(exposant_id);
CREATE INDEX idx_produits_featured ON produits_catalogue(exposant_id, is_featured) WHERE is_featured = true;
CREATE INDEX idx_produits_categorie ON produits_catalogue(category_id);
CREATE INDEX idx_visuels_produit ON produits_visuels(produit_id);
CREATE INDEX idx_documents_exposant ON documents_professionnels(exposant_id);
CREATE INDEX idx_documents_status ON documents_professionnels(status);
CREATE INDEX idx_partages_document ON documents_partages(document_id);
CREATE INDEX idx_partages_target ON documents_partages(target_user_id, status);
CREATE INDEX idx_audit_document ON documents_audit(document_id);
CREATE INDEX idx_vitrine_exposant ON vitrine_pages(exposant_id);
CREATE INDEX idx_editions_exposants_edition ON editions_exposants(edition_id, exposant_id);
```

---

## 4. RLS (Row Level Security)

### 4.1 `exposants`

```sql
-- Lecture publique (annuaire)
CREATE POLICY "exposants_select_visible"
  ON exposants FOR SELECT
  USING (visible_annuaire = true);

-- Lecture propriétaire (tous champs)
CREATE POLICY "exposants_select_own"
  ON exposants FOR SELECT
  USING (id = auth.uid());

-- Insert propriétaire
CREATE POLICY "exposants_insert_own"
  ON exposants FOR INSERT
  WITH CHECK (id = auth.uid());

-- Update propriétaire
CREATE POLICY "exposants_update_own"
  ON exposants FOR UPDATE
  USING (id = auth.uid())
  WITH CHECK (id = auth.uid());
```

### 4.2 `produits_catalogue`

```sql
-- Lecture publique (exposant visible)
CREATE POLICY "produits_select_public"
  ON produits_catalogue FOR SELECT
  USING (EXISTS (SELECT 1 FROM exposants e WHERE e.id = exposant_id AND e.visible_annuaire = true));

-- CRUD propriétaire
CREATE POLICY "produits_insert_own" ON produits_catalogue FOR INSERT
  WITH CHECK (exposant_id = auth.uid());
CREATE POLICY "produits_update_own" ON produits_catalogue FOR UPDATE
  USING (exposant_id = auth.uid()) WITH CHECK (exposant_id = auth.uid());
CREATE POLICY "produits_delete_own" ON produits_catalogue FOR DELETE
  USING (exposant_id = auth.uid());
```

### 4.3 `documents_professionnels`

```sql
-- Lecture propriétaire uniquement
CREATE POLICY "documents_select_own"
  ON documents_professionnels FOR SELECT
  USING (exposant_id = auth.uid());

-- Lecture via partage accepté
CREATE POLICY "documents_select_shared"
  ON documents_professionnels FOR SELECT
  USING (EXISTS (
    SELECT 1 FROM documents_partages dp
    WHERE dp.document_id = id AND dp.target_user_id = auth.uid()
      AND dp.status = 'accepte' AND (dp.expires_at IS NULL OR dp.expires_at > now())
  ));

-- Insert/Update propriétaire
CREATE POLICY "documents_insert_own" ON documents_professionnels FOR INSERT
  WITH CHECK (exposant_id = auth.uid());
CREATE POLICY "documents_update_own" ON documents_professionnels FOR UPDATE
  USING (exposant_id = auth.uid()) WITH CHECK (exposant_id = auth.uid());
```

---

## 5. Buckets Storage

| Bucket | Accès | Contenu | Taille max fichier |
|--------|-------|---------|-------------------|
| `logos` | Lecture publique, écriture propriétaire. | Logos exposants. | 2 Mo. |
| `banners` | Lecture publique, écriture propriétaire. | Bannières. | 5 Mo. |
| `produits` | Lecture publique, écriture propriétaire. | Visuels produits. | 5 Mo. |
| `documents-professionnels` | Lecture propriétaire + partage, écriture propriétaire. | Documents (PDF, images). **Privé.** | 10 Mo. |

**Nommage** :
- Logos : `{exposant_id}/logo.{ext}`
- Bannières : `{exposant_id}/banner.{ext}`
- Produits : `{exposant_id}/produits/{produit_id}/{visuel_id}.{ext}`
- Documents : `{exposant_id}/documents/{document_id}/v{version}.{ext}`

---

## 6. Stratégie de migration : Supabase → SQLite + KindMother

### 6.1 Objectif

Post-alpha : persistance COG-native. **SQLite + KindMother**, sans dépendance à Supabase (LOI-1, LOI-2, LOI-3).

### 6.2 Tables SQLite cible

Toutes les tables décrites en section 2 sont migrées. Types adaptés :
- UUID → TEXT
- TIMESTAMPTZ → TEXT (ISO 8601)
- JSONB → TEXT (JSON sérialisé)
- BOOLEAN → INTEGER (0/1)
- NUMERIC → REAL

### 6.3 Étapes de migration

| Étape | Description |
|-------|-------------|
| 1. Schéma SQLite | Définir toutes les tables JayXpose (+ JayFestival communes). |
| 2. Contrats KindMother | WriteIntent pour chaque entité (Exposant, Produit, Document, Partage, VitrinePages). |
| 3. Export Supabase | Export CSV/JSON de toutes les tables. |
| 4. Import SQLite | Import + vérification intégrité. |
| 5. Migration Storage | Fichiers vers stockage local ou KindMother Media. |
| 6. Couche abstraction | Services JayXpose configurables : Supabase (alpha) ou KindMother (post-alpha). |
| 7. Bascule | Production sur SQLite + KindMother. |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md) | Vision, intégration. |
| [JayXpose - Analyse des besoins](../JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins et champs. |
| [JayXpose - Catalogue Produits](../JayXpose%20-%20Catalogue%20Produits.md) | Spec catalogue. |
| [JayXpose - Documents Professionnels et Coffre-Fort](../JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md) | Spec coffre-fort. |
| [JayFestival - Reference Base de Donnees](../../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Schéma global JayFestival. |

---

**Document** : JayXpose — Base de données Supabase et migration SQLite
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document de référence (DB alpha, migration)
