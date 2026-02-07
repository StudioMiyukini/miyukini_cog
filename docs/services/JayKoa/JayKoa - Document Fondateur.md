# JayKoa — Document Fondateur

## Contexte

**JayKoa** est un **Service d'Agenda** au sein de l'écosystème Miyukini COG (Core-Orchestrated Governance Environment).

Son objectif est de fournir une capacité complète de gestion du temps : agendas, événements, engagements, rendez-vous et planification, au bénéfice des utilisateurs du COG.

JayKoa est conçu comme un **Service transversal du COG**, spécialisé dans le domaine temporel :

- Multi-agenda
- Multi-utilisateur
- Offline-first
- Synchronisable
- Gouverné par les Cores Miyukini

JayKoa n'est pas une application.
C'est un **Service**, rendu par un ou plusieurs **Opérateurs**, conformément au modèle COG.

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

Ce document est le **document fondateur** du Service JayKoa : il en fixe la raison d'être, le positionnement architectural, les règles de gouvernance, le modèle conceptuel des données et les interactions avec les autres Services du COG. Il s'adresse aux équipes produit, architecture, sécurité et à toute partie prenante de l'écosystème Miyukini.

## Portée / Scope

- **Applicable à :** Architecture JayKoa, interactions inter-Services, modèle de données conceptuel, gouvernance
- **Audience :** Architectes, équipes produit, équipes sécurité, parties prenantes
- **Statut :** Document fondateur normatif — référence pour le Service

### Hors périmètre

- Aucun code
- Aucune implémentation technique
- Aucun choix UI
- Aucun framework
- Aucun endpoint ou API
- Aucune spécification de schéma de base de données

---

## 1. Contexte général

JayKoa est un Service d'Agenda comparable, sur le plan fonctionnel, à Google Agenda.

Il fournit une capacité complète de gestion du temps au sein de l'écosystème Miyukini :

| Capacité | Description |
|----------|-------------|
| **Agendas** | Création et gestion d'agendas personnels et partagés |
| **Événements** | Gestion d'événements temporels de toute nature |
| **Engagements** | Lecture réfléchie des engagements et rendez-vous issus d'autres Services |
| **Planification** | Vision temporelle unifiée et consolidée |
| **Synchronisation** | Synchronisation d'événements issus d'autres Services du COG |

JayKoa respecte les Lois d'Autonomie Miyukini :

- **LOI-1** — Aucune dépendance externe critique à l'exécution
- **LOI-2** — Le système accepte l'isolement comme état normal
- **LOI-3** — L'état local est souverain
- **LOI-4** — Pas de temps global requis
- **LOI-5** — Le coût doit être proportionnel au hardware

JayKoa fonctionne en mode **offline-first**. Il est capable d'opérer en isolation complète, de stocker localement les données temporelles, et de synchroniser de manière différée lorsque les conditions le permettent.

---

## 2. Intérêt fondamental du Service

L'un des intérêts fondamentaux de JayKoa est sa capacité à se synchroniser nativement avec **tous les Services du COG exposant des capacités temporelles**.

JayKoa agit comme un **réceptacle temporel transversal** :

- Il reçoit des informations temporelles
- Il les traduit dans un modèle d'agenda
- Il les rend lisibles et exploitables dans le temps

Sans jamais devenir dépendant des Services sources.

### Principe fondateur

> **JayKoa ne crée pas les événements métier. Il reflète, agrège et orchestre le temps issu des autres Services.**

### Synchronisation inter-Services

JayKoa est conçu pour se synchroniser notamment avec :

| Service | Domaine temporel | Nature de la synchronisation |
|---------|-----------------|-------------------------------|
| **JayFestival** | Organisation d'événements (festivals, concours, inscriptions, deadlines, dates clés) | Dates de festival, dates d'inscription, deadlines, événements favoris |
| **JayRDV** | Booking et prise de rendez-vous (disponibilités, créneaux, confirmations, annulations) | Rendez-vous confirmés, créneaux bloqués, modifications et annulations |

JayKoa peut synchroniser ces informations dans les agendas des utilisateurs en fonction :

- De leurs **préférences**
- De leurs **abonnements**
- De leurs **événements favoris**
- De leurs **engagements actifs**

### Extensibilité

Tout Service du COG exposant des données temporelles compatibles peut devenir une source de synchronisation pour JayKoa :

