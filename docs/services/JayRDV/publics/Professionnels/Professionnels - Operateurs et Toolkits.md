# Professionnels — Besoins en Opérateurs et Toolkits

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du public **Professionnels** du service JayRDV. Il s’appuie sur l’[analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md) et le document [Parcours, capacités et livrables](./Professionnels%20-%20Parcours%20Capacites%20Livrables.md). Il vise à fournir une référence produit pour l’architecture gouvernée : quels Opérateurs exposent les capacités aux professionnels, et quels Kits d’outils agrègent les outils sous-jacents.

## Portée / Scope

- **Public** : Professionnels (praticiens, entreprises, équipes) qui proposent des créneaux de réservation.
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir les livrables du public (dashboard, calendrier, services, plannings, notifications, lien/widget, stats, équipe).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores (StrongFather, Master Butler, etc.) — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). Ne décide pas stratégiquement ; exécute sous Mandat. |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier, sans connaissance de l’Opérateur appelant (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). N’ajoute pas de capacité nouvelle ; orchestre. |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather, permettant aux Opérateurs de collaborer. |

Les professionnels **interagissent avec** des Opérateurs gouvernés ; ces Opérateurs s’appuient sur des Toolkits et des Cores (Master Butler pour les permissions, KindMother pour la persistance, Miyunotify pour les notifications, etc.).

---

## 2. Besoins en Opérateurs (public Professionnels)

Les capacités livrées au professionnel sont exposées par un ou plusieurs **Opérateurs** gouvernés. La liste ci-dessous décrit les Opérateurs nécessaires pour couvrir les parcours et livrables du public Professionnels.

### 2.1 Opérateur « JayRDV Pro » (espace professionnel)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer l’espace professionnel de réservation : tableau de bord, calendrier, gestion des services, plannings, RDV, notifications, lien de réservation, widget, statistiques, équipe. |
| **Public servi** | Professionnels (authentifiés, rôle Admin, Gestionnaire ou Praticien). |
| **Gouvernance** | Mandat de Permission (StrongFather) pour accéder aux données du professionnel ; permissions (Master Butler) par rôle ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Dashboard (RDV du jour, semaine, indicateurs), calendrier (vue jour/semaine/mois, création/modification/annulation RDV), services (CRUD), plannings (horaires récurrents, exceptions), paramétrage notifications, génération lien de réservation, intégration widget, statistiques, gestion équipe (invitation, rôles, établissements). |
| **Ne fait pas** | Décision stratégique (StrongFather), exécution des notifications (déléguée à Miyunotify), calcul des créneaux (délégué au Kit Calendrier / Miyubooking). |

Cet Opérateur est le **point d’entrée** principal du professionnel : il agrège les capacités métier (calendrier, services, plannings, exposition) et s’appuie sur les Toolkits et composants Miyukini listés en § 4.

### 2.2 Opérateur « JayRDV Exposition » (lien public, widget)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **page de réservation publique** (lien du professionnel ou widget) : affichage des services et des créneaux disponibles, formulaire de réservation, confirmation. Ne gère pas l’authentification client ; sert aussi bien les utilisateurs non connectés que les clients connectés (pré-remplissage si compte). |
| **Public servi** | Clients et Utilisateurs non connectés (via Façade publique gouvernée). |
| **Gouvernance** | Mandat public d’accès (pour utilisateur non connecté) ou Mandat de Permission (pour client connecté) ; Border Guard pour les limites de la Façade (pas d’agenda détaillé, pas de noms d’autres clients). |
| **Capacités exposées** | Liste des services actifs du professionnel, liste des créneaux disponibles (temps réel), formulaire de réservation (guest ou pré-rempli), création de RDV, page de confirmation. |
| **Lien avec Professionnels** | Le professionnel configure les services et plannings via **JayRDV Pro** ; **JayRDV Exposition** lit ces données pour afficher les créneaux et enregistrer les réservations. |

Cet Opérateur est partagé entre les publics **Clients** et **Utilisateur non connecté** ; il est **configuré** par le public Professionnels (lien, widget, paramètres visibles).

### 2.3 Synthèse des Opérateurs (public Professionnels)

