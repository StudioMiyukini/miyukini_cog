# Odoo Invoicing — Index de Documentation

## Contexte

Ce dossier contient l’**analyse complète** de l’application **Invoicing** d’Odoo, réalisée selon la méthodologie standardisée. L’analyse couvre la logique métier, les parcours utilisateur, l’UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l’intégration COG et les guides d’implémentation.

**Date d’analyse :** 2026-02-01  
**Source :** Code source Odoo 19.0 (module account — périmètre Invoicing)

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Invoicing - Logique Métier Complète](./00_logique_metier/Odoo%20Invoicing%20-%20Logique%20Metier%20Complete.md)**
  - Modèles facturation (account.move, account.move.line, account.payment, account.payment.term)
  - Règles métier et workflows (création, validation, envoi, paiement)
  - Calculs (montants HT/TTC, taxes, échéances)
  - Réconciliation facture/paiement
  - Points d’attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Invoicing - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Invoicing%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (facturation)
  - Parcours d’onboarding Invoicing
  - Scénarios d’usage (création, envoi, paiement, avoirs)
  - Points de friction et recommandations Miyukini

### 02_ui_ux/
- **[Odoo Invoicing - Analyse UI/UX](./02_ui_ux/Odoo%20Invoicing%20-%20Analyse%20UI%20UX.md)**
  - Vues principales (Liste, Kanban, Formulaire factures)
  - Composants et widgets facturation
  - Patterns de navigation et actions contextuelles
  - Wizard paiement, rapports et exports

### 03_integrations/
- **[Odoo Invoicing - Intégrations Cross-App](./03_integrations/Odoo%20Invoicing%20-%20Integrations%20Cross%20App.md)**
  - Intégration avec Sales
  - Intégration avec Purchase
  - Intégration avec Stock, CRM, Project, Expense
  - Portail et Website
  - Mécanismes d’intégration et recommandations Miyukini

### 04_specifications_miyukini/
- **[Odoo Invoicing - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Invoicing%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI)
  - Équipe d’Opérateurs InvoiceService
  - Contrat d’Équipe et Mandats de Permission (Standard, Validation, Envoi, Paiement, Configuration)
  - Niveaux de sécurité (1–3 selon données)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Invoicing - Guide Intégration COG](./05_integration_cog/Odoo%20Invoicing%20-%20Guide%20Integration%20COG.md)**
  - Architecture d’intégration COG (InvoiceService)
  - Patterns d’implémentation (WriteIntent, Mandats, validation, envoi, paiement)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec MiyuInvoice et Cores

### 06_guides_implementation/
- **[Odoo Invoicing - Guide Implémentation](./06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md)**
  - Architecture technique (crates Rust : InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI)
  - Schémas de données (Invoice, InvoiceLine, Payment, PaymentTerm)
  - API et contrats
  - Plan de développement par phases (MVP → Essentiel → Complet/Portail)
  - Bornage fonctionnel et critères d’acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Facturation clients**
   - Création, validation, envoi (email + PDF)
   - Suivi des paiements et réconciliation
   - Conditions de paiement et échéanciers

2. **Facturation fournisseurs**
   - Création, validation, enregistrement des paiements
   - Réconciliation facture/paiement

3. **Avoirs**
   - Création d’avoirs (clients/fournisseurs) liés à une facture
   - Envoi et suivi

4. **Paiements**
   - Enregistrement des paiements (client/fournisseur)
   - Réconciliation partielle ou totale (plusieurs factures / plusieurs paiements)

5. **Intégrations**
   - Sales (factures depuis commandes)
   - Purchase (factures fournisseur depuis commandes)
   - Project / Timesheet, Expense, CRM (selon modules)
   - Portail client (consultation factures, paiement en ligne si activé)

### Architecture Miyukini Proposée

**5 Opérateurs (InvoiceService) :**
- InvoiceLedger (factures, validation)
- InvoicePayment (paiements, réconciliation)
- InvoiceSend (envoi email, PDF, portail)
- InvoiceTerms (conditions de paiement)
- InvoiceUI (interface)

**1 Équipe d’Opérateurs :** InvoiceService

**Réutilisation :** MiyuInvoice (calculs, PDF), KindMother (WriteIntent), StrongFather (décisions), Ever Buddy (séquences), Master Butler (Mandats), WorrySentinel (sécurité)

**Niveaux de sécurité :** 1–3 selon données (Standard à Critical)

**Correspondance Miyukini :** MiyuInvoice (existant) + InvoiceService (Opérateurs facturation)

---

## Statut de l’Analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

## Prochaines Étapes

1. **Valider les spécifications** : Revue avec l’équipe technique
2. **Démarrer l’implémentation** : Phase 1 (MVP) selon le guide
3. **Itérer** : Selon retours et besoins utilisateurs
4. **Intégration** : Avec Miyukini Sales, Purchase, portail client

---

**Document** : Odoo Invoicing — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100 % — référence pour implémentation Miyukini
