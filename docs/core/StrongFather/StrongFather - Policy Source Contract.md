# StrongFather — Policy Source Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **StrongFather — Policy Source Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit l'unique origine valide des politiques de StrongFather, leur cycle de vie pré-application, et les règles absolues d'alimentation du moteur de politiques dans le système Miyukini Core System v2.4.

Ce contrat ferme la lacune contractuelle identifiée concernant l'origine et la gestion des politiques avant leur application par le Policy Engine.

### Portée

Ce contrat s'applique à **toutes les politiques utilisées par StrongFather** et définit de manière absolue :
- la définition formelle d'une source de politiques,
- les types de sources autorisées,
- le cycle de vie des politiques pré-application,
- les règles de chargement et validation,
- les interdictions d'injection dynamique,
- les invariants de source.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **StrongFather — Policy Engine Contract** : Définit la structure et l'application des politiques (document maître pour la structure des politiques)
- **StrongFather — Boundary & Isolation Contract** : Autorise la lecture depuis une source de politiques configurée
- **StrongFather — Invariants & Guarantees** : INV-POL-SOURCE est défini dans ce contrat
- **[Miyukini Framework - Lois Autonomie Systeme](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité aux lois d'autonomie, notamment **LOI-1** (aucune dépendance externe critique) : la source de politiques est locale et configurée, jamais découverte dynamiquement

Il n'introduit aucune contradiction, et constitue la définition formelle de l'origine et du cycle de vie des politiques.

---

## 2. Définition d'une source de politiques

### 2.1. Nature d'une source

Une **source de politiques** est l'unique origine autorisée d'où StrongFather peut obtenir les politiques qu'il applique. Une source est un concept systémique qui représente un réservoir de politiques validées, sans présupposer de technologie particulière.

**Caractéristiques d'une source :**

- **Unique** : Il existe une et une seule source de politiques par instance de StrongFather
- **Configurée** : La source est explicitement configurée, jamais découverte dynamiquement
- **Validée** : Les politiques de la source sont validées avant utilisation
- **Immuable pendant évaluation** : La source ne change pas pendant une évaluation

### 2.2. Ce qu'une source représente

Une source de politiques représente :

1. **Un réservoir de politiques** : L'ensemble des politiques disponibles pour évaluation
2. **Un point de configuration** : Le point unique où les politiques sont définies
3. **Une garantie de cohérence** : L'assurance que les politiques sont cohérentes entre elles
4. **Un périmètre fermé** : L'ensemble exhaustif des politiques applicables

### 2.3. Ce qu'une source ne représente jamais

Une source de politiques ne représente **jamais** :

1. **Un générateur de politiques** : Une source ne génère pas de politiques dynamiquement
2. **Un point d'injection** : Une source n'accepte pas de politiques injectées à l'exécution
3. **Un canal de communication** : Une source n'est pas un canal de communication externe
4. **Un système externe actif** : Une source n'initie jamais de communication vers StrongFather

---

## 3. Types de sources autorisées

### 3.1. Source déclarative statique

**Définition :**

Une **source déclarative statique** est une source dont les politiques sont définies de manière déclarative et ne changent pas pendant l'exécution du système.

**Caractéristiques :**

- **Déclarative** : Les politiques sont déclarées, pas générées
- **Statique** : Les politiques ne changent pas sans rechargement explicite
- **Versionnable** : Les politiques peuvent être versionnées
- **Auditée** : Les politiques sont auditées avant déploiement

**Exemples conceptuels :**

- Configuration déclarative chargée au démarrage
- Ensemble de règles définies par l'équipe produit
- Politiques versionnées et déployées avec l'application

### 3.2. Source déclarative rechargeable

**Définition :**

Une **source déclarative rechargeable** est une source déclarative qui peut être rechargée explicitement, permettant une mise à jour des politiques sans redémarrage.

**Caractéristiques :**

- Mêmes caractéristiques que la source statique
- **Rechargeable** : Peut être rechargée sur demande explicite
- **Atomique** : Le rechargement est atomique (tout ou rien)
- **Non-disruptif** : Les évaluations en cours ne sont pas affectées

**Règles de rechargement :**

- **R-RELOAD-1** : Le rechargement est déclenché explicitement, jamais automatiquement
- **R-RELOAD-2** : Le rechargement est atomique : la nouvelle version remplace entièrement l'ancienne
- **R-RELOAD-3** : Les évaluations en cours utilisent les politiques chargées au début de l'évaluation
- **R-RELOAD-4** : Un échec de rechargement n'affecte pas les politiques en cours d'utilisation

### 3.3. Sources explicitement interdites

Les types de sources suivants sont **explicitement interdits** :

**INTERD-SRC-1 : Source générative**

Aucune source ne peut générer des politiques dynamiquement ou algorithmiquement.

**INTERD-SRC-2 : Source externe distante**

Aucune source ne peut être un service externe distant nécessitant une communication réseau à chaque évaluation.

**INTERD-SRC-3 : Source par injection**

Aucune politique ne peut être injectée dans StrongFather par un appelant ou un adaptateur.

**INTERD-SRC-4 : Source par dérivation**

Aucune politique ne peut être dérivée ou calculée à partir des données d'une intention.

**INTERD-SRC-5 : Source par apprentissage**

Aucune politique ne peut être générée ou modifiée par un système d'apprentissage automatique.

---

## 4. Cycle de vie des politiques pré-application

### 4.1. Phases du cycle de vie

Le cycle de vie d'une politique avant son application comprend les phases suivantes :

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CYCLE DE VIE PRÉ-APPLICATION                          │
│                                                                         │
│   [DÉFINITION] → [VALIDATION] → [CHARGEMENT] → [ACTIVATION]            │
│                                                                         │
│   Hors StrongFather        │        Dans StrongFather                   │
│   ─────────────────────────┼────────────────────────────────────        │
│   Définition, Validation   │   Chargement, Activation                   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2. Phase de définition

**Objectif :** Créer les politiques de manière déclarative.

**Responsable :** Équipe produit ou configuration (hors StrongFather)

**Règles :**

- **R-DEF-1** : Les politiques sont définies de manière déclarative
- **R-DEF-2** : Les politiques respectent la structure définie dans Policy Engine Contract
- **R-DEF-3** : Les politiques sont documentées avec leur justification

**Sortie :** Ensemble de politiques définies

### 4.3. Phase de validation

**Objectif :** Vérifier la validité des politiques avant chargement.

**Responsable :** Processus de validation (hors StrongFather)

**Validations obligatoires :**

1. **Validation structurelle** : Chaque politique possède les composants obligatoires (identifiant, type, condition, règle, effet)
2. **Validation de cohérence** : Les politiques ne contiennent pas de contradictions internes
3. **Validation de complétude** : L'ensemble des politiques couvre les cas prévus
4. **Validation de fermeture** : Les politiques ne référencent pas d'éléments non définis

**Règles :**

- **R-VAL-1** : Aucune politique invalide ne peut être chargée
- **R-VAL-2** : La validation est effectuée avant le chargement, pas pendant
- **R-VAL-3** : Un échec de validation bloque le chargement

**Sortie :** Ensemble de politiques validées

### 4.4. Phase de chargement

**Objectif :** Charger les politiques validées dans StrongFather.

**Responsable :** StrongFather

**Règles :**

- **R-LOAD-1** : Seules les politiques validées peuvent être chargées
- **R-LOAD-2** : Le chargement est atomique (tout ou rien)
- **R-LOAD-3** : Un échec de chargement préserve les politiques précédentes
- **R-LOAD-4** : Le chargement est tracé pour audit

**Sortie :** Politiques chargées dans StrongFather

### 4.5. Phase d'activation

**Objectif :** Rendre les politiques disponibles pour évaluation.

**Responsable :** StrongFather

**Règles :**

- **R-ACT-1** : L'activation rend les politiques disponibles pour les nouvelles évaluations
- **R-ACT-2** : Les évaluations en cours ne sont pas affectées par l'activation
- **R-ACT-3** : L'activation est instantanée une fois le chargement terminé

**Sortie :** Politiques actives et utilisables

---

## 5. Règles de chargement

### 5.1. Chargement initial

**R-INIT-1 : Chargement obligatoire**

StrongFather DOIT charger ses politiques depuis la source configurée avant toute évaluation.

**R-INIT-2 : Échec bloquant**

Si le chargement initial échoue, StrongFather NE PEUT PAS effectuer d'évaluations.

**R-INIT-3 : Source unique**

Le chargement initial provient de l'unique source configurée.

### 5.2. Rechargement

**R-RECHG-1 : Rechargement explicite**

Le rechargement est toujours explicitement déclenché, jamais automatique.

**R-RECHG-2 : Atomicité**

Le rechargement est atomique : succès total ou échec total.

**R-RECHG-3 : Isolation des évaluations**

Les évaluations en cours ne sont jamais affectées par un rechargement.

**R-RECHG-4 : Rollback automatique**

En cas d'échec de rechargement, les politiques précédentes restent actives.

### 5.3. Traçabilité du chargement

**R-TRACE-LOAD-1 : Trace obligatoire**

Tout chargement ou rechargement est tracé avec :
- Horodatage du chargement
- Identifiant de version des politiques
- Nombre de politiques chargées
- Résultat (succès/échec)

**R-TRACE-LOAD-2 : Trace d'activation**

Toute activation est tracée avec :
- Horodatage d'activation
- Politiques actives (identifiants)

---

## 6. Règles de validation

### 6.1. Validation structurelle

Chaque politique DOIT être validée structurellement :

**VALID-STRUCT-1 : Identifiant unique**

Chaque politique possède un identifiant unique dans l'ensemble des politiques.

**VALID-STRUCT-2 : Type valide**

Le type de chaque politique est l'un des types autorisés (permission, contrainte, priorité, validation, composite).

**VALID-STRUCT-3 : Composants obligatoires**

Chaque politique possède tous les composants obligatoires définis dans Policy Engine Contract.

**VALID-STRUCT-4 : Effet explicite**

Chaque politique possède un effet explicitement défini.

### 6.2. Validation de cohérence

L'ensemble des politiques DOIT être validé pour la cohérence :

**VALID-COHER-1 : Pas de contradiction directe**

Deux politiques ne peuvent pas être en contradiction directe non résoluble.

**VALID-COHER-2 : Références valides**

Toute référence à une autre politique pointe vers une politique existante.

**VALID-COHER-3 : Pas de cycle dans les composites**

Les politiques composites ne forment pas de cycles de référence.

### 6.3. Validation de contenu

Le contenu de chaque politique DOIT être validé :

**VALID-CONT-1 : Pas de logique d'exécution**

Aucune politique ne contient de logique d'exécution.

**VALID-CONT-2 : Pas de logique métier spécifique**

Aucune politique ne contient de logique métier spécifique à un domaine produit.

**VALID-CONT-3 : Pas de logique temporelle technique**

Aucune politique ne contient de logique temporelle technique (horodatages, timestamps).

---

## 7. Interdictions d'injection

### 7.1. Principe d'interdiction

**Aucune politique ne peut être injectée dans StrongFather en dehors du cycle de vie défini.**

Ce principe est absolu et sans exception.

### 7.2. Cas d'injection interdits

**INTERD-INJ-1 : Injection par intention**

Aucune intention ne peut contenir ou référencer une politique à appliquer.

**INTERD-INJ-2 : Injection par adaptateur**

Aucun adaptateur ne peut fournir des politiques à appliquer lors d'une soumission.

**INTERD-INJ-3 : Injection par contexte**

Aucun contexte d'appel ne peut contenir des politiques supplémentaires.

**INTERD-INJ-4 : Injection par métadonnées**

Aucune métadonnée ne peut être interprétée comme une politique.

**INTERD-INJ-5 : Injection par modification**

Aucune modification des politiques chargées n'est possible pendant l'exécution.

### 7.3. Conséquences de tentative d'injection

**CONSEQ-INJ-1 : Rejet de l'intention**

Toute tentative d'injection détectée entraîne le rejet de l'intention associée.

**CONSEQ-INJ-2 : Violation contractuelle**

Toute tentative d'injection constitue une violation critique de ce contrat.

**CONSEQ-INJ-3 : Traçabilité**

Toute tentative d'injection est tracée comme incident de sécurité.

---

## 8. Invariants de source

### 8.1. Invariants fondamentaux

**INV-POL-SOURCE : Source unique et configurée**

Les politiques de StrongFather proviennent exclusivement d'une source unique, explicitement configurée, et validée. Aucune politique ne peut être injectée, générée, ou dérivée dynamiquement.

*Cet invariant est référencé dans le document Invariants & Guarantees.*

**INV-SRC-1 : Unicité de la source**

Il existe exactement une source de politiques par instance de StrongFather.

**INV-SRC-2 : Configuration explicite**

La source est toujours explicitement configurée, jamais découverte ou déduite.

**INV-SRC-3 : Validation préalable**

Aucune politique n'est utilisée sans validation préalable.

**INV-SRC-4 : Immuabilité pendant évaluation**

Les politiques ne changent jamais pendant une évaluation en cours.

### 8.2. Invariants de chargement

**INV-SRC-5 : Chargement atomique**

Le chargement est toujours atomique : succès total ou échec total.

**INV-SRC-6 : Isolation des évaluations**

Une évaluation utilise toujours l'ensemble de politiques actif au début de l'évaluation.

### 8.3. Invariants d'interdiction

**INV-SRC-7 : Pas d'injection**

Aucune politique n'est jamais injectée en dehors du cycle de vie défini.

**INV-SRC-8 : Pas de génération**

Aucune politique n'est jamais générée dynamiquement ou algorithmiquement.

---

## 9. Garanties offertes

### 9.1. Garanties de stabilité

**G-SRC-1 : Stabilité des politiques**

Les politiques actives sont stables entre les rechargements explicites.

**G-SRC-2 : Prévisibilité**

L'ensemble des politiques applicables est toujours prévisible et auditable.

### 9.2. Garanties de sécurité

**G-SRC-3 : Pas de politique malveillante injectée**

Aucune politique malveillante ne peut être injectée via les intentions ou le contexte.

**G-SRC-4 : Traçabilité complète**

L'origine et le cycle de vie de chaque politique sont traçables.

### 9.3. Garanties de cohérence

**G-SRC-5 : Cohérence garantie**

Les politiques actives sont toujours cohérentes entre elles (validées avant activation).

**G-SRC-6 : Complétude garantie**

L'ensemble des politiques actives est complet et fermé.

---

## 10. Règles de fermeture du contrat

### 10.1. Contrat fermé

Ce contrat est **fermé**. Seuls les types de sources, les phases du cycle de vie, les règles, et les invariants explicitement définis dans ce contrat sont autorisés.

### 10.2. Interdiction d'extension implicite

Aucune extension implicite n'est autorisée :

- **INTERD-EXT-SRC-1** : Aucun type de source non défini n'est reconnu
- **INTERD-EXT-SRC-2** : Aucune phase de cycle de vie non définie n'est autorisée
- **INTERD-EXT-SRC-3** : Aucune règle de chargement non définie n'est applicable
- **INTERD-EXT-SRC-4** : Aucun mécanisme d'injection n'est autorisé

---

## 11. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable l'origine et le cycle de vie des politiques de StrongFather.

Il garantit que :
- les politiques proviennent d'une source unique et configurée,
- les politiques suivent un cycle de vie défini,
- les politiques sont validées avant utilisation,
- aucune injection de politique n'est possible,
- les invariants de source sont respectés,
- le contrat est fermé et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

## 12. Validation conceptuelle

### 12.1. Cas conformes

Les cas suivants sont **conformes** à ce contrat :

1. **Chargement initial** : StrongFather charge ses politiques depuis une source configurée au démarrage.

2. **Rechargement explicite** : Un administrateur déclenche un rechargement des politiques, les nouvelles politiques sont validées puis activées.

3. **Évaluation isolée** : Une évaluation en cours utilise les politiques actives au début, non affectée par un rechargement concurrent.

### 12.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Injection par intention** : Une intention contient une politique à appliquer. Viole INTERD-INJ-1.

2. **Source multiple** : StrongFather utilise des politiques provenant de plusieurs sources. Viole INV-SRC-1.

3. **Politique générée** : Une politique est générée algorithmiquement à partir du contexte. Viole INV-SRC-8 et INTERD-SRC-4.

4. **Politique non validée** : Une politique est utilisée sans validation préalable. Viole INV-SRC-3.

5. **Chargement depuis service externe** : Les politiques sont récupérées depuis une API externe à chaque évaluation. Viole INTERD-SRC-2.

---

**Document créé le :** 2026-01-25  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, StrongFather Documentation Fondatrice  
**Type :** Contrat de source de politiques non négociable

---

## 13. Mini log de génération

### Contexte de création

**Origine :** Ce contrat a été créé suite à l'audit global de StrongFather qui a identifié une lacune contractuelle (C.5) concernant l'absence de définition de la source des politiques.

**Objectif :** Fermer la lacune C.5 et réduire les risques D.1, D.4, D.5 identifiés dans l'audit.

### Décisions prises

**E1 : Types de sources**

Décision prise : Deux types de sources autorisées (statique et rechargeable), liste fermée de sources interdites.

Application : Sections 3.1, 3.2 définissent les sources autorisées, section 3.3 liste les interdictions.

**E2 : Cycle de vie en 4 phases**

Décision prise : Cycle de vie en 4 phases (Définition, Validation, Chargement, Activation) avec responsabilités claires.

Application : Section 4 définit le cycle de vie complet.

**E3 : Interdictions d'injection exhaustives**

Décision prise : Liste exhaustive des cas d'injection interdits avec conséquences.

Application : Section 7 définit les interdictions et leurs conséquences.

**E4 : Invariant INV-POL-SOURCE**

Décision prise : Définition de l'invariant INV-POL-SOURCE demandé par l'audit.

Application : Section 8.1 définit l'invariant qui sera référencé dans Invariants & Guarantees.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Policy Engine Contract : Confirmée (structure des politiques référencée)
- ✅ Cohérence avec Boundary & Isolation Contract : Confirmée (source de politiques autorisée)
- ✅ Cohérence avec Execution Prohibition Contract : Confirmée (pas de logique d'exécution dans les politiques)
- ✅ Aucune contradiction avec les contrats existants

**Conclusion :** Document créé conformément aux décisions de l'audit, aucune contradiction détectée.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
