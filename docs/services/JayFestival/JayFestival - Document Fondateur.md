# JayFestival â€” Document fondateur

## Contexte

**JayFestival** est le service officiel Miyukini dÃ©diÃ© Ã  la **gestion dâ€™Ã©vÃ©nements et de festivals** au sein de lâ€™Ã©cosystÃ¨me COG. Il reprend et dÃ©veloppe les spÃ©cificitÃ©s issues de **Catakana Orga** pour les porter Ã  lâ€™Ã©chelle de la plateforme : organisation de la **macro** (catalogue, annuaires, gouvernance), **distribution du service** auprÃ¨s des **organisateurs**, des **exposants** et des **visiteurs** (qui disposent chacun dâ€™un espace dÃ©diÃ© et dâ€™un compte cross-Ã©vÃ©nement).

**Tous les types de comptes sont cross-Ã©vÃ©nements** : un organisateur peut gÃ©rer plusieurs festivals/Ã©vÃ©nements ; un exposant peut participer Ã  plusieurs festivals et dispose de son propre dashboard ; un visiteur peut suivre ses activitÃ©s sur plusieurs Ã©vÃ©nements. La plateforme propose une **gestion dâ€™agenda** pour Ã©viter les conflits de dates (ex. quâ€™un exposant ne sâ€™inscrive pas Ã  deux Ã©vÃ©nements Ã  la mÃªme date).

Ce document est le **document fondateur** du service : il en fixe la raison dâ€™Ãªtre, la vision, le modÃ¨le de distribution (organisateurs, exposants, visiteurs) et les principes directeurs. Il sâ€™adresse aux parties prenantes internes et externes (Ã©quipes produit, technique, partenaires, organisateurs, exposants, visiteurs).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Service JayFestival â€” dÃ©finition, positionnement, macro et distribution.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es, contrats dâ€™API, implÃ©mentation des crates (rÃ©fÃ©rencÃ©s dans dâ€™autres documents).
- **RÃ©fÃ©rences** : Glossaire Miyukini, Vision stratÃ©gique, Audit Catakana â†’ Miyukini COG B2B2C.

---

## 1. Raison dâ€™Ãªtre

### 1.1 Proposition de valeur

**JayFestival** permet Ã  des **organisateurs** (associations, collectivitÃ©s, sociÃ©tÃ©s) dâ€™utiliser une plateforme gouvernÃ©e pour :

- **CrÃ©er et gÃ©rer plusieurs Ã©vÃ©nements** (Ã©ditions) : un mÃªme organisateur peut organiser **plusieurs festivals/Ã©vÃ©nements** ; paramÃ¨tres, dates, lieu, programme, plan de salle, budget, documents par Ã©dition.
- **GÃ©rer leurs exposants** : annuaire, candidatures, validation, facturation, emplacements.
- **Exposer leur offre** dans un **catalogue commun** : annuaire dâ€™Ã©vÃ©nements, rÃ©pertoire dâ€™organisateurs, rÃ©pertoire dâ€™exposants, accessible en B2B2C.
- **Proposer des services aux visiteurs** : jeux, concours, inscriptions ateliers, etc. â€” chaque organisateur choisit ce quâ€™il met Ã  disposition pour ses Ã©vÃ©nements.

Les **exposants** disposent de leur **propre dashboard** : candidatures, participations Ã  **plusieurs festivals**, documents, factures ; la plateforme propose une **gestion dâ€™agenda** pour Ã©viter quâ€™un exposant sâ€™inscrive Ã  deux Ã©vÃ©nements Ã  la mÃªme date (conflits de dates dÃ©jÃ  rencontrÃ©s en pratique).

Les **visiteurs** disposent dâ€™un **espace dÃ©diÃ©** : onboarding par festival ou par groupe de festivals, compte cross-Ã©vÃ©nement pour suivre leurs activitÃ©s et organiser leur visite (agenda, compte Ã  rebours, billet, rÃ©servation, pass VIP, etc.).

La plateforme propose le **service** ; les organisateurs lâ€™**adaptent** Ã  leurs besoins (identitÃ©, rÃ¨gles, contenu) sans en dÃ©tenir la gouvernance technique. **Tous les comptes sont cross-Ã©vÃ©nements** (organisateur, exposant, visiteur). Le modÃ¨le sâ€™apparente Ã  un **Store** : un lieu unique oÃ¹ lâ€™on dÃ©couvre les Ã©vÃ©nements, les organisateurs et les exposants, et oÃ¹ chaque type dâ€™acteur gÃ¨re son parcours sur plusieurs Ã©vÃ©nements.

