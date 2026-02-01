# Odoo Rental — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Rental** (Location) d'Odoo. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo Rental (14.0–18.0), app `sale_rental`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données (commandes de location, lignes de location, tarification, planning)
- Règles métier et contraintes (tarification, disponibilité, pénalités)
- Workflows (devis → confirmé → enlèvement → retour → facturation)
- Gestion des stocks « Rental In » / « Rental Out »
- Calcul des prix de location (règles temps, option la moins chère)
- Pénalités (heures/jours supplémentaires, retard)
- Intégrations Sales, Inventory, Sign, Invoicing

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Extension `sale.order` (Commande de vente / Location)

**Rôle :** Une commande de vente peut contenir des lignes de **location**. Les champs et comportements spécifiques location sont ajoutés au modèle `sale.order`.

**Champs / comportements clés (conceptuels) :**
- Statut de location : suivi global (draft, confirmed, pickup, return, invoiced)
- Dates d’enlèvement et de retour planifiées
- Lien avec tâches planifiées (enlèvement / retour)
- Génération automatique des livraisons / réceptions (stock) à la confirmation
- Suivi des produits enlevés vs retournés

### 1.2 Lignes de commande / Lignes de location

**Rôle :** Chaque ligne de commande peut être une **ligne de location** avec dates de début et fin, et tarification au temps.

**Champs clés (conceptuels) :**
- `product_id` : Produit loué (doit être « louable »)
- `start_date` / `end_date` (ou équivalent) : Période de location
- Unité de temps : heure, jour, semaine, mois
- Durée calculée et prix selon les règles de tarification location
- Pénalités : heure/jour supplémentaire, retard
- Statut de la ligne : planifié, enlevé, retourné, facturé

### 1.3 Produit et tarification location (`product.template` / `product.product`)

**Rôle :** Seuls les produits marqués **« Can be Rented »** et avec **tarification location** sont proposés à la location.

**Configuration location :**
- **Can be Rented** : Boolean (produit louable)
- **Rental Pricing** : Lignes de prix par unité de temps (heure, jour, semaine, mois) et par durée
- **Réservations / pénalités** :
  - **Extra Hour** : pénalité par heure supplémentaire
  - **Extra Day** : pénalité par jour supplémentaire
  - **Security Time** : durée (heures) de blocage du produit entre deux locations (indisponibilité)
- Règle de calcul : **une seule ligne de prix utilisée**, **option la moins chère** pour couvrir la durée

**Exemple de calcul (documentation Odoo) :**
- 1 jour : 100 €, 3 jours : 250 €, 1 semaine : 500 €
- Location 8 jours → Odoo choisit 3× « 3 jours » = 750 € (option la moins chère)

### 1.4 Stock et emplacements location

**Rôle :** Deux emplacements spécifiques par entrepôt pour la location.

**Emplacements :**
- **Rental In** : stock disponible à la location (produits « à louer »)
- **Rental Out** : stock actuellement chez le client (produits « loués »)

**Règles :**
- À la confirmation de la commande location : mouvements automatiques (livraison vers client → Rental Out ; retour → Rental In)
- Génération automatique des bons de livraison et des réceptions selon les dates d’enlèvement et de retour
- Disponibilité : prise en compte du Security Time entre deux locations

### 1.5 Tâches planifiées (enlèvement / retour)

**Rôle :** Création automatique de **tâches** (ou activités) pour les enlèvements et retours planifiés.

**Comportement :**
- Une tâche (ou activité) par enlèvement planifié
- Une tâche (ou activité) par retour planifié
- Lien avec la commande / la ligne de location pour le suivi

### 1.6 Documents et signatures (optionnel, avec Sign)

**Rôle :** Accord de location (contrat) signé par le client avant enlèvement.

**Comportement :**
- Activation dans Paramètres Rental : « Digital Documents »
- Modèle de document Sign : « Rental Agreement » (configurable)
- Workflow : Demande de signature → client signe → document validé
- Option : signature obligatoire avant enlèvement

### 1.7 Reçu d’enlèvement et de retour

**Rôle :** Impression d’un **reçu PDF** (Pickup and Return Receipt) par commande.

**Contenu typique :**
- État des articles : enlevés / à retourner / retournés
- Dates prévues d’enlèvement et de retour
- Coûts de retard éventuels

---

## 2. Workflows et Transitions d’État

### 2.1 Workflow commande de location

