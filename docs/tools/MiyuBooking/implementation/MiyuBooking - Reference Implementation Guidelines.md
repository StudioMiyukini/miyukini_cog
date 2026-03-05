# MiyuBooking â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuBooking conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuBooking en logique d'implÃ©mentation (crÃ©neaux, rÃ©servations, ressources, tarification ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuBooking (rÃ©servation en ligne : crÃ©neaux, rÃ©servations, ressources, prix, participants) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuBooking - Documentation Fondatrice** : ToolkitId `toolkit.booking.reservations`, liste des Tools (slots.*, create, update, cancel, resource.*, price.compute, participants.compute).
- **MiyuBooking - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuBooking - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (dÃ©cision StrongFather, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuBooking est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (crÃ©ation rÃ©servation, annulation). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur la politique de rÃ©servation, les rÃ¨gles de crÃ©neaux ou les tarifs. RÃ¨gles fournies par KindMother ou dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation/mise Ã  jour/annulation de rÃ©servation = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. Lectures (slots.list, resource.availability, price.compute, participants.compute) sur donnÃ©es fournies dans le flux ou gouvernÃ©es.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (slots.list/resolve, create, update, cancel, resource.resolve/availability, price.compute, participants.compute).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** (lecture crÃ©neaux 0â€“1, crÃ©ation/annulation rÃ©servation 1â€“2). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. VÃ©rifier WorrySentinel / Caring Nanny avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `booking`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision politique rÃ©servation, crÃ©neaux, tarifs |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s (slots, create, update, cancel, resource.*, price.compute, participants.compute) |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e) remontÃ©es sans exposer de donnÃ©es sensibles.
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuBooking - Documentation Fondatrice | [Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md) |
| MiyuBooking - Reference Outils | [Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md) |
| MiyuBooking - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

