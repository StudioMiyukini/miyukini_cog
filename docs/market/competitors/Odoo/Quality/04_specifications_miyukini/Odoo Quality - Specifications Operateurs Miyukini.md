# Odoo Quality — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Quality** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent Quality
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **QualityControlPointOperator** | Gestion des points de contrôle qualité (QCP) | Opérateur de Service |
| **QualityCheckOperator** | Gestion des contrôles qualité (création, traitement) | Opérateur de Service |
| **QualityAlertOperator** | Gestion des alertes qualité | Opérateur de Service |
| **QualityTeamOperator** | Gestion des équipes qualité | Opérateur de Service |
| **FailureLocationOperator** | Gestion des lieux de défaillance | Opérateur de Service |
| **QualityMetricsOperator** | Calculs et rapports (taux conformité, causes) | Opérateur de Service |
| **QualityUI** | Interface utilisateur Quality | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : QualityService

**Définition :**
> **QualityService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de contrôle qualité (points de contrôle, contrôles, alertes, équipes et référentiels).**

**Composition :**
- QualityControlPointOperator (niveau sécurité 2)
- QualityCheckOperator (niveau sécurité 2)
- QualityAlertOperator (niveau sécurité 2)
- QualityTeamOperator (niveau sécurité 2)
- FailureLocationOperator (niveau sécurité 2)
- QualityMetricsOperator (niveau sécurité 2, lecture / calcul)
- QualityUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 QualityControlPointOperator

