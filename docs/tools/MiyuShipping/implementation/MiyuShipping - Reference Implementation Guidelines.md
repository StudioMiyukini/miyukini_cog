# MiyuShipping â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuShipping conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuShipping en logique d'implÃ©mentation (tarifs, zones, Ã©tiquettes, suivi, expÃ©ditions ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuShipping (livraison et expÃ©dition : tarifs, zones, Ã©tiquettes, comparaison transporteurs, suivi, expÃ©ditions) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuShipping - Documentation Fondatrice** : ToolkitId `toolkit.commerce.shipping`, liste des Tools (rate, zones.resolve, label.create/print, rates.compare, tracking.get, shipment.create/list).
- **MiyuShipping - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuShipping - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (dÃ©cision StrongFather pour Ã©tiquettes/expÃ©ditions, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuShipping est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (crÃ©ation Ã©tiquette, expÃ©dition). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur la politique de livraison ou le choix des transporteurs. RÃ¨gles (zones, tarifs) fournies par KindMother ou dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation d'expÃ©dition ou mise Ã  jour d'Ã©tat commande = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. Lectures (rate, zones, tracking.get, shipment.list) sur donnÃ©es fournies dans le flux ou gouvernÃ©es.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (rate, zones.resolve, label.*, rates.compare, tracking.get, shipment.create/list).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** (tarifs/suivi 0â€“1, Ã©tiquettes/expÃ©ditions 2). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. MiyuStore inclut rate et zones.resolve pour le checkout ; MiyuShipping agrÃ¨ge l'ensemble des Tools livraison.

### 2.8 Alignement MIP/MSCM

Domaine `commerce`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision politique livraison, transporteurs |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s (rate, zones, label.*, rates.compare, tracking, shipment.*) |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e, transporteur indisponible) remontÃ©es sans exposer de donnÃ©es sensibles (adresses complÃ¨tes, rÃ©fÃ©rences colis).
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuShipping - Documentation Fondatrice | [Documentation Fondatrice](../MiyuShipping%20-%20Documentation%20Fondatrice.md) |
| MiyuShipping - Reference Outils | [Reference Outils](../MiyuShipping%20-%20Reference%20Outils.md) |
| MiyuShipping - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuShipping%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

