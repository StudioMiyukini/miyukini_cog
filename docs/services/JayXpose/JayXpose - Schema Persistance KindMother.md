# JayXpose - Schema Persistance KindMother

## 1. Objectif

Documenter le schema SQLite local JayXpose sous gouvernance KindMother.

## 2. Moteur

- DB locale: SQLite
- Driver: rusqlite
- Service: `JayXposeDb`
- Fichier par defaut: `jayxpose.db`

## 3. Tables metier

### 3.1 `exposants`

- Identite entreprise et metadonnees vitrine
- Cle primaire: `id`

Champs principaux:
- `company_name`
- `legal_form`
- `description_short`
- `description_long`
- `contact_email`
- `contact_phone`
- `vitrine_slug`
- `vitrine_status`
- `seo_title`
- `seo_description`
- `created_at`
- `updated_at`

### 3.2 `produits_catalogue`

- Produits de l'exposant
- FK: `exposant_id -> exposants.id`

Champs principaux:
- `name`
- `description`
- `price`
- `currency`
- `category_id`
- `availability`
- `is_featured`
- `sort_order`

### 3.3 `categories_produits`

- Categories catalogue
- FK: `exposant_id -> exposants.id`

### 3.4 `produits_visuels`

- Galerie images produit
- FK: `produit_id -> produits_catalogue.id`

### 3.5 `documents_professionnels`

- Coffre-fort documentaire
- FK: `exposant_id -> exposants.id`

### 3.6 `documents_versions`

- Historique versions documents
- FK: `document_id -> documents_professionnels.id`

### 3.7 `documents_partages`

- Partage gouverne des documents
- FK: `document_id`, `exposant_id`

### 3.8 `vitrine_pages`

- Pages publiees du site exposant
- FK: `exposant_id -> exposants.id`

### 3.9 `confidentialite_profil`

- Regles de visibilite par champ
- FK: `exposant_id -> exposants.id`

## 4. Tables M3/M7 ajoutees

### 4.1 `vitrine_blocs`

Role:
- granularite bloc pour page builder

Champs:
- `id`
- `page_id`
- `block_key`
- `block_type`
- `props_json`
- `position`
- `created_at`
- `updated_at`

Index:
- `idx_vitrine_blocs_page(page_id, position)`

### 4.2 `vitrine_templates`

Role:
- bibliotheque templates

Champs:
- `id`
- `name` (unique)
- `site_type`
- `schema_version`
- `content_json`
- `is_default`
- `created_at`
- `updated_at`

### 4.3 `sync_logs`

Role:
- audit fonctionnel des synchronisations

Champs:
- `id`
- `exposant_id`
- `sync_source`
- `sync_type`
- `status`
- `payload_json`
- `error_message`
- `created_at`

Index:
- `idx_sync_logs_exposant(exposant_id, created_at)`

### 4.4 `pos_stock_links`

Role:
- mapping produit JayXpose <-> SKU PoS

Champs:
- `id`
- `produit_id` (unique)
- `pos_sku`
- `stock_qty`
- `last_sync_at`
- `updated_at`

Index:
- `idx_pos_stock_sku(pos_sku)`

## 5. Seeds

Templates par defaut inseres si absents:
- `tpl-mini-site`
- `tpl-e-shop`
- `tpl-service-shop`

## 6. Regles de coherence

- `pos_stock_upsert` met aussi a jour `produits_catalogue.availability`
- `vitrine_blocks_replace` remplace tout l'ensemble de blocs de la page
- `vitrine_page_upsert_return_id` garantit l'identifiant page

## 7. API DB associees

- Profil: `exposant_*`
- Catalogue: `produit_*`, `categories_*`, `visuels_*`
- Documents: `document_*`, `partage_*`
- Vitrine: `vitrine_page_*`, `vitrine_blocks_*`, `vitrine_templates_*`
- Sync: `sync_log_*`, `pos_stock_*`

## 8. Strategie migration

- Migration in place par `init_schema` idempotent
- Ajout de nouvelles colonnes/tables via `CREATE TABLE IF NOT EXISTS`
- Versionnement logique dans la doc (pas encore table `schema_version`)

## 9. Backup / restore recommande

- Snapshot regulier du fichier `jayxpose.db`
- Export JSON periodique des tables critiques:
- `exposants`
- `produits_catalogue`
- `documents_professionnels`
- `vitrine_pages`
- `vitrine_blocs`

## 10. Securite

- Donnees sensibles (documents, identite) classees niveau eleve
- Acces pilote par mandats COG
- Journalisation obligatoire des flux inter-services
