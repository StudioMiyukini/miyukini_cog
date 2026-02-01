# Odoo Documents — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Documents** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Documents
- Contrats d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d’ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **DocumentsFolderOperator** | Gestion des dossiers (hiérarchie, sections, partage, alias email) | Opérateur de Service |
| **DocumentsFileOperator** | Gestion des fichiers (upload, lien, tableur, versions, lock, split/merge PDF) | Opérateur de Service |
| **DocumentsTagOperator** | Gestion des tags (création, affectation aux fichiers) | Opérateur de Service |
| **DocumentsShareOperator** | Gestion des droits et du partage (Viewer/Editor, expiration, portail) | Opérateur de Service |
| **DocumentsRequestOperator** | Demandes de documents (placeholder, rappels, activités) | Opérateur de Service |
| **DocumentsUI** | Interface utilisateur Documents (arbre, liste, grille, prévisualisation, partage) | Opérateur d’Interface |

### 1.2 Équipe d’Opérateurs : DocumentsService

**Définition :**
> **DocumentsService est une Équipe d’Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion documentaire (DMS).**

**Composition :**
- DocumentsFolderOperator (niveau sécurité 2)
- DocumentsFileOperator (niveau sécurité 2)
- DocumentsTagOperator (niveau sécurité 1)
- DocumentsShareOperator (niveau sécurité 2)
- DocumentsRequestOperator (niveau sécurité 2)
- DocumentsUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 DocumentsFolderOperator

**Rôle :** Gestion des dossiers (création, hiérarchie, sections, alias email, actions sur sélection, automatisations).

**Capacités :**
- Création/modification/suppression de dossiers et sous-dossiers
- Gestion des sections (All, Company, My Drive, Shared with me, Recent, Trash)
- Configuration alias email (dossier, domaine, activité, tags)
- Actions sur sélection (actions serveur pour les fichiers du dossier)
- Raccourcis (création de shortcut vers un sous-dossier)
- Favoris (étoile) — utilisateur
- Téléchargement dossier en .zip
- Déplacement vers Trash (délai de suppression configurable)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification/suppression dossier, partage
- **KindMother** : Persistance des dossiers (WriteIntent)
- **Master Butler** : Permissions création/modification/partage dossier
- **WorrySentinel** : Niveau sécurité, isolation par dossier/entreprise
- **Ever Buddy** : Cycle de vie (Trash, délai de suppression)

**Contrat d’équipe :**
- Consomme : DocumentsFileOperator (fichiers du dossier), DocumentsShareOperator (partage), DocumentsTagOperator (tags alias), MiyuNotify (activités alias)
- Expose : `folder.create`, `folder.update`, `folder.delete`, `folder.share`, `folder.download_zip`

**Mandat de Permission requis :**
- Création dossier : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Modification dossier : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Partage dossier : Mandat DocumentsShareOperator + StrongFather (décision)
- Alias email : Mandat avec MiyuNotify (activités) + KindMother (WriteIntent)

### 2.2 DocumentsFileOperator

**Rôle :** Gestion des fichiers (upload, lien, tableur, versions, lock, split/merge PDF, raccourcis).

**Capacités :**
- Création fichier (upload, lien URL, tableur, placeholder demande)
- Modification métadonnées (nom, dossier, propriétaire, contact, tags)
- Versions (historique, téléchargement, upload nouvelle version)
- Verrouillage (Lock) / déverrouillage
- Raccourcis (création pointeur sans duplication)
- Split PDF (découpage en plusieurs documents)
- Merge PDF (fusion de plusieurs PDF en un)
- Déplacement vers Trash
- Lien de partage (Copy Links) avec droits associés

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification/suppression fichier, partage
- **KindMother** : Persistance des fichiers et blobs (WriteIntent)
- **Master Butler** : Permissions lecture/écriture/téléchargement
- **WorrySentinel** : Niveau sécurité, contrôle accès par fichier
- **Ever Buddy** : Cycle de vie (versions, Trash)

