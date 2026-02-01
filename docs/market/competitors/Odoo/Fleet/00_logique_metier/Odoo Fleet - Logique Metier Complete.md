# Odoo Fleet — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Fleet** (Flotte véhicules) d'Odoo (version 19.0), à partir de la documentation officielle et du modèle HR. Il identifie les modèles de données, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module `fleet` (Fleet Management, Human Resources)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (fabricants, modèles, catégories, véhicules, contrats, services, accidents)
- Règles métier et contraintes (assignation conducteur, alertes fin de contrat, coûts)
- Workflows (demande véhicule, enregistrement véhicule, maintenance, sinistres)
- Analyse des coûts (par véhicule, par conducteur, par période)
- Gestion des documents (assurance, garantie, entretien)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle Fabricants (fleet.vehicle.model.brand)

**Rôle :** Référentiel des fabricants de véhicules (marques). Odoo Fleet est livré avec environ 67 fabricants préconfigurés (voitures et vélos), avec logos.

**Champs clés :**
- `name` : Nom du fabricant
- `image_128` / logo : Image / logo du fabricant
- Modèles associés (one2many vers fleet.vehicle.model)

**Règles métier :**
- Un fabricant doit exister avant de créer un modèle.
- Filtre par défaut « With Models » : n'affiche que les fabricants ayant au moins un modèle.
- Les fabricants sont listés par ordre alphabétique ; chaque carte affiche le nombre de modèles configurés.

### 1.2 Modèle Modèles de véhicules (fleet.vehicle.model)

**Rôle :** Référentiel des modèles de véhicules (ex. BMW X2, Audi A3). Contrairement aux fabricants, les modèles ne sont **pas** préconfigurés ; ils doivent être ajoutés à la base.

**Champs clés :**

#### Général
- `name` : Nom du modèle
- `brand_id` : Fabricant (obligatoire)
- `vehicle_type` : Type de véhicule — **fixe** : `car` ou `bike` (pas d’ajout possible ; conservé pour intégration Payroll)
- `categ_id` : Catégorie de modèle (fleet.vehicle.model.category)

#### Onglet Information (Model)
- `model_year` : Année du modèle
- `seats` : Nombre de places
- `doors` : Nombre de portes
- `color` : Couleur
- `trailer_hitch` : Attache remorque (booléen)

#### Onglet Salary (localisation Belgique)
- `can_be_requested` : Peut être demandé par les employés (véhicule de fonction)
- `catalog_value` : Valeur catalogue TTC (MSRP)
- `co2_fee` : Taxe CO2 (calculée automatiquement selon réglementation belge, basée sur CO2 Emissions)
- `cost_depreciated` : Coût mensuel déprécié (impacte salaire brut/net dans configurateur paie)
- `total_cost_depreciated` : Coût total déprécié (cost_depreciated + co2_fee)

#### Onglet Engine
- `fuel_type` : Type de carburant (Diesel, Gasoline, Full Hybrid, Plug-in Hybrid Diesel/Gasoline, CNG, LPG, Hydrogen, Electric)
- `range` : Autonomie (km ou mi)
- `co2` : Émissions CO2 (g/km ou g/mi)
- `emission_standard` : Norme (EPA, Euro 6, etc.)
- `transmission` : Manual / Automatic
- `power_unit` : kW ou Horsepower
- `power` : Puissance (kW si power_unit = kW)
- `horsepower` : Puissance (hp) — affiché si power_unit = Horsepower
- `horsepower_taxation` : Fiscalité selon puissance (local)
- `tax_deduction` : Pourcentage déductible (localisation, non modifiable)

#### Onglet Vendors
- `vendor_ids` : Fournisseurs auprès desquels le véhicule peut être acheté (liens vers res.partner) — permet de créer des demandes d’achat (Purchase / RFQ)

**Règles métier :**
- Un véhicule ne peut être ajouté qu’à partir d’un modèle (et fabricant) existant.
- Les types `car` et `bike` sont fixes pour l’intégration Paie (avantages en nature).
- Coût déprécié : ne se déprécie pas sur le modèle, mais sur le **contrat** lié au véhicule.
- CO2 fee (Belgique) : calculée automatiquement à partir du champ CO2 Emissions ; non modifiable manuellement.

