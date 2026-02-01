# Odoo Quality — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Quality** d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les modèles de données, règles métier, workflows, types de contrôles et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, Quality (Supply Chain / Quality)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Points de contrôle qualité (Quality Control Points — QCP)
- Contrôles qualité (Quality Checks)
- Alertes qualité (Quality Alerts)
- Lieux de défaillance (Failure Locations)
- Types de contrôles (Instructions, Pass-Fail, Measure, Picture, Worksheet, Spreadsheet)
- Règles de fréquence et de déclenchement (Control Per, Control Frequency)
- Intégration Manufacturing (MO, WO) et Inventory (picking, livraison, réception)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Quality Control Points (QCP) — Points de contrôle qualité

**Rôle :** Créer **automatiquement** des contrôles qualité à des intervalles prédéfinis. Les QCP sont configurés pour des opérations spécifiques (fabrication, livraison, réception, etc.) et éventuellement pour des produits ou catégories de produits.

**Champs clés :**

#### Identification
- `title` : Titre du point de contrôle (identifiant unique lisible)
- `company_id` : Société (multi-société)

#### Périmètre
- **Operations** : Opération(s) déclenchant le QCP (Manufacturing, Delivery, Receipt, etc.)
- **Work Order Operation** : (si Manufacturing) opération de travail spécifique (ex. Assembly) plutôt que l’ordre de fabrication entier
- **Products** : Un ou plusieurs produits concernés (optionnel ; si vide = tous les produits de l’opération)
- **Product Categories** : Catégorie(s) de produits (optionnel)

#### Fréquence et granularité
- **Control Per** : Sélection (Operation, Product, Quantity)
  - **Operation** : Un contrôle pour l’opération entière
  - **Product** : Un contrôle par produit **unique** dans l’opération (ex. une table + quatre chaises = deux contrôles)
  - **Quantity** : Un contrôle pour un pourcentage ou une quantité d’unités ; option « Partial Transfer Test » avec champ **Percentage**
