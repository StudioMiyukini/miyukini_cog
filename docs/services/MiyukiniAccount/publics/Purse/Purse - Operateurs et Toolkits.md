# Miyukini Purse — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du point d’entrée **Miyukini Purse** du service COG Miyukini Account. Il s’appuie sur l’[analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) et le document [Parcours, capacités et livrables](./Purse%20-%20Parcours%20Capacites%20Livrables.md). Il vise à fournir une **réponse explicite** pour chaque besoin : **Service**, **Opérateur** (ou Équipe d’Opérateurs / Contrat d’équipe), **Toolkit**.

## Portée / Scope

- **Public** : Particuliers, foyers (point d’entrée Miyukini Purse).
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
| **Service** | Capacité perçue par l’utilisateur ; ici le **service COG Miyukini Account** exposé via le point d’entrée **Miyukini Purse**. |

Les utilisateurs Purse **interagissent avec** l’Opérateur gouverné « Miyukini Purse » ; cet Opérateur s’appuie sur les Toolkits du service COG Miyukini Account (mouvements, budgets occasionnels, objectifs, rapports, alertes) et sur Miyauth, Miyunotify, Miyukini Agenda (optionnel).

---

## 2. Besoins en Opérateurs (point d’entrée Purse)

### 2.1 Opérateur « Miyukini Purse » (tableau de bord et capacités Purse)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **point d’entrée Purse** : tableau de bord (solde, synthèse), mouvements, budgets occasionnels, objectifs, rapports, export, alertes. |
| **Public servi** | Particuliers et foyers authentifiés (point d’entrée Miyukini Purse, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) pour accéder aux données Purse ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel niveau 2). |
| **Capacités exposées** | Compte Purse (Miyauth) ; enregistrement des mouvements ; catégories ; solde et synthèse ; historique ; budgets occasionnels (création, suivi, liste/détail) ; objectifs (définition, suivi) ; rapports et tableaux de bord ; export PDF/CSV ; alertes budget ; rappels optionnels (Miyukini Agenda). |
| **Ne fait pas** | Devis ni facturation légale (réservés au point d’entrée Miyukini Account). |

Cet Opérateur est le **point d’entrée** unique du public Purse : il agrège toutes les capacités du périmètre Purse et s’appuie sur les Toolkits listés en § 3.

### 2.2 Synthèse des Opérateurs (Purse)

| Opérateur | Usage par l’utilisateur | Livrables couverts |
|-----------|-------------------------|---------------------|
| **Miyukini Purse** | Connexion, tableau de bord, mouvements, catégories, budgets occasionnels, objectifs, rapports, export, alertes. | Tous les besoins PUR-01 à PUR-16. |

*Note :* Selon l’architecture, des sous-Opérateurs (ex. Purse Budgets occasionnels, Purse Objectifs) peuvent être distingués au sein de l’Équipe d’Opérateurs Purse ; le **Contrat d’équipe** définit les flux entre eux. Pour la traçabilité besoin → réponse, l’**Opérateur** de référence reste « Miyukini Purse » (point d’entrée).

---

## 3. Besoins en Toolkits (point d’entrée Purse)

### 3.1 Kit « Compte Purse » (Miyauth / Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Authentification et compte Purse : inscription, connexion, déconnexion, récupération mot de passe ; pas d’exigence SIRET ni facturation. |
| **Outils agrégés (exemples)** | `auth.register`, `auth.login`, `auth.logout`, `auth.resetPassword`, `profile.get`, `profile.update` (identité minimale). |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | Miyauth, Master Butler (permissions), WorrySentinel (niveau 2). |

### 3.2 Kit « Budget Mouvements Purse » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Enregistrement des revenus et dépenses, catégories, solde, synthèse, historique, export liste. |
| **Outils agrégés (exemples)** | `budget.movements.record`, `budget.movements.list`, `budget.categories.list`, `budget.categories.update`, `report.balance`, `report.export` (CSV mouvements). |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | KindMother (persistance), WorrySentinel (niveau 2). |

### 3.3 Kit « Budgets occasionnels Purse » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Création et suivi des budgets occasionnels (vacances, Noël, mariage, travaux) : montant cible/plafond, dépenses affectées, solde, archivage/clôture. |
| **Outils agrégés (exemples)** | `budget.occasionnel.create`, `budget.occasionnel.list`, `budget.occasionnel.get`, `budget.occasionnel.assignMovement`, `budget.occasionnel.close`, `budget.occasionnel.balance`. |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | KindMother, WorrySentinel (niveau 2). |

