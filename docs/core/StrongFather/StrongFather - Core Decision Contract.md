# StrongFather — Core Decision Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Core Decision Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit la surface d'évaluation unique et autorisée entre les adaptateurs produits et StrongFather, constituant l'unique point d'entrée légal vers l'évaluation d'intentions, l'application de politiques, et la production de décisions dans le système Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle des décisions StrongFather, les types de décisions autorisées, les entrées acceptées, les sorties garanties, les garanties décisionnelles, et les règles absolues d'évaluation et de production de décisions.

### Portée

Ce contrat s'applique à **toutes les décisions produites par StrongFather** et définit de manière absolue :
- la définition formelle d'une décision StrongFather et son rôle systémique,
- le principe d'unicité de la surface d'évaluation,
- la typologie conceptuelle des décisions autorisées,
- les différences formelles entre décision acceptée, refusée, ambiguë, et différée,
- ce qu'une décision PEUT et NE PEUT JAMAIS représenter,
- les règles absolues d'évaluation et de production,
- les garanties offertes aux adaptateurs SF-compliant,
- les invariants systémiques associés.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **StrongFather — Documentation Fondatrice** : Définition philosophique et fonctionnelle de StrongFather
- **StrongFather — Intent Model Contract** : Modèle conceptuel des intentions
- **StrongFather — Policy Engine Contract** : Moteur d'application des politiques
- **StrongFather — Execution Prohibition Contract** : Interdiction absolue d'exécution
- **KindMother — CoreDataAPI Contract** : StrongFather n'exécute pas d'opérations CoreDataAPI

Il n'introduit aucune contradiction, et constitue la définition formelle de ce que signifie produire une décision dans StrongFather.

---

## 2. Définition d'une décision StrongFather

### Nature d'une décision

Une **décision StrongFather** est le résultat conceptuel produit par StrongFather après évaluation d'une intention selon des politiques et des contraintes. Une décision est un jugement stratégique et politique qui indique la validité, la priorité, ou le besoin de clarification d'une intention, sans jamais posséder d'autorité sur l'exécution ou la persistance.

### Ce qu'une décision représente

Une décision StrongFather représente :

1. **Un jugement stratégique** : L'évaluation d'une intention selon des politiques et des contraintes stratégiques
2. **Une indication de validité** : La détermination de la validité d'une intention selon les politiques applicables
3. **Une priorité relative** : L'ordre d'importance d'une intention par rapport à d'autres intentions
4. **Une demande de clarification** : L'identification d'éléments manquants ou insuffisamment définis dans une intention
5. **Une justification** : L'explication conceptuelle de la décision produite, incluant les politiques appliquées et les raisons de la décision

### Ce qu'une décision ne représente jamais

Une décision StrongFather ne représente **jamais** :

1. **Une commande d'exécution** : Une décision n'est pas une instruction d'exécution. Elle ne peut pas être exécutée directement.
2. **Une modification d'état** : Une décision ne modifie jamais un état ou un fait. Elle est purement consultative.
3. **Une persistance** : Une décision n'est pas persistée par StrongFather. Elle est produite et retournée, mais n'est pas stockée.
4. **Une garantie d'exécution** : Une décision acceptée ne garantit pas l'exécution. L'exécution reste la responsabilité de l'appelant.
5. **Une autorité sur le temps** : Une décision ne contient aucune logique temporelle technique. Elle ne gère jamais le temps, les horodatages, ou l'ordonnancement. Cette absence de logique temporelle garantit la conformité à **LOI-4** (pas de temps global requis) : les décisions ne dépendent pas d'une horloge réseau, d'un ordre global, ou de timestamps synchronisés entre nœuds.
6. **Une validation technique** : Une décision ne valide jamais la structure technique des données. Elle évalue uniquement la validité stratégique et politique.

### Nature systémique

Une décision StrongFather est un **concept systémique**, pas un objet technique. Elle représente un jugement stratégique produit par le moteur de décision, sans présupposer aucune technologie, aucun format de données, ou aucun mécanisme de transmission.

**Important :** Cette définition est purement conceptuelle et systémique. Elle ne présuppose aucune structure technique, aucun format de sérialisation, ou aucun protocole de communication.

