# JayXpose — Base de données Supabase et migration SQLite

## Contexte

**JayXpose** s’appuie en **version alpha** sur les mêmes tables **Supabase** que JayFestival (genèse Catakana) : **exposants**, **profiles**, **editions_exposants**. Ce document décrit (1) le **schéma des tables** utiles à JayXpose, (2) les **règles RLS** et (3) les **requêtes SQL** utilisées pour le parcours exposant (inscription, fiche entreprise, fiche publique, répertoire), puis (4) la **stratégie de migration** vers **SQLite + KindMother** pour la version COG-native.

**Références** : [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md), [JayXpose - Parcours utilisateur exposant](../JayXpose%20-%20Parcours%20utilisateur%20exposant.md), [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](../../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).

## Portée / Scope

- **Périmètre** : Tables Supabase (exposants, profiles, editions_exposants) ; RLS ; requêtes SQL alpha ; stratégie migration SQLite + KindMother.
- **Hors périmètre** : Tables purement JayFestival (editions, stands, budget_entries, invoices, etc.) — documentées dans la référence JayFestival ; implémentation détaillée KindMother.

---

## 1. Exception pré-COG : Supabase comme backend alpha

- **Alpha** : Backend = Supabase (PostgreSQL, Auth). JayXpose lit/écrit **exposants** et **profiles** ; JayFestival consomme les mêmes tables pour fiche exposant et répertoire.
- **Migration obligatoire** : Pour la version COG-native (post-alpha), la persistance JayXpose devra migrer vers **SQLite + KindMother** selon la stratégie décrite en section 4.

---

## 2. Tables Supabase (JayXpose)

### 2.1 Table `profiles`

Table utilisateurs (Supabase Auth + extension profil). Colonnes pertinentes pour JayXpose :

| Colonne | Type | Rôle |
|---------|------|------|
| id | UUID (PK, FK auth.users) | Identifiant utilisateur ; lien 1:1 avec exposants.id. |
| email | TEXT | Email (Auth). |
| username | TEXT | Nom d’affichage optionnel. |
| user_type | TEXT | `'exhibitor'`, `'admin'`, `'manager'`, `'volunteer'`, `'visitor'`. |
| avatar_url | TEXT | Avatar (optionnel). |
| created_at, updated_at | TIMESTAMPTZ | Horodatage. |

**Création exposant** : À l’inscription, une ligne `profiles` est créée (trigger ou service) avec `user_type = 'exhibitor'` ; une ligne **exposants** est créée avec `id = profiles.id`.

### 2.2 Table `exposants`

Table centrale **JayXpose** : profil vitrine / fiche entreprise. Une ligne par exposant (1:1 avec `profiles`).

| Colonne | Type | Contrainte | Rôle |
|---------|------|------------|------|
| id | UUID | PK, FK profiles(id) | Identifiant ; égal à auth.uid() pour l’exposant. |
| company_name | TEXT | | Nom entreprise / structure. |
| stand_name | TEXT | | Nom du stand (optionnel, par édition). |
| contact_email | TEXT | | Email de contact. |
| contact_phone | TEXT | | Téléphone. |
| adresse | TEXT | | Adresse postale. |
| logo_url | TEXT | | URL du logo (Supabase Storage). |
| site_web | TEXT | | Site web. |
| siret | TEXT | | SIRET (optionnel). |
| secteur | TEXT | | Secteur / catégorie d’activité. |
| category | TEXT | | Alias ou complément catégorie (selon schéma Catakana). |
| description | TEXT | | Description courte (vitrine). |
| visible_repertoire | BOOLEAN | DEFAULT true | Afficher dans le répertoire des exposants. |
| created_at | TIMESTAMPTZ | | Création. |
| updated_at | TIMESTAMPTZ | | Dernière mise à jour. |

**Optionnel (selon schéma Catakana)** : champs réseaux sociaux, champs « publiés » (liste de champs autorisés pour le répertoire) si gérés par colonnes dédiées.

### 2.3 Table `editions_exposants`

Table de liaison **exposant × édition** (participations / candidatures). Utilisée par JayFestival ; JayXpose n’écrit pas directement, mais les **lectures** (répertoire par édition, fiche exposant côté organisateur) s’appuient sur cette table.

| Colonne | Type | Rôle |
|---------|------|------|
| id | UUID | PK. |
| exposant_id | UUID | FK exposants(id). |
| edition_id | UUID | FK editions(id). |
| is_accepted | BOOLEAN | Candidature acceptée. |
| is_validated | BOOLEAN | Participation validée (affichage répertoire édition). |
| is_paid | BOOLEAN | Payé (optionnel). |
| assigned_stand | TEXT | Stand attribué (optionnel). |
| size_meters | NUMERIC | Surface (optionnel). |
| created_at, updated_at | TIMESTAMPTZ | Horodatage. |

---

## 3. Row Level Security (RLS) — exposants

### 3.1 Politiques recommandées (alpha)