**Rôle :** Gestion des points de contrôle qualité (QCP) : création, configuration (opérations, produits, fréquence, type de contrôle, équipe, instructions, message en cas d'échec).

**Capacités :**
- Création / modification de QCP
- Configuration : opérations (Manufacturing, Receipt, Delivery, etc.), Work Order Operation (si Manufacturing), produits et catégories
- Control Per (Operation / Product / Quantity), Control Frequency (All / Randomly / Periodically)
- Type de contrôle (Instructions, Pass-Fail, Measure, Picture, Worksheet, Spreadsheet, Register Production, Print label)
- Template (si Worksheet / Spreadsheet)
- Équipe et responsable
- Instructions, Message If Failure, Notes

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification QCP
- **KindMother** : Persistance des QCP (WriteIntent)
- **Master Butler** : Permissions de création / modification QCP
- **WorrySentinel** : Vérification niveau sécurité
- **Ever Buddy** : Cycle de vie QCP

**Contrat d'équipe :**
- Consomme : QualityTeamOperator (équipe), catalogue produits (MiyuStore ou équivalent), éventuellement MiyuManufacturing (work order operations), MiyuInventory (types d'opérations)
- Expose : `qcp.create`, `qcp.update`, `qcp.evaluate_trigger` (évaluation des conditions de création de contrôles)

**Mandat de Permission requis :**
- Création / modification QCP : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.2 QualityCheckOperator

**Rôle :** Gestion des contrôles qualité : création (automatique via QCP ou manuelle), traitement (Pass / Fail / Validate), lien ordre (MO, picking, work order).

**Capacités :**
- Création automatique de contrôles (évaluation QCP sur événements MO / picking / WO)
- Création manuelle de contrôles (Operation / Product / Quantity, Picking ou MO, type, équipe)
- Traitement : Pass, Fail, Validate (selon type) ; saisie mesure, photo, worksheet
- Exposition des contrôles par ordre (MO, picking) pour affichage dans l'UI
- Déclenchement « Message If Failure » (alerte, notification) en cas d'échec

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création contrôle (auto ou manuel), décision de validation (Pass/Fail)
- **KindMother** : Persistance des contrôles (WriteIntent)
- **Master Butler** : Permissions de création et traitement
- **WorrySentinel** : Vérification niveau sécurité
- **Ever Buddy** : Cycle de vie contrôle (pending → passed / failed)

**Contrat d'équipe :**
- Consommé par : QualityUI
- Consomme : QualityControlPointOperator (évaluation QCP), QualityTeamOperator (équipe), QualityAlertOperator (création alerte si échec), MiyuNotify (notifications), MiyuManufacturing (MO, WO), MiyuInventory (picking)
- Expose : `check.create`, `check.create_manual`, `check.process`, `check.list_by_order`

**Mandat de Permission requis :**
- Création automatique : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + événement ordre (MO/picking/WO)
- Création manuelle : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Traitement (Pass/Fail/Validate) : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.3 QualityAlertOperator

**Rôle :** Gestion des alertes qualité (création, assignation, workflow, description, actions correctives/préventives).

**Capacités :**
- Création d'alertes (depuis app Quality, depuis MO, picking, Shop Floor)
- Champs : titre, produit, work center, picking, équipe, responsable, tags, cause racine, priorité
- Onglets : Description, Corrective Actions, Preventive Actions, Miscellaneous
- Workflow (stages Kanban) : déplacement d'étapes (glisser-déposer ou barre de stage)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification alerte, changement de stage
- **KindMother** : Persistance des alertes (WriteIntent)
- **Master Butler** : Permissions de création / assignation
- **WorrySentinel** : Vérification niveau sécurité
- **Ever Buddy** : Cycle de vie alerte (stages)

**Contrat d'équipe :**
- Consommé par : QualityUI
- Consomme : QualityTeamOperator (équipe), QualityCheckOperator (lien contrôle en échec si créée depuis contrôle), MiyuNotify (notifications), MiyuManufacturing (MO, work center), MiyuInventory (picking)
- Expose : `alert.create`, `alert.update`, `alert.change_stage`, `alert.assign`

**Mandat de Permission requis :**
- Création alerte : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification / changement stage : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.4 QualityTeamOperator

**Rôle :** Gestion des équipes qualité (membres, responsable).

**Capacités :**
- Création / modification d'équipes
- Gestion des membres (utilisateurs)
- Rattachement société (multi-société)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification équipe
- **KindMother** : Persistance des équipes (WriteIntent)
- **Master Butler** : Permissions de gestion équipes
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consommé par : QualityControlPointOperator, QualityCheckOperator, QualityAlertOperator
- Consomme : res.users (membres)
- Expose : `team.create`, `team.update`, `team.members`

**Mandat de Permission requis :**
- Création / modification équipe : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.5 FailureLocationOperator

**Rôle :** Gestion du référentiel des lieux de défaillance (Failure Locations).

**Capacités :**
- Création / modification des lieux de défaillance
- Exposition pour rapports et sélection (cause racine, alertes)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consommé par : QualityAlertOperator, QualityMetricsOperator, QualityUI
- Expose : `failure_location.create`, `failure_location.update`, `failure_location.list`

**Mandat de Permission requis :**
- Création / modification : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.6 QualityMetricsOperator

**Rôle :** Calculs et rapports (taux de conformité, statut des contrôles, causes de défauts, lieux de défaillance).

**Capacités :**
- Agrégation : contrôles passés / échoués / en attente par produit, opération, équipe, période
- Taux de conformité (passed / total)
- Rapports par cause racine et Failure Location
- Données pour tableaux de bord (lecture seule)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Pas de décision d'écriture ; lecture gouvernée
- **KindMother** : Lecture des données (pas d'écriture)
- **Master Butler** : Permissions de lecture rapports
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consomme : QualityCheckOperator (lecture contrôles), QualityAlertOperator (lecture alertes), FailureLocationOperator (lecture lieux)
- Expose : `metrics.conformity_rate`, `metrics.checks_by_status`, `metrics.alerts_by_cause`, `metrics.failure_locations_report`

**Mandat de Permission requis :**
- Lecture rapports : Mandat avec QualityMetricsOperator (lecture)

### 2.7 QualityUI

**Rôle :** Interface utilisateur Quality (Control Points, Quality Checks, Quality Alerts, Configuration).

**Capacités :**
- Vues : liste / formulaire QCP, liste / formulaire Quality Checks, Kanban / formulaire Quality Alerts
- Configuration : équipes, templates (Worksheet/Spreadsheet), Failure Locations
- Traitement des contrôles (Pass / Fail / Validate) depuis l’UI Quality ou depuis le contexte ordre (MO, picking, Shop Floor)
- Création d’alertes depuis l’app ou depuis un ordre

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision déléguée via Mandats pour les actions utilisateur
- **Master Butler** : Permissions d’affichage et d’action
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consomme : QualityControlPointOperator, QualityCheckOperator, QualityAlertOperator, QualityTeamOperator, FailureLocationOperator, QualityMetricsOperator
- Expose : écrans et actions utilisateur (pas d’API métier directe)

**Mandat de Permission requis :**
- Affichage et actions selon profil (Quality User / Quality Manager) et Mandats émis par StrongFather

---

## 3. Contrat d'Équipe QualityService

**Opérateurs membres :** QualityControlPointOperator, QualityCheckOperator, QualityAlertOperator, QualityTeamOperator, FailureLocationOperator, QualityMetricsOperator, QualityUI

**Flux autorisés :**
- QualityUI → QualityControlPointOperator, QualityCheckOperator, QualityAlertOperator, QualityTeamOperator, FailureLocationOperator, QualityMetricsOperator (lecture/écriture selon Mandat)
- QualityCheckOperator → QualityControlPointOperator (évaluation QCP), QualityAlertOperator (création alerte si échec), QualityTeamOperator (équipe)
- QualityAlertOperator → QualityTeamOperator (équipe), FailureLocationOperator (cause/lieu)
- QualityControlPointOperator → QualityTeamOperator (équipe)

**Types d'échanges :** WriteIntent (création/modification QCP, Check, Alert, Team, FailureLocation), lectures (contrôles par ordre, métriques, listes)

**Conditions préalables :** Mandat de Permission valide pour la session utilisateur et l’action demandée

**Niveau de validation :** StrongFather pour toute décision d’écriture ; Master Butler pour permissions ; WorrySentinel pour niveau de sécurité

---

## 4. Mandats de Permission Typiques

| Action | Opérateurs impliqués | Mandat |
|--------|----------------------|--------|
| Créer / modifier un QCP | QualityControlPointOperator, StrongFather, KindMother, QualityTeamOperator | Mandat Quality Manager (configuration) |
| Créer un contrôle (auto) | QualityCheckOperator, QualityControlPointOperator, StrongFather, KindMother | Mandat déclenché par événement ordre (MO/picking/WO) |
| Créer un contrôle (manuel) | QualityCheckOperator, StrongFather, KindMother | Mandat Quality User (création manuelle) |
| Traiter un contrôle (Pass/Fail) | QualityCheckOperator, StrongFather, KindMother | Mandat Quality User (traitement) |
| Créer une alerte | QualityAlertOperator, StrongFather, KindMother | Mandat Quality User ou Manager |
| Consulter rapports | QualityMetricsOperator | Mandat lecture (Quality User / Manager) |
| Configurer équipes / Failure Locations | QualityTeamOperator, FailureLocationOperator, StrongFather, KindMother | Mandat Quality Manager |

---

## 5. Intégration avec les Cores

- **StrongFather** : Toute décision de création, modification, changement d’état (contrôle, alerte) et configuration (QCP, équipe, Failure Location)
- **KindMother** : Persistance de toutes les entités Quality (WriteIntent)
- **Master Butler** : Permissions (Quality User, Quality Manager) et capacités (création QCP, création contrôle, traitement, création alerte)
- **WorrySentinel** : Niveau de sécurité 2 (Sensitive) pour les données qualité
- **Ever Buddy** : Cycle de vie QCP, contrôles (pending → passed/failed), alertes (stages)
- **BondingBrother** : Médiation entre QualityUI et les Opérateurs Quality ; traduction des intentions en demandes gouvernées

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