---

## 3. Types de décisions autorisées

### 3.1. Décision acceptée

**Définition formelle :**

Une **décision acceptée** est une décision StrongFather qui indique qu'une intention est valide selon les politiques et les contraintes évaluées, et peut être considérée pour exécution par l'appelant.

**Caractéristiques :**

- **Validité confirmée** : L'intention est valide selon toutes les politiques applicables
- **Contraintes satisfaites** : Toutes les contraintes requises sont satisfaites
- **Priorité établie** : Une priorité relative est établie pour l'intention
- **Justification fournie** : La justification de l'acceptation est fournie avec les politiques appliquées
- **Non-exécutable** : L'acceptation ne déclenche jamais d'exécution automatique

**Contenu obligatoire :**

- L'identifiant de l'intention évaluée
- Le résultat : ACCEPTÉE
- Les politiques appliquées qui ont conduit à l'acceptation
- La priorité relative établie
- La justification de l'acceptation
- Le contexte d'évaluation

**Garanties :**

- Une décision acceptée est toujours justifiable selon les politiques
- Une décision acceptée ne garantit pas l'exécution
- Une décision acceptée ne modifie jamais un état

### 3.2. Décision refusée

**Définition formelle :**

Une **décision refusée** est une décision StrongFather qui indique qu'une intention est invalide selon les politiques et les contraintes évaluées, et ne doit pas être exécutée.

**Caractéristiques :**

- **Invalidité confirmée** : L'intention est invalide selon au moins une politique applicable
- **Contrainte violée** : Au moins une contrainte requise n'est pas satisfaite
- **Raison explicite** : La raison du refus est explicite et détaillée
- **Justification fournie** : La justification du refus est fournie avec les politiques violées
- **Définitivité** : Un refus est définitif pour l'intention évaluée sans modification

**Contenu obligatoire :**

- L'identifiant de l'intention évaluée
- Le résultat : REFUSÉE
- Les politiques violées qui ont conduit au refus
- La raison explicite du refus
- La justification détaillée du refus
- Le contexte d'évaluation

**Garanties :**

- Une décision refusée est toujours justifiable selon les politiques
- Une décision refusée est définitive pour l'intention évaluée
- Une décision refusée ne peut pas être réévaluée sans modification de l'intention ou des politiques

### 3.3. Décision ambiguë

**Définition formelle :**

Une **décision ambiguë** est une décision StrongFather qui indique qu'une intention est insuffisamment définie pour être évaluée, et nécessite des clarifications avant évaluation.

**Caractéristiques :**

- **Information manquante** : L'intention contient des informations manquantes ou insuffisamment définies
- **Clarifications requises** : Des clarifications explicites sont requises pour permettre l'évaluation
- **Suspension d'évaluation** : L'évaluation est suspendue jusqu'à clarification
- **Pas de priorité** : Aucune priorité ne peut être calculée tant que l'ambiguïté persiste
- **Non-définitive** : Une ambiguïté n'est pas un refus. Une fois clarifiée, l'intention peut être réévaluée

**Contenu obligatoire :**

- L'identifiant de l'intention évaluée
- Le résultat : AMBIGUË
- Les éléments manquants ou insuffisamment définis
- Les clarifications nécessaires
- Les politiques qui nécessitent ces clarifications
- Le contexte d'évaluation

**Garanties :**

- Une décision ambiguë suspend toute évaluation ultérieure de l'intention
- Une décision ambiguë empêche le calcul de toute priorité
- Une décision ambiguë n'est pas un refus et permet la réévaluation après clarification

### 3.4. Décision différée

**Définition formelle :**

Une **décision différée** est une décision StrongFather qui indique qu'une intention ne peut pas être évaluée immédiatement car elle dépend d'un contexte futur qui n'est pas encore disponible.

**Caractéristiques :**

- **Contexte futur requis** : L'évaluation nécessite un contexte qui n'est pas encore disponible
- **Dépendance temporelle conceptuelle** : La décision dépend d'un événement ou d'un état futur (pas technique)
- **Suspension d'évaluation** : L'évaluation est suspendue jusqu'à disponibilité du contexte
- **Pas de priorité** : Aucune priorité ne peut être calculée tant que le contexte n'est pas disponible
- **Réévaluation possible** : Une fois le contexte disponible, l'intention peut être réévaluée

