# Odoo PLM — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **PLM (Product Lifecycle Management)** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents PLM
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **EcoOperator** | Gestion des ordres de modification (ECO) | Opérateur de Service |
| **EcoTypeOperator** | Gestion des types d'ECO et des stages | Opérateur de Service |
| **BomRevisionOperator** | Gestion des révisions BoM (versioning) | Opérateur de Service |
| **EcoApprovalOperator** | Gestion des approbations et vérifications | Opérateur de Service |
| **EcoDocumentOperator** | Gestion des documents de conception (ECO/BoM) | Opérateur de Service |
| **PlmUI** | Interface utilisateur PLM (Overview, ECO, révisions) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : PlmService

**Définition :**
> **PlmService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion du cycle de vie produit et des nomenclatures (ECO, révisions, approbations, versioning).**

**Composition :**
- EcoOperator (niveau sécurité 2)
- EcoTypeOperator (niveau sécurité 2)
- BomRevisionOperator (niveau sécurité 2)
- EcoApprovalOperator (niveau sécurité 2)
- EcoDocumentOperator (niveau sécurité 2)
- PlmUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 EcoOperator

**Rôle :** Gestion des ordres de modification (ECO) — création, révision, application des changements, rebase.

**Capacités :**
- Création et modification d'ECO (Type, Produit, BoM, Apply on, Responsible, Effective, Tags)
- Démarrage de révision (Start Revision) : création copie BoM, attribution version
- Déplacement entre stages (workflow ECO Type)
- Application des changements (Apply Changes) : bascule révision → BoM de production
- Rebase (Apply Rebase) : résolution conflits ECO concurrents
- Comparaison révision vs production (BoM Changes, Operation Changes)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création ECO, Apply Changes, changement de stage
- **KindMother** : Persistance des ECO et des révisions (WriteIntent) ; versioning BoM
- **Master Butler** : Permissions création/modification ECO, Apply Changes
- **WorrySentinel** : Niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Cycle de vie ECO, compatibilité versions, rebase

**Contrat d'équipe :**
- Consomme : EcoTypeOperator (types, stages), BomRevisionOperator (révisions), EcoApprovalOperator (approbations), EcoDocumentOperator (documents), Opérateur Manufacturing/BoM (lecture BoM production)
- Expose : `eco.create`, `eco.update`, `eco.start_revision`, `eco.apply_changes`, `eco.apply_rebase`, `eco.move_stage`

**Mandat de Permission requis :**
- Création ECO : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Start Revision : Mandat avec BomRevisionOperator + KindMother (WriteIntent)
- Apply Changes : Mandat avec StrongFather (décision) + EcoApprovalOperator (approbations validées) + KindMother (WriteIntent sur BoM)
- Apply Rebase : Mandat avec Ever Buddy (compatibilité) + KindMother (WriteIntent sur révision)

### 2.2 EcoTypeOperator

**Rôle :** Gestion des types d'ECO et des stages (workflow).

**Capacités :**
- Création et modification des types d'ECO (nom, usage : nouveau produit, mise à jour gamme, conformité)
- Configuration des stages par type (Nouveau, En cours, Vérification, Clôture)
- Définition des stages « vérification » (approbation requise)
- Configuration alias email (création ECO par email)
- Filtrage des ECO par type (visibilité par responsabilité)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Validation des types et stages
- **KindMother** : Persistance des types et stages (WriteIntent)
- **Master Butler** : Permissions de configuration
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consommé par : EcoOperator, PlmUI
- Expose : `eco_type.create`, `eco_type.update`, `eco_type.stages`, `eco_type.approval_stages`

**Mandat de Permission requis :**
- Modification types/stages : Mandat avec StrongFather (décision) + KindMother (WriteIntent)

### 2.3 BomRevisionOperator

**Rôle :** Gestion des révisions BoM (copie, modification, comparaison).

