# Master Butler — Violations & Anti-Patterns

## 1. Introduction

### Objet du contrat

Ce document définit le **Master Butler — Violations & Anti-Patterns** : un contrat normatif, non négociable, et de statut FONDATION qui établit le catalogue des violations contractuelles et des anti-patterns à éviter lors de l'implémentation ou de l'utilisation de Master Butler dans le système Miyukini Core System v2.4.

Ce contrat précise ce qui constitue une violation, les catégories de violations, les anti-patterns identifiés, et les conséquences associées.

### Portée

Ce contrat s'applique à **toutes les implémentations et utilisations de Master Butler** et définit de manière absolue :
- la définition formelle d'une violation,
- les catégories de violations,
- le catalogue des violations explicites,
- les anti-patterns à éviter,
- les conséquences des violations.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat **référence et consolide** les violations définies dans :
- **Master Butler — Documentation Fondatrice**
- **Master Butler — Capability Registry Contract**
- **Master Butler — Permission Registry Contract**
- **Master Butler — Capability API Contract**
- **Master Butler — Permission API Contract**
- **Master Butler — Boundary & Scope Contract**
- **Master Butler — Tool Governance Contract**
- **Master Butler — Audit & Traceability Contract**
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Violations des lois d'autonomie système

Ce contrat est le **catalogue de référence** pour toutes les violations Master Butler.

---

## 2. Définition d'une violation

### 2.1. Nature d'une violation

Une **violation** est un non-respect d'une règle, d'un invariant, ou d'une garantie définie dans les contrats Master Butler.

**Caractéristiques d'une violation :**

- **Contractuelle** : Une violation concerne toujours un contrat spécifique
- **Identifiable** : Une violation peut être identifiée et référencée
- **Conséquentielle** : Une violation a des conséquences définies
- **Non-tolérable** : Une violation ne peut pas être ignorée ou tolérée

### 2.2. Gravité des violations

Les violations sont classées selon leur gravité :

**CRITIQUE :**

Violation d'un invariant fondamental ou d'une interdiction absolue. La violation compromet l'intégrité de Master Butler.

**MAJEURE :**

Violation d'une règle importante qui affecte le comportement de Master Butler mais ne compromet pas ses propriétés fondamentales.

**MINEURE :**

Violation d'une règle secondaire qui n'affecte pas le comportement principal de Master Butler.

---

## 3. Catégories de violations

### 3.1. Violations de décision

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice, INV-MB-2

**Violations :**

**VIOL-DEC-1 : Production de décision d'autorisation**

Master Butler produit une décision "autorisé" ou "refusé" pour une action.

*Invariant violé : INV-MB-2 (Non-décision)*

**VIOL-DEC-2 : Vérification de permissions en temps réel**

Master Butler vérifie si un contexte possède effectivement une permission au moment d'une action, au lieu de simplement fournir les informations.

*Invariant violé : INV-MB-2 (Non-décision)*

**VIOL-DEC-3 : Retour d'un booléen d'autorisation**

Une méthode de Master Butler retourne un booléen d'autorisation (isAuthorized, canAccess, etc.).

*Invariant violé : INV-MB-2 (Non-décision)*

**VIOL-DEC-4 : Suggestion de décision**

Master Butler suggère ou recommande une décision à StrongFather.

*Invariant violé : INV-MB-2 (Non-décision)*

### 3.2. Violations d'exécution

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice, Section 6

**Violations :**

**VIOL-EXEC-1 : Exécution d'action fonctionnelle**

Master Butler exécute une action fonctionnelle (création de contenu, modification de hiérarchie, etc.).

*Règle violée : "Master Butler n'exécute jamais"*

**VIOL-EXEC-2 : Stockage de données métier**

Master Butler stocke des données métier au lieu de métadonnées sur les capacités et permissions.

*Règle violée : "Master Butler ne stocke jamais de données métier"*

**VIOL-EXEC-3 : Application de contraintes métier**

Master Butler applique des contraintes métier (limites, règles de gestion, quotas).

*Règle violée : "Master Butler n'applique jamais de contraintes métier"*

**VIOL-EXEC-4 : Définition de politiques**

Master Butler définit des politiques de décision au lieu de fournir les informations aux décideurs.

*Règle violée : "Master Butler ne définit jamais de politiques"*

