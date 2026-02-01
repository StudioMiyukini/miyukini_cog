# Odoo Maintenance — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Maintenance** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent Maintenance
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **EquipmentOperator** | Gestion des équipements | Opérateur de Service |
| **MaintenanceRequestOperator** | Gestion des demandes de maintenance | Opérateur de Service |
| **MaintenanceTeamOperator** | Gestion des équipes de maintenance | Opérateur de Service |
| **EquipmentCategoryOperator** | Gestion des catégories d'équipement | Opérateur de Service |
| **MaintenanceMetricsOperator** | Calcul des métriques (MTBF, MTTR, etc.) | Opérateur de Service |
| **MaintenanceUI** | Interface utilisateur Maintenance | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : MaintenanceService

**Définition :**
> **MaintenanceService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des équipements et des demandes de maintenance (préventive et corrective).**

**Composition :**
- EquipmentOperator (niveau sécurité 2)
- MaintenanceRequestOperator (niveau sécurité 2)
- MaintenanceTeamOperator (niveau sécurité 2)
- EquipmentCategoryOperator (niveau sécurité 2)
- MaintenanceMetricsOperator (niveau sécurité 2, lecture / calcul uniquement)
- MaintenanceUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 EquipmentOperator

**Rôle :** Gestion des équipements (création, configuration, affectation, métadonnées produit et maintenance).

**Capacités :**
- Création / modification d'équipements
- Affectation catégorie, équipe, technicien, lieu, centre de travail (si intégration Manufacturing)
- Affectation « Used By » (département, employé) si intégration HR
- Saisie des informations produit (fournisseur, modèle, série, coût, garantie)
- Saisie de l'Expected MTBF (MTBF attendu)
- Exposition des métriques calculées (MTBF, MTTR, Latest Failure, Estimated Next Failure) en lecture

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification équipement, affectation équipe
- **KindMother** : Persistance des équipements (WriteIntent)
- **Master Butler** : Permissions de création / modification équipement
- **WorrySentinel** : Vérification niveau sécurité
- **Ever Buddy** : Cycle de vie équipement

**Contrat d'équipe :**
- Consomme : EquipmentCategoryOperator (catégorie), MaintenanceTeamOperator (équipe), MiyuContacts (fournisseur), éventuellement MiyuManufacturing (work center), MiyuHR (département / employé)
- Expose : `equipment.create`, `equipment.update`, `equipment.assign_team`, `equipment.metrics_read`

**Mandat de Permission requis :**
- Création équipement : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification équipement : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Lecture métriques : Mandat avec MaintenanceMetricsOperator (lecture)

### 2.2 MaintenanceRequestOperator

**Rôle :** Gestion des demandes de maintenance (création, assignation, workflow, planification).

**Capacités :**
- Création / modification de demandes
- Assignation équipe et responsable (technicien)
- Gestion des stages (New Request, In Progress, Repaired, Scrap)
- Planification (Scheduled Date, Duration)
- Priorité (0–3)
- Type (Corrective / Preventive)
- Cible (Equipment ou Work Center si intégration MRP)
- Lien MO / WO (si panne en production)
- Instructions (PDF, lien, texte) et notes
- Option Block Workcenter (si Work Center)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification demande, changement de stage
- **KindMother** : Persistance des demandes (WriteIntent)
- **Master Butler** : Permissions de création / modification / assignation
- **WorrySentinel** : Vérification niveau sécurité
- **Ever Buddy** : Cycle de vie demande, transitions de stage

**Contrat d'équipe :**
- Consommé par : MaintenanceUI
- Consomme : EquipmentOperator (équipement), MaintenanceTeamOperator (équipe), MiyuNotify (notifications), MiyuClock (dates), éventuellement MiyuManufacturing (work center, MO, WO)
- Expose : `request.create`, `request.update`, `request.assign`, `request.change_stage`

**Mandat de Permission requis :**
- Création demande : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + vérification accès équipement (Follower ou Equipment Manager)
- Modification / changement stage : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Assignation : Mandat avec StrongFather (décision) + MiyuNotify (notification)

### 2.3 MaintenanceTeamOperator

**Rôle :** Gestion des équipes de maintenance (membres / techniciens).

**Capacités :**
- Création / modification d'équipes
- Gestion des membres (techniciens)
- Rattachement société (multi-société)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification équipe
- **KindMother** : Persistance des équipes (WriteIntent)
- **Master Butler** : Permissions de gestion équipes
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consommé par : EquipmentOperator, MaintenanceRequestOperator
- Consomme : MiyuContacts / res.users (membres)
- Expose : `team.create`, `team.update`, `team.members`

**Mandat de Permission requis :**
- Création / modification équipe : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.4 EquipmentCategoryOperator

**Rôle :** Gestion des catégories d'équipement.

**Capacités :**
- Création / modification de catégories
- Affectation responsable, alias email, commentaires
- Exposition des équipements et demandes de la catégorie (smart buttons)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création / modification catégorie
- **KindMother** : Persistance des catégories (WriteIntent)
- **Master Butler** : Permissions de gestion catégories
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consommé par : EquipmentOperator
- Consomme : MiyuNotify (alias email si création demande par email)
- Expose : `category.create`, `category.update`, `category.equipments`, `category.requests`

