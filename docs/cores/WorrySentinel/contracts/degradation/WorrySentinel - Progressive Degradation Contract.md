# WorrySentinel — Progressive Degradation Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **WorrySentinel — Progressive Degradation Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles absolues de dégradation progressive du système, les principes de réduction contrôlée des capacités, les mécanismes de préservation de l'intégrité, et les garanties de non-blocage brutal de l'écosystème Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle de la dégradation progressive, les règles d'orchestration, les capacités désactivées par niveau de confiance, et les garanties de protection, sans jamais introduire de détail d'implémentation technique, de mécanisme algorithmique concret, ou de contrôle procédural.

### Portée

Ce contrat s'applique à **toutes les opérations impliquant une dégradation de capacités** dans WorrySentinel et définit de manière absolue :
- le principe fondamental de dégradation progressive,
- les règles de dégradation par état de confiance (T0-T4),
- les capacités désactivées et restrictions à chaque niveau,
- l'interaction entre niveaux de sécurité (0-4) et états de confiance (T0-T4),
- les invariants de dégradation progressive,
- les garanties de préservation de l'intégrité,
- la distinction entre dégradation et blocage brutal.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **WorrySentinel — Documentation Fondatrice** : Source des principes de dégradation (Section 8)
- **WorrySentinel — Trust States Governance Contract** : Contrat parent définissant les états T0-T4
- **WorrySentinel — Security Levels Governance Contract** : Contrat définissant les niveaux 0-4
- **WorrySentinel — Invariants & Guarantees** : Catalogue consolidé des invariants WorrySentinel
- **[Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Référence conceptuelle du système de dégradation
- **CaringNanny — Documentation Fondatrice** : Core responsable de la consolidation des signaux d'intégrité
- **StrongFather — Documentation Fondatrice** : Core responsable des décisions selon les états de confiance
- **TAMR — Documentation Fondatrice** : Mécanisme d'intervention humaine en états dégradés
- **LogisticsSteward — Documentation Fondatrice** : Core adaptant ses quotas selon l'état de confiance

Il n'introduit aucune contradiction et constitue la définition formelle de la dégradation progressive dans WorrySentinel.

---

## 2. Principe fondamental

### 2.1. Axiome de dégradation

**"Un système autonome ne bloque jamais brutalement. Il observe, interprète, dégrade, puis bloque seulement quand il est sûr."**

Cet axiome est la fondation conceptuelle de toute la logique de dégradation progressive dans l'écosystème Miyukini.

### 2.2. Implications de l'axiome

**IMPL-AX-1 : Progression contrôlée**

La dégradation est toujours progressive, jamais instantanée. Le système passe par des états intermédiaires avant d'atteindre un blocage total.

**IMPL-AX-2 : Explicabilité interne**

Toute dégradation est explicable. Le système peut toujours justifier pourquoi il dégrade ses capacités.

**IMPL-AX-3 : Observation avant action**

Le système observe et interprète avant de dégrader. Aucune dégradation préventive sans signaux consolidés.

**IMPL-AX-4 : Blocage uniquement sur certitude**

Le blocage total (T4) n'intervient que lorsque le système est certain de la compromission. Le doute conduit à la dégradation, pas au blocage.

### 2.3. Ce que la dégradation progressive n'est pas

| Ce que la dégradation n'est PAS | Description |
|--------------------------------|-------------|
| Un blocage brutal | Le système ne coupe jamais toutes les capacités instantanément |
| Une mesure préventive aveugle | La dégradation répond toujours à des signaux consolidés |
| Une punition | La dégradation protège, elle ne punit pas |
| Un état permanent | La dégradation est réversible (sauf T4 sans intervention) |
| Une décision unilatérale | La dégradation suit des règles explicites et tracées |

---

## 3. Règles de dégradation progressive

### 3.1. Règle RÈGLE-DEGRAD-1 : Dégradation par niveau

WorrySentinel gouverne la dégradation progressive selon les états de confiance :

| Transition | Effet sur les capacités |
|------------|------------------------|
| **T0 → T1** | Aucune dégradation de capacité, uniquement surveillance renforcée |
| **T1 → T2** | Dégradation légère, certaines capacités non essentielles désactivées |
| **T2 → T3** | Dégradation modérée, gel des produits non essentiels |
| **T3 → T4** | Dégradation totale, arrêt opérationnel |

**Principe :** La sévérité de la dégradation est proportionnelle à la gravité de la menace détectée.

### 3.2. Règle RÈGLE-DEGRAD-2 : Préservation des invariants

La dégradation progressive ne peut jamais compromettre les invariants FONDATION.

**Garantie absolue :**
- ✅ En T0, tous les invariants sont préservés
- ✅ En T1, tous les invariants sont préservés
- ✅ En T2, tous les invariants sont préservés
- ✅ En T3, tous les invariants sont préservés
- ✅ En T4, tous les invariants sont préservés

**Corollaire :** Même en état de blocage total (T4), le système ne corrompt jamais ses données et ne viole jamais un invariant FONDATION.

### 3.3. Règle RÈGLE-DEGRAD-3 : Explicabilité

Toute dégradation est explicable. WorrySentinel gouverne les règles selon lesquelles chaque dégradation DOIT être justifiée et tracée.

**Éléments de traçabilité obligatoires :**
| Élément | Description |
|---------|-------------|
| **État source** | État de confiance avant dégradation |
| **État cible** | État de confiance après dégradation |
| **Raison** | Justification conceptuelle de la dégradation |
| **Signaux** | Signaux consolidés ayant déclenché la dégradation |
| **Capacités affectées** | Liste des capacités désactivées ou restreintes |
| **Horodatage** | Moment de la dégradation |
| **Contexte** | Informations contextuelles pertinentes |

### 3.4. Règle RÈGLE-DEGRAD-4 : Interaction avec niveaux de sécurité

WorrySentinel gouverne l'interaction entre les niveaux de sécurité (0-4) et les états de confiance (T0-T4) :

**Principe d'interaction :**
- Un produit de niveau de sécurité N en état de confiance T doit adapter son comportement selon les deux dimensions
- Les restrictions sont **cumulatives** : niveau de sécurité élevé + état de confiance dégradé = restrictions maximales

**Matrice d'interaction simplifiée :**

| Niveau \ État | T0 | T1 | T2 | T3 | T4 |
|---------------|----|----|----|----|----| 
| **Niveau 0** | Normal | + Traces | Bridé | Minimal | Bloqué |
| **Niveau 1** | Normal | + Traces | Bridé | Minimal | Bloqué |
| **Niveau 2** | Normal+ | + Traces+ | Bridé+ | Minimal+ | Bloqué |
| **Niveau 3** | Strict | + Traces++ | Très bridé | Ultra-minimal | Bloqué |
| **Niveau 4** | Maximum | + Traces+++ | Maximum bridé | Critique | Bloqué |

**Légende :**
- Normal : Fonctionnement standard pour ce niveau de sécurité
- + : Renforcement des contraintes
- Bridé : Capacités réduites
- Minimal : Mode minimal uniquement
- Bloqué : Aucune opération métier

---

## 4. Dégradation par état de confiance

### 4.1. État T0 — Aucune dégradation

**Situation :** Système sain, aucune anomalie détectée.

**Dégradation :** Aucune

**Capacités préservées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ✅ 100% |
| Extensions dynamiques | ✅ 100% |
| Nouveaux modules | ✅ 100% |
| Décisions critiques | ✅ Normales |
| Fonctions sensibles | ✅ 100% |
| Intégrations externes | ✅ 100% |

**Impact sur les cores :**
| Core | Impact |
|------|--------|
| StrongFather | Décisions normales |
| MasterButler | Permissions normales |
| BorderGuard | I/O normal |
| LogisticsSteward | Quotas normaux |
| TAMR | Droits humains normaux |
| Kernel | Sondes standard |

### 4.2. État T1 — Dégradation nulle, surveillance renforcée

**Situation :** Anomalie détectée, pas encore confirmée.

**Dégradation :** Aucune dégradation de capacité opérationnelle

**Capacités préservées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ✅ 100% |
| Extensions dynamiques | ✅ 100% (avec traçabilité) |
| Nouveaux modules | ✅ 100% (avec traçabilité) |
| Décisions critiques | ✅ Normales (avec log renforcé) |
| Fonctions sensibles | ✅ 100% (avec surveillance) |
| Intégrations externes | ✅ 100% (avec surveillance) |

**Renforcements (non dégradants) :**
| Renforcement | Description |
|--------------|-------------|
| **R-T1-1** | Log renforcé : niveau de détail accru |
| **R-T1-2** | Traçabilité étendue : toutes les opérations tracées |
| **R-T1-3** | Surveillance accrue : patterns comportementaux |
| **R-T1-4** | Monitoring : fréquence accrue des sondes |

**Impact sur les cores :**
| Core | Impact |
|------|--------|
| StrongFather | Décisions normales + log renforcé |
| MasterButler | Permissions normales + traces |
| BorderGuard | I/O normal + surveillance |
| LogisticsSteward | Quotas normaux + monitoring |
| TAMR | Droits humains normaux |
| Kernel | Sondes renforcées |

**Principe :** T1 n'est pas une dégradation, c'est une vigilance accrue.

### 4.3. État T2 — Dégradation légère

**Situation :** Incohérence persistante, suspicion modérée.

**Dégradation :** Légère — certaines capacités non essentielles désactivées

**Capacités affectées :**
| Capacité | Statut | Dégradation |
|----------|--------|-------------|
| Opérations normales | ✅ Disponibles | Aucune |
| Extensions dynamiques | ❌ Bloquées | 100% |
| Nouveaux modules | ❌ Bloqués | 100% |
| Décisions critiques | ⚠️ Strictes | Seuils abaissés |
| Fonctions sensibles | ⚠️ Bridées | Partielle |
| Intégrations externes | ⚠️ Restrictives | Partielle |

**Restrictions appliquées :**
| Restriction | Code | Description |
|-------------|------|-------------|
| Gel des extensions | R-T2-1 | Aucune extension dynamique autorisée |
| Gel des modules | R-T2-2 | Aucun nouveau module autorisé |
| Seuils de décision | R-T2-3 | Seuils StrongFather abaissés (plus de refus) |
| Bridage fonctionnel | R-T2-4 | Fonctions sensibles partiellement désactivées |
| Monitoring visible | R-T2-5 | État visible dans MiyukiniAdmin |
| Quotas restrictifs | R-T2-6 | LogisticsSteward applique des quotas réduits |

**Impact sur les cores :**
| Core | Impact |
|------|--------|
| StrongFather | Décisions plus strictes, seuils abaissés |
| MasterButler | Permissions restrictives |
| BorderGuard | I/O durci |
| LogisticsSteward | Quotas restrictifs |
| TAMR | Droits humains normaux (surveillance) |
| Kernel | Sondes haute fréquence |

### 4.4. État T3 — Dégradation modérée

**Situation :** Suspicion forte, intégrité potentiellement compromise.

**Dégradation :** Modérée — gel des produits non essentiels

**Capacités affectées :**
| Capacité | Statut | Dégradation |
|----------|--------|-------------|
| Opérations normales | ⚠️ Mode minimal | Importante |
| Extensions dynamiques | ❌ Bloquées | 100% |
| Nouveaux modules | ❌ Bloqués | 100% |
| Décisions critiques | ⚠️ AMBIGUË/DIFFÉRÉE | Maximale |
| Fonctions sensibles | ❌ Bloquées | 100% |
| Intégrations externes | ❌ Gelées | 100% |
| Produits non essentiels | ❌ Gelés | 100% |

**Restrictions appliquées :**
| Restriction | Code | Description |
|-------------|------|-------------|
| Gel des produits | R-T3-1 | Produits non essentiels gelés |
| Mode minimal | R-T3-2 | Uniquement fonctions critiques |
| Décisions TAMR | R-T3-3 | Décisions critiques requièrent TAMR |
| Blocage sensible | R-T3-4 | Fonctions sensibles bloquées |
| Aucune intégration | R-T3-5 | Nouvelles intégrations refusées |
| Audit continu | R-T3-6 | Audit obligatoire de toutes les opérations |
| Quotas minimaux | R-T3-7 | LogisticsSteward en mode survie |

**Impact sur les cores :**
| Core | Impact |
|------|--------|
| StrongFather | Décisions critiques → AMBIGUË/DIFFÉRÉE |
| MasterButler | Permissions minimales |
| BorderGuard | I/O en mode défensif |
| LogisticsSteward | Mode survie |
| TAMR | Validation requise pour décisions critiques |
| Kernel | Sondes de diagnostic |

**Intervention humaine :**
- TAMR requis pour toute décision critique
- TAMR peut autoriser un override vers T2 si confirmation de sécurité

### 4.5. État T4 — Dégradation totale

**Situation :** Intégrité rompue, système compromis.

**Dégradation :** Totale — arrêt opérationnel

**Capacités affectées :**
| Capacité | Statut | Dégradation |
|----------|--------|-------------|
| Opérations normales | ❌ Bloquées | 100% |
| Extensions dynamiques | ❌ Bloquées | 100% |
| Nouveaux modules | ❌ Bloqués | 100% |
| Décisions critiques | ❌ Bloquées | 100% |
| Fonctions sensibles | ❌ Bloquées | 100% |
| Intégrations externes | ❌ Bloquées | 100% |
| Produits | ❌ Tous bloqués | 100% |

**Capacités préservées (non dégradables) :**
| Capacité | Statut | Justification |
|----------|--------|---------------|
| Diagnostics | ✅ Disponibles | Nécessaires pour analyse |
| Lecture d'état | ✅ Disponible | Nécessaire pour diagnostic |
| Sortie propre | ✅ Disponible | Shutdown graceful toujours possible |
| Intégrité des données | ✅ Préservée | Invariant FONDATION |

**Restrictions appliquées :**
| Restriction | Code | Description |
|-------------|------|-------------|
| Arrêt opérationnel | R-T4-1 | Aucune opération métier |
| Diagnostic seul | R-T4-2 | Uniquement lecture et analyse |
| Sortie propre | R-T4-3 | Shutdown graceful autorisé |
| Non-corruption | R-T4-4 | Invariant : jamais de corruption |
| Non-exécution sauvage | R-T4-5 | Invariant : jamais d'exécution non contrôlée |

**Impact sur les cores :**
| Core | Impact |
|------|--------|
| StrongFather | Aucune décision opérationnelle |
| MasterButler | Aucune permission |
| BorderGuard | I/O bloqué (sauf diagnostics) |
| LogisticsSteward | Arrêté |
| TAMR | Mode diagnostic uniquement |
| Kernel | Mode diagnostic uniquement |

**Garanties absolues en T4 :**
- 📌 Jamais de corruption des données
- 📌 Jamais d'exécution sauvage
- 📌 État toujours lisible
- 📌 Sortie propre toujours possible

---

## 5. Orchestration de la dégradation

### 5.1. Rôle de WorrySentinel

WorrySentinel **gouverne** l'orchestration de la dégradation mais ne l'**exécute** pas directement :

| Responsabilité | WorrySentinel | Autres cores |
|----------------|---------------|--------------|
| Règles de dégradation | ✅ Définit | ❌ |
| Capacités par état | ✅ Définit | ❌ |
| Restrictions par état | ✅ Définit | ❌ |
| Interaction niveaux/états | ✅ Définit | ❌ |
| Détection d'anomalies | ❌ | CaringNanny |
| Décision de transition | ❌ | StrongFather |
| Application des restrictions | ❌ | Chaque core |
| Intervention humaine | ❌ | TAMR |

### 5.2. Flux d'orchestration

```
Anomalie détectée (Sondes Kernel)
         │
         ▼
CaringNanny (consolidation des signaux)
         │
         ▼
StrongFather (évaluation, décision de transition)
         │
         ▼
WorrySentinel (règles de dégradation applicables)
         │
         ▼
Propagation aux cores (application des restrictions)
         │
         ├─→ StrongFather : ajuste sévérité
         ├─→ MasterButler : ajuste permissions
         ├─→ BorderGuard : durcit I/O
         ├─→ LogisticsSteward : ajuste quotas
         ├─→ TAMR : ajuste droits humains
         └─→ Kernel : ajuste fréquence sondes
```

### 5.3. Règles d'orchestration

**ORCH-1 : Propagation immédiate**

Toute transition d'état DOIT déclencher une propagation immédiate des nouvelles restrictions à tous les cores concernés.

**ORCH-2 : Application atomique**

L'application des restrictions DOIT être atomique. Soit toutes les restrictions sont appliquées, soit aucune.

**ORCH-3 : Non-ignorabilité**

Aucun core ne peut ignorer les restrictions imposées par l'état de confiance courant.

**ORCH-4 : Ordre de propagation**

L'ordre de propagation est défini par WorrySentinel :
1. StrongFather (décisions)
2. MasterButler (permissions)
3. BorderGuard (frontières)
4. LogisticsSteward (ressources)
5. TAMR (droits humains)
6. Kernel (sondes)

**ORCH-5 : Rollback interdit**

Une fois une dégradation appliquée, le retour à un état moins dégradé ne peut se faire que via une transition d'état formelle, jamais via un rollback direct des restrictions.

---

## 6. Dégradation et produits

### 6.1. Impact sur les produits

Les produits de l'écosystème Miyukini sont affectés par la dégradation selon deux dimensions :
- Leur niveau de sécurité intrinsèque (0-4)
- L'état de confiance courant du système (T0-T4)

**Matrice d'impact produit :**

| État | Produits Niveau 0-1 | Produits Niveau 2 | Produits Niveau 3-4 |
|------|---------------------|-------------------|---------------------|
| T0 | Fonctionnement normal | Fonctionnement normal | Fonctionnement strict |
| T1 | Normal + traces | Normal + traces+ | Strict + traces++ |
| T2 | Bridé | Très bridé | Maximum bridé |
| T3 | Gelé si non essentiel | Mode minimal | Ultra-minimal |
| T4 | Bloqué | Bloqué | Bloqué |

### 6.2. Classification des produits

**Produits essentiels :**
- Continuent en mode minimal jusqu'en T3
- Uniquement diagnostics en T4

**Produits non essentiels :**
- Gelés dès T3
- Bloqués en T4

**Règle de classification :**
WorrySentinel gouverne les critères de classification essentiel/non essentiel. Un produit est essentiel si son arrêt compromettrait l'intégrité du système ou empêcherait les diagnostics.

### 6.3. Adaptation comportementale des produits

**ADAPT-1 : Obligation d'adaptation**

Tout produit DOIT adapter son comportement selon l'état de confiance courant.

**ADAPT-2 : Dégradation gracieuse**

Les produits DOIVENT implémenter une dégradation gracieuse de leurs fonctionnalités selon les restrictions applicables.

**ADAPT-3 : Non-contournement**

Aucun produit ne peut contourner les restrictions de dégradation imposées par l'état de confiance.

**ADAPT-4 : Signalement**

Les produits DOIVENT signaler leur état de dégradation à BondingBrother pour visibilité.

---

## 7. Invariants de dégradation progressive

### 7.1. Invariants de processus

**INV-DEG-1 : Séquentialité**

La dégradation est toujours séquentielle. Aucun saut d'état n'est autorisé.

**INV-DEG-2 : Progressivité**

La dégradation est toujours progressive. Chaque transition est justifiée par des signaux consolidés.

**INV-DEG-3 : Réversibilité conditionnelle**

La dégradation est réversible via une transition d'état formelle, sauf pour T4 qui est terminal sans intervention humaine.

### 7.2. Invariants de protection

**INV-DEG-4 : Préservation des invariants FONDATION**

Aucune dégradation ne peut compromettre un invariant FONDATION.

**INV-DEG-5 : Non-corruption**

La dégradation ne peut jamais conduire à une corruption de données.

**INV-DEG-6 : Non-exécution sauvage**

La dégradation ne peut jamais conduire à une exécution non contrôlée.

### 7.3. Invariants de gouvernance

**INV-DEG-7 : WorrySentinel gouverne, n'exécute pas**

WorrySentinel définit les règles de dégradation mais n'exécute jamais directement une dégradation.

**INV-DEG-8 : Traçabilité complète**

Toute dégradation est traçable avec justification complète.

**INV-DEG-9 : Explicabilité**

Toute dégradation est explicable. Le système peut toujours justifier pourquoi il dégrade.

**INV-DEG-10 : Proportionnalité**

La sévérité de la dégradation est proportionnelle à la gravité de la menace.

---

## 8. Garanties offertes

### 8.1. Garanties de processus

**G-DEG-1 : Jamais de blocage brutal**

WorrySentinel garantit que le système ne bloque jamais brutalement. La dégradation est toujours progressive.

**G-DEG-2 : Observation avant dégradation**

WorrySentinel garantit que toute dégradation est précédée d'une observation et consolidation des signaux.

**G-DEG-3 : Explicabilité complète**

WorrySentinel garantit que toute dégradation est explicable avec justification.

### 8.2. Garanties de capacités

**G-DEG-4 : Capacités T0-T1 préservées**

En états T0 et T1, toutes les capacités opérationnelles sont préservées.

**G-DEG-5 : Diagnostics toujours disponibles**

Même en T4, les capacités de diagnostic restent disponibles.

**G-DEG-6 : Sortie propre toujours possible**

En tout état, une sortie propre (shutdown graceful) reste possible.

### 8.3. Garanties de protection

**G-DEG-7 : Invariants préservés**

En tout état (T0 à T4), les invariants FONDATION sont préservés.

**G-DEG-8 : Non-corruption garantie**

WorrySentinel garantit qu'aucune dégradation ne corrompt les données.

**G-DEG-9 : Non-exécution sauvage garantie**

WorrySentinel garantit qu'aucune dégradation ne conduit à une exécution non contrôlée.

### 8.4. Garanties de réversibilité

**G-DEG-10 : Réversibilité T1 → T0**

Le retour de T1 à T0 est possible si l'anomalie est résolue.

**G-DEG-11 : Réversibilité T2 → T1**

Le retour de T2 à T1 est possible si l'état s'améliore.

**G-DEG-12 : Réversibilité T3 → T2**

Le retour de T3 à T2 est possible via validation TAMR.

---

## 9. Violations et comportements interdits

### 9.1. Violations de processus

**VIOL-DEG-1 : Blocage brutal**

Le système bloque toutes les capacités instantanément sans passer par les états intermédiaires.

*Violation :* INV-DEG-1, INV-DEG-2

**VIOL-DEG-2 : Saut d'état**

Une dégradation saute un état intermédiaire (ex: T0 → T3 directement).

*Violation :* INV-DEG-1

**VIOL-DEG-3 : Dégradation sans justification**

Une dégradation se produit sans signaux consolidés ni justification.

*Violation :* INV-DEG-2, INV-DEG-8

### 9.2. Violations de protection

**VIOL-DEG-4 : Corruption par dégradation**

Une dégradation conduit à une corruption de données.

*Violation :* INV-DEG-5, Invariants FONDATION

**VIOL-DEG-5 : Exécution sauvage**

Une dégradation conduit à une exécution non contrôlée.

*Violation :* INV-DEG-6, Invariants FONDATION

**VIOL-DEG-6 : Violation d'invariant**

Une dégradation compromet un invariant FONDATION.

*Violation :* INV-DEG-4

### 9.3. Violations de gouvernance

**VIOL-DEG-7 : Exécution par WorrySentinel**

WorrySentinel exécute directement une dégradation au lieu de gouverner les règles.

*Violation :* INV-DEG-7, INV-WS-2

**VIOL-DEG-8 : Contournement des restrictions**

Un composant contourne les restrictions de dégradation imposées.

*Violation :* ORCH-3, INV-DEG-10

**VIOL-DEG-9 : Rollback direct**

Un composant effectue un rollback direct des restrictions sans transition d'état formelle.

*Violation :* ORCH-5

### 9.4. Comportements interdits

**INTERD-DEG-1 : Dégradation préventive**

Aucune dégradation préventive sans signaux consolidés n'est autorisée.

**INTERD-DEG-2 : Dégradation punitive**

Aucune dégradation ne peut être appliquée comme punition. La dégradation protège, elle ne punit pas.

**INTERD-DEG-3 : Dégradation disproportionnée**

Aucune dégradation disproportionnée par rapport à la menace n'est autorisée.

**INTERD-DEG-4 : Ignorance des restrictions**

Aucun composant ne peut ignorer les restrictions de dégradation.

**INTERD-DEG-5 : Création de nouveaux niveaux de dégradation**

Aucun composant ne peut créer de nouveaux niveaux de dégradation en dehors de l'échelle T0-T4.

---

## 10. Interaction avec les autres cores

### 10.1. CaringNanny — Consolidation

**Rôle dans la dégradation :** Consolider les signaux d'intégrité qui déclenchent les dégradations.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| CaringNanny → StrongFather | Signaux consolidés pour décision de transition |
| WorrySentinel → CaringNanny | Règles de seuils de consolidation |

### 10.2. StrongFather — Décision

**Rôle dans la dégradation :** Décider des transitions d'état qui déclenchent les dégradations.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| StrongFather → Système | Décision de transition d'état |
| WorrySentinel → StrongFather | Règles de transition et sévérité |

**Impact de la dégradation sur StrongFather :**
| État | Comportement StrongFather |
|------|--------------------------|
| T0 | Décisions normales |
| T1 | Décisions normales + log renforcé |
| T2 | Seuils abaissés, plus de refus |
| T3 | Décisions critiques → AMBIGUË/DIFFÉRÉE |
| T4 | Aucune décision opérationnelle |

### 10.3. LogisticsSteward — Ressources

**Rôle dans la dégradation :** Adapter les quotas et priorités selon l'état de confiance.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| WorrySentinel → LogisticsSteward | Contraintes de quotas selon état |
| LogisticsSteward → Système | Application des quotas adaptés |

**Impact de la dégradation sur LogisticsSteward :**
| État | Comportement LogisticsSteward |
|------|------------------------------|
| T0 | Quotas normaux |
| T1 | Quotas normaux + monitoring |
| T2 | Quotas restrictifs |
| T3 | Mode survie |
| T4 | Arrêté |

### 10.4. TAMR — Intervention humaine

**Rôle dans la dégradation :** Permettre l'intervention humaine pour les états dégradés.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| WorrySentinel → TAMR | Conditions d'intervention par état |
| TAMR → StrongFather | Autorisations d'override |

**Impact de la dégradation sur TAMR :**
| État | Comportement TAMR |
|------|------------------|
| T0-T2 | Droits humains normaux |
| T3 | Validation requise pour décisions critiques |
| T4 | Mode diagnostic uniquement |

### 10.5. BorderGuard — Frontières

**Rôle dans la dégradation :** Durcir les frontières I/O selon l'état de confiance.

**Impact de la dégradation sur BorderGuard :**
| État | Comportement BorderGuard |
|------|-------------------------|
| T0 | I/O normal |
| T1 | I/O normal + surveillance |
| T2 | I/O durci |
| T3 | I/O mode défensif |
| T4 | I/O bloqué (sauf diagnostics) |

---

## 11. Règles de fermeture du contrat

### 11.1. Contrat fermé

Ce contrat est **fermé**. Seules les règles de dégradation, capacités, restrictions, invariants, et garanties explicitement définis dans ce contrat sont autorisés.

### 11.2. Interdiction d'extension implicite

Aucune extension implicite de ce contrat n'est autorisée. Les règles suivantes s'appliquent :

- **INTERD-EXT-1** : Aucune règle de dégradation non définie dans ce contrat n'est autorisée
- **INTERD-EXT-2** : Aucun niveau de dégradation non défini dans ce contrat n'est reconnu
- **INTERD-EXT-3** : Aucune capacité non définie dans ce contrat n'est garantie

### 11.3. Primauté des invariants

**Règle absolue :**

Les invariants FONDATION priment toujours sur les considérations de dégradation. Aucune règle de dégradation ne peut violer un invariant, même si elle améliore la sécurité.

---

## 12. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la dégradation progressive dans WorrySentinel.

Il garantit que :
- la dégradation est toujours progressive, jamais brutale,
- l'axiome fondamental ("observer, interpréter, dégrader, puis bloquer") est respecté,
- les règles RÈGLE-DEGRAD-1 à RÈGLE-DEGRAD-4 sont appliquées,
- l'interaction entre niveaux de sécurité et états de confiance est gouvernée,
- les capacités sont dégradées de manière proportionnelle à la menace,
- les invariants FONDATION sont préservés en tout état,
- WorrySentinel gouverne mais n'exécute jamais la dégradation.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, WorrySentinel Documentation Fondatrice, Miyukini Conceptual References - Integrity Degradation System  
**Type :** Contrat de dégradation progressive

---

## 13. Mini log de génération

### Décision éditoriale E1 : Structure par état

**Décision prise :** Chaque niveau de dégradation (T0-T4) est décrit de manière uniforme avec : situation, dégradation, capacités affectées, restrictions appliquées, impact sur les cores.

**Application :** Section 4 rédigée avec format standardisé pour les 5 états.

### Décision éditoriale E2 : Distinction T0/T1

**Décision prise :** T0 et T1 n'impliquent pas de dégradation de capacités opérationnelles. T1 est une vigilance accrue, pas une dégradation.

**Application :** Section 4.1 et 4.2 rédigées avec cette distinction explicite.

### Décision éditoriale E3 : Axiome fondamental

**Décision prise :** L'axiome "Un système autonome ne bloque jamais brutalement..." est mis en avant comme fondation conceptuelle de tout le contrat.

**Application :** Section 2 dédiée à l'axiome et ses implications.

### Ambiguïté A1 : Interaction niveaux/états

**Ambiguïté rencontrée :** Comment représenter l'interaction entre les deux dimensions (niveaux de sécurité 0-4 et états de confiance T0-T4) ?

**Décision prise :** Ajout d'une matrice d'interaction simplifiée (Section 3.4) montrant le cumul des restrictions. Les restrictions sont cumulatives : niveau élevé + état dégradé = restrictions maximales.

**Correction effectuée :** Section 3.4 avec matrice et légende explicative.

### Ambiguïté A2 : Gouvernance vs exécution

**Ambiguïté rencontrée :** Comment clarifier que WorrySentinel gouverne la dégradation mais ne l'exécute pas ?

**Décision prise :** Ajout d'un tableau explicite (Section 5.1) répartissant les responsabilités entre WorrySentinel et les autres cores.

**Correction effectuée :** Section 5 dédiée à l'orchestration avec distinction claire gouvernance/exécution.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice WorrySentinel (Section 8 dégradation)
- ✅ Cohérence avec Trust States Governance Contract (états T0-T4)
- ✅ Cohérence avec Security Levels Governance Contract (niveaux 0-4)
- ✅ Cohérence avec Integrity Degradation System
- ✅ Cohérence avec les invariants INV-WS-1 à INV-WS-8
- ✅ Séparation gouvernance / exécution respectée
- ✅ Progressivité de la dégradation garantie
- ✅ Préservation des invariants FONDATION en T4 garantie
- ✅ Interaction niveaux/états documentée

**Conclusion :** Contrat cohérent et complet, sans contradiction avec les documents existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce contrat.*
