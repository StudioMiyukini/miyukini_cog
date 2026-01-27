# KindMother — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter KindMother correctement, sans violer les contrats FONDATION.

**Objectif pédagogique :** Ce document vise à aider les développeurs à comprendre comment traduire les contrats FONDATION en implémentation, en respectant strictement les invariants, garanties, et interdictions.

**Avertissement :** Ce document ne doit pas être interprété abusivement. Il ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats FONDATION priment toujours sur ce guide.

**Relation avec les contrats FONDATION :** Ce document fait référence aux contrats FONDATION existants mais ne les étend pas, ne les modifie pas, et ne crée aucune nouvelle obligation contractuelle.

---

## 1. Introduction

### 1.1. Objectif

Ce document fournit des lignes directrices pour implémenter KindMother de manière conforme aux contrats FONDATION. Il explique comment traduire les concepts contractuels en logique d'implémentation sans interprétation abusive.

### 1.2. Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats FONDATION.

### 1.3. Sources contractuelles

Ce document se base sur tous les contrats FONDATION, avec un focus particulier sur :

- **Instance Model Contract** : Invariants INST-*, responsabilités, droits, interdictions
- **CoreDataAPI Contract** : Unicité de la surface d'appel (UNIQ-*), interdictions (INTERDIT-*)
- **Runtime Boundary & Enforcement Contract** : Réponses systémiques (R1 à R4), violations (V1 à V7)
- **Persistence & Storage Contract** : Garanties de persistance (G-PERSIST-*), corruption (INV-CORR-*)
- **Write Intent Lifecycle Contract** : Cycle de vie des intentions, invariants (INV-LIFE-*)
- **[Miyukini Conceptual References — Lois Autonomie Système](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Les lignes directrices d'implémentation doivent respecter les 6 lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique), **LOI-2** (isolement comme état normal), **LOI-3** (état local souverain), **LOI-5** (coût proportionnel au hardware), et **LOI-6** (autonomie n'empêche pas la fédération).

---

## 2. Principes généraux à respecter absolument

### 2.1. Autorité exclusive de KindMother (INST-2)

**Principe contractuel :**

L'invariant INST-2 établit que toute instance reconnaît l'autorité exclusive de KindMother sur la validation, la cohérence, et l'intégrité des données. Aucune opération ne peut contourner cette autorité.

**Traduction en logique d'implémentation :**

- **Toute validation DOIT être effectuée par KindMother** : Aucune validation ne peut être déléguée à un adaptateur, même certifié KM-compliant. Toute opération DOIT passer par les validations de KindMother.

- **Aucun contournement n'est autorisé** : Aucun mécanisme ne peut permettre de contourner les validations de KindMother, même pour des raisons d'optimisation ou de performance.

- **L'autorité est non négociable** : Les décisions de validation de KindMother sont définitives et non négociables. Aucune exception ne peut être faite.

**Ce que cela signifie concrètement :**

- Toute opération CoreDataAPI DOIT traverser toutes les Runtime Boundaries avant exécution
- Aucune opération ne peut être exécutée sans validation préalable
- Les décisions de validation sont finales et ne peuvent pas être contestées

### 2.2. Validation obligatoire avant exécution (INST-6)

**Principe contractuel :**

L'invariant INST-6 établit que toute opération sur une instance DOIT être validée par KindMother avant exécution. Aucune opération non validée ne peut être exécutée.

**Traduction en logique d'implémentation :**

- **Validation systématique** : Chaque opération DOIT être validée avant exécution, sans exception. Aucune opération ne peut être exécutée sans validation préalable.

- **Ordre de validation** : Les validations DOIVENT être effectuées dans l'ordre des Runtime Boundaries (appel, contexte, instance, permissions, cohérence, contournement, charge).

- **Pas d'exécution partielle** : Si une validation échoue, l'opération est complètement rejetée. Aucune exécution partielle n'est autorisée.

**Ce que cela signifie concrètement :**

- Toute opération CoreDataAPI DOIT être validée avant d'être exécutée
- Si une validation échoue, l'opération est rejetée avec une erreur explicite
- L'état des données reste inchangé après un rejet

### 2.3. Isolation systémique (INST-3)

**Principe contractuel :**

L'invariant INST-3 établit que toute instance est isolée systémiquement des autres instances. Les données d'une instance ne sont pas directement accessibles depuis une autre instance.

