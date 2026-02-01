# Utilisateur non connecté — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) pour le public **Utilisateur non connecté** du service Miyukini Festival Service. L’utilisateur non connecté n’a pas de compte ; il accède à la **Façade publique gouvernée** (catalogue en lecture seule : annuaire des événements, répertoire des organisateurs, répertoire des exposants). Les Opérateurs et Toolkits décrits ici sont ceux qui **servent** cette Façade.

Il s’appuie sur le [Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md) et les documents associés.

## Portée / Scope

- **Public** : Utilisateur non connecté (toute personne accédant au catalogue sans compte ni authentification).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du catalogue (annuaire événements, répertoire organisateurs, répertoire exposants, recherche, passerelles inscription/connexion).
- **Hors périmètre** : Spécifications d’implémentation ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Façade publique gouvernée** | Zone tampon d’exposition permettant aux utilisateurs externes d’interagir avec un COG **sans y entrer**. Strictement unidirectionnelle ; sans identité persistante obligatoire. |
| **Utilisateur externe** | Consommateur non certifié de services exposés par un COG, sans gouvernance propre. Accès uniquement via Façade publique ; soumis à un Mandat public d’accès. |
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |

L’utilisateur non connecté **ne pénètre pas** dans les espaces dédiés (organisateur, exposant, visiteur) ; il consomme des **surfaces exposées** par un Opérateur Catalogue (annuaire, répertoires, recherche).

---

## 2. Besoins en Opérateurs (public Utilisateur non connecté)

### 2.1 Opérateur « MFS Catalogue » (Façade publique — catalogue)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **catalogue** en **lecture seule** : annuaire des événements, répertoire des organisateurs, répertoire des exposants. Aucune authentification ; aucune donnée privée. |
| **Public servi** | Utilisateurs non connectés (et visiteurs/organisateurs/exposants connectés pour la consultation du catalogue). |
| **Gouvernance** | **Mandat public d’accès** (Border Guard) : exposition limitée aux données **publiées** par les organisateurs et la plateforme ; pas d’accès aux espaces dédiés ni aux données privées. |
| **Capacités exposées** | Liste/carte des événements publiés ; filtres (date, lieu, organisateur, thème) ; fiche événement (présentation, dates, lieu, organisateur, exposants, programme public) ; répertoire des organisateurs (liste, fiches : nom, événements, contact, charte) ; répertoire des exposants (liste globale ou par événement, fiches : entreprise, stands, éditions participées, contact) ; recherche (Miyusearch) sur les trois piliers. |
| **Limites** | Pas de création/modification d’événements, pas de dépôt de candidature exposant, pas de réservation, pas d’achat de billet, pas d’accès aux espaces dédiés. |

Cet Opérateur est le **seul** point d’entrée de l’utilisateur non connecté pour le service Miyukini Festival Service. Les **passerelles** « S’inscrire » et « Se connecter » (organisateur, exposant, visiteur) sont des **liens** vers Miyauth / inscriptions dédiées ; elles ne sont pas des capacités de MFS Catalogue, mais des options d’orientation.

### 2.2 Synthèse des Opérateurs (public Utilisateur non connecté)

| Opérateur | Usage par l’utilisateur non connecté | Livrables couverts |
|-----------|--------------------------------------|---------------------|
| **MFS Catalogue** | Accès au site/application, exploration du catalogue (événements, organisateurs, exposants), recherche, décision de s’inscrire ou se connecter. | Annuaire des événements, répertoire des organisateurs, répertoire des exposants, recherche, passerelles inscription/connexion. |

---

## 3. Besoins en Toolkits (public Utilisateur non connecté)

### 3.1 Kit « Catalogue — Événements » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**affichage de l’annuaire des événements** : liste/carte des éditions publiées, filtres (date, lieu, organisateur, thème), fiche événement (présentation, dates, lieu, organisateur, exposants, programme public). |
| **Outils agrégés (exemples)** | `event.list.public` (éditions publiées), `event.get.public` (fiche événement), `event.filter` (date, lieu, organisateur, thème), `event.map.get` (données pour carte). |
| **Consommé par** | MFS Catalogue. |
| **Composants sous-jacents** | KindMother (données éditions publiées), Border Guard (règles d’exposition). |