- Services événementiels
- Services de booking
- Services organisationnels
- Services de planification
- Tout Service produisant des plages temporelles qualifiées

La synchronisation inter-Services est :

- **Déclarative** — chaque source déclare ses capacités temporelles
- **Gouvernée** — encadrée par les Cores et les Mandats de Permission
- **Non intrusive** — aucune modification des données source
- **Sans dépendance directe** — aucun couplage technique entre Services

---

## 3. Objectif de la documentation

Ce document a pour objectif de produire une référence :

- **Normative** — il fixe les règles et les limites du Service
- **Structurante** — il définit le positionnement architectural de JayKoa dans le COG
- **Stable dans le temps** — il ne dépend d'aucun choix d'implémentation

### Ce document explique

- Ce que fait JayKoa
- Ce que JayKoa ne fait pas
- Comment JayKoa s'insère dans l'architecture Miyukini
- Comment JayKoa interagit avec les autres Services du COG
- Comment JayKoa utilise KindMother pour la persistance
- Quelles sont les règles de sécurité et de gouvernance de JayKoa

### Ce document ne contient pas

- Aucun code
- Aucune implémentation technique
- Aucun choix UI ou UX
- Aucun framework ou librairie
- Aucun endpoint, API ou schéma de données technique

---

## 4. Portée / Scope

### JayKoa FAIT

- Gérer des agendas personnels et partagés
- Gérer des événements temporels
- Gérer des engagements et rendez-vous en lecture réfléchie
- Synchroniser des événements issus d'autres Services du COG
- Consolider des informations temporelles multi-sources
- Offrir une vision temporelle unifiée
- S'abonner à des flux temporels inter-Services
- Filtrer les événements synchronisés
- Identifier l'origine de chaque événement
- Marquer les événements comme informatifs ou bloquants

### JayKoa NE FAIT PAS

- Aucune logique métier événementielle
- Aucune logique de réservation
- Aucune gestion de disponibilité
- Aucune notification technique
- Aucune authentification
- Aucune prise de décision métier
- Aucune modification des données des Services sources
- Aucune création d'événement métier dans un autre Service
- Aucune résolution de conflit de booking
- Aucun calcul de disponibilité

---

## 5. Rôle architectural

### Positionnement dans la Pyramide Miyukini

JayKoa est un **Service transversal du COG**, spécialisé dans le domaine du temps.

Dans la Pyramide Miyukini, JayKoa se situe au niveau des **Services** (Strate 7 — Opérateurs), rendus par un ou plusieurs Opérateurs gouvernés qui s'appuient sur les Outils et Kits d'Outils (Strate 6), eux-mêmes gouvernés par les Cores (Strate 4).

| Strate | Élément | Rôle vis-à-vis de JayKoa |
|--------|---------|--------------------------|
| **7** | Opérateurs JayKoa | Exécutent les rôles du Service pour le compte de l'utilisateur |
| **6** | Outils & Kits d'Outils | Capacités exécutables utilisées par les Opérateurs |
| **5** | BondingBrother | Médiation entre les Opérateurs et les Cores |
| **4** | Cores | Gouvernent le comportement de JayKoa (StrongFather, KindMother, WorrySentinel, Master Butler, etc.) |

### Transversalité

JayKoa n'appartient à aucun Service métier. Il est **transversal** :

- Il peut consommer des capacités temporelles provenant de n'importe quel Service du COG
- Il ne possède pas les données métier des Services sources
- Il ne duplique pas la logique métier des Services sources
- Il traduit les informations temporelles dans son propre modèle d'agenda

### Règles de couplage inter-Services

JayKoa n'initie jamais une action métier dans un autre Service. Il ne fait que **refléter l'état temporel validé** par le Service source.

| Règle | Description |
|-------|-------------|
| **Reflet, pas action** | JayKoa reflète le temps, il ne le décide pas |
| **Lecture, pas écriture** | JayKoa ne modifie jamais les données d'un Service source |
| **Traduction, pas copie** | JayKoa traduit les informations temporelles dans son modèle, il ne copie pas les données métier |
| **Déclaratif, pas impératif** | La synchronisation est déclarée, jamais imposée |
| **Gouverné, pas libre** | Toute synchronisation passe par un Mandat de Permission |

---

## 6. Modèle conceptuel des données

Ce modèle décrit les notions fondamentales manipulées par JayKoa. Il s'agit d'un modèle **conceptuel**, sans schéma technique, sans structure de base de données, sans format d'échange.

