# Odoo Purchase — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Purchase** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Purchase
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **PurchaseOrder** | Gestion des RFQ/commandes d'achat | Opérateur de Service |
| **PurchaseOrderLine** | Gestion des lignes de commande | Opérateur de Service |
| **PurchaseApproval** | Gestion des approbations | Opérateur de Service |
| **PurchaseInvoice** | Génération de factures fournisseur | Opérateur de Service |
| **PurchaseReception** | Gestion des réceptions (si Inventory) | Opérateur de Service |
| **PurchaseUI** | Interface utilisateur Purchase | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : PurchaseService

**Définition :**
> **PurchaseService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des achats et RFQ.**

**Composition :**
- PurchaseOrder (niveau sécurité 2)
- PurchaseOrderLine (niveau sécurité 2)
- PurchaseApproval (niveau sécurité 2-3)
- PurchaseInvoice (niveau sécurité 3)
- PurchaseReception (niveau sécurité 2, si Inventory)
- PurchaseUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 PurchaseOrder

**Rôle :** Gestion des RFQ (Demandes de Devis) et commandes d'achat.

**Capacités :**
- Création/modification de RFQ/commandes
- Envoi RFQ aux fournisseurs
- Confirmation de commande
- Gestion des états (draft, sent, to approve, purchase, cancel)
- Calcul des montants (HT, TTC, taxes)
- Gestion des approbations
- Génération de factures fournisseur

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de confirmation, approbation, modification commande confirmée
- **KindMother** : Persistance des RFQ/commandes (WriteIntent)
- **Master Butler** : Permissions de création/modification/confirmation
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Gestion du cycle de vie (draft → sent → to approve → purchase → cancel)

**Contrat d'équipe :**
- Consomme : PurchaseOrderLine (lignes), MiyuContacts (fournisseurs), MiyuStore (produits), PurchaseApproval (approbations)
- Expose : `order.create`, `order.update`, `order.send_rfq`, `order.confirm`, `order.approve`, `order.cancel`

**Mandat de Permission requis :**
- Création RFQ : Mandat avec PurchaseOrderLine + MiyuContacts + MiyuStore
- Envoi RFQ : Mandat avec MiyuNotify (email)
- Confirmation commande : Mandat avec StrongFather (décision) + KindMother (WriteIntent)
- Approbation : Mandat avec PurchaseApproval + StrongFather (décision)
- Génération facture : Mandat avec PurchaseInvoice + StrongFather (décision)

### 2.2 PurchaseOrderLine

**Rôle :** Gestion des lignes de commande d'achat (produits, quantités, prix, taxes).

**Capacités :**
- Création/modification de lignes
- Calcul des prix (seller, remises)
- Calcul des taxes
- Gestion des quantités (commandées, reçues, facturées)
- Gestion des dates prévues

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des lignes (WriteIntent)
- **Master Butler** : Permissions de modification
- **WorrySentinel** : Vérification modification lignes confirmées

**Contrat d'équipe :**
- Consommé par : PurchaseOrder
- Consomme : MiyuStore (produits, sellers)

### 2.3 PurchaseApproval

**Rôle :** Gestion des approbations de commandes d'achat.

**Capacités :**
- Validation des règles d'approbation
- Approbation de commandes
- Gestion de la double validation
- Notifications d'approbation

**Niveau de sécurité :** 2-3 (Sensitive à Critical selon montant)

**Gouvernance :**
- **StrongFather** : Décision d'approbation
- **Master Butler** : Permissions d'approbation (selon groupe)
- **WorrySentinel** : Niveau sécurité selon montant

**Contrat d'équipe :**
- Consommé par : PurchaseOrder
- Consomme : StrongFather (décision)

### 2.4 PurchaseInvoice

**Rôle :** Génération de factures fournisseur depuis les commandes.

**Capacités :**
- Génération de factures depuis commandes
- Groupement de commandes
- Matching factures avec commandes
- Gestion des acomptes

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision de génération facture
- **KindMother** : Persistance des factures (WriteIntent)
- **MiyuInvoice** : Outils de facturation fournisseur

**Contrat d'équipe :**
- Consomme : PurchaseOrder (commandes), MiyuInvoice (outils)

### 2.5 PurchaseReception (si Inventory)

**Rôle :** Gestion des réceptions de produits.

**Capacités :**
- Création de réceptions depuis commandes
- Validation des quantités reçues
- Synchronisation avec Inventory

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des réceptions (WriteIntent)
- **MiyuInventory** : Outils de gestion stock

**Contrat d'équipe :**
- Consomme : PurchaseOrder (commandes), MiyuInventory (outils)

