# MiyuWeb — Unit Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests unitaires** des Tools du kit MiyuWeb. Les tests unitaires vérifient le comportement de chaque Tool (rendu HTML, layout, thème, script, asset, formulaire, événements) sans modifier les données métier et sans décision de contenu ni accès direct à la base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests unitaires sur les Tools MiyuWeb (html.render, layout.render, theme.resolve, script.execute, script.compile, asset.serve, form.validate, event.dispatch, input.capture)
- Les critères de succès et d'échec
- La non-destructivité et l'absence de modification de données métier
- Les protocoles de vérification (sandbox / mocks pour données et assets)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuWeb - Cycle Tests Contract)
- L'implémentation technique des tests
- Les tests de cohérence applicative (voir MiyukiniAdmin - Unit Tests Contract si pertinent)

---

## 3. Principe fondamental

### 3.1 Non-destructivité

> **Les tests unitaires MiyuWeb vérifient le comportement des Tools sans modifier les données métier. Les tests utilisant des templates ou assets utilisent une sandbox ou des mocks ; aucune donnée métier n'est modifiée. MiyuWeb ne lit jamais la base directement — les données sont fournies dans le flux.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MWEB-1** | Aucune modification des données métier (templates ou assets de production) |
| **INV-UT-MWEB-2** | Les tests utilisant templates/assets utilisent une sandbox ou des mocks avec nettoyage obligatoire |
| **INV-UT-MWEB-3** | Aucun accès direct à la base par MiyuWeb ; toutes les entrées sont fournies dans le flux (mock ou sandbox) |
| **INV-UT-MWEB-4** | Traçabilité complète de chaque test (contexte, verdict, durée) |
| **INV-UT-MWEB-5** | Rapports conservés pour audit |

---

## 4. Catégories de tests par Tool

### 4.1 Tests Rendu HTML (tool.web.html.render)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-HR-001** | Rendu template valide | Vérifie qu'un template et des données fournis en flux produisent du HTML conforme | Template et données de test (mock) ; pas de données métier |
| **MWEB-HR-002** | Rendu template vide | Vérifie le comportement pour un template vide ou des données vides | Données de test |
| **MWEB-HR-003** | Rendu sans décision contenu | Vérifie que le Tool ne choisit pas le contenu ; il rend uniquement ce qui est fourni | Données de test |
| **MWEB-HR-004** | Sanitization sortie | Vérifie que la sortie HTML est conforme aux attentes de sécurité (pas d'injection brute non gouvernée) | Données de test contrôlées |

### 4.2 Tests Rendu Layout (tool.web.layout.render)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-LR-001** | Rendu layout valide | Vérifie qu'un layout (zones, placeholders) et des données fournis produisent une structure conforme | Layout et données de test (mock) |
| **MWEB-LR-002** | Rendu layout partiel | Vérifie le comportement pour des zones manquantes ou partielles | Données de test |
| **MWEB-LR-003** | Pas de décision contenu zones | Vérifie que le Tool ne décide pas du contenu des zones ; exécute uniquement le rendu de structure | Données de test |

### 4.3 Tests Résolution thème (tool.web.theme.resolve)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-TR-001** | Résolution thème valide | Vérifie qu'un contexte (ex. mode clair/sombre, identifiant thème) produit des données de thème conformes | Contexte de test |
| **MWEB-TR-002** | Résolution thème inconnu | Vérifie le comportement pour un identifiant de thème inconnu ou absent | Données de test |
| **MWEB-TR-003** | Pas de politique thème | Vérifie que le Tool ne décide pas de la politique de thème ; il résout uniquement pour le contexte fourni | Données de test |

### 4.4 Tests Exécution script (tool.web.script.execute)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-SE-001** | Exécution script valide | Vérifie qu'un script fourni dans le flux s'exécute dans un contexte sandboxé et retourne un résultat attendu | Script et données de test (mock) |
| **MWEB-SE-002** | Sandbox isolation | Vérifie qu'aucun accès direct à la base ni décision métier n'est possible depuis le script exécuté | Script de test contrôlé |
| **MWEB-SE-003** | Script invalide | Vérifie le comportement pour un script mal formé ou interdit | Données de test |
| **MWEB-SE-004** | Entrées fournies dans le flux | Vérifie que les entrées (données, contexte) sont bien celles fournies dans le flux | Données de test |

### 4.5 Tests Compilation script (tool.web.script.compile)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-SC-001** | Compilation script valide | Vérifie qu'un script fourni est compilé ou validé (syntaxe, types) sans exécution | Script de test |
| **MWEB-SC-002** | Refus script invalide | Vérifie que un script invalide est rejeté ou signale des erreurs conformes | Script de test |
| **MWEB-SC-003** | Pas d'exécution | Vérifie que le Tool ne fait que compiler/valider, jamais exécuter | Script de test |

### 4.6 Tests Service asset (tool.web.asset.serve)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-AS-001** | Service asset valide | Vérifie qu'un asset (contenu ou métadonnées fournis dans le flux) est servi correctement | Asset de test (mock) ; pas de lecture base |
| **MWEB-AS-002** | Asset absent ou vide | Vérifie le comportement pour un asset absent ou vide | Données de test |
| **MWEB-AS-003** | Pas de lecture base | Vérifie que le Tool ne lit pas la base ; l'asset est uniquement celui fourni dans le flux | Données de test |

### 4.7 Tests Validation formulaire (tool.web.form.validate)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-FV-001** | Validation formulaire valide | Vérifie qu'un formulaire (structure, champs) et des règles fournies sont validés correctement | Formulaire et règles de test |
| **MWEB-FV-002** | Validation formulaire invalide | Vérifie le rejet ou les erreurs pour une structure/champs invalides | Données de test |
| **MWEB-FV-003** | Pas de règles métier | Vérifie que le Tool n'introduit pas de règles métier ; exécute une validation gouvernée sur des règles fournies | Données de test |

### 4.8 Tests Dispatch événement (tool.web.event.dispatch)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-ED-001** | Dispatch événement valide | Vérifie qu'un événement est propagé dans le flux gouverné conformément au contrat | Événement de test |
| **MWEB-ED-002** | Pas de décision traitement | Vérifie que le Tool ne décide pas du traitement ; il dispatche uniquement | Données de test |

### 4.9 Tests Capture entrée (tool.web.input.capture)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MWEB-IC-001** | Capture entrée valide | Vérifie qu'une entrée utilisateur (clic, saisie, touche) est capturée et transmise au flux gouverné | Entrée de test |
| **MWEB-IC-002** | Pas de décision usage | Vérifie que le Tool ne décide pas de l'usage ; il capture uniquement | Données de test |

---

## 5. Critères de succès et d'échec

### 5.1 Critères de succès

| Critères | Description |
|----------|-------------|
| **Exécution conforme** | Le Tool s'exécute comme spécifié (pas d'exception non contractuelle) |
| **Résultat attendu** | Sortie (HTML, layout, thème, résultat script, asset servi, verdict validation, événement, entrée) conforme au contrat |
| **Pas de fuite** | Aucune donnée métier exposée ; sandbox nettoyée après test si applicable |
| **Traçabilité** | Contexte, verdict, durée enregistrés |

