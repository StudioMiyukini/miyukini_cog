# Miyukini Core System — Lois d'Autonomie Système

## 1. Contexte

Ce document définit la **contrainte d'autonomie** comme contrainte architecturale de premier rang du Miyukini Core System. Cette contrainte stipule que l'écosystème doit fonctionner correctement sur du hardware simple, parfois isolé, sans dépendance externe critique.

Cette contrainte n'est pas un détail d'implémentation. C'est un **invariant architectural non négociable** qui influence toutes les décisions de conception, de l'écosystème jusqu'aux Opérateurs.

L'autonomie est ce qui distingue Miyukini des architectures "cloud-native" modernes qui présupposent une connectivité permanente. Miyukini adopte une posture inverse : **la déconnexion est un état normal du système**, pas une erreur à corriger.

## 2. Portée / Scope

Ce document définit :
- Les 8 lois d'autonomie non négociables
- La définition opérationnelle de l'autonomie dans Miyukini
- Les implications pour chaque core de l'écosystème
- Les états d'isolement reconnus du système
- La distinction entre ce qui est synchronisable et ce qui ne l'est jamais
- Le positionnement de Bonding Brother comme fédérateur

Ce document **ne couvre pas** :
- Les détails d'implémentation des mécanismes de synchronisation (voir KindMother - Sync & Conflict Resolution Contract)
- Les protocoles de communication inter-instances (voir BondingBrother - Bilateral Flow Contract)
- Les stratégies de résolution de conflits (voir KindMother - Failure & Degradation Contract)

---

## 3. Définition opérationnelle de l'autonomie

### Ce que "autonome" signifie dans Miyukini

Un système autonome Miyukini est un système qui :

| Caractéristique | Description |
|-----------------|-------------|
| **Démarrable sans réseau** | Le système démarre et atteint un état opérationnel sans aucune connexion externe |
| **Fonctionnel sans cloud** | Toutes les opérations métier essentielles sont exécutables localement |
| **Dégradé proprement en isolation** | L'isolement active un mode dégradé explicite, pas une cascade d'erreurs |
| **Prévisible sans synchronisation** | Le comportement du système est déterministe même sans données synchronisées |
| **Administrable localement** | L'administration et le diagnostic ne nécessitent pas de connexion externe |
| **Évolutif quand le réseau revient** | La reconnexion déclenche une réconciliation, pas une reconstruction |

### Ce que "autonome" ne signifie PAS

L'autonomie Miyukini n'est pas :

- **"Offline-first" marketing** : Miyukini ne cache pas une dépendance réseau derrière un cache local. L'autonomie est structurelle, pas cosmétique.
- **Du cache déguisé** : Les données locales ne sont pas un cache de données distantes. Elles sont une source de vérité locale avec réconciliation ultérieure.
- **De la réplication magique** : La synchronisation est explicite, contrôlée, et observable. Pas de synchronisation automatique silencieuse.
- **Du temps réel distribué obligatoire** : Miyukini ne présuppose pas de cohérence temps réel entre nœuds. Chaque nœud est cohérent avec lui-même.

---

## 4. Les 8 lois d'autonomie (non négociables)

Ces 8 lois constituent des **invariants architecturaux**. Toute décision de conception, d'implémentation, ou d'évolution doit respecter ces lois. Une violation de l'une de ces lois est une violation architecturale.

---

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Énoncé :** Un Opérateur Miyukini doit pouvoir démarrer, décider, fonctionner, et être audité sans aucun appel externe obligatoire.

**Implications :**
- Les APIs externes sont **optionnelles**, pas fondamentales
- Les services distants sont des **améliorations**, jamais des fondations
- Les cores (StrongFather, KindMother, Caring Nanny) sont **auto-suffisants**
- L'absence de connexion ne bloque jamais le démarrage du système

**Vérification :** Pour chaque composant, poser la question : *"Ce composant fonctionne-t-il si le réseau est indisponible ?"* Si la réponse est non, il y a violation de LOI-1.

**Relation avec les cores :**
- **StrongFather** : Les politiques sont locales. Aucune évaluation ne nécessite un appel externe.
- **KindMother** : La persistance locale est toujours disponible (DB Fille en mode offline-first).
- **Caring Nanny** : L'observation d'état fonctionne localement.

---

### LOI-2 : Le système accepte l'isolement comme état normal

**Énoncé :** L'isolement n'est pas une erreur. C'est un état valide et explicitement reconnu du système.

