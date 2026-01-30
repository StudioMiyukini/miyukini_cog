# MiyuAuth — Unit Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests unitaires** des Tools du kit MiyuAuth. Les tests unitaires vérifient le comportement de chaque Tool (resolve, attest, verify, role) sans modifier les données réelles et sans décision de confiance ni d'autorisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests unitaires sur les Tools MiyuAuth (resolve, attest, verify, role)
- Les critères de succès et d'échec
- La non-destructivité et l'absence de modification de données métier
- Les protocoles de vérification (sandbox / mocks pour attest/verify si nécessaire)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuAuth - Cycle Tests Contract)
- L'implémentation technique des tests
- Les tests de cohérence applicative (voir MiyukiniAdmin - Unit Tests Contract si pertinent)

---

## 3. Principe fondamental

### 3.1 Non-destructivité

> **Les tests unitaires MiyuAuth vérifient le comportement des Tools sans modifier les données métier. Les tests d'attestation et de vérification utilisent une sandbox ou des mocks si nécessaire ; aucune donnée métier n'est modifiée.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MAUTH-1** | Aucune modification des données métier (identités de production, Passeports/Visas réels) |
| **INV-UT-MAUTH-2** | Les tests d'attestation et de vérification utilisent une sandbox ou des mocks (Passeport/Visa de test) avec nettoyage obligatoire |
| **INV-UT-MAUTH-3** | Traçabilité complète de chaque test (contexte, verdict, durée) |
| **INV-UT-MAUTH-4** | Rapports conservés pour audit |

---

## 4. Catégories de tests par Tool

### 4.1 Tests Résolution (tool.identity.resolve)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MAUTH-R-001** | Résolution citoyen | Vérifie qu'un contexte citoyen (données fournies) est résolu correctement | Contexte de test ; pas de données métier |
| **MAUTH-R-002** | Résolution visiteur | Vérifie qu'un contexte visiteur (Passeport/Visa de test) est résolu correctement | Sandbox / mocks ; pas de données métier |
| **MAUTH-R-003** | Résolution externe | Vérifie qu'un contexte externe (absence de certificat) est résolu correctement | Aucune |
| **MAUTH-R-004** | Contexte invalide | Vérifie le comportement pour un contexte invalide ou incomplet | Aucune |

### 4.2 Tests Attestation (tool.identity.attest)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MAUTH-A-001** | Attestation contexte validé | Vérifie qu'une attestation est produite pour un contexte validé par KindMother (mock) | Sandbox / mocks ; pas de données métier |
| **MAUTH-A-002** | Attestation contexte non validé | Vérifie le refus d'attestation sans validation KindMother | Aucune |
| **MAUTH-A-003** | Structure attestation | Vérifie que la structure de l'attestation est conforme au contrat | Sandbox / mocks |

### 4.3 Tests Vérification (tool.identity.verify)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MAUTH-V-001** | Vérification Passeport valide | Vérifie qu'un Passeport Utilisateur de test (mock) est reconnu comme valide (structure, signature) | Sandbox / mocks ; pas de données métier |
| **MAUTH-V-002** | Vérification Visa valide | Vérifie qu'un Visa de Connexion de test (mock) est reconnu comme valide | Sandbox / mocks |
| **MAUTH-V-003** | Vérification invalide | Vérifie le rejet pour un Passeport/Visa invalide ou mal formé | Aucune |
| **MAUTH-V-004** | Vérification sans confiance | Vérifie que la vérification technique ne valide pas la confiance (KindMother reste validateur) | Sandbox / mocks |

### 4.4 Tests Rôle (tool.identity.role)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MAUTH-RO-001** | Rôle citoyen | Vérifie que le rôle « citoyen » est retourné pour un contexte citoyen gouverné | Contexte de test |
| **MAUTH-RO-002** | Rôle visiteur | Vérifie que le rôle « visiteur » est retourné pour un contexte visiteur gouverné | Sandbox / mocks |
| **MAUTH-RO-003** | Rôle externe | Vérifie que le rôle « externe » est retourné pour un contexte externe gouverné | Aucune |
| **MAUTH-RO-004** | Contexte absent | Vérifie le comportement pour un contexte absent ou invalide | Aucune |

---

## 5. Critères de succès et d'échec

### 5.1 Critères de succès

| Critères | Description |
|----------|-------------|
| **Exécution conforme** | Le Tool s'exécute comme spécifié (pas d'exception non contractuelle) |
| **Résultat attendu** | Pour resolve/role : contexte et rôle conformes ; pour attest/verify : structure et verdict conformes |
| **Pas de fuite** | Aucune donnée métier exposée ; sandbox nettoyée après test si applicable |
| **Traçabilité** | Contexte, verdict, durée enregistrés |

### 5.2 Critères d'échec

| Critères | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lève une exception non prévue par le contrat |
| **Modification hors sandbox** | Une donnée métier (identité, Passeport/Visa réel) est modifiée |
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

### 6.1 Exécution d'un test unitaire MiyuAuth

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Chargement définition du test                              │
├─────────────────────────────────────────────────────────────┤
│ - ID du test (MAUTH-*)                                        │
│ - ToolId concerné                                             │
│ - Paramètres (sandbox, mocks, timeout, etc.)                   │
│ - Critères de succès                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Préparation (si sandbox / mocks)                            │
├─────────────────────────────────────────────────────────────┤
│ - Création sandbox / mocks (Passeport/Visa de test) si besoin │
│ - Via gouvernance (KindMother validation mock)                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Exécution du Tool (via gouvernance)                        │
├─────────────────────────────────────────────────────────────┤
│ - BondingBrother → Master Butler → WorrySentinel →            │
│   Caring Nanny → StrongFather → MiyuAuth Tool                 │
│ - Collecte résultat ou exception                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Nettoyage (si sandbox)                                     │
├─────────────────────────────────────────────────────────────┤
│ - Suppression données test / tear-down sandbox                 │
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
|-------|--------------|--------------|-------|
| **Quick** | MAUTH-R-001, MAUTH-RO-001, MAUTH-RO-003 | < 1 min | Vérification rapide |
| **Standard** | Tous MAUTH-R, MAUTH-RO, MAUTH-V (mocks) | 2–5 min | Vérification quotidienne |
| **Full** | Tous MAUTH-* (avec sandbox pour attest/verify) | 5–10 min | Vérification complète |

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Reference Outils | [MiyuAuth - Reference Outils](../../MiyuAuth%20-%20Reference%20Outils.md) |
| MiyuAuth - Cycle Tests Contract | [MiyuAuth - Cycle Tests Contract](./MiyuAuth%20-%20Cycle%20Tests%20Contract.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
