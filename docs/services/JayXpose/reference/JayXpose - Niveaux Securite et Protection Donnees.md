# JayXpose — Niveaux de Sécurité et Protection des Données

## Contexte

Ce document définit la **classification de sécurité** des données JayXpose, la **politique de résidence** et les **règles de protection** selon la gouvernance WorrySentinel et les niveaux de sécurité Miyukini.

**Références** : [Miyukini Conceptual References — Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Niveaux de sécurité, WorrySentinel, États de confiance), [JayXpose - Confidentialite et Partage Inter-Services](../JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md).

## Portée / Scope

- **Périmètre** : Classification des données par niveau de sécurité, politique de résidence, règles d'accès, chiffrement, audit.
- **Hors périmètre** : Conformité RGPD détaillée (document juridique dédié) ; implémentation cryptographique.

---

## 1. Classification des données

### 1.1 Par catégorie fonctionnelle

| Catégorie | Exemples | Niveau | Justification |
|-----------|----------|--------|---------------|
| **Vitrine publique** | company_name, logo, secteur, description_short, slogan, site_web, social_*, catalogue produits (si visible). | **Public (0)** | Données destinées à être accessibles sans authentification. |
| **Profil standard** | description_long, tags, banner_url, seo_*, vitrine_slug, vitrine_status, catégories produits. | **Standard (1)** | Données non sensibles mais identifiantes. |
| **Données personnelles** | contact_email, contact_phone, adresse_siege, adresse_correspondance, contacts facturation/logistique. | **Sensitive (2)** | Données personnelles / commerciales relevant du RGPD. |
| **Données juridiques** | siret, siren, code_ape, num_immatriculation, legal_form. | **Sensitive (2)** à **Critical (3)** | Données d'identification légale. Sensitive si publiquement consultables (SIRET) ; Critical si liées à des documents internes. |
| **Documents professionnels** | RIB, attestation assurance, KBIS, certificats, licences, attestation URSSAF, carte pro, diplômes. | **Critical (3)** | Documents réglementaires et financiers. Risque élevé en cas de fuite. |
| **Données d'audit** | Logs d'accès, historique partages, actions sur documents. | **Sensitive (2)** | Traçabilité gouvernée, pas de données métier. |

### 1.2 Par table

| Table | Niveau global | Champs à niveau supérieur |
|-------|---------------|--------------------------|
| `exposants` | **Standard (1)** à **Sensitive (2)** | contact_*, adresse_* = Sensitive (2) ; siret, siren = Sensitive (2). |
| `produits_catalogue` | **Standard (1)** | price = Sensitive (2) si masqué par l'exposant. |
| `categories_produits` | **Public (0)** | — |
| `produits_visuels` | **Public (0)** | — |
| `documents_professionnels` | **Critical (3)** | Tous les champs. |
| `documents_versions` | **Critical (3)** | Tous les champs. |
| `documents_partages` | **Critical (3)** | Tous les champs (lié aux documents). |
| `documents_audit` | **Sensitive (2)** | — |
| `vitrine_pages` | **Standard (1)** | content peut contenir des données Standard. |
| `confidentialite_profil` | **Sensitive (2)** | Configuration de la politique de l'exposant. |
| `editions_exposants` | **Standard (1)** | — |

---

## 2. Politique de résidence

### 2.1 Alpha (Supabase)

| Données | Résidence | Justification |
|---------|-----------|---------------|
| Profil, catalogue, vitrine | Supabase (cloud, EU). | Exception pré-COG documentée. |
| Documents professionnels | Supabase Storage (cloud, EU). | Exception pré-COG. Bucket privé. |
| Audit | Supabase (table). | Exception pré-COG. |

### 2.2 Post-alpha (COG-native)

| Données | Résidence | Gouvernance |
|---------|-----------|-------------|
| Profil, catalogue, vitrine | SQLite local + KindMother. | LOI-1, LOI-2, LOI-3. |
| Documents professionnels | Stockage local chiffré + KindMother. | Critical (3) → chiffrement at rest obligatoire. |
| Audit | SQLite local. | Retention : 2 ans minimum. |

---

## 3. Règles d'accès

### 3.1 Matrice d'accès par niveau

