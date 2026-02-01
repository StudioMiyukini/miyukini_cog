# Utilisateur non connecté — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) pour le public **Utilisateur non connecté** du service JayRDV. L’utilisateur non connecté n’a pas de compte ; il accède à la **Façade publique gouvernée** (parcours guest). Les Opérateurs et Toolkits décrits ici sont ceux qui **servent** cette Façade : ils exposent les capacités (réservation sans compte, annulation/modification via lien email) sans exposer de données sensibles.

Il s’appuie sur l’[analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md) et le document [Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md).

## Portée / Scope

- **Public** : Utilisateur non connecté (toute personne accédant à un lien de réservation ou à un widget sans être authentifiée).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du parcours guest (page réservation, confirmation, rappels, annulation/modification par lien email, règles d’accès Façade).
- **Hors périmètre** : Spécifications d’implémentation ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Façade publique gouvernée** | Zone tampon d’exposition permettant aux utilisateurs externes d’interagir avec un COG **sans y entrer**. Strictement unidirectionnelle ; sans identité persistante obligatoire. |
| **Utilisateur externe** | Consommateur non certifié de services exposés par un COG, sans gouvernance propre. Accès uniquement via Façade publique ; soumis à un Mandat public d’accès. |
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |

L’utilisateur non connecté **n’interagit pas** avec un Opérateur au sens « compte utilisateur » ; il consomme des **surfaces exposées** par des Opérateurs gouvernés (page réservation, page annulation/modification). Les Opérateurs listés ci-dessous sont ceux qui **alimentent** la Façade publique pour le parcours guest.

---

## 2. Besoins en Opérateurs (public Utilisateur non connecté)

### 2.1 Opérateur « JayRDV Exposition » (page de réservation — mode guest)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **page de réservation publique** en **mode guest** : affichage des services et des créneaux disponibles, formulaire sans compte (nom, email, téléphone), création de RDV, page de confirmation. Aucune authentification ; aucune donnée « Mes RDV » exposée. |
| **Public servi** | Utilisateurs non connectés (et clients en parcours guest). |
| **Gouvernance** | **Mandat public d’accès** (Border Guard) : exposition limitée aux seules disponibilités et au formulaire de réservation ; pas d’agenda détaillé du professionnel, pas de noms d’autres clients ; rate limiting. |
| **Capacités exposées** | Liste des services actifs du pro, liste des créneaux disponibles (temps réel), formulaire guest (nom, email, téléphone, remarque), création de RDV (sans client_id), confirmation à l’écran, envoi email/SMS avec lien « Ajouter à mon agenda » et lien « Annuler ou modifier le RDV ». |
| **Limites** | Pas de pré-remplissage (pas de compte) ; pas d’accès à « Mes RDV » ; pas d’historique affiché. |

Cet Opérateur est le **même** que celui utilisé pour le public Clients (page réservation) ; la **différence** est le **contexte d’appel** : sans authentification, formulaire non pré-rempli, RDV non associé à un compte.

### 2.2 Opérateur « JayRDV Lien Annulation/Modification » (accès par token — sans connexion)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer les **pages d’annulation et de modification** accessibles via le lien dans l’email de confirmation (token sécurisé), **sans connexion**. |
| **Public servi** | Utilisateurs non connectés (et clients) ayant reçu l’email de confirmation. |
| **Gouvernance** | Mandat public d’accès (token valide) ; token unique, temporaire, non devinable ; invalidé après utilisation ou expiration ; pas d’accès aux données d’autres RDV ou clients. |
| **Capacités exposées** | Vérification du token, affichage du récapitulatif du RDV (date, heure, service, pro, lieu), bouton « Annuler le RDV », bouton « Modifier le créneau », confirmation avant annulation (motif optionnel), liste des créneaux disponibles pour modification, message « Ce lien a expiré » si token invalide ou utilisé. |
| **Limites** | Aucun accès à l’historique des RDV ; aucun accès au profil ou aux préférences. |

Cet Opérateur est **partagé** entre les publics Utilisateur non connecté et Clients pour le parcours « annulation/modification depuis l’email ».

### 2.3 Synthèse des Opérateurs (public Utilisateur non connecté)

| Opérateur | Usage par l’utilisateur non connecté | Livrables couverts |
|-----------|--------------------------------------|---------------------|
| **JayRDV Exposition** | Accès au lien du pro (ou widget), choix service/créneau, formulaire guest, confirmation. | Page réservation, confirmation à l’écran, email confirmation, lien agenda, lien annulation/modification (dans l’email). |
| **JayRDV Lien Annulation/Modification** | Clic sur le lien dans l’email ; annulation ou modification sans connexion. | Page annulation/modification (token), confirmation par email, message lien expiré. |

Les **passerelles** « Créer un compte » et « Se connecter » sont des **liens** vers Miyauth (hors périmètre Opérateur JayRDV pour ce public) ; elles ne sont pas des capacités de l’Opérateur Façade, mais des options d’orientation.

---

## 3. Besoins en Toolkits (public Utilisateur non connecté)

