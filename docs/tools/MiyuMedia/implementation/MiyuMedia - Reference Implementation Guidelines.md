# MiyuMedia â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuMedia conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuMedia en logique d'implÃ©mentation (upload, service, transformation des mÃ©dias ; WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuMedia (gestion des mÃ©dias : upload, service, transformation) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuMedia - Documentation Fondatrice** : ToolkitId `toolkit.content.media`, liste des Tools (tool.media.upload, tool.media.serve, tool.media.transform).
- **MiyuMedia - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuMedia - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (WriteIntent KindMother pour upload ; pas de dÃ©cision politique stockage/quota).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuMedia est invoquÃ© aprÃ¨s passage par la gouvernance. Ne pas rÃ©-Ã©valuer les permissions. Politique de stockage et quotas = StrongFather / Cores ; le kit exÃ©cute uniquement upload, serve, transform.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur la politique de stockage, les quotas ou le classement. RÃ¨gles fournies dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

`tool.media.upload` produit une **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. `tool.media.serve` et `tool.media.transform` opÃ¨rent sur donnÃ©es fournies dans le flux (pas de persistance directe).

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (upload, serve, transform).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** selon politique d'exposition (mÃ©dias publics Ã  sensibles). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. MiyuCMS peut agrÃ©ger MiyuMedia pour le Service CMS complet ; MiyuMedia peut Ãªtre utilisÃ© seul.

### 2.8 Alignement MIP/MSCM

Domaine `content`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat ; politique stockage = Cores |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision politique stockage, quotas |
| **BOUND-3** | Pas d'accÃ¨s direct | Upload = WriteIntent KindMother ; serve/transform sur flux |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement upload, serve, transform |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (fichier invalide, WriteIntent refusÃ©e) remontÃ©es sans exposer de chemins ou mÃ©tadonnÃ©es sensibles.
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuMedia - Documentation Fondatrice | [Documentation Fondatrice](../MiyuMedia%20-%20Documentation%20Fondatrice.md) |
| MiyuMedia - Reference Outils | [Reference Outils](../MiyuMedia%20-%20Reference%20Outils.md) |
| MiyuMedia - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuMedia%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