**Traduction en logique d'implémentation :**

- **Isolation stricte** : Les données d'une instance DOIVENT être strictement isolées des données des autres instances. Aucun accès direct croisé n'est autorisé.

- **Communication contrôlée** : Toute communication entre instances DOIT passer par des mécanismes contrôlés par KindMother (synchronisation, Intentions Certifiées).

- **Isolation par domaine** : Au sein d'une instance, les données DOIVENT être isolées par Authority Domain. Aucun partage direct entre domaines n'est autorisé.

**Ce que cela signifie concrètement :**

- Aucun accès direct aux données d'une autre instance n'est autorisé
- Toute communication entre instances passe par la CoreDataAPI et les mécanismes de synchronisation
- Les données sont isolées par instance et par domaine d'autorité

### 2.4. Zero-trust systématique

**Principe contractuel :**

Le Runtime Boundary & Enforcement Contract établit que KindMother applique un principe de zero-trust : aucune confiance implicite n'est accordée à un appelant, même certifié KM-compliant.

**Traduction en logique d'implémentation :**

- **Validation à chaque appel** : Chaque appel CoreDataAPI DOIT être validé, même si l'adaptateur est certifié KM-compliant. Aucune confiance implicite n'est accordée.

- **Vérification systématique** : Toutes les préconditions DOIVENT être vérifiées à chaque appel, sans exception. Aucune information n'est supposée vraie sans vérification.

- **Pas d'exception pour conformité** : Même un adaptateur certifié KM-compliant DOIT passer par toutes les validations. Aucune exception n'est autorisée.

**Ce que cela signifie concrètement :**

- Chaque appel est validé indépendamment de la conformité de l'adaptateur
- Aucune information n'est supposée vraie sans vérification
- Toutes les Runtime Boundaries sont traversées à chaque appel

### 2.5. Traçabilité complète (INST-7)

**Principe contractuel :**

L'invariant INST-7 établit que toutes les opérations sur une instance DOIVENT être tracées de manière complète. Aucune opération ne peut être exécutée sans traçabilité.

**Traduction en logique d'implémentation :**

- **Traçabilité systématique** : Chaque opération DOIT être tracée avec son contexte complet, son résultat, et son moment d'exécution.

- **Traçabilité immuable** : Les traces DOIVENT être immuables. Aucune modification des traces n'est autorisée après leur création.

- **Traçabilité accessible** : Les traces DOIVENT être accessibles pour audit par les acteurs autorisés.

**Ce que cela signifie concrètement :**

- Toute opération est tracée avec son contexte complet
- Les traces sont immuables et accessibles pour audit
- Aucune opération ne peut être exécutée sans traçabilité

---

## 3. Comment traduire les contrats en logique sans interprétation abusive

### 3.1. Respecter les invariants comme contraintes absolues

**Principe :**

Les invariants contractuels (INST-*, INV-*) sont des contraintes absolues qui DOIVENT toujours être vraies. Ils ne sont pas des suggestions ou des recommandations.

**Traduction :**

- **Vérification systématique** : Chaque invariant DOIT être vérifié et préservé à chaque opération. Aucun invariant ne peut être violé, même temporairement.

- **Préservation garantie** : Toute opération DOIT garantir que les invariants sont préservés après exécution. Si une opération violerait un invariant, elle DOIT être rejetée.

- **Pas d'interprétation** : Les invariants ne peuvent pas être interprétés ou adaptés. Ils sont absolus et non négociables.

**Exemple conceptuel :**

Si l'invariant INST-8 (protection contre corruption) exige que toutes les opérations soient bloquées en cas de corruption détectée, alors aucune opération ne peut être exécutée sur une instance corrompue, même pour des raisons de "récupération" ou de "secours".

### 3.2. Implémenter les garanties comme obligations, pas comme suggestions

**Principe :**

Les garanties contractuelles (G-*) sont des obligations que KindMother DOIT respecter. Elles ne sont pas des objectifs ou des optimisations.

**Traduction :**

- **Garanties obligatoires** : Chaque garantie DOIT être respectée. Aucune garantie ne peut être ignorée ou relâchée.

- **Vérification de conformité** : L'implémentation DOIT vérifier que les garanties sont respectées. Si une garantie ne peut pas être respectée, l'opération DOIT être rejetée.