### 1.2 HÃ©ritage Catakana Orga

Les spÃ©cificitÃ©s fonctionnelles du service sâ€™appuient sur lâ€™expÃ©rience et le pÃ©rimÃ¨tre de **Catakana Orga** :

- Gestion des **Ã©ditions** (Ã©vÃ©nements) avec tableau de bord par Ã©dition.
- Gestion des **exposants** : fiche, statuts, candidatures, documents, devis et factures.
- **Plan de salle** interactif (zones, stands, attribution).
- **Programme** : animations, crÃ©neaux, salles/scÃ¨nes, conflits horaires.
- **Budget** par Ã©dition, documents et lÃ©gal, notifications et communication.
- **RÃ´les** : admin, manager, exposant, bÃ©nÃ©vole â€” avec permissions et Mandats.
- **Auth** : JayFestival dispose dâ€™**une Auth Ã  lui**, dÃ©rivÃ©e de celle de Catakana, qui utilise lâ€™Auth Supabase ; en alpha, lâ€™Auth JayFestival sâ€™appuie sur Supabase Auth (voir [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md)).

Ces capacitÃ©s sont **reprises, normalisÃ©es et distribuÃ©es** dans le cadre COG : OpÃ©rateurs, Kits dâ€™outils, Contrats dâ€™Ã©quipe, Mandats de Permission.

---

## 2. Vision

### 2.1 Ã‰noncÃ© de vision

> **JayFestival** est le service de rÃ©fÃ©rence Miyukini pour la gestion dâ€™Ã©vÃ©nements et de festivals : une plateforme gouvernÃ©e, en B2B2C, oÃ¹ **tous les comptes sont cross-Ã©vÃ©nements** â€” les organisateurs gÃ¨rent plusieurs festivals, les exposants participent Ã  plusieurs Ã©vÃ©nements avec leur dashboard, les visiteurs organisent leur visite sur plusieurs Ã©vÃ©nements â€” et oÃ¹ le public dÃ©couvre un annuaire dâ€™Ã©vÃ©nements, un rÃ©pertoire dâ€™organisateurs et un rÃ©pertoire dâ€™exposants.

### 2.2 Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather (dÃ©cisions), KindMother (donnÃ©es), Master Butler (permissions), WorrySentinel (sÃ©curitÃ©). Les organisateurs opÃ¨rent dans le cadre de Mandats de Permission. |
| **B2B2C** | La plateforme livre le service aux **organisateurs** (B2B) ; les organisateurs exposent **Ã©vÃ©nements et exposants** aux **utilisateurs finaux** (B2C). |
| **Store** | Un catalogue commun : annuaire dâ€™Ã©vÃ©nements + rÃ©pertoire dâ€™organisateurs + rÃ©pertoire dâ€™exposants, avec recherche, filtres et FaÃ§ade publique gouvernÃ©e. |
| **Comptes cross-Ã©vÃ©nements** | **Tous les types de comptes** (organisateur, exposant, visiteur) sont cross-Ã©vÃ©nements : un organisateur peut organiser plusieurs festivals ; un exposant peut sâ€™inscrire Ã  plusieurs festivals et dispose de son dashboard ; un visiteur peut suivre ses activitÃ©s sur plusieurs Ã©vÃ©nements. |
| **Gestion dâ€™agenda** | Une fonctionnalitÃ© de **gestion dâ€™agenda** (calendrier, conflits de dates) permet dâ€™Ã©viter quâ€™un exposant â€” ou un visiteur â€” ne sâ€™inscrive Ã  deux Ã©vÃ©nements Ã  la mÃªme date ; besoin dÃ©jÃ  identifiÃ© en pratique. |
| **Espace visiteur** | Les visiteurs disposent dâ€™un espace dÃ©diÃ© pour suivre leurs activitÃ©s et organiser leur visite (agenda, billets, rÃ©servations, pass VIP, etc.), avec onboarding possible par festival ou par groupe de festivals. |
| **RÃ©utilisabilitÃ©** | Le service sâ€™appuie sur les Kits dâ€™outils Miyukini existants (Miyauth, Miyuinvoice, Miyucontacts, Miyusearch, Miyunotify, Miyubooking, etc.) et dÃ©finit les OpÃ©rateurs et Kits spÃ©cifiques Â« Ã©vÃ©nement Â» (Ã‰dition, Exposant, Plan de salle, Programme, Candidature, Visiteur, **Agenda cross-Ã©vÃ©nements**). |
| **Autonomie organisateur** | Chaque organisateur dispose dâ€™un espace gouvernÃ© (**plusieurs Ã©ditions**, ses exposants, son Ã©quipe, les services visiteur quâ€™il active) sans empiÃ©ter sur la souverainetÃ© des donnÃ©es ni sur les autres organisateurs. |

