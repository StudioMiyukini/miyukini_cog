# StrongFather — Violations & Anti-Patterns

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Violations & Anti-Patterns** : un contrat normatif, non négociable, et de statut FONDATION qui établit le catalogue des violations contractuelles et des anti-patterns à éviter lors de l'implémentation ou de l'utilisation de StrongFather dans le système Miyukini Core System v2.4.

Ce contrat précise ce qui constitue une violation, les catégories de violations, les anti-patterns identifiés, et les conséquences associées.

### Portée

Ce contrat s'applique à **toutes les implémentations et utilisations de StrongFather** et définit de manière absolue :
- la définition formelle d'une violation,
- les catégories de violations,
- le catalogue des violations explicites,
- les anti-patterns à éviter,
- les conséquences des violations.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat **référence et consolide** les violations définies dans :
- **StrongFather — Documentation Fondatrice**
- **StrongFather — Core Decision Contract**
- **StrongFather — Intent Model Contract**
- **StrongFather — Policy Engine Contract**
- **StrongFather — Execution Prohibition Contract**
- **StrongFather — Boundary & Isolation Contract**
- **StrongFather — Audit & Trace Contract**
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Violations des lois d'autonomie système

Ce contrat est le **catalogue de référence** pour toutes les violations StrongFather.

---

## 2. Définition d'une violation

### 2.1. Nature d'une violation

Une **violation** est un non-respect d'une règle, d'un invariant, ou d'une garantie définie dans les contrats StrongFather.

**Caractéristiques d'une violation :**

- **Contractuelle** : Une violation concerne toujours un contrat spécifique
- **Identifiable** : Une violation peut être identifiée et référencée
- **Conséquentielle** : Une violation a des conséquences définies
- **Non-tolérable** : Une violation ne peut pas être ignorée ou tolérée

### 2.2. Gravité des violations

Les violations sont classées selon leur gravité :

**CRITIQUE :**

Violation d'un invariant fondamental ou d'une interdiction absolue. La violation compromet l'intégrité de StrongFather.

**MAJEURE :**

Violation d'une règle importante qui affecte le comportement de StrongFather mais ne compromet pas ses propriétés fondamentales.

**MINEURE :**

Violation d'une règle secondaire qui n'affecte pas le comportement principal de StrongFather.

---

## 3. Catégories de violations

### 3.1. Violations d'exécution

**Catégorie :** CRITIQUE

**Source :** Execution Prohibition Contract

**Violations :**

**VIOL-EXEC-1 : Exécution d'action**

StrongFather exécute une action (création, modification, suppression).

*Invariant violé : INV-EXEC-1*

**VIOL-EXEC-2 : Modification d'état**

StrongFather modifie un état du système.

*Invariant violé : INV-EXEC-2*

**VIOL-EXEC-3 : Persistance opérationnelle**

StrongFather persiste des données opérationnelles.

*Invariant violé : INV-EXEC-3*

**VIOL-EXEC-4 : Communication externe**

StrongFather initie une communication externe.

*Invariant violé : INV-EXEC-4*

### 3.2. Violations de frontière

**Catégorie :** CRITIQUE

**Source :** Boundary & Isolation Contract

**Violations :**

**VIOL-BOUND-1 : Appel à KindMother**

StrongFather appelle KindMother directement ou indirectement.

*Interdiction violée : INTERD-KM-1*

**VIOL-BOUND-2 : Appel à un module SPM**

StrongFather appelle un module SPM directement.

*Interdiction violée : INTERD-SPM-1*

**VIOL-BOUND-3 : Appel réseau**

StrongFather effectue un appel réseau externe.

*Interdiction violée : INTERD-EXT-1*

**VIOL-BOUND-4 : Communication directe produit**

Un produit communique directement avec StrongFather sans passer par un adaptateur.

*Interdiction violée : INTERD-PROD-1*

### 3.3. Violations de décision

**Catégorie :** MAJEURE

**Source :** Core Decision Contract

**Violations :**

**VIOL-DEC-1 : Décision sans justification**

Une décision est produite sans justification explicite.

*Garantie violée : G-JUST-1*

**VIOL-DEC-2 : Décision ambiguë**

Une décision produite est ambiguë (ni acceptée, ni refusée, ni ambiguë, ni différée clairement).

*Invariant violé : INV-SF-6*

**VIOL-DEC-3 : Décisions multiples**

Plusieurs décisions sont produites pour une même intention.

*Invariant violé : INV-CYCLE-2*

**VIOL-DEC-4 : Décision avec commande d'exécution**

Une décision contient une commande d'exécution.