- **Control Frequency** : Sélection (All, Randomly, Periodically)
  - **All** : Un contrôle à chaque fois que les conditions sont remplies
  - **Randomly** : Contrôle aléatoire sur un pourcentage d’opérations (**Every #% of Operations**)
  - **Periodically** : Contrôle à intervalle fixe (jours, semaines, mois)

#### Type de contrôle
- **Type** : Type de contrôle qualité à exécuter (Instructions, Take a Picture, Register Production, Pass-Fail, Measure, Worksheet, Spreadsheet)
- **Template** : (si Worksheet ou Spreadsheet) modèle de feuille / spreadsheet à dupliquer pour chaque contrôle

#### Équipe et responsable
- **Team** : Équipe qualité responsable du QCP et des contrôles créés
- **Responsible** : Membre de l’équipe responsable du QCP

#### Documentation
- **Step Document** : Emplacement du document d’instruction (Specific Page of Operation Worksheet / Custom)
- **Worksheet Page** : (si Step Document = Specific Page) numéro de page
- **Instructions** (onglet) : Instructions pour réaliser les contrôles créés par le QCP
- **Message If Failure** (onglet) : Instructions en cas d’échec (ex. créer une alerte qualité)
- **Notes** (onglet) : Notes internes (non visibles par les opérateurs)

**Règles métier :**
- Au moins une **opération** doit être renseignée. Products et Product Categories peuvent être vides (alors le QCP s’applique à toutes les opérations du type choisi).
- Un QCP de type **Instructions** est équivalent à une étape sur un ordre de travail (work order) ; ajouter une étape sur un WO crée un QCP dans l’app Quality.
- Pour Worksheet / Spreadsheet, un **template** est obligatoire et doit être rempli pour valider le contrôle.

---

### 1.2 Quality Checks — Contrôles qualité

**Rôle :** Inspections **manuelles** réalisées par les employés pour garantir la qualité des produits. Un contrôle peut porter sur un produit unique ou sur plusieurs produits au sein d’une même opération (ordre de fabrication, bon de livraison, réception, etc.).

**Création :**
- **Automatique** : via un QCP lorsque les conditions du QCP sont remplies
- **Manuelle** : Quality ‣ Quality Control ‣ Quality Checks ‣ New

**Champs clés :**

#### Périmètre
- **Control per** : Operation / Product / Quantity
  - **Operation** : Contrôle pour l’opération entière et tous les produits
  - **Product** : Contrôle pour chaque unité d’un produit dans l’opération
  - **Quantity** : Contrôle pour une quantité (ex. un contrôle pour 5 unités) ; champ **Lot/Serial** si Quantity
- **Picking** : Opération stock (réception, livraison, retour, etc.) concernée
- **Production Order** : Ordre de fabrication (MO) concerné
- **Control Point** : QCP d’origine (optionnel, pour rattacher un contrôle manuel à un QCP)

#### Type et contenu
- **Type** : Instructions, Take a Picture, Print label, Pass-Fail, Measure, Worksheet (avec **Quality Template**)
- **Measure** : (si type Measure) valeur mesurée à saisir
- **Team** : Équipe qualité responsable
- **Company** : Société propriétaire du produit inspecté
- **Instructions** (Notes) : Instructions pour réaliser le contrôle
- **Notes** : Informations internes (créateur, raison, etc.)

#### Workflow
- **État** : En attente → Passé / Échoué (boutons Pass / Fail ou Validate selon le type)

**Règles métier :**
- Les contrôles créés par un QCP apparaissent sur l’ordre (MO ou picking) ; l’opérateur est invité à les réaliser.
- Pour un **work order** (opération de travail), le contrôle ne peut être créé que par un QCP (pas de création manuelle pour une opération de travail spécifique) ; le traitement se fait dans le module Shop Floor.
- Pass / Fail ou Validate selon le type de contrôle ; cocher la case à côté de l’étape peut marquer automatiquement le contrôle comme **Passed**.

---

### 1.3 Quality Alerts — Alertes qualité

**Rôle :** Notifier les équipes qualité des **défauts ou incidents** constatés sur les produits. Une alerte peut être créée depuis l’app Quality, depuis un MO, depuis un picking, ou depuis le Shop Floor (carte work order).

**Champs clés :**

#### Identification
- **Title** : Titre court résumant le problème
- **Company** : (multi-société)

#### Contexte
- **Product** / **Product Variant** : Produit concerné
- **Work Center** : Centre de travail concerné
- **Picking** : Bon de mouvement (réception, livraison, etc.) concerné

#### Gestion
- **Team** : Équipe qualité responsable
- **Responsible** : Responsable de l’alerte
- **Tags** : Étiquettes pour filtrage
- **Root Cause** : Cause racine du problème (si connue)
- **Priority** : 1 à 3 étoiles (impact sur l’ordre dans le Kanban)

#### Contenu
- **Description** (onglet) : Description du problème
- **Corrective Actions** (onglet) : Actions correctives à mener
- **Preventive Actions** (onglet) : Actions préventives pour éviter la récurrence
- **Miscellaneous** (onglet) : Vendor, Company, Date Assigned

**Workflow (Kanban) :**
- Étapes typiques : Nouvelle alerte → En cours → Résolue / Clôturée (selon configuration)

**Règles métier :**
- Création depuis un MO ou un picking : le bouton « Quality Alert » n’apparaît que si au moins un **contrôle qualité** est demandé pour cet ordre.
- Depuis le Shop Floor : menu ⋮ sur la carte work order → « Create a Quality Alert » (Product et Work Center pré-remplis).

---

### 1.4 Failure Locations — Lieux de défaillance

**Rôle :** Classification des **lieux** où une non-conformité peut survenir (pour reporting et analyse des causes). Utilisés pour structurer les causes de défaillance et les rapports qualité.

**Usage :** Référentiel pour les alertes et les analyses de cause (root cause, lieu de défaillance). Permet des rapports et tableaux de bord sur les défauts par lieu.

---

## 2. Types de Contrôles Qualité

| Type | Description | Critère de succès |
|------|-------------|-------------------|
| **Instructions** | Instructions pas à pas pour réaliser le contrôle | Suivi des instructions puis validation |
| **Take a Picture** | Photo du produit obligatoire | Pièce jointe photo avant complétion |
| **Print label** | Impression d’étiquettes (pop-up) ; peut inclure instructions de placement | Validation de l’étape |
| **Pass - Fail** | Critère binaire (pass / fail) | Clic Pass ou Fail |
| **Measure** | Mesure à saisir ; comparée à une valeur de référence et tolérance | Valeur dans la plage tolérée = Pass |
| **Worksheet** | Feuille de travail interactive à remplir (template) | Template complété |
| **Spreadsheet** | Tableur interactif à remplir (template) | Template complété |
| **Register Production** | Confirmation de la quantité produite (contexte manufacturing) | Quantité confirmée |

**Measure :** Valeur norm (norm value), tolérance (min/max ou pourcentage). La mesure saisie doit être dans la plage pour que le contrôle soit réussi.

---

## 3. Métriques et Reporting

- **Statut des contrôles** : Passed, Failed, en attente
- **Taux de conformité** : par produit, par opération, par équipe, par période
- **Alertes** : par priorité, par cause racine, par lieu de défaillance
- **Rapports personnalisables** : statut des contrôles, causes de défauts

---

## 4. Droits d’Accès

- **Quality User** : Création et traitement des contrôles, création d’alertes, vue des ordres avec contrôles
- **Quality Manager** : Configuration des QCP, équipes qualité, templates, lieux de défaillance, paramètres, rapports

---

## 5. Intégrations Métier

### 5.1 Manufacturing (MRP)

- QCP sur opération **Manufacturing** : création de contrôles à la confirmation des MO (ou selon fréquence).
- **Work Order Operation** : QCP ciblant une opération de travail précise (ex. Assembly) ; les contrôles sont traités dans le **Shop Floor**.
- Bouton **Quality Checks** sur le MO : pop-up listant les contrôles à réaliser.
- Bouton **Quality Alert** sur le MO (si au moins un contrôle demandé pour ce MO).

### 5.2 Inventory (Stock)

- QCP sur opérations **Receipt**, **Delivery**, **Return**, etc. : création de contrôles sur les pickings.
- **# To Process** sur une carte d’opération (Réceptions, Livraisons, etc.) : sélection d’un bon puis traitement des contrôles.
- Bouton **Quality Checks** sur le picking ; bouton **Quality Alert** (ou via menu ⚙️ si pas de contrôle préalable).

### 5.3 Mail

- Chatter, followers, activités sur contrôles et alertes (si module mail activé).

---

## 6. Synthèse pour Miyukini

**Entités à modéliser :**
- Quality Control Point (QCP) : périmètre (opérations, produits, catégories), fréquence (Control Per, Control Frequency), type de contrôle, équipe, instructions, message en cas d’échec.
- Quality Check : lien opération (MO / picking / WO), type de contrôle, état (pending, passed, failed), mesure (si type Measure), template (si Worksheet/Spreadsheet).
- Quality Alert : titre, produit, contexte (MO, picking, work center), équipe, responsable, priorité, cause racine, description, actions correctives/préventives, workflow.
- Quality Team : équipe dédiée à des produits ou opérations.
- Failure Location : référentiel des lieux de défaillance.

**Règles à préserver :**
- QCP = seule source de contrôles pour les **work orders** ; création manuelle possible pour MO et pickings.
- Control Per (Operation / Product / Quantity) et Control Frequency (All / Randomly / Periodically).
- Types de contrôle : Instructions, Pass-Fail, Measure, Picture, Worksheet, Spreadsheet, Register Production, Print label.
- Alertes créables depuis Quality, MO, picking ou Shop Floor ; visibilité du bouton Quality Alert conditionnée par l’existence de contrôles sur l’ordre.

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
