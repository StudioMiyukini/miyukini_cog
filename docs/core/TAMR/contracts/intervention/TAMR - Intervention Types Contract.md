# TAMR — Intervention Types Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **TAMR — Intervention Types Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la définition formelle des quatre types d'intervention humaine dans le Miyukini Core System v2.4, leurs caractéristiques, leurs conditions d'usage, et les règles absolues qui les gouvernent.

Ce contrat précise la nature conceptuelle de chaque type d'intervention, les propriétés distinctives, les conditions de validité, et les invariants associés.

### Portée

Ce contrat s'applique à **toutes les interventions humaines dans le système Miyukini** et définit de manière absolue :
- les quatre types d'intervention reconnus,
- les caractéristiques distinctives de chaque type,
- les conditions de validité de chaque type,
- les relations entre types,
- les règles de traçabilité par type,
- les invariants associés à chaque type,
- les cas d'usage typiques.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **[TAMR — Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Définition philosophique de TAMR et introduction des types
- **[TAMR — Intervention Points Contract](./TAMR%20-%20Intervention%20Points%20Contract.md)** : Définition des points où les interventions peuvent se produire
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie officielle
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie, notamment **LOI-2** (le système accepte l'isolement comme état normal) : les interventions humaines restent possibles en mode isolé

Il n'introduit aucune contradiction, et constitue la définition formelle des types d'intervention humaine dans TAMR.

---

## 2. Vue d'ensemble des types d'intervention

### Les quatre types fondamentaux

TAMR reconnaît exactement **quatre types d'intervention humaine**. Cette liste est exhaustive, fermée, et non extensible :

| Type | Nom français | Description courte |
|------|--------------|-------------------|
| **APPROVAL** | Approbation | Valider une action avant son exécution |
| **OVERRIDE** | Dérogation | Contredire une décision automatique |
| **ESCALATION** | Escalade | Élever une décision vers un niveau supérieur |
| **SUPERVISION** | Supervision | Observer avec capacité d'intervention |

### Principe de fermeture

**INV-TYPE-1 : Liste fermée**

Les quatre types définis dans ce contrat sont les **seuls types reconnus**. Aucun type supplémentaire ne peut être introduit sans modification formelle de ce contrat.

**INV-TYPE-2 : Unicité de type**

Toute intervention humaine appartient à **exactement un type**. Une intervention ne peut pas appartenir à plusieurs types simultanément.

### Classification conceptuelle

Les types se distinguent selon trois axes :

| Type | Moment de l'intervention | Nature de l'action | Impact sur le flux |
|------|--------------------------|--------------------|--------------------|
| **APPROVAL** | Avant l'action | Validation | Bloquant ou non |
| **OVERRIDE** | Après la décision automatique | Dérogation | Immédiat |
| **ESCALATION** | En cours de traitement | Délégation | Différé |
| **SUPERVISION** | Continu | Observation | Conditionnel |

---

## 3. Type APPROVAL (Approbation)

### 3.1. Définition

Une **approbation** est un type d'intervention où l'humain valide ou refuse une action proposée par le système **avant** son exécution.

Le système propose, l'humain décide.

### 3.2. Caractéristiques

| Caractéristique | Description |
|-----------------|-------------|
| **Préventive** | L'approbation se produit AVANT l'exécution de l'action |
| **Binaire** | Le résultat est soit APPROUVÉ soit REFUSÉ — pas d'état intermédiaire |
| **Conditionnellement bloquante** | Selon la configuration, l'action peut attendre ou non la décision |
| **Traçable** | L'approbation et son résultat sont enregistrés |

### 3.3. États d'une approbation

Une demande d'approbation traverse les états suivants :

```
1. DEMANDÉE    → L'approbation est sollicitée
2. EN_ATTENTE  → L'approbateur n'a pas encore répondu
3. RÉSOLUE     → L'approbateur a rendu sa décision (APPROUVÉ ou REFUSÉ)
```

**INV-APPR-1 : Terminaison**

Toute demande d'approbation atteint l'état RÉSOLUE, soit par décision humaine, soit par mécanisme de timeout défini par le produit.

### 3.4. Résultats possibles

| Résultat | Signification |
|----------|---------------|
| **APPROUVÉ** | L'humain valide l'action — elle peut être exécutée |
| **REFUSÉ** | L'humain refuse l'action — elle ne sera pas exécutée |
| **EXPIRÉ** | Le délai d'attente est dépassé — comportement par défaut appliqué |

### 3.5. Règles d'approbation

**R-APPR-1 : Identité obligatoire**

Toute approbation DOIT identifier l'humain qui approuve ou refuse. Une approbation anonyme est invalide.

**R-APPR-2 : Unicité de réponse**

Une demande d'approbation ne peut recevoir qu'une seule réponse. Une fois APPROUVÉE ou REFUSÉE, la décision est définitive.

**R-APPR-3 : Non-rétroactivité**

Une approbation ne peut s'appliquer qu'à une action future, jamais à une action déjà exécutée.

**R-APPR-4 : Comportement par défaut explicite**

Le comportement en cas d'expiration (timeout) DOIT être explicitement défini (refus par défaut ou approbation par défaut).

### 3.6. Données de traçabilité

Toute approbation DOIT être tracée avec :

| Donnée | Obligatoire | Description |
|--------|-------------|-------------|
| `intervention_id` | ✅ Oui | Identifiant unique de l'intervention |
| `type` | ✅ Oui | Toujours "APPROVAL" |
| `approver_id` | ✅ Oui | Identité de l'approbateur |
| `requested_at` | ✅ Oui | Moment de la demande (horodatage local) |
| `resolved_at` | ✅ Oui | Moment de la résolution (horodatage local) |
| `result` | ✅ Oui | APPROUVÉ, REFUSÉ, ou EXPIRÉ |
| `subject` | ✅ Oui | Action concernée par l'approbation |
| `context` | ✅ Oui | Contexte de l'approbation |
| `comment` | ❌ Non | Commentaire optionnel de l'approbateur |

---

## 4. Type OVERRIDE (Dérogation)

### 4.1. Définition

Un **override** est un type d'intervention où l'humain **contredit** une décision automatique du système, soit pour forcer une action refusée, soit pour empêcher une action approuvée.

L'humain prend la responsabilité de contredire le système.

### 4.2. Caractéristiques

| Caractéristique | Description |
|-----------------|-------------|
| **Dérogatoire** | L'override contredit explicitement une décision automatique |
| **Exceptionnel** | L'override ne doit pas être la norme — c'est une exception |
| **Justifié** | L'override nécessite une justification explicite obligatoire |
| **Audité** | L'override fait l'objet d'un suivi renforcé |
| **Limité** | Certaines limites infranchissables ne peuvent pas être overridées |

### 4.3. Types d'override

| Sous-type | Description |
|-----------|-------------|
| **FORCE** | Forcer l'exécution d'une action automatiquement refusée |
| **BLOCK** | Bloquer l'exécution d'une action automatiquement approuvée |

### 4.4. Règles d'override

**R-OVER-1 : Justification obligatoire**

Tout override DOIT être accompagné d'une justification explicite. Un override sans justification est invalide.

**INV-TAMR-7** (repris de la Documentation Fondatrice) : *Tout override nécessite une justification explicite enregistrée.*

**R-OVER-2 : Limites infranchissables**

Un override NE PEUT JAMAIS franchir une limite infranchissable. Les limites infranchissables sont définies dans le contrat [TAMR — Inviolable Limits Contract](../boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md).

**R-OVER-3 : Décision automatique préalable**

Un override ne peut se produire qu'après une décision automatique. Overrider sans décision préalable est invalide.

**R-OVER-4 : Responsabilité assumée**

L'humain qui override assume explicitement la responsabilité des conséquences.

**R-OVER-5 : Audit renforcé**

Tout override déclenche un audit renforcé traçant le contexte complet.

### 4.5. Données de traçabilité

Toute dérogation DOIT être tracée avec :

| Donnée | Obligatoire | Description |
|--------|-------------|-------------|
| `intervention_id` | ✅ Oui | Identifiant unique de l'intervention |
| `type` | ✅ Oui | Toujours "OVERRIDE" |
| `override_type` | ✅ Oui | FORCE ou BLOCK |
| `overrider_id` | ✅ Oui | Identité de l'humain qui override |
| `justification` | ✅ Oui | Justification explicite obligatoire |
| `original_decision` | ✅ Oui | Décision automatique contredite |
| `overridden_at` | ✅ Oui | Moment de l'override (horodatage local) |
| `subject` | ✅ Oui | Action concernée |
| `context` | ✅ Oui | Contexte complet |
| `limits_checked` | ✅ Oui | Confirmation que les limites ont été vérifiées |

### 4.6. Invariants spécifiques

**INV-OVER-1 : Non-franchissement des limites**

Aucun override ne franchit jamais une limite infranchissable, quelle que soit la justification fournie.

**INV-OVER-2 : Traçabilité renforcée**

Tout override est traçable avec un niveau de détail supérieur aux autres types d'intervention.

---

## 5. Type ESCALATION (Escalade)

### 5.1. Définition

Une **escalade** est un type d'intervention où l'humain élève une décision vers un **niveau d'autorité supérieur** humain pour révision ou arbitrage.

La responsabilité est transférée ou partagée avec un niveau supérieur.

### 5.2. Caractéristiques

| Caractéristique | Description |
|-----------------|-------------|
| **Hiérarchique** | L'escalade monte dans une chaîne de responsabilité définie |
| **Non bloquante immédiatement** | L'escalade peut différer la décision sans bloquer le système |
| **Collaborative** | L'escalade implique plusieurs humains |
| **Tracée** | Le chemin d'escalade complet est enregistré |
| **Terminante** | L'escalade ne peut pas durer indéfiniment (INV-TAMR-8) |

### 5.3. États d'une escalade

Une escalade traverse les états suivants :

```
1. INITIÉE      → L'escalade est déclenchée par un humain
2. EN_COURS     → L'escalade est transmise au niveau supérieur
3. RÉSOLUE      → Le niveau supérieur a rendu une décision
4. ANNULÉE      → L'escalade est annulée (par l'initiateur ou par timeout)
```

### 5.4. Règles d'escalade

**R-ESC-1 : Chaîne définie**

Toute escalade DOIT suivre une chaîne de responsabilité préalablement définie. Une escalade vers un destinataire non défini est invalide.

**R-ESC-2 : Non-blocage**

Une escalade NE DOIT JAMAIS bloquer indéfiniment le système. Des mécanismes de timeout, de délégation automatique, ou de rejet par défaut DOIVENT être prévus.

**INV-TAMR-8** (repris de la Documentation Fondatrice) : *Une escalade ne bloque pas indéfiniment le système.*

**R-ESC-3 : Motif explicite**

Toute escalade DOIT être accompagnée d'un motif explicite justifiant pourquoi le niveau supérieur est sollicité.

**R-ESC-4 : Traçabilité du chemin**

Le chemin complet de l'escalade (niveaux traversés, moments, décisions intermédiaires) DOIT être tracé.

**R-ESC-5 : Comportement par défaut**

Le comportement en cas de non-résolution de l'escalade dans le délai imparti DOIT être explicitement défini.

### 5.5. Niveaux d'escalade

Les niveaux d'escalade sont définis conceptuellement. Chaque produit définit sa propre chaîne de responsabilité :

| Niveau conceptuel | Description |
|-------------------|-------------|
| **Niveau 1** | Opérateur initial / Utilisateur concerné |
| **Niveau 2** | Superviseur direct / Manager |
| **Niveau 3** | Autorité fonctionnelle / Administrateur |
| **Niveau 4** | Autorité supérieure / Direction |
| **Niveau N** | Niveaux supplémentaires selon le produit |

### 5.6. Données de traçabilité

Toute escalade DOIT être tracée avec :

| Donnée | Obligatoire | Description |
|--------|-------------|-------------|
| `intervention_id` | ✅ Oui | Identifiant unique de l'intervention |
| `type` | ✅ Oui | Toujours "ESCALATION" |
| `initiator_id` | ✅ Oui | Identité de l'humain qui escalade |
| `motif` | ✅ Oui | Motif explicite de l'escalade |
| `escalation_path` | ✅ Oui | Chemin d'escalade prévu |
| `current_level` | ✅ Oui | Niveau actuel dans la chaîne |
| `initiated_at` | ✅ Oui | Moment de l'initiation (horodatage local) |
| `resolved_at` | Selon état | Moment de la résolution |
| `resolver_id` | Selon état | Identité du résolveur final |
| `resolution` | Selon état | Décision finale de l'escalade |
| `subject` | ✅ Oui | Sujet de l'escalade |
| `context` | ✅ Oui | Contexte complet |
| `timeout_behavior` | ✅ Oui | Comportement prévu en cas de timeout |

---

## 6. Type SUPERVISION (Observation avec capacité d'intervention)

### 6.1. Définition

Une **supervision** est un type d'intervention où l'humain **observe** le système de manière continue, avec la capacité de déclencher une intervention si nécessaire.

L'humain surveille et peut intervenir, mais n'intervient pas par défaut.

### 6.2. Caractéristiques

| Caractéristique | Description |
|-----------------|-------------|
| **Passive par défaut** | La supervision observe sans modifier le comportement normal |
| **Activable** | Le superviseur peut déclencher une intervention si nécessaire |
| **Continue** | La supervision s'étend sur une période, pas sur un instant |
| **Non intrusive** | La supervision n'affecte pas le fonctionnement normal du système |
| **Terminable** | La supervision peut se terminer explicitement ou par timeout |

### 6.3. États d'une supervision

Une supervision traverse les états suivants :

```
1. ACTIVÉE      → La supervision est active, l'humain observe
2. INTERVENUE   → Le superviseur a déclenché une intervention
3. TERMINÉE     → La supervision est terminée (explicitement ou par timeout)
```

### 6.4. Règles de supervision

**R-SUP-1 : Identité du superviseur**

Toute supervision DOIT identifier l'humain superviseur. Une supervision anonyme est invalide.

**R-SUP-2 : Périmètre défini**

Toute supervision DOIT avoir un périmètre défini (ce qui est observé, ce qui peut déclencher une intervention).

**R-SUP-3 : Durée limitée**

Toute supervision DOIT avoir une durée définie (explicite ou par timeout). Une supervision infinie est invalide.

**R-SUP-4 : Non-interférence**

La supervision en état passif NE DOIT JAMAIS modifier le comportement du système. L'observation est neutre.

**R-SUP-5 : Intervention typée**

Si le superviseur déclenche une intervention, cette intervention DOIT être d'un des autres types (APPROVAL, OVERRIDE, ou ESCALATION).

### 6.5. Données de traçabilité

Toute supervision DOIT être tracée avec :

| Donnée | Obligatoire | Description |
|--------|-------------|-------------|
| `intervention_id` | ✅ Oui | Identifiant unique de l'intervention |
| `type` | ✅ Oui | Toujours "SUPERVISION" |
| `supervisor_id` | ✅ Oui | Identité du superviseur |
| `scope` | ✅ Oui | Périmètre de la supervision |
| `started_at` | ✅ Oui | Moment de début (horodatage local) |
| `ended_at` | Selon état | Moment de fin (horodatage local) |
| `end_reason` | Selon état | Raison de la fin (explicite, timeout, intervention) |
| `interventions_triggered` | ❌ Non | Liste des interventions déclenchées pendant la supervision |
| `duration_planned` | ✅ Oui | Durée prévue de la supervision |
| `context` | ✅ Oui | Contexte de la supervision |

---

## 7. Relations entre les types

### 7.1. Matrice de compatibilité

Les types peuvent être liés dans certaines conditions :

| Type initial | Peut déclencher | Condition |
|--------------|-----------------|-----------|
| **APPROVAL** | ESCALATION | Si l'approbateur souhaite déléguer la décision |
| **OVERRIDE** | ESCALATION | Si l'override nécessite une autorisation supérieure |
| **ESCALATION** | APPROVAL | Si le niveau supérieur demande une validation |
| **ESCALATION** | OVERRIDE | Si le niveau supérieur décide d'overrider |
| **SUPERVISION** | APPROVAL | Si le superviseur demande une validation |
| **SUPERVISION** | OVERRIDE | Si le superviseur contredit une décision |
| **SUPERVISION** | ESCALATION | Si le superviseur escalade |

### 7.2. Règles de relation

**R-REL-1 : Traçabilité des liens**

Lorsqu'une intervention en déclenche une autre, le lien DOIT être tracé explicitement.

**R-REL-2 : Indépendance des traces**

Chaque intervention a sa propre trace, même si elle est déclenchée par une autre.

**R-REL-3 : Non-circularité**

Une chaîne d'interventions NE PEUT JAMAIS être circulaire (A déclenche B qui déclenche A).

---

## 8. Invariants des types d'intervention

### 8.1. Invariants communs à tous les types

**INV-TYPE-3 : Traçabilité absolue**

Toute intervention, quel que soit son type, est tracée avec toutes les données obligatoires.

**INV-TYPE-4 : Identité obligatoire**

Toute intervention identifie l'humain intervenant. Aucune intervention anonyme n'est valide.

**INV-TYPE-5 : Non-exécution par TAMR**

TAMR définit les types d'intervention mais n'exécute jamais une intervention. L'exécution est la responsabilité du produit.

**INV-TYPE-6 : Non-décision par TAMR**

TAMR définit les types d'intervention mais ne décide jamais si une intervention est autorisée. La décision appartient à StrongFather.

### 8.2. Table récapitulative des invariants par type

| Invariant | APPROVAL | OVERRIDE | ESCALATION | SUPERVISION |
|-----------|----------|----------|------------|-------------|
| Traçabilité obligatoire | ✅ | ✅ | ✅ | ✅ |
| Identité obligatoire | ✅ | ✅ | ✅ | ✅ |
| Terminaison garantie | ✅ | ✅ | ✅ | ✅ |
| Justification obligatoire | ❌ | ✅ | ✅ | ❌ |
| Limites infranchissables | ❌ | ✅ | ❌ | ❌ |
| Non-blocage | ❌ | ❌ | ✅ | ❌ |
| Durée limitée | ❌ | ❌ | ❌ | ✅ |

---

## 9. Cas d'usage typiques

### 9.1. Cas d'usage APPROVAL

| Contexte | Description |
|----------|-------------|
| **Publication de contenu** | Un article nécessite une approbation éditoriale avant publication |
| **Dépense financière** | Une dépense supérieure à un seuil nécessite une approbation managériale |
| **Accès sensible** | Un accès à des données sensibles nécessite une approbation de sécurité |
| **Modification de configuration** | Un changement de configuration critique nécessite une validation |

### 9.2. Cas d'usage OVERRIDE

| Contexte | Description |
|----------|-------------|
| **Blocage injustifié** | Une action légitime est bloquée par une règle trop stricte |
| **Urgence métier** | Une situation d'urgence nécessite de contourner une validation normale |
| **Erreur de règle** | Une règle automatique produit un résultat manifestement incorrect |
| **Cas exceptionnel** | Une situation non prévue par les règles automatiques |

### 9.3. Cas d'usage ESCALATION

| Contexte | Description |
|----------|-------------|
| **Doute sur la décision** | L'approbateur initial doute de la décision à prendre |
| **Conflit de règles** | Plusieurs règles contradictoires s'appliquent |
| **Impact important** | La décision a un impact significatif nécessitant un niveau supérieur |
| **Hors compétence** | La décision dépasse les compétences du niveau actuel |

### 9.4. Cas d'usage SUPERVISION

| Contexte | Description |
|----------|-------------|
| **Surveillance de sécurité** | Un administrateur surveille les accès sensibles |
| **Observation de processus** | Un superviseur observe le déroulement d'un processus critique |
| **Monitoring opérationnel** | Un opérateur surveille les opérations automatisées |
| **Audit temps réel** | Un auditeur observe les actions pour conformité |

---

## 10. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les **Lois d'Autonomie Système** définies dans [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ **Conforme**

Les types d'intervention sont définis conceptuellement et ne nécessitent aucune dépendance externe. Toute intervention peut être évaluée et tracée localement.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

En mode isolé :
- Les **approbations** peuvent être accordées ou refusées localement
- Les **overrides** peuvent être effectués localement avec traçabilité locale
- Les **escalades** prévoient un comportement par défaut en cas d'indisponibilité du niveau supérieur
- Les **supervisions** peuvent être actives localement

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ **Conforme**

Les horodatages utilisés sont locaux. Aucune comparaison temporelle entre nœuds n'est requise pour le fonctionnement des types d'intervention.

---

## 11. Règles de fermeture du contrat

### 11.1. Contrat fermé

Ce contrat est **fermé**. Seuls les types, les caractéristiques, les règles, et les invariants explicitement définis dans ce contrat sont autorisés. Tout type, caractéristique, règle, ou invariant non explicitement défini est **interdit**.

### 11.2. Interdictions explicites

- **INTERD-TYPE-1** : Aucun type d'intervention non défini dans ce contrat n'est reconnu
- **INTERD-TYPE-2** : Aucune caractéristique non définie dans ce contrat n'est applicable
- **INTERD-TYPE-3** : Aucune règle non définie dans ce contrat n'est exécutoire
- **INTERD-TYPE-4** : Aucun invariant non défini dans ce contrat n'est garanti

### 11.3. Conditions d'évolution

Ce contrat peut être évolué uniquement selon les conditions suivantes :

1. **Modification explicite** : Toute modification doit être explicite et documentée
2. **Rétrocompatibilité** : Toute modification doit préserver la rétrocompatibilité
3. **Validation contractuelle** : Toute modification doit être validée selon les processus contractuels
4. **Préservation des invariants** : Les invariants fondamentaux de TAMR doivent être préservés

---

## 12. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les types d'intervention humaine dans TAMR.

Il garantit que :
- les quatre types (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) sont exhaustifs et fermés,
- chaque type a des caractéristiques distinctives clairement définies,
- chaque type a des règles spécifiques non négociables,
- la traçabilité est obligatoire pour tous les types,
- l'identité de l'intervenant est toujours connue,
- les invariants de TAMR sont respectés,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4  
**Type :** Contrat de définition des types d'intervention non négociable

---

## 13. Mini log de génération

### Warning W1 : Exhaustivité des types

**Warning rencontré :** Risque d'oubli de types d'intervention ou de confusion avec d'autres concepts.

**Décision prise :** Définition d'une liste fermée et exhaustive de 4 types (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) strictement alignée sur la Documentation Fondatrice TAMR.

**Correction effectuée :** Section 2 rédigée avec liste exhaustive et INV-TYPE-1 établissant que la liste est fermée.

### Warning W2 : Confusion OVERRIDE et limites infranchissables

**Warning rencontré :** Risque de confusion entre les overrides autorisés et les limites qui ne peuvent jamais être franchies.

**Décision prise :** Rappel explicite de INV-TAMR-3 (limites infranchissables) dans les règles d'override et référence au contrat dédié.

**Correction effectuée :** R-OVER-2 établit clairement que les limites infranchissables ne peuvent jamais être overridées.

### Ambiguïté A1 : SUPERVISION et déclenchement d'autres types

**Ambiguïté rencontrée :** Comment la SUPERVISION peut-elle déclencher d'autres interventions sans violer l'unicité de type ?

**Décision prise :** La SUPERVISION peut déclencher d'autres interventions (APPROVAL, OVERRIDE, ESCALATION), mais chaque intervention reste de son propre type avec sa propre trace. La SUPERVISION est le contexte, pas le type de l'intervention déclenchée.

**Correction effectuée :** Section 7 détaille les relations entre types avec règles de traçabilité des liens.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice TAMR : Confirmée (4 types identiques)
- ✅ Cohérence avec INV-TAMR-1 à INV-TAMR-8 : Confirmée
- ✅ Cohérence avec Lois d'Autonomie : Confirmée (LOI-1, LOI-2, LOI-4)
- ✅ TAMR ne décide pas : Confirmée (INV-TYPE-6)
- ✅ TAMR n'exécute pas : Confirmée (INV-TYPE-5)
- ✅ Traçabilité absolue : Confirmée (INV-TYPE-3)
- ✅ Contrat fermé : Confirmée (section 11)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
