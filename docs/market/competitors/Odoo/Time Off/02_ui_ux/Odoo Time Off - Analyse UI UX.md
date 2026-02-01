# Odoo Time Off — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Time Off** (Congés / Absences) d'Odoo (version 19.0), à partir de la documentation officielle et du code source. Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, `https://github.com/odoo/odoo/tree/19.0/addons/hr_holidays/views` et `static/src`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Sections principales de l'app (My Time Off, Overview, Management, Configuration, Reporting)
- Vues List, Form, Calendar, Dashboard
- Widgets et composants spécifiques
- Patterns de navigation et d'interaction
- Responsive et accessibilité (éléments connus)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de l'Application

### 1.1 Menu et sections

L'app **Time Off** expose les sections suivantes (selon droits) :

| Section | Description | Accès |
|--------|-------------|--------|
| **My Time Off** | Mes soldes et mes demandes | Tous |
| **Overview** | Calendrier (mes congés / équipe) | Tous (périmètre selon droits) |
| **Management** | Demandes à approuver, gestion | Time Off Officer / Admin |
| **Allocations** | Liste et création d'allocations | Time Off Officer / Admin |
| **Configuration** | Types, plans d'acquisition, fériés, jours obligatoires | Time Off Officer / Admin |
| **Reporting** | Time Off Summary, By Department, Analysis | Time Off Officer / Admin |

**Navigation :**
- Menu latéral ou dashboard avec entrées par section
- Filtres et regroupements dans les vues liste
- Actions contextuelles (Approve, Refuse, Cancel) sur les demandes et allocations

---

## 2. My Time Off

### 2.1 Vue principale

**Rôle :** Point d'entrée employé — soldes par type de congé et liste des demandes (en cours et passées).

**Composants attendus :**
- **Soldes (dashboard / cartes)** : pour chaque type de congé actif, affichage du solde (jours ou heures) — souvent sous forme de cartes avec icône/couleur du type, nom du type, solde disponible
- **Liste des demandes** : vue liste des `hr.leave` de l'utilisateur (statut, type, dates, nombre de jours/heures)
- **Actions** : bouton "New" / "Request Time Off" pour créer une demande

**Filtres typiques :**
- Statut : To Approve, Approved, Refused, Cancelled
- Type de congé
- Période (dates)

**Design :**
- Couleurs et icônes par type (définis sur `hr.leave.type` : Color, Cover Image)
- Soldes en évidence (chiffres, éventuellement barre ou indicateur)
- Lien vers Overview (calendrier)

---

## 3. Overview (Calendrier)

### 3.1 Vue calendrier

**Rôle :** Calendrier coloré des congés (utilisateur ou équipe/entreprise) pour visualiser qui est absent et quand.

**Caractéristiques (documentation Odoo 19) :**
- **Période** : Today, This week, This month, This year, ou période personnalisée
- **Vue par défaut** : trimestre (3 mois)
- **Navigation** : flèches gauche/droite pour avancer/reculer dans le temps ; bouton "Focus Today" pour revenir à aujourd'hui
- **Légende** : liste des employés (ou "Me") avec couleur par employé
- **Barres de congé** : par employé, une barre par période de congé ; **pleine** = validé, **rayée** = à approuver
- **Texte sur la barre** : nombre de jours/heures si la place le permet
- **Ligne Total** : en bas, graphique en barres du nombre de personnes absentes par jour ; nombre affiché sur chaque barre
- **Clic sur un congé** : détail (heures/jours, début, fin) ; bouton "View" pour ouvrir la demande en modal/forme

**Filtres :**
- Mes congés / Mon équipe / Toute l'entreprise (selon droits)
- Période et granularité (jour, semaine, mois, année)

**Design :**
- Couleurs distinctes par employé (pas par type) pour faciliter la lecture "qui est absent"
- Responsive : adaptation à la largeur d'écran (calendrier horizontal scroll si besoin)
- Accessibilité : libellés et contraste pour statut validé / à approuver

---

## 4. Management

