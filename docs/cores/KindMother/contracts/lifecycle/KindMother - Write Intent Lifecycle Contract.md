# KindMother — Write Intent Lifecycle Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **KindMother — Write Intent Lifecycle Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la définition formelle d'une Write Intent (intention d'écriture) et décrit son cycle de vie complet dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle d'une Write Intent, ses états, ses transitions, et les règles qui régissent chaque étape de son cycle de vie, constituant le cœur du modèle offline-first de KindMother.

### Portée

Ce contrat s'applique à **toutes les intentions d'écriture** dans KindMother et définit de manière absolue :
- la définition formelle d'une Write Intent,
- le cycle de vie complet (création, validation, rejet, acceptation, application, archivage),
- la traçabilité obligatoire,
- la non-réutilisation des intentions,
- les états conceptuels d'une Write Intent,
- les invariants du cycle de vie.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **KindMother — CoreDataAPI Contract** : Définit les opérations d'écriture et la différence entre intention et écriture appliquée
- **KindMother — Runtime Boundary & Enforcement Contract** : Définit les validations à l'exécution
- **KindMother — Persistence & Storage Contract** : Définit l'application et la persistance
- **KindMother — Sync & Conflict Resolution Contract** : Définit la soumission et validation lors de la synchronisation
- **KindMother — Instance Model Contract** : Définit les rôles des instances dans le traitement des intentions
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Ce contrat respecte **LOI-2** (le système accepte l'isolement comme état normal) et **LOI-3** (l'état local est souverain) en permettant aux Write Intent d'être créées et appliquées localement sur une Instance Fille même sans connexion à l'Instance Mère, avec réconciliation explicite et traçable lors de la synchronisation.

Il n'introduit aucune contradiction et constitue la définition formelle du cœur du modèle offline-first.

---

## 2. Définition formelle d'une Write Intent

### Définition formelle

