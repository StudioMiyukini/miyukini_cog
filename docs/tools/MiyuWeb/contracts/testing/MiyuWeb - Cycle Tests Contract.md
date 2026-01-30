# MiyuWeb — Cycle Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests de cycle** du kit MiyuWeb. Les tests de cycle vérifient le chemin complet (résolution thème → chargement template → rendu → formulaire / événement) dans un scénario gouverné et peuvent être exécutés par MiyukiniAdmin pour valider MiyuWeb de façon précise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests de cycle applicables à MiyuWeb (chemin complet : résolution thème → template → rendu → formulaire / événement)
- Le **test chemin complet MiyuWeb** : scénario E2E (résolution thème → données template fournies dans le flux → rendu HTML/layout → validation formulaire ou dispatch/capture événement dans un flux gouverné)
- Le lien avec MiyukiniAdmin comme exécutant du test
- Les métriques et critères de succès

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuWeb - Unit Tests Contract)
- L'implémentation technique des tests

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuWeb sont exécutés dans un environnement de diagnostic contrôlé. Le test chemin complet utilise des données de test (templates, assets, thème) fournies dans le flux ; aucune donnée métier réelle n'est modifiée. MiyuWeb ne lit jamais la base directement.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MWEB-1** | Aucune modification des données métier (templates ou assets de production) |
| **INV-CT-MWEB-2** | Les templates/assets de test sont fournis dans le flux ; isolation et nettoyage obligatoires si sandbox utilisée |
| **INV-CT-MWEB-3** | Traçabilité complète de chaque étape du test |
| **INV-CT-MWEB-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuWeb |
| **INV-CT-MWEB-5** | Aucune lecture directe en base par MiyuWeb ; toutes les données (template, assets, thème) viennent du flux |
| **INV-CT-MWEB-6** | Rapports conservés pour audit |

---

## 4. Types de tests de cycle MiyuWeb

### 4.1 Test chemin complet (thème → template → rendu → formulaire / événement)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MWEB-LAT-001** | Latence tool.web.theme.resolve | < seuil configuré |
| **MWEB-LAT-002** | Latence tool.web.html.render | < seuil configuré |
| **MWEB-LAT-003** | Latence chemin complet (BondingBrother → StrongFather → MiyuWeb Tools) | < seuil configuré (ex. 200 ms) |

---

## 5. Test chemin complet MiyuWeb

### 5.1 Objectif

Ce test vérifie le **chemin complet d'affichage web** : résolution du thème, utilisation de données de template fournies dans le flux, rendu HTML et layout, puis validation de formulaire ou dispatch/capture d'événement dans un scénario gouverné. MiyukiniAdmin peut exécuter ce test pour valider MiyuWeb de façon précise.

### 5.2 Données de test

| Élément | Valeur |
|---------|--------|
| **Template / assets / thème** | Données de test fournies dans le flux (mock ou sandbox) ; aucune donnée métier réelle |
| **Usage** | Réservé à ce test ; isolation et nettoyage obligatoires si sandbox utilisée |
| **Isolation** | Les données sont préparées, fournies dans le flux, utilisées par les Tools MiyuWeb, puis supprimées dans le cadre du test si applicable |

### 5.3 Scénario E2E (étapes)

Les étapes suivantes sont exécutées dans l'ordre. Chaque étape doit réussir pour que le test soit considéré réussi.

| Étape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrée** | Préparation des données de test (thème, template, données de rendu, schéma formulaire ou événement). L'Opérateur (ou MiyukiniAdmin) prépare les données d'entrée et les fournit dans le flux. Aucune lecture directe en base par MiyuWeb. | Opérateur / MiyukiniAdmin → BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. La décision doit être ALLOW pour l'utilisation des Tools MiyuWeb. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Résolution thème** | Appel à tool.web.theme.resolve avec un contexte de test (ex. mode clair, identifiant thème). Vérification que les données de thème sont retournées. | MiyuWeb (tool.web.theme.resolve) |
| **4. Rendu HTML / layout** | Appel à tool.web.html.render (et optionnellement tool.web.layout.render) avec template et données fournis dans le flux. Vérification que le HTML/layout produit est conforme. | MiyuWeb (tool.web.html.render, tool.web.layout.render) |
| **5. Formulaire ou événement** | Soit : appel à tool.web.form.validate pour un formulaire de test (structure, champs) ; soit : tool.web.event.dispatch puis tool.web.input.capture pour un scénario événementiel. Vérification que le résultat est conforme. | MiyuWeb (tool.web.form.validate ou tool.web.event.dispatch / tool.web.input.capture) |
| **6. Nettoyage** | Suppression des données de test si sandbox utilisée ; tear-down documenté. | Via gouvernance |

### 5.4 Critères de succès du test

| Critère | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exécutées et ALLOW obtenu |
| **C2** | tool.web.theme.resolve retourne des données de thème cohérentes pour le contexte fourni |
| **C3** | tool.web.html.render (et tool.web.layout.render si applicable) retourne du HTML/layout conforme aux entrées du flux |
| **C4** | tool.web.form.validate (ou tool.web.event.dispatch / tool.web.input.capture) retourne un résultat conforme |
| **C5** | Aucune lecture directe en base par MiyuWeb ; toutes les entrées proviennent du flux |
| **C6** | Nettoyage effectué ; aucune donnée métier modifiée |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les étapes 1 à 6 réussies et tous les critères C1–C6 remplis |
| **FAIL** | Une étape échoue ou un critère n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 Exécutant : MiyukiniAdmin

MiyukiniAdmin peut exécuter ce test pour vérifier le chemin complet MiyuWeb. La référence croisée peut être établie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuWeb Full Path Test), qui pointe vers ce contrat pour la spécification du scénario et des critères.

---

## 6. Protocole d'exécution

### 6.1 Phases

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1 : Préparation                                        │
├─────────────────────────────────────────────────────────────┤
│ - Vérification pré-conditions (gouvernance, état système)    │
│ - Préparation données de test (thème, template, formulaire   │
│   ou événement) à fournir dans le flux ; pas de lecture base │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 2 : Exécution (étapes 1 à 6)                           │
├─────────────────────────────────────────────────────────────┤
│ - Contexte → Validations Cores → theme.resolve →            │
│   html.render / layout.render → form.validate ou             │
│   event.dispatch / input.capture → Nettoyage                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 3 : Rapport                                            │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/FAIL/ERROR)                                 │
│ - Détails par étape                                          │
│ - Stockage pour audit                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Métriques et rapports

### 7.1 Métriques collectées (optionnel)

| Métrique | Description |
|----------|-------------|
| `step_duration_ms` | Durée par étape (1 à 6) |
| `total_duration_ms` | Durée totale du test |
| `validation_latency_ms` | Latence cumulée des validations Cores |
| `render_latency_ms` | Latence du rendu HTML/layout |

### 7.2 Structure rapport (résumé)

```json
{
  "test_id": "MiyuWeb_FullPath",
  "timestamp": "2026-01-30T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "Contexte entree", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Resolution theme", "status": "OK"},
    {"step": 4, "name": "Rendu HTML / layout", "status": "OK"},
    {"step": 5, "name": "Formulaire ou evenement", "status": "OK"},
    {"step": 6, "name": "Nettoyage", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 450
}
```

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Unit Tests Contract | [MiyuWeb - Unit Tests Contract](./MiyuWeb%20-%20Unit%20Tests%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
