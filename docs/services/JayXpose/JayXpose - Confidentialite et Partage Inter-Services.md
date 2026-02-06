# JayXpose — Confidentialité et Partage Inter-Services

## Contexte

Ce document définit la **politique de confidentialité des données exposant** et les **règles de partage inter-services** dans **JayXpose**. Il spécifie quelles données sont visibles par qui, selon quels niveaux, et comment le partage est gouverné dans l'écosystème Jay (JayFestival, JayKonta, JayRDV, etc.).

**Principe fondateur** : L'exposant est souverain sur ses données. Le partage est un acte explicite et gouverné, jamais implicite.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md), [Glossaire Miyukini](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md).

## Portée / Scope

- **Périmètre** : Politique de confidentialité par champ, matrice de visibilité par rôle/service, gouvernance du partage, Mandats de Permission.
- **Hors périmètre** : Implémentation technique (RLS, API) ; conformité RGPD détaillée (document juridique dédié).

---

## 1. Principes fondamentaux

| Principe | Description |
|----------|-------------|
| **Confidentialité par défaut** | Les champs sensibles ne sont jamais exposés par défaut. L'exposant active explicitement la visibilité. |
| **Granularité par champ** | Chaque champ du profil a un niveau de visibilité configurable individuellement. |
| **Partage gouverné** | Tout partage inter-services passe par un Mandat de Permission (StrongFather). |
| **Traçabilité** | Chaque accès à des données confidentielles est audité (WorrySentinel). |
| **Révocabilité** | L'exposant peut révoquer un partage à tout moment. L'accès est coupé immédiatement. |
| **Séparation données/présentation** | Les données stockées et les données affichées sont distinctes. Le filtrage est appliqué au rendu, pas au stockage. |

---

## 2. Niveaux de visibilité

| Niveau | Identifiant | Description | Qui peut voir |
|--------|-------------|-------------|---------------|
| **Public** | `public` | Visible par tous, sans authentification. | Tout le monde (annuaire, vitrine, JayFestival visiteur). |
| **Authentifié** | `authentifie` | Visible uniquement par les utilisateurs connectés. | Utilisateurs authentifiés (exposants, visiteurs connectés, organisateurs). |
| **Organisateur** | `organisateur` | Visible uniquement par les organisateurs d'événements JayFestival auxquels l'exposant participe. | Organisateurs avec un lien de participation (editions_exposants) actif. |
| **Privé** | `prive` | Visible uniquement par l'exposant lui-même. | Exclusivement le propriétaire (exposant). |

---

## 3. Matrice de visibilité par champ

### 3.1 Profil exposant

| Champ | Défaut | Configurable | Niveaux possibles |
|-------|--------|-------------|-------------------|
| company_name | `public` | Non (toujours public) | `public` uniquement. |
| logo_url | `public` | Non | `public` uniquement. |
| slogan | `public` | Non | `public` uniquement. |
| description_short | `public` | Non | `public` uniquement. |
| description_long | `public` | Oui | `public`, `authentifie`. |
| secteur | `public` | Non | `public` uniquement. |
| tags | `public` | Non | `public` uniquement. |
| banner_url | `public` | Non | `public` uniquement. |
| contact_email | `authentifie` | Oui | `public`, `authentifie`, `organisateur`, `prive`. |
| contact_phone | `organisateur` | Oui | `public`, `authentifie`, `organisateur`, `prive`. |
| adresse_siege | `organisateur` | Oui | `public`, `authentifie`, `organisateur`, `prive`. |
| adresse_correspondance | `prive` | Oui | `organisateur`, `prive`. |
| site_web | `public` | Non | `public` uniquement. |
| social_* (réseaux sociaux) | `public` | Non | `public` uniquement. |
| legal_form | `organisateur` | Oui | `organisateur`, `prive`. |
| siret | `organisateur` | Oui | `organisateur`, `prive`. |
| siren | `prive` | Oui | `organisateur`, `prive`. |
| code_ape | `prive` | Oui | `organisateur`, `prive`. |
| num_immatriculation | `prive` | Oui | `organisateur`, `prive`. |
| contact_facturation_* | `prive` | Non | `prive` uniquement. |
| contact_logistique_* | `organisateur` | Oui | `organisateur`, `prive`. |

