# Odoo Quality — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Quality** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Quality

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Responsable Qualité / Quality Manager

**Profil :**
- Rôle stratégique : Définition des points de contrôle, équipes qualité, templates et référentiels
- Responsabilités :
  - Créer et configurer les Quality Control Points (QCP)
  - Définir les équipes qualité et les responsables
  - Créer les templates (Worksheet, Spreadsheet) pour les contrôles
  - Configurer les lieux de défaillance (Failure Locations)
  - Consulter les rapports (taux de conformité, causes de défauts)
  - Gérer les alertes qualité (priorisation, assignation)

**Besoins :**
- Vue d'ensemble des contrôles (en attente, passés, échoués)
- Configuration des QCP par opération et par produit
- Rapports personnalisables sur la qualité
- Gestion des alertes en Kanban

**Permissions :**
- Quality Manager (Supply Chain ‣ Quality)
- Accès à la configuration (QCP, équipes, templates, Failure Locations)

### 1.2 Opérateur Qualité / Contrôleur

**Profil :**
- Rôle opérationnel : Réalisation des contrôles qualité sur le terrain ou en production
- Responsabilités :
  - Exécuter les contrôles qualité (Pass/Fail, Measure, Picture, Instructions, Worksheet)
  - Traiter les contrôles depuis un MO, un picking ou le Shop Floor
  - Saisir les mesures, joindre les photos, remplir les feuilles de travail
  - Valider (Pass) ou refuser (Fail) les contrôles
  - Créer une alerte qualité en cas d'échec (si configuré)

