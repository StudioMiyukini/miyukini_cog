# Odoo Rental — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Rental** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Rental
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **RentalOrderOperator** | Gestion des commandes et lignes de location | Opérateur de Service |
| **RentalPricingOperator** | Tarification location (grilles, calcul prix, pénalités) | Opérateur de Service |
| **RentalStockOperator** | Disponibilité, Security Time, emplacements Rental In/Out | Opérateur de Service |
| **RentalUI** | Interface utilisateur Location (commandes, planning, reçus) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : RentalService

**Définition :**
> **RentalService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de location de produits.**

**Composition :**
- RentalOrderOperator (niveau sécurité 2)
- RentalPricingOperator (niveau sécurité 1–2)
- RentalStockOperator (niveau sécurité 2)
- RentalUI (niveau sécurité 1)

**Contrat d'équipe :**
- Flux autorisés : RentalUI → RentalOrderOperator ; RentalOrderOperator ↔ RentalPricingOperator, RentalStockOperator
- Types d'échanges : création/modification commande, calcul prix, vérification disponibilité, enlèvement/retour
- Validation StrongFather pour création/modification commande et enregistrement enlèvement/retour
- Persistance via KindMother (WriteIntent)

---

## 2. Opérateurs Détaillés

### 2.1 RentalOrderOperator

**Rôle :** Gestion des commandes de location et des lignes (devis, confirmation, enlèvement, retour, facturation).

**Capacités :**
- Création et modification de commandes de location (devis)
- Confirmation de commande (déclenchement livraison/réception et tâches)
- Enregistrement enlèvement et retour effectifs
- Suivi des statuts (draft, confirmed, pickup, return, invoiced)
- Lien avec facturation (lignes location + pénalités)
- Lien optionnel avec signature (contrat location)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification commande, confirmation, enlèvement, retour
- **KindMother** : Persistance des commandes et lignes (WriteIntent)
- **Master Butler** : Permissions création/modification commande, enlèvement, retour
- **WorrySentinel** : Niveau sécurité (données clients, engagements financiers)
- **Ever Buddy** : Cycle de vie commande (états, prolongations, annulations)

**Contrat d'équipe :**
- Consomme : RentalPricingOperator (calcul prix), RentalStockOperator (disponibilité, mouvements), MiyuInvoice (facturation), MiyuContacts (client), MiyuSign ou équivalent (signature)
- Expose : `rental_order.create`, `rental_order.update`, `rental_order.confirm`, `rental_order.pickup`, `rental_order.return`, `rental_order.invoice`

**Mandat de Permission requis :**
- Création commande : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + RentalPricingOperator (calcul)
- Confirmation : Mandat avec RentalStockOperator (disponibilité, mouvements) + StrongFather (décision)
- Enlèvement / Retour : Mandat avec RentalStockOperator (mouvements) + KindMother (WriteIntent)
- Facturation : Mandat avec MiyuInvoice + KindMother (WriteIntent)

### 2.2 RentalPricingOperator

**Rôle :** Tarification location (grilles par produit, calcul du prix, pénalités).

**Capacités :**
- Configuration des grilles de prix par produit (unité de temps, durée, prix)
- Calcul du prix pour une période donnée (règle : une ligne, option la moins chère)
- Calcul des pénalités (Extra Hour, Extra Day) en cas de retard
- Configuration Extra Hour, Extra Day, Security Time par produit

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données produits)

**Gouvernance :**
- **StrongFather** : Décision de modification des grilles (optionnel selon politique)
- **KindMother** : Persistance des configurations produit (WriteIntent)
- **Master Butler** : Permissions lecture/écriture grilles
- **WorrySentinel** : Niveau sécurité données tarifaires

**Contrat d'équipe :**
- Consommé par : RentalOrderOperator, RentalUI
- Consomme : Données produit (MiyuStore ou équivalent)
- Expose : `rental_pricing.compute_price`, `rental_pricing.get_penalties`, `rental_pricing.get_config`

**Mandat de Permission requis :**
- Calcul prix : Mandat avec RentalOrderOperator (contexte commande)
- Modification grille : Mandat avec KindMother (WriteIntent) + StrongFather (décision) si applicable

### 2.3 RentalStockOperator

**Rôle :** Disponibilité, Security Time, emplacements Rental In / Rental Out, mouvements.

**Capacités :**
- Gestion des emplacements Rental In et Rental Out par entrepôt
- Vérification disponibilité (quantités Rental In, Security Time, chevauchements)
- Création des mouvements à la confirmation (livraison → Rental Out) et au retour (réception → Rental In)
- Suivi des quantités louées et disponibles

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de réserver créneau, valider mouvement
- **KindMother** : Persistance des mouvements et réservations (WriteIntent)
- **Master Butler** : Permissions lecture/écriture stock location
- **WorrySentinel** : Cohérence stock, pas de surréservation

