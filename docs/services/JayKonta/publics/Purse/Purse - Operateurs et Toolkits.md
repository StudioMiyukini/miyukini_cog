# JayBudget — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du point d’entrée **JayBudget** du service COG JayKonta. Il s’appuie sur l’[analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) et le document [Parcours, capacités et livrables](./Purse%20-%20Parcours%20Capacites%20Livrables.md). Il vise à fournir une **réponse explicite** pour chaque besoin : **Service**, **Opérateur** (ou Équipe d’Opérateurs / Contrat d’équipe), **Toolkit**.

## Portée / Scope

- **Public** : Particuliers, foyers (point d’entrée JayBudget).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir tous les besoins du public Purse (compte, mouvements, budgets occasionnels, objectifs, rapports, export, alertes).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Équipe d’Opérateurs** | Collectif gouverné d’Opérateurs qui collaborent sous règles explicites (Contrat d’équipe). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |
| **Service** | Capacité perçue par l’utilisateur ; ici le **service COG JayKonta** exposé via le point d’entrée **JayBudget**. |

Les utilisateurs Purse **interagissent avec** l’Opérateur gouverné « JayBudget » ; cet Opérateur s’appuie sur les Toolkits du service COG JayKonta (mouvements, budgets occasionnels, objectifs, rapports, alertes) et sur Miyauth, Miyunotify, JayKoa (optionnel).

---

## 2. Besoins en Opérateurs (point d’entrée Purse)

### 2.1 Opérateur « JayBudget » (tableau de bord et capacités Purse)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **point d’entrée Purse** : tableau de bord (solde, synthèse), mouvements, budgets occasionnels, objectifs, rapports, export, alertes. |
| **Public servi** | Particuliers et foyers authentifiés (point d’entrée JayBudget, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) pour accéder aux données Purse ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel niveau 2). |
| **Capacités exposées** | Compte Purse (Miyauth) ; enregistrement des mouvements ; catégories ; solde et synthèse ; historique ; budgets occasionnels (création, suivi, liste/détail) ; objectifs (définition, suivi) ; rapports et tableaux de bord ; export PDF/CSV ; alertes budget ; rappels optionnels (JayKoa). |
| **Ne fait pas** | Devis ni facturation légale (réservés au point d’entrée JayKonta). |

Cet Opérateur est le **point d’entrée** unique du public Purse : il agrège toutes les capacités du périmètre Purse et s’appuie sur les Toolkits listés en § 3.

### 2.2 Synthèse des Opérateurs (Purse)

| Opérateur | Usage par l’utilisateur | Livrables couverts |
|-----------|-------------------------|---------------------|
| **JayBudget** | Connexion, tableau de bord, mouvements, catégories, budgets occasionnels, objectifs, rapports, export, alertes. | Tous les besoins PUR-01 à PUR-16. |

*Note :* Selon l’architecture, des sous-Opérateurs (ex. Purse Budgets occasionnels, Purse Objectifs) peuvent être distingués au sein de l’Équipe d’Opérateurs Purse ; le **Contrat d’équipe** définit les flux entre eux. Pour la traçabilité besoin → réponse, l’**Opérateur** de référence reste « JayBudget » (point d’entrée).

---

## 3. Besoins en Toolkits (point d’entrée Purse)

### 3.1 Kit « Compte Purse » (Miyauth / JayKonta)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Authentification et compte Purse : inscription, connexion, déconnexion, récupération mot de passe ; pas d’exigence SIRET ni facturation. |
| **Outils agrégés (exemples)** | `auth.register`, `auth.login`, `auth.logout`, `auth.resetPassword`, `profile.get`, `profile.update` (identité minimale). |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | Miyauth, Master Butler (permissions), WorrySentinel (niveau 2). |

### 3.2 Kit « Budget Mouvements Purse » (JayKonta)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Enregistrement des revenus et dépenses, catégories, solde, synthèse, historique, export liste. |
| **Outils agrégés (exemples)** | `budget.movements.record`, `budget.movements.list`, `budget.categories.list`, `budget.categories.update`, `report.balance`, `report.export` (CSV mouvements). |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | KindMother (persistance), WorrySentinel (niveau 2). |

### 3.3 Kit « Budgets occasionnels Purse » (JayKonta)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Création et suivi des budgets occasionnels (vacances, Noël, mariage, travaux) : montant cible/plafond, dépenses affectées, solde, archivage/clôture. |
| **Outils agrégés (exemples)** | `budget.occasionnel.create`, `budget.occasionnel.list`, `budget.occasionnel.get`, `budget.occasionnel.assignMovement`, `budget.occasionnel.close`, `budget.occasionnel.balance`. |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | KindMother, WorrySentinel (niveau 2). |