---

## 3. Macro : organisation du service au niveau plateforme

### 3.1 Trois piliers du catalogue (Store)

| Pilier | Contenu | RÃ´le |
|--------|---------|------|
| **Annuaire des Ã©vÃ©nements** | Liste/carte des Ã©ditions (Ã©vÃ©nements) publiÃ©es ; filtres (date, lieu, organisateur, thÃ¨me) ; fiche Ã©vÃ©nement (prÃ©sentation, dates, lieu, organisateur, exposants, programme public). | DÃ©couverte des Ã©vÃ©nements par le public et les professionnels. |
| **RÃ©pertoire des organisateurs** | Liste des structures organisatrices ; fiche organisateur (nom, Ã©vÃ©nements, contact, charte). | Identification et confiance dans les organisateurs. |
| **RÃ©pertoire des exposants** | Liste des exposants (global ou par Ã©vÃ©nement) ; fiche exposant (entreprise, stands, Ã©ditions participÃ©es, contact). | DÃ©couverte des exposants et mise en relation. |

La **macro** dÃ©signe tout ce qui est gÃ©rÃ© au **niveau plateforme** : catalogue commun, gouvernance des accÃ¨s, annuaires, recherche (Miyusearch), FaÃ§ade publique gouvernÃ©e. Elle ne contient pas la logique mÃ©tier dÃ©taillÃ©e de chaque Ã©dition (celle-ci relÃ¨ve de lâ€™espace organisateur).

### 3.2 Gouvernance macro

- **StrongFather** : dÃ©cision dâ€™ouverture dâ€™un nouvel organisateur, validation des contrats, rÃ©vocation de Mandats.
- **Master Butler** : permissions par rÃ´le (plateforme, organisateur, exposant, visiteur, public) ; accÃ¨s aux annuaires et aux espaces (organisateur, visiteur, exposant).
- **KindMother** : persistance des entitÃ©s catalogue (Ã©vÃ©nements, organisateurs, exposants) et des donnÃ©es par Ã©dition.
- **WorrySentinel** : niveaux de sÃ©curitÃ© et Ã©tats de confiance pour les donnÃ©es et les accÃ¨s.

Les **Mandats de Permission** encadrent ce que chaque organisateur peut faire (crÃ©er des Ã©ditions, gÃ©rer des exposants, publier au catalogue, etc.).

### 3.3 Politique de rÃ©sidence des donnÃ©es sensibles (exposants)