### 3.2 Catalogue produits

| Champ | Défaut | Configurable | Niveaux possibles |
|-------|--------|-------------|-------------------|
| Nom produit | `public` | Non | `public` (si catalogue visible). |
| Description produit | `public` | Non | `public`. |
| Prix | `public` | Oui | `public`, `authentifie`, `prive`. |
| Visuels | `public` | Non | `public`. |
| Catégorie | `public` | Non | `public`. |
| Disponibilité | `public` | Non | `public`. |

### 3.3 Documents professionnels

| Champ | Défaut | Configurable | Niveaux possibles |
|-------|--------|-------------|-------------------|
| Existence du document (type) | `prive` | Non | `prive` (sauf partage explicite). |
| Contenu du document | `prive` | Non | `prive` (partage via Mandat uniquement). |
| Statut du document | `prive` | Non | `prive` + visible par le destinataire d'un partage accepté. |

---

## 4. Matrice de visibilité par rôle / service

### 4.1 Par rôle utilisateur

| Donnée | Visiteur anonyme | Visiteur authentifié | Exposant (self) | Organisateur (lié) | Admin |
|--------|------------------|---------------------|-----------------|-------------------|-------|
| Profil public (nom, logo, secteur) | Oui | Oui | Oui | Oui | Oui |
| Description longue | Selon config | Selon config | Oui | Oui | Oui |
| Contact email | Selon config | Selon config | Oui | Selon config | Oui |
| Contact téléphone | Selon config | Selon config | Oui | Selon config | Oui |
| Adresse siège | Selon config | Selon config | Oui | Selon config | Oui |
| Infos juridiques (SIRET…) | Non | Non | Oui | Selon config | Oui |
| Contacts facturation | Non | Non | Oui | Non | Oui |
| Catalogue produits | Si vitrine publiée | Si vitrine publiée | Oui | Oui | Oui |
| Documents professionnels | Non | Non | Oui | Si partage accepté | Oui |

### 4.2 Par service Jay

