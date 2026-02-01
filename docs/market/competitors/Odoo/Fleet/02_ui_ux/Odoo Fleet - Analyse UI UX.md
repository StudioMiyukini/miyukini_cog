# Odoo Fleet — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Fleet** (Flotte véhicules) d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, onglets, composants et patterns de navigation pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Fleet, Models & manufacturers, Adding vehicles, Services, Accidents, Cost analysis)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure de navigation et menus
- Vues Fabricants, Modèles, Catégories, Véhicules
- Formulaire véhicule (onglets et sections)
- Services et sinistres (liste, formulaire, Kanban)
- Contrats et alertes
- Rapports et analyse des coûts
- Configuration (Settings)
- Patterns d'interaction et feedback

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de Navigation

### 1.1 Menu principal Fleet

- **Fleet** (app racine)
  - **Dashboard / Vue véhicules** : Vue par défaut (liste ou Kanban selon configuration)
  - **Véhicules** : Liste et création de véhicules
  - **Services** : Liste des services (entretiens, réparations, sinistres)
  - **Contrats** : Liste des contrats (assurance, leasing)
  - **Rapports** : Cost analysis (Total costs, Cost by vehicle, Cost by driver, Detailed comparison)
- **Configuration**
  - **Settings** : End Date Contract Alert, New Vehicle Request (Belgique)
  - **Fabricants** : Référentiel fabricants (Manufacturers)
  - **Modèles** : Référentiel modèles (Models)
  - **Catégories** : Catégories de modèles (Categories)
  - **Types de service** : Types de service (Vidange, Révision, Accident, etc.)

### 1.2 Entrées secondaires

- Depuis une fiche véhicule : liens vers Modèle, Fabricant, Conducteur (contact), Contrat actif, Services et relevés odomètre.
- Depuis une fiche modèle : liens vers Fabricant, Catégorie, Fournisseurs (Contacts).
- Depuis une fiche service : liens vers Véhicule, Conducteur, Fournisseur, Type de service.

---

## 2. Vues Référentiels

### 2.1 Fabricants (Manufacturers)

- **Vue** : Tableau de bord / liste de cartes (cards).
- **Contenu** : Cartes par fabricant avec logo, nom et **nombre de modèles** configurés.
- **Filtre par défaut** : « With Models » (uniquement les fabricants ayant au moins un modèle). Option pour afficher tous les fabricants.
- **Tri** : Alphabétique.
- **Action** : New → formulaire fabricant (Name, Logo).
- **Interaction** : Clic sur une carte → liste des modèles du fabricant ou détail fabricant selon version.

### 2.2 Modèles (Models)

- **Vue** : Liste ou formulaire.
- **Accès** : Fleet → Configuration → Models → New.
- **Formulaire modèle** :
  - **En-tête** : Model name, Manufacturer, Vehicle Type (Car / Bike), Category.
  - **Onglet Information** :
    - **Model** : Model Year, Seating Capacity, Number of Doors, Color, Trailer Hitch.
    - **Salary** (Belgique) : Can be requested, Catalog Value (VAT Incl.), CO2 fee (auto), Cost (Depreciated), Total Cost (Depreciated).
    - **Engine** : Fuel Type, Range, CO2 Emissions, Emission Standard, Transmission, Power Unit, Power / Horsepower, Horsepower Taxation, Tax Deduction (si localisation).
  - **Onglet Vendors** : Liste de fournisseurs (Add Vendors → sélection ou création contact).
- **Règles d’affichage** : Champs Salary et Tax Deduction selon localisation ; CO2 fee en lecture seule (calculée).

### 2.3 Catégories (Model category)

- **Vue** : Liste (list view).
- **Contenu** : Lignes nom de catégorie ; ordre modifiable par glisser-déposer (icône draggable).
- **Action** : New → nouvelle ligne ; sauvegarde par Save ou clic hors champ.
- **Usage** : Sélection dans le formulaire modèle ; avec Inventory : champs Max Weight, Max Volume sur catégorie ou modèle pour capacité de chargement.

---

## 3. Formulaire Véhicule (fleet.vehicle)

### 3.1 En-tête / Général

- **Driver** : Conducteur (res.partner) — sélection ou « Create a new driver » (ouvre formulaire contact).
- **Vehicle** : Nom / libellé, Model (obligatoire), License Plate, VIN (vin_sn), Company.
- **Statut** : Actif / inactif (archivage).
- **Sauvegarde** : Bouton Save ; enregistrement automatique possible.

