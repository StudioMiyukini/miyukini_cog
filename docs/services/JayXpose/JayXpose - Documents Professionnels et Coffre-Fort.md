# JayXpose — Documents Professionnels et Coffre-Fort

## Contexte

Ce document spécifie le **coffre-fort documentaire** du service **JayXpose**. Il permet à chaque exposant de centraliser, gérer et partager ses **documents professionnels** (RIB, attestations d'assurance, KBIS, certificats, licences, etc.) de manière sécurisée. Les documents peuvent être partagés de manière gouvernée avec des organisateurs JayFestival dans le cadre de candidatures.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) (JXP-40 à JXP-49), [Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md).

## Portée / Scope

- **Périmètre** : Spécification complète du coffre-fort (types de documents, upload, versioning, statuts, alertes expiration, partage gouverné, sécurité, intégration JayFestival).
- **Hors périmètre** : Validation réglementaire automatique des documents (processus humain) ; archivage légal long terme ; signature électronique.

---

## 1. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Sécurité Critical (3)** | Les documents professionnels sont classés **Critical (3)** par WorrySentinel. Stockage chiffré, accès restreint, audit complet. |
| **Propriété exclusive** | Chaque document appartient exclusivement à l'exposant. Aucun tiers ne peut modifier un document. |
| **Partage gouverné** | Le partage est un acte explicite, traçable, révocable. Gouverné par un Mandat de Permission (StrongFather). |
| **Centralisation** | Un document uploadé une fois sert pour N candidatures / N événements. Pas de ré-upload. |
| **Auditabilité** | Toute opération (upload, modification, partage, consultation) est tracée : qui, quand, quoi. |
| **Souveraineté exposant** | L'exposant contrôle intégralement ses documents : upload, remplacement, suppression, partage, révocation. |

---

## 2. Types de documents

| Type (identifiant) | Libellé | Description | Expiration typique |
|---------------------|---------|-------------|---------------------|
| `rib` | RIB / IBAN | Relevé d'identité bancaire. | Non (changement = remplacement). |
| `assurance` | Attestation d'assurance | Assurance responsabilité civile professionnelle ou autre. | Annuelle. |
| `kbis` | Extrait KBIS | Extrait d'immatriculation au RCS (sociétés). | 3 mois (recommandé récent). |
| `immatriculation` | Certificat d'immatriculation | Inscription au Répertoire des Métiers (RM) ou autre registre. | Variable. |
| `licence` | Licence / Autorisation | Licence d'exploitation, autorisation de vente, licence spectacle, etc. | Variable (selon type). |
| `urssaf` | Attestation URSSAF | Attestation de vigilance ou de régularité URSSAF. | 6 mois. |
| `carte_pro` | Carte professionnelle | Carte d'artisan, de commerçant ambulant, etc. | Variable (1 à 10 ans). |
| `diplome` | Diplôme / Certification | Diplôme, certification professionnelle, label qualité. | Non (sauf certification à renouveler). |
| `sanitaire` | Certificat sanitaire | Certificat de conformité sanitaire (food, cosmétique). | Annuelle ou variable. |
| `autre` | Autre document | Type libre ; l'exposant renseigne un libellé personnalisé. | Selon document. |

---

## 3. Modèle de données

### 3.1 Table `documents_professionnels`

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| exposant_id | UUID (FK exposants) | Oui | Propriétaire du document. |
| type | TEXT | Oui | Type du document (voir section 2). |
| label | TEXT | Non | Libellé personnalisé (si type = 'autre'). |
| file_url | TEXT | Oui | URL du fichier (Storage sécurisé). |
| file_name | TEXT | Oui | Nom du fichier original. |
| file_size | INTEGER | Oui | Taille en octets. |
| mime_type | TEXT | Oui | Type MIME (application/pdf, image/png, image/jpeg). |
| status | TEXT | Oui | `en_attente` / `valide` / `expire` / `rejete`. Défaut : `en_attente`. |
| expires_at | TIMESTAMPTZ | Non | Date d'expiration (nullable). |
| version | INTEGER | Oui | Numéro de version (incrémental, départ 1). |
| notes | TEXT | Non | Notes internes de l'exposant. |
| rejection_reason | TEXT | Non | Motif de rejet (si status = 'rejete'). |
| uploaded_at | TIMESTAMPTZ | Oui (auto) | Date d'upload de cette version. |
| validated_at | TIMESTAMPTZ | Non | Date de validation (nullable). |
| validated_by | UUID | Non | UUID de l'utilisateur qui a validé (admin/organisateur). |
| created_at | TIMESTAMPTZ | Oui (auto) | Date de création du document (première version). |
| updated_at | TIMESTAMPTZ | Oui (auto) | Dernière modification. |

### 3.2 Table `documents_versions` (historique des versions)

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| document_id | UUID (FK documents_professionnels) | Oui | Document parent. |
| version | INTEGER | Oui | Numéro de version. |
| file_url | TEXT | Oui | URL du fichier de cette version. |
| file_name | TEXT | Oui | Nom du fichier. |
| file_size | INTEGER | Oui | Taille en octets. |
| uploaded_at | TIMESTAMPTZ | Oui (auto) | Date d'upload. |

### 3.3 Table `documents_partages`

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| document_id | UUID (FK documents_professionnels) | Oui | Document partagé. |
| exposant_id | UUID (FK exposants) | Oui | Exposant propriétaire. |
| target_user_id | UUID | Oui | Organisateur / service destinataire. |
| target_context_type | TEXT | Oui | Type de contexte (`candidature`, `edition`, `administratif`). |
| target_context_id | UUID | Non | ID du contexte (candidature_id, edition_id). |
| status | TEXT | Oui | `demande` / `accepte` / `refuse` / `revoque` / `expire`. |
| requested_at | TIMESTAMPTZ | Oui (auto) | Date de la demande. |
| responded_at | TIMESTAMPTZ | Non | Date de réponse (acceptation/refus). |
| revoked_at | TIMESTAMPTZ | Non | Date de révocation. |
| expires_at | TIMESTAMPTZ | Non | Expiration automatique du partage. |
| message | TEXT | Non | Message de l'organisateur avec la demande. |

### 3.4 Table `documents_audit` (journal d'audit)

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | Oui (auto) | Identifiant unique. |
| document_id | UUID (FK) | Oui | Document concerné. |
| actor_id | UUID | Oui | Utilisateur ayant effectué l'action. |
| action | TEXT | Oui | `upload` / `replace` / `validate` / `reject` / `delete` / `share_request` / `share_accept` / `share_refuse` / `share_revoke` / `view`. |
| details | JSON | Non | Détails supplémentaires (motif rejet, contexte partage…). |
| created_at | TIMESTAMPTZ | Oui (auto) | Horodatage. |

---

## 4. Cycle de vie d'un document

```
[Upload] → status = en_attente, version = 1
    ↓
[Validation par admin/organisateur] → status = valide
    ou
[Rejet] → status = rejete (+ motif)
    ↓
[Expiration date atteinte] → status = expire (automatique)
    ↓
[Remplacement (nouvelle version)] → version += 1, status = en_attente
    ancienne version → archivée dans documents_versions
```

### 4.1 Transitions de statut

| De | Vers | Déclencheur |
|----|------|-------------|
| — | `en_attente` | Upload initial ou remplacement. |
| `en_attente` | `valide` | Validation par admin ou organisateur autorisé. |
| `en_attente` | `rejete` | Rejet par admin ou organisateur (avec motif). |
| `valide` | `expire` | Date d'expiration atteinte (automatique, cron ou vérification à la lecture). |
| `expire` | `en_attente` | Remplacement par une nouvelle version. |
| `rejete` | `en_attente` | Remplacement par une nouvelle version corrigée. |
| Tout statut | (supprimé) | Suppression par l'exposant (si aucun partage actif). |

---

## 5. Mécanisme de partage gouverné

### 5.1 Flux de partage

```
[Organisateur JayFestival] 
    → Demande document (type: assurance) pour candidature X
        → INSERT documents_partages (status = 'demande')
        → Notification à l'exposant

[Exposant]
    → Reçoit la demande dans « Mes documents » (XP-E11)
    → Choisit le document correspondant dans son coffre-fort
    → Accepte le partage
        → UPDATE documents_partages SET status = 'accepte'
        → L'organisateur peut consulter le document (lecture seule)

    ou
    → Refuse le partage
        → UPDATE documents_partages SET status = 'refuse'

[Plus tard — Révocation]
    → L'exposant révoque le partage
        → UPDATE documents_partages SET status = 'revoque'
        → L'organisateur perd l'accès
```

### 5.2 Règles de partage

| Règle | Description |
|-------|-------------|
| **Unitaire** | Le partage se fait document par document. Pas de partage « tous les documents ». |
| **Explicite** | L'exposant doit accepter activement chaque demande. Pas de partage automatique. |
| **Révocable** | L'exposant peut révoquer un partage à tout moment. L'accès est coupé immédiatement. |
| **Temporaire** | Le partage a une durée de validité (défaut : durée de la candidature + 30 jours). |
| **Traçable** | Chaque action de partage est enregistrée dans `documents_audit`. |
| **Lecture seule** | L'organisateur consulte le document en lecture seule. Téléchargement soumis à politique. |
| **Gouverné** | Le partage est encadré par un Mandat de Permission émis par StrongFather. |

### 5.3 Gouvernance du partage

| Acteur | Rôle |
|--------|------|
| **StrongFather** | Émet le Mandat de Permission pour le partage. |
| **WorrySentinel** | Vérifie le niveau de sécurité (Critical 3). Bloque si état de confiance dégradé (T2+). |
| **KindMother** | Gère la persistance des documents et des partages. |
| **Master Butler** | Vérifie les capacités : l'organisateur a-t-il la permission de demander ce type de document ? |
| **BondingBrother** | Médie la demande entre JayFestival (demandeur) et JayXpose (propriétaire). |

---

## 6. Alertes d'expiration

### 6.1 Mécanisme

| Délai avant expiration | Action |
|------------------------|--------|
| 30 jours | Notification in-app : « Votre [type document] expire dans 30 jours. » |
| 15 jours | Notification in-app + email (si configuré). |
| 7 jours | Notification in-app + email. Badge alerte sur le document. |
| 0 jours (expiré) | Statut automatiquement passé à `expire`. Bandeau alerte dans le coffre-fort. |

### 6.2 Impact de l'expiration

| Contexte | Comportement |
|----------|-------------|
| Coffre-fort exposant | Document affiché avec badge « Expiré » en rouge. Invitation à remplacer. |
| Partage actif | Le document partagé est marqué « expiré » pour l'organisateur. Accès maintenu en lecture (historique) mais signalé comme périmé. |
| Candidature JayFestival | L'organisateur voit que le document est expiré. Peut demander un remplacement. |

---

## 7. Stockage et sécurité

### 7.1 Stockage (alpha Supabase)

| Paramètre | Valeur |
|-----------|--------|
| Bucket | `documents-professionnels` (privé). |
| Nommage | `{exposant_id}/{document_id}/v{version}.{ext}`. |
| Accès | RLS : lecture uniquement par le propriétaire ou via partage actif. |
| Formats | PDF, PNG, JPG. |
| Taille max par fichier | 10 Mo. |
| Quota par exposant | 50 Mo (alpha). |

### 7.2 Sécurité

| Mesure | Description |
|--------|-------------|
| Niveau WorrySentinel | **Critical (3)** — accès restreint, audit obligatoire, chiffrement. |
| Chiffrement at rest | Supabase Storage (chiffrement par défaut). Post-alpha : chiffrement KindMother. |
| Accès en écriture | Exclusivement le propriétaire (exposant). |
| Accès en lecture | Propriétaire + organisateurs avec partage accepté (via Mandat). |
| Téléchargement | URL signée avec expiration courte (1h). Pas de lien permanent. |
| Audit | Toute consultation enregistrée dans `documents_audit`. |
| Suppression | Soft delete avec conservation 90 jours (compliance). Purge après. |

---

## 8. RLS (alpha Supabase)

```sql
-- Lecture : propriétaire uniquement
CREATE POLICY "documents_select_own"
  ON documents_professionnels FOR SELECT
  USING (exposant_id = auth.uid());

-- Insert : propriétaire
CREATE POLICY "documents_insert_own"
  ON documents_professionnels FOR INSERT
  WITH CHECK (exposant_id = auth.uid());

-- Update : propriétaire
CREATE POLICY "documents_update_own"
  ON documents_professionnels FOR UPDATE
  USING (exposant_id = auth.uid())
  WITH CHECK (exposant_id = auth.uid());

-- Lecture partage : organisateur avec partage accepté
CREATE POLICY "documents_select_shared"
  ON documents_professionnels FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM documents_partages dp
      WHERE dp.document_id = documents_professionnels.id
        AND dp.target_user_id = auth.uid()
        AND dp.status = 'accepte'
        AND (dp.expires_at IS NULL OR dp.expires_at > now())
    )
  );
```

---

## 9. Intégration JayFestival

### 9.1 Demande de documents pour candidature

| Étape | Acteur | Action |
|-------|--------|--------|
| 1 | Organisateur | Configure les types de documents requis pour la candidature (ex. : assurance, KBIS). |
| 2 | Exposant | Remplit sa candidature. Voit la liste des documents demandés. |
| 3 | Exposant | Pour chaque document demandé : sélectionne un document existant dans son coffre-fort et accepte le partage. |
| 4 | Système | Création du partage (status = 'accepte', contexte = candidature). |
| 5 | Organisateur | Consulte les documents partagés dans la fiche exposant (ORG-E11). |
| 6 | Organisateur | Peut valider ou rejeter un document (avec motif). |

### 9.2 Données échangées

| Donnée | Direction | Description |
|--------|-----------|-------------|
| Types de documents requis | JayFestival → JayXpose | Liste des types demandés par l'organisateur. |
| Document partagé (accès) | JayXpose → JayFestival | URL signée pour consultation lecture seule. |
| Statut document | JayXpose → JayFestival | Statut actuel (valide, expiré, etc.). |
| Validation / rejet | JayFestival → JayXpose | Changement de statut + motif de rejet. |

---

## 10. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md)
- [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md)
- [JayXpose - Operateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md)

---

**Document** : JayXpose — Documents Professionnels et Coffre-Fort
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Référence produit