### 3.2 Kit « Catalogue — Organisateurs » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**affichage du répertoire des organisateurs** : liste des structures organisatrices, fiche organisateur (nom, événements, contact, charte). |
| **Outils agrégés (exemples)** | `organiser.list.public`, `organiser.get.public` (fiche organisateur), `organiser.events.get.public` (événements de l’organisateur). |
| **Consommé par** | MFS Catalogue. |
| **Composants sous-jacents** | Miyuprofile (fiches), KindMother, Border Guard. |

### 3.3 Kit « Catalogue — Exposants » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**affichage du répertoire des exposants** : liste (globale ou par événement), fiche exposant (entreprise, stands, éditions participées, contact) selon politique plateforme et choix de l’organisateur. |
| **Outils agrégés (exemples)** | `exposant.list.public` (liste des exposants visibles), `exposant.get.public` (fiche exposant), `exposant.list.byEvent.public` (exposants par événement). |
| **Consommé par** | MFS Catalogue. |
| **Composants sous-jacents** | Miyuprofile (fiches), KindMother, Border Guard. |

### 3.4 Kit « Recherche Catalogue » (MFS / Miyusearch)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **recherche** sur les trois piliers du catalogue (événements, organisateurs, exposants) : requête, filtres, résultats ciblés. |
| **Outils agrégés (exemples)** | `search.query` (requête full-text ou structurée), `search.filter.apply`, `search.results.get` (événements, organisateurs, exposants). |
| **Consommé par** | MFS Catalogue. |
| **Composants sous-jacents** | Miyusearch (ou équivalent), KindMother. |

### 3.5 Kit « Façade & Sécurité » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer les **règles d’accès** à la Façade publique : rate limiting (recherche, listes), validation des requêtes, pas d’exposition de données non publiées. |
| **Outils agrégés (exemples)** | `rateLimit.check` (par IP), `exposure.validate` (vérifier que seules les données autorisées sont renvoyées), `publicOnly.filter` (filtrer les champs non publics). |
| **Consommé par** | MFS Catalogue (avant toute réponse). |
| **Composants sous-jacents** | WorrySentinel, Border Guard. |

### 3.6 Synthèse des Toolkits (public Utilisateur non connecté)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Catalogue — Événements** | MFS Catalogue | Annuaire des événements, fiches événement. |
| **Catalogue — Organisateurs** | MFS Catalogue | Répertoire des organisateurs. |
| **Catalogue — Exposants** | MFS Catalogue | Répertoire des exposants. |
| **Recherche Catalogue** | MFS Catalogue | Recherche, filtres. |
| **Façade & Sécurité** | MFS Catalogue | Rate limiting, données publiées uniquement. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Arrivée (landing, accueil) | MFS Catalogue | Façade & Sécurité. |
| Exploration catalogue (événements) | MFS Catalogue | Catalogue — Événements. |
| Exploration catalogue (organisateurs) | MFS Catalogue | Catalogue — Organisateurs. |
| Exploration catalogue (exposants) | MFS Catalogue | Catalogue — Exposants. |
| Recherche, filtres | MFS Catalogue | Recherche Catalogue, Catalogue — Événements / Organisateurs / Exposants. |
| Décision (s’inscrire, se connecter) | Liens vers Miyauth / inscriptions | Hors périmètre Opérateur MFS Catalogue (passerelles). |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Données éditions publiées | KindMother | Persistance des éditions, statut publication. |
| Fiches organisateurs, exposants | Miyuprofile | Données publiques (nom, contact, charte, etc.). |
| Recherche | Miyusearch | Indexation, requêtes full-text ou structurées. |
| Façade publique | Mandat public d’accès, Border Guard | Exposition des seules données autorisées (publiées). |
| Rate limiting, sécurité | WorrySentinel, traçabilité | Protection des endpoints publics. |

---

## 6. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Public Visiteurs](../Visiteurs/_index.md)
- Glossaire Miyukini (Façade publique gouvernée, Utilisateur externe, Opérateur, Kit d’Outils, Mandat public d’accès)

---

**Document** : Utilisateur non connecté — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (Miyukini Festival Service)