| Opérateur | Usage par le professionnel | Livrables couverts |
|-----------|----------------------------|---------------------|
| **JayRDV Pro** | Connexion, dashboard, calendrier, services, plannings, notifications, lien/widget, stats, équipe. | Dashboard, calendrier, services, plannings, notifications, lien, widget, statistiques, équipe. |
| **JayRDV Exposition** | Configuration indirecte (données exposées) ; pas d’interface dédiée au pro pour « gérer la page publique » autre que les paramètres (services, plannings) déjà dans JayRDV Pro. | Lien de réservation, widget (données alimentées par JayRDV Pro). |

Le professionnel n’interagit **directement** qu’avec **JayRDV Pro**. **JayRDV Exposition** est l’Opérateur qui sert la page de réservation aux clients et aux utilisateurs non connectés ; le professionnel en bénéficie via la génération du lien et du widget dans JayRDV Pro.

---

## 3. Besoins en Toolkits (public Professionnels)

Les **Toolkits** sont des compositions d’**Outils** (capacités exécutables sans autorité). Ils sont consommés par les Opérateurs (JayRDV Pro, JayRDV Exposition) et par les composants Miyukini (Miyubooking, MiyuClock, etc.). La liste ci-dessous décrit les Toolkits **nécessaires** pour livrer les capacités du public Professionnels.

### 3.1 Kit « Calendrier & Créneaux » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la gestion des plannings, des créneaux et des RDV : calcul des créneaux disponibles, réservation, modification, annulation, verrouillage temps réel. |
| **Outils agrégés (exemples)** | `slot.list` (lister les créneaux disponibles pour un service/pro sur une plage), `slot.hold` (réserver temporairement un créneau), `slot.release` (libérer un hold), `booking.create` (créer un RDV), `booking.update` (modifier un RDV), `booking.cancel` (annuler un RDV), `booking.get` (détail d’un RDV), `schedule.get` (récupérer le planning d’un pro/praticien), `exception.list` (exceptions congés/absences). |
| **Consommé par** | JayRDV Pro (calendrier, création/modification/annulation RDV), JayRDV Exposition (affichage créneaux, création RDV côté client). |
| **Composants sous-jacents** | Miyubooking, MiyuClock (référence implémentation). |

### 3.2 Kit « Services & Plannings » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la gestion des types de RDV (services) et des plannings récurrents : CRUD services, horaires par jour, exceptions. |
| **Outils agrégés (exemples)** | `service.list`, `service.create`, `service.update`, `service.deactivate`, `schedule.set` (définir horaires récurrents), `exception.create`, `exception.delete`, `buffer.get` (temps entre deux RDV), `preavis.get` (préavis min/max). |
| **Consommé par** | JayRDV Pro (écrans Services, Plannings, Exceptions). |
| **Composants sous-jacents** | Miyubooking, MiyuClock. |

### 3.3 Kit « Exposition Réservation » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Générer et exposer le lien de réservation et le widget : URL unique par professionnel, paramètres d’intégration (couleurs, étapes), code embed. |
| **Outils agrégés (exemples)** | `link.generate` (générer l’URL de réservation du pro), `link.preview` (prévisualisation), `widget.config` (options d’intégration), `widget.embed` (code snippet). |
| **Consommé par** | JayRDV Pro (écran Intégrations / Lien / Widget). |
| **Composants sous-jacents** | Miyubooking (lien), front Façade publique (widget). |

### 3.4 Kit « Notifications Pro » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’envoi et la configuration des notifications : confirmation client, rappel client, notification pro (nouveau RDV, annulation). Modèles personnalisables par le professionnel. |
| **Outils agrégés (exemples)** | `notify.config.get` (récupérer la config notifications du pro), `notify.config.set` (activer/désactiver canaux, modèles), `notify.send` (déclencher un envoi — confirmation, rappel, alerte pro). |
| **Consommé par** | JayRDV Pro (écran Notifications), JayRDV Exposition / backend (envoi automatique après réservation, rappels planifiés). |
| **Composants sous-jacents** | Miyunotify. |

### 3.5 Kit « Statistiques Réservation » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Agréger les données de pilotage : nombre de RDV (jour, semaine, mois), taux de remplissage, taux de no-show, créneaux les plus demandés. Export CSV/PDF. |
| **Outils agrégés (exemples)** | `stats.aggregate` (volume RDV par période), `stats.noShow` (taux no-show), `stats.topSlots` (créneaux les plus réservés), `export.csv`, `export.pdf`. |
| **Consommé par** | JayRDV Pro (dashboard, écran Statistiques). |
| **Composants sous-jacents** | Miyubooking (données RDV), KindMother (persistance). |

