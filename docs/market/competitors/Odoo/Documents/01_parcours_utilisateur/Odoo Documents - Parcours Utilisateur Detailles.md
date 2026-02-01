# Odoo Documents — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Documents** d'Odoo, en identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Productivity / Documents

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur Documents

**Profil :**
- Rôle : Configuration et gouvernance du DMS
- Responsabilités :
  - Définir le délai de suppression (corbeille)
  - Activer et configurer la centralisation des fichiers par app (dossiers, tags, sous-dossiers)
  - Créer et gérer les tags (nom, couleur, tooltip)
  - Définir les droits par défaut et les bonnes pratiques de partage

**Besoins :**
- Paramètres centralisés (Documents ‣ Configuration ‣ Paramètres / Tags)
- Contrôle de la rétention des données (Trash)
- Alignement des dossiers avec les processus métier (HR, Comptabilité, etc.)

**Permissions :**
- Accès Configuration Documents
- Droits d'édition sur les dossiers partagés entreprise

### 1.2 Utilisateur interne (Collaborateur)

**Profil :**
- Rôle : Utilisation quotidienne du DMS (consultation, dépôt, classement, partage)
- Responsabilités :
  - Créer des dossiers (Company, My Drive) et sous-dossiers
  - Uploader des fichiers, créer des liens, des tableurs
  - Taguer, déplacer, renommer, télécharger
  - Partager dossiers/fichiers (Viewer/Editor) avec collègues ou contacts externes
  - Utiliser les raccourcis et favoris (étoile)
  - Consulter Recent et Shared with me

**Besoins :**
- Arborescence claire (sections, favoris)
- Recherche et filtres (tags, recherche plein texte)
- Actions groupées (tag, déplacer, fusionner PDF)
- Chatter (Info & Tags) pour commenter et suivre les changements
- Rappel des demandes de documents (activités)

**Permissions :**
- Accès Documents selon droits attribués
- Édition sur ses propres éléments et ceux partagés en Editor

### 1.3 Responsable de dossier / Chef de projet

**Profil :**
- Rôle : Organisation d’un espace partagé (projet, département, client)
- Responsabilités :
  - Créer et maintenir la structure de dossiers
  - Définir les actions sur sélection et les automatisations (Studio)
  - Configurer l’alias email du dossier pour réception automatique
  - Partager le dossier avec une équipe ou des externes (dates de validité si besoin)
  - Lancer des demandes de documents et relancer via rappels

**Besoins :**
- Actions sur sélection et automatisations configurables
- Partage avec expiration et niveau Viewer/Editor
- Vue Activités pour suivre les demandes de documents
- Téléchargement dossier en .zip

**Permissions :**
- Droits Editor/Owner sur les dossiers concernés
- Accès éventuel à Studio pour automatisations (selon licence)

### 1.4 Utilisateur portail (externe)

**Profil :**
- Rôle : Partenaire (client, fournisseur) accédant aux documents partagés
- Responsabilités :
  - Consulter les dossiers/fichiers partagés (Viewer ou Editor selon cas)
  - Télécharger, éventuellement uploader si Editor
  - Accéder via le portail (carte Documents) avec « Must have the link » au premier accès si configuré

**Besoins :**
- Accès simple au portail, sans formation lourde
- Compréhension des droits (voir vs modifier)
- Notifications ou rappels pour les demandes de documents

**Permissions :**
- Utilisateur portail ; accès uniquement aux dossiers/fichiers partagés avec lui (ou avec le lien)

### 1.5 Comptable / Responsable finance

**Profil :**
- Rôle : Traitement des factures et pièces comptables
- Responsabilités :
  - Centraliser les factures dans le dossier Finance (centralisation)
  - Utiliser la numérisation IA (Create Vendor Bill / Customer Invoice / Credit Note ‣ Send for Digitization)
  - Partager des listes de documents avec fournisseurs/clients (validité, droits)

**Besoins :**
- Dossiers par journal ou par type (configuration centralisation)
- Workflow clair : dépôt → numérisation IA → validation facture
- Tags automatiques pour filtrage

**Permissions :**
- Accès Documents + droits sur dossiers Finance
- Accès Invoicing/Accounting pour valider les factures créées depuis Documents

---

## 2. Parcours d'Onboarding

### 2.1 Premier accès (Administrateur)

1. Activer l’app Documents
2. Aller dans Documents ‣ Configuration ‣ Paramètres
3. Ajuster **Deletion delay (days)** si besoin
4. Configurer la **centralisation** pour les apps concernées (ex. HR, Accounting) : dossier cible, tags, sous-dossiers si besoin
5. Créer les **tags** utiles (Documents ‣ Configuration ‣ Tags)

### 2.2 Premier accès (Utilisateur)

1. Ouvrir Documents
2. Découvrir les sections : All, Company, My Drive, Shared with me, Recent, Trash
3. Créer un dossier test dans My Drive (New ‣ Folder)
4. Uploader un fichier (New ‣ Upload ou glisser-déposer)
5. Ouvrir un fichier, utiliser Info & Tags (détails, chatter, tags)
6. Partager un fichier (Share) avec un collègue (Viewer/Editor)

