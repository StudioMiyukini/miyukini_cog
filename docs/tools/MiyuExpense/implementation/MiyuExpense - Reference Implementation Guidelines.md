# MiyuExpense — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuExpense conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuExpense en logique d'implémentation (justificatifs, notes de frais, validation, indemnités kilométriques, export ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuExpense (notes de frais et indemnités : justificatifs, OCR, notes de frais, validation, indemnités kilométriques, export vers compta) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuExpense - Documentation Fondatrice** : ToolkitId `toolkit.expense.claims`, liste des Tools (receipt.*, claim.*, mileage.calculate/export, claim.export).
- **MiyuExpense - Reference Outils** : Détail de chaque ToolId.
- **MiyuExpense - Tool Governance Compliance Contract** : Obligations spécifiques (validation note de frais et export = StrongFather ; WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuExpense est invoqué uniquement après décision StrongFather (validation note de frais, export vers compta). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur les règles de validation, les barèmes kilométriques ou l'export. Règles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour (receipt.capture, claim.create/update/validate) = **WriteIntent** vers KindMother. Aucun accès direct à la base. `claim.export` et `mileage.export` exécutent sur mandat ; pas d'écriture métier directe hors WriteIntent pour les notes de frais.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (receipt.*, claim.*, mileage.*).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** (validation, export = sensible). États autorisés : `HEALTHY`, `DEGRADED`. Ne pas exposer de montants ou de justificatifs dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `expense`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision validation, barème, export |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (receipt.*, claim.*, mileage.*) |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée, OCR échoué) remontées sans exposer de données sensibles (montants, justificatifs).
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier sensible).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuExpense - Documentation Fondatrice | [Documentation Fondatrice](../MiyuExpense%20-%20Documentation%20Fondatrice.md) |
| MiyuExpense - Reference Outils | [Reference Outils](../MiyuExpense%20-%20Reference%20Outils.md) |
| MiyuExpense - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuExpense%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
