# JayXpose â€” Niveaux de SÃ©curitÃ© et Protection des DonnÃ©es

## Contexte

Ce document dÃ©finit la **classification de sÃ©curitÃ©** des donnÃ©es JayXpose, la **politique de rÃ©sidence** et les **rÃ¨gles de protection** selon la gouvernance WorrySentinel et les niveaux de sÃ©curitÃ© Miyukini.

**RÃ©fÃ©rences** : [Miyukini Conceptual References â€” Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) (Niveaux de sÃ©curitÃ©, WorrySentinel, Ã‰tats de confiance), [JayXpose - Confidentialite et Partage Inter-Services](../JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Classification des donnÃ©es par niveau de sÃ©curitÃ©, politique de rÃ©sidence, rÃ¨gles d'accÃ¨s, chiffrement, audit.
- **Hors pÃ©rimÃ¨tre** : ConformitÃ© RGPD dÃ©taillÃ©e (document juridique dÃ©diÃ©) ; implÃ©mentation cryptographique.

---

## 1. Classification des donnÃ©es

### 1.1 Par catÃ©gorie fonctionnelle

| CatÃ©gorie | Exemples | Niveau | Justification |
|-----------|----------|--------|---------------|
| **Vitrine publique** | company_name, logo, secteur, description_short, slogan, site_web, social_*, catalogue produits (si visible). | **Public (0)** | DonnÃ©es destinÃ©es Ã  Ãªtre accessibles sans authentification. |
| **Profil standard** | description_long, tags, banner_url, seo_*, vitrine_slug, vitrine_status, catÃ©gories produits. | **Standard (1)** | DonnÃ©es non sensibles mais identifiantes. |
| **DonnÃ©es personnelles** | contact_email, contact_phone, adresse_siege, adresse_correspondance, contacts facturation/logistique. | **Sensitive (2)** | DonnÃ©es personnelles / commerciales relevant du RGPD. |
| **DonnÃ©es juridiques** | siret, siren, code_ape, num_immatriculation, legal_form. | **Sensitive (2)** Ã  **Critical (3)** | DonnÃ©es d'identification lÃ©gale. Sensitive si publiquement consultables (SIRET) ; Critical si liÃ©es Ã  des documents internes. |
| **Documents professionnels** | RIB, attestation assurance, KBIS, certificats, licences, attestation URSSAF, carte pro, diplÃ´mes. | **Critical (3)** | Documents rÃ©glementaires et financiers. Risque Ã©levÃ© en cas de fuite. |
| **DonnÃ©es d'audit** | Logs d'accÃ¨s, historique partages, actions sur documents. | **Sensitive (2)** | TraÃ§abilitÃ© gouvernÃ©e, pas de donnÃ©es mÃ©tier. |

### 1.2 Par table

| Table | Niveau global | Champs Ã  niveau supÃ©rieur |
|-------|---------------|--------------------------|
| `exposants` | **Standard (1)** Ã  **Sensitive (2)** | contact_*, adresse_* = Sensitive (2) ; siret, siren = Sensitive (2). |
| `produits_catalogue` | **Standard (1)** | price = Sensitive (2) si masquÃ© par l'exposant. |
| `categories_produits` | **Public (0)** | â€” |
| `produits_visuels` | **Public (0)** | â€” |
| `documents_professionnels` | **Critical (3)** | Tous les champs. |
| `documents_versions` | **Critical (3)** | Tous les champs. |
| `documents_partages` | **Critical (3)** | Tous les champs (liÃ© aux documents). |
| `documents_audit` | **Sensitive (2)** | â€” |
| `vitrine_pages` | **Standard (1)** | content peut contenir des donnÃ©es Standard. |
| `confidentialite_profil` | **Sensitive (2)** | Configuration de la politique de l'exposant. |
| `editions_exposants` | **Standard (1)** | â€” |

---

## 2. Politique de rÃ©sidence

### 2.1 Alpha (Supabase)

| DonnÃ©es | RÃ©sidence | Justification |
|---------|-----------|---------------|
| Profil, catalogue, vitrine | Supabase (cloud, EU). | Exception prÃ©-COG documentÃ©e. |
| Documents professionnels | Supabase Storage (cloud, EU). | Exception prÃ©-COG. Bucket privÃ©. |
| Audit | Supabase (table). | Exception prÃ©-COG. |

### 2.2 Post-alpha (COG-native)

| DonnÃ©es | RÃ©sidence | Gouvernance |
|---------|-----------|-------------|
| Profil, catalogue, vitrine | SQLite local + KindMother. | LOI-1, LOI-2, LOI-3. |
| Documents professionnels | Stockage local chiffrÃ© + KindMother. | Critical (3) â†’ chiffrement at rest obligatoire. |
| Audit | SQLite local. | Retention : 2 ans minimum. |

---

## 3. RÃ¨gles d'accÃ¨s

### 3.1 Matrice d'accÃ¨s par niveau

| Niveau | Lecture | Ã‰criture | Audit | Chiffrement |
|--------|---------|----------|-------|-------------|
| **Public (0)** | Tous | PropriÃ©taire | Non requis | Non requis. |
| **Standard (1)** | AuthentifiÃ©s + Public (si exposant visible) | PropriÃ©taire | RecommandÃ© | Non requis. |
| **Sensitive (2)** | PropriÃ©taire + rÃ´les autorisÃ©s (selon confidentialitÃ©) | PropriÃ©taire | Obligatoire | RecommandÃ©. |
| **Critical (3)** | PropriÃ©taire + destinataires de partage (Mandat) | PropriÃ©taire | Obligatoire | Obligatoire. |

### 3.2 RÃ¨gles spÃ©cifiques documents

| RÃ¨gle | Description |
|-------|-------------|
| AccÃ¨s en Ã©criture | Exclusivement le propriÃ©taire (exposant). Jamais un organisateur ni un admin (sauf MiyukiniAdmin sous protocole spÃ©cial). |
| AccÃ¨s en lecture (propriÃ©taire) | Toujours autorisÃ© sur ses propres documents. |
| AccÃ¨s en lecture (partage) | Uniquement si `documents_partages.status = 'accepte'` ET `expires_at > now()`. |
| TÃ©lÃ©chargement | URL signÃ©e avec expiration 1h. Pas de lien permanent. |
| Validation/rejet | Organisateur ou admin peut changer le statut mais pas accÃ©der au contenu sans partage. |

---

## 4. Chiffrement

### 4.1 Alpha (Supabase)

| Ã‰lÃ©ment | Chiffrement |
|---------|-------------|
| DonnÃ©es en transit | TLS 1.3 (Supabase par dÃ©faut). |
| DonnÃ©es at rest (DB) | Chiffrement Supabase natif (AES-256). |
| DonnÃ©es at rest (Storage) | Chiffrement Supabase Storage natif. |
| ClÃ©s | GÃ©rÃ©es par Supabase. |

### 4.2 Post-alpha (COG-native)

| Ã‰lÃ©ment | Chiffrement |
|---------|-------------|
| Documents professionnels (at rest) | AES-256 avec clÃ© par exposant (gÃ©rÃ©e par KindMother). |
| Base SQLite | Chiffrement SQLCipher (optionnel, selon dÃ©ploiement). |
| Communications inter-services | TLS / mTLS selon contexte. |

---

## 5. Audit et traÃ§abilitÃ©

### 5.1 Actions auditÃ©es

| Action | Niveau minimum | Enregistrement |
|--------|---------------|----------------|
| Upload document | Critical (3) | `documents_audit` : actor, action='upload', timestamp. |
| Remplacement document | Critical (3) | `documents_audit` : action='replace', details (ancienne version). |
| Consultation document (propriÃ©taire) | Critical (3) | Non auditÃ© (propriÃ©taire = accÃ¨s permanent). |
| Consultation document (partage) | Critical (3) | `documents_audit` : action='view', actor=target_user_id. |
| Validation document | Critical (3) | `documents_audit` : action='validate', actor=validated_by. |
| Rejet document | Critical (3) | `documents_audit` : action='reject', details (motif). |
| Demande de partage | Critical (3) | `documents_audit` : action='share_request'. |
| Acceptation partage | Critical (3) | `documents_audit` : action='share_accept'. |
| RÃ©vocation partage | Critical (3) | `documents_audit` : action='share_revoke'. |
| Modification politique confidentialitÃ© | Sensitive (2) | Log dÃ©diÃ© ou `documents_audit`. |
| Consultation profil (organisateur) | Sensitive (2) | Log d'accÃ¨s si champs niveau >= Sensitive. |

### 5.2 RÃ©tention des logs

| Type de log | RÃ©tention |
|-------------|-----------|
| Audit documents (Critical) | 2 ans minimum. |
| Audit accÃ¨s (Sensitive) | 1 an minimum. |
| Logs gÃ©nÃ©raux | 6 mois. |

---

## 6. Ã‰tats de confiance (WorrySentinel)

### 6.1 Impact sur JayXpose

| Ã‰tat | Impact JayXpose |
|------|-----------------|
| **T0 (Normal)** | Toutes les fonctionnalitÃ©s disponibles. Partages autorisÃ©s. |
| **T1 (Instable)** | Surveillance accrue. Partages autorisÃ©s avec log renforcÃ©. |
| **T2 (DÃ©gradÃ©)** | Partages de documents **bloquÃ©s** (nouveaux). Partages existants maintenus en lecture. |
| **T3 (Restreint)** | Coffre-fort en **lecture seule**. Pas d'upload, pas de partage. Vitrine publique maintenue. |
| **T4 (BloquÃ©)** | Tout accÃ¨s aux documents **bloquÃ©**. Seul le diagnostic est autorisÃ©. Vitrine suspendue. |

---

## 7. RÃ©fÃ©rences

- [Miyukini Conceptual References â€” Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)
- [JayXpose - Confidentialite et Partage Inter-Services](../JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](../JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Document Fondateur](../JayXpose%20-%20Document%20Fondateur.md)

---

**Document** : JayXpose â€” Niveaux de SÃ©curitÃ© et Protection des DonnÃ©es
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Document de rÃ©fÃ©rence