**Capacités :**
- Création d'une révision BoM à partir de la BoM de production (Start Revision)
- Modification des composants et opérations sur la révision (sans affecter la production)
- Calcul des différences révision vs production (BoM Changes, Operation Changes)
- Traçabilité version (numéro de version, effective date)
- Historique des versions (liste ECO Done)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création révision, application
- **KindMother** : Persistance des révisions et versioning (WriteIntent) ; bascule production
- **Master Butler** : Permissions lecture/écriture révisions
- **Ever Buddy** : Compatibilité versions, rebase

**Contrat d'équipe :**
- Consommé par : EcoOperator
- Consomme : Opérateur Manufacturing/BoM (lecture BoM production, écriture après Apply Changes)
- Expose : `bom_revision.create`, `bom_revision.update`, `bom_revision.compare`, `bom_revision.history`

**Mandat de Permission requis :**
- Création révision : Mandat avec KindMother (WriteIntent) + EcoOperator
- Comparaison / historique : Mandat lecture (Master Butler)

### 2.4 EcoApprovalOperator

**Rôle :** Gestion des approbations (stages vérification, déblocage Apply Changes).

**Capacités :**
- Enregistrement des approbations par stage (vérification)
- Vérification que toutes les approbations requises sont obtenues
- Déblocage du bouton Apply Changes lorsque les approbations sont valides
- Historique des approbations et commentaires (Chatter / MiyuNotify)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation (approbation = acte de gouvernance)
- **TAMR** : Intervention humaine (approbateur) requise
- **Master Butler** : Permissions d'approbation par rôle/stage
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consommé par : EcoOperator (avant Apply Changes)
- Consomme : MiyuNotify (notifications, commentaires)
- Expose : `approval.request`, `approval.grant`, `approval.revoke`, `approval.status`

**Mandat de Permission requis :**
- Approbation : Mandat avec StrongFather (décision) + TAMR (intervention humaine) + Master Butler (permission approbateur)

### 2.5 EcoDocumentOperator

**Rôle :** Gestion des documents de conception (CAD, PDF) attachés à l'ECO et à la BoM.