**Implications :**
- Pas d'exception "network unreachable" qui remonte en cascade
- Pas de retry infini bloquant
- Pas de blocage décisionnel en attente de synchronisation
- Le système prend des décisions **avec ce qu'il a**, pas avec ce qu'il pourrait avoir

**Vérification :** Pour chaque opération, poser la question : *"Cette opération bloque-t-elle en attente d'une ressource externe ?"* Si oui, il y a violation de LOI-2.

**Relation avec les cores :**
- **StrongFather** : Génère des décisions avec le contexte local disponible. Ne refuse jamais une décision au motif d'un contexte externe manquant.
- **KindMother** : Les WriteIntent sont acceptés localement et synchronisés plus tard. Pas de blocage en attente de validation distante.
- **Caring Nanny** : Reconnaît et signale l'état "isolé" comme un état normal, pas comme une anomalie.

---

### LOI-3 : L'état local est souverain

**Énoncé :** Quand un nœud est isolé, son état est la vérité locale, ses décisions sont valables localement, ses logs sont auditables localement.

**Implications :**
- Un nœud isolé ne met pas en doute sa propre cohérence
- Les décisions prises en isolation ne sont pas invalidées a posteriori
- Les logs locaux constituent une trace d'audit complète
- À la reconnexion : réconciliation, comparaison, explication — jamais de "correction en douce"

**Vérification :** Pour chaque donnée locale, poser la question : *"Cette donnée est-elle considérée comme valide tant qu'elle n'est pas explicitement réconciliée ?"* Si non, il y a violation de LOI-3.

**Relation avec les cores :**
- **KindMother** : La DB Fille détient l'autorité locale. La réconciliation avec la DB Mère est explicite et traçable (voir KindMother - Sync & Conflict Resolution Contract).
- **StrongFather** : Les décisions prises localement sont enregistrées et ne sont jamais effacées, même si une décision différente aurait été prise avec plus de contexte.
- **Caring Nanny** : Enregistre l'historique local de manière complète et autonome.

---

### LOI-4 : Pas de temps global requis

**Énoncé :** Un système autonome ne dépend pas d'une horloge réseau, d'un ordre global, ou de timestamps synchronisés entre nœuds.

**Implications :**
- Le temps est local, relatif, et contextuel
- Les comparaisons temporelles entre nœuds sont évitées ou explicitement encadrées
- Les conflits de synchronisation ne se résolvent pas par "le plus récent gagne" de manière implicite
- Les horloges logiques ou vectorielles sont préférées aux timestamps absolus pour l'ordonnancement

**Vérification :** Pour chaque mécanisme temporel, poser la question : *"Ce mécanisme fonctionne-t-il si les horloges des nœuds diffèrent de plusieurs minutes/heures ?"* Si non, il y a violation de LOI-4.

**Relation avec les cores :**
- **StrongFather** : Déjà conçu sans logique temporelle technique (voir INV-SF-4 dans la Documentation Fondatrice).
- **KindMother** : La synchronisation utilise des deltas et des points de synchronisation, pas des timestamps absolus.
- **Caring Nanny** : Les observations sont horodatées localement. La comparaison inter-nœuds est explicitement encadrée.

---

### LOI-5 : Le coût doit être proportionnel au hardware

**Énoncé :** Le système doit tourner sur du hardware simple : mini PC, NAS, Raspberry Pi, VM isolée, serveur de terrain.

**Implications :**
- Mémoire maîtrisée et prévisible
- CPU prévisible, pas de pics imprévisibles
- Pas de services fantômes consommant des ressources en arrière-plan
- Pas de workers inutiles ou de processus dormants coûteux
- La pureté fonctionnelle (pas d'effets de bord, pas d'état caché) est un allié pour la prédictibilité

**Vérification :** Pour chaque composant, poser la question : *"Ce composant fonctionne-t-il de manière acceptable sur un Raspberry Pi 4 avec 4 Go de RAM ?"* Si non, il y a violation de LOI-5.

**Relation avec les cores :**
- **StrongFather** : Moteur de décision pur, sans état persistant, sans worker permanent.
- **KindMother** : SQLite interne, optimisé pour les ressources limitées.
- **Caring Nanny** : Observateur passif, consommation minimale.

---

### LOI-6 : L'autonomie n'empêche pas la fédération

**Énoncé :** Autonome ne signifie pas solitaire. Le système doit pouvoir fonctionner seul, puis se connecter à d'autres, sans changer de nature.

