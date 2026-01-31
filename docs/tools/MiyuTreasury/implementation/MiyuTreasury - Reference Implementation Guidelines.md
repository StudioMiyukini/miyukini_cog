# MiyuTreasury — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuTreasury conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuTreasury en logique d'implémentation (tableau de bord, prévisionnel, alertes ; lecture agrégée, règles alertes = StrongFather).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuTreasury (trésorerie et prévisionnel : tableau de bord, prévisionnel, alertes) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuTreasury - Documentation Fondatrice** : ToolkitId `toolkit.treasury.forecast`, liste des Tools (dashboard.aggregate, forecast.compute, alert.check).
- **MiyuTreasury - Reference Outils** : Détail de chaque ToolId.
- **MiyuTreasury - Tool Governance Compliance Contract** : Obligations spécifiques (règles alertes = StrongFather ; Tools **lisent** les données KindMother ; pas d'écriture métier sauf paramètres alertes si définis).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuTreasury est invoqué après passage par la gouvernance. Règles d'alerte (seuils, échéances) = StrongFather ; le kit exécute la vérification sur critères fournis. Ne pas ré-évaluer les permissions.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur les seuils, les échéances ou la politique d'alerte. Règles fournies par StrongFather ou dans le flux. `alert.check` vérifie selon critères fournis, ne décide pas.

### 2.3 Pas d'écriture métier directe (BOUND-3)

**Règle fondamentale MiyuTreasury :** Les Tools **lisent** les données KindMother (écritures, factures, échéances) pour agrégation et prévisionnel. Pas d'écriture métier (sauf paramètres alertes si définis et documentés). Aucun accès direct à la base ; lectures via flux gouverné.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (dashboard.aggregate, forecast.compute, alert.check).

### 2.7 Niveau de sécurité et états

Niveau **1 à 2** (données trésorerie sensibles). États autorisés : `HEALTHY`, `DEGRADED`. Ne pas exposer d'agrégats ou de montants dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `treasury`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution sur mandat ; règles alertes = StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision seuils, échéances, politique alerte |
| **BOUND-3** | Pas d'écriture métier directe | Lecture uniquement (flux gouverné) ; pas d'écriture sauf paramètres alertes documentés |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement dashboard.aggregate, forecast.compute, alert.check |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, données indisponibles) remontées sans exposer de données sensibles (montants, agrégats).
- En cas de violation de bornage (tentative d'écriture métier non documentée), refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier sensible).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuTreasury - Documentation Fondatrice | [Documentation Fondatrice](../MiyuTreasury%20-%20Documentation%20Fondatrice.md) |
| MiyuTreasury - Reference Outils | [Reference Outils](../MiyuTreasury%20-%20Reference%20Outils.md) |
| MiyuTreasury - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuTreasury%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