### 3.1 Kit « Réservation Guest » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer le parcours de réservation **sans compte** : liste des services (public), liste des créneaux (public), validation du formulaire guest, création du RDV (sans client_id), génération du token d’annulation/modification, génération du lien « Ajouter à mon agenda ». |
| **Outils agrégés (exemples)** | `service.list.public` (services actifs du pro, pas de données internes), `slot.list.public` (créneaux disponibles uniquement, pas d’agenda détaillé), `form.validate.guest` (nom, email, téléphone), `booking.create.guest` (création RDV sans compte), `token.generate.cancelModify` (token unique, non devinable, expiration configurable), `ical.generate` (lien ajout agenda). |
| **Consommé par** | JayRDV Exposition (mode guest). |
| **Composants sous-jacents** | Miyubooking, Miyucontacts (léger pour données guest). |

### 3.2 Kit « Lien Annulation/Modification » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’accès par token **sans authentification** : vérification du token, récupération du RDV (récap uniquement), annulation, modification (liste des créneaux disponibles, sélection d’un nouveau créneau), invalidation du token après utilisation, envoi des emails de confirmation d’annulation/modification. |
| **Outils agrégés (exemples)** | `token.verify` (valider le token, récupérer le RDV en lecture seule), `token.invalidate` (après utilisation ou expiration), `booking.cancel.byToken`, `booking.reschedule.byToken`, `slot.list.public` (pour la modification), `notify.send.cancelConfirm`, `notify.send.rescheduleConfirm`. |
| **Consommé par** | JayRDV Lien Annulation/Modification. |
| **Composants sous-jacents** | Miyubooking (stockage token, RDV), Miyunotify (emails). |

### 3.3 Kit « Notifications Guest » (JayRDV / Miyunotify)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’envoi des notifications pour le parcours guest : confirmation à la réservation (email/SMS), rappel (24h, 2h avant), confirmation d’annulation, confirmation de modification. Pas de personnalisation par « compte » (pas de préférences stockées pour un utilisateur non connecté) ; les modèles sont ceux configurés par le professionnel. |
| **Outils agrégés (exemples)** | `notify.send.confirmation` (email/SMS après réservation, avec liens agenda et annulation/modification), `notify.send.reminder` (rappel planifié), `notify.send.cancelConfirm`, `notify.send.rescheduleConfirm`. |
| **Consommé par** | Backend après réservation/annulation/modification ; planificateur pour les rappels. |
| **Composants sous-jacents** | Miyunotify. |

### 3.4 Kit « Façade & Sécurité » (JayRDV / Border Guard)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer les **règles d’accès** à la Façade publique : rate limiting (affichage créneaux, soumission formulaire, lien token), validation des champs (taille max, format), pas d’exposition de données sensibles (agenda détaillé, noms d’autres clients). |
| **Outils agrégés (exemples)** | `rateLimit.check` (par IP, par lien pro), `exposure.validate` (vérifier qu’aucune donnée interdite n’est renvoyée), `token.expiry.check`. |
| **Consommé par** | JayRDV Exposition, JayRDV Lien Annulation/Modification (avant toute réponse). |
| **Composants sous-jacents** | WorrySentinel, Border Guard (règles), traçabilité. |

### 3.5 Synthèse des Toolkits (public Utilisateur non connecté)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Réservation Guest** | JayRDV Exposition | Page réservation, formulaire guest, confirmation, lien agenda, lien annulation/modification (génération). |
| **Lien Annulation/Modification** | JayRDV Lien Annulation/Modification | Page annulation/modification (token), email confirmation, message lien expiré. |
| **Notifications Guest** | Backend (après actions) | Confirmation, rappels, confirmation annulation/modification. |
| **Façade & Sécurité** | JayRDV Exposition, JayRDV Lien Annulation/Modification | Rate limiting, pas d’exposition de données sensibles, token sécurisé. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Réservation guest (accès, service, créneau, formulaire, confirmation) | JayRDV Exposition | Réservation Guest, Façade & Sécurité. |
| Confirmation, rappels | Backend + Miyunotify | Notifications Guest. |
| Annulation (lien email) | JayRDV Lien Annulation/Modification | Lien Annulation/Modification, Notifications Guest, Façade & Sécurité. |
| Modification (lien email) | JayRDV Lien Annulation/Modification | Lien Annulation/Modification, Notifications Guest, Façade & Sécurité. |
| Règles d’accès (Façade, token, rate limiting) | JayRDV Exposition, JayRDV Lien Annulation/Modification | Façade & Sécurité. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Page réservation, créneaux | Miyubooking, lien pro / widget | Page publique, calcul des créneaux disponibles. |
| Formulaire guest, réservation | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client (nom, email, téléphone). |
| Confirmation, rappels | Miyunotify | Email et SMS. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire ; page dédiée. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits. |
| Façade publique | Mandat public d’accès, Border Guard | Exposition des seules capacités autorisées ; pas d’agenda détaillé, pas de noms d’autres clients. |

---

## 6. Références

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Utilisateur non connecté — Analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md)
- [Utilisateur non connecté — Parcours et accès](./UtilisateurNonConnecte%20-%20Parcours%20et%20acces.md)
- [Public Professionnels](../Professionnels/_index.md) | [Public Clients](../Clients/_index.md)
- Glossaire Miyukini (Façade publique gouvernée, Utilisateur externe, Opérateur, Kit d’Outils, Mandat public d’accès)

---

**Document** : Utilisateur non connecté — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (JayRDV)