### 6.1 Agenda

Un **Agenda** est un conteneur temporel appartenant à un utilisateur ou partagé entre plusieurs utilisateurs.

- Un utilisateur peut posséder plusieurs agendas
- Un agenda peut être personnel ou partagé
- Un agenda peut recevoir des événements internes (créés par l'utilisateur) et des événements synchronisés (issus d'autres Services)
- Un agenda possède une identité propre et des règles de visibilité

### 6.2 Événement

Un **Événement** est une occurrence temporelle placée dans un agenda.

- Un événement possède une plage temporelle (début, fin)
- Un événement possède un type (informatif, bloquant, annulé, modifié)
- Un événement peut être interne (créé dans JayKoa) ou synchronisé (issu d'un Service source)
- Un événement synchronisé conserve la référence à son Service source
- Un événement ne contient jamais de données métier appartenant au Service source — uniquement une représentation temporelle

### 6.3 Engagement temporel

Un **Engagement temporel** est un événement issu d'un Service externe qui représente un engagement de l'utilisateur dans le temps.

- Rendez-vous confirmé (issu de JayRDV)
- Participation à un festival (issue de JayFestival)
- Inscription à un atelier, une formation, un créneau

Un engagement temporel est toujours en **lecture réfléchie** dans JayKoa : il est visible, mais JayKoa ne peut ni le modifier ni le supprimer. Seul le Service source détient l'autorité sur l'engagement.

### 6.4 Occurrence

Une **Occurrence** est une instance unique d'un événement dans le temps.

- Un événement ponctuel produit une seule occurrence
- Un événement récurrent produit plusieurs occurrences selon sa règle de récurrence
- Chaque occurrence peut être individuellement modifiée ou annulée (pour les événements internes)
- Les occurrences d'événements synchronisés reflètent l'état du Service source

### 6.5 Règle de récurrence

Une **Règle de récurrence** définit le schéma de répétition d'un événement dans le temps.

- Fréquence (quotidienne, hebdomadaire, mensuelle, annuelle, personnalisée)
- Intervalle
- Jours de la semaine, du mois, de l'année
- Date de fin ou nombre d'occurrences
- Exceptions (dates exclues)

Les règles de récurrence ne s'appliquent qu'aux événements **internes** de JayKoa. Les événements synchronisés arrivent comme des occurrences individuelles provenant du Service source.

### 6.6 Participant

Un **Participant** est un utilisateur associé à un événement.

- Propriétaire de l'événement
- Invité
- Observateur

Les participants ne sont identifiés que par des références. JayKoa ne stocke pas de données personnelles au-delà de ce qui est nécessaire à la représentation temporelle.

### 6.7 Source d'événement

Une **Source d'événement** identifie le Service du COG à l'origine d'un événement synchronisé.

- Chaque événement synchronisé est marqué de sa source
- La source est immuable une fois l'événement créé
- JayKoa distingue toujours un événement interne d'un événement synchronisé
- L'utilisateur peut filtrer sa vue par source

| Source | Exemples d'événements |
|--------|----------------------|
| **JayKoa** (interne) | Événement créé par l'utilisateur directement dans son agenda |
| **JayFestival** | Date de festival, deadline d'inscription, événement favori |
| **JayRDV** | Rendez-vous confirmé, créneau bloqué, modification, annulation |
| **Autre Service COG** | Toute plage temporelle exposée par un Service compatible |

### 6.8 Statut temporel

Un **Statut temporel** qualifie l'état d'un événement dans le temps.

| Statut | Description |
|--------|-------------|
| **Informatif** | L'événement est une information temporelle sans engagement (ex. date publique d'un festival) |
| **Bloquant** | L'événement représente un engagement ferme dans le temps (ex. rendez-vous confirmé) |
| **Annulé** | L'événement a été annulé par sa source ou par l'utilisateur |
| **Modifié** | L'événement a subi une modification temporelle (changement de date, d'heure ou de durée) |

Le statut est déclaré par le Service source pour les événements synchronisés. Pour les événements internes, il est géré par JayKoa.

### 6.9 Conflit temporel

Un **Conflit temporel** est un chevauchement détecté entre deux ou plusieurs événements dans l'agenda d'un utilisateur.