- **Pas d'optimisation au détriment des garanties** : Aucune optimisation ne peut compromettre une garantie. Les garanties priment sur toute considération de performance.

**Exemple conceptuel :**

Si la garantie G-PERSIST-2 (atomicité garantie) exige que toute opération de persistance soit atomique, alors aucune persistance partielle n'est autorisée, même pour des raisons de performance ou d'optimisation.

### 3.3. Traiter les interdictions comme non-négociables

**Principe :**

Les interdictions contractuelles (I-*, INTERDIT-*) sont absolues et non négociables. Elles ne peuvent pas être contournées, même pour des raisons pratiques.

**Traduction :**

- **Interdictions absolues** : Chaque interdiction DOIT être respectée. Aucune exception n'est autorisée.

- **Détection systématique** : Les tentatives de violation des interdictions DOIVENT être détectées et bloquées immédiatement.

- **Pas de contournement** : Aucun mécanisme ne peut permettre de contourner une interdiction. Les interdictions sont inviolables.

**Exemple conceptuel :**

Si l'interdiction INTERDIT-2 (exposition des données directement) interdit l'accès direct à la persistance, alors aucun mécanisme ne peut permettre un accès direct, même pour des raisons de "performance" ou de "commodité".

### 3.4. Ne pas "optimiser" en contournant les validations

**Principe :**

Aucune optimisation ne peut contourner les validations ou les règles contractuelles. Les validations sont obligatoires, même si elles semblent "redondantes" ou "inefficaces".

**Traduction :**

- **Validations obligatoires** : Toutes les validations DOIVENT être effectuées, même si elles semblent redondantes ou coûteuses.

- **Pas de cache de validation** : Les résultats de validation ne peuvent pas être mis en cache de manière à contourner les validations. Chaque appel DOIT être validé.

- **Pas d'optimisation au détriment de la sécurité** : Aucune optimisation ne peut compromettre la sécurité ou l'intégrité. Les validations priment sur toute considération de performance.

**Exemple conceptuel :**

Même si un adaptateur est certifié KM-compliant et a déjà été validé, chaque appel DOIT être validé à nouveau. Aucun cache de validation n'est autorisé.

---

## 4. Ce qu'un développeur ne doit jamais faire

### 4.1. Contourner la CoreDataAPI (UNIQ-1 à UNIQ-5)

**Interdiction contractuelle :**

Les règles UNIQ-1 à UNIQ-5 établissent que la CoreDataAPI est l'unique surface d'appel vers KindMother. Aucune surface d'appel alternative n'est autorisée.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Créer une surface d'appel alternative ou parallèle à la CoreDataAPI
- Permettre un accès direct aux données sans passer par la CoreDataAPI
- Créer des "raccourcis" ou des "optimisations" qui contournent la CoreDataAPI
- Exposer des mécanismes internes qui permettent de contourner la CoreDataAPI

**Conséquence de la violation :**

- Violation de l'invariant INV-API-1 (unicité de la surface d'appel)
- Violation des règles UNIQ-1 à UNIQ-5
- Compromission de l'autorité exclusive de KindMother
- Compromission de la traçabilité complète

### 4.2. Accéder directement à la persistance (INTERDIT-2)

**Interdiction contractuelle :**

L'interdiction INTERDIT-2 établit que la CoreDataAPI ne peut jamais exposer les données directement sans passer par les mécanismes de contrôle de KindMother. Aucun accès direct à la persistance n'est autorisé.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Exposer un accès direct à la persistance (base de données, système de fichiers, etc.)
- Permettre à un adaptateur d'accéder directement aux données stockées
- Créer des mécanismes de "lecture directe" ou d'"écriture directe"
- Exposer des détails d'implémentation de la persistance

**Conséquence de la violation :**

- Violation de l'interdiction INTERDIT-2
- Violation de l'invariant INST-4 (persistance interne)
- Compromission de l'isolation systémique (INST-3)
- Violation de **LOI-1** (aucune dépendance externe critique) : un accès direct à la persistance peut introduire des dépendances externes critiques, compromettant l'autonomie du système.
- Compromission de l'autorité exclusive de KindMother (INST-2)

### 4.3. Mettre de la logique métier dans les adaptateurs

**Principe contractuel :**