| Table | Politique | Règle |
|-------|-----------|--------|
| **exposants** | SELECT public ou authentifié | Lecture répertoire : `visible_repertoire = true`. Lecture fiche détail : selon politique (public ou authentifié). |
| **exposants** | SELECT propriétaire | Exposant lit sa propre ligne : `id = auth.uid()`. |
| **exposants** | INSERT propriétaire | Un utilisateur peut créer une ligne exposants si `id = auth.uid()` (création à l’inscription). |
| **exposants** | UPDATE propriétaire | Seul le propriétaire : `id = auth.uid()`. |
| **exposants** | DELETE | Interdit ou réservé admin (selon politique). |
| **profiles** | (existant Catakana) | Lecture/écriture selon auth.uid() et rôle ; exposant ne modifie que son profil (ou champs autorisés). |
| **editions_exposants** | (existant JayFestival) | SELECT selon rôle (organisateur, exposant sur ses lignes) ; INSERT/UPDATE par organisateur ou exposant (candidature). |

### 3.2 Exemple de politiques RLS (exposants)

```sql
-- Lecture : tout le monde peut lire les exposants visibles en répertoire
CREATE POLICY "exposants_select_visible"
  ON exposants FOR SELECT
  USING (visible_repertoire = true);

-- Lecture : l'exposant peut lire sa propre ligne (tous champs)
CREATE POLICY "exposants_select_own"
  ON exposants FOR SELECT
  USING (id = auth.uid());

-- Insert : l'utilisateur peut créer sa ligne exposant (id = auth.uid())
CREATE POLICY "exposants_insert_own"
  ON exposants FOR INSERT
  WITH CHECK (id = auth.uid());

-- Update : seul le propriétaire
CREATE POLICY "exposants_update_own"
  ON exposants FOR UPDATE
  USING (id = auth.uid())
  WITH CHECK (id = auth.uid());
```

**Note** : En alpha, les politiques exactes dépendent du schéma Catakana (`.Catakana/docs/reference/README_RLS_PERMISSIONS.md`). Les exemples ci-dessus sont une cible cohérente avec le parcours JayXpose.

---

## 4. Requêtes SQL (alpha)

### 4.1 Inscription exposant (création profil JayXpose)

Après `signUp` Supabase Auth, création ou mise à jour **profiles** (trigger ou service), puis création **exposants** :

```sql
-- Insertion exposant (après création compte Auth + profiles)
INSERT INTO exposants (
  id,
  company_name,
  contact_email,
  contact_phone,
  adresse,
  site_web,
  siret,
  secteur,
  description,
  visible_repertoire,
  created_at,
  updated_at
) VALUES (
  auth.uid(),
  :company_name,
  :contact_email,
  :contact_phone,
  :adresse,
  :site_web,
  :siret,
  :secteur,
  :description,
  true,
  now(),
  now()
)
ON CONFLICT (id) DO UPDATE SET
  company_name = EXCLUDED.company_name,
  contact_email = EXCLUDED.contact_email,
  contact_phone = EXCLUDED.contact_phone,
  adresse = EXCLUDED.adresse,
  site_web = EXCLUDED.site_web,
  siret = EXCLUDED.siret,
  secteur = EXCLUDED.secteur,
  description = EXCLUDED.description,
  updated_at = now();
```

(En client Supabase/PostgREST, les paramètres sont passés en objet ; `auth.uid()` est fourni côté serveur via RLS ou fonction RPC.)

### 4.2 Mon compte — Fiche entreprise (lecture)

```sql
SELECT id, company_name, stand_name, contact_email, contact_phone, adresse,
       logo_url, site_web, siret, secteur, category, description,
       visible_repertoire, created_at, updated_at
FROM exposants
WHERE id = auth.uid();
```

### 4.3 Mon compte — Fiche entreprise (mise à jour)

```sql
UPDATE exposants
SET
  company_name = :company_name,
  stand_name = :stand_name,
  contact_email = :contact_email,
  contact_phone = :contact_phone,
  adresse = :adresse,
  logo_url = :logo_url,
  site_web = :site_web,
  siret = :siret,
  secteur = :secteur,
  category = :category,
  description = :description,
  updated_at = now()
WHERE id = auth.uid();
```

### 4.4 Fiche publique — Visibilité répertoire

```sql
UPDATE exposants
SET visible_repertoire = :visible_repertoire, updated_at = now()
WHERE id = auth.uid();
```

### 4.5 Répertoire (catalogue) — Liste des exposants visibles

```sql
SELECT id, company_name, secteur, category, logo_url, site_web, description
FROM exposants
WHERE visible_repertoire = true
ORDER BY company_name;
```

Avec pagination (alpha) :

```sql
SELECT id, company_name, secteur, category, logo_url, site_web, description
FROM exposants
WHERE visible_repertoire = true
ORDER BY company_name
LIMIT :limit OFFSET :offset;
```

Filtre par secteur (si applicable) :

```sql
SELECT id, company_name, secteur, category, logo_url, site_web, description
FROM exposants
WHERE visible_repertoire = true AND secteur = :secteur
ORDER BY company_name;
```

### 4.6 Fiche exposant (détail public)

