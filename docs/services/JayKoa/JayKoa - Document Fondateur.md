# JayKoa â€” Document Fondateur

## Contexte

**JayKoa** est un **Service d'Agenda** au sein de l'Ã©cosystÃ¨me Miyukini COG (Core-Orchestrated Governance Environment).

Son objectif est de fournir une capacitÃ© complÃ¨te de gestion du temps : agendas, Ã©vÃ©nements, engagements, rendez-vous et planification, au bÃ©nÃ©fice des utilisateurs du COG.

JayKoa est conÃ§u comme un **Service transversal du COG**, spÃ©cialisÃ© dans le domaine temporel :

- Multi-agenda
- Multi-utilisateur
- Offline-first
- Synchronisable
- GouvernÃ© par les Cores Miyukini

JayKoa n'est pas une application.
C'est un **Service**, rendu par un ou plusieurs **OpÃ©rateurs**, conformÃ©ment au modÃ¨le COG.

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des OpÃ©rateurs gouvernÃ©s qui exÃ©cutent des rÃ´les pour leur compte.**

Ce document est le **document fondateur** du Service JayKoa : il en fixe la raison d'Ãªtre, le positionnement architectural, les rÃ¨gles de gouvernance, le modÃ¨le conceptuel des donnÃ©es et les interactions avec les autres Services du COG. Il s'adresse aux Ã©quipes produit, architecture, sÃ©curitÃ© et Ã  toute partie prenante de l'Ã©cosystÃ¨me Miyukini.

## PortÃ©e / Scope

- **Applicable Ã  :** Architecture JayKoa, interactions inter-Services, modÃ¨le de donnÃ©es conceptuel, gouvernance
- **Audience :** Architectes, Ã©quipes produit, Ã©quipes sÃ©curitÃ©, parties prenantes
- **Statut :** Document fondateur normatif â€” rÃ©fÃ©rence pour le Service

### Hors pÃ©rimÃ¨tre

- Aucun code
- Aucune implÃ©mentation technique
- Aucun choix UI
- Aucun framework
- Aucun endpoint ou API
- Aucune spÃ©cification de schÃ©ma de base de donnÃ©es

---

## 1. Contexte gÃ©nÃ©ral

JayKoa est un Service d'Agenda comparable, sur le plan fonctionnel, Ã  Google Agenda.

Il fournit une capacitÃ© complÃ¨te de gestion du temps au sein de l'Ã©cosystÃ¨me Miyukini :

| CapacitÃ© | Description |
|----------|-------------|
| **Agendas** | CrÃ©ation et gestion d'agendas personnels et partagÃ©s |
| **Ã‰vÃ©nements** | Gestion d'Ã©vÃ©nements temporels de toute nature |
| **Engagements** | Lecture rÃ©flÃ©chie des engagements et rendez-vous issus d'autres Services |
| **Planification** | Vision temporelle unifiÃ©e et consolidÃ©e |
| **Synchronisation** | Synchronisation d'Ã©vÃ©nements issus d'autres Services du COG |

JayKoa respecte les Lois d'Autonomie Miyukini :

- **LOI-1** â€” Aucune dÃ©pendance externe critique Ã  l'exÃ©cution
- **LOI-2** â€” Le systÃ¨me accepte l'isolement comme Ã©tat normal
- **LOI-3** â€” L'Ã©tat local est souverain
- **LOI-4** â€” Pas de temps global requis
- **LOI-5** â€” Le coÃ»t doit Ãªtre proportionnel au hardware

JayKoa fonctionne en mode **offline-first**. Il est capable d'opÃ©rer en isolation complÃ¨te, de stocker localement les donnÃ©es temporelles, et de synchroniser de maniÃ¨re diffÃ©rÃ©e lorsque les conditions le permettent.

---

## 2. IntÃ©rÃªt fondamental du Service

L'un des intÃ©rÃªts fondamentaux de JayKoa est sa capacitÃ© Ã  se synchroniser nativement avec **tous les Services du COG exposant des capacitÃ©s temporelles**.

JayKoa agit comme un **rÃ©ceptacle temporel transversal** :

- Il reÃ§oit des informations temporelles
- Il les traduit dans un modÃ¨le d'agenda
- Il les rend lisibles et exploitables dans le temps

Sans jamais devenir dÃ©pendant des Services sources.

### Principe fondateur