### 4.1 Demandes à approuver

**Vue liste** des demandes de congé (et éventuellement d'allocations) en attente de validation.

**Colonnes typiques :**
- Employé, Type de congé, Date début, Date fin, Nombre de jours/heures, Statut (To Approve, validate1), Demandeur
- Filtres : par département, type, période, statut
- Actions en lot : Approve, Refuse (si proposé)

**Vue formulaire** (détail d'une demande) :
- En-tête : employé, type, dates, nombre de jours/heures, statut
- Solde avant/après (si affiché)
- Pièce jointe (si type autorise)
- Chatter : messages, activités
- Boutons : Approve, Refuse, Cancel (si brouillon ou annulation)

### 4.2 Allocations

**Vue liste** des allocations (`hr.leave.allocation`) :
- Colonnes : Employé(s), Type, Nombre de jours/heures, Période, Statut, etc.
- Filtres : par employé, type, statut, période
- Action "Generate Allocations" (wizard) pour créer en lot

**Vue formulaire** allocation :
- Champs : nom, employé(s), type, nombre de jours/heures, dates, accrual (oui/non)
- Workflow : draft → confirm → validate / validate1 → validate
- Boutons : Confirm, Approve, Refuse

---

## 5. Configuration

### 5.1 Time Off Types (`hr.leave.type`)

**Vue liste** : tous les types (Paid, Sick, Unpaid, Compensatory, Extra Hours par défaut).

**Vue formulaire** — sections :
- **Time off requests** : Approval (No Validation / By Time Off Officer / By Employee's Approver / Both), Notified Time Off Officer
- **Allocation requests** : Requires allocation (Yes / No Limit), Employee Requests (Extra Allowed / Not Allowed), Approval (idem)
- **Configuration** : Take Time Off in (Day / Half Day / Hours), Deduct Extra Hours, Public Holiday Included, Allow To Attach Supporting Document, Kind of Time Off (Worked Time / Absence), Company
- **Negative cap** : Allow Negative Cap, Maximum Excess Amount (jours)
- **Payroll** : Work Entry Type
- **Timesheets** (mode dev) : Project, Task
- **Display** : Color, Cover Image

**Widgets** : sélections, checkboxes, many2one (users, company, work entry type).

### 5.2 Accrual Plans (`hr.leave.accrual`)

**Vue liste** : liste des plans.

**Vue formulaire** :
- Champs plan : Name, Accrued Gain Time, Carry-Over Time, Based on worked time, Milestone Transition, Company
- **Rules (milestones)** : sous-vue ou tableau des règles — Employee accrue (Days/Hours × montant × fréquence), Cap accrued time, Start Accruing, Carry over, Milestone cap, Carry Over Validity
- Bouton "New Milestone" pour ajouter une règle

### 5.3 Public Holidays

**Vue liste** éditable (liste de lignes) :
- Colonnes : Name, Company, Start Date, End Date, Working Hours, Work Entry Type
- Bouton "New" pour ajouter une ligne

### 5.4 Mandatory Days

**Vue liste** :
- Colonnes : Name, Company, Departments, Start Date, End Date, Color
- "Departments" parfois masqué (colonnes optionnelles)
- Couleur affichée dans le calendrier Overview

---

## 6. Reporting

### 6.1 Time Off Summary

- Rapport synthèse : congés par employé, type, période (liste ou tableau).
- Export PDF possible (rapport Odoo standard).

### 6.2 Time Off by Department

- Congés agrégés par département.
- Vue liste ou graphique.

### 6.3 Time Off Analysis

- Analyse détaillée (pivot / graph) : dimensions (employé, type, département, période), mesures (jours, heures).
- Filtres et regroupements configurables.

**Patterns** : vues rapport Odoo classiques (list, pivot, graph), filtres par période et dimension.

---

## 7. Formulaires de demande et wizards

### 7.1 Demande de congé (Request Time Off)

- **Champs** : Type de congé (sélection), Date début, Date fin (ou demi-journées / heures selon type), motif ou pièce jointe (optionnel)
- **Affichage dynamique** : nombre de jours/heures calculé et solde restant (avant envoi)
- **Boutons** : Submit / Send, Cancel
- **Validation côté client** : dates cohérentes, type requis, solde suffisant (ou avertissement si Extra Days / Negative Cap)

### 7.2 Annulation (Cancel Leave)

- **Wizard** : confirmation d'annulation, motif optionnel
- **Effet** : passage en état cancel, recrédit du solde, mise à jour Overview / Payroll / Timesheet si intégrés

### 7.3 Generate Allocations (wizard)

- **Champs** : Type de congé, Employés (multi), Nombre de jours/heures, Période (date début, date fin)
- **Action** : création de N lignes `hr.leave.allocation` (une par employé ou une ligne multi-employés selon modèle Odoo)
- **Étapes** : choix options → prévisualisation ou création directe

### 7.4 Autres wizards

- **Summary Employees** (si présent) : sélection employés pour rapport synthèse
- **Generate multi** (allocations / congés) : génération en lot selon critères

---

## 8. Widgets et composants récurrents

- **Badge / statut** : couleur selon état (draft, confirm, validate, refuse, cancel)
- **Many2one Employé/User** : avec avatar (many2one_avatar_user si utilisé)
- **Dates** : date picker, datetime picker (heure selon contexte)
- **Unité jour/heure** : affichage cohérent avec le type (Day / Half Day / Hours)
- **Couleur par type** : pastille ou barre (Color, Cover Image du type)
- **Chatter** : messages, activités, suivi sur demande et allocation
- **Calendrier** : composant calendar (fullcalendar ou équivalent) pour Overview
- **Graphique en barres** : ligne "Total" du nombre de personnes absentes par jour

---

## 9. Patterns de navigation

- **Entrée unique employé** : My Time Off (soldes + demandes) avec lien vers Overview
- **Entrée approbateur** : Management → liste "To Approve" avec filtre par défaut
- **Entrée officier** : Management, Allocations, Configuration, Reporting dans le même menu Time Off
- **Actions contextuelles** : Approve / Refuse / Cancel depuis la liste ou le formulaire
- **Breadcrumb** : App Time Off > Section > Sous-vue > Détail
- **Retour** : retour liste après création/édition ou annulation

---

## 10. Responsive et accessibilité

- **Desktop** : vue principale ; calendrier Overview en largeur
- **Tablette / mobile** : liste des demandes et soldes prioritaires ; formulaire de demande en colonne ; calendrier en scroll horizontal ou vue semaine/jour
- **Contraste** : couleurs validé / à approuver différenciables (plein / rayé + texte)
- **Labels** : champs formulaires et boutons nommés pour lecteurs d'écran
- **Clavier** : navigation standard Odoo (tabs, entrée sur boutons)

---

## 11. Recommandations pour Miyukini

- **Unifier My Time Off et Overview** : une page "Mes congés" avec onglets ou zones Soldes, Demandes, Calendrier
- **Cartes de solde** : une carte par type (icône, nom, solde, lien "Poser une demande")
- **Calendrier** : conserver légende par utilisateur, barres pleines/rayées, ligne Total ; exposant en lecture seule par défaut, actions (détail, annuler) selon droits
- **Validation** : boutons Approve/Refuse visibles et accessibles ; motif de refus obligatoire ou fortement encouragé
- **Configuration** : formulaires structurés par blocs (Validation, Allocation, Options, Payroll, Affichage) ; règles d'accrual éditables en ligne (tableau de règles)
- **Rapports** : Time Off Summary et Analysis avec filtres période/département/type et export ; gouvernance des données (niveau 2) et traçabilité des accès

---

## Références

- Documentation Odoo 19.0 — Time Off (Overview, Request time off, Management, Allocations, Configuration, Reporting)
- Odoo GitHub — `addons/hr_holidays/views`, `static/src`
- Analyse Odoo Project — Analyse UI UX (structure de document)
