# Odoo Quality — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Quality** d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, formulaires et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Quality

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (Control Points, Quality Checks, Quality Alerts)
- Structure des formulaires (QCP, Check, Alert)
- Patterns de navigation (menu Quality, boutons sur MO/picking, Shop Floor)
- Widgets et champs clés
- Configuration (équipes, templates, Failure Locations)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Menu et Navigation

### 1.1 Menu principal Quality

**Accès :** Quality (app Supply Chain)

**Structure typique :**
- **Quality Control**
  - Control Points (liste / formulaire QCP)
  - Quality Checks (liste / formulaire contrôles)
  - Quality Alerts (Kanban / formulaire alertes)
- **Configuration**
  - Quality Teams
  - Quality Worksheet/Spreadsheet Templates
  - Failure Locations

### 1.2 Points d'entrée transverses

- **Manufacturing** : Sur un MO, bouton **Quality Checks** (pop-up ou page) ; bouton **Quality Alert** (si contrôles demandés pour ce MO).
- **Inventory** : Carte opération (Réceptions, Livraisons, etc.) → **# To Process** → sélection d'un ordre → sur le picking : **Quality Checks**, **Quality Alert** (ou ⚙️ ‣ Quality Alert).
- **Shop Floor** : Carte work order → étape contrôle (clic pour pop-up) ; menu ⋮ → **Create a Quality Alert**.

---

## 2. Vues — Quality Control Points (QCP)

### 2.1 Liste / Formulaire QCP

**Accès :** Quality ‣ Quality Control ‣ Control Points ‣ New

**Formulaire QCP — champs principaux :**
- **Title** : Titre unique
- **Products** : Produits concernés (optionnel)
- **Product Categories** : Catégories (optionnel)
- **Operations** : Opération(s) déclenchant le QCP (obligatoire ; ex. Manufacturing, Delivery)
- **Work Order Operation** : (si Manufacturing) opération de travail spécifique
- **Control Per** : Operation / Product / Quantity
- **Partial Transfer Test** + **Percentage** : (si Quantity) pourcentage d'unités à contrôler
- **Control Frequency** : All / Randomly / Periodically
- **Every #% of Operations** : (si Randomly)
- **Valeur + Days/Weeks/Months** : (si Periodically)
- **Type** : Instructions, Take a Picture, Register Production, Pass-Fail, Measure, Worksheet, Spreadsheet
- **Template** : (si Worksheet ou Spreadsheet) modèle à dupliquer
- **Team** : Équipe qualité responsable
- **Responsible** : Responsable du QCP
- **Step Document** : Specific Page of Operation Worksheet / Custom
- **Worksheet Page** : (si Step Document = Specific Page)
- **Onglet Instructions** : Instructions pour réaliser les contrôles
- **Onglet Message If Failure** : Instructions en cas d'échec (ex. créer une alerte)
- **Onglet Notes** : Notes internes (non visibles opérateurs)

**Widgets :**
- Listes multi-sélection (Products, Product Categories, Operations)
- Sélections (Control Per, Control Frequency, Type)
- Champs conditionnels (Work Order Operation, Template, Percentage, etc.)

---

## 3. Vues — Quality Checks

### 3.1 Liste Quality Checks

**Accès :** Quality ‣ Quality Control ‣ Quality Checks

**Usage :** Liste des contrôles avec filtres (état, type, équipe, ordre, produit).

### 3.2 Formulaire Quality Check