**Implications :**
- La fédération est explicite (décision consciente de se connecter)
- La fédération est contrôlée (règles de ce qui est partagé)
- La fédération est observable (traçabilité des échanges)
- La fédération est réversible (possibilité de se déconnecter)
- Un nœud fédéré reste autonome au sens de LOI-1 à LOI-5

**Vérification :** Pour chaque mécanisme de fédération, poser les questions :
- *"Le nœud peut-il décider de ne pas fédérer ?"*
- *"Le nœud peut-il quitter la fédération sans perte de fonctionnalité locale ?"*
- *"Les échanges fédérés sont-ils traçables et explicites ?"*

Si une réponse est non, il y a violation de LOI-6.

**Relation avec les cores :**
- **Bonding Brother** : Devient le **pont de synchronisation** et le médiateur de fédération. C'est le point d'entrée/sortie contrôlé pour les échanges inter-nœuds.
- **Border Guard** : Contrôle ce qui entre et sort du système. Rien d'implicite.

---

### LOI-7 : La strate Cores est immuable — évolution par environnement

**Énoncé :** Dans Miyukini, la strate Cores (Strate 4) n'est jamais patchée individuellement. Toute évolution se fait par la création d'un nouvel environnement complet. Les Opérateurs sont liés à un environnement unique et ne peuvent exister hors de celui-ci.

**Implications :**
- Pas de micro-patch sur la strate Cores
- Versions complètes d'environnement uniquement
- Pas de hotfix sauvage en production
- Un COG (Core Operating Group) est une entité souveraine liée à un environnement

**Vérification :** Pour toute évolution, poser les questions :
- *"Modifie-t-on un core existant en place ou déploie-t-on un nouvel environnement ?"*
- *"Un Opérateur peut-il exister dans plus d'un environnement à la fois ?"*

Si l'évolution patch un core en place ou qu'un Opérateur est partagé entre environnements, il y a violation de LOI-7.

**Relation avec les cores :**
- **Tous les cores (Strate 4)** : Évoluent par version d'environnement, jamais par patch isolé.
- **Voir :** [Miyukini Conceptual References - Souveraineté Environnement](Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md), [Pyramide Architecture Complete](Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md).

---

### LOI-8 : Migration = diplomatie entre environnements

**Énoncé :** La migration d'un Opérateur ou de données entre environnements est une opération explicite, négociée et traçable — jamais automatique ni implicite.

**Implications :**
- La migration est une décision consciente, pas un effet de bord
- Les protocoles de migration sont documentés et gouvernés
- Chaque environnement reste souverain ; l'échange est diplomatique
- Pas de "sync magique" qui déplace des Opérateurs entre environnements sans accord

**Vérification :** Pour tout mécanisme de déplacement entre environnements, poser les questions :
- *"La migration est-elle explicitement déclenchée et autorisée ?"*
- *"Les deux environnements (source et cible) participent-ils à la décision ?"*

Si la migration est implicite, automatique ou non traçable, il y a violation de LOI-8.

**Relation avec les cores :**
- **Bonding Brother** : Peut faciliter les échanges entre environnements, mais selon des règles explicites (LOI-6, LOI-8).
- **Voir :** [Miyukini Conceptual References - Souveraineté Environnement](Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md).

---

## 5. États d'isolement reconnus du système

Le système reconnaît explicitement les états d'isolement suivants. Caring Nanny est responsable de l'observation et de la classification de ces états.

### 5.1 État : Connecté (healthy)

**Description :** Le système est connecté à la DB Mère et/ou aux autres nœuds fédérés. Toutes les fonctionnalités sont disponibles.

**Caractéristiques :**
- Synchronisation active
- Réconciliation en temps réel ou quasi-réel
- État global cohérent

### 5.2 État : Isolé (offline)

**Description :** Le système fonctionne sans aucune connexion externe. C'est un état normal, pas une erreur.

**Caractéristiques :**
- Toutes les opérations locales fonctionnent
- Les WriteIntent sont stockés pour synchronisation ultérieure
- L'état local est souverain
- Pas de tentative de reconnexion automatique agressive

### 5.3 État : Partiellement synchronisé (syncing)

**Description :** Le système est en cours de réconciliation avec un ou plusieurs nœuds. Certaines opérations peuvent être différées.

**Caractéristiques :**
- Deltas en cours de traitement
- Conflits potentiellement en attente de résolution
- État transitoire vers "connecté" ou "isolé"

### 5.4 État : Dégradé (degraded)

