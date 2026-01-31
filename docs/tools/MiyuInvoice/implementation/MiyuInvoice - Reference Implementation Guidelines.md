# MiyuInvoice — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuInvoice conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuInvoice en logique d'implémentation (devis, factures, relances, facturation électronique ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuInvoice (facturation métier indépendants : devis, factures, relances, facturation électronique B2B) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuInvoice - Documentation Fondatrice** : ToolkitId `toolkit.invoice.standalone`, liste des Tools (quote.*, invoice.*, electronic.submit, reminder.send, payment.link.generate, customer.*).
- **MiyuInvoice - Reference Outils** : Détail de chaque ToolId.
- **MiyuInvoice - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather pour relance, conversion devis → facture ; WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuInvoice est invoqué uniquement après décision StrongFather (envoi relance, conversion devis → facture, création facture). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur les règles de facturation, les relances ou la politique commerciale. Règles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour (devis, facture) = **WriteIntent** vers KindMother. Aucun accès direct à la base. Envoi (send, reminder, electronic.submit) = exécution sur mandat ; pas d'écriture métier directe hors WriteIntent.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (quote.*, invoice.*, reminder.send, payment.link.generate, customer.*).

### 2.7 Niveau de sécurité et états

Niveau **1 à 2**. États autorisés : `HEALTHY`, `DEGRADED`. Données facturation sensibles ; ne pas exposer montants ou identifiants clients dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `invoice`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision relance, conversion devis, politique facturation |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (quote.*, invoice.*, reminder, payment.link, customer.*) |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée, plateforme facturation électronique indisponible) remontées sans exposer de données sensibles (montants, clients).
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier sensible).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuInvoice - Documentation Fondatrice | [Documentation Fondatrice](../MiyuInvoice%20-%20Documentation%20Fondatrice.md) |
| MiyuInvoice - Reference Outils | [Reference Outils](../MiyuInvoice%20-%20Reference%20Outils.md) |
| MiyuInvoice - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuInvoice%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