### 2.6 PurchaseUI

**Rôle :** Interface utilisateur Purchase.

**Capacités :**
- Affichage des RFQ/commandes
- Création/modification via interface
- Tableaux de bord et KPIs
- Rapports et analyses

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d'affichage
- **WorrySentinel** : Filtrage selon niveau sécurité

**Contrat d'équipe :**
- Consomme : PurchaseOrder, PurchaseOrderLine, PurchaseApproval, PurchaseInvoice

---

## 3. Contrat d'Équipe PurchaseService

### 3.1 Flux Autorisés

1. **PurchaseUI → PurchaseOrder** : Création/modification/consultation
2. **PurchaseOrder → PurchaseOrderLine** : Gestion des lignes
3. **PurchaseOrder → PurchaseApproval** : Demande d'approbation
4. **PurchaseOrder → PurchaseInvoice** : Génération factures
5. **PurchaseOrder → PurchaseReception** : Création réceptions (si Inventory)
6. **PurchaseOrder → MiyuContacts** : Consultation fournisseurs
7. **PurchaseOrder → MiyuStore** : Consultation produits/sellers

### 3.2 Mandats de Permission

**Mandat : Création RFQ**
- Opérateurs : PurchaseOrder, PurchaseOrderLine, MiyuContacts, MiyuStore
- Flux : PurchaseUI → PurchaseOrder → PurchaseOrderLine → MiyuStore
- Niveau sécurité max : 2

**Mandat : Envoi RFQ**
- Opérateurs : PurchaseOrder, MiyuNotify
- Flux : PurchaseOrder → MiyuNotify (email)
- Niveau sécurité max : 2

**Mandat : Confirmation Commande**
- Opérateurs : PurchaseOrder, StrongFather, KindMother
- Flux : PurchaseOrder → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 2

**Mandat : Approbation Commande**
- Opérateurs : PurchaseApproval, PurchaseOrder, StrongFather
- Flux : PurchaseApproval → PurchaseOrder (lecture) → StrongFather (décision) → PurchaseOrder (mise à jour état)
- Niveau sécurité max : 3 (si montant élevé)

**Mandat : Génération Facture**
- Opérateurs : PurchaseInvoice, PurchaseOrder, MiyuInvoice, StrongFather
- Flux : PurchaseInvoice → PurchaseOrder (lecture) → MiyuInvoice (génération) → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 3

---

## 4. Intégration avec les Cores

### 4.1 StrongFather

**Rôles :**
- Décision de confirmation de commande
- Décision d'approbation (si double validation)
- Décision de génération de facture
- Décision de modification de commande confirmée
- Validation des règles d'approbation

### 4.2 KindMother

**Rôles :**
- Persistance de toutes les données Purchase (WriteIntent)
- Autorité absolue sur les données
- Synchronisation avec factures (via PurchaseInvoice)

### 4.3 Master Butler

**Rôles :**
- Déclaration des capacités Purchase
- Permissions d'accès aux RFQ/commandes
- Permissions d'approbation (selon groupe)
- Isolation cross-équipe

### 4.4 WorrySentinel

**Rôles :**
- Niveau de sécurité : 2 (données achats), 3 (factures fournisseur)
- Vérification isolation cross-équipe
- Audit des confirmations et facturations
- Vérification approbations

### 4.5 Ever Buddy

**Rôles :**
- Gestion du cycle de vie (draft → sent → to approve → purchase → cancel)
- Gestion des versions de sellers
- Gestion dépréciation/retrait fonctionnalités

---

## 5. Intégrations avec Autres Services Miyukini

### 5.1 Miyukini Accounting

**Intégration :**
- Génération factures fournisseur depuis commandes
- Lien bidirectionnel commande ↔ facture
- Synchronisation montants

### 5.2 MiyuStore

**Intégration :**
- Utilisation des produits et catalogues
- Calcul prix depuis sellers (`product.supplierinfo`)
- Ajout fournisseur aux produits automatique

### 5.3 MiyuContacts

**Intégration :**
- Utilisation pour fournisseurs
- Gestion adresses dropship
- Historique commandes par fournisseur

### 5.4 MiyuInventory (si développé)

**Intégration :**
- Création réceptions depuis commandes
- Synchronisation quantités reçues
- Gestion dates prévues

### 5.5 MiyuNotify

**Intégration :**
- Envoi RFQ par email
- Notifications approbation
- Rappels réception

### 5.6 MiyuPortal (si nécessaire)

**Intégration :**
- Portail fournisseur pour consultation commandes
- Reconnaissance (acknowledge)
- Mise à jour dates prévues

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
