# MiyuWidgets â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuWidgets conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuWidgets en logique d'implÃ©mentation (layout, widgets, template ; donnÃ©es dans le flux, pas de lecture base directe).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuWidgets (rendu de blocs et composition de layout : layout.apply, widget.*.render, template.resolve) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuWidgets - Documentation Fondatrice** : ToolkitId `toolkit.web.widgets`, liste des Tools (layout.apply, widget.text/image/button/grid/container.render, template.resolve).
- **MiyuWidgets - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuWidgets - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (donnÃ©es dans le flux uniquement ; pas de lecture base directe ; persistance templates/layouts = KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuWidgets est invoquÃ© aprÃ¨s passage par la gouvernance. Ne pas rÃ©-Ã©valuer les permissions. DÃ©cision de contenu ou de logique mÃ©tier = OpÃ©rateurs et Cores, pas le kit.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les **donnÃ©es fournies dans le flux** ; aucune dÃ©cision sur le contenu, la structure ou la visibilitÃ©. MiyuWidgets ne lit pas la base directement.

### 2.3 Pas d'accÃ¨s direct ; donnÃ©es dans le flux (BOUND-3)

**RÃ¨gle fondamentale MiyuWidgets :** Les Tools opÃ¨rent uniquement sur des **donnÃ©es fournies dans le flux**. Aucune lecture directe de la base. Persistance des templates et layouts = KindMother (Ã©crite par d'autres flux). Pas d'Ã©criture mÃ©tier depuis le kit.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (layout.apply, widget.*.render, template.resolve).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** selon politique d'exposition (page builder Ã©ditorial). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. MiyuWeb fournit les capacitÃ©s de base ; MiyuWidgets complÃ¨te pour l'Ã©dition visuelle de pages et de thÃ¨mes.

### 2.8 Alignement MIP/MSCM

Domaine `web`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat ; pas de dÃ©cision contenu |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision contenu, structure, visibilitÃ© |
| **BOUND-3** | Pas d'accÃ¨s direct | DonnÃ©es uniquement dans le flux ; pas de lecture base |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement layout.apply, widget.*.render, template.resolve |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (donnÃ©es manquantes dans le flux, template introuvable) remontÃ©es sans exposer de contenu sensible.
- En cas de violation de bornage (tentative de lecture base, modification contexte), refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuWidgets - Documentation Fondatrice | [Documentation Fondatrice](../MiyuWidgets%20-%20Documentation%20Fondatrice.md) |
| MiyuWidgets - Reference Outils | [Reference Outils](../MiyuWidgets%20-%20Reference%20Outils.md) |
| MiyuWidgets - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuWidgets%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