**Contenu obligatoire :**

- L'identifiant de l'intention évaluée
- Le résultat : DIFFÉRÉE
- Le contexte futur requis
- Les politiques qui nécessitent ce contexte
- La justification de la différation
- Le contexte d'évaluation

**Garanties :**

- Une décision différée suspend toute évaluation ultérieure jusqu'à disponibilité du contexte
- Une décision différée empêche le calcul de toute priorité
- Une décision différée permet la réévaluation une fois le contexte disponible

**Distinction avec décision ambiguë :**

- **Ambiguë** : Information manquante dans l'intention elle-même
- **Différée** : Contexte futur requis pour l'évaluation, indépendamment de l'intention

---

## 4. Entrées acceptées

### 4.1. Faits

**Définition :**

Un **fait** est une information déclarative qui représente un état ou une réalité observable, sans interprétation ou jugement.

**Caractéristiques :**

- **Déclaratif** : Un fait est déclaré, pas calculé ou dérivé
- **Observable** : Un fait représente une réalité observable
- **Non-interprété** : Un fait n'est pas interprété ou jugé
- **Statique** : Un fait représente un état à un moment donné

**Exemples conceptuels :**

- L'utilisateur X existe
- L'entité Y a été créée
- La limite Z est définie à 100
- La politique P est active

**Utilisation :**

Les faits sont utilisés par StrongFather pour évaluer les intentions selon les politiques. StrongFather ne modifie jamais les faits fournis.

### 4.2. États

**Définition :**

Un **état** est une représentation conceptuelle de la condition ou de la situation d'une entité, d'un système, ou d'un contexte à un moment donné.

**Caractéristiques :**

- **Représentation conceptuelle** : Un état est une représentation, pas la réalité elle-même
- **Momentané** : Un état représente une condition à un moment donné
- **Non-technique** : Un état est conceptuel, pas technique (pas de structure de données, schémas, etc.)

**Exemples conceptuels :**

- L'utilisateur X est actif
- L'entité Y est en cours de validation
- Le système est en mode maintenance
- La synchronisation est en cours

**Utilisation :**

Les états sont utilisés par StrongFather pour évaluer les intentions selon les politiques. StrongFather ne modifie jamais les états fournis.

### 4.3. Signaux

**Définition :**

Un **signal** est une notification conceptuelle d'un événement ou d'un changement, sans détail d'implémentation technique.

**Caractéristiques :**

- **Notification conceptuelle** : Un signal notifie un événement ou un changement
- **Non-technique** : Un signal est conceptuel, pas technique (pas de protocole, format, etc.)
- **Événementiel** : Un signal représente un événement ou un changement

**Exemples conceptuels :**

- Signal de limite atteinte
- Signal de changement de politique
- Signal de disponibilité de contexte
- Signal de modification d'état

**Utilisation :**

Les signaux sont utilisés par StrongFather pour déclencher des réévaluations ou des mises à jour de contexte. StrongFather ne génère jamais de signaux techniques.

### 4.4. Contexte

**Définition :**

Le **contexte** est l'ensemble des informations environnementales nécessaires à l'évaluation d'une intention selon les politiques.

**Caractéristiques :**

- **Environnemental** : Le contexte représente l'environnement d'évaluation
- **Complet** : Le contexte doit être complet pour permettre l'évaluation
- **Non-technique** : Le contexte est conceptuel, pas technique

**Composants conceptuels :**