### 5.2 Critères d'échec

| Critères | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lève une exception non prévue par le contrat |
| **Modification hors sandbox** | Une donnée métier (template ou asset de production) est modifiée |
| **Accès direct base** | MiyuWeb tente une lecture ou écriture directe en base pendant le test |
| **Nettoyage non effectué** | La sandbox ou les mocks ne sont pas nettoyés après un test |
| **Timeout dépassé** | Le test dépasse le timeout configuré sans résultat |

### 5.3 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les critères de succès sont remplis |
| **WARN** | Comportement conforme avec alertes mineures (ex. performance) |
| **FAIL** | Un ou plusieurs critères d'échec sont remplis |
| **SKIP** | Pré-condition non remplie (ex. sandbox indisponible) |
| **ERROR** | Erreur technique pendant le test (configuration, environnement) |

---

## 6. Protocole de test

### 6.1 Exécution d'un test unitaire MiyuWeb

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Chargement définition du test                              │
├─────────────────────────────────────────────────────────────┤
│ - ID du test (MWEB-*)                                         │
│ - ToolId concerné                                             │
│ - Paramètres (sandbox, mocks, timeout, etc.)                   │
│ - Critères de succès                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Préparation (si sandbox / mocks)                            │
├─────────────────────────────────────────────────────────────┤
│ - Création sandbox / mocks (templates, assets, données)       │
│ - Via gouvernance ; aucune lecture directe en base             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Exécution du Tool (via gouvernance)                        │
├─────────────────────────────────────────────────────────────┤
│ - BondingBrother → Master Butler → WorrySentinel →            │
│   Caring Nanny → StrongFather → MiyuWeb Tool                 │
│ - Données fournies dans le flux ; collecte résultat ou exception │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Nettoyage (si sandbox)                                     │
├─────────────────────────────────────────────────────────────┤
│ - Suppression données test / tear-down sandbox                │
│ - Via gouvernance                                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Évaluation et rapport                                      │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/WARN/FAIL/SKIP/ERROR)                        │
│ - Détails, durée, traçabilité                                │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Suites de tests

### 7.1 Suites prédéfinies

| Suite | Tests inclus | Durée estimée | Usage |
|-------|--------------|---------------|-------|
| **Quick** | MWEB-HR-001, MWEB-TR-001, MWEB-ED-001, MWEB-IC-001 | < 1 min | Vérification rapide |
| **Standard** | Tous MWEB-HR, MWEB-LR, MWEB-TR, MWEB-AS, MWEB-FV, MWEB-ED, MWEB-IC (mocks) | 2–5 min | Vérification quotidienne |
| **Full** | Tous MWEB-* (avec sandbox pour script.execute / script.compile si applicable) | 5–10 min | Vérification complète |

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Reference Outils | [MiyuWeb - Reference Outils](../../MiyuWeb%20-%20Reference%20Outils.md) |
| MiyuWeb - Cycle Tests Contract | [MiyuWeb - Cycle Tests Contract](./MiyuWeb%20-%20Cycle%20Tests%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