### 3.4 Kit « Objectifs Purse » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Définition et suivi des objectifs d’épargne ou de dépense ; lien optionnel à un budget occasionnel ou une catégorie ; alerte si atteint ou en retard. |
| **Outils agrégés (exemples)** | `budget.objective.create`, `budget.objective.list`, `budget.objective.get`, `budget.objective.progress`, `budget.objective.alert`. |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | KindMother, Miyunotify (notifications optionnelles), WorrySentinel (niveau 2). |

### 3.5 Kit « Rapports et Export Purse » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Rapports prédéfinis (mensuel, trimestriel, annuel), tableaux de bord, export PDF (synthèse, rapport) et CSV (mouvements) ; périmètre utilisateur, niveau 2. |
| **Outils agrégés (exemples)** | `report.balance`, `report.byCategory`, `report.byPeriod`, `report.export.pdf`, `report.export.csv`. |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | KindMother, WorrySentinel (pas d’export au-delà du niveau autorisé). |

### 3.6 Kit « Alertes Purse » (Miyukini Account / Miyunotify)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Configuration et envoi des alertes : dépassement budget occasionnel, objectif atteint/en retard, solde sous seuil ; notification email et/ou in-app. |
| **Outils agrégés (exemples)** | `alert.config.set`, `alert.config.get`, `alert.trigger` (déclenchement selon règles), intégration Miyunotify (envoi email/in-app). |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | Miyunotify, KindMother (données budget/objectifs). |

### 3.7 Kit « Rappels Agenda Purse » (optionnel — Miyukini Agenda)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Références temporelles vers Miyukini Agenda pour rappels (échéance objectif, clôture budget) ; **pas de donnée financière canonique** dans l’agenda ; source de vérité reste Miyukini Account. |
| **Outils agrégés (exemples)** | `agenda.reminder.create` (date, type, id opaque), `agenda.reminder.list`. |
| **Consommé par** | Miyukini Purse. |
| **Composants sous-jacents** | Miyukini Agenda (références uniquement). |

### 3.8 Synthèse des Toolkits (Purse)

| Toolkit | Opérateur consommateur | Livrables couverts |
|---------|------------------------|---------------------|
| **Compte Purse** | Miyukini Purse | PUR-01, PUR-02, PUR-03 (compte, session, données niveau 2). |
| **Budget Mouvements Purse** | Miyukini Purse | PUR-04, PUR-05, PUR-06, PUR-07 (mouvements, catégories, solde, historique). |
| **Budgets occasionnels Purse** | Miyukini Purse | PUR-08, PUR-09, PUR-10. |
| **Objectifs Purse** | Miyukini Purse | PUR-11, PUR-12. |
| **Rapports et Export Purse** | Miyukini Purse | PUR-13, PUR-14. |
| **Alertes Purse** | Miyukini Purse | PUR-15. |
| **Rappels Agenda Purse** | Miyukini Purse | PUR-16 (optionnel). |

---

## 4. Matrice Besoin → Service / Opérateur / Toolkit (exhaustive)

Chaque besoin dispose d’une **réponse explicite** par **Service**, **Opérateur** et **Toolkit**.

### 4.1 Besoins fonctionnels (PUR-01 à PUR-16)

| Id besoin | Besoin | Service | Opérateur | Toolkit(s) |
|-----------|--------|---------|-----------|------------|
| **PUR-01** | Création de compte Purse | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Compte Purse (Miyauth) |
| **PUR-02** | Connexion et session | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Compte Purse (Miyauth) |
| **PUR-03** | Données personnelles (niveau 2, résidence) | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Gouvernance (WorrySentinel, KindMother) — appliqué à tous les Toolkits Purse |
| **PUR-04** | Enregistrement des mouvements | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budget Mouvements Purse |
| **PUR-05** | Catégories | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budget Mouvements Purse |
| **PUR-06** | Solde et synthèse | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budget Mouvements Purse, Rapports et Export Purse |
| **PUR-07** | Historique des mouvements | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budget Mouvements Purse, Rapports et Export Purse |
| **PUR-08** | Création d’un budget occasionnel | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budgets occasionnels Purse |
| **PUR-09** | Suivi des dépenses par budget occasionnel | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budgets occasionnels Purse |
| **PUR-10** | Liste et détail des budgets occasionnels | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Budgets occasionnels Purse |
| **PUR-11** | Définition d’objectifs | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Objectifs Purse |
| **PUR-12** | Suivi des objectifs | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Objectifs Purse |
| **PUR-13** | Rapports et tableaux de bord | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Rapports et Export Purse |
| **PUR-14** | Export (PDF, CSV) | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Rapports et Export Purse |
| **PUR-15** | Alertes budget | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Alertes Purse |
| **PUR-16** | Rappels (optionnel, Agenda) | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Rappels Agenda Purse (optionnel) |

