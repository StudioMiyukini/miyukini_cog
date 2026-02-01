# Odoo Documents — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Documents - Logique Metier Complete.md](./00_logique_metier/Odoo%20Documents%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles conceptuels (Dossiers, Documents, Tags, Droits d'accès)
- Règles métier (suppression différée, centralisation, alias email, demandes de documents)
- Sections (All, Company, My Drive, Shared with me, Recent, Trash)
- Partage et droits (Viewer, Editor, expiration, portail)
- Intégrations (Project, Sign, Invoicing, PLM, IA)

### 2. Parcours Utilisateur
📄 [Odoo Documents - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Documents%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Administrateur, Collaborateur, Responsable dossier, Utilisateur portail, Comptable)
- Parcours d'onboarding
- Scénarios d'usage principaux (dépôt, partage, demande document, alias email, IA, PDF)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Documents - Analyse UI UX.md](./02_ui_ux/Odoo%20Documents%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Structure générale (arbre + zone de contenu, prévisualisation)
- Arborescence et actions sur dossier
- Vue liste / grille, prévisualisation fichier
- Panneau Détails (Info & Tags), Share, Request
- Configuration (Paramètres, Tags)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Documents - Integrations Cross App.md](./03_integrations/Odoo%20Documents%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (Mail, Portal, Accounting, Project, Sign, PLM, Spreadsheet)
- Centralisation des fichiers, numérisation IA, portail
- Flux de données inter-apps
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Documents - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Documents%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs (DocumentsFolderOperator, DocumentsFileOperator, DocumentsTagOperator, DocumentsShareOperator, DocumentsRequestOperator, DocumentsUI)
- Contrat d'équipe DocumentsService
- Mandats de Permission et niveaux de sécurité
- Intégrations externes (MiyuInvoice, MiyuProject, MiyuNotify, MiyuPortal)

### 6. Guide Intégration COG
📄 [Odoo Documents - Guide Integration COG.md](./05_integration_cog/Odoo%20Documents%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (dossier, fichier, partage, demande, Trash)
- Exemples de code pseudo-Rust
- Vérification d'accès (partage)

### 7. Guide Implémentation
📄 [Odoo Documents - Guide Implementation.md](./06_guides_implementation/Odoo%20Documents%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique (crates proposées)
- Schémas de données (Folder, Document, Tag, Access, DocumentRequest)
- API et contrats par Opérateur
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

---

## Service Miyukini Proposé

**Nom :** `MiyukiniDocuments` ou `MiyuDocuments`

**Opérateurs :**
- **DocumentsFolderOperator** : Gestion des dossiers (sections, hiérarchie, alias email)
- **DocumentsFileOperator** : Gestion des fichiers (upload, lien, versions, lock, split/merge PDF)
- **DocumentsTagOperator** : Gestion des tags
- **DocumentsShareOperator** : Droits et partage (Viewer/Editor, portail)
- **DocumentsRequestOperator** : Demandes de documents (placeholder, rappels)
- **DocumentsUI** : Interface utilisateur Documents

**Équipe d'Opérateurs :** `DocumentsService`

---

## Source d'Analyse

**Documentation :** Odoo 18.0/19.0 — Productivity / Documents

**Version analysée :** Odoo 18.0 / 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application DMS (Document Management System) avec sections, partage et portail
- Intégrations multiples (Accounting, Project, Sign, PLM, Spreadsheet, IA)
- Centralisation des fichiers et délai de suppression (Trash) à prendre en compte
- Partage externe à aligner sur Façade Publique Gouvernée et Mandat Public d'Accès
