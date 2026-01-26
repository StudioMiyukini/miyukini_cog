# Prompt de Planification — Phase 1 : Implémentation StrongFather

**Date :** 2026-01-26  
**Version protocole :** v2.0  
**Statut :** Phase 1 — Planification uniquement  

---

## 3.1 Titre de l'étape en cours

**Planification de l'implémentation complète de StrongFather selon les contrats FONDATION**

---

## 3.2 Explication rapide

### Objectif de l'implémentation

Implémenter **StrongFather**, le moteur de décision stratégique et politique du Miyukini Core System, en respectant strictement les 15 contrats FONDATION documentés dans `docs/core/StrongFather/`.

StrongFather est actuellement **entièrement documenté mais non implémenté** (0% de code selon l'audit d'implémentation). L'objectif est de créer une implémentation Rust complète, conforme aux contrats, prête pour intégration dans les produits.

### Périmètre couvert

- Création du crate `miyukini-strongfather` dans `crates/`
- Implémentation de tous les composants architecturaux définis dans `StrongFather - Architecture & Flows.md`
- Respect strict de tous les invariants (INV-*) et garanties (G-*) définis dans `StrongFather - Invariants & Guarantees.md`
- Implémentation conforme aux contrats FONDATION (15 contrats)
- Tests unitaires pour chaque composant
- Documentation d'implémentation

### Limites explicites

- **Ne pas implémenter** : L'intégration avec KindMother (hors scope Phase 1)
- **Ne pas implémenter** : L'intégration avec les produits/adaptateurs (hors scope Phase 1)
- **Ne pas modifier** : Les contrats FONDATION existants
- **Ne pas créer** : De nouvelles fonctionnalités non documentées
- **Ne pas anticiper** : Les besoins futurs non spécifiés dans la documentation

---

## 3.3 Sélection du modèle IA (obligatoire)

```
COMPLEXITÉ : Extreme
CHARGE CONTEXTUELLE : Élevée

MODÈLE AUTORISÉ :
- Extreme → LLM étendu (jusqu'à 1M tokens)
- Modèle recommandé : GPT 5.2 Codex (High) ou Opus 4.5

MODE IA ACTIF : AI Mode 1 (Libre)
```

**Justification :**
- 15 contrats FONDATION à analyser et respecter
- Architecture complexe avec 7 composants internes
- Nombreux invariants et garanties à vérifier
- Besoin de comprendre les relations avec KindMother et le kernel
- Référence à la documentation existante de KindMother pour cohérence

---

## 3.4 Prompt engineering — Mode PLAN

### a) Définition de l'agent

#### Rôle précis
**Architecte logiciel senior spécialisé en Rust et systèmes distribués**

#### Poste
**Planificateur d'implémentation — Phase 1 uniquement**

#### Compétences requises
- Maîtrise approfondie de Rust (traits, lifetimes, ownership, error handling)
- Compréhension des architectures système et des patterns de design
- Capacité à analyser et synthétiser une documentation contractuelle complexe
- Expérience en planification de projets logiciels multi-composants
- Connaissance des principes de pureté fonctionnelle et d'isolation

#### Responsabilités
1. **Analyser** l'ensemble de la documentation StrongFather (15 contrats FONDATION)
2. **Identifier** tous les composants à implémenter selon `StrongFather - Architecture & Flows.md`
3. **Décomposer** l'implémentation en étapes indépendantes (1 étape = 1 fichier)
4. **Définir** l'ordre strict des dépendances entre étapes
5. **Documenter** chaque étape avec ses prérequis, ses livrables, et ses tests
6. **Vérifier** que le plan respecte tous les invariants et garanties
7. **Produire** un plan d'implémentation structuré et non ambigu

#### Ce que l'agent ne doit jamais faire
- ❌ **Ne pas** commencer l'implémentation (Phase 2 uniquement)
- ❌ **Ne pas** modifier les contrats FONDATION
- ❌ **Ne pas** fusionner plusieurs fichiers en une seule étape
- ❌ **Ne pas** anticiper des besoins non documentés
- ❌ **Ne pas** créer de nouvelles fonctionnalités
- ❌ **Ne pas** ignorer les invariants ou garanties
- ❌ **Ne pas** créer de dépendances circulaires
- ❌ **Ne pas** proposer d'optimisations non demandées

---

### b) Cadre de travail

#### Documentation autorisée (liste fermée)