> **JayKoa ne crÃ©e pas les Ã©vÃ©nements mÃ©tier. Il reflÃ¨te, agrÃ¨ge et orchestre le temps issu des autres Services.**

### Synchronisation inter-Services

JayKoa est conÃ§u pour se synchroniser notamment avec :

| Service | Domaine temporel | Nature de la synchronisation |
|---------|-----------------|-------------------------------|
| **JayFestival** | Organisation d'Ã©vÃ©nements (festivals, concours, inscriptions, deadlines, dates clÃ©s) | Dates de festival, dates d'inscription, deadlines, Ã©vÃ©nements favoris |
| **JayRDV** | Booking et prise de rendez-vous (disponibilitÃ©s, crÃ©neaux, confirmations, annulations) | Rendez-vous confirmÃ©s, crÃ©neaux bloquÃ©s, modifications et annulations |

JayKoa peut synchroniser ces informations dans les agendas des utilisateurs en fonction :

- De leurs **prÃ©fÃ©rences**
- De leurs **abonnements**
- De leurs **Ã©vÃ©nements favoris**
- De leurs **engagements actifs**

### ExtensibilitÃ©

Tout Service du COG exposant des donnÃ©es temporelles compatibles peut devenir une source de synchronisation pour JayKoa :

- Services Ã©vÃ©nementiels
- Services de booking
- Services organisationnels
- Services de planification
- Tout Service produisant des plages temporelles qualifiÃ©es

La synchronisation inter-Services est :

- **DÃ©clarative** â€” chaque source dÃ©clare ses capacitÃ©s temporelles
- **GouvernÃ©e** â€” encadrÃ©e par les Cores et les Mandats de Permission
- **Non intrusive** â€” aucune modification des donnÃ©es source
- **Sans dÃ©pendance directe** â€” aucun couplage technique entre Services

---

## 3. Objectif de la documentation

Ce document a pour objectif de produire une rÃ©fÃ©rence :

- **Normative** â€” il fixe les rÃ¨gles et les limites du Service
- **Structurante** â€” il dÃ©finit le positionnement architectural de JayKoa dans le COG
- **Stable dans le temps** â€” il ne dÃ©pend d'aucun choix d'implÃ©mentation

### Ce document explique

- Ce que fait JayKoa
- Ce que JayKoa ne fait pas
- Comment JayKoa s'insÃ¨re dans l'architecture Miyukini
- Comment JayKoa interagit avec les autres Services du COG
- Comment JayKoa utilise KindMother pour la persistance
- Quelles sont les rÃ¨gles de sÃ©curitÃ© et de gouvernance de JayKoa

### Ce document ne contient pas

- Aucun code
- Aucune implÃ©mentation technique
- Aucun choix UI ou UX
- Aucun framework ou librairie
- Aucun endpoint, API ou schÃ©ma de donnÃ©es technique

---

## 4. PortÃ©e / Scope

### JayKoa FAIT

- GÃ©rer des agendas personnels et partagÃ©s
- GÃ©rer des Ã©vÃ©nements temporels
- GÃ©rer des engagements et rendez-vous en lecture rÃ©flÃ©chie
- Synchroniser des Ã©vÃ©nements issus d'autres Services du COG
- Consolider des informations temporelles multi-sources
- Offrir une vision temporelle unifiÃ©e
- S'abonner Ã  des flux temporels inter-Services
- Filtrer les Ã©vÃ©nements synchronisÃ©s
- Identifier l'origine de chaque Ã©vÃ©nement
- Marquer les Ã©vÃ©nements comme informatifs ou bloquants

### JayKoa NE FAIT PAS

- Aucune logique mÃ©tier Ã©vÃ©nementielle
- Aucune logique de rÃ©servation
- Aucune gestion de disponibilitÃ©
- Aucune notification technique
- Aucune authentification
- Aucune prise de dÃ©cision mÃ©tier
- Aucune modification des donnÃ©es des Services sources
- Aucune crÃ©ation d'Ã©vÃ©nement mÃ©tier dans un autre Service
- Aucune rÃ©solution de conflit de booking
- Aucun calcul de disponibilitÃ©

---

## 5. RÃ´le architectural

### Positionnement dans la Pyramide Miyukini

JayKoa est un **Service transversal du COG**, spÃ©cialisÃ© dans le domaine du temps.

