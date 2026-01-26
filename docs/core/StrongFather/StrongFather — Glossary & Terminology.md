# StrongFather — Glossary & Terminology

## 1. Introduction

### Objet du document

Ce document définit le **StrongFather — Glossary & Terminology** : un document de référence consolidé qui établit les définitions normalisées de tous les termes utilisés dans l'écosystème StrongFather, leurs références croisées, et identifie les termes interdits ou ambigus dans le système Miyukini Core System v2.4.

Ce document constitue la référence unique pour la terminologie StrongFather et garantit la cohérence sémantique à travers tous les contrats FONDATION.

### Portée

Ce glossaire s'applique à **toute la documentation et les contrats StrongFather** et définit de manière normalisée :
- les définitions formelles de tous les termes techniques,
- les références croisées entre termes,
- les termes interdits ou ambigus,
- les distinctions conceptuelles critiques.

### Statut

Ce document est **informatif et de référence**. Il ne crée pas de nouvelles règles contractuelles mais consolide et normalise la terminologie utilisée dans les contrats FONDATION.

### Relation avec les autres contrats

Ce glossaire consolide les termes définis dans :
- **StrongFather — Documentation Fondatrice** : Termes fondamentaux
- **StrongFather — Core Decision Contract** : Termes décisionnels
- **StrongFather — Intent Model Contract** : Termes d'intention
- **StrongFather — Policy Engine Contract** : Termes de politique
- **StrongFather — Invariants & Guarantees** : Termes d'invariants
- **StrongFather — Architecture & Flows** : Termes architecturaux
- **StrongFather — Error & Rejection Model** : Termes d'erreur
- **StrongFather — Audit & Trace Contract** : Termes de traçabilité
- **StrongFather — Execution Prohibition Contract** : Termes d'interdiction
- **StrongFather — Boundary & Isolation Contract** : Termes de frontière
- Tous les autres contrats StrongFather
- **[Miyukini Framework - Lois Autonomie Systeme](docs/reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md)** : Termes liés aux lois d'autonomie système

---

## 2. Termes fondamentaux

### 2.1. StrongFather

**Définition :**

StrongFather est le **moteur de décision stratégique et politique** du Miyukini Core System. Il incarne la capacité conceptuelle du système à évaluer des intentions, à appliquer des politiques, à établir des priorités, et à produire des décisions sans jamais posséder d'autorité sur l'exécution ou la persistance.

**Caractéristiques :**
- Moteur d'évaluation et de décision
- Centralise l'évaluation des intentions selon des politiques cohérentes
- Établit des priorités de manière globale
- Détecte les ambiguïtés avant exécution
- Aucune autorité sur l'exécution ou la persistance

**Références :**
- Documentation Fondatrice (section 2)
- Architecture & Flows (section 2.2)

**Termes associés :** Intention, Décision, Politique, Évaluation, Moteur de décision

---

### 2.2. Intention

**Définition :**

Une **intention** est une demande d'action soumise à StrongFather pour évaluation. Une intention contient l'action demandée, les données associées, le contexte (utilisateur, produit, instance), et les métadonnées.

**Caractéristiques :**
- Identifiant unique et immutable (INV-INT-1, INV-ID-GLOBAL)
- Type d'action (CRÉATION, MODIFICATION, SUPPRESSION, LECTURE, ÉVALUATION)
- Sujet de l'intention
- Contexte d'appel obligatoire
- Composants optionnels (priorité, contraintes, métadonnées)

**Cycle de vie :**
- SOUMISE → EN_ÉVALUATION → DÉCIDÉE

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Intent Model Contract (sections 2, 3, 4)

**Termes associés :** Décision, Évaluation, Contexte, Politique, Action

**Distinctions :**
- ❌ Une intention n'est **pas** une commande d'exécution
- ❌ Une intention n'est **jamais** exécutée par StrongFather (INV-INT-2)
- ✅ Une intention est uniquement évaluée

---

### 2.3. Décision

**Définition :**

Une **décision** est le résultat produit par StrongFather après évaluation d'une intention selon des politiques et des contraintes. Une décision est toujours non ambiguë et prend l'une des valeurs suivantes : ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE.

**Caractéristiques :**
- Toujours non ambiguë (INV-DEC-1)
- Justifiée selon les politiques appliquées (INV-DEC-2, G-JUST-1)
- Unique pour chaque intention (INV-DEC-3)
- Contient l'identifiant de l'intention évaluée
- Contient les politiques appliquées
- Contient la justification