**Capacités :**
- Ajout, modification, suppression de fichiers dans l'ECO (smart button Documents)
- Archivage des fichiers retirés (accessibles dans l'ECO, pas sur la BoM après Apply Changes)
- Synchronisation des documents ECO → BoM de production après Apply Changes
- Traçabilité et audit (qui a ajouté/retiré, quand)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des liens document ↔ ECO / BoM (WriteIntent)
- **Master Butler** : Permissions attachement / suppression
- **WorrySentinel** : Niveau sécurité, pas d'exfiltration

**Contrat d'équipe :**
- Consommé par : EcoOperator
- Consomme : MiyuMedia (stockage fichiers) ou équivalent
- Expose : `document.attach`, `document.remove`, `document.sync_to_bom`

**Mandat de Permission requis :**
- Attachement / suppression dans ECO : Mandat avec KindMother (WriteIntent) + EcoOperator
- Sync vers BoM : Mandat avec KindMother (WriteIntent) au moment Apply Changes

### 2.6 PlmUI

**Rôle :** Interface utilisateur PLM — Overview, listes ECO, formulaire ECO, révision BoM, comparaisons.

**Capacités :**
- Vue d'ensemble (Overview) par type d'ECO avec indicateurs et accès aux listes
- Formulaires ECO (champs, stages, smart buttons Revisions, Documents)
- Formulaires révision BoM (Components, Operations, indication Archived)
- Onglets BoM Changes, Operation Changes, Previous Eco Bom Changes
- Gestion des documents (upload, remove, download)
- Historique des versions (liste ECO, filtre Done)
- Actions Start Revision, Apply Changes, Apply Rebase

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs
- **Master Butler** : Permissions d'affichage et d'action selon rôle
- **WorrySentinel** : Niveau sécurité affichage

**Contrat d'équipe :**
- Consomme : EcoOperator, EcoTypeOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator
- Expose : écrans Overview, ECO, Révision, Documents, Historique

**Mandat de Permission requis :**
- Affichage et actions selon Mandat utilisateur (Master Butler + flux autorisés vers PlmService)

---

## 3. Contrats d'Équipe et Mandats

### 3.1 Contrat d'équipe PlmService

**Opérateurs membres :** EcoOperator, EcoTypeOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator, PlmUI

**Flux autorisés :**
- PlmUI → EcoOperator, EcoTypeOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator (intentions utilisateur)
- EcoOperator → EcoTypeOperator (lecture types/stages), BomRevisionOperator (création révision, comparaison), EcoApprovalOperator (demande approbation), EcoDocumentOperator (documents)
- EcoOperator → KindMother (WriteIntent ECO, Apply Changes), StrongFather (décisions), Ever Buddy (rebase)
- EcoApprovalOperator → StrongFather (validation approbation), MiyuNotify (notifications)
- EcoDocumentOperator → MiyuMedia, KindMother (liens documents)

**Types d'échanges :** Intentions (create, update, start_revision, apply_changes, apply_rebase, approval.grant), données ECO, révision, approbations, documents

**Conditions préalables :** Mandat de Permission valide pour l'utilisateur sur PlmService ; approbations requises satisfaites pour Apply Changes

**Niveau de validation :** StrongFather pour Apply Changes et approbations ; KindMother pour toute écriture

### 3.2 Mandats de Permission typiques

| Action | Mandat requis |
|--------|----------------|
| Créer / modifier ECO | KindMother (WriteIntent) + StrongFather (décision) |
| Start Revision | BomRevisionOperator + KindMother (WriteIntent) |
| Apply Changes | StrongFather (décision) + EcoApprovalOperator (approbations OK) + KindMother (WriteIntent BoM) |
| Apply Rebase | Ever Buddy (compatibilité) + KindMother (WriteIntent révision) |
| Approuver un ECO | StrongFather (décision) + TAMR (intervention) + Master Butler (rôle approbateur) |
| Gérer documents ECO | KindMother (WriteIntent) + EcoDocumentOperator |
| Configurer types/stages | StrongFather + KindMother (WriteIntent) |

---

## 4. Niveaux de Sécurité

- **PlmUI** : 1 (Standard) — affichage et actions guidées
- **EcoOperator, EcoTypeOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator** : 2 (Sensitive) — données produit, nomenclatures, approbations
- **Sécurité hétérogène** : PlmService combine niveau 1 (interface) et niveau 2 (métier) ; flux entre niveaux explicites et validés par WorrySentinel

---

## 5. Intégration avec les Cores

- **StrongFather** : Toute décision (création ECO, Apply Changes, approbation, changement stage)
- **KindMother** : Toute persistance (ECO, révisions, versioning BoM, documents)
- **Master Butler** : Permissions (création, modification, approbation, configuration)
- **WorrySentinel** : Niveau sécurité 1–2, isolation cross-équipe
- **Ever Buddy** : Cycle de vie ECO, versions, rebase
- **TAMR** : Approbation = intervention humaine (point d'intervention)
- **BondingBrother** : Traduction intentions (PlmUI → Opérateurs)

---

## 6. Correspondance Odoo → Miyukini

| Odoo | Miyukini |
|------|----------|
| ECO | EcoOperator + KindMother (WriteIntent) |
| ECO Type / Stages | EcoTypeOperator |
| Révision BoM | BomRevisionOperator + KindMother (versioning) |
| Apply Changes | StrongFather (décision) + KindMother (WriteIntent) + EcoApprovalOperator |
| Approvals | EcoApprovalOperator + TAMR + StrongFather |
| Documents ECO / BoM | EcoDocumentOperator + MiyuMedia |
| Rebase | Ever Buddy + BomRevisionOperator + KindMother |
| Overview / UI | PlmUI |

---

**Document rédigé selon la méthodologie d'analyse Odoo et le glossaire Miyukini.**