```
Devis (Draft)
  → Confirmé (Confirmed)  [génération livraison/réception, tâches enlèvement/retour]
  → Enlèvement (Pickup)   [enregistrement enlèvement effectif]
  → Retour (Return)       [enregistrement retour effectif]
  → Facturé (Invoiced)
```

**Transitions clés :**
- **Confirm** : création des mouvements de stock (Rental Out / Rental In selon dates), création des tâches enlèvement/retour
- **Pickup** : enregistrement de l’enlèvement réel (optionnel : signature si Sign activé)
- **Return** : enregistrement du retour réel, calcul des pénalités (heures/jours supplémentaires)
- **Invoice** : facturation (ligne location + éventuelles pénalités)

### 2.2 Workflow ligne de location

- **Planifié** : dates saisies, prix calculé
- **Enlevé** : produit sorti (Rental Out)
- **Retourné** : produit rentré (Rental In), pénalités calculées si retard
- **Facturé** : ligne et pénalités facturées

---

## 3. Règles Métier et Contraintes

### 3.1 Tarification

- **Une seule ligne de prix** utilisée par calcul (pas de mélange de lignes).
- **Prix minimum** : choix de la combinaison la moins chère pour couvrir la durée (ex. 8 jours → 3× « 3 jours »).
- **Unités de temps** : heure, jour, semaine, mois.
- **Durée** : calculée à partir de `start_date` et `end_date` dans l’unité configurée.

### 3.2 Disponibilité et Security Time

- **Security Time** (heures) : entre deux commandes, le produit est indisponible pendant cette durée (nettoyage, contrôle).
- Blocage des créneaux concernés pour les nouvelles réservations.
- Contrôle des chevauchements : pas de double location sur la même période pour un même produit (avec prise en compte du Security Time).

### 3.3 Pénalités

- **Extra Hour** : montant par heure au-delà de la fin prévue.
- **Extra Day** : montant par jour au-delà de la fin prévue.
- Calcul au moment du retour effectif ; lignes de facturation dédiées possibles.

### 3.4 Contraintes produit

- Produit **stockable** (gestion stock activée).
- **Can be Rented** coché.
- **Rental Pricing** renseignée (au moins une ligne par unité de temps/durée).

### 3.5 Stock

- Mouvements **automatiques** à la confirmation (vers Rental Out) et au retour (vers Rental In).
- Quantités cohérentes entre Rental In, Rental Out et commandes confirmées.

---

## 4. Intégrations avec Autres Modules

### 4.1 Sales (`sale`)

- Rental s’appuie sur **sale.order** et **sale.order.line**.
- Workflow : devis → confirmation → livraison / réception.
- Prix et remises gérés dans le cadre vente.

### 4.2 Stock / Warehouse (`sale_stock`, `stock`)

- **Rental In** et **Rental Out** : emplacements spécifiques par entrepôt.
- Génération automatique des **procurements** / mouvements à la confirmation et au retour.
- Suivi des quantités louées et disponibles.

### 4.3 Invoicing (`account`, `sale`)

- Facturation des lignes de location et des pénalités.
- Statut de facturation sur la commande / les lignes.

### 4.4 Sign (`sign`)

- Option **Digital Documents** : contrat de location (Rental Agreement).
- Workflow : envoi → signature client → validation.
- Peut être requis avant enlèvement.

### 4.5 Project / Tasks (si utilisé)

- Tâches ou activités pour **enlèvement** et **retour** planifiés.
- Lien avec la commande pour suivi opérationnel.

---

## 5. Considérations pour Miyukini COG

### 5.1 Opérateurs proposés (aperçu)

- **RentalOrderOperator** : gestion des commandes et lignes de location.
- **RentalPricingOperator** : règles de tarification et calcul des prix.
- **RentalStockOperator** : disponibilité, Security Time, emplacements Rental In/Out.
- **RentalUI** : interface location (devis, planning, enlèvement, retour, reçus).

### 5.2 Gouvernance COG

- **StrongFather** : autorisation création/modification commande location, validation signature.
- **KindMother** : persistance commandes, lignes, mouvements stock via WriteIntent.
- **Master Butler** : droits sur création/modification commandes, enlèvement, retour.
- **WorrySentinel** : niveau de sécurité (données clients, engagements financiers).
- **Ever Buddy** : cycle de vie commande (états, prolongations, annulations).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