### 3.2 Onglets

#### Onglet Tax Info (Fiscality)

- Champs fiscaux selon localisation (taxe véhicule, déduction, etc.).

#### Onglet Contract

- Contrat actif (assurance, leasing) : type, dates, montant, responsable.
- Lien vers création / édition de contrat.
- Information sur prochaine échéance et alerte (selon paramètre End Date Contract Alert).

#### Onglet Model tab

- Données héritées du modèle (motorisation, carburant, places, etc.) ; possible surcharge ou affichage en lecture seule selon version.

#### Onglet Note tab

- Notes libres.
- Documents : pièces jointes (assurance, garantie, carnets d’entretien) — selon version (attachment ou champs binaires).

### 3.3 Zones liées (selon version)

- **Services** : Liste des services (entretiens, sinistres) du véhicule ; accès rapide à New service.
- **Odometer** : Historique des relevés kilométriques ; saisie nouveau relevé.
- **Contrats** : Historique des contrats du véhicule.

---

## 4. Services et Sinistres

### 4.1 Liste des services

- **Vue** : Liste (colonnes : Véhicule, Type de service, Date, Coût, Fournisseur, Conducteur, Odomètre, État).
- **Filtres** : Par véhicule, conducteur, type, date, fournisseur.
- **Actions** : New, Export (pivot, CSV).

### 4.2 Formulaire service

- **Champs** : Vehicle (obligatoire), Service Type (obligatoire), Date, Amount, Vendor, Driver, Odometer, Description, Notes.
- **État** : Selon workflow (brouillon, planifié, en cours, terminé).
- **Lien** : Depuis fiche véhicule (bouton « New service ») ou menu Fleet → Services → New.

### 4.3 Kanban services

- **Vue** : Cartes par stade (ex. New, In Progress, Done).
- **Contenu carte** : Véhicule, type, date, coût, fournisseur.
- **Interaction** : Glisser-déposer pour changer de stade.
- **Usage** : Suivi des interventions en cours et planification.

### 4.4 Sinistres (Accidents)

