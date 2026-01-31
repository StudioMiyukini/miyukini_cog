# MiyuBilling — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuBilling conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuBilling en logique d'implémentation (souscriptions, factures, paiements, tenant ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuBilling (facturation et abonnements SaaS : souscriptions, factures, enregistrement paiements, résolution tenant) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuBilling - Documentation Fondatrice** : ToolkitId `toolkit.billing.saas`, liste des Tools (subscription.*, invoice.*, payment.record, tenant.resolve).
- **MiyuBilling - Reference Outils** : Détail de chaque ToolId.
- **MiyuBilling - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather, WriteIntent KindMother, multi-tenant).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuBilling est invoqué uniquement après décision StrongFather (création souscription, enregistrement paiement, résiliation). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur les offres, tarifs, politique de facturation. Règles fournies par KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour (souscription, facture, paiement) = **WriteIntent** vers KindMother. Aucun accès direct à la base. `tool.billing.tenant.resolve` ne persiste pas ; isolation tenant = KindMother + Border Guard.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (tool.billing.subscription.*, tool.billing.invoice.*, tool.billing.payment.record, tool.billing.tenant.resolve).

### 2.7 Niveau de sécurité et états

Niveau **1 à 3** selon outil (liste factures 1–2, création souscription / enregistrement paiement 2–3). États autorisés : `HEALTHY`, `DEGRADED`. Vérifier WorrySentinel / Caring Nanny avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `billing`, layer Strate 6. Baliser le code MSCM pour blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision offres, tarifs, politique facturation |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement subscription.*, invoice.*, payment.record, tenant.resolve |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée) remontées sans exposer de données sensibles (montants, identifiants clients).
- En cas de violation de bornage (appel sans mandat, tentative d'accès direct), refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier sensible).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuBilling - Documentation Fondatrice | [Documentation Fondatrice](../MiyuBilling%20-%20Documentation%20Fondatrice.md) |
| MiyuBilling - Reference Outils | [Reference Outils](../MiyuBilling%20-%20Reference%20Outils.md) |
| MiyuBilling - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuBilling%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
