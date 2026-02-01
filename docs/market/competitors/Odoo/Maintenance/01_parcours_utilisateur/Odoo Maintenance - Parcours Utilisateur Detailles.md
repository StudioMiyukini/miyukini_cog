# Odoo Maintenance — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Maintenance** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Maintenance

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

### 1.1 Responsable Maintenance / Equipment Manager

**Profil :**
- Rôle stratégique : Gestion globale des équipements et des équipes de maintenance
- Responsabilités :
  - Créer et configurer les catégories d'équipements
  - Créer et gérer les équipes de maintenance
  - Enregistrer les équipements (machines, outils)
  - Assigner techniciens et équipes aux équipements
  - Suivre les métriques (MTBF, MTTR, prochaine panne estimée)
  - Valider les demandes et les déplacer en Repaired / Scrap

**Besoins :**
- Vue d'ensemble des équipements et de leur état
- Calendrier des maintenances planifiées
- Rapports sur les pannes et temps de réparation
- Configuration des équipes et catégories
- Accès à tous les équipements

**Permissions :**
- Equipment Manager (Supply Chain ‣ Maintenance)
- Accès à tous les équipements et demandes
- Configuration (équipes, catégories)

### 1.2 Technicien de Maintenance

**Profil :**
- Rôle opérationnel : Exécution des maintenances
- Responsabilités :
  - Consulter les demandes assignées
  - Exécuter les maintenances (préventives ou correctives)
  - Mettre à jour le statut des demandes (stage)
  - Saisir la durée réelle
  - Suivre les instructions (PDF, Google Slide, texte)
  - Remplir les feuilles de travail (si activées)

**Besoins :**
- Vue « Mes demandes » ou Kanban par stage
- Calendrier des maintenances planifiées
- Instructions claires par demande
- Saisie rapide de la durée et du stage
- Notifications sur nouvelles assignations

**Permissions :**
- Membre d'une ou plusieurs équipes de maintenance
- Accès aux demandes assignées à son équipe ou à lui-même
- Pas nécessairement Equipment Manager

### 1.3 Utilisateur Opérationnel (Follower)

**Profil :**
- Rôle limité : Création de demandes pour des équipements spécifiques
- Responsabilités :
  - Signaler une panne ou demander une maintenance préventive
  - Créer une demande de maintenance pour un équipement auquel il a accès (Follower)
  - Consulter le suivi de ses demandes

**Besoins :**
- Création simple de demande (titre, équipement, type, priorité)
- Suivi du statut de la demande
- Notifications sur avancement
- Pas d'accès à toute la base équipements

**Permissions :**
- Follower d'un ou plusieurs équipements (chatter ‣ Add Followers)
- Ou droits Equipment Manager pour tous les équipements

### 1.4 Responsable Production / Work Center

**Profil :**
- Rôle transverse : Demande de maintenance sur centres de travail
- Responsabilités :
  - Créer des demandes pour un centre de travail (For = Work Center)
  - Option « Block Workcenter » pour bloquer la planification pendant la maintenance
  - Lier une demande à un ordre de fabrication ou ordre de travail (si panne pendant production)

**Besoins :**
- Création de demande pour Work Center
- Lien MO / WO si panne en production
- Visibilité sur les maintenances planifiées par centre

**Permissions :**
- Accès Maintenance et éventuellement MRP (centres de travail, MO, WO)

---

## 2. Parcours d'Onboarding

### 2.1 Premier déploiement (Responsable Maintenance)

1. **Activation du module** : Installation de l'app Maintenance (dépend de `mail`).
2. **Configuration des équipes** : Maintenance ‣ Configuration ‣ Maintenance Teams ‣ Créer (nom, membres, société).
3. **Création des catégories** : Configuration ‣ Equipment Categories ‣ Créer (nom, responsable, alias email optionnel).
4. **Enregistrement des équipements** : Equipment ‣ Machines & Tools ‣ Créer (nom, catégorie, équipe, technicien, lieu, centre de travail, infos produit, métriques attendues).
5. **Droits utilisateurs** : Settings ‣ Users ‣ Access Rights ‣ Supply Chain ‣ Maintenance (Equipment Manager ou ajout en Follower par équipement).

### 2.2 Premier usage (Technicien)

1. Accès au menu Maintenance ‣ Maintenance Requests.
2. Vue Kanban par défaut (stages : New Request, In Progress, Repaired, Scrap).
3. Ouverture d'une demande assignée, lecture des instructions, exécution, mise à jour du stage et de la durée.

### 2.3 Premier usage (Utilisateur Follower)