La CoreDataAPI fournit les opérations de données, pas la logique métier. La logique métier appartient aux adaptateurs, mais les validations appartiennent à KindMother.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Déplacer des validations métier dans les adaptateurs
- Permettre aux adaptateurs de prendre des décisions de validation
- Déléguer la responsabilité de validation aux adaptateurs
- Créer des "validations préalables" dans les adaptateurs qui contournent les validations de KindMother

**Conséquence de la violation :**

- Violation de l'invariant INST-6 (validation obligatoire)
- Violation de l'interdiction INTERDIT-8 (délégation de validation)
- Compromission de l'autorité exclusive de KindMother (INST-2)
- Compromission de la cohérence (les validations peuvent être contournées)

### 4.4. Accorder une confiance implicite

**Principe contractuel :**

Le principe de zero-trust établit qu'aucune confiance implicite n'est accordée à un appelant, même certifié KM-compliant.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Supposer qu'un adaptateur certifié KM-compliant est toujours valide
- Mettre en cache les résultats de validation pour éviter les validations répétées
- Accorder des "privilèges" ou des "exceptions" aux adaptateurs conformes
- Supposer que le contexte fourni par un adaptateur conforme est toujours valide

**Conséquence de la violation :**

- Violation du principe de zero-trust
- Compromission de la sécurité (les validations peuvent être contournées)
- Compromission de l'intégrité (des opérations non validées peuvent être exécutées)

### 4.5. Ignorer les erreurs de validation (INTERDIT-6)

**Interdiction contractuelle :**

L'interdiction INTERDIT-6 établit que la CoreDataAPI ne peut jamais ignorer une erreur de validation ou continuer après un rejet. Toute erreur DOIT être retournée à l'appelant.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Ignorer silencieusement une erreur de validation
- Continuer l'exécution après une validation échouée
- "Corriger" automatiquement une erreur de validation sans la retourner
- Masquer une erreur de validation pour "faciliter" l'utilisation

**Conséquence de la violation :**

- Violation de l'interdiction INTERDIT-6
- Violation de l'interdiction I4 (exécution silencieuse)
- Compromission de la traçabilité (les erreurs ne sont pas tracées)
- Compromission de l'intégrité (des opérations invalides peuvent être exécutées)

### 4.6. Exécuter partiellement une opération (INTERDIT-5)

**Interdiction contractuelle :**

L'interdiction INTERDIT-5 établit que la CoreDataAPI ne peut jamais exécuter partiellement une opération. Chaque opération est atomique : tout ou rien.

**Ce qu'un développeur ne doit JAMAIS faire :**

- Exécuter partiellement une opération même si une partie échoue
- Laisser un état intermédiaire après une erreur
- Appliquer certaines modifications d'une opération batch même si d'autres échouent
- Permettre une "exécution optimiste" qui peut laisser des états partiels

**Conséquence de la violation :**

- Violation de l'interdiction INTERDIT-5
- Violation de l'invariant INV-API-4 (atomicité des opérations)
- Violation de la garantie G-PERSIST-2 (atomicité garantie)
- Compromission de la cohérence (des états incohérents peuvent être créés)

---

## 5. Anti-patterns classiques

### 5.1. Anti-pattern 1 : Contournement de CoreDataAPI

**Description :**

Tentative de créer un accès direct aux données ou une surface d'appel alternative pour "optimiser" ou "simplifier" l'accès aux données.

**Exemple conceptuel :**

Un développeur crée une fonction "readDirect()" qui permet de lire directement depuis la persistance sans passer par la CoreDataAPI, pensant "optimiser" les performances.

**Conséquence :**

- Violation de l'invariant INV-API-1 (unicité de la surface d'appel)
- Violation des règles UNIQ-1 à UNIQ-5
- Compromission de l'autorité exclusive de KindMother (INST-2)
- Compromission de la traçabilité complète (INST-7)
- Compromission de l'isolation systémique (INST-3)

**Correction :**

Toute opération DOIT passer par la CoreDataAPI. Aucun accès direct n'est autorisé. Si des optimisations sont nécessaires, elles DOIVENT être implémentées dans KindMother, pas en contournant la CoreDataAPI.

### 5.2. Anti-pattern 2 : Accès direct à la persistance

**Description :**

Tentative d'exposer un accès direct à la persistance (base de données, système de fichiers) pour permettre aux adaptateurs d'accéder directement aux données.