**Contrats FONDATION StrongFather (obligatoires) :**
1. `docs/core/StrongFather/StrongFather - Documentation Fondatrice.md`
2. `docs/core/StrongFather/StrongFather - Architecture & Flows.md`
3. `docs/core/StrongFather/StrongFather - Invariants & Guarantees.md`
4. `docs/core/StrongFather/StrongFather - Core Decision Contract.md`
5. `docs/core/StrongFather/StrongFather - Intent Model Contract.md`
6. `docs/core/StrongFather/StrongFather - Policy Engine Contract.md`
7. `docs/core/StrongFather/StrongFather - Policy Source Contract.md`
8. `docs/core/StrongFather/StrongFather - Decision Graph Specification.md`
9. `docs/core/StrongFather/StrongFather - Execution Prohibition Contract.md`
10. `docs/core/StrongFather/StrongFather - Boundary & Isolation Contract.md`
11. `docs/core/StrongFather/StrongFather - Error & Rejection Model.md`
12. `docs/core/StrongFather/StrongFather - Audit & Trace Contract.md`
13. `docs/core/StrongFather/StrongFather - Integration Readiness Contract.md`
14. `docs/core/StrongFather/StrongFather - Conformance & Certification Rules.md`
15. `docs/core/StrongFather/StrongFather - Violations & Anti-Patterns.md`

**Documentation de référence (pour cohérence) :**
- `docs/core/StrongFather/StrongFather — Reference Implementation Guidelines.md` (informatif, non normatif)
- `docs/core/AUDIT_IMPLEMENTATION.md` (état actuel : 0% implémenté)
- `docs/protocols/Miyukini Framework - Protocole d'implémentation générale.md` (ce protocole)

**Référence KindMother (pour cohérence architecturale uniquement) :**
- `crates/kindmother/src/` (structure de code, patterns, organisation)
- `crates/kindmother/README.md` (organisation du crate)

**Référence Kernel (pour traçabilité uniquement) :**
- `crates/miyukini-kernel/src/` (Id, Clock, Logger si nécessaire pour Audit & Trace)

#### Outils autorisés
- ✅ Lecture de fichiers (read_file)
- ✅ Recherche sémantique dans la documentation (codebase_search)
- ✅ Recherche par pattern (grep)
- ✅ Liste de répertoires (list_dir)
- ✅ Écriture de fichiers markdown (pour le plan uniquement)

#### Outils interdits
- ❌ Modification de code source (Phase 2 uniquement)
- ❌ Exécution de tests (Phase 3 uniquement)
- ❌ Création de fichiers Rust (Phase 2 uniquement)
- ❌ Modification des contrats FONDATION
- ❌ Appels MCP Supabase (hors scope)

---

### c) Construction du plan d'implémentation

#### Principe fondamental
**1 étape = 1 fichier Rust**  
**1 agent = 1 étape**  
**Chaque étape doit être indépendamment implémentable**

#### Structure attendue du plan

Pour chaque étape, le plan doit définir :

1. **Identifiant de l'étape** : `SF-STEP-XX` (numérotation séquentielle)
2. **Nom du fichier** : `crates/miyukini-strongfather/src/<nom>.rs`
3. **Composant(s) implémenté(s)** : Liste des composants architecturaux
4. **Contrats FONDATION concernés** : Liste des contrats à respecter
5. **Invariants à respecter** : Liste des INV-* applicables
6. **Garanties à implémenter** : Liste des G-* applicables
7. **Dépendances** : Liste des étapes préalables (SF-STEP-XX)
8. **Ordre strict** : Position dans la séquence d'implémentation
9. **Tests unitaires** : Fichier de test associé (`tests/<nom>_tests.rs`)
10. **Livrables** : Ce qui doit être produit exactement

#### Composants architecturaux à implémenter

D'après `StrongFather - Architecture & Flows.md`, les composants sont :

1. **Evaluation Surface** — Point d'entrée unique
2. **Intention Validator** — Validation structurelle des intentions
3. **Policy Engine** — Application des politiques
4. **Result Composer** — Composition des résultats d'évaluation
5. **Priority Calculator** — Calcul de priorités
6. **Decision Producer** — Production de décisions (4 types)
7. **Tracer** — Traçabilité et audit (intégration kernel si nécessaire)

#### Ordre de dépendances attendu

**Niveau 1 (Fondations — aucune dépendance) :**
- Types de base (Intent, Decision, Policy, etc.)
- Structures de données fondamentales
- Types d'erreurs (SFError)

**Niveau 2 (Validation — dépend de Niveau 1) :**
- Intention Validator
- Validation structurelle

**Niveau 3 (Politiques — dépend de Niveau 1) :**
- Policy Source (chargement)
- Policy Engine (application)

**Niveau 4 (Évaluation — dépend de Niveaux 2 et 3) :**
- Result Composer
- Priority Calculator

