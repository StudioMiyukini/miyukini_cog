# MiyuAuth — Cycle Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests de cycle** du kit MiyuAuth. Les tests de cycle vérifient le chemin complet (résolution identité → rôle → vérification Passeport/Visa) dans un scénario gouverné et peuvent être exécutés par MiyukiniAdmin pour valider MiyuAuth de façon précise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests de cycle applicables à MiyuAuth (chemin complet, résolution → rôle → vérification)
- Le **test chemin complet MiyuAuth** : scénario E2E (résolution identité → rôle → vérification Passeport/Visa dans un flux gouverné)
- Le lien avec MiyukiniAdmin comme exécutant du test
- Les métriques et critères de succès

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuAuth - Unit Tests Contract)
- L'implémentation technique des tests

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuAuth sont exécutés dans un environnement de diagnostic contrôlé. Le test chemin complet utilise des données de test (Passeport/Visa de test) ; aucune donnée métier réelle n'est modifiée.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MAUTH-1** | Aucune modification des données métier (identités de production, Passeports/Visas réels) |
| **INV-CT-MAUTH-2** | Les Passeports/Visas de test sont réservés au test ; isolation et nettoyage obligatoires |
| **INV-CT-MAUTH-3** | Traçabilité complète de chaque étape du test |
| **INV-CT-MAUTH-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuAuth |
| **INV-CT-MAUTH-5** | Validation KindMother (confiance) pour toute utilisation de confiance inter-domaines dans le scénario |
| **INV-CT-MAUTH-6** | Rapports conservés pour audit |

---

## 4. Types de tests de cycle MiyuAuth

### 4.1 Test chemin complet (résolution → rôle → vérification)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MAUTH-LAT-001** | Latence tool.identity.resolve | < seuil configuré |
| **MAUTH-LAT-002** | Latence tool.identity.verify | < seuil configuré |
| **MAUTH-LAT-003** | Latence chemin complet (BondingBrother → KindMother → MiyuAuth) | < 100 ms |

---

## 5. Test chemin complet MiyuAuth

### 5.1 Objectif

Ce test vérifie le **chemin complet d'identité** : résolution d'un contexte d'identité (citoyen / visiteur / externe), résolution du rôle, et vérification d'un Passeport Utilisateur ou Visa de Connexion dans un scénario gouverné. MiyukiniAdmin peut exécuter ce test pour valider MiyuAuth de façon précise.

### 5.2 Données de test

| Élément | Valeur |
|---------|--------|
| **Passeport / Visa** | Données de test (mock ou sandbox) ; aucune donnée métier réelle |
| **Usage** | Réservé à ce test ; isolation et nettoyage obligatoires |
| **Isolation** | Les données sont créées, utilisées et supprimées dans le cadre du test |

### 5.3 Scénario E2E (étapes)

Les étapes suivantes sont exécutées dans l'ordre. Chaque étape doit réussir pour que le test soit considéré réussi.

| Étape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrée** | Préparation d'un contexte d'identité de test (citoyen, visiteur ou externe). L'Opérateur (ou MiyukiniAdmin) prépare les données d'entrée (Passeport/Visa de test ou absence de certificat). | Opérateur / MiyukiniAdmin → BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. La décision doit être ALLOW pour l'utilisation des Tools MiyuAuth. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Validation KindMother** | Pour toute utilisation de confiance inter-domaines (ex. visiteur), KindMother valide la confiance. | KindMother |
| **4. Résolution identité** | Appel à tool.identity.resolve avec le contexte de test. Vérification que le contexte est résolu (citoyen / visiteur / externe). | MiyuAuth (tool.identity.resolve) |
| **5. Rôle résolu** | Appel à tool.identity.role pour le contexte gouverné. Vérification que le rôle retourné est cohérent (citoyen, visiteur, externe). | MiyuAuth (tool.identity.role) |
| **6. Vérification Passeport/Visa** | Si contexte visiteur : appel à tool.identity.verify avec Passeport/Visa de test. Vérification que la structure et la signature sont reconnues (vérification technique ; confiance validée par KindMother). | MiyuAuth (tool.identity.verify) |
| **7. Attestation (optionnel)** | Si applicable : appel à tool.identity.attest pour un contexte validé par KindMother. Vérification que l'attestation est produite. | MiyuAuth (tool.identity.attest) |
| **8. Nettoyage** | Suppression des données de test ; tear-down documenté. | Via gouvernance |

### 5.4 Critères de succès du test

| Critère | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exécutées et ALLOW obtenu |
| **C2** | Validation KindMother (confiance) obtenue pour toute utilisation de confiance inter-domaines |
| **C3** | tool.identity.resolve retourne un contexte cohérent (citoyen / visiteur / externe) |
| **C4** | tool.identity.role retourne le rôle attendu |
| **C5** | tool.identity.verify (si applicable) retourne un verdict conforme (structure, signature) |
| **C6** | tool.identity.attest (si applicable) produit une attestation pour un contexte validé |
| **C7** | Nettoyage effectué ; aucune donnée métier modifiée |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les étapes 1 à 8 réussies et tous les critères C1–C7 remplis |
| **FAIL** | Une étape échoue ou un critère n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 Exécutant : MiyukiniAdmin

MiyukiniAdmin peut exécuter ce test pour vérifier le chemin complet MiyuAuth. La référence croisée peut être établie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuAuth Full Path Test), qui pointe vers ce contrat pour la spécification du scénario et des critères.

---

## 6. Protocole d'exécution

### 6.1 Phases

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1 : Préparation                                        │
├─────────────────────────────────────────────────────────────┤
│ - Vérification pré-conditions (gouvernance, état système)    │
│ - Création données de test (Passeport/Visa mock) si besoin   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 2 : Exécution (étapes 1 à 8)                           │
├─────────────────────────────────────────────────────────────┤
│ - Contexte → Validations Cores → KindMother → resolve       │
│ - role → verify (si visiteur) → attest (optionnel) → Nettoyage│
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
| `step_duration_ms` | Durée par étape (1 à 8) |
| `total_duration_ms` | Durée totale du test |
| `validation_latency_ms` | Latence cumulée des validations Cores et KindMother |

### 7.2 Structure rapport (résumé)

```json
{
  "test_id": "MiyuAuth_FullPath",
  "timestamp": "2026-01-30T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "Contexte entree", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Validation KindMother", "status": "OK"},
    {"step": 4, "name": "Resolution identite", "status": "OK"},
    {"step": 5, "name": "Role resolu", "status": "OK"},
    {"step": 6, "name": "Verification Passeport/Visa", "status": "OK"},
    {"step": 7, "name": "Attestation", "status": "OK"},
    {"step": 8, "name": "Nettoyage", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 350
}
```

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Unit Tests Contract | [MiyuAuth - Unit Tests Contract](./MiyuAuth%20-%20Unit%20Tests%20Contract.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
