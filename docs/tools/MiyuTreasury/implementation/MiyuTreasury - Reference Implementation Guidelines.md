# MiyuTreasury â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuTreasury conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuTreasury en logique d'implÃ©mentation (tableau de bord, prÃ©visionnel, alertes ; lecture agrÃ©gÃ©e, rÃ¨gles alertes = StrongFather).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuTreasury (trÃ©sorerie et prÃ©visionnel : tableau de bord, prÃ©visionnel, alertes) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuTreasury - Documentation Fondatrice** : ToolkitId `toolkit.treasury.forecast`, liste des Tools (dashboard.aggregate, forecast.compute, alert.check).
- **MiyuTreasury - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuTreasury - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (rÃ¨gles alertes = StrongFather ; Tools **lisent** les donnÃ©es KindMother ; pas d'Ã©criture mÃ©tier sauf paramÃ¨tres alertes si dÃ©finis).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuTreasury est invoquÃ© aprÃ¨s passage par la gouvernance. RÃ¨gles d'alerte (seuils, Ã©chÃ©ances) = StrongFather ; le kit exÃ©cute la vÃ©rification sur critÃ¨res fournis. Ne pas rÃ©-Ã©valuer les permissions.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur les seuils, les Ã©chÃ©ances ou la politique d'alerte. RÃ¨gles fournies par StrongFather ou dans le flux. `alert.check` vÃ©rifie selon critÃ¨res fournis, ne dÃ©cide pas.

### 2.3 Pas d'Ã©criture mÃ©tier directe (BOUND-3)

**RÃ¨gle fondamentale MiyuTreasury :** Les Tools **lisent** les donnÃ©es KindMother (Ã©critures, factures, Ã©chÃ©ances) pour agrÃ©gation et prÃ©visionnel. Pas d'Ã©criture mÃ©tier (sauf paramÃ¨tres alertes si dÃ©finis et documentÃ©s). Aucun accÃ¨s direct Ã  la base ; lectures via flux gouvernÃ©.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (dashboard.aggregate, forecast.compute, alert.check).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **1 Ã  2** (donnÃ©es trÃ©sorerie sensibles). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. Ne pas exposer d'agrÃ©gats ou de montants dans les erreurs.

### 2.8 Alignement MIP/MSCM

Domaine `treasury`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution sur mandat ; rÃ¨gles alertes = StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision seuils, Ã©chÃ©ances, politique alerte |
| **BOUND-3** | Pas d'Ã©criture mÃ©tier directe | Lecture uniquement (flux gouvernÃ©) ; pas d'Ã©criture sauf paramÃ¨tres alertes documentÃ©s |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement dashboard.aggregate, forecast.compute, alert.check |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, donnÃ©es indisponibles) remontÃ©es sans exposer de donnÃ©es sensibles (montants, agrÃ©gats).
- En cas de violation de bornage (tentative d'Ã©criture mÃ©tier non documentÃ©e), refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier sensible).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuTreasury - Documentation Fondatrice | [Documentation Fondatrice](../MiyuTreasury%20-%20Documentation%20Fondatrice.md) |
| MiyuTreasury - Reference Outils | [Reference Outils](../MiyuTreasury%20-%20Reference%20Outils.md) |
| MiyuTreasury - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuTreasury%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