*Champ interdit : Core Decision Contract section 5.3*

### 3.4. Violations d'intention

**Catégorie :** MAJEURE

**Source :** Intent Model Contract

**Violations :**

**VIOL-INT-1 : Intention sans identifiant**

Une intention est évaluée sans identifiant unique.

*Invariant violé : INV-INT-1*

**VIOL-INT-2 : Intention exécutée**

Une intention est exécutée par StrongFather.

*Invariant violé : INV-INT-4*

**VIOL-INT-3 : Intention modifiée post-soumission**

Une intention est modifiée après sa soumission.

*Règle violée : R-SOUM-3*

### 3.5. Violations de politique

**Catégorie :** MAJEURE

**Source :** Policy Engine Contract

**Violations :**

**VIOL-POL-1 : Politique implicite**

Une politique implicite est appliquée.

*Invariant violé : INV-POL-1*

**VIOL-POL-2 : Politique modifiée pendant évaluation**

Une politique est modifiée pendant l'évaluation d'une intention.

*Invariant violé : INV-POL-2*

**VIOL-POL-3 : Non-déterminisme**

Une même évaluation produit des résultats différents.

*Invariant violé : INV-POL-6*

### 3.6. Violations de traçabilité

**Catégorie :** MINEURE à MAJEURE

**Source :** Audit & Trace Contract

**Violations :**

**VIOL-TRACE-1 : Évaluation sans trace**

Une évaluation ne produit pas de trace.

*Invariant violé : INV-TRACE-1*

**VIOL-TRACE-2 : Trace modifiée**

Une trace est modifiée après production.

*Invariant violé : INV-TRACE-4*

**VIOL-TRACE-3 : Trace incomplète**

Une trace ne contient pas tous les éléments obligatoires.

*Invariant violé : INV-TRACE-5*

---

## 4. Anti-patterns

### 4.1. Anti-pattern : StrongFather comme orchestrateur

**Description :**

Utiliser StrongFather pour orchestrer des actions, des workflows, ou des processus au lieu de simplement évaluer des intentions.

**Pourquoi c'est un anti-pattern :**

StrongFather est un moteur de décision, pas un orchestrateur. L'orchestration implique l'exécution et le contrôle de flux, ce qui viole l'interdiction d'exécution.

**Symptômes :**

- StrongFather déclenche des actions suite à des décisions
- StrongFather maintient un état de workflow
- StrongFather attend des événements pour progresser

**Solution :**

L'orchestration doit être effectuée par les adaptateurs produits, pas par StrongFather.

### 4.2. Anti-pattern : StrongFather comme cache

**Description :**

Utiliser StrongFather pour stocker des données ou des résultats pour accès ultérieur.

**Pourquoi c'est un anti-pattern :**

StrongFather ne persiste pas de données opérationnelles. Utiliser StrongFather comme cache viole l'interdiction de persistance.

**Symptômes :**

- StrongFather mémorise des décisions pour réutilisation
- StrongFather maintient un état entre évaluations
- StrongFather optimise via la mise en cache de résultats

**Solution :**

Le cache doit être géré par les composants appelants, pas par StrongFather.

### 4.3. Anti-pattern : Contournement par adaptateur

**Description :**

Utiliser un adaptateur pour contourner les règles de StrongFather en effectuant des actions interdites au nom de StrongFather.

**Pourquoi c'est un anti-pattern :**

Le contournement via adaptateur viole l'esprit des contrats et peut introduire des incohérences systémiques.

**Symptômes :**

- L'adaptateur exécute des actions "pour" StrongFather
- L'adaptateur communique avec KindMother "au nom de" StrongFather
- L'adaptateur modifie des résultats de StrongFather avant de les utiliser

**Solution :**

Les adaptateurs doivent respecter les frontières de StrongFather et ne jamais agir en son nom.

### 4.4. Anti-pattern : Politiques techniques

**Description :**

Définir des politiques qui portent sur des aspects techniques (schémas, formats, protocoles) au lieu d'aspects stratégiques et politiques.

**Pourquoi c'est un anti-pattern :**

StrongFather évalue des intentions selon des politiques stratégiques, pas selon des règles techniques. Les validations techniques sont hors-scope.

**Symptômes :**

- Politiques qui vérifient des formats de données
- Politiques qui valident des schémas
- Politiques qui contrôlent des protocoles

**Solution :**

La validation technique doit être effectuée par les composants appropriés (adaptateurs, modules SPM).

### 4.5. Anti-pattern : Logique métier dans les politiques

**Description :**

Inclure de la logique métier spécifique à un domaine dans les politiques de StrongFather.

**Pourquoi c'est un anti-pattern :**

