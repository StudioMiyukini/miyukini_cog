# Odoo Maintenance — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Maintenance** d'Odoo (version 19.0), à partir de la documentation officielle et du code source. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, `https://github.com/odoo/odoo/tree/19.0/addons/maintenance`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (maintenance.equipment, maintenance.request, maintenance.team, maintenance.equipment.category)
- Règles métier et contraintes (équipements, demandes, équipes)
- Workflows et transitions d'état (demandes de maintenance)
- Maintenance préventive et corrective
- Métriques (MTBF, MTTR, prochaine panne estimée)
- Intégration avec Work Centers (MRP) et Manufacturing

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `maintenance.equipment` (Équipement)

**Rôle :** Représente un **équipement** — machine, outil ou dispositif utilisé dans les opérations (entrepôt, production, bureau). Peut être suivi pour la maintenance préventive et corrective.

**Champs clés :**

#### Identification
- `name` : Nom de l'équipement (produit)
- `category_id` : Many2one vers `maintenance.equipment.category` (catégorie)
- `company_id` : Many2one vers `res.company` (entreprise propriétaire ; peut être tiers, ex. location)
- `description` : Description (onglet Description)

#### Utilisation
- `used_by` : Sélection (Department, Employee, Other) — qui utilise l'équipement
- `department_id` : Many2one vers `hr.department` (si Used By = Department ou Other)
- `employee_id` : Many2one vers `hr.employee` (si Used By = Employee ou Other)
- `used_in_location` : Char (lieu d'utilisation, ex. bureau)
- `workcenter_id` : Many2one vers `mrp.workcenter` (centre de travail, si utilisé en production)

#### Maintenance
- `maintenance_team_id` : Many2one vers `maintenance.team` (équipe responsable)
- `technician_id` : Many2one vers `res.users` (technicien responsable)
- `maintenance_ids` : One2many vers `maintenance.request` (demandes de maintenance)
- `maintenance_count` : Integer (nombre de maintenances, smart button)

#### Informations produit
- `vendor_id` : Many2one vers `res.partner` (fournisseur)
- `vendor_reference` : Char (référence fournisseur)
- `model` : Char (modèle)
- `serial_no` : Char (numéro de série)
- `effective_date` : Date (mise en service ; utilisée pour MTBF)
- `cost` : Monetary (coût d'acquisition)
- `warranty_expiration_date` : Date (fin de garantie)

#### Métriques maintenance (calculées)
- `expected_mtbf` : Float (MTBF attendu en jours ; seul champ éditable)
- `mtbf` : Float (MTBF réel, calculé à partir des pannes corrigées)
- `estimated_next_failure` : Date (prochaine panne estimée ; Latest Failure + MTBF)
- `latest_failure_date` : Date (dernière panne ; date de création de la dernière demande corrective terminée)
- `mttr` : Float (temps moyen de réparation en jours ; calculé à partir des demandes terminées)

**Règles métier :**
- Un équipement appartient à une catégorie.
- MTBF, MTTR, Estimated Next Failure, Latest Failure sont calculés automatiquement et non modifiables manuellement.
- Les demandes de maintenance sont liées via `maintenance_ids`.

---

### 1.2 Modèle `maintenance.equipment.category` (Catégorie d'équipement)

**Rôle :** Classification des types d'équipements (ordinateurs, machines, outils, etc.).

**Champs clés :**
- `name` : Nom de la catégorie
- `user_id` : Many2one vers `res.users` (responsable de la catégorie ; défaut : créateur)
- `company_id` : Many2one vers `res.company` (multi-société)
- `alias_id` : Many2one vers `mail.alias` (alias email pour créer des demandes par email)
- `comment` : Texte (commentaires internes)
- `equipment_ids` : One2many vers `maintenance.equipment` (équipements)
- `maintenance_ids` : One2many vers `maintenance.request` (demandes ; smart button)

**Règles métier :**
- Une catégorie peut avoir un alias email pour créer des demandes.
- Tous les équipements et demandes de la catégorie sont accessibles depuis le formulaire catégorie.

---

### 1.3 Modèle `maintenance.team` (Équipe de maintenance)

**Rôle :** Équipe de techniciens responsable du traitement des demandes de maintenance.

**Champs clés :**
- `name` : Nom de l'équipe
- `member_ids` : Many2many vers `res.users` (membres / techniciens)
- `company_id` : Many2one vers `res.company` (multi-société)

**Règles métier :**
- Les membres sont aussi appelés « Techniciens » dans le calendrier de maintenance.
- Une équipe est rattachée à une société en environnement multi-société.

---

### 1.4 Modèle `maintenance.request` (Demande de maintenance)

**Rôle :** Représente une **demande de maintenance** (préventive ou corrective) sur un équipement ou un centre de travail.

**Champs clés :**

#### Identification
- `name` : Titre de la demande (obligatoire)
- `create_uid` / `request_date` : Créateur et date de création (Request Date, non modifiable par l'utilisateur)

#### Cible
- `for_type` : Sélection (Equipment, Work Center) — type de cible
- `equipment_id` : Many2one vers `maintenance.equipment` (si For = Equipment)
- `workcenter_id` : Many2one vers `mrp.workcenter` (si For = Work Center)
- `worksheet_template_id` : Many2one vers une feuille de travail (si Custom Maintenance Worksheets activé)

#### Type et contexte
- `maintenance_type` : Sélection (Corrective, Preventive)
- `manufacturing_order_id` : Many2one vers `mrp.production` (ordre de fabrication lié, si panne pendant un MO)
- `workorder_id` : Many2one vers `mrp.workorder` (ordre de travail, si panne pendant un WO)

#### Assignation et planification
- `maintenance_team_id` : Many2one vers `maintenance.team` (équipe responsable)
- `user_id` : Many2one vers `res.users` (responsable / technicien)
- `schedule_date` : Datetime (date et heure prévues)
- `duration` : Float (durée en heures, format 00:00)
- `block_workcenter` : Boolean (bloquer le centre de travail pendant la maintenance ; affiché si For = Work Center)

#### Priorité et suivi
- `priority` : Sélection ou Integer (0 à 3 étoiles ; priorité pour le Kanban)
- `stage_id` : Many2one vers un modèle de stages (workflow)

#### Contenu
- `description` : HTML (Notes)
- Instructions : PDF, Google Slide ou Texte (onglet Instructions)

**Workflow (stages typiques) :**
- **New Request** : Nouvelle demande
- **In Progress** : En cours de traitement
- **Done** : Terminée (traitement effectué)
- **Repaired** : Équipement / centre réparé (succès)
- **Scrap** : Mise au rebut (échec de réparation)

**Règles métier :**
- Pour créer une demande sur un équipement : l'utilisateur doit être « Equipment Manager » ou Follower de cet équipement.
- Request Date n'est pas modifiable par l'utilisateur.
- Les demandes peuvent être déplacées par glisser-déposer (Kanban) ou en changeant le stage dans le formulaire.
- Corrective = réparer un problème existant ; Preventive = éviter des pannes futures.

---

## 2. Métriques et Calculs

### 2.1 MTBF (Mean Time Between Failure)

- **Définition :** Temps moyen (en jours) entre deux pannes.
- **Expected MTBF :** Saisi manuellement sur l'équipement (objectif).
- **MTBF (calculé) :** Moyenne des intervalles entre les dates de création des demandes **correctives** terminées. Mis à jour automatiquement, non éditable.

### 2.2 MTTR (Mean Time To Repair)

- **Définition :** Temps moyen (en jours) pour réparer l'équipement.
- **Calcul :** Moyenne des durées (`duration`) des demandes de maintenance **terminées** pour cet équipement. Mis à jour automatiquement.

### 2.3 Estimated Next Failure

- **Calcul :** Latest Failure Date + MTBF (en jours).
- **Usage :** Planification de la maintenance préventive.

### 2.4 Latest Failure

- **Source :** Date de création de la **dernière** demande de maintenance **corrective** terminée pour cet équipement. Non modifiable manuellement.

---

## 3. Droits d'Accès

### 3.1 Création de demandes pour un équipement

- **Option 1 :** Droits « Equipment Manager » dans l'app Maintenance (Settings ‣ Users ‣ Access Rights ‣ Supply Chain ‣ Maintenance).
- **Option 2 :** Utilisateur ajouté comme **Follower** de l'équipement (chatter ‣ Add Followers). Permet de créer des demandes pour cet équipement sans donner accès à tous les équipements.

### 3.2 Niveaux typiques

- **User** : Création et suivi des demandes (selon accès équipements).
- **Manager (Equipment Manager)** : Accès à tous les équipements et à la configuration (équipes, catégories).

---

## 4. Intégrations Métier

### 4.1 Work Centers (MRP)

- Les **centres de travail** (`mrp.workcenter`) peuvent être une cible de demande de maintenance (For = Work Center).
- Option « Block Workcenter » : empêche la planification d'ordres de travail / autres maintenances sur ce centre pendant la demande.
- Depuis un centre de travail, on peut lister les équipements associés (onglet Equipment) et leurs métriques (MTBF, MTTR, Est. Next Failure).

### 4.2 Manufacturing (MRP)

- Si la panne survient pendant un **ordre de fabrication** (MO) ou un **ordre de travail** (WO), on peut lier la demande au MO et éventuellement au WO.
- Champs : `manufacturing_order_id`, `workorder_id`.

### 4.3 Mail

- Module **mail** : chatter sur équipements et demandes (messages, followers, activités).
- Notifications et historique des échanges sur chaque enregistrement.

---

## 5. Synthèse pour Miyukini

**Entités à modéliser :**
- Équipement (Equipment) : identité, catégorie, affectation, métriques.
- Catégorie d'équipement (Equipment Category).
- Équipe de maintenance (Maintenance Team).
- Demande de maintenance (Maintenance Request) : workflow, type (préventif / correctif), planification, priorité, instructions.

**Règles à préserver :**
- Calcul automatique MTBF, MTTR, Latest Failure, Estimated Next Failure.
- Contrôle d'accès (manager vs follower par équipement).
- Stages de demande (New → In Progress → Repaired / Scrap).
- Lien optionnel avec centres de travail et manufacturing (si modules présents).

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
