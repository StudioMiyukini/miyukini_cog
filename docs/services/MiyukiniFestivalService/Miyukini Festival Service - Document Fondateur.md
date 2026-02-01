# Miyukini Festival Service — Document fondateur

## Contexte

**Miyukini Festival Service** est le service officiel Miyukini dédié à la **gestion d’événements et de festivals** au sein de l’écosystème COG. Il reprend et développe les spécificités issues de **Catakana Orga** pour les porter à l’échelle de la plateforme : organisation de la **macro** (catalogue, annuaires, gouvernance), **distribution du service** auprès des **organisateurs**, des **exposants** et des **visiteurs** (qui disposent chacun d’un espace dédié et d’un compte cross-événement).

**Tous les types de comptes sont cross-événements** : un organisateur peut gérer plusieurs festivals/événements ; un exposant peut participer à plusieurs festivals et dispose de son propre dashboard ; un visiteur peut suivre ses activités sur plusieurs événements. La plateforme propose une **gestion d’agenda** pour éviter les conflits de dates (ex. qu’un exposant ne s’inscrive pas à deux événements à la même date).

Ce document est le **document fondateur** du service : il en fixe la raison d’être, la vision, le modèle de distribution (organisateurs, exposants, visiteurs) et les principes directeurs. Il s’adresse aux parties prenantes internes et externes (équipes produit, technique, partenaires, organisateurs, exposants, visiteurs).

## Portée / Scope

