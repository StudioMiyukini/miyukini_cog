# Miyukini Purse — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du point d’entrée **Miyukini Purse** (perso/individuel) du service COG Miyukini Account. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les personas, ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur Miyukini Account](../../Miyukini%20Account%20-%20Document%20Fondateur.md), [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md), [Niveaux de sécurité et protection](../../reference/Miyukini%20Account%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

## Portée / Scope

- **Public** : Particuliers, foyers (point d’entrée Miyukini Purse).
- **Périmètre** : Tous les besoins identifiés pour ce point d’entrée (budgets personnels, budgets occasionnels, rapports, export).
- **Hors périmètre** : Devis et facturation légale (réservés au point d’entrée Miyukini Account), spécifications techniques détaillées (API, schémas).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) :

| Élément | Description |
|--------|-------------|
| **Documentation autorisée (liste fermée)** | Document fondateur Miyukini Account ; Points d’entrée Purse et Account ; Niveaux sécurité et protection ; Glossaire Miyukini ; Politique de résidence des données sensibles. |
| **Ce document ne fusionne pas** | Avec le Document fondateur, les documents référence ou l’analyse des besoins Account. |
| **Ce document n’anticipe pas** | Les parcours capacités / livrables détaillés ni les spécifications d’Opérateurs/Kits Purse. |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Ne pas anticiper** | Les écrans, flux détaillés ou contrats d’API ne sont pas rédigés dans ce document. |
| ❌ **Ne pas fusionner** | Ce document reste limité au point d’entrée Purse ; pas de mélange avec Account. |
| ❌ **Ne pas corriger hors périmètre** | Les besoins Account, MFS ou JayRDV ne sont pas modifiés depuis ce document. |
| ✅ **Source de vérité** | Ce document est la **référence** pour l’analyse des besoins du point d’entrée Miyukini Purse. |

### Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-PUR-01** | Purse : pas de devis ni de facturation légale. | Périmètre strictement perso/occasionnel ; conformité et différenciation avec Account. |
| **DS-PUR-02** | Données Purse au minimum niveau 2 (Sensitive) ; résidence selon politique (COG ou environnement utilisateur). | Alignement Document fondateur et Politique de résidence ; option perso pour résidence. |
| **DS-PUR-03** | Rappels optionnels via Miyukini Agenda : références temporelles uniquement ; source de vérité financière reste Miyukini Account. | Pas de duplication des données financières dans l’agenda ; lien explicite. |
| **Dépendance critique** | Ce document dépend du Document fondateur Miyukini Account et du document Points d’entrée Purse et Account ; toute évolution du périmètre Purse doit être cohérente avec ces références. | — |

---

## 1. Profil du public et personas

### 1.1 Définition du public

Les **utilisateurs Miyukini Purse** sont des **particuliers** ou des **foyers** qui souhaitent **tenir un budget personnel** et/ou **gérer des budgets occasionnels** (vacances, cadeaux de Noël, mariage, travaux, etc.) sans exigence de facturation légale ni de comptabilité d’entreprise. Ils accèdent au service COG Miyukini Account via le point d’entrée **Miyukini Purse**.

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **Particulier autonome** | Gère seul son budget ; revenus/dépenses simples ; peu de catégories. | Suivre ses dépenses, voir son solde, éviter les découverts. | Applications éparpillées, pas de vue consolidée, oubli des dépenses occasionnelles. |
| **Foyer (couple / famille)** | Budget partagé ou plusieurs budgets (perso + commun) ; vacances, Noël, projets. | Plusieurs budgets (perso, commun, vacances, Noël), objectifs d’épargne, alertes. | Qui dépense quoi, budget Noël dépassé, pas de rappel pour les échéances. |
| **Projet occasionnel** | Budget dédié à un projet court (mariage, travaux, voyage). | Un budget par projet, suivi des dépenses et du solde, alerte si dépassement. | Mélange avec le budget courant, oubli des dépenses projet. |
| **Épargnant cible** | Objectifs d’épargne par catégorie ou projet (vacances, voiture, imprévus). | Définir des objectifs, suivre l’évolution, être alerté quand objectif atteint ou en retard. | Pas de suivi par objectif, pas de rappel. |