StrongFather applique des politiques générales, pas des règles métier spécifiques. L'inclusion de logique métier crée un couplage inapproprié.

**Symptômes :**

- Politiques qui contiennent des calculs métier
- Politiques qui référencent des concepts spécifiques à un domaine
- Politiques qui changent selon le produit

**Solution :**

La logique métier doit rester dans les produits. StrongFather applique uniquement des politiques générales.

### 4.6. Anti-pattern : Dépendance temporelle technique

**Description :**

Faire dépendre les décisions de StrongFather du temps technique (horodatages, timestamps, délais).

**Pourquoi c'est un anti-pattern :**

StrongFather ne possède pas de logique temporelle technique. Les décisions ne doivent pas dépendre du temps technique.

**Symptômes :**

- Décisions qui changent selon l'heure
- Politiques basées sur des timestamps
- Évaluations qui attendent des délais

**Solution :**

Le temps conceptuel (période, cycle, saison) peut être utilisé via le contexte, mais pas le temps technique.

### 4.7. Anti-pattern : StrongFather comme point d'entrée unique

**Description :**

Faire de StrongFather le point d'entrée unique de toutes les opérations du système, même celles qui ne nécessitent pas d'évaluation.

**Pourquoi c'est un anti-pattern :**

StrongFather est un moteur de décision, pas une gateway. Toutes les opérations ne nécessitent pas une évaluation de politiques.

**Symptômes :**

- Toutes les requêtes passent par StrongFather
- StrongFather est appelé pour des opérations triviales
- StrongFather devient un goulot d'étranglement

**Solution :**

StrongFather doit être utilisé uniquement pour les intentions nécessitant une évaluation de politiques.

---

## 5. Conséquences des violations

### 5.1. Violations critiques

**Conséquences :**

1. **Non-conformité immédiate** : L'implémentation est considérée non conforme
2. **Arrêt requis** : L'évaluation en cours doit être arrêtée
3. **Audit obligatoire** : Un audit doit être effectué
4. **Correction impérative** : La correction est obligatoire avant toute utilisation

### 5.2. Violations majeures

**Conséquences :**

1. **Warning de non-conformité** : L'implémentation est signalée comme non conforme
2. **Décision invalide** : La décision associée est invalide
3. **Correction requise** : La correction doit être planifiée

### 5.3. Violations mineures

**Conséquences :**

1. **Signalement** : La violation est signalée
2. **Correction recommandée** : La correction est recommandée
3. **Traçabilité** : La violation est tracée pour suivi

---

## 6. Règles de fermeture du contrat

### 6.1. Contrat fermé

Ce contrat est **fermé**. Seules les violations et les anti-patterns explicitement définis sont reconnus.

### 6.2. Catalogue de référence

Ce contrat est le **catalogue de référence** pour toutes les violations StrongFather. Toute nouvelle violation doit être ajoutée à ce catalogue.

---

## 7. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable le catalogue des violations et anti-patterns de StrongFather.

Il garantit que :
- les violations sont exhaustivement cataloguées,
- les anti-patterns sont identifiés et documentés,
- les conséquences sont explicites,
- le contrat est fermé et constitue la référence unique.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 8. Validation conceptuelle

### 8.1. Vérification de complétude

Ce document catalogue les violations de :
- ✅ Execution Prohibition Contract : VIOL-EXEC-*
- ✅ Boundary & Isolation Contract : VIOL-BOUND-*
- ✅ Core Decision Contract : VIOL-DEC-*
- ✅ Intent Model Contract : VIOL-INT-*
- ✅ Policy Engine Contract : VIOL-POL-*
- ✅ Audit & Trace Contract : VIOL-TRACE-*

### 8.2. Vérification de cohérence

- ✅ Toutes les violations référencent un contrat source
- ✅ Toutes les violations référencent un invariant ou une règle
- ✅ Les gravités sont cohérentes avec l'importance des règles

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Catalogue des violations et anti-patterns non négociable

---

## 9. Mini log de génération

### Décision éditoriale E1 : Consolidation des violations

**Décision prise :** Consolidation de toutes les violations dispersées dans les contrats en un catalogue unique.

**Application :** Chaque violation référence son contrat et invariant source.

### Décision éditoriale E2 : Anti-patterns

**Décision prise :** Inclusion d'anti-patterns avec description, symptômes et solutions.

**Application :** 7 anti-patterns identifiés et documentés.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Toutes les violations des contrats sont incluses
- ✅ Les références aux invariants sont correctes
- ✅ Les gravités sont cohérentes

**Conclusion :** Catalogue complet et cohérent.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