**Contrat d'équipe :**
- Consommé par : RentalOrderOperator
- Consomme : Stock général (MiyuStore / Inventory)
- Expose : `rental_stock.check_availability`, `rental_stock.reserve`, `rental_stock.create_pickup_movement`, `rental_stock.create_return_movement`

**Mandat de Permission requis :**
- Vérification disponibilité : Mandat avec RentalOrderOperator
- Création mouvements : Mandat avec KindMother (WriteIntent) + StrongFather (décision) à la confirmation et au retour

### 2.4 RentalUI

**Rôle :** Interface utilisateur Location (commandes, planning, reçus, signature).

**Capacités :**
- Formulaire commande location (lignes, dates, prix, statuts)
- Vue planning des locations (calendrier / timeline par produit)
- Actions : Sign Documents, Print Pickup and Return Receipt
- Liste des enlèvements et retours planifiés
- Configuration produits (grilles, pénalités, Security Time) et paramètres Rental

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision déléguée via RentalOrderOperator
- **Master Butler** : Permissions d'affichage et d'action selon rôle (commercial, magasinier, gestionnaire)
- **WorrySentinel** : Pas d'exposition de données sensibles hors mandat

**Contrat d'équipe :**
- Consomme : RentalOrderOperator, RentalPricingOperator, RentalStockOperator, MiyuSign (optionnel)
- Expose : écrans et actions utilisateur (pas d'API métier directe)

**Mandat de Permission requis :**
- Toute action passant par BondingBrother avec Mandat couvrant RentalOrderOperator (ou Pricing/Stock selon action)

---

## 3. Contrats d'Équipe et Mandats

### 3.1 Contrat d'équipe RentalService

**Opérateurs membres :** RentalOrderOperator, RentalPricingOperator, RentalStockOperator, RentalUI

**Flux autorisés :**
- RentalUI → RentalOrderOperator (création, modification, confirmation, enlèvement, retour)
- RentalOrderOperator → RentalPricingOperator (calcul prix, pénalités)
- RentalOrderOperator → RentalStockOperator (disposition, mouvements)
- RentalOrderOperator → MiyuInvoice (facturation)
- RentalOrderOperator → MiyuSign (signature contrat, si activé)

**Types de données échangeables :** Commandes, lignes, dates, prix, statuts, mouvements, identifiants facture/signature

**Conditions préalables :** Mandat de Permission valide émis par StrongFather pour la session / l'action

**Niveau de validation :** StrongFather pour toute écriture (commande, mouvement, facture)

### 3.2 Mandats typiques

| Action | Opérateurs impliqués | Mandat |
|--------|----------------------|--------|
| Créer devis location | RentalOrderOperator, RentalPricingOperator | Create rental order |
| Confirmer commande | RentalOrderOperator, RentalStockOperator | Confirm rental order |
| Enregistrer enlèvement | RentalOrderOperator, RentalStockOperator | Register pickup |
| Enregistrer retour | RentalOrderOperator, RentalStockOperator, RentalPricingOperator (pénalités) | Register return |
| Facturer | RentalOrderOperator, MiyuInvoice | Invoice rental |
| Demander signature | RentalOrderOperator, MiyuSign | Request signature |
| Configurer produit louable | RentalPricingOperator | Configure rental product |

---

## 4. Niveaux de Sécurité

### 4.1 Par opérateur

- **RentalOrderOperator** : 2 (Sensitive) — données clients, engagements, montants
- **RentalPricingOperator** : 1–2 — grilles tarifaires (1) ; données stratégiques (2) si applicable
- **RentalStockOperator** : 2 (Sensitive) — mouvements stock, cohérence inventaire
- **RentalUI** : 1 (Standard) — interface, pas de persistance directe

### 4.2 Règles

- Un flux ne peut pas descendre en niveau de sécurité (données Sensitive ne pas exposer en Standard)
- Ponts entre Opérateurs explicites, validés par WorrySentinel
- Audit des actions Rental (création commande, confirmation, enlèvement, retour, facturation)

---

## 5. Intégration avec les Cores

### 5.1 StrongFather

- Décision création/modification commande location
- Décision confirmation (après vérification disponibilité)
- Décision enlèvement et retour (validation mouvement)
- Validation Mandats pour RentalService

### 5.2 KindMother

- WriteIntent pour : commandes, lignes, mouvements stock location, lignes facture (pénalités)
- Aucune écriture directe ; tout passe par WriteIntent validé

### 5.3 Master Butler

- Permissions : `rental_order.create`, `rental_order.update`, `rental_order.confirm`, `rental_order.pickup`, `rental_order.return`, `rental_pricing.configure`, `rental_stock.check`, `rental_stock.move`
- Capacités déclarées pour chaque Opérateur Rental

### 5.4 WorrySentinel

- Niveau de sécurité par type de donnée (commande, client, stock, facture)
- Vérification pas de surréservation, pas de mouvement incohérent
- Audit des retards et pénalités

### 5.5 Ever Buddy

- Cycle de vie commande (draft → confirmed → pickup → return → invoiced)
- Gestion prolongations et annulations (compatibilité, dépréciation de champs si besoin)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