### 1.3 Contexte d’usage

- **Fréquence** : connexion régulière (saisie des dépenses, consultation solde) ou ponctuelle (budget occasionnel, rapport).
- **Appareils** : desktop et mobile (saisie rapide, consultation, export).
- **Concurrence** : applications budget perso, tableurs ; attente d’une **expérience simple**, **sécurisée** et **sans facturation légale**.

---

## 2. Besoins fonctionnels

### 2.1 Compte et accès

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-01 | Création de compte Purse | Pouvoir créer un compte Miyukini Purse (email, mot de passe ou lien magique, identité minimale). | Formulaire d’inscription dédié Purse ; validation email si configurée ; pas d’exigence SIRET ni de facturation. |
| PUR-02 | Connexion et session | Se connecter et gérer sa session (connexion, déconnexion, récupération mot de passe). | Connexion sécurisée (Miyauth) ; session gouvernée par Mandat ; déconnexion et récupération mot de passe. |
| PUR-03 | Données personnelles | Les données Purse (mouvements, budgets occasionnels) sont au minimum niveau 2 (Sensitive) ; résidence selon politique (COG de référence ou environnement utilisateur). | Niveau WorrySentinel 2 ; résidence définie par contrat ; pas d’exposition hors Mandat. |

### 2.2 Budget personnel (mouvements, catégories)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-04 | Enregistrement des mouvements | Enregistrer des revenus et dépenses (date, montant, libellé, catégorie). | Saisie manuelle ou import (CSV) ; catégories configurables (alimentation, transport, loisirs, etc.) ; correction et suppression selon règles. |
| PUR-05 | Catégories | Définir et utiliser des catégories (revenus/dépenses) pour ventiler les mouvements. | Liste de catégories personnalisables ; sous-catégories optionnelles ; catégories par défaut proposées. |
| PUR-06 | Solde et synthèse | Consulter le solde courant et une synthèse (revenus vs dépenses, par catégorie, par période). | Tableau de bord : solde, évolution, répartition par catégorie ; filtres par période (mois, trimestre, année). |
| PUR-07 | Historique des mouvements | Consulter l’historique des mouvements (liste, filtres, recherche). | Liste paginée et filtrable (date, catégorie, montant) ; recherche par libellé ; export liste (CSV) pour usage personnel. |

### 2.3 Budgets occasionnels

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-08 | Création d’un budget occasionnel | Créer un budget dédié à un projet ou une occasion (vacances, Noël, mariage, travaux). | Nom du budget, montant cible ou plafond, date de début/fin optionnelle ; un utilisateur peut avoir plusieurs budgets occasionnels. |
| PUR-09 | Suivi des dépenses par budget occasionnel | Enregistrer des dépenses affectées à un budget occasionnel et suivre le solde (dépensé / restant). | Affectation d’un mouvement à un budget occasionnel ; solde du budget (montant dépensé, restant) ; alerte si dépassement ou seuil configuré. |
| PUR-10 | Liste et détail des budgets occasionnels | Consulter la liste des budgets occasionnels et le détail de chacun (mouvements, solde, évolution). | Liste des budgets avec indicateur (solde, % utilisé) ; fiche par budget (mouvements, solde, objectif) ; archivage ou clôture quand projet terminé. |

### 2.4 Objectifs

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-11 | Définition d’objectifs | Définir des objectifs d’épargne ou de dépense (ex. épargne vacances 2000 €, dépenses loisirs < 300 €/mois). | Objectif : libellé, montant cible, période ou récurrent ; lien optionnel à un budget occasionnel ou à une catégorie. |
| PUR-12 | Suivi des objectifs | Consulter l’évolution des objectifs (atteint, en cours, en retard) et être alerté. | Tableau de bord objectifs : progression, alerte si objectif atteint ou en retard ; notification (Miyunotify) optionnelle. |

