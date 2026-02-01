# Odoo Purchase — Index Documentation

## Contexte

Ce document sert d'**index central** pour l'analyse complète de l'application **Purchase** (Achats) d'Odoo, extraite du code source GitHub.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/purchase`

**Date d'analyse :** 2026-02-01

---

## Structure de Documentation

Cette analyse complète de l'application Purchase est organisée en **7 documents** couvrant tous les aspects nécessaires pour comprendre et implémenter un équivalent dans l'écosystème Miyukini.

### 📋 Documents Disponibles

#### 1. Logique Métier Complète
**Fichier :** `00_logique_metier/Odoo Purchase - Logique Metier Complete.md`

**Contenu :**
- Modèles de données principaux (PurchaseOrder, PurchaseOrderLine)
- Règles métier et contraintes
- Workflows et transitions d'état (Draft → Sent → To Approve → Purchase → Cancel)
- Calculs de prix (fournisseur, taxes, remises)
- Gestion des quantités (commandées, reçues, facturées)
- Système d'approbation
- Gestion des doublons

**Statut :** ✅ Complété

---

#### 2. Parcours Utilisateur Détaillés
**Fichier :** `01_parcours_utilisateur/Odoo Purchase - Parcours Utilisateur Detailles.md`

**Contenu :**
- Personas et rôles utilisateurs (Acheteur, Manager, Comptable, Fournisseur)
- Parcours d'onboarding
- Scénarios d'usage principaux (création RFQ, confirmation, approbation, facturation, réception)
- Points de friction identifiés
- Recommandations pour Miyukini

**Statut :** ✅ Complété

---

#### 3. Analyse UI/UX
**Fichier :** `02_ui_ux/Odoo Purchase - Analyse UI UX.md`

**Contenu :**
- Vues principales (List, Kanban, Form, Calendar, Graph, Pivot, Activity)
- Composants d'interface spécialisés
- Patterns de navigation
- Formulaires et validations
- Tableaux et listes
- Design responsive et accessibilité
- Recommandations pour Miyukini

**Statut :** ✅ Complété

---

#### 4. Intégrations Cross-App
**Fichier :** `03_integrations/Odoo Purchase - Integrations Cross App.md`

**Contenu :**
- Dépendances avec autres apps Odoo (Accounting, Inventory, Product, Portal)
- Flux de données inter-apps
- Mécanismes d'intégration (hooks, overrides, événements)
- APIs et méthodes publiques
- Recommandations pour Miyukini

**Statut :** ✅ Complété

---

#### 5. Spécifications Opérateurs Miyukini
**Fichier :** `04_specifications_miyukini/Odoo Purchase - Specifications Operateurs Miyukini.md`

**Contenu :**
- Opérateurs identifiés (PurchaseOrder, PurchaseOrderLine, PurchaseApproval, PurchaseInvoice, PurchaseReception, PurchaseUI)
- Contrat d'équipe PurchaseService
- Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)
- Intégrations avec autres services Miyukini

**Statut :** ✅ Complété

---

#### 6. Guide Intégration COG
**Fichier :** `05_integration_cog/Odoo Purchase - Guide Integration COG.md`

**Contenu :**
- Architecture d'intégration COG
- Patterns d'implémentation (création RFQ, envoi, confirmation, approbation, génération facture)
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent
- Gestion des Mandats
- Intégration avec autres services

**Statut :** ✅ Complété

---

#### 7. Guide Implémentation
**Fichier :** `06_guides_implementation/Odoo Purchase - Guide Implementation.md`

**Contenu :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

**Statut :** ✅ Complété

---

## Statut Global

**✅ Analyse complète à 100% (7/7 documents)**

Tous les documents de l'analyse Purchase sont complétés et prêts pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

---

## Correspondance Miyukini

**Service Miyukini proposé :** `MiyukiniPurchase` ou `MiyuPurchase`

**Opérateurs proposés :**
- `PurchaseOrderOperator`
- `PurchaseOrderLineOperator`
- `PurchaseApprovalOperator`
- `PurchaseInvoiceOperator`
- `PurchaseReceptionOperator` (si Inventory développé)
- `PurchaseUI`

**Équipe d'Opérateurs :** `PurchaseService`

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
