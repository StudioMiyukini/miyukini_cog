# MiyuInvoice â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuInvoice conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuInvoice en logique d'implÃ©mentation (devis, factures, relances, facturation Ã©lectronique ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuInvoice (facturation mÃ©tier indÃ©pendants : devis, factures, relances, facturation Ã©lectronique B2B) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuInvoice - Documentation Fondatrice** : ToolkitId `toolkit.invoice.standalone`, liste des Tools (quote.*, invoice.*, electronic.submit, reminder.send, payment.link.generate, customer.*).
- **MiyuInvoice - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuInvoice - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (dÃ©cision StrongFather pour relance, conversion devis â†’ facture ; WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuInvoice est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (envoi relance, conversion devis â†’ facture, crÃ©ation facture). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur les rÃ¨gles de facturation, les relances ou la politique commerciale. RÃ¨gles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation/mise Ã  jour (devis, facture) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. Envoi (send, reminder, electronic.submit) = exÃ©cution sur mandat ; pas d'Ã©criture mÃ©tier directe hors WriteIntent.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (quote.*, invoice.*, reminder.send, payment.link.generate, customer.*).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **1 Ã  2**. Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. DonnÃ©es facturation sensibles ; ne pas exposer montants ou identifiants clients dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `invoice`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision relance, conversion devis, politique facturation |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s (quote.*, invoice.*, reminder, payment.link, customer.*) |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e, plateforme facturation Ã©lectronique indisponible) remontÃ©es sans exposer de donnÃ©es sensibles (montants, clients).
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier sensible).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuInvoice - Documentation Fondatrice | [Documentation Fondatrice](../MiyuInvoice%20-%20Documentation%20Fondatrice.md) |
| MiyuInvoice - Reference Outils | [Reference Outils](../MiyuInvoice%20-%20Reference%20Outils.md) |
| MiyuInvoice - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuInvoice%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