### 2.5 Rapports et export

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-13 | Rapports et tableaux de bord | Consulter des rapports (synthèse, évolution, répartition par catégorie, comparaison de périodes). | Rapports prédéfinis (mensuel, trimestriel, annuel) ; graphiques et tableaux ; pas de données au-delà du niveau autorisé (niveau 2). |
| PUR-14 | Export (PDF, CSV) | Exporter des données pour usage personnel (liste des mouvements, synthèse, rapport). | Export PDF (synthèse, rapport) ou CSV (mouvements) ; périmètre limité aux données de l’utilisateur ; pas d’export de données de paiement brutes. |

### 2.6 Alertes et rappels

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| PUR-15 | Alertes budget | Être alerté en cas de dépassement d’un budget occasionnel ou d’un objectif, ou si solde sous un seuil. | Configuration des alertes (seuil, budget, objectif) ; notification (Miyunotify) par email et/ou in-app ; préférences utilisateur. |
| PUR-16 | Rappels (optionnel) | Intégration optionnelle avec Miyukini Agenda pour rappels (échéance objectif, clôture budget). | Référence vers Miyukini Agenda (date, type) ; pas de donnée financière canonique dans l’agenda ; source de vérité reste Miyukini Account. |

---

## 3. Besoins non fonctionnels

### 3.1 Sécurité et confidentialité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PUR-01 | Données au minimum niveau 2 (Sensitive) | Tous les mouvements, budgets occasionnels et objectifs sont classés niveau 2 ; flux chiffrés, accès via Mandat. |
| NFR-PUR-02 | Résidence des données | Résidence définie par contrat (COG de référence ou environnement utilisateur) ; pas de copie non gouvernée sur des tiers. |
| NFR-PUR-03 | Audit des accès | Traçabilité des lectures et écritures (qui a consulté/modifié quoi, quand) selon règles WorrySentinel. |

### 3.2 Performance et disponibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PUR-04 | Temps de chargement du tableau de bord | Le tableau de bord Purse se charge en moins de 3 secondes (réseau standard). |
| NFR-PUR-05 | Saisie des mouvements | La saisie d’un mouvement (montant, libellé, catégorie) est enregistrée et visible en moins de 2 secondes. |

### 3.3 Utilisabilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-PUR-06 | Interface simple et claire | Interface orientée particulier (pas de jargon comptable entreprise) ; parcours courts (saisie rapide, vue synthèse). |
| NFR-PUR-07 | Mobile et desktop | Usage possible sur desktop et mobile (responsive ou app dédiée) pour consultation et saisie. |

---

## 4. Priorisation et dépendances

### 4.1 Priorisation (exemple)

| Priorité | Besoins | Justification |
|----------|---------|---------------|
| **P0** | PUR-01 à PUR-07 (compte, mouvements, catégories, solde, historique) | Fondamentaux du budget personnel. |
| **P1** | PUR-08 à PUR-10 (budgets occasionnels) | Différenciation Purse (vacances, Noël, projets). |
| **P2** | PUR-11 à PUR-14 (objectifs, rapports, export) | Valorisation et autonomie utilisateur. |
| **P3** | PUR-15 à PUR-16 (alertes, rappels) | Confort et rétention. |

### 4.2 Dépendances

| Besoin | Dépendance |
|--------|-------------|
| Compte Purse | Miyauth, Master Butler (permissions), WorrySentinel (niveau 2). |
| Mouvements, rapports | Opérateurs et Kits Miyukini Account (COG) : `budget.movements.record`, `report.balance`, `report.export`. |
| Alertes | Miyunotify (notifications). |
| Rappels agenda (optionnel) | Miyukini Agenda (références temporelles uniquement). |

### 4.3 Dépendances explicites (ordre de lecture recommandé)

Pour cohérence inter-documents, l’ordre suivant est recommandé :

| Ordre | Document | Rôle |
|-------|----------|------|
| 1 | [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée, sécurité. |
| 2 | [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Purse, capacités exposées. |
| 3 | Ce document (Purse - Analyse des besoins) | Besoins fonctionnels et non fonctionnels Purse. |

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte, besoins stratégiques, positionnement Purse. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Purse, données, résidence. |
| [Niveaux de sécurité et protection](../../reference/Miyukini%20Account%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Niveaux WorrySentinel, mesures de protection. |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes, décisions structurantes). |

---

**Document** : Miyukini Purse — Analyse des besoins  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document d’analyse (point d’entrée Purse). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