| Service | Données accessibles | Condition |
|---------|---------------------|-----------|
| **JayFestival** | Profil public + contacts (selon config) + catalogue + documents (si partagés) + historique participations. | Mandat de Permission inter-services (Liaison JayFestival). |
| **JayKonta** | RIB (si partagé depuis coffre-fort), infos facturation. | Mandat de Permission + partage explicite du RIB. |
| **JayRDV** | Profil public (pour le lien vitrine → RDV). | Lecture publique (pas de données privées). |
| **JayKoa** | Profil public (pour l'agenda des participations). | Lecture publique + editions_exposants. |
| **JayFaim** | Profil public (si exposant restaurateur). | Phase 2. |

---

## 5. Gouvernance du partage

### 5.1 Partage profil → JayFestival

Le partage du profil vers JayFestival est **automatique pour les données publiques** (annuaire, répertoire). Pour les données à confidentialité `organisateur` ou `prive`, le partage suit les règles de visibilité configurées par l'exposant.

**Flux** :
```
[Exposant configure sa confidentialité]
    → Chaque champ a un niveau de visibilité
    → JayFestival respecte ces niveaux au rendu
    → Pas de Mandat spécifique nécessaire pour les données publiques/authentifiées
    → Pour les données « organisateur » : vérification du lien participation (editions_exposants)
```

### 5.2 Partage documents → JayFestival

Le partage de documents est **toujours explicite** (voir [Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)).

**Gouvernance** :

| Acteur | Rôle |
|--------|------|
| **StrongFather** | Autorise le Mandat de Permission pour le partage. |
| **WorrySentinel** | Vérifie le niveau de sécurité (Critical 3). |
| **Master Butler** | Vérifie que l'organisateur a la capacité « demander des documents ». |
| **KindMother** | Persiste l'état du partage (demande, acceptation, révocation). |
| **BondingBrother** | Traduit la demande JayFestival en demande JayXpose. |

### 5.3 Partage RIB → JayKonta

| Étape | Action |
|-------|--------|
| 1 | JayKonta a besoin du RIB de l'exposant pour un paiement/remboursement. |
| 2 | Demande de partage via BondingBrother (JayKonta → JayXpose). |
| 3 | L'exposant reçoit la demande dans son coffre-fort. |
| 4 | L'exposant accepte le partage du document RIB. |
| 5 | JayKonta accède au RIB en lecture seule (URL signée). |
| 6 | L'exposant peut révoquer à tout moment. |

---

## 6. Modèle de données — Politique de confidentialité

### 6.1 Table `confidentialite_profil`

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID (PK) | Identifiant unique. |
| exposant_id | UUID (FK, UNIQUE) | Exposant propriétaire. |
| contact_email_visibility | TEXT | `public` / `authentifie` / `organisateur` / `prive`. Défaut : `authentifie`. |
| contact_phone_visibility | TEXT | Défaut : `organisateur`. |
| adresse_siege_visibility | TEXT | Défaut : `organisateur`. |
| adresse_correspondance_visibility | TEXT | Défaut : `prive`. |
| description_long_visibility | TEXT | Défaut : `public`. |
| legal_form_visibility | TEXT | Défaut : `organisateur`. |
| siret_visibility | TEXT | Défaut : `organisateur`. |
| siren_visibility | TEXT | Défaut : `prive`. |
| code_ape_visibility | TEXT | Défaut : `prive`. |
| num_immatriculation_visibility | TEXT | Défaut : `prive`. |
| contact_logistique_visibility | TEXT | Défaut : `organisateur`. |
| prix_catalogue_visibility | TEXT | `public` / `authentifie` / `prive`. Défaut : `public`. |
| updated_at | TIMESTAMPTZ | Dernière modification. |

**Alternative** : Stocker en JSON dans `exposants.confidentialite_config` plutôt qu'une table séparée (à décider à l'implémentation).

---

## 7. Application de la confidentialité

### 7.1 Côté serveur (requêtes)

Pour chaque requête de lecture de données exposant, le serveur :

1. Identifie le **rôle de l'appelant** (anonyme, authentifié, organisateur lié, propriétaire, admin).
2. Charge la **politique de confidentialité** de l'exposant.
3. **Filtre les champs** selon le niveau de visibilité et le rôle de l'appelant.
4. Retourne uniquement les champs autorisés.

### 7.2 Règle de résolution

```
Si rôle_appelant >= niveau_visibilité_champ → champ visible
Sinon → champ masqué (non retourné)
```

Hiérarchie des rôles :
```
admin > proprietaire > organisateur > authentifie > public
```

### 7.3 Cas particulier : données publiques dans l'annuaire

Les champs toujours publics (company_name, logo, secteur, description_short) sont retournés sans vérification pour optimiser les performances de l'annuaire.

---

## 8. Audit et conformité

| Action auditée | Acteur | Enregistrement |
|----------------|--------|----------------|
| Modification politique confidentialité | Exposant | `documents_audit` ou log dédié. |
| Consultation données « organisateur » | Organisateur | Log d'accès avec contexte (edition_id). |
| Partage document accepté | Exposant | `documents_audit`. |
| Consultation document partagé | Organisateur | `documents_audit` (action = 'view'). |
| Révocation partage | Exposant | `documents_audit`. |

---

## 9. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md)
- [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md)
- [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

**Document** : JayXpose — Confidentialité et Partage Inter-Services
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Référence produit