Dans la Pyramide Miyukini, JayKoa se situe au niveau des **Services** (Strate 7 â€” OpÃ©rateurs), rendus par un ou plusieurs OpÃ©rateurs gouvernÃ©s qui s'appuient sur les Outils et Kits d'Outils (Strate 6), eux-mÃªmes gouvernÃ©s par les Cores (Strate 4).

| Strate | Ã‰lÃ©ment | RÃ´le vis-Ã -vis de JayKoa |
|--------|---------|--------------------------|
| **7** | OpÃ©rateurs JayKoa | ExÃ©cutent les rÃ´les du Service pour le compte de l'utilisateur |
| **6** | Outils & Kits d'Outils | CapacitÃ©s exÃ©cutables utilisÃ©es par les OpÃ©rateurs |
| **5** | BondingBrother | MÃ©diation entre les OpÃ©rateurs et les Cores |
| **4** | Cores | Gouvernent le comportement de JayKoa (StrongFather, KindMother, WorrySentinel, Master Butler, etc.) |

### TransversalitÃ©

JayKoa n'appartient Ã  aucun Service mÃ©tier. Il est **transversal** :

- Il peut consommer des capacitÃ©s temporelles provenant de n'importe quel Service du COG
- Il ne possÃ¨de pas les donnÃ©es mÃ©tier des Services sources
- Il ne duplique pas la logique mÃ©tier des Services sources
- Il traduit les informations temporelles dans son propre modÃ¨le d'agenda

### RÃ¨gles de couplage inter-Services

JayKoa n'initie jamais une action mÃ©tier dans un autre Service. Il ne fait que **reflÃ©ter l'Ã©tat temporel validÃ©** par le Service source.

| RÃ¨gle | Description |
|-------|-------------|
| **Reflet, pas action** | JayKoa reflÃ¨te le temps, il ne le dÃ©cide pas |
| **Lecture, pas Ã©criture** | JayKoa ne modifie jamais les donnÃ©es d'un Service source |
| **Traduction, pas copie** | JayKoa traduit les informations temporelles dans son modÃ¨le, il ne copie pas les donnÃ©es mÃ©tier |
| **DÃ©claratif, pas impÃ©ratif** | La synchronisation est dÃ©clarÃ©e, jamais imposÃ©e |
| **GouvernÃ©, pas libre** | Toute synchronisation passe par un Mandat de Permission |

---

## 6. ModÃ¨le conceptuel des donnÃ©es

Ce modÃ¨le dÃ©crit les notions fondamentales manipulÃ©es par JayKoa. Il s'agit d'un modÃ¨le **conceptuel**, sans schÃ©ma technique, sans structure de base de donnÃ©es, sans format d'Ã©change.

### 6.1 Agenda

Un **Agenda** est un conteneur temporel appartenant Ã  un utilisateur ou partagÃ© entre plusieurs utilisateurs.

- Un utilisateur peut possÃ©der plusieurs agendas
- Un agenda peut Ãªtre personnel ou partagÃ©
- Un agenda peut recevoir des Ã©vÃ©nements internes (crÃ©Ã©s par l'utilisateur) et des Ã©vÃ©nements synchronisÃ©s (issus d'autres Services)
- Un agenda possÃ¨de une identitÃ© propre et des rÃ¨gles de visibilitÃ©

### 6.2 Ã‰vÃ©nement

Un **Ã‰vÃ©nement** est une occurrence temporelle placÃ©e dans un agenda.