**Exemple conceptuel :**

Un développeur expose une connexion de base de données ou un système de fichiers directement aux adaptateurs, pensant "faciliter" l'accès aux données.

**Conséquence :**

- Violation de l'interdiction INTERDIT-2 (exposition des données directement)
- Violation de l'invariant INST-4 (persistance interne)
- Compromission de l'isolation systémique (INST-3)
- Compromission de l'autorité exclusive de KindMother (INST-2)
- Compromission de la protection contre corruption (INST-8)

**Correction :**

La persistance est interne à KindMother et n'est jamais exposée. Tous les accès DOIVENT passer par la CoreDataAPI. Aucun accès direct n'est autorisé.

### 5.3. Anti-pattern 3 : Logique métier dans les adaptateurs

**Description :**

Tentative de déplacer des validations métier dans les adaptateurs, pensant "simplifier" ou "décentraliser" la logique.

**Exemple conceptuel :**

Un développeur implémente des validations métier dans l'adaptateur et suppose que KindMother peut "faire confiance" à ces validations, évitant ainsi de les refaire.

**Conséquence :**

- Violation de l'invariant INST-6 (validation obligatoire)
- Violation de l'interdiction INTERDIT-8 (délégation de validation)
- Compromission de l'autorité exclusive de KindMother (INST-2)
- Compromission de la cohérence (les validations peuvent être contournées)

**Correction :**

Toutes les validations DOIVENT être effectuées par KindMother. Aucune validation ne peut être déléguée à un adaptateur. Les adaptateurs fournissent le contexte, KindMother valide.

### 5.4. Anti-pattern 4 : Confiance implicite

**Description :**

Tentative d'optimiser en accordant une confiance implicite aux adaptateurs certifiés KM-compliant, évitant ainsi les validations répétées.

**Exemple conceptuel :**

Un développeur met en cache les résultats de validation pour les adaptateurs certifiés KM-compliant, pensant "optimiser" les performances en évitant les validations répétées.

**Conséquence :**

- Violation du principe de zero-trust
- Compromission de la sécurité (les validations peuvent être contournées)
- Compromission de l'intégrité (des opérations non validées peuvent être exécutées)
- Violation de l'invariant INST-6 (validation obligatoire)

**Correction :**

Chaque appel DOIT être validé, même si l'adaptateur est certifié KM-compliant. Aucune confiance implicite n'est autorisée. Le principe de zero-trust s'applique à chaque appel.

---

## 6. Bonnes pratiques conceptuelles

### 6.1. Validation systématique à chaque boundary

**Pratique :**

Chaque Runtime Boundary DOIT être traversée et validée pour chaque opération CoreDataAPI, sans exception.

**Justification :**

- Respecte l'invariant INST-6 (validation obligatoire)
- Respecte le principe de zero-trust
- Garantit que toutes les préconditions sont vérifiées
- Préserve l'intégrité et la sécurité

**Implémentation conceptuelle :**

- Boundary d'appel : Vérifier que l'appel est légal et bien formé
- Boundary de contexte : Vérifier que le contexte est complet et valide
- Boundary d'instance : Vérifier que l'instance est valide et accessible
- Boundary de permissions : Vérifier que les permissions sont suffisantes
- Boundary de cohérence : Vérifier que la cohérence est préservée
- Boundary de contournement : Vérifier qu'aucun contournement n'est détecté
- Boundary de charge : Vérifier que la charge est acceptable

### 6.2. Refus explicite avec erreur actionnable

**Pratique :**

Tout rejet DOIT retourner une erreur explicite et actionnable qui permet à l'adaptateur de comprendre et corriger le problème.

**Justification :**