- **Périmètre** : Service Miyukini Festival Service — définition, positionnement, macro et distribution.
- **Hors périmètre** : Spécifications techniques détaillées, contrats d’API, implémentation des crates (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, Vision stratégique, Audit Catakana → Miyukini COG B2B2C.

---

## 1. Raison d’être

### 1.1 Proposition de valeur

**Miyukini Festival Service** permet à des **organisateurs** (associations, collectivités, sociétés) d’utiliser une plateforme gouvernée pour :

- **Créer et gérer plusieurs événements** (éditions) : un même organisateur peut organiser **plusieurs festivals/événements** ; paramètres, dates, lieu, programme, plan de salle, budget, documents par édition.
- **Gérer leurs exposants** : annuaire, candidatures, validation, facturation, emplacements.
- **Exposer leur offre** dans un **catalogue commun** : annuaire d’événements, répertoire d’organisateurs, répertoire d’exposants, accessible en B2B2C.
- **Proposer des services aux visiteurs** : jeux, concours, inscriptions ateliers, etc. — chaque organisateur choisit ce qu’il met à disposition pour ses événements.

Les **exposants** disposent de leur **propre dashboard** : candidatures, participations à **plusieurs festivals**, documents, factures ; la plateforme propose une **gestion d’agenda** pour éviter qu’un exposant s’inscrive à deux événements à la même date (conflits de dates déjà rencontrés en pratique).

Les **visiteurs** disposent d’un **espace dédié** : onboarding par festival ou par groupe de festivals, compte cross-événement pour suivre leurs activités et organiser leur visite (agenda, compte à rebours, billet, réservation, pass VIP, etc.).

La plateforme propose le **service** ; les organisateurs l’**adaptent** à leurs besoins (identité, règles, contenu) sans en détenir la gouvernance technique. **Tous les comptes sont cross-événements** (organisateur, exposant, visiteur). Le modèle s’apparente à un **Store** : un lieu unique où l’on découvre les événements, les organisateurs et les exposants, et où chaque type d’acteur gère son parcours sur plusieurs événements.

### 1.2 Héritage Catakana Orga

Les spécificités fonctionnelles du service s’appuient sur l’expérience et le périmètre de **Catakana Orga** :

- Gestion des **éditions** (événements) avec tableau de bord par édition.
- Gestion des **exposants** : fiche, statuts, candidatures, documents, devis et factures.
- **Plan de salle** interactif (zones, stands, attribution).
- **Programme** : animations, créneaux, salles/scènes, conflits horaires.
- **Budget** par édition, documents et légal, notifications et communication.
- **Rôles** : admin, manager, exposant, bénévole — avec permissions et Mandats.

Ces capacités sont **reprises, normalisées et distribuées** dans le cadre COG : Opérateurs, Kits d’outils, Contrats d’équipe, Mandats de Permission.

---

## 2. Vision

### 2.1 Énoncé de vision

> **Miyukini Festival Service** est le service de référence Miyukini pour la gestion d’événements et de festivals : une plateforme gouvernée, en B2B2C, où **tous les comptes sont cross-événements** — les organisateurs gèrent plusieurs festivals, les exposants participent à plusieurs événements avec leur dashboard, les visiteurs organisent leur visite sur plusieurs événements — et où le public découvre un annuaire d’événements, un répertoire d’organisateurs et un répertoire d’exposants.

### 2.2 Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather (décisions), KindMother (données), Master Butler (permissions), WorrySentinel (sécurité). Les organisateurs opèrent dans le cadre de Mandats de Permission. |
| **B2B2C** | La plateforme livre le service aux **organisateurs** (B2B) ; les organisateurs exposent **événements et exposants** aux **utilisateurs finaux** (B2C). |
| **Store** | Un catalogue commun : annuaire d’événements + répertoire d’organisateurs + répertoire d’exposants, avec recherche, filtres et Façade publique gouvernée. |
| **Comptes cross-événements** | **Tous les types de comptes** (organisateur, exposant, visiteur) sont cross-événements : un organisateur peut organiser plusieurs festivals ; un exposant peut s’inscrire à plusieurs festivals et dispose de son dashboard ; un visiteur peut suivre ses activités sur plusieurs événements. |
| **Gestion d’agenda** | Une fonctionnalité de **gestion d’agenda** (calendrier, conflits de dates) permet d’éviter qu’un exposant — ou un visiteur — ne s’inscrive à deux événements à la même date ; besoin déjà identifié en pratique. |
| **Espace visiteur** | Les visiteurs disposent d’un espace dédié pour suivre leurs activités et organiser leur visite (agenda, billets, réservations, pass VIP, etc.), avec onboarding possible par festival ou par groupe de festivals. |
| **Réutilisabilité** | Le service s’appuie sur les Kits d’outils Miyukini existants (Miyauth, Miyuinvoice, Miyucontacts, Miyusearch, Miyunotify, Miyubooking, etc.) et définit les Opérateurs et Kits spécifiques « événement » (Édition, Exposant, Plan de salle, Programme, Candidature, Visiteur, **Agenda cross-événements**). |
| **Autonomie organisateur** | Chaque organisateur dispose d’un espace gouverné (**plusieurs éditions**, ses exposants, son équipe, les services visiteur qu’il active) sans empiéter sur la souveraineté des données ni sur les autres organisateurs. |

---

## 3. Macro : organisation du service au niveau plateforme

### 3.1 Trois piliers du catalogue (Store)

| Pilier | Contenu | Rôle |
|--------|---------|------|
| **Annuaire des événements** | Liste/carte des éditions (événements) publiées ; filtres (date, lieu, organisateur, thème) ; fiche événement (présentation, dates, lieu, organisateur, exposants, programme public). | Découverte des événements par le public et les professionnels. |
| **Répertoire des organisateurs** | Liste des structures organisatrices ; fiche organisateur (nom, événements, contact, charte). | Identification et confiance dans les organisateurs. |
| **Répertoire des exposants** | Liste des exposants (global ou par événement) ; fiche exposant (entreprise, stands, éditions participées, contact). | Découverte des exposants et mise en relation. |

La **macro** désigne tout ce qui est géré au **niveau plateforme** : catalogue commun, gouvernance des accès, annuaires, recherche (Miyusearch), Façade publique gouvernée. Elle ne contient pas la logique métier détaillée de chaque édition (celle-ci relève de l’espace organisateur).

### 3.2 Gouvernance macro

- **StrongFather** : décision d’ouverture d’un nouvel organisateur, validation des contrats, révocation de Mandats.
- **Master Butler** : permissions par rôle (plateforme, organisateur, exposant, visiteur, public) ; accès aux annuaires et aux espaces (organisateur, visiteur, exposant).
- **KindMother** : persistance des entités catalogue (événements, organisateurs, exposants) et des données par édition.
- **WorrySentinel** : niveaux de sécurité et états de confiance pour les données et les accès.

Les **Mandats de Permission** encadrent ce que chaque organisateur peut faire (créer des éditions, gérer des exposants, publier au catalogue, etc.).

### 3.3 Politique de résidence des données sensibles (exposants)

Les données personnelles et métier des **exposants** (fiche, candidatures, documents, facturation) sont soumises à la **politique de résidence centralisée** (voir [Miyukini Conceptual References - Politique Residence Donnees Sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)) :

- **COG de référence** pour ces données : **COG de l'organisateur** (ou COG du Service Festival, selon architecture retenue). La copie canonique réside sur ce COG (Instance Mère KindMother).
- **Effet** : en cas de coupure du terminal de l'exposant (réseau, appareil), les données restent **disponibles pour les organisateurs** sur le COG de référence.
- Le terminal exposant accède via Visite gouvernée ou synchronisation ; il ne détient pas la seule copie de ces données.

---

## 4. Distribution : le service au niveau des organisateurs

### 4.1 Modèle de distribution

La **distribution** désigne la manière dont le service est **livré aux organisateurs** et utilisé par eux :

1. **Onboarding organisateur** : création du compte organisateur (Miyauth, Miyuprofile), attribution des permissions (Master Butler), émission du Mandat de Permission pour gérer **plusieurs éditions** et exposants. Le compte est **cross-événement** : un même organisateur peut organiser plusieurs festivals/événements.
2. **Espace organisateur** : tableau de bord, **liste de toutes ses éditions** (multi-festivals), accès par édition (dashboard édition : plan, programme, exposants, candidatures, budget, documents).
3. **Personnalisation** : l’organisateur configure chacune de ses éditions (nom, dates, lieu, thème, règles), ses équipes (rôles, bénévoles), sa communication (Miyunotify), sans modifier la gouvernance ni les Kits de la plateforme.
4. **Publication au catalogue** : les éditions validées peuvent être exposées dans l’annuaire des événements ; l’organisateur apparaît dans le répertoire des organisateurs ; les exposants peuvent apparaître dans le répertoire des exposants (selon politique plateforme).

### 4.2 Rôles côté organisateur

| Rôle | Périmètre |
|------|------------|
| **Admin organisateur** | Gestion complète de la structure : **toutes ses éditions** (multi-festivals), équipe, paramètres, publication catalogue. |
| **Manager** | Gestion opérationnelle d’une ou plusieurs éditions : exposants, plan, programme, budget, documents. |
| **Exposant** | **Dashboard exposant dédié** : candidatures, **participations à plusieurs festivals**, documents, factures (Miyuinvoice), **agenda** pour éviter les conflits de dates (inscription à deux événements à la même date). |
| **Bénévole** | Accès limité selon attribution (zones, créneaux, informations de terrain). |

Les rôles sont gérés par **Master Butler** ; les Contrats d’équipe définissent les flux autorisés entre Opérateurs (Édition, Exposant, Programme, Plan de salle, etc.).

### 4.3 Livrables par organisateur

Chaque organisateur, dans son espace gouverné, dispose de :

- **Éditions (multi-festivals)** : création et gestion de **plusieurs éditions** ; paramétrage et tableau de bord par édition.
- **Exposants** : annuaire local, candidatures, validation, fiches, documents, devis/factures (Miyuinvoice).
- **Plan de salle** : zones, stands, attribution (Opérateur Plan de salle).
- **Programme** : animations, créneaux, salles (Opérateur Programme).
- **Budget** : revenus/dépenses, ventilation (Miyucptaledger, Miyuexpense, Miyucomptareports).
- **Documents** : contrats types, CGV, règlements (Miyucms, Miyumedia).
- **Notifications** : annonces, alertes (Miyunotify).
- **Services visiteur** : activation et paramétrage par édition des services proposés aux visiteurs (jeux, concours, inscriptions ateliers, réservations, pass VIP, etc.) ; l’organisateur choisit ce qu’il met à disposition.

La **macro** (catalogue, annuaires, recherche) reste sous contrôle plateforme ; la **micro** (contenu et opérations de chaque édition, dont services visiteur) reste sous contrôle organisateur dans le cadre du Mandat.

---

## 5. Distribution : le service au niveau des exposants

### 5.1 Compte cross-événement et dashboard exposant

Les **exposants** disposent d’un **compte cross-événement** et de leur **propre dashboard** pour leurs besoins :

- **Participation à plusieurs festivals** : un même exposant peut **s’inscrire et participer à plusieurs événements** ; son dashboard agrège candidatures, participations, documents et factures pour tous les festivals concernés.
- **Dashboard dédié** : vue unifiée sur ses candidatures (en attente, validées, refusées), ses éditions en cours ou à venir, ses documents et factures (Miyuinvoice), et son **agenda** cross-événements.

### 5.2 Gestion d’agenda et conflits de dates

Une **fonctionnalité de gestion d’agenda** est proposée pour éviter qu’un exposant (ou un visiteur) **ne s’inscrive à deux événements à la même date**. Besoin déjà rencontré en pratique (« déjà vu ») :

- **Calendrier cross-événements** : visualisation des dates des événements auxquels l’exposant est inscrit ou candidat ; alerte ou blocage en cas de chevauchement de dates.
- **Cohérence** : la plateforme peut signaler un conflit de dates avant validation d’une nouvelle candidature ou inscription, et l’exposant peut organiser son planning sur plusieurs festivals sans double engagement.

Cette capacité relève de l’Opérateur ou Kit **Agenda cross-événements** (MiyuClock, Miyubooking, données d’édition).

---

## 6. Distribution : le service au niveau des visiteurs

### 6.1 Espace dédié visiteur

Les **visiteurs** disposent d’un **espace dédié** gouverné, distinct de la simple consultation du catalogue. Comme pour les organisateurs et les exposants, **tous les comptes sont cross-événements** : un même compte visiteur permet de :

- **S’onboarder** par festival ou par **groupe de festivals** : le visiteur peut rejoindre un événement isolé ou une famille d’événements (ex. une série de festivals partenaires), selon ce que l’organisateur ou la plateforme propose.
- **Suivre ses activités et organiser sa visite** sur **plusieurs événements**, sans recréer d’identité à chaque fois.

La **gestion d’agenda** (voir § 5.2) s’applique aussi aux visiteurs : éviter les inscriptions à deux événements ou créneaux à la même date.

### 6.2 Compte cross-événement (visiteur)

Le **compte cross-événement** offre au visiteur une vue unifiée pour :

| Capacité | Description |
|----------|-------------|
| **Agenda** | Programme personnel : ateliers, animations, concours auxquels le visiteur est inscrit ou qu’il souhaite suivre, synchronisé entre événements. |
| **Compte à rebours** | Jours/heures restants avant les événements ou créneaux réservés. |
| **Billets / tickets** | Accès centralisé aux billets et tickets acquis (par événement ou groupe d’événements). |
| **Réservations** | Ateliers, créneaux, places réservés ; annulation ou modification dans le cadre des règles de l’édition. |
| **Pass VIP** | Pass et avantages associés (par événement ou multi-événements), selon ce que l’organisateur met en place. |
| **Suivi d’activités** | Historique et suivi des participations : jeux joués, concours, ateliers suivis, récompenses, etc. |

La gouvernance (StrongFather, Master Butler, KindMother) garantit que les données du visiteur restent souveraines et que l’accès cross-événement respecte les Mandats et les choix de chaque organisateur.

### 6.3 Services proposés aux visiteurs (activables par l’organisateur)

Chaque **organisateur** peut **mettre à disposition** pour ses événements tout ou partie des services suivants. La plateforme les fournit ; l’organisateur décide lesquels activer et avec quelles règles :

| Service | Description |
|---------|-------------|
| **Jeux** | Jeux liés au festival (quizz, chasses au trésor, défis) ; participation et suivi dans l’espace visiteur. |
| **Concours** | Inscription et participation à des concours ; résultats, récompenses, historique. |
| **Inscriptions ateliers** | Réservation de créneaux d’ateliers ; annulation, rappels, intégration à l’agenda visiteur. |
| **Réservations** | Réservation de places, créneaux ou activités (Miyubooking) ; billets et pass. |
| **Pass et avantages** | Pass VIP, pass journée, avantages fidélité ; liaison avec le compte cross-événement. |
| **Notifications** | Rappels, changements de programme, alertes (Miyunotify), selon préférences visiteur et règles édition. |

L’organisateur configure, par édition ou par groupe d’éditions, quels services sont ouverts et selon quelles conditions (places limitées, dates, publics). La **distribution** visiteur est ainsi **paramétrable** par organisateur sans modifier la gouvernance plateforme.

### 6.4 Onboarding visiteur : par festival ou par groupe de festivals

- **Onboarding par festival** : le visiteur crée un compte ou se connecte dans le contexte d’un seul événement ; il peut ensuite étendre son compte à d’autres événements (compte cross-événement).
- **Onboarding par groupe de festivals** : l’organisateur ou la plateforme propose une **famille d’événements** (ex. « Festivals partenaires 2026 ») ; le visiteur s’inscrit une fois et accède à tous les événements du groupe avec le même compte, agenda et billets unifiés.

Cela permet des partenariats entre organisateurs (groupes de festivals) et une expérience visiteur cohérente sur plusieurs événements.

---

## 7. Positionnement et communication

### 7.1 Message central (elevator pitch)

**Miyukini Festival Service** est le service Miyukini pour organiser et promouvoir des événements et festivals. **Tous les comptes sont cross-événements** : les **organisateurs** peuvent gérer **plusieurs festivals** ; les **exposants** disposent de leur **dashboard** pour participer à **plusieurs festivals** et bénéficier d’une **gestion d’agenda** pour éviter les conflits de dates ; les **visiteurs** organisent leur visite sur plusieurs événements (agenda, billets, réservations, pass VIP, jeux, concours, ateliers). Le public découvre les événements, les organisateurs et les exposants dans un catalogue commun, fiable et sécurisé.

### 7.2 Publics cibles

| Public | Message prioritaire |
|-------|----------------------|
| **Organisateurs** | Une plateforme complète pour gérer **plusieurs festivals/événements**, vos exposants et équipes, activer des services visiteurs (jeux, concours, ateliers), et publier dans un annuaire commun, dans le respect de la gouvernance et de la vie privée. Compte cross-événements. |
| **Exposants** | Votre **dashboard exposant** : candidater et participer à **plusieurs festivals**, suivre vos participations, documents et factures, et **gérer votre agenda** pour éviter les inscriptions à deux événements à la même date. Compte cross-événements. Visible dans le répertoire des exposants. |
| **Visiteurs** | Un espace dédié et un compte cross-événement : organisez votre visite (agenda, billets, réservations, pass VIP), participez aux jeux, concours et ateliers proposés par les festivals, et suivez vos activités sur un ou plusieurs événements. Onboarding par festival ou par groupe de festivals. |
| **Public (non connecté)** | Découvrez les événements, les organisateurs et les exposants dans un annuaire clair et à jour. |
| **Partenaires / intégrateurs** | Un service COG, modulaire, réutilisant les Kits Miyukini et extensible via Opérateurs et Mandats. |

### 7.3 Différenciation

- **Gouvernance COG** : pas un simple SaaS événementiel, mais un service inscrit dans l’architecture Miyukini (Cores, Opérateurs, Mandats).
- **Comptes cross-événements pour tous** : organisateur (plusieurs festivals), exposant (dashboard, plusieurs festivals, agenda), visiteur (plusieurs événements).
- **Gestion d’agenda** : éviter qu’un exposant — ou un visiteur — s’inscrive à deux événements à la même date ; besoin déjà vu en pratique.
- **B2B2C et Store** : catalogue commun (événements, organisateurs, exposants) + espaces dédiés organisateur, **exposant** (dashboard) et **visiteur**.
- **Héritage éprouvé** : spécificités inspirées de Catakana Orga, portées à l’échelle et normalisées.

---

## 8. Prochaines étapes (orientation)

1. **Fonder** : valider ce document fondateur et le diffuser (interne / partenaires).
2. **Spécifier** : documenter les Opérateurs et Kits (Édition, Organisateur, Exposant, Visiteur, Plan de salle, Programme, Candidature, **Agenda cross-événements**) et leurs Contrats d’équipe.
3. **Macro** : définir et implémenter le catalogue (annuaires, recherche, Façade publique).
4. **Distribution organisateur** : finaliser l’onboarding organisateur (compte cross-événement, **plusieurs festivals**), les Mandats et l’espace organisateur (dashboard multi-éditions, exposants, plan, programme, budget, **activation des services visiteur**).
5. **Distribution exposant** : dashboard exposant, participation à **plusieurs festivals**, **gestion d’agenda** (conflits de dates), candidatures, documents, factures.
6. **Distribution visiteur** : espace dédié visiteur, compte cross-événement, onboarding par festival / groupe de festivals, agenda, billets, réservations, pass VIP, jeux/concours/ateliers, **gestion d’agenda** (conflits de dates).
7. **Migration** : planifier la reprise des spécificités Catakana Orga (voir audit B2B2C) et la convergence des écrans et flux.

---

## 9. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie officielle (Opérateur, Mandat, COG, etc.). |
| [Miyukini Conceptual References — Vision stratégique](../reference/Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md) | Objectifs stratégiques, B2B2C, modèles de livraison. |
| [Audit Catakana → Miyukini COG B2B2C](../modules/Catakana%20-%20Audit%20Conversion%20Miyukini%20COG%20B2B2C.md) | Kits manquants, correspondance Opérateurs, besoins UI. |

---

**Document** : Miyukini Festival Service — Document fondateur  
**Version** : 1.2  
**Date** : 2026-01-31  
**Statut** : Document fondateur — référence pour le service (comptes cross-événements pour tous ; dashboard exposant ; gestion d’agenda)