### 2.3 Mise en place d’un espace projet

1. Créer un dossier dans Company (ex. « Projet X »)
2. Créer des sous-dossiers (Livrables, Factures, Contrats)
3. Configurer l’alias email du dossier si réception par email souhaitée
4. Partager le dossier avec l’équipe (Editor) et éventuellement le client (Viewer) avec date d’expiration
5. Définir les actions sur sélection utiles (ex. créer tâche Project, envoyer à Sign)
6. Optionnel : ajouter une étoile (favori) pour accès rapide

---

## 3. Scénarios d'Usage Principaux

### 3.1 Dépôt et classement

- **Acteur :** Collaborateur
- ** Étapes :** Choisir section/dossier → New ‣ Upload (ou glisser-déposer) → renommer/taguer si besoin → éventuellement déplacer vers un autre dossier
- **Variante :** New ‣ Link (URL + nom + dossier) pour une ressource externe
- **Variante :** New ‣ Spreadsheet pour créer un tableur

### 3.2 Partage avec expiration

- **Acteur :** Responsable de dossier
- **Étapes :** Sélectionner dossier ou fichier → Share → ajouter utilisateurs/contacts (Viewer/Editor) → définir date d’expiration si besoin → Enregistrer
- **Portail :** Les utilisateurs portail voient les documents partagés dans le portail (carte Documents)

### 3.3 Demande de document et suivi

- **Acteur :** Responsable de dossier / Collaborateur
- **Étapes :** New ‣ Request → Document Name, Request To, Due Date In, Folder, Tags, Message → Request
- **Côté destinataire :** Ouvrir le placeholder (ou activité) → Upload du fichier
- **Suivi :** Vue Activités, colonne Requested Document ; rappel unitaire ou groupé (Document Request: Reminder)

### 3.4 Réception de documents par email

- **Acteur :** Toute personne connaissant l’alias
- **Étapes :** Configurer l’alias sur le dossier (Info & Tags ‣ Email alias, domaine, optionnel : activité + tags) → envoyer un email avec pièce jointe à cette adresse → le fichier apparaît dans le dossier avec les tags configurés

### 3.5 Traitement de factures (IA)

- **Acteur :** Comptable
- **Étapes :** Ouvrir le dossier Finance (centralisé) → sélectionner un PDF facture → Create Vendor Bill (ou Customer Invoice / Credit Note) → Send for Digitization → compléter/valider dans Comptabilité

### 3.6 Découpage et fusion de PDF

- **Split :** Ouvrir le PDF → Split PDF → définir les coupures → Split
- **Merge :** Vue liste → sélectionner plusieurs PDF → Action ‣ Merge PDFs → ordre et ajout éventuel → Split (confirmer fusion)

### 3.7 Raccourcis et favoris

- **Raccourci dossier :** Sélectionner un sous-dossier → Actions (engrenage) ‣ Add shortcut → le raccourci apparaît (même dossier si édition, sinon My Drive) ; déplacer par glisser-déposer si besoin
- **Raccourci fichier :** Ouvrir le fichier → Action ‣ Create shortcut
- **Favori dossier :** Actions ‣ Add star ; filtre « Starred » pour y accéder rapidement

---

## 4. Points de Friction Identifiés

- **Limite 64 MB** (Odoo Online) : peut bloquer des fichiers volumineux sans message explicite côté métier
- **Centralisation** : ne s’applique qu’aux **nouveaux** fichiers ; les existants ne sont pas migrés automatiquement
- **Merge PDF** : remplace les originaux ; pas de « fusion en un nouveau fichier » sans perte des originaux par défaut
- **Portail** : premier accès « Must have the link » peut dérouter si le lien n’est pas communiqué clairement
- **Pricing** : actions personnalisées et automatisations Studio peuvent impacter le plan tarifaire (mention Odoo)
- **IA digitization** : limitée au dossier Finance et aux factures ; dépend de la qualité du PDF

---

## 5. Recommandations pour Miyukini

- **Opérateur Documents** : équivalent clair (dossiers, fichiers, tags, droits) avec sections logiques (All, Company, My Drive, Shared with me, Recent, Trash)
- **Mandats et Contrats d’équipe** : partage = Mandat de Permission (StrongFather, KindMother) ; demandes de documents = flux avec MiyuNotify et suivi d’activités
- **WriteIntent** : toute création/modification/suppression de dossier ou document passe par KindMother ; droits par Master Butler et WorrySentinel
- **Portail** : aligner sur Façade Publique Gouvernée et Mandat Public d’Accès pour les utilisateurs externes
- **Centralisation** : configurer par « app Miyukini » (équivalent modules Odoo) avec dossier cible et tags, sans modifier les fichiers existants sauf politique explicite
- **Onboarding** : parcours guidé (admin puis utilisateur) et documentation courte (délai corbeille, partage, demandes, alias email)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
