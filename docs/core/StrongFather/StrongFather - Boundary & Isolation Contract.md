# StrongFather — Boundary & Isolation Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Boundary & Isolation Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les frontières de StrongFather au sein de l'écosystème Miyukini, les règles d'isolation entre StrongFather et les autres composants, et les interdictions de communication directe dans le système Miyukini Core System v2.4.

Ce contrat précise ce que StrongFather peut et ne peut pas connaître, avec quels composants il peut et ne peut pas interagir, et comment l'isolation est maintenue.

### Portée

Ce contrat s'applique à **toutes les interactions de StrongFather** et définit de manière absolue :
- les frontières conceptuelles de StrongFather,
- les composants avec lesquels StrongFather peut interagir,
- les composants avec lesquels StrongFather ne peut jamais interagir,
- les règles d'isolation,
- les invariants de frontière.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **StrongFather — Documentation Fondatrice** : Positionnement architectural de StrongFather
- **StrongFather — Execution Prohibition Contract** : Interdictions de communication externe
- **KindMother — Documentation Fondatrice** : Complémentarité et indépendance

Il n'introduit aucune contradiction, et constitue la définition formelle des frontières et de l'isolation de StrongFather.

---

## 2. Frontières de StrongFather

### 2.1. Définition des frontières

Les **frontières** de StrongFather définissent ce qui est à l'intérieur de StrongFather (sous son autorité) et ce qui est à l'extérieur (hors de son autorité).

**À l'intérieur de StrongFather :**

- Évaluation des intentions
- Application des politiques
- Production de décisions
- Calcul de priorités
- Détection d'ambiguïtés
- Traçabilité des évaluations

**À l'extérieur de StrongFather :**

- Exécution des actions
- Persistance des données
- Communication externe
- Logique temporelle technique
- Validation technique des données
- Règles métier spécifiques

### 2.2. Caractère absolu des frontières

Les frontières de StrongFather sont **absolues et non négociables** :

- **FRONT-1** : Aucune opération à l'extérieur ne peut être effectuée par StrongFather
- **FRONT-2** : Aucun composant extérieur ne peut accéder directement à l'intérieur de StrongFather
- **FRONT-3** : Les frontières ne peuvent pas être temporairement suspendues ou contournées

---

## 3. Relations autorisées

### 3.1. Adaptateurs produits

**Type de relation :** COMMUNICATION AUTORISÉE

**Nature de la relation :**

Les adaptateurs produits sont les **seuls composants autorisés** à soumettre des intentions à StrongFather et à recevoir des décisions.

**Interactions autorisées :**

1. Soumettre une intention à StrongFather pour évaluation
2. Recevoir une décision de StrongFather
3. Fournir le contexte nécessaire à l'évaluation
4. Recevoir les métadonnées de traçabilité

**Règles :**

- **R-ADAPT-1** : Seuls les adaptateurs produits peuvent communiquer avec StrongFather
- **R-ADAPT-2** : La communication est unidirectionnelle : intention → décision
- **R-ADAPT-3** : Les adaptateurs sont responsables de l'exécution suite aux décisions

### 3.2. Source de politiques

**Type de relation :** LECTURE AUTORISÉE

**Nature de la relation :**

StrongFather reçoit ses politiques d'une source de politiques configurée. Cette source est en lecture seule pour StrongFather.

**Interactions autorisées :**

1. Charger les politiques depuis la source
2. Actualiser les politiques (rechargement)
3. Lire les métadonnées des politiques

**Règles :**

- **R-SRC-1** : StrongFather ne peut que lire les politiques, jamais les modifier
- **R-SRC-2** : La source de politiques est configurée, pas découverte
- **R-SRC-3** : Les politiques sont chargées de manière explicite

---

## 4. Relations interdites

### 4.1. KindMother

**Type de relation :** INTERDICTION ABSOLUE

**Justification :**

StrongFather et KindMother sont complémentaires mais indépendants. StrongFather décide, KindMother persiste. Aucune communication directe n'est autorisée.

**Interdictions :**

- **INTERD-KM-1** : StrongFather ne peut jamais appeler KindMother
- **INTERD-KM-2** : StrongFather ne peut jamais lire des données gérées par KindMother
- **INTERD-KM-3** : StrongFather ne peut jamais demander à KindMother de persister
- **INTERD-KM-4** : StrongFather ne connaît pas l'existence de KindMother

**Conséquence :**

Toute tentative de communication avec KindMother est une violation de ce contrat.

### 4.2. Kernel