### 3.3. Violations de registre

**Catégorie :** CRITIQUE à MAJEURE

**Source :** Capability Registry Contract, Permission Registry Contract, INV-MB-1

**Violations :**

**VIOL-REG-1 : Capacité non recensée** (CRITIQUE)

Une capacité est exposée par un module sans être déclarée dans Master Butler.

*Invariant violé : INV-MB-1 (Exhaustivité du registre)*

**VIOL-REG-2 : Permission sans capacité associée** (MAJEURE)

Une permission est définie sans référencer au moins une capacité existante.

*Règle violée : Permission Registry Contract*

**VIOL-REG-3 : Capacité fantôme**

Une capacité référencée dans une permission n'existe pas dans le registre.

*Invariant violé : INV-MB-6 (Séparation capacité/permission)*

**VIOL-REG-4 : Registre partiel**

Le registre ne contient qu'un sous-ensemble des capacités du système.

*Invariant violé : INV-MB-1 (Exhaustivité du registre)*

### 3.4. Violations d'identifiant

**Catégorie :** CRITIQUE

**Source :** Documentation Fondatrice, INV-MB-4

**Violations :**

**VIOL-ID-1 : Mutation d'identifiant**

Un identifiant de capacité est modifié après sa déclaration initiale.

*Invariant violé : INV-MB-4 (Immutabilité des identifiants)*

**VIOL-ID-2 : Réutilisation d'identifiant**

Un identifiant de capacité supprimée est réutilisé pour une nouvelle capacité.

*Invariant violé : INV-MB-4 (Immutabilité des identifiants)*

**VIOL-ID-3 : Identifiant ambigu**

Un identifiant ne permet pas d'identifier de manière unique une capacité.

*Invariant violé : INV-MB-4 (Immutabilité des identifiants)*

### 3.5. Violations de traçabilité

**Catégorie :** MAJEURE

**Source :** Audit & Traceability Contract, INV-MB-5

**Violations :**

**VIOL-TRACE-1 : Déclaration sans trace**

Une déclaration de capacité n'est pas tracée.

*Invariant violé : INV-MB-5 (Traçabilité complète)*

**VIOL-TRACE-2 : Définition sans trace**

Une définition de permission n'est pas tracée.

*Invariant violé : INV-MB-5 (Traçabilité complète)*

**VIOL-TRACE-3 : Modification silencieuse**

Une modification du registre est effectuée sans trace.

*Invariant violé : INV-MB-5 (Traçabilité complète)*

**VIOL-TRACE-4 : Suppression sans historique**

Une capacité ou permission est supprimée sans conservation de l'historique.

*Invariant violé : INV-MB-5 (Traçabilité complète)*

### 3.6. Violations de frontière

**Catégorie :** MAJEURE

**Source :** Boundary & Scope Contract, INV-MB-7

**Violations :**

**VIOL-BOUND-1 : Logique métier intégrée**

Master Butler contient de la logique métier (règles de domaine, validations métier).

*Invariant violé : INV-MB-7 (Pas de logique métier)*

**VIOL-BOUND-2 : Gestion d'identité**

Master Butler gère les identités des utilisateurs ou des systèmes.

*Règle violée : "Master Butler ne gère jamais les identités"*

**VIOL-BOUND-3 : Persistance directe**

Master Butler manipule directement une base de données ou un système de fichiers.

*Règle violée : "Master Butler ne persiste jamais directement"*

**VIOL-BOUND-4 : Validation métier**

Master Butler valide des actions selon des critères métier.

*Invariant violé : INV-MB-7 (Pas de logique métier)*

### 3.7. Violations d'accessibilité

**Catégorie :** MAJEURE

**Source :** Documentation Fondatrice, INV-MB-8

**Violations :**

**VIOL-ACCESS-1 : Composant bloqué**

Un composant autorisé est empêché d'interroger Master Butler.

*Invariant violé : INV-MB-8 (Accessibilité universelle)*

**VIOL-ACCESS-2 : Accès sélectif injustifié**

L'accès à Master Butler est restreint sans justification de sécurité.

*Invariant violé : INV-MB-8 (Accessibilité universelle)*

### 3.8. Violations de Tools et Toolkits

**Catégorie :** CRITIQUE