- Un Ã©vÃ©nement possÃ¨de une plage temporelle (dÃ©but, fin)
- Un Ã©vÃ©nement possÃ¨de un type (informatif, bloquant, annulÃ©, modifiÃ©)
- Un Ã©vÃ©nement peut Ãªtre interne (crÃ©Ã© dans JayKoa) ou synchronisÃ© (issu d'un Service source)
- Un Ã©vÃ©nement synchronisÃ© conserve la rÃ©fÃ©rence Ã  son Service source
- Un Ã©vÃ©nement ne contient jamais de donnÃ©es mÃ©tier appartenant au Service source â€” uniquement une reprÃ©sentation temporelle

### 6.3 Engagement temporel

Un **Engagement temporel** est un Ã©vÃ©nement issu d'un Service externe qui reprÃ©sente un engagement de l'utilisateur dans le temps.

- Rendez-vous confirmÃ© (issu de JayRDV)
- Participation Ã  un festival (issue de JayFestival)
- Inscription Ã  un atelier, une formation, un crÃ©neau

Un engagement temporel est toujours en **lecture rÃ©flÃ©chie** dans JayKoa : il est visible, mais JayKoa ne peut ni le modifier ni le supprimer. Seul le Service source dÃ©tient l'autoritÃ© sur l'engagement.

### 6.4 Occurrence

Une **Occurrence** est une instance unique d'un Ã©vÃ©nement dans le temps.

- Un Ã©vÃ©nement ponctuel produit une seule occurrence
- Un Ã©vÃ©nement rÃ©current produit plusieurs occurrences selon sa rÃ¨gle de rÃ©currence
- Chaque occurrence peut Ãªtre individuellement modifiÃ©e ou annulÃ©e (pour les Ã©vÃ©nements internes)
- Les occurrences d'Ã©vÃ©nements synchronisÃ©s reflÃ¨tent l'Ã©tat du Service source

### 6.5 RÃ¨gle de rÃ©currence

Une **RÃ¨gle de rÃ©currence** dÃ©finit le schÃ©ma de rÃ©pÃ©tition d'un Ã©vÃ©nement dans le temps.

- FrÃ©quence (quotidienne, hebdomadaire, mensuelle, annuelle, personnalisÃ©e)
- Intervalle
- Jours de la semaine, du mois, de l'annÃ©e
- Date de fin ou nombre d'occurrences
- Exceptions (dates exclues)

Les rÃ¨gles de rÃ©currence ne s'appliquent qu'aux Ã©vÃ©nements **internes** de JayKoa. Les Ã©vÃ©nements synchronisÃ©s arrivent comme des occurrences individuelles provenant du Service source.

### 6.6 Participant

Un **Participant** est un utilisateur associÃ© Ã  un Ã©vÃ©nement.

- PropriÃ©taire de l'Ã©vÃ©nement
- InvitÃ©
- Observateur

Les participants ne sont identifiÃ©s que par des rÃ©fÃ©rences. JayKoa ne stocke pas de donnÃ©es personnelles au-delÃ  de ce qui est nÃ©cessaire Ã  la reprÃ©sentation temporelle.

### 6.7 Source d'Ã©vÃ©nement

Une **Source d'Ã©vÃ©nement** identifie le Service du COG Ã  l'origine d'un Ã©vÃ©nement synchronisÃ©.

- Chaque Ã©vÃ©nement synchronisÃ© est marquÃ© de sa source
- La source est immuable une fois l'Ã©vÃ©nement crÃ©Ã©
- JayKoa distingue toujours un Ã©vÃ©nement interne d'un Ã©vÃ©nement synchronisÃ©
- L'utilisateur peut filtrer sa vue par source

| Source | Exemples d'Ã©vÃ©nements |
|--------|----------------------|
| **JayKoa** (interne) | Ã‰vÃ©nement crÃ©Ã© par l'utilisateur directement dans son agenda |
| **JayFestival** | Date de festival, deadline d'inscription, Ã©vÃ©nement favori |
| **JayRDV** | Rendez-vous confirmÃ©, crÃ©neau bloquÃ©, modification, annulation |
| **Autre Service COG** | Toute plage temporelle exposÃ©e par un Service compatible |

### 6.8 Statut temporel

Un **Statut temporel** qualifie l'Ã©tat d'un Ã©vÃ©nement dans le temps.

| Statut | Description |
|--------|-------------|
| **Informatif** | L'Ã©vÃ©nement est une information temporelle sans engagement (ex. date publique d'un festival) |
| **Bloquant** | L'Ã©vÃ©nement reprÃ©sente un engagement ferme dans le temps (ex. rendez-vous confirmÃ©) |
| **AnnulÃ©** | L'Ã©vÃ©nement a Ã©tÃ© annulÃ© par sa source ou par l'utilisateur |
| **ModifiÃ©** | L'Ã©vÃ©nement a subi une modification temporelle (changement de date, d'heure ou de durÃ©e) |

Le statut est dÃ©clarÃ© par le Service source pour les Ã©vÃ©nements synchronisÃ©s. Pour les Ã©vÃ©nements internes, il est gÃ©rÃ© par JayKoa.

### 6.9 Conflit temporel

Un **Conflit temporel** est un chevauchement dÃ©tectÃ© entre deux ou plusieurs Ã©vÃ©nements dans l'agenda d'un utilisateur.

