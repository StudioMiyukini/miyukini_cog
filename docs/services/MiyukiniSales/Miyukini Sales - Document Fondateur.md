# Miyukini Sales — Document Fondateur

## Contexte

**Miyukini Sales** est le **service Miyukini unifié du domaine ventes et devis** au sein de l'écosystème COG. Il couvre la **gestion complète du cycle de vente** : de la création de devis à la confirmation de commande, en passant par la génération de factures et le suivi des paiements.

**Un seul service COG** qui expose des **Opérateurs spécialisés** pour la gestion des ventes B2B et B2C, avec intégration native avec les services CRM, Facturation, et Stock.

Ce document est le **document fondateur** du service : il en fixe la raison d'être, les besoins stratégiques, le positionnement, l'intégration avec les autres services et les niveaux de sécurité associés.

## Portée / Scope

- **Périmètre** : Définition du service Miyukini Sales — besoins, positionnement stratégique, intégration multi-services, niveaux de sécurité.
- **Hors périmètre** : Spécifications techniques détaillées (API, schémas), implémentation des crates.

---

## 1. Besoins Stratégiques

### 1.1 Origine du Besoin

La définition des services **Miyukini CRM** (conversion Opportunity → Quotation), **Miyukini Account** (facturation), **Miyukini Festival Service** (devis exposants), **JayRDV** (devis professionnels) et les besoins **B2B/B2C** ont fait émerger un **besoin transversal** :

- **CRM** : Conversion des opportunities en devis/commandes
- **Account** : Génération de factures depuis les commandes
- **MFS** : Devis et commandes pour exposants
- **JayRDV** : Devis et commandes pour professionnels
- **B2B/B2C** : Gestion complète du cycle de vente (devis → commande → facture → paiement)

Sans service unifié, chaque service définit sa propre logique de devis/commande, et l'utilisateur final n'a pas d'offre cohérente pour gérer ses ventes dans le même écosystème.

### 1.2 Besoins Fonctionnels Identifiés

| Besoin | Description | Consommateurs typiques |
|--------|-------------|------------------------|
| **Gestion de devis** | Création, modification, envoi, suivi des devis | Tous les services |
| **Gestion de commandes** | Confirmation de devis, suivi des commandes | Tous les services |
| **Gestion des lignes** | Produits, quantités, prix, remises, taxes | Tous les services |
| **Génération de factures** | Création automatique de factures depuis commandes | Miyukini Account, MFS, JayRDV |
| **Gestion des prix** | Listes de prix, remises, conditions commerciales | Tous les services |
| **Gestion des paiements** | Paiement en ligne, acomptes, signatures | B2C, Portail client |
| **Suivi des livraisons** | Quantités commandées, livrées, facturées | Services avec stock |
| **Intégration CRM** | Conversion Opportunity → Quotation | Miyukini CRM |
| **Intégration Facturation** | Génération factures, suivi encaissements | Miyukini Account |

### 1.3 Besoin Stratégique de Fond

> **Un service de ventes unifié permet de couvrir le continuum devis → commande → facture → paiement, avec une base COG unique, des Opérateurs spécialisés et des intégrations natives avec CRM, Facturation et Stock.**

La création de **Miyukini Sales** (service COG) répond à la fois au besoin des services métier (CRM, Account, MFS, JayRDV) et à l'offre marché pour les ventes B2B et B2C.

---

## 2. Positionnement Stratégique

### 2.1 Raison d'Être

**Miyukini Sales** (service COG) a pour objectif de :

- **Centraliser le domaine « ventes et devis »** : devis, commandes, lignes, prix, taxes, avec des règles de gouvernance et de sécurité cohérentes.
- **Exposer des Opérateurs réutilisables** : les services métier (CRM, Account, MFS, JayRDV, etc.) consomment le même socle.
- **Intégrer nativement** : avec CRM (conversion), Account (facturation), Store (produits), Booking (rendez-vous).

### 2.2 Différenciation par Contexte

| Aspect | B2B | B2C |
|--------|-----|-----|
| **Workflow** | Devis → Validation → Commande → Facture | Devis → Paiement → Commande → Facture |
| **Signature** | Optionnelle (selon contexte) | Requise pour confirmation |
| **Paiement** | Facturation différée | Paiement en ligne (acompte ou total) |
| **Pricelist** | Négociée, multi-niveaux | Standard, publique |
| **Livraison** | Multi-adresses, dates précises | Adresse unique, délais standards |