1. Ajout comme Follower sur un équipement (par un Manager).
2. Maintenance ‣ Maintenance Requests ‣ New.
3. Saisie titre, For = Equipment, sélection de l'équipement (visible car Follower), type (Corrective / Preventive), équipe, priorité, date prévue, durée, notes.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Demande corrective (panne)

1. Un utilisateur (Follower ou Manager) constate une panne sur un équipement.
2. Maintenance ‣ Maintenance Requests ‣ New.
3. Titre (ex. « Perceuse en panne »), Created By (auto), For = Equipment, Equipment = [équipement], Request Date (auto), Maintenance Type = Corrective.
4. Option : lier un Manufacturing Order / Work Order si panne en production.
5. Équipe, Responsable, Scheduled Date, Duration, Priorité (0–3 étoiles), Notes, Instructions.
6. Enregistrement ; la demande apparaît en « New Request ».
7. L'équipe déplace la demande en « In Progress », exécute la maintenance, puis en « Repaired » ou « Scrap ».

### 3.2 Demande préventive

1. Responsable ou technicien planifie une maintenance préventive (calendrier ou métrique Estimated Next Failure).
2. Maintenance ‣ Maintenance Requests ‣ New.
3. For = Equipment (ou Work Center), Maintenance Type = Preventive.
4. Sélection équipement / centre, équipe, responsable, date et heure, durée, priorité, instructions.
5. Si Work Center : option « Block Workcenter » pour bloquer le centre pendant la maintenance.
6. Suivi du workflow jusqu'à Repaired.

### 3.3 Suivi des équipements (Manager)

1. Maintenance ‣ Equipment ‣ Machines & Tools (ou par catégorie).
2. Consultation des smart buttons Maintenance par équipement.
3. Consultation des métriques : MTBF, MTTR, Latest Failure, Estimated Next Failure.
4. Ajustement de l'Expected MTBF si besoin.
5. Création de demandes préventives à partir des dates « Estimated Next Failure ».

### 3.4 Calendrier des maintenances

1. Maintenance ‣ Maintenance ‣ Maintenance Calendar.
2. Visualisation des demandes planifiées (Scheduled Date).
3. Clic sur une demande : popover avec détail, champ Technician (membre d'équipe).
4. Liste des techniciens avec demandes ouvertes (sidebar).

---

## 4. Points de Friction Identifiés

### 4.1 Droits d'accès

- **Friction :** Un utilisateur qui n'est pas Equipment Manager doit être ajouté comme Follower sur chaque équipement pour pouvoir créer une demande. Pas de notion de « droit par catégorie » ou « droit par équipe » pour la création.
- **Recommandation Miyukini :** Permissions granulaires (par catégorie, par équipe, par équipement) avec Mandats de Permission.

### 4.2 Request Date non modifiable

- **Friction :** La date de création de la demande (Request Date) n'est pas modifiable. Pour une panne survenue la veille mais signalée aujourd'hui, l'historique affiche la date du jour.
- **Recommandation Miyukini :** Champ optionnel « Date de survenue » ou « Date de constat » distinct de la date de création.

### 4.3 Instructions multi-formats

- **Friction :** Instructions en PDF, Google Slide ou Texte — trois chemins différents. Pas de modèle unique (ex. template structuré par type d'équipement).
- **Recommandation Miyukini :** Modèle d'instructions unifié (template par catégorie / type) avec pièces jointes et champs structurés.

### 4.4 Work Centers et MRP

- **Friction :** Fonctionnalités Work Center / Block Workcenter / MO-WO liés dépendent des modules MRP. Sans MRP, seule la cible « Equipment » est pleinement disponible.
- **Recommandation Miyukini :** Service Maintenance découplé avec intégration optionnelle Manufacturing (Contrat d'équipe).

### 4.5 Métriques en lecture seule

- **Friction :** MTBF (calculé), MTTR, Latest Failure, Estimated Next Failure sont en lecture seule. Pas de correction manuelle en cas d'erreur de saisie (ex. mauvaise durée).
- **Recommandation Miyukini :** Stratégie claire : soit recalcul automatique strict, soit correction manuelle avec traçabilité (Ever Buddy).

---

## 5. Recommandations pour Miyukini

- **Personas :** Conserver les rôles Manager, Technicien, Utilisateur (Follower), Responsable Production ; les mapper sur Opérateurs et Mandats.
- **Onboarding :** Guides pas à pas (équipes, catégories, équipements, droits) ; templates de catégories et équipes.
- **Parcours :** Demande corrective / préventive, suivi équipements, calendrier ; intégration COG (WriteIntent, Mandats) pour création et changement de stage.
- **Friction :** Permissions granulaires, date de survenue optionnelle, instructions unifiées, intégration Manufacturing optionnelle, politique claire sur correction des métriques.

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
