# MiyuShipping — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuShipping conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuShipping en logique d'implémentation (tarifs, zones, étiquettes, suivi, expéditions ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuShipping (livraison et expédition : tarifs, zones, étiquettes, comparaison transporteurs, suivi, expéditions) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuShipping - Documentation Fondatrice** : ToolkitId `toolkit.commerce.shipping`, liste des Tools (rate, zones.resolve, label.create/print, rates.compare, tracking.get, shipment.create/list).
- **MiyuShipping - Reference Outils** : Détail de chaque ToolId.
- **MiyuShipping - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather pour étiquettes/expéditions, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuShipping est invoqué uniquement après décision StrongFather (création étiquette, expédition). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur la politique de livraison ou le choix des transporteurs. Règles (zones, tarifs) fournies par KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création d'expédition ou mise à jour d'état commande = **WriteIntent** vers KindMother. Aucun accès direct à la base. Lectures (rate, zones, tracking.get, shipment.list) sur données fournies dans le flux ou gouvernées.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (rate, zones.resolve, label.*, rates.compare, tracking.get, shipment.create/list).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** (tarifs/suivi 0–1, étiquettes/expéditions 2). États autorisés : `HEALTHY`, `DEGRADED`. MiyuStore inclut rate et zones.resolve pour le checkout ; MiyuShipping agrège l'ensemble des Tools livraison.

### 2.8 Alignement MIP/MSCM

Domaine `commerce`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision politique livraison, transporteurs |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (rate, zones, label.*, rates.compare, tracking, shipment.*) |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée, transporteur indisponible) remontées sans exposer de données sensibles (adresses complètes, références colis).
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuShipping - Documentation Fondatrice | [Documentation Fondatrice](../MiyuShipping%20-%20Documentation%20Fondatrice.md) |
| MiyuShipping - Reference Outils | [Reference Outils](../MiyuShipping%20-%20Reference%20Outils.md) |
| MiyuShipping - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuShipping%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