| Niveau | Lecture | Écriture | Audit | Chiffrement |
|--------|---------|----------|-------|-------------|
| **Public (0)** | Tous | Propriétaire | Non requis | Non requis. |
| **Standard (1)** | Authentifiés + Public (si exposant visible) | Propriétaire | Recommandé | Non requis. |
| **Sensitive (2)** | Propriétaire + rôles autorisés (selon confidentialité) | Propriétaire | Obligatoire | Recommandé. |
| **Critical (3)** | Propriétaire + destinataires de partage (Mandat) | Propriétaire | Obligatoire | Obligatoire. |

### 3.2 Règles spécifiques documents

| Règle | Description |
|-------|-------------|
| Accès en écriture | Exclusivement le propriétaire (exposant). Jamais un organisateur ni un admin (sauf MiyukiniAdmin sous protocole spécial). |
| Accès en lecture (propriétaire) | Toujours autorisé sur ses propres documents. |
| Accès en lecture (partage) | Uniquement si `documents_partages.status = 'accepte'` ET `expires_at > now()`. |
| Téléchargement | URL signée avec expiration 1h. Pas de lien permanent. |
| Validation/rejet | Organisateur ou admin peut changer le statut mais pas accéder au contenu sans partage. |

---

## 4. Chiffrement

### 4.1 Alpha (Supabase)

| Élément | Chiffrement |
|---------|-------------|
| Données en transit | TLS 1.3 (Supabase par défaut). |
| Données at rest (DB) | Chiffrement Supabase natif (AES-256). |
| Données at rest (Storage) | Chiffrement Supabase Storage natif. |
| Clés | Gérées par Supabase. |

### 4.2 Post-alpha (COG-native)

| Élément | Chiffrement |
|---------|-------------|
| Documents professionnels (at rest) | AES-256 avec clé par exposant (gérée par KindMother). |
| Base SQLite | Chiffrement SQLCipher (optionnel, selon déploiement). |
| Communications inter-services | TLS / mTLS selon contexte. |

---

## 5. Audit et traçabilité

### 5.1 Actions auditées

| Action | Niveau minimum | Enregistrement |
|--------|---------------|----------------|
| Upload document | Critical (3) | `documents_audit` : actor, action='upload', timestamp. |
| Remplacement document | Critical (3) | `documents_audit` : action='replace', details (ancienne version). |
| Consultation document (propriétaire) | Critical (3) | Non audité (propriétaire = accès permanent). |
| Consultation document (partage) | Critical (3) | `documents_audit` : action='view', actor=target_user_id. |
| Validation document | Critical (3) | `documents_audit` : action='validate', actor=validated_by. |
| Rejet document | Critical (3) | `documents_audit` : action='reject', details (motif). |
| Demande de partage | Critical (3) | `documents_audit` : action='share_request'. |
| Acceptation partage | Critical (3) | `documents_audit` : action='share_accept'. |
| Révocation partage | Critical (3) | `documents_audit` : action='share_revoke'. |
| Modification politique confidentialité | Sensitive (2) | Log dédié ou `documents_audit`. |
| Consultation profil (organisateur) | Sensitive (2) | Log d'accès si champs niveau >= Sensitive. |

### 5.2 Rétention des logs

| Type de log | Rétention |
|-------------|-----------|
| Audit documents (Critical) | 2 ans minimum. |
| Audit accès (Sensitive) | 1 an minimum. |
| Logs généraux | 6 mois. |

---

## 6. États de confiance (WorrySentinel)

### 6.1 Impact sur JayXpose

| État | Impact JayXpose |
|------|-----------------|
| **T0 (Normal)** | Toutes les fonctionnalités disponibles. Partages autorisés. |
| **T1 (Instable)** | Surveillance accrue. Partages autorisés avec log renforcé. |
| **T2 (Dégradé)** | Partages de documents **bloqués** (nouveaux). Partages existants maintenus en lecture. |
| **T3 (Restreint)** | Coffre-fort en **lecture seule**. Pas d'upload, pas de partage. Vitrine publique maintenue. |
| **T4 (Bloqué)** | Tout accès aux documents **bloqué**. Seul le diagnostic est autorisé. Vitrine suspendue. |

---

## 7. Références

- [Miyukini Conceptual References — Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [JayXpose - Confidentialite et Partage Inter-Services](../JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](../JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md)

---

**Document** : JayXpose — Niveaux de Sécurité et Protection des Données
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Document de référence
