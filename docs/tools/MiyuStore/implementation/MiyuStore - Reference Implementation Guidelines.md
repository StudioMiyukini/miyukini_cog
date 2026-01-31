# MiyuStore — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuStore conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuStore en logique d'implémentation (catalogue, panier, checkout, paiement, livraison, commandes ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuStore (boutique en ligne : produits, panier, checkout, paiement, livraison, commandes) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuStore - Documentation Fondatrice** : ToolkitId `toolkit.commerce.store`, liste des Tools (product.*, cart.*, checkout.*, payment.*, shipping.rate/zones.resolve, order.*).
- **MiyuStore - Reference Outils** : Détail de chaque ToolId.
- **MiyuStore - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather pour checkout/paiement, WriteIntent KindMother ; WorrySentinel niveau paiement).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuStore est invoqué uniquement après décision StrongFather (checkout, capture paiement, création commande, promo). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur les promos, la politique commerciale ou l'autorisation de paiement. Règles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour (produit, panier, commande) = **WriteIntent** vers KindMother. Aucun accès direct à la base. WorrySentinel applique le niveau sécurité paiement (jusqu'à 3 pour payment.*).

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (product.*, cart.*, checkout.*, payment.*, shipping.rate/zones.resolve, order.*).

### 2.7 Niveau de sécurité et états

Niveau **0 à 3** (catalogue 0–1, panier/checkout 1–2, paiement 3). États autorisés : `HEALTHY`, `DEGRADED`. MiyuShipping complète MiyuStore pour étiquettes, comparaison transporteurs, suivi, expéditions.

### 2.8 Alignement MIP/MSCM

Domaine `commerce`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision promo, politique commerciale, autorisation paiement |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (product.*, cart.*, checkout.*, payment.*, shipping.*, order.*) |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée, paiement refusé) remontées sans exposer de données sensibles (numéros carte, coordonnées complètes).
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier sensible).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuStore - Documentation Fondatrice | [Documentation Fondatrice](../MiyuStore%20-%20Documentation%20Fondatrice.md) |
| MiyuStore - Reference Outils | [Reference Outils](../MiyuStore%20-%20Reference%20Outils.md) |
| MiyuStore - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuStore%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