- JayKoa détecte les conflits au sens de la **visualisation** : il signale le chevauchement
- JayKoa ne résout jamais un conflit de booking — cette responsabilité appartient au Service métier concerné
- JayKoa ne bloque jamais une action utilisateur en raison d'un conflit — il informe
- Le conflit est un état visuel et informatif, pas un état décisionnel

> **JayKoa signale les conflits temporels. Il ne les résout jamais.**

---

## 7. Persistance & Données

### Autorité de persistance

**KindMother** est l'autorité exclusive de persistance pour JayKoa.

Toute écriture de données passe par des **Intentions d'Écriture (WriteIntent)** soumises à KindMother. KindMother décide de la validation, du refus ou du report de chaque écriture.

### Règles de persistance

| Règle | Description |
|-------|-------------|
| **KindMother exclusif** | Aucun accès direct au stockage. Toute opération de données passe par KindMother |
| **WriteIntent obligatoire** | Toute écriture est une intention soumise, jamais une écriture directe |
| **DB Mère / DB Fille** | Support du modèle KindMother de réplication (Instance Mère souveraine, Instances Filles synchronisées) |
| **Offline-first** | L'état local est souverain (LOI-3). JayKoa fonctionne sans réseau |
| **Synchronisation différée** | La synchronisation avec d'autres instances se fait de manière asynchrone et gouvernée |
| **Aucune écriture directe** | Aucun composant de JayKoa n'écrit directement dans un système de stockage |

### Nature des données persistées

JayKoa ne stocke jamais de données métier appartenant à un autre Service.

Il stocke uniquement des **représentations temporelles** :

- Plages temporelles (début, fin, fuseau)
- Types et statuts d'événements
- Références opaques vers les Services sources (identifiants, pas de contenu métier)
- Règles de récurrence
- Préférences d'agenda et de filtrage
- Références de participants

