# Odoo Documents — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Documents** d'Odoo, en identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Productivity / Documents, et fonctionnalités publiées

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (centralisation, IA, portail)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (conceptuels)

**Fonctionnalités de base :**
- **Base / Web** : Framework, vues, authentification
- **Mail** : Chatter (Info & Tags), activités, notifications
- **Portal** : Accès utilisateurs externes (carte Documents, partage par lien)
- **Storage / Attachments** : Stockage des fichiers (ir.attachment ou équivalent)
- **Spreadsheet** (optionnel) : Création de tableurs depuis Documents (New ‣ Spreadsheet)

**Fonctionnalités avancées :**
- **Accounting / Invoicing** : Centralisation des factures, numérisation IA (Create Vendor Bill / Customer Invoice / Credit Note ‣ Send for Digitization)
- **Project** : Liens tâches ↔ documents, centralisation possible des pièces jointes projet
- **Sign** : Envoi de documents à signer depuis Documents
- **PLM** : Documents liés au cycle de vie produit, centralisation possible
- **Studio** (optionnel) : Actions personnalisées et automatisations par dossier

### 1.2 Flux de données

```
Documents (dossiers/fichiers)
    ↑ centralisation
    ├── Accounting → dossier Finance, sous-dossiers par journal
    ├── HR → dossier HR, sous-dossiers (ex. Payroll)
    ├── Project → pièces jointes tâches → corbeille Documents si enregistrement supprimé
    └── Autres apps → dossiers + tags configurables

Documents → Accounting : numérisation IA (PDF facture → facture fournisseur/client)
Documents → Sign : envoi document à signer
Documents → Portal : partage dossiers/fichiers (Viewer/Editor, expiration)
```

---

## 2. Intégrations Détaillées

### 2.1 Centralisation des fichiers (File centralization)

**Principe :** Pour une app donnée, les **nouveaux** fichiers associés à des enregistrements de cette app sont automatiquement rangés dans un dossier Documents dédié, avec tags optionnels.

**Configuration :** Documents ‣ Configuration ‣ Paramètres ; choix de l’app, du **dossier** cible, des **tags** ; sous-dossiers possibles (ex. Comptabilité : par journal).

**Règles :**
- Ne s’applique qu’aux **nouveaux** fichiers ; les existants ne sont pas modifiés
- Si la centralisation est activée et qu’un enregistrement de l’app est **supprimé**, les pièces jointes de cet enregistrement sont déplacées vers la **corbeille** Documents (Trash)

**Recommandations Miyukini :**
- Équivalent « centralisation » par Opérateur Miyukini : configuration (dossier cible + tags) dans DocumentsOperator ; création de documents via WriteIntent KindMother avec lien vers l’entité source
- Suppression d’entité source : décision StrongFather + déplacement vers Trash (Ever Buddy / cycle de vie) sans suppression immédiate

### 2.2 Accounting / Invoicing

**Flux :**
- **Centralisation** : Factures et pièces jointes comptables → dossier(s) Finance (optionnel par journal)
- **Numérisation IA** : Fichier PDF facture dans le dossier Finance → Create Vendor Bill / Customer Invoice / Customer Credit Note → **Send for Digitization** → extraction des données (fournisseur, montants, lignes) → création brouillon facture dans Comptabilité

**Documentation Odoo :** AI-powered document digitization (Finance / Vendor bills)

**Recommandations Miyukini :**
- DocumentsOperator expose une capacité « digitize_invoice » ; flux avec MiyuInvoice (ou équivalent facturation) et service IA (hors scope COG : délégation contrôlée)
- Mandat de Permission pour lier document → facture (StrongFather, KindMother)
- Niveau de sécurité élevé (données financières) : WorrySentinel

### 2.3 Project

**Flux :**
- Pièces jointes des tâches/projets peuvent être **centralisées** dans Documents (dossier + tags)
- Liens directs depuis les tâches vers les documents (affichage, ouverture)
- Suppression d’une tâche (si centralisation activée) → pièces jointes déplacées vers Trash Documents

**Recommandations Miyukini :**
- Contrat d’équipe DocumentsService ↔ ProjectService : création document depuis tâche = Mandat + WriteIntent ; suppression tâche = décision StrongFather + déplacement pièces vers Trash
- MiyuProject (ou équivalent) consomme DocumentsOperator pour afficher/ouvrir les documents liés

### 2.4 Sign

**Flux :**
- Depuis Documents : sélection d’un fichier (ou dossier) → action « Envoyer à Sign » (ou équivalent) → envoi vers Odoo Sign pour signature
- Document signé peut être re-stocké ou lié dans Documents selon configuration