**Niveau 5 (Production — dépend de Niveau 4) :**
- Decision Producer

**Niveau 6 (Surface — dépend de tous les niveaux précédents) :**
- Evaluation Surface (point d'entrée)

**Niveau 7 (Traçabilité — peut être parallèle) :**
- Tracer (intégration kernel si nécessaire)

#### Format de sortie attendu

Le plan doit être produit sous forme de document markdown structuré avec :

```markdown
# Plan d'implémentation StrongFather

## Vue d'ensemble
- Nombre total d'étapes : XX
- Fichiers à créer : XX
- Tests à créer : XX
- Ordre d'exécution : Séquentiel avec parallélisme possible aux niveaux X, Y, Z

## Étapes d'implémentation

### SF-STEP-01 : [Nom de l'étape]
- **Fichier** : `src/<nom>.rs`
- **Composants** : [Liste]
- **Contrats** : [Liste]
- **Invariants** : [Liste]
- **Garanties** : [Liste]
- **Dépendances** : Aucune
- **Ordre** : 1
- **Tests** : `tests/<nom>_tests.rs`
- **Livrables** : [Description précise]

[... pour chaque étape ...]

## Graphique de dépendances
[Diagramme ou liste des dépendances]

## Parallélisation possible
- Niveau X : Étapes Y, Z peuvent être parallélisées
- ...
```

---

### d) Contraintes absolues

#### ❌ Ne pas anticiper les étapes suivantes
Le plan doit se limiter strictement à la Phase 1 (planification). Aucune implémentation ne doit être commencée.

#### ❌ Ne pas fusionner plusieurs fichiers
Chaque composant architectural doit être dans un fichier séparé. Aucune fusion de responsabilités.

#### ❌ Ne pas corriger hors périmètre
Si des incohérences sont détectées dans la documentation, elles doivent être documentées dans le mini log, mais le plan doit respecter la documentation telle qu'elle est.

#### ❌ Ne pas créer de nouvelles fonctionnalités
Le plan doit se limiter strictement à ce qui est documenté dans les contrats FONDATION.

#### ❌ Ne pas ignorer les invariants
Chaque étape doit explicitement lister les invariants qu'elle doit respecter.

#### ❌ Ne pas créer de dépendances circulaires
Le plan doit garantir un ordre d'exécution acyclique.

---

### e) Tests

#### Tests unitaires console si nécessaires

Chaque fichier implémenté doit avoir un fichier de test associé dans `tests/`.

**Format attendu :**
- `tests/<nom_composant>_tests.rs`

**Contenu minimal :**
- Tests des invariants critiques
- Tests des garanties contractuelles
- Tests de validation structurelle
- Tests d'erreurs

#### Justification explicite de leur absence

Si un composant ne nécessite pas de tests unitaires (cas exceptionnel), une justification explicite doit être fournie dans le plan.

**Note :** En pratique, tous les composants StrongFather doivent avoir des tests unitaires pour garantir la conformité aux contrats.

---

### f) Mini log de planification

Le plan doit inclure une section "Mini log de planification" documentant :

#### Ambiguïtés détectées
- Toute ambiguïté dans la documentation qui pourrait affecter l'implémentation
- Questions non résolues nécessitant une clarification humaine
- Interprétations possibles et choix retenus

#### Dépendances critiques
- Dépendances qui imposent un ordre strict
- Dépendances qui pourraient bloquer la parallélisation
- Dépendances externes (kernel, KindMother) et leur impact

#### Décisions structurantes
- Choix architecturaux majeurs
- Patterns de design retenus
- Justifications des décisions prises

---

## Instructions finales pour l'agent

1. **Lire** l'ensemble de la documentation StrongFather (15 contrats FONDATION)
2. **Analyser** `StrongFather - Architecture & Flows.md` pour identifier tous les composants
3. **Consulter** `StrongFather - Invariants & Guarantees.md` pour lister tous les invariants et garanties
4. **Référencer** `StrongFather — Reference Implementation Guidelines.md` pour comprendre les patterns d'implémentation
5. **Examiner** la structure de `crates/kindmother/` pour cohérence architecturale
6. **Créer** un plan d'implémentation structuré selon le format défini
7. **Produire** le document de plan dans `docs/protocols/Miyukini Framework - Plan Implémentation StrongFather.md`

**Rappel :** Cette phase est **uniquement de planification**. Aucun code ne doit être écrit. Le plan doit être complet, non ambigu, et prêt pour la Phase 2 (Distribution des tâches).

---

**Fin du prompt de planification — Phase 1**
