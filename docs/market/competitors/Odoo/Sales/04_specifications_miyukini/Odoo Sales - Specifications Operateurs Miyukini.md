# Odoo Sales — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Sales d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Sales
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SalesOrder** | Gestion des devis/commandes | Opérateur de Service |
| **SalesOrderLine** | Gestion des lignes de commande | Opérateur de Service |
| **SalesPricelist** | Gestion des listes de prix | Opérateur de Service |
| **SalesInvoice** | Génération de factures | Opérateur de Service |
| **SalesPayment** | Gestion des paiements et signatures | Opérateur de Service |
| **SalesUI** | Interface utilisateur Sales | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : SalesService

**Définition :**
> **SalesService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des ventes et devis.**

**Composition :**
- SalesOrder (niveau sécurité 2)
- SalesOrderLine (niveau sécurité 2)
- SalesPricelist (niveau sécurité 1-2)
- SalesInvoice (niveau sécurité 2-3)
- SalesPayment (niveau sécurité 3)
- SalesUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 SalesOrder

**Rôle :** Gestion des devis et commandes de vente.

**Capacités :**
- Création/modification de devis/commandes
- Confirmation de commande
- Gestion des états (draft, sent, sale, cancel)
- Calcul des montants (HT, TTC, taxes)
- Gestion des paiements et signatures

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de confirmation, modification commande confirmée
- **KindMother** : Persistance des devis/commandes (WriteIntent)
- **Master Butler** : Permissions de création/modification/confirmation
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Gestion du cycle de vie (draft → sent → sale → cancel)

**Contrat d'équipe :**
- Consomme : SalesOrderLine (lignes), SalesPricelist (prix), MiyuContacts (clients), MiyuStore (produits)
- Expose : `order.create`, `order.update`, `order.confirm`, `order.cancel`

**Mandat de Permission requis :**
- Création devis : Mandat avec SalesOrderLine + SalesPricelist
- Confirmation commande : Mandat avec StrongFather (décision) + KindMother (WriteIntent)
- Génération facture : Mandat avec SalesInvoice + StrongFather (décision)

### 2.2 SalesOrderLine

**Rôle :** Gestion des lignes de commande (produits, quantités, prix, taxes).

**Capacités :**
- Création/modification de lignes
- Calcul des prix (pricelist, remises)
- Calcul des taxes
- Gestion des quantités (commandées, livrées, facturées)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des lignes (WriteIntent)
- **Master Butler** : Permissions de modification
- **WorrySentinel** : Vérification modification lignes confirmées

**Contrat d'équipe :**
- Consommé par : SalesOrder
- Consomme : MiyuStore (produits), SalesPricelist (prix)

### 2.3 SalesPricelist

**Rôle :** Gestion des listes de prix et règles de tarification.

**Capacités :**
- Création/modification de pricelists
- Calcul des prix selon règles
- Gestion des remises

**Niveau de sécurité :** 1-2 (Standard à Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des pricelists
- **Master Butler** : Permissions de configuration

### 2.4 SalesInvoice

**Rôle :** Génération de factures depuis les commandes.

**Capacités :**
- Génération de factures depuis commandes
- Groupement de commandes
- Gestion des acomptes

**Niveau de sécurité :** 2-3 (Sensitive à Critical)

**Gouvernance :**
- **StrongFather** : Décision de génération facture
- **KindMother** : Persistance des factures (WriteIntent)
- **MiyuInvoice** : Outils de facturation

**Contrat d'équipe :**
- Consomme : SalesOrder (commandes), MiyuInvoice (outils)

### 2.5 SalesPayment

**Rôle :** Gestion des paiements en ligne et signatures.

**Capacités :**
- Gestion des transactions de paiement
- Gestion des signatures en ligne
- Confirmation automatique après paiement

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision de confirmation après paiement
- **KindMother** : Persistance des signatures (WriteIntent)
- **WorrySentinel** : Niveau sécurité 3 pour données paiement

---

## 3. Contrat d'Équipe SalesService

### 3.1 Flux Autorisés

1. **SalesUI → SalesOrder** : Création/modification/consultation
2. **SalesOrder → SalesOrderLine** : Gestion des lignes
3. **SalesOrder → SalesPricelist** : Calcul des prix
4. **SalesOrder → SalesInvoice** : Génération factures
5. **SalesOrder → SalesPayment** : Paiements et signatures

### 3.2 Mandats de Permission

**Mandat : Création Devis**
- Opérateurs : SalesOrder, SalesOrderLine, SalesPricelist
- Flux : SalesUI → SalesOrder → SalesOrderLine → SalesPricelist
- Niveau sécurité max : 2

**Mandat : Confirmation Commande**
- Opérateurs : SalesOrder, StrongFather, KindMother
- Flux : SalesOrder → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 2

**Mandat : Génération Facture**
- Opérateurs : SalesInvoice, SalesOrder, MiyuInvoice, StrongFather
- Flux : SalesInvoice → SalesOrder (lecture) → MiyuInvoice (génération) → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 3

---

## 4. Intégration avec les Cores

### 4.1 StrongFather

**Rôles :**
- Décision de confirmation de commande
- Décision de génération de facture
- Décision de modification de commande confirmée
- Décision de confirmation après paiement

### 4.2 KindMother

**Rôles :**
- Persistance de toutes les données Sales (WriteIntent)
- Autorité absolue sur les données

### 4.3 Master Butler

**Rôles :**
- Déclaration des capacités Sales
- Permissions d'accès aux devis/commandes
- Isolation cross-équipe

### 4.4 WorrySentinel

**Rôles :**
- Niveau de sécurité : 2 (données commerciales), 3 (paiements)
- Vérification isolation cross-équipe
- Audit des confirmations et facturations

### 4.5 Ever Buddy

**Rôles :**
- Gestion du cycle de vie (draft → sent → sale → cancel)
- Gestion des versions de pricelist

---

## 5. Intégrations avec Autres Services Miyukini

### 5.1 Miyukini CRM

**Intégration :**
- Conversion Opportunity → Quotation
- Lien bidirectionnel
- Synchronisation équipe commerciale

### 5.2 MiyuInvoice

**Intégration :**
- Génération factures depuis commandes
- Utilisation des outils MiyuInvoice

### 5.3 MiyuStore

**Intégration :**
- Utilisation des produits et catalogues
- Intégration avec pricelist

### 5.4 MiyuContacts

**Intégration :**
- Utilisation pour clients
- Gestion adresses facturation/livraison

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