Les donnÃ©es personnelles et mÃ©tier des **exposants** (fiche, candidatures, documents, facturation) sont soumises Ã  la **politique de rÃ©sidence centralisÃ©e** (voir [Miyukini Conceptual References - Politique Residence Donnees Sensibles](..//..//miyukini-webway-system//reference//_index.md)) :

- **COG de rÃ©fÃ©rence** pour ces donnÃ©es : **COG de l'organisateur** (ou COG du Service Festival, selon architecture retenue). La copie canonique rÃ©side sur ce COG (Instance MÃ¨re KindMother).
- **Effet** : en cas de coupure du terminal de l'exposant (rÃ©seau, appareil), les donnÃ©es restent **disponibles pour les organisateurs** sur le COG de rÃ©fÃ©rence.
- Le terminal exposant accÃ¨de via Visite gouvernÃ©e ou synchronisation ; il ne dÃ©tient pas la seule copie de ces donnÃ©es.

---

## 4. Distribution : le service au niveau des organisateurs

### 4.1 ModÃ¨le de distribution

La **distribution** dÃ©signe la maniÃ¨re dont le service est **livrÃ© aux organisateurs** et utilisÃ© par eux :

1. **Onboarding organisateur** : crÃ©ation du compte organisateur (Miyauth, Miyuprofile), attribution des permissions (Master Butler), Ã©mission du Mandat de Permission pour gÃ©rer **plusieurs Ã©ditions** et exposants. Le compte est **cross-Ã©vÃ©nement** : un mÃªme organisateur peut organiser plusieurs festivals/Ã©vÃ©nements.
2. **Espace organisateur** : tableau de bord, **liste de toutes ses Ã©ditions** (multi-festivals), accÃ¨s par Ã©dition (dashboard Ã©dition : plan, programme, exposants, candidatures, budget, documents).
3. **Personnalisation** : lâ€™organisateur configure chacune de ses Ã©ditions (nom, dates, lieu, thÃ¨me, rÃ¨gles), ses Ã©quipes (rÃ´les, bÃ©nÃ©voles), sa communication (Miyunotify), sans modifier la gouvernance ni les Kits de la plateforme.
4. **Publication au catalogue** : les Ã©ditions validÃ©es peuvent Ãªtre exposÃ©es dans lâ€™annuaire des Ã©vÃ©nements ; lâ€™organisateur apparaÃ®t dans le rÃ©pertoire des organisateurs ; les exposants peuvent apparaÃ®tre dans le rÃ©pertoire des exposants (selon politique plateforme).

### 4.2 RÃ´les cÃ´tÃ© organisateur

| RÃ´le | PÃ©rimÃ¨tre |
|------|------------|
| **Admin organisateur** | Gestion complÃ¨te de la structure : **toutes ses Ã©ditions** (multi-festivals), Ã©quipe, paramÃ¨tres, publication catalogue. |
| **Manager** | Gestion opÃ©rationnelle dâ€™une ou plusieurs Ã©ditions : exposants, plan, programme, budget, documents. |
| **Exposant** | **Dashboard exposant dÃ©diÃ©** : candidatures, **participations Ã  plusieurs festivals**, documents, factures (Miyuinvoice), **agenda** pour Ã©viter les conflits de dates (inscription Ã  deux Ã©vÃ©nements Ã  la mÃªme date). |
| **BÃ©nÃ©vole** | AccÃ¨s limitÃ© selon attribution (zones, crÃ©neaux, informations de terrain). |

Les rÃ´les sont gÃ©rÃ©s par **Master Butler** ; les Contrats dâ€™Ã©quipe dÃ©finissent les flux autorisÃ©s entre OpÃ©rateurs (Ã‰dition, Exposant, Programme, Plan de salle, etc.).

### 4.3 Livrables par organisateur

Chaque organisateur, dans son espace gouvernÃ©, dispose de :

- **Ã‰ditions (multi-festivals)** : crÃ©ation et gestion de **plusieurs Ã©ditions** ; paramÃ©trage et tableau de bord par Ã©dition.
- **Exposants** : annuaire local, candidatures, validation, fiches, documents, devis/factures (Miyuinvoice).
- **Plan de salle** : zones, stands, attribution (OpÃ©rateur Plan de salle).
- **Programme** : animations, crÃ©neaux, salles (OpÃ©rateur Programme).
- **Budget** : revenus/dÃ©penses, ventilation (Miyucptaledger, Miyuexpense, Miyucomptareports).
- **Documents** : contrats types, CGV, rÃ¨glements (Miyucms, Miyumedia).
- **Notifications** : annonces, alertes (Miyunotify).
- **Services visiteur** : activation et paramÃ©trage par Ã©dition des services proposÃ©s aux visiteurs (jeux, concours, inscriptions ateliers, rÃ©servations, pass VIP, etc.) ; lâ€™organisateur choisit ce quâ€™il met Ã  disposition.

La **macro** (catalogue, annuaires, recherche) reste sous contrÃ´le plateforme ; la **micro** (contenu et opÃ©rations de chaque Ã©dition, dont services visiteur) reste sous contrÃ´le organisateur dans le cadre du Mandat.

---

## 5. Distribution : le service au niveau des exposants

### 5.1 Compte cross-Ã©vÃ©nement et dashboard exposant

Les **exposants** disposent dâ€™un **compte cross-Ã©vÃ©nement** et de leur **propre dashboard** pour leurs besoins :

- **Participation Ã  plusieurs festivals** : un mÃªme exposant peut **sâ€™inscrire et participer Ã  plusieurs Ã©vÃ©nements** ; son dashboard agrÃ¨ge candidatures, participations, documents et factures pour tous les festivals concernÃ©s.
- **Dashboard dÃ©diÃ©** : vue unifiÃ©e sur ses candidatures (en attente, validÃ©es, refusÃ©es), ses Ã©ditions en cours ou Ã  venir, ses documents et factures (Miyuinvoice), et son **agenda** cross-Ã©vÃ©nements.

### 5.2 Gestion dâ€™agenda et conflits de dates

Une **fonctionnalitÃ© de gestion dâ€™agenda** est proposÃ©e pour Ã©viter quâ€™un exposant (ou un visiteur) **ne sâ€™inscrive Ã  deux Ã©vÃ©nements Ã  la mÃªme date**. Besoin dÃ©jÃ  rencontrÃ© en pratique (Â« dÃ©jÃ  vu Â») :

- **Calendrier cross-Ã©vÃ©nements** : visualisation des dates des Ã©vÃ©nements auxquels lâ€™exposant est inscrit ou candidat ; alerte ou blocage en cas de chevauchement de dates.
- **CohÃ©rence** : la plateforme peut signaler un conflit de dates avant validation dâ€™une nouvelle candidature ou inscription, et lâ€™exposant peut organiser son planning sur plusieurs festivals sans double engagement.

Cette capacitÃ© relÃ¨ve de lâ€™OpÃ©rateur ou Kit **Agenda cross-Ã©vÃ©nements** (MiyuClock, Miyubooking, donnÃ©es dâ€™Ã©dition).

---

## 6. Distribution : le service au niveau des visiteurs

### 6.1 Espace dÃ©diÃ© visiteur

Les **visiteurs** disposent dâ€™un **espace dÃ©diÃ©** gouvernÃ©, distinct de la simple consultation du catalogue. Comme pour les organisateurs et les exposants, **tous les comptes sont cross-Ã©vÃ©nements** : un mÃªme compte visiteur permet de :

- **Sâ€™onboarder** par festival ou par **groupe de festivals** : le visiteur peut rejoindre un Ã©vÃ©nement isolÃ© ou une famille dâ€™Ã©vÃ©nements (ex. une sÃ©rie de festivals partenaires), selon ce que lâ€™organisateur ou la plateforme propose.
- **Suivre ses activitÃ©s et organiser sa visite** sur **plusieurs Ã©vÃ©nements**, sans recrÃ©er dâ€™identitÃ© Ã  chaque fois.

La **gestion dâ€™agenda** (voir Â§ 5.2) sâ€™applique aussi aux visiteurs : Ã©viter les inscriptions Ã  deux Ã©vÃ©nements ou crÃ©neaux Ã  la mÃªme date.

### 6.2 Compte cross-Ã©vÃ©nement (visiteur)

Le **compte cross-Ã©vÃ©nement** offre au visiteur une vue unifiÃ©e pour :

| CapacitÃ© | Description |
|----------|-------------|
| **Agenda** | Programme personnel : ateliers, animations, concours auxquels le visiteur est inscrit ou quâ€™il souhaite suivre, synchronisÃ© entre Ã©vÃ©nements. |
| **Compte Ã  rebours** | Jours/heures restants avant les Ã©vÃ©nements ou crÃ©neaux rÃ©servÃ©s. |
| **Billets / tickets** | AccÃ¨s centralisÃ© aux billets et tickets acquis (par Ã©vÃ©nement ou groupe dâ€™Ã©vÃ©nements). |
| **RÃ©servations** | Ateliers, crÃ©neaux, places rÃ©servÃ©s ; annulation ou modification dans le cadre des rÃ¨gles de lâ€™Ã©dition. |
| **Pass VIP** | Pass et avantages associÃ©s (par Ã©vÃ©nement ou multi-Ã©vÃ©nements), selon ce que lâ€™organisateur met en place. |
| **Suivi dâ€™activitÃ©s** | Historique et suivi des participations : jeux jouÃ©s, concours, ateliers suivis, rÃ©compenses, etc. |

La gouvernance (StrongFather, Master Butler, KindMother) garantit que les donnÃ©es du visiteur restent souveraines et que lâ€™accÃ¨s cross-Ã©vÃ©nement respecte les Mandats et les choix de chaque organisateur.

### 6.3 Services proposÃ©s aux visiteurs (activables par lâ€™organisateur)

Chaque **organisateur** peut **mettre Ã  disposition** pour ses Ã©vÃ©nements tout ou partie des services suivants. La plateforme les fournit ; lâ€™organisateur dÃ©cide lesquels activer et avec quelles rÃ¨gles :

| Service | Description |
|---------|-------------|
| **Jeux** | Jeux liÃ©s au festival (quizz, chasses au trÃ©sor, dÃ©fis) ; participation et suivi dans lâ€™espace visiteur. |
| **Concours** | Inscription et participation Ã  des concours ; rÃ©sultats, rÃ©compenses, historique. |
| **Inscriptions ateliers** | RÃ©servation de crÃ©neaux dâ€™ateliers ; annulation, rappels, intÃ©gration Ã  lâ€™agenda visiteur. |
| **RÃ©servations** | RÃ©servation de places, crÃ©neaux ou activitÃ©s (Miyubooking) ; billets et pass. |
| **Pass et avantages** | Pass VIP, pass journÃ©e, avantages fidÃ©litÃ© ; liaison avec le compte cross-Ã©vÃ©nement. |
| **Notifications** | Rappels, changements de programme, alertes (Miyunotify), selon prÃ©fÃ©rences visiteur et rÃ¨gles Ã©dition. |

Lâ€™organisateur configure, par Ã©dition ou par groupe dâ€™Ã©ditions, quels services sont ouverts et selon quelles conditions (places limitÃ©es, dates, publics). La **distribution** visiteur est ainsi **paramÃ©trable** par organisateur sans modifier la gouvernance plateforme.

### 6.4 Onboarding visiteur : par festival ou par groupe de festivals

- **Onboarding par festival** : le visiteur crÃ©e un compte ou se connecte dans le contexte dâ€™un seul Ã©vÃ©nement ; il peut ensuite Ã©tendre son compte Ã  dâ€™autres Ã©vÃ©nements (compte cross-Ã©vÃ©nement).
- **Onboarding par groupe de festivals** : lâ€™organisateur ou la plateforme propose une **famille dâ€™Ã©vÃ©nements** (ex. Â« Festivals partenaires 2026 Â») ; le visiteur sâ€™inscrit une fois et accÃ¨de Ã  tous les Ã©vÃ©nements du groupe avec le mÃªme compte, agenda et billets unifiÃ©s.

Cela permet des partenariats entre organisateurs (groupes de festivals) et une expÃ©rience visiteur cohÃ©rente sur plusieurs Ã©vÃ©nements.

---

## 7. Positionnement et communication

### 7.1 Message central (elevator pitch)

**JayFestival** est le service Miyukini pour organiser et promouvoir des Ã©vÃ©nements et festivals. **Tous les comptes sont cross-Ã©vÃ©nements** : les **organisateurs** peuvent gÃ©rer **plusieurs festivals** ; les **exposants** disposent de leur **dashboard** pour participer Ã  **plusieurs festivals** et bÃ©nÃ©ficier dâ€™une **gestion dâ€™agenda** pour Ã©viter les conflits de dates ; les **visiteurs** organisent leur visite sur plusieurs Ã©vÃ©nements (agenda, billets, rÃ©servations, pass VIP, jeux, concours, ateliers). Le public dÃ©couvre les Ã©vÃ©nements, les organisateurs et les exposants dans un catalogue commun, fiable et sÃ©curisÃ©.

### 7.2 Publics cibles

| Public | Message prioritaire |
|-------|----------------------|
| **Organisateurs** | Une plateforme complÃ¨te pour gÃ©rer **plusieurs festivals/Ã©vÃ©nements**, vos exposants et Ã©quipes, activer des services visiteurs (jeux, concours, ateliers), et publier dans un annuaire commun, dans le respect de la gouvernance et de la vie privÃ©e. Compte cross-Ã©vÃ©nements. |
| **Exposants** | Votre **dashboard exposant** : candidater et participer Ã  **plusieurs festivals**, suivre vos participations, documents et factures, et **gÃ©rer votre agenda** pour Ã©viter les inscriptions Ã  deux Ã©vÃ©nements Ã  la mÃªme date. Compte cross-Ã©vÃ©nements. Visible dans le rÃ©pertoire des exposants. |
| **Visiteurs** | Un espace dÃ©diÃ© et un compte cross-Ã©vÃ©nement : organisez votre visite (agenda, billets, rÃ©servations, pass VIP), participez aux jeux, concours et ateliers proposÃ©s par les festivals, et suivez vos activitÃ©s sur un ou plusieurs Ã©vÃ©nements. Onboarding par festival ou par groupe de festivals. |
| **Public (non connectÃ©)** | DÃ©couvrez les Ã©vÃ©nements, les organisateurs et les exposants dans un annuaire clair et Ã  jour. |
| **Partenaires / intÃ©grateurs** | Un service COG, modulaire, rÃ©utilisant les Kits Miyukini et extensible via OpÃ©rateurs et Mandats. |

### 7.3 DiffÃ©renciation

- **Gouvernance COG** : pas un simple SaaS Ã©vÃ©nementiel, mais un service inscrit dans lâ€™architecture Miyukini (Cores, OpÃ©rateurs, Mandats).
- **Comptes cross-Ã©vÃ©nements pour tous** : organisateur (plusieurs festivals), exposant (dashboard, plusieurs festivals, agenda), visiteur (plusieurs Ã©vÃ©nements).
- **Gestion dâ€™agenda** : Ã©viter quâ€™un exposant â€” ou un visiteur â€” sâ€™inscrive Ã  deux Ã©vÃ©nements Ã  la mÃªme date ; besoin dÃ©jÃ  vu en pratique.
- **B2B2C et Store** : catalogue commun (Ã©vÃ©nements, organisateurs, exposants) + espaces dÃ©diÃ©s organisateur, **exposant** (dashboard) et **visiteur**.
- **HÃ©ritage Ã©prouvÃ©** : spÃ©cificitÃ©s inspirÃ©es de Catakana Orga, portÃ©es Ã  lâ€™Ã©chelle et normalisÃ©es.

---

## 8. Prochaines Ã©tapes (orientation)

1. **Fonder** : valider ce document fondateur et le diffuser (interne / partenaires).
2. **SpÃ©cifier** : documenter les OpÃ©rateurs et Kits (Ã‰dition, Organisateur, Exposant, Visiteur, Plan de salle, Programme, Candidature, **Agenda cross-Ã©vÃ©nements**) et leurs Contrats dâ€™Ã©quipe.
3. **Macro** : dÃ©finir et implÃ©menter le catalogue (annuaires, recherche, FaÃ§ade publique).
4. **Distribution organisateur** : finaliser lâ€™onboarding organisateur (compte cross-Ã©vÃ©nement, **plusieurs festivals**), les Mandats et lâ€™espace organisateur (dashboard multi-Ã©ditions, exposants, plan, programme, budget, **activation des services visiteur**).
5. **Distribution exposant** : dashboard exposant, participation Ã  **plusieurs festivals**, **gestion dâ€™agenda** (conflits de dates), candidatures, documents, factures.
6. **Distribution visiteur** : espace dÃ©diÃ© visiteur, compte cross-Ã©vÃ©nement, onboarding par festival / groupe de festivals, agenda, billets, rÃ©servations, pass VIP, jeux/concours/ateliers, **gestion dâ€™agenda** (conflits de dates).
7. **Migration** : planifier la reprise des spÃ©cificitÃ©s Catakana Orga (voir audit B2B2C) et la convergence des Ã©crans et flux.

---

## 9. InterpolaritÃ© (services Jay)

JayFestival sâ€™intÃ¨gre avec les **services Jay** suivants : **JayXpose** (fiche et rÃ©pertoire exposants), **JayFaim** (restauration sur Ã©vÃ©nement), **JayKoa** (agenda agrÃ©gÃ©, conflits de dates), **JayKonta** (budget Ã©dition, devis et factures exposants). Les couplages sont explicites et gouvernÃ©s (Mandats de Permission, niveaux de sÃ©curitÃ©).

**RÃ©fÃ©rence dÃ©taillÃ©e** : [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) ; [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md).

---

## 10. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie officielle (OpÃ©rateur, Mandat, COG, etc.). |
| [Miyukini Conceptual References â€” Vision stratÃ©gique](..//..//miyukini-webway-system//reference//_index.md) | Objectifs stratÃ©giques, B2B2C, modÃ¨les de livraison. |
| [JayFestival - Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages avec JayXpose, JayFaim, JayKoa, JayKonta. |
| [Audit Catakana â†’ Miyukini COG B2B2C](..//..//_index.md) | Kits manquants, correspondance OpÃ©rateurs, besoins UI. |

---

**Document** : JayFestival â€” Document fondateur  
**Version** : 1.2  
**Date** : 2026-01-31  
**Statut** : Document fondateur â€” rÃ©fÃ©rence pour le service (comptes cross-Ã©vÃ©nements pour tous ; dashboard exposant ; gestion dâ€™agenda)