Les données métier complètes (détail d'un rendez-vous, contenu d'une candidature, programme d'un festival) restent sous la responsabilité exclusive de leur Service source et de leur COG de référence.

---

## 8. Sécurité & Gouvernance

### Gouvernance par les Cores

JayKoa est gouverné par les Cores Miyukini. Chaque Core exerce son autorité exclusive dans son domaine :

| Core | Rôle vis-à-vis de JayKoa |
|------|--------------------------|
| **StrongFather** | Décide si une action devrait être faite. Émet les Mandats de Permission pour la synchronisation inter-Services et l'accès aux agendas. Ne fait jamais d'exécution |
| **KindMother** | Autorité absolue de persistance. Valide, refuse ou reporte les WriteIntent. Gère la réplication DB Mère / DB Fille |
| **Master Butler** | Registre des capacités et permissions. Déclare les capacités de JayKoa, définit les permissions d'accès aux agendas et événements |
| **WorrySentinel** | Gouverne les niveaux de sécurité et les états de confiance. Peut restreindre les capacités en cas d'état dégradé |
| **Caring Nanny** | Observe l'état du système. Peut bloquer les Outils si l'environnement est dégradé |
| **Border Guard** | Définit les frontières de JayKoa pour les interactions inter-COG |
| **Ever Buddy** | Gouverne l'évolution du Service dans le temps (versions, dépréciation, compatibilité) |
| **TAMR** | Définit les points d'intervention humaine dans le Service |

### Permissions conceptuelles

Les permissions de JayKoa sont conceptuelles et gouvernées par Master Butler, sous décision de StrongFather :

| Permission | Description |
|------------|-------------|
| **Voir un agenda** | Qui peut consulter un agenda donné (personnel, partagé) |
| **Voir certains événements** | Qui peut voir les événements d'un agenda, selon leur source, leur type ou leur niveau de sécurité |
| **Créer un événement interne** | Qui peut ajouter un événement dans un agenda |
| **S'abonner à un flux temporel** | Qui peut activer la synchronisation avec un Service source |
| **Exporter un agenda** | Qui peut générer un export des événements d'un agenda |

### Mandats de Permission

Toute synchronisation inter-Services est encadrée par un **Mandat de Permission** émis par StrongFather.

- Un Mandat autorise JayKoa à consommer les capacités temporelles d'un Service source
- Un Mandat définit les flux autorisés, les types de données, les niveaux de sécurité
- Un Mandat est temporaire, révocable et auditable
- Un Mandat n'est pas une optimisation — c'est un acte de gouvernance délégué

### Règles de sécurité

| Règle | Description |
|-------|-------------|
| **Aucun mécanisme d'authentification propre** | JayKoa ne gère pas l'authentification. Celle-ci est assurée par les mécanismes du COG |
| **Aucun bypass de permission** | Il est impossible de contourner les permissions définies par Master Butler et StrongFather |
| **Niveaux de sécurité WorrySentinel** | Les données temporelles sont classées par niveau de sensibilité (0 à 4). Les règles de visibilité et d'export respectent ces niveaux |
| **Restriction en état dégradé** | En état de confiance dégradé (T2–T4), les capacités de synchronisation et d'export peuvent être restreintes par WorrySentinel et Caring Nanny |
| **Aucune donnée personnelle exposée hors Mandat** | Les exports et vues agrégées ne contiennent jamais de données au-delà du niveau autorisé |

---

## 9. Capacités exposées (Service Capabilities)

JayKoa expose les capacités suivantes en tant que Service du COG :

| Capacité | Description |
|----------|-------------|
| **Créer et gérer des agendas** | Création d'agendas personnels et partagés, paramétrage de visibilité et de préférences |
| **Ajouter des événements internes** | Création d'événements directement dans JayKoa par l'utilisateur |
| **Synchroniser des événements issus d'autres Services** | Réception et traduction d'informations temporelles provenant de Services du COG autorisés |
| **S'abonner à des flux temporels inter-Services** | Activation de la synchronisation avec un Service source sous Mandat de Permission |
| **Filtrer les événements synchronisés** | Filtrage par source, type, statut, période, visibilité |
| **Identifier l'origine de chaque événement** | Chaque événement synchronisé conserve la référence immuable à son Service source |
| **Afficher les engagements issus de JayRDV** | Rendez-vous confirmés, créneaux bloqués, modifications et annulations — en lecture réfléchie |
| **Afficher les événements issus de JayFestival** | Dates de festival, dates d'inscription, deadlines, événements favoris — en lecture réfléchie |
| **Marquer les événements comme informatifs ou bloquants** | Qualification du statut temporel de chaque événement |

Ces capacités sont déclarées par Master Butler et soumises aux permissions et Mandats de Permission définis par StrongFather.

---

## 10. Flux conceptuels

### 10.1 Synchronisation inter-Services (flux général)

La synchronisation entre JayKoa et un Service du COG suit le flux conceptuel suivant :

| Étape | Description |
|-------|-------------|
| **1** | Un Service du COG expose des capacités temporelles (plages, événements, engagements) |
| **2** | Un Mandat de Permission autorise JayKoa à consommer ces capacités |
| **3** | JayKoa reçoit les informations temporelles du Service source |
| **4** | JayKoa traduit ces informations en événements temporels dans son propre modèle |
| **5** | Chaque événement conserve la référence immuable à son Service source |
| **6** | Les mises à jour du Service source sont reflétées par synchronisation gouvernée |
| **7** | L'utilisateur visualise une timeline consolidée dans ses agendas |

**Invariant** : JayKoa n'initie jamais une action métier dans le Service source. Le flux est unidirectionnel en lecture.

### 10.2 Synchronisation avec JayFestival

JayFestival (Service d'organisation d'événements et de festivals) expose des capacités temporelles que JayKoa peut synchroniser :

| Donnée temporelle | Nature | Statut typique |
|-------------------|--------|----------------|
| **Dates de festival** | Plage temporelle d'une édition (début, fin) | Informatif |
| **Dates d'inscription** | Périodes d'ouverture des inscriptions exposants ou visiteurs | Informatif |
| **Deadlines** | Dates limites de candidature, de validation, de paiement | Bloquant |
| **Événements favoris** | Événements marqués par l'utilisateur dans JayFestival | Informatif |
| **Participations confirmées** | Inscription validée à un festival, un atelier, un créneau | Bloquant |

JayKoa reflète ces données dans l'agenda de l'utilisateur. Si une date de festival est modifiée dans JayFestival, JayKoa reflète la modification. Si une inscription est annulée, JayKoa reflète l'annulation.

JayKoa ne crée pas de candidature, ne valide pas d'inscription, ne gère pas de plan de salle. Ces responsabilités appartiennent exclusivement à JayFestival.

### 10.3 Synchronisation avec JayRDV

JayRDV (Service de booking et de prise de rendez-vous) expose des capacités temporelles que JayKoa peut synchroniser :

| Donnée temporelle | Nature | Statut typique |
|-------------------|--------|----------------|
| **Rendez-vous confirmés** | Plage temporelle d'un rendez-vous validé | Bloquant |
| **Créneaux bloqués** | Plages réservées par un professionnel ou un client | Bloquant |
| **Modifications** | Changements de date, d'heure ou de durée d'un rendez-vous | Modifié |
| **Annulations** | Rendez-vous annulés par le professionnel ou le client | Annulé |

JayKoa reflète ces données dans l'agenda de l'utilisateur. Les engagements issus de JayRDV apparaissent en **lecture seule** dans JayKoa — l'utilisateur ne peut pas modifier un rendez-vous depuis JayKoa, il doit passer par JayRDV.

JayKoa ne réserve pas de créneau, ne calcule pas de disponibilité, ne gère pas de confirmation. Ces responsabilités appartiennent exclusivement à JayRDV.

---

## 11. Contraintes et invariants

Les contraintes suivantes sont **non négociables** et s'appliquent à toute implémentation, toute évolution et toute extension de JayKoa.

| # | Contrainte | Description |
|---|------------|-------------|
| **C-1** | JayKoa ne modifie jamais les données métier des Services sources | Les données métier restent sous l'autorité exclusive du Service source |
| **C-2** | JayKoa ne crée pas d'événement métier externe | JayKoa ne peut pas créer de rendez-vous dans JayRDV, ni d'inscription dans JayFestival |
| **C-3** | JayKoa ne réserve jamais de créneau | La réservation est une responsabilité métier qui appartient au Service de booking |
| **C-4** | JayKoa ne calcule aucune disponibilité | Le calcul de disponibilité est une logique métier qui ne relève pas de l'agenda |
| **C-5** | JayKoa ne résout aucun conflit de booking | La résolution de conflits de réservation appartient au Service métier concerné |
| **C-6** | Toute synchronisation est gouvernée | Un Mandat de Permission est requis pour toute synchronisation inter-Services |
| **C-7** | Toute écriture passe par KindMother | Aucune écriture directe dans un système de stockage |
| **C-8** | Aucun couplage direct entre Services | JayKoa ne dépend techniquement d'aucun Service source pour fonctionner |
| **C-9** | Offline-first | JayKoa fonctionne sans réseau (LOI-1, LOI-2, LOI-3) |
| **C-10** | Pas de temps global requis | JayKoa utilise les références temporelles locales (LOI-4) |
| **C-11** | JayKoa ne stocke que des représentations temporelles | Aucune donnée métier d'un autre Service n'est persistée par JayKoa |
| **C-12** | La source d'un événement synchronisé est immuable | Une fois créé, l'événement conserve sa référence source sans modification possible |

---

## 12. Résumé exécutif

JayKoa est le **calendrier universel du COG**.

Il ne possède aucune logique métier. Il ne prend aucune décision. Il ne réserve rien, n'inscrit personne, ne valide aucun engagement.

JayKoa **reflète le temps de tous les Services**.

> **JayFestival annonce le temps.**
> **JayRDV décide du temps.**
> **JayKoa le rend visible, lisible et orchestré.**

JayKoa n'appartient à aucun Service. Il reflète le temps de tous.

Dans l'écosystème Miyukini, JayKoa est le Service qui transforme des fragments temporels dispersés en une **vision cohérente et unifiée du temps**, au bénéfice de l'utilisateur, dans le respect de la souveraineté de chaque Service et de la gouvernance des Cores.

---

## 13. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie officielle (Service, Opérateur, Mandat de Permission, COG, Cores, WriteIntent, etc.) |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Besoins en écrans et UI pour les services consommateurs |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs et parcours côté service |
| [JayKoa - Bornage Implementation](./JayKoa%20-%20Bornage%20Implementation.md) | Bornage pour l'implémentation (MVP, phases, hors scope) |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits d'Outils, Équipe, filtres supportés |
| [JayKoa - Niveaux Securite et Protection Donnees](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Niveaux de sécurité WorrySentinel et mesures de protection |
| [JayKoa - Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Schémas d'intégration avec JayRDV, JayFestival, futurs Services |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel fonctionnel inspiré de Google Agenda |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service de booking et prise de rendez-vous |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service d'organisation d'événements et de festivals |
| [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | Résidence centralisée, COG de référence, niveaux 2+ |

---

**Document** : JayKoa — Document Fondateur
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document fondateur normatif — référence pour le Service
