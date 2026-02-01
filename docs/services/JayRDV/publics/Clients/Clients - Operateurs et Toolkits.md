# Clients — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du public **Clients** du service JayRDV. Il s’appuie sur l’[analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md) et le document [Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md). Il vise à fournir une référence produit pour l’architecture gouvernée : quels Opérateurs exposent les capacités aux clients, et quels Kits d’outils agrègent les outils sous-jacents.

## Portée / Scope

- **Public** : Clients (particuliers qui prennent rendez-vous auprès des professionnels — B2C), avec ou sans compte.
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du public (page réservation, compte client, Mes RDV, annulation/modification, confirmation, rappels).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

Les clients **interagissent avec** des Opérateurs gouvernés (page réservation, espace « Mes RDV », profil) ; ces Opérateurs s’appuient sur des Toolkits et des composants Miyukini (Miyauth, Miyubooking, Miyunotify, etc.).

---

## 2. Besoins en Opérateurs (public Clients)

### 2.1 Opérateur « JayRDV Exposition » (page de réservation)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **page de réservation publique** (lien du professionnel ou widget) : services, créneaux, formulaire (guest ou pré-rempli si client connecté), création de RDV, page de confirmation. |
| **Public servi** | Clients (avec ou sans compte) et Utilisateurs non connectés. |
| **Gouvernance** | Mandat public d’accès (utilisateur non connecté) ou Mandat de Permission (client connecté) ; Border Guard pour les limites de la Façade. |
| **Capacités exposées** | Liste des services actifs, liste des créneaux disponibles (temps réel), formulaire de réservation (guest ou pré-rempli), création de RDV, confirmation à l’écran, lien « Ajouter à mon agenda », lien « Annuler ou modifier le RDV » (dans l’email). |
| **Lien avec compte client** | Si le client est connecté (Miyauth), le formulaire est pré-rempli et la réservation est associée à son compte ; sinon, parcours guest. |

Cet Opérateur est **partagé** entre les publics Clients et Utilisateur non connecté. Pour le client, il fournit en plus le **pré-remplissage** et l’**association du RDV au compte**.

### 2.2 Opérateur « JayRDV Client » (espace client)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer l’**espace client** : Mes RDV (liste à venir, détail, annulation, modification), Mon profil, Préférences de notification. |
| **Public servi** | Clients authentifiés (compte client). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; authentification (Miyauth) ; permissions (Master Butler) ; persistance (KindMother). |
| **Capacités exposées** | Liste des RDV à venir et passés, détail d’un RDV, annulation depuis l’espace, modification de créneau depuis l’espace, conflits de dates (alerte si chevauchement), profil (nom, email, téléphone), préférences (canaux et types de notifications). |
| **Ne fait pas** | Authentification (Miyauth), stockage du profil (Miyuprofile) ; il **consomme** ces composants. |

Cet Opérateur est le **point d’entrée** du client connecté pour tout ce qui est « Mes RDV », profil et préférences.

### 2.3 Opérateur « JayRDV Lien Annulation/Modification » (accès par token)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer les **pages d’annulation et de modification** accessibles via le lien dans l’email de confirmation (token sécurisé), **sans connexion**. |
| **Public servi** | Clients et Utilisateurs non connectés ayant reçu l’email de confirmation. |
| **Gouvernance** | Mandat public d’accès (token valide) ; Border Guard (pas d’accès aux données d’autres clients) ; token unique, temporaire, non devinable. |
| **Capacités exposées** | Vérification du token, affichage du récapitulatif du RDV, annulation (avec confirmation), modification (liste des créneaux disponibles, sélection d’un nouveau créneau), message « Lien expiré » si token invalide ou déjà utilisé. |
| **Lien avec Clients** | Le client peut aussi annuler/modifier depuis **JayRDV Client** (Mes RDV) ; le lien email est une **alternative** sans connexion. |

Cet Opérateur est **partagé** entre les publics Clients et Utilisateur non connecté pour le parcours « annulation/modification depuis l’email ».

### 2.4 Synthèse des Opérateurs (public Clients)

| Opérateur | Usage par le client | Livrables couverts |
|-----------|---------------------|---------------------|
| **JayRDV Exposition** | Accès au lien du pro, choix service/créneau, formulaire (pré-rempli si connecté), confirmation. | Page réservation, confirmation à l’écran, email confirmation, lien agenda, lien annulation/modification. |
| **JayRDV Client** | Connexion puis accès à Mes RDV, profil, préférences ; annulation/modification depuis l’espace. | Mes RDV, détail RDV, annulation/modification depuis espace, profil, préférences, conflits de dates. |
| **JayRDV Lien Annulation/Modification** | Clic sur le lien dans l’email ; annulation ou modification sans connexion. | Page annulation/modification (token), confirmation par email. |

---

## 3. Besoins en Toolkits (public Clients)

### 3.1 Kit « Réservation Client » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le parcours de réservation côté client : liste des services, liste des créneaux, validation du formulaire, création du RDV, génération des liens (agenda, annulation/modification). |
| **Outils agrégés (exemples)** | `service.list.public` (services actifs du pro), `slot.list.public` (créneaux disponibles), `form.validate` (nom, email, téléphone), `booking.create` (création RDV, guest ou avec client_id), `booking.confirm` (confirmation à l’écran), `ical.generate` (lien ajout agenda), `token.generate` (token annulation/modification). |
| **Consommé par** | JayRDV Exposition. |
| **Composants sous-jacents** | Miyubooking, Miyucontacts (léger pour données guest). |