**Mandat de Permission requis :**
- Création / modification catégorie : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.5 MaintenanceMetricsOperator

**Rôle :** Calcul et exposition des métriques (MTBF, MTTR, Latest Failure, Estimated Next Failure). Ne modifie pas les données ; calcule à partir des demandes terminées.

**Capacités :**
- Calcul MTBF (moyenne des intervalles entre pannes correctives)
- Calcul MTTR (moyenne des durées des demandes terminées)
- Dérivation Latest Failure (dernière demande corrective terminée)
- Dérivation Estimated Next Failure (Latest Failure + MTBF)
- Exposition en lecture seule sur l'équipement

**Niveau de sécurité :** 2 (Sensitive) — lecture de données sensibles

**Gouvernance :**
- **KindMother** : Lecture des demandes et équipements (pas d'écriture)
- **Master Butler** : Permission de lecture métriques
- **WorrySentinel** : Niveau sécurité lecture

**Contrat d'équipe :**
- Consommé par : EquipmentOperator, MaintenanceUI
- Consomme : KindMother (lecture demandes / équipements)
- Expose : `metrics.compute_mtbf`, `metrics.compute_mttr`, `metrics.estimated_next_failure`, `metrics.latest_failure`

**Mandat de Permission requis :**
- Lecture métriques : Mandat avec Master Butler (permission read) + WorrySentinel (niveau)

### 2.6 MaintenanceUI

**Rôle :** Interface utilisateur Maintenance (listes, Kanban, formulaires, calendrier, configuration).

**Capacités :**
- Vues Liste / Kanban / Formulaire pour demandes et équipements
- Calendrier des maintenances (Scheduled Date)
- Configuration équipes et catégories
- Création / édition demandes et équipements via BondingBrother
- Affichage des métriques (lecture)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs
- **Master Butler** : Permissions d'affichage
- **WorrySentinel** : Niveau sécurité affichage

**Contrat d'équipe :**
- Consomme : EquipmentOperator, MaintenanceRequestOperator, MaintenanceTeamOperator, EquipmentCategoryOperator, MaintenanceMetricsOperator
- Expose : UI uniquement (pas d'API métier directe)

**Mandat de Permission requis :**
- Affichage et actions UI : Mandat couvrant les Opérateurs appelés (demandes, équipements, équipes, catégories, métriques)

---

## 3. Contrat d'Équipe : MaintenanceService

**Flux autorisés :**
- MaintenanceUI → EquipmentOperator, MaintenanceRequestOperator, MaintenanceTeamOperator, EquipmentCategoryOperator, MaintenanceMetricsOperator
- MaintenanceRequestOperator → EquipmentOperator, MaintenanceTeamOperator, MiyuNotify, MiyuClock, (MiyuManufacturing)
- EquipmentOperator → EquipmentCategoryOperator, MaintenanceTeamOperator, MaintenanceMetricsOperator (lecture)
- MaintenanceMetricsOperator → KindMother (lecture)

**Direction des flux :** UI vers Opérateurs ; Opérateurs vers Cores et Kits (KindMother, StrongFather, Master Butler, MiyuNotify, etc.).

**Types d'échanges :** WriteIntent (création / mise à jour), Mandat (autorisation), Lecture (métriques, listes).

**Conditions préalables :** Mandat de Permission valide émis par StrongFather pour la session / l'action.

**Niveau de validation requis :** StrongFather pour toute création / modification ; Master Butler pour permissions ; WorrySentinel pour niveau sécurité.

---

## 4. Règles de Sécurité Hétérogène

- **MaintenanceUI** : Niveau 1 (Standard) — affichage et traduction d'intentions.
- **EquipmentOperator, MaintenanceRequestOperator, MaintenanceTeamOperator, EquipmentCategoryOperator, MaintenanceMetricsOperator** : Niveau 2 (Sensitive) — données équipements, demandes, équipes, catégories, métriques.
- Un flux ne peut pas descendre en niveau de sécurité (données sensibles non exposées à l'UI au-delà du nécessaire).
- Ponts entre niveaux : explicites, via BondingBrother et Mandats ; validés par WorrySentinel.

---

## 5. Correspondance Odoo → Miyukini

| Odoo | Miyukini |
|------|----------|
| maintenance.equipment | EquipmentOperator + KindMother (entité Equipment) |
| maintenance.request | MaintenanceRequestOperator + KindMother (entité MaintenanceRequest) |
| maintenance.team | MaintenanceTeamOperator + KindMother (entité MaintenanceTeam) |
| maintenance.equipment.category | EquipmentCategoryOperator + KindMother (entité EquipmentCategory) |
| Métriques (MTBF, MTTR, etc.) | MaintenanceMetricsOperator (calcul) + lecture sur Equipment |
| Equipment Manager | Mandat avec accès tous équipements / demandes |
| Follower équipement | Mandat limité à création demande pour équipements suivis |
| mail (chatter, followers) | MiyuNotify + chatter / followers sur équipement et demande |

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
