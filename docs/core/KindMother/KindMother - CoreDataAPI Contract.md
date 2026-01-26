# KindMother — CoreDataAPI Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **KindMother — CoreDataAPI Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la surface d'appel unique et autorisée entre les adaptateurs produits et KindMother, constituant l'unique point d'entrée légal vers la lecture, l'écriture, la synchronisation et l'inspection des données dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle de la CoreDataAPI, les opérations autorisées, les règles d'appel, les garanties offertes, et les interactions avec les autres composants contractuels du système.

### Portée

Ce contrat s'applique à **tous les adaptateurs produits** interagissant avec KindMother et définit de manière absolue :
- la définition formelle de la CoreDataAPI et son rôle systémique,
- le principe d'unicité de la surface d'appel,
- la typologie conceptuelle des opérations autorisées,
- les différences formelles entre lecture, intention d'écriture, et écriture appliquée,
- ce que la CoreDataAPI PEUT et NE PEUT JAMAIS faire,
- les règles absolues d'appel et de rejet,
- les garanties offertes aux adaptateurs KM-compliant,
- les invariants systémiques associés.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **KindMother — Instance Model Contract** : Modèle conceptuel systémique des instances
- **KindMother — Runtime Boundary & Enforcement Contract** : Frontières runtime et enforcement dynamique
- **KindMother — Authority Graph & Cross-Domain Contract** : Structure graphique des autorités et relations cross-domain
- **KindMother — Identity & Cross-Domain Trust Contract** : Identité et confiance inter-domaines
- **KM Adapter Compliance Contract** : Obligations statiques des adaptateurs
- **[Miyukini Framework — Lois Autonomie Système](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-1** (aucune dépendance externe critique) en garantissant que toutes les opérations fonctionnent sans appel externe obligatoire, et **LOI-2** (le système accepte l'isolement comme état normal) en permettant aux opérations de fonctionner localement même sans connexion à l'Instance Mère.

Il n'introduit aucune contradiction, et constitue la définition formelle de la surface d'appel qui traverse les Runtime Boundaries définies dans le Runtime Boundary & Enforcement Contract.

---

## 2. Rôle et nature de la CoreDataAPI

### Définition formelle

La **CoreDataAPI** est la surface d'appel conceptuelle unique et autorisée qui constitue l'interface formelle entre les adaptateurs produits et KindMother. Elle représente l'ensemble des opérations conceptuelles que KindMother expose aux adaptateurs pour interagir avec les données du système.

### Caractéristiques formelles fondamentales

**Surface d'appel unique :** La CoreDataAPI est l'unique point d'entrée vers KindMother. Aucune autre surface d'appel n'existe et aucune autre méthode d'interaction n'est autorisée. Toute interaction avec les données DOIT passer par la CoreDataAPI.

**Interface conceptuelle :** La CoreDataAPI est une interface conceptuelle, pas une implémentation technique. Elle définit les opérations autorisées de manière abstraite, sans présupposer aucune technologie, aucun protocole, ou aucun format de données.

**Point de contrôle autoritaire :** La CoreDataAPI est le point de contrôle où KindMother exerce son autorité exclusive sur toutes les opérations. Chaque appel traverse les Runtime Boundaries avant exécution.

**Médiation obligatoire :** Toute opération sur les données d'une instance KindMother DOIT passer par la CoreDataAPI. Aucun accès direct aux données n'est autorisé. La CoreDataAPI est la seule médiation entre les adaptateurs et les données.

Cette garantie respecte **LOI-1** (aucune dépendance externe critique) : en centralisant toutes les opérations via la CoreDataAPI, KindMother garantit que toutes les opérations fonctionnent localement sans nécessiter d'appels externes obligatoires.

**Abstraction de l'implémentation :** La CoreDataAPI abstrait complètement l'implémentation interne de KindMother. Les adaptateurs interagissent avec des concepts, pas avec des mécanismes techniques.

### Nature systémique

La CoreDataAPI est un **concept systémique**, pas une interface technique. Elle représente la frontière conceptuelle entre le monde externe (adaptateurs) et le monde interne (KindMother). Cette frontière est inviolable et non négociable.

**Important :** Cette définition est purement conceptuelle et systémique. Elle ne présuppose aucune technologie, aucun langage de programmation, aucun protocole de communication, ou aucun format d'échange.

---

## 3. Principe d'unicité de la surface d'appel

### Énoncé formel

La CoreDataAPI constitue l'**unique surface d'appel** vers KindMother. Aucune autre surface d'appel n'existe, n'est autorisée, ou ne peut être créée.

### Caractéristiques du principe d'unicité

**Unicité absolue :** Il n'existe qu'une seule CoreDataAPI. Aucune surface d'appel alternative, parallèle, ou de contournement n'est autorisée.

**Exclusivité totale :** Toute opération sur les données DOIT passer par la CoreDataAPI. Aucune exception n'est autorisée, même pour des raisons d'optimisation, de performance, ou de commodité.

**Non-contournabilité :** La CoreDataAPI ne peut pas être contournée. Aucun mécanisme permettant d'accéder aux données sans passer par la CoreDataAPI n'est autorisé.

**Centralisation du contrôle :** L'unicité de la surface d'appel garantit que tout contrôle, toute validation, et tout enforcement sont centralisés au point d'entrée unique.

### Implications du principe d'unicité

**Contrôle absolu :** KindMother a un contrôle absolu sur toutes les interactions avec les données, car toutes passent par un point unique.

**Traçabilité complète :** Toutes les opérations sont traçables, car elles passent par un point unique où la traçabilité est garantie.

**Sécurité renforcée :** La sécurité est renforcée par l'absence de chemins alternatifs qui pourraient contourner les validations.

**Cohérence garantie :** La cohérence est garantie, car toutes les opérations sont validées au même point de contrôle.

### Non-négociabilités

- **UNIQ-1 :** La CoreDataAPI est l'unique surface d'appel vers KindMother
- **UNIQ-2 :** Aucune surface d'appel alternative n'est autorisée
- **UNIQ-3 :** Toute opération DOIT passer par la CoreDataAPI
- **UNIQ-4 :** Aucun contournement n'est autorisé
- **UNIQ-5 :** Aucune exception n'est autorisée

---

## 4. Définition conceptuelle d'une opération CoreDataAPI

### Définition formelle

Une **opération CoreDataAPI** est une demande d'action conceptuelle formulée par un adaptateur à destination de KindMother, accompagnée d'un contexte complet, et soumise à validation avant exécution.

### Caractéristiques formelles d'une opération

**Demande d'action :** Une opération CoreDataAPI est une demande d'action sur les données. Elle exprime une intention (lecture, écriture, synchronisation, inspection) que l'adaptateur souhaite que KindMother exécute.

**Contexte complet :** Chaque opération CoreDataAPI est accompagnée d'un contexte complet qui inclut :
- le contexte utilisateur (identité de l'appelant),
- le contexte d'autorisation (permissions et règles applicables),
- le contexte d'instance (instance cible de l'opération),
- le contexte d'exécution (mode online/offline, état de synchronisation).

**Soumission à validation :** Chaque opération CoreDataAPI est soumise à validation par KindMother avant exécution. La validation traverse toutes les Runtime Boundaries définies dans le Runtime Boundary & Enforcement Contract.

**Atomicité conceptuelle :** Une opération CoreDataAPI est atomique conceptuellement. Elle est exécutée complètement ou pas du tout. Aucune exécution partielle n'est autorisée.

**Traçabilité obligatoire :** Chaque opération CoreDataAPI est tracée de manière complète, permettant l'audit et le debugging.

### Structure conceptuelle d'une opération

Conceptuellement, une opération CoreDataAPI comprend :
- **Type d'opération :** la catégorie de l'opération (lecture, écriture, synchronisation, inspection)
- **Paramètres :** les données et références nécessaires à l'exécution de l'opération
- **Contexte :** l'ensemble des informations contextuelles requises
- **Résultat attendu :** le type de résultat que l'opération retourne

### Nature conceptuelle

Une opération CoreDataAPI est un **concept systémique**, pas un appel technique. Elle représente une demande d'action conceptuelle qui sera validée et potentiellement exécutée par KindMother.

**Important :** Cette définition est purement conceptuelle. Elle ne présuppose aucune signature technique, aucun format de paramètres, ou aucune structure de données.

---

## 5. Typologie des opérations autorisées

### 5.1. Opérations de lecture

**Définition formelle :**

Une **opération de lecture** est une opération CoreDataAPI qui récupère des données sans les modifier. Elle permet à un adaptateur de consulter l'état des données d'une instance.

**Caractéristiques :**

- **Non-modification :** Une opération de lecture ne modifie jamais l'état des données. Elle est strictement consultative.
- **Contexte requis :** Une opération de lecture nécessite un contexte complet, incluant les permissions de lecture.
- **Validation des permissions :** Les permissions de lecture sont validées avant exécution. Seules les données autorisées sont retournées.
- **Cohérence garantie :** Les données lues sont cohérentes avec l'état de l'instance au moment de la lecture.
- **Isolation :** Une opération de lecture est isolée des autres opérations concurrentes.

**Sous-types conceptuels :**

- Lecture d'entité unique (par identifiant)
- Lecture de collection (avec filtres et pagination)
- Lecture de relation (entités liées)
- Lecture d'état (statut de l'instance ou de la synchronisation)

### 5.2. Opérations d'écriture (intention)

**Définition formelle :**

Une **opération d'écriture (intention)** est une opération CoreDataAPI qui soumet une intention de modification des données à KindMother pour validation et application.

**Caractéristiques :**

- **Expression d'intention :** Une opération d'écriture exprime une intention de modification, pas une modification directe. L'intention est validée avant application.
- **Contexte requis :** Une opération d'écriture nécessite un contexte complet, incluant les permissions d'écriture.
- **Validation complète :** L'intention est validée de manière complète (contexte, permissions, cohérence) avant application.
- **Atomicité :** Une opération d'écriture est atomique. L'intention est appliquée complètement ou pas du tout.
- **Traçabilité :** Chaque intention d'écriture est tracée, qu'elle soit validée ou rejetée.

**Sous-types conceptuels :**

- Création d'entité (nouvelle entité)
- Modification d'entité (mise à jour d'une entité existante)
- Suppression d'entité (suppression logique ou physique)
- Création de relation (lien entre entités)
- Suppression de relation (suppression d'un lien)

### 5.3. Opérations d'écriture batch

**Définition formelle :**

Une **opération d'écriture batch** est une opération CoreDataAPI qui soumet plusieurs intentions de modification groupées pour validation et application atomique.

**Caractéristiques :**

- **Groupement d'intentions :** Une opération batch groupe plusieurs intentions de modification en une seule opération logique.
- **Atomicité globale :** Toutes les intentions du batch sont appliquées ensemble ou aucune n'est appliquée. Il n'y a pas d'application partielle.
- **Contexte partagé :** Toutes les intentions du batch partagent le même contexte d'exécution.
- **Validation séquentielle ou parallèle :** Les intentions du batch peuvent être validées séquentiellement ou en parallèle, mais l'application est atomique.
- **Cohérence transactionnelle :** Le batch garantit la cohérence transactionnelle de toutes les intentions groupées.

**Contraintes :**

- Les intentions d'un batch DOIVENT être cohérentes entre elles
- Les intentions d'un batch DOIVENT cibler la même instance
- Le batch NE PEUT PAS contenir d'intentions contradictoires

### 5.4. Opérations de synchronisation

**Définition formelle :**

Une **opération de synchronisation** est une opération CoreDataAPI qui gère la synchronisation de données entre instances (Instance Mère et Instance Fille).

**Caractéristiques :**

- **Coordination entre instances :** Une opération de synchronisation coordonne l'échange de données entre instances selon les règles de la hiérarchie autoritaire.
- **Direction de la synchronisation :** La synchronisation peut être de l'Instance Fille vers l'Instance Mère (soumission) ou de l'Instance Mère vers l'Instance Fille (propagation).
- **Validation par l'Instance Mère :** Lors de la synchronisation Fille → Mère, l'Instance Mère valide les opérations soumises avec autorité définitive.
- **Cohérence garantie :** La synchronisation garantit la cohérence entre les instances après exécution.
- **Gestion des conflits :** La synchronisation gère les conflits selon les règles définies (l'Instance Mère a l'autorité définitive).

**Sous-types conceptuels :**

- Synchronisation de soumission (Fille → Mère)
- Synchronisation de propagation (Mère → Fille)
- Synchronisation complète (bidirectionnelle)
- Synchronisation incrémentale (différences uniquement)

### 5.5. Opérations d'inspection / statut

**Définition formelle :**

Une **opération d'inspection** est une opération CoreDataAPI qui permet de consulter l'état systémique d'une instance ou de la synchronisation sans accéder aux données métier.

**Caractéristiques :**

- **Consultation d'état :** Une opération d'inspection consulte l'état systémique, pas les données métier.
- **Non-modification :** Une opération d'inspection ne modifie jamais l'état.
- **Contexte minimal :** Une opération d'inspection peut nécessiter un contexte minimal, selon le niveau d'inspection.
- **Informations systémiques :** Les informations retournées sont de nature systémique (état de synchronisation, santé de l'instance, etc.).

**Sous-types conceptuels :**

- Inspection de l'état de synchronisation
- Inspection de la santé de l'instance
- Inspection des opérations en attente
- Inspection des conflits non résolus
- Inspection de la cohérence

---

## 6. Différence formelle entre lecture, intention d'écriture, et écriture appliquée

### 6.1. Lecture

**Définition formelle :**

Une **lecture** est une opération qui récupère des données existantes sans les modifier. Elle est strictement consultative et n'a aucun effet sur l'état des données.

**Caractéristiques formelles :**

- **Nature :** Consultation pure, sans effet
- **État des données :** Inchangé après l'opération
- **Résultat :** Données lues (ou erreur explicite)
- **Traçabilité :** Tracée pour audit
- **Permissions requises :** Permissions de lecture sur les données demandées

**Garanties :**

- Les données retournées reflètent l'état au moment de la lecture
- Aucune modification n'est effectuée, même en cas d'erreur
- La lecture est isolée des écritures concurrentes

### 6.2. Intention d'écriture

**Définition formelle :**

Une **intention d'écriture** est une demande de modification formulée par un adaptateur, soumise à KindMother pour validation. Elle exprime ce que l'adaptateur souhaite modifier, mais n'est pas encore appliquée.

**Caractéristiques formelles :**

- **Nature :** Demande de modification, pas encore appliquée
- **État des données :** Inchangé tant que l'intention n'est pas validée et appliquée
- **Résultat :** Acceptation ou rejet de l'intention
- **Traçabilité :** Tracée avec le résultat de validation
- **Permissions requises :** Permissions d'écriture sur les données ciblées

**Garanties :**

- L'intention est validée avant toute application
- Si l'intention est rejetée, l'état reste inchangé
- L'intention est distincte de l'application

### 6.3. Écriture appliquée

**Définition formelle :**

Une **écriture appliquée** est une intention d'écriture qui a été validée par KindMother et appliquée aux données. Elle représente la modification effective de l'état des données.

**Caractéristiques formelles :**

- **Nature :** Modification effective et définitive
- **État des données :** Modifié de manière atomique
- **Résultat :** Confirmation de l'application
- **Traçabilité :** Tracée comme opération complétée
- **Définitivité :** L'écriture appliquée est définitive (sauf nouvelle intention de modification)

**Garanties :**

- L'écriture appliquée est atomique (tout ou rien)
- L'état après application est cohérent
- L'écriture appliquée est traçable et auditable

### 6.4. Flux conceptuel : intention → validation → application

```
┌─────────────────────────────────────────────────────────────┐
│              FLUX CONCEPTUEL D'ÉCRITURE                      │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     1. INTENTION D'ÉCRITURE                          │  │
│  │     Formulée par l'adaptateur                        │  │
│  │     Exprime la modification souhaitée                │  │
│  │     Accompagnée d'un contexte complet                │  │
│  │                                                       │  │
│  │     État des données : INCHANGÉ                      │  │
│  └──────────────────────────────────────────────────────┘  │
│                        │                                     │
│                        ▼                                     │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     2. VALIDATION PAR KINDMOTHER                     │  │
│  │     Traverse les Runtime Boundaries                  │  │
│  │     Vérifie contexte, permissions, cohérence        │  │
│  │                                                       │  │
│  │     Résultat : ACCEPTATION ou REJET                  │  │
│  │     Si rejet → État des données : INCHANGÉ           │  │
│  └──────────────────────────────────────────────────────┘  │
│                        │                                     │
│                        ▼ (si acceptée)                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     3. ÉCRITURE APPLIQUÉE                            │  │
│  │     Intention validée et appliquée                   │  │
│  │     Modification effective et atomique               │  │
│  │                                                       │  │
│  │     État des données : MODIFIÉ                       │  │
│  │     Modification : DÉFINITIVE                        │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  PRINCIPE : Aucune modification sans validation préalable   │
└─────────────────────────────────────────────────────────────┘
```

### 6.5. Règles absolues

- **R-DIFF-1 :** Une lecture NE MODIFIE JAMAIS l'état des données
- **R-DIFF-2 :** Une intention d'écriture N'EST PAS une écriture appliquée
- **R-DIFF-3 :** Une intention d'écriture DOIT être validée avant application
- **R-DIFF-4 :** Une écriture appliquée est DÉFINITIVE et ATOMIQUE
- **R-DIFF-5 :** Si la validation échoue, l'état reste INCHANGÉ
- **R-DIFF-6 :** Aucune exception n'est autorisée à ces règles

---

## 7. Ce que la CoreDataAPI PEUT faire

### 7.1. Opérations autorisées

La CoreDataAPI PEUT effectuer les opérations suivantes :

**PEUT-1 : Lire des données**

La CoreDataAPI PEUT lire des données d'une instance, sous réserve que le contexte soit valide et que les permissions de lecture soient suffisantes.

**PEUT-2 : Soumettre des intentions d'écriture**

La CoreDataAPI PEUT soumettre des intentions d'écriture pour validation par KindMother, sous réserve que le contexte soit valide et que les permissions d'écriture soient suffisantes.

**PEUT-3 : Soumettre des intentions d'écriture batch**

La CoreDataAPI PEUT soumettre des intentions d'écriture groupées (batch) pour validation et application atomique, sous réserve de cohérence du batch.

**PEUT-4 : Déclencher des opérations de synchronisation**

La CoreDataAPI PEUT déclencher des opérations de synchronisation entre instances, sous réserve que le contexte de synchronisation soit valide et que les instances soient dans un état permettant la synchronisation.

**PEUT-5 : Inspecter l'état systémique**

La CoreDataAPI PEUT inspecter l'état systémique d'une instance (synchronisation, santé, opérations en attente), sous réserve que le contexte d'inspection soit valide.

**PEUT-6 : Retourner des erreurs explicites**

La CoreDataAPI PEUT retourner des erreurs explicites et actionnables lorsqu'une opération ne peut pas être exécutée, permettant à l'adaptateur de comprendre et corriger le problème.

**PEUT-7 : Appliquer des écritures validées**

La CoreDataAPI PEUT appliquer des intentions d'écriture validées de manière atomique, modifiant l'état des données de manière définitive.

### 7.2. Garanties associées

Chaque opération autorisée est accompagnée des garanties suivantes :
- Validation complète avant exécution
- Atomicité de l'opération
- Traçabilité complète
- Erreur explicite en cas de rejet
- Cohérence préservée après exécution

---

## 8. Ce que la CoreDataAPI NE PEUT JAMAIS faire

### 8.1. Interdictions absolues

La CoreDataAPI NE PEUT JAMAIS effectuer les actions suivantes. Ces interdictions sont absolues et non négociables.

**INTERDIT-1 : Contourner les validations**

La CoreDataAPI NE PEUT JAMAIS contourner les validations de KindMother, même pour des raisons d'optimisation, de performance, ou de commodité. Toute opération DOIT être validée.

**INTERDIT-2 : Exposer les données directement**

La CoreDataAPI NE PEUT JAMAIS exposer les données directement sans passer par les mécanismes de contrôle de KindMother. Aucun accès direct à la persistance n'est autorisé.

Cette interdiction respecte **LOI-1** (aucune dépendance externe critique) : en interdisant l'accès direct à la persistance, KindMother garantit que toutes les opérations sont gérées localement sans créer de dépendances externes critiques.

**INTERDIT-3 : Exécuter une opération sans contexte complet**

La CoreDataAPI NE PEUT JAMAIS exécuter une opération sans contexte complet. Chaque opération DOIT être accompagnée de tous les éléments contextuels requis.

**INTERDIT-4 : Appliquer une écriture non validée**

La CoreDataAPI NE PEUT JAMAIS appliquer une intention d'écriture qui n'a pas été validée par KindMother. L'application ne peut suivre que la validation réussie.

**INTERDIT-5 : Exécuter partiellement une opération**

La CoreDataAPI NE PEUT JAMAIS exécuter partiellement une opération. Chaque opération est atomique : tout ou rien.

**INTERDIT-6 : Ignorer une erreur de validation**

La CoreDataAPI NE PEUT JAMAIS ignorer une erreur de validation ou continuer après un rejet. Toute erreur DOIT être retournée à l'appelant.

**INTERDIT-7 : Modifier l'état après un rejet**

La CoreDataAPI NE PEUT JAMAIS modifier l'état des données après avoir rejeté une opération. L'état DOIT rester inchangé après un rejet.

**INTERDIT-8 : Déléguer la validation à l'adaptateur**

La CoreDataAPI NE PEUT JAMAIS déléguer la responsabilité de validation à un adaptateur. La validation est exclusive à KindMother.

**INTERDIT-9 : Exposer des détails d'implémentation**

La CoreDataAPI NE PEUT JAMAIS exposer des détails d'implémentation interne dans les résultats ou les erreurs. L'abstraction DOIT être préservée.

**INTERDIT-10 : Opérer sur une instance corrompue**

La CoreDataAPI NE PEUT JAMAIS exécuter une opération sur une instance détectée comme corrompue. Toutes les opérations sont bloquées jusqu'à réparation.

**INTERDIT-11 : Permettre une communication inter-domaines directe**

La CoreDataAPI NE PEUT JAMAIS permettre une communication directe entre Authority Domains. Toute communication inter-domaines DOIT passer par des Intentions Certifiées validées par KindMother.

**INTERDIT-12 : Accorder une confiance implicite**

La CoreDataAPI NE PEUT JAMAIS accorder une confiance implicite à un adaptateur, même certifié KM-compliant. Le principe de zero-trust s'applique à chaque appel.

### 8.2. Justifications

Ces interdictions sont justifiées par :
- la préservation de l'intégrité du système,
- le maintien de l'autorité exclusive de KindMother,
- la garantie de la cohérence des données,
- la protection contre les corruptions et les contournements,
- le respect du principe de zero-trust.

---

## 9. Règles absolues d'appel (préconditions)

### 9.1. Préconditions obligatoires

Chaque appel CoreDataAPI DOIT respecter les préconditions suivantes. Si une précondition n'est pas satisfaite, l'appel est rejeté immédiatement.

**PRECOND-1 : Contexte complet obligatoire**

Chaque appel CoreDataAPI DOIT être accompagné d'un contexte complet incluant :
- le contexte utilisateur (identité),
- le contexte d'autorisation (permissions),
- le contexte d'instance (instance cible),
- le contexte d'exécution (mode, état de synchronisation).

**PRECOND-2 : Instance valide obligatoire**

L'instance cible de l'opération DOIT être valide, accessible, et dans un état permettant l'opération. Une instance corrompue, verrouillée, ou en maintenance ne peut pas recevoir d'opérations.

**PRECOND-3 : Permissions suffisantes obligatoires**

Les permissions fournies dans le contexte DOIVENT être suffisantes pour l'opération demandée. Les permissions sont évaluées selon les règles du domaine d'autorité.

**PRECOND-4 : Authority Domain valide obligatoire**

L'Authority Domain associé à l'opération DOIT être valide et accessible. L'opération s'exécute dans le périmètre d'autorité du domaine spécifié.

**PRECOND-5 : Paramètres valides obligatoires**

Les paramètres de l'opération DOIVENT être valides, complets, et conformes aux attentes de l'opération. Les paramètres invalides entraînent un rejet.

**PRECOND-6 : Appel légal obligatoire**

L'opération demandée DOIT être une opération légale et documentée de la CoreDataAPI. Les appels à des opérations inexistantes ou obsolètes sont rejetés.

**PRECOND-7 : Cohérence d'intention obligatoire (pour les écritures)**

Pour les opérations d'écriture, l'intention DOIT être cohérente avec l'état actuel des données et ne DOIT PAS violer les contraintes de cohérence.

### 9.2. Règles de validation des préconditions

- Les préconditions sont validées dans l'ordre des Runtime Boundaries
- Si une précondition échoue, l'appel est rejeté immédiatement
- L'erreur de rejet indique la précondition non satisfaite
- Aucune exécution partielle n'est autorisée après un échec de précondition

---

## 10. Règles absolues de rejet

### 10.1. Conditions de rejet

Un appel CoreDataAPI est rejeté si l'une des conditions suivantes est détectée :

**REJET-1 : Contexte invalide**

L'appel est rejeté si le contexte est invalide, incomplet, ou incohérent.
- Erreur retournée : indication de contexte invalide
- État des données : inchangé
- Traçabilité : violation tracée

**REJET-2 : Permissions insuffisantes**

L'appel est rejeté si les permissions sont insuffisantes pour l'opération demandée.
- Erreur retournée : indication de permission insuffisante
- État des données : inchangé
- Traçabilité : tentative tracée

**REJET-3 : Instance invalide**

L'appel est rejeté si l'instance cible est invalide, inaccessible, ou corrompue.
- Erreur retournée : indication d'instance invalide
- État des données : inchangé
- Traçabilité : violation tracée

**REJET-4 : Appel illégal**

L'appel est rejeté si l'opération demandée est illégale, inexistante, ou obsolète.
- Erreur retournée : indication d'appel invalide
- État des données : inchangé
- Traçabilité : violation tracée

**REJET-5 : Cohérence compromise**

L'appel est rejeté si l'opération compromettrait la cohérence des données.
- Erreur retournée : indication de cohérence compromise
- État des données : inchangé
- Traçabilité : violation tracée

**REJET-6 : Tentative de contournement détectée**

L'appel est rejeté si une tentative de contournement des validations est détectée.
- Erreur retournée : indication de tentative de contournement
- État des données : inchangé
- Traçabilité : violation tracée
- Conséquence : mise en quarantaine potentielle

**REJET-7 : Charge excessive**

L'appel est rejeté ou neutralisé si la charge est excessive.
- Erreur retournée : indication de charge excessive
- État des données : inchangé ou partiellement traité
- Traçabilité : violation tracée
- Conséquence : dégradation contrôlée potentielle

### 10.2. Garanties après rejet

Après tout rejet, les garanties suivantes s'appliquent :
- L'état des données reste inchangé
- Aucune modification partielle n'est appliquée
- L'erreur est explicite et actionnable
- La violation est tracée pour audit
- Aucun effet de bord n'est créé

### 10.3. Règles absolues

- **R-REJ-1 :** Tout rejet laisse l'état inchangé
- **R-REJ-2 :** Tout rejet retourne une erreur explicite
- **R-REJ-3 :** Tout rejet est tracé
- **R-REJ-4 :** Aucune exception au rejet n'est autorisée
- **R-REJ-5 :** Un rejet ne déclenche jamais d'exécution partielle

---

## 11. Garanties offertes aux adaptateurs KM-compliant

### 11.1. Garanties de traitement

**G-API-1 : Traitement prévisible des opérations valides**

Si un adaptateur certifié KM-compliant fournit un contexte valide et effectue des appels légaux, KindMother traite les opérations de manière prévisible et conforme au contrat CoreDataAPI.

**G-API-2 : Messages d'erreur explicites et actionnables**

Si une opération est rejetée, KindMother retourne toujours un message d'erreur explicite et actionnable qui permet à l'adaptateur de comprendre et corriger le problème, sans révéler de détails internes.

**G-API-3 : Pas de rejet arbitraire**

KindMother ne rejette jamais une opération de manière arbitraire. Tout rejet est justifié par une violation de précondition ou une condition de rejet documentée.

**G-API-4 : Atomicité garantie**

Toute opération CoreDataAPI est atomique. Elle est exécutée complètement ou pas du tout. Aucune exécution partielle n'est autorisée.

### 11.2. Garanties de cohérence

**G-API-5 : Cohérence après exécution**

Après toute opération réussie, l'état des données est cohérent et conforme aux contraintes de cohérence.

**G-API-6 : État inchangé après rejet**

Après tout rejet, l'état des données reste inchangé. Aucune modification partielle n'est laissée.

**G-API-7 : Isolation des opérations**

Les opérations sont isolées les unes des autres. Une opération ne peut pas interférer avec une autre opération concurrente de manière non contrôlée.

### 11.3. Garanties de traçabilité

**G-API-8 : Traçabilité complète**

Toutes les opérations sont tracées de manière complète, permettant l'audit et le debugging.

**G-API-9 : Résultats traçables**

Les résultats de chaque opération (succès ou échec) sont traçables et auditables.

### 11.4. Garanties de disponibilité

**G-API-10 : Dégradation contrôlée**

En cas de charge excessive, KindMother applique une dégradation contrôlée et réversible, préservant l'intégrité.

**G-API-12 : Offline-first**

Toutes les opérations CoreDataAPI fonctionnent en mode offline. Une Instance Fille peut exécuter toutes les opérations (lecture, écriture, inspection) sans connexion à l'Instance Mère. Les opérations de synchronisation gèrent les périodes de déconnexion de manière transparente.

Cette garantie respecte **LOI-1** (aucune dépendance externe critique) et **LOI-2** (le système accepte l'isolement comme état normal) : toutes les opérations fonctionnent localement sans dépendance externe, et l'isolement est un état normal du système, pas une erreur.

**G-API-11 : Pas de quarantaine sans violation répétée**

KindMother ne met pas en quarantaine un adaptateur certifié KM-compliant sans violation répétée ou violation de sécurité critique.

### 11.5. Non-négociabilité

Ces garanties sont absolues et non négociables. Elles s'appliquent à tous les adaptateurs certifiés KM-compliant, sans exception.

---

## 12. Interaction avec les contrats existants

### 12.1. Interaction avec Runtime Boundary & Enforcement Contract

**Relation formelle :**

Chaque appel CoreDataAPI traverse les Runtime Boundaries définies dans le Runtime Boundary & Enforcement Contract. La CoreDataAPI constitue le point d'entrée vers ces boundaries.

**Points d'interaction :**

- **Boundary d'appel :** Vérifie que l'appel CoreDataAPI est légal et bien formé
- **Boundary de contexte :** Vérifie que le contexte fourni est complet et valide
- **Boundary d'instance :** Vérifie que l'instance cible est valide et accessible
- **Boundary de permissions :** Vérifie que les permissions sont suffisantes
- **Boundary de cohérence :** Vérifie que l'opération préserve la cohérence
- **Boundary de contournement :** Vérifie qu'aucune tentative de contournement n'est détectée
- **Boundary de charge :** Vérifie que la charge est raisonnable

**Réponses systémiques :**

Les réponses systémiques (Rejet, Neutralisation, Quarantaine, Dégradation) définies dans le Runtime Boundary & Enforcement Contract s'appliquent aux appels CoreDataAPI.

**Cohérence garantie :**

La CoreDataAPI garantit que tous les appels traversent toutes les Runtime Boundaries. Aucun appel ne peut contourner les boundaries.

### 12.2. Interaction avec Authority Graph & Cross-Domain Contract

**Relation formelle :**

La CoreDataAPI opère dans le cadre des Authority Graphs définis dans le Authority Graph & Cross-Domain Contract. Chaque opération s'exécute dans le périmètre d'un Authority Domain spécifique.

**Points d'interaction :**

- **Authority Domain :** Chaque opération cible un Authority Domain spécifique fourni dans le contexte
- **Authority Instance :** L'opération s'exécute dans le contexte d'une Authority Instance du domaine
- **Hiérarchie autoritaire :** Les opérations de synchronisation respectent la hiérarchie autoritaire (Mère/Fille)
- **Communication inter-domaines :** La CoreDataAPI ne permet pas de communication directe inter-domaines ; toute communication passe par des Intentions Certifiées

**Respect des règles cross-domain :**

- Aucune lecture directe inter-domaines
- Aucune écriture directe inter-domaines
- Aucun partage direct de données
- Communication uniquement par Intentions Certifiées validées

**Cohérence garantie :**

La CoreDataAPI garantit que toutes les opérations respectent les règles de l'Authority Graph et les restrictions cross-domain.

### 12.3. Interaction avec Identity & Cross-Domain Trust Contract

**Relation formelle :**

La CoreDataAPI intègre le contexte d'identité défini dans le Identity & Cross-Domain Trust Contract. L'identité est un élément du contexte, mais ne confère pas d'autorisation implicite.

**Points d'interaction :**

- **Contexte utilisateur :** L'identité de l'appelant est fournie dans le contexte
- **Séparation identité/autorisation :** L'identité n'est pas une autorisation ; les permissions sont évaluées séparément
- **Confiance validée :** Toute confiance inter-domaines est validée par KindMother
- **Non-transférabilité :** La confiance n'est pas transférable entre domaines

**Respect des règles d'identité :**

- Identité ≠ reconnaissance ≠ confiance ≠ autorisation
- Aucune autorisation implicite par l'identité
- Confiance contextuelle et non transférable

**Cohérence garantie :**

La CoreDataAPI garantit que l'identité est traitée conformément au contrat Identity & Cross-Domain Trust, sans créer d'autorité implicite.

---

## 13. Invariants systémiques liés à la CoreDataAPI

### 13.1. Invariants globaux

**INV-API-1 : Unicité de la surface d'appel**

La CoreDataAPI est l'unique surface d'appel vers KindMother. Aucune autre surface d'appel n'existe ou n'est autorisée.

**INV-API-2 : Validation obligatoire**

Toute opération CoreDataAPI est validée par KindMother avant exécution. Aucune opération non validée ne peut être exécutée.

**INV-API-3 : Contexte complet obligatoire**

Toute opération CoreDataAPI est accompagnée d'un contexte complet. Aucune opération sans contexte n'est autorisée.

**INV-API-4 : Atomicité des opérations**

Toute opération CoreDataAPI est atomique. Elle est exécutée complètement ou pas du tout.

**INV-API-5 : Traçabilité complète**

Toute opération CoreDataAPI est tracée de manière complète. Aucune opération non tracée n'est autorisée.

**INV-API-6 : État inchangé après rejet**

Après tout rejet, l'état des données reste inchangé. Aucune modification partielle n'est laissée.

**INV-API-7 : Erreur explicite après rejet**

Après tout rejet, une erreur explicite et actionnable est retournée à l'appelant.

**INV-API-8 : Non-contournabilité**

La CoreDataAPI ne peut pas être contournée. Toute tentative de contournement est détectée et rejetée.

### 13.2. Invariants de lecture

**INV-READ-1 : Non-modification**

Une opération de lecture ne modifie jamais l'état des données.

**INV-READ-2 : Cohérence de lecture**

Les données lues sont cohérentes avec l'état de l'instance au moment de la lecture.

**INV-READ-3 : Isolation de lecture**

Une opération de lecture est isolée des écritures concurrentes.

### 13.3. Invariants d'écriture

**INV-WRITE-1 : Intention avant application**

Toute écriture passe par une intention validée avant application.

**INV-WRITE-2 : Validation avant application**

Aucune intention d'écriture n'est appliquée sans validation préalable.

**INV-WRITE-3 : Atomicité d'écriture**

Toute écriture appliquée est atomique et définitive.

**INV-WRITE-4 : Cohérence après écriture**

L'état après une écriture appliquée est cohérent.

### 13.4. Invariants de synchronisation

**INV-SYNC-1 : Hiérarchie autoritaire respectée**

Toute synchronisation respecte la hiérarchie autoritaire (Instance Mère/Instance Fille).

**INV-SYNC-2 : Validation par l'Instance Mère**

Lors de la synchronisation Fille → Mère, l'Instance Mère valide les opérations avec autorité définitive.

**INV-SYNC-3 : Cohérence après synchronisation**

L'état après synchronisation est cohérent entre les instances concernées.

---

## 14. Cas explicitement hors périmètre

### 14.1. Ce que la CoreDataAPI n'inclut PAS

Les éléments suivants sont **explicitement hors du périmètre** de la CoreDataAPI :

**HORS-1 : Détails d'implémentation**

La CoreDataAPI ne définit pas les détails d'implémentation techniques (langages, protocoles, formats de données). Elle est purement conceptuelle.

**HORS-2 : Mécanismes de persistance**

La CoreDataAPI ne définit pas les mécanismes de persistance (bases de données, systèmes de fichiers). La persistance est interne à KindMother.

**HORS-3 : Protocoles de communication**

La CoreDataAPI ne définit pas les protocoles de communication (HTTP, gRPC, WebSocket). Les protocoles sont des choix d'implémentation.

**HORS-4 : Formats de données**

La CoreDataAPI ne définit pas les formats de données (JSON, XML, Protobuf). Les formats sont des choix d'implémentation.

**HORS-5 : Mécanismes d'authentification**

La CoreDataAPI ne définit pas les mécanismes d'authentification (JWT, OAuth, sessions). L'authentification fournit le contexte utilisateur, mais ses mécanismes sont hors périmètre.

**HORS-6 : Logique métier**

La CoreDataAPI ne définit pas la logique métier des adaptateurs. Elle fournit les opérations de données, pas la logique de traitement métier.

**HORS-7 : Interface utilisateur**

La CoreDataAPI ne définit pas les interfaces utilisateur. Elle est une surface d'appel pour les adaptateurs, pas pour les utilisateurs finaux.

**HORS-8 : Optimisations techniques**

La CoreDataAPI ne définit pas les optimisations techniques (cache, indexation, parallélisation). Les optimisations sont des choix d'implémentation.

### 14.2. Justification

Ces éléments sont hors périmètre car :
- la CoreDataAPI est une abstraction conceptuelle, pas une implémentation technique,
- les détails d'implémentation peuvent varier sans affecter le contrat conceptuel,
- la séparation des préoccupations garantit la stabilité du contrat.

---

## 15. Schémas ASCII

### 15.1. Position de la CoreDataAPI dans l'architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    ZONE EXTERNE (ADAPTATEURS)                     │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              ADAPTATEUR PRODUIT A                          │ │
│  │  (certifié KM-compliant ou non)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              ADAPTATEUR PRODUIT B                          │ │
│  │  (certifié KM-compliant ou non)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              ADAPTATEUR PRODUIT C                          │ │
│  │  (certifié KM-compliant ou non)                            │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Appels CoreDataAPI
                            │ (UNIQUE POINT D'ENTRÉE)
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│               COREDATAAPI (SURFACE D'APPEL UNIQUE)               │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  OPÉRATIONS AUTORISÉES :                                  │ │
│  │                                                            │ │
│  │  • Lecture (entités, collections, relations, état)       │ │
│  │  • Écriture (intention, validation, application)         │ │
│  │  • Écriture batch (groupement atomique)                  │ │
│  │  • Synchronisation (Mère ↔ Fille)                       │ │
│  │  • Inspection (état systémique)                          │ │
│  │                                                            │ │
│  │  PRINCIPES :                                              │ │
│  │  ✓ Unicité de la surface d'appel                         │ │
│  │  ✓ Contexte complet obligatoire                          │ │
│  │  ✓ Validation obligatoire                                │ │
│  │  ✓ Atomicité des opérations                              │ │
│  │  ✓ Traçabilité complète                                  │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Traverse les Runtime Boundaries
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    RUNTIME BOUNDARIES                             │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BOUNDARY 1 : Appel       (légalité de l'opération)       │ │
│  │  BOUNDARY 2 : Contexte    (validité du contexte)          │ │
│  │  BOUNDARY 3 : Instance    (état de l'instance)            │ │
│  │  BOUNDARY 4 : Permissions (suffisance des droits)         │ │
│  │  BOUNDARY 5 : Cohérence   (préservation de l'intégrité)  │ │
│  │  BOUNDARY 6 : Contournement (détection des abus)         │ │
│  │  BOUNDARY 7 : Charge      (ressources disponibles)        │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ Toutes boundaries passées
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│              ZONE INTERNE KINDMOTHER (EXÉCUTION)                  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │         EXÉCUTION PROTÉGÉE                                │ │
│  │  - Isolation transactionnelle                             │ │
│  │  - Atomicité garantie                                     │ │
│  │  - Traçabilité complète                                   │ │
│  │  - Intégrité garantie                                     │ │
│  │  - Cohérence préservée                                    │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 15.2. Flux d'une opération CoreDataAPI

```
┌─────────────────────────────────────────────────────────────────┐
│              FLUX D'UNE OPÉRATION COREDATAAPI                    │
│                                                                   │
│  ADAPTATEUR                                                       │
│      │                                                            │
│      │ 1. Formulation de l'opération                             │
│      │    - Type d'opération (lecture/écriture/sync/inspection)  │
│      │    - Paramètres de l'opération                            │
│      │    - Contexte complet                                     │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              COREDATAAPI                                   │ │
│  │                                                            │ │
│  │  2. Réception de l'appel                                  │ │
│  │     - Vérification de la forme de l'appel                 │ │
│  │     - Extraction du contexte                              │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 3. Traversée des Runtime Boundaries                       │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Boundary 1: Appel légal ? ──────────────→ Rejet si non  │ │
│  │  Boundary 2: Contexte valide ? ──────────→ Rejet si non  │ │
│  │  Boundary 3: Instance valide ? ──────────→ Rejet si non  │ │
│  │  Boundary 4: Permissions suffisantes ? ──→ Rejet si non  │ │
│  │  Boundary 5: Cohérence préservée ? ──────→ Rejet si non  │ │
│  │  Boundary 6: Contournement détecté ? ────→ Rejet si oui  │ │
│  │  Boundary 7: Charge acceptable ? ────────→ Neutralisation│ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 4. Toutes boundaries passées                              │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              EXÉCUTION                                     │ │
│  │                                                            │ │
│  │  5. Exécution de l'opération                              │ │
│  │     - Lecture : récupération des données                  │ │
│  │     - Écriture : application de l'intention validée       │ │
│  │     - Sync : coordination entre instances                 │ │
│  │     - Inspection : consultation de l'état                 │ │
│  │                                                            │ │
│  │  6. Traçabilité de l'opération                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      │ 7. Retour du résultat                                     │
│      ▼                                                            │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              RÉSULTAT                                      │ │
│  │                                                            │ │
│  │  • Succès : données/confirmation retournées               │ │
│  │  • Erreur : erreur explicite et actionnable               │ │
│  └───────────────────────────────────────────────────────────┘ │
│      │                                                            │
│      ▼                                                            │
│  ADAPTATEUR (reçoit le résultat)                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 15.3. Typologie des opérations

```
┌─────────────────────────────────────────────────────────────────┐
│              TYPOLOGIE DES OPÉRATIONS COREDATAAPI                │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  LECTURE                                                  │ │
│  │  ─────────                                                │ │
│  │  • Consultation pure (sans modification)                  │ │
│  │  • Retourne des données existantes                        │ │
│  │  • État après opération : INCHANGÉ                        │ │
│  │                                                            │ │
│  │  Sous-types :                                             │ │
│  │  - Lecture d'entité unique                                │ │
│  │  - Lecture de collection                                  │ │
│  │  - Lecture de relation                                    │ │
│  │  - Lecture d'état                                         │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉCRITURE (INTENTION)                                     │ │
│  │  ─────────────────────                                    │ │
│  │  • Soumission d'une intention de modification            │ │
│  │  • Validée avant application                              │ │
│  │  • État après opération : MODIFIÉ (si validée)           │ │
│  │                                                            │ │
│  │  Sous-types :                                             │ │
│  │  - Création d'entité                                      │ │
│  │  - Modification d'entité                                  │ │
│  │  - Suppression d'entité                                   │ │
│  │  - Création/suppression de relation                       │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉCRITURE BATCH                                           │ │
│  │  ──────────────                                           │ │
│  │  • Groupement d'intentions                                │ │
│  │  • Application atomique (tout ou rien)                    │ │
│  │  • État après opération : MODIFIÉ (si validé)            │ │
│  │                                                            │ │
│  │  Contraintes :                                            │ │
│  │  - Intentions cohérentes entre elles                      │ │
│  │  - Même instance cible                                    │ │
│  │  - Pas d'intentions contradictoires                       │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  SYNCHRONISATION                                          │ │
│  │  ───────────────                                          │ │
│  │  • Coordination entre Instance Mère et Instance Fille    │ │
│  │  • Respect de la hiérarchie autoritaire                  │ │
│  │  • Cohérence garantie après exécution                    │ │
│  │                                                            │ │
│  │  Sous-types :                                             │ │
│  │  - Synchronisation Fille → Mère (soumission)             │ │
│  │  - Synchronisation Mère → Fille (propagation)            │ │
│  │  - Synchronisation complète                               │ │
│  │  - Synchronisation incrémentale                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INSPECTION                                               │ │
│  │  ──────────                                               │ │
│  │  • Consultation de l'état systémique                      │ │
│  │  • Informations sur la synchronisation, santé, etc.      │ │
│  │  • État après opération : INCHANGÉ                        │ │
│  │                                                            │ │
│  │  Sous-types :                                             │ │
│  │  - État de synchronisation                                │ │
│  │  - Santé de l'instance                                    │ │
│  │  - Opérations en attente                                  │ │
│  │  - Conflits non résolus                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 15.4. Différence entre intention et écriture appliquée

```
┌─────────────────────────────────────────────────────────────────┐
│     DIFFÉRENCE ENTRE INTENTION ET ÉCRITURE APPLIQUÉE            │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  INTENTION D'ÉCRITURE                                     │ │
│  │  ════════════════════                                     │ │
│  │                                                            │ │
│  │  Nature : Demande de modification                         │ │
│  │  Statut : NON ENCORE APPLIQUÉE                           │ │
│  │  État des données : INCHANGÉ                              │ │
│  │                                                            │ │
│  │  L'intention exprime ce que l'adaptateur souhaite        │ │
│  │  modifier, mais les données ne sont pas encore modifiées │ │
│  │                                                            │ │
│  │  Exemple conceptuel :                                     │ │
│  │  "Je souhaite modifier le nom de l'entité X en 'Y'"      │ │
│  └───────────────────────────────────────────────────────────┘ │
│                        │                                         │
│                        │ VALIDATION PAR KINDMOTHER              │
│                        │ (traverse les Runtime Boundaries)      │
│                        │                                         │
│                        ▼                                         │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  RÉSULTAT DE VALIDATION                                   │ │
│  │                                                            │ │
│  │  ┌─────────────────┐    ┌─────────────────────────────┐  │ │
│  │  │    ACCEPTÉE     │    │          REJETÉE            │  │ │
│  │  │                 │    │                             │  │ │
│  │  │ L'intention est │    │ L'intention ne respecte    │  │ │
│  │  │ valide et sera  │    │ pas les règles, elle      │  │ │
│  │  │ appliquée       │    │ n'est pas appliquée       │  │ │
│  │  │                 │    │                             │  │ │
│  │  │ État : MODIFIÉ  │    │ État : INCHANGÉ           │  │ │
│  │  └─────────────────┘    └─────────────────────────────┘  │ │
│  └───────────────────────────────────────────────────────────┘ │
│                        │                                         │
│                        │ (si acceptée)                          │
│                        ▼                                         │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  ÉCRITURE APPLIQUÉE                                       │ │
│  │  ══════════════════                                       │ │
│  │                                                            │ │
│  │  Nature : Modification effective                          │ │
│  │  Statut : DÉFINITIVE                                     │ │
│  │  État des données : MODIFIÉ                              │ │
│  │                                                            │ │
│  │  L'intention validée a été appliquée de manière          │ │
│  │  atomique. Les données sont maintenant modifiées.        │ │
│  │                                                            │ │
│  │  Exemple conceptuel :                                     │ │
│  │  "Le nom de l'entité X est maintenant 'Y'"               │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  RÈGLE ABSOLUE :                                                  │
│  ═══════════════                                                  │
│  Aucune écriture n'est appliquée sans validation préalable      │
│  Si la validation échoue, l'état reste INCHANGÉ                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 16. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la surface d'appel unique entre les adaptateurs produits et KindMother.

Il garantit que :
- la CoreDataAPI est l'unique point d'entrée vers KindMother,
- toute opération est validée avant exécution,
- les opérations sont atomiques et traçables,
- les erreurs sont explicites et actionnables,
- les adaptateurs KM-compliant bénéficient de garanties stables,
- l'intégrité et la cohérence sont préservées en toutes circonstances.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, KM Adapter Compliance Contract, KindMother Runtime Boundary & Enforcement Contract, KindMother Authority Graph & Cross-Domain Contract, KindMother Identity & Cross-Domain Trust Contract  
**Type :** Contrat de surface d'appel non négociable

---

## 17. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : Confusion entre CoreDataAPI et protocole technique

**Ambiguïté rencontrée :** Risque de confusion entre la CoreDataAPI comme concept systémique et une API technique (REST, gRPC, etc.).

**Décision prise :** Définition explicite de la CoreDataAPI comme interface conceptuelle, pas technique. Section "Cas explicitement hors périmètre" ajoutée pour clarifier ce qui n'est PAS dans le périmètre du contrat.

**Correction effectuée :** Sections 2, 4, et 14 rédigées avec clarification de la nature conceptuelle et exclusion explicite des détails d'implémentation.

### Ambiguïté A2 : Distinction entre intention d'écriture et écriture appliquée

**Ambiguïté rencontrée :** Risque de confondre l'intention d'écriture (demande non encore appliquée) avec l'écriture appliquée (modification effective).

**Décision prise :** Section 6 dédiée à la distinction formelle entre lecture, intention d'écriture, et écriture appliquée, avec schéma ASCII explicatif.

**Correction effectuée :** Section 6 et schéma 15.4 rédigés avec distinction formelle et règles absolues.

### Ambiguïté A3 : Relation avec les Runtime Boundaries

**Ambiguïté rencontrée :** Nécessité de clarifier comment la CoreDataAPI interagit avec les Runtime Boundaries définies dans le contrat existant.

**Décision prise :** Section 12.1 dédiée à l'interaction avec le Runtime Boundary & Enforcement Contract, explicitant que chaque appel CoreDataAPI traverse toutes les boundaries.

**Correction effectuée :** Section 12.1 rédigée avec points d'interaction explicites et schéma 15.1 montrant la position de la CoreDataAPI.

### Ambiguïté A4 : Communication inter-domaines via CoreDataAPI

**Ambiguïté rencontrée :** Nécessité de clarifier que la CoreDataAPI ne permet pas de communication directe inter-domaines, conformément au contrat Authority Graph & Cross-Domain.

**Décision prise :** Interdiction explicite (INTERDIT-11) et section 12.2 clarifiant le respect des règles cross-domain.

**Correction effectuée :** Interdiction INTERDIT-11 ajoutée et section 12.2 rédigée avec points d'interaction explicites.

### Vérification de compatibilité

**Vérification effectuée :** Vérification systématique de la compatibilité avec les quatre contrats de fondation existants (Instance Model, Runtime Boundary & Enforcement, Authority Graph & Cross-Domain, Identity & Cross-Domain Trust). Aucune contradiction détectée.

**Conclusion :** Le contrat est strictement compatible avec le système contractuel existant. Il complète les contrats existants en définissant formellement la surface d'appel qui traverse les Runtime Boundaries.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