**Types de décisions :**
- **ACCEPTÉE** : L'intention est valide selon les politiques et peut être exécutée
- **REFUSÉE** : L'intention est invalide selon les politiques et ne doit pas être exécutée
- **AMBIGUË** : L'intention est insuffisamment définie et nécessite des clarifications
- **DIFFÉRÉE** : L'intention nécessite un contexte futur (sans planification, INV-DIFF-NOPLAN)

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Core Decision Contract (sections 2, 3, 4)
- Invariants & Guarantees (INV-DEC-*)

**Termes associés :** Intention, Évaluation, Politique, Justification, Refus, Ambiguïté

**Distinctions :**
- ❌ Une décision n'est **jamais** exécutable directement (G-NOEXEC-1, INV-EXEC-1)
- ❌ Une décision n'entraîne **jamais** d'exécution automatique (INV-AUTH-1)
- ✅ Une décision est une structure de données immuable

---

### 2.4. Politique

**Définition :**

Une **politique** est une règle déclarative qui détermine la validité d'une intention. Une politique est explicite, déclarative, centralisée, et versionnée.

**Caractéristiques :**
- Explicite et déclarative (INV-POL-1)
- Centralisée (définie une fois, appliquée de manière cohérente)
- Versionnée (peut évoluer dans le temps avec traçabilité)
- Immutable pendant évaluation (INV-POL-2)
- Source unique et configurée (INV-POL-SOURCE)

**Types de politiques :**
- **PERMISSION** : Détermine si un acteur est autorisé à effectuer une action
- **CONSTRAINT** : Définit des conditions qui doivent être satisfaites
- **PRIORITY** : Détermine l'ordre d'importance relative
- **VALIDATION** : Valide la cohérence ou la conformité
- **COMPOSITE** : Combine plusieurs politiques avec opérateurs logiques

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Policy Engine Contract (sections 2, 3, 4)
- Policy Language Specification (section 2)
- Policy Source Contract (INV-POL-SOURCE)

**Termes associés :** Intention, Décision, Évaluation, Moteur de politiques, Source de politiques

**Distinctions :**
- ❌ Une politique ne contient **jamais** de logique d'exécution
- ❌ Une politique ne contient **jamais** de logique métier spécifique
- ✅ Une politique est purement déclarative

---

### 2.5. Évaluation

**Définition :**

L'**évaluation** est le processus par lequel StrongFather applique des politiques sur une intention pour produire une décision.

**Caractéristiques :**
- Déterministe (INV-POL-3, INV-POL-6)
- Complète (toutes les politiques applicables sont évaluées)
- Ordonnée (selon la priorité des politiques)
- Traçable (INV-TRACE-1)

**Références :**
- Policy Engine Contract (section 5)
- Architecture & Flows (section 4)
- Invariants & Guarantees (INV-POL-3, INV-POL-6)

**Termes associés :** Intention, Décision, Politique, Moteur de politiques, Résultat d'évaluation

**Distinctions :**
- ❌ L'évaluation n'est **jamais** une exécution (INV-INT-2)
- ❌ L'évaluation ne modifie **jamais** l'état du système (INV-EXEC-2)
- ✅ L'évaluation est purement fonctionnelle (INV-EXEC-5)

---

### 2.6. Priorité

**Définition :**

Une **priorité** est l'ordre d'importance relatif d'une intention par rapport à d'autres intentions. Une priorité est relative, globale, et dynamique.