### 4.2 Besoins non fonctionnels (NFR-PUR-01 à NFR-PUR-07)

| Id besoin | Besoin | Service | Opérateur | Toolkit / gouvernance |
|-----------|--------|---------|-----------|------------------------|
| **NFR-PUR-01** | Données au minimum niveau 2 (Sensitive) | Miyukini Account (COG) | Miyukini Purse | WorrySentinel, tous les Toolkits Purse (flux chiffrés, Mandat) |
| **NFR-PUR-02** | Résidence des données | Miyukini Account (COG) | Miyukini Purse | KindMother, contrat du service (COG de référence ou environnement utilisateur) |
| **NFR-PUR-03** | Audit des accès | Miyukini Account (COG) | Miyukini Purse | WorrySentinel, Master Butler (traçabilité lectures/écritures) |
| **NFR-PUR-04** | Temps de chargement tableau de bord | Miyukini Account (COG) | Miyukini Purse | Budget Mouvements Purse, Rapports et Export Purse (performance) |
| **NFR-PUR-05** | Saisie des mouvements (< 2 s) | Miyukini Account (COG) | Miyukini Purse | Budget Mouvements Purse |
| **NFR-PUR-06** | Interface simple et claire | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | UX couvrant tous les Toolkits Purse |
| **NFR-PUR-07** | Mobile et desktop | Miyukini Account (COG), point d’entrée Miyukini Purse | Miyukini Purse | Tous les Toolkits Purse (responsive) |

---

## 5. Matrice Parcours / Livrables / Opérateur / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) | Service |
|----------------------|-----------|------------|---------|
| Onboarding (inscription Purse) | Miyukini Purse | Compte Purse (Miyauth) | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Tableau de bord (solde, synthèse) | Miyukini Purse | Budget Mouvements Purse, Rapports et Export Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Saisie mouvements, catégories | Miyukini Purse | Budget Mouvements Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Budgets occasionnels (vacances, Noël, etc.) | Miyukini Purse | Budgets occasionnels Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Objectifs d’épargne / dépense | Miyukini Purse | Objectifs Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Rapports, export PDF/CSV | Miyukini Purse | Rapports et Export Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Alertes (dépassement, objectif, seuil) | Miyukini Purse | Alertes Purse | Miyukini Account (COG), point d’entrée Miyukini Purse |
| Rappels (échéance, clôture budget) | Miyukini Purse | Rappels Agenda Purse (optionnel) | Miyukini Account (COG), point d’entrée Miyukini Purse |

---

## 6. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Compte, session | Miyauth | Authentification, récupération mot de passe. |
| Permissions, Mandat | Master Butler, StrongFather | Permissions Purse, émission Mandat. |
| Persistance, résidence | KindMother | Données mouvements, budgets occasionnels, objectifs ; résidence selon contrat. |
| Niveau 2, audit | WorrySentinel | Classification niveau 2, traçabilité. |
| Alertes, notifications | Miyunotify | Envoi email/in-app pour alertes. |
| Rappels (optionnel) | Miyukini Agenda | Références temporelles (pas de donnée financière). |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Purse - Analyse des besoins](./Purse%20-%20Analyse%20des%20besoins.md) | Liste exhaustive des besoins PUR-01 à PUR-16, NFR-PUR-01 à NFR-PUR-07. |
| [Purse - Parcours Capacites Livrables](./Purse%20-%20Parcours%20Capacites%20Livrables.md) | Parcours, capacités et livrables Purse. |
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée Purse/Account. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Purse, capacités exposées. |
| [Glossaire Miyukini](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Opérateur, Toolkit, Mandat, Service. |

---

**Document** : Miyukini Purse — Operateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (réponse besoin → Service / Opérateur / Toolkit)