```sql
SELECT id, company_name, stand_name, contact_email, contact_phone, adresse,
       logo_url, site_web, siret, secteur, category, description
FROM exposants
WHERE id = :exposant_id;
```

(La visibilité des champs contact peut être restreinte par politique ou par masquage côté application ; en alpha, on suppose que les champs « publics » sont ceux listés.)

### 4.7 Liste exposants par édition (JayFestival — consommateur JayXpose)

```sql
SELECT e.id, e.company_name, e.secteur, e.logo_url, e.site_web, e.description,
       ee.is_validated, ee.assigned_stand
FROM exposants e
JOIN editions_exposants ee ON ee.exposant_id = e.id
WHERE ee.edition_id = :edition_id AND ee.is_validated = true
ORDER BY e.company_name;
```

### 4.8 Fiche exposant côté organisateur (avec statut participation)

```sql
SELECT e.*, ee.is_accepted, ee.is_validated, ee.assigned_stand, ee.size_meters
FROM exposants e
LEFT JOIN editions_exposants ee ON ee.exposant_id = e.id AND ee.edition_id = :edition_id
WHERE e.id = :exposant_id;
```

---

## 5. Stockage fichiers (alpha) — Logo

- **Bucket Supabase Storage** : par ex. `logos` ou `exposants`.
- **Workflow** : upload fichier → récupération URL publique ou signée → mise à jour `exposants.logo_url`.
- **Contraintes** : format (PNG, JPG, WEBP), taille max (ex. 2 Mo), nommage (ex. `{exposant_id}/logo.{ext}`).

Détail des politiques Storage (RLS bucket) : lecture publique pour les logos du répertoire ; écriture limitée au propriétaire (exposant) ou admin.

---

## 6. Stratégie de migration : Supabase → SQLite + KindMother

### 6.1 Objectif

- **Post-alpha** : persistance **COG-native** pour JayXpose : **SQLite** + **KindMother** (WriteIntent, contrats de persistance), sans dépendance critique à Supabase (alignement LOI-1, LOI-2, LOI-3).
- **JayXpose** partage le même schéma logique que JayFestival pour **exposants** et **editions_exposants** ; la migration peut être **commune** avec JayFestival (même fichier SQLite ou même base gouvernée par KindMother).

### 6.2 Tables SQLite cible (JayXpose)

| Table | Rôle |
|-------|------|
| **profiles** | Équivalent Supabase ; id (TEXT UUID ou BLOB), email, username, user_type, avatar_url, created_at, updated_at. |
| **exposants** | Même colonnes que section 2.2 ; id (FK profiles). |
| **editions_exposants** | Même colonnes que section 2.3 ; pour répertoire par édition et fiche organisateur. |

Types : UUID → TEXT ; TIMESTAMPTZ → TEXT (ISO 8601) ou INTEGER (epoch). Contraintes : PK, FK, index sur `exposants.visible_repertoire`, `editions_exposants(edition_id, exposant_id)`.

### 6.3 Étapes (alignées JayFestival)

| Étape | Description |
|-------|-------------|
| **1. Schéma SQLite** | Définir `profiles`, `exposants`, `editions_exposants` (et tables JayFestival nécessaires) ; contraintes, index. |
| **2. Contrats KindMother** | Contrats de persistance pour entités Exposant, Profile (lectures, WriteIntent pour création/mise à jour exposant). |
| **3. Export Supabase** | Export des tables `exposants`, `profiles`, `editions_exposants` (et liées) vers CSV/JSON ou SQL. |
| **4. Import SQLite** | Import dans le schéma SQLite ; vérification intégrité. |
| **5. Couche d’abstraction** | Services JayXpose (et JayFestival) appellent une couche « persistance » configurable : Supabase (alpha) ou KindMother/SQLite (post-alpha). |
| **6. Bascule** | Passage en production sur SQLite + KindMother ; Supabase en lecture seule ou décommissionné pour JayXpose/JayFestival. |

### 6.4 Contrats KindMother (orientation)

- **Exposant** : Create (inscription), Read (fiche entreprise, fiche publique, répertoire), Update (fiche entreprise, visible_repertoire). Pas de suppression métier en alpha.
- **Profile** : Read/Update (géré par Miyauth/Miyuprofile ; JayXpose consomme la donnée).
- Les **requêtes SQL** décrites en section 4 deviennent des **opérations de lecture/écriture** via KindMother (sans exposer SQL direct aux Opérateurs).

---

## 7. Références

| Document | Rôle |
|----------|------|
| [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md) | Vision, intégration JayFestival. |
| [JayXpose - Parcours utilisateur exposant](../JayXpose%20-%20Parcours%20utilisateur%20exposant.md) | Parcours inscription, fiche entreprise, fiche publique. |
| [JayXpose - Analyse des besoins](../JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins et champs métier. |
| [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](../../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Schéma global, RLS, stratégie migration commune. |
| Catakana `docs/reference/database_schema.md`, `README_RLS_PERMISSIONS.md` | Schéma et RLS détaillés. |

---

**Document** : JayXpose — Base de données Supabase et migration SQLite  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence (DB alpha, migration)
