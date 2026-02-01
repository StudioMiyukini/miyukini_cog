# Utilisateur non connecté — Parcours et accès

## Contexte

Ce document détaille le **parcours** et les **accès** du public **Utilisateur non connecté** dans le cadre du service Miyukini Festival Service. Il complète le [document fondateur](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md).

## Portée / Scope

- **Public** : Toute personne accédant au catalogue **sans compte** ni authentification.
- **Périmètre** : accès au catalogue (Façade publique gouvernée), parcours de découverte, limites, passerelles vers inscription/connexion.
- **Hors périmètre** : spécifications techniques d’implémentation (Opérateurs, Kits, API).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Visiteurs du site ou de l’application sans compte : curieux, futurs organisateurs, exposants ou visiteurs, presse, partenaires. |
| **Compte** | Aucun — **non connecté**. |
| **Accès** | **Façade publique gouvernée** : catalogue en **lecture seule** (annuaire des événements, répertoire des organisateurs, répertoire des exposants). |
| **Objectif** | **Découvrir** les événements, les organisateurs et les exposants ; s’informer ; décider de s’inscrire ou de se connecter pour accéder aux espaces dédiés. |

---

## 2. Parcours utilisateur

### 2.1 Parcours type

| Étape | Action | Résultat |
|-------|--------|----------|
| **Arrivée** | Accès au site ou à l’application (landing, accueil). | Affichage de la **Façade publique** : accès au catalogue et aux appels à l’action (inscription, connexion). |
| **Exploration du catalogue** | Consultation de l’**annuaire des événements** : liste/carte, filtres (date, lieu, organisateur, thème). | Liste des événements publiés ; accès aux **fiches événement** (présentation, dates, lieu, organisateur, exposants, programme public). |
| **Exploration des organisateurs** | Consultation du **répertoire des organisateurs** : liste, fiches organisateur (nom, événements, contact, charte). | Identification et confiance dans les organisateurs. |
| **Exploration des exposants** | Consultation du **répertoire des exposants** : liste (global ou par événement), fiches exposant (entreprise, stands, éditions participées, contact). | Découverte des exposants et mise en relation. |
| **Recherche** | Utilisation de la **recherche** (Miyusearch) et des **filtres** sur les trois piliers du catalogue. | Résultats ciblés (événements, organisateurs, exposants). |
| **Décision** | Choix de **s’inscrire** ou de **se connecter** pour accéder à un espace dédié (organisateur, exposant, visiteur). | Redirection vers inscription/connexion ; après authentification, accès à l’espace correspondant au type de compte. |

### 2.2 Contenu accessible (lecture seule)

| Pilier | Contenu visible |
|--------|-----------------|
| **Annuaire des événements** | Liste/carte des éditions (événements) publiées ; filtres (date, lieu, organisateur, thème) ; fiche événement (présentation, dates, lieu, organisateur, exposants, programme public). |
| **Répertoire des organisateurs** | Liste des structures organisatrices ; fiche organisateur (nom, événements, contact, charte). |
| **Répertoire des exposants** | Liste des exposants (global ou par événement) ; fiche exposant (entreprise, stands, éditions participées, contact). |

La **macro** (catalogue, annuaires, recherche) est gérée au niveau plateforme ; la Façade publique expose uniquement les données **publiées** par les organisateurs et la plateforme (pas de données privées ni d’espaces dédiés).

### 2.3 Passerelles vers connexion / inscription

| Action | Cible | Résultat |
|--------|--------|----------|
| **S’inscrire en tant qu’organisateur** | [Public Organisateurs](../Organisateurs/_index.md) | Création de compte organisateur ; accès à l’espace organisateur (tableau de bord, éditions, exposants, etc.). |
| **S’inscrire en tant qu’exposant** | [Public Exposants](../Exposants/_index.md) | Création de compte exposant ; accès au dashboard exposant (candidatures, participations, agenda, documents, factures). |
| **S’inscrire en tant que visiteur** | [Public Visiteurs](../Visiteurs/_index.md) | Création de compte visiteur ; accès à l’espace dédié visiteur (agenda, billets, réservations, pass VIP, jeux, concours, ateliers). |
| **Se connecter** | Selon type de compte existant | Accès à l’espace correspondant (organisateur, exposant, visiteur). |

---

## 3. Limites et gouvernance

| Aspect | Règle |
|--------|--------|
| **Lecture seule** | L’utilisateur non connecté **ne peut pas** : créer ou modifier des événements, déposer une candidature exposant, réserver un atelier, acheter un billet, participer à un jeu ou concours, accéder aux espaces dédiés. |
| **Façade publique gouvernée** | Le catalogue est exposé selon les règles de la plateforme (WorrySentinel, Master Butler) ; seules les données **publiées** et **autorisées pour le public** sont visibles. |
| **Données** | Aucune donnée personnelle de l’utilisateur non connecté n’est stockée pour le catalogue (hors cookies ou analytics selon politique plateforme) ; la création de compte ou la connexion déclenche le traitement des données selon le type de compte. |
| **Sécurité** | L’accès public reste soumis à la gouvernance (niveaux de sécurité, états de confiance) ; pas d’accès aux données sensibles ni aux espaces protégés. |

---

## 4. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) — § 3 Macro (Trois piliers du catalogue), § 7.2 Publics cibles
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md)
