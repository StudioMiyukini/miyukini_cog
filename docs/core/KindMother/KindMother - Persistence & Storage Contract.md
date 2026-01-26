# KindMother — Persistence & Storage Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **KindMother — Persistence & Storage Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit ce que signifie persister des données dans KindMother, les caractéristiques conceptuelles du stockage autoritaire, et les garanties associées à la durabilité des données dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle de la persistance, les invariants de stockage, les notions de corruption et de réparation, sans jamais introduire de détail d'implémentation technique.

### Portée

Ce contrat s'applique à **toutes les opérations de persistance** dans KindMother et définit de manière absolue :
- la définition formelle du stockage autoritaire,
- la notion de durabilité conceptuelle,
- l'atomicité de persistance,
- les invariants de stockage,
- la corruption et la réparation (conceptuelle uniquement),
- les garanties de persistance offertes,
- les distinctions entre persistance de référence (Mère) et persistance locale (Fille).

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **KindMother — Instance Model Contract** : Définit les instances et leur persistance interne (INST-4, INST-M-3)
- **KindMother — CoreDataAPI Contract** : Définit les écritures appliquées qui déclenchent la persistance
- **KindMother — Runtime Boundary & Enforcement Contract** : Définit les protections contre les corruptions (I8)
- **KindMother — Authority Graph & Cross-Domain Contract** : Définit l'isolation des données par domaine
- **[Miyukini Framework — Lois Autonomie Système](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-5** (le coût doit être proportionnel au hardware) en garantissant que le stockage est optimisé pour fonctionner sur des ressources limitées (mini PC, NAS, Raspberry Pi).

Il n'introduit aucune contradiction et constitue la définition formelle de ce que signifie persister dans KindMother.

---

## 2. Définition formelle du stockage autoritaire

### Définition formelle

Le **stockage autoritaire** est le mécanisme conceptuel par lequel KindMother conserve de manière durable les données validées et appliquées, sous son autorité exclusive, garantissant leur intégrité, leur cohérence, et leur disponibilité.

### Caractéristiques formelles fondamentales

**Autorité exclusive :** Le stockage est sous l'autorité exclusive de KindMother. Aucun accès direct au stockage n'est autorisé. Toute interaction avec les données stockées DOIT passer par la CoreDataAPI.

**Non-contournabilité :** Le stockage ne peut pas être contourné. Aucun mécanisme permettant d'accéder ou de modifier les données stockées sans passer par KindMother n'est autorisé.

**Intégrité garantie :** Le stockage garantit l'intégrité des données. Une donnée stockée ne peut pas être corrompue de manière silencieuse ; toute corruption est détectable.

**Cohérence maintenue :** Le stockage maintient la cohérence des données. Les données stockées sont toujours dans un état cohérent, conformes aux contraintes validées par KindMother.

**Isolation par instance :** Chaque instance KindMother possède son propre stockage, isolé des autres instances. Les données d'une instance ne sont pas directement accessibles depuis une autre instance.

**Isolation par domaine :** Au sein d'une instance, les données sont isolées par Authority Domain. Les données d'un domaine ne sont pas directement accessibles depuis un autre domaine.

### Nature systémique

Le stockage autoritaire est un **concept systémique**, pas un mécanisme technique. Il représente la capacité conceptuelle de KindMother à conserver des données de manière durable et fiable, sous son autorité exclusive.

**Important :** Cette définition est purement conceptuelle. Elle ne présuppose aucune technologie de stockage, aucun système de fichiers, aucune base de données, ou aucun mécanisme de persistance technique.

**Conformité LOI-5 :** Le stockage autoritaire est conçu pour être proportionnel au hardware disponible. L'implémentation de référence utilise SQLite interne, optimisé pour les ressources limitées, permettant à KindMother de fonctionner efficacement sur du hardware simple (mini PC, NAS, Raspberry Pi, VM isolée).

---

## 3. Notion de durabilité conceptuelle

### Définition formelle

La **durabilité conceptuelle** est la propriété garantissant qu'une donnée validée et appliquée par KindMother survit à tout événement normal du système et reste accessible tant que l'instance existe et n'est pas explicitement supprimée.

### Caractéristiques de la durabilité

**Survie aux arrêts :** Une donnée durable survit à un arrêt normal de l'instance. Après redémarrage, la donnée est disponible dans l'état où elle a été persistée.

**Survie aux redémarrages :** Une donnée durable survit à un redémarrage de l'instance. L'état persisté est restauré de manière cohérente.

**Non-volatilité :** Une donnée durable n'est pas volatile. Elle ne disparaît pas de manière silencieuse ou non contrôlée.

**Accessibilité garantie :** Une donnée durable reste accessible tant que l'instance existe et que la donnée n'est pas supprimée par une opération valide.

**Indépendance temporelle :** La durabilité ne dépend pas du temps. Une donnée persistée il y a longtemps est aussi durable qu'une donnée persistée récemment.

### Limites de la durabilité

**Événements exceptionnels :** La durabilité ne garantit pas la survie à des événements exceptionnels destructeurs (corruption matérielle catastrophique, perte totale du support). Ces événements relèvent du domaine de la corruption et de la réparation.

**Suppression explicite :** La durabilité ne protège pas contre la suppression explicite par une opération valide de KindMother. Une donnée supprimée de manière valide n'existe plus.

**Corruption détectée :** La durabilité ne garantit pas la disponibilité d'une donnée corrompue. Une corruption détectée entraîne l'indisponibilité de la donnée jusqu'à réparation.

### Niveaux de durabilité

**Durabilité de référence (Instance Mère) :** La durabilité de référence est la durabilité absolue. Les données de l'Instance Mère constituent la source de vérité autoritaire et bénéficient de la durabilité maximale.

**Durabilité locale (Instance Fille) :** La durabilité locale est relative à l'Instance Fille. Les données de l'Instance Fille sont durables localement mais peuvent être resynchronisées avec l'Instance Mère.

**Durabilité temporaire (Instance Éphémère) :** L'Instance Éphémère ne possède pas de durabilité. Ses données sont détruites avec l'instance.

---

## 4. Atomicité de persistance

### Définition formelle

L'**atomicité de persistance** est la propriété garantissant qu'une opération de persistance est indivisible : elle est exécutée complètement ou pas du tout. Aucun état intermédiaire n'est jamais observable ou persisté.

### Caractéristiques de l'atomicité

**Tout ou rien :** Une opération de persistance applique toutes ses modifications ou aucune. Il n'existe pas de persistance partielle.

**Pas d'état intermédiaire :** Aucun état intermédiaire d'une opération de persistance n'est observable par une autre opération ou après un incident.

**Cohérence transactionnelle :** L'atomicité garantit que le stockage passe d'un état cohérent à un autre état cohérent, sans jamais être dans un état incohérent.

**Isolation des opérations :** Les opérations de persistance sont isolées les unes des autres. Une opération en cours n'est pas affectée par une autre opération concurrente.

### Portée de l'atomicité

**Opération unique :** L'atomicité s'applique à chaque opération de persistance individuelle. Une écriture appliquée est atomique.

**Opérations batch :** L'atomicité s'applique à un batch d'opérations groupées. Toutes les opérations du batch sont appliquées ensemble ou aucune n'est appliquée.

**Synchronisation :** L'atomicité s'applique aux opérations de synchronisation. Les modifications synchronisées sont appliquées de manière atomique.

### Garanties d'atomicité

**ATOM-1 :** Toute opération de persistance est atomique (tout ou rien)

**ATOM-2 :** Aucun état intermédiaire n'est jamais observable

**ATOM-3 :** En cas d'incident pendant la persistance, l'état revient à l'état précédent cohérent

**ATOM-4 :** L'atomicité est préservée même en cas de charge élevée

**ATOM-5 :** Aucune exception à l'atomicité n'est autorisée

---

## 5. Invariants de stockage

### 5.1. Invariants globaux de stockage

**INV-STOR-1 : Intégrité absolue**

Les données stockées sont toujours intègres. Si une corruption est détectée, les opérations sont bloquées jusqu'à réparation.

**INV-STOR-2 : Cohérence permanente**

Les données stockées sont toujours dans un état cohérent. Aucune opération ne peut laisser le stockage dans un état incohérent.

**INV-STOR-3 : Isolation stricte**

Les données stockées d'une instance sont strictement isolées des autres instances. Aucun accès croisé direct n'est possible.

**INV-STOR-4 : Autorité exclusive**

Seul KindMother peut accéder au stockage. Aucun accès direct externe n'est autorisé.

**INV-STOR-5 : Traçabilité complète**

Toutes les modifications du stockage sont traçables. Aucune modification silencieuse n'est autorisée.

**INV-STOR-6 : Durabilité garantie**

Une donnée validée et persistée est durable jusqu'à suppression explicite ou corruption détectée.

**INV-STOR-7 : Atomicité préservée**

Toute opération de persistance est atomique, sans exception.

### 5.2. Invariants de stockage de référence (Instance Mère)

**INV-STOR-M-1 : Source de vérité**

Le stockage de l'Instance Mère constitue la source de vérité autoritaire pour son périmètre d'autorité.

**INV-STOR-M-2 : Durabilité maximale**

Le stockage de l'Instance Mère bénéficie de la durabilité maximale. Les données de référence sont préservées avec la plus grande rigueur.

**INV-STOR-M-3 : Point de convergence**

Le stockage de l'Instance Mère est le point de convergence pour les synchronisations des Instances Filles.

**INV-STOR-M-4 : Validation définitive**

Les données validées et persistées par l'Instance Mère sont définitives. Elles constituent la référence pour toutes les Instances Filles.

### 5.3. Invariants de stockage local (Instance Fille)

**INV-STOR-F-1 : Copie locale**

Le stockage de l'Instance Fille maintient une copie locale des données, synchronisée avec l'Instance Mère.

**INV-STOR-F-2 : Autonomie opérationnelle**

Le stockage de l'Instance Fille permet un fonctionnement autonome, même en l'absence de connexion avec l'Instance Mère.

Cette garantie respecte **LOI-5** (le coût doit être proportionnel au hardware) : le stockage local de l'Instance Fille est optimisé pour fonctionner efficacement sur des ressources limitées, sans nécessiter de services distants coûteux en ressources.

**INV-STOR-F-3 : Soumission à synchronisation**

Les données du stockage de l'Instance Fille sont soumises à la validation de l'Instance Mère lors de la synchronisation.

**INV-STOR-F-4 : Cohérence avec la référence**

Le stockage de l'Instance Fille maintient une cohérence avec la source de vérité de l'Instance Mère, préservée par synchronisation.

---

## 6. Corruption et réparation

### 6.1. Définition formelle de la corruption

**Corruption :** État anormal du stockage où l'intégrité, la cohérence, ou la disponibilité des données est compromise de manière détectable.

### 6.2. Types de corruption conceptuels

**Corruption d'intégrité :** Les données stockées ne correspondent plus à ce qui a été validé et persisté. Quelque chose a altéré les données de manière non autorisée.

**Corruption de cohérence :** Les données stockées violent les contraintes de cohérence. Des invariants sont violés de manière détectable.

**Corruption de structure :** La structure du stockage est endommagée. Les données ne peuvent plus être lues ou interprétées correctement.

**Corruption partielle :** Une partie du stockage est corrompue, tandis qu'une autre partie reste intègre.

**Corruption totale :** L'ensemble du stockage est corrompu. Aucune donnée n'est récupérable directement.

### 6.3. Détection de corruption

**Détection systématique :** KindMother détecte systématiquement les corruptions lors de l'accès aux données. Aucune corruption ne peut passer inaperçue lors d'une opération.

**Détection proactive :** KindMother peut détecter proactivement les corruptions par vérification périodique de l'intégrité.

**Signalement immédiat :** Toute corruption détectée est signalée immédiatement. Aucune corruption n'est ignorée silencieusement.

### 6.4. Comportement en cas de corruption

**Blocage des opérations :** En cas de corruption détectée, toutes les opérations sur les données concernées sont bloquées. Aucune opération ne peut être exécutée sur des données corrompues.

**Signalement explicite :** La corruption est signalée de manière explicite. Les opérations rejetées indiquent clairement la raison du rejet.

**Isolation de la corruption :** La corruption est isolée. Les données non corrompues restent accessibles si elles sont isolables.

**Traçabilité de la détection :** La détection de corruption est tracée pour audit et analyse.

### 6.5. Réparation conceptuelle

**Définition :** La réparation est le processus conceptuel par lequel le stockage corrompu est restauré dans un état intègre et cohérent.

**Réparation par source de vérité :** Pour une Instance Fille, la réparation peut s'effectuer par resynchronisation avec l'Instance Mère (source de vérité).

**Réparation par restauration :** La réparation peut s'effectuer par restauration à partir d'un état antérieur connu comme intègre.

**Réparation manuelle :** Dans certains cas, la réparation nécessite une intervention manuelle sous autorité légitime.

### 6.6. Invariants de corruption

**INV-CORR-1 :** Toute corruption est détectable

**INV-CORR-2 :** Aucune opération n'est exécutée sur des données corrompues

**INV-CORR-3 :** La corruption est signalée immédiatement

**INV-CORR-4 :** Le blocage persiste jusqu'à réparation

**INV-CORR-5 :** La réparation restaure un état intègre et cohérent

**INV-CORR-6 :** La détection et la réparation sont tracées

---

## 7. Garanties de persistance

### 7.1. Garanties offertes à KindMother

**G-PERSIST-1 : Durabilité des données validées**

Toute donnée validée par KindMother et persistée est durable jusqu'à suppression explicite ou corruption détectée.

**G-PERSIST-2 : Atomicité garantie**

Toute opération de persistance est atomique. Aucune persistance partielle n'est possible.

**G-PERSIST-3 : Cohérence préservée**

Le stockage est toujours dans un état cohérent après une opération de persistance.

**G-PERSIST-4 : Intégrité protégée**

L'intégrité des données stockées est protégée. Toute altération non autorisée est détectable.

**G-PERSIST-5 : Isolation garantie**

L'isolation entre instances et entre domaines est garantie. Aucun accès croisé n'est possible.

### 7.2. Garanties offertes aux adaptateurs KM-compliant

**G-ADAPT-PERSIST-1 : Persistance prévisible**

Si un adaptateur certifié KM-compliant soumet une intention d'écriture validée, la persistance s'effectue de manière prévisible et conforme au contrat.

**G-ADAPT-PERSIST-2 : Confirmation de persistance**

Après une écriture appliquée, l'adaptateur reçoit une confirmation que la donnée est persistée et durable.

**G-ADAPT-PERSIST-3 : Erreur explicite en cas d'échec**

Si la persistance échoue, l'adaptateur reçoit une erreur explicite. Aucune persistance silencieuse ou partielle n'est possible.

**G-ADAPT-PERSIST-4 : Cohérence des lectures**

Les lectures retournent des données cohérentes avec l'état persisté au moment de la lecture.

**G-ADAPT-PERSIST-5 : Traçabilité accessible**

Les opérations de persistance sont traçables et auditables par les adaptateurs autorisés.

---

## 8. Distinction entre persistance de référence et persistance locale

### 8.1. Persistance de référence (Instance Mère)

**Rôle :** Le stockage de l'Instance Mère constitue la persistance de référence, la source de vérité autoritaire pour le périmètre d'autorité.

**Caractéristiques :**
- Durabilité maximale
- Autorité définitive sur les données
- Point de convergence pour les synchronisations
- Validations définitives

**Responsabilités :**
- Maintenir la source de vérité
- Valider les synchronisations des Instances Filles
- Préserver l'intégrité de référence
- Servir de base pour la réparation des Instances Filles

### 8.2. Persistance locale (Instance Fille)

**Rôle :** Le stockage de l'Instance Fille constitue la persistance locale, permettant un fonctionnement autonome avec synchronisation ultérieure.

**Caractéristiques :**
- Durabilité locale
- Autorité dérivée (soumise à validation Mère)
- Copie locale synchronisable
- Validations locales en attente de confirmation
- Optimisation pour ressources limitées (respecte **LOI-5** : coût proportionnel au hardware)

**Responsabilités :**
- Maintenir une copie locale cohérente
- Fonctionner de manière autonome
- Synchroniser avec l'Instance Mère
- Accepter les décisions de validation de l'Instance Mère

### 8.3. Relation entre les deux persistances

**Hiérarchie autoritaire :** La persistance de référence a autorité sur la persistance locale. En cas de conflit, la persistance de référence prime.

**Synchronisation :** La persistance locale se synchronise avec la persistance de référence pour maintenir la cohérence.

**Réparation :** La persistance de référence peut servir à réparer la persistance locale en cas de corruption.

**Indépendance opérationnelle :** La persistance locale permet un fonctionnement autonome, mais reste soumise à la persistance de référence.

---

## 9. Interaction avec les contrats existants

### 9.1. Interaction avec Instance Model Contract

**Cohérence avec INST-4 (Persistance interne) :**

Ce contrat formalise ce que signifie la "persistance interne" définie dans INST-4. La persistance est interne à chaque instance, isolée, et sous l'autorité exclusive de KindMother.

**Cohérence avec INST-M-3 (Persistance de référence) :**

Ce contrat détaille les caractéristiques de la persistance de référence de l'Instance Mère mentionnée dans INST-M-3, établissant ses propriétés de durabilité maximale et de source de vérité.

**Cohérence avec INST-8 (Protection contre les corruptions) :**

Ce contrat formalise la détection et le traitement des corruptions, aligné avec l'invariant INST-8 qui exige la protection contre les corruptions.

### 9.2. Interaction avec CoreDataAPI Contract

**Écritures appliquées :**

La persistance s'effectue lors de l'application des écritures validées via la CoreDataAPI. Ce contrat définit ce qui se passe au niveau du stockage lorsqu'une écriture est appliquée.

**Atomicité alignée :**

L'atomicité de persistance est alignée avec l'atomicité des opérations CoreDataAPI (INV-API-4). Une opération atomique produit une persistance atomique.

**Traçabilité cohérente :**

La traçabilité de persistance complète la traçabilité des opérations CoreDataAPI (G-API-8).

### 9.3. Interaction avec Runtime Boundary & Enforcement Contract

**Interdiction I8 (Continuation après corruption) :**

Ce contrat formalise le blocage des opérations en cas de corruption, aligné avec l'interdiction I8 qui interdit la continuation après une corruption détectée.

**Boundary d'instance :**

La boundary d'instance vérifie que l'instance n'est pas corrompue avant d'autoriser une opération. Ce contrat définit ce que signifie "corrompu" au niveau du stockage.

### 9.4. Interaction avec Authority Graph & Cross-Domain Contract

**Isolation par domaine :**

L'isolation des données par Authority Domain mentionnée dans l'Authority Graph Contract est formalisée au niveau du stockage. Chaque domaine a son propre périmètre de stockage isolé.

**Absence de partage direct :**

L'interdiction de partage direct entre domaines (INTERD-9) est respectée au niveau du stockage. Aucun partage direct de stockage entre domaines n'est autorisé.

---

## 10. Schémas ASCII conceptuels

### 10.1. Architecture conceptuelle du stockage

```
┌─────────────────────────────────────────────────────────────────┐
│              ARCHITECTURE CONCEPTUELLE DU STOCKAGE               │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE KINDMOTHER                           │ │
│  │                                                            │ │
│  │  ┌────────────────────────────────────────────────────┐  │ │
│  │  │         STOCKAGE AUTORITAIRE                        │  │ │
│  │  │         (sous autorité exclusive KindMother)        │  │ │
│  │  │                                                      │  │ │
│  │  │  ┌──────────────┐    ┌──────────────┐              │  │ │
│  │  │  │ DOMAINE A    │    │ DOMAINE B    │              │  │ │
│  │  │  │ (isolé)      │    │ (isolé)      │              │  │ │
│  │  │  │              │    │              │              │  │ │
│  │  │  │ ┌──────────┐ │    │ ┌──────────┐ │              │  │ │
│  │  │  │ │ Données  │ │    │ │ Données  │ │              │  │ │
│  │  │  │ │ durables │ │    │ │ durables │ │              │  │ │
│  │  │  │ │ intègres │ │    │ │ intègres │ │              │  │ │
│  │  │  │ │ cohérent │ │    │ │ cohérent │ │              │  │ │
│  │  │  │ └──────────┘ │    │ └──────────┘ │              │  │ │
│  │  │  └──────────────┘    └──────────────┘              │  │ │
│  │  │                                                      │  │ │
  │  │  │  PROPRIÉTÉS :                                        │  │ │
  │  │  │  ✓ Intégrité garantie                               │  │ │
  │  │  │  ✓ Cohérence permanente                             │  │ │
  │  │  │  ✓ Isolation stricte par domaine                    │  │ │
  │  │  │  ✓ Durabilité assurée                               │  │ │
  │  │  │  ✓ Atomicité préservée                              │  │ │
  │  │  │  ✓ Performance proportionnelle au hardware (LOI-5) │  │ │
  │  │  └────────────────────────────────────────────────────┘  │ │
│  │                                                            │ │
│  │  ACCÈS UNIQUE : via CoreDataAPI                           │ │
│  │  ✗ Aucun accès direct autorisé                           │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 10.2. Persistance de référence vs persistance locale

```
┌─────────────────────────────────────────────────────────────────┐
│     PERSISTANCE DE RÉFÉRENCE vs PERSISTANCE LOCALE              │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE MÈRE                                 │ │
│  │              (Persistance de référence)                    │ │
│  │                                                            │ │
│  │  ┌────────────────────────────────────────────────────┐  │ │
│  │  │  STOCKAGE AUTORITAIRE DE RÉFÉRENCE                  │  │ │
│  │  │                                                      │  │ │
│  │  │  • Source de vérité autoritaire                      │  │ │
│  │  │  • Durabilité maximale                              │  │ │
│  │  │  • Validations définitives                          │  │ │
│  │  │  • Point de convergence                             │  │ │
│  │  └────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ Synchronisation                     │
│                            │ (soumission / validation)           │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE FILLE                                │ │
│  │              (Persistance locale)                          │ │
│  │                                                            │ │
│  │  ┌────────────────────────────────────────────────────┐  │ │
│  │  │  STOCKAGE AUTORITAIRE LOCAL                         │  │ │
│  │  │                                                      │  │ │
│  │  │  • Copie locale synchronisée                        │  │ │
│  │  │  • Durabilité locale                                │  │ │
│  │  │  • Validations locales (en attente)                 │  │ │
│  │  │  • Autonomie opérationnelle                         │  │ │
│  │  └────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  RELATION :                                                       │
│  • Persistance de référence > Persistance locale (autorité)     │
│  • Synchronisation maintient la cohérence                        │
│  • Référence peut réparer locale en cas de corruption           │
└─────────────────────────────────────────────────────────────────┘
```

### 10.3. Atomicité de persistance

```
┌─────────────────────────────────────────────────────────────────┐
│                  ATOMICITÉ DE PERSISTANCE                        │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉTAT INITIAL (cohérent)                                   │ │
│  │  ┌─────────────────────────────────────────────────────┐  │ │
│  │  │  Données : A, B, C                                   │  │ │
│  │  │  État : COHÉRENT                                     │  │ │
│  │  └─────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ Opération de persistance            │
│                            │ (Modifier B, Ajouter D)             │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  DEUX SCÉNARIOS POSSIBLES (atomicité)                     │ │
│  │                                                            │ │
│  │  ┌─────────────────────┐    ┌─────────────────────────┐  │ │
│  │  │ SUCCÈS              │    │ ÉCHEC                    │  │ │
│  │  │ (tout appliqué)     │    │ (rien appliqué)          │  │ │
│  │  │                     │    │                          │  │ │
│  │  │ Données : A, B', C, D│    │ Données : A, B, C       │  │ │
│  │  │ État : COHÉRENT     │    │ État : COHÉRENT         │  │ │
│  │  │                     │    │ (inchangé)              │  │ │
│  │  └─────────────────────┘    └─────────────────────────┘  │ │
│  │                                                            │ │
│  │  ✓ Jamais d'état intermédiaire                            │ │
│  │  ✓ Jamais de persistance partielle                        │ │
│  │  ✓ Toujours cohérent après l'opération                   │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 10.4. Corruption et réparation

```
┌─────────────────────────────────────────────────────────────────┐
│                  CORRUPTION ET RÉPARATION                        │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉTAT NORMAL                                               │ │
│  │  ┌─────────────────────────────────────────────────────┐  │ │
│  │  │  Stockage : INTÈGRE, COHÉRENT, DISPONIBLE           │  │ │
│  │  │  Opérations : AUTORISÉES                            │  │ │
│  │  └─────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ Corruption détectée                 │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉTAT CORROMPU                                             │ │
│  │  ┌─────────────────────────────────────────────────────┐  │ │
│  │  │  Stockage : CORROMPU                                 │  │ │
│  │  │  Opérations : BLOQUÉES                               │  │ │
│  │  │  Signalement : IMMÉDIAT                              │  │ │
│  │  │  Traçabilité : ENREGISTRÉE                           │  │ │
│  │  └─────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ Réparation                          │
│                            │ (resync / restauration)            │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉTAT RÉPARÉ                                               │ │
│  │  ┌─────────────────────────────────────────────────────┐  │ │
│  │  │  Stockage : INTÈGRE, COHÉRENT, DISPONIBLE           │  │ │
│  │  │  Opérations : AUTORISÉES                            │  │ │
│  │  │  Réparation : TRACÉE                                │  │ │
│  │  └─────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  INVARIANT : Aucune opération sur données corrompues            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 11. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable ce que signifie persister des données dans KindMother.

Il garantit que :
- le stockage est sous l'autorité exclusive de KindMother,
- les données persistées sont durables, intègres, et cohérentes,
- les opérations de persistance sont atomiques,
- les corruptions sont détectées et traitées,
- la distinction entre persistance de référence et locale est claire,
- le modèle mono-domaine reste valide.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, KindMother Instance Model Contract, KindMother CoreDataAPI Contract, KindMother Runtime Boundary & Enforcement Contract  
**Type :** Contrat de persistance et stockage non négociable

---

## 12. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Distinction entre durabilité et disponibilité

**Ambiguïté rencontrée :** Risque de confondre durabilité (données persistantes dans le temps) et disponibilité (données accessibles à un instant donné).

**Décision prise :** Clarification explicite que la durabilité garantit la survie des données aux arrêts/redémarrages, tandis que la disponibilité peut être temporairement compromise (corruption détectée, maintenance). Section 3 rédigée avec cette distinction.

**Correction effectuée :** Section 3 "Notion de durabilité conceptuelle" inclut les limites de durabilité, notamment la non-garantie de disponibilité en cas de corruption.

### Ambiguïté A2 : Atomicité vs cohérence transactionnelle

**Ambiguïté rencontrée :** Nécessité de clarifier que l'atomicité de persistance est un concept distinct de la cohérence transactionnelle au sens ACID.

**Décision prise :** L'atomicité de persistance est définie comme la propriété "tout ou rien" sans référence technique aux transactions ACID. Le concept est purement systémique.

**Correction effectuée :** Section 4 rédigée avec une définition conceptuelle de l'atomicité, sans référence à des mécanismes transactionnels techniques.

### Ambiguïté A3 : Corruption détectable vs corruption silencieuse

**Ambiguïté rencontrée :** Nécessité de clarifier que toute corruption DOIT être détectable, sans présupposer de mécanisme technique de détection.

**Décision prise :** La corruption est définie comme un "état anormal détectable". L'invariant INV-CORR-1 établit que toute corruption est détectable, sans spécifier comment.

**Correction effectuée :** Section 6 rédigée avec la définition conceptuelle de la corruption et l'invariant de détectabilité.

### Ambiguïté A4 : Réparation sans mécanisme technique

**Ambiguïté rencontrée :** Comment définir la réparation sans introduire de mécanismes techniques (backup, restore, etc.) ?

**Décision prise :** La réparation est définie comme "le processus conceptuel par lequel le stockage corrompu est restauré dans un état intègre". Trois approches conceptuelles sont mentionnées (source de vérité, restauration, intervention manuelle) sans détails techniques.

**Correction effectuée :** Section 6.5 rédigée avec des approches conceptuelles de réparation.

### Vérification de compatibilité

**Vérification effectuée :**
- ✅ Cohérence avec INST-4 (Persistance interne) : Confirmée
- ✅ Cohérence avec INST-M-3 (Persistance de référence) : Confirmée
- ✅ Cohérence avec INST-8 (Protection contre corruptions) : Confirmée
- ✅ Cohérence avec INV-API-4 (Atomicité) : Confirmée
- ✅ Cohérence avec I8 (Pas de continuation après corruption) : Confirmée
- ✅ Aucune autorité implicite créée : Confirmée
- ✅ Zero-trust respecté : Confirmée
- ✅ Aucune dépendance technique : Confirmée

**Conclusion :** Aucune contradiction détectée avec les contrats existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
