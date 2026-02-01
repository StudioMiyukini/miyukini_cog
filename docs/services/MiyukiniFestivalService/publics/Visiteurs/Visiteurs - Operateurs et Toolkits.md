# Visiteurs — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du public **Visiteurs** du service Miyukini Festival Service. Il s’appuie sur le [Parcours, capacités et services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md) et les documents associés. Il vise à fournir une référence produit pour l’architecture gouvernée : quels Opérateurs exposent les capacités aux visiteurs, et quels Kits d’outils agrègent les outils sous-jacents.

## Portée / Scope

- **Public** : Visiteurs (personnes qui fréquentent les événements/festivals : public, participants ateliers, jeux, concours, réservations, pass VIP).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du public (espace visiteur, agenda cross-événements, billets, réservations, pass VIP, suivi d’activités — jeux, concours, ateliers).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

Les visiteurs **interagissent avec** des Opérateurs gouvernés (espace dédié visiteur, agenda, billets, réservations, pass VIP, jeux, concours, ateliers) ; ces Opérateurs s’appuient sur des Toolkits et des composants Miyukini (Miyauth, Miyuprofile, MiyuClock, Miyubooking, Miyunotify, etc.).

---

## 2. Besoins en Opérateurs (public Visiteurs)

### 2.1 Opérateur « MFS Visiteur » (espace dédié visiteur)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer l’**espace dédié visiteur** : agenda personnel (ateliers, animations, concours auxquels le visiteur est inscrit ou souhaite suivre), billets/tickets, réservations (ateliers, créneaux), pass VIP, suivi d’activités (jeux joués, concours, ateliers suivis, récompenses). Compte **cross-événements** : un même visiteur peut suivre ses activités sur plusieurs événements. |
| **Public servi** | Visiteurs authentifiés (rôle visiteur, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Agenda (programme personnel, synchronisé entre événements) ; compte à rebours (jours/heures avant événement ou créneau) ; billets/tickets (accès centralisé par événement ou groupe d’événements) ; réservations (ateliers, créneaux, places) avec annulation/modification selon règles de l’édition ; pass VIP (avantages par événement ou multi-événements) ; suivi d’activités (historique participations : jeux, concours, ateliers, récompenses). |
| **Ne fait pas** | Décision des services proposés (organisateur active et paramètre) ; émission des billets (organisateur / plateforme). |

Cet Opérateur est le **point d’entrée** principal du visiteur : il agrège les capacités métier (agenda, billets, réservations, pass VIP, suivi d’activités) et s’appuie sur les Toolkits listés en § 3.

### 2.2 Opérateur « MFS Catalogue » (découverte — partagé avec utilisateur non connecté)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **catalogue** (annuaire des événements, répertoire des organisateurs, répertoire des exposants) en **lecture** : le visiteur connecté peut consulter le catalogue comme un utilisateur non connecté, puis accéder en plus à son espace dédié. |
| **Public servi** | Visiteurs (connectés) et Utilisateurs non connectés. |
| **Gouvernance** | Mandat public d’accès (utilisateur non connecté) ou Mandat de Permission (visiteur connecté) ; Border Guard pour les données publiées uniquement. |
| **Capacités exposées** | Liste/carte des événements publiés ; fiches événement (présentation, dates, lieu, organisateur, exposants, programme public) ; répertoire des organisateurs ; répertoire des exposants ; recherche et filtres (Miyusearch). |
| **Lien avec MFS Visiteur** | Le visiteur **découvre** via MFS Catalogue puis **réserve / s’inscrit** via MFS Visiteur (ou parcours d’inscription à un atelier, achat billet, etc., selon services activés par l’organisateur). |

### 2.3 Synthèse des Opérateurs (public Visiteurs)

| Opérateur | Usage par le visiteur | Livrables couverts |
|-----------|------------------------|---------------------|
| **MFS Visiteur** | Connexion, agenda, billets, réservations, pass VIP, suivi d’activités (jeux, concours, ateliers). | Espace dédié visiteur, compte cross-événements, conflits de dates. |
| **MFS Catalogue** | Découverte des événements, organisateurs, exposants ; recherche. | Catalogue (annuaire, répertoires) — partagé avec utilisateur non connecté. |

---

## 3. Besoins en Toolkits (public Visiteurs)

### 3.1 Kit « Agenda Visiteur » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**agenda personnel** du visiteur : ateliers, animations, concours auxquels il est inscrit ou qu’il souhaite suivre ; **synchronisé entre événements** ; compte à rebours ; détection des conflits de dates (chevauchement de créneaux ou événements). |
| **Outils agrégés (exemples)** | `agenda.list.byVisitor` (activités inscrites ou suivies), `agenda.dates.get` (dates des événements et créneaux), `agenda.conflict.check` (chevauchement avant nouvelle réservation/inscription), `agenda.countdown.get` (jours/heures avant événement ou créneau). |
| **Consommé par** | MFS Visiteur. |
| **Composants sous-jacents** | MiyuClock, Miyubooking (ou équivalent), KindMother. |

### 3.2 Kit « Billets & Réservations » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’**accès aux billets et aux réservations** : ateliers, créneaux, places réservés ; annulation ou modification dans le cadre des règles de l’édition ; centralisation par événement ou groupe d’événements. |
| **Outils agrégés (exemples)** | `ticket.list.byVisitor` (billets acquis), `ticket.download` (QR, PDF), `reservation.list.byVisitor` (réservations ateliers, créneaux), `reservation.create`, `reservation.cancel`, `reservation.reschedule`, `reservation.eligibility.check` (places, règles édition). |
| **Consommé par** | MFS Visiteur. |
| **Composants sous-jacents** | Miyubooking (créneaux, places), KindMother, Miyuinvoice (si achat billet payant). |

### 3.3 Kit « Pass VIP » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le **pass VIP** et les avantages associés (par événement ou multi-événements), selon ce que l’organisateur met en place. |
| **Outils agrégés (exemples)** | `pass.get.byVisitor` (pass actifs), `pass.benefits.get` (avantages associés), `pass.activate` (activation pour un événement). |
| **Consommé par** | MFS Visiteur. |
| **Composants sous-jacents** | KindMother ; paramétrage par MFS Édition (organisateur). |

### 3.4 Kit « Suivi d’activités » (MFS)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le **suivi des participations** : jeux joués, concours, ateliers suivis, récompenses ; historique et statistiques par événement ou global. |
| **Outils agrégés (exemples)** | `activity.list.byVisitor` (participations : jeux, concours, ateliers), `activity.reward.get` (récompenses), `activity.history.get`, `activity.stats.get`. |
| **Consommé par** | MFS Visiteur. |
| **Composants sous-jacents** | KindMother ; services visiteur (jeux, concours, ateliers) activés par l’organisateur. |

### 3.5 Kit « Catalogue » (MFS) — voir Utilisateur non connecté

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **consultation du catalogue** (annuaire événements, répertoire organisateurs, répertoire exposants) en lecture seule ; recherche et filtres. |
| **Outils agrégés (exemples)** | `event.list.public`, `event.get.public`, `organiser.list.public`, `exposant.list.public`, `search.query` (Miyusearch). |
| **Consommé par** | MFS Catalogue (partagé avec utilisateur non connecté). |
| **Composants sous-jacents** | KindMother, Border Guard (données publiées). |

### 3.6 Synthèse des Toolkits (public Visiteurs)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Agenda Visiteur** | MFS Visiteur | Agenda personnel, compte à rebours, conflits de dates. |
| **Billets & Réservations** | MFS Visiteur | Billets, réservations (ateliers, créneaux), annulation/modification. |
| **Pass VIP** | MFS Visiteur | Pass VIP, avantages. |
| **Suivi d’activités** | MFS Visiteur | Jeux, concours, ateliers, récompenses, historique. |
| **Catalogue** | MFS Catalogue | Annuaire événements, répertoires, recherche. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Onboarding (inscription visiteur, par festival ou groupe) | Miyauth, Miyuprofile ; MFS Visiteur | Équipe & Permissions (Miyauth), Agenda Visiteur. |
| Découverte (catalogue) | MFS Catalogue | Catalogue. |
| Réservation / Inscription (atelier, concours, billet) | MFS Visiteur | Billets & Réservations, Agenda Visiteur (conflit check). |
| Organisation (agenda, billets, pass) | MFS Visiteur | Agenda Visiteur, Billets & Réservations, Pass VIP. |
| Participation (jeux, concours, ateliers) | MFS Visiteur | Suivi d’activités, Billets & Réservations. |
| Conflits de dates | MFS Visiteur | Agenda Visiteur (conflict.check). |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles | Miyauth, Master Butler | Compte visiteur, Mandat, permissions. |
| Profil visiteur | Miyuprofile | Données profil, préférences. |
| Agenda, créneaux | MiyuClock, Miyubooking (ou équivalent) | Plages de dates, conflits, réservations. |
| Billets, paiement | Miyuinvoice (si achat payant) | Billets, pass payants. |
| Notifications | Miyunotify | Rappels, confirmations, annonces. |
| Persistance | KindMother | Agenda, réservations, participations, récompenses. |
| Sécurité, audit | WorrySentinel, traçabilité | Niveaux de sécurité, audit. |
| Recherche | Miyusearch | Recherche catalogue. |

---

## 6. Références

- [Document fondateur Miyukini Festival Service](../../Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md)
- [Visiteurs — Parcours, capacités et services](./Visiteurs%20-%20Parcours%20Capacites%20Services.md)
- [Public Organisateurs](../Organisateurs/_index.md) | [Public Exposants](../Exposants/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission)

---

**Document** : Visiteurs — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (Miyukini Festival Service)
