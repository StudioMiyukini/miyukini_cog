# JayXpose â€” ConfidentialitÃ© et Partage Inter-Services

## Contexte

Ce document dÃ©finit la **politique de confidentialitÃ© des donnÃ©es exposant** et les **rÃ¨gles de partage inter-services** dans **JayXpose**. Il spÃ©cifie quelles donnÃ©es sont visibles par qui, selon quels niveaux, et comment le partage est gouvernÃ© dans l'Ã©cosystÃ¨me Jay (JayFestival, JayKonta, JayRDV, etc.).

**Principe fondateur** : L'exposant est souverain sur ses donnÃ©es. Le partage est un acte explicite et gouvernÃ©, jamais implicite.

**RÃ©fÃ©rences** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md), [Glossaire Miyukini](..//..//miyukini-webway-system//reference//_index.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Politique de confidentialitÃ© par champ, matrice de visibilitÃ© par rÃ´le/service, gouvernance du partage, Mandats de Permission.
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation technique (RLS, API) ; conformitÃ© RGPD dÃ©taillÃ©e (document juridique dÃ©diÃ©).

---

## 1. Principes fondamentaux

| Principe | Description |
|----------|-------------|
| **ConfidentialitÃ© par dÃ©faut** | Les champs sensibles ne sont jamais exposÃ©s par dÃ©faut. L'exposant active explicitement la visibilitÃ©. |
| **GranularitÃ© par champ** | Chaque champ du profil a un niveau de visibilitÃ© configurable individuellement. |
| **Partage gouvernÃ©** | Tout partage inter-services passe par un Mandat de Permission (StrongFather). |
| **TraÃ§abilitÃ©** | Chaque accÃ¨s Ã  des donnÃ©es confidentielles est auditÃ© (WorrySentinel). |
| **RÃ©vocabilitÃ©** | L'exposant peut rÃ©voquer un partage Ã  tout moment. L'accÃ¨s est coupÃ© immÃ©diatement. |
| **SÃ©paration donnÃ©es/prÃ©sentation** | Les donnÃ©es stockÃ©es et les donnÃ©es affichÃ©es sont distinctes. Le filtrage est appliquÃ© au rendu, pas au stockage. |

---

## 2. Niveaux de visibilitÃ©

| Niveau | Identifiant | Description | Qui peut voir |
|--------|-------------|-------------|---------------|
| **Public** | `public` | Visible par tous, sans authentification. | Tout le monde (annuaire, vitrine, JayFestival visiteur). |
| **AuthentifiÃ©** | `authentifie` | Visible uniquement par les utilisateurs connectÃ©s. | Utilisateurs authentifiÃ©s (exposants, visiteurs connectÃ©s, organisateurs). |
| **Organisateur** | `organisateur` | Visible uniquement par les organisateurs d'Ã©vÃ©nements JayFestival auxquels l'exposant participe. | Organisateurs avec un lien de participation (editions_exposants) actif. |
| **PrivÃ©** | `prive` | Visible uniquement par l'exposant lui-mÃªme. | Exclusivement le propriÃ©taire (exposant). |

---

## 3. Matrice de visibilitÃ© par champ

### 3.1 Profil exposant

| Champ | DÃ©faut | Configurable | Niveaux possibles |
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
| social_* (rÃ©seaux sociaux) | `public` | Non | `public` uniquement. |
| legal_form | `organisateur` | Oui | `organisateur`, `prive`. |
| siret | `organisateur` | Oui | `organisateur`, `prive`. |
| siren | `prive` | Oui | `organisateur`, `prive`. |
| code_ape | `prive` | Oui | `organisateur`, `prive`. |
| num_immatriculation | `prive` | Oui | `organisateur`, `prive`. |
| contact_facturation_* | `prive` | Non | `prive` uniquement. |
| contact_logistique_* | `organisateur` | Oui | `organisateur`, `prive`. |

### 3.2 Catalogue produits

| Champ | DÃ©faut | Configurable | Niveaux possibles |
|-------|--------|-------------|-------------------|
| Nom produit | `public` | Non | `public` (si catalogue visible). |
| Description produit | `public` | Non | `public`. |
| Prix | `public` | Oui | `public`, `authentifie`, `prive`. |
| Visuels | `public` | Non | `public`. |
| CatÃ©gorie | `public` | Non | `public`. |
| DisponibilitÃ© | `public` | Non | `public`. |

### 3.3 Documents professionnels

| Champ | DÃ©faut | Configurable | Niveaux possibles |
|-------|--------|-------------|-------------------|
| Existence du document (type) | `prive` | Non | `prive` (sauf partage explicite). |
| Contenu du document | `prive` | Non | `prive` (partage via Mandat uniquement). |
| Statut du document | `prive` | Non | `prive` + visible par le destinataire d'un partage acceptÃ©. |

---

## 4. Matrice de visibilitÃ© par rÃ´le / service

### 4.1 Par rÃ´le utilisateur

| DonnÃ©e | Visiteur anonyme | Visiteur authentifiÃ© | Exposant (self) | Organisateur (liÃ©) | Admin |
|--------|------------------|---------------------|-----------------|-------------------|-------|
| Profil public (nom, logo, secteur) | Oui | Oui | Oui | Oui | Oui |
| Description longue | Selon config | Selon config | Oui | Oui | Oui |
| Contact email | Selon config | Selon config | Oui | Selon config | Oui |
| Contact tÃ©lÃ©phone | Selon config | Selon config | Oui | Selon config | Oui |
| Adresse siÃ¨ge | Selon config | Selon config | Oui | Selon config | Oui |
| Infos juridiques (SIRETâ€¦) | Non | Non | Oui | Selon config | Oui |
| Contacts facturation | Non | Non | Oui | Non | Oui |
| Catalogue produits | Si vitrine publiÃ©e | Si vitrine publiÃ©e | Oui | Oui | Oui |
| Documents professionnels | Non | Non | Oui | Si partage acceptÃ© | Oui |

### 4.2 Par service Jay

| Service | DonnÃ©es accessibles | Condition |
|---------|---------------------|-----------|
| **JayFestival** | Profil public + contacts (selon config) + catalogue + documents (si partagÃ©s) + historique participations. | Mandat de Permission inter-services (Liaison JayFestival). |
| **JayKonta** | RIB (si partagÃ© depuis coffre-fort), infos facturation. | Mandat de Permission + partage explicite du RIB. |
| **JayRDV** | Profil public (pour le lien vitrine â†’ RDV). | Lecture publique (pas de donnÃ©es privÃ©es). |
| **JayKoa** | Profil public (pour l'agenda des participations). | Lecture publique + editions_exposants. |
| **JayFaim** | Profil public (si exposant restaurateur). | Phase 2. |

---

## 5. Gouvernance du partage

### 5.1 Partage profil â†’ JayFestival

Le partage du profil vers JayFestival est **automatique pour les donnÃ©es publiques** (annuaire, rÃ©pertoire). Pour les donnÃ©es Ã  confidentialitÃ© `organisateur` ou `prive`, le partage suit les rÃ¨gles de visibilitÃ© configurÃ©es par l'exposant.

**Flux** :
```
[Exposant configure sa confidentialitÃ©]
    â†’ Chaque champ a un niveau de visibilitÃ©
    â†’ JayFestival respecte ces niveaux au rendu
    â†’ Pas de Mandat spÃ©cifique nÃ©cessaire pour les donnÃ©es publiques/authentifiÃ©es
    â†’ Pour les donnÃ©es Â« organisateur Â» : vÃ©rification du lien participation (editions_exposants)
```

### 5.2 Partage documents â†’ JayFestival

Le partage de documents est **toujours explicite** (voir [Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)).

**Gouvernance** :

| Acteur | RÃ´le |
|--------|------|
| **StrongFather** | Autorise le Mandat de Permission pour le partage. |
| **WorrySentinel** | VÃ©rifie le niveau de sÃ©curitÃ© (Critical 3). |
| **Master Butler** | VÃ©rifie que l'organisateur a la capacitÃ© Â« demander des documents Â». |
| **KindMother** | Persiste l'Ã©tat du partage (demande, acceptation, rÃ©vocation). |
| **BondingBrother** | Traduit la demande JayFestival en demande JayXpose. |

### 5.3 Partage RIB â†’ JayKonta

| Ã‰tape | Action |
|-------|--------|
| 1 | JayKonta a besoin du RIB de l'exposant pour un paiement/remboursement. |
| 2 | Demande de partage via BondingBrother (JayKonta â†’ JayXpose). |
| 3 | L'exposant reÃ§oit la demande dans son coffre-fort. |
| 4 | L'exposant accepte le partage du document RIB. |
| 5 | JayKonta accÃ¨de au RIB en lecture seule (URL signÃ©e). |
| 6 | L'exposant peut rÃ©voquer Ã  tout moment. |

---

## 6. ModÃ¨le de donnÃ©es â€” Politique de confidentialitÃ©

### 6.1 Table `confidentialite_profil`

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID (PK) | Identifiant unique. |
| exposant_id | UUID (FK, UNIQUE) | Exposant propriÃ©taire. |
| contact_email_visibility | TEXT | `public` / `authentifie` / `organisateur` / `prive`. DÃ©faut : `authentifie`. |
| contact_phone_visibility | TEXT | DÃ©faut : `organisateur`. |
| adresse_siege_visibility | TEXT | DÃ©faut : `organisateur`. |
| adresse_correspondance_visibility | TEXT | DÃ©faut : `prive`. |
| description_long_visibility | TEXT | DÃ©faut : `public`. |
| legal_form_visibility | TEXT | DÃ©faut : `organisateur`. |
| siret_visibility | TEXT | DÃ©faut : `organisateur`. |
| siren_visibility | TEXT | DÃ©faut : `prive`. |
| code_ape_visibility | TEXT | DÃ©faut : `prive`. |
| num_immatriculation_visibility | TEXT | DÃ©faut : `prive`. |
| contact_logistique_visibility | TEXT | DÃ©faut : `organisateur`. |
| prix_catalogue_visibility | TEXT | `public` / `authentifie` / `prive`. DÃ©faut : `public`. |
| updated_at | TIMESTAMPTZ | DerniÃ¨re modification. |

**Alternative** : Stocker en JSON dans `exposants.confidentialite_config` plutÃ´t qu'une table sÃ©parÃ©e (Ã  dÃ©cider Ã  l'implÃ©mentation).

---

## 7. Application de la confidentialitÃ©

### 7.1 CÃ´tÃ© serveur (requÃªtes)

Pour chaque requÃªte de lecture de donnÃ©es exposant, le serveur :

1. Identifie le **rÃ´le de l'appelant** (anonyme, authentifiÃ©, organisateur liÃ©, propriÃ©taire, admin).
2. Charge la **politique de confidentialitÃ©** de l'exposant.
3. **Filtre les champs** selon le niveau de visibilitÃ© et le rÃ´le de l'appelant.
4. Retourne uniquement les champs autorisÃ©s.

### 7.2 RÃ¨gle de rÃ©solution

```
Si rÃ´le_appelant >= niveau_visibilitÃ©_champ â†’ champ visible
Sinon â†’ champ masquÃ© (non retournÃ©)
```

HiÃ©rarchie des rÃ´les :
```
admin > proprietaire > organisateur > authentifie > public
```

### 7.3 Cas particulier : donnÃ©es publiques dans l'annuaire

Les champs toujours publics (company_name, logo, secteur, description_short) sont retournÃ©s sans vÃ©rification pour optimiser les performances de l'annuaire.

---

## 8. Audit et conformitÃ©

| Action auditÃ©e | Acteur | Enregistrement |
|----------------|--------|----------------|
| Modification politique confidentialitÃ© | Exposant | `documents_audit` ou log dÃ©diÃ©. |
| Consultation donnÃ©es Â« organisateur Â» | Organisateur | Log d'accÃ¨s avec contexte (edition_id). |
| Partage document acceptÃ© | Exposant | `documents_audit`. |
| Consultation document partagÃ© | Organisateur | `documents_audit` (action = 'view'). |
| RÃ©vocation partage | Exposant | `documents_audit`. |

---

## 9. RÃ©fÃ©rences

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md)
- [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md)
- [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

**Document** : JayXpose â€” ConfidentialitÃ© et Partage Inter-Services
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : RÃ©fÃ©rence produit