- JayKoa dÃ©tecte les conflits au sens de la **visualisation** : il signale le chevauchement
- JayKoa ne rÃ©sout jamais un conflit de booking â€” cette responsabilitÃ© appartient au Service mÃ©tier concernÃ©
- JayKoa ne bloque jamais une action utilisateur en raison d'un conflit â€” il informe
- Le conflit est un Ã©tat visuel et informatif, pas un Ã©tat dÃ©cisionnel

> **JayKoa signale les conflits temporels. Il ne les rÃ©sout jamais.**

---

## 7. Persistance & DonnÃ©es

### AutoritÃ© de persistance

**KindMother** est l'autoritÃ© exclusive de persistance pour JayKoa.

Toute Ã©criture de donnÃ©es passe par des **Intentions d'Ã‰criture (WriteIntent)** soumises Ã  KindMother. KindMother dÃ©cide de la validation, du refus ou du report de chaque Ã©criture.

### RÃ¨gles de persistance

| RÃ¨gle | Description |
|-------|-------------|
| **KindMother exclusif** | Aucun accÃ¨s direct au stockage. Toute opÃ©ration de donnÃ©es passe par KindMother |
| **WriteIntent obligatoire** | Toute Ã©criture est une intention soumise, jamais une Ã©criture directe |
| **DB MÃ¨re / DB Fille** | Support du modÃ¨le KindMother de rÃ©plication (Instance MÃ¨re souveraine, Instances Filles synchronisÃ©es) |
| **Offline-first** | L'Ã©tat local est souverain (LOI-3). JayKoa fonctionne sans rÃ©seau |
| **Synchronisation diffÃ©rÃ©e** | La synchronisation avec d'autres instances se fait de maniÃ¨re asynchrone et gouvernÃ©e |
| **Aucune Ã©criture directe** | Aucun composant de JayKoa n'Ã©crit directement dans un systÃ¨me de stockage |

### Nature des donnÃ©es persistÃ©es

JayKoa ne stocke jamais de donnÃ©es mÃ©tier appartenant Ã  un autre Service.

Il stocke uniquement des **reprÃ©sentations temporelles** :

- Plages temporelles (dÃ©but, fin, fuseau)
- Types et statuts d'Ã©vÃ©nements
- RÃ©fÃ©rences opaques vers les Services sources (identifiants, pas de contenu mÃ©tier)
- RÃ¨gles de rÃ©currence
- PrÃ©fÃ©rences d'agenda et de filtrage
- RÃ©fÃ©rences de participants

