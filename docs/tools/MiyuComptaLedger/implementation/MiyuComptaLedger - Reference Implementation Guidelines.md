# MiyuComptaLedger â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuComptaLedger conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuComptaLedger en logique d'implÃ©mentation (banque, Ã©critures, rapprochement, TVA, structure ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuComptaLedger (tenue des livres : synchro bancaire, Ã©critures, catÃ©gorisation, TVA, rapprochement, structure) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuComptaLedger - Documentation Fondatrice** : ToolkitId `toolkit.compta.ledger`, liste des Tools (bank.sync, transaction.categorize, transaction.vat.resolve, reconciliation.suggest/record, company.structure.*, company.siret.resolve).
- **MiyuComptaLedger - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuComptaLedger - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (validation rapprochement = StrongFather ; WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuComptaLedger est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (validation rapprochement, enregistrement Ã©criture). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur les rÃ¨gles de catÃ©gorisation, TVA ou rapprochement. RÃ¨gles fournies par KindMother ou dans le flux. `reconciliation.suggest` propose sans dÃ©cider ; dÃ©cision = StrongFather.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation/mise Ã  jour (bank.sync, transaction.categorize, reconciliation.record, company.structure.register) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. `company.siret.resolve` = lecture seule (donnÃ©es externes).

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (bank.sync, transaction.*, reconciliation.*, company.structure.*, company.siret.resolve).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** (donnÃ©es bancaires sensibles). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. Ne pas exposer d'Ã©critures ou de rÃ©fÃ©rences bancaires dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `compta`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision catÃ©gorisation, rapprochement, TVA |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s (bank.*, transaction.*, reconciliation.*, company.*) |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e, API bancaire indisponible) remontÃ©es sans exposer de donnÃ©es sensibles (Ã©critures, rÃ©fÃ©rences bancaires).
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier sensible).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuComptaLedger - Documentation Fondatrice | [Documentation Fondatrice](../MiyuComptaLedger%20-%20Documentation%20Fondatrice.md) |
| MiyuComptaLedger - Reference Outils | [Reference Outils](../MiyuComptaLedger%20-%20Reference%20Outils.md) |
| MiyuComptaLedger - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuComptaLedger%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