### 3.4 Kit « Objectifs Purse » (JayKonta)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Définition et suivi des objectifs d’épargne ou de dépense ; lien optionnel à un budget occasionnel ou une catégorie ; alerte si atteint ou en retard. |
| **Outils agrégés (exemples)** | `budget.objective.create`, `budget.objective.list`, `budget.objective.get`, `budget.objective.progress`, `budget.objective.alert`. |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | KindMother, Miyunotify (notifications optionnelles), WorrySentinel (niveau 2). |

### 3.5 Kit « Rapports et Export Purse » (JayKonta)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Rapports prédéfinis (mensuel, trimestriel, annuel), tableaux de bord, export PDF (synthèse, rapport) et CSV (mouvements) ; périmètre utilisateur, niveau 2. |
| **Outils agrégés (exemples)** | `report.balance`, `report.byCategory`, `report.byPeriod`, `report.export.pdf`, `report.export.csv`. |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | KindMother, WorrySentinel (pas d’export au-delà du niveau autorisé). |

### 3.6 Kit « Alertes Purse » (JayKonta / Miyunotify)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Configuration et envoi des alertes : dépassement budget occasionnel, objectif atteint/en retard, solde sous seuil ; notification email et/ou in-app. |
| **Outils agrégés (exemples)** | `alert.config.set`, `alert.config.get`, `alert.trigger` (déclenchement selon règles), intégration Miyunotify (envoi email/in-app). |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | Miyunotify, KindMother (données budget/objectifs). |

### 3.7 Kit « Rappels Agenda Purse » (optionnel — JayKoa)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Références temporelles vers JayKoa pour rappels (échéance objectif, clôture budget) ; **pas de donnée financière canonique** dans l’agenda ; source de vérité reste JayKonta. |
| **Outils agrégés (exemples)** | `agenda.reminder.create` (date, type, id opaque), `agenda.reminder.list`. |
| **Consommé par** | JayBudget. |
| **Composants sous-jacents** | JayKoa (références uniquement). |

### 3.8 Synthèse des Toolkits (Purse)

| Toolkit | Opérateur consommateur | Livrables couverts |
|---------|------------------------|---------------------|
| **Compte Purse** | JayBudget | PUR-01, PUR-02, PUR-03 (compte, session, données niveau 2). |
| **Budget Mouvements Purse** | JayBudget | PUR-04, PUR-05, PUR-06, PUR-07 (mouvements, catégories, solde, historique). |
| **Budgets occasionnels Purse** | JayBudget | PUR-08, PUR-09, PUR-10. |
| **Objectifs Purse** | JayBudget | PUR-11, PUR-12. |
| **Rapports et Export Purse** | JayBudget | PUR-13, PUR-14. |
| **Alertes Purse** | JayBudget | PUR-15. |
| **Rappels Agenda Purse** | JayBudget | PUR-16 (optionnel). |

---

## 4. Matrice Besoin → Service / Opérateur / Toolkit (exhaustive)

Chaque besoin dispose d’une **réponse explicite** par **Service**, **Opérateur** et **Toolkit**.

### 4.1 Besoins fonctionnels (PUR-01 à PUR-16)

| Id besoin | Besoin | Service | Opérateur | Toolkit(s) |
|-----------|--------|---------|-----------|------------|
| **PUR-01** | Création de compte Purse | JayKonta (COG), point d’entrée JayBudget | JayBudget | Compte Purse (Miyauth) |
| **PUR-02** | Connexion et session | JayKonta (COG), point d’entrée JayBudget | JayBudget | Compte Purse (Miyauth) |
| **PUR-03** | Données personnelles (niveau 2, résidence) | JayKonta (COG), point d’entrée JayBudget | JayBudget | Gouvernance (WorrySentinel, KindMother) — appliqué à tous les Toolkits Purse |
| **PUR-04** | Enregistrement des mouvements | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budget Mouvements Purse |
| **PUR-05** | Catégories | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budget Mouvements Purse |
| **PUR-06** | Solde et synthèse | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budget Mouvements Purse, Rapports et Export Purse |
| **PUR-07** | Historique des mouvements | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budget Mouvements Purse, Rapports et Export Purse |
| **PUR-08** | Création d’un budget occasionnel | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budgets occasionnels Purse |
| **PUR-09** | Suivi des dépenses par budget occasionnel | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budgets occasionnels Purse |
| **PUR-10** | Liste et détail des budgets occasionnels | JayKonta (COG), point d’entrée JayBudget | JayBudget | Budgets occasionnels Purse |
| **PUR-11** | Définition d’objectifs | JayKonta (COG), point d’entrée JayBudget | JayBudget | Objectifs Purse |
| **PUR-12** | Suivi des objectifs | JayKonta (COG), point d’entrée JayBudget | JayBudget | Objectifs Purse |
| **PUR-13** | Rapports et tableaux de bord | JayKonta (COG), point d’entrée JayBudget | JayBudget | Rapports et Export Purse |
| **PUR-14** | Export (PDF, CSV) | JayKonta (COG), point d’entrée JayBudget | JayBudget | Rapports et Export Purse |
| **PUR-15** | Alertes budget | JayKonta (COG), point d’entrée JayBudget | JayBudget | Alertes Purse |
| **PUR-16** | Rappels (optionnel, Agenda) | JayKonta (COG), point d’entrée JayBudget | JayBudget | Rappels Agenda Purse (optionnel) |

