# Odoo Maintenance — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Maintenance** d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, patterns de navigation, formulaires et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Maintenance

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Calendar)
- Structure des formulaires (équipement, demande, équipe, catégorie)
- Patterns de navigation
- Widgets et champs clés
- Calendrier des maintenances
- Configuration (équipes, catégories)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales — Demandes de Maintenance

### 1.1 Vue Kanban — Maintenance Requests

**Accès :** Maintenance ‣ Maintenance ‣ Maintenance Requests

**Caractéristiques :**
- Vue par défaut pour les demandes de maintenance
- Colonnes = stages (New Request, In Progress, Repaired, Scrap)
- Glisser-déposer pour changer de stage
- Priorité : 0 à 3 étoiles ; les demandes à priorité plus haute apparaissent plus haut dans la colonne

**Contenu type des cartes :**
- Titre de la demande
- Équipement ou Work Center
- Équipe / Responsable
- Date prévue (Scheduled Date)
- Priorité (étoiles)

**Interaction :**
- Clic sur une carte : ouverture du formulaire en page dédiée
- Barre de stage au-dessus du formulaire (à droite) pour changer de stage sans passer par le Kanban

### 1.2 Vue Formulaire — Maintenance Request

**Champs principaux (ordre typique) :**
- **Request** : Titre (obligatoire)
- **Created By** : Créateur (auto, modifiable)
- **For** : Equipment ou Work Center
- **Equipment** / **Work Center** : Sélection selon For
- **Worksheet Template** : (si Custom Maintenance Worksheets activé)
- **Request Date** : Date de création (non modifiable)
- **Maintenance Type** : Corrective / Preventive
- **Manufacturing Order** : (optionnel) MO lié
- **Work Order** : (optionnel, si MO sélectionné) WO lié
- **Team** : Équipe responsable
- **Responsible** : Technicien responsable
- **Scheduled Date** : Date et heure prévues (calendrier + heure/minute, bouton Apply)
- **Duration** : Durée (format 00:00)
- **Block Workcenter** : (si For = Work Center) Bloquer le centre pendant la maintenance
- **Priority** : 0 à 3 étoiles
- **Notes** : Onglet en bas de formulaire
- **Instructions** : Onglet (PDF : upload ; Google Slide : lien ; Text : saisie texte)

**Widgets :**
- Calendrier popup pour Scheduled Date avec champs heure/minute
- Étoiles pour Priorité
- Chatter (messages, followers, activités)

### 1.3 Vue Liste — Maintenance Requests

**Usage :** Liste des demandes avec filtres et groupements possibles (par équipe, stage, type, équipement, etc.).

---

## 2. Vues — Équipements

### 2.1 Machines & Tools (List / Form)

**Accès :** Maintenance ‣ Equipment ‣ Machines & Tools

**Formulaire équipement :**
- **En-tête :** Nom, Catégorie, Société
- **Used By** : Department / Employee / Other (puis champs Department, Employee selon le choix)
- **Maintenance Team** : Équipe responsable
- **Technician** : Technicien responsable
- **Used in location** : Lieu (texte)
- **Work Center** : Centre de travail (si utilisé en production)
- **Onglet Description** : Description
- **Onglet Product Information** : Vendor, Vendor Reference, Model, Serial Number, Effective Date, Cost, Warranty Expiration Date
- **Onglet Maintenance** : Expected MTBF (éditable), MTBF (calculé), Estimated Next Failure, Latest Failure, Mean Time To Repair (tous en lecture seule sauf Expected MTBF)
- **Smart button** : Maintenances (nombre de demandes, lien vers les demandes de cet équipement)
- **Chatter** : Messages, followers, activités

### 2.2 Work Centers

**Accès :** Maintenance ‣ Equipment ‣ Work Centers (intégration MRP)

**Contenu :**
- Formulaire centre de travail avec onglet **Equipment**
- Liste des équipements du centre : Nom, Technicien, Catégorie, MTBF, MTTR, Est. Next Failure
- Bouton « Add a line » pour ajouter un équipement au centre (popup « Add: Maintenance Equipment »)

---

## 3. Calendrier des Maintenances

**Accès :** Maintenance ‣ Maintenance ‣ Maintenance Calendar

**Caractéristiques :**
- Calendrier basé sur **Scheduled Date** des demandes
- Clic sur un événement : popover avec détail de la demande
- Popover : champ **Technician** (membre d'équipe responsable)
- Sidebar à droite : mini-calendrier (date du jour) + liste des **Techniciens** avec demandes ouvertes
- Les techniciens listés correspondent aux membres des équipes de maintenance

---

## 4. Configuration

### 4.1 Maintenance Teams

**Accès :** Maintenance ‣ Configuration ‣ Maintenance Teams

**Vue liste :**
- Colonnes : Team Name, Team Members, Company
- Bouton **New** : nouvelle ligne, saisie nom, membres (dropdown + « Search More… » pour utilisateurs), société (multi-company)

### 4.2 Equipment Categories

**Accès :** Maintenance ‣ Configuration ‣ Equipment Categories

**Formulaire catégorie :**
- Category Name
- Responsible (défaut : créateur)
- Company (multi-company)
- Email Alias
- Comments
- **Smart buttons** : Equipments, Maintenances (accès aux équipements et demandes de la catégorie)

---

## 5. Patterns de Navigation

- **Menu principal :** Maintenance (app) ‣ sous-menus Maintenance (Requests, Calendar), Equipment (Machines & Tools, Work Centers), Configuration (Maintenance Teams, Equipment Categories).
- **Création demande :** Maintenance ‣ Maintenance Requests ‣ New.
- **Création équipement :** Equipment ‣ Machines & Tools ‣ Create.
- **Depuis un équipement :** Smart button « Maintenances » pour accéder aux demandes liées.
- **Depuis une catégorie :** Smart buttons « Equipments » et « Maintenances ».
- **Changement de stage :** Kanban (glisser-déposer) ou formulaire (barre de stage en haut à droite).

---

## 6. Droits et Affichage

- **Equipment Manager :** Accès à tous les équipements et demandes, configuration.
- **Follower :** Création de demandes uniquement pour les équipements suivis ; liste filtrée.
- **Technicien (membre d'équipe) :** Accès aux demandes de son équipe / assignées ; calendrier avec ses demandes.

---

## 7. Recommandations pour Miyukini

- Conserver une **vue Kanban** par stage pour les demandes, avec priorité visible et glisser-déposer.
- **Formulaire demande** : regroupement logique (cible, type, planification, priorité, contenu) ; instructions unifiées (template + pièces jointes).
- **Formulaire équipement** : onglets Description, Product Information, Maintenance ; métriques en lecture seule avec indication « calculé » ; smart button Maintenances.
- **Calendrier** : basé sur Scheduled Date ; popover détaillé ; sidebar techniciens.
- **Configuration** : écrans dédiés Équipes et Catégories avec smart buttons vers équipements et demandes.
- **Navigation** : raccourcis depuis équipement / catégorie vers demandes ; cohérence avec le glossaire Miyukini (Opérateurs, Mandats).

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
