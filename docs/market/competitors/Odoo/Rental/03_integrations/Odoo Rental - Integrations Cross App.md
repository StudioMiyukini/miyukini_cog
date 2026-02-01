# Odoo Rental — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Rental** d'Odoo : dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation et architecture Odoo Rental (sale_rental)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (Sales, Stock, Sign, Invoicing)
- Flux de données inter-apps
- Mécanismes d'intégration (extensions de modèles, wizards, automatisations)
- Événements et hooks
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (sale_rental) :**
- **sale** : Commandes de vente (sale.order, sale.order.line)
- **sale_stock** (ou équivalent) : Lien vente–stock, livraisons et réceptions
- **stock** : Entrepôts, emplacements, mouvements (Rental In, Rental Out)
- **sales_team** (selon versions) : Équipes de vente

**Sans ces modules :** L'app Rental ne peut pas fonctionner (pas de commandes, pas de stock location).

### 1.2 Modules Optionnels

**Dépendances optionnelles (fonctionnalités conditionnelles) :**
- **sign** : Documents numériques (contrat de location, signature client)
  - Si activé : option « Digital Documents » dans Paramètres Rental, modèle « Rental Agreement »
  - Installation possible à la demande lors de l'activation de Digital Documents
- **project** / **mail** : Tâches ou activités pour enlèvement/retour planifiés (selon implémentation)
- **account** / **sale** : Facturation des locations et pénalités (déjà couvert par sale + account)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Sales (sale)

**Flux :**
```
Rental → sale.order / sale.order.line (extension)
         → Devis, confirmation, statuts, montants
```

**Mécanismes :**
- Extension du modèle **sale.order** : champs et comportements spécifiques location (dates enlèvement/retour, statut location, lien tâches)
- Extension du modèle **sale.order.line** : dates début/fin, durée, prix calculé location, pénalités, statut ligne location
- Réutilisation du workflow Sales : brouillon → envoyé → confirmé
- Actions additionnelles : Sign Documents, Print Pickup and Return Receipt
- Calcul des montants : prix selon grille location (règle « option la moins chère ») ; pénalités au retour

**Champs / comportements ajoutés (conceptuels) :**
- Sur commande : statut location, dates globales ou dérivées des lignes
- Sur ligne : start_date, end_date, duration, rental_price, delay_costs (pénalités), state (planned, picked_up, returned, invoiced)

**Recommandations pour Miyukini :**
- Opérateur Location qui s'appuie sur le flux « Commande » (MiyuSales ou équivalent) pour devis/confirmation
- Contrat d'équipe : RentalOrderOperator ↔ Opérateur Vente (création commande, mise à jour statut)
- Mandats : création/modification commande location avec permission Vente + Location

### 2.2 Intégration avec Stock / Warehouse (sale_stock, stock)

**Flux :**
```
Rental → Emplacements Rental In / Rental Out
       → Mouvements à la confirmation (livraison) et au retour (réception)
       → Disponibilité pour nouvelles locations (Security Time)
```

**Mécanismes :**
- **Emplacements spécifiques** par entrepôt :
  - **Rental In** : stock disponible à la location
  - **Rental Out** : stock actuellement chez le client (loué)
- À la **confirmation** de la commande location : génération automatique des livraisons (sortie Rental In → client / Rental Out)
- Au **retour** : génération des réceptions (client / Rental Out → Rental In)
- Contrôle de **disponibilité** : pas de double réservation sur la même période ; prise en compte du Security Time entre deux commandes
- Quantités : suivi des unités en Rental In vs Rental Out pour éviter surréservation

**Hooks / automatisations (conceptuels) :**
- On confirm order : créer picking (delivery) pour les lignes location, dates planifiées selon start_date
- On return registered : créer picking (incoming) pour les lignes concernées
- Contrôle stock avant confirmation : vérifier quantités Rental In suffisantes

**Recommandations pour Miyukini :**
- Opérateur Stock Location (ou extension MiyuStore / Inventory) pour emplacements Rental In/Out et mouvements
- WriteIntent pour création/mise à jour des mouvements (KindMother)
- Vérification disponibilité (Security Time, chevauchements) avant validation commande (StrongFather ou service dédié)

### 2.3 Intégration avec Invoicing (account, sale)

**Flux :**
```
Rental → Lignes de facture (location + pénalités)
       → Statut facturation commande / lignes
```

**Mécanismes :**
- Facturation des **lignes de location** au prix calculé (grille location)
- Facturation des **pénalités** (Extra Hour / Extra Day) après retour effectif
- Statut de facturation sur la commande et les lignes (partiellement facturé, totalement facturé)
- Réutilisation des flux Sale : facture depuis commande, facture différée (après retour), etc.

**Champs liés (conceptuels) :**
- Lien sale.order.line → account.move.line (facture)
- Champs ou lignes dédiés pour « delay costs » (pénalités) sur la facture

**Recommandations pour Miyukini :**
- Intégration avec MiyuInvoice : facturation des lignes location et des pénalités
- Mandat : facturation location avec permission Invoice + Rental
- Traçabilité : commande location ↔ facture(s)

### 2.4 Intégration avec Sign (sign)

**Flux :**
```
Rental → Paramètres : Digital Documents, modèle Rental Agreement
       → Action « Sign Documents » sur commande
       → Envoi document au client → Signature → Document complété
```

