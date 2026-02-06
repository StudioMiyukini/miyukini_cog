# Miyukini Core System

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

**Miyukini est un COG — Core-Orchestrated Governance Environment.** Un environnement de gouvernance orchestré par des cores, qui coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.

---

## Table des matières

1. [Introduction](#1-introduction)
2. [Vision et philosophie](#2-vision-et-philosophie)
3. [Mécaniques générales](#3-mécaniques-générales)
4. [Environnements, fédération et sécurité](#4-environnements-fédération-et-sécurité)
5. [Méthodologie de développement](#5-méthodologie-de-développement)
6. [État actuel et roadmap](#6-état-actuel-et-roadmap)
7. [Écosystème Jay — Services interopérables](#7-écosystème-jay--services-interopérables)
8. [Jeux Miyukini — Gameplay et influences croisées](#8-jeux-miyukini--gameplay-et-influences-croisées)
9. [Gouvernance et sécurité](#9-gouvernance-et-sécurité)
10. [À qui s'adresse Miyukini](#10-à-qui-sadresse-miyukini)
11. [Documentation de référence](#11-documentation-de-référence)
12. [Licence](#12-licence)
13. [Conclusion](#13-conclusion)
14. [Log de rédaction](#14-log-de-rédaction)

---

## 1. Introduction

### Qu'est-ce que Miyukini

Miyukini Core System (MCS) est un **écosystème logiciel gouverné** conçu pour produire des applications autonomes, sécurisées structurellement, et capables de fonctionner dans des conditions de contrainte extrême (offline, ressources limitées, environnements isolés).

Miyukini n'est pas un framework. Ce n'est pas une bibliothèque. C'est un **environnement gouverné dans lequel des Opérateurs opèrent**.

La distinction est fondamentale : un framework fournit des outils que le développeur utilise comme bon lui semble. Miyukini fournit un **cadre non négociable** dans lequel les Opérateurs opèrent selon des règles strictes, des invariants vérifiables, et une gouvernance centralisée.

### Pourquoi Miyukini existe

Les architectures logicielles modernes reposent sur des hypothèses implicites : connectivité permanente, ressources cloud élastiques, services tiers toujours accessibles. Ces hypothèses excluent une part significative des cas d'usage réels.

Miyukini adopte la posture inverse : **la déconnexion est un état normal du système, pas une erreur à corriger**.

Résultat : systèmes qui démarrent sans réseau, fonctionnent sans cloud, dégradent proprement en isolation, restent administrables localement, et évoluent quand le réseau revient (sans reconstruction).

---

## 2. Vision et philosophie

### Systèmes autonomes

Un système Miyukini est **autonome** au sens strict : démarrable sans réseau, fonctionnel sans cloud, dégradé proprement en isolation, prévisible sans synchronisation, administrable localement, évolutif à la reconnexion. Cette autonomie est structurelle et vérifiable.

### Séparation stricte des responsabilités

- **Décision** (StrongFather) ≠ **Exécution** (Opérateurs, Outils) ≠ **Persistance** (KindMother). Aucun core n'empiète sur les autres.
- *« Un Outil fait, mais ne décide jamais. »*
- *« Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué. »*

### Complexité par collaboration

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

Les Services sont portés par des Opérateurs seuls ou par des Équipes d'Opérateurs ; les Opérateurs orchestrent des Outils et des Kits d'Outils. La complexité ne s'empile pas — elle se répartit sous gouvernance explicite.

### Lois d'autonomie (LOI-1 à LOI-8)

Les **8 lois d'autonomie** sont des invariants architecturaux non négociables :

| Loi | Énoncé |
|-----|--------|
| **LOI-1** | Aucune dépendance externe critique à l'exécution |
| **LOI-2** | Le système accepte l'isolement comme état normal |
| **LOI-3** | L'état local est souverain |
| **LOI-4** | Pas de temps global requis |
| **LOI-5** | Le coût doit être proportionnel au hardware |
| **LOI-6** | L'autonomie n'empêche pas la fédération |
| **LOI-7** | La strate Cores est immuable — évolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements |

Question de conception : *« Est-ce que ça fonctionne encore si le système est seul, lent, et isolé ? »*

### Pyramide et Cores

L'écosystème est organisé en **strates** hiérarchiques ; la dépendance est strictement unidirectionnelle (de haut en bas).

```
┌─────────────────────────────────────────────────────────────┐
│ 🔧 STRATE 9 — MiyukiniAdmin (EXCEPTION)                      │
│    Opérateur Souverain d'administration                      │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟧 STRATE 8 — SERVICES                                       │
│    Capacités perçues par l'utilisateur (facturation, caisse, │
│    réservation…) — délivrées par les Opérateurs (Strate 7)   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟩 STRATE 7 — OPÉRATEURS                                     │
│    Entités fonctionnelles gouvernées (exécutent des rôles)   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟦 STRATE 6 — TOOLS & TOOLKITS                               │
│    Capacités exécutables gouvernées (Outils, Kits d'Outils)  │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟨 STRATE 5 — INTERFACES & ADAPTATION                        │
│    BondingBrother (médiation intentions ↔ Cores)             │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟥 STRATE 4 — CORES SYSTÈME                                  │
│    StrongFather · KindMother · Caring Nanny · Master Butler  │
│    Border Guard · Ever Buddy · WorrySentinel · TAMR · …       │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟪 STRATE 3 — INVARIANTS & CONTRATS                          │
│    Principes architecturaux                                   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ ⚙️  KERNEL — Id · Logger · Clock · Config · Lifecycle         │
│    Substrat technique neutre (aucune logique métier)          │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟫 STRATE 0 — HARDWARE & OS                                  │
│    Réalité physique                                           │
└─────────────────────────────────────────────────────────────┘
```

Les **Cores** (Strate 4) gouvernent ; ils ne décident pas à la place de l'exécution, ils n'exécutent jamais. BondingBrother (Strate 5) assure la médiation entre les intentions et les Cores.

---

## 3. Mécaniques générales

### Du besoin utilisateur au résultat

1. **Service** — Ce que l'utilisateur perçoit et consomme (ex. « facturation », « réservation », « caisse »).
2. **Opérateur(s)** — Entité(s) fonctionnelle(s) gouvernée(s) qui portent le service. Un service peut être délivré par un seul Opérateur ou par une **Équipe d'Opérateurs** (collaboration mandatée sous Contrat d'équipe et Mandat de Permission).
3. **Outils et Kits d'Outils** — Capacités exécutables que les Opérateurs appellent. Les Outils sont atomiques et sans autorité ; les Kits d'Outils sont des compositions officielles d'Outils, optimisées pour l'efficience.
4. **Gouvernance** — Toute action passe par BondingBrother ; les décisions (autoriser/refuser) par StrongFather ; les écritures par KindMother (WriteIntent). Master Butler tient le registre des capacités et permissions ; WorrySentinel gouverne les niveaux de sécurité et les états de confiance.

### Règle fondamentale

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

Les concepts de services identifient les besoins en Opérateurs ; les Opérateurs orchestrent les Toolkits implémentés selon les Mandats de Permission et Contrats d'équipe.

---

## 4. Environnements, fédération et sécurité

### Environnements (COG) : identification et souveraineté

Un **environnement Miyukini (COG)** est une **entité souveraine, versionnée, isolée et identifiée de manière unique**. Ce n'est pas un simple runtime : c'est une instance de gouvernance.

| Propriété | Description |
|-----------|-------------|
| **Version complète des Cores** | Ensemble cohérent et figé de tous les Cores (Strate 4) |
| **ID d'environnement unique** | Identifiant généré à la création par le kernel |
| **Frontières strictes** | Limites claires entre l'environnement et l'extérieur |
| **Opérateurs assujettis** | Chaque Opérateur est lié à un environnement unique |

**Règle fondatrice (LOI-7)** : *« La strate Cores est immuable. Toute évolution se fait par la création d'un nouvel environnement complet. »* Pas de micro-patch, pas de hotfix sauvage — uniquement des environnements complets, auditable et versionnés.

**Identification des environnements** : selon le contexte, un COG peut être identifié par une **LSI** (Local Sovereign ID, générée localement), une **VID** (Verified ID, vérifiée par un registre global en contexte fédéré) ou une **WID** (Witnessed ID, attestée par échange indirect — clé USB, QR, signature). L'identité d'environnement est la base de toute relation inter-COG.

### Échanges entre environnements fédérés

L'autonomie n'empêche pas la fédération (LOI-6). Les échanges entre COG ne sont **jamais** une fusion de gouvernance : ils passent par une **visite gouvernée**.

- **COG Origine** — Atteste l'identité de l'utilisateur (émetteur du **Passeport Utilisateur**). Ne participe pas à l'exécution distante.
- **COG Hébergeur** — Souverain exécutif de la session. Vérifie le visiteur, accorde ou refuse l'accès, émet un **Visa de Connexion**, surveille et peut révoquer à tout moment.
- **Utilisateur Visiteur** — Citoyen dans son COG d'origine, visiteur gouverné dans le COG hôte. Agit uniquement via le Visa ; ne transporte aucun core, aucune logique, aucun état.
- **Bridge inter-COG** — Canal diplomatique (BondingBrother étendu). Transporte identités, intentions et autorisations. **Aucun pouvoir décisionnel, aucun état métier.**

> **Le bridge ne fait jamais confiance, il transporte.**

**Migration** (LOI-8) = diplomatie entre environnements : processus formel, contrat explicite, frontière contrôlée, traduction (pas copie brute). Acteurs : Border Guard (règles), BondingBrother (traduction), StrongFather (décision), KindMother (persistance), Ever Buddy (compatibilité).

### Logique de sécurité sous-jacente

- **Identité ≠ autorité** — Le Passeport prouve qui tu es et d'où tu viens ; il ne donne aucun droit. L'autorité reste au COG Hébergeur (Visa).
- **Un seul souverain par session** — Le COG Hébergeur est l'unique source de vérité de l'état pendant la visite. Aucun core n'est partagé ; aucun état n'est migré en direct.
- **Sécurité avant fluidité** — *« Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul. »*
- **Zero-trust** — Aucun appelant (environnement, utilisateur, Opérateur) n'est présumé valide ; toute intention est évaluée selon les politiques (WorrySentinel, StrongFather).

Cette logique s'applique aussi en interne : chaque environnement est souverain sur son périmètre ; la fédération repose sur des protocoles explicites (Passeport, Demande de Visite, Visa, Bridge), jamais sur une confiance implicite.

---

## 5. Méthodologie de développement

Le projet suit un **cycle strict**, de l'idée jusqu'à l'audit, sans court-circuit.

### Enchaînement des phases

1. **Idée** — Besoin ou fonctionnalité cible (ex. compta indépendants, caisse PoS, réservation, CMS).
2. **Analyse d'équivalents (PR)** — Étude des produits / logiciels existants du marché (Indy, Pennylane, logiciels caisse, moteurs forum, CMS boutique + réservation SaaS, etc.) pour en extraire les périmètres fonctionnels et les cas d'usage.
3. **Transcription conceptuelle dans la référence** — Rédaction ou mise à jour de documents **Miyukini Conceptual References — Équivalents …** (ex. Équivalents Comptabilité Indépendants, Équivalents PoS Logiciel Caisse, Équivalents Moteur Forum). La sémantique Miyukini (Outil, Kit d'Outils, Opérateur, Service, KindMother, StrongFather, etc.) est appliquée ; aucun code à ce stade.
4. **Documentation enrichie** — Rédaction de la **documentation fondatrice** des Tools/Toolkits ou Opérateurs : définition canonique, identifiants, liste d'outils, **contrats** (gouvernance, sécurité, intégration KindMother, tests), implémentation, alignement MIP. Planification et **bornage** de l'implémentation (périmètre, limites, livrables).
5. **Implémentation** — Développement selon le [protocole d'implémentation générale](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md) : planification → distribution des tâches → vérification, corrections et tests → gel et versionnement. Conformité MSCM/MIP.
6. **Test et audit** — Vérification des contrats, qualité, sécurité ; audit documentation si nécessaire.

### Principes

- **La référence conceptuelle précède le code.** On ne code pas un « équivalent Pennylane » sans avoir transposé le périmètre en termes Miyukini (Tools, Toolkits, Opérateurs, Services).
- **La documentation est normative.** Contrats, gouvernance et sécurité sont écrits et gelés avant ou en parallèle de l'implémentation.
- **Planification et bornage** évitent la dérive de périmètre et permettent un suivi explicite (y compris avec des agents IA en mode PLAN / Auto).

---

## 6. État actuel et roadmap

### Kits d'Outils (Toolkits) — Implémentés

**49 Toolkits** sont **implémentés** comme crates Rust dans le workspace (`crates/`), couvrant un large spectre de besoins. Chaque Toolkit est documenté (documentation fondatrice, contrats de gouvernance, référence outils) et positionné dans la pyramide (Strate 6).

**Domaines couverts** : données (MiyuSQL), identité (MiyuAuth), web (MiyuWeb), horloge (MiyuClock), contenu (MiyuCMS, MiyuMedia), widgets, boutique (MiyuStore), livraison (MiyuShipping), réservation (MiyuBooking), facturation SaaS (MiyuBilling), caisse et PoS (MiyuPosSales, MiyuPosInventory, MiyuPosAnalytics, MiyuPosLoyalty, MiyuPosKitchen, MiyuPosPayment), RH (MiyuHR), facturation métier (MiyuInvoice), comptabilité (MiyuComptaLedger, MiyuComptaReports, MiyuDeclarations), notes de frais (MiyuExpense), trésorerie (MiyuTreasury), et bien d'autres.

**État d'implémentation** : Phase 1 (squelettes) complète — toutes les crates compilent. Phase 2 (implémentation progressive) en cours — plusieurs Toolkits déjà fonctionnels (MiyuSQL, MiyuClock, MiyuNotify, MiyuAuth, etc.).

**Documentation** : [docs/tools/_index.md](docs/tools/_index.md) — index des Toolkits documentés.

### Miyukini Central (Hub Services)

**Miyukini Central** est l'application desktop (egui/eframe) qui sert de point d'entrée au COG : le Hub des Services. Elle permet de parcourir et d'ouvrir les Opérateurs disponibles dans l'environnement.

| Composant | Description |
|-----------|-------------|
| **Écran de chargement** | Au démarrage : titre « MIYUKINI COG System », barre de progression à remplissage irrégulier (10–12 s), phrases de chargement aléatoires (alternance 1,5–3 s). |
| **HUB** | Onglet principal : message de bienvenue, catalogue des services en grille ou en liste. |
| **Sidebar gauche** | Recherche (nom ou description), filtres par catégorie (Toutes, Utilitaires, Loisirs, Productivité, Design), type d'affichage (Grille / Liste). |
| **Cartes de services** | Chaque service affiche nom, catégorie, description courte (ServiceMeta) et bouton « Ouvrir ». Grille 3 colonnes à largeur fixe ou liste compacte. |
| **Services disponibles (MVP)** | Calculatrice, Jeu, Lord of the Click (MiyuClicker), Traitement de texte, Notes, Miyukini UI Library. |
| **Overlays** | Profil utilisateur, Paramètres (thème clair/obscur, persistance en storage). |
| **Header** | Onglets (HUB + services ouverts), icônes Lucide (utilisateur, paramètres), thème appliqué selon préférence. |

**Lancement** : exécutable produit par la crate `miyukini-central` (workspace Rust). Référence : [Miyukini Conceptual References - Miyukini Central Hub Services](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md).

### Services en cours d'élaboration

Les **concepts de services** identifient les **besoins en Opérateurs** qui orchestreront les Toolkits implémentés. L'écosystème Miyukini comprend deux familles de services :

- **Services Jay** — Services métier officiels de la famille « Jay », conçus pour être **interopérables** au sein de l'environnement COG unifié (voir [section 7](#7-écosystème-jay--services-interopérables)).
- **Services Miyukini** — Services transversaux qui fournissent des capacités partagées à tout l'écosystème.

Chaque service est défini par un **document fondateur**, des **analyses par public** (besoins, parcours, capacités livrables) et des documents **Opérateurs et Toolkits** qui précisent quels Opérateurs et Kits délivreront le service sous gouvernance.

| Service | Famille | Description |
|---------|---------|-------------|
| **JayFestival** | Jay | Gestion d'événements et festivals (B2B2C). Catalogue, exposants, visiteurs, comptes cross-événements, dashboard organisateur, billetterie. |
| **JayXpose** | Jay | Identité professionnelle de l'exposant : profil complet, catalogue de produits, site vitrine, coffre-fort documentaire (RIB, KBIS, assurances), annuaire des exposants. |
| **JayRDV** | Jay | Prise de rendez-vous et réservation en ligne. Créneaux, calendriers, ressources, confirmations, rappels. |
| **JayKoa** | Jay | Service unifié du domaine agenda : entrées, détection de conflits, vue calendrier agrégée, fuseaux, export (iCal, PDF). |
| **JayKonta** | Jay | Comptabilité et budget multi-échelle. Deux points d'entrée : **JayBudget** (perso) et **JayKonta** (entreprise : devis, facturation, comptabilité). |
| **JayFaim** | Jay | Réservation de tables et commande en ligne de nourriture (restaurants, traiteurs, food trucks). |
| **MiyukiniSurvivor** | Jeux | Jeu hybride Survivor + Tower Defense (voir [section 8](#8-jeux-miyukini--gameplay-et-influences-croisées)). |
| **MiyuClicker** | Jeux | Idle/Clicker + Grande stratégie (carte, conquête). |

**Documentation** : [docs/services/](docs/services/) — chaque service dispose d'un document fondateur, d'analyses des besoins par public, de parcours capacités/livrables et de documents **Opérateurs et Toolkits**.

### Prochaine phase : implémentation des Opérateurs

Une fois les services suffisamment spécifiés (fondateurs, analyses, Opérateurs et Toolkits), le travail se déplacera vers l'**implémentation des Opérateurs** (Strate 7). Les Opérateurs orchestrent les Toolkits implémentés — seuls ou en équipe — en fonction des besoins des services consommés par les utilisateurs, sous gouvernance (StrongFather, Mandats de Permission, Contrats d'équipe).

### Maturité conceptuelle

La pyramide, les Cores, les Lois d'autonomie, les contrats de sécurité et de gouvernance d'écosystème sont **stabilisés et documentés**. Le Kernel et les Cores du workspace sont en place ; l'outillage MIP (index structurel MSCM) est disponible. Les **services Jay** (JayFestival, JayXpose, JayRDV, JayKoa, JayKonta, JayFaim) et les **jeux** (MiyukiniSurvivor, MiyuClicker) sont en phase de conception produit et de spécification des besoins en Opérateurs.

---

## 7. Écosystème Jay — Services interopérables

### Vue d'ensemble

Les **services Jay** forment une famille de services métier conçus pour fonctionner ensemble dans l'environnement COG unifié. Chaque service Jay est un Opérateur (ou une Équipe d'Opérateurs) gouverné, autonome dans son domaine, mais capable de **collaborer avec les autres** sous gouvernance stricte (Mandats de Permission, Contrats d'équipe, BondingBrother).

> **L'interpolarité Jay : chaque service enrichit les autres sans jamais les envahir.**

### Les 6 services Jay

| Service | Domaine | Description |
|---------|---------|-------------|
| **JayFestival** | Événementiel | Gestion complète d'événements et festivals : création d'éditions, gestion des organisateurs, exposants et visiteurs, catalogue d'événements, candidatures, stands, billetterie, espace visiteur (jeux, ateliers, réservations). Hub central de l'écosystème événementiel. |
| **JayXpose** | Identité exposant | Identité professionnelle de l'exposant : profil enrichi (raison sociale, juridique, contacts multiples), catalogue de produits, site vitrine complet (4 pages, URL unique, SEO), coffre-fort documentaire sécurisé (RIB, KBIS, assurances, licences), référencement dans l'annuaire. |
| **JayRDV** | Réservation | Prise de rendez-vous et réservation en ligne (B2B2C). Créneaux, calendriers, ressources, confirmations, rappels, réduction des no-shows. Intégration site/CRM. |
| **JayKoa** | Agenda | Service unifié du domaine agenda. Agrège toutes les entrées temporelles (RDV, éditions, ateliers), détecte les conflits, fournit une vue calendrier unifiée, gère les fuseaux horaires, exporte en iCal/PDF. |
| **JayKonta** | Comptabilité | Comptabilité et budget multi-échelle. Deux points d'entrée : **JayBudget** (budgets perso et occasionnels) et **JayKonta** (devis, facturation, comptabilité entreprise, déclarations). |
| **JayFaim** | Restauration | Réservation de tables et commande en ligne de nourriture (restaurants, traiteurs, food trucks). Gestion des menus, commandes, créneaux, encaissements. |

### Carte d'interopérabilité

```
                          ┌─────────────┐
                          │   JayKoa    │ ← Agenda unifié
                          │  (Agenda)   │
                          └──────┬──────┘
                                 │ agrège dates
                    ┌────────────┼────────────┐
                    │            │             │
             ┌──────┴──────┐  ┌─┴───────────┐ │
             │   JayRDV    │  │ JayFestival  │ │
             │(Réservation)│  │(Événements)  │ │
             └──────┬──────┘  └──┬──┬──┬─────┘ │
                    │            │  │  │        │
                    │     ┌──────┘  │  └──────┐ │
                    │     │        │         │ │
               ┌────┴─────┴──┐  ┌─┴────┐ ┌──┴─┴──────┐
               │  JayKonta   │  │JayX- │ │  JayFaim   │
               │(Comptabilité)│  │pose  │ │(Restaura-  │
               └─────────────┘  │(Expo-│ │   tion)    │
                                │sant) │ └────────────┘
                                └──────┘
```

### Interopérabilité détaillée

| Flux | Direction | Données échangées | Gouvernance |
|------|-----------|-------------------|-------------|
| **JayXpose → JayFestival** | Lecture | Profil exposant, catalogue produits, documents partagés (coffre-fort). | Mandat de Permission ; partage documents = acte explicite de l'exposant. |
| **JayFestival → JayXpose** | Demandes | Demandes de documents pour candidatures, notifications (acceptation, rejet). | BondingBrother (médiation). |
| **JayFaim → JayFestival** | Lecture | Stands restauration, menus, disponibilités sur événement. | Mandat inter-services. |
| **JayKoa ← JayRDV** | Agrégation | Entrées agenda (RDV, créneaux, exceptions). | Contrat de lecture. |
| **JayKoa ← JayFestival** | Agrégation | Éditions, participations, ateliers réservés. | Contrat de lecture. |
| **JayKonta ← JayFestival** | Facturation | Budget par édition, devis et factures exposants. | Mandat de Permission. |
| **JayKonta ← JayRDV** | Facturation | Facturation professionnels, abonnements, encaissements. | Mandat de Permission. |
| **JayXpose → JayKonta** | Partage | RIB partagé depuis le coffre-fort documentaire. | Mandat explicite + acceptation exposant. |
| **JayXpose → JayRDV** | Lien | Lien depuis la vitrine exposant vers la prise de rendez-vous. | Lecture publique (lien). |

### Avantages de l'environnement unifié COG

L'exécution de tous les services Jay dans un même environnement COG apporte des avantages structurels impossibles avec une architecture en silos :

| Avantage | Description |
|----------|-------------|
| **Identité unique** | Un utilisateur = un profil. L'exposant crée son profil JayXpose une fois et le réutilise dans JayFestival, JayKonta, JayRDV. Pas de re-saisie, pas de comptes multiples. |
| **Données sans duplication** | Le profil, le catalogue et les documents vivent dans JayXpose (source unique). JayFestival, JayKonta et les autres lisent, ne dupliquent jamais. |
| **Coffre-fort centralisé** | Un document (assurance, KBIS, RIB) est uploadé une fois et sert pour N candidatures, N événements, N services — avec partage gouverné, document par document. |
| **Agenda unifié** | JayKoa agrège tous les événements (RDV JayRDV, éditions JayFestival, échéances JayKonta) dans une seule vue calendrier. Détection automatique des conflits. |
| **Facturation transversale** | JayKonta facture les exposants (via JayFestival), les professionnels (via JayRDV), les restaurateurs (via JayFaim) avec les mêmes outils comptables et la même gouvernance. |
| **Confidentialité souveraine** | L'exposant contrôle champ par champ ce qui est visible (public, authentifié, organisateur, privé). Chaque partage est un acte explicite, traçable et révocable. |
| **Sécurité par niveaux** | WorrySentinel applique des niveaux de sécurité différenciés : Public (0) pour la vitrine, Sensitive (2) pour le profil, Critical (3) pour les documents. Pas de sécurité uniforme — risque segmenté. |
| **Fonctionnement offline** | Chaque service fonctionne localement (LOI-1, LOI-2). Un organisateur de festival peut gérer ses exposants, consulter les fiches et le catalogue même sans connexion internet. |
| **Évolution sans rupture** | Ajouter un nouveau service Jay (ex. JayFaim) n'impacte pas les services existants. L'interpolarité est additive : chaque service enrichit l'écosystème sans modifier les autres. |

### Principe d'interpolarité

> **Les services Jay ne sont pas des applications isolées. Ce sont des Opérateurs gouvernés qui collaborent sous Mandat de Permission dans un environnement COG unifié.**

L'interpolarité repose sur trois piliers :

1. **Source unique** — Chaque donnée a un propriétaire (un service) et les autres lisent, ne dupliquent jamais.
2. **Partage gouverné** — Tout échange inter-services passe par BondingBrother et est encadré par un Mandat de Permission émis par StrongFather.
3. **Enrichissement additif** — Chaque nouveau service enrichit les autres sans les modifier. JayXpose enrichit JayFestival (catalogue dans le répertoire) ; JayFaim enrichit JayFestival (restauration sur événement) ; JayKoa unifie les agendas.

Référence : [Miyukini Conceptual References - Interpolarite Services Jay](docs/reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

## 8. Jeux Miyukini — Gameplay et influences croisées

### Vue d'ensemble

Les jeux Miyukini ne sont pas des applications isolées : ce sont des **Opérateurs gouvernés** qui s'exécutent dans l'environnement COG, bénéficient des mêmes Toolkits que les services métier, et peuvent **interagir avec les autres services et entre eux**.

### Jeux en développement

| Jeu | Genre | Description |
|-----|-------|-------------|
| **Miyukini Survivor** | Survivor + Tower Defense | Le joueur se déplace en 8 directions, attaque (mêlée, armes de jet, sortilèges), protège le Château au centre de la carte. Phase Préparation (construction de tours, upgrades) et phase Bataille (vagues d'ennemis). Mode 2 joueurs local. Bestiaire riche, système de loot (préfixes/suffixes), équipement. |
| **MiyuClicker** | Idle/Clicker + Grande stratégie | Gestion de ressources, production automatique, et conquête de carte (type Risk). Démo montrant la coexistence de plusieurs Opérateurs (UI, simulation, combat, sauvegarde, carte) dans un environnement COG. |

### Influences croisées entre jeux

Les jeux Miyukini sont conçus pour que la **progression dans un jeu puisse influencer un autre** :

| Influence | Source | Cible | Mécanisme |
|-----------|--------|-------|-----------|
| Récompenses croisées | MiyuClicker | Miyukini Survivor | Ressources ou bonus débloqués dans MiyuClicker utilisables dans Miyukini Survivor (cosmétiques, bonus de départ). |
| Succès partagés | Miyukini Survivor | MiyuClicker | Battre un boss dans Survivor débloque un territoire ou un bonus dans MiyuClicker. |
| Profil joueur unifié | Tous les jeux | Tous les jeux | Un seul profil joueur dans le COG : statistiques, succès, temps de jeu — partagés entre tous les jeux. |

### Influences jeux / services

L'environnement COG unifié permet des **interactions entre jeux et services métier** — une possibilité unique liée à l'architecture Miyukini :

| Influence | Jeu | Service | Mécanisme |
|-----------|-----|---------|-----------|
| Gamification événements | Miyukini Survivor | JayFestival | Un festival peut proposer un mode spécial « Survivor » (challenge thématique, classement visiteurs, lots). Les visiteurs jouent sur le stand et les scores alimentent un classement JayFestival. |
| Récompenses fidélité | MiyuClicker | JayFestival / JayXpose | La participation à des événements JayFestival débloque des bonus dans MiyuClicker (territoire événement, skin exclusif). |
| Catalogue gamifié | Miyukini Survivor | JayXpose | Un exposant artisan peut présenter ses créations sous forme de « butin » dans Miyukini Survivor (arme inspirée du catalogue, skin thématique). |
| Vitrine interactive | MiyuClicker | JayXpose | La vitrine d'un exposant peut intégrer un mini-jeu MiyuClicker thématique (clicker aux couleurs de la marque). |

### Architecture technique

Les jeux sont implémentés comme des crates Rust dans le workspace :

| Crate | Jeu | Toolkits consommés |
|-------|-----|-------------------|
| `lord_of_the_castle` | Miyukini Survivor | egui/eframe (UI), MiyuClock (temps), KindMother (sauvegarde), sprites/animations. |
| `miyukini-central` (module clicker) | MiyuClicker | egui/eframe (UI), simulation, sauvegarde, carte. |

Les jeux sont exposés dans **Miyukini Central** (Hub Services) au même titre que les services métier — l'utilisateur les ouvre depuis le catalogue de services.

---

## 9. Gouvernance et sécurité

- **Zero-trust** : aucun appelant présumé valide ; toute intention évaluée selon les politiques.
- **Niveaux de sécurité** (0–4) et **états de confiance** (T0–T4) : gouvernés par WorrySentinel ; dégradation progressive, pas de blocage brutal.
- **Offline-first** : WriteIntent acceptés localement ; réconciliation explicite à la reconnexion.
- **MiyukiniAdmin** : Opérateur Souverain (Strate 9), exception — installation, diagnostic, arbitrage, accès exceptionnel ; strictement encadré.

---

## 10. À qui s'adresse Miyukini

| Acteur | Besoin |
|--------|--------|
| **Architectes système** | Autonomie structurelle, sécurité par conception, traçabilité auditable, fonctionnement déterministe en isolation |
| **Développeurs d'Opérateurs** | Collectivités, événements sans réseau fiable, IoT/edge, contextes réglementés |
| **Décideurs techniques** | Projets long terme (5–10 ans), systèmes critiques, contrôle total non négociable |

**Miyukini n'est pas destiné** aux projets exigeant une mise en production immédiate sans comprendre l'architecture, aux applications temps réel critique (&lt;100 ms), aux équipes refusant les contraintes de gouvernance, ou aux contextes où la connectivité permanente et la dépendance cloud sont assumées sans exigence d'autonomie.

---

## 11. Documentation de référence

| Thème | Document principal |
|-------|--------------------|
| **Terminologie** | [Glossaire officiel](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Architecture** | [Pyramide Architecture Complete](docs/reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| **Autonomie** | [Lois Autonomie Système](docs/reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| **COG** | [Definition COG](docs/reference/Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) |
| **Souveraineté environnement** | [Souverainete Environnement](docs/reference/Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |
| **Connexion inter-COG** | [Connexion Inter-COG](docs/reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |
| **Opérateurs** | [Operators et Terminologie](docs/reference/Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| **Tools et Toolkits** | [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| **Objectif final** | [Objectif Final](docs/reference/Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) |

**Services Jay** : [docs/services/](docs/services/) — 6 services Jay interopérables (JayFestival, JayXpose, JayRDV, JayKoa, JayKonta, JayFaim) + 2 jeux (MiyukiniSurvivor, MiyuClicker) : documents fondateurs, analyses par public, Opérateurs et Toolkits.
**Interpolarité des services Jay** : [Miyukini Conceptual References - Interpolarite Services Jay](docs/reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) — principe de couplage entre services Jay (JayXpose ↔ JayFestival, JayFaim ↔ JayFestival, JayKoa intégrateur des dates).
**JayXpose (documentation complète)** : [docs/services/JayXpose/](docs/services/JayXpose/) — 14 documents : profil, catalogue, vitrine, coffre-fort, annuaire, confidentialité, synchronisation JayFestival, base de données, niveaux de sécurité.  
**Kits d'Outils** : [docs/tools/_index.md](docs/tools/_index.md) — index des Toolkits documentés.  
**Protocoles** : [docs/protocols/](docs/protocols/) — MIP, écriture documentation conceptuelle, implémentation générale.  
**Cores** : [docs/core/](docs/core/) — par Core (StrongFather, KindMother, Master Butler, MiyukiniAdmin, etc.).  
**Stack UI (egui / eframe)** : [Miyukini - Stack UI egui eframe](docs/ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md) — applications desktop et web en pur Rust (Hub, clients natifs).  
**Miyukini Central (Hub Services)** : [Miyukini Conceptual References - Miyukini Central Hub Services](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) — point d'entrée COG : écran de chargement, HUB avec sidebar (recherche, filtres, grille/liste), catalogue de services (ServiceMeta), overlays Profil/Paramètres, thème clair/obscur persistant.

---

## 12. Licence

Miyukini est distribué sous une **politique de licence duale** :

- **Usage domestique / personnel** (personne physique, à des fins non commerciales) : **gratuit** — voir le fichier [LICENSE](LICENSE) à la racine du dépôt.
- **Usage par une société ou une collectivité** (entreprise, association, administration, établissement public, etc.) : **payant** — une **licence commerciale** est requise.

Détails et conditions : [Miyukini — Politique de licence](docs/legal/Miyukini%20-%20Politique%20de%20Licence.md).

---

## 13. Conclusion

Miyukini ne vise pas à être le plus rapide ni le plus flexible, mais **prévisible, traçable, autonome et structurellement sécurisé**. Il demande un investissement initial (architecture en strates, contraintes de gouvernance, invariants) en échange de garanties : fonctionnement déterministe en isolation, sécurité par conception, évolution sans rupture, traçabilité complète.

*« Miyukini n'est pas une bibliothèque. C'est un environnement gouverné dans lequel des Opérateurs opèrent. »*

---

## 14. Log de rédaction

**2026-02-06 — Écosystème Jay, jeux et interopérabilité**

- Ajout section **7. Écosystème Jay — Services interopérables** : présentation des 6 services Jay (JayFestival, JayXpose, JayRDV, JayKoa, JayKonta, JayFaim), carte d'interopérabilité (schéma ASCII), tableau des flux inter-services, avantages de l'environnement COG unifié (identité unique, données sans duplication, coffre-fort centralisé, agenda unifié, facturation transversale, confidentialité souveraine, sécurité par niveaux, offline-first, évolution sans rupture), principe d'interpolarité.
- Ajout section **8. Jeux Miyukini — Gameplay et influences croisées** : Miyukini Survivor et MiyuClicker, influences croisées entre jeux (récompenses, succès, profil joueur unifié), influences jeux/services (gamification événements JayFestival, récompenses fidélité, catalogue gamifié JayXpose, vitrine interactive).
- Mise à jour section **6. État actuel et roadmap** : tableau enrichi des services (Jay + Jeux), nomenclature clarifiée (famille Jay vs Miyukini).
- Mise à jour section **Documentation de référence** : ajout lien JayXpose (14 documents).
- Renumérotation des sections (7→9, 8→10, 9→11, 10→12, 11→13, 12→14).
- Date de dernière mise à jour : 2026-02-06.

**2026-02-02 — Miyukini Central (Hub)**

- Ajout section **Miyukini Central (Hub Services)** dans l'état actuel : écran de chargement (barre à-coups, phrases aléatoires), HUB avec sidebar (recherche, filtres par catégorie, affichage Grille/Liste), cartes de services (ServiceMeta), services MVP (Calculatrice, Jeu, Lord of the Click, Traitement de texte, Notes, Miyukini UI Library), overlays Profil/Paramètres, thème persistant.
- Mise à jour référence documentation Miyukini Central (point d'entrée COG, fonctionnalités actuelles).
- Date de dernière mise à jour : 2026-02-02.

**2026-01-31 — Correction état réel et suppression répétitions**

- Mise à jour section **État actuel** : les Toolkits sont **implémentés** (49 crates dans le workspace), pas seulement documentés. Phase 1 (squelettes) complète, Phase 2 (implémentation progressive) en cours.
- Réorganisation section **Services** : simplification et clarification du lien Service → Opérateurs → Toolkits implémentés.
- Suppression des répétitions entre "Mécaniques générales" et "État actuel" : consolidation du message sur le rôle des services et l'orchestration des Toolkits.
- Reformulation **Prochaine phase** : implémentation des Opérateurs une fois les services suffisamment spécifiés, avec référence aux Toolkits déjà implémentés.

**2026-01-30 — Refonte README racine**

- Ajout section **Licence** : politique duale (usage domestique gratuit, sociétés et collectivités payant) ; lien vers LICENSE et docs/legal/Miyukini - Politique de Licence.md.
- Suppression de l'arborescence du projet et de la cartographie des crates (aspects purement techniques).
- Ajout section **Environnements, fédération et sécurité** : identification des environnements (COG, LSI/VID/WID), échanges entre environnements fédérés (visite gouvernée, Passeport, Visa, Bridge), logique de sécurité sous-jacente (identité ≠ autorité, un seul souverain, zero-trust).
- Recentrage sur la **vision**, la **philosophie**, les **mécaniques générales** (Service → Opérateur(s) → Tools/Toolkits → Gouvernance).
- Ajout de la **méthodologie de développement** : Idée → Analyse PR équivalent → Transcription conceptuelle (référence) → Documentation enrichie (contrats, implémentation, sécurité, planification et bornage) → Implémentation → Test et audit.
- Ajout de l'**état actuel** : travail concentré sur les Toolkits pour couvrir le spectre des besoins ; **phase suivante** : Opérateurs qui orchestrent les Outils seuls ou en équipe selon les services consommés par les utilisateurs.
- Table des matières simplifiée ; documentation de référence conservée sous forme de liens, sans arborescence détaillée.

**2026-01-29 — Réorganisation README racine (version précédente)**

- Structure en 6 parties ; arborescence, cartographie crates, documentation avec mapping dossiers.

---

**Document** : README racine officiel  
**Dernière mise à jour** : 2026-02-06  
**Références** : Glossaire officiel, Pyramide Architecture Complete, Lois d'autonomie, Objectif Final, Tools et Toolkits (implémentés), docs/services (Services en cours d'élaboration)