### 4.2 Besoins non fonctionnels (NFR-PUR-01 à NFR-PUR-07)

| Id besoin | Besoin | Service | Opérateur | Toolkit / gouvernance |
|-----------|--------|---------|-----------|------------------------|
| **NFR-PUR-01** | Données au minimum niveau 2 (Sensitive) | JayKonta (COG) | JayBudget | WorrySentinel, tous les Toolkits Purse (flux chiffrés, Mandat) |
| **NFR-PUR-02** | Résidence des données | JayKonta (COG) | JayBudget | KindMother, contrat du service (COG de référence ou environnement utilisateur) |
| **NFR-PUR-03** | Audit des accès | JayKonta (COG) | JayBudget | WorrySentinel, Master Butler (traçabilité lectures/écritures) |
| **NFR-PUR-04** | Temps de chargement tableau de bord | JayKonta (COG) | JayBudget | Budget Mouvements Purse, Rapports et Export Purse (performance) |
| **NFR-PUR-05** | Saisie des mouvements (< 2 s) | JayKonta (COG) | JayBudget | Budget Mouvements Purse |
| **NFR-PUR-06** | Interface simple et claire | JayKonta (COG), point d’entrée JayBudget | JayBudget | UX couvrant tous les Toolkits Purse |
| **NFR-PUR-07** | Mobile et desktop | JayKonta (COG), point d’entrée JayBudget | JayBudget | Tous les Toolkits Purse (responsive) |

---

## 5. Matrice Parcours / Livrables / Opérateur / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) | Service |
|----------------------|-----------|------------|---------|
| Onboarding (inscription Purse) | JayBudget | Compte Purse (Miyauth) | JayKonta (COG), point d’entrée JayBudget |
| Tableau de bord (solde, synthèse) | JayBudget | Budget Mouvements Purse, Rapports et Export Purse | JayKonta (COG), point d’entrée JayBudget |
| Saisie mouvements, catégories | JayBudget | Budget Mouvements Purse | JayKonta (COG), point d’entrée JayBudget |
| Budgets occasionnels (vacances, Noël, etc.) | JayBudget | Budgets occasionnels Purse | JayKonta (COG), point d’entrée JayBudget |
| Objectifs d’épargne / dépense | JayBudget | Objectifs Purse | JayKonta (COG), point d’entrée JayBudget |
| Rapports, export PDF/CSV | JayBudget | Rapports et Export Purse | JayKonta (COG), point d’entrée JayBudget |
| Alertes (dépassement, objectif, seuil) | JayBudget | Alertes Purse | JayKonta (COG), point d’entrée JayBudget |
| Rappels (échéance, clôture budget) | JayBudget | Rappels Agenda Purse (optionnel) | JayKonta (COG), point d’entrée JayBudget |

---

## 6. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Compte, session | Miyauth | Authentification, récupération mot de passe. |
| Permissions, Mandat | Master Butler, StrongFather | Permissions Purse, émission Mandat. |
| Persistance, résidence | KindMother | Données mouvements, budgets occasionnels, objectifs ; résidence selon contrat. |
| Niveau 2, audit | WorrySentinel | Classification niveau 2, traçabilité. |
| Alertes, notifications | Miyunotify | Envoi email/in-app pour alertes. |
| Rappels (optionnel) | JayKoa | Références temporelles (pas de donnée financière). |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Purse - Analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) | Liste exhaustive des besoins PUR-01 à PUR-16, NFR-PUR-01 à NFR-PUR-07. |
| [Purse - Parcours Capacites Livrables](./Purse%20-%20Parcours%20Capacites%20Livrables.md) | Parcours, capacités et livrables Purse. |
| [JayKonta - Document Fondateur](../../JayKonta%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée Purse/Account. |
| [Points d’entrée Purse et Account](../../reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | Périmètre Purse, capacités exposées. |
| [Glossaire Miyukini](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Opérateur, Toolkit, Mandat, Service. |

---

**Document** : JayBudget — Operateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (réponse besoin → Service / Opérateur / Toolkit)
