# JayRDV — Document fondateur

## Contexte

**JayRDV** est le service Miyukini dédié à la **prise de rendez-vous et à la réservation en ligne**. Il vise à offrir, dans l’écosystème COG, une solution gouvernée de réservation (RDV, créneaux, ressources) pour les professionnels et leurs clients, en B2B et B2C.

Ce document est le **document fondateur** du service : il en fixe la raison d’être, la portée et le lien avec les analyses produit réalisées (benchmark des fonctionnalités des solutions de réservation en ligne). Il s’adresse aux équipes produit, technique et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayRDV, positionnement, référence aux livrables d’analyse produit.
- **Hors périmètre** : Spécifications techniques détaillées, contrats d’API, implémentation (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, analyse [Fonctionnalités des solutions de réservation en ligne](./reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md).

---

## 1. Raison d’être

### 1.1 Proposition de valeur (cible)

**JayRDV** a pour objectif de permettre à des **professionnels** (praticiens, entreprises, équipes) d’utiliser une plateforme gouvernée pour :

- **Proposer des créneaux de réservation en ligne** accessibles 24h/24 et 7j/7 sans intervention manuelle.
- **Gérer calendriers et ressources** (personnes, salles, équipements) avec synchronisation et cohérence des disponibilités.
- **Réduire les non-présentations (no-show)** via confirmations, rappels et notifications automatiques.
- **Intégrer la réservation** dans leurs outils (site web, CRM, agendas) via widget, API et webhooks.

Les **clients finaux** (B2C) bénéficient d’une prise de rendez-vous en libre-service : affichage des disponibilités, formulaire de réservation, confirmation et rappels, sans création de compte obligatoire selon les cas d’usage.

### 1.2 Positionnement par rapport au benchmark

L’analyse des **fonctionnalités des solutions de réservation en ligne** (voir document dédié dans `reference/`) sert de base pour :

- Identifier les **fonctionnalités attendues** du marché (gestion calendaire, rappels, paiements, intégrations).
- Aligner la **roadmap produit** sur les standards et bonnes pratiques (Calendly, Doctolib, SimplyBook.me, Bookeo, Reservio, SmartAgenda, etc.).
- Définir le **périmètre fonctionnel** de JayRDV dans le cadre COG (Opérateurs, Kits d’outils, Mandats).

---

## 2. Exclusion du domaine médical

**Le domaine médical est formellement exclu de JayRDV.** Les spécificités médicales (fiche patient, antécédents, prescriptions, consentement éclairé, téléconsultation médicale, conformité RGPD santé / HDS, intégration carte Vitale, protocoles HL7/FHIR) seront couvertes par un service dédié : **JayBobo**.

JayRDV et JayBobo partagent les briques communes (MiyuBooking, MiyuNotify, JayKoa, JayXpose) mais JayBobo ajoute une couche de conformité santé au-dessus.

**Référence :** [Spécification Complète du Service](./JayRDV%20-%20Specification%20Complete%20du%20Service.md) § 2.

---

## 3. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather, KindMother, Master Butler, WorrySentinel. |
| **B2B2C** | Livraison du service aux **professionnels** (B2B) ; les professionnels exposent la **réservation** aux **clients finaux** (B2C). |
| **Réutilisabilité** | S’appuyer sur les Kits d’outils Miyukini existants (Miyauth, Miyunotify, Miyubooking, etc.) et définir les Opérateurs et Kits spécifiques « réservation ». |
| **Benchmark continu** | Les fonctionnalités identifiées dans le benchmark « solutions de réservation en ligne » alimentent la vision produit et le backlog. |

---

## 3. Livrables d’analyse produit

Les livrables suivants sont hébergés dans ce dossier :

| Livrable | Emplacement | Description |
|----------|-------------|-------------|
| **Fonctionnalités des solutions de réservation en ligne** | `reference/JayRDV - Fonctionnalites Solutions Reservation en Ligne.md` | Analyse produit senior : inventaire structuré des fonctionnalités des solutions de réservation en ligne (état de l’art, sources web, synthèse par domaine). |

---

## 4. Structure par public

La documentation produit JayRDV est **structurée par public** pour couvrir les besoins, parcours et livrables de chaque cible :

| Public | Dossier | Contenu principal |
|--------|---------|-------------------|
| **Professionnels** | `publics/Professionnels/` | Analyse des besoins, Parcours capacités livrables (exposition créneaux, lien pro, widget, confirmation, rappels, tableau de bord). |
| **Clients** | `publics/Clients/` | Analyse des besoins, Parcours capacités livrables (réservation guest et avec compte, Mes RDV, annulation/modification, confirmation, rappels). |
| **Utilisateur non connecté** | `publics/UtilisateurNonConnecte/` | Analyse des besoins, Parcours et accès (parcours guest, Façade publique, règles d’accès, token annulation/modification). |

Chaque public dispose d’un **index** (`_index.md`) et de documents d’**analyse des besoins** (besoins fonctionnels et non fonctionnels, user stories, priorisation MoSCoW) et de **parcours / capacités / livrables** (ou **parcours et accès** pour l’utilisateur non connecté). Le parcours **réservation guest** est **partagé** entre le public Clients et le public Utilisateur non connecté ; les deux documentations le décrivent pour traçabilité.

Voir l’[index du service](./_index.md) pour l’arborescence complète et les liens vers chaque document.

---

*Document de référence — non contractuel pour l’implémentation. Dernière mise à jour : 2026-01-31.*