### 3.2 Kit « Mes RDV » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la liste et la gestion des RDV du client : liste à venir, détail, annulation, modification (reprise de créneau), détection des conflits de dates. |
| **Outils agrégés (exemples)** | `booking.list.byClient` (RDV à venir et passés), `booking.detail` (détail d’un RDV), `booking.cancel` (annulation), `booking.reschedule` (changement de créneau), `slot.list.public` (créneaux disponibles pour modification), `conflict.check` (chevauchement avec un autre RDV du client). |
| **Consommé par** | JayRDV Client. |
| **Composants sous-jacents** | Miyubooking, KindMother. |

### 3.3 Kit « Lien Annulation/Modification » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’accès par token : vérification du token, affichage du RDV, annulation ou modification sans authentification. |
| **Outils agrégés (exemples)** | `token.verify` (valider le token, récupérer le RDV), `token.invalidate` (après utilisation), `booking.cancel.byToken`, `booking.reschedule.byToken`, `slot.list.public` (pour la modification). |
| **Consommé par** | JayRDV Lien Annulation/Modification. |
| **Composants sous-jacents** | Miyubooking (stockage token, RDV), Miyunotify (email confirmation annulation/modification). |

### 3.4 Kit « Compte Client » (Miyauth / Miyuprofile)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Authentification et profil client : inscription, connexion, récupération mot de passe, édition profil (nom, email, téléphone), préférences de notification. |
| **Outils agrégés (exemples)** | `auth.register`, `auth.login`, `auth.logout`, `auth.resetPassword`, `profile.get`, `profile.update`, `preferences.get`, `preferences.update`. |
| **Consommé par** | JayRDV Client (profil, préférences) ; JayRDV Exposition (pré-remplissage si connecté). |
| **Composants sous-jacents** | Miyauth, Miyuprofile. |

*Note :* Le « Compte Client » peut être un Toolkit Miyukini existant (Miyauth, Miyuprofile) ou un Kit JayRDV qui les orchestre pour le contexte réservation (préférences notifications). Selon l’architecture, les outils `preferences.update` (notifications) peuvent être dans un Kit « Préférences Réservation » (JayRDV) qui s’appuie sur Miyuprofile.

### 3.5 Kit « Notifications Client » (JayRDV / Miyunotify)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’envoi des notifications au client : confirmation à la réservation, rappel (24h, 2h avant), confirmation d’annulation/modification, notification si le pro annule ou modifie le RDV. |
| **Outils agrégés (exemples)** | `notify.send.confirmation` (email/SMS après réservation), `notify.send.reminder` (rappel planifié), `notify.send.cancelConfirm`, `notify.send.rescheduleConfirm`, `notify.send.proCancel` (pro a annulé). |
| **Consommé par** | Backend après réservation/annulation/modification ; planificateur pour les rappels. |
| **Composants sous-jacents** | Miyunotify. |

### 3.6 Synthèse des Toolkits (public Clients)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Réservation Client** | JayRDV Exposition | Page réservation, formulaire, confirmation, lien agenda, lien annulation/modification (génération). |
| **Mes RDV** | JayRDV Client | Liste RDV, détail, annulation, modification, conflits de dates. |
| **Lien Annulation/Modification** | JayRDV Lien Annulation/Modification | Page annulation/modification (token), envoi email confirmation. |
| **Compte Client** | JayRDV Client, JayRDV Exposition | Inscription, connexion, profil, préférences, pré-remplissage. |
| **Notifications Client** | Backend (après actions) | Confirmation, rappels, confirmation annulation/modification, notification pro. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Réservation guest | JayRDV Exposition | Réservation Client. |
| Réservation avec compte (pré-rempli) | JayRDV Exposition, Miyauth | Réservation Client, Compte Client. |
| Confirmation, rappels | Backend + Miyunotify | Notifications Client. |
| Mes RDV (liste, détail, annulation, modification) | JayRDV Client | Mes RDV, Compte Client. |
| Annulation/modification (lien email) | JayRDV Lien Annulation/Modification | Lien Annulation/Modification, Notifications Client. |
| Profil, préférences | JayRDV Client | Compte Client. |
| Conflits de dates | JayRDV Client | Mes RDV (conflict.check). |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Page réservation, créneaux | Miyubooking, lien pro | Page publique, calcul des créneaux disponibles. |
| Formulaire guest, réservation | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client. |
| Compte client, connexion | Miyauth, Miyuprofile | Authentification, profil client. |
| Confirmation, rappels | Miyunotify | Email et SMS. |
| Mes RDV, historique | Miyubooking, KindMother | Liste des RDV par client, persistance. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits. |
| Paiement (si activé) | Miyuinvoice ou partenaire | Paiement en ligne à la réservation. |

---

## 6. Références

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Clients — Analyse des besoins](./Clients%20-%20Analyse%20des%20besoins.md)
- [Clients — Parcours, capacités et livrables](./Clients%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Professionnels](../Professionnels/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission)

---

**Document** : Clients — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (JayRDV)
