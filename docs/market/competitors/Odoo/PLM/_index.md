# Odoo PLM — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo PLM - Logique Metier Complete.md](./00_logique_metier/Odoo%20PLM%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (ECO, ECO Type, révision BoM, documents)
- Règles métier et contraintes (gestion des changements, approbations)
- Workflows (création ECO → révision → approbation → application)
- Contrôle de version des nomenclatures (BoM)
- Rebase et résolution de conflits ECO concurrents
- Intégrations Manufacturing, Inventory, Quality

### 2. Parcours Utilisateur
📄 [Odoo PLM - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20PLM%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Ingénieur Produit, Approbateur, Opérateur Fabrication, Responsable PLM)
- Parcours d'onboarding
- Scénarios d'usage principaux (créer ECO, approuver, historique versions, rebase)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo PLM - Analyse UI UX.md](./02_ui_ux/Odoo%20PLM%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vue d'ensemble (PLM Overview) par type d'ECO
- Formulaires ECO et révision BoM
- Onglets BoM Changes, Operation Changes, Previous Eco Bom Changes
- Gestion des documents et pièces jointes
- Patterns de navigation et breadcrumbs
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo PLM - Integrations Cross App.md](./03_integrations/Odoo%20PLM%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances Manufacturing, Inventory, Quality, Mail
- Flux de données (PLM ↔ mrp, stock, quality, mail)
- Mécanismes d'intégration (extension BoM, workflow, alias email)
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo PLM - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20PLM%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (EcoOperator, EcoTypeOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator, PlmUI)
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo PLM - Guide Integration COG.md](./05_integration_cog/Odoo%20PLM%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (création ECO, Start Revision, Apply Changes, approbation, Apply Rebase)
- Exemples de code pseudo-Rust
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo PLM - Guide Implementation.md](./06_guides_implementation/Odoo%20PLM%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique des crates Rust (miyuplm, miyuplm-ui)
- Schémas de données (Eco, EcoType, BomRevision, Approval, Document)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniPLM` ou `MiyuPLM`

**Opérateurs :**
- **EcoOperator** : Gestion des ordres de modification (ECO)
- **EcoTypeOperator** : Gestion des types d'ECO et des stages
- **BomRevisionOperator** : Gestion des révisions BoM (versioning)
- **EcoApprovalOperator** : Gestion des approbations
- **EcoDocumentOperator** : Gestion des documents de conception
- **PlmUI** : Interface utilisateur PLM

**Équipe d'Opérateurs :** `PlmService`

---

## Source d'Analyse

**Documentation :** Odoo 19.0 PLM (Product Lifecycle Management)

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application Supply Chain centrée sur la gestion des changements produit et nomenclatures (BoM)
- Intégrations Manufacturing, Inventory, Quality, Mail
- Workflow ECO avec stages et approbations ; versioning BoM ; rebase pour conflits concurrents
- Correspondance Miyukini : PlmService (EcoOperator, BomRevisionOperator, EcoApprovalOperator, EcoDocumentOperator, PlmUI)
