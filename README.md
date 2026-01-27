# Miyukini Core System

## Table des matières

1. [Introduction](#1-introduction)
2. [Vision et philosophie](#2-vision-et-philosophie)
3. [La pyramide Miyukini](#3-la-pyramide-miyukini)
4. [Description des cores](#4-description-des-cores)
5. [Sécurité et gouvernance](#5-sécurité-et-gouvernance)
6. [Performance et scalabilité](#6-performance-et-scalabilité)
7. [Cas d'usage](#7-cas-dusage)
8. [MiyukiniAdmin](#8-miyukiniadmin)
9. [Comparatif avec l'existant](#9-comparatif-avec-lexistant)
10. [Apports inédits](#10-apports-inédits)
11. [Personal Vibe Coding Gouverné](#11-personal-vibe-coding-gouverné)
12. [À qui Miyukini n'est PAS destiné](#12-à-qui-miyukini-nest-pas-destiné)
13. [État du projet](#13-état-du-projet)
14. [Conclusion](#14-conclusion)
15. [Mini log de rédaction](#15-mini-log-de-rédaction)

---

## 1. Introduction

### Qu'est-ce que Miyukini

**Miyukini est un COG — Core-Orchestrated Governance Environment.**

> 🇫🇷 Un environnement de gouvernance orchestré par des cores. Il coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.
>
> 🇬🇧 *A Core-Orchestrated Governance Environment. It governs, coordinates and operates software systems from the core to the end user.*

Miyukini Core System (MCS) est un **écosystème logiciel gouverné** conçu pour produire des applications autonomes, sécurisées structurellement, et capables de fonctionner dans des conditions de contrainte extrême (offline, ressources limitées, environnements isolés).

Miyukini n'est pas un framework. Ce n'est pas une bibliothèque. C'est un **environnement gouverné dans lequel des produits existent**.

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

La distinction est fondamentale : un framework fournit des outils que le développeur utilise comme bon lui semble. Miyukini fournit un **cadre non négociable** dans lequel les produits opèrent selon des règles strictes, des invariants vérifiables, et une gouvernance centralisée.

### À qui s'adresse le projet

Miyukini s'adresse à trois catégories d'acteurs techniques :

**Architectes système** qui conçoivent des infrastructures nécessitant :
- Une autonomie structurelle (pas de dépendance cloud obligatoire)
- Une sécurité par conception (pas par configuration)
- Une traçabilité complète et auditable
- Un fonctionnement déterministe même en isolation

**Développeurs de produits** qui construisent des applications pour :
- Des collectivités avec budgets et connectivité limités
- Des événements avec déploiement temporaire sans réseau fiable
- Des systèmes IoT et edge computing
- Des contextes réglementés nécessitant des garanties vérifiables

**Décideurs techniques** qui évaluent des solutions pour :
- Des projets à horizon long (5-10 ans)
- Des systèmes critiques où l'échec n'est pas acceptable
- Des environnements où le contrôle total est non négociable

### Pourquoi Miyukini existe

Les architectures logicielles modernes reposent sur des hypothèses implicites :
- Connectivité permanente disponible
- Ressources cloud élastiques
- Services tiers toujours accessibles
- Cohérence garantie par des systèmes distribués externes

Ces hypothèses excluent une part significative des cas d'usage réels : zones blanches, hardware contraint, budgets limités, exigences réglementaires, indépendance technique.

Miyukini adopte la posture inverse : **la déconnexion est un état normal du système, pas une erreur à corriger**.

Cette inversion philosophique permet de construire des systèmes qui :
- Démarrent sans réseau
- Fonctionnent sans cloud
- Dégradent proprement en isolation
- Se comportent de manière prévisible sans synchronisation externe
- Restent administrables localement
- Évoluent quand le réseau revient (sans reconstruction)

---

## 2. Vision et philosophie

### Systèmes autonomes

Un système Miyukini est **autonome** au sens strict :

| Caractéristique | Description |
|-----------------|-------------|
| Démarrable sans réseau | Le système atteint un état opérationnel sans connexion externe |
| Fonctionnel sans cloud | Toutes les opérations métier essentielles s'exécutent localement |
| Dégradé proprement en isolation | L'isolement active un mode dégradé explicite, pas une cascade d'erreurs |
| Prévisible sans synchronisation | Le comportement est déterministe même sans données synchronisées |
| Administrable localement | Diagnostic et administration ne nécessitent pas de connexion externe |
| Évolutif à la reconnexion | La reconnexion déclenche une réconciliation, pas une reconstruction |

Cette autonomie n'est pas cosmétique. Elle est structurelle et vérifiable. Un composant qui ne respecte pas ces contraintes est en **violation architecturale**.

### Séparation stricte des responsabilités

Miyukini repose sur une séparation **non négociable** entre trois fonctions distinctes :

**Décision ≠ Exécution ≠ Persistance**

- **Décision** : StrongFather décide si une action est valide selon les politiques. Il ne persiste jamais, n'exécute jamais.
- **Exécution** : Les produits et adaptateurs exécutent les actions. Ils ne décident jamais de la validité politique.
- **Persistance** : KindMother persiste les données. Elle ne prend pas de décision stratégique.

Cette séparation élimine les ambiguïtés qui créent les failles de sécurité, les incohérences, et les bugs difficiles à tracer.

### IA gouvernée, non magique

Si une intelligence artificielle intervient dans Miyukini, elle opère selon des **contrats explicites** :
- L'IA propose, les cores valident
- L'IA n'a pas d'autorité sur les invariants
- Toute décision IA est traçable et auditable
- L'IA ne peut pas contourner StrongFather

Il n'y a pas de "magie" dans Miyukini. Chaque comportement est explicable, chaque décision est justifiable, chaque action est traçable.

### Fonctionnement en environnement isolé

Miyukini est conçu pour des environnements où :
- Le réseau est intermittent ou absent
- Le hardware est contraint (Raspberry Pi, mini PC, NAS)
- L'isolation est une contrainte, pas un choix
- Le temps n'est pas synchronisé entre nœuds

Les **6 lois d'autonomie** codifient ces contraintes :

| Loi | Énoncé |
|-----|--------|
| **LOI-1** | Aucune dépendance externe critique à l'exécution |
| **LOI-2** | Le système accepte l'isolement comme état normal |
| **LOI-3** | L'état local est souverain |
| **LOI-4** | Pas de temps global requis |
| **LOI-5** | Le coût doit être proportionnel au hardware |
| **LOI-6** | L'autonomie n'empêche pas la fédération |

Ces lois sont des **invariants architecturaux**. Toute décision de conception doit répondre à la question : *"Est-ce que ça fonctionne encore si le système est seul, lent, et isolé ?"*

---

## 3. La pyramide Miyukini

### Architecture en strates

L'écosystème Miyukini est organisé en **7 strates + Kernel**, avec une dépendance strictement unidirectionnelle (de haut en bas) :

```
┌──────────────────────────────────────────────┐
│ 🔧 STRATE 9 — MiyukiniAdmin (EXCEPTION)       │
│ Console souveraine d'administration          │
│ → Out-of-band, comme BIOS/hyperviseur        │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟩 STRATE 7 — PRODUITS FINIS                  │
│ SaaS · Apps · Jeux · CMS · Outils métier     │
│ B2C · B2B · B2B2C                             │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟦 STRATE 6 — PRODUITS INTERMÉDIAIRES         │
│ Auth · Billing · Content · Realtime · Admin  │
│ Monitoring · Workflow · Notification         │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟨 STRATE 5 — INTERFACES & ADAPTATION         │
│ UI · API · CLI · WebSocket · Edge             │
│ BondingBrother                               │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟥 STRATE 4 — CORES SYSTÈME                   │
│ StrongFather · KindMother · Caring Nanny      │
│ Master Butler · Border Guard · Ever Buddy     │
│ TAMR · WorrySentinel                         │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟪 STRATE 3 — INVARIANTS & CONTRATS            │
│ Décision ≠ Exécution · Zero-trust             │
│ Déterminisme · Auditabilité · Autonomie       │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ ⚙️ KERNEL — SUBSTRAT TECHNIQUE                │
│ Id · Logger · Clock (trace) · IO minimal      │
│ Portable · Local · Offline                   │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟫 STRATE 0 — HARDWARE & OS                   │
│ CPU · RAM · Disque · Réseau · Pannes          │
└──────────────────────────────────────────────┘
```

### Kernel

Le **Kernel** est la fondation technique réutilisable, agnostique produit, sans logique métier.

**Ce qu'il fournit :**
- **Id** : Génération d'identifiants uniques
- **Logger** : Logging structuré et traçable
- **Clock** : Horloge locale (trace only, pas de synchronisation)
- **Config** : Configuration locale
- **Lifecycle** : Gestion du cycle de vie (boot, arrêt)

**Ce qu'il ne fait jamais :**
- Logique métier
- Protocoles applicatifs (HTTP, WebSocket)
- ORM ou accès données
- Dépendance externe critique

### Cores fondamentaux

Les cores de la **Strate 4** sont les moteurs conceptuels qui gouvernent le comportement du système. Chaque core a une autorité exclusive dans son domaine et ne peut jamais empiéter sur les responsabilités d'un autre core.

### Adaptateurs (BondingBrother)

**BondingBrother** est l'interface fraternelle qui relie les produits autonomes à l'écosystème autoritaire. Il traduit les intentions des produits en demandes pour les cores, et traduit les réponses des cores en résultats pour les produits.

BondingBrother ne détient **aucune autorité**. Il médiatise, traduit, filtre, mais ne décide jamais.

### Modules transverses et produits intermédiaires

Les **produits intermédiaires** (Strate 6) sont la clé stratégique de Miyukini. Ce sont des capacités produits prêtes à l'emploi, recomposables, indépendantes du contexte business :

- Auth / Identity
- Billing Core
- Content Engine
- Realtime Engine
- Workflow Engine
- Notification
- Search / Index

Ces produits intermédiaires évitent le piège du monolithe : on ne fait pas "un CMS avec des plugins", on fait "un système qui peut produire un CMS".

### Produits finis

Les **produits finis** (Strate 7) combinent des produits intermédiaires avec une logique métier spécifique pour créer des livrables clients : SaaS, apps, jeux, CMS, outils métier.

### Position spéciale de MiyukiniAdmin

**MiyukiniAdmin** (Strate 9) est une **exception volontaire** à la logique produit standard. Il est au-dessus de la pyramide, pas dedans.

C'est une console root, comparable à un BIOS/hyperviseur : il observe, installe, arbitre, mais ne vit pas dans le flux normal. Aucun autre produit ne dépend de lui, il ne consomme aucun produit intermédiaire, il n'expose aucune API publique.

---

## 4. Description des cores

### StrongFather — Moteur de décision

**Question fondamentale** : *"Devrait-on faire cette action ?"*

**Rôle** : Évalue les intentions selon des politiques explicites et produit des décisions (acceptée, refusée, ambiguë).

**Ce qu'il fait** :
- Évalue des intentions selon des politiques déclaratives
- Applique le principe zero-trust (aucune confiance implicite)
- Établit des priorités entre intentions
- Détecte les ambiguïtés avant exécution
- Garantit la traçabilité complète des décisions

**Ce qu'il ne fait jamais** :
- Exécuter une action
- Persister des données
- Modifier l'état du système
- Gérer le temps technique
- Contenir de la logique métier spécifique

**Invariants clés** :
- INV-SF-1 : Aucune autorité sur l'exécution
- INV-SF-2 : Aucune autorité sur la persistance
- INV-SF-5 : Zero-trust systématique
- INV-SF-6 : Décisions non ambiguës

---

### KindMother — Moteur de données

**Question fondamentale** : *"Comment les données sont-elles persistées et synchronisées ?"*

**Rôle** : Autorité absolue sur la persistance, la synchronisation, et la cohérence des données.

**Ce qu'il fait** :
- Gère l'identité des instances (DB Mère / DB Filles)
- Garantit la cohérence locale, globale, et transactionnelle
- Supporte le mode offline-first
- Orchestre la synchronisation mère/fille
- Applique les permissions conceptuelles

**Ce qu'il ne fait jamais** :
- Prendre des décisions stratégiques
- Exposer SQLite directement
- Contenir de la logique métier

**Concepts clés** :
- **DB Mère** : Source de vérité unique, autorité finale
- **DB Fille** : Instance locale dérivée, fonctionne offline
- **WriteIntent** : Intention d'écriture avant validation
- **Delta** : Différence pour synchronisation

---

### Caring Nanny — Observateur d'état

**Question fondamentale** : *"Dans quel état se trouve le système ?"*

**Rôle** : Observe, détecte, classe, propage, et historise les états du système sans jamais modifier, décider, ou exécuter.

**Ce qu'il fait** :
- Observe l'état système global
- Détecte les anomalies proactivement
- Classifie les états (healthy, degraded, offline, syncing, error)
- Propage les changements d'état
- Maintient un historique complet

**Ce qu'il ne fait jamais** :
- Modifier des données
- Prendre des décisions
- Exécuter des actions correctives
- Valider des opérations

**États reconnus** :
- **healthy** : Fonctionnement normal
- **degraded** : Mode dégradé, système opérationnel
- **offline** : Mode déconnecté (état normal, pas erreur)
- **syncing** : Synchronisation en cours
- **error** : Erreur critique détectée

---

### Master Butler — Registre des capacités

**Question fondamentale** : *"Qu'est-ce qui peut être fait, et qui a le droit de le faire ?"*

**Rôle** : Registre central des capacités et permissions, exposant ce qui est possible sans jamais décider de ce qui est autorisé.

**Ce qu'il fait** :
- Recense toutes les capacités du système
- Définit les permissions associées
- Fournit les informations aux décideurs (StrongFather)
- Permet la découverte des capacités

**Ce qu'il ne fait jamais** :
- Décider si une action est autorisée
- Vérifier les permissions en temps réel
- Exécuter des actions
- Définir des politiques

**Distinction clé** :
- **Capacité** : Pouvoir technique intrinsèque (le composant peut le faire)
- **Permission** : Droit accordé pour utiliser une capacité

---

### Border Guard — Définition des frontières

**Question fondamentale** : *"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"*

**Rôle** : Définit les frontières, classifie les niveaux de confiance, établit les règles de franchissement. Ne filtre pas, ne bloque pas, n'applique pas.

**Ce qu'il fait** :
- Définit les frontières (externe, interne, intégration)
- Classifie les niveaux de confiance (Trusted, Verified, Unknown, Hostile)
- Établit les règles de franchissement
- Gouverne conceptuellement les intégrations

**Ce qu'il ne fait jamais** :
- Filtrer les interactions
- Bloquer les accès
- Authentifier techniquement
- Persister des données

---

### Ever Buddy — Cycle de vie et évolution

**Question fondamentale** : *"Comment le système évolue-t-il sans jamais se rompre ?"*

**Rôle** : Gouverne l'évolution des structures, des contrats, et des entités dans le temps.

**Ce qu'il fait** :
- Définit les états de cycle de vie (DRAFT, ACTIVE, DEPRECATED, RETIRED, ARCHIVED)
- Établit les règles de compatibilité
- Assure la traçabilité des évolutions
- Surveille la dette structurelle
- Planifie les transitions

**Ce qu'il ne fait jamais** :
- Migrer directement les données
- Forcer une évolution
- Garantir la compatibilité technique (responsabilité produit)

**Invariant clé** : Aucun élément ACTIVE ne peut passer directement à RETIRED. La période DEPRECATED est obligatoire.

---

### TAMR — Intervention humaine

**Question fondamentale** : *"Quand l'humain a-t-il le droit d'intervenir dans le système ?"*

**Rôle** : Définit les points d'intervention humaine, les types d'intervention, et les limites de l'autorité humaine.

**Types d'intervention** :
- **Approbation** : Valide une action avant exécution
- **Override** : Force une action malgré refus automatique (nécessite justification)
- **Escalade** : Élève une décision vers un niveau supérieur
- **Supervision** : Observe avec capacité d'intervention

**Ce qu'il ne fait jamais** :
- Décider (StrongFather décide)
- Persister (KindMother persiste)
- Définir d'interface utilisateur (responsabilité produit)

**Invariants clés** :
- Traçabilité absolue de toute intervention
- Responsabilité explicite de l'intervenant
- Limites infranchissables même par override

---

### WorrySentinel — Gouvernance de sécurité

**Question fondamentale** : *"Quel niveau de sécurité et quel état de confiance s'appliquent ?"*

**Rôle** : Core de gouvernance transversale qui définit les niveaux de sécurité, les états de confiance, et orchestre la dégradation progressive.

**Ce qu'il décide** :
- Niveau de confiance global (T0-T4)
- Niveau de sécurité actif (0-4)
- Mode de fonctionnement autorisé
- Niveau de dégradation requis

**Ce qu'il ne décide pas** :
- Actions spécifiques
- Permissions individuelles
- Intégrations
- Données

WorrySentinel agit comme une **pression verticale**, pas comme une brique horizontale. Il gouverne sans exécuter, contraint sans remplacer.

---

## 5. Sécurité et gouvernance

### Zero-trust

Miyukini applique le principe **zero-trust** à tous les niveaux :

- Aucun appelant n'est présumé valide
- Toute intention est évaluée selon les politiques
- Aucune confiance implicite entre composants
- Toute décision est justifiable et auditable

Ce zero-trust n'est pas une politique configurable. C'est un **invariant architectural** (INV-SF-5).

### WorrySentinel et les niveaux de sécurité

**5 niveaux de sécurité** (profil de risque du produit) :

| Niveau | Nom | Cas d'usage | Impact performance |
|--------|-----|-------------|-------------------|
| 0 | PUBLIC | Site vitrine, affichage | 🟢 Quasi nul |
| 1 | STANDARD | CMS, backoffice simple | 🟢 Faible |
| 2 | SENSITIVE | Données personnelles | 🟡 Modéré |
| 3 | CRITICAL | Auth, paiement, cores | 🟠 Accepté |
| 4 | HARDENED | Environnement hostile | 🔴 Secondaire |

**Principe** : La sécurité est un paramètre de gouvernance, pas un choix applicatif. Un produit déclare son profil de risque ; les cores adaptent leur comportement.

### Dégradation progressive

**5 états de confiance** (intégrité du système) :

| État | Niveau | Comportement |
|------|--------|--------------|
| Normal | T0 | Toutes capacités disponibles |
| Instable | T1 | Log renforcé, surveillance accrue |
| Dégradé | T2 | Certaines capacités désactivées |
| Restreint | T3 | Gel des produits non essentiels, TAMR requis |
| Bloqué | T4 | Uniquement diagnostics |

**Principe fondamental** : *"Un système autonome ne bloque jamais brutalement. Il observe, interprète, dégrade, puis bloque seulement quand il est sûr."*

### Fonctionnement offline/online

Le mode **offline-first** est un invariant, pas une option :

- Les WriteIntent sont acceptés localement et synchronisés plus tard
- La DB Fille détient l'autorité locale
- L'isolement est un état normal (offline), pas une erreur
- À la reconnexion : réconciliation explicite, jamais de "correction en douce"

### Contrôle d'intégrité

Le système utilise des **sondes d'intégrité** qui s'exécutent à chaque cycle critique :

- **Sondes structurelles** : Invariants des cores, cohérence inter-cores
- **Sondes comportementales** : Décisions incohérentes, patterns anormaux
- **Sondes environnementales** : Mémoire, disque, CPU
- **Sondes d'identité** : Validité de la System Identity, continuité

Ces sondes permettent de distinguer une panne hardware d'une intrusion, un bug d'un comportement modifié.

---

## 6. Performance et scalabilité

### Contraintes assumées

Miyukini assume des contraintes de performance explicites :

**LOI-5 : Le coût doit être proportionnel au hardware**

Le système doit tourner sur :
- Raspberry Pi 4 (4 Go RAM)
- Mini PC
- NAS
- VM isolée
- Serveur de terrain

Cela implique :
- Mémoire maîtrisée et prévisible
- CPU prévisible, pas de pics imprévisibles
- Pas de services fantômes en arrière-plan
- Pas de workers inutiles

### Ce qui est volontairement non optimisé

Miyukini ne cherche pas :
- La performance maximale à tout prix
- Le temps réel distribué
- La scalabilité horizontale infinie
- L'optimisation prématurée

### Pourquoi la pureté prime

La **pureté fonctionnelle** (pas d'effets de bord cachés, déterminisme) est un allié pour :
- La prédictibilité des performances
- La facilité de diagnostic
- La maintenabilité long terme
- La sécurité structurelle

L'impact performance de la sécurité est **proportionnel au risque** :
- Niveau 0 : Quasi nul
- Niveau 2 : Modéré mais contrôlé
- Niveau 4 : Secondaire (l'intégrité prime)

---

## 7. Cas d'usage

### B2B (Business to Business)

**Livrable** : Produits intermédiaires (Strate 6)

**Exemples** :
- Auth / Identity vendu comme brique
- Billing Core intégré dans un produit tiers
- Realtime Engine pour applications temps réel

**Caractéristiques** :
- Briques recomposables
- Contrats stables et documentés
- Intégration contrôlée via BondingBrother

### B2C (Business to Consumer)

**Livrable** : Produits finis (Strate 7)

**Exemples** :
- CMS complet pour PME
- Application de gestion événementielle
- Système de réservation autonome

**Caractéristiques** :
- Produit clé en main
- Fonctionnement offline
- Administration via MiyukiniAdmin

### B2B2C (Business to Business to Consumer)

**Livrable** : Produits finis + produits intermédiaires sous licence

**Exemples** :
- CMS + Auth/Billing pour revendeurs
- Plateforme événementielle personnalisable
- Système de commerce local avec franchise

**Caractéristiques** :
- Revendeur personnalise et revend
- Briques communes, personnalisation métier
- Revenus diversifiés

### Environnements contraints

**Cas d'usage prioritaires** :

| Contexte | Avantage Miyukini |
|----------|-------------------|
| Collectivités | Budgets limités, zones isolées, indépendance cloud |
| Événements | Déploiement temporaire, pas de réseau fiable |
| IoT / Edge | Hardware faible, ressources limitées |
| Zones blanches | Pas de connectivité, autonomie totale |
| Contraintes réglementaires | Données locales, pas de cloud obligatoire |
| Long terme | Fonctionnel même si le fournisseur cloud disparaît |

### Systèmes critiques

Miyukini convient aux systèmes où :
- L'échec n'est pas acceptable
- La traçabilité est légalement requise
- L'audit doit être possible a posteriori
- Le contrôle total est non négociable

### Produits temps réel / asynchrones

**Temps réel** : Pour le temps réel critique (jeux d'action, latence <100ms), Miyukini n'est pas adapté. La sécurité structurelle introduit une latence incompatible.

**Asynchrone** : Parfaitement adapté. Les jeux asynchrones, applications collaboratives, systèmes de workflow bénéficient pleinement de l'architecture.

---

## 8. MiyukiniAdmin

### Produit à part entière

MiyukiniAdmin est un **produit autonome, souverain, et non réutilisable**. C'est une exception volontaire à la logique produit standard.

### Rôle

MiyukiniAdmin est la **console root** de l'écosystème :

- **Installation & Bootstrap** : Installation complète de l'environnement Miyukini
- **Monitoring & Métriques** : Lecture passive de métriques système
- **Tests Techniques** : Environnement de diagnostic
- **Sécurité & Arbitrage** : Changement manuel des niveaux de sécurité
- **Accès aux Données** : Via KindMother (cas normal)
- **Recovery Exceptionnel** : Écriture DB directe (cas extrême, conditions strictes)

### Pouvoirs

MiyukiniAdmin peut :
- Installer et configurer l'écosystème complet
- Consulter tous les états et métriques
- Modifier les niveaux de sécurité
- Activer les modes de dégradation
- Intervenir en mode maintenance

Toute action est :
- Traçable
- Horodatée
- Justifiée
- Auditable

### Dangers

MiyukiniAdmin a un **pouvoir quasi absolu** sur l'écosystème. Mal utilisé, il peut :
- Compromettre l'intégrité du système
- Contourner les protections normales
- Modifier des données directement (mode recovery)

C'est pourquoi il est **strictement encadré** :
- Niveau de sécurité maximal
- Protocoles de sécurité renforcés
- Journalisation complète obligatoire

### Pourquoi il est volontairement isolé

MiyukiniAdmin est isolé pour des raisons architecturales :

- **Aucun produit ne dépend de lui** : Le système fonctionne sans MiyukiniAdmin une fois installé
- **Il ne consomme aucun produit intermédiaire** : Pas de dépendance vers le bas
- **Il n'expose aucune API publique** : Pas d'intégration possible
- **Il n'est jamais embarqué** : Pas de version "lite" dans un produit

**Signature conceptuelle** : MiyukiniAdmin est au Miyukini Core ce que le BIOS/UEFI est à un OS moderne : autonome, puissant, dangereux s'il est mal utilisé — et absolument nécessaire.

---

## 9. Comparatif avec l'existant

### CMS modernes (WordPress, Drupal)

| Aspect | WordPress/Drupal | Miyukini |
|--------|------------------|----------|
| Architecture | Monolithique avec plugins | Écosystème en strates |
| Offline | Non supporté | Natif (offline-first) |
| Sécurité | Par configuration | Par conception |
| Évolution | Refonte fréquente | Évolution progressive |
| Dépendances | Base de données externe | SQLite interne, autonome |
| Gouvernance | Dispersée | Centralisée par cores |

**Ce que Miyukini fait mieux** : Autonomie, sécurité structurelle, évolution sans rupture.

**Ce que Miyukini fait différemment** : Séparation décision/exécution/persistance.

**Ce que Miyukini ne cherche pas à faire** : Être un CMS clé-en-main immédiat. Miyukini produit des CMS, il n'en est pas un.

### Frameworks backend (Laravel, Rails, Spring)

| Aspect | Frameworks classiques | Miyukini |
|--------|----------------------|----------|
| Philosophie | Outils à disposition | Cadre gouverné |
| Liberté | Totale (développeur décide) | Contrainte (écosystème impose) |
| Sécurité | Responsabilité développeur | Responsabilité architecturale |
| Offline | À implémenter | Natif |
| Traçabilité | Optionnelle | Obligatoire |

**Ce que Miyukini fait mieux** : Garanties architecturales, fonctionnement isolé.

**Ce que Miyukini fait différemment** : Le développeur code "à l'intérieur" de Miyukini, pas "au-dessus".

**Ce que Miyukini ne cherche pas à faire** : Être un framework flexible. La flexibilité est dans la composition, pas dans la violation des invariants.

### Architectures microservices classiques

| Aspect | Microservices | Miyukini |
|--------|--------------|----------|
| Dépendances | Services distribués | Cores locaux |
| Réseau | Obligatoire | Optionnel |
| Cohérence | Eventual consistency | Cohérence locale garantie |
| Complexité | Élevée (orchestration) | Contrôlée (gouvernance) |
| Déploiement | Cloud-native | Local-first |

**Ce que Miyukini fait mieux** : Fonctionnement sans réseau, simplicité de déploiement.

**Ce que Miyukini fait différemment** : Pas de "services" séparés, mais des "cores" avec autorités exclusives.

**Ce que Miyukini ne cherche pas à faire** : Scalabilité horizontale infinie. Miyukini scale par instances autonomes, pas par distribution.

### Systèmes orientés événements

| Aspect | Event-driven | Miyukini |
|--------|-------------|----------|
| Communication | Messages asynchrones | Intentions via BondingBrother |
| État | Souvent distribué | Local-first |
| Ordonnancement | Brokers externes | Priorités StrongFather |
| Replay | Event sourcing | Journalisation traçable |

**Ce que Miyukini fait mieux** : Pas de dépendance à un broker externe.

**Ce que Miyukini fait différemment** : Les intentions sont des demandes évaluées, pas des commandes exécutées.

**Ce que Miyukini ne cherche pas à faire** : Event sourcing pur. La traçabilité est complète, mais la reconstruction d'état n'est pas l'objectif premier.

### Plateformes no-code / low-code

| Aspect | No-code/Low-code | Miyukini |
|--------|-----------------|----------|
| Public | Non-développeurs | Architectes et développeurs |
| Flexibilité | Contrainte par l'interface | Contrainte par l'architecture |
| Sécurité | Souvent cosmétique | Structurelle |
| Portabilité | Lock-in plateforme | Autonomie totale |
| Offline | Rarement | Natif |

**Ce que Miyukini fait mieux** : Contrôle total, pas de lock-in, sécurité réelle.

**Ce que Miyukini fait différemment** : Destiné à des développeurs, pas à des utilisateurs finaux.

**Ce que Miyukini ne cherche pas à faire** : Permettre le développement sans code.

### Backends serverless

| Aspect | Serverless | Miyukini |
|--------|-----------|----------|
| Dépendance | Cloud provider | Aucune |
| Coût | À l'usage (imprévisible) | Hardware fixe (prévisible) |
| Cold start | Présent | Absent |
| Offline | Impossible | Natif |
| Contrôle | Limité | Total |

**Ce que Miyukini fait mieux** : Indépendance totale, coûts prévisibles, contrôle absolu.

**Ce que Miyukini fait différemment** : Pas de "fonctions" isolées, mais un écosystème cohérent.

**Ce que Miyukini ne cherche pas à faire** : Scalabilité automatique à la demande. Miyukini assume une capacité fixe et prévisible.

---

## 10. Apports inédits

### Décision pure sans exécution

StrongFather est un **moteur de décision pur** :
- Il évalue des intentions selon des politiques
- Il produit des décisions (acceptée, refusée, ambiguë)
- Il ne possède aucune capacité d'exécution
- Il ne peut pas modifier l'état du système

Cette pureté élimine toute une catégorie de bugs et de failles : si le décideur ne peut pas exécuter, il ne peut pas mal exécuter.

### IA gouvernée par contrats

Si une IA intervient dans Miyukini :
- Elle propose, les cores valident
- Elle opère selon des contrats explicites
- Ses décisions sont traçables et auditables
- Elle ne peut pas contourner les invariants

Il n'y a pas de "boîte noire" dans Miyukini. Tout comportement est explicable.

### Architecture réellement modulaire

La modularité Miyukini n'est pas cosmétique :
- Chaque core a une autorité **exclusive**
- Les frontières sont **explicites et vérifiables**
- La composition est **contrôlée par BondingBrother**
- L'évolution est **gouvernée par Ever Buddy**

Un core ne peut jamais empiéter sur le domaine d'un autre. Cette garantie est architecturale, pas conventionnelle.

### Fonctionnement dégradé intelligent

La dégradation n'est pas un échec, c'est un **mode de fonctionnement** :
- Progression contrôlée (T0 → T1 → T2 → T3 → T4)
- Chaque niveau réduit des capacités de manière explicite
- Jamais de blocage brutal
- Sortie propre toujours possible

Le système "se défend sans devenir paranoïaque".

### Sécurité structurelle, pas cosmétique

La sécurité Miyukini n'est pas une couche ajoutée :
- Zero-trust est un invariant architectural
- Les niveaux de sécurité sont des paramètres de gouvernance
- Les cores adaptent leur comportement automatiquement
- Un produit ne peut pas contourner la sécurité

**"La sécurité est un paramètre de gouvernance, pas un choix applicatif."**

---

## 11. Personal Vibe Coding Gouverné

### Une approche inédite

Miyukini est développé selon une approche expérimentale : le **Personal Vibe Coding Gouverné**.

Le concept : utiliser le vibe coding (développement intuitif, rapide, assisté par IA) pour construire un écosystème logiciel complet — du Kernel jusqu'aux Opérateurs — mais encadré par une architecture contractuelle stricte.

> **"L'IA génère. Les contrats gouvernent. Les invariants ne négocient pas."**

### Qu'est-ce que le vibe coding ?

Le vibe coding est une approche de développement où le développeur :
- Utilise l'IA comme partenaire de génération de code
- Avance rapidement, par intuition
- Itère sans friction
- Accepte l'imperfection temporaire

**Le problème du vibe coding classique** : il produit souvent du code jetable, difficile à maintenir, sans architecture cohérente.

### Ce qui rend l'approche inédite

Le Personal Vibe Coding Gouverné inverse le paradigme :

| Vibe Coding classique | Personal Vibe Coding Gouverné |
|----------------------|------------------------------|
| Intuition pure | Intuition + Contrats |
| Prototype jetable | Système de production |
| "Ça marche" suffit | Invariants vérifiables |
| Refacto plus tard | Architecture dès le départ |
| Code = produit | Code = instance d'architecture |
| L'IA génère librement | L'IA génère, l'architecture valide |

**La thèse** : Si on peut vibe coder un écosystème complet avec une architecture rigoureuse, alors le vibe coding devient une méthode de production sérieuse — pas juste un outil de prototypage.

### Comparatif avec les approches traditionnelles

| Aspect | Développement traditionnel | Vibe Coding classique | Personal Vibe Coding Gouverné |
|--------|---------------------------|----------------------|------------------------------|
| Vitesse | Lente, méthodique | Très rapide | Rapide |
| Architecture | Définie en amont | Absente ou émergente | Définie en amont, respectée |
| IA | Outil ponctuel | Partenaire principal | Partenaire gouverné |
| Qualité | Contrôlée | Variable | Garantie par contrats |
| Scalabilité | Planifiée | Problématique | Structurelle |
| Maintenabilité | Élevée | Faible | Élevée |

### Le pari de Miyukini

Construire un écosystème logiciel complet :
- **Du plus bas niveau** (Kernel : Id, Logger, Clock)
- **Au plus haut niveau** (Opérateurs, produits finis)
- **En solo**, avec l'assistance de l'IA
- **Sans jamais violer** les invariants architecturaux

C'est la première fois qu'on tente cette approche sur un projet de cette envergure.

### L'enjeu

**Si l'expérimentation fonctionne**, cela pourrait changer l'approche de la programmation :

1. **Le vibe coding devient légitime** : Plus synonyme de "code jetable", mais méthode de production viable
2. **L'architecture prime** : L'IA peut générer du code à l'infini — sans architecture, c'est du bruit ; avec architecture, c'est un système
3. **L'abstraction devient critique** : Les développeurs qui maîtrisent l'abstraction et les contrats pourront produire des systèmes entiers assistés par IA
4. **Le solo devient viable** : Un développeur avec une architecture solide peut rivaliser avec des équipes entières

> **"L'IA peut générer du code à l'infini. Mais sans architecture, c'est du bruit. Avec architecture, c'est un système."**

### Ce que cela implique pour le futur

Si le Personal Vibe Coding Gouverné prouve sa viabilité :

- **L'architecture et l'abstraction seront plus que jamais importantes**
- Les développeurs devront maîtriser les contrats, les invariants, les frontières
- La capacité à gouverner la génération IA deviendra une compétence clé
- Les systèmes complexes pourront être construits plus rapidement, sans sacrifier la qualité

**Miyukini est autant un produit qu'une expérimentation méthodologique.**

---

## 12. À qui Miyukini n'est PAS destiné

La crédibilité exige d'identifier les cas où Miyukini n'est **pas** la bonne solution.

### Projets nécessitant une mise en production immédiate

Miyukini n'est pas un framework "batteries included". Il faut :
- Comprendre l'architecture avant d'utiliser
- Respecter les invariants (pas de raccourcis)
- Accepter les contraintes de gouvernance

Si le délai prime sur l'architecture, Miyukini n'est pas adapté.

### Applications temps réel critique

Pour les applications où la latence est critique (<100ms) :
- Jeux d'action multijoueurs
- Trading haute fréquence
- Systèmes de contrôle industriel temps réel

Les vérifications de sécurité structurelle introduisent une latence incompatible avec ces cas d'usage.

### Développeurs souhaitant une liberté totale

Miyukini impose des contraintes :
- Les produits ne décident pas de la sécurité
- Les invariants ne sont pas négociables
- La gouvernance est centralisée

Les développeurs qui refusent ces contraintes ne pourront pas travailler efficacement avec Miyukini.

### Projets sans exigence d'autonomie

Si le projet :
- A une connectivité permanente garantie
- Peut dépendre de services cloud
- N'a pas de contrainte de ressources

Miyukini apporte de la complexité sans bénéfice évident. Des solutions plus simples existent.

### Startups en phase d'exploration

En phase de discovery, où le produit change fréquemment :
- L'architecture Miyukini est trop structurée
- Les invariants ralentissent l'itération
- La gouvernance est un overhead

Miyukini convient aux systèmes dont les fondations sont stabilisées.

---

## 13. État du projet

### Maturité conceptuelle

L'architecture conceptuelle de Miyukini est **stabilisée et documentée** :

- **Pyramide architecturale** : Définie et validée (7 strates + Kernel)
- **Cores fondamentaux** : Documentation fondatrice complète pour tous les cores
- **Lois d'autonomie** : 6 lois non négociables formalisées
- **Contrats de sécurité** : Niveaux de sécurité (0-4) et états de confiance (T0-T4) documentés
- **Gouvernance d'écosystème** : Principes de dépendance verticale établis

Cette maturité conceptuelle permet :
- L'évaluation technique par des architectes
- La planification d'implémentation
- La comparaison avec des alternatives

### Implémentation

L'implémentation est en cours avec une approche progressive :

**Réalisé** :
- Kernel minimal (Id, Logger, Clock, Config, Lifecycle)
- Structure de base des modules SPM CMS
- Adaptateurs produits en mémoire (Content, Hierarchy, Taxonomies)

**En cours** :
- Traits fonctionnels des modules CMS
- Tests de conformité architecturale

**À venir** :
- Implémentation complète des cores en Rust
- Produits intermédiaires (Auth, Billing, Realtime)
- MiyukiniAdmin

### Roadmap implicite

L'ordre de construction suit la logique architecturale :

1. **Kernel** → Fondation technique (en cours)
2. **Cores** → Moteurs conceptuels
3. **MiyukiniAdmin** → Supervision système
4. **Produits Intermédiaires** → Capacités recomposables (étape stratégique)
5. **Produits Finis** → Livrables clients

L'étape 4 (Produits Intermédiaires) est la **clé stratégique** qui différencie Miyukini d'un simple framework.

### Statut expérimental

Miyukini est autant un **produit** qu'une **expérimentation méthodologique**.

L'approche Personal Vibe Coding Gouverné est testée en conditions réelles :
- Construction bottom-up (Kernel → Cores → Opérateurs)
- Assistance IA intensive
- Respect strict des contrats et invariants
- Développement solo

Les résultats de cette expérimentation seront documentés et partagés.

---

## 14. Conclusion

### Positionnement clair

Miyukini n'est pas un concurrent de WordPress, Laravel, ou Kubernetes. C'est une **réponse à un problème différent** : comment construire des systèmes autonomes, sécurisés structurellement, et capables de fonctionner dans des conditions de contrainte extrême.

Miyukini ne cherche pas à être :
- Le plus rapide
- Le plus flexible
- Le plus facile à adopter

Miyukini cherche à être :
- Le plus prévisible
- Le plus traçable
- Le plus autonome
- Le plus structurellement sécurisé

### Invitation à comprendre avant d'utiliser

Miyukini demande un investissement initial :
- Comprendre l'architecture en strates
- Accepter les contraintes de gouvernance
- Respecter les invariants non négociables

Cet investissement n'est pas une barrière artificielle. C'est la contrepartie des garanties offertes :
- Fonctionnement déterministe même en isolation
- Sécurité structurelle, pas cosmétique
- Évolution sans rupture
- Traçabilité complète et auditable

**"Miyukini n'est pas une bibliothèque. C'est un environnement gouverné dans lequel des produits existent."**

Les architectes et développeurs qui partagent cette vision trouveront dans Miyukini une fondation solide pour leurs systèmes les plus critiques.

### L'enjeu de l'expérimentation

Si le Personal Vibe Coding Gouverné prouve sa viabilité avec Miyukini, cela démontrera que :

1. **L'architecture domestique l'IA** : La génération de code assistée peut produire des systèmes de qualité production
2. **Le vibe coding mûrit** : D'un outil de prototypage à une méthode de développement légitime
3. **L'abstraction devient la compétence clé** : Ceux qui maîtrisent les contrats et les invariants pourront produire plus, mieux, plus vite

> **"L'architecture et l'abstraction seront plus que jamais importantes. C'est la condition pour que le vibe coding devienne une méthode de production, pas juste un raccourci."**

L'expérimentation est en cours.

---

## 15. Mini log de rédaction

### Mise à jour 2026-01-27

**Ajout de la section 11 : Personal Vibe Coding Gouverné**

Cette section documente l'approche de développement inédite utilisée pour construire Miyukini. Elle explique :
- La différence entre vibe coding classique et vibe coding gouverné
- Le comparatif avec les approches traditionnelles
- L'enjeu de l'expérimentation pour le futur de la programmation

**Raison de l'ajout** : Miyukini est autant un produit qu'une expérimentation méthodologique. Documenter cette approche permet de :
- Partager la méthodologie avec la communauté
- Valider ou invalider l'hypothèse du vibe coding gouverné
- Contribuer à la réflexion sur le futur du développement assisté par IA

---

### Ambiguïtés rencontrées

**A1 : Position exacte de WorrySentinel dans la pyramide**

Les documents montrent deux représentations : WorrySentinel en Strate 4 avec les autres cores (Pyramide principale) ou en position séparée comme "gouvernance transversale". La documentation fondatrice de WorrySentinel clarifie qu'il est en STRATE 4 mais agit comme une "pression verticale" sur les autres cores. J'ai retenu cette interprétation : WorrySentinel est techniquement en Strate 4 mais gouverne verticalement.

**A2 : Numérotation des strates**

Les documents présentent des numérotations légèrement différentes (0-7 vs 1-9). J'ai retenu la version la plus récente (Pyramide Architecture Complete v2.1) avec Strate 0 (Hardware), Kernel, Strate 3-7, et Strate 9 (MiyukiniAdmin exception).

**A3 : Distinction niveaux de sécurité / états de confiance**

Les niveaux de sécurité (0-4) et les états de confiance (T0-T4) sont deux axes indépendants. La confusion est possible. J'ai clarifié : niveaux de sécurité = profil de risque du produit ; états de confiance = intégrité du système.

### Choix éditoriaux structurants

**E1 : Longueur et exhaustivité**

Conformément à la demande, le README est long et exhaustif. Chaque section peut être lue indépendamment (lecture profonde) ou survolée (lecture rapide via les tableaux).

**E2 : Ton technique sans marketing**

Aucun superlatif commercial. Les comparaisons sont honnêtes (ce que Miyukini fait mieux, différemment, et ce qu'il ne cherche pas à faire).

**E3 : Section "À qui Miyukini n'est PAS destiné"**

Cette section est critique pour la crédibilité. Un système qui prétend convenir à tout le monde ne convient à personne.

### Sections volontairement limitées

**S1 : Détails d'implémentation**

Aucun code, aucun exemple technique. La documentation existante est conceptuelle et contractuelle, pas technique.

**S2 : Roadmap avec dates**

Aucune date ni estimation de délai. Conformément aux consignes, les timelines sont exclues.

**S3 : Comparatifs commerciaux**

Pas de comparaison de prix, de licences, ou de modèles économiques. Le README reste technique.

### Hypothèses explicitement refusées

**H1 : Miyukini est un framework**

Refusé. Miyukini est un écosystème gouverné. La distinction est fondamentale et répétée.

**H2 : Les invariants sont des recommandations**

Refusé. Les invariants sont des règles absolues. Les violer constitue une faute architecturale.

**H3 : L'offline est une fonctionnalité optionnelle**

Refusé. L'offline-first est un invariant structurel, pas une feature configurable.

**H4 : MiyukiniAdmin est un produit parmi d'autres**

Refusé. MiyukiniAdmin est une exception volontaire, hors de la pyramide produit standard.

---

**Document rédigé le :** 2026-01-27  
**Basé sur :** Documentation Miyukini Core System v2.5  
**Statut :** README racine officiel  
**Approche :** Personal Vibe Coding Gouverné (expérimental)