**Contrat d’équipe :**
- Consommé par : DocumentsFolderOperator (contenu dossier)
- Consomme : DocumentsFolderOperator (dossier), DocumentsTagOperator (tags), DocumentsShareOperator (partage), MiyuMedia (stockage blob si délégué)
- Expose : `file.create`, `file.update`, `file.delete`, `file.upload_version`, `file.lock`, `file.split_pdf`, `file.merge_pdf`, `file.create_shortcut`

**Mandat de Permission requis :**
- Upload / création fichier : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Modification fichier : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Partager fichier : Mandat DocumentsShareOperator + StrongFather (décision)
- Split/Merge PDF : Mandat KindMother (WriteIntent) + StrongFather (décision)

### 2.3 DocumentsTagOperator

**Rôle :** Gestion des tags (création, configuration, affectation aux fichiers).

**Capacités :**
- Création/modification de tags (nom, couleur, tooltip)
- Affectation de tags aux fichiers (et optionnellement dossiers)
- Tags automatiques via alias email (configuration par dossier)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision création/modification tag
- **KindMother** : Persistance des tags et des affectations (WriteIntent)
- **Master Butler** : Permissions configuration tags (souvent réservé admin)

**Contrat d’équipe :**
- Consommé par : DocumentsFolderOperator (tags alias), DocumentsFileOperator (tags fichier)
- Expose : `tag.create`, `tag.update`, `tag.assign_to_file`, `tag.assign_to_folder`

**Mandat de Permission requis :**
- Création tag : Mandat KindMother (WriteIntent) + StrongFather (décision)
- Affectation tag : Mandat KindMother (WriteIntent) + DocumentsFileOperator ou DocumentsFolderOperator

### 2.4 DocumentsShareOperator

**Rôle :** Gestion des droits d’accès et du partage (Viewer/Editor, expiration, lien public, portail).

**Capacités :**
- Attribution Viewer/Editor à des utilisateurs ou contacts
- Expiration des droits (date de fin)
- Accès général : Internal users / Anyone with the link (Discoverable ou Must have the link)
- Restriction : None (accès interdit)
- Exposition portail : carte Documents pour utilisateurs portail selon droits

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de partage, révocation
- **KindMother** : Persistance des règles d’accès (WriteIntent)
- **Master Butler** : Vérification des permissions à chaque accès (lecture/écriture)
- **WorrySentinel** : Niveau sécurité, contrôle « Anyone with the link » et portail

**Contrat d’équipe :**
- Consommé par : DocumentsFolderOperator, DocumentsFileOperator
- Consomme : MiyuNotify (optionnel, notifications de partage), MiyuPortal (accès portail)
- Expose : `share.folder`, `share.file`, `share.revoke`, `share.check_access`

**Mandat de Permission requis :**
- Partager dossier/fichier : Mandat StrongFather (décision) + KindMother (WriteIntent)
- Accès portail : Mandat Façade Publique Gouvernée / Mandat Public d’Accès (utilisateurs externes)

### 2.5 DocumentsRequestOperator

**Rôle :** Demandes de documents (création placeholder, assignation, rappels, suivi activités).

**Capacités :**
- Création demande (Document Name, Request To, Due Date In, Folder, Tags, Message)
- Création placeholder dans le dossier et activité « document demandé »
- Suivi dans vue Activités (colonne Requested Document)
- Upload fichier sur placeholder (remplacement)
- Rappel unitaire (email) et rappel groupé (Document Request: Reminder)
- Annulation / édition activité

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création demande, rappel
- **KindMother** : Persistance des demandes et placeholders (WriteIntent)
- **Master Butler** : Permissions créer demande, upload sur placeholder
- **MiyuNotify** : Activités et envoi des rappels

**Contrat d’équipe :**
- Consomme : DocumentsFolderOperator (dossier), DocumentsFileOperator (placeholder puis fichier), MiyuNotify (activités, rappels)
- Expose : `request.create`, `request.upload`, `request.remind`, `request.cancel`

**Mandat de Permission requis :**
- Création demande : Mandat KindMother (WriteIntent) + StrongFather (décision) + MiyuNotify (activité)
- Upload sur placeholder : Mandat DocumentsFileOperator + KindMother (WriteIntent)
- Rappel : Mandat MiyuNotify (envoi email)

