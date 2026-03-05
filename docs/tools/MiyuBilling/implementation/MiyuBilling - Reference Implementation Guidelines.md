# MiyuBilling â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuBilling conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuBilling en logique d'implÃ©mentation (souscriptions, factures, paiements, tenant ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuBilling (facturation et abonnements SaaS : souscriptions, factures, enregistrement paiements, rÃ©solution tenant) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuBilling - Documentation Fondatrice** : ToolkitId `toolkit.billing.saas`, liste des Tools (subscription.*, invoice.*, payment.record, tenant.resolve).
- **MiyuBilling - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuBilling - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (dÃ©cision StrongFather, WriteIntent KindMother, multi-tenant).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuBilling est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (crÃ©ation souscription, enregistrement paiement, rÃ©siliation). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur les offres, tarifs, politique de facturation. RÃ¨gles fournies par KindMother ou dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation/mise Ã  jour (souscription, facture, paiement) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. `tool.billing.tenant.resolve` ne persiste pas ; isolation tenant = KindMother + Border Guard.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (tool.billing.subscription.*, tool.billing.invoice.*, tool.billing.payment.record, tool.billing.tenant.resolve).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **1 Ã  3** selon outil (liste factures 1â€“2, crÃ©ation souscription / enregistrement paiement 2â€“3). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. VÃ©rifier WorrySentinel / Caring Nanny avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `billing`, layer Strate 6. Baliser le code MSCM pour blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision offres, tarifs, politique facturation |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement subscription.*, invoice.*, payment.record, tenant.resolve |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e) remontÃ©es sans exposer de donnÃ©es sensibles (montants, identifiants clients).
- En cas de violation de bornage (appel sans mandat, tentative d'accÃ¨s direct), refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier sensible).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuBilling - Documentation Fondatrice | [Documentation Fondatrice](../MiyuBilling%20-%20Documentation%20Fondatrice.md) |
| MiyuBilling - Reference Outils | [Reference Outils](../MiyuBilling%20-%20Reference%20Outils.md) |
| MiyuBilling - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuBilling%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

