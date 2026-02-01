# Odoo Maintenance — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Maintenance** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** `__manifest__.py` et documentation Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (Work Centers, MRP)
- Données partagées (mail, activités)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (`__manifest__.py`) :**
- `mail` : Messagerie et activités (chatter, notifications, followers, activités planifiées)

**Aucune dépendance** à `stock`, `mrp`, `hr` dans le manifeste de base. Le module Maintenance fonctionne avec **mail** uniquement.

### 1.2 Modules Optionnels (Intégrations)

**Intégrations lorsque les modules sont installés :**
- **MRP (Manufacturing)** : Centres de travail (`mrp.workcenter`), ordres de fabrication (`mrp.production`), ordres de travail (`mrp.workorder`). Permet :
  - Demande de maintenance **pour** un Work Center (For = Work Center)
  - Lien Manufacturing Order / Work Order sur une demande
  - Option « Block Workcenter » pendant la maintenance
  - Vue Maintenance ‣ Equipment ‣ Work Centers et onglet Equipment sur le formulaire work center
- **HR (Employees)** : Champs `department_id`, `employee_id` sur l'équipement (Used By : Department / Employee). Si HR non installé, ces champs peuvent être absents ou désactivés.

---

## 2. Intégrations Détaillées

### 2.1 Mail (obligatoire)

**Flux :**
```
Equipment / Request → mail.thread → Chatter, Followers, Activities, Notifications
```

**Mécanismes :**
- Héritage `mail.thread` sur équipements et demandes de maintenance
- Chatter intégré (commentaires, pièces jointes)
- Followers : utilisés pour les droits de création de demande (Follower d'un équipement = droit de créer une demande pour cet équipement)
- Activités planifiées (mail.activity)
- Sous-types de messages (mail.message.subtype) pour notifications ciblées
- Types d'activités (mail.activity.type) pour le module Maintenance

**Fichiers de données :**
- `data/mail_activity_type_data.xml`
- `data/mail_message_subtype_data.xml`
- `views/mail_activity_views.xml`

**Recommandations pour Miyukini :**
- Intégration avec MiyuNotify pour notifications et abonnements
- Conserver le lien Follower ↔ droit de créer une demande (ou équivalent Mandat par équipement / catégorie)

### 2.2 MRP / Work Centers (optionnel)

**Flux :**
```
maintenance.request (for_type=Work Center) → mrp.workcenter
maintenance.equipment → mrp.workcenter (workcenter_id)
mrp.workcenter → Equipment tab (liste maintenance.equipment)
```

**Mécanismes :**
- Champ `workcenter_id` sur `maintenance.equipment` : équipement utilisé dans un centre de travail
- Champ `workcenter_id` sur `maintenance.request` : demande ciblant un centre de travail (si For = Work Center)
- Champ `block_workcenter` : bloque la planification du centre pendant la maintenance
- Champs `manufacturing_order_id`, `workorder_id` sur `maintenance.request` : lien avec un MO/WO si panne en production
- Vue « Work Centers » dans le menu Maintenance : lecture des centres de travail MRP avec onglet Equipment

**Recommandations pour Miyukini :**
- Service Maintenance découplé ; intégration Manufacturing via Contrat d'équipe (MiyuManufacturing ou équivalent) pour Work Center, MO, WO
- WriteIntent pour « bloquer centre » et lier MO/WO sans couplage fort

### 2.3 HR (optionnel)

**Flux :**
```
maintenance.equipment (used_by) → hr.department / hr.employee
```

**Mécanismes :**
- Champs `department_id`, `employee_id` sur l'équipement selon « Used By » (Department, Employee, Other)
- Permet de filtrer / grouper les équipements par département ou employé

**Recommandations pour Miyukini :**
- Intégration optionnelle avec MiyuHR (départements, employés) pour affectation « Used By »

### 2.4 Paramètres (Settings)

**Vue :** `views/res_config_settings_views.xml`

**Contenu typique :**
- Option « Custom Maintenance Worksheets » : affichage du champ Worksheet Template sur les demandes
- Autres paramètres applicatifs (selon version)

**Recommandations pour Miyukini :**
- Paramètres centralisés (Configuration COG ou Opérateur) pour options comme feuilles de travail personnalisées

---

## 3. Flux de Données Résumés

| Source        | Cible              | Données / Usage                          |
|---------------|--------------------|------------------------------------------|
| mail          | equipment, request | Chatter, followers, activités, subtypes  |
| mrp.workcenter| maintenance.request| Cible demande (For = Work Center)      |
| mrp.workcenter| maintenance.equipment | workcenter_id, liste équipements     |
| mrp.production| maintenance.request| manufacturing_order_id                  |
| mrp.workorder | maintenance.request| workorder_id                            |
| hr.department | maintenance.equipment| department_id (Used By)                 |
| hr.employee   | maintenance.equipment| employee_id (Used By)                   |
| res.users     | maintenance.team    | member_ids (techniciens)                 |
| res.users     | equipment (follower)| Droit création demande pour cet équipement |
| res.company   | equipment, team, category | company_id (multi-société)         |

---

## 4. APIs et Hooks (à compléter depuis le code)

**Typiques dans un module Odoo Maintenance :**
- Calcul des métriques (MTBF, MTTR, Latest Failure, Estimated Next Failure) : `compute` ou `_compute_*` sur `maintenance.equipment`
- Mise à jour des métriques lors du passage d'une demande en stage « Repaired » ou « Done » : `write` / `_write` ou override sur `maintenance.request`
- Contraintes d'accès : règles d'enregistrement (`ir.rule`) et droits d'accès (`ir.model.access`) dans `security/`
- Création de demande depuis alias email (catégorie) : `mail.alias` + méthode `message_new` ou équivalent sur `maintenance.request`

**Recommandations pour Miyukini :**
- Exposer les calculs de métriques comme Outils ou sous-opérateurs (sans autorité) ; persistance via KindMother (WriteIntent)
- Contrôle d'accès via Master Butler et Mandats ; pas de règles métier dispersées dans l'UI

---

## 5. Synthèse pour Miyukini

- **Mail** : Intégration obligatoire (MiyuNotify, chatter, followers, activités).
- **MRP / Work Centers** : Intégration optionnelle via Contrat d'équipe (équipements liés au centre, demande pour centre, blocage, MO/WO).
- **HR** : Intégration optionnelle (MiyuHR) pour Department / Employee sur équipement.
- **Paramètres** : Centraliser dans la configuration du service Maintenance (équivalent res.config.settings).
- **Sécurité** : Modéliser « Follower = droit de créer demande » par Mandat ou permission granulaire (équipement / catégorie / équipe).

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