- Respecte la garantie G-API-2 (messages d'erreur explicites)
- Respecte l'invariant INV-API-7 (erreur explicite après rejet)
- Facilite le debugging et la correction
- Préserve la traçabilité

**Implémentation conceptuelle :**

- Erreur explicite : Indiquer clairement la raison du rejet
- Erreur actionnable : Fournir des informations permettant la correction
- Erreur traçable : Tracer l'erreur pour audit
- Pas de détails internes : Ne pas exposer de détails d'implémentation (interdiction I2)

### 6.3. Traçabilité complète de toutes les opérations

**Pratique :**

Toute opération DOIT être tracée avec son contexte complet, son résultat, et son moment d'exécution.

**Justification :**

- Respecte l'invariant INST-7 (traçabilité complète)
- Respecte la garantie G-API-8 (traçabilité complète)
- Permet l'audit et le debugging
- Préserve l'historique complet

**Implémentation conceptuelle :**

- Traçabilité systématique : Tracer chaque opération
- Contexte complet : Inclure le contexte complet dans la trace
- Résultat tracé : Tracer le résultat (succès ou échec)
- Traces immuables : Les traces sont immuables après création
- Traces accessibles : Les traces sont accessibles pour audit

### 6.4. Respect strict des invariants

**Pratique :**

Tous les invariants contractuels DOIVENT être vérifiés et préservés à chaque opération.

**Justification :**

- Les invariants sont des contraintes absolues
- La violation d'un invariant compromet l'intégrité
- Les invariants garantissent la cohérence du système

**Implémentation conceptuelle :**

- Vérification systématique : Vérifier chaque invariant avant et après chaque opération
- Préservation garantie : Garantir que les invariants sont préservés après exécution
- Rejet si violation : Rejeter toute opération qui violerait un invariant

### 6.5. Isolation préservée à tous les niveaux

**Pratique :**

L'isolation DOIT être préservée à tous les niveaux : entre instances, entre domaines, et entre opérations.

**Justification :**

- Respecte l'invariant INST-3 (isolation systémique)
- Respecte la garantie G-PERSIST-5 (isolation garantie)
- Préserve la sécurité et la cohérence

**Implémentation conceptuelle :**

- Isolation entre instances : Aucun accès direct croisé entre instances
- Isolation entre domaines : Aucun partage direct entre domaines d'autorité
- Isolation entre opérations : Les opérations sont isolées les unes des autres

---

## 7. Check-list mentale avant toute feature

Avant d'implémenter une nouvelle fonctionnalité, un développeur DOIT vérifier mentalement :

### 7.1. Vérification des invariants

- **L'invariant INST-X est-il préservé ?** : Vérifier que tous les invariants contractuels sont préservés par la nouvelle fonctionnalité.

- **Aucun invariant n'est-il violé ?** : S'assurer qu'aucun invariant n'est violé, même temporairement.

- **Les invariants sont-ils vérifiés ?** : S'assurer que les invariants sont vérifiés avant et après chaque opération.

### 7.2. Vérification des garanties

- **La garantie G-Y est-elle respectée ?** : Vérifier que toutes les garanties contractuelles sont respectées par la nouvelle fonctionnalité.

- **Aucune garantie n'est-elle compromise ?** : S'assurer qu'aucune garantie n'est compromise, même pour des raisons d'optimisation.

- **Les garanties sont-elles vérifiables ?** : S'assurer que les garanties peuvent être vérifiées et validées.

### 7.3. Vérification des interdictions

- **L'interdiction I-Z est-elle respectée ?** : Vérifier que toutes les interdictions contractuelles sont respectées.

- **Aucune interdiction n'est-elle violée ?** : S'assurer qu'aucune interdiction n'est violée, même indirectement.

- **Les interdictions sont-elles appliquées ?** : S'assurer que les interdictions sont appliquées systématiquement.

### 7.4. Vérification de la CoreDataAPI

- **La CoreDataAPI est-elle le seul point d'entrée ?** : Vérifier que la nouvelle fonctionnalité n'introduit pas de surface d'appel alternative.

- **Toutes les opérations passent-elles par la CoreDataAPI ?** : S'assurer que toutes les opérations passent par la CoreDataAPI.

- **Aucun contournement n'est-il possible ?** : S'assurer qu'aucun mécanisme ne permet de contourner la CoreDataAPI.

### 7.5. Vérification du zero-trust

- **Le zero-trust est-il appliqué ?** : Vérifier que le principe de zero-trust est appliqué à chaque appel.

- **Aucune confiance implicite n'est-elle accordée ?** : S'assurer qu'aucune confiance implicite n'est accordée, même aux adaptateurs conformes.

- **Toutes les validations sont-elles effectuées ?** : S'assurer que toutes les validations sont effectuées à chaque appel.

---

## 8. Conclusion

Ce document fournit des lignes directrices pour implémenter KindMother de manière conforme aux contrats FONDATION.

**Points clés :**
- Les principes généraux DOIVENT être respectés absolument
- Les contrats DOIVENT être traduits en logique sans interprétation abusive
- Les anti-patterns DOIVENT être évités
- Les bonnes pratiques conceptuelles DOIVENT être suivies
- La check-list mentale DOIT être utilisée avant toute feature

**Nature informative :**
Ce document est purement informatif et ne crée aucune nouvelle obligation contractuelle. Il sert uniquement à guider la compréhension et l'application des contrats FONDATION.

**Rappel :** Les contrats FONDATION priment toujours sur ce guide. En cas de doute, se référer aux contrats FONDATION.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** POST-FONDATION — Informatif, non normatif, non contractuel  
**Référence :** Miyukini Core System v2.4, KindMother Documentation, Tous les contrats FONDATION  
**Type :** Guide d'implémentation informatif

---

## 9. Mini log — erreurs / warnings / arbitrages rencontrés

### Arbitrage A1 : Niveau de détail technique

**Arbitrage rencontré :** Quel niveau de détail technique inclure dans ce guide ? Le document doit rester conceptuel et ne pas prescrire de technologies.

**Décision prise :** Le document reste purement conceptuel. Aucun détail technique (langages, structures de données, algorithmes) n'est inclus. Seuls les concepts et principes sont décrits.

**Justification :** Ce document est informatif et non normatif. Il guide la compréhension des contrats, pas l'implémentation technique. Les détails techniques sont des choix d'implémentation.

**Documentation :** Toutes les sections restent conceptuelles, sans détails techniques.

### Arbitrage A2 : Exemples conceptuels vs exemples techniques

**Arbitrage rencontré :** Comment illustrer les anti-patterns sans donner d'exemples techniques qui pourraient être interprétés comme des prescriptions ?

**Décision prise :** Les exemples sont purement conceptuels et narratifs. Ils décrivent des situations conceptuelles sans détails techniques.

**Justification :** Les exemples conceptuels illustrent les principes sans prescrire de solutions techniques. Ils aident à comprendre sans imposer d'implémentation.

**Documentation :** Section 5 (Anti-patterns) avec exemples conceptuels uniquement.

### Arbitrage A3 : Balance entre guidance et liberté

**Arbitrage rencontré :** Comment fournir des lignes directrices utiles sans restreindre la liberté d'implémentation ?

**Décision prise :** Le document se concentre sur les principes et les contraintes contractuelles, pas sur les solutions techniques. Il guide ce qui DOIT être fait (contraintes) sans prescrire comment le faire (solutions).

**Justification :** Cette approche respecte la nature informative du document tout en fournissant une guidance utile. Les développeurs ont la liberté de choisir les solutions techniques tant qu'ils respectent les contraintes contractuelles.

**Documentation :** Toutes les sections se concentrent sur les "quoi" (contraintes) plutôt que sur les "comment" (solutions).

### Arbitrage A4 : Références aux contrats

**Arbitrage rencontré :** Comment référencer les contrats FONDATION sans créer de dépendances trop strictes qui pourraient devenir obsolètes ?

**Décision prise :** Les références aux contrats utilisent des identifiants stables (INST-*, G-*, INTERDIT-*, etc.) qui sont définis dans les contrats FONDATION. Ces identifiants sont stables et ne changent pas.

**Justification :** Les identifiants contractuels sont stables et font partie de la structure contractuelle. Les références à ces identifiants restent valides même si les contrats évoluent.

**Documentation :** Toutes les références utilisent les identifiants contractuels stables.

### Arbitrage A5 : Check-list vs prescription

**Arbitrage rencontré :** La check-list mentale est-elle trop prescriptive ou suffisamment guidante ?

**Décision prise :** La check-list est organisée autour des catégories contractuelles (invariants, garanties, interdictions) plutôt que des étapes techniques. Elle guide la vérification conceptuelle sans prescrire de processus technique.

**Justification :** Cette approche guide la pensée conceptuelle sans imposer de processus technique. Elle aide les développeurs à vérifier la conformité contractuelle sans restreindre leur liberté d'implémentation.

**Documentation :** Section 7 (Check-list mentale) organisée par catégories contractuelles.

---

*Aucune autre erreur, warning, ou arbitrage rencontré lors de la rédaction de ce document.*