**Besoins :**
- Accès rapide aux contrôles à traiter (depuis l'ordre ou la liste Quality Checks)
- Instructions claires par type de contrôle
- Saisie simple (mesure, photo, case à cocher)
- Traitement depuis le bon contexte (MO, picking, Shop Floor)

**Permissions :**
- Quality User (ou équivalent)
- Accès aux ordres (Manufacturing, Inventory) pour lesquels des contrôles sont demandés
- Accès Shop Floor pour les contrôles sur work orders

### 1.3 Responsable Production / Manufacturing

**Profil :**
- Rôle transverse : Garantir que les contrôles sont réalisés au bon moment dans le flux de production
- Responsabilités :
  - Lancer les MO et s'assurer que les contrôles QCP sont créés
  - Traiter ou faire traiter les contrôles sur les MO et work orders
  - Créer une alerte qualité si un défaut est constaté pendant la production
  - Consulter les contrôles en attente sur les ordres

**Besoins :**
- Bouton « Quality Checks » visible sur les MO et pickings
- Pop-up ou page dédiée pour traiter les contrôles sans quitter l'ordre
- Création d'alerte depuis le Shop Floor (carte work order)

**Permissions :**
- Accès Manufacturing (et éventuellement Quality) pour voir et traiter les contrôles sur les MO
- Accès Shop Floor pour les contrôles sur work orders

### 1.4 Responsable Entrepôt / Inventory

**Profil :**
- Rôle transverse : Contrôles sur réceptions, livraisons, retours
- Responsabilités :
  - Traiter les contrôles sur les pickings (réceptions, livraisons)
  - Créer une alerte qualité si non-conformité à la réception ou à l'expédition
  - Utiliser « # To Process » pour sélectionner les ordres avec contrôles

**Besoins :**
- Bouton « Quality Checks » sur les pickings
- Création d'alerte depuis un picking (bouton ou menu ⚙️)
- Filtrage des ordres avec contrôles en attente

**Permissions :**
- Accès Inventory et Quality pour voir et traiter les contrôles sur les pickings

---

## 2. Parcours d'Onboarding

### 2.1 Premier déploiement (Responsable Qualité)

1. **Activation du module** : Installation de l'app Quality (dépendances : stock, mail ; optionnel : mrp).
2. **Configuration des équipes** : Quality ‣ Configuration ‣ Quality Teams ‣ Créer (nom, membres, société).
3. **Création des lieux de défaillance** : Configuration ‣ Failure Locations ‣ Créer (pour rapports et causes racine).
4. **Création des templates** : (si Worksheet/Spreadsheet) Configuration ‣ Quality Worksheet/Spreadsheet Templates ‣ New.
5. **Création des QCP** : Quality ‣ Quality Control ‣ Control Points ‣ New (titre, opérations, produits/catégories, Control Per, Control Frequency, type de contrôle, équipe, instructions, message en cas d'échec).
6. **Droits utilisateurs** : Settings ‣ Users ‣ Access Rights ‣ Supply Chain ‣ Quality (Quality User / Quality Manager).

### 2.2 Premier contrôle automatique (QCP Manufacturing)

1. Configurer un QCP pour l'opération **Manufacturing**, produits ou catégorie concernés, Control Per (Operation/Product/Quantity), Control Frequency (All/Randomly/Periodically), type (ex. Pass-Fail).
2. Confirmer un ordre de fabrication (MO) concerné.
3. Vérifier que les contrôles qualité sont créés et visibles sur le MO (bouton Quality Checks).
4. Traiter les contrôles depuis le MO (Pass/Fail ou Validate).

### 2.3 Premier contrôle sur picking

1. Configurer un QCP pour une opération stock (ex. Receipt, Delivery).
2. Créer ou valider un picking concerné.
3. Cliquer sur « # To Process » sur la carte Réceptions (ou Livraisons), sélectionner le bon.
4. Sur le picking, ouvrir Quality Checks et traiter les contrôles.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Contrôle automatique sur MO (QCP)

1. Responsable Qualité crée un QCP : Manufacturing, produit « Table », Control Per = Product, Control Frequency = All, type Pass-Fail.
2. Production confirme un MO pour 10 tables.
3. 10 contrôles qualité sont créés (un par produit).
4. Opérateur ouvre le MO, clique sur Quality Checks, traite chaque contrôle (Pass/Fail).
5. Si Fail : selon configuration, création d'alerte ou instructions dans « Message If Failure ».

### 3.2 Contrôle sur work order (Shop Floor)

1. QCP configuré pour une **Work Order Operation** spécifique (ex. Assembly pour produit Coffee Table).
2. MO confirmé pour Coffee Table ; work order Assembly a une étape contrôle.
3. Opérateur ouvre Shop Floor, centre de travail concerné, carte du work order.
4. À l'étape contrôle : clic sur l'étape → pop-up avec instructions → Validate ou Pass/Fail.
5. Option : menu ⋮ sur la carte → Create a Quality Alert (Product et Work Center pré-remplis).

### 3.3 Contrôle manuel ponctuel

1. Opérateur va dans Quality ‣ Quality Control ‣ Quality Checks ‣ New.
2. Sélectionne Control per (Operation/Product/Quantity), Picking ou Production Order, type de contrôle, équipe, instructions.
3. Traite le contrôle (Pass/Fail ou Validate selon le type).

### 3.4 Alerte qualité depuis un picking

1. Sur un picking avec contrôles demandés : constat d'un défaut.
2. Clic sur Quality Alert (ou ⚙️ ‣ Quality Alert si le bouton n'est pas affiché).
3. Formulaire : titre, produit, picking, équipe, responsable, priorité, cause racine, description, actions correctives/préventives.
4. Alerte visible dans Quality ‣ Quality Control ‣ Quality Alerts (Kanban).

### 3.5 Reporting et suivi

1. Responsable Qualité ouvre Quality ‣ rapports (selon version : tableau de bord, statut des contrôles, causes de défauts).
2. Filtrage par produit, opération, équipe, période.
3. Analyse des Failure Locations et causes racine pour actions préventives.

---

## 4. Points de Friction Identifiés

- **Visibilité du bouton Quality Alert** : Sur MO/picking, le bouton n'apparaît que s'il existe déjà des contrôles pour l'ordre ; sinon il faut passer par le menu ⚙️ ou l'app Quality. Risque de confusion.
- **Work order vs MO** : Les contrôles pour une opération de travail ne peuvent être créés que par un QCP ; pas de création manuelle pour un WO spécifique. Courbe d'apprentissage.
- **Templates Worksheet/Spreadsheet** : Obligation de créer et maintenir les templates en configuration ; champ Template obligatoire pour ces types. Charge de configuration initiale.
- **Multi-contexte** : Traitement possible depuis Quality Checks, MO, picking ou Shop Floor ; les utilisateurs doivent savoir où aller selon le type d'ordre.
- **Control Frequency « Periodically »** : Compréhension du pas de temps (jours/semaines/mois) et de l'effet sur la création des contrôles.

---

## 5. Recommandations pour Miyukini

- **Service Quality découplé** : Opérateurs Quality (QCP, Check, Alert, Team, FailureLocation) avec Contrats d'équipe pour Manufacturing et Inventory ; pas de couplage fort aux ordres.
- **Mandats** : Création de contrôle (auto ou manuelle) et création d'alerte via Mandats ; contrôle d'accès par équipe et par type d'opération.
- **Parcours unifié** : Proposer un point d'entrée « Mes contrôles à traiter » (agrégé MO + picking + WO) en plus du traitement dans le contexte de l'ordre.
- **Message d'échec** : Modéliser « Message If Failure » (QCP) comme règle gouvernée (ex. création automatique d'alerte, notification) sans logique dispersée dans l'UI.
- **Documentation utilisateur** : Clarifier dans l'UI quand créer une alerte depuis l'ordre vs depuis l'app Quality, et quand le contrôle est créé par QCP vs manuel.

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