### 3.6 Kit « Équipe & Permissions » (JayRDV)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer l’invitation de membres (Gestionnaire, Praticien), l’attribution des rôles et des établissements, et la délégation. |
| **Outils agrégés (exemples)** | `team.invite` (envoyer une invitation), `team.list` (liste des membres), `team.role.set`, `team.establishment.assign`, `permission.check` (vérifier si l’utilisateur peut accéder à une ressource). |
| **Consommé par** | JayRDV Pro (écran Équipe), Cores (Master Butler pour les permissions). |
| **Composants sous-jacents** | Miyauth (comptes), Master Butler (permissions), Miyuprofile (profils). |

### 3.7 Synthèse des Toolkits (public Professionnels)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Calendrier & Créneaux** | JayRDV Pro, JayRDV Exposition | Calendrier, créneaux, RDV (création, modification, annulation). |
| **Services & Plannings** | JayRDV Pro | Services, plannings, exceptions. |
| **Exposition Réservation** | JayRDV Pro | Lien de réservation, widget. |
| **Notifications Pro** | JayRDV Pro, backend Exposition | Confirmation, rappels, notification pro. |
| **Statistiques Réservation** | JayRDV Pro | Dashboard, statistiques, export. |
| **Équipe & Permissions** | JayRDV Pro | Équipe, rôles, établissements. |

---

## 4. Matrice Parcours / Livrables / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Onboarding (inscription pro) | Miyauth, Miyuprofile (comptes) ; JayRDV Pro (premier paramétrage) | Équipe & Permissions (attribution rôle) ; Services & Plannings (premier service, premier planning). |
| Dashboard (RDV du jour, semaine, indicateurs) | JayRDV Pro | Calendrier & Créneaux (booking.get, slot.list) ; Statistiques Réservation (stats.aggregate). |
| Calendrier (vue jour/semaine/mois, CRUD RDV) | JayRDV Pro | Calendrier & Créneaux. |
| Gestion des services | JayRDV Pro | Services & Plannings. |
| Gestion des plannings et exceptions | JayRDV Pro | Services & Plannings ; Calendrier & Créneaux (schedule.get). |
| Paramétrage notifications | JayRDV Pro | Notifications Pro. |
| Lien de réservation, widget | JayRDV Pro (config) ; JayRDV Exposition (exposition) | Exposition Réservation ; Calendrier & Créneaux (données créneaux). |
| Statistiques, export | JayRDV Pro | Statistiques Réservation. |
| Équipe (invitation, rôles, établissements) | JayRDV Pro | Équipe & Permissions. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles, permissions | Miyauth, Master Butler | Compte pro, Mandat, isolation des données par professionnel. |
| Profil professionnel, établissements | Miyuprofile | Données structure, établissement(s). |
| Notifications (email, SMS) | Miyunotify | Envoi effectif des confirmations, rappels, alertes pro. |
| Calendrier, créneaux, RDV (données) | Miyubooking, MiyuClock | Gestion des plannings, des créneaux et des RDV (persistance, calcul). |
| Fiche client, CRM léger | Miyucontacts (ou équivalent) | Fiche client, historique RDV, notes (côté pro). |
| Paiement (si activé) | Miyuinvoice ou partenaire | Paiement en ligne, acompte, historique. |
| Persistance, cohérence | KindMother | Données RDV, plannings, services. |
| Sécurité, audit | WorrySentinel, traçabilité | Niveaux de sécurité, audit des actions. |

---

## 6. Références

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Professionnels — Analyse des besoins](./Professionnels%20-%20Analyse%20des%20besoins.md)
- [Professionnels — Parcours, capacités et livrables](./Professionnels%20-%20Parcours%20Capacites%20Livrables.md)
- [Public Clients](../Clients/_index.md) | [Utilisateur non connecté](../UtilisateurNonConnecte/_index.md)
- Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission)

---

**Document** : Professionnels — Besoins en Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence produit pour l’architecture gouvernée (JayRDV)