**Type de relation :** INTERDICTION ABSOLUE (pour l'exécution) avec SOUS-CONTRAT DE TRAÇABILITÉ

**Justification :**

Le kernel fournit des capacités techniques (Id, Clock, Logger) qui sont hors du périmètre de StrongFather pour l'exécution. Cependant, la traçabilité étant une responsabilité interne de StrongFather, un accès limité et encadré est autorisé sous forme de sous-contrat.

**Interdictions absolues :**

- **INTERD-KERN-1** : StrongFather ne peut jamais utiliser le kernel pour exécuter des actions
- **INTERD-KERN-2** : StrongFather ne peut jamais utiliser Clock pour de la logique temporelle technique (décisions, priorités, ordonnancement)
- **INTERD-KERN-3** : StrongFather ne peut jamais dépendre du kernel pour ses décisions
- **INTERD-KERN-4** : StrongFather ne peut jamais utiliser Clock pour influencer le résultat d'une évaluation

---

#### 4.2.1. SOUS-CONTRAT : Kernel Trace Access Contract (embedded)

**Statut :** Sous-contrat intégré, même niveau de rigueur que le contrat parent

**Objet :** Définir les seuls accès autorisés au kernel pour la traçabilité passive

##### Appels kernel explicitement autorisés

**KERN-AUTH-1 : Id pour identification de trace**

StrongFather PEUT utiliser `Id` pour générer des identifiants uniques destinés exclusivement aux traces (identifiant de trace, corrélation).

*Conditions :*
- Utilisation uniquement pour la traçabilité
- Pas d'influence sur le résultat de l'évaluation
- L'identifiant généré n'est pas utilisé dans la logique décisionnelle

**KERN-AUTH-2 : Logger pour enregistrement de trace**

StrongFather PEUT utiliser `Logger` pour enregistrer les traces d'évaluation définies dans le Audit & Trace Contract.

*Conditions :*
- Utilisation uniquement pour l'enregistrement passif
- Pas d'influence sur le résultat de l'évaluation
- Échec du Logger = la décision continue (voir règle ci-dessous)

**KERN-AUTH-3 : Clock pour horodatage de trace uniquement**

StrongFather PEUT utiliser `Clock` **exclusivement** pour horodater les traces produites.

*Conditions strictes :*
- Utilisation **uniquement** pour horodater une trace après production de décision
- **JAMAIS** pour influencer une évaluation
- **JAMAIS** pour la logique temporelle décisionnelle
- **JAMAIS** pour l'ordonnancement ou la planification
- L'horodatage est une métadonnée de trace, pas une donnée décisionnelle

##### Appels kernel explicitement interdits

**KERN-INTERD-1 : Clock pour logique décisionnelle**

StrongFather NE PEUT JAMAIS utiliser `Clock` pour :
- Déterminer si une intention est valide selon l'heure
- Calculer des priorités basées sur le temps
- Ordonnancer des évaluations
- Planifier des réévaluations
- Toute logique temporelle technique

**KERN-INTERD-2 : Tout autre appel kernel**

Tout appel au kernel non listé dans les autorisations (KERN-AUTH-*) est **interdit**.

##### Règle de résilience de la traçabilité

**R-TRACE-FAIL-1 : Échec de trace = Décision continue**

Si un appel au kernel pour la traçabilité échoue (Logger indisponible, Id non générable, Clock inaccessible), StrongFather DOIT :
1. Continuer l'évaluation normalement
2. Produire la décision sans interruption
3. Marquer la trace comme "dégradée" ou l'omettre
4. Ne jamais bloquer ou modifier la décision à cause d'un échec de traçabilité

**Justification :** La traçabilité est une fonction passive d'observation. Son échec ne doit jamais affecter la fonction principale de StrongFather (évaluation et décision).

##### Invariant de traçabilité kernel

**INV-TRACE-KERNEL : Utilisation kernel strictement passive**

Le kernel n'est utilisé que pour Id et Logger (identification et enregistrement de traces), et Clock uniquement pour l'horodatage passif des traces. Aucun appel kernel n'influence jamais le résultat d'une évaluation ou d'une décision.

*Cet invariant est référencé dans le document Invariants & Guarantees.*

---

**Fin du sous-contrat Kernel Trace Access**

---

### 4.3. Modules SPM CMS

**Type de relation :** INTERDICTION ABSOLUE

**Justification :**

Les modules SPM CMS exposent des traits fonctionnels. StrongFather n'interagit pas avec eux directement.

**Interdictions :**

- **INTERD-SPM-1** : StrongFather ne peut jamais appeler un module SPM
- **INTERD-SPM-2** : StrongFather ne connaît pas les traits des modules SPM
- **INTERD-SPM-3** : StrongFather ne peut jamais dépendre d'un module SPM

**Règle fondamentale :**

Toute interaction avec les modules SPM passe par les adaptateurs produits, jamais par StrongFather.

### 4.4. Systèmes externes

**Type de relation :** INTERDICTION ABSOLUE

**Justification :**

StrongFather est isolé de tout système externe pour garantir la pureté fonctionnelle et l'absence d'effet de bord. Cette isolation respecte **LOI-1** (aucune dépendance externe critique à l'exécution) définie dans [Miyukini Framework - Lois Autonomie Systeme](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : StrongFather fonctionne sans aucun appel externe obligatoire.

**Interdictions :**

- **INTERD-EXT-1** : StrongFather ne peut jamais effectuer d'appels réseau
- **INTERD-EXT-2** : StrongFather ne peut jamais accéder à des bases de données
- **INTERD-EXT-3** : StrongFather ne peut jamais accéder à des systèmes de fichiers
- **INTERD-EXT-4** : StrongFather ne peut jamais envoyer de notifications

### 4.5. Produits

**Type de relation :** INTERDICTION DE COMMUNICATION DIRECTE

**Justification :**

Les produits interagissent avec StrongFather uniquement via leurs adaptateurs, jamais directement.

**Interdictions :**

- **INTERD-PROD-1** : Les produits ne peuvent pas communiquer directement avec StrongFather
- **INTERD-PROD-2** : StrongFather ne connaît pas les produits directement

**Règle :**

Toute communication produit ↔ StrongFather passe par un adaptateur produit.

---

## 5. Règles d'isolation

### 5.1. Isolation fonctionnelle

**R-ISOL-1 : Pureté fonctionnelle**

StrongFather est fonctionnellement pur. Aucune entrée externe non explicite n'influence l'évaluation.

**R-ISOL-2 : Entrées explicites**

Toutes les entrées de StrongFather (intentions, contexte, politiques) sont explicites et déclarées.

**R-ISOL-3 : Sorties explicites**

Toutes les sorties de StrongFather (décisions) sont explicites et déclarées.

### 5.2. Isolation des données

**R-ISOL-4 : Pas d'accès aux données persistées**

StrongFather n'accède jamais aux données persistées dans le système.

**R-ISOL-5 : Pas de mémoire persistante**

StrongFather ne maintient pas de mémoire persistante entre les évaluations.

**R-ISOL-6 : Contexte fourni**

Le contexte nécessaire à l'évaluation est toujours fourni par l'appelant, jamais recherché par StrongFather.

### 5.3. Isolation temporelle

**R-ISOL-7 : Pas de dépendance temporelle technique**

StrongFather ne dépend jamais du temps technique pour ses évaluations.

**R-ISOL-8 : Pas d'ordonnancement**

StrongFather n'ordonnance jamais ses évaluations selon le temps.

**R-ISOL-9 : Pas de planification**

StrongFather ne planifie jamais d'évaluations futures.

---

## 6. Invariants de frontière

### 6.1. Invariants de relation

**INV-BOUND-1 : Adaptateurs uniquement**

Seuls les adaptateurs produits peuvent communiquer avec StrongFather.

**INV-BOUND-2 : Indépendance KindMother**

StrongFather et KindMother sont totalement indépendants. Aucune communication directe n'existe.

**INV-BOUND-3 : Indépendance modules SPM**

StrongFather et les modules SPM sont totalement indépendants. Aucune communication directe n'existe.

### 6.2. Invariants d'isolation

**INV-BOUND-4 : Isolation totale**

StrongFather est totalement isolé de tout système externe. Cette isolation garantit la conformité à **LOI-1** (aucune dépendance externe critique) : StrongFather peut démarrer, décider, fonctionner, et être audité sans aucun appel externe obligatoire.

**INV-BOUND-5 : Pureté préservée**

L'isolation garantit la pureté fonctionnelle de StrongFather.

**INV-BOUND-6 : Frontières immuables**

Les frontières de StrongFather sont immuables et ne peuvent pas être modifiées à l'exécution.

---

## 7. Flux de communication

### 7.1. Flux entrant (vers StrongFather)

```
┌─────────────────────────────────────────────┐
│              ADAPTATEUR PRODUIT              │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  Intention + Contexte               │   │
│  └──────────────┬──────────────────────┘   │
│                 │                           │
│                 ▼                           │
│  ┌─────────────────────────────────────┐   │
│  │         STRONGFATHER                 │   │
│  │      (Surface d'évaluation)         │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

**Éléments du flux entrant :**

1. Intention (structure définie par Intent Model Contract)
2. Contexte d'appel (appelant, origine, instance)
3. Données de l'intention
4. Métadonnées optionnelles

### 7.2. Flux sortant (depuis StrongFather)

```
┌─────────────────────────────────────────────┐
│              STRONGFATHER                    │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │           Décision                   │   │
│  └──────────────┬──────────────────────┘   │
│                 │                           │
│                 ▼                           │
│  ┌─────────────────────────────────────┐   │
│  │       ADAPTATEUR PRODUIT             │   │
│  │  (Responsable de l'exécution)        │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

**Éléments du flux sortant :**

1. Décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
2. Politiques appliquées
3. Justification
4. Métadonnées de traçabilité

### 7.3. Flux interdit

```
┌─────────────────────────────────────────────┐
│              STRONGFATHER                    │
│                                             │
│          ╳ ──────────────────▶ KINDMOTHER   │
│          ╳ ──────────────────▶ KERNEL       │
│          ╳ ──────────────────▶ SPM MODULES  │
│          ╳ ──────────────────▶ EXTERNE      │
│                                             │
└─────────────────────────────────────────────┘
```

**Aucun flux direct** entre StrongFather et ces composants.

---

## 8. Règles de fermeture du contrat

### 8.1. Contrat fermé

Ce contrat est **fermé**. Seules les relations autorisées, les interdictions, et les règles explicitement définies dans ce contrat sont valides.

### 8.2. Interdiction d'extension

Aucune nouvelle relation ne peut être établie sans modification explicite de ce contrat.

### 8.3. Interdiction de contournement

Aucun mécanisme de contournement des frontières n'est autorisé.

---

## 9. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les frontières et l'isolation de StrongFather.

Il garantit que :
- les frontières sont explicitement définies,
- les relations autorisées sont limitées aux adaptateurs produits,
- les relations interdites sont absolues,
- l'isolation est complète et non contournable,
- les invariants de frontière sont respectés.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 10. Validation conceptuelle

### 10.1. Cas conformes

Les cas suivants sont **conformes** à ce contrat :

1. **Adaptateur soumet intention** : Un adaptateur produit soumet une intention à StrongFather et reçoit une décision.

2. **Chargement de politiques** : StrongFather charge des politiques depuis une source configurée en lecture seule.

### 10.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Appel à KindMother** : StrongFather appelle KindMother pour persister une décision. Viole INTERD-KM-1.

2. **Communication directe produit** : Un produit communique directement avec StrongFather sans passer par un adaptateur. Viole INTERD-PROD-1.

3. **Appel réseau** : StrongFather effectue un appel réseau externe. Viole INTERD-EXT-1.

4. **Accès module SPM** : StrongFather appelle un trait de module SPM. Viole INTERD-SPM-1.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Contrat de frontières et isolation non négociable (DOCUMENT MAÎTRE pour les frontières)

---

## 11. Mini log de génération

### Warning W1 : Kernel et traçabilité

**Warning rencontré :** Le kernel (Id, Logger) pourrait être utilisé pour la traçabilité. Est-ce une violation ?

**Décision prise :** Exception limitée : le kernel peut être utilisé uniquement pour la traçabilité (Id, Logger). Cette utilisation ne constitue pas une violation car elle ne relève pas de l'exécution.

**Correction effectuée :** Section 4.2 précise l'exception limitée pour la traçabilité.

### Warning W2 : Source de politiques

**Warning rencontré :** D'où viennent les politiques de StrongFather ?

**Décision prise :** Définition d'une "source de politiques" comme relation autorisée en lecture seule.

**Correction effectuée :** Section 3.2 définit la relation avec la source de politiques.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice : Confirmée (section 9 architecture)
- ✅ Cohérence avec Execution Prohibition Contract : Confirmée (interdictions de communication)
- ✅ Indépendance KindMother : Confirmée (INTERD-KM-*)
- ✅ Indépendance modules SPM : Confirmée (INTERD-SPM-*)

**Conclusion :** Aucune contradiction détectée.

---

### Modification v1.1 : Kernel Trace Access Contract (embedded)

**Date :** 2026-01-25

**Origine :** Audit global StrongFather — Problème C.2 (Exception du Kernel insuffisamment encadrée)

**Modification apportée :**

Remplacement de "l'exception limitée" par un sous-contrat formel **Kernel Trace Access Contract** intégré dans la section 4.2.

**Contenu ajouté :**
- Liste exhaustive des appels kernel autorisés (KERN-AUTH-1, KERN-AUTH-2, KERN-AUTH-3)
- Interdiction explicite de Clock hors trace passive (KERN-INTERD-1)
- Règle de résilience : si trace échoue → décision continue (R-TRACE-FAIL-1)
- Invariant INV-TRACE-KERNEL défini et référencé

**Objectif :** Neutraliser le problème C.2 et le risque D.3 identifiés dans l'audit.

**Cohérence vérifiée :**
- ✅ Compatible avec Audit & Trace Contract (traçabilité passive)
- ✅ Compatible avec Execution Prohibition Contract (pas d'exécution)
- ✅ Invariant INV-TRACE-KERNEL prêt pour consolidation dans Invariants & Guarantees

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée.*