**Description :** Le système fonctionne mais certaines fonctionnalités sont indisponibles ou limitées.

**Caractéristiques :**
- Fonctionnalités essentielles disponibles
- Fonctionnalités non essentielles désactivées ou limitées
- Cause identifiée et communiquée

### 5.5 État : Fédéré (federated)

**Description :** Le système est connecté à d'autres nœuds dans une fédération. Il conserve son autonomie tout en participant à un réseau plus large.

**Caractéristiques :**
- Autonomie locale préservée (LOI-1 à LOI-5)
- Échanges fédérés actifs via Bonding Brother
- Règles de partage explicites et contrôlées

---

## 6. Ce qui est synchronisable vs ce qui ne l'est jamais

### 6.1 Synchronisable (via KindMother + Bonding Brother)

| Catégorie | Description | Mécanisme |
|-----------|-------------|-----------|
| **Données métier** | Contenus, entités, relations | KindMother Deltas |
| **Décisions produites** | Résultats des évaluations StrongFather | Journaux exportables |
| **États observés** | Historique des états Caring Nanny | Journaux exportables |
| **Métadonnées de synchronisation** | Points de synchronisation, checksums | KindMother interne |

### 6.2 Jamais synchronisable

| Catégorie | Raison | Conséquence |
|-----------|--------|-------------|
| **Politiques actives StrongFather** | Souveraineté locale | Chaque nœud définit ses propres politiques |
| **Règles de permissions locales** | Souveraineté locale | Permissions évaluées localement uniquement |
| **État système temps réel** | Volatile et local | Caring Nanny observe localement |
| **Sessions et tokens d'authentification** | Sécurité et volatilité | Gérés localement par l'Opérateur |
| **Cache et données transitoires** | Nature éphémère | Non persisté, non synchronisé |

### 6.3 Synchronisable conditionnellement

| Catégorie | Condition | Décision par |
|-----------|-----------|--------------|
| **Politiques (templates)** | Si fédération autorise le partage de politiques | Bonding Brother + Border Guard |
| **Schémas de données** | Si versions compatibles | Vérification de compatibilité Ever Buddy |
| **Règles métier (déclaratives)** | Si explicitement partagées | Configuration Opérateur |

---

## 7. Positionnement de Bonding Brother comme fédérateur

### 7.1 Rôle dans l'autonomie

Bonding Brother est le **point de contrôle unique** pour toute communication inter-nœuds. Dans un contexte d'autonomie :

- **Sans fédération :** Bonding Brother gère uniquement la médiation locale entre produits et autorités (KindMother, StrongFather).
- **Avec fédération :** Bonding Brother devient le pont de synchronisation vers les nœuds fédérés.

### 7.2 Garanties de fédération

Bonding Brother garantit que la fédération respecte les lois d'autonomie :

| Garantie | Description |
|----------|-------------|
| **Non-obligatoire** | Un nœud peut refuser toute fédération (LOI-1) |
| **Non-bloquante** | La fédération ne bloque jamais les opérations locales (LOI-2) |
| **Traçable** | Tous les échanges fédérés sont journalisés (LOI-3) |
| **Indépendante du temps global** | Les échanges utilisent des horloges logiques (LOI-4) |
| **Légère** | Les échanges sont optimisés (deltas, compression) (LOI-5) |
| **Réversible** | Un nœud peut quitter la fédération à tout moment (LOI-6) |

### 7.3 Flux de fédération

```
NŒUD A (autonome)                    NŒUD B (autonome)
      │                                    │
      ▼                                    ▼
[Bonding Brother A]  ◄──────────────►  [Bonding Brother B]
      │                                    │
      │  Échange fédéré explicite          │
      │  - Filtré par Border Guard         │
      │  - Traduit par Bonding Brother     │
      │  - Journalisé localement           │
      │                                    │
      ▼                                    ▼
[KindMother A]                       [KindMother B]
(souverain sur ses données)          (souverain sur ses données)
```

---

## 8. Impact sur les cores de l'écosystème

### 8.1 StrongFather

**Statut actuel :** Déjà compatible avec les lois d'autonomie.

**Points de conformité :**
- ✅ Décisions locales (INV-SF-1, INV-SF-2)
- ✅ Pas d'exécution (moteur pur)
- ✅ Pas de dépendance réseau
- ✅ Pas de logique temporelle technique (INV-SF-4)

**Aucune modification requise.**

### 8.2 KindMother

**Statut actuel :** Compatible si les principes offline-first sont respectés.

