# Odoo POS Shop — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Point of Sale (POS) Shop d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent POS Shop
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **PosSession** | Gestion des sessions de caisse (ouverture, clôture, contrôle) | Opérateur de Service |
| **PosOrder** | Gestion des commandes (tickets) et lignes | Opérateur de Service |
| **PosPayment** | Gestion des paiements et rapprochement caisse | Opérateur de Service |
| **PosConfig** | Configuration des points de vente | Opérateur de Service |
| **PosUI** | Interface utilisateur POS (écran de vente, paiement) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : PosShopService

**Définition :**
> **PosShopService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de point de vente (caisse, tickets, paiements, sessions).**

**Composition :**
- PosSession (niveau sécurité 2)
- PosOrder (niveau sécurité 2)
- PosPayment (niveau sécurité 3)
- PosConfig (niveau sécurité 1-2)
- PosUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 PosSession

**Rôle :** Gestion des sessions de caisse (ouverture, clôture, contrôle d'ouverture et de clôture).

**Capacités :**
- Ouverture de session (saisie fonds de caisse)
- Clôture de session (comptage, validation écarts)
- Consultation des commandes et totaux de la session
- Cash In / Cash Out (entrées/sorties de caisse)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d'ouverture/clôture de session, autorisation de clôture avec écart
- **KindMother** : Persistance des sessions et des mouvements de caisse (WriteIntent)
- **Master Butler** : Permissions par point de vente (caissier, responsable)
- **WorrySentinel** : Vérification niveau sécurité, isolation par point de vente
- **Ever Buddy** : Gestion du cycle de vie (opening_control → opened → closing_control → closed)

**Contrat d'équipe :**
- Consomme : PosConfig (configuration POS), PosOrder (commandes de la session)
- Expose : `session.open`, `session.close`, `session.cash_in_out`

**Mandat de Permission requis :**
- Ouverture session : Mandat avec PosConfig + StrongFather (décision)
- Clôture session : Mandat avec StrongFather (décision) + KindMother (WriteIntent)
- Cash In/Out : Mandat avec PosSession

### 2.2 PosOrder

**Rôle :** Gestion des commandes (tickets) et des lignes (produits, quantités, prix, remises, taxes).

**Capacités :**
- Création/modification de commandes (draft)
- Ajout/suppression de lignes, remises, notes client
- Calcul des totaux (HT, TTC, taxes)
- Validation de commande (paid → done)
- Génération de facture (optionnel)
- Gestion des retours (remboursements)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation de commande, décision de facturation
- **KindMother** : Persistance des commandes et lignes (WriteIntent)
- **Master Butler** : Permissions de création/modification/validation
- **WorrySentinel** : Vérification modification commandes validées, niveau sécurité
- **Ever Buddy** : Gestion du cycle de vie (draft → paid → done → invoiced)

**Contrat d'équipe :**
- Consomme : PosSession (session active), PosPayment (paiements), MiyuStore (produits, prix), MiyuContacts (client)
- Expose : `order.create`, `order.update`, `order.validate`, `order.refund`, `order.invoice`

**Mandat de Permission requis :**
- Création commande : Mandat avec PosSession + MiyuStore (produits)
- Validation commande : Mandat avec PosOrder + PosPayment + StrongFather (décision) + KindMother (WriteIntent)
- Facturation : Mandat avec MiyuInvoice + StrongFather (décision)

### 2.3 PosPayment

**Rôle :** Gestion des paiements (montant, méthode) et rapprochement avec la caisse / les relevés.

**Capacités :**
- Enregistrement des paiements (espèces, carte, etc.)
- Rapprochement avec les lignes de relevé (caisse / bancaire)
- Gestion des remboursements (méthode de remboursement)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision de validation du paiement (si règles métier)
- **KindMother** : Persistance des paiements et des lignes de relevé (WriteIntent)
- **Master Butler** : Permissions par méthode de paiement
- **WorrySentinel** : Niveau sécurité 3 pour données paiement, audit des écarts de caisse

**Contrat d'équipe :**
- Consommé par : PosOrder
- Consomme : PosConfig (méthodes de paiement), MiyuTreasury ou MiyuBilling (relevés, selon périmètre)

**Mandat de Permission requis :**
- Enregistrement paiement : Mandat avec PosOrder + PosPayment
- Clôture session (validation relevé) : Mandat avec PosSession + StrongFather (décision) + KindMother (WriteIntent)

### 2.4 PosConfig

**Rôle :** Configuration des points de vente (nom, journal de caisse, méthodes de paiement, pricelist, etc.).

**Capacités :**
- Création/modification des configurations POS
- Gestion des méthodes de paiement
- Liaison pricelist, position fiscale, journal de caisse

**Niveau de sécurité :** 1-2 (Standard à Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des configurations
- **Master Butler** : Permissions de configuration (réservé responsable / admin)

**Contrat d'équipe :**
- Consommé par : PosSession, PosOrder, PosPayment
- Consomme : MiyuStore (pricelist), MiyuInvoice (journal factures), trésorerie (journaux caisse)

### 2.5 PosUI

**Rôle :** Interface utilisateur POS (écran de vente, panier, paiement, contrôle d'ouverture/clôture).

**Capacités :**
- Affichage du catalogue et du panier
- Saisie des quantités, remises, notes
- Écran de paiement (méthodes, montants)
- Ouverture/clôture de session (formulaires)
- Actions : New Order, Refund, Customer, Cash In/Out

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d'accès à l'interface par point de vente et rôle
- **WorrySentinel** : Vérification accès selon session et environnement

**Contrat d'équipe :**
- Consomme : PosSession, PosOrder, PosPayment, PosConfig
- Expose : écrans et actions utilisateur (pas d'autorité métier)

---

## 3. Contrat d'Équipe PosShopService

### 3.1 Flux Autorisés

1. **PosUI → PosSession** : Ouverture, clôture, Cash In/Out
2. **PosUI → PosOrder** : Création, modification, validation, remboursement, facturation
3. **PosOrder → PosPayment** : Enregistrement des paiements
4. **PosSession → PosOrder** : Commandes rattachées à la session
5. **PosOrder → MiyuStore** : Produits, prix (pricelist)
6. **PosOrder → MiyuContacts** : Client
7. **PosOrder → MiyuInvoice** : Génération facture

### 3.2 Mandats de Permission

**Mandat : Ouverture Session**
- Opérateurs : PosSession, PosConfig, StrongFather
- Flux : PosUI → PosSession → PosConfig
- Niveau sécurité max : 2

**Mandat : Vente et Paiement**
- Opérateurs : PosOrder, PosPayment, PosSession, MiyuStore, MiyuContacts
- Flux : PosUI → PosOrder → PosPayment → KindMother (WriteIntent)
- Niveau sécurité max : 3 (au moment du paiement)

**Mandat : Clôture Session**
- Opérateurs : PosSession, PosPayment, StrongFather, KindMother
- Flux : PosSession → PosPayment (validation relevé) → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 3

**Mandat : Facturation POS**
- Opérateurs : PosOrder, MiyuInvoice, StrongFather, KindMother
- Flux : PosOrder → MiyuInvoice (génération) → StrongFather (décision) → KindMother (WriteIntent)
- Niveau sécurité max : 3

---

## 4. Intégration avec les Cores

### 4.1 StrongFather

**Rôles :**
- Décision d'ouverture de session
- Décision de clôture de session (et clôture avec écart)
- Décision de validation de commande (done)
- Décision de génération de facture depuis le ticket

### 4.2 KindMother

**Rôles :**
- Persistance de toutes les données POS (sessions, commandes, lignes, paiements) via WriteIntent
- Autorité absolue sur les données

### 4.3 Master Butler

**Rôles :**
- Déclaration des capacités PosShopService
- Permissions par point de vente et par rôle (caissier, responsable)
- Isolation par environnement (multi-société / multi-POS)

### 4.4 WorrySentinel

**Rôles :**
- Niveau de sécurité : 2 (données caisse, commandes), 3 (paiements)
- Vérification des écarts de caisse et audit
- Isolation des données par point de vente

### 4.5 Ever Buddy

**Rôles :**
- Gestion du cycle de vie session (opening_control → opened → closing_control → closed)
- Gestion du cycle de vie commande (draft → paid → done → invoiced)
- Compatibilité des versions de configuration POS

---

## 5. Intégrations avec Autres Services Miyukini

### 5.1 MiyuStore

**Intégration :**
- Catalogue produits, pricelist, code-barres, taxes
- Calcul des prix et des taxes selon position fiscale

### 5.2 MiyuInvoice

**Intégration :**
- Génération de factures depuis les commandes POS
- Lien bidirectionnel commande POS ↔ facture

### 5.3 MiyuContacts

**Intégration :**
- Recherche et sélection du client
- Création rapide de client depuis le POS
- Pricelist et adresses pour facturation

### 5.4 MiyuTreasury / MiyuBilling

**Intégration :**
- Rapprochement caisse (lignes de relevé par méthode de paiement)
- Journaux de caisse et validation à la clôture

### 5.5 Miyukini Sales

**Intégration :**
- Encaissement de commandes Sales au POS
- Création de commandes Sales depuis le POS (si besoin)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
