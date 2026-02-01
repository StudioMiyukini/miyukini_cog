# Miyukini Conceptual References — Glossaire

## Contexte

Ce document constitue le **dictionnaire officiel** de l'écosystème Miyukini. Il regroupe toutes les définitions canoniques, la terminologie officielle, et les concepts fondamentaux.

**Ce glossaire est la source de vérité terminologique.**

## Portée / Scope

- **Applicable à :** Toute documentation, communication, développement
- **Audience :** Tous (architectes, développeurs, marketing, IA)
- **Statut :** Document de référence normatif — GLOSSAIRE OFFICIEL

---

## A

### ACTIF (ACTIVE) — état de vie

État d'un élément en usage normal. L'élément est stable, documenté, supporté, et utilisable par tous les consommateurs autorisés. Les changements sont soumis aux règles de compatibilité.

**Voir aussi :** BROUILLON, DÉPRÉCIÉ, RETIRÉ, Ever Buddy

---

## B

### BondingBrother

**Core de médiation** (Strate 5). Interface fraternelle qui traduit les intentions des Opérateurs en demandes pour les Cores, et traduit les réponses en résultats.

**Rôle :** Médiation uniquement, jamais d'autorité.

**Question fondamentale :** *"Comment traduire cette intention pour les autorités ?"*

**Voir aussi :** Cores, Opérateur

---

### Border Guard

**Core de frontières** (Strate 4). Définit les frontières du système et les niveaux de confiance.

**Rôle :** Définition conceptuelle des frontières, pas d'application directe.

**Question fondamentale :** *"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"*

**Voir aussi :** Cores, Migration

---

### Bridge inter-COG

**Canal diplomatique** entre COG, extension de BondingBrother pour les communications inter-environnements.

**Rôle :**
- Transport des identités, intentions et autorisations
- **Aucun pouvoir décisionnel**
- **Aucun état métier**

**Règle fondamentale :**

> **Le bridge ne fait jamais confiance, il transporte.**

**Voir aussi :** BondingBrother, COG Hébergeur, COG Origine, Visite gouvernée inter-COG

---

## C

### Capacité (Capability)

Pouvoir technique qu'un composant possède. C'est ce qu'un module, un adaptateur, ou un Opérateur peut faire techniquement, indépendamment des permissions.

**Caractéristiques :**

- Intrinsèque au composant
- Technique (décrit un pouvoir fonctionnel)
- Déclarative (déclarée par le composant)
- Identifiable (identifiant unique et stable)

**Voir aussi :** Permission, Outil, Master Butler

---

### Caring Nanny

**Core d'observation d'état** (Strate 4). Observateur d'état du système qui détecte, classe et propage les états.

**Rôle :** Observer et rapporter l'état du système, sans jamais modifier, décider, ou exécuter.

**Question fondamentale :** *"Dans quel état se trouve le système à un instant donné ?"*

**Responsabilité Outils :** Cohérence d'état — bloque les Outils si l'environnement est dégradé.

**Voir aussi :** Cores, États de confiance

---

### Collaboration Mandatée (Mandated Collaboration)

**Coopération entre Opérateurs sous Mandat de Permission.** Les Opérateurs ne collaborent jamais librement — toute collaboration est encadrée par un mandat émis par StrongFather.

**Règles :**

- Pas de communication directe entre Opérateurs
- Passage obligatoire par BondingBrother
- Respect strict du Mandat de Permission

**Voir aussi :** Mandat de Permission, Équipe d'Opérateurs, BondingBrother

---

### Contrat d'Équipe (Team Contract)

**Règles statiques de collaboration** entre Opérateurs d'une même Équipe d'Opérateurs.

**Contenu du contrat :**

- Opérateurs membres
- Flux autorisés (qui parle à qui)
- Direction des flux
- Types d'échanges
- Types de données échangeables
- Conditions préalables
- Niveau de validation requis

**Caractéristiques :**

- Statique (défini à la conception)
- Validé par StrongFather
- Modification = processus formel

**Règle clé :** Le contrat est validé UNE FOIS, pas à chaque appel.

**Voir aussi :** Équipe d'Opérateurs, Mandat de Permission

---

### COG (Core-Orchestrated Governance Environment)

**Définition officielle de Miyukini.**

> **Miyukini est un COG — un environnement de gouvernance orchestré par des cores.**


| Lettre | Signification                                                 |
| ------ | ------------------------------------------------------------- |
| **C**  | Core — Les cores sont les unités fondamentales de gouvernance |
| **O**  | Orchestrated — Actif, coordonné (pas "operating")             |
| **G**  | Governance Environment — Environnement de gouvernance actif   |


**Ce que COG implique :**

- Orchestrated > Operating — Miyukini n'est pas un OS
- Governance > Governed — Actif, institutionnel
- Environment — Écosystème complet

**Voir aussi :** Environnement, Cores

---

### COG Hébergeur (Host COG)

**COG souverain qui accueille un Utilisateur Visiteur** provenant d'un autre environnement.

**Rôle :**
- Souverain exécutif de la session
- Unique source de vérité de l'état
- Autorité de sécurité et d'arbitrage

**Responsabilités :**
- Vérifier le visiteur
- Accorder ou refuser l'accès
- Encadrer strictement l'exécution
- Surveiller la session (WorrySentinel)
- Révoquer à tout moment

**Règle fondamentale :**

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

**Voir aussi :** COG Origine, Visa de Connexion, Visite gouvernée inter-COG

---

### COG Origine (Home COG)

**COG d'appartenance d'un Utilisateur Visiteur**, qui atteste de son identité.

**Rôle :**
- Autorité d'identité de l'utilisateur
- Garant de la conformité de l'environnement d'origine
- Émetteur du Passeport Utilisateur

**Responsabilités :**
- Vérifier l'intégrité locale
- Attester la version de la strate Core
- Fournir une identité vérifiable
- **Ne participe PAS à l'exécution distante**