- Contexte utilisateur (identité de l'appelant, rôles, permissions)
- Contexte produit (produit, instance, domaine)
- Contexte temporel conceptuel (pas technique : saison, période, cycle)
- Contexte opérationnel (mode, état du système)

**Utilisation :**

Le contexte est utilisé par StrongFather pour évaluer les intentions selon les politiques. StrongFather ne modifie jamais le contexte fourni.

### 4.5. Métadonnées

**Définition :**

Les **métadonnées** sont des informations descriptives sur une intention, sans être l'intention elle-même.

**Caractéristiques :**

- **Descriptives** : Les métadonnées décrivent l'intention
- **Non-essentielles** : Les métadonnées ne sont pas essentielles à l'intention elle-même
- **Informatives** : Les métadonnées informent sur l'intention

**Exemples conceptuels :**

- Priorité demandée par l'appelant
- Source de l'intention
- Métadonnées de traçabilité
- Tags ou catégories

**Utilisation :**

Les métadonnées sont utilisées par StrongFather pour informer l'évaluation, mais ne sont pas évaluées elles-mêmes. StrongFather peut utiliser les métadonnées pour établir des priorités ou pour la traçabilité.

### 4.6. Entrées explicitement rejetées

Les entrées suivantes sont **explicitement rejetées** par StrongFather :

1. **Commandes d'exécution** : StrongFather n'accepte jamais de commandes d'exécution. Seules les intentions d'évaluation sont acceptées.

2. **Modifications d'état** : StrongFather n'accepte jamais de demandes de modification d'état. StrongFather ne modifie jamais d'états.

3. **Accès à la persistance** : StrongFather n'accepte jamais d'accès à la persistance. StrongFather ne lit jamais de données persistées.

4. **Logique temporelle technique** : StrongFather n'accepte jamais de logique temporelle technique (horodatages, timestamps, ordonnancement technique).

5. **Validation technique** : StrongFather n'accepte jamais de demandes de validation technique (structure de données, schémas, formats).

6. **Règles métier spécifiques** : StrongFather n'accepte jamais de règles métier spécifiques. Seules les politiques générales sont acceptées.

7. **Appels à KindMother** : StrongFather n'accepte jamais d'appels à KindMother. StrongFather ne connaît pas KindMother.

8. **Appels au kernel** : StrongFather n'accepte jamais d'appels directs au kernel. StrongFather n'utilise pas le kernel directement.

---

## 5. Sorties garanties

### 5.1. Structure minimale d'une décision

Toute décision produite par StrongFather contient **obligatoirement** les éléments suivants :

1. **Identifiant de l'intention** : L'identifiant unique de l'intention évaluée
2. **Résultat** : Le type de décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
3. **Politiques appliquées** : La liste des politiques appliquées lors de l'évaluation
4. **Justification** : L'explication conceptuelle de la décision produite
5. **Contexte d'évaluation** : Le contexte utilisé pour l'évaluation

### 5.2. Champs obligatoires

**Pour toute décision :**

- `intention_id` : Identifiant unique de l'intention évaluée
- `result` : Type de décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
- `policies_applied` : Liste des politiques appliquées
- `justification` : Justification conceptuelle de la décision
- `evaluation_context` : Contexte d'évaluation utilisé

**Pour décision ACCEPTÉE :**

- `priority` : Priorité relative établie
- `acceptance_reason` : Raison de l'acceptation

**Pour décision REFUSÉE :**

- `rejection_reason` : Raison explicite du refus
- `policies_violated` : Liste des politiques violées
- `violation_details` : Détails des violations

**Pour décision AMBIGUË :**

- `missing_elements` : Liste des éléments manquants
- `clarifications_required` : Clarifications nécessaires
- `policies_requiring_clarification` : Politiques nécessitant ces clarifications

**Pour décision DIFFÉRÉE :**

- `required_future_context` : Contexte futur requis
- `deferral_reason` : Raison de la différation
- `policies_requiring_context` : Politiques nécessitant ce contexte

### 5.3. Champs interdits

Les champs suivants sont **strictement interdits** dans toute décision StrongFather :

1. **Commandes d'exécution** : Aucune commande d'exécution n'est autorisée dans une décision
2. **Modifications d'état** : Aucune modification d'état n'est autorisée dans une décision
3. **Accès à la persistance** : Aucun accès à la persistance n'est autorisé dans une décision
4. **Logique temporelle technique** : Aucune logique temporelle technique n'est autorisée dans une décision
5. **Validation technique** : Aucune validation technique n'est autorisée dans une décision
6. **Appels à d'autres composants** : Aucun appel à KindMother, au kernel, ou à d'autres composants n'est autorisé dans une décision

### 5.4. Propriétés garanties

Toute décision produite par StrongFather garantit les propriétés suivantes :

1. **Justifiabilité** : Toute décision est justifiable selon les politiques appliquées
2. **Non-exécutabilité** : Toute décision est non exécutable. Elle ne peut pas être exécutée directement
3. **Non-persistance** : Toute décision est non persistante. Elle n'est pas stockée par StrongFather
4. **Indépendance temporelle technique** : Toute décision est indépendante du temps technique
5. **Zero-trust** : Toute décision est produite en zero-trust, sans présupposer la validité de l'appelant
6. **Non-ambiguïté** : Toute décision est non ambiguë. Elle est claire et explicite

---

## 6. Garanties décisionnelles

### 6.1. Déterminisme décisionnel

**Définition :**

Le **déterminisme décisionnel** est la propriété garantissant que pour une intention donnée, un contexte donné, et des politiques données, StrongFather produit toujours la même décision.

**Caractéristiques :**

- **Reproductibilité** : Une même intention, avec le même contexte et les mêmes politiques, produit toujours la même décision
- **Indépendance de l'ordre** : L'ordre d'évaluation des intentions n'affecte pas les décisions individuelles
- **Cohérence** : Les décisions sont cohérentes entre elles selon les politiques

**Garanties :**

- **G-DEC-1** : Pour une intention I, un contexte C, et des politiques P, StrongFather produit toujours la même décision
- **G-DEC-2** : L'ordre d'évaluation n'affecte pas les décisions individuelles
- **G-DEC-3** : Les décisions sont cohérentes selon les politiques appliquées

### 6.2. Justifiabilité

**Définition :**

La **justifiabilité** est la propriété garantissant que toute décision produite par StrongFather peut être justifiée de manière explicite selon les politiques appliquées.

**Caractéristiques :**

- **Explicite** : La justification est explicite et détaillée
- **Traçable** : La justification est traçable jusqu'aux politiques appliquées
- **Complète** : La justification couvre tous les aspects de la décision

**Garanties :**

- **G-JUST-1** : Toute décision contient une justification explicite
- **G-JUST-2** : Toute justification référence les politiques appliquées
- **G-JUST-3** : Toute justification est complète et non ambiguë

### 6.3. Non-exécution

**Définition :**

La **non-exécution** est la propriété garantissant qu'aucune décision produite par StrongFather ne peut être exécutée directement, et que StrongFather ne possède jamais d'autorité sur l'exécution.

**Caractéristiques :**

- **Non-exécutable** : Aucune décision n'est exécutable directement
- **Aucune autorité** : StrongFather ne possède jamais d'autorité sur l'exécution
- **Séparation stricte** : La décision est strictement séparée de l'exécution

**Garanties :**

- **G-NOEXEC-1** : Aucune décision n'est exécutable directement
- **G-NOEXEC-2** : StrongFather ne possède jamais d'autorité sur l'exécution
- **G-NOEXEC-3** : La décision est strictement séparée de l'exécution

### 6.4. Non-persistance

**Définition :**

La **non-persistance** est la propriété garantissant qu'aucune décision produite par StrongFather n'est persistée par StrongFather, et que StrongFather ne possède jamais d'autorité sur la persistance.

**Caractéristiques :**

- **Non-persistante** : Aucune décision n'est persistée par StrongFather
- **Aucune autorité** : StrongFather ne possède jamais d'autorité sur la persistance
- **Séparation stricte** : La décision est strictement séparée de la persistance

**Garanties :**

- **G-NOPERS-1** : Aucune décision n'est persistée par StrongFather
- **G-NOPERS-2** : StrongFather ne possède jamais d'autorité sur la persistance
- **G-NOPERS-3** : La décision est strictement séparée de la persistance

### 6.5. Absence de logique temporelle technique

**Définition :**

L'**absence de logique temporelle technique** est la propriété garantissant qu'aucune décision produite par StrongFather ne contient de logique temporelle technique, et que StrongFather ne gère jamais le temps technique.

**Caractéristiques :**

- **Pas de temps technique** : Aucune décision ne contient de logique temporelle technique
- **Aucune gestion du temps** : StrongFather ne gère jamais le temps technique
- **Indépendance temporelle** : Les décisions sont indépendantes du temps technique

**Garanties :**

- **G-NOTIME-1** : Aucune décision ne contient de logique temporelle technique
- **G-NOTIME-2** : StrongFather ne gère jamais le temps technique
- **G-NOTIME-3** : Les décisions sont indépendantes du temps technique

### 6.6. Zero-trust

**Définition :**

Le **zero-trust** est la propriété garantissant que StrongFather ne fait confiance à aucun appelant, et que toute intention est évaluée selon les politiques sans présupposer la validité, l'authenticité, ou la légitimité de l'appelant.

**Caractéristiques :**

- **Aucune confiance** : StrongFather ne fait confiance à aucun appelant
- **Évaluation selon politiques** : Toute intention est évaluée selon les politiques, sans présupposer la validité de l'appelant
- **Vérification systématique** : Toute information fournie par l'appelant est vérifiée selon les politiques

**Garanties :**

- **G-ZT-1** : StrongFather ne fait confiance à aucun appelant
- **G-ZT-2** : Toute intention est évaluée selon les politiques, sans présupposer la validité de l'appelant
- **G-ZT-3** : Toute information fournie par l'appelant est vérifiée selon les politiques

---

## 7. Non-garanties explicites

### 7.1. Performance

StrongFather **ne garantit pas** :

- Le temps d'évaluation d'une intention
- Le débit d'évaluation des intentions
- L'optimisation des performances
- La latence de production d'une décision

Les performances sont des considérations d'implémentation, pas des garanties contractuelles.

### 7.2. Exhaustivité des informations

StrongFather **ne garantit pas** :

- L'exhaustivité des informations fournies dans une décision
- La complétude des justifications
- L'inclusion de toutes les politiques applicables
- La présence de toutes les métadonnées possibles

StrongFather garantit uniquement les champs obligatoires définis dans ce contrat.

### 7.3. Ordonnancement

StrongFather **ne garantit pas** :

- L'ordre d'évaluation des intentions
- L'ordonnancement des décisions
- La priorité d'évaluation
- La séquence de traitement

L'ordonnancement est une considération d'implémentation, pas une garantie contractuelle.

### 7.4. Résolution automatique

StrongFather **ne garantit pas** :

- La résolution automatique des ambiguïtés
- La résolution automatique des conflits
- La génération automatique de clarifications
- La correction automatique des intentions

StrongFather produit des décisions, mais ne résout pas automatiquement les problèmes.

### 7.5. Convergence globale

StrongFather **ne garantit pas** :

- La convergence globale des décisions
- L'absence de contradictions entre décisions
- La cohérence globale du système
- L'optimalité des décisions

StrongFather garantit la cohérence selon les politiques appliquées, mais pas la convergence globale.

---

## 8. Règles de fermeture du contrat

### 8.1. Contrat fermé

Ce contrat est **fermé**. Seules les opérations, les entrées, les sorties, et les garanties explicitement définies dans ce contrat sont autorisées. Toute opération, entrée, sortie, ou garantie non explicitement définie est **interdite**.

### 8.2. Interdiction d'extension implicite

Aucune extension implicite de ce contrat n'est autorisée. Les règles suivantes s'appliquent :

- **INTERD-EXT-1** : Aucune opération non définie dans ce contrat n'est autorisée
- **INTERD-EXT-2** : Aucune entrée non définie dans ce contrat n'est acceptée
- **INTERD-EXT-3** : Aucune sortie non définie dans ce contrat n'est produite
- **INTERD-EXT-4** : Aucune garantie non définie dans ce contrat n'est offerte

### 8.3. Conditions d'évolution du contrat

Ce contrat peut être évolué uniquement selon les conditions suivantes :

1. **Modification explicite** : Toute modification doit être explicite et documentée
2. **Rétrocompatibilité** : Toute modification doit préserver la rétrocompatibilité avec les versions antérieures
3. **Validation contractuelle** : Toute modification doit être validée selon les processus contractuels
4. **Documentation complète** : Toute modification doit être documentée de manière complète

**Important :** Ce contrat est de statut FONDATION. Toute modification doit respecter ce statut et ne peut pas introduire de contradictions avec les autres contrats FONDATION.

---

## 9. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable ce que signifie produire une décision dans StrongFather.

Il garantit que :
- les décisions sont produites selon des règles strictes et non ambiguës,
- les entrées acceptées sont explicitement définies,
- les sorties garanties sont contractuellement établies,
- les garanties décisionnelles sont respectées,
- les non-garanties sont explicitement déclarées,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 10. Validation conceptuelle

### 10.1. Cas de décisions valides

Les cas suivants sont **valides** selon ce contrat :

1. **Décision acceptée avec justification complète** : Une intention valide produit une décision acceptée avec justification complète, politiques appliquées, et priorité établie.

2. **Décision refusée avec raison explicite** : Une intention invalide produit une décision refusée avec raison explicite, politiques violées, et justification détaillée.

3. **Décision ambiguë avec clarifications** : Une intention insuffisamment définie produit une décision ambiguë avec éléments manquants, clarifications nécessaires, et politiques nécessitant ces clarifications.

4. **Décision différée avec contexte requis** : Une intention dépendant d'un contexte futur produit une décision différée avec contexte requis, raison de différation, et politiques nécessitant ce contexte.

### 10.2. Cas de décisions invalides

Les cas suivants sont **invalides** et violent explicitement ce contrat :

1. **Décision avec commande d'exécution** : Toute décision contenant une commande d'exécution viole G-NOEXEC-1 (non-exécution).

2. **Décision avec modification d'état** : Toute décision contenant une modification d'état viole la section 2 "Ce qu'une décision ne représente jamais" (point 2).

3. **Décision avec accès à la persistance** : Toute décision contenant un accès à la persistance viole G-NOPERS-1 (non-persistance).

4. **Décision avec logique temporelle technique** : Toute décision contenant de la logique temporelle technique viole G-NOTIME-1 (absence de logique temporelle technique).

5. **Décision sans justification** : Toute décision sans justification explicite viole G-JUST-1 (justifiabilité).

6. **Décision avec appel à KindMother** : Toute décision contenant un appel à KindMother viole la section 4.6 "Entrées explicitement rejetées" (point 7).

7. **Décision avec appel au kernel** : Toute décision contenant un appel au kernel viole la section 4.6 "Entrées explicitement rejetées" (point 8).

8. **Décision avec champ non autorisé** : Toute décision contenant un champ non défini dans la section 5.2 "Champs obligatoires" et non autorisé viole la section 5.3 "Champs interdits".

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Contrat de décision non négociable (DOCUMENT MAÎTRE pour les types de décisions)

---

## 11. Mini log de génération

### Warning W1 : Distinction entre décision différée et ambiguë

**Warning rencontré :** Risque de confusion entre décision différée (contexte futur requis) et décision ambiguë (information manquante).

**Décision prise :** Clarification explicite dans la section 3.4 "Décision différée" avec distinction formelle : ambiguë = information manquante dans l'intention, différée = contexte futur requis indépendamment de l'intention.

**Correction effectuée :** Section 3.4 rédigée avec distinction explicite entre les deux types de décisions. Sous-section "Distinction avec décision ambiguë" ajoutée pour clarifier.

### Warning W2 : Justifiabilité vs exhaustivité

**Warning rencontré :** Risque de confusion entre justifiabilité (garantie contractuelle) et exhaustivité (non-garantie).

**Décision prise :** Clarification explicite que la justifiabilité garantit une justification explicite et traçable, mais pas l'exhaustivité complète. Section 6.2 "Justifiabilité" garantit la justification, section 7.2 "Exhaustivité des informations" déclare explicitement la non-garantie d'exhaustivité.

**Correction effectuée :** Sections 6.2 et 7.2 rédigées avec cette distinction. Garanties G-JUST-1, G-JUST-2, G-JUST-3 établissent la justifiabilité sans garantir l'exhaustivité.

### Ambiguïté A1 : Faits vs états

**Ambiguïté rencontrée :** Risque de confusion entre faits (informations déclaratives) et états (représentations conceptuelles de condition).

**Décision prise :** Clarification explicite dans les sections 4.1 "Faits" et 4.2 "États" : faits = informations déclaratives observables, états = représentations conceptuelles de condition. Distinction maintenue dans les exemples conceptuels.

**Correction effectuée :** Sections 4.1 et 4.2 rédigées avec distinction explicite. Exemples conceptuels fournis pour chaque type d'entrée.

### Ambiguïté A2 : Contexte temporel conceptuel vs technique

**Ambiguïté rencontrée :** Comment distinguer le contexte temporel conceptuel (saison, période, cycle) du temps technique (horodatages, timestamps) ?

**Décision prise :** Clarification explicite dans la section 4.4 "Contexte" : contexte temporel conceptuel = saison, période, cycle (pas technique). Section 4.6 "Entrées explicitement rejetées" liste explicitement la logique temporelle technique comme rejetée.

**Correction effectuée :** Section 4.4 précise "Contexte temporel conceptuel (pas technique : saison, période, cycle)". Section 4.6 liste explicitement "Logique temporelle technique" comme rejetée.

### Incohérence I1 : Décision différée et suspension d'évaluation

**Incohérence rencontrée :** Comment garantir que les décisions différées suspendent l'évaluation sans introduire de logique temporelle technique ?

**Décision prise :** La suspension d'évaluation est conceptuelle, pas technique. Elle représente l'attente d'un contexte futur conceptuel (événement, état), pas d'un temps technique. Section 3.4 "Décision différée" précise que la dépendance est "temporelle conceptuelle (pas technique)".

**Correction effectuée :** Section 3.4 rédigée avec précision sur la nature conceptuelle (pas technique) de la suspension. Garantie G-NOTIME-1, G-NOTIME-2, G-NOTIME-3 établissent l'absence de logique temporelle technique.

### Décision éditoriale E1 : Structure du document

**Décision prise :** Respect strict de la structure imposée par l'utilisateur. Aucune modification de l'ordre des sections. Chaque section est explicitement rédigée sans remplissage vague.

**Application :** Structure respectée exactement comme demandé. Chaque section contient du contenu substantiel et non ambigu.

### Décision éditoriale E2 : Ton contractuel

**Décision prise :** Utilisation d'un ton contractuel, normatif, non ambigu, comparable au niveau de rigueur du CoreDataAPI Contract de KindMother. Utilisation de formulations absolues ("ne fait jamais", "est interdit", "garantit").

**Application :** Tout le document utilise un ton contractuel avec des formulations absolues. Les garanties sont énoncées de manière non négociable.

### Décision éditoriale E3 : Section de validation conceptuelle

**Décision prise :** Ajout d'une section 10 "Validation conceptuelle" listant des cas valides vs invalides sans code, avec référence explicite aux violations contractuelles pour chaque cas invalide.

**Application :** Section 10 créée avec cas valides et invalides. Chaque cas invalide référence explicitement la violation contractuelle correspondante.

### Décision éditoriale E4 : Décision différée incluse

**Décision prise :** Inclusion de la décision différée dans la section 3.4, avec justification de sa nécessité (dépendance de contexte futur conceptuel) et distinction explicite avec la décision ambiguë.

**Application :** Section 3.4 rédigée avec décision différée. Distinction formelle avec décision ambiguë fournie.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (décisions non exécutables, non persistantes)
- ✅ Aucune autorité sur l'exécution : Confirmée (G-NOEXEC-1, G-NOEXEC-2, G-NOEXEC-3)
- ✅ Aucune autorité sur la persistance : Confirmée (G-NOPERS-1, G-NOPERS-2, G-NOPERS-3)
- ✅ Aucune logique temporelle technique : Confirmée (G-NOTIME-1, G-NOTIME-2, G-NOTIME-3)
- ✅ Zero-trust respecté : Confirmée (G-ZT-1, G-ZT-2, G-ZT-3)
- ✅ Justifiabilité garantie : Confirmée (G-JUST-1, G-JUST-2, G-JUST-3)
- ✅ Déterminisme décisionnel : Confirmé (G-DEC-1, G-DEC-2, G-DEC-3)
- ✅ Contrat fermé : Confirmé (section 8)
- ✅ Aucune dépendance technique : Confirmée
- ✅ Structure imposée respectée : Confirmée

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
