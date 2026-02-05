# JayKoa — Référentiel fonctionnel inspiré de Google Agenda

## Contexte

Ce document constitue un **référentiel fonctionnel** pour JayKoa, inspiré des capacités des agendas grand public (notamment **Google Agenda / Google Calendar**). Il ne décrit pas Google Agenda en tant que tel : il **traduit** les concepts et usages repérés (vues, rappels, partage, libre/occupé, calendriers multiples, etc.) en **capacités et contraintes JayKoa**, dans le cadre COG (Mandats, WorrySentinel, services consommateurs).

**Objectif** : Enrichir la documentation conceptuelle et contractuelle de JayKoa en s’appuyant sur des usages éprouvés, tout en respectant la gouvernance, la sécurité et le bornage du service.

## Portée / Scope

- **Périmètre** : Référentiel des capacités inspirées de Google Agenda ; correspondance avec les documents fondateurs et les écrans/parcours JayKoa.
- **Hors périmètre** : Spécifications techniques (API, schémas) ; description détaillée de Google Agenda.
- **Références** : [Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md), [Écrans et UI](../JayKoa%20-%20Ecrans%20et%20UI.md), [Parcours Utilisateurs](../JayKoa%20-%20Parcours%20Utilisateurs.md), [Protocole d’écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 1. Vues calendrier (inspirées Google Agenda)

Google Agenda propose plusieurs **vues** : Jour, Semaine, Mois, Année, **Agenda** (liste chronologique des événements à venir), et des vues personnalisées (ex. 4 jours). JayKoa positionne ces capacités comme suit :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Vue Jour** | Vue calendrier **jour** : grille horaire, entrées (RDV, événements, ateliers) sur une journée. | AGD-UI-01 (vue calendrier jour/semaine/mois). |
| **Vue Semaine** | Vue calendrier **semaine** : grille 7 jours (ou personnalisable), entrées par jour. | AGD-UI-01. |
| **Vue Mois** | Vue calendrier **mois** : aperçu du mois, indicateur de charge ou entrées par jour. | AGD-UI-01. |
| **Vue Agenda / Liste** | Vue **liste chronologique** des entrées à venir (ordre temporel, sans grille) : « Prochains événements ». | À intégrer comme **option** de la vue calendrier (AGD-UI-01) ou composant dédié **AGD-UI-07** (vue liste/agenda). |
| **Vue Année** | Aperçu **année** : vue synthétique (optionnel, phase 2 ou 3). | Hors scope MVP ; à traiter en phase ultérieure. |
| **Début de semaine personnalisable** | Paramètre utilisateur ou service (lundi vs dimanche) ; MiyuClock / fuseau. | Données fournies par JayKoa (fuseau, préférence début de semaine si stockée par le service consommateur). |

**Règle** : Les vues sont **fournies en données** par JayKoa ; l’affichage (grille, liste) est assuré par les UIs des services consommateurs (JayRDV, JayFestival). Les composants recommandés (AGD-UI-01, et option liste/agenda) décrivent le **contrat** (période, filtres, entrées).

---

## 2. Rappels et notifications (inspirés Google Agenda)

Google Agenda permet des **rappels** par événement (notification X minutes/heures/jours avant). JayKoa ne gère pas l’envoi direct des notifications (délégué à Miyunotify ou aux services consommateurs) ; il peut **fournir les données** nécessaires pour déclencher les rappels :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Rappel avant événement** | JayKoa fournit les **entrées à venir** (plage, type, libellé, id opaque) ; le **service consommateur** ou **Miyunotify** déclenche le rappel (email, push, in-app) selon la règle métier et les préférences utilisateur. | Parcours utilisateurs ; Bornage (phase 3 : rappels). |
| **Rappel « prochaine entrée »** | Données pour l’indicateur **prochaine entrée** (AGD-UI-05) et, en phase ultérieure, pour un rappel automatique (Miyunotify). | AGD-UI-05. |

**Règle** : JayKoa **ne envoie pas** les notifications ; il **expose les entrées** (dates, types) pour que les services consommateurs ou Miyunotify déclenchent les rappels dans le respect des Mandats et du niveau de sécurité.

---

## 3. Partage et niveaux d’accès (inspirés Google Agenda)

Google Agenda propose un **partage au niveau calendrier** avec des niveaux : voir uniquement **libre/occupé** (sans détail), **voir tous les événements**, **modifier les événements**, **gérer le partage**. JayKoa transpose ces notions dans le cadre **Mandats et permissions** :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Libre / Occupé** | Exposition d’un **indicateur de disponibilité** (plages libres vs occupées) **sans détail des entrées** : pour qu’un tiers (ex. professionnel JayRDV, organisateur JayFestival) puisse proposer des créneaux sans voir le détail des RDV ou événements. | Niveau de visibilité **0–1** ; données agrégées (occupé oui/non par plage) ; Mandat avec périmètre « libre/occupé » uniquement. À documenter comme **option** (phase 2) : AGD-UI-08 (vue libre/occupé) ou API dédiée. |
| **Voir les événements** | Accès **lecture** aux entrées agenda (plage, type, libellé) selon Mandat et Master Butler. | Vue calendrier (AGD-UI-01), liste ; Niveaux Sécurité. |
| **Modifier les événements** | La **modification** des entrées reste du ressort des **services consommateurs** (JayRDV, JayFestival) ; JayKoa enregistre les **références** mises à jour (plage, type) après validation métier. | Integration Services Consommateurs. |
| **Gérer le partage** | Équivalent COG : **StrongFather** (Mandats), **Master Butler** (permissions) ; pas de « propriétaire calendrier » unique hors contexte utilisateur et service. | Document Fondateur, Niveaux Sécurité. |

**Règle** : Le partage et les niveaux d’accès sont gouvernés par **Mandats de Permission** et **niveaux de sécurité** (WorrySentinel) ; JayKoa n’expose que les données autorisées pour le contexte (libre/occupé, détail, export).

---

## 4. Calendriers multiples et agrégation (inspirés Google Agenda)

Google Agenda permet **plusieurs calendriers** (personnel, travail, etc.) affichés ensemble ou séparément, avec couleurs par calendrier. JayKoa gère l’**agrégation multi-sources** (plusieurs services : JayRDV, JayFestival) pour un même utilisateur :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Plusieurs calendriers** | **Plusieurs sources** d’entrées (JayRDV, JayFestival, futurs services) pour un même utilisateur ; affichage agrégé ou filtré par source/type. | Document Fondateur (§ 3 Intégration), Integration Services Consommateurs, AGD-UI-04 (filtre par source/type). |
| **Couleur / distinction par source** | Les entrées sont **typées et sourcées** (type, source) ; les UIs des services consommateurs peuvent afficher des couleurs ou styles par source/type. | Données fournies par JayKoa (type, source) ; implémentation UI côté consommateur. |
| **Masquer / afficher une source** | Filtre par **source** ou **type** (AGD-UI-04) : l’utilisateur choisit d’afficher uniquement RDV, ou uniquement festivals, ou tout. | AGD-UI-04. |

**Règle** : L’agrégation multi-sources est soumise au **Mandat** et au **niveau de sécurité** (AGD-SEC-2) ; JayKoa fournit les entrées agrégées ou filtrées selon le contexte.

---

## 5. Invitations à un événement (inspirées Google Agenda)

Google Agenda permet d’**inviter des personnes** à un événement (acceptation, refus, peut-être) et de voir les réponses. JayKoa ne gère pas directement les invitations ni les réponses : celles-ci relèvent des **services consommateurs** (JayFestival pour un atelier, JayRDV pour un RDV). JayKoa peut :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Inviter / accepter / refuser** | Géré par le **service consommateur** (création événement, envoi invitation, mise à jour statut). JayKoa reçoit les **entrées agenda** une fois l’inscription ou la réservation validée (plage, type, référence utilisateur). | Integration Services Consommateurs. |
| **Voir qui est invité** | Données métier (liste des invités, réponses) détenues par le service consommateur ; JayKoa ne stocke pas la liste des invités, uniquement les **références** des entrées (qui a quelle plage). | AGD-SEC-1 (pas de copie canonique des données métier). |

**Règle** : Les invitations et réponses sont du **domaine métier** des services consommateurs ; JayKoa travaille sur les **entrées agenda** (plages, types, conflits, vues, export).

---

## 6. Tâches vs événements (inspiré Google Agenda)

Google Agenda distingue **événements** (avec date/heure) et **tâches** (to-do, optionnellement datées). JayKoa se concentre sur les **entrées temporelles** (plages début/fin) :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Événement** | **Entrée agenda** avec plage (début, fin), type, source. Pleinement couvert. | Document Fondateur, tous les docs. |
| **Tâche (to-do)** | Une **tâche** peut être modélisée comme entrée avec **plage optionnelle** ou **date d’échéance** si un service consommateur la publie vers JayKoa (ex. « À faire avant le X »). Sinon, les tâches restent hors périmètre JayKoa (gérées par un autre module ou service). | Bornage : tâches avec date/plage peuvent être des entrées ; tâches sans date hors périmètre MVP. |

**Règle** : JayKoa couvre les **entrées avec plage temporelle** (ou date d’échéance) ; les tâches sans dimension temporelle explicite ne sont pas dans le périmètre cœur.

---

## 7. Export et synchronisation (inspirés Google Agenda)

Google Agenda permet l’**export** (iCal, etc.) et la **synchronisation** avec d’autres calendriers (Apple, Outlook). JayKoa :

| Concept Google | Capacité JayKoa | Référence |
|----------------|--------------------------|-----------|
| **Export iCal / PDF** | **Export iCal et PDF** (MVP : iCal ; phase 2 : PDF) ; pas de données au-delà du niveau autorisé (AGD-SEC-3). | AGD-UI-03, Document Fondateur, Bornage. |
| **Synchronisation externe** | **Hors scope MVP** ; en phase ultérieure, les services consommateurs ou une extension peuvent consommer les données JayKoa pour synchroniser avec des calendriers externes (Google, Outlook, Apple). | Bornage (phase 3). |
| **Accès hors ligne** | L’écosystème Miyukini accepte l’**isolement comme état normal** (LOI-2) ; l’accès hors ligne relève de l’architecture des services consommateurs et du COG (cache, synchronisation différée). | Document Fondateur (LOI-2), hors périmètre direct JayKoa. |

**Règle** : Export contrôlé (Mandat, niveau de sécurité) ; synchronisation externe et hors ligne en phase ultérieure ou déléguées aux consommateurs.

---

## 8. Synthèse : correspondance Google Agenda → JayKoa

| Domaine | Google Agenda | JayKoa (référentiel) |
|---------|---------------|-------------------------------|
| **Vues** | Jour, Semaine, Mois, Année, Agenda (liste) | AGD-UI-01 (jour/semaine/mois) ; option vue liste/agenda (AGD-UI-07) ; année en phase 2+. |
| **Rappels** | Rappel avant événement | Données pour Miyunotify / services ; AGD-UI-05 (prochaine entrée) ; phase 3 pour rappels automatiques. |
| **Partage** | Libre/occupé, lecture, écriture, gestion | Mandats, niveaux (WorrySentinel) ; option libre/occupé (phase 2, AGD-UI-08 ou API). |
| **Calendriers multiples** | Plusieurs calendriers, couleurs, masquer/afficher | Agrégation multi-sources (JayRDV, JayFestival) ; filtre par source/type (AGD-UI-04). |
| **Invitations** | Inviter, accepter, refuser | Service consommateur ; JayKoa reçoit les entrées validées. |
| **Tâches** | Tâches (to-do) | Entrées avec plage ou échéance si publiées ; tâches sans date hors périmètre cœur. |
| **Export / sync** | iCal, sync externe, hors ligne | Export iCal/PDF (AGD-SEC-3) ; sync externe / hors ligne en phase 2+ ou par consommateurs. |

---

## 9. Références

| Document | Rôle |
|----------|------|
| [JayKoa - Document Fondateur](../JayKoa%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement. |
| [JayKoa - Ecrans et UI](../JayKoa%20-%20Ecrans%20et%20UI.md) | Composants UI (AGD-UI-01 à 06 ; options 07, 08). |
| [JayKoa - Parcours Utilisateurs](../JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs et côté service. |
| [JayKoa - Bornage Implementation](../JayKoa%20-%20Bornage%20Implementation.md) | MVP, phases, hors scope. |
| [Miyukini Prompt Protocol - Ecriture Documentation Conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle. |

---

**Document** : JayKoa — Référentiel fonctionnel inspiré de Google Agenda  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (enrichissement conceptuel)
