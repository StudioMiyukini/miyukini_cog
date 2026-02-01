# Organisateurs — Parcours, capacités et livrables

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **livrables** du public cible **Organisateurs** dans le cadre du service Miyukini Festival Service. Il complète le [document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Public** : Organisateurs (structures qui créent et gèrent des événements/festivals).
- **Périmètre** : onboarding, espace dédié, rôles, capacités métier, livrables, limites.
- **Hors périmètre** : spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Associations, collectivités, sociétés, toute structure organisant des événements ou festivals. |
| **Compte** | Cross-événements : un même organisateur gère **plusieurs éditions** (festivals/événements). |
| **Accès** | Authentification (Miyauth), permissions (Master Butler), Mandat de Permission pour gérer éditions et exposants. |
| **Espace** | Tableau de bord organisateur ; liste de toutes ses éditions ; dashboard par édition. |

---

## 2. Parcours utilisateur

### 2.1 Onboarding

1. **Création de compte** : inscription en tant qu’organisateur (Miyauth, Miyuprofile).
2. **Validation** : selon politique plateforme (validation manuelle ou automatique).
3. **Attribution des permissions** : rôle organisateur (Admin ou Manager), émission du Mandat de Permission (StrongFather, Master Butler).
4. **Première édition** : création d’une première édition (événement) ou rattachement à une structure existante.

Le compte est **cross-événements** dès l’origine : l’organisateur peut ajouter autant d’éditions que son Mandat l’autorise.

### 2.2 Parcours type (cycle de vie)

| Étape | Action | Résultat |
|-------|--------|----------|
| **Connexion** | Connexion avec identifiants organisateur. | Accès au tableau de bord organisateur. |
| **Vue d’ensemble** | Consultation de la liste de **toutes ses éditions** (multi-festivals). | Liste des événements en cours, à venir, passés. |
| **Choix d’une édition** | Clic sur une édition. | Accès au **dashboard édition** (plan, programme, exposants, candidatures, budget, documents). |
| **Configuration** | Paramétrage de l’édition (dates, lieu, thème, règles) et de l’équipe (rôles, bénévoles). | Édition prête à recevoir candidatures exposants et visiteurs. |
| **Publication** | Demande de publication au catalogue (annuaire des événements). | Édition visible dans l’annuaire (selon politique plateforme). |
| **Exploitation** | Gestion des candidatures, plan de salle, programme, budget, documents, services visiteur. | Édition opérationnelle. |
| **Clôture** | Clôture de l’édition, débriefing, archivage. | Données conservées ; édition passée. |

### 2.3 Points de sortie / passerelles

- **Vers exposants** : l’organisateur consulte et gère les exposants de chaque édition ; les exposants ont leur propre dashboard (voir [Public Exposants](../Exposants/_index.md)).
- **Vers visiteurs** : l’organisateur active les services visiteur (jeux, concours, ateliers, etc.) par édition ; les visiteurs accèdent à ces services via leur espace (voir [Public Visiteurs](../Visiteurs/_index.md)).
- **Vers catalogue** : publication des éditions et de la fiche organisateur dans l’annuaire et le répertoire des organisateurs (visible par [utilisateur non connecté](../UtilisateurNonConnecte/_index.md) et tous les publics).

---

## 3. Rôles côté organisateur

| Rôle | Périmètre | Capacités principales |
|------|-----------|------------------------|
| **Admin organisateur** | Gestion complète de la structure. | **Toutes ses éditions** (multi-festivals), équipe, paramètres, publication catalogue, activation des services visiteur. |
| **Manager** | Gestion opérationnelle d’une ou plusieurs éditions. | Exposants, plan de salle, programme, budget, documents, candidatures, au sein des éditions qui lui sont assignées. |
| **Bénévole** | Terrain, zones, créneaux. | Accès limité selon attribution (zones, créneaux, informations de terrain) ; pas d’accès à la configuration ni au budget. |

*Note :* le rôle **Exposant** correspond à un **compte exposant** distinct (dashboard exposant) ; une même personne peut avoir un compte organisateur et un compte exposant, avec des Mandats différents.

---

## 4. Capacités et livrables

### 4.1 Éditions (multi-festivals)

- **Création** d’une ou plusieurs éditions (événements).
- **Paramétrage** par édition : nom, dates, lieu, thème, règles, objectifs.
- **Tableau de bord par édition** : vue synthétique (exposants, candidatures, budget, programme, plan).
- **Liste globale** : toutes les éditions de l’organisateur (passées, en cours, à venir).

### 4.2 Exposants

- **Annuaire local** : liste des exposants par édition.
- **Candidatures** : réception, consultation, validation ou refus des candidatures exposants.
- **Fiches exposants** : coordonnées, statut, documents, emplacement attribué.
- **Devis et factures** (Miyuinvoice) : génération, envoi, suivi des paiements.

### 4.3 Plan de salle

- **Zones et stands** : définition des zones, tailles de stand, légende.
- **Attribution** : attribution des emplacements aux exposants (formulaire ou drag & drop).
- **Export** : export visuel pour impressions, marquage des zones techniques ou réservées.

### 4.4 Programme

- **Animations** : ajout, modification, suppression d’animations.
- **Salles / scènes** : association des animations à une scène, une salle ou un lieu.
- **Horaires** : gestion des créneaux, blocage des chevauchements.
- **Vues** : chronologique ou par salle ; filtres (jour, scène, type d’activité).

### 4.5 Budget

- **Revenus et dépenses** : saisie, ventilation par catégorie.
- **Statistiques et balance** : par édition ou période (Miyucptaledger, Miyuexpense, Miyucomptareports).

### 4.6 Documents et légal

- **Contrats types** : CGV, conventions, règlements (Miyucms, Miyumedia).
- **Historique** : documents validés ou partagés avec les exposants.
- **Accès** : restreint selon rôle (Master Butler).

### 4.7 Notifications et communication

- **Annonces globales** : diffusion d’annonces (ex. changement de programme) (Miyunotify).
- **Notifications ciblées** : par rôle, par équipe.
- **Paramétrage** : activation et règles par édition.

### 4.8 Services visiteur (activables par l’organisateur)

L’organisateur **choisit** quels services proposer aux visiteurs pour chaque édition :

- Jeux, concours, inscriptions ateliers, réservations, pass VIP, notifications.
- Configuration par édition : places limitées, dates, publics éligibles.

La plateforme fournit les capacités ; l’organisateur les active et les paramètre sans modifier la gouvernance.

### 4.9 Publication au catalogue

- **Annuaire des événements** : les éditions validées peuvent être exposées (liste/carte, filtres, fiche événement).
- **Répertoire des organisateurs** : la fiche organisateur (nom, événements, contact, charte) est visible dans le catalogue.
- **Répertoire des exposants** : selon politique plateforme, les exposants de l’édition peuvent apparaître dans le répertoire.

---

## 5. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Données** | Les données de chaque édition sont souveraines à l’organisateur dans le cadre de son Mandat ; pas d’accès aux données des autres organisateurs. |
| **Macro** | Le catalogue (annuaires, recherche) reste sous contrôle plateforme ; l’organisateur ne modifie pas la gouvernance ni les Kits. |
| **Mandat** | Les actions (créer des éditions, gérer des exposants, publier) sont encadrées par le Mandat de Permission (StrongFather, Master Butler). |
| **Révocation** | Le Mandat peut être révoqué (conditions définies par la gouvernance) ; l’organisateur perd alors l’accès aux actions concernées. |

---

## 6. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) — § 4 Distribution organisateur
- [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