**Points de conformité :**
- ✅ Stockage local prioritaire (DB Fille)
- ✅ Synchronisation optionnelle
- ✅ Journaux explicites

**Points de vigilance :**
- La synchronisation Mère/Fille doit rester explicite et non-bloquante
- La résolution de conflits ne doit pas présupposer de temps global

### 8.3 Bonding Brother

**Statut actuel :** Devient stratégique pour la fédération.

**Nouveau rôle :**
- Pont de synchronisation inter-nœuds
- Contrôle des échanges fédérés
- Jamais une dépendance vitale (le système fonctionne sans fédération)

### 8.4 Caring Nanny

**Statut actuel :** Compatible avec extension requise.

**Points de conformité :**
- ✅ Observation locale
- ✅ Historique local

**Extension requise :**
- Doit reconnaître et signaler les états d'isolement (Section 5)
- Doit distinguer "isolé" de "erreur"

### 8.5 Border Guard

**Statut actuel :** Devient critique pour l'autonomie.

**Rôle renforcé :**
- Contrôle tout ce qui entre et sort du système
- Rien d'implicite dans les communications externes
- Validation explicite des échanges fédérés

### 8.6 Master Butler, Ever Buddy, TAMR

**Statut :** À évaluer lors de leur documentation fondatrice.

**Principe :** Ces cores doivent respecter les lois d'autonomie dès leur conception.

---

## 9. Question de validation architecturale

À partir de ce document, **toute décision d'architecture doit répondre à cette question** :

> *"Est-ce que ça fonctionne encore si le système est seul, lent, et isolé ?"*

- Si la réponse est **oui** → la décision est conforme.
- Si la réponse est **non** → la décision doit être reconsidérée ou explicitement justifiée comme exception.
- Si la réponse est **"ça dépend"** → la décision nécessite une clarification des conditions.

---

## 10. Avantages stratégiques de l'autonomie

L'autonomie positionne Miyukini pour des contextes que les architectures "cloud-native" ne peuvent pas servir :

| Contexte | Avantage Miyukini |
|----------|-------------------|
| **Industrie** | Fonctionnement en usine sans connexion internet fiable |
| **Terrain** | Applications mobiles en zones blanches |
| **Associatif** | Structures sans budget cloud ou connexion limitée |
| **Petites structures** | Déploiement sur hardware existant minimal |
| **Contraintes réglementaires** | Données locales, pas de cloud obligatoire |
| **Long terme** | Système fonctionnel même si le fournisseur cloud disparaît |

Ce positionnement est **délibéré et stratégique**. Miyukini ne cherche pas :
- "Scalable cloud-native"
- "Microservices partout"
- "Temps réel distribué"

Miyukini cherche :
> **Résilient, explicable, local-first, fédérable.**

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut RÉFÉRENCE**. Il établit des contraintes architecturales de premier rang qui s'appliquent à tous les composants de l'écosystème Miyukini.

Les 8 lois d'autonomie sont **non négociables**. Toute exception doit être :
1. Explicitement documentée
2. Justifiée par une contrainte technique insurmontable
3. Approuvée comme exception, pas comme règle

---

**Version :** 1.1  
**Date :** 2026-01-26  
**Statut :** RÉFÉRENCE — Contrainte architecturale normative  
**Dépendances :**
- KindMother - Documentation Fondatrice v1.1
- StrongFather - Documentation Fondatrice v1.1
- BondingBrother - Documentation Fondatrice v1.1
- Caring Nanny - Documentation Fondatrice v1.2
- Border Guard - Documentation Fondatrice v1.2
- TAMR - Documentation Fondatrice v1.1
- Ever Buddy - Documentation Fondatrice v1.1
- Master Butler - Documentation Fondatrice v1.1

**Documents de référence associés :**
- [Miyukini Conceptual References - Objectif Final](Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) : Vision synthèse et pilier 6 "Autonomie opérationnelle"
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) : Dégradation graduée conforme aux lois d'autonomie
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Internet comme signal, pas comme dépendance (LOI-1)
- [Miyukini Conceptual References - Carte Optimisation](Miyukini%20Conceptual%20References%20-%20Carte%20Optimisation.md) : Leviers d'optimisation par zone sans violer les invariants
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) : Capacités bas niveau de maintenance (observation sans correction)
- [Miyukini Conceptual References - Souveraineté Environnement](Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) : Règles de souveraineté, versioning et migration (LOI-7, LOI-8)