### 1.3 Catégorie de modèles (fleet.vehicle.model.category)

**Rôle :** Catégorisation des modèles (taille, usage, etc.). Aucune catégorie n’est préconfigurée ; toutes doivent être créées.

**Champs clés :**
- `name` : Nom de la catégorie
- Ordre d’affichage (liste déplaçable)

**Règles métier :**
- Utilisée pour organiser les modèles et les véhicules.
- Avec l’app Inventory : champs `max_weight` et `max_volume` pour la capacité de chargement (livraisons).

### 1.4 Modèle Véhicule (fleet.vehicle)

**Rôle :** Représente un véhicule physique du parc (acquisition, immatriculation, assignation conducteur, coûts, documents).

**Champs clés (synthèse) :**

#### Général / Identification
- `name` : Libellé / identification du véhicule
- `driver_id` : Conducteur assigné (res.partner — peut être lié à un employé)
- `model_id` : Modèle (fleet.vehicle.model) — obligatoire
- `license_plate` : Immatriculation
- `vin_sn` : Numéro de châssis (VIN)
- `company_id` : Société (multi-company)
- `active` : Actif (soft delete)

#### Fiscality (Tax Info)
- Champs fiscaux selon localisation (taxe véhicule, déduction, etc.)

#### Contract (Contrat en cours)
- Lien vers le contrat actif (fleet.vehicle.log.contract) — assurance, leasing, etc.
- Alertes fin de contrat : paramètre « End Date Contract Alert » (nombre de jours avant échéance) ; notification par email au responsable du contrat.

#### Model tab
- Données héritées ou surchargées du modèle (motorisation, carburant, etc.)

#### Note tab
- Notes libres, documents (assurance, garantie, historique entretien)

**Règles métier :**
- Le conducteur peut être un contact (res.partner) ; si Employees est installé, lien possible avec hr.employee.
- Un véhicule a un seul conducteur assigné à la fois.
- Responsable du contrat : personne recevant les alertes d’échéance.
- New Vehicle Request (option Belgique / Payroll) : limites sur les demandes de nouveau véhicule selon disponibilité du parc.

### 1.5 Contrat véhicule (fleet.vehicle.log.contract)

**Rôle :** Contrats liés au véhicule (assurance, leasing, LOA, etc.). Supervision des échéances et alertes automatiques.

**Champs clés :**
- `vehicle_id` : Véhicule
- `type` : Type de contrat (ex. assurance, leasing)
- `company_id` : Société
- `amount` : Montant (mensuel ou selon type)
- `start_date` / `expiration_date` : Début et fin de contrat
- `responsible_id` : Responsable (utilisateur) — reçoit les alertes « fin de contrat »
- `state` : État (en cours, expiré, etc.)
- `cost_generated` : Coût enregistré (lien éventuel avec Accounting)

**Règles métier :**
- **End Date Contract Alert** (Configuration) : nombre de jours avant l’échéance pour envoi d’un email d’alerte au responsable.
- Un seul contrat actif par type (ou règle métier équivalente selon version).
- Les coûts contrats sont distingués des coûts « services » dans les rapports d’analyse des coûts.

### 1.6 Services / Entretiens (fleet.vehicle.log.services)

**Rôle :** Enregistrement des interventions (entretien, réparation, remplacement pneus, etc.) et des sinistres.

**Champs clés :**
- `vehicle_id` : Véhicule
- `driver_id` : Conducteur (au moment de l’intervention)
- `service_type_id` : Type de service (fleet.service.type) — obligatoire
- `amount` : Coût
- `vendor_id` : Fournisseur (res.partner)
- `date` : Date de l’intervention
- `odometer` : Kilométrage / odomètre
- `description` : Description des travaux
- `notes` : Notes (ex. détails sinistre)
- `state` : État (brouillon, effectué, etc.) — selon workflow