**Recommandations Miyukini :**
- Action « send_to_sign » exposée par DocumentsOperator avec Mandat vers Opérateur Sign (équivalent Odoo Sign) ; pas de stockage direct dans Documents du flux Sign, uniquement lien/référence si besoin

### 2.5 PLM (Product Lifecycle Management)

**Flux :**
- Documents liés aux produits/versions (nomenclatures, specs, certificats) ; centralisation possible vers un dossier PLM dans Documents
- Consultation et partage des documents PLM depuis Documents

**Recommandations Miyukini :**
- Si un Opérateur PLM existe : centralisation configurable (dossier + tags) ; Contrat d’équipe DocumentsService ↔ PLMOperator pour création/lien de documents

### 2.6 Mail / Chatter / Activités

**Flux :**
- **Chatter** sur dossiers et fichiers (Info & Tags) : messages, pièces jointes, historique
- **Activités** : planification d’activités sur un dossier/fichier ; type d’activité et assignation configurables (ex. sur alias email)
- **Demandes de documents** : création d’une activité « document demandé » ; suivi dans la vue Activités (colonne Requested Document) ; rappels (email) unitaires ou groupés

**Recommandations Miyukini :**
- MiyuNotify pour activités et notifications ; chatter = flux de messages gouverné (KindMother pour persistance, Master Butler pour droits)
- Demandes de documents = modèle DocumentRequest + activité + rappels via MiyuNotify

### 2.7 Portail (Portal)

**Flux :**
- Utilisateurs **portail** : accès aux dossiers/fichiers partagés avec eux (Viewer/Editor) via la **carte Documents** du portail
- Partage : « Anyone with the link » avec option **Discoverable** ou **Must have the link to access**
- Premier accès portail : utilisateurs publics doivent avoir le lien pour accéder (Must have the link)
- Chaque URL de dossier/fichier reflète les droits ; partager un dossier redirige vers un portail dédié (fichiers à accès restreint exclus)

**Recommandations Miyukini :**
- Aligner sur **Façade Publique Gouvernée** et **Mandat Public d’Accès** pour utilisateurs externes ; pas d’entrée dans le COG, uniquement consommation de surfaces exposées
- DocumentsUI (portail) : lecture/téléchargement (et upload si Editor) selon Mandat Public attaché au dossier/fichier partagé

### 2.8 Spreadsheet

**Flux :**
- Depuis Documents : New ‣ **Spreadsheet** → création d’un tableur (app Spreadsheet) → stockage et partage comme tout document dans Documents

**Recommandations Miyukini :**
- Si un Opérateur Spreadsheet existe : création de document de type « spreadsheet » via DocumentsOperator avec Mandat ; stockage référence ou blob selon architecture

---

## 3. Synthèse des flux

| App cible      | Direction           | Mécanisme principal                          |
|----------------|--------------------|----------------------------------------------|
| Accounting     | Documents → Accounting | Numérisation IA (PDF → facture)             |
| Accounting     | Accounting → Documents | Centralisation factures (dossier + tags)    |
| Project        | Project → Documents    | Centralisation pièces jointes tâches       |
| Project        | Documents ↔ Project    | Liens tâche ↔ document                     |
| Sign           | Documents → Sign       | Envoi document à signer                    |
| PLM            | PLM → Documents       | Centralisation docs produit                |
| Mail           | Documents ↔ Mail       | Chatter, activités, demandes, rappels      |
| Portal         | Documents → Portal     | Partage dossiers/fichiers (Viewer/Editor)   |
| Spreadsheet    | Documents ↔ Spreadsheet| Création tableur, stockage dans Documents   |

---

## 4. Recommandations pour Miyukini

- **DocumentsOperator** : autorité sur dossiers, fichiers, tags, droits ; toutes les écritures via KindMother (WriteIntent) ; partage et accès via Master Butler et WorrySentinel
- **Contrats d’équipe** : avec MiyuInvoice (facturation), MiyuProject (projet), MiyuNotify (activités/rappels), Opérateur Sign si existant, Opérateur PLM si existant
- **Centralisation** : configuration par « app » Miyukini (dossier cible, tags, sous-dossiers) ; déclenchement à la création de pièce jointe / document lié à une entité ; suppression entité → déplacement vers Trash (pas de suppression immédiate)
- **Portail** : Façade Publique Gouvernée + Mandat Public d’Accès ; pas d’identité COG pour utilisateurs externes
- **IA (digitization)** : capacité exposée par DocumentsOperator avec Mandat vers service facturation ; niveau sécurité élevé (WorrySentinel)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