---

## 3. Intégration avec Autres Services

### 3.1 Miyukini CRM

**Intégration :**
- Conversion Opportunity → Quotation
- Lien bidirectionnel : `opportunity_id` sur `sale.order`
- Synchronisation équipe commerciale (`team_id`, `user_id`)

**Flux :**
```
CRM Opportunity → Sales Quotation → Sales Order → Account Invoice
```

### 3.2 Miyukini Account

**Intégration :**
- Génération de factures depuis commandes confirmées
- Lien bidirectionnel : `invoice_ids` ↔ `sale_line_ids`
- Suivi des montants facturés et à facturer

**Flux :**
```
Sales Order → Account Invoice (via SalesInvoiceOperator)
```

### 3.3 MiyuStore

**Intégration :**
- Utilisation des produits et catalogues
- Intégration avec pricelist
- Gestion des variantes et attributs produits

### 3.4 MiyuInvoice

**Intégration :**
- Utilisation des outils de facturation (MiyuInvoice)
- Génération PDF, envoi email
- Conformité légale

### 3.5 MiyuContacts

**Intégration :**
- Utilisation pour clients (`partner_id`)
- Gestion adresses facturation/livraison
- Synchronisation des informations

### 3.6 MiyuBooking

**Intégration :**
- Lien avec rendez-vous commerciaux
- Planification depuis opportunity/order

---

## 4. Niveaux de Sécurité

### 4.1 Classification des Données

| Donnée | Niveau | Justification |
|--------|--------|---------------|
| **Devis/Commandes** | 2 (Sensitive) | Données commerciales sensibles (montants, clients, produits) |
| **Lignes de commande** | 2 (Sensitive) | Détails commerciaux |
| **Pricelist** | 1-2 (Standard à Sensitive) | Selon contexte (publique vs négociée) |
| **Paiements** | 3 (Critical) | Données de paiement sensibles |
| **Signatures** | 2-3 (Sensitive à Critical) | Selon contexte légal |

### 4.2 Protection des Données

- **Isolation cross-équipe** : Les commerciaux ne voient que leurs devis/commandes ou ceux de leur équipe
- **Audit** : Toutes les confirmations, modifications importantes, générations de factures sont auditées
- **Chiffrement** : Données de paiement chiffrées (niveau 3)

---

## 5. Architecture Opérateurs (Vue d'Ensemble)

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SalesOrder** | Gestion des devis/commandes | Opérateur de Service |
| **SalesOrderLine** | Gestion des lignes de commande | Opérateur de Service |
| **SalesPricelist** | Gestion des listes de prix | Opérateur de Service |
| **SalesInvoice** | Génération de factures | Opérateur de Service |
| **SalesPayment** | Gestion des paiements et signatures | Opérateur de Service |
| **SalesUI** | Interface utilisateur Sales | Opérateur d'Interface |

**Équipe d'Opérateurs :** SalesService

**Contrat d'Équipe :** À définir dans les spécifications Opérateurs

---

## 6. Points d'Entrée

### 6.1 B2B (Entreprise)

**Workflow typique :**
1. Création devis depuis CRM ou manuellement
2. Envoi devis au client
3. Validation client (signature optionnelle)
4. Confirmation commande
5. Génération facture
6. Suivi paiement

### 6.2 B2C (Consommateur)

**Workflow typique :**
1. Création devis depuis Store ou manuellement
2. Envoi devis au client (portail)
3. Signature + Paiement en ligne (acompte ou total)
4. Confirmation automatique après paiement
5. Génération facture
6. Livraison

---

## 7. Décisions Structurantes

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | Service COG unifié pour ventes B2B et B2C | Réduction duplication, cohérence gouvernance |
| **DS-02** | Données commerciales niveau 2 (Sensitive) ; paiements niveau 3 (Critical) | Alignement avec politique sécurité |
| **DS-03** | Intégration native avec CRM, Account, Store | Fluidité du workflow commercial |
| **DS-04** | Génération factures via MiyuInvoice | Réutilisation outils existants |
| **DS-05** | Paiement en ligne optionnel selon contexte | Flexibilité B2B vs B2C |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