**Types de service :**
- Créés par l’administrateur (ex. « Vidange », « Révision », « Accident - Faute du conducteur », « Accident - Sans faute »).
- Les sinistres sont gérés via des types de service dédiés (Accident - Driver's Fault, Accident - No Fault) ; même enregistrement que les autres services (description, notes, coût, fournisseur).

**Règles métier :**
- Service Type et Vehicle sont obligatoires.
- Les coûts « services » sont distincts des coûts « contrats » dans l’analyse des coûts.
- Vue Kanban possible par stade (planifié, en cours, terminé).
- Plusieurs réparations (plusieurs fournisseurs) pour un même sinistre : plusieurs enregistrements de service avec les mêmes Notes pour tracer l’unicité du sinistre.

### 1.7 Enregistrement odomètre (fleet.vehicle.odometer)

**Rôle :** Relevés de kilométrage (ou équivalent pour véhicules électriques). Permet de suivre l’usage et d’alerter sur les prochains entretiens.

**Champs clés :**
- `vehicle_id` : Véhicule
- `driver_id` : Conducteur ayant saisi / concerné
- `value` : Valeur odomètre
- `date` : Date du relevé

**Règles métier :**
- Historique des valeurs pour calcul d’usage (km parcourus sur période) et comparaison avec plannings d’entretien.

### 1.8 Demande de véhicule (Belgian Payroll / New Vehicle Request)

**Rôle :** Lorsque la localisation Belgique et l’option « New Vehicle Request » sont activées, les employés peuvent demander un véhicule de fonction. Les limites sont définies selon la disponibilité du parc (paramètre dans Configuration).

**Règles métier :**
- Modèles avec `can_be_requested` = true (onglet Salary du modèle) sont éligibles.
- Workflow : demande → validation → attribution véhicule (ou refus).

---

## 2. Règles Métier et Contraintes

### 2.1 Fabricants et modèles

- **Fabricants** : 67 environ préchargés ; ajout possible (nom, logo).
- **Modèles** : Aucun préchargé ; création obligatoire avant d’ajouter un véhicule. Dépendance : fabricant (et éventuellement catégorie).
- **Type véhicule** : Uniquement `car` et `bike` ; non extensible (compatibilité Paie).

### 2.2 Véhicules et conducteurs

- Un véhicule a **un seul conducteur** (driver_id = res.partner). Si Employees est installé, le conducteur peut être le contact travail de l’employé (Fleet Mobility Card sur fiche employé).
- Assignation : choix du conducteur sur la fiche véhicule ; historique implicite via les services / odomètre (driver_id sur chaque enregistrement).

### 2.3 Contrats et alertes

- **End Date Contract Alert** : paramètre global (Configuration) en nombre de jours avant échéance.
- Le **responsable** du contrat (responsible_id) reçoit l’email d’alerte.
- Les coûts des contrats sont suivis séparément des coûts services pour les rapports.

### 2.4 Services et sinistres

- Sinistres : créés comme **services** avec un type de service « Accident » (ex. avec faute / sans faute). Description et notes décrivent le sinistre ; plusieurs lignes de service peuvent partager les mêmes notes pour une même déclaration avec plusieurs fournisseurs.
- Coûts : montant, fournisseur, véhicule, conducteur — alimentation des rapports « coût par véhicule » et « coût par conducteur ».

### 2.5 Coûts et analyse

- **Total des coûts** : agrégation sur une période (contrats + services).
- **Coût par véhicule** : répartition des coûts par véhicule.
- **Coût par conducteur** : répartition par conducteur (via services et éventuellement contrats selon configuration).
- **Comparaison détaillée** : export possible (pivot, rapports) pour analyse.
- Intégration **Accounting** : les coûts Fleet peuvent être exportés ou reliés à la comptabilité (dépend de la version et des modules).

### 2.6 Documents

- Dépôt de documents sur le véhicule (assurance, garantie, carnets d’entretien) — onglet Note ou Documents selon version.
- Pas de modèle dédié « document » dans la description standard ; stockage en pièce jointe (ir.attachment) ou champs binaires selon implémentation Odoo.

---

## 3. Workflows

### 3.1 Ajout d’un véhicule au parc

1. Vérifier / créer le **fabricant** (Configuration → Fabricants).
2. Créer le **modèle** (Configuration → Modèles) : nom, fabricant, type car/bike, catégorie, informations (motorisation, fiscalité si Belgique, fournisseurs).
3. Créer le **véhicule** (Fleet → Véhicules → New) : modèle, immatriculation, VIN, conducteur, contrat éventuel, notes/documents.
4. Saisir les **relevés odomètre** et les **services** au fil du temps.

### 3.2 Gestion des contrats

1. Création du contrat (type, dates, montant, responsable).
2. En cours : suivi des échéances.
3. Alerte automatique (email) au responsable X jours avant expiration (paramètre End Date Contract Alert).
4. Renouvellement ou clôture : nouveau contrat ou fin d’assignation.

### 3.3 Entretien et réparation

1. Définir les **types de service** (Configuration) si besoin (ex. Vidange, Révision, Accident - Faute conducteur).
2. Créer un **enregistrement de service** : véhicule, type, date, coût, fournisseur, odomètre, description, notes.
3. Workflow Kanban : états (ex. planifié → en cours → terminé) selon configuration.
4. Relevés odomètre réguliers pour suivi et planification des prochains entretiens.

### 3.4 Sinistre

1. Créer un type de service « Accident » (avec ou sans faute).
2. Créer une fiche **service** avec ce type ; renseigner description, notes (détails sinistre), coût, fournisseur, véhicule, conducteur.
3. Si plusieurs réparations (plusieurs fournisseurs) : plusieurs fiches service avec les **mêmes notes** pour garder le lien au même sinistre.
4. Consultation : filtres par véhicule, conducteur, faute, coût (rapports et tableaux de bord).

### 3.5 Demande de véhicule (Belgique)

1. Paramétrage : Configuration → New Vehicle Request (limites selon parc) ; modèles avec « Can be requested » coché.
2. Employé : demande d’un modèle éligible.
3. Validation (RH / Fleet manager) : acceptation ou refus.
4. Si accepté : attribution d’un véhicule (ou commande) et liaison conducteur (employé).

---

## 4. Calculs et Indicateurs

### 4.1 Coûts totaux

- Somme des montants des **contrats** (sur période) + somme des montants des **services** (sur période).
- Filtres : par véhicule, par conducteur, par société, par date.

### 4.2 Coût par véhicule

- Agrégation des coûts (contrats + services) par véhicule.
- Utile pour comparer le coût de revient par véhicule.

### 4.3 Coût par conducteur

- Agrégation par conducteur (via services et, selon règles, contrats assignés au conducteur).
- Utile pour analyse usage et coût par collaborateur.

### 4.4 Fiscalité (Belgique)

- **CO2 fee** : calculée automatiquement à partir des émissions CO2 du modèle (réglementation belge).
- **Cost (Depreciated)** / **Total Cost (Depreciated)** : dépréciation selon le **contrat** lié au véhicule, pas sur le modèle seul ; impact sur le salaire (configurateur Paie).

---

## 5. Synthèse pour Miyukini

### Entités métier à couvrir

| Entité Odoo | Rôle | Équivalent Miyukini cible |
|-------------|------|----------------------------|
| fleet.vehicle.model.brand | Fabricants | Référentiel Fabricants (Fleet) |
| fleet.vehicle.model | Modèles véhicules | Référentiel Modèles (Fleet) |
| fleet.vehicle.model.category | Catégories modèles | Catégories Modèles |
| fleet.vehicle | Véhicules | Véhicules (parc) |
| fleet.vehicle.log.contract | Contrats (assurance, leasing) | Contrats véhicule |
| fleet.vehicle.log.services | Services / entretiens / sinistres | Services et sinistres |
| fleet.vehicle.odometer | Relevés kilométrage | Relevés odomètre |
| fleet.service.type | Types de service | Types de service / sinistre |

### Règles à reproduire

- Hiérarchie Fabricant → Modèle → Véhicule ; types car/bike fixes si intégration Paie.
- Un conducteur par véhicule ; lien possible employé (MiyuHR).
- Alertes fin de contrat (paramètre jours, responsable, email).
- Séparation coûts contrats vs services ; rapports par véhicule, par conducteur, par période.
- Sinistres comme services typés (Accident) avec description/notes.
- Option « demande de véhicule » (limites parc, modèles éligibles) si besoin localisation Belgique / avantage en nature.

---

**Document** : Odoo Fleet — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