**Source :** Tool Governance Contract, Toolkit Composition Contract

**Violations :**

**VIOL-TOOL-1 : Tool non déclaré**

Un Tool est utilisé dans le système sans être déclaré dans Master Butler.

*Règle violée : "Pas d'injection sauvage"*

**VIOL-TOOL-2 : Tool local non gouverné**

Un Tool "local" existe sans déclaration dans l'environnement.

*Règle violée : "Pas de Tool local"*

**VIOL-TOOL-3 : Dépendance externe cachée**

Une librairie externe non gouvernée est utilisée.

*Règle violée : "Pas de dépendance externe cachée"*

**VIOL-TOOL-4 : Toolkit avec capacité nouvelle**

Un Toolkit crée une capacité nouvelle au lieu d'orchestrer des Tools existants.

*Règle violée : "Un Toolkit orchestre, mais n'ajoute pas de capacité"*

---

## 4. Anti-patterns

### 4.1. Anti-pattern : Master Butler comme décideur

**Description :**

Utiliser Master Butler pour prendre des décisions d'autorisation au lieu de simplement fournir les informations sur les capacités et permissions.

**Pourquoi c'est un anti-pattern :**

Master Butler est un registre d'information, pas un moteur de décision. Les décisions d'autorisation appartiennent exclusivement à StrongFather.

**Symptômes :**

- Master Butler retourne "autorisé" ou "refusé"
- Master Butler évalue si une action peut être effectuée
- Master Butler bloque des opérations
- Master Butler filtre selon des critères de décision

**Solution :**

Master Butler fournit les informations (capacités existantes, permissions définies, associations). StrongFather prend les décisions.

### 4.2. Anti-pattern : Master Butler comme contrôleur d'accès

**Description :**

Utiliser Master Butler pour contrôler l'accès en temps réel, vérifiant si un utilisateur peut effectuer une action.

**Pourquoi c'est un anti-pattern :**

Le contrôle d'accès en temps réel est une décision. Master Butler fournit les définitions de permissions, pas les vérifications.

**Symptômes :**

- Master Butler vérifie des permissions à chaque requête
- Master Butler maintient un état de session
- Master Butler rejette des requêtes non autorisées

**Solution :**

Master Butler fournit les définitions de permissions. StrongFather effectue les vérifications lors de l'évaluation des intentions.

### 4.3. Anti-pattern : Registre dispersé

**Description :**

Maintenir plusieurs registres de capacités dans différents composants au lieu d'utiliser Master Butler comme source unique.

**Pourquoi c'est un anti-pattern :**

La dispersion des registres viole l'exhaustivité et crée des incohérences. Master Butler est le registre central et unique.

**Symptômes :**

- Chaque module maintient sa propre liste de capacités
- Les permissions sont définies localement dans les produits
- Il existe des capacités non connues de Master Butler

**Solution :**

Tous les modules déclarent leurs capacités à Master Butler. Aucun registre local de capacités n'est maintenu.

### 4.4. Anti-pattern : Capacités implicites

**Description :**

Exposer des capacités fonctionnelles sans les déclarer explicitement dans Master Butler.

**Pourquoi c'est un anti-pattern :**

Les capacités implicites violent l'exhaustivité du registre et créent des zones d'ombre dans le système.

**Symptômes :**

- Un module expose des actions non documentées
- Des fonctionnalités existent sans permission associée
- La découverte ne révèle pas toutes les possibilités

**Solution :**

Toute capacité exposée est déclarée dans Master Butler. Pas de capacité implicite ou cachée.

### 4.5. Anti-pattern : Master Butler comme gestionnaire d'identité

**Description :**

Utiliser Master Butler pour gérer les identités des utilisateurs (création, authentification, sessions).

**Pourquoi c'est un anti-pattern :**

Master Butler gère les capacités et permissions, pas les identités. L'identité appartient au système d'authentification.

**Symptômes :**

- Master Butler stocke des utilisateurs
- Master Butler vérifie des credentials
- Master Butler maintient des sessions

**Solution :**

Master Butler connaît les associations rôles-permissions. L'identité et l'attribution des rôles appartiennent au système d'authentification.

### 4.6. Anti-pattern : Permissions dynamiques calculées

**Description :**

Faire calculer dynamiquement les permissions par Master Butler selon des règles métier au lieu de les définir explicitement.

