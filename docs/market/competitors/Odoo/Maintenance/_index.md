# Odoo Maintenance — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Maintenance - Logique Metier Complete.md](./00_logique_metier/Odoo%20Maintenance%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (maintenance.equipment, maintenance.request, maintenance.team, maintenance.equipment.category)
- Règles métier et contraintes
- Workflows et transitions d'état (demandes : New, In Progress, Repaired, Scrap)
- Métriques (MTBF, MTTR, Latest Failure, Estimated Next Failure)
- Droits d'accès (Equipment Manager, Follower)
- Intégration Work Centers et Manufacturing

### 2. Parcours Utilisateur
📄 [Odoo Maintenance - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Maintenance%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Responsable Maintenance, Technicien, Utilisateur Follower, Responsable Production)
- Parcours d'onboarding
- Scénarios d'usage (demande corrective, préventive, suivi équipements, calendrier)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Maintenance - Analyse UI UX.md](./02_ui_ux/Odoo%20Maintenance%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vues principales (Kanban, List, Form pour demandes et équipements)
- Calendrier des maintenances
- Configuration (équipes, catégories)
- Patterns de navigation
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Maintenance - Integrations Cross App.md](./03_integrations/Odoo%20Maintenance%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (mail obligatoire ; MRP, HR optionnels)
- Flux de données (mail, work centers, manufacturing, HR)
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Maintenance - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Maintenance%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (EquipmentOperator, MaintenanceRequestOperator, MaintenanceTeamOperator, EquipmentCategoryOperator, MaintenanceMetricsOperator, MaintenanceUI)
- Contrat d'équipe MaintenanceService
- Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Maintenance - Guide Integration COG.md](./05_integration_cog/Odoo%20Maintenance%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création équipement, demande, changement stage, métriques)
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Maintenance - Guide Implementation.md](./06_guides_implementation/Odoo%20Maintenance%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données (Equipment, MaintenanceRequest, MaintenanceTeam, EquipmentCategory)
- API et contrats
- Plan de développement par phases (MVP → Calendrier → Work Center → HR)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniMaintenance` ou `MiyuMaintenance`

**Opérateurs :**
- `EquipmentOperator` : Gestion des équipements
- `MaintenanceRequestOperator` : Gestion des demandes de maintenance
- `MaintenanceTeamOperator` : Gestion des équipes de maintenance
- `EquipmentCategoryOperator` : Gestion des catégories d'équipement
- `MaintenanceMetricsOperator` : Calcul des métriques (MTBF, MTTR, etc.)
- `MaintenanceUI` : Interface utilisateur Maintenance

**Équipe d'Opérateurs :** `MaintenanceService`

---

## Source d'Analyse

**Repository :** `https://github.com/odoo/odoo/tree/19.0/addons/maintenance`

**Documentation :** Odoo 19.0 — Supply Chain / Maintenance

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application Supply Chain ; dépendance unique obligatoire : `mail`.
- Intégrations optionnelles : MRP (Work Centers, MO, WO), HR (Department, Employee).
- Droits : Equipment Manager (tous équipements) ou Follower (création demande pour équipements suivis).
- Métriques (MTBF, MTTR, Latest Failure, Estimated Next Failure) calculées automatiquement à partir des demandes terminées.
