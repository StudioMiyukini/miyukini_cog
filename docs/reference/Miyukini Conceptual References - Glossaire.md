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

### DÉPRÉCIÉ (DEPRECATED) — état de vie

État d'un élément toujours fonctionnel mais dont l'usage est découragé. Un successeur existe ou est en préparation.

**Caractéristiques :**

- Période de dépréciation définie
- Consommateurs avertis de migrer
- Passage obligatoire avant RETIRÉ

**Voir aussi :** ACTIF, RETIRÉ, Ever Buddy

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


---

**Date de création :** 2026-01-27  
**Version :** 1.4 (ajout équivalents anglais aux titres)  
**Statut :** Document de référence normatif — GLOSSAIRE OFFICIEL

**Références croisées :**

- [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Miyukini Conceptual References - Mandats et Équipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) : **Mandats de Permission et Équipes**
- [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Miyukini Conceptual References - Souveraineté Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md)
- [Miyukini Conceptual References - Objectif Final](./Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md)