**Voir aussi :** COG Hébergeur, Passeport Utilisateur, Visite gouvernée inter-COG

---

### COG de référence (Reference COG / Official COG)

**COG désigné comme détenteur canonique** des données sensibles d'un domaine donné. Il héberge l'Instance Mère KindMother (ou l'équivalent « serveur ») pour ce domaine.

**Rôle :**
- Détenteur canonique des données à résidence centralisée
- Source de vérité pour les données sensibles du domaine
- Accessible par les acteurs autorisés (Visite gouvernée, sync) — les terminaux ou autres COG n'en sont pas propriétaires, ils accèdent sans en être la seule copie

**Règle fondamentale :**

> **Les données sensibles à résidence centralisée ne doivent pas avoir pour seule copie un terminal ou un COG tiers. Leur copie canonique réside sur le COG de référence.**

**Voir aussi :** Politique de résidence des données sensibles, KindMother (Instance Mère), COG Hébergeur, WorrySentinel, Niveaux de sécurité

---

### COG Tracker (Webway Tracker)

**COG dont l'administrateur a choisi d'endosser le rôle de Tracker** : exposer volontairement une adresse (IP ou nom de domaine) pour participer au maillage Miyukini Webway System (MWS) et servir de point de rendez-vous pour la découverte.

**Port officiel :** les COGs Tracker MWS exposent leur endpoint sur le **port 21000**. Les COGs participants se connectent aux Trackers sur ce port par défaut.

**Rôle :**
- Point de rendez-vous pour la découverte (annonces de présence, requêtes de découverte)
- **Devoir de protection du réseau** par des mécanismes passifs et actifs (à créer)

**Ce qu'un COG Tracker N'EST PAS :**
- ❌ Un super-COG qui gouverne les autres
- ❌ Un transporteur de données métier
- ❌ Une autorité de Visa ou de Passeport

**Voir aussi :** Miyukini Webway System, COG Hébergeur, Bridge inter-COG

---

### Cores

**Moteurs conceptuels** (Strate 4) qui gouvernent le comportement du système. Chaque core a une autorité exclusive dans son domaine.


| Core          | Domaine                   |
| ------------- | ------------------------- |
| StrongFather  | Décision stratégique      |
| KindMother    | Données et persistance    |
| Caring Nanny  | Observation d'état        |
| Master Butler | Capacités et permissions  |
| Border Guard  | Frontières et confiance   |
| Ever Buddy    | Cycle de vie et évolution |
| WorrySentinel | Gouvernance de sécurité   |
| TAMR          | Intervention humaine      |


**Règle fondamentale :** Les cores décident ou gouvernent, mais n'exécutent jamais.

**Voir aussi :** Chaque core individuellement

---

## D

### Déclaration d'hébergement de session (Webway Host Session Declaration)

**Annonce par un COG Hébergeur au réseau MWS** indiquant qu'il héberge une session d'un service donné et qu'il attend des connexions vers lui (adresse et port).

**Contenu minimal (orientation) :** identifiant du service ou type de session, identifiant du COG Hébergeur, adresse de connexion (IP ou nom de domaine, port). La déclaration **ne donne aucun droit d'accès** ; elle indique où se présenter pour demander un Visa.

**Voir aussi :** Miyukini Webway System, COG Hébergeur, Norme de déclaration sécurisée (MWS)

---

### Demande de Visite (Visit Intent)

**Intention d'accès** émise par un Utilisateur Visiteur vers un COG Hébergeur.

**Émise par :** Utilisateur Visiteur

**Contient :**
| Champ | Description |
|-------|-------------|
| `requested_services` | Liste des Services demandés |
| `usage_nature` | Nature de l'usage (lecture, interaction, temps réel, etc.) |
| `security_level` | Niveau de sécurité requis |
| `terminal_context` | Contexte terminal (PC, mobile, web…) |

**Règle fondamentale :**

> **C'est une intention, pas une permission.**

**Voir aussi :** Passeport Utilisateur, Visa de Connexion, Visite gouvernée inter-COG

---

### DÉPRÉCIÉ (DEPRECATED) — état de vie

État d'un élément toujours fonctionnel mais dont l'usage est découragé. Un successeur existe ou est en préparation.

**Caractéristiques :**

- Période de dépréciation définie
- Consommateurs avertis de migrer
- Passage obligatoire avant RETIRÉ

**Voir aussi :** ACTIF, RETIRÉ, Ever Buddy

---

### Divergence silencieuse (Silent Divergence)

**Situation détectable par le Kernel** où un système déclare une version mais présente une empreinte comportementale différente.

**Causes typiques :**
- Build recompilé différemment
- Dépendance modifiée silencieusement
- Compilation non reproductible
- Injection de code ou modification post-build

**Caractéristiques :**
- Signal de maintenance, pas d'erreur
- Détectable sans réseau
- Déterministe et rejouable

**Règle fondamentale :**

> **Le Kernel signale la divergence mais ne la corrige jamais.**

**Voir aussi :** Empreinte comportementale, Maintenance explicable, Kernel Maintenance Observability Contract

---

### Opérateur de Domaine (Domain Operator)

**Type d'Opérateur** qui exerce un métier précis.

**Exemples :** Blog, Catalogue, Support, Base de connaissances, Forum

**Phrase type :** *"Exerce ce métier précis."*

**Voir aussi :** Opérateur, Opérateur de Service

---

### BROUILLON (DRAFT) — état de vie

État d'un élément en cours de définition. Non utilisable en production, peut changer librement, aucun engagement de stabilité.

**Voir aussi :** ACTIF, DÉPRÉCIÉ, RETIRÉ, Ever Buddy

---

## E

### Empreinte comportementale (Behavior Fingerprint)

**Signature structurelle** du système chargé, produite par le Kernel.

**Éléments capturés :**

| Élément | Description |
|---------|-------------|
| Ordre de chargement | Séquence d'initialisation des composants |
| Graphe d'appel structurel | Relations entre composants (pas métier) |
| Contrats invoqués | Liste des contrats activés |
| Invariants sollicités | Invariants vérifiés au chargement |

**Caractéristiques :**
- C'est une signature, pas un log
- Aucun contenu métier
- Aucune donnée runtime
- Déterministe et rejouable

**Utilité :**
- Comparer deux versions du système
- Détecter une dérive silencieuse
- Prouver l'équivalence fonctionnelle de builds

**Règle fondamentale :**

> **L'empreinte observe et atteste, mais ne corrige jamais.**

**Voir aussi :** Divergence silencieuse, Maintenance explicable, Kernel Maintenance Observability Contract

---

### Environnement (COG)

**Entité souveraine, versionnée, isolée et identifiée de manière unique.**


| Propriété                  | Description                |
| -------------------------- | -------------------------- |
| Version complète des cores | Ensemble cohérent et figé  |
| Itération unique           | Numéro de version distinct |
| ID unique                  | Généré par le kernel       |
| Opérateurs assujettis      | Liés à cet environnement   |
| Frontières strictes        | Limites claires            |


**Règle fondamentale :** La strate Cores est immuable. Toute évolution se fait par création d'un nouvel environnement complet.

**Voir aussi :** COG, Souveraineté, LOI-7

---

### États de confiance (T0-T4)

États caractérisant l'intégrité du système, gouvernés par WorrySentinel.


| État   | Nom       | Description                                 |
| ------ | --------- | ------------------------------------------- |
| **T0** | Normal    | Système sain, toutes capacités disponibles  |
| **T1** | Instable  | Anomalie détectée, surveillance accrue      |
| **T2** | Dégradé   | Incohérence persistante, capacités réduites |
| **T3** | Restreint | Suspicion forte, gel des non-essentiels     |
| **T4** | Bloqué    | Intégrité rompue, uniquement diagnostics    |


**Voir aussi :** WorrySentinel, Niveaux de sécurité

---

### Équipe d'Opérateurs (Operator Team)

**Collectif gouverné d'Opérateurs** qui collaborent sous règles explicites pour délivrer un Service.

**Définition canonique :**

> **Une Équipe d'Opérateurs est un collectif gouverné d'Opérateurs qui collaborent sous règles explicites pour délivrer un Service.**

**Composition :**

- Plusieurs Opérateurs (minimum 2)
- Hétérogènes en sécurité, responsabilités, exposition
- Liés par un Contrat d'Équipe
- Règles validées par StrongFather

**Ce qu'une Équipe N'EST PAS :**

- ❌ Un nouvel Opérateur
- ❌ Un produit
- ❌ Une hiérarchie libre

**👉 C'est une structure d'orchestration supérieure.**

**Règle clé :** Une Équipe d'Opérateurs ne peut exister opérationnellement que sous un Mandat de Permission valide.

**Voir aussi :** Opérateur, Service, Mandat de Permission, Contrat d'Équipe

---

### Ever Buddy

**Core de cycle de vie** (Strate 4). Gouverne l'évolution des structures, des contrats, et des entités dans le temps.

**Rôle :** Observer ce qui a été, ce qui est, et ce qui sera, sans jamais exécuter de migration.

**Question fondamentale :** *"Comment le système évolue-t-il sans jamais se rompre ?"*

**Responsabilité Tools :** Versions, dépréciation, compatibilité, migration.

**Voir aussi :** États de vie, Tool

---

### Façade Publique Gouvernée (Public Exposure Surface)

**Zone tampon d'exposition** permettant aux utilisateurs externes d'interagir avec un COG sans y entrer.

**Caractéristiques :**
- Strictement unidirectionnelle
- Sans identité persistante obligatoire
- Sans accès aux cores
- Sans accès à la logique interne
- Sans état souverain

**Règle fondamentale :**

> **C'est le COG qui sort vers l'utilisateur externe, jamais l'inverse.**

**Voir aussi :** Utilisateur Externe, Mandat Public d'Accès, BorderGuard

---

## G

### Gel local (Local Freeze)

**Capacité du Kernel** à marquer un composant comme gelé structurellement, sans affecter le reste du système.

**Actions permises :**
- Marquer un composant comme gelé
- Refuser son remplacement ou rechargement
- Laisser le reste du système évoluer

**Utilité :**
- Stabiliser une zone critique pendant une intervention
- Corriger ailleurs sans risque de régression
- Maintenir des SLA forts sur des composants spécifiques

**Gouvernance :**

| Acteur | Rôle |
|--------|------|
| StrongFather | Décide l'autorisation du gel |
| EverBuddy | Valide la compatibilité du gel |
| Kernel | Exécute le gel et l'applique |

**Règle fondamentale :**

> **Le gel est décidé par la gouvernance, exécuté par le Kernel, jamais inversé.**

**Voir aussi :** Kernel Maintenance Observability, StrongFather, Ever Buddy

---

## I

### Opérateur d'Interface (Interface Operator)

**Type d'Opérateur** qui expose les services de façon utilisable.

**Exemples :** UI web, App mobile, Tableau de bord, Panneau d'administration

**Phrase type :** *"Expose les services de façon utilisable."*

**Voir aussi :** Opérateur, Opérateur de Service

---

## K

### Kernel

**Substrat technique neutre** (entre Strate 0 et Strate 3). Fondation technique réutilisable, agnostique, sans logique métier.


| Composant | Rôle                              |
| --------- | --------------------------------- |
| Id        | Génération d'identifiants uniques |
| Logger    | Logging structuré                 |
| Clock     | Horloge locale (trace only)       |
| Config    | Configuration locale              |
| Lifecycle | Gestion du cycle de vie           |


**Invariants :**

- Aucune logique métier
- Aucune dépendance externe critique
- Pas de protocole applicatif

**Voir aussi :** Pyramide, Cores

---

### Kernel Maintenance Observability

**Ensemble de capacités bas niveau du Kernel** pour assister la maintenance du code sans jamais exécuter de correction automatique.

**Capacités incluses :**

| Capacité | Description |
|----------|-------------|
| Empreinte comportementale | Signature structurelle du système |
| Détection de divergence | Même version, comportement différent |
| Carte de complexité | Zones de couplage et fragilité |
| Gel local | Stabilisation par composant |
| Détection d'ambiguïté | Contrats incomplets ou code mort |
| Maintenance explicable | Traçabilité gouvernée des incidents |

**Ce que le Kernel PEUT faire :**
- Observer, attester, comparer, signaler, expliquer

**Ce que le Kernel ne peut JAMAIS faire :**
- Corriger, muter, auto-réparer

**Phrase fondatrice :**

> **Miyukini ne maintient pas le code à la place de l'humain. Il rend le code maintenable sans ambiguïté.**

**Voir aussi :** Empreinte comportementale, Divergence silencieuse, Maintenance explicable, Gel local

---

### KindMother

**Core de données** (Strate 4). Autorité absolue des données et de la persistance.

**Rôle :** Persistance, synchronisation, cohérence des données.

**Question fondamentale :** *"Comment les données sont-elles persistées et synchronisées ?"*

**Voir aussi :** Cores, WriteIntent

---

## L

### Local Sovereign ID (LSI)

**Niveau 1 d'identité d'environnement.** Générée par le kernel local, toujours valide localement, garantie localement.

**Cas d'usage :** Environnement isolé, offline permanent.

**Confiance :** Souveraine — l'environnement s'auto-déclare.

**Voir aussi :** Verified ID, Witnessed ID

---

### Liste de COGs avec statuts (Webway COG List)

**Liste maintenue par chaque COG participant au Miyukini Webway System (MWS)** associant à chaque COG connu un **statut** (Trusted, Neutral, Under review, Distrusted, Rejected). Permet d'analyser et, le cas échéant, de rejeter un COG ou une connexion considérée comme malveillante ou non fiable.

**Échange :** les COGs se transfèrent des listes ou des mises à jour de statuts selon le protocole MWS ; chaque COG reste souverain dans l'usage qu'il en fait.

**Voir aussi :** Miyukini Webway System, COG Tracker

---

### LOI-1 à LOI-8 (Lois d'Autonomie)

**8 lois d'autonomie non négociables** qui régissent l'architecture Miyukini.


| Loi       | Énoncé                                                     |
| --------- | ---------------------------------------------------------- |
| **LOI-1** | Aucune dépendance externe critique à l'exécution           |
| **LOI-2** | Le système accepte l'isolement comme état normal           |
| **LOI-3** | L'état local est souverain                                 |
| **LOI-4** | Pas de temps global requis                                 |
| **LOI-5** | Le coût doit être proportionnel au hardware                |
| **LOI-6** | L'autonomie n'empêche pas la fédération                    |
| **LOI-7** | La strate Cores est immuable — évolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements                |


**Voir aussi :** Autonomie, Souveraineté

---

## M

### Maintenance explicable (Explainable Maintenance)

**Mode de diagnostic du Kernel** pour fournir une traçabilité gouvernée lors d'incidents, sans exposer de données techniques sensibles.

**Informations fournies :**
- Pourquoi une décision est arrivée jusqu'ici
- Quels contrats ont été traversés
- Où la gouvernance s'est arrêtée

**Ce qui n'est JAMAIS fourni :**
- Stacktrace classique (fuite d'information technique)
- Dump mémoire (fuite de données sensibles)
- Données utilisateur (protection vie privée)

**Caractéristiques :**
- Traçabilité gouvernée, pas technique
- Compréhensible par un humain sans connaissance du code source
- Fonctionne offline

**Règle fondamentale :**

> **Le diagnostic explique le chemin de gouvernance, jamais l'implémentation.**

**Voir aussi :** Kernel Maintenance Observability, Caring Nanny

---

### Master Butler

**Core de capacités** (Strate 4). Registre central des capacités et permissions du système.

**Rôle :** Catalogue des capacités, définition des permissions, découverte.

**Question fondamentale :** *"Qu'est-ce qui est possible dans cet environnement ?"*

**Responsabilité Tools :** Déclare quels Tools existent, lie Capability → Tool, définit les permissions d'accès.

**Ce que Master Butler NE fait PAS :**

- N'implémente pas les Tools
- N'exécute pas les Tools
- Ne décide pas si un Tool doit être appelé

**Voir aussi :** Capability, Permission, Tool

---

### Mandat de Permission (Allow Mandate)

**Autorisation déléguée, temporaire et encadrée** émise par StrongFather, permettant à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.

**Définition canonique :**

> **Un Mandat de Permission est une autorisation déléguée, temporaire et encadrée, émise par StrongFather, qui permet à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.**

**Contenu d'un Mandat :**

- ID unique
- Opérateurs autorisés
- Flux autorisés
- Types de données
- Niveau de sécurité maximum
- Conditions de validité
- Règles de révocation

**Ce qu'un Mandat N'EST PAS :**

- ❌ Un token libre
- ❌ Une session classique
- ❌ Un cache de décision
- ❌ Un droit implicite
- ❌ Une permission globale

**Phrase fondatrice :**

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

**Causes de révocation :**

- Service terminé
- Condition hors cadre
- Violation de règle
- Alerte WorrySentinel
- Utilisateur quitte le flux
- Environnement change

**Voir aussi :** StrongFather, Équipe d'Opérateurs, Contrat d'Équipe

---

### Mandat Public d'Accès (Public Access Mandate)

**Autorisation attachée à un service public** pour encadrer l'accès des utilisateurs externes non certifiés.

**Défini par :** Host COG

**Contient :**
| Champ | Description |
|-------|-------------|
| `public_services` | Services publics accessibles |
| `allowed_methods` | Méthodes autorisées |
| `quotas` | Quotas d'utilisation |
| `rate_limits` | Limitations de fréquence |
| `security_level` | Niveau de sécurité (S1-S3) |
| `expected_behavior` | Comportement attendu |

**Différence avec le Visa de Connexion :**

| Aspect | Visa de Connexion | Mandat Public |
|--------|------------------|---------------|
| Destinataire | Utilisateur Visiteur | Service exposé |
| Identité requise | ✅ Passeport | ❌ Non |
| Accès cores | Indirect | ❌ Jamais |

**Règle fondamentale :**

> **Le mandat est attaché au service, pas à l'utilisateur.**

**Voir aussi :** Utilisateur Externe, Façade Publique Gouvernée, Visa de Connexion

---

### Migration

**Processus d'échange de données entre environnements.** Migration ≠ communication directe.

**Règles :**

- Migration = processus formel
- Migration = contrat explicite
- Migration = frontière contrôlée
- Migration = traduction, pas copie brute

**Acteurs :** Border Guard (règles), BondingBrother (traduction), StrongFather (décision), KindMother (persistance), Ever Buddy (compatibilité).

**Voir aussi :** LOI-8, Environnement

---

### Miyukini Webway System (MWS)

**Couche de présence et de découverte** des environnements COG disposant d'un accès réseau. Permet aux COGs de se déclarer, de savoir qui est présent sur le maillage, et de faciliter l'initiation des visites gouvernées (Passeport, Visa) sans transférer de données métier.

**Rôle :**
- Normaliser *qui est là* et *où se présenter* pour demander un Visa
- Système de sécurité fondé sur l'échange de listes de COGs avec statuts (Webway COG List)
- Les COGs Tracker ont le devoir de protéger le réseau par des mécanismes passifs et actifs (à créer)

**Règle fondamentale :**

> **Le Webway normalise la présence et facilite l'échange ; il ne transporte pas la gouvernance ni les données.**

**Développement Outils et Opérateurs :** voir [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) (annexe conceptuelle).

**Voir aussi :** COG Tracker, Liste de COGs avec statuts, Connexion Inter-COG, Bridge inter-COG

---

### MiyukiniAdmin

**Opérateur Souverain** — Console souveraine d'administration (Strate 9).

**Caractéristiques spéciales :**

- Exception à la logique Opérateur standard
- Autorité quasi institutionnelle
- N'est pas utilisable par d'autres Opérateurs
- Agit sous protocole spécial

**Fonctions :** Installation, diagnostic, arbitrage, accès exceptionnel.

**Voir aussi :** Opérateur Souverain, Opérateur

---

## N

### Norme de déclaration sécurisée (MWS)

**Norme à créer et à appliquer** pour les annonces MWS : services exposés, adresses (IP/ports) et sessions hébergées. Elle vise l'authentification de l'origine des déclarations, l'intégrité, un format unifié et la limitation des abus. Les COGs Tracker peuvent exiger la conformité pour accepter ou relayer les annonces.

**Développement :** voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (document conceptuel annexe).

**Voir aussi :** Miyukini Webway System, Déclaration d'hébergement de session, COG Tracker

---

### Niveaux de sécurité (0-4)

**Niveaux caractérisant le profil de risque**, gouvernés par WorrySentinel.


| Niveau | Nom       | Description                                  |
| ------ | --------- | -------------------------------------------- |
| **0**  | Public    | Données publiques, aucune contrainte stricte |
| **1**  | Standard  | Données standard, contraintes de base        |
| **2**  | Sensitive | Données sensibles, contraintes renforcées    |
| **3**  | Critical  | Données critiques, contraintes strictes      |
| **4**  | Highest   | Sécurité maximale, contraintes maximales     |


**Voir aussi :** WorrySentinel, États de confiance

---

## O

### Opérateur (Operator)

**Entité fonctionnelle gouvernée** qui exécute un rôle pour le compte de l'utilisateur (Strate 7).

**Définition canonique :**

> **Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.**

**Types d'Opérateurs :**


| Type                       | Rôle                         |
| -------------------------- | ---------------------------- |
| Opérateur de Service       | Gère un domaine fonctionnel  |
| Opérateur d'Interface      | Expose les services          |
| Opérateur d'Automatisation | Agit automatiquement         |
| Opérateur de Domaine       | Exerce un métier             |
| Opérateur Souverain        | Autorité système (exception) |


**Ce qu'un Opérateur N'EST PAS :**

- ❌ Un produit
- ❌ Une app
- ❌ Autonome
- ❌ Souverain

**Phrase fondatrice :**

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

**Voir aussi :** Outil, Kit d'Outils

---

## P

### Permission

**Droit accordé pour accéder à une capacité.** Autorisation conceptuelle d'utiliser une capacité.

**Caractéristiques :**

- Définie explicitement
- Associée à des capacités
- Attribuable à des rôles
- Révocable
- Traçable

**Distinction Capability vs Permission :**


| Aspect   | Capability           | Permission          |
| -------- | -------------------- | ------------------- |
| Nature   | Pouvoir technique    | Droit accordé       |
| Question | "Peut-on le faire ?" | "A-t-on le droit ?" |


**Voir aussi :** Capability, Master Butler

---

### Passeport Utilisateur (User Passport)

**Attestation d'identité** émise par un COG Origine pour permettre à un utilisateur de visiter d'autres COG.

**Émis par :** COG Origine

**Contient :**
| Champ | Description |
|-------|-------------|
| `user_id` | Identité utilisateur unique |
| `cog_origin_id` | ID du COG d'origine |
| `core_version` | Version exacte de la strate Core |
| `integrity_hash` | Empreinte d'intégrité (Core + Kernel) |
| `issued_at` | Timestamp d'émission |
| `valid_until` | Durée de validité |
| `signature` | Signature du COG Origine |

**Garanties :**
- Non falsifiable
- Non transférable
- Lisible mais non modifiable

**Règle fondamentale :**

> **Le passeport ne donne aucun droit. Il prouve seulement qui tu es et d'où tu viens.**

**Voir aussi :** COG Origine, Visa de Connexion, Visite gouvernée inter-COG

---

### Politique de résidence des données sensibles (Sensitive Data Residence Policy)

**Règle gouvernant où réside la copie canonique** des données sensibles : certaines données (personnelles, métier critique) ne doivent pas être dupliquées comme seule copie sur des terminaux ou des COG tiers.

**Contenu de la politique :**
- **Données à résidence centralisée** : liste ou critères (domaine, niveau WorrySentinel 2+) des données dont la copie canonique doit résider sur un COG de référence
- **COG de référence** : désignation du COG détenteur canonique (ex. COG organisateur, COG du Service)
- **Terminaux / COG tiers** : accès en lecture via Visite gouvernée ou sync ; écritures soumises en WriteIntent, validées et persistées sur la Mère (COG de référence)
- **Interdiction** : la seule copie de ces données ne doit jamais résider uniquement sur un terminal ou un COG non désigné comme COG de référence

**Effet :** En cas de coupure du terminal (ex. exposant), les données restent disponibles sur le COG de référence (ex. pour les organisateurs).

**Voir aussi :** COG de référence, KindMother (Instance Mère), WorrySentinel, Niveaux de sécurité, Migration

---

### Pyramide Miyukini

**Architecture en strates** de l'écosystème Miyukini.


| Strate | Nom                     | Contenu                           |
| ------ | ----------------------- | --------------------------------- |
| **9**  | MiyukiniAdmin           | Opérateur Souverain (exception)   |
| **7**  | Opérateurs              | Entités fonctionnelles gouvernées |
| **6**  | Tools & Toolkits        | Capacités exécutables             |
| **5**  | Interfaces & Adaptation | BondingBrother                    |
| **4**  | Cores Système           | StrongFather, KindMother, etc.    |
| **3**  | Invariants & Contrats   | Principes architecturaux          |
| **K**  | Kernel                  | Substrat technique neutre         |
| **0**  | Hardware & OS           | Réalité physique                  |


**Voir aussi :** Chaque strate individuellement

---

## R

### RETIRÉ (RETIRED) — état de vie

État d'un élément retiré du système. Non disponible, usage impossible.

**Caractéristiques :**

- Transition obligatoire depuis DÉPRÉCIÉ
- Pas de retour possible

**Voir aussi :** DÉPRÉCIÉ, Ever Buddy

---

## S

### Sécurité Hétérogène (Heterogeneous Security)

**Principe selon lequel une Équipe d'Opérateurs peut combiner différents niveaux de sécurité**, chaque Opérateur gardant son propre niveau.

**Règle fondamentale :**

> **Un Opérateur n'a qu'un seul niveau de sécurité. Une Équipe peut en combiner plusieurs.**

**Exemple :**


| Opérateur         | Rôle      | Sécurité      |
| ----------------- | --------- | ------------- |
| Opérateur UI      | Affichage | 🟢 Faible (1) |
| Opérateur Contenu | CMS       | 🟡 Moyen (2)  |
| Opérateur Auth    | Identité  | 🔴 Élevé (3)  |


**Résultat :** Risque segmenté, pas sécurité uniforme forcée.

**Règles absolues :**

- Un Opérateur ne peut jamais élever son niveau
- Un flux ne peut jamais descendre en sécurité
- Les ponts entre niveaux sont explicites, rares, auditables
- Les ponts sont validés par WorrySentinel

**Voir aussi :** Niveaux de sécurité, Équipe d'Opérateurs, WorrySentinel

---

### Service

**Capacité perçue par l'utilisateur.** Le Service est ce que l'utilisateur voit et utilise.

**Distinction fondamentale :**


| Concept       | Définition                        |
| ------------- | --------------------------------- |
| **Service**   | Capacité perçue par l'utilisateur |
| **Opérateur** | Unité d'exécution gouvernée       |


**Règle clé :**

> **Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs.**

**Implications :**

- L'utilisateur voit des Services
- Le système exécute via des Opérateurs
- La complexité est gérée par collaboration, pas accumulation

**Voir aussi :** Opérateur, Équipe d'Opérateurs

---

### Opérateur de Service (Service Operator)

**Type d'Opérateur** qui gère un domaine fonctionnel.

**Exemples :** CMS, Auth, E-commerce, CRM, Surveillance, Recherche, Facturation

**Phrase type :** *"Gère ce domaine pour moi."*

**Voir aussi :** Opérateur, Opérateur de Domaine, Service

---

### Souveraineté (Environnement)

**Principe selon lequel un COG est une entité souveraine**, versionnée, isolée et identifiable.

**Règles fondamentales :**

- La strate Cores est immuable
- Pas de patch, que des environnements complets
- Un Opérateur est lié à un environnement unique
- Migration = diplomatie explicite

**Voir aussi :** Environnement, LOI-7, LOI-8

---

### Opérateur Souverain (Sovereign Operator)

**Type d'Opérateur d'exception** avec autorité quasi institutionnelle.

**Seul exemple :** MiyukiniAdmin

**Caractéristiques :**

- N'est pas un citoyen normal
- Agit sous protocole spécial
- N'est pas utilisable par d'autres Opérateurs

**Voir aussi :** MiyukiniAdmin, Opérateur

---

### StrongFather

**Core de décision** (Strate 4). Moteur de décision stratégique et politique. **Émetteur des Mandats de Permission.**

**Rôle :** Décider si une action devrait être faite, sans jamais l'exécuter.

**Question fondamentale :** *"Devrait-on faire cette action ?"*

**Responsabilités clés :**

- Décision stratégique
- Validation des Contrats d'Équipe
- Émission des Mandats de Permission
- Révocation des mandats si nécessaire

**Invariants clés :**

- Ne possède jamais d'autorité d'exécution
- Ne modifie jamais un état ou un fait
- Décision ≠ Exécution

**Voir aussi :** Cores, KindMother, Mandat de Permission, Contrat d'Équipe

---

## T

### TAMR (Trust & Authority Mediation Resolver)

**Core d'intervention humaine** (Strate 4). Définit les points d'intervention humaine dans le système.

**Rôle :** Définir quand l'humain a le droit d'intervenir.

**Question fondamentale :** *"Quand l'humain a-t-il le droit d'intervenir dans le système ?"*

**Voir aussi :** Cores, WorrySentinel

---

### Outil (Tool)

**Capacité exécutable gouvernée** (Strate 6), sans autorité, sans décision métier, sans connaissance du contexte.

**Définition canonique :**

> **Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.**

**Caractéristiques :**

- Capacité atomique
- Sans autorité
- Sans logique métier
- Gouverné par les Cores

**Règle fondamentale :**

> **👉 Un Outil fait, mais ne décide jamais.**

**Exemples :** `layout.render`, `form.validate`, `query.execute`

**Voir aussi :** Kit d'Outils, Opérateur

---

### Kit d'Outils (Toolkit)

**Composition officielle d'Outils** (Strate 6), validée et déclarée par l'environnement.

**Définition canonique :**

> **Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience, cohérence et performance.**

**Caractéristiques :**

- Agrège des Outils existants
- N'ajoute aucune capacité nouvelle
- Sans logique métier
- Gouverné

**Règle fondamentale :**

> **👉 Un Kit d'Outils orchestre, mais n'ajoute pas de capacité.**

**Voir aussi :** Outil, Master Butler

---

## U

### Utilisateur Externe (Public User / Anonymous User / Web Visitor)

**Consommateur non certifié** de services exposés par un COG, sans aucune gouvernance propre.

**Ce qu'un utilisateur externe N'EST PAS :**
- ❌ Un citoyen
- ❌ Un visiteur inter-COG
- ❌ Un participant au système

**Caractéristiques :**
- Sans identité souveraine
- Sans COG d'origine
- Sans Passeport ni Visa
- Accès uniquement via Façade Publique Gouvernée
- Soumis à un Mandat Public d'Accès

**Dégradation agressive possible :**

| Action | Description |
|--------|-------------|
| Throttle | Ralentissement |
| Downgrade | Moins de fonctionnalités |
| Freeze | Lecture seule |
| Block | IP / session / pattern |
| Blackhole | Réponse neutre, pas d'erreur exploitable |

**Règle fondamentale :**

> **Un utilisateur externe n'entre jamais dans un COG. Il interagit uniquement avec une façade d'exposition gouvernée.**

**Phrase de synthèse :**

> **Les utilisateurs externes ne sont pas des visiteurs. Ce sont des consommateurs de surfaces exposées, sous mandat public.**

**Voir aussi :** Façade Publique Gouvernée, Mandat Public d'Accès, Utilisateur Visiteur

---

### Utilisateur Visiteur (Visitor User)

**Utilisateur accédant temporairement à un COG étranger** via un mécanisme de visite gouvernée.

**Statut :**
- Citoyen dans son COG d'origine
- Visiteur gouverné dans le COG hôte

**Caractéristiques :**
- Conserve son identité
- Perd toute souveraineté d'exécution
- Agit uniquement via un Visa de Connexion
- Ne transporte aucun core, aucune logique, aucun état

**Règle fondamentale :**

> **L'utilisateur n'est jamais souverain hors de son COG.**

**Voir aussi :** COG Origine, COG Hébergeur, Visa de Connexion, Passeport Utilisateur

---

## V

### Verified ID (VID)

**Niveau 2 d'identité d'environnement.** LSI vérifiée par un registre global.

**Cas d'usage :** Environnement connecté, fédéré.

**Confiance :** Attestée — un tiers a vérifié l'identité.

**Voir aussi :** Local Sovereign ID, Witnessed ID

---

## W

### Witnessed ID (WID)

**Niveau 3 d'identité d'environnement.** LSI vérifiée par échange indirect.

**Cas d'usage :** Environnement semi-connecté, clé USB, QR, signature.

**Confiance :** Témoignée — d'autres environnements attestent.

**Voir aussi :** Local Sovereign ID, Verified ID

---

### Visa de Connexion (Connection Visa)

**Autorisation temporaire** émise par un COG Hébergeur pour encadrer la session d'un Utilisateur Visiteur.

**Émis par :** Host COG

**Contient :**
| Champ | Description |
|-------|-------------|
| `authorized_services` | Services autorisés |
| `accessible_cores` | Cores accessibles (indirectement) |
| `security_level` | Niveau de sécurité accordé |
| `execution_rules` | Règles d'exécution |
| `time_limits` | Limites temporelles |
| `functional_limits` | Limites fonctionnelles |
| `terminal_constraints` | Contraintes terminal |
| `revocation_conditions` | Conditions de révocation |

**Caractéristiques :**
- Temporaire
- Révocable
- Non transférable
- Auditée
- Strictement interprétée

**Niveaux de sécurité du Visa :**

| Niveau | Nom | Usage typique |
|--------|-----|---------------|
| **S1** | Observation | Lecture, spectateur |
| **S2** | Interaction contrôlée | UI, formulaires |
| **S3** | Temps réel | Jeu, collaboration |
| **S4** | Sensible | Admin, finance |
| **S5** | Critique | MiyukiniAdmin |

**Règle fondamentale :**

> **Le Visa définit l'univers légal du visiteur. Un utilisateur = un Visa = un niveau unique.**

**Voir aussi :** COG Hébergeur, Utilisateur Visiteur, Visite gouvernée inter-COG

---

### Visite gouvernée inter-COG (Inter-COG Governed Visit)

**Modèle d'accès temporaire** permettant à un utilisateur d'un COG d'accéder aux services d'un autre COG sans importer sa gouvernance.

**Acteurs :**
| Acteur | Rôle |
|--------|------|
| **COG Origine** | Autorité d'identité, émetteur du Passeport |
| **Utilisateur Visiteur** | Citoyen visitant sous gouvernance étrangère |
| **COG Hébergeur** | Souverain exécutif, émetteur du Visa |
| **Bridge inter-COG** | Canal diplomatique (BondingBrother étendu) |

**Séquence :**
1. Pré-validation locale (COG Origine)
2. Présentation au Bridge (Passeport + Demande de Visite)
3. Douane du Host COG (vérification)
4. Émission du Visa
5. Session active (gouvernée)
6. Fin ou rupture

**Principes non négociables :**
- ❌ Aucun core n'est partagé
- ❌ Aucun état n'est migré en direct
- ❌ Aucun pouvoir n'est délégué
- ✅ Une seule gouvernance active
- ✅ Identité ≠ autorité
- ✅ Sécurité avant fluidité

**Phrase fondatrice :**

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

**Voir aussi :** COG Hébergeur, COG Origine, Passeport Utilisateur, Visa de Connexion, Utilisateur Visiteur, Bridge inter-COG

---

### WorrySentinel

**Core de gouvernance de sécurité** (Strate 4). Gouverne les niveaux de sécurité et les états de confiance.

**Rôle :** Gouverner la sécurité sans exécuter de contrôle technique.

**Question fondamentale :** *"Quel niveau de sécurité et quel état de confiance sont applicables ?"*

**Responsabilité Tools :** Niveau de sécurité requis, blocage en cas de menace, audit.

**Ce que WorrySentinel décide :**

- ✅ Niveau de confiance global
- ✅ Niveau de sécurité actif
- ✅ Mode de fonctionnement autorisé

**Ce que WorrySentinel ne décide PAS :**

- ❌ Des actions
- ❌ Des permissions
- ❌ Des données

**Voir aussi :** Niveaux de sécurité, États de confiance

---

### Intention d'Écriture (WriteIntent)

**Intention d'écriture** soumise à KindMother. Représente une demande de modification de données.

**Caractéristiques :**

- Soumise à validation
- Traçable
- Peut être acceptée, refusée, ou différée

**Voir aussi :** KindMother

---

## Phrases Fondatrices (Résumé)

### COG

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

### Opérateur (Operator)

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

### Outil & Kit d'Outils (Tool & Toolkit)

> **Les Outils sont des capacités exécutables gouvernées. Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

### Souveraineté

> **Dans Miyukini, la strate Cores est immuable. Toute évolution se fait par la création d'un nouvel environnement complet. Les Opérateurs sont liés à un environnement unique et ne peuvent exister hors de celui-ci.**

### Autonomie

> **Le réseau améliore le système, il ne le conditionne pas.**

### Complexité

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

### Mandat de Permission

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Sécurité

> **Risque segmenté, pas sécurité uniforme.**

### Visite inter-COG

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

### Utilisateurs externes

> **Les utilisateurs externes ne sont pas des visiteurs. Ce sont des consommateurs de surfaces exposées, sous mandat public.**

> **Un utilisateur externe n'entre jamais dans un COG. C'est le COG qui sort vers lui, jamais l'inverse.**

---

## Tableau de correspondance terminologique


| ❌ Terme incorrect               | ✅ Terme correct                            |
| ------------------------------- | ------------------------------------------ |
| Produit                         | **Opérateur**                              |
| App                             | **Opérateur** ou **Opérateur d'Interface** |
| Produit final                   | **Opérateur**                              |
| Produit intermédiaire           | **Outil** ou **Kit d'Outils**              |
| Créer un produit                | **Déployer un Opérateur**                  |
| Utiliser une app                | **Interagir avec un Opérateur**            |
| Marketplace                     | **Registre d'Opérateurs**                  |
| Decision Window                 | **Mandat de Permission**                   |
| Temporary Decision              | **Autorisation Mandatée**                  |
| Fast Path                       | **Chemin Mandaté**                         |
| Collaboration Opérateur (libre) | **Collaboration Mandatée**                 |
| Super-Opérateur                 | **Équipe d'Opérateurs**                    |
| Tool                            | **Outil**                                  |
| Toolkit                         | **Kit d'Outils**                           |
| Operator                        | **Opérateur**                              |
| User Passport                   | **Passeport Utilisateur**                  |
| Connection Visa                 | **Visa de Connexion**                      |
| Visit Intent                    | **Demande de Visite**                      |
| Visitor User                    | **Utilisateur Visiteur**                   |
| Host COG                        | **COG Hébergeur**                          |
| Home COG                        | **COG Origine**                            |
| Reference COG / Official COG    | **COG de référence**                       |
| Inter-COG Bridge                | **Bridge inter-COG**                       |
| Public User                     | **Utilisateur Externe**                    |
| Anonymous User                  | **Utilisateur Externe**                    |
| Web Visitor                     | **Utilisateur Externe**                    |
| Public Exposure Surface         | **Façade Publique Gouvernée**              |
| Public Access Mandate           | **Mandat Public d'Accès**                  |
| Tracker (rôle Webway)           | **COG Tracker**                           |


---

**Date de création :** 2026-01-27  
**Version :** 1.10 (ajout COG de référence, Politique de résidence des données sensibles)  
**Statut :** Document de référence normatif — GLOSSAIRE OFFICIEL

**Références croisées :**

- [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)
- [Miyukini Conceptual References - Politique Residence Donnees Sensibles](./Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) : **Centralisation et résidence des données sensibles**
- [Miyukini Conceptual References - Miyukini Webway System](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) : **Couche de présence et découverte (MWS)**
- [Miyukini Conceptual References - Miyukini Webway System Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) : **Annexe MWS — normes, formats, protocole, matrice des statuts**
- [Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) : **Annexe MWS — Outils, Kits d'Outils, Opérateurs MWS**
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Miyukini Conceptual References - Mandats et Équipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) : **Mandats de Permission et Équipes**
- [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Miyukini Conceptual References - Souveraineté Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md)
- [Miyukini Conceptual References - Objectif Final](./Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md)
- [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) : **Architecture de visite gouvernée**
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](./Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) : **Capacités bas niveau de maintenance**