**Champs principaux :**
- **Control per** : Operation / Product / Quantity
- **Lot/Serial** : (si Quantity) lot ou série concerné
- **Picking** : Bon de mouvement (optionnel si Production Order renseigné)
- **Production Order** : MO (optionnel si Picking renseigné)
- **Control Point** : QCP d'origine (optionnel)
- **Type** : Instructions, Take a Picture, Print label, Pass-Fail, Measure, Worksheet
- **Quality Template** : (si Worksheet)
- **Measure** : (si type Measure) valeur à saisir
- **Team** : Équipe responsable
- **Company** : Société
- **Onglet Notes** : Instructions (pour l'opérateur) et Notes (internes)
- **Boutons Pass / Fail** (ou **Validate** selon le type) en haut de formulaire

**Interaction :**
- Clic **Pass** ou **Fail** pour clôturer le contrôle (Pass-Fail).
- **Validate** pour les autres types (après saisie mesure, photo, ou complétion worksheet).
- Coche à côté de l'étape (en contexte ordre / Shop Floor) peut marquer automatiquement **Passed**.

### 3.3 Pop-up Quality Check sur ordre (MO / Picking)

**Contexte :** Depuis un MO ou un picking, clic sur **Quality Checks**.

**Contenu :** Fenêtre pop-up listant tous les contrôles requis pour cet ordre. Pour chaque contrôle : instructions, champs à remplir (mesure, photo, etc.), boutons Pass / Fail ou Validate.

**Interaction :** Traiter les contrôles sans quitter l'ordre ; fermeture de la pop-up après validation.

### 3.4 Contrôle sur Work Order (Shop Floor)

**Contexte :** Shop Floor ‣ centre de travail ‣ carte du work order.

**Affichage :** Les étapes du work order incluent l'étape contrôle (créée par QCP). Clic sur l'étape → pop-up avec instructions du contrôle.

**Interaction :** Suivre les instructions, puis **Validate** ou **Pass** / **Fail**. Option : cocher la case à droite de l'étape pour marquer **Passed** automatiquement.

---

## 4. Vues — Quality Alerts

### 4.1 Kanban Quality Alerts

**Accès :** Quality ‣ Quality Control ‣ Quality Alerts

**Caractéristiques :**
- Colonnes = stages (ex. New, In Progress, Done / Resolved)
- Glisser-déposer pour changer de stage
- Priorité (1–3 étoiles) : alertes à priorité plus haute en haut de colonne
- Clic sur une carte : ouverture du formulaire
- Bouton + à droite du nom du stage : création rapide d'alerte dans ce stage

### 4.2 Formulaire Quality Alert

**Champs principaux :**
- **Title** : Titre court
- **Product** / **Product Variant** : Produit concerné
- **Work Center** : Centre de travail
- **Picking** : Bon concerné
- **Team** : Équipe responsable
- **Responsible** : Responsable
- **Tags** : Étiquettes
- **Root Cause** : Cause racine
- **Priority** : 1 à 3 étoiles
- **Onglet Description** : Description du problème
- **Onglet Corrective Actions** : Actions correctives
- **Onglet Preventive Actions** : Actions préventives
- **Onglet Miscellaneous** : Vendor, Company, Date Assigned

**Widgets :**
- Barre de stages en haut à droite (changement de stage sans Kanban)
- Chatter (si mail activé)

---

## 5. Configuration

### 5.1 Quality Teams

**Accès :** Quality ‣ Configuration ‣ Quality Teams

**Contenu :** Liste / formulaire (nom, membres, société).

### 5.2 Quality Worksheet/Spreadsheet Templates

**Accès :** Quality ‣ Configuration ‣ Quality Worksheet/Spreadsheet Templates

**Contenu :** Création et édition des templates utilisés par les QCP et contrôles de type Worksheet / Spreadsheet. Chaque contrôle créé duplique le template pour remplissage.

### 5.3 Failure Locations

**Accès :** Quality ‣ Configuration ‣ Failure Locations

**Contenu :** Référentiel des lieux de défaillance pour rapports et causes racine.

---

## 6. Patterns de Navigation Résumés

| Action | Entrée | Cible |
|--------|--------|-------|
| Créer un QCP | Quality ‣ Control Points ‣ New | Formulaire QCP |
| Voir / traiter les contrôles | Quality ‣ Quality Checks | Liste / formulaire Check |
| Traiter les contrôles d'un MO | Manufacturing ‣ MO ‣ Quality Checks | Pop-up contrôles |
| Traiter les contrôles d'un picking | Inventory ‣ # To Process ‣ picking ‣ Quality Checks | Pop-up contrôles |
| Traiter un contrôle sur WO | Shop Floor ‣ carte WO ‣ étape contrôle | Pop-up étape |
| Créer une alerte | Quality ‣ Quality Alerts ‣ New, ou MO/picking/Shop Floor ‣ Quality Alert | Formulaire Alerte |
| Gérer les alertes | Quality ‣ Quality Alerts | Kanban alertes |

---

## 7. Recommandations pour Miyukini

- **Vue agrégée « Mes contrôles à traiter »** : Liste ou tableau de bord regroupant les contrôles en attente (tous contextes : MO, picking, WO) pour éviter la dispersion des points d'entrée.
- **Indication claire du contexte** : Sur chaque contrôle, afficher explicitement « MO #… », « Picking #… » ou « Work Order … » pour faciliter le passage au contexte source.
- **Message If Failure visible** : Dans l’UI de traitement d’un contrôle, rappeler les instructions en cas d’échec (créer alerte, notifier, etc.) sans obliger à rouvrir le QCP.
- **Cohérence Pass / Fail / Validate** : Unifier le libellé et le comportement (bouton unique « Valider » avec choix Pass/Fail en sous-étape si besoin) pour réduire la charge cognitive.
- **Configuration progressive** : Wizard ou guide pour premier QCP (opération → type → fréquence → équipe) pour réduire la charge de configuration initiale.

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