**Mécanismes :**
- **Paramètres Rental** : activer « Digital Documents », choisir le modèle Sign « Rental Agreement » (ou en créer un)
- **Bouton « Sign Documents »** sur la commande : ouverture wizard Sign (sélection modèle, sélection signataire = client)
- **Workflow Sign** : envoi par email → client signe → « Validate & Send Completed Document » côté back-office
- Document final attaché (commande ou partenaire) ; option : signature obligatoire avant enlèvement
- Si Sign n'est pas installé : proposition d'installation lors de l'activation de Digital Documents

**APIs / hooks (conceptuels) :**
- Création demande de signature depuis la commande (référence commande, client, modèle)
- Callback ou mise à jour statut quand le document est signé (pour débloquer enlèvement si règle métier)

**Recommandations pour Miyukini :**
- Intégration optionnelle avec un Opérateur Signature (équivalent Sign) : envoi contrat, suivi signature, attachement du document
- Mandat : « Demander signature » avec permission Signature + Rental
- Pas de dépendance obligatoire : location possible sans signature électronique (reçu papier ou pas de contrat signé)

### 2.5 Intégration avec Project / Tasks (project, mail)

**Flux (selon implémentation Odoo) :**
```
Rental → Tâches ou activités planifiées (enlèvement, retour)
       → Liste des enlèvements/retours à la date du jour
```

**Mécanismes :**
- À la confirmation : création de **tâches** (ou activités) pour chaque enlèvement et chaque retour planifié
- Lien tâche ↔ commande / ligne location (dates, produit, client)
- Utilisation pour : liste « À enlever aujourd'hui », « À retourner cette semaine », rappels
- Complément possible : assignation aux magasiniers, suivi de réalisation (enlèvement/retour effectué)

**Recommandations pour Miyukini :**
- Option : créer des « activités » ou tâches planifiées (MiyuJobs, MiyukiniProject ou équivalent) pour enlèvement/retour
- Lien explicite commande location ↔ tâche (référence, dates)
- Pas obligatoire pour le MVP : enlèvement/retour peuvent être enregistrés directement sur la commande

---

## 3. Flux de Données Résumés

### 3.1 Création et confirmation commande location

```
Utilisateur (Commercial)
  → Crée/édite sale.order + lignes location (dates, produits)
  → Confirme la commande
Rental
  → Calcule prix (grille location)
  → Vérifie disponibilité (stock Rental In, Security Time, chevauchements)
  → Crée livraison(s) (stock → Rental Out)
  → Crée tâches/activités enlèvement et retour
Sales / Stock
  → Enregistrent commande et mouvements
```

### 3.2 Enlèvement et retour

```
Utilisateur (Magasinier) / Système
  → Enregistre enlèvement effectif
  → Valide livraison (mouvement stock confirmé)
  → Enregistre retour effectif
  → Calcule pénalités (Extra Hour / Day) si retard
  → Valide réception (mouvement Rental Out → Rental In)
Rental / Stock
  → Mise à jour statuts commande et lignes
  → Mise à jour quantités Rental In et Rental Out
```

### 3.3 Facturation

```
Utilisateur (Commercial) / Automatisation
  → Génère facture depuis commande location
  → Inclut lignes location + lignes pénalités (si retard)
Account / Sale
  → Créent account.move, lignes de facture
  → Mise à jour statut facturation commande
```

### 3.4 Signature (si Sign activé)

```
Utilisateur (Commercial)
  → Clique « Sign Documents » sur commande
Rental / Sign
  → Ouvrent wizard Sign (modèle Rental Agreement, client)
  → Envoi document au client
Client
  → Signe dans Sign
Back-office
  → Validate & Send Completed Document
  → Document attaché à la commande / partenaire
```

---

## 4. Recommandations pour Miyukini

### 4.1 Dépendances

- **Obligatoires** : Opérateur type « Commande » (Vente) + Opérateur Stock (emplacements Rental In/Out, mouvements)
- **Optionnelles** : Opérateur Signature (contrat location), Opérateur Projet/Tâches (planning enlèvement/retour)
- **Facturation** : Opérateur Facturation (MiyuInvoice) pour lignes location et pénalités

### 4.2 Contrats d'équipe

- **RentalService** : RentalOrderOperator, RentalPricingOperator, RentalStockOperator, RentalUI
- **Contrats** : RentalOrderOperator ↔ Vente (commandes), RentalOrderOperator ↔ Stock (mouvements), RentalOrderOperator ↔ Invoice (facturation), RentalOrderOperator ↔ Signature (contrat)

### 4.3 Mandats

- Création/modification commande location : Mandat Vente + Location
- Enlèvement / retour : Mandat Stock + Location (ou rôle magasinier)
- Facturation : Mandat Invoice + Location
- Demande signature : Mandat Signature + Location
- Configuration produits et paramètres : Mandat avec niveau sécurité adapté (gestionnaire location)

### 4.4 APIs et événements

- **Événements à publier** : commande confirmée, enlèvement enregistré, retour enregistré, facture créée, document signé
- **APIs** : calcul prix location (entrée : produit, dates ; sortie : prix, règle utilisée), vérification disponibilité (entrée : produit, période ; sortie : disponible ou conflit)
- **WriteIntent** : commandes, lignes, mouvements stock, lignes facture (KindMother)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