- **Structure** : Les sinistres sont des **services** avec un type de service « Accident » (ex. Accident - Driver's Fault, Accident - No Fault).
- **Service record** : Mêmes champs que tout service ; **Description** et **Notes** utilisés pour les détails du sinistre (lieu, circonstances, tiers).
- **Plusieurs réparations** : Plusieurs enregistrements de service avec les **mêmes Notes** pour lier au même sinistre.
- **Tableaux de bord** :
  - **Services dashboard** : Vue d’ensemble des services (filtres par véhicule, type, date).
  - **Reporting dashboard** : Vue sinistres (par faute, véhicule, conducteur, coût).
- **Accident reporting** : Filtres et regroupements pour analyser les sinistres par faute, véhicule, conducteur, coût.

---

## 5. Contrats

### 5.1 Liste des contrats

- **Vue** : Liste (colonnes : Véhicule, Type, Dates, Montant, Responsable, État).
- **Filtres** : À échéance, par véhicule, par responsable, par type.
- **Action** : New → formulaire contrat (véhicule, type, dates, montant, responsable).

### 5.2 Alertes fin de contrat

- **Mécanisme** : Email automatique au **responsible_id** (utilisateur) X jours avant **expiration_date** (paramètre Settings : End Date Contract Alert).
- **UI** : Indication sur la fiche contrat et/ou sur le tableau de bord (widget « Contrats à échéance ») selon version.
- **Action** : Lien depuis l’email ou le dashboard vers la fiche contrat pour renouveler ou clôturer.

---

## 6. Analyse des coûts (Cost analysis report)

### 6.1 Total costs

- **Vue** : Rapport / graphique des coûts totaux sur une période.
- **Filtres** : Période, société, véhicule, conducteur.
- **Contenu** : Agrégation contrats + services (montants).
- **Export** : Pivot, CSV.

### 6.2 Cost by vehicle

- **Vue** : Répartition des coûts par véhicule.
- **Usage** : Comparer le coût de revient par véhicule.
- **Export** : Pivot, CSV.

### 6.3 Cost by driver

- **Vue** : Répartition des coûts par conducteur.
- **Usage** : Analyse usage et coût par collaborateur.
- **Export** : Pivot, CSV.

### 6.4 Detailed comparison

- **Vue** : Comparaison détaillée (tableau croisé, filtres avancés).
- **Usage** : Analyse fine pour reporting et décision (renouvellement, réforme de véhicules).

---

## 7. Configuration (Settings)

### 7.1 Paramètres globaux

- **End Date Contract Alert** : Nombre de jours avant l’échéance du contrat pour envoi de l’email d’alerte au responsable (ex. 30).
- **New Vehicle Request** (Belgique) : Activation et limites sur les demandes de nouveau véhicule selon disponibilité du parc (option liée à la localisation et à Payroll).

### 7.2 Responsible parties

- Documenté dans la doc Odoo : désignation du responsable par contrat (responsible_id) ; ce responsable reçoit les alertes.

### 7.3 New Vehicle Request (Belgian Payroll - Fleet)

- Configuration des règles d’éligibilité (modèles avec « Can be requested ») et des limites de demande (parc, politique).

---

## 8. Patterns d’Interaction et Feedback

### 8.1 Création en cascade

- **Véhicule** : Si le modèle n’existe pas, l’utilisateur doit quitter le formulaire véhicule et aller dans Configuration → Models → New. Pas de création inline du modèle depuis la fiche véhicule dans la doc standard.
- **Conducteur** : « Create a new driver » ouvre le formulaire contact (Contacts) ; après sauvegarde, retour au champ Driver pour sélection.
- **Fournisseur** (sur modèle ou service) : Add Vendors → pop-up liste contacts ; bouton New pour créer un contact puis Select pour l’ajouter.

### 8.2 Feedback

- **Sauvegarde** : Message de confirmation ou auto-save (selon paramètre Odoo).
- **Alertes** : Email au responsable contrat ; pas de notification in-app standard documentée pour Fleet.
- **Validation** : Champs obligatoires (Model, Vehicle sur service, Service Type) ; messages d’erreur si champs manquants.

### 8.3 Navigation

- Breadcrumb : Fleet → Véhicules → [Nom véhicule].
- Liens relationnels : Clic sur Modèle → fiche modèle ; clic sur Conducteur → fiche contact (ou employé si lien).
- Retour liste : Bouton retour ou menu Fleet → Véhicules.

### 8.4 Cohérence avec le reste Odoo

- **Style** : Formulaire type Odoo (onglets, champs groupés, boutons Save / Discard).
- **Chatter** : Si module mail activé, zone Chatter (messages, activités) possible sur véhicule, contrat, service selon version.
- **Recherche** : Barre de recherche globale et filtres sur listes (véhicule, conducteur, date, type).

---

## 9. Synthèse pour Miyukini

### À reproduire / améliorer

- **Navigation claire** : Menu Fleet avec Véhicules, Services, Contrats, Rapports, Configuration (Fabricants, Modèles, Catégories, Types de service).
- **Formulaire véhicule** : En-tête (conducteur, modèle, immatriculation, VIN), onglets Tax Info, Contract, Model, Note ; zones liées Services et Odometer.
- **Services** : Liste + formulaire + Kanban par stade ; champs Vehicle, Service Type, Date, Amount, Vendor, Driver, Odometer, Description, Notes.
- **Sinistres** : Même modèle que service avec type Accident ; amélioration possible : modèle « Sinistre » dédié avec champs structurés et liaison à plusieurs lignes de réparation.
- **Contrats** : Liste avec filtre « À échéance » ; alerte email au responsable avec lien vers la fiche ; paramètre jours par type de contrat (évolution).
- **Coûts** : Vues Total costs, Cost by vehicle, Cost by driver, Detailed comparison ; export pivot/CSV ; liaison analytique si possible.
- **Configuration** : Settings (alertes, demande véhicule) ; référentiels Fabricants, Modèles, Catégories, Types de service avec création et ordre (catégories).
- **Conducteur** : Création rapide « Create a new driver » (contact) ; lien employé (Fleet Mobility Card) si module HR.

### Points d’attention

- Réduire la sortie du formulaire véhicule pour créer un modèle (suggestion : création de modèle en une étape depuis la fiche véhicule avec valeurs par défaut).
- Notifications in-app en plus de l’email pour les alertes contrat.
- Tableau de bord Fleet Manager : résumé véhicules actifs, contrats à échéance, prochains entretiens, coûts du mois.

---

**Document** : Odoo Fleet — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