Les donnÃ©es mÃ©tier complÃ¨tes (dÃ©tail d'un rendez-vous, contenu d'une candidature, programme d'un festival) restent sous la responsabilitÃ© exclusive de leur Service source et de leur COG de rÃ©fÃ©rence.

---

## 8. SÃ©curitÃ© & Gouvernance

### Gouvernance par les Cores

JayKoa est gouvernÃ© par les Cores Miyukini. Chaque Core exerce son autoritÃ© exclusive dans son domaine :

| Core | RÃ´le vis-Ã -vis de JayKoa |
|------|--------------------------|
| **StrongFather** | DÃ©cide si une action devrait Ãªtre faite. Ã‰met les Mandats de Permission pour la synchronisation inter-Services et l'accÃ¨s aux agendas. Ne fait jamais d'exÃ©cution |
| **KindMother** | AutoritÃ© absolue de persistance. Valide, refuse ou reporte les WriteIntent. GÃ¨re la rÃ©plication DB MÃ¨re / DB Fille |
| **Master Butler** | Registre des capacitÃ©s et permissions. DÃ©clare les capacitÃ©s de JayKoa, dÃ©finit les permissions d'accÃ¨s aux agendas et Ã©vÃ©nements |
| **WorrySentinel** | Gouverne les niveaux de sÃ©curitÃ© et les Ã©tats de confiance. Peut restreindre les capacitÃ©s en cas d'Ã©tat dÃ©gradÃ© |
| **Caring Nanny** | Observe l'Ã©tat du systÃ¨me. Peut bloquer les Outils si l'environnement est dÃ©gradÃ© |
| **Border Guard** | DÃ©finit les frontiÃ¨res de JayKoa pour les interactions inter-COG |
| **Ever Buddy** | Gouverne l'Ã©volution du Service dans le temps (versions, dÃ©prÃ©ciation, compatibilitÃ©) |
| **TAMR** | DÃ©finit les points d'intervention humaine dans le Service |

### Permissions conceptuelles

Les permissions de JayKoa sont conceptuelles et gouvernÃ©es par Master Butler, sous dÃ©cision de StrongFather :

| Permission | Description |
|------------|-------------|
| **Voir un agenda** | Qui peut consulter un agenda donnÃ© (personnel, partagÃ©) |
| **Voir certains Ã©vÃ©nements** | Qui peut voir les Ã©vÃ©nements d'un agenda, selon leur source, leur type ou leur niveau de sÃ©curitÃ© |
| **CrÃ©er un Ã©vÃ©nement interne** | Qui peut ajouter un Ã©vÃ©nement dans un agenda |
| **S'abonner Ã  un flux temporel** | Qui peut activer la synchronisation avec un Service source |
| **Exporter un agenda** | Qui peut gÃ©nÃ©rer un export des Ã©vÃ©nements d'un agenda |

### Mandats de Permission

Toute synchronisation inter-Services est encadrÃ©e par un **Mandat de Permission** Ã©mis par StrongFather.

- Un Mandat autorise JayKoa Ã  consommer les capacitÃ©s temporelles d'un Service source
- Un Mandat dÃ©finit les flux autorisÃ©s, les types de donnÃ©es, les niveaux de sÃ©curitÃ©
- Un Mandat est temporaire, rÃ©vocable et auditable
- Un Mandat n'est pas une optimisation â€” c'est un acte de gouvernance dÃ©lÃ©guÃ©

### RÃ¨gles de sÃ©curitÃ©

| RÃ¨gle | Description |
|-------|-------------|
| **Aucun mÃ©canisme d'authentification propre** | JayKoa ne gÃ¨re pas l'authentification. Celle-ci est assurÃ©e par les mÃ©canismes du COG |
| **Aucun bypass de permission** | Il est impossible de contourner les permissions dÃ©finies par Master Butler et StrongFather |
| **Niveaux de sÃ©curitÃ© WorrySentinel** | Les donnÃ©es temporelles sont classÃ©es par niveau de sensibilitÃ© (0 Ã  4). Les rÃ¨gles de visibilitÃ© et d'export respectent ces niveaux |
| **Restriction en Ã©tat dÃ©gradÃ©** | En Ã©tat de confiance dÃ©gradÃ© (T2â€“T4), les capacitÃ©s de synchronisation et d'export peuvent Ãªtre restreintes par WorrySentinel et Caring Nanny |
| **Aucune donnÃ©e personnelle exposÃ©e hors Mandat** | Les exports et vues agrÃ©gÃ©es ne contiennent jamais de donnÃ©es au-delÃ  du niveau autorisÃ© |

---

## 9. CapacitÃ©s exposÃ©es (Service Capabilities)

JayKoa expose les capacitÃ©s suivantes en tant que Service du COG :

| CapacitÃ© | Description |
|----------|-------------|
| **CrÃ©er et gÃ©rer des agendas** | CrÃ©ation d'agendas personnels et partagÃ©s, paramÃ©trage de visibilitÃ© et de prÃ©fÃ©rences |
| **Ajouter des Ã©vÃ©nements internes** | CrÃ©ation d'Ã©vÃ©nements directement dans JayKoa par l'utilisateur |
| **Synchroniser des Ã©vÃ©nements issus d'autres Services** | RÃ©ception et traduction d'informations temporelles provenant de Services du COG autorisÃ©s |
| **S'abonner Ã  des flux temporels inter-Services** | Activation de la synchronisation avec un Service source sous Mandat de Permission |
| **Filtrer les Ã©vÃ©nements synchronisÃ©s** | Filtrage par source, type, statut, pÃ©riode, visibilitÃ© |
| **Identifier l'origine de chaque Ã©vÃ©nement** | Chaque Ã©vÃ©nement synchronisÃ© conserve la rÃ©fÃ©rence immuable Ã  son Service source |
| **Afficher les engagements issus de JayRDV** | Rendez-vous confirmÃ©s, crÃ©neaux bloquÃ©s, modifications et annulations â€” en lecture rÃ©flÃ©chie |
| **Afficher les Ã©vÃ©nements issus de JayFestival** | Dates de festival, dates d'inscription, deadlines, Ã©vÃ©nements favoris â€” en lecture rÃ©flÃ©chie |
| **Marquer les Ã©vÃ©nements comme informatifs ou bloquants** | Qualification du statut temporel de chaque Ã©vÃ©nement |

Ces capacitÃ©s sont dÃ©clarÃ©es par Master Butler et soumises aux permissions et Mandats de Permission dÃ©finis par StrongFather.

---

## 10. Flux conceptuels

### 10.1 Synchronisation inter-Services (flux gÃ©nÃ©ral)

La synchronisation entre JayKoa et un Service du COG suit le flux conceptuel suivant :

| Ã‰tape | Description |
|-------|-------------|
| **1** | Un Service du COG expose des capacitÃ©s temporelles (plages, Ã©vÃ©nements, engagements) |
| **2** | Un Mandat de Permission autorise JayKoa Ã  consommer ces capacitÃ©s |
| **3** | JayKoa reÃ§oit les informations temporelles du Service source |
| **4** | JayKoa traduit ces informations en Ã©vÃ©nements temporels dans son propre modÃ¨le |
| **5** | Chaque Ã©vÃ©nement conserve la rÃ©fÃ©rence immuable Ã  son Service source |
| **6** | Les mises Ã  jour du Service source sont reflÃ©tÃ©es par synchronisation gouvernÃ©e |
| **7** | L'utilisateur visualise une timeline consolidÃ©e dans ses agendas |

**Invariant** : JayKoa n'initie jamais une action mÃ©tier dans le Service source. Le flux est unidirectionnel en lecture.

### 10.2 Synchronisation avec JayFestival

JayFestival (Service d'organisation d'Ã©vÃ©nements et de festivals) expose des capacitÃ©s temporelles que JayKoa peut synchroniser :

| DonnÃ©e temporelle | Nature | Statut typique |
|-------------------|--------|----------------|
| **Dates de festival** | Plage temporelle d'une Ã©dition (dÃ©but, fin) | Informatif |
| **Dates d'inscription** | PÃ©riodes d'ouverture des inscriptions exposants ou visiteurs | Informatif |
| **Deadlines** | Dates limites de candidature, de validation, de paiement | Bloquant |
| **Ã‰vÃ©nements favoris** | Ã‰vÃ©nements marquÃ©s par l'utilisateur dans JayFestival | Informatif |
| **Participations confirmÃ©es** | Inscription validÃ©e Ã  un festival, un atelier, un crÃ©neau | Bloquant |

JayKoa reflÃ¨te ces donnÃ©es dans l'agenda de l'utilisateur. Si une date de festival est modifiÃ©e dans JayFestival, JayKoa reflÃ¨te la modification. Si une inscription est annulÃ©e, JayKoa reflÃ¨te l'annulation.

JayKoa ne crÃ©e pas de candidature, ne valide pas d'inscription, ne gÃ¨re pas de plan de salle. Ces responsabilitÃ©s appartiennent exclusivement Ã  JayFestival.

### 10.3 Synchronisation avec JayRDV

JayRDV (Service de booking et de prise de rendez-vous) expose des capacitÃ©s temporelles que JayKoa peut synchroniser :

| DonnÃ©e temporelle | Nature | Statut typique |
|-------------------|--------|----------------|
| **Rendez-vous confirmÃ©s** | Plage temporelle d'un rendez-vous validÃ© | Bloquant |
| **CrÃ©neaux bloquÃ©s** | Plages rÃ©servÃ©es par un professionnel ou un client | Bloquant |
| **Modifications** | Changements de date, d'heure ou de durÃ©e d'un rendez-vous | ModifiÃ© |
| **Annulations** | Rendez-vous annulÃ©s par le professionnel ou le client | AnnulÃ© |

JayKoa reflÃ¨te ces donnÃ©es dans l'agenda de l'utilisateur. Les engagements issus de JayRDV apparaissent en **lecture seule** dans JayKoa â€” l'utilisateur ne peut pas modifier un rendez-vous depuis JayKoa, il doit passer par JayRDV.

JayKoa ne rÃ©serve pas de crÃ©neau, ne calcule pas de disponibilitÃ©, ne gÃ¨re pas de confirmation. Ces responsabilitÃ©s appartiennent exclusivement Ã  JayRDV.

---

## 11. Contraintes et invariants

Les contraintes suivantes sont **non nÃ©gociables** et s'appliquent Ã  toute implÃ©mentation, toute Ã©volution et toute extension de JayKoa.

| # | Contrainte | Description |
|---|------------|-------------|
| **C-1** | JayKoa ne modifie jamais les donnÃ©es mÃ©tier des Services sources | Les donnÃ©es mÃ©tier restent sous l'autoritÃ© exclusive du Service source |
| **C-2** | JayKoa ne crÃ©e pas d'Ã©vÃ©nement mÃ©tier externe | JayKoa ne peut pas crÃ©er de rendez-vous dans JayRDV, ni d'inscription dans JayFestival |
| **C-3** | JayKoa ne rÃ©serve jamais de crÃ©neau | La rÃ©servation est une responsabilitÃ© mÃ©tier qui appartient au Service de booking |
| **C-4** | JayKoa ne calcule aucune disponibilitÃ© | Le calcul de disponibilitÃ© est une logique mÃ©tier qui ne relÃ¨ve pas de l'agenda |
| **C-5** | JayKoa ne rÃ©sout aucun conflit de booking | La rÃ©solution de conflits de rÃ©servation appartient au Service mÃ©tier concernÃ© |
| **C-6** | Toute synchronisation est gouvernÃ©e | Un Mandat de Permission est requis pour toute synchronisation inter-Services |
| **C-7** | Toute Ã©criture passe par KindMother | Aucune Ã©criture directe dans un systÃ¨me de stockage |
| **C-8** | Aucun couplage direct entre Services | JayKoa ne dÃ©pend techniquement d'aucun Service source pour fonctionner |
| **C-9** | Offline-first | JayKoa fonctionne sans rÃ©seau (LOI-1, LOI-2, LOI-3) |
| **C-10** | Pas de temps global requis | JayKoa utilise les rÃ©fÃ©rences temporelles locales (LOI-4) |
| **C-11** | JayKoa ne stocke que des reprÃ©sentations temporelles | Aucune donnÃ©e mÃ©tier d'un autre Service n'est persistÃ©e par JayKoa |
| **C-12** | La source d'un Ã©vÃ©nement synchronisÃ© est immuable | Une fois crÃ©Ã©, l'Ã©vÃ©nement conserve sa rÃ©fÃ©rence source sans modification possible |

---

## 12. RÃ©sumÃ© exÃ©cutif

JayKoa est le **calendrier universel du COG**.

Il ne possÃ¨de aucune logique mÃ©tier. Il ne prend aucune dÃ©cision. Il ne rÃ©serve rien, n'inscrit personne, ne valide aucun engagement.

JayKoa **reflÃ¨te le temps de tous les Services**.

> **JayFestival annonce le temps.**
> **JayRDV dÃ©cide du temps.**
> **JayKoa le rend visible, lisible et orchestrÃ©.**

JayKoa n'appartient Ã  aucun Service. Il reflÃ¨te le temps de tous.

Dans l'Ã©cosystÃ¨me Miyukini, JayKoa est le Service qui transforme des fragments temporels dispersÃ©s en une **vision cohÃ©rente et unifiÃ©e du temps**, au bÃ©nÃ©fice de l'utilisateur, dans le respect de la souverainetÃ© de chaque Service et de la gouvernance des Cores.

---

## 13. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie officielle (Service, OpÃ©rateur, Mandat de Permission, COG, Cores, WriteIntent, etc.) |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Besoins en Ã©crans et UI pour les services consommateurs |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs et parcours cÃ´tÃ© service |
| [JayKoa - Bornage Implementation](./JayKoa%20-%20Bornage%20Implementation.md) | Bornage pour l'implÃ©mentation (MVP, phases, hors scope) |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | OpÃ©rateurs, Kits d'Outils, Ã‰quipe, filtres supportÃ©s |
| [JayKoa - Niveaux Securite et Protection Donnees](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Niveaux de sÃ©curitÃ© WorrySentinel et mesures de protection |
| [JayKoa - Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | SchÃ©mas d'intÃ©gration avec JayRDV, JayFestival, futurs Services |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | RÃ©fÃ©rentiel fonctionnel inspirÃ© de Google Agenda |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service de booking et prise de rendez-vous |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service d'organisation d'Ã©vÃ©nements et de festivals |
| [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//miyukini-webway-system//reference//_index.md) | RÃ©sidence centralisÃ©e, COG de rÃ©fÃ©rence, niveaux 2+ |

---

**Document** : JayKoa â€” Document Fondateur
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document fondateur normatif â€” rÃ©fÃ©rence pour le Service