**Pourquoi c'est un anti-pattern :**

Master Butler définit les permissions, il ne les calcule pas. Le calcul dynamique introduit de la logique métier.

**Symptômes :**

- Les permissions changent selon le contexte métier
- Master Butler applique des règles de calcul
- Les permissions sont générées à la volée

**Solution :**

Les permissions sont définies explicitement. Le contexte de capacité peut filtrer, mais ne calcule pas de nouvelles permissions.

### 4.7. Anti-pattern : Couplage Tool-Permission direct

**Description :**

Associer directement les Tools aux permissions sans passer par les capacités.

**Pourquoi c'est un anti-pattern :**

La structure est Capability → Tool et Permission → Capability. Un Tool n'est pas directement lié à une permission.

**Symptômes :**

- Les permissions référencent directement des Tools
- Les Tools vérifient des permissions
- La séparation capacité/permission est ignorée

**Solution :**

Les capacités sont liées aux Tools. Les permissions référencent des capacités. La chaîne est : Permission → Capability → Tool.

### 4.8. Anti-pattern : Master Butler comme orchestrateur de Tools

**Description :**

Utiliser Master Butler pour orchestrer l'exécution des Tools au lieu de simplement les cataloguer.

**Pourquoi c'est un anti-pattern :**

Master Butler catalogue les Tools, il ne les exécute pas. L'exécution appartient aux Tools eux-mêmes.

**Symptômes :**

- Master Butler appelle des Tools
- Master Butler séquence des opérations
- Master Butler gère le cycle de vie des Tools

**Solution :**

Master Butler répond "quels Tools existent". L'orchestration appartient aux composants appelants.

---

## 5. Conséquences des violations

### 5.1. Violations critiques

**Conséquences :**

1. **Non-conformité immédiate** : L'implémentation est considérée non conforme
2. **Arrêt requis** : L'opération en cours doit être arrêtée
3. **Audit obligatoire** : Un audit doit être effectué
4. **Correction impérative** : La correction est obligatoire avant toute utilisation

### 5.2. Violations majeures

**Conséquences :**

1. **Warning de non-conformité** : L'implémentation est signalée comme non conforme
2. **Résultat invalide** : Le résultat associé est invalide
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

Ce contrat est le **catalogue de référence** pour toutes les violations Master Butler. Toute nouvelle violation doit être ajoutée à ce catalogue.

---

## 7. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable le catalogue des violations et anti-patterns de Master Butler.

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
- ✅ Documentation Fondatrice : INV-MB-1 à INV-MB-8
- ✅ Capability Registry Contract : VIOL-REG-*
- ✅ Permission Registry Contract : VIOL-REG-*
- ✅ Tool Governance Contract : VIOL-TOOL-*
- ✅ Boundary & Scope Contract : VIOL-BOUND-*
- ✅ Audit & Traceability Contract : VIOL-TRACE-*

### 8.2. Vérification de cohérence

- ✅ Toutes les violations référencent un contrat source
- ✅ Toutes les violations référencent un invariant ou une règle
- ✅ Les gravités sont cohérentes avec l'importance des règles

---

**Document créé le :** 2026-01-27  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, Master Butler Documentation Fondatrice  
**Type :** Catalogue des violations et anti-patterns non négociable

---

## 9. Mini log de génération

### Décision éditoriale E1 : Consolidation des violations

**Décision prise :** Consolidation de toutes les violations dispersées dans les contrats en un catalogue unique.

**Application :** Chaque violation référence son contrat et invariant source (INV-MB-*).

### Décision éditoriale E2 : Anti-patterns spécifiques

**Décision prise :** Inclusion d'anti-patterns spécifiques au rôle de registre de Master Butler.

**Application :** 8 anti-patterns identifiés et documentés, centrés sur la distinction registre/décideur.

### Décision éditoriale E3 : Violations Tools et Toolkits

**Décision prise :** Inclusion des violations liées à la gouvernance des Tools et Toolkits.

**Application :** Section VIOL-TOOL-* couvrant les règles de souveraineté applicative.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Toutes les violations des contrats sont incluses
- ✅ Les références aux invariants sont correctes
- ✅ Les gravités sont cohérentes

**Conclusion :** Catalogue complet et cohérent.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