Une **Write Intent** (intention d'écriture) est une demande formelle de modification des données, formulée par un adaptateur, accompagnée d'un contexte complet, et soumise à KindMother pour validation et potentielle application.

### Caractéristiques formelles fondamentales

**Expression de volonté :** Une Write Intent exprime la volonté de l'adaptateur de modifier l'état des données. Elle représente une demande, pas une modification effective.

**Contexte complet :** Une Write Intent est accompagnée d'un contexte complet incluant l'identité de l'appelant, les permissions, l'instance cible, et le domaine d'autorité.

**Identité unique :** Chaque Write Intent possède une identité unique et immuable qui la distingue de toutes les autres intentions dans le système.

**Immutabilité :** Une Write Intent est immuable après sa création. Son contenu, ses paramètres, et son contexte ne peuvent pas être modifiés.

**Non-modification directe :** Une Write Intent ne modifie pas directement les données. Elle exprime une intention qui doit être validée et appliquée par KindMother.

**Soumission à validation :** Toute Write Intent DOIT être soumise à validation par KindMother avant toute application.

### Composition conceptuelle d'une Write Intent

Conceptuellement, une Write Intent comprend :
- **Identité :** Identifiant unique et immuable de l'intention
- **Type d'opération :** La nature de la modification souhaitée (création, modification, suppression, relation)
- **Cible :** L'entité ou les entités concernées par la modification
- **Contenu :** Les données ou changements souhaités
- **Contexte :** Les informations contextuelles (utilisateur, permissions, instance, domaine)
- **Horodatage :** Le moment de création de l'intention
- **Origine :** L'instance qui a créé l'intention (Mère ou Fille)

### Nature systémique

Une Write Intent est un **concept systémique**, pas un objet technique. Elle représente une demande formelle de modification qui traversera le cycle de vie défini par ce contrat.

**Important :** Cette définition est purement conceptuelle. Elle ne présuppose aucune structure de données, aucun format, ou aucune implémentation technique.

---

## 3. États conceptuels d'une Write Intent

### 3.1. Vue d'ensemble des états

Une Write Intent passe par une séquence d'états conceptuels bien définis :

```
CRÉÉE → EN_VALIDATION → [ACCEPTÉE | REJETÉE] → [APPLIQUÉE] → ARCHIVÉE
```

### 3.2. État CRÉÉE

**Définition :** L'état initial d'une Write Intent immédiatement après sa création par un adaptateur.

**Caractéristiques :**
- L'intention vient d'être formulée
- Le contexte est attaché mais non encore vérifié
- Aucune validation n'a été effectuée
- L'intention est en attente de traitement

**Transitions possibles :**
- CRÉÉE → EN_VALIDATION (soumission pour validation)

### 3.3. État EN_VALIDATION

**Définition :** L'état d'une Write Intent pendant sa traversée des Runtime Boundaries pour validation.

**Caractéristiques :**
- L'intention est en cours de validation par KindMother
- Les boundaries sont traversées séquentiellement
- L'intention peut être rejetée à n'importe quelle boundary
- L'état des données n'est pas encore modifié

**Transitions possibles :**
- EN_VALIDATION → ACCEPTÉE (toutes les validations réussies)
- EN_VALIDATION → REJETÉE (une validation échoue)

### 3.4. État ACCEPTÉE

**Définition :** L'état d'une Write Intent qui a passé toutes les validations avec succès.

**Caractéristiques :**
- Toutes les Runtime Boundaries ont été traversées avec succès
- L'intention est éligible pour application
- L'intention n'est pas encore appliquée
- La transition vers APPLIQUÉE est imminente

**Transitions possibles :**
- ACCEPTÉE → APPLIQUÉE (application effective)

**Note :** L'état ACCEPTÉE est généralement transitoire. Une intention acceptée est immédiatement appliquée dans le flux normal.

### 3.5. État REJETÉE

**Définition :** L'état d'une Write Intent qui a échoué à une validation.

**Caractéristiques :**
- Une ou plusieurs validations ont échoué
- L'intention ne sera jamais appliquée
- La raison du rejet est documentée
- L'état des données reste inchangé

**Transitions possibles :**
- REJETÉE → ARCHIVÉE (archivage pour traçabilité)

**État terminal :** L'état REJETÉE est un état terminal du point de vue de l'application. L'intention ne peut pas être "dérejetée".

### 3.6. État APPLIQUÉE

**Définition :** L'état d'une Write Intent qui a été appliquée de manière effective aux données.

**Caractéristiques :**
- La modification souhaitée a été effectuée
- Les données ont été modifiées de manière atomique
- La persistance a été réalisée
- L'application est définitive

**Transitions possibles :**
- APPLIQUÉE → ARCHIVÉE (archivage pour traçabilité)

### 3.7. État ARCHIVÉE

**Définition :** L'état final d'une Write Intent conservée pour traçabilité et audit.

**Caractéristiques :**
- L'intention a terminé son cycle de vie actif
- L'intention est conservée pour traçabilité
- L'intention ne peut plus être modifiée ou réutilisée
- L'historique complet est préservé

**État terminal :** L'état ARCHIVÉE est l'état terminal définitif. Aucune transition n'est possible depuis cet état.

---

## 4. Cycle de vie complet

### 4.1. Création

**Définition :** La création est l'étape initiale où un adaptateur formule une Write Intent et la soumet à KindMother.

**Acteur :** Adaptateur

**Processus conceptuel :**
1. L'adaptateur formule la modification souhaitée
2. L'adaptateur construit le contexte complet
3. L'adaptateur soumet l'intention via la CoreDataAPI
4. KindMother attribue une identité unique à l'intention
5. L'intention passe à l'état CRÉÉE

**Règles de création :**

**CREAT-1 :** Toute Write Intent DOIT être créée via la CoreDataAPI. Aucune création directe n'est autorisée.

**CREAT-2 :** Toute Write Intent DOIT être accompagnée d'un contexte complet. Une intention sans contexte est rejetée immédiatement.

**CREAT-3 :** L'identité d'une Write Intent est attribuée par KindMother, jamais par l'adaptateur.

**CREAT-4 :** Une Write Intent est immuable dès sa création. Aucune modification ultérieure n'est autorisée.

### 4.2. Validation

**Définition :** La validation est l'étape où KindMother vérifie que l'intention est conforme à toutes les règles et contraintes avant de l'appliquer.

**Acteur :** KindMother

**Processus conceptuel :**
1. L'intention passe à l'état EN_VALIDATION
2. L'intention traverse les Runtime Boundaries :
   - Boundary d'appel (légalité de l'opération)
   - Boundary de contexte (validité du contexte)
   - Boundary d'instance (état de l'instance)
   - Boundary de permissions (suffisance des droits)
   - Boundary de cohérence (préservation de l'intégrité)
   - Boundary de contournement (détection des abus)
   - Boundary de charge (ressources disponibles)
3. Si toutes les boundaries sont passées → ACCEPTÉE
4. Si une boundary échoue → REJETÉE

**Règles de validation :**

**VALID-1 :** Toute Write Intent DOIT traverser toutes les Runtime Boundaries. Aucune boundary ne peut être contournée.

**VALID-2 :** Si une boundary échoue, l'intention est immédiatement rejetée. La validation s'arrête à la première erreur.

**VALID-3 :** La validation est effectuée par KindMother exclusivement. Aucune validation externe n'est autorisée.

**VALID-4 :** Le résultat de la validation est déterministe. La même intention dans les mêmes conditions produit toujours le même résultat.

### 4.3. Rejet

**Définition :** Le rejet est l'étape où une Write Intent échoue à la validation et est marquée comme non applicable.

**Acteur :** KindMother

**Processus conceptuel :**
1. Une validation échoue
2. L'intention passe à l'état REJETÉE
3. La raison du rejet est documentée
4. L'erreur explicite est retournée à l'adaptateur
5. L'état des données reste inchangé
6. L'intention est archivée pour traçabilité

**Règles de rejet :**

**REJECT-1 :** Un rejet DOIT indiquer explicitement la raison de l'échec. Aucun rejet silencieux n'est autorisé.

**REJECT-2 :** Un rejet DOIT laisser l'état des données inchangé. Aucune modification partielle n'est autorisée.

**REJECT-3 :** Une intention rejetée ne peut pas être "dérejetée" ou réessayée. Une nouvelle intention doit être créée.

**REJECT-4 :** Le rejet est tracé pour audit. La raison, le contexte, et le moment sont enregistrés.

### 4.4. Acceptation

**Définition :** L'acceptation est l'étape où une Write Intent a passé toutes les validations et est éligible pour application.

**Acteur :** KindMother

**Processus conceptuel :**
1. Toutes les Runtime Boundaries sont passées avec succès
2. L'intention passe à l'état ACCEPTÉE
3. L'intention est immédiatement éligible pour application
4. La transition vers l'application est effectuée

**Règles d'acceptation :**

**ACCEPT-1 :** Une intention ACCEPTÉE DOIT être appliquée. L'acceptation implique l'application imminente.

**ACCEPT-2 :** L'état ACCEPTÉE est transitoire. Une intention ne reste pas indéfiniment dans cet état.

**ACCEPT-3 :** L'acceptation ne peut pas être révoquée. Une fois acceptée, l'intention sera appliquée.

### 4.5. Application

**Définition :** L'application est l'étape où la modification souhaitée est effectivement réalisée sur les données.

**Acteur :** KindMother

**Processus conceptuel :**
1. L'intention ACCEPTÉE est appliquée
2. Les données sont modifiées de manière atomique
3. La persistance est effectuée
4. L'intention passe à l'état APPLIQUÉE
5. La confirmation est retournée à l'adaptateur

**Règles d'application :**

**APPLY-1 :** L'application est atomique. Toutes les modifications sont appliquées ou aucune n'est appliquée.

**APPLY-2 :** L'application est définitive. Une fois appliquée, la modification ne peut être annulée que par une nouvelle intention.

**APPLY-3 :** L'application déclenche la persistance. Les données modifiées sont immédiatement persistées.

**APPLY-4 :** L'application est traçable. L'intention appliquée est conservée pour audit.

### 4.6. Archivage

**Définition :** L'archivage est l'étape finale où l'intention est conservée pour traçabilité et audit, quelle que soit son issue (rejetée ou appliquée).

**Acteur :** KindMother

**Processus conceptuel :**
1. L'intention a atteint un état terminal (REJETÉE ou APPLIQUÉE)
2. L'intention est archivée avec son historique complet
3. L'intention passe à l'état ARCHIVÉE
4. L'intention reste accessible pour consultation mais non modifiable

**Règles d'archivage :**

**ARCHIV-1 :** Toute intention terminée DOIT être archivée. Aucune intention ne disparaît silencieusement.

**ARCHIV-2 :** L'archive inclut l'historique complet : création, validation, décision, application (si applicable).

**ARCHIV-3 :** Une intention archivée est immuable. Aucune modification de l'archive n'est autorisée.

**ARCHIV-4 :** L'archive est consultable pour audit. Les intentions archivées sont accessibles aux acteurs autorisés.

---

## 5. Traçabilité obligatoire

### 5.1. Principe de traçabilité

**Énoncé :** Toute Write Intent DOIT être traçable tout au long de son cycle de vie. Aucune étape ne peut être effectuée sans traçabilité.

### 5.2. Éléments traçables

**TRACE-1 : Création**
- Identité de l'intention
- Moment de création
- Adaptateur d'origine
- Contexte complet
- Contenu de l'intention

**TRACE-2 : Validation**
- Boundaries traversées
- Résultat de chaque boundary
- Moment de chaque validation
- Erreurs rencontrées (si applicable)

**TRACE-3 : Décision**
- Acceptation ou rejet
- Raison de la décision
- Moment de la décision
- Autorité ayant pris la décision

**TRACE-4 : Application**
- Moment de l'application
- Modifications effectuées
- État résultant
- Confirmation de persistance

**TRACE-5 : Archivage**
- Moment de l'archivage
- État final
- Historique complet préservé

### 5.3. Garanties de traçabilité

**G-TRACE-1 :** Aucune intention ne peut exister sans traçabilité.

**G-TRACE-2 :** L'historique de traçabilité est immuable. Il ne peut pas être modifié après coup.

**G-TRACE-3 :** La traçabilité est accessible pour audit par les acteurs autorisés.

**G-TRACE-4 :** La traçabilité couvre l'intégralité du cycle de vie.

---

## 6. Non-réutilisation des intentions

### 6.1. Principe de non-réutilisation

**Énoncé :** Une Write Intent ne peut être utilisée qu'une seule fois. Elle ne peut pas être réutilisée, résoumise, ou recyclée.

### 6.2. Règles de non-réutilisation

**NOREUSE-1 : Unicité d'usage**

Une Write Intent ne peut être soumise qu'une seule fois pour validation. Après sa soumission, elle ne peut pas être resoumise.

**NOREUSE-2 : Pas de réessai direct**

Si une Write Intent est rejetée, elle ne peut pas être réessayée. Une nouvelle intention doit être créée avec une nouvelle identité.

**NOREUSE-3 : Pas de recyclage**

Une Write Intent terminée (REJETÉE ou APPLIQUÉE) ne peut pas être recyclée ou transformée en une nouvelle intention.

**NOREUSE-4 : Identité non réutilisable**

L'identité d'une Write Intent ne peut pas être réutilisée pour une autre intention. Chaque intention a une identité unique et éphémère.

### 6.3. Justification

La non-réutilisation garantit :
- La traçabilité claire (une intention = un cycle de vie)
- La prévention du replay (une intention = une seule application)
- L'immutabilité de l'historique (chaque intention est distincte)
- La sécurité du système (pas de réutilisation malveillante)

---

## 7. Intentions locales vs intentions définitives

### 7.1. Intention locale (Instance Fille)

**Définition :** Une intention locale est une Write Intent créée et appliquée localement sur une Instance Fille, en attente de validation définitive par l'Instance Mère.

**Caractéristiques :**
- Créée par un adaptateur sur une Instance Fille
- Validée et appliquée localement
- En attente de soumission à l'Instance Mère
- Non définitive tant que non validée par la Mère

**Conformité LOI-2 et LOI-3 :** Cette caractéristique respecte **LOI-2** (le système accepte l'isolement comme état normal) : les Write Intent peuvent être créées et appliquées localement même sans connexion à l'Instance Mère, l'isolement n'est pas traité comme une erreur. Elle respecte également **LOI-3** (l'état local est souverain) : l'Instance Fille détient l'autorité locale sur les Write Intent appliquées localement, et la réconciliation avec l'Instance Mère est explicite et traçable.

**Cycle de vie local :**
1. CRÉÉE (sur Fille)
2. EN_VALIDATION (validation locale)
3. ACCEPTÉE (localement)
4. APPLIQUÉE (localement) — *en attente de confirmation Mère*
5. Soumission à la Mère lors de la synchronisation
6. Validation par la Mère → Définitive ou Annulée
7. ARCHIVÉE (avec statut final)

### 7.2. Intention définitive (Instance Mère)

**Définition :** Une intention définitive est une Write Intent validée et appliquée par l'Instance Mère, constituant une modification de la source de vérité.

**Caractéristiques :**
- Validée par l'Instance Mère (autorité définitive)
- Appliquée sur la source de vérité
- Définitive et non révocable
- Propageable vers les Instances Filles

**Cycle de vie définitif :**
1. CRÉÉE (directement sur Mère ou soumise par Fille)
2. EN_VALIDATION (validation Mère)
3. ACCEPTÉE (définitivement)
4. APPLIQUÉE (sur source de vérité)
5. ARCHIVÉE (définitive)

### 7.3. Transition locale → définitive

**Processus :**
1. Intention locale appliquée sur Fille
2. Soumission à la Mère lors de la synchronisation
3. Validation par la Mère :
   - Si validée → devient définitive, conservée sur Fille
   - Si rejetée → annulée localement sur Fille

**Règles de transition :**

**TRANS-1 :** Une intention locale ne devient définitive qu'après validation par la Mère.

**TRANS-2 :** Si l'intention locale est rejetée par la Mère, les modifications locales sont annulées.

**TRANS-3 :** L'état local de la Fille est mis à jour pour refléter la décision de la Mère.

**Conformité LOI-3 :** Cette règle respecte **LOI-3** (l'état local est souverain) : avant la réconciliation, l'état local de l'Instance Fille (incluant les Write Intent appliquées localement) est souverain et valable localement. La réconciliation avec l'Instance Mère est explicite et traçable, préservant la souveraineté locale jusqu'à la réconciliation.

---

## 8. Invariants du cycle de vie

### 8.1. Invariants de création

**INV-LIFE-1 :** Toute Write Intent DOIT être créée via la CoreDataAPI.

**INV-LIFE-2 :** Toute Write Intent DOIT avoir un contexte complet dès la création.

**INV-LIFE-3 :** Toute Write Intent reçoit une identité unique et immuable.

**INV-LIFE-4 :** Une Write Intent est immuable après création.

### 8.2. Invariants de validation

**INV-LIFE-5 :** Toute Write Intent DOIT être validée avant application.

**INV-LIFE-6 :** La validation traverse toutes les Runtime Boundaries sans exception.

**INV-LIFE-7 :** Le résultat de validation est binaire : acceptée ou rejetée.

**INV-LIFE-8 :** La validation est effectuée exclusivement par KindMother.

### 8.3. Invariants de terminaison

**INV-LIFE-9 :** Toute Write Intent atteint un état terminal (REJETÉE ou APPLIQUÉE puis ARCHIVÉE).

**INV-LIFE-10 :** Un rejet laisse l'état des données inchangé.

**INV-LIFE-11 :** Une application modifie l'état de manière atomique.

**INV-LIFE-12 :** Toute intention terminée est archivée.

### 8.4. Invariants de non-réutilisation

**INV-LIFE-13 :** Une Write Intent ne peut être soumise qu'une seule fois.

**INV-LIFE-14 :** Une identité d'intention ne peut pas être réutilisée.

**INV-LIFE-15 :** Une intention rejetée ne peut pas être réessayée directement.

### 8.5. Invariants de traçabilité

**INV-LIFE-16 :** Toute Write Intent est traçable tout au long de son cycle de vie.

**INV-LIFE-17 :** L'historique de traçabilité est immuable.

**INV-LIFE-18 :** La traçabilité est accessible pour audit.

---

## 9. Interaction avec les contrats existants

### 9.1. Interaction avec CoreDataAPI Contract

**Cohérence avec la section 6 (différence intention/écriture appliquée) :**

Ce contrat formalise le cycle de vie complet décrit conceptuellement dans le CoreDataAPI Contract. La différence entre intention et écriture appliquée correspond à la différence entre les états CRÉÉE/EN_VALIDATION et APPLIQUÉE.

**Cohérence avec les opérations d'écriture :**

Les opérations d'écriture de la CoreDataAPI (section 5.2) correspondent à la création d'une Write Intent au sens de ce contrat.

### 9.2. Interaction avec Runtime Boundary Contract

**Traversée des boundaries :**

La validation d'une Write Intent correspond à la traversée des Runtime Boundaries définies dans le Runtime Boundary Contract. Les 7 boundaries sont traversées dans l'ordre.

**Réponses systémiques :**

Les réponses systémiques (Rejet R1, etc.) du Runtime Boundary Contract s'appliquent lors de la validation des Write Intents.

### 9.3. Interaction avec Persistence & Storage Contract

**Application et persistance :**

L'étape d'application de ce contrat déclenche la persistance définie dans le Persistence & Storage Contract. L'atomicité de persistance s'applique à l'application des Write Intents.

### 9.4. Interaction avec Sync & Conflict Resolution Contract

**Soumission lors de la synchronisation :**

Les intentions locales (Fille) sont soumises à la Mère lors de la synchronisation. Le processus de validation par la Mère et la résolution de conflits s'appliquent.

**Intentions et conflits :**

Les conflits de synchronisation impliquent des Write Intents conflictuelles, résolues selon les règles du Sync & Conflict Resolution Contract.

---

## 10. Schémas ASCII conceptuels

### 10.1. Machine à états d'une Write Intent

```
┌─────────────────────────────────────────────────────────────────┐
│           MACHINE À ÉTATS D'UNE WRITE INTENT                     │
│                                                                   │
│  ┌───────────┐                                                   │
│  │  CRÉÉE    │ ◄─── Création par adaptateur via CoreDataAPI     │
│  │           │      (identité attribuée, contexte attaché)       │
│  └─────┬─────┘                                                   │
│        │                                                          │
│        │ Soumission pour validation                              │
│        ▼                                                          │
│  ┌─────────────────┐                                             │
│  │  EN_VALIDATION  │ ◄─── Traversée des Runtime Boundaries      │
│  │                 │      (validation par KindMother)            │
│  └────────┬────────┘                                             │
│           │                                                       │
│     ┌─────┴─────┐                                                │
│     │           │                                                │
│     ▼           ▼                                                │
│  ┌───────┐  ┌───────────┐                                        │
│  │REJETÉE│  │ ACCEPTÉE  │                                        │
│  │       │  │           │                                        │
│  │ (état │  │ (toutes   │                                        │
│  │ incha-│  │ validations│                                       │
│  │ ngé)  │  │ réussies) │                                        │
│  └───┬───┘  └─────┬─────┘                                        │
│      │            │                                              │
│      │            │ Application immédiate                        │
│      │            ▼                                              │
│      │      ┌───────────┐                                        │
│      │      │ APPLIQUÉE │ ◄─── Modification effective + persist. │
│      │      │           │                                        │
│      │      │ (données  │                                        │
│      │      │ modifiées)│                                        │
│      │      └─────┬─────┘                                        │
│      │            │                                              │
│      └──────┬─────┘                                              │
│             │ Archivage                                          │
│             ▼                                                     │
│       ┌───────────┐                                              │
│       │ ARCHIVÉE  │ ◄─── Conservation pour traçabilité          │
│       │           │      (état terminal définitif)               │
│       │ (historique│                                             │
│       │  préservé) │                                             │
│       └───────────┘                                              │
│                                                                   │
│  RÈGLE : Aucune transition arrière n'est autorisée              │
└─────────────────────────────────────────────────────────────────┘
```

### 10.2. Cycle de vie complet

```
┌─────────────────────────────────────────────────────────────────┐
│              CYCLE DE VIE COMPLET D'UNE WRITE INTENT             │
│                                                                   │
│  1. CRÉATION                                                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Adaptateur → CoreDataAPI → KindMother                      │ │
│  │                                                            │ │
│  │ • Formulation de l'intention                              │ │
│  │ • Construction du contexte                                │ │
│  │ • Attribution d'identité unique                           │ │
│  │ • État : CRÉÉE                                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            ▼                                     │
│  2. VALIDATION                                                    │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ KindMother (Runtime Boundaries)                            │ │
│  │                                                            │ │
│  │ • Boundary d'appel ✓                                      │ │
│  │ • Boundary de contexte ✓                                  │ │
│  │ • Boundary d'instance ✓                                   │ │
│  │ • Boundary de permissions ✓                               │ │
│  │ • Boundary de cohérence ✓                                 │ │
│  │ • Boundary de contournement ✓                             │ │
│  │ • Boundary de charge ✓                                    │ │
│  │                                                            │ │
│  │ État : EN_VALIDATION → ACCEPTÉE ou REJETÉE               │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│              ┌─────────────┴─────────────┐                      │
│              ▼                           ▼                      │
│  3a. REJET                    3b. ACCEPTATION + APPLICATION     │
│  ┌─────────────────┐          ┌─────────────────────────────┐  │
│  │ • Validation    │          │ • Toutes validations OK     │  │
│  │   échouée       │          │ • Application atomique      │  │
│  │ • Erreur        │          │ • Persistance immédiate     │  │
│  │   explicite     │          │ • Confirmation              │  │
│  │ • État inchangé │          │ • État : APPLIQUÉE          │  │
│  │ • État : REJETÉE│          │                             │  │
│  └────────┬────────┘          └──────────────┬──────────────┘  │
│           │                                   │                  │
│           └───────────────┬───────────────────┘                  │
│                           ▼                                      │
│  4. ARCHIVAGE                                                    │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ KindMother                                                 │ │
│  │                                                            │ │
│  │ • Conservation de l'historique complet                    │ │
│  │ • Intention non modifiable                                │ │
│  │ • Accessible pour audit                                   │ │
│  │ • État : ARCHIVÉE (terminal)                              │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 10.3. Intention locale vs intention définitive

```
┌─────────────────────────────────────────────────────────────────┐
│        INTENTION LOCALE vs INTENTION DÉFINITIVE                  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE FILLE                                │ │
│  │                                                            │ │
│  │  1. Création de l'intention locale                        │ │
│  │  2. Validation locale (boundaries Fille)                  │ │
│  │  3. Application locale                                    │ │
│  │                                                            │ │
│  │  État : APPLIQUÉE_LOCALEMENT                              │ │
│  │  Statut : EN ATTENTE DE CONFIRMATION MÈRE                 │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ SYNCHRONISATION                     │
│                            │ (soumission à la Mère)             │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE MÈRE                                 │ │
│  │                                                            │ │
│  │  4. Réception de l'intention                              │ │
│  │  5. Validation par la Mère (autorité définitive)          │ │
│  │                                                            │ │
│  │  ┌─────────────────────┐  ┌─────────────────────┐        │ │
│  │  │ CAS A : VALIDÉE     │  │ CAS B : REJETÉE     │        │ │
│  │  │                     │  │                     │        │ │
│  │  │ • Devient           │  │ • Conflit détecté   │        │ │
│  │  │   définitive        │  │   ou règle violée   │        │ │
│  │  │ • Appliquée sur     │  │ • Rejet définitif   │        │ │
│  │  │   source de vérité  │  │                     │        │ │
│  │  └──────────┬──────────┘  └──────────┬──────────┘        │ │
│  └─────────────┼─────────────────────────┼───────────────────┘ │
│                │                         │                      │
│                ▼                         ▼                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              INSTANCE FILLE (retour)                       │ │
│  │                                                            │ │
│  │  ┌─────────────────────┐  ┌─────────────────────┐        │ │
│  │  │ CAS A : CONSERVÉE   │  │ CAS B : ANNULÉE     │        │ │
│  │  │                     │  │                     │        │ │
│  │  │ • Intention         │  │ • Modifications     │        │ │
│  │  │   définitive        │  │   locales annulées  │        │ │
│  │  │ • Archivée comme    │  │ • Rejet tracé       │        │ │
│  │  │   validée           │  │ • Archivée comme    │        │ │
│  │  │                     │  │   rejetée           │        │ │
│  │  └─────────────────────┘  └─────────────────────┘        │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  PRINCIPE : La Mère a l'autorité définitive                     │
└─────────────────────────────────────────────────────────────────┘
```

### 10.4. Non-réutilisation

```
┌─────────────────────────────────────────────────────────────────┐
│                  NON-RÉUTILISATION DES INTENTIONS                │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Write Intent #123                                         │ │
│  │  Identité : unique et éphémère                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            │ Cycle de vie                        │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  CRÉÉE → EN_VALIDATION → REJETÉE → ARCHIVÉE              │ │
│  │                                                            │ │
│  │  Ou : CRÉÉE → EN_VALIDATION → ACCEPTÉE → APPLIQUÉE →     │ │
│  │       ARCHIVÉE                                            │ │
│  └───────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  APRÈS TERMINAISON                                         │ │
│  │                                                            │ │
│  │  ✗ Résoumission de #123 → INTERDIT                       │ │
│  │  ✗ Réessai de #123 → INTERDIT                            │ │
│  │  ✗ Recyclage de #123 → INTERDIT                          │ │
│  │  ✗ Réutilisation de l'identité #123 → INTERDIT           │ │
│  │                                                            │ │
│  │  ✓ Création d'une NOUVELLE intention #456 → AUTORISÉ     │ │
│  │    (nouvelle identité, nouveau cycle de vie)              │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                   │
│  RAISONS :                                                        │
│  • Traçabilité claire (1 intention = 1 cycle)                   │
│  • Prévention du replay                                          │
│  • Immutabilité de l'historique                                  │
│  • Sécurité du système                                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 11. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable le cycle de vie d'une Write Intent dans KindMother.

Il garantit que :
- chaque intention suit un cycle de vie strict et prévisible,
- la validation est obligatoire avant toute application,
- les rejets laissent l'état inchangé,
- les applications sont atomiques et définitives,
- la traçabilité est complète,
- la non-réutilisation est absolue.

Ce contrat constitue le cœur du modèle offline-first de KindMother.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, KindMother CoreDataAPI Contract, KindMother Runtime Boundary Contract, KindMother Persistence Contract, KindMother Sync Contract  
**Type :** Contrat de cycle de vie des intentions d'écriture non négociable

---

## 12. Mini log — erreurs / warnings / ambiguïtés rencontrées et corrigées

### Ambiguïté A1 : État ACCEPTÉE transitoire ou persistant

**Ambiguïté rencontrée :** L'état ACCEPTÉE est-il un état persistant ou transitoire ? Une intention peut-elle rester ACCEPTÉE sans être appliquée ?

**Décision prise :** L'état ACCEPTÉE est transitoire. Une intention acceptée est immédiatement appliquée dans le flux normal. ACCEPT-1 établit que l'acceptation implique l'application imminente.

**Correction effectuée :** Section 3.4 et règle ACCEPT-1 clarifient la nature transitoire de l'état ACCEPTÉE.

### Ambiguïté A2 : Intention locale appliquée mais rejetée par Mère

**Ambiguïté rencontrée :** Que se passe-t-il pour une intention appliquée localement sur Fille mais rejetée par la Mère lors de la synchronisation ?

**Décision prise :** Les modifications locales sont annulées. L'intention locale devient définitivement rejetée. Le Sync Contract définit ce comportement, ce contrat le complète avec TRANS-2.

**Correction effectuée :** Section 7.3 inclut les règles de transition locale → définitive, notamment TRANS-2 pour le cas de rejet.

### Ambiguïté A3 : Nouvelle intention après rejet

**Ambiguïté rencontrée :** Si une intention est rejetée, comment l'adaptateur peut-il réessayer son opération ?

**Décision prise :** L'intention rejetée ne peut pas être réessayée directement. L'adaptateur DOIT créer une nouvelle intention avec une nouvelle identité. Les informations du rejet peuvent guider la création de la nouvelle intention.

**Correction effectuée :** REJECT-3 et NOREUSE-2 clarifient que le réessai nécessite une nouvelle intention.

### Ambiguïté A4 : Immutabilité vs archivage

**Ambiguïté rencontrée :** Si une intention est immuable, comment peut-elle passer par différents états ?

**Décision prise :** L'immutabilité concerne le contenu, les paramètres et le contexte de l'intention. L'état fait partie du cycle de vie géré par KindMother, pas du contenu de l'intention.

**Correction effectuée :** INV-LIFE-4 précise que l'immutabilité s'applique au contenu après création.

### Vérification de compatibilité

**Vérification effectuée :**
- ✅ Cohérence avec CoreDataAPI section 6 : Confirmée
- ✅ Cohérence avec Runtime Boundary Contract : Confirmée
- ✅ Cohérence avec Persistence Contract (atomicité) : Confirmée
- ✅ Cohérence avec Sync Contract (soumission, conflits) : Confirmée
- ✅ Aucune autorité implicite créée : Confirmée
- ✅ Zero-trust respecté : Confirmée
- ✅ Aucune dépendance technique : Confirmée

**Conclusion :** Aucune contradiction détectée avec les contrats existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