**Caractéristiques :**
- Relative (déterminée par comparaison avec d'autres intentions)
- Globale (établie de manière cohérente à travers le système)
- Dynamique (peut changer selon le contexte et les politiques)
- Non temporelle (pas d'ordonnancement technique)

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Policy Engine Contract (section 3.3)
- Architecture & Flows (section 3.5)

**Termes associés :** Intention, Décision, Politique, Calculateur de priorité

**Distinctions :**
- ❌ Une priorité n'est **jamais** un ordonnancement temporel (INV-AUTH-3)
- ❌ Une priorité n'est **jamais** une planification (INTERD-TIME-2)
- ✅ Une priorité est un ordre d'importance relatif

---

### 2.7. Contrainte

**Définition :**

Une **contrainte** est une condition qui doit être satisfaite pour qu'une intention soit acceptée. Une contrainte est déclarative, évaluable, et non technique.

**Caractéristiques :**
- Déclarative (exprime une condition, pas une vérification technique)
- Évaluable (peut être évaluée par StrongFather sans exécution)
- Non technique (ne porte pas sur des aspects techniques)

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Policy Engine Contract (section 3.2)

**Termes associés :** Intention, Politique, Évaluation, Validation

**Distinctions :**
- ❌ Une contrainte n'est **jamais** une validation technique de données
- ❌ Une contrainte n'est **jamais** une vérification de schéma
- ✅ Une contrainte est conceptuelle et déclarative

---

## 3. Termes d'architecture

### 3.1. Surface d'évaluation

**Définition :**

La **surface d'évaluation** est le point d'entrée unique de StrongFather. Elle reçoit les intentions et retourne les décisions.

**Caractéristiques :**
- Point d'entrée unique (INV-ARCH-1)
- Interface conceptuelle standardisée
- Pas de logique métier

**Références :**
- Architecture & Flows (section 3.1)

**Termes associés :** Intention, Décision, Validateur d'intention, Flux d'évaluation

---

### 3.2. Validateur d'intention

**Définition :**

Le **validateur d'intention** vérifie la validité structurelle des intentions avant l'évaluation des politiques.

**Caractéristiques :**
- Vérifie la présence des composants obligatoires
- Vérifie la cohérence structurelle
- Rejette les intentions structurellement invalides

**Références :**
- Architecture & Flows (section 3.2)
- Intent Model Contract (section 6)

**Termes associés :** Intention, Validation, Rejet structurel, Surface d'évaluation

---

### 3.3. Moteur de politiques

**Définition :**

Le **moteur de politiques** applique les politiques sur les intentions et produit les résultats d'évaluation.

**Caractéristiques :**
- Sélectionne les politiques applicables
- Évalue chaque politique
- Produit les résultats d'évaluation
- Déterministe (INV-POL-3)

**Références :**
- Architecture & Flows (section 3.3)
- Policy Engine Contract (sections 5, 7)

**Termes associés :** Politique, Évaluation, Résultat d'évaluation, Compositeur de résultats

---

### 3.4. Compositeur de résultats

**Définition :**

Le **compositeur de résultats** agrège les résultats des évaluations de politiques selon les règles de composition.

**Caractéristiques :**
- Agrège les résultats des politiques
- Applique les règles de composition
- Détermine le résultat global

**Références :**
- Architecture & Flows (section 3.4)
- Policy Engine Contract (section 6)

**Termes associés :** Résultat d'évaluation, Politique, Moteur de politiques, Producteur de décision

---

### 3.5. Calculateur de priorité

**Définition :**

Le **calculateur de priorité** établit la priorité relative d'une intention si les politiques sont satisfaites.

**Caractéristiques :**
- Applique les politiques de priorité
- Calcule la priorité relative
- Fournit la priorité à la décision
- Activé uniquement si toutes les politiques sont satisfaites

**Références :**
- Architecture & Flows (section 3.5)

**Termes associés :** Priorité, Décision, Politique, Compositeur de résultats

---

### 3.6. Producteur de décision

**Définition :**

Le **producteur de décision** génère la décision finale à partir des résultats d'évaluation.

**Caractéristiques :**
- Produit la décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
- Assemble la justification
- Référence les politiques appliquées
- Point de sortie unique (INV-ARCH-2)

**Références :**
- Architecture & Flows (section 3.6)
- Core Decision Contract (sections 3, 4, 5)

**Termes associés :** Décision, Justification, Résultat d'évaluation, Compositeur de résultats

---

### 3.7. Traceur

**Définition :**

Le **traceur** enregistre les traces d'évaluation pour audit et diagnostic.

**Caractéristiques :**
- Trace les intentions reçues
- Trace les évaluations de politiques
- Trace les décisions produites
- Trace les erreurs
- Fonctionne en parallèle sans affecter le flux principal (INV-ARCH-6)

**Références :**
- Architecture & Flows (section 3.7)
- Audit & Trace Contract (sections 2, 3, 4)

**Termes associés :** Traçabilité, Trace, Audit, Intention, Décision

---

## 4. Termes d'invariants et garanties

### 4.1. Invariant

**Définition :**

Un **invariant** est une propriété qui doit toujours être vraie dans StrongFather, quelle que soit la situation, le contexte, ou l'état du système.

**Caractéristiques :**
- Absolu (toujours vrai, sans exception)
- Non négociable (ne peut pas être temporairement suspendu)
- Vérifiable (peut être vérifié conceptuellement)
- Fondamental (représente une propriété fondamentale du système)

**Références :**
- Invariants & Guarantees (section 2.1, section 3)

**Termes associés :** Garantie, Propriété, Contrat, Violation

**Distinctions :**
- ❌ Un invariant n'est **jamais** conditionnel
- ❌ Un invariant ne peut **jamais** être violé
- ✅ Un invariant est absolu et permanent

---

### 4.2. Garantie

**Définition :**

Une **garantie** est un engagement pris par StrongFather envers les appelants, définissant ce qu'ils peuvent attendre du système.

**Caractéristiques :**
- Contractuelle (engagement contractuel)
- Conditionnelle (s'applique si les conditions sont respectées)
- Observable (produit un effet observable)
- Bénéficiaire (bénéficie à l'appelant)

**Références :**
- Invariants & Guarantees (section 2.2, section 4)

**Termes associés :** Invariant, Engagement, Contrat, Appelant

**Distinctions :**
- ❌ Une garantie n'est **jamais** absolue (contrairement à un invariant)
- ❌ Une garantie peut **ne pas** s'appliquer si les conditions ne sont pas respectées
- ✅ Une garantie est un engagement conditionnel

---

### 4.3. Pureté fonctionnelle

**Définition :**

La **pureté fonctionnelle** est la propriété selon laquelle StrongFather se comporte comme une fonction pure : pour une entrée donnée, il produit une sortie sans effet de bord.

**Caractéristiques :**
- Aucun effet de bord (INV-EXEC-5, INV-BEHAV-3)
- Aucune mutation d'état (INV-EXEC-2)
- Déterminisme (INV-POL-3)
- Transparence référentielle (INV-EXEC-6)

**Références :**
- Invariants & Guarantees (INV-EXEC-5, INV-BEHAV-3, INV-EXEC-6)
- Execution Prohibition Contract (INV-EXEC-5, INV-EXEC-6)

**Termes associés :** Fonction pure, Effet de bord, Mutation d'état, Déterminisme

**Distinctions :**
- ❌ La pureté fonctionnelle n'autorise **jamais** d'effet de bord
- ❌ La pureté fonctionnelle n'autorise **jamais** de mutation d'état
- ✅ La pureté fonctionnelle garantit l'isolation totale

---

### 4.4. Zero-trust

**Définition :**

Le **zero-trust** est le principe selon lequel StrongFather ne fait confiance à aucun appelant. Toute intention est évaluée selon les politiques, sans présupposer la validité, l'authenticité, ou la légitimité de l'appelant.

**Caractéristiques :**
- Aucune confiance présupposée (INV-BEHAV-2)
- Validation systématique
- Vérification selon politiques

**Références :**
- Documentation Fondatrice (INV-SF-5)
- Invariants & Guarantees (INV-BEHAV-2)
- Core Decision Contract (G-ZT-*)

**Termes associés :** Validation, Politique, Intention, Appelant, Sécurité

**Distinctions :**
- ❌ Le zero-trust n'autorise **jamais** de whitelist d'appelants "de confiance"
- ❌ Le zero-trust n'autorise **jamais** de bypass de validation
- ✅ Le zero-trust garantit la validation systématique

---

## 5. Termes d'erreur et rejet

### 5.1. Erreur

**Définition :**

Une **erreur** est un dysfonctionnement interne qui empêche l'évaluation. Une erreur n'est jamais un résultat normal d'évaluation.

**Caractéristiques :**
- Dysfonctionnement interne
- Empêche l'évaluation
- Retourne `Err(SFError)`
- Traçable dans les logs d'erreur

**Catégories d'erreurs :**
- **Erreur structurelle** : Violation de structure interne
- **Erreur de cohérence** : Violation d'invariant
- **Erreur de ressource** : Problème de ressource

**Références :**
- Error & Rejection Model (sections 2, 3, 4)
- Invariants & Guarantees (INV-ERR-1, INV-ERR-2)

**Termes associés :** Rejet, Dysfonctionnement, SFError, Traçabilité

**Distinctions :**
- ❌ Une erreur n'est **jamais** un rejet (INV-ERR-1)
- ❌ Une erreur n'est **jamais** un résultat normal
- ✅ Une erreur est un dysfonctionnement interne

---

### 5.2. Rejet

**Définition :**

Un **rejet** est une décision indiquant qu'une intention est invalide selon les politiques et ne doit pas être exécutée. Un rejet est un résultat normal d'évaluation.

**Caractéristiques :**
- Résultat normal d'évaluation
- Décision REFUSÉE
- Retourne `Ok(Decision)` avec `DecisionType::Refused`
- Contient la raison du refus
- Contient les politiques violées

**Types de rejets :**
- **Rejet structurel** : Intention structurellement invalide
- **Rejet de politique** : Politique violée
- **Rejet de contrainte** : Contrainte non satisfaite

**Références :**
- Error & Rejection Model (sections 2, 3, 4)
- Core Decision Contract (section 3.2)
- Invariants & Guarantees (INV-ERR-1)

**Termes associés :** Décision, Erreur, Refus, Politique, Intention

**Distinctions :**
- ❌ Un rejet n'est **jamais** une erreur (INV-ERR-1)
- ❌ Un rejet n'est **jamais** un dysfonctionnement
- ✅ Un rejet est un résultat normal d'évaluation

---

### 5.3. Ambiguïté

**Définition :**

Une **ambiguïté** est une décision indiquant qu'une intention est insuffisamment définie pour être évaluée. Une ambiguïté n'est pas un refus mais une demande de clarification.

**Caractéristiques :**
- Décision AMBIGUË
- Intention insuffisamment définie
- Clarifications nécessaires identifiées
- Peut être réévaluée après clarification

**Références :**
- Documentation Fondatrice (section 10, glossaire)
- Core Decision Contract (section 3.3)
- Error & Rejection Model (section 3.3)

**Termes associés :** Décision, Intention, Clarification, Évaluation

**Distinctions :**
- ❌ Une ambiguïté n'est **jamais** un refus
- ❌ Une ambiguïté n'est **jamais** une erreur
- ✅ Une ambiguïté est une demande de clarification

---

## 6. Termes de traçabilité

### 6.1. Traçabilité

**Définition :**

La **traçabilité** dans StrongFather est la capacité de suivre et de documenter toutes les évaluations effectuées, les décisions produites, et les politiques appliquées, permettant une reconstruction complète du processus décisionnel.

**Caractéristiques :**
- Complète (toute évaluation est tracée, INV-TRACE-1)
- Non-intrusive (ne modifie pas le comportement, INV-TRACE-2)
- Auditée (permet l'audit a posteriori)
- Immuable (les traces ne sont jamais modifiées, INV-TRACE-4)

**Références :**
- Audit & Trace Contract (sections 2, 3, 4)
- Invariants & Guarantees (INV-TRACE-1, INV-TRACE-2, INV-TRACE-4)
- Documentation Fondatrice (INV-SF-8)

**Termes associés :** Trace, Audit, Traceur, Intention, Décision

**Distinctions :**
- ❌ La traçabilité n'est **jamais** de la persistance opérationnelle
- ❌ La traçabilité n'affecte **jamais** le comportement (INV-TRACE-2)
- ✅ La traçabilité est passive et observationnelle

---

### 6.2. Trace

**Définition :**

Une **trace** est un enregistrement immuable d'un événement d'évaluation (intention, évaluation, décision, erreur) permettant l'audit et le diagnostic.

**Types de traces :**
- **Trace d'intention** : Enregistrement d'une intention soumise
- **Trace d'évaluation** : Enregistrement d'une évaluation de politique
- **Trace de décision** : Enregistrement d'une décision produite
- **Trace d'erreur** : Enregistrement d'une erreur rencontrée

**Caractéristiques :**
- Immuable après production (INV-TRACE-4)
- Auto-suffisante pour l'audit
- Contient tous les éléments obligatoires (INV-TRACE-5)

**Références :**
- Audit & Trace Contract (sections 3, 4)
- Invariants & Guarantees (INV-TRACE-4, INV-TRACE-5)

**Termes associés :** Traçabilité, Audit, Traceur, Intention, Décision

---

## 7. Termes d'interdiction

### 7.1. Exécution

**Définition (interdite) :**

L'**exécution** est l'action d'appliquer concrètement une intention. L'exécution est **strictement interdite** pour StrongFather.

**Interdictions :**
- ❌ StrongFather n'exécute **jamais** d'action (INV-AUTH-1, INV-EXEC-1)
- ❌ StrongFather ne possède **jamais** d'autorité sur l'exécution (INV-AUTH-1)
- ❌ Une décision n'entraîne **jamais** d'exécution automatique (INV-AUTH-1)

**Références :**
- Documentation Fondatrice (INV-SF-1)
- Execution Prohibition Contract (INTERD-EXEC-*, INV-EXEC-1)
- Invariants & Guarantees (INV-AUTH-1)

**Termes associés :** Décision, Intention, Autorité, Persistance

**Distinctions :**
- ✅ StrongFather **produit** des décisions
- ❌ StrongFather n'**exécute** jamais d'actions

---

### 7.2. Persistance opérationnelle

**Définition (interdite) :**

La **persistance opérationnelle** est le stockage de données métier qui affecte le comportement du système. La persistance opérationnelle est **strictement interdite** pour StrongFather.

**Interdictions :**
- ❌ StrongFather ne persiste **jamais** de données opérationnelles (INV-EXEC-3, INTERD-PERS-*)
- ❌ StrongFather ne possède **jamais** d'autorité sur la persistance (INV-AUTH-2)
- ❌ Aucun cache décisionnel (INTERD-PERS-3)

**Références :**
- Documentation Fondatrice (INV-SF-2)
- Execution Prohibition Contract (INTERD-PERS-*, INV-EXEC-3)
- Invariants & Guarantees (INV-AUTH-2, INV-EXEC-3)

**Termes associés :** Traçabilité, Cache, KindMother, État

**Distinctions :**
- ✅ La **traçabilité** est autorisée (passive, observationnelle)
- ❌ La **persistance opérationnelle** est interdite (active, comportementale)

---

### 7.3. Logique temporelle technique

**Définition (interdite) :**

La **logique temporelle technique** est l'utilisation du temps technique (horodatages, ordonnancement, planification) pour influencer les décisions. La logique temporelle technique est **strictement interdite** pour StrongFather.

**Interdictions :**
- ❌ StrongFather ne gère **jamais** le temps technique (INV-AUTH-3, INTERD-TIME-*)
- ❌ StrongFather n'ordonnance **jamais** (INTERD-TIME-1)
- ❌ StrongFather ne planifie **jamais** (INTERD-TIME-2, INV-DIFF-NOPLAN)
- ❌ Clock n'est utilisé que pour horodatage de traces (KERN-AUTH-3, KERN-INTERD-1)

**Références :**
- Documentation Fondatrice (INV-SF-4)
- Execution Prohibition Contract (INTERD-TIME-*)
- Boundary & Isolation Contract (KERN-AUTH-3, KERN-INTERD-1)
- Invariants & Guarantees (INV-AUTH-3, INV-DIFF-NOPLAN)

**Termes associés :** Priorité, Ordonnancement, Planification, Clock, Horodatage

**Distinctions :**
- ✅ Les **priorités** sont autorisées (ordre d'importance relatif)
- ❌ L'**ordonnancement temporel** est interdit (moment d'exécution)

---

## 8. Termes interdits ou ambigus

### 8.1. Termes interdits

#### 8.1.1. Cache décisionnel

**Statut :** ❌ **INTERDIT**

**Raison :** Violation de INTERD-PERS-3, INV-EXEC-3, INV-POL-3

**Définition interdite :** Mémorisation de décisions précédentes pour réutilisation.

**Pourquoi interdit :**
- Persistance opérationnelle (INTERD-PERS-3)
- Effet de bord entre évaluations (INV-EXEC-2, INV-EXEC-3)
- Non-déterminisme potentiel (INV-POL-3)

**Références :**
- Execution Prohibition Contract (INTERD-PERS-3)
- Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-POL-3)
- Performance & Scalability Contract (OPT-INTERD-1)

---

#### 8.1.2. Ordonnancement

**Statut :** ❌ **INTERDIT**

**Raison :** Violation de INTERD-TIME-1, INTERD-TIME-2

**Définition interdite :** Détermination du moment d'exécution d'une action.

**Pourquoi interdit :**
- Logique temporelle technique (INTERD-TIME-1)
- Autorité sur l'exécution (INV-AUTH-1)

**Références :**
- Execution Prohibition Contract (INTERD-TIME-1, INTERD-TIME-2)
- Invariants & Guarantees (INV-AUTH-1, INV-AUTH-3)

**Distinction :**
- ❌ **Ordonnancement** : Interdit (moment d'exécution)
- ✅ **Priorité** : Autorisée (ordre d'importance)

---

#### 8.1.3. Planification

**Statut :** ❌ **INTERDIT**

**Raison :** Violation de INTERD-TIME-2, INV-DIFF-NOPLAN

**Définition interdite :** Organisation d'exécutions futures selon le temps.

**Pourquoi interdit :**
- Logique temporelle technique (INTERD-TIME-2)
- Décision différée sans planification (INV-DIFF-NOPLAN)

**Références :**
- Execution Prohibition Contract (INTERD-TIME-2)
- Invariants & Guarantees (INV-DIFF-NOPLAN)

**Distinction :**
- ❌ **Planification** : Interdite (organisation temporelle)
- ✅ **Décision DIFFÉRÉE** : Autorisée (sans planification)

---

#### 8.1.4. Logique métier spécifique

**Statut :** ❌ **INTERDIT**

**Raison :** Violation de Policy Engine Contract section 2.3, Execution Prohibition Contract section 3.5

**Définition interdite :** Règles métier spécifiques à un domaine (e-commerce, finance, etc.).

**Pourquoi interdit :**
- Séparation des responsabilités
- Réutilisabilité des politiques
- Généralité requise

**Références :**
- Policy Engine Contract (section 2.3)
- Execution Prohibition Contract (section 3.5)

**Distinction :**
- ❌ **Logique métier spécifique** : Interdite
- ✅ **Politiques générales** : Autorisées

---

### 8.2. Termes ambigus (à éviter ou clarifier)

#### 8.2.1. Exécution vs Décision

**Ambiguïté :** Confusion entre la production d'une décision et l'exécution d'une action.

**Clarification :**
- ✅ **Décision** : Résultat produit par StrongFather (autorisé)
- ❌ **Exécution** : Application concrète d'une action (interdit)

**Références :**
- Documentation Fondatrice (INV-SF-1)
- Execution Prohibition Contract (INV-EXEC-1)

---

#### 8.2.2. Persistance vs Traçabilité

**Ambiguïté :** Confusion entre la persistance opérationnelle (interdite) et la traçabilité (autorisée).

**Clarification :**
- ✅ **Traçabilité** : Enregistrement passif pour audit (autorisé)
- ❌ **Persistance opérationnelle** : Stockage affectant le comportement (interdit)

**Références :**
- Audit & Trace Contract (section 2.3)
- Execution Prohibition Contract (INTERD-PERS-*)

---

#### 8.2.3. Priorité vs Ordonnancement

**Ambiguïté :** Confusion entre l'ordre d'importance (priorité) et le moment d'exécution (ordonnancement).

**Clarification :**
- ✅ **Priorité** : Ordre d'importance relatif (autorisé)
- ❌ **Ordonnancement** : Moment d'exécution (interdit)

**Références :**
- Documentation Fondatrice (section 10, glossaire "Priorité")
- Execution Prohibition Contract (INTERD-TIME-1)

---

#### 8.2.4. Politique vs Règle métier

**Ambiguïté :** Confusion entre les politiques générales (autorisées) et les règles métier spécifiques (interdites).

**Clarification :**
- ✅ **Politique** : Règle déclarative générale (autorisée)
- ❌ **Règle métier spécifique** : Règle spécifique à un domaine (interdite)

**Références :**
- Policy Engine Contract (section 2.3)
- Execution Prohibition Contract (section 3.5)

---

#### 8.2.5. Erreur vs Rejet

**Ambiguïté :** Confusion entre un dysfonctionnement interne (erreur) et un résultat normal d'évaluation (rejet).

**Clarification :**
- ✅ **Erreur** : Dysfonctionnement interne → `Err(SFError)`
- ✅ **Rejet** : Résultat normal → `Ok(Decision)` avec `DecisionType::Refused`

**Références :**
- Error & Rejection Model (section 2)
- Invariants & Guarantees (INV-ERR-1)

---

## 9. Références croisées

### 9.1. Concepts fondamentaux

| Terme | Concepts liés | Contrats principaux |
|-------|---------------|---------------------|
| **StrongFather** | Intention, Décision, Politique, Évaluation | Documentation Fondatrice, Architecture & Flows |
| **Intention** | Décision, Évaluation, Politique, Contexte | Intent Model Contract, Documentation Fondatrice |
| **Décision** | Intention, Politique, Justification, Évaluation | Core Decision Contract, Documentation Fondatrice |
| **Politique** | Intention, Décision, Évaluation, Moteur de politiques | Policy Engine Contract, Policy Language Specification |
| **Évaluation** | Intention, Décision, Politique, Moteur de politiques | Policy Engine Contract, Architecture & Flows |

---

### 9.2. Concepts d'architecture

| Terme | Concepts liés | Contrats principaux |
|-------|---------------|---------------------|
| **Surface d'évaluation** | Intention, Décision, Validateur | Architecture & Flows |
| **Moteur de politiques** | Politique, Évaluation, Résultat | Policy Engine Contract, Architecture & Flows |
| **Producteur de décision** | Décision, Justification, Résultat | Core Decision Contract, Architecture & Flows |
| **Traceur** | Traçabilité, Trace, Audit | Audit & Trace Contract, Architecture & Flows |

---

### 9.3. Concepts d'invariants

| Terme | Concepts liés | Contrats principaux |
|-------|---------------|---------------------|
| **Invariant** | Garantie, Propriété, Contrat | Invariants & Guarantees |
| **Garantie** | Invariant, Engagement, Contrat | Invariants & Guarantees, Core Decision Contract |
| **Pureté fonctionnelle** | Fonction pure, Effet de bord, Déterminisme | Invariants & Guarantees, Execution Prohibition Contract |
| **Zero-trust** | Validation, Politique, Sécurité | Invariants & Guarantees, Documentation Fondatrice |

---

### 9.4. Concepts d'interdiction

| Terme | Concepts interdits | Contrats principaux |
|-------|-------------------|---------------------|
| **Exécution** | ❌ Interdit | Execution Prohibition Contract, Documentation Fondatrice |
| **Persistance opérationnelle** | ❌ Interdit | Execution Prohibition Contract, Invariants & Guarantees |
| **Logique temporelle technique** | ❌ Interdit | Execution Prohibition Contract, Boundary & Isolation Contract |
| **Logique métier spécifique** | ❌ Interdit | Policy Engine Contract, Execution Prohibition Contract |

---

## 10. Index alphabétique

**A**
- Ambiguïté (section 5.3)
- Appelant (section 4.4)
- Audit (section 6.1)
- Autorité (section 7.1)

**C**
- Cache décisionnel (section 8.1.1) ❌ INTERDIT
- Calculateur de priorité (section 3.5)
- Clarification (section 5.3)
- Compositeur de résultats (section 3.4)
- Contexte (section 2.2)
- Contrainte (section 2.7)

**D**
- Décision (section 2.3)
- Déterminisme (section 4.3)
- Dysfonctionnement (section 5.1)

**E**
- Effet de bord (section 4.3)
- Erreur (section 5.1)
- Évaluation (section 2.5)
- Exécution (section 7.1) ❌ INTERDIT

**F**
- Flux d'évaluation (section 3.1)
- Fonction pure (section 4.3)

**G**
- Garantie (section 4.2)

**I**
- Intention (section 2.2)
- Invariant (section 4.1)

**J**
- Justification (section 2.3)

**L**
- Logique métier spécifique (section 8.1.4) ❌ INTERDIT
- Logique temporelle technique (section 7.3) ❌ INTERDIT

**M**
- Moteur de décision (section 2.1)
- Moteur de politiques (section 3.3)
- Mutation d'état (section 4.3)

**O**
- Ordonnancement (section 8.1.2) ❌ INTERDIT

**P**
- Persistance opérationnelle (section 7.2) ❌ INTERDIT
- Planification (section 8.1.3) ❌ INTERDIT
- Politique (section 2.4)
- Priorité (section 2.6)
- Producteur de décision (section 3.6)
- Pureté fonctionnelle (section 4.3)

**R**
- Rejet (section 5.2)
- Résultat d'évaluation (section 3.3)

**S**
- StrongFather (section 2.1)
- Surface d'évaluation (section 3.1)

**T**
- Trace (section 6.2)
- Traceur (section 3.7)
- Traçabilité (section 6.1)
- Transparence référentielle (section 4.3)

**V**
- Validateur d'intention (section 3.2)
- Validation (section 3.2)

**Z**
- Zero-trust (section 4.4)

---

## 11. Conclusion

Ce glossaire consolide et normalise tous les termes utilisés dans l'écosystème StrongFather. Il garantit :

- **Cohérence sémantique** : Tous les termes ont des définitions normalisées
- **Références croisées** : Les relations entre termes sont explicites
- **Clarification des ambiguïtés** : Les distinctions critiques sont explicitées
- **Identification des interdictions** : Les termes interdits sont clairement marqués

Ce document constitue la référence unique pour la terminologie StrongFather et doit être consulté en cas de doute sur la signification d'un terme.

---

**Document créé le :** 2026-01-26  
**Version :** 1.0  
**Statut :** Référence — Document informatif consolidé  
**Référence :** Miyukini Core System v2.4, Tous les contrats StrongFather FONDATION  
**Type :** Glossaire et terminologie consolidés

---

## 12. Mini log de génération

### Décision éditoriale E1 : Structure du glossaire

**Décision prise :** Organisation du glossaire en sections thématiques (fondamentaux, architecture, invariants, erreurs, traçabilité, interdictions) plutôt qu'alphabétique pure, avec index alphabétique en fin de document.

**Application :** Sections 2 à 8 organisées thématiquement. Section 10 fournit l'index alphabétique pour navigation rapide.

---

### Décision éditoriale E2 : Termes interdits

**Décision prise :** Section dédiée aux termes interdits (section 8.1) avec statut explicite ❌ INTERDIT, raison de l'interdiction, et références contractuelles.

**Application :** Section 8.1 liste tous les termes interdits avec justifications et références.

---

### Décision éditoriale E3 : Termes ambigus

**Décision prise :** Section dédiée aux termes ambigus (section 8.2) avec clarifications explicites des distinctions critiques.

**Application :** Section 8.2 clarifie les ambiguïtés fréquentes avec distinctions ✅/❌.

---

### Décision éditoriale E4 : Références croisées

**Décision prise :** Section de références croisées (section 9) organisée par catégories conceptuelles avec tableaux de relations.

**Application :** Section 9 fournit des tableaux de références croisées par catégorie.

---

### Décision éditoriale E5 : Index alphabétique

**Décision prise :** Index alphabétique complet (section 10) avec références aux sections et marquage des termes interdits.

**Application :** Section 10 fournit un index alphabétique avec numéros de section et marquage ❌ pour les termes interdits.

---

### Vérification de complétude

**Vérification effectuée :**
- ✅ Tous les termes des documents lus sont inclus
- ✅ Références croisées vérifiées
- ✅ Termes interdits identifiés et marqués
- ✅ Ambiguïtés clarifiées
- ✅ Index alphabétique complet

**Conclusion :** Glossaire consolidé complet et cohérent.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