### 2.6 DocumentsUI

**Rôle :** Interface utilisateur Documents (arbre, liste, grille, prévisualisation, panneau détails, partage).

**Capacités :**
- Affichage arborescence (sections, dossiers, favoris)
- Liste et grille de fichiers/dossiers
- Prévisualisation fichier (PDF, images, vidéos)
- Panneau Info & Tags (détails, chatter)
- Dialogue Share (utilisateurs, Viewer/Editor, expiration)
- Formulaire Request (demande de document)
- Configuration (Paramètres, Tags)
- Recherche et filtres (dont Starred)
- Actions : New (Upload, Link, Spreadsheet, Folder, Request), actions sur sélection, Split PDF, Merge PDFs

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs Documents
- **Master Butler** : Permissions d’affichage (selon droits partage)
- **WorrySentinel** : Niveau sécurité affichage (pas d’exposition de données au-delà des droits)

**Contrat d’équipe :**
- Consommé par : Utilisateur (interne ou portail)
- Consomme : DocumentsFolderOperator, DocumentsFileOperator, DocumentsTagOperator, DocumentsShareOperator, DocumentsRequestOperator
- N’expose pas d’API métier : uniquement interface ; toutes les actions passent par BondingBrother vers les Opérateurs ci-dessus

**Mandat de Permission requis :**
- Affichage liste/grille/arbre : Mandat Master Butler (lecture) selon partage
- Toute action (upload, share, request, etc.) : Mandat correspondant à l’Opérateur concerné (voir ci-dessus)

---

## 3. Contrat d’Équipe DocumentsService

**Membres :** DocumentsFolderOperator, DocumentsFileOperator, DocumentsTagOperator, DocumentsShareOperator, DocumentsRequestOperator, DocumentsUI

**Flux autorisés :**
- DocumentsUI → BondingBrother → DocumentsFolderOperator, DocumentsFileOperator, DocumentsTagOperator, DocumentsShareOperator, DocumentsRequestOperator
- DocumentsFolderOperator ↔ DocumentsFileOperator (contenu dossier)
- DocumentsFolderOperator ↔ DocumentsShareOperator (partage dossier)
- DocumentsFileOperator ↔ DocumentsTagOperator (tags fichier)
- DocumentsFileOperator ↔ DocumentsShareOperator (partage fichier)
- DocumentsRequestOperator → DocumentsFolderOperator, DocumentsFileOperator, MiyuNotify

**Types d’échanges :** WriteIntent (KindMother), décisions (StrongFather), vérifications permissions (Master Butler), activités/rappels (MiyuNotify)

**Conditions préalables :** Environnement COG actif ; Mandats de Permission valides pour chaque action

**Niveau de validation :** StrongFather pour toute création/modification/suppression/partage ; KindMother pour toute persistance

---

## 4. Intégrations externes à l’équipe

- **MiyuInvoice** (ou équivalent facturation) : numérisation IA (PDF → facture) ; Mandat + niveau sécurité élevé
- **MiyuProject** (ou équivalent projet) : centralisation pièces jointes tâches ; liens tâche ↔ document
- **MiyuNotify** : activités, chatter, rappels demandes de documents
- **MiyuPortal** : exposition Façade Publique Gouvernée pour partage externe (carte Documents)
- **Opérateur Sign** (si existant) : envoi document à signer
- **MiyuMedia** (si délégué) : stockage blobs

---

## 5. Synthèse

| Opérateur | Sécurité | Décision | Persistance | Permissions |
|-----------|----------|----------|-------------|--------------|
| DocumentsFolderOperator | 2 | StrongFather | KindMother | Master Butler |
| DocumentsFileOperator | 2 | StrongFather | KindMother | Master Butler |
| DocumentsTagOperator | 1 | StrongFather | KindMother | Master Butler |
| DocumentsShareOperator | 2 | StrongFather | KindMother | Master Butler |
| DocumentsRequestOperator | 2 | StrongFather | KindMother + MiyuNotify | Master Butler |
| DocumentsUI | 1 | — | — | Master Butler (lecture) |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
