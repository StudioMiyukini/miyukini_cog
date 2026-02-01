# Odoo Documents — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Documents** (Gestion documentaire / DMS) d'Odoo (version 18/19), à partir de la documentation officielle et des fonctionnalités publiées. Il identifie les modèles de données conceptuels, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Productivity / Documents

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles conceptuels (Dossiers, Documents, Tags, Droits d'accès)
- Règles métier et contraintes
- Workflows (suppression différée, centralisation fichiers, demandes de documents)
- Organisation (sections : All, Company, My Drive, Shared with me, Recent, Trash)
- Partage et droits (Viewer, Editor, expiration, portail)
- Intégrations (Project, Sign, Invoicing, PLM, IA)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles Conceptuels

### 1.1 Dossier (Folder)

**Rôle :** Conteneur hiérarchique pour organiser les fichiers. Les dossiers appartiennent à des **sections** (Company, My Drive, etc.).

**Concepts clés :**
- **Nom** : Libellé du dossier
- **Section** : All, Company, My Drive, Shared with me, Recent, Trash
- **Parent** : Dossier parent pour les sous-dossiers
- **Propriétaire (Owner)** : Créateur par défaut, droits complets
- **Contact** : Personne avec droits Viewer (ex. fournisseur)
- **Droits d'accès** : Définis au niveau dossier (partage utilisateurs internes, lien public)
- **Alias email** : Adresse email pour envoyer des fichiers directement dans le dossier
- **Actions sur sélection** : Actions serveur disponibles pour les fichiers du dossier
- **Automatisations** : Règles d'actions automatiques (Studio)

**Règles métier :**
- Les dossiers sont créés dans Company ou My Drive
- Sous-dossiers créés à partir d'un dossier parent
- Certains dossiers/sous-dossiers sont créés automatiquement via la centralisation des fichiers (paramétrage par app)
- Suppression : déplacement vers Trash, puis suppression définitive après délai (défaut 30 jours)
- Raccourcis : un sous-dossier peut avoir un raccourci (même dossier si édition, sinon My Drive)

**Workflow :**
- Création → utilisation → (optionnel) partage → (optionnel) déplacement Trash → suppression définitive après délai

### 1.2 Document / Fichier (Document / File)

**Rôle :** Unité de contenu stockée — fichier uploadé, lien URL ou tableur (Spreadsheet).

**Types de documents :**
- **Fichier uploadé** : PDF, images, vidéos, etc. (limite 64 MB sur Odoo Online)
- **Lien** : URL (ex. vidéo, Google Docs)
- **Tableur** : Création via app Spreadsheet
- **Demande de document** : Placeholder créé lors d'une « Request », remplacé par le fichier une fois uploadé

**Concepts clés :**
- **Nom** : Libellé du fichier
- **Dossier** : Dossier de classement
- **Propriétaire (Owner)** : Créateur par défaut, droits complets
- **Contact** : Personne avec droits Viewer
- **Tags** : Étiquettes pour filtrage et recherche
- **Verrouillage (Lock)** : Empêche toute modification
- **Versions** : Historique des versions (consultation, téléchargement, upload nouvelle version)
- **Raccourci** : Pointeur vers un fichier sans dupliquer (même dossier si édition, sinon My Drive)
- **Lien de partage** : URL avec droits associés (Viewer/Editor, expiration, discoverable ou « Must have the link »)

**Règles métier :**
- Un document appartient à un dossier (sauf vue Recent / Shared with me qui agrègent)
- Droits héritables ou spécifiques au fichier
- Split PDF : découpage d'un PDF en plusieurs documents (pages ou groupes de pages)
- Merge PDF : fusion de plusieurs PDF sélectionnés en un seul (les originaux sont remplacés par la version fusionnée)
- Demande de document : création d'un placeholder (Document Name, Request To, Due Date, Folder, Tags, Message) ; une fois le fichier fourni, le placeholder est remplacé
- Fichiers centralisés : si la centralisation est activée pour une app, la suppression d'un enregistrement dans cette app envoie les pièces jointes à la corbeille Documents

**Workflow typique :**
- Création (upload / lien / tableur / demande) → édition / partage / versions → (optionnel) verrouillage → (optionnel) déplacement Trash → suppression définitive après délai

### 1.3 Tags

**Rôle :** Catégorisation des fichiers (et éventuellement dossiers) pour filtrage et recherche.

**Concepts clés :**
- **Nom** : Libellé du tag
- **Couleur** : Affichage visuel
- **Tooltip** : Info-bulle optionnelle
- **Configuration** : Documents ‣ Configuration ‣ Tags
- **Alias email** : Les tags peuvent être appliqués automatiquement aux fichiers reçus via un alias

**Règles métier :**
- Les tags sont globaux (configuration centrale)
- Un fichier peut avoir plusieurs tags
- Les tags des dossiers (par centralisation) s'appliquent aux nouveaux fichiers uniquement, pas aux existants

### 1.4 Droits d'accès et partage

**Rôle :** Contrôler qui peut voir, modifier ou gérer dossiers et fichiers.

**Niveaux :**
- **Viewer** : Consultation, téléchargement
- **Editor** : Viewer + modification, renommage, déplacement, suppression (selon contexte)
- **Propriétaire (Owner)** : Droits complets (défaut : créateur)
- **Contact** : Viewer uniquement (ex. fournisseur dans la fiche)

**Portée :**
- **Utilisateurs internes** : Sélection par utilisateur ou groupe
- **Anyone with the link** : Accès par lien avec option « Discoverable » (accessible en parcourant) ou « Must have the link to access »
- **Restriction** : None pour interdire l'accès

**Règles métier :**
- Partage possible uniquement si l'utilisateur a des droits d'édition sur le dossier/fichier
- Expiration : date de fin de validité du droit (optionnel)
- Portail : les utilisateurs portail accèdent aux dossiers/fichiers partagés via le portail (carte Documents)
- L'URL d'un dossier/fichier reflète les droits ; partager un dossier renvoie vers un portail dédié (fichiers avec accès restreint exclus)

### 1.5 Sections (Vues logiques)

**Rôle :** Organisation de l'arborescence côté utilisateur sans être un modèle de données séparé — ce sont des vues/filtres sur dossiers et fichiers.

| Section | Description |
|--------|-------------|
| **All** | Tous les dossiers et fichiers auxquels l'utilisateur a accès |
| **Company** | Dossiers et fichiers partagés au niveau entreprise (droits par dossier/fichier) |
| **My Drive** | Espace personnel : dossiers et fichiers dont l'utilisateur est propriétaire ou qu'il a uploadés |
| **Shared with me** | Fichiers partagés avec l'utilisateur mais n'appartenant à aucun dossier auquel il a accès |
| **Recent** | Fichiers récemment modifiés (lecture ou édition) |
| **Trash** | Éléments supprimés, conservés pendant le délai de suppression (défaut 30 jours) |

---

## 2. Règles Métier Transverses

### 2.1 Suppression (Deletion delay)

- Les éléments envoyés à la corbeille restent **30 jours** par défaut avant suppression définitive
- Paramétrage : Documents ‣ Configuration ‣ Paramètres ‣ **Deletion delay (days)**
- Comportement : déplacement vers Trash (réversible pendant le délai) puis purge automatique

### 2.2 Centralisation des fichiers (File centralization)

- Paramétrage : Documents ‣ Configuration ‣ Paramètres
- Pour une app donnée (ex. Human Resources), on choisit un **dossier** cible et des **tags** à appliquer
- Les **nouveaux** fichiers associés à cette app sont automatiquement rangés dans ce dossier avec ces tags
- Les fichiers existants ne sont pas modifiés
- Sous-dossiers possibles (ex. Comptabilité : configuration par journal)
- Si la centralisation est activée et qu'un enregistrement de l'app est supprimé, ses pièces jointes sont déplacées vers la corbeille Documents

### 2.3 Alias email

- Un dossier peut avoir un **alias email** (et domaine)
- Les fichiers envoyés à cette adresse sont enregistrés dans le dossier
- Options : type d'activité et assignation pour créer une activité à la réception ; tags à appliquer automatiquement
- Dépend de la configuration des serveurs de messagerie entrants

### 2.4 Demandes de documents (Document Request)

- Création : New ‣ Request
- Champs : Document Name, Request To (personne), Due Date In, Folder, Tags, Message
- Une activité « document demandé » est créée ; suivi possible dans la vue Activités (colonne Requested Document)
- Actions : upload du fichier sur le placeholder, éditer l'activité, annuler, envoyer rappel (email)
- Rappel groupé : depuis la vue Activités, action « Document Request: Reminder »

### 2.5 PDF : Split et Merge

- **Split PDF** : ouvrir le PDF, action « Split PDF », définir les coupures entre pages (icône ciseaux), confirmer → plusieurs documents créés
- **Merge PDF** : en vue liste, sélectionner les PDF, Action ‣ Merge PDFs, ordre et ajout éventuel de fichiers, confirmer → un seul document (remplace les originaux)
- Raccourci clavier : Shift+S pour ajouter/supprimer toutes les coupures entre pages ; suppression de page possible après sélection

### 2.6 Numérisation / IA (File digitization with AI)

- Réservé aux fichiers dans le **dossier Finance** (ou équivalent centralisé comptabilité)
- Actions : Create Vendor Bill, Create Customer Invoice, Create Customer Credit Note puis **Send for Digitization**
- Utilisation du moteur IA Odoo pour extraction des données de factures (voir documentation Comptabilité / Factures fournisseurs)

---

## 3. Intégrations Métier (Logique)

### 3.1 Project

- Pièces jointes des tâches/projets peuvent être centralisées dans Documents (dossier + tags)
- Liens directs depuis les tâches vers les documents

### 3.2 Sign

- Intégration avec Odoo Sign pour envoi de documents à signer

### 3.3 Invoicing / Accounting

- Centralisation des factures (dossiers par journal possible)
- Numérisation IA des factures fournisseurs depuis Documents (création facture fournisseur / avoir client)

### 3.4 PLM

- Documents liés au cycle de vie produit (centralisation possible)

### 3.5 Spreadsheet

- Création de tableurs depuis Documents (New ‣ Spreadsheet) ; stockage et partage comme tout document

---

## 4. Synthèse pour Miyukini

**Entités à modéliser :**
- **Folder** : hiérarchie, section, propriétaire, contact, droits, alias email, actions sur sélection, automatisations
- **Document** : nom, type (fichier/lien/tableur/demande), dossier, propriétaire, contact, tags, lock, versions, raccourcis, lien de partage
- **Tag** : nom, couleur, tooltip
- **Access rights** : Viewer / Editor, utilisateurs, lien public, expiration, discoverable
- **Document Request** : placeholder, demandeur, destinataire, échéance, dossier, message, activité

**Règles à implémenter :**
- Délai de suppression configurable (Trash)
- Centralisation par app (dossier + tags) pour nouveaux fichiers
- Partage et portail (Viewer/Editor, expiration, lien)
- Split / Merge PDF
- Alias email par dossier
- Demandes de documents avec suivi d’activité et rappels

**Références :**
- Documentation Odoo 18.0 — Documents (Productivity)
- Documentation Odoo — AI-powered document digitization (Finance)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